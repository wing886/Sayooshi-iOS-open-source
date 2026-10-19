//! E-mote object composition, shared by the renderers: the sprites of an
//! E-mote scene become draws (vertices, texture, native blend mode and the
//! stencil masks that clip them) into the object's render target.

use std::collections::{HashMap, HashSet};

use bytemuck::{Pod, Zeroable};
use eluna::{EmoteDrawFrameInfo, EmoteDrawPass, EmoteStaticScene, EmoteStaticSprite};

use crate::emote::EmoteRenderPacket;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct EmoteVertex {
    pub(crate) clip_position: [f32; 2],
    pub(crate) model_position: [f32; 2],
    pub(crate) texcoord: [f32; 2],
    pub(crate) color: [f32; 4],
    pub(crate) blend_mode: f32,
    pub(crate) clip_rect: [f32; 4],
    pub(crate) wipe: [f32; 3],
}

/// Where a draw's texture comes from.
#[derive(Debug, Clone, Copy)]
pub(crate) enum EmoteTexture {
    Resource(u32),
    Feedback,
}

/// One sprite of an E-mote scene.
#[derive(Debug)]
pub(crate) struct EmoteDraw {
    pub(crate) texture: EmoteTexture,
    pub(crate) vertices: Vec<EmoteVertex>,
    /// Native blend mode 0..=5 (see `native_blend_index`).
    pub(crate) blend_index: usize,
    /// Masks drawn into the stencil before the sprite; empty for none.
    pub(crate) stencil_groups: Vec<EmoteStencilGroup>,
    pub(crate) stencil_initial_reference: u32,
    pub(crate) stencil_final_reference: u32,
}

/// Masks of one stencil phase: 1 increments inside, 2 decrements.
#[derive(Debug)]
pub(crate) struct EmoteStencilGroup {
    pub(crate) phase: u32,
    pub(crate) sources: Vec<EmoteStencilSource>,
}

#[derive(Debug)]
pub(crate) struct EmoteStencilSource {
    pub(crate) texture: EmoteTexture,
    pub(crate) vertices: Vec<EmoteVertex>,
}

pub(crate) fn native_blend_index(blend_mode: u32) -> usize {
    match blend_mode & 0xFF0F {
        0..=5 => (blend_mode & 0xFF0F) as usize,
        _ => 0,
    }
}

/// The draws that render `packet` into its object target, in order.
pub(crate) fn plan_emote_draws(packet: &EmoteRenderPacket) -> Vec<EmoteDraw> {
    let scene = packet.scene.as_ref();
    let visible: Vec<&EmoteStaticSprite> = scene
        .sprites
        .iter()
        .filter(|sprite| sprite.visible && sprite.opacity > 0.0)
        .collect();
    let mut key_to_sprite = HashMap::<Vec<u64>, &EmoteStaticSprite>::new();
    for sprite in &visible {
        key_to_sprite.insert(sprite.draw_frame_info.native_draw_key.clone(), *sprite);
    }
    let layer_infos: HashMap<Vec<u64>, &EmoteDrawFrameInfo> = scene
        .layer_states
        .iter()
        .map(|state| {
            (
                state.draw_frame_info.native_draw_key.clone(),
                &state.draw_frame_info,
            )
        })
        .collect();

    let mut draws = Vec::new();
    for sprite in visible {
        if matches!(
            sprite.draw_frame_info.pass,
            EmoteDrawPass::MaskGeneration | EmoteDrawPass::StencilCompositeMask
        ) {
            continue;
        }
        if !sprite.feedback_history && !packet.textures.contains_key(&sprite.texture_resource_index)
        {
            continue;
        }
        let (stencil_groups, initial_reference, final_reference) =
            stencil_groups_for_sprite(packet, scene, &key_to_sprite, &layer_infos, sprite);
        let vertices = sprite_vertices(packet, sprite);
        if vertices.is_empty() {
            continue;
        }
        draws.push(EmoteDraw {
            texture: if sprite.feedback_history {
                EmoteTexture::Feedback
            } else {
                EmoteTexture::Resource(sprite.texture_resource_index)
            },
            vertices,
            blend_index: native_blend_index(sprite.blend_mode),
            stencil_groups,
            stencil_initial_reference: initial_reference,
            stencil_final_reference: final_reference,
        });
    }
    draws
}

