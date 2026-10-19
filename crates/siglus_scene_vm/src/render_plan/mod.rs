//! Backend-neutral frame planning, shared by the wgpu renderer
//! (`render/mod.rs`) and the PS Vita GXM renderer: the sprites of a frame
//! become draw commands (vertices, per-draw uniforms, textures and the
//! technique/pipeline identity), and wipes their composite parameters. A
//! backend only uploads textures, binds programs and runs the passes, so
//! every backend draws the same effects the same way.

pub(crate) mod emote;

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use bytemuck::{Pod, Zeroable};

use crate::image_manager::{ImageHandle, ImageManager};
use crate::layer::{
    ClipRect, RenderSprite, SpriteBlend, SpriteFit, SpriteSizeMode, WipeRenderPlan,
};
use crate::mesh3d::{MeshAsset, load_mesh_asset};
use crate::render_math::sprite_quad_geometry_rect;

/// The target a frame's sprites are planned for.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PlanTarget {
    /// The game's logical screen.
    pub(crate) win_w: f32,
    pub(crate) win_h: f32,
    /// The render target, and where the logical screen lies in it.
    pub(crate) surface_w: u32,
    pub(crate) surface_h: u32,
    pub(crate) surface_viewport: SurfaceViewport,
}

/// A frame's draw commands, kept between frames to reuse their storage.
#[derive(Debug, Default)]
pub(crate) struct FramePlanner {
    pub(crate) verts: Vec<Vertex>,
    pub(crate) draws: Vec<DrawCommand>,
    /// Bone palettes are only needed by skinned mesh draws.  Keep them out of
    /// DrawCommand so ordinary 2D draws do not retain a 4 KiB zero matrix block
    /// each in the frame command arena.
    pub(crate) draw_bone_uniforms: Vec<BoneUniform>,
    pub(crate) mesh_assets: HashMap<String, MeshAsset>,
}

impl FramePlanner {
    pub(crate) fn ensure_mesh_asset(
        &mut self,
        images: &ImageManager,
        file_name: &str,
    ) -> Option<MeshAsset> {
        if let Some(asset) = self.mesh_assets.get(file_name) {
            return Some(asset.clone());
        }
        let asset =
            load_mesh_asset(images.project_dir(), images.current_append_dir(), file_name).ok()?;
        self.mesh_assets
            .insert(file_name.to_string(), asset.clone());
        Some(asset)
    }