fn stencil_groups_for_sprite(
    packet: &EmoteRenderPacket,
    scene: &EmoteStaticScene,
    key_to_sprite: &HashMap<Vec<u64>, &EmoteStaticSprite>,
    layer_infos: &HashMap<Vec<u64>, &EmoteDrawFrameInfo>,
    sprite: &EmoteStaticSprite,
) -> (Vec<EmoteStencilGroup>, u32, u32) {
    let mut chain = Vec::<&EmoteDrawFrameInfo>::new();
    let mut cursor = sprite.draw_frame_info.stencil_parent_native_key.as_ref();
    let mut visited = HashSet::<Vec<u64>>::new();
    while let Some(key) = cursor {
        if !visited.insert(key.clone()) {
            break;
        }
        let Some(info) = layer_infos.get(key).copied() else {
            break;
        };
        if matches!(info.stencil_phase, 1 | 2) {
            chain.push(info);
        }
        cursor = info.stencil_parent_native_key.as_ref();
    }
    if chain.is_empty() {
        return (Vec::new(), 0, 0);
    }
    let initial_reference: u32 = if chain.iter().any(|info| info.stencil_phase == 2) {
        1
    } else {
        0
    };
    let mut final_reference = initial_reference;
    let mut groups = Vec::new();
    for phase in [1i64, 2i64] {
        for info in chain
            .iter()
            .copied()
            .filter(|info| info.stencil_phase == phase)
        {
            let source_keys = if (info.stencil_type & 4) != 0 {
                scene
                    .composite_mask_sources_by_key
                    .get(&info.native_draw_key)
                    .cloned()
                    .unwrap_or_default()
            } else {
                vec![info.native_draw_key.clone()]
            };
            let mut sources = Vec::new();
            for key in source_keys {
                let Some(source) = key_to_sprite.get(&key).copied() else {
                    continue;
                };
                if !source.feedback_history
                    && !packet.textures.contains_key(&source.texture_resource_index)
                {
                    continue;
                }
                let vertices = sprite_vertices(packet, source);
                if vertices.is_empty() {
                    continue;
                }
                sources.push(EmoteStencilSource {
                    texture: if source.feedback_history {
                        EmoteTexture::Feedback
                    } else {
                        EmoteTexture::Resource(source.texture_resource_index)
                    },
                    vertices,
                });
            }
            groups.push(EmoteStencilGroup {
                phase: phase as u32,
                sources,
            });
            if phase == 1 {
                final_reference = final_reference.saturating_add(1).min(255);
            }
        }
    }
    (groups, initial_reference, final_reference)
}

fn sprite_vertices(packet: &EmoteRenderPacket, sprite: &EmoteStaticSprite) -> Vec<EmoteVertex> {
    if let Some(mesh) = &sprite.mesh {
        return mesh_sprite_vertices(packet, sprite, mesh);
    }
    let left = sprite.left();
    let right = sprite.right();
    let top = sprite.top();
    let bottom = sprite.bottom();
    let tl = native_corner_color(sprite, sprite.corner_colors[0]);
    let tr = native_corner_color(sprite, sprite.corner_colors[1]);
    let bl = native_corner_color(sprite, sprite.corner_colors[2]);
    let br = native_corner_color(sprite, sprite.corner_colors[3]);
    let make = |position: [f32; 2], texcoord: [f32; 2], color: [f32; 4]| {
        make_vertex(
            packet,
            sprite,
            transform_sprite_point(sprite, position),
            texcoord,
            color,
        )
    };
    vec![
        make([left, top], [sprite.uv_left, sprite.uv_top], tl),
        make([left, bottom], [sprite.uv_left, sprite.uv_bottom], bl),
        make([right, top], [sprite.uv_right, sprite.uv_top], tr),
        make([right, top], [sprite.uv_right, sprite.uv_top], tr),
        make([left, bottom], [sprite.uv_left, sprite.uv_bottom], bl),
        make([right, bottom], [sprite.uv_right, sprite.uv_bottom], br),
    ]
}

fn mesh_sprite_vertices(
    packet: &EmoteRenderPacket,
    sprite: &EmoteStaticSprite,
    mesh: &eluna::EmoteMeshPatch,
) -> Vec<EmoteVertex> {
    let division_x = mesh.division_x.max(1) as usize;
    let division_y = mesh.division_y.max(1) as usize;
    let left = sprite.left();
    let top = sprite.top();
    let corner_colors = [
        native_corner_color(sprite, sprite.corner_colors[0]),
        native_corner_color(sprite, sprite.corner_colors[1]),
        native_corner_color(sprite, sprite.corner_colors[2]),
        native_corner_color(sprite, sprite.corner_colors[3]),
    ];
    let vertex_at = |ix: usize, iy: usize| {
        let u = ix as f32 / division_x as f32;
        let v = iy as f32 / division_y as f32;
        let p = mesh.sample(u, v);
        let position = [left + p[0] * sprite.width, top + p[1] * sprite.height];
        make_vertex(
            packet,
            sprite,
            transform_sprite_point(sprite, position),
            [
                sprite.uv_left + (sprite.uv_right - sprite.uv_left) * u,
                sprite.uv_top + (sprite.uv_bottom - sprite.uv_top) * v,
            ],
            bilerp_color(corner_colors, u, v),
        )
    };
    let mut vertices = Vec::with_capacity(division_x * division_y * 6);
    for y in 0..division_y {
        for x in 0..division_x {
            let tl = vertex_at(x, y);
            let bl = vertex_at(x, y + 1);
            let tr = vertex_at(x + 1, y);
            let br = vertex_at(x + 1, y + 1);
            vertices.extend_from_slice(&[tl, bl, tr, tr, bl, br]);
        }
    }
    vertices
}

fn make_vertex(
    packet: &EmoteRenderPacket,
    sprite: &EmoteStaticSprite,
    model_position: [f32; 2],
    texcoord: [f32; 2],
    color: [f32; 4],
) -> EmoteVertex {
    // Original Siglus D3D9 Emote render target transform:
    // world translation (-rep_x,+rep_y), half-pixel-adjusted orthographic projection.
    let clip_x = 2.0 * (model_position[0] - packet.rep_x - 0.5) / packet.width.max(1) as f32;
    let clip_y = 2.0 * (0.5 - (model_position[1] + packet.rep_y)) / packet.height.max(1) as f32;
    EmoteVertex {
        clip_position: [clip_x, clip_y],
        model_position,
        texcoord,
        color,
        blend_mode: sprite.blend_mode as f32,
        clip_rect: sprite
            .draw_frame_info
            .clip_rect
            .unwrap_or([-1.0e30, -1.0e30, 1.0e30, 1.0e30]),
        wipe: [
            sprite.draw_frame_info.stencil_wipe_scale,
            sprite.draw_frame_info.stencil_wipe_bias,
            if sprite.draw_frame_info.stencil_wipe_enabled {
                1.0
            } else {
                0.0
            },
        ],
    }
}

fn native_corner_color(sprite: &EmoteStaticSprite, packed: u32) -> [f32; 4] {
    let r = ((packed >> 24) & 0xff) as f32 / 255.0;
    let g = ((packed >> 16) & 0xff) as f32 / 255.0;
    let b = ((packed >> 8) & 0xff) as f32 / 255.0;
    let a = (packed & 0xff) as f32 / 255.0;
    [r, g, b, (a * sprite.opacity).clamp(0.0, 1.0)]
}

fn bilerp_color(corners: [[f32; 4]; 4], u: f32, v: f32) -> [f32; 4] {
    let mut out = [0.0; 4];
    for i in 0..4 {
        let top = corners[0][i] + (corners[1][i] - corners[0][i]) * u;
        let bottom = corners[2][i] + (corners[3][i] - corners[2][i]) * u;
        out[i] = top + (bottom - top) * v;
    }
    out
}

fn transform_sprite_point(sprite: &EmoteStaticSprite, point: [f32; 2]) -> [f32; 2] {
    let sx = if sprite.scale_x.is_finite() {
        sprite.scale_x
    } else {
        1.0
    };
    let sy = if sprite.scale_y.is_finite() {
        sprite.scale_y
    } else {
        1.0
    };
    let angle = if sprite.rotation_degrees.is_finite() {
        sprite.rotation_degrees.to_radians()
    } else {
        0.0
    };
    let (sin, cos) = angle.sin_cos();
    let dx = (point[0] - sprite.center_x) * sx;
    let dy = (point[1] - sprite.center_y) * sy;
    let local = [
        sprite.center_x + dx * cos - dy * sin,
        sprite.center_y + dx * sin + dy * cos,
    ];
    let m = sprite.world_transform;
    [
        m[0] * local[0] + m[1] * local[1] + m[4],
        m[2] * local[0] + m[3] * local[1] + m[5],
    ]
}