    /// Plans `sprites` in painter order, replacing the previous frame's
    /// commands.
    pub(crate) fn plan(
        &mut self,
        images: &ImageManager,
        sprites: &[RenderSprite],
        target: &PlanTarget,
    ) -> Result<()> {
        self.verts.clear();
        self.draws.clear();
        self.draw_bone_uniforms.clear();
        let PlanTarget {
            win_w,
            win_h,
            surface_w,
            surface_h,
            surface_viewport,
        } = *target;

        for s in sprites {
            let sprite = &s.sprite;
            let img_id = &sprite.image_id;
            let img = img_id.as_ref().and_then(|id| images.get(id));
            let emote_packet = sprite.emote_render.as_deref();
            // The backend prepares the E-mote texture itself (before
            // drawing); the plan only names it.
            let emote_render_id = emote_packet.map(|packet| packet.render_id);
            let (source_width, source_height) = if let Some(img) = img.as_ref() {
                (img.width, img.height)
            } else if let Some(packet) = emote_packet {
                (packet.width, packet.height)
            } else {
                (1, 1)
            };
            // Tona3's SRC_CLIP rectangle is expressed in the sprite's
            // center-relative local coordinate system, not in 0-based texture
            // pixels. For an intrinsic PCT sprite its initial rectangle is
            // [-center, size-center], then rp.src_clip is intersected with it.
            // UVs are derived only after translating the clipped local rectangle
            // back by +center. Keep full-screen presentation on the existing
            // screen-space path; ordinary object sprites use the original local
            // coordinate semantics below.
            let (dst_x, dst_y, local_left, local_top, local_right, local_bottom, u0, v0, u1, v1) =
                match sprite.fit {
                    SpriteFit::FullScreen => {
                        let (src_left, src_top, src_right, src_bottom) =
                            src_clip_rect(sprite.src_clip, source_width, source_height)?;
                        let sw = source_width.max(1) as f32;
                        let sh = source_height.max(1) as f32;
                        (
                            0.0f32,
                            0.0f32,
                            0.0f32,
                            0.0f32,
                            win_w,
                            win_h,
                            (src_left / sw).clamp(0.0, 1.0),
                            (src_top / sh).clamp(0.0, 1.0),
                            (src_right / sw).clamp(0.0, 1.0),
                            (src_bottom / sh).clamp(0.0, 1.0),
                        )
                    }
                    SpriteFit::PixelRect => {
                        let (logical_w, logical_h) = match sprite.size_mode {
                            SpriteSizeMode::Intrinsic => {
                                (source_width.max(1) as f32, source_height.max(1) as f32)
                            }
                            SpriteSizeMode::Explicit { width, height } => {
                                (width.max(1) as f32, height.max(1) as f32)
                            }
                        };
                        let Some((left, top, right, bottom)) =
                            tona_src_clip_local_rect(sprite, logical_w, logical_h)
                        else {
                            continue;
                        };
                        (
                            sprite.x as f32,
                            sprite.y as f32,
                            left,
                            top,
                            right,
                            bottom,
                            (left / logical_w).clamp(0.0, 1.0),
                            (top / logical_h).clamp(0.0, 1.0),
                            (right / logical_w).clamp(0.0, 1.0),
                            (bottom / logical_h).clamp(0.0, 1.0),
                        )
                    }
                };

            let scissor = dst_scissor_rect_to_viewport(
                sprite.dst_clip,
                surface_viewport,
                win_w,
                win_h,
                surface_w,
                surface_h,
            );
            if let Some(sci) = scissor
                && (sci.w == 0 || sci.h == 0)
            {
                continue;
            }

            let alpha = (sprite.alpha as f32) / 255.0;
            let tr = (sprite.tr as f32) / 255.0;
            let mono = (sprite.mono as f32) / 255.0;
            let reverse = (sprite.reverse as f32) / 255.0;
            let bright = (sprite.bright as f32) / 255.0;
            let dark = (sprite.dark as f32) / 255.0;
            let color_rate = (sprite.color_rate as f32) / 255.0;
            let color_add_r = (sprite.color_add_r as f32) / 255.0;
            let color_add_g = (sprite.color_add_g as f32) / 255.0;
            let color_add_b = (sprite.color_add_b as f32) / 255.0;
            let color_r = (sprite.color_r as f32) / 255.0;
            let color_g = (sprite.color_g as f32) / 255.0;
            let color_b = (sprite.color_b as f32) / 255.0;
            let effects1 = [tr, mono, reverse, bright];
            let effects2 = [dark, color_rate, color_add_r, color_add_g];
            let effects3 = [color_add_b, color_r, color_g, color_b];

            let has_mask = sprite
                .mask_image_id
                .as_ref()
                .and_then(|id| images.get(id))
                .is_some();
            let has_tonecurve = sprite
                .tonecurve_image_id
                .as_ref()
                .and_then(|id| images.get(id))
                .is_some();
            let has_wipe_src = sprite
                .wipe_src_image_id
                .as_ref()
                .and_then(|id| images.get(id))
                .is_some();
            let has_fog_tex = sprite
                .fog_texture_image_id
                .as_ref()
                .and_then(|id| images.get(id))
                .is_some();

            let effects4 = [
                sprite.mask_mode as f32,
                if sprite.alpha_test { 1.0 } else { 0.0 },
                if sprite.light_enabled { 1.0 } else { 0.0 },
                if sprite.fog_enabled { 1.0 } else { 0.0 },
            ];
            let effects5 = [
                if has_mask { 1.0 } else { 0.0 },
                if has_tonecurve { 1.0 } else { 0.0 },
                sprite.tonecurve_row,
                sprite.tonecurve_sat,
            ];
            let effects6 = [
                sprite.wipe_fx_mode as f32,
                sprite.wipe_fx_params[0],
                sprite.wipe_fx_params[1],
                sprite.wipe_fx_params[2],
            ];
            let blend_code = match sprite.blend {
                SpriteBlend::Normal => 0.0,
                SpriteBlend::Add => 1.0,
                SpriteBlend::Sub => 2.0,
                SpriteBlend::Mul => 3.0,
                SpriteBlend::Screen => 4.0,
                SpriteBlend::Overlay => 5.0,
            };
            let effects7 = if sprite.wipe_fx_mode >= 10 {
                [
                    sprite.wipe_fx_params[3],
                    if has_wipe_src { 1.0 } else { 0.0 },
                    blend_code,
                    sprite.tonecurve_sat,
                ]
            } else {
                [0.0, if has_wipe_src { 1.0 } else { 0.0 }, blend_code, 0.0]
            };
            let effects8 = [
                sprite.light_diffuse[0],
                sprite.light_diffuse[1],
                sprite.light_diffuse[2],
                sprite.light_factor,
            ];
            let effects9 = [
                sprite.light_ambient[0],
                sprite.light_ambient[1],
                sprite.light_ambient[2],
                sprite.fog_scroll_x,
            ];
            let effects10 = [
                sprite.fog_color[0],
                sprite.fog_color[1],
                sprite.fog_color[2],
                sprite.z,
            ];
            let effects11 = [
                sprite.fog_near,
                sprite.fog_far,
                if has_fog_tex { 1.0 } else { 0.0 },
                sprite.camera_eye[2],
            ];
            let zero4 = [0.0f32; 4];
            let light_pos_kind_base = [
                sprite.light_pos[0],
                sprite.light_pos[1],
                sprite.light_pos[2],
                sprite.light_kind as f32,
            ];
            let light_dir_shadow_base = [
                sprite.light_dir[0],
                sprite.light_dir[1],
                sprite.light_dir[2],
                if sprite.shadow_receive && sprite.light_cone[3] > 0.5 {
                    1.0
                } else {
                    0.0
                },
            ];
            let light_atten_base = sprite.light_atten;
            let light_cone_base = sprite.light_cone;

            let mut special_override = None;
            let mut mesh_batches: Option<Vec<crate::mesh3d::MeshGpuPrimitiveBatch>> = None;
            if sprite.mesh_kind != 0
                && let Some(file_name) = sprite.mesh_file_name.as_deref()
                && let Some(asset) = self.ensure_mesh_asset(images, file_name)
            {
                let anim_state = mesh_animation_state_for_sprite(sprite);
                let sampled = asset.sample_gpu_primitives_with_state(&anim_state);
                if !sampled.is_empty() {
                    special_override = Some(if asset.is_skinned() {
                        TechniqueSpecial::SkinnedMesh
                    } else {
                        TechniqueSpecial::Mesh
                    });
                    mesh_batches = Some(sampled);
                }
            }

            let use_depth = uses_depth_pipeline(sprite);
            let technique = build_technique_key(
                sprite,
                has_mask,
                has_tonecurve,
                has_wipe_src,
                special_override,
            );
            let draw_kind = if matches!(technique.special, TechniqueSpecial::Shadow) {
                MeshDrawKind::ShadowCaster
            } else if matches!(technique.special, TechniqueSpecial::SkinnedMesh) {
                MeshDrawKind::SkinnedMesh
            } else if matches!(technique.special, TechniqueSpecial::Mesh) {
                MeshDrawKind::StaticMesh
            } else {
                MeshDrawKind::SpriteQuad
            };
            let requires_alpha_composition = sprite.alpha < 255
                || sprite.tr < 255
                || has_mask
                || has_tonecurve
                || has_wipe_src
                || sprite.wipe_fx_mode != 0;
            let alpha_blend = if matches!(technique.special, TechniqueSpecial::Overlay) {
                false
            } else {
                sprite.alpha_blend || requires_alpha_composition
            };
            let pipeline_key = PipelineKey {
                technique,
                blend: sprite.blend,
                alpha_blend,
                use_depth,
                depth_write: depth_write_enabled(use_depth, alpha_blend),
                depth_attachment: true,
                cull_back: pipeline_cull_back(sprite, false),
                mesh_fx_variant: 0,
                pipeline_name: String::new(),
                // Mesh batches select the concrete program from each primitive
                // below; this base key is never submitted for that path.
                program: if mesh_batches.is_some() {
                    EffectProgram::Sprite2D
                } else {
                    pipeline_program_for_special(technique.special)
                },
            };

            if let Some(mesh_batches) = mesh_batches {
                let technique_special = special_override.unwrap_or(TechniqueSpecial::Mesh);
                for batch in mesh_batches {
                    if batch.vertices.is_empty() {
                        continue;
                    }
                    let batch_special = if batch.skinned {
                        TechniqueSpecial::SkinnedMesh
                    } else {
                        technique_special
                    };
                    let mut batch_technique = build_technique_key(
                        sprite,
                        has_mask,
                        has_tonecurve,
                        has_wipe_src,
                        Some(batch_special),
                    );
                    batch_technique.tex = batch_technique
                        .tex
                        .max(u8::from(batch.runtime_desc.material_key.use_mesh_tex));
                    batch_technique.mrbd =
                        batch_technique.mrbd || batch.runtime_desc.material_key.use_mrbd;
                    batch_technique.rgb =
                        batch_technique.rgb || batch.runtime_desc.material_key.use_rgb;
                    let batch_draw_kind =
                        if matches!(batch_technique.special, TechniqueSpecial::Shadow) {
                            MeshDrawKind::ShadowCaster
                        } else if matches!(batch_technique.special, TechniqueSpecial::SkinnedMesh) {
                            MeshDrawKind::SkinnedMesh
                        } else if matches!(batch_technique.special, TechniqueSpecial::Mesh) {
                            MeshDrawKind::StaticMesh
                        } else {
                            MeshDrawKind::SpriteQuad
                        };
                    let batch_alpha_blend =
                        if matches!(batch_technique.special, TechniqueSpecial::Overlay) {
                            false
                        } else {
                            sprite.alpha_blend || requires_alpha_composition
                        };
                    let batch_pipeline_key = PipelineKey {
                        technique: batch_technique,
                        blend: sprite.blend,
                        alpha_blend: batch_alpha_blend,
                        use_depth,
                        depth_write: depth_write_enabled(use_depth, batch_alpha_blend),
                        depth_attachment: true,
                        cull_back: pipeline_cull_back(sprite, batch.material.cull_disable),
                        mesh_fx_variant: crate::mesh3d::mesh_effect_variant_bits_from_runtime_desc(
                            &batch.runtime_desc,
                        ),
                        pipeline_name: resolved_mesh_pipeline_name_from_runtime_desc(
                            &batch.runtime_desc,
                            batch_technique,
                        ),
                        program: mesh_effect_program_from_runtime_desc(&batch.runtime_desc),
                    };
                    let base = self.verts.len() as u32;
                    let mut added = 0u32;
                    let mut vs_uniform = if batch.skinned {
                        render_sprite_frame_per_mesh_set_effect_constant_skinned_mesh(
                            sprite,
                            sprite.x as f32,
                            sprite.y as f32,
                            win_w,
                            win_h,
                            batch.frame_cols,
                            &batch.material,
                        )
                    } else {
                        render_sprite_frame_per_mesh_set_effect_constant_mesh(
                            sprite,
                            sprite.x as f32,
                            sprite.y as f32,
                            win_w,
                            win_h,
                            batch.frame_cols,
                            &batch.material,
                        )
                    };
                    debug_assert!(batch.bone_cols.len() <= MAX_BONES);
                    let effects4 = [
                        sprite.mask_mode as f32,
                        if sprite.alpha_test || batch.material.alpha_test_enable {
                            1.0
                        } else {
                            0.0
                        },
                        if sprite.light_enabled { 1.0 } else { 0.0 },
                        if sprite.fog_enabled { 1.0 } else { 0.0 },
                    ];
                    set_sprite2d_effect_uniforms(
                        &mut vs_uniform,
                        effects1,
                        effects2,
                        effects3,
                        effects4,
                        effects5,
                        effects6,
                        effects7,
                        effects8,
                        effects9,
                        effects10,
                        effects11,
                    );
                    for tri in batch.vertices.chunks(3) {
                        if tri.len() != 3 {
                            continue;
                        }
                        let v0_bones = [
                            tri[0].bone_indices[0] as f32,
                            tri[0].bone_indices[1] as f32,
                            tri[0].bone_indices[2] as f32,
                            tri[0].bone_indices[3] as f32,
                        ];
                        let v1_bones = [
                            tri[1].bone_indices[0] as f32,
                            tri[1].bone_indices[1] as f32,
                            tri[1].bone_indices[2] as f32,
                            tri[1].bone_indices[3] as f32,
                        ];
                        let v2_bones = [
                            tri[2].bone_indices[0] as f32,
                            tri[2].bone_indices[1] as f32,
                            tri[2].bone_indices[2] as f32,
                            tri[2].bone_indices[3] as f32,
                        ];
                        let mut v0_effects8 = effects8;
                        let mut v1_effects8 = effects8;
                        let mut v2_effects8 = effects8;
                        let mut v0_effects9 = effects9;
                        let mut v1_effects9 = effects9;
                        let mut v2_effects9 = effects9;
                        v0_effects8[0] = tri[0].color[0];
                        v0_effects8[1] = tri[0].color[1];
                        v0_effects8[2] = tri[0].color[2];
                        v1_effects8[0] = tri[1].color[0];
                        v1_effects8[1] = tri[1].color[1];
                        v1_effects8[2] = tri[1].color[2];
                        v2_effects8[0] = tri[2].color[0];
                        v2_effects8[1] = tri[2].color[1];
                        v2_effects8[2] = tri[2].color[2];
                        v0_effects9[0] = tri[0].color[3];
                        v1_effects9[0] = tri[1].color[3];
                        v2_effects9[0] = tri[2].color[3];
                        self.verts.extend_from_slice(&[
                            Vertex {
                                pos: tri[0].pos,
                                uv: tri[0].uv,
                                uv_aux: [0.0, 0.0],
                                alpha,
                                effects1,
                                effects2,
                                effects3,
                                effects4,
                                effects5,
                                effects6,
                                effects7,
                                effects8: v0_effects8,
                                effects9: v0_effects9,
                                effects10,
                                effects11,
                                world_pos: zero4,
                                world_normal: [
                                    tri[0].normal[0],
                                    tri[0].normal[1],
                                    tri[0].normal[2],
                                    0.0,
                                ],
                                world_tangent: [
                                    tri[0].tangent[0],
                                    tri[0].tangent[1],
                                    tri[0].tangent[2],
                                    0.0,
                                ],
                                world_binormal: [
                                    tri[0].binormal[0],
                                    tri[0].binormal[1],
                                    tri[0].binormal[2],
                                    0.0,
                                ],
                                shadow_pos: zero4,
                                bone_indices: v0_bones,
                                bone_weights: tri[0].bone_weights,
                                light_pos_kind: light_pos_kind_base,
                                light_dir_shadow: light_dir_shadow_base,
                                light_atten: light_atten_base,
                                light_cone: light_cone_base,
                            },
                            Vertex {
                                pos: tri[1].pos,
                                uv: tri[1].uv,
                                uv_aux: [0.0, 0.0],
                                alpha,
                                effects1,
                                effects2,
                                effects3,
                                effects4,
                                effects5,
                                effects6,
                                effects7,
                                effects8: v1_effects8,
                                effects9: v1_effects9,
                                effects10,
                                effects11,
                                world_pos: zero4,
                                world_normal: [
                                    tri[1].normal[0],
                                    tri[1].normal[1],
                                    tri[1].normal[2],
                                    0.0,
                                ],
                                world_tangent: [
                                    tri[1].tangent[0],
                                    tri[1].tangent[1],
                                    tri[1].tangent[2],
                                    0.0,
                                ],
                                world_binormal: [
                                    tri[1].binormal[0],
                                    tri[1].binormal[1],
                                    tri[1].binormal[2],
                                    0.0,
                                ],
                                shadow_pos: zero4,
                                bone_indices: v1_bones,
                                bone_weights: tri[1].bone_weights,
                                light_pos_kind: light_pos_kind_base,
                                light_dir_shadow: light_dir_shadow_base,
                                light_atten: light_atten_base,
                                light_cone: light_cone_base,
                            },
                            Vertex {
                                pos: tri[2].pos,
                                uv: tri[2].uv,
                                uv_aux: [0.0, 0.0],
                                alpha,
                                effects1,
                                effects2,
                                effects3,
                                effects4,
                                effects5,
                                effects6,
                                effects7,
                                effects8: v2_effects8,
                                effects9: v2_effects9,
                                effects10,
                                effects11,
                                world_pos: zero4,
                                world_normal: [
                                    tri[2].normal[0],
                                    tri[2].normal[1],
                                    tri[2].normal[2],
                                    0.0,
                                ],
                                world_tangent: [
                                    tri[2].tangent[0],
                                    tri[2].tangent[1],
                                    tri[2].tangent[2],
                                    0.0,
                                ],
                                world_binormal: [
                                    tri[2].binormal[0],
                                    tri[2].binormal[1],
                                    tri[2].binormal[2],
                                    0.0,
                                ],
                                shadow_pos: zero4,
                                bone_indices: v2_bones,
                                bone_weights: tri[2].bone_weights,
                                light_pos_kind: light_pos_kind_base,
                                light_dir_shadow: light_dir_shadow_base,
                                light_atten: light_atten_base,
                                light_cone: light_cone_base,
                            },
                        ]);
                        added += 3;
                    }
                    if added != 0 {
                        let mesh_material_key =
                            mesh_material_key_for_batch(sprite, batch_technique.special, &batch);
                        let bone_uniform_index =
                            if matches!(
                                batch_draw_kind,
                                MeshDrawKind::SkinnedMesh | MeshDrawKind::ShadowCaster
                            ) && mesh_material_key.as_ref().is_some_and(|key| key.skinned)
                            {
                                let index = self.draw_bone_uniforms.len();
                                self.draw_bone_uniforms
                                    .push(BoneUniform::from_cols_list(&batch.bone_cols));
                                Some(u32::try_from(index).expect("too many bone palette entries"))
                            } else {
                                None
                            };
                        self.draws.push(DrawCommand {
                            image_id: img_id.clone(),
                            emote_render_id: None,
                            mesh_texture_path: batch.texture_path.clone(),
                            mesh_normal_texture_path: batch.material.normal_texture_path.clone(),
                            mesh_toon_texture_path: batch.material.toon_texture_path.clone(),
                            mask_image_id: None,
                            tonecurve_image_id: if has_tonecurve {
                                sprite.tonecurve_image_id.clone()
                            } else {
                                None
                            },
                            fog_image_id: if has_fog_tex {
                                sprite.fog_texture_image_id.clone()
                            } else {
                                None
                            },
                            wipe_src_image_id: if has_wipe_src {
                                sprite.wipe_src_image_id.clone()
                            } else {
                                None
                            },
                            range: base..base + added,
                            scissor,
                            pipeline_key: batch_pipeline_key,
                            shadow_pipeline_name: Some(
                                resolved_shadow_pipeline_name_from_runtime_desc(
                                    &batch.runtime_desc,
                                ),
                            ),
                            draw_kind: batch_draw_kind,
                            mesh_material_key,
                            shadow_cast: sprite.shadow_cast
                                && use_depth
                                && sprite.light_cone[3] > 0.5
                                && batch.material.shadow_map_enable,
                            vs_uniform,
                            bone_uniform_index,
                        });
                    }
                }
                continue;
            }
            if img.is_none() && emote_render_id.is_none() {
                continue;
            }
            let Some(quad) = sprite_quad_geometry_rect(
                sprite,
                dst_x,
                dst_y,
                local_left,
                local_top,
                local_right,
                local_bottom,
                win_w,
                win_h,
            ) else {
                continue;
            };
            let [p0, p1, p2, p3] = quad.projected;

            // tona3 d3-rect/PCT vertices carry their world position and a
            // local +Z normal into the pixel shader. The old Rust path CPU-
            // projected the quad to NDC and discarded both values, so lighting
            // and fog could only use one approximation for the whole sprite.
            // Reconstruct the transformed plane normal from the same world
            // corners used for projection and preserve the per-vertex world
            // positions for interpolation in the fragment shader.
            let has_legacy_world = sprite.camera_enabled && sprite.mesh_kind == 0;
            let (quad_world_pos, quad_world_normal) = if has_legacy_world {
                let world = quad.world;
                let edge_x = [
                    world[1][0] - world[0][0],
                    world[1][1] - world[0][1],
                    world[1][2] - world[0][2],
                ];
                let edge_y = [
                    world[3][0] - world[0][0],
                    world[3][1] - world[0][1],
                    world[3][2] - world[0][2],
                ];
                let mut normal = [
                    edge_x[1] * edge_y[2] - edge_x[2] * edge_y[1],
                    edge_x[2] * edge_y[0] - edge_x[0] * edge_y[2],
                    edge_x[0] * edge_y[1] - edge_x[1] * edge_y[0],
                ];
                let normal_len =
                    (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
                if normal_len > 1e-6 {
                    normal[0] /= normal_len;
                    normal[1] /= normal_len;
                    normal[2] /= normal_len;
                } else {
                    normal = [0.0, 0.0, 1.0];
                }
                (
                    [
                        [world[0][0], world[0][1], world[0][2], p0.clip_w],
                        [world[1][0], world[1][1], world[1][2], p1.clip_w],
                        [world[2][0], world[2][1], world[2][2], p2.clip_w],
                        [world[3][0], world[3][1], world[3][2], p3.clip_w],
                    ],
                    [normal[0], normal[1], normal[2], 1.0],
                )
            } else {
                ([[0.0; 4]; 4], [0.0; 4])
            };

            // Tona3 computes mask texture coordinates from the final 2D vertex
            // positions, not from the source-image UV rectangle.  In
            // C_d3d_sprite::set_d2_vertex_param() the original formula is:
            //
            //   u = linear(vertex.x + 0.5 - mask_x,
            //              -mask_center_x, 0,
            //              -mask_center_x + mask_width_ex, 1)
            //
            // The original D3D9 vertex has already been shifted by -0.5 px, so
            // the +0.5 cancels that rasterization adjustment.  `p0..p3` are
            // logical pixel positions before any half-pixel correction, hence
            // the equivalent coordinate here is simply
            // (vertex - mask_pos + mask_center) / mask_size.
            let mask_uv = if let Some(ref mask_id) = sprite.mask_image_id {
                if let Some(mask_img) = images.get(mask_id) {
                    let mw = mask_img.width.max(1) as f32;
                    let mh = mask_img.height.max(1) as f32;
                    let mask_x = sprite.mask_offset_x as f32;
                    let mask_y = sprite.mask_offset_y as f32;
                    let center_x = mask_img.center_x as f32;
                    let center_y = mask_img.center_y as f32;
                    let uv_for = |p: crate::render_math::ProjectedPoint| {
                        [
                            (p.x - mask_x + center_x) / mw,
                            (p.y - mask_y + center_y) / mh,
                        ]
                    };
                    [uv_for(p0), uv_for(p1), uv_for(p2), uv_for(p3)]
                } else {
                    [[0.0, 0.0]; 4]
                }
            } else {
                [[0.0, 0.0]; 4]
            };
            let base = self.verts.len() as u32;
            let (x0, y0, z0) = pixel_to_ndc(p0.x, p0.y, p0.depth, win_w, win_h);
            let (x1, y1, z1) = pixel_to_ndc(p1.x, p1.y, p1.depth, win_w, win_h);
            let (x2, y2, z2) = pixel_to_ndc(p2.x, p2.y, p2.depth, win_w, win_h);
            let (x3, y3, z3) = pixel_to_ndc(p3.x, p3.y, p3.depth, win_w, win_h);
            self.verts.extend_from_slice(&[
                Vertex {
                    pos: [x0, y0, z0],
                    uv: [u0, v0],
                    uv_aux: mask_uv[0],
                    alpha,
                    effects1,
                    effects2,
                    effects3,
                    effects4,
                    effects5,
                    effects6,
                    effects7,
                    effects8,
                    effects9,
                    effects10,
                    effects11,
                    world_pos: quad_world_pos[0],
                    world_normal: quad_world_normal,
                    world_tangent: zero4,
                    world_binormal: zero4,
                    shadow_pos: zero4,
                    bone_indices: zero4,
                    bone_weights: zero4,
                    light_pos_kind: light_pos_kind_base,
                    light_dir_shadow: light_dir_shadow_base,
                    light_atten: light_atten_base,
                    light_cone: light_cone_base,
                },
                Vertex {
                    pos: [x1, y1, z1],
                    uv: [u1, v0],
                    uv_aux: mask_uv[1],
                    alpha,
                    effects1,
                    effects2,
                    effects3,
                    effects4,
                    effects5,
                    effects6,
                    effects7,
                    effects8,
                    effects9,
                    effects10,
                    effects11,
                    world_pos: quad_world_pos[1],
                    world_normal: quad_world_normal,
                    world_tangent: zero4,
                    world_binormal: zero4,
                    shadow_pos: zero4,
                    bone_indices: zero4,
                    bone_weights: zero4,
                    light_pos_kind: light_pos_kind_base,
                    light_dir_shadow: light_dir_shadow_base,
                    light_atten: light_atten_base,
                    light_cone: light_cone_base,
                },
                Vertex {
                    pos: [x2, y2, z2],
                    uv: [u1, v1],
                    uv_aux: mask_uv[2],
                    alpha,
                    effects1,
                    effects2,
                    effects3,
                    effects4,
                    effects5,
                    effects6,
                    effects7,
                    effects8,
                    effects9,
                    effects10,
                    effects11,
                    world_pos: quad_world_pos[2],
                    world_normal: quad_world_normal,
                    world_tangent: zero4,
                    world_binormal: zero4,
                    shadow_pos: zero4,
                    bone_indices: zero4,
                    bone_weights: zero4,
                    light_pos_kind: light_pos_kind_base,
                    light_dir_shadow: light_dir_shadow_base,
                    light_atten: light_atten_base,
                    light_cone: light_cone_base,
                },
                Vertex {
                    pos: [x0, y0, z0],
                    uv: [u0, v0],
                    uv_aux: mask_uv[0],
                    alpha,
                    effects1,
                    effects2,
                    effects3,
                    effects4,
                    effects5,
                    effects6,
                    effects7,
                    effects8,
                    effects9,
                    effects10,
                    effects11,
                    world_pos: quad_world_pos[0],
                    world_normal: quad_world_normal,
                    world_tangent: zero4,
                    world_binormal: zero4,
                    shadow_pos: zero4,
                    bone_indices: zero4,
                    bone_weights: zero4,
                    light_pos_kind: light_pos_kind_base,
                    light_dir_shadow: light_dir_shadow_base,
                    light_atten: light_atten_base,
                    light_cone: light_cone_base,
                },
                Vertex {
                    pos: [x2, y2, z2],
                    uv: [u1, v1],
                    uv_aux: mask_uv[2],
                    alpha,
                    effects1,
                    effects2,
                    effects3,
                    effects4,
                    effects5,
                    effects6,
                    effects7,
                    effects8,
                    effects9,
                    effects10,
                    effects11,
                    world_pos: quad_world_pos[2],
                    world_normal: quad_world_normal,
                    world_tangent: zero4,
                    world_binormal: zero4,
                    shadow_pos: zero4,
                    bone_indices: zero4,
                    bone_weights: zero4,
                    light_pos_kind: light_pos_kind_base,
                    light_dir_shadow: light_dir_shadow_base,
                    light_atten: light_atten_base,
                    light_cone: light_cone_base,
                },
                Vertex {
                    pos: [x3, y3, z3],
                    uv: [u0, v1],
                    uv_aux: mask_uv[3],
                    alpha,
                    effects1,
                    effects2,
                    effects3,
                    effects4,
                    effects5,
                    effects6,
                    effects7,
                    effects8,
                    effects9,
                    effects10,
                    effects11,
                    world_pos: quad_world_pos[3],
                    world_normal: quad_world_normal,
                    world_tangent: zero4,
                    world_binormal: zero4,
                    shadow_pos: zero4,
                    bone_indices: zero4,
                    bone_weights: zero4,
                    light_pos_kind: light_pos_kind_base,
                    light_dir_shadow: light_dir_shadow_base,
                    light_atten: light_atten_base,
                    light_cone: light_cone_base,
                },
            ]);
            let mut sprite_vs_uniform = sprite2d_uniform_for_effects(
                win_w, win_h, effects1, effects2, effects3, effects4, effects5, effects6, effects7,
                effects8, effects9, effects10, effects11,
            );
            // The original d3 sprite effect receives g_camera_pos and
            // g_light_pos alongside interpolated world-space attributes.
            sprite_vs_uniform.camera_eye = [
                sprite.camera_eye[0],
                sprite.camera_eye[1],
                sprite.camera_eye[2],
                1.0,
            ];
            sprite_vs_uniform.single_light_pos_kind = light_pos_kind_base;

            self.draws.push(DrawCommand {
                image_id: img_id.clone(),
                emote_render_id,
                mesh_texture_path: None,
                mesh_normal_texture_path: None,
                mesh_toon_texture_path: None,
                mask_image_id: if has_mask {
                    sprite.mask_image_id.clone()
                } else {
                    None
                },
                tonecurve_image_id: if has_tonecurve {
                    sprite.tonecurve_image_id.clone()
                } else {
                    None
                },
                fog_image_id: if has_fog_tex {
                    sprite.fog_texture_image_id.clone()
                } else {
                    None
                },
                wipe_src_image_id: if has_wipe_src {
                    sprite.wipe_src_image_id.clone()
                } else {
                    None
                },
                range: base..base + 6,
                scissor,
                pipeline_key,
                shadow_pipeline_name: None,
                draw_kind,
                mesh_material_key: mesh_material_key_for_sprite(sprite, technique.special),
                shadow_cast: sprite.shadow_cast && use_depth,
                vs_uniform: sprite_vs_uniform,
                bone_uniform_index: None,
            });
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    pub(crate) pos: [f32; 3],
    pub(crate) uv: [f32; 2],
    pub(crate) uv_aux: [f32; 2],
    pub(crate) alpha: f32,
    pub(crate) effects1: [f32; 4],
    pub(crate) effects2: [f32; 4],
    pub(crate) effects3: [f32; 4],
    pub(crate) effects4: [f32; 4],
    pub(crate) effects5: [f32; 4],
    pub(crate) effects6: [f32; 4],
    pub(crate) effects7: [f32; 4],
    pub(crate) effects8: [f32; 4],
    pub(crate) effects9: [f32; 4],
    pub(crate) effects10: [f32; 4],
    pub(crate) effects11: [f32; 4],
    pub(crate) world_pos: [f32; 4],
    pub(crate) world_normal: [f32; 4],
    pub(crate) world_tangent: [f32; 4],
    pub(crate) world_binormal: [f32; 4],
    pub(crate) shadow_pos: [f32; 4],
    pub(crate) bone_indices: [f32; 4],
    pub(crate) bone_weights: [f32; 4],
    pub(crate) light_pos_kind: [f32; 4],
    pub(crate) light_dir_shadow: [f32; 4],
    pub(crate) light_atten: [f32; 4],
    pub(crate) light_cone: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct WipeUniform {
    pub(crate) kind_progress: [f32; 4],
    pub(crate) option0: [f32; 4],
    pub(crate) option1: [f32; 4],
    pub(crate) option2: [f32; 4],
    pub(crate) option3: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct PageWipeVertex {
    pub(crate) clip_position: [f32; 4],
    pub(crate) uv: [f32; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WipeMaskCacheKey {
    pub(crate) wipe_type: i32,
    pub(crate) option: Vec<i32>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) seed: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct VsUniform {
    pub(crate) model_col0: [f32; 4],
    pub(crate) model_col1: [f32; 4],
    pub(crate) model_col2: [f32; 4],
    pub(crate) model_col3: [f32; 4],
    pub(crate) normal_col0: [f32; 4],
    pub(crate) normal_col1: [f32; 4],
    pub(crate) normal_col2: [f32; 4],
    pub(crate) frame_col0: [f32; 4],
    pub(crate) frame_col1: [f32; 4],
    pub(crate) frame_col2: [f32; 4],
    pub(crate) frame_col3: [f32; 4],
    pub(crate) frame_normal0: [f32; 4],
    pub(crate) frame_normal1: [f32; 4],
    pub(crate) frame_normal2: [f32; 4],
    pub(crate) camera_eye: [f32; 4],
    pub(crate) camera_forward: [f32; 4],
    pub(crate) camera_right: [f32; 4],
    pub(crate) camera_up: [f32; 4],
    pub(crate) camera_params: [f32; 4],
    pub(crate) shadow_eye: [f32; 4],
    pub(crate) shadow_forward: [f32; 4],
    pub(crate) shadow_right: [f32; 4],
    pub(crate) shadow_up: [f32; 4],
    pub(crate) shadow_params: [f32; 4],
    pub(crate) mtrl_diffuse: [f32; 4],
    pub(crate) mtrl_ambient: [f32; 4],
    pub(crate) mtrl_specular: [f32; 4],
    pub(crate) mtrl_emissive: [f32; 4],
    pub(crate) mtrl_params: [f32; 4],
    pub(crate) mtrl_rim: [f32; 4],
    pub(crate) mtrl_extra: [f32; 4],
    pub(crate) light_diffuse_u: [f32; 4],
    pub(crate) light_ambient_u: [f32; 4],
    pub(crate) light_specular_u: [f32; 4],
    /// Per-draw CFX values. Mesh pipelines read these from the uniform
    /// instead of exceeding WebGPU's 16 vertex-attribute limit.
    pub(crate) sprite_effects: [[f32; 4]; 11],
    pub(crate) single_light_pos_kind: [f32; 4],
    pub(crate) single_light_dir_shadow: [f32; 4],
    pub(crate) single_light_atten: [f32; 4],
    pub(crate) single_light_cone: [f32; 4],
    pub(crate) mesh_flags: [f32; 4],
    pub(crate) mesh_mrbd: [f32; 4],
    pub(crate) mesh_rgb_rate: [f32; 4],
    pub(crate) mesh_add_rgb: [f32; 4],
    pub(crate) mesh_misc: [f32; 4],
    pub(crate) mesh_light_counts: [f32; 4],
    pub(crate) dir_light_diffuse: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) dir_light_ambient: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) dir_light_specular: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) dir_light_dir: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) point_light_diffuse: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) point_light_ambient: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) point_light_specular: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) point_light_pos: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) point_light_atten: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) spot_light_diffuse: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) spot_light_ambient: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) spot_light_specular: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) spot_light_pos: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) spot_light_dir: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) spot_light_atten: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) spot_light_cone: [[f32; 4]; MAX_BATCH_LIGHTS],
    pub(crate) flags: [f32; 4],
}

// The Vita shaders read `VsUniform` as `float4 vs[120]` and `Vertex` with
// the offsets of `Vertex::ATTRS` (render/vita/gpu.rs `MESH_LAYOUT`).
const _: () = assert!(std::mem::size_of::<VsUniform>() == 120 * 16);
const _: () = assert!(std::mem::size_of::<Vertex>() == 384);

impl VsUniform {
    pub(crate) fn for_2d(win_w: f32, win_h: f32) -> Self {
        Self {
            model_col0: [1.0, 0.0, 0.0, 0.0],
            model_col1: [0.0, 1.0, 0.0, 0.0],
            model_col2: [0.0, 0.0, 1.0, 0.0],
            model_col3: [0.0, 0.0, 0.0, 1.0],
            normal_col0: [1.0, 0.0, 0.0, 0.0],
            normal_col1: [0.0, 1.0, 0.0, 0.0],
            normal_col2: [0.0, 0.0, 1.0, 0.0],
            frame_col0: [1.0, 0.0, 0.0, 0.0],
            frame_col1: [0.0, 1.0, 0.0, 0.0],
            frame_col2: [0.0, 0.0, 1.0, 0.0],
            frame_col3: [0.0, 0.0, 0.0, 1.0],
            frame_normal0: [1.0, 0.0, 0.0, 0.0],
            frame_normal1: [0.0, 1.0, 0.0, 0.0],
            frame_normal2: [0.0, 0.0, 1.0, 0.0],
            camera_eye: [0.0, 0.0, 0.0, 0.0],
            camera_forward: [0.0, 0.0, 1.0, 0.0],
            camera_right: [1.0, 0.0, 0.0, 0.0],
            camera_up: [0.0, 1.0, 0.0, 0.0],
            camera_params: [0.0, 0.0, win_w, win_h],
            shadow_eye: [0.0, 0.0, 0.0, 0.0],
            shadow_forward: [0.0, 0.0, 1.0, 0.0],
            shadow_right: [1.0, 0.0, 0.0, 0.0],
            shadow_up: [0.0, 1.0, 0.0, 0.0],
            shadow_params: [1.0, 1.0, 0.0, 0.0],
            mtrl_diffuse: [1.0, 1.0, 1.0, 1.0],
            mtrl_ambient: [1.0, 1.0, 1.0, 1.0],
            mtrl_specular: [0.0, 0.0, 0.0, 1.0],
            mtrl_emissive: [0.0, 0.0, 0.0, 1.0],
            mtrl_params: [16.0, 0.0, 0.0, 0.0],
            mtrl_rim: [1.0, 1.0, 1.0, 1.0],
            mtrl_extra: [0.016, 0.001, 0.0, 0.0],
            light_diffuse_u: [1.0, 1.0, 1.0, 1.0],
            light_ambient_u: [0.0, 0.0, 0.0, 1.0],
            light_specular_u: [0.0, 0.0, 0.0, 1.0],
            sprite_effects: [[0.0; 4]; 11],
            single_light_pos_kind: [0.0, 0.0, 0.0, -1.0],
            single_light_dir_shadow: [0.0, 0.0, -1.0, 0.0],
            single_light_atten: [1.0, 0.0, 0.0, 5000.0],
            single_light_cone: [0.0, 0.0, 1.0, 0.0],
            mesh_flags: [1.0, 0.0, 0.0, 0.0],
            mesh_mrbd: [0.0, 0.0, 0.0, 0.0],
            mesh_rgb_rate: [0.0, 0.0, 0.0, 0.0],
            mesh_add_rgb: [0.0, 0.0, 0.0, 0.0],
            mesh_misc: [1.0, 0.03, 0.0, 0.0],
            mesh_light_counts: [0.0, 0.0, 0.0, 0.0],
            dir_light_diffuse: [[0.0; 4]; MAX_BATCH_LIGHTS],
            dir_light_ambient: [[0.0; 4]; MAX_BATCH_LIGHTS],
            dir_light_specular: [[0.0; 4]; MAX_BATCH_LIGHTS],
            dir_light_dir: [[0.0; 4]; MAX_BATCH_LIGHTS],
            point_light_diffuse: [[0.0; 4]; MAX_BATCH_LIGHTS],
            point_light_ambient: [[0.0; 4]; MAX_BATCH_LIGHTS],
            point_light_specular: [[0.0; 4]; MAX_BATCH_LIGHTS],
            point_light_pos: [[0.0; 4]; MAX_BATCH_LIGHTS],
            point_light_atten: [[0.0; 4]; MAX_BATCH_LIGHTS],
            spot_light_diffuse: [[0.0; 4]; MAX_BATCH_LIGHTS],
            spot_light_ambient: [[0.0; 4]; MAX_BATCH_LIGHTS],
            spot_light_specular: [[0.0; 4]; MAX_BATCH_LIGHTS],
            spot_light_pos: [[0.0; 4]; MAX_BATCH_LIGHTS],
            spot_light_dir: [[0.0; 4]; MAX_BATCH_LIGHTS],
            spot_light_atten: [[0.0; 4]; MAX_BATCH_LIGHTS],
            spot_light_cone: [[0.0; 4]; MAX_BATCH_LIGHTS],
            flags: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

pub(crate) fn set_sprite2d_effect_uniforms(
    u: &mut VsUniform,
    effects1: [f32; 4],
    effects2: [f32; 4],
    effects3: [f32; 4],
    effects4: [f32; 4],
    effects5: [f32; 4],
    effects6: [f32; 4],
    effects7: [f32; 4],
    effects8: [f32; 4],
    effects9: [f32; 4],
    effects10: [f32; 4],
    effects11: [f32; 4],
) {
    u.sprite_effects = [
        effects1, effects2, effects3, effects4, effects5, effects6, effects7, effects8, effects9,
        effects10, effects11,
    ];
}

pub(crate) fn sprite2d_uniform_for_effects(
    win_w: f32,
    win_h: f32,
    effects1: [f32; 4],
    effects2: [f32; 4],
    effects3: [f32; 4],
    effects4: [f32; 4],
    effects5: [f32; 4],
    effects6: [f32; 4],
    effects7: [f32; 4],
    effects8: [f32; 4],
    effects9: [f32; 4],
    effects10: [f32; 4],
    effects11: [f32; 4],
) -> VsUniform {
    let mut u = VsUniform::for_2d(win_w, win_h);
    set_sprite2d_effect_uniforms(
        &mut u, effects1, effects2, effects3, effects4, effects5, effects6, effects7, effects8,
        effects9, effects10, effects11,
    );
    u
}

pub(crate) fn plain_sprite2d_uniform(win_w: f32, win_h: f32) -> VsUniform {
    sprite2d_uniform_for_effects(
        win_w,
        win_h,
        [1.0, 0.0, 0.0, 0.0],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
        [0.0; 4],
    )
}

pub(crate) const MAX_BONES: usize = crate::mesh3d::MAX_GPU_BONE_PALETTE;
pub(crate) const MAX_BATCH_LIGHTS: usize = 4;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct BoneUniform {
    pub(crate) matrices: [[[f32; 4]; 4]; MAX_BONES],
}

impl BoneUniform {
    pub(crate) fn zero() -> Self {
        Self {
            matrices: [[[0.0; 4]; 4]; MAX_BONES],
        }
    }

    pub(crate) fn from_cols_list(cols: &[[[f32; 4]; 4]]) -> Self {
        let mut out = Self::zero();
        for (dst, src) in out.matrices.iter_mut().zip(cols.iter()) {
            *dst = *src;
        }
        out
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct VertexSprite2dData {
    pub(crate) pos: [f32; 3],
    pub(crate) uv: [f32; 2],
    pub(crate) uv_aux: [f32; 2],
    pub(crate) alpha: f32,
    pub(crate) world_pos: [f32; 4],
    pub(crate) world_normal: [f32; 4],
}

impl From<Vertex> for VertexSprite2dData {
    fn from(v: Vertex) -> Self {
        Self {
            pos: v.pos,
            uv: v.uv,
            uv_aux: v.uv_aux,
            alpha: v.alpha,
            world_pos: v.world_pos,
            world_normal: v.world_normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TechniqueSpecial {
    None,
    Overlay,
    WipeMosaic,
    WipeRasterH,
    WipeRasterV,
    WipeExplosionBlur,
    WipeShimi,
    WipeShimiInv,
    WipeCrossMosaic,
    WipeCrossRasterH,
    WipeCrossRasterV,
    WipeCrossExplosionBlur,
    Mesh,
    SkinnedMesh,
    Shadow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EffectProgram {
    Sprite2D,
    OverlayGpu,
    WipeMosaic,
    WipeRasterH,
    WipeRasterV,
    WipeExplosionBlur,
    WipeShimi,
    WipeShimiInv,
    WipeCrossMosaic,
    WipeCrossRasterH,
    WipeCrossRasterV,
    WipeCrossExplosionBlur,
    MeshStaticUnlit,
    MeshStaticLambert,
    MeshStaticBlinnPhong,
    MeshStaticPerPixelBlinnPhong,
    MeshStaticPerPixelHalfLambert,
    MeshStaticToon,
    MeshStaticFixedFunction,
    MeshStaticPerPixelFixedFunction,
    MeshStaticBump,
    MeshStaticParallax,
    MeshSkinnedUnlit,
    MeshSkinnedLambert,
    MeshSkinnedBlinnPhong,
    MeshSkinnedPerPixelBlinnPhong,
    MeshSkinnedPerPixelHalfLambert,
    MeshSkinnedToon,
    MeshSkinnedFixedFunction,
    MeshSkinnedPerPixelFixedFunction,
    MeshSkinnedBump,
    MeshSkinnedParallax,
    ShadowStatic,
    ShadowSkinned,
}

impl EffectProgram {
    pub(crate) fn uses_sprite2d_layout(self) -> bool {
        matches!(
            self,
            EffectProgram::Sprite2D
                | EffectProgram::OverlayGpu
                | EffectProgram::WipeMosaic
                | EffectProgram::WipeRasterH
                | EffectProgram::WipeRasterV
                | EffectProgram::WipeExplosionBlur
                | EffectProgram::WipeShimi
                | EffectProgram::WipeShimiInv
                | EffectProgram::WipeCrossMosaic
                | EffectProgram::WipeCrossRasterH
                | EffectProgram::WipeCrossRasterV
                | EffectProgram::WipeCrossExplosionBlur
        )
    }

    pub(crate) fn vertex_entry(self) -> &'static str {
        match self {
            EffectProgram::Sprite2D
            | EffectProgram::OverlayGpu
            | EffectProgram::WipeMosaic
            | EffectProgram::WipeRasterH
            | EffectProgram::WipeRasterV
            | EffectProgram::WipeExplosionBlur
            | EffectProgram::WipeShimi
            | EffectProgram::WipeShimiInv
            | EffectProgram::WipeCrossMosaic
            | EffectProgram::WipeCrossRasterH
            | EffectProgram::WipeCrossRasterV
            | EffectProgram::WipeCrossExplosionBlur => "vs_sprite_2d",
            EffectProgram::MeshStaticUnlit
            | EffectProgram::MeshStaticLambert
            | EffectProgram::MeshStaticBlinnPhong
            | EffectProgram::MeshStaticPerPixelBlinnPhong
            | EffectProgram::MeshStaticPerPixelHalfLambert
            | EffectProgram::MeshStaticToon
            | EffectProgram::MeshStaticFixedFunction
            | EffectProgram::MeshStaticPerPixelFixedFunction
            | EffectProgram::MeshStaticBump
            | EffectProgram::MeshStaticParallax => "vs_mesh_static",
            EffectProgram::MeshSkinnedUnlit
            | EffectProgram::MeshSkinnedLambert
            | EffectProgram::MeshSkinnedBlinnPhong
            | EffectProgram::MeshSkinnedPerPixelBlinnPhong
            | EffectProgram::MeshSkinnedPerPixelHalfLambert
            | EffectProgram::MeshSkinnedToon
            | EffectProgram::MeshSkinnedFixedFunction
            | EffectProgram::MeshSkinnedPerPixelFixedFunction
            | EffectProgram::MeshSkinnedBump
            | EffectProgram::MeshSkinnedParallax => "vs_mesh_skinned",
            EffectProgram::ShadowStatic => "vs_shadow_static",
            EffectProgram::ShadowSkinned => "vs_shadow_skinned",
        }
    }

    pub(crate) fn fragment_entry(self) -> &'static str {
        match self {
            EffectProgram::Sprite2D => "fs_sprite_2d",
            EffectProgram::OverlayGpu => "fs_overlay_gpu",
            EffectProgram::WipeMosaic => "fs_wipe_mosaic",
            EffectProgram::WipeRasterH => "fs_wipe_raster_h",
            EffectProgram::WipeRasterV => "fs_wipe_raster_v",
            EffectProgram::WipeExplosionBlur => "fs_wipe_explosion_blur",
            EffectProgram::WipeShimi => "fs_wipe_shimi",
            EffectProgram::WipeShimiInv => "fs_wipe_shimi_inv",
            EffectProgram::WipeCrossMosaic => "fs_wipe_cross_mosaic",
            EffectProgram::WipeCrossRasterH => "fs_wipe_cross_raster_h",
            EffectProgram::WipeCrossRasterV => "fs_wipe_cross_raster_v",
            EffectProgram::WipeCrossExplosionBlur => "fs_wipe_cross_explosion_blur",
            EffectProgram::MeshStaticUnlit | EffectProgram::MeshSkinnedUnlit => "fs_mesh_unlit",
            EffectProgram::MeshStaticLambert | EffectProgram::MeshSkinnedLambert => {
                "fs_mesh_lambert"
            }
            EffectProgram::MeshStaticBlinnPhong | EffectProgram::MeshSkinnedBlinnPhong => {
                "fs_mesh_blinn_phong"
            }
            EffectProgram::MeshStaticPerPixelBlinnPhong
            | EffectProgram::MeshSkinnedPerPixelBlinnPhong => "fs_mesh_pp_blinn_phong",
            EffectProgram::MeshStaticPerPixelHalfLambert
            | EffectProgram::MeshSkinnedPerPixelHalfLambert => "fs_mesh_pp_half_lambert",
            EffectProgram::MeshStaticToon | EffectProgram::MeshSkinnedToon => "fs_mesh_toon",
            EffectProgram::MeshStaticFixedFunction | EffectProgram::MeshSkinnedFixedFunction => {
                "fs_mesh_ffp"
            }
            EffectProgram::MeshStaticPerPixelFixedFunction
            | EffectProgram::MeshSkinnedPerPixelFixedFunction => "fs_mesh_pp_ffp",
            EffectProgram::MeshStaticBump | EffectProgram::MeshSkinnedBump => "fs_mesh_bump",
            EffectProgram::MeshStaticParallax | EffectProgram::MeshSkinnedParallax => {
                "fs_mesh_parallax"
            }
            EffectProgram::ShadowStatic | EffectProgram::ShadowSkinned => "fs_shadow_map",
        }
    }

    pub(crate) fn short_name(self) -> &'static str {
        match self {
            EffectProgram::Sprite2D => "sprite2d",
            EffectProgram::OverlayGpu => "overlay_gpu",
            EffectProgram::WipeMosaic => "wipe_mosaic",
            EffectProgram::WipeRasterH => "wipe_raster_h",
            EffectProgram::WipeRasterV => "wipe_raster_v",
            EffectProgram::WipeExplosionBlur => "wipe_explosion_blur",
            EffectProgram::WipeShimi => "wipe_shimi",
            EffectProgram::WipeShimiInv => "wipe_shimi_inv",
            EffectProgram::WipeCrossMosaic => "wipe_cross_mosaic",
            EffectProgram::WipeCrossRasterH => "wipe_cross_raster_h",
            EffectProgram::WipeCrossRasterV => "wipe_cross_raster_v",
            EffectProgram::WipeCrossExplosionBlur => "wipe_cross_explosion_blur",
            EffectProgram::MeshStaticUnlit => "mesh_static_unlit",
            EffectProgram::MeshStaticLambert => "mesh_static_lambert",
            EffectProgram::MeshStaticBlinnPhong => "mesh_static_blinn_phong",
            EffectProgram::MeshStaticPerPixelBlinnPhong => "mesh_static_pp_blinn_phong",
            EffectProgram::MeshStaticPerPixelHalfLambert => "mesh_static_pp_half_lambert",
            EffectProgram::MeshStaticToon => "mesh_static_toon",
            EffectProgram::MeshStaticFixedFunction => "mesh_static_ffp",
            EffectProgram::MeshStaticPerPixelFixedFunction => "mesh_static_pp_ffp",
            EffectProgram::MeshStaticBump => "mesh_static_bump",
            EffectProgram::MeshStaticParallax => "mesh_static_parallax",
            EffectProgram::MeshSkinnedUnlit => "mesh_skinned_unlit",
            EffectProgram::MeshSkinnedLambert => "mesh_skinned_lambert",
            EffectProgram::MeshSkinnedBlinnPhong => "mesh_skinned_blinn_phong",
            EffectProgram::MeshSkinnedPerPixelBlinnPhong => "mesh_skinned_pp_blinn_phong",
            EffectProgram::MeshSkinnedPerPixelHalfLambert => "mesh_skinned_pp_half_lambert",
            EffectProgram::MeshSkinnedToon => "mesh_skinned_toon",
            EffectProgram::MeshSkinnedFixedFunction => "mesh_skinned_ffp",
            EffectProgram::MeshSkinnedPerPixelFixedFunction => "mesh_skinned_pp_ffp",
            EffectProgram::MeshSkinnedBump => "mesh_skinned_bump",
            EffectProgram::MeshSkinnedParallax => "mesh_skinned_parallax",
            EffectProgram::ShadowStatic => "shadow_static",
            EffectProgram::ShadowSkinned => "shadow_skinned",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TechniqueKey {
    pub(crate) d3: bool,
    pub(crate) light: bool,
    pub(crate) fog: bool,
    pub(crate) tex: u8,
    pub(crate) diffuse: bool,
    pub(crate) mrbd: bool,
    pub(crate) rgb: bool,
    pub(crate) tonecurve: bool,
    pub(crate) mask: bool,
    pub(crate) special: TechniqueSpecial,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PipelineKey {
    pub(crate) technique: TechniqueKey,
    pub(crate) blend: SpriteBlend,
    pub(crate) alpha_blend: bool,
    pub(crate) use_depth: bool,
    pub(crate) depth_write: bool,
    pub(crate) depth_attachment: bool,
    pub(crate) cull_back: bool,
    pub(crate) mesh_fx_variant: u64,
    pub(crate) pipeline_name: String,
    pub(crate) program: EffectProgram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct RenderPipelineDepthKey {
    pub(crate) use_depth: bool,
    pub(crate) depth_write: bool,
}

/// Canonical GPU pipeline identity.
///
/// `PipelineKey` keeps the full Siglus/CFX technique identity for semantic
/// decisions and diagnostics. Only fields that actually change the wgpu
/// RenderPipelineDescriptor belong here. This prevents distinct CFX technique
/// names that map to the same generalized WGSL entry point from compiling and
/// retaining duplicate native Metal pipelines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct RenderPipelineKey {
    pub(crate) program: EffectProgram,
    pub(crate) blend: Option<SpriteBlend>,
    pub(crate) depth: Option<RenderPipelineDepthKey>,
    pub(crate) cull_back: bool,
}

impl PipelineKey {
    pub(crate) fn render_pipeline_key(&self) -> RenderPipelineKey {
        RenderPipelineKey {
            program: self.program,
            blend: self.alpha_blend.then_some(self.blend),
            depth: self.depth_attachment.then_some(RenderPipelineDepthKey {
                use_depth: self.use_depth,
                depth_write: self.depth_write,
            }),
            cull_back: self.cull_back,
        }
    }

    pub(crate) fn shadow_render_pipeline_key(&self) -> RenderPipelineKey {
        RenderPipelineKey {
            program: shadow_effect_program_from_source(self.program),
            blend: None,
            depth: Some(RenderPipelineDepthKey {
                use_depth: true,
                depth_write: true,
            }),
            cull_back: self.cull_back,
        }
    }
}

pub(crate) fn sprite2d_copy_render_pipeline_key() -> RenderPipelineKey {
    RenderPipelineKey {
        program: EffectProgram::Sprite2D,
        blend: None,
        depth: None,
        cull_back: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshDrawKind {
    SpriteQuad,
    StaticMesh,
    SkinnedMesh,
    ShadowCaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshMaterialKey {
    pub lighting: bool,
    pub fog: bool,
    pub shadow: bool,
    pub use_mesh_tex: bool,
    pub use_mrbd: bool,
    pub use_rgb: bool,
    pub use_normal_tex: bool,
    pub use_toon_tex: bool,
    pub skinned: bool,
}

#[derive(Debug, Clone)]
pub struct SkinnedPoseState {
    pub world_matrix_count: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SurfaceViewport {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) w: u32,
    pub(crate) h: u32,
}

impl SurfaceViewport {
    pub(crate) fn full(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            w: width.max(1),
            h: height.max(1),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DrawCommand {
    pub(crate) image_id: Option<ImageHandle>,
    pub(crate) emote_render_id: Option<u64>,
    pub(crate) mesh_texture_path: Option<PathBuf>,
    pub(crate) mesh_normal_texture_path: Option<PathBuf>,
    pub(crate) mesh_toon_texture_path: Option<PathBuf>,
    pub(crate) mask_image_id: Option<ImageHandle>,
    pub(crate) tonecurve_image_id: Option<ImageHandle>,
    pub(crate) fog_image_id: Option<ImageHandle>,
    pub(crate) wipe_src_image_id: Option<ImageHandle>,
    pub(crate) range: std::ops::Range<u32>,
    pub(crate) scissor: Option<ScissorRect>,
    pub(crate) pipeline_key: PipelineKey,
    pub(crate) shadow_pipeline_name: Option<String>,
    pub(crate) draw_kind: MeshDrawKind,
    pub(crate) mesh_material_key: Option<MeshMaterialKey>,
    pub(crate) shadow_cast: bool,
    pub(crate) vs_uniform: VsUniform,
    pub(crate) bone_uniform_index: Option<u32>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct ScissorRect {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) w: u32,
    pub(crate) h: u32,
}

pub(crate) fn uses_depth_pipeline(sprite: &crate::layer::Sprite) -> bool {
    sprite.camera_enabled
        || sprite.billboard
        || sprite.z.abs() > f32::EPSILON
        || sprite.pivot_z.abs() > f32::EPSILON
        || (sprite.scale_z - 1.0).abs() > 1e-6
        || sprite.rotate_x.abs() > f32::EPSILON
        || sprite.rotate_y.abs() > f32::EPSILON
}

pub(crate) fn depth_write_enabled(use_depth: bool, alpha_blend: bool) -> bool {
    use_depth && !alpha_blend
}

pub(crate) fn pipeline_cull_back(
    sprite: &crate::layer::Sprite,
    material_cull_disable: bool,
) -> bool {
    uses_depth_pipeline(sprite) && sprite.culling && !material_cull_disable
}

pub(crate) fn sprite_has_mrbd(sprite: &crate::layer::Sprite) -> bool {
    sprite.mono != 0 || sprite.reverse != 0 || sprite.bright != 0 || sprite.dark != 0
}

pub(crate) fn sprite_has_rgb(sprite: &crate::layer::Sprite) -> bool {
    sprite.color_rate != 0
        || sprite.color_add_r != 0
        || sprite.color_add_g != 0
        || sprite.color_add_b != 0
        || sprite.color_r != 0
        || sprite.color_g != 0
        || sprite.color_b != 0
}

pub(crate) fn sprite_has_diffuse(sprite: &crate::layer::Sprite) -> bool {
    uses_depth_pipeline(sprite) || sprite.tr != 255 || sprite.alpha != 255
}

pub(crate) fn is_mesh_special(special: TechniqueSpecial) -> bool {
    matches!(
        special,
        TechniqueSpecial::Mesh | TechniqueSpecial::SkinnedMesh | TechniqueSpecial::Shadow
    )
}

pub(crate) fn is_wipe_special(special: TechniqueSpecial) -> bool {
    matches!(
        special,
        TechniqueSpecial::WipeMosaic
            | TechniqueSpecial::WipeRasterH
            | TechniqueSpecial::WipeRasterV
            | TechniqueSpecial::WipeExplosionBlur
            | TechniqueSpecial::WipeShimi
            | TechniqueSpecial::WipeShimiInv
            | TechniqueSpecial::WipeCrossMosaic
            | TechniqueSpecial::WipeCrossRasterH
            | TechniqueSpecial::WipeCrossRasterV
            | TechniqueSpecial::WipeCrossExplosionBlur
    )
}

pub(crate) fn wipe_special_for_sprite(
    sprite: &crate::layer::Sprite,
    has_wipe_src: bool,
) -> Option<TechniqueSpecial> {
    match (sprite.wipe_fx_mode, has_wipe_src) {
        (1, _) => Some(TechniqueSpecial::WipeMosaic),
        (2, _) => Some(TechniqueSpecial::WipeRasterH),
        (3, _) => Some(TechniqueSpecial::WipeRasterV),
        (4, _) => Some(TechniqueSpecial::WipeExplosionBlur),
        (5, _) => Some(TechniqueSpecial::WipeShimi),
        (6, _) => Some(TechniqueSpecial::WipeShimiInv),
        (10, true) => Some(TechniqueSpecial::WipeCrossMosaic),
        (11, true) => Some(TechniqueSpecial::WipeCrossRasterH),
        (12, true) => Some(TechniqueSpecial::WipeCrossRasterV),
        (13, true) => Some(TechniqueSpecial::WipeCrossExplosionBlur),
        _ => None,
    }
}

pub(crate) fn sprite_has_emote_texture(sprite: &crate::layer::Sprite) -> bool {
    sprite.emote_render.is_some()
}

pub(crate) fn build_technique_key(
    sprite: &crate::layer::Sprite,
    has_mask: bool,
    has_tonecurve: bool,
    has_wipe_src: bool,
    special_override: Option<TechniqueSpecial>,
) -> TechniqueKey {
    let d3 = uses_depth_pipeline(sprite);
    let special = if let Some(s) = special_override {
        s
    } else if matches!(sprite.blend, SpriteBlend::Overlay) {
        TechniqueSpecial::Overlay
    } else if let Some(wipe) = wipe_special_for_sprite(sprite, has_wipe_src) {
        wipe
    } else if sprite.mesh_kind == 3 {
        TechniqueSpecial::SkinnedMesh
    } else if sprite.mesh_kind == 1 || sprite.mesh_kind == 2 {
        TechniqueSpecial::Mesh
    } else {
        TechniqueSpecial::None
    };
    let light = d3 && sprite.light_enabled && !has_mask;
    let fog = d3 && sprite.fog_enabled && !has_mask;
    TechniqueKey {
        d3,
        light,
        fog,
        tex: u8::from(sprite.image_id.is_some() || sprite_has_emote_texture(sprite)),
        diffuse: sprite_has_diffuse(sprite),
        mrbd: sprite_has_mrbd(sprite),
        rgb: sprite_has_rgb(sprite),
        tonecurve: has_tonecurve,
        mask: has_mask,
        special,
    }
}

pub(crate) fn mesh_material_key_for_sprite(
    sprite: &crate::layer::Sprite,
    special: TechniqueSpecial,
) -> Option<MeshMaterialKey> {
    if !is_mesh_special(special) {
        return None;
    }
    Some(MeshMaterialKey {
        lighting: sprite.light_enabled,
        fog: sprite.fog_enabled,
        shadow: sprite.shadow_receive,
        use_mesh_tex: sprite.image_id.is_some() || is_mesh_special(special),
        use_mrbd: sprite_has_mrbd(sprite),
        use_rgb: sprite_has_rgb(sprite),
        use_normal_tex: false,
        use_toon_tex: false,
        skinned: matches!(special, TechniqueSpecial::SkinnedMesh),
    })
}

pub(crate) fn mesh_material_key_for_batch(
    sprite: &crate::layer::Sprite,
    special: TechniqueSpecial,
    batch: &crate::mesh3d::MeshGpuPrimitiveBatch,
) -> Option<MeshMaterialKey> {
    if !is_mesh_special(special) {
        return None;
    }
    Some(MeshMaterialKey {
        lighting: sprite.light_enabled,
        fog: sprite.fog_enabled,
        shadow: sprite.shadow_receive,
        use_mesh_tex: batch.runtime_desc.material_key.use_mesh_tex,
        use_mrbd: sprite_has_mrbd(sprite) || batch.runtime_desc.material_key.use_mrbd,
        use_rgb: sprite_has_rgb(sprite) || batch.runtime_desc.material_key.use_rgb,
        use_normal_tex: batch.runtime_desc.material_key.use_normal_tex,
        use_toon_tex: batch.runtime_desc.material_key.use_toon_tex,
        skinned: batch.runtime_desc.material_key.skinned
            || matches!(special, TechniqueSpecial::SkinnedMesh),
    })
}

#[derive(Clone, Copy)]
pub(crate) struct RVec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl RVec3 {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub(crate) fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
    pub(crate) fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
    pub(crate) fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub(crate) fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    pub(crate) fn normalize(self) -> Self {
        let len = (self.dot(self)).sqrt();
        if len <= 1e-6 {
            Self::new(0.0, 0.0, 0.0)
        } else {
            Self::new(self.x / len, self.y / len, self.z / len)
        }
    }
}

pub(crate) fn rrotate_x(v: RVec3, angle: f32) -> RVec3 {
    let (s, c) = angle.sin_cos();
    RVec3::new(v.x, v.y * c - v.z * s, v.y * s + v.z * c)
}

pub(crate) fn rrotate_y(v: RVec3, angle: f32) -> RVec3 {
    let (s, c) = angle.sin_cos();
    RVec3::new(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

pub(crate) fn rrotate_z(v: RVec3, angle: f32) -> RVec3 {
    let (s, c) = angle.sin_cos();
    RVec3::new(v.x * c - v.y * s, v.x * s + v.y * c, v.z)
}

pub(crate) fn sprite_camera_basis(sprite: &crate::layer::Sprite) -> (RVec3, RVec3, RVec3, RVec3) {
    let eye = RVec3::new(
        sprite.camera_eye[0],
        sprite.camera_eye[1],
        sprite.camera_eye[2],
    );
    let target = RVec3::new(
        sprite.camera_target[0],
        sprite.camera_target[1],
        sprite.camera_target[2],
    );
    let up = RVec3::new(
        sprite.camera_up[0],
        sprite.camera_up[1],
        sprite.camera_up[2],
    );
    let forward = target.sub(eye).normalize();
    let right = up.cross(forward).normalize();
    let up2 = forward.cross(right).normalize();
    (eye, forward, right, up2)
}

pub(crate) fn transform_model_point_world(
    sprite: &crate::layer::Sprite,
    local: [f32; 3],
    anchor_x: f32,
    anchor_y: f32,
) -> [f32; 3] {
    // tona3's mesh world matrix is exactly Scale * Rotation * Translation;
    // rp.center is used to build polygon vertices but is not part of a mesh
    // transform.  Billboards still use their image-space center below.
    let mesh = sprite.mesh_kind != 0 && !sprite.billboard;
    let mut p = if mesh {
        RVec3::new(local[0], local[1], local[2])
    } else {
        RVec3::new(
            local[0] - sprite.pivot_x,
            local[1] - sprite.pivot_y,
            local[2] - sprite.pivot_z,
        )
    };
    p.x *= sprite.scale_x;
    p.y *= sprite.scale_y;
    p.z *= sprite.scale_z;
    if sprite.billboard {
        let (_, _, right, up) = sprite_camera_basis(sprite);
        let (s, c) = sprite.rotate.sin_cos();
        let image_y = -p.y;
        let rx = p.x * c - image_y * s;
        let ry = p.x * s + image_y * c;
        let anchor = RVec3::new(
            anchor_x + sprite.pivot_x,
            anchor_y + sprite.pivot_y,
            sprite.z + sprite.pivot_z,
        );
        let out = anchor.add(RVec3::new(
            right.x * rx + up.x * ry,
            right.y * rx + up.y * ry,
            right.z * rx + up.z * ry,
        ));
        return [out.x, out.y, out.z];
    }
    // D3DXMatrixRotationYawPitchRoll(yaw, pitch, roll) applies roll,
    // then pitch, then yaw to row vectors.
    p = rrotate_z(p, sprite.rotate);
    p = rrotate_x(p, sprite.rotate_x);
    p = rrotate_y(p, sprite.rotate_y);
    p = if mesh {
        p.add(RVec3::new(anchor_x, anchor_y, sprite.z))
    } else {
        p.add(RVec3::new(
            anchor_x + sprite.pivot_x,
            anchor_y + sprite.pivot_y,
            sprite.z + sprite.pivot_z,
        ))
    };
    [p.x, p.y, p.z]
}

pub(crate) fn transform_model_normal_world(
    sprite: &crate::layer::Sprite,
    normal: [f32; 3],
) -> [f32; 3] {
    let mut n = RVec3::new(normal[0], normal[1], normal[2]);
    if sprite.billboard {
        let (_, forward, right, up) = sprite_camera_basis(sprite);
        let basis_z = forward.normalize();
        let basis_x = right.normalize();
        let basis_y = up.normalize();
        let out = RVec3::new(
            basis_x.x * n.x + basis_y.x * n.y + basis_z.x * n.z,
            basis_x.y * n.x + basis_y.y * n.y + basis_z.y * n.z,
            basis_x.z * n.x + basis_y.z * n.y + basis_z.z * n.z,
        )
        .normalize();
        return [out.x, out.y, out.z];
    }
    n = rrotate_z(n, sprite.rotate);
    n = rrotate_x(n, sprite.rotate_x);
    n = rrotate_y(n, sprite.rotate_y);
    n = n.normalize();
    [n.x, n.y, n.z]
}

pub(crate) fn project_shadow_point(
    sprite: &crate::layer::Sprite,
    world: [f32; 3],
) -> Option<[f32; 4]> {
    if sprite.light_kind < 2 {
        return None;
    }
    let eye = RVec3::new(
        sprite.light_pos[0],
        sprite.light_pos[1],
        sprite.light_pos[2],
    );
    let light_dir = RVec3::new(
        sprite.light_dir[0],
        sprite.light_dir[1],
        sprite.light_dir[2],
    )
    .normalize();
    let target = eye.add(light_dir);
    let mut up = RVec3::new(0.0, 1.0, 0.0);
    if up.cross(light_dir).dot(up.cross(light_dir)) <= 1e-6 {
        up = RVec3::new(1.0, 0.0, 0.0);
    }
    let forward = target.sub(eye).normalize();
    let right = up.cross(forward).normalize();
    let up2 = forward.cross(right).normalize();
    let rel = RVec3::new(world[0], world[1], world[2]).sub(eye);
    let cx = rel.dot(right);
    let cy = rel.dot(up2);
    let cz = rel.dot(forward);
    if cz <= 1e-3 {
        return None;
    }
    let fov_deg = if sprite.light_cone[0] > 0.0 {
        (2.0 * sprite.light_cone[0].acos()).to_degrees().max(1.0)
    } else {
        45.0
    };
    let tan_half = (fov_deg.to_radians() * 0.5).tan().max(1e-3);
    let x_ndc = cx / (cz * tan_half);
    let y_ndc = cy / (cz * tan_half);
    if x_ndc.abs() > 1.5 || y_ndc.abs() > 1.5 {
        return None;
    }
    let depth = (cz / sprite.light_atten[3].max(1.0)).clamp(0.0, 1.0);
    Some([x_ndc, y_ndc, depth, 1.0])
}

pub(crate) fn sprite_model_cols(
    sprite: &crate::layer::Sprite,
    anchor_x: f32,
    anchor_y: f32,
) -> ([[f32; 4]; 4], [[f32; 4]; 3]) {
    let origin = transform_model_point_world(sprite, [0.0, 0.0, 0.0], anchor_x, anchor_y);
    let px = transform_model_point_world(sprite, [1.0, 0.0, 0.0], anchor_x, anchor_y);
    let py = transform_model_point_world(sprite, [0.0, 1.0, 0.0], anchor_x, anchor_y);
    let pz = transform_model_point_world(sprite, [0.0, 0.0, 1.0], anchor_x, anchor_y);
    let nx = transform_model_normal_world(sprite, [1.0, 0.0, 0.0]);
    let ny = transform_model_normal_world(sprite, [0.0, 1.0, 0.0]);
    let nz = transform_model_normal_world(sprite, [0.0, 0.0, 1.0]);
    (
        [
            [px[0] - origin[0], px[1] - origin[1], px[2] - origin[2], 0.0],
            [py[0] - origin[0], py[1] - origin[1], py[2] - origin[2], 0.0],
            [pz[0] - origin[0], pz[1] - origin[1], pz[2] - origin[2], 0.0],
            [origin[0], origin[1], origin[2], 1.0],
        ],
        [
            [nx[0], nx[1], nx[2], 0.0],
            [ny[0], ny[1], ny[2], 0.0],
            [nz[0], nz[1], nz[2], 0.0],
        ],
    )
}

pub(crate) type ShadowUniformData = ([f32; 4], [f32; 4], [f32; 4], [f32; 4], [f32; 4]);
pub(crate) fn shadow_uniform_data(sprite: &crate::layer::Sprite) -> ShadowUniformData {
    if sprite.light_kind < 2 {
        return (
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
        );
    }
    let eye = RVec3::new(
        sprite.light_pos[0],
        sprite.light_pos[1],
        sprite.light_pos[2],
    );
    let light_dir = RVec3::new(
        sprite.light_dir[0],
        sprite.light_dir[1],
        sprite.light_dir[2],
    )
    .normalize();
    let mut up = RVec3::new(0.0, 1.0, 0.0);
    if up.cross(light_dir).dot(up.cross(light_dir)) <= 1e-6 {
        up = RVec3::new(1.0, 0.0, 0.0);
    }
    let forward = light_dir;
    let right = up.cross(forward).normalize();
    let up2 = forward.cross(right).normalize();
    let fov_deg = if sprite.light_cone[0] > 0.0 {
        (2.0 * sprite.light_cone[0].acos()).to_degrees().max(1.0)
    } else {
        45.0
    };
    let tan_half = (fov_deg.to_radians() * 0.5).tan().max(1e-3);
    (
        [eye.x, eye.y, eye.z, 0.0],
        [forward.x, forward.y, forward.z, 0.0],
        [right.x, right.y, right.z, 0.0],
        [up2.x, up2.y, up2.z, 0.0],
        [tan_half, sprite.light_atten[3].max(1.0), 1.0, 0.0],
    )
}

pub(crate) fn normalize_col3(v: [f32; 4]) -> [f32; 4] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len <= 1e-6 {
        [0.0, 0.0, 1.0, 0.0]
    } else {
        [v[0] / len, v[1] / len, v[2] / len, 0.0]
    }
}

pub(crate) fn light_id_selected(ids: &[i32], light_id: i32) -> bool {
    ids.is_empty() || ids.contains(&light_id)
}

pub(crate) fn fill_mesh_light_uniforms(
    sprite: &crate::layer::Sprite,
    material: &crate::mesh3d::MeshMaterial,
    u: &mut VsUniform,
) {
    let mut dir_count = 0usize;
    let mut point_count = 0usize;
    let mut spot_count = 0usize;
    for lt in &sprite.mesh_runtime_lights {
        match lt.kind {
            0 if dir_count < MAX_BATCH_LIGHTS
                && light_id_selected(&material.directional_light_ids, lt.id) =>
            {
                u.dir_light_diffuse[dir_count] = lt.diffuse;
                u.dir_light_ambient[dir_count] = lt.ambient;
                u.dir_light_specular[dir_count] = lt.specular;
                u.dir_light_dir[dir_count] = lt.dir;
                dir_count += 1;
            }
            1 if point_count < MAX_BATCH_LIGHTS
                && light_id_selected(&material.point_light_ids, lt.id) =>
            {
                u.point_light_diffuse[point_count] = lt.diffuse;
                u.point_light_ambient[point_count] = lt.ambient;
                u.point_light_specular[point_count] = lt.specular;
                u.point_light_pos[point_count] = lt.pos;
                u.point_light_atten[point_count] = lt.atten;
                point_count += 1;
            }
            2 | 3
                if spot_count < MAX_BATCH_LIGHTS
                    && light_id_selected(&material.spot_light_ids, lt.id) =>
            {
                u.spot_light_diffuse[spot_count] = lt.diffuse;
                u.spot_light_ambient[spot_count] = lt.ambient;
                u.spot_light_specular[spot_count] = lt.specular;
                u.spot_light_pos[spot_count] = lt.pos;
                u.spot_light_dir[spot_count] = lt.dir;
                u.spot_light_atten[spot_count] = lt.atten;
                u.spot_light_cone[spot_count] = lt.cone;
                spot_count += 1;
            }
            _ => {}
        }
    }
    u.mesh_light_counts = [dir_count as f32, point_count as f32, spot_count as f32, 0.0];
}

pub(crate) fn render_sprite_frame_per_mesh_set_effect_constant_common(
    sprite: &crate::layer::Sprite,
    anchor_x: f32,
    anchor_y: f32,
    win_w: f32,
    win_h: f32,
    frame_cols: [[f32; 4]; 4],
    material: &crate::mesh3d::MeshMaterial,
) -> VsUniform {
    let mut u = vertex_uniform_for_mesh(sprite, anchor_x, anchor_y, win_w, win_h);
    u.frame_col0 = frame_cols[0];
    u.frame_col1 = frame_cols[1];
    u.frame_col2 = frame_cols[2];
    u.frame_col3 = frame_cols[3];
    u.frame_normal0 = normalize_col3(frame_cols[0]);
    u.frame_normal1 = normalize_col3(frame_cols[1]);
    u.frame_normal2 = normalize_col3(frame_cols[2]);
    u.mtrl_diffuse = material.diffuse;
    u.mtrl_ambient = material.ambient;
    u.mtrl_specular = material.specular;
    u.mtrl_emissive = material.emissive;
    u.mtrl_params = [
        material.power.max(1.0),
        material.lighting_type as i32 as f32,
        material.shading_type as i32 as f32,
        material.rim_light_power.max(0.0),
    ];
    u.mtrl_rim = material.rim_light_color;
    u.mtrl_extra = [
        material.parallax_max_height.max(0.0),
        material.alpha_ref.clamp(0.0, 1.0),
        material.shader_option as f32,
        0.0,
    ];
    u.light_diffuse_u = sprite.light_diffuse;
    u.light_ambient_u = sprite.light_ambient;
    u.light_specular_u = sprite.light_specular;
    u.mesh_flags = [
        if material.use_mesh_tex { 1.0 } else { 0.0 },
        if material.use_mrbd { 1.0 } else { 0.0 },
        if material.use_rgb { 1.0 } else { 0.0 },
        if material.use_mul_vertex_color {
            1.0
        } else {
            0.0
        },
    ];
    u.mesh_mrbd = material.mrbd;
    u.mesh_rgb_rate = material.rgb_rate;
    u.mesh_add_rgb = material.add_rgb;
    u.mesh_misc = [
        material.mul_vertex_color_rate.max(0.0),
        material.depth_buffer_shadow_bias,
        0.0,
        0.0,
    ];
    fill_mesh_light_uniforms(sprite, material, &mut u);
    u
}

pub(crate) fn render_sprite_frame_per_mesh_set_effect_constant_mesh(
    sprite: &crate::layer::Sprite,
    anchor_x: f32,
    anchor_y: f32,
    win_w: f32,
    win_h: f32,
    frame_cols: [[f32; 4]; 4],
    material: &crate::mesh3d::MeshMaterial,
) -> VsUniform {
    let mut u = render_sprite_frame_per_mesh_set_effect_constant_common(
        sprite, anchor_x, anchor_y, win_w, win_h, frame_cols, material,
    );
    u.flags[3] = 0.0;
    u
}

pub(crate) fn render_sprite_frame_per_mesh_set_effect_constant_skinned_mesh(
    sprite: &crate::layer::Sprite,
    anchor_x: f32,
    anchor_y: f32,
    win_w: f32,
    win_h: f32,
    frame_cols: [[f32; 4]; 4],
    material: &crate::mesh3d::MeshMaterial,
) -> VsUniform {
    let mut u = render_sprite_frame_per_mesh_set_effect_constant_common(
        sprite, anchor_x, anchor_y, win_w, win_h, frame_cols, material,
    );
    u.flags[3] = 1.0;
    u
}

pub(crate) fn vertex_uniform_for_mesh(
    sprite: &crate::layer::Sprite,
    anchor_x: f32,
    anchor_y: f32,
    win_w: f32,
    win_h: f32,
) -> VsUniform {
    let (model_cols, normal_cols) = sprite_model_cols(sprite, anchor_x, anchor_y);
    let (eye, forward, right, up) = sprite_camera_basis(sprite);
    let aspect = if win_h.abs() > f32::EPSILON {
        win_w / win_h
    } else {
        1.0
    };
    let vfov = sprite
        .camera_view_angle_deg
        .to_radians()
        .clamp(1e-3, std::f32::consts::PI - 1e-3);
    let tan_half_v = (vfov * 0.5).tan().max(1e-3);
    let tan_half_h = (tan_half_v * aspect.max(1e-3)).max(1e-3);
    let (shadow_eye, shadow_forward, shadow_right, shadow_up, shadow_params) =
        shadow_uniform_data(sprite);
    VsUniform {
        model_col0: model_cols[0],
        model_col1: model_cols[1],
        model_col2: model_cols[2],
        model_col3: model_cols[3],
        normal_col0: normal_cols[0],
        normal_col1: normal_cols[1],
        normal_col2: normal_cols[2],
        frame_col0: [1.0, 0.0, 0.0, 0.0],
        frame_col1: [0.0, 1.0, 0.0, 0.0],
        frame_col2: [0.0, 0.0, 1.0, 0.0],
        frame_col3: [0.0, 0.0, 0.0, 1.0],
        frame_normal0: [1.0, 0.0, 0.0, 0.0],
        frame_normal1: [0.0, 1.0, 0.0, 0.0],
        frame_normal2: [0.0, 0.0, 1.0, 0.0],
        camera_eye: [eye.x, eye.y, eye.z, 0.0],
        camera_forward: [forward.x, forward.y, forward.z, 0.0],
        camera_right: [right.x, right.y, right.z, 0.0],
        camera_up: [up.x, up.y, up.z, 0.0],
        camera_params: [tan_half_h, tan_half_v, win_w.max(1.0), win_h.max(1.0)],
        shadow_eye,
        shadow_forward,
        shadow_right,
        shadow_up,
        shadow_params,
        mtrl_diffuse: [1.0, 1.0, 1.0, 1.0],
        mtrl_ambient: [1.0, 1.0, 1.0, 1.0],
        mtrl_specular: [0.0, 0.0, 0.0, 1.0],
        mtrl_emissive: [0.0, 0.0, 0.0, 1.0],
        mtrl_params: [16.0, 0.0, 0.0, 0.0],
        mtrl_rim: [1.0, 1.0, 1.0, 1.0],
        mtrl_extra: [0.016, 0.001, 0.0, 0.0],
        light_diffuse_u: sprite.light_diffuse,
        light_ambient_u: sprite.light_ambient,
        light_specular_u: sprite.light_specular,
        sprite_effects: [[0.0; 4]; 11],
        single_light_pos_kind: [
            sprite.light_pos[0],
            sprite.light_pos[1],
            sprite.light_pos[2],
            sprite.light_kind as f32,
        ],
        single_light_dir_shadow: [
            sprite.light_dir[0],
            sprite.light_dir[1],
            sprite.light_dir[2],
            if sprite.shadow_receive { 1.0 } else { 0.0 },
        ],
        single_light_atten: sprite.light_atten,
        single_light_cone: sprite.light_cone,
        mesh_flags: [1.0, 0.0, 0.0, 0.0],
        mesh_mrbd: [0.0, 0.0, 0.0, 0.0],
        mesh_rgb_rate: [0.0, 0.0, 0.0, 0.0],
        mesh_add_rgb: [0.0, 0.0, 0.0, 0.0],
        mesh_misc: [1.0, 0.03, 0.0, 0.0],
        mesh_light_counts: [0.0, 0.0, 0.0, 0.0],
        dir_light_diffuse: [[0.0; 4]; MAX_BATCH_LIGHTS],
        dir_light_ambient: [[0.0; 4]; MAX_BATCH_LIGHTS],
        dir_light_specular: [[0.0; 4]; MAX_BATCH_LIGHTS],
        dir_light_dir: [[0.0; 4]; MAX_BATCH_LIGHTS],
        point_light_diffuse: [[0.0; 4]; MAX_BATCH_LIGHTS],
        point_light_ambient: [[0.0; 4]; MAX_BATCH_LIGHTS],
        point_light_specular: [[0.0; 4]; MAX_BATCH_LIGHTS],
        point_light_pos: [[0.0; 4]; MAX_BATCH_LIGHTS],
        point_light_atten: [[0.0; 4]; MAX_BATCH_LIGHTS],
        spot_light_diffuse: [[0.0; 4]; MAX_BATCH_LIGHTS],
        spot_light_ambient: [[0.0; 4]; MAX_BATCH_LIGHTS],
        spot_light_specular: [[0.0; 4]; MAX_BATCH_LIGHTS],
        spot_light_pos: [[0.0; 4]; MAX_BATCH_LIGHTS],
        spot_light_dir: [[0.0; 4]; MAX_BATCH_LIGHTS],
        spot_light_atten: [[0.0; 4]; MAX_BATCH_LIGHTS],
        spot_light_cone: [[0.0; 4]; MAX_BATCH_LIGHTS],
        flags: [
            1.0,
            if sprite.camera_enabled { 1.0 } else { 0.0 },
            if sprite.light_kind >= 2 { 1.0 } else { 0.0 },
            0.0,
        ],
    }
}

pub(crate) fn mesh_animation_state_for_sprite(
    sprite: &crate::layer::Sprite,
) -> crate::mesh3d::MeshAnimationState {
    sprite.mesh_animation.sanitized()
}

pub(crate) fn resolved_mesh_pipeline_name_from_runtime_desc(
    desc: &crate::mesh3d::MeshPrimitiveRuntimeDesc,
    technique: TechniqueKey,
) -> String {
    let mut technique_name = desc.technique_name.clone();
    if technique.light {
        technique_name.push_str("_light");
    } else if technique.fog {
        technique_name.push_str("_fog");
    }
    if technique.d3 {
        technique_name.push_str("_d3");
    }
    format!("{}::{}", desc.effect_key, technique_name)
}

pub(crate) fn resolved_shadow_pipeline_name_from_runtime_desc(
    desc: &crate::mesh3d::MeshPrimitiveRuntimeDesc,
) -> String {
    format!("{}::{}", desc.shadow_effect_key, desc.shadow_technique_name)
}

pub(crate) fn pipeline_program_for_special(special: TechniqueSpecial) -> EffectProgram {
    match special {
        TechniqueSpecial::Overlay => EffectProgram::OverlayGpu,
        TechniqueSpecial::WipeMosaic => EffectProgram::WipeMosaic,
        TechniqueSpecial::WipeRasterH => EffectProgram::WipeRasterH,
        TechniqueSpecial::WipeRasterV => EffectProgram::WipeRasterV,
        TechniqueSpecial::WipeExplosionBlur => EffectProgram::WipeExplosionBlur,
        TechniqueSpecial::WipeShimi => EffectProgram::WipeShimi,
        TechniqueSpecial::WipeShimiInv => EffectProgram::WipeShimiInv,
        TechniqueSpecial::WipeCrossMosaic => EffectProgram::WipeCrossMosaic,
        TechniqueSpecial::WipeCrossRasterH => EffectProgram::WipeCrossRasterH,
        TechniqueSpecial::WipeCrossRasterV => EffectProgram::WipeCrossRasterV,
        TechniqueSpecial::WipeCrossExplosionBlur => EffectProgram::WipeCrossExplosionBlur,
        TechniqueSpecial::None => EffectProgram::Sprite2D,
        TechniqueSpecial::Mesh | TechniqueSpecial::SkinnedMesh | TechniqueSpecial::Shadow => {
            unreachable!("mesh/shadow techniques must resolve through MeshPrimitiveRuntimeDesc")
        }
    }
}

pub(crate) fn mesh_effect_program_from_runtime_desc(
    desc: &crate::mesh3d::MeshPrimitiveRuntimeDesc,
) -> EffectProgram {
    let skinned = matches!(
        desc.effect_profile,
        crate::mesh3d::MeshEffectProfile::SkinnedMesh
    ) || desc.material_key.skinned;
    match (skinned, desc.material_key.lighting_type) {
        (false, crate::mesh3d::MeshLightingType::None) => EffectProgram::MeshStaticUnlit,
        (false, crate::mesh3d::MeshLightingType::Lambert) => EffectProgram::MeshStaticLambert,
        (false, crate::mesh3d::MeshLightingType::BlinnPhong) => EffectProgram::MeshStaticBlinnPhong,
        (false, crate::mesh3d::MeshLightingType::PerPixelBlinnPhong) => {
            EffectProgram::MeshStaticPerPixelBlinnPhong
        }
        (false, crate::mesh3d::MeshLightingType::PerPixelHalfLambert) => {
            EffectProgram::MeshStaticPerPixelHalfLambert
        }
        (false, crate::mesh3d::MeshLightingType::Toon) => EffectProgram::MeshStaticToon,
        (false, crate::mesh3d::MeshLightingType::FixedFunction) => {
            EffectProgram::MeshStaticFixedFunction
        }
        (false, crate::mesh3d::MeshLightingType::PerPixelFixedFunction) => {
            EffectProgram::MeshStaticPerPixelFixedFunction
        }
        (false, crate::mesh3d::MeshLightingType::Bump) => EffectProgram::MeshStaticBump,
        (false, crate::mesh3d::MeshLightingType::Parallax) => EffectProgram::MeshStaticParallax,
        (true, crate::mesh3d::MeshLightingType::None) => EffectProgram::MeshSkinnedUnlit,
        (true, crate::mesh3d::MeshLightingType::Lambert) => EffectProgram::MeshSkinnedLambert,
        (true, crate::mesh3d::MeshLightingType::BlinnPhong) => EffectProgram::MeshSkinnedBlinnPhong,
        (true, crate::mesh3d::MeshLightingType::PerPixelBlinnPhong) => {
            EffectProgram::MeshSkinnedPerPixelBlinnPhong
        }
        (true, crate::mesh3d::MeshLightingType::PerPixelHalfLambert) => {
            EffectProgram::MeshSkinnedPerPixelHalfLambert
        }
        (true, crate::mesh3d::MeshLightingType::Toon) => EffectProgram::MeshSkinnedToon,
        (true, crate::mesh3d::MeshLightingType::FixedFunction) => {
            EffectProgram::MeshSkinnedFixedFunction
        }
        (true, crate::mesh3d::MeshLightingType::PerPixelFixedFunction) => {
            EffectProgram::MeshSkinnedPerPixelFixedFunction
        }
        (true, crate::mesh3d::MeshLightingType::Bump) => EffectProgram::MeshSkinnedBump,
        (true, crate::mesh3d::MeshLightingType::Parallax) => EffectProgram::MeshSkinnedParallax,
    }
}

pub(crate) fn shadow_effect_program_from_source(src: EffectProgram) -> EffectProgram {
    match src {
        EffectProgram::MeshSkinnedUnlit
        | EffectProgram::MeshSkinnedLambert
        | EffectProgram::MeshSkinnedBlinnPhong
        | EffectProgram::MeshSkinnedPerPixelBlinnPhong
        | EffectProgram::MeshSkinnedPerPixelHalfLambert
        | EffectProgram::MeshSkinnedToon
        | EffectProgram::MeshSkinnedFixedFunction
        | EffectProgram::MeshSkinnedPerPixelFixedFunction
        | EffectProgram::MeshSkinnedBump
        | EffectProgram::MeshSkinnedParallax
        | EffectProgram::ShadowSkinned => EffectProgram::ShadowSkinned,
        _ => EffectProgram::ShadowStatic,
    }
}

pub(crate) fn technique_name_for_pipeline(key: &PipelineKey) -> String {
    let base = if !key.pipeline_name.is_empty() {
        key.pipeline_name.clone()
    } else {
        match key.technique.special {
            TechniqueSpecial::Overlay => "tec_overlay_gpu".to_string(),
            TechniqueSpecial::WipeMosaic => "tec_tex1_mosaic".to_string(),
            TechniqueSpecial::WipeRasterH => "tec_tex1_raster_h".to_string(),
            TechniqueSpecial::WipeRasterV => "tec_tex1_raster_v".to_string(),
            TechniqueSpecial::WipeExplosionBlur => "tec_tex1_explosion_blur".to_string(),
            TechniqueSpecial::WipeShimi => "tec_tex1_shimi".to_string(),
            TechniqueSpecial::WipeShimiInv => "tec_tex1_shimi_inv".to_string(),
            TechniqueSpecial::WipeCrossMosaic => "tec_tex2_mosaic".to_string(),
            TechniqueSpecial::WipeCrossRasterH => "tec_tex2_raster_h".to_string(),
            TechniqueSpecial::WipeCrossRasterV => "tec_tex2_raster_v".to_string(),
            TechniqueSpecial::WipeCrossExplosionBlur => "tec_tex2_explosion_blur".to_string(),
            TechniqueSpecial::Mesh | TechniqueSpecial::SkinnedMesh => {
                let mut name = crate::mesh3d::mesh_effect_key_from_variant(key.mesh_fx_variant);
                if key.technique.light {
                    name.push_str("::tech_light");
                } else if key.technique.fog {
                    name.push_str("::tech_fog");
                } else {
                    name.push_str("::tech");
                }
                if key.technique.d3 {
                    name.push_str("_d3");
                }
                name
            }
            TechniqueSpecial::Shadow => {
                let base_key = crate::mesh3d::MeshRuntimeMaterialKey {
                    use_mesh_tex: false,
                    use_shadow_tex: false,
                    use_toon_tex: false,
                    use_normal_tex: false,
                    use_mul_vertex_color: false,
                    use_mrbd: false,
                    use_rgb: false,
                    lighting_type: crate::mesh3d::MeshLightingType::None,
                    shading_type: crate::mesh3d::MeshShadingType::None,
                    shader_option: crate::mesh3d::MESH_SHADER_OPTION_NONE,
                    skinned: matches!(key.program, EffectProgram::ShadowSkinned),
                    alpha_test_enable: false,
                    cull_disable: false,
                    shadow_map_enable: true,
                };
                format!(
                    "{}::tech",
                    crate::mesh3d::mesh_effect_filename_from_runtime_key(
                        crate::mesh3d::MeshEffectProfile::ShadowMap,
                        base_key,
                    )
                )
            }
            TechniqueSpecial::None => {
                let vertex_name = format!(
                    "{}{}",
                    if key.technique.d3 { "_d3" } else { "" },
                    if key.technique.light {
                        "_light"
                    } else if key.technique.fog {
                        "_fog"
                    } else {
                        ""
                    }
                );
                let pixel_name = format!(
                    "{}{}{}{}{}{}{}{}",
                    if key.technique.light {
                        "_v2"
                    } else if key.technique.fog {
                        "_v1"
                    } else {
                        "_v0"
                    },
                    if key.technique.tex != 0 { "_tex" } else { "" },
                    if key.technique.diffuse {
                        "_diffuse"
                    } else {
                        ""
                    },
                    if key.technique.mrbd { "_mrbd" } else { "" },
                    if key.technique.rgb { "_rgb" } else { "" },
                    if key.technique.tonecurve {
                        "_tonecurve"
                    } else {
                        ""
                    },
                    if key.technique.mask { "_mask" } else { "" },
                    match key.blend {
                        SpriteBlend::Normal => "",
                        SpriteBlend::Add => "_add",
                        SpriteBlend::Sub => "_sub",
                        SpriteBlend::Mul => "_mul",
                        SpriteBlend::Screen => "_screen",
                        SpriteBlend::Overlay => "_overlay",
                    }
                );
                format!("tec{}{}", vertex_name, pixel_name)
            }
        }
    };
    format!("{}#{}", base, key.program.short_name())
}

pub(crate) fn append_fullscreen_blit_vertices(verts: &mut Vec<Vertex>) -> std::ops::Range<u32> {
    let base = verts.len() as u32;
    let effects1 = [1.0, 0.0, 0.0, 0.0];
    let zero = [0.0; 4];
    verts.extend_from_slice(&[
        Vertex {
            pos: [-1.0, 1.0, 0.0],
            uv: [0.0, 0.0],
            uv_aux: [0.0, 0.0],
            alpha: 1.0,
            effects1,
            effects2: zero,
            effects3: zero,
            effects4: zero,
            effects5: zero,
            effects6: zero,
            effects7: zero,
            effects8: zero,
            effects9: zero,
            effects10: zero,
            effects11: zero,
            world_pos: zero,
            world_normal: zero,
            world_tangent: zero,
            world_binormal: zero,
            shadow_pos: zero,
            bone_indices: zero,
            bone_weights: zero,
            light_pos_kind: zero,
            light_dir_shadow: zero,
            light_atten: zero,
            light_cone: zero,
        },
        Vertex {
            pos: [1.0, 1.0, 0.0],
            uv: [1.0, 0.0],
            uv_aux: [0.0, 0.0],
            alpha: 1.0,
            effects1,
            effects2: zero,
            effects3: zero,
            effects4: zero,
            effects5: zero,
            effects6: zero,
            effects7: zero,
            effects8: zero,
            effects9: zero,
            effects10: zero,
            effects11: zero,
            world_pos: zero,
            world_normal: zero,
            world_tangent: zero,
            world_binormal: zero,
            shadow_pos: zero,
            bone_indices: zero,
            bone_weights: zero,
            light_pos_kind: zero,
            light_dir_shadow: zero,
            light_atten: zero,
            light_cone: zero,
        },
        Vertex {
            pos: [1.0, -1.0, 0.0],
            uv: [1.0, 1.0],
            uv_aux: [0.0, 0.0],
            alpha: 1.0,
            effects1,
            effects2: zero,
            effects3: zero,
            effects4: zero,
            effects5: zero,
            effects6: zero,
            effects7: zero,
            effects8: zero,
            effects9: zero,
            effects10: zero,
            effects11: zero,
            world_pos: zero,
            world_normal: zero,
            world_tangent: zero,
            world_binormal: zero,
            shadow_pos: zero,
            bone_indices: zero,
            bone_weights: zero,
            light_pos_kind: zero,
            light_dir_shadow: zero,
            light_atten: zero,
            light_cone: zero,
        },
        Vertex {
            pos: [-1.0, 1.0, 0.0],
            uv: [0.0, 0.0],
            uv_aux: [0.0, 0.0],
            alpha: 1.0,
            effects1,
            effects2: zero,
            effects3: zero,
            effects4: zero,
            effects5: zero,
            effects6: zero,
            effects7: zero,
            effects8: zero,
            effects9: zero,
            effects10: zero,
            effects11: zero,
            world_pos: zero,
            world_normal: zero,
            world_tangent: zero,
            world_binormal: zero,
            shadow_pos: zero,
            bone_indices: zero,
            bone_weights: zero,
            light_pos_kind: zero,
            light_dir_shadow: zero,
            light_atten: zero,
            light_cone: zero,
        },
        Vertex {
            pos: [1.0, -1.0, 0.0],
            uv: [1.0, 1.0],
            uv_aux: [0.0, 0.0],
            alpha: 1.0,
            effects1,
            effects2: zero,
            effects3: zero,
            effects4: zero,
            effects5: zero,
            effects6: zero,
            effects7: zero,
            effects8: zero,
            effects9: zero,
            effects10: zero,
            effects11: zero,
            world_pos: zero,
            world_normal: zero,
            world_tangent: zero,
            world_binormal: zero,
            shadow_pos: zero,
            bone_indices: zero,
            bone_weights: zero,
            light_pos_kind: zero,
            light_dir_shadow: zero,
            light_atten: zero,
            light_cone: zero,
        },
        Vertex {
            pos: [-1.0, -1.0, 0.0],
            uv: [0.0, 1.0],
            uv_aux: [0.0, 0.0],
            alpha: 1.0,
            effects1,
            effects2: zero,
            effects3: zero,
            effects4: zero,
            effects5: zero,
            effects6: zero,
            effects7: zero,
            effects8: zero,
            effects9: zero,
            effects10: zero,
            effects11: zero,
            world_pos: zero,
            world_normal: zero,
            world_tangent: zero,
            world_binormal: zero,
            shadow_pos: zero,
            bone_indices: zero,
            bone_weights: zero,
            light_pos_kind: zero,
            light_dir_shadow: zero,
            light_atten: zero,
            light_cone: zero,
        },
    ]);
    base..base + 6
}

pub(crate) fn pixel_to_ndc(x: f32, y: f32, depth: f32, win_w: f32, win_h: f32) -> (f32, f32, f32) {
    let nx = (x / win_w) * 2.0 - 1.0;
    let ny = 1.0 - (y / win_h) * 2.0;
    // WGPU follows the Direct3D/Vulkan clip-space convention: z is 0..1.
    // The previous OpenGL-style mapping (depth * 2 - 1) put all 2D quads at z=-1,
    // which is outside WGPU clip space and makes the game window render black even
    // though the VM submits sprites correctly.
    let nz = depth.clamp(0.0, 1.0);
    (nx, ny, nz)
}

pub(crate) fn tona_src_clip_local_rect(
    sprite: &crate::layer::Sprite,
    logical_w: f32,
    logical_h: f32,
) -> Option<(f32, f32, f32, f32)> {
    let logical_w = logical_w.max(1.0);
    let logical_h = logical_h.max(1.0);

    // C_d3d_sprite::set_d2_vertex_param(): when the sprite size follows
    // texture 0, the texture center is added to rp.center before clipping.
    // Our object_anchor transform subtracts exactly the same combined center.
    let center_x = sprite.pivot_x
        + if sprite.object_anchor {
            sprite.texture_center_x
        } else {
            0.0
        };
    let center_y = sprite.pivot_y
        + if sprite.object_anchor {
            sprite.texture_center_y
        } else {
            0.0
        };

    let mut local_left = -center_x;
    let mut local_top = -center_y;
    let mut local_right = logical_w - center_x;
    let mut local_bottom = logical_h - center_y;

    if let Some(clip) = sprite.src_clip {
        local_left = local_left.max(clip.left as f32);
        local_top = local_top.max(clip.top as f32);
        local_right = local_right.min(clip.right as f32);
        local_bottom = local_bottom.min(clip.bottom as f32);
    }

    if local_right <= local_left || local_bottom <= local_top {
        return None;
    }

    // Translate the center-relative local coordinates back to the 0-based
    // sprite rectangle. These values are both the pre-transform vertex
    // coordinates used by our renderer and the numerator of Tona3's UV
    // calculation: (src_clip + center) / size.
    Some((
        local_left + center_x,
        local_top + center_y,
        local_right + center_x,
        local_bottom + center_y,
    ))
}

pub(crate) fn src_clip_rect(
    clip: Option<ClipRect>,
    img_w: u32,
    img_h: u32,
) -> Result<(f32, f32, f32, f32)> {
    if let Some(c) = clip {
        let mut left = c.left.max(0) as f32;
        let mut top = c.top.max(0) as f32;
        let mut right = c.right.max(0) as f32;
        let mut bottom = c.bottom.max(0) as f32;
        let max_w = img_w as f32;
        let max_h = img_h as f32;
        left = left.min(max_w);
        right = right.min(max_w);
        top = top.min(max_h);
        bottom = bottom.min(max_h);
        if right <= left || bottom <= top {
            return Ok((0.0, 0.0, max_w, max_h));
        }
        Ok((left, top, right, bottom))
    } else {
        Ok((0.0, 0.0, img_w as f32, img_h as f32))
    }
}

pub(crate) fn dst_scissor_rect_to_viewport(
    clip: Option<ClipRect>,
    viewport: SurfaceViewport,
    logical_w: f32,
    logical_h: f32,
    surface_w: u32,
    surface_h: u32,
) -> Option<ScissorRect> {
    let c = clip?;
    let sx = (viewport.w as f32) / logical_w.max(1.0);
    let sy = (viewport.h as f32) / logical_h.max(1.0);
    let mut left = viewport.x as i64 + ((c.left.max(0) as f32) * sx).floor() as i64;
    let mut top = viewport.y as i64 + ((c.top.max(0) as f32) * sy).floor() as i64;
    let mut right = viewport.x as i64 + ((c.right.max(0) as f32) * sx).ceil() as i64;
    let mut bottom = viewport.y as i64 + ((c.bottom.max(0) as f32) * sy).ceil() as i64;
    let max_w = surface_w as i64;
    let max_h = surface_h as i64;
    left = left.min(max_w);
    right = right.min(max_w);
    top = top.min(max_h);
    bottom = bottom.min(max_h);
    if right <= left || bottom <= top {
        return Some(ScissorRect {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        });
    }
    Some(ScissorRect {
        x: left as u32,
        y: top as u32,
        w: (right - left) as u32,
        h: (bottom - top) as u32,
    })
}

#[derive(Debug)]
pub(crate) struct PageWipeDraw {
    pub(crate) use_current: bool,
    pub(crate) vertices: Vec<PageWipeVertex>,
}

pub(crate) fn wipe_option(options: &[i32], index: usize, default: i32) -> i32 {
    options.get(index).copied().unwrap_or(default)
}

pub(crate) fn page_range_angle(start: f32, end: f32, range_type: i32, progress: f32) -> f32 {
    let half = (start + end) * 0.5;
    match range_type {
        1 => half + (end - half) * progress,
        2 => start + (half - start) * progress,
        _ => start + (end - start) * progress,
    }
}

pub(crate) fn project_page_wipe_vertex(
    position: [f32; 3],
    uv: [f32; 2],
    angle: f32,
    width: f32,
    height: f32,
    fov_radians: f32,
) -> PageWipeVertex {
    let (sin, cos) = angle.sin_cos();
    let world_x = position[0] * cos + position[2] * sin;
    let world_z = -position[0] * sin + position[2] * cos;
    let tan_half = (fov_radians * 0.5).tan().max(0.000001);
    let focal = height * 0.5 / tan_half;
    let view_z = (world_z + focal).max(0.000001);
    let y_scale = 1.0 / tan_half;
    let aspect = (width / height.max(1.0)).max(0.000001);
    let x_scale = y_scale / aspect;
    let near = 1.0;
    let far = 10000.0;
    let clip_z = far / (far - near) * view_z - near * far / (far - near);
    PageWipeVertex {
        clip_position: [world_x * x_scale, position[1] * y_scale, clip_z, view_z],
        uv,
    }
}

pub(crate) fn page_quad_vertices(
    positions: [[f32; 3]; 4],
    uvs: [[f32; 2]; 4],
    angle: f32,
    width: f32,
    height: f32,
    fov_radians: f32,
) -> Vec<PageWipeVertex> {
    let projected = positions
        .into_iter()
        .zip(uvs)
        .map(|(position, uv)| {
            project_page_wipe_vertex(position, uv, angle, width, height, fov_radians)
        })
        .collect::<Vec<_>>();
    [0usize, 2, 1, 2, 3, 1]
        .into_iter()
        .map(|index| projected[index])
        .collect()
}

pub(crate) fn build_page_300_draw(
    wipe: &WipeRenderPlan,
    width: f32,
    height: f32,
    use_current: bool,
    is_front: bool,
) -> PageWipeDraw {
    let reverse = wipe_option(&wipe.option, 0, 0) != 0;
    let (start, end) = if is_front {
        if reverse {
            (std::f32::consts::PI, 0.0)
        } else {
            (std::f32::consts::PI, std::f32::consts::TAU)
        }
    } else if reverse {
        (0.0, -std::f32::consts::PI)
    } else {
        (0.0, std::f32::consts::PI)
    };
    let angle = page_range_angle(
        start,
        end,
        wipe_option(&wipe.option, 2, 0),
        wipe.progress.clamp(0.0, 1.0),
    );
    let half_width = width * 0.5;
    let half_height = height * 0.5;
    let positions = [
        [-half_width - 0.5, half_height + 0.5, 0.0],
        [half_width - 0.5, half_height + 0.5, 0.0],
        [-half_width - 0.5, -half_height + 0.5, 0.0],
        [half_width - 0.5, -half_height + 0.5, 0.0],
    ];
    let uvs = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let fov = (wipe_option(&wipe.option, 1, 450) as f32 / 10.0)
        .clamp(1.0, 179.0)
        .to_radians();
    PageWipeDraw {
        use_current,
        vertices: page_quad_vertices(positions, uvs, angle, width, height, fov),
    }
}

pub(crate) fn build_page_301_draws_for_stage(
    wipe: &WipeRenderPlan,
    width: f32,
    height: f32,
    use_current: bool,
    is_front: bool,
) -> Vec<PageWipeDraw> {
    let reverse = wipe_option(&wipe.option, 0, 0) != 0;
    let fov = (wipe_option(&wipe.option, 1, 450) as f32 / 10.0)
        .clamp(1.0, 179.0)
        .to_radians();
    let half_width = width * 0.5;
    let half_height = height * 0.5;
    let mut out = Vec::with_capacity(2);
    for half in 0..2 {
        let (start, end, z) = match (half, is_front, reverse) {
            (0, true, true) => (0.0, 0.0, 0.0),
            (0, true, false) => (std::f32::consts::PI, std::f32::consts::TAU, -1.0),
            (0, false, true) => (std::f32::consts::TAU, std::f32::consts::PI, -1.0),
            (0, false, false) => (0.0, 0.0, 0.0),
            (1, true, true) => (std::f32::consts::PI, 0.0, -1.0),
            (1, true, false) => (0.0, 0.0, 0.0),
            (1, false, true) => (0.0, 0.0, 0.0),
            (1, false, false) => (0.0, std::f32::consts::PI, -1.0),
            _ => unreachable!(),
        };
        let angle = page_range_angle(
            start,
            end,
            wipe_option(&wipe.option, 2, 0),
            wipe.progress.clamp(0.0, 1.0),
        );
        let (x0, x1, u0, u1) = if half == 0 {
            (-half_width - 0.5, -0.5, 0.0, 0.5)
        } else {
            (-0.5, half_width - 0.5, 0.5, 1.0)
        };
        let positions = [
            [x0, half_height + 0.5, z],
            [x1, half_height + 0.5, z],
            [x0, -half_height + 0.5, z],
            [x1, -half_height + 0.5, z],
        ];
        let uvs = [[u0, 0.0], [u1, 0.0], [u0, 1.0], [u1, 1.0]];
        out.push(PageWipeDraw {
            use_current,
            vertices: page_quad_vertices(positions, uvs, angle, width, height, fov),
        });
    }
    out
}

pub(crate) fn build_page_wipe_draws(
    wipe: &WipeRenderPlan,
    width: f32,
    height: f32,
) -> Vec<PageWipeDraw> {
    match wipe.wipe_type {
        300 => vec![
            build_page_300_draw(wipe, width, height, false, true),
            build_page_300_draw(wipe, width, height, true, false),
        ],
        301 => {
            let front_stage_first = wipe.progress < 0.5;
            let first_current = front_stage_first;
            let mut out = build_page_301_draws_for_stage(wipe, width, height, first_current, true);
            out.extend(build_page_301_draws_for_stage(
                wipe,
                width,
                height,
                !first_current,
                false,
            ));
            out
        }
        _ => Vec::new(),
    }
}

/// The wipe compositor's parameters (`WIPE_SHADER`'s uniform) for `wipe`
/// on a `logical_w` x `logical_h` screen.
pub(crate) fn wipe_uniform(wipe: &WipeRenderPlan, logical_w: f32, logical_h: f32) -> WipeUniform {
    let mut values = [0.0f32; 16];
    for (dst, src) in values.iter_mut().zip(wipe.option.iter().copied()) {
        *dst = src as f32;
    }
    // Types 242/243 choose their random explosion center at wipe start.
    // The seed is stable for the wipe lifetime but changes on the next WIPE.
    values[15] = wipe.random_seed as f32;
    WipeUniform {
        kind_progress: [
            wipe.wipe_type as f32,
            wipe.progress.clamp(0.0, 1.0),
            logical_w,
            logical_h,
        ],
        option0: values[0..4].try_into().expect("four wipe options"),
        option1: values[4..8].try_into().expect("four wipe options"),
        option2: values[8..12].try_into().expect("four wipe options"),
        option3: values[12..16].try_into().expect("four wipe options"),
    }
}

/// The mask a mask wipe without its own image generates (grey in RGBA),
/// or None when its type needs none.
pub(crate) fn generated_wipe_mask(
    wipe: &WipeRenderPlan,
    width: u32,
    height: u32,
) -> Option<crate::assets::RgbaImage> {
    let gray = crate::runtime::wipe_mask::generate(
        wipe.wipe_type,
        &wipe.option,
        width,
        height,
        wipe.random_seed,
    )?;
    let mut rgba = Vec::with_capacity(gray.pixels.len() * 4);
    for value in gray.pixels {
        rgba.extend_from_slice(&[value, value, value, 255]);
    }
    Some(crate::assets::RgbaImage {
        width: gray.width,
        height: gray.height,
        center_x: 0,
        center_y: 0,
        rgba,
    })
}
