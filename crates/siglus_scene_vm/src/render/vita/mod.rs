//! PS Vita renderer: the desktop renderer's frame plan (`crate::render_plan`)
//! drawn with GXM, with the desktop's WGSL shaders ported to Cg
//! (`shaders/`). The passes follow `render/mod.rs`: ordinary frames go
//! straight to the display; wipes and overlay blending go through
//! render targets (scene A/B, wipe A/B), E-mote objects through their own
//! targets with stencil masks. Nothing is composited on the CPU.
//!
//! The desktop draws its render targets at the logical screen size; here
//! they have the on-screen size (the logical screen fitted into 960x544),
//! and the shaders receive the logical sizes they compute with.

mod gpu;
pub mod vita_stats;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use crate::assets::RgbaImage;
use crate::emote::EmoteRenderPacket;
use crate::image_manager::{ImageHandle, ImageManager};
use crate::layer::{RenderFrame, RenderSprite, SpriteBlend, WipeRenderPlan};
use crate::render_plan::emote::{EmoteTexture, plan_emote_draws};
use std::ffi::CStr;
use crate::render_plan::{
    BoneUniform, DrawCommand, EffectProgram, FramePlanner, PageWipeVertex, PlanTarget,
    SurfaceViewport, TechniqueSpecial, VertexSprite2dData, WipeMaskCacheKey,
    build_page_wipe_draws, generated_wipe_mask, wipe_uniform,
};
use crate::runtime::FrameCaptureBackend;

use gpu::{
    Blend, Bound, Cull, Depth, Draw, Gpu, GpuSlice, Program, Stencil, Texture, Viewport,
    WipeVertex,
};

const DISPLAY_W: u32 = 960;
const DISPLAY_H: u32 = 544;
/// Images above 720p are kept at half size: the 1080p screen is drawn at
/// 960x540, so their full size is never seen (and would not fit in video
/// memory). Tone curves, which are look-up tables, stay exact.
const REDUCE_ABOVE_PIXELS: u64 = 1280 * 720;

#[cfg(target_os = "vita")]
unsafe extern "C" {
    fn siglus_vita_log_marker(message: *const u8);
}

fn report_marker(message: &[u8]) {
    #[cfg(target_os = "vita")]
    unsafe {
        siglus_vita_log_marker(message.as_ptr())
    };
}

/// A texture in the cache.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TexKey {
    /// An image, at its size or reduced (`REDUCE_ABOVE_PIXELS`).
    Image(u32, bool),
    /// A mesh texture file.
    External(PathBuf),
}

/// The render targets frames that sample the scene go through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tgt {
    SceneA = 0,
    SceneB = 1,
    WipeA = 2,
    WipeB = 3,
}

impl Tgt {
    fn opposite(self) -> Tgt {
        match self {
            Tgt::SceneA => Tgt::SceneB,
            _ => Tgt::SceneA,
        }
    }
}

/// Where draws land: the target's size and the logical screen in it.
#[derive(Debug, Clone, Copy)]
struct TargetGeom {
    width: u32,
    height: u32,
    viewport: Viewport,
}

/// An E-mote object's render targets: `output` is the latest composition,
/// `feedback` the one before (sprites may draw the previous frame).
struct EmoteTarget {
    output: Texture,
    feedback: Texture,
    feedback_valid: bool,
    resources: HashMap<u32, Texture>,
    version: u64,
    hit_version: Option<u64>,
}

pub struct Renderer {
    gpu: Gpu,
    logical_w: u32,
    logical_h: u32,
    /// The logical screen on the display (letterboxed).
    screen: Viewport,
    wait_display_vsync: bool,
    plan: FramePlanner,
    sprite_verts: Vec<VertexSprite2dData>,
    textures: HashMap<TexKey, Texture>,
    white: Texture,
    targets: Vec<Texture>,
    wipe_mask: Option<(WipeMaskCacheKey, Texture)>,
    emotes: HashMap<u64, EmoteTarget>,
    /// Where to save the next frame (`dump_next_frame`).
    dump: Option<PathBuf>,
    /// Shadow map (depth as grey) of shadow-casting 3D sprites; made on
    /// first use.
    shadow_map: Option<Texture>,
}

/// A planned list's vertices in this frame's ring: the 2D sprite layout,
/// and the full layout when 3D draws need it.
#[derive(Clone, Copy)]
struct PlanVertices {
    sprite: GpuSlice,
    full: Option<GpuSlice>,
}

/// The shadow map's side (the desktop's is 2048).
const SHADOW_MAP_SIDE: u32 = 1024;

impl Renderer {
    pub fn new(_width: u32, _height: u32) -> Result<Self> {
        let mut gpu = Gpu::new().map_err(|error| anyhow::anyhow!(error))?;
        let white = gpu
            .create_texture(1, 1, &[255; 4], (0, 0))
            .ok_or_else(|| anyhow::anyhow!("GPU texture allocation failed"))?;
        let mut renderer = Self {
            gpu,
            logical_w: DISPLAY_W,
            logical_h: DISPLAY_H,
            screen: Viewport {
                x: 0.0,
                y: 0.0,
                w: DISPLAY_W as f32,
                h: DISPLAY_H as f32,
            },
            wait_display_vsync: true,
            plan: FramePlanner::default(),
            sprite_verts: Vec::new(),
            textures: HashMap::new(),
            white,
            targets: Vec::new(),
            wipe_mask: None,
            emotes: HashMap::new(),
            dump: None,
            shadow_map: None,
        };
        renderer.resize(DISPLAY_W, DISPLAY_H);
        Ok(renderer)
    }

    pub fn adapter_name(&self) -> String {
        "PS Vita GXM renderer".to_owned()
    }

    /// Sets the logical screen size; it is fitted into the display.
    pub fn resize(&mut self, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);
        let scale = (DISPLAY_W as f64 / width as f64).min(DISPLAY_H as f64 / height as f64);
        let draw_w = (width as f64 * scale).round().max(1.0);
        let draw_h = (height as f64 * scale).round().max(1.0);
        let screen = Viewport {
            x: ((DISPLAY_W as f64 - draw_w) / 2.0).floor() as f32,
            y: ((DISPLAY_H as f64 - draw_h) / 2.0).floor() as f32,
            w: draw_w as f32,
            h: draw_h as f32,
        };
        if (width, height) != (self.logical_w, self.logical_h) || screen != self.screen {
            self.logical_w = width;
            self.logical_h = height;
            self.screen = screen;
            for target in self.targets.drain(..) {
                self.gpu.retire(target);
            }
            if let Some((_, mask)) = self.wipe_mask.take() {
                self.gpu.retire(mask);
            }
        }
    }

    pub fn resize_with_scale(&mut self, width: u32, height: u32, _scale_factor: f32) {
        self.resize(width, height);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn resize_with_logical_viewport(
        &mut self,
        _surface_width: u32,
        _surface_height: u32,
        _scale_factor: f32,
        logical_width: u32,
        logical_height: u32,
        _viewport_x: u32,
        _viewport_y: u32,
        _viewport_width: u32,
        _viewport_height: u32,
    ) {
        self.resize(logical_width, logical_height);
    }

    pub fn set_wait_display_vsync(&mut self, wait_display_vsync: bool) {
        self.wait_display_vsync = wait_display_vsync;
    }

    pub fn clear_runtime_image_textures(&mut self) {
        for (_, texture) in self.textures.drain() {
            self.gpu.retire(texture);
        }
    }

    pub fn logical_size(&self) -> (u32, u32) {
        (self.logical_w, self.logical_h)
    }

    /// Saves the next rendered frame as a PNG at `path` (diagnosis).
    pub fn dump_next_frame(&mut self, path: PathBuf) {
        self.dump = Some(path);
    }

    pub fn render_frame(&mut self, images: &ImageManager, frame: &RenderFrame) -> Result<()> {
        let start = crate::platform_time::Instant::now();
        self.gpu.begin_frame();
        self.prepare_emotes(frame);
        // As on the desktop: a frame goes through render targets only when
        // something samples the drawn scene.
        let needs_scene_texture = frame.wipe.is_some()
            || frame
                .sprites
                .iter()
                .any(|entry| matches!(entry.sprite.blend, SpriteBlend::Overlay));
        if needs_scene_texture {
            vita_stats::offscreen_frame();
            let final_target = self.render_frame_to_targets(images, frame)?;
            if let Some(path) = self.dump.as_ref() {
                self.gpu.finish();
                let target = &self.targets[final_target as usize];
                let (data, stride) = target.pixels();
                let (w, h) = (target.width as usize, target.height as usize);
                let mut rgba = vec![0u8; w * h * 4];
                for y in 0..h {
                    let row = unsafe { std::slice::from_raw_parts(data.add(y * stride), w * 4) };
                    rgba[y * w * 4..][..w * 4].copy_from_slice(row);
                }
                let _ = image::save_buffer(
                    path.with_extension("target.png"),
                    &rgba,
                    w as u32,
                    h as u32,
                    image::ColorType::Rgba8,
                );
            }
            self.gpu.begin_scene(None);
            let display = self.display_geom();
            self.clear(display, [0.0, 0.0, 0.0, 1.0]);
            let source = self.targets[final_target as usize].linear();
            self.copy(source, display.viewport, Blend::Replace);
        } else {
            self.render_to_display(images, &frame.sprites)?;
        }
        let dump = self.dump.take();
        self.gpu.end_frame(self.wait_display_vsync, dump.as_deref());
        vita_stats::record_frame(&mut self.gpu, start);
        Ok(())
    }

    fn display_geom(&self) -> TargetGeom {
        TargetGeom {
            width: DISPLAY_W,
            height: DISPLAY_H,
            viewport: self.screen,
        }
    }

    fn target_geom(&self) -> TargetGeom {
        let width = self.screen.w as u32;
        let height = self.screen.h as u32;
        TargetGeom {
            width,
            height,
            viewport: Viewport {
                x: 0.0,
                y: 0.0,
                w: width as f32,
                h: height as f32,
            },
        }
    }

    fn ensure_targets(&mut self) -> Result<()> {
        if self.targets.len() == 4 {
            return Ok(());
        }
        let geom = self.target_geom();
        while self.targets.len() < 4 {
            let target = self
                .gpu
                .create_target(geom.width, geom.height)
                .ok_or_else(|| anyhow::anyhow!("render target allocation failed"))?;
            self.targets.push(target);
        }
        Ok(())
    }

    /// `render_ordinary_frame_to_surface`: the sprites straight onto the
    /// display.
    fn render_to_display(&mut self, images: &ImageManager, sprites: &[RenderSprite]) -> Result<()> {
        let geom = self.display_geom();
        self.plan_sprites(images, sprites, geom)?;
        let vertices = self.upload_plan_vertices();
        // The desktop clears its direct-path shadow map white.
        if let Some(vertices) = vertices {
            self.render_shadow_pass(images, vertices, [1.0; 4]);
        }
        self.gpu.begin_scene(None);
        self.clear(
            TargetGeom {
                viewport: Viewport {
                    x: 0.0,
                    y: 0.0,
                    w: DISPLAY_W as f32,
                    h: DISPLAY_H as f32,
                },
                ..geom
            },
            [0.0, 0.0, 0.0, 1.0],
        );
        if let Some(vertices) = vertices {
            for index in 0..self.plan.draws.len() {
                self.draw_command(images, index, geom, vertices, None);
            }
        }
        Ok(())
    }

    /// `render_frame_to_internal`: the frame into the render targets; the
    /// target holding the result.
    fn render_frame_to_targets(&mut self, images: &ImageManager, frame: &RenderFrame) -> Result<Tgt> {
        self.ensure_targets()?;
        let Some(wipe) = frame.wipe.as_ref() else {
            return self.render_list(images, &frame.sprites, Tgt::SceneA, false);
        };
        let current = self.render_list(images, &wipe.current, Tgt::WipeA, false)?;
        if current != Tgt::WipeA {
            self.copy_target(current, Tgt::WipeA);
        }
        let next = self.render_list(images, &wipe.next, Tgt::WipeB, false)?;
        if next != Tgt::WipeB {
            self.copy_target(next, Tgt::WipeB);
        }
        let under = self.render_list(images, &wipe.under, Tgt::SceneA, false)?;
        let composed = self.render_wipe_composite(images, wipe, under)?;
        self.render_list(images, &wipe.over, composed, true)
    }

    /// `render_sprite_list_to_scene_pair`: `sprites` into `start` (over its
    /// content when `load`). Overlay sprites read the scene drawn so far
    /// from the other scene target; returns the target holding the result.
    fn render_list(
        &mut self,
        images: &ImageManager,
        sprites: &[RenderSprite],
        start: Tgt,
        load: bool,
    ) -> Result<Tgt> {
        let geom = self.target_geom();
        self.plan_sprites(images, sprites, geom)?;
        let vertices = self.upload_plan_vertices();
        if let Some(vertices) = vertices {
            self.render_shadow_pass(images, vertices, [0.0, 0.0, 0.0, 1.0]);
        }
        let mut current = start;
        self.gpu.begin_scene(Some(&self.targets[current as usize]));
        if !load {
            self.clear(geom, [0.0, 0.0, 0.0, 1.0]);
        }
        let Some(vertices) = vertices else {
            return Ok(current);
        };
        let count = self.plan.draws.len();
        let mut index = 0;
        while index < count {
            let overlay = is_overlay(&self.plan.draws[index]);
            let first = index;
            while index < count && is_overlay(&self.plan.draws[index]) == overlay {
                index += 1;
            }
            if overlay {
                // The scene so far is copied to the other target, which the
                // overlay sprites then draw into while reading this one.
                let backdrop = if matches!(current, Tgt::SceneA | Tgt::SceneB) {
                    current
                } else {
                    self.copy_target(current, Tgt::SceneA);
                    Tgt::SceneA
                };
                let dst = backdrop.opposite();
                self.gpu.begin_scene(Some(&self.targets[dst as usize]));
                let source = self.targets[backdrop as usize].linear();
                self.copy(source, geom.viewport, Blend::Replace);
                for draw in first..index {
                    self.draw_command(images, draw, geom, vertices, Some(backdrop));
                }
                current = dst;
            } else {
                for draw in first..index {
                    self.draw_command(images, draw, geom, vertices, None);
                }
            }
        }
        self.gpu.end_scene();
        Ok(current)
    }

    /// Plans `sprites` for a target and uploads their textures.
    fn plan_sprites(
        &mut self,
        images: &ImageManager,
        sprites: &[RenderSprite],
        geom: TargetGeom,
    ) -> Result<()> {
        self.plan.plan(
            images,
            sprites,
            &PlanTarget {
                win_w: self.logical_w as f32,
                win_h: self.logical_h as f32,
                surface_w: geom.width,
                surface_h: geom.height,
                surface_viewport: SurfaceViewport {
                    x: geom.viewport.x as u32,
                    y: geom.viewport.y as u32,
                    w: geom.viewport.w as u32,
                    h: geom.viewport.h as u32,
                },
            },
        )?;
        // Textures are uploaded before any draw of the frame reads them.
        for index in 0..self.plan.draws.len() {
            let draw = &self.plan.draws[index];
            let handles = [
                (draw.image_id.clone(), true),
                (draw.mask_image_id.clone(), true),
                (draw.tonecurve_image_id.clone(), false),
                (draw.fog_image_id.clone(), true),
                (draw.wipe_src_image_id.clone(), true),
            ];
            let paths = [
                draw.mesh_texture_path.clone(),
                draw.mesh_normal_texture_path.clone(),
                draw.mesh_toon_texture_path.clone(),
            ];
            for (handle, reduce) in handles {
                if let Some(handle) = handle {
                    self.image_texture(images, &handle, reduce);
                }
            }
            for path in paths.into_iter().flatten() {
                self.external_texture(&path);
            }
        }
        Ok(())
    }

    fn upload_plan_vertices(&mut self) -> Option<PlanVertices> {
        self.sprite_verts.clear();
        self.sprite_verts
            .extend(self.plan.verts.iter().map(|v| VertexSprite2dData::from(*v)));
        if self.sprite_verts.is_empty() {
            return None;
        }
        let Some(sprite) = self.gpu.upload(&self.sprite_verts) else {
            gpu::log_once("frame vertex ring full; sprites skipped");
            return None;
        };
        let needs_full = self
            .plan
            .draws
            .iter()
            .any(|cmd| cmd.shadow_cast || !cmd.pipeline_key.program.uses_sprite2d_layout());
        let full = if needs_full {
            let full = self.gpu.upload(&self.plan.verts);
            if full.is_none() {
                gpu::log_once("frame vertex ring full; 3D draws skipped");
            }
            full
        } else {
            None
        };
        Some(PlanVertices { sprite, full })
    }

    /// `render_command_slice` into the shadow map for the shadow casters.
    fn render_shadow_pass(&mut self, images: &ImageManager, vertices: PlanVertices, clear: [f32; 4]) {
        let casters: Vec<usize> = (0..self.plan.draws.len())
            .filter(|&i| self.plan.draws[i].shadow_cast)
            .collect();
        if casters.is_empty() {
            return;
        }
        if self.shadow_map.is_none() {
            self.shadow_map = self.gpu.create_target(SHADOW_MAP_SIDE, SHADOW_MAP_SIDE);
        }
        let Some(shadow) = self.shadow_map.as_ref() else {
            gpu::log_once("shadow map allocation failed");
            return;
        };
        let geom = TargetGeom {
            width: SHADOW_MAP_SIDE,
            height: SHADOW_MAP_SIDE,
            viewport: Viewport {
                x: 0.0,
                y: 0.0,
                w: SHADOW_MAP_SIDE as f32,
                h: SHADOW_MAP_SIDE as f32,
            },
        };
        self.gpu.begin_scene(Some(shadow));
        self.clear(geom, clear);
        for index in casters {
            self.draw_mesh(images, index, geom, vertices, true);
        }
        self.gpu.end_scene();
    }

    /// The texture of an image (uploaded or updated as needed), with the
    /// image's own size.
    fn image_texture(
        &mut self,
        images: &ImageManager,
        handle: &ImageHandle,
        reduce: bool,
    ) -> Option<(Bound, [f32; 2])> {
        let (image, version) = images.get_entry(handle)?;
        let size = [image.width as f32, image.height as f32];
        let pixels = u64::from(image.width) * u64::from(image.height);
        let reduce = (reduce && pixels > REDUCE_ABOVE_PIXELS)
            || image.width > gpu::MAX_TEXTURE_SIDE
            || image.height > gpu::MAX_TEXTURE_SIDE;
        let key = TexKey::Image(handle.key().0, reduce);
        let source = (Arc::as_ptr(&image) as usize, version);
        let frame = self.gpu.frame;
        if let Some(texture) = self.textures.get_mut(&key)
            && texture.source == source
        {
            texture.last_frame = frame;
            return Some((texture.linear(), size));
        }
        let reduced;
        let upload: &RgbaImage = if reduce {
            reduced = half_size(&image);
            &reduced
        } else {
            &image
        };
        let texture = self.store_texture(key, upload, source)?;
        Some((texture.linear(), size))
    }

    fn external_texture(&mut self, path: &PathBuf) -> Option<Bound> {
        let key = TexKey::External(path.clone());
        let frame = self.gpu.frame;
        if let Some(texture) = self.textures.get_mut(&key) {
            texture.last_frame = frame;
            return Some(texture.linear());
        }
        let image = crate::assets::load_image_any(path, 0).ok()?;
        let image = if u64::from(image.width) * u64::from(image.height) > REDUCE_ABOVE_PIXELS {
            half_size(&image)
        } else {
            image
        };
        Some(self.store_texture(key, &image, (0, 0))?.linear())
    }

    /// Puts `image` in the cache under `key`: rewritten in place when a
    /// texture of its size is not drawn yet this frame, else a new one.
    fn store_texture(
        &mut self,
        key: TexKey,
        image: &RgbaImage,
        source: (usize, u64),
    ) -> Option<&Texture> {
        let frame = self.gpu.frame;
        if let Some(texture) = self.textures.get_mut(&key)
            && texture.width == image.width
            && texture.height == image.height
            && texture.last_frame != frame
        {
            texture.source = source;
            texture.last_frame = frame;
            let texture = self.textures.remove(&key)?;
            self.gpu.write_texture(&texture, &image.rgba);
            self.textures.insert(key.clone(), texture);
            return self.textures.get(&key);
        }
        if let Some(old) = self.textures.remove(&key) {
            self.gpu.retire(old);
        }
        let bytes = image.width as usize * image.height as usize * 4;
        while self.gpu.texture_bytes + bytes > gpu::TEXTURE_BUDGET {
            let Some(oldest) = self
                .textures
                .iter()
                .filter(|(_, t)| t.last_frame != frame)
                .min_by_key(|(_, t)| t.last_frame)
                .map(|(k, _)| k.clone())
            else {
                break;
            };
            if let Some(old) = self.textures.remove(&oldest) {
                self.gpu.retire(old);
            }
        }
        let texture = self
            .gpu
            .create_texture(image.width, image.height, &image.rgba, source)?;
        self.textures.insert(key.clone(), texture);
        self.textures.get(&key)
    }

    fn cached_image(&self, handle: &ImageHandle) -> Option<(&Texture, bool)> {
        [false, true].into_iter().find_map(|reduced| {
            self.textures
                .get(&TexKey::Image(handle.key().0, reduced))
                .map(|texture| (texture, reduced))
        })
    }

    /// One draw command of the plan.
    fn draw_command(
        &mut self,
        images: &ImageManager,
        index: usize,
        geom: TargetGeom,
        vertices: PlanVertices,
        backdrop: Option<Tgt>,
    ) {
        let cmd = &self.plan.draws[index];
        if !cmd.pipeline_key.program.uses_sprite2d_layout() {
            self.draw_mesh(images, index, geom, vertices, false);
            return;
        }
        let white = self.white.linear();
        let image_size = |handle: &Option<ImageHandle>| -> [f32; 2] {
            handle
                .as_ref()
                .and_then(|h| images.get(h))
                .map_or([1.0, 1.0], |image| [image.width as f32, image.height as f32])
        };
        let base = if let Some(render_id) = cmd.emote_render_id {
            self.emotes
                .get(&render_id)
                .map(|target| target.output.linear())
        } else {
            cmd.image_id
                .as_ref()
                .and_then(|h| self.cached_image(h))
                .map(|(t, _)| t.linear())
        };
        let Some(base) = base else {
            return;
        };
        let base_size = if let Some(render_id) = cmd.emote_render_id {
            self.emotes
                .get(&render_id)
                .map_or([1.0, 1.0], |t| [t.output.width as f32, t.output.height as f32])
        } else {
            image_size(&cmd.image_id)
        };
        let lookup = |handle: &Option<ImageHandle>, reduced: bool| -> Option<Bound> {
            let handle = handle.as_ref()?;
            [reduced, !reduced].into_iter().find_map(|r| {
                self.textures
                    .get(&TexKey::Image(handle.key().0, r))
                    .map(Texture::linear)
            })
        };
        let mask = lookup(&cmd.mask_image_id, true).unwrap_or(white);
        let tone = lookup(&cmd.tonecurve_image_id, false).unwrap_or(white);
        let fog = lookup(&cmd.fog_image_id, true).unwrap_or(white);
        let (aux, aux_size) = if matches!(
            cmd.pipeline_key.technique.special,
            TechniqueSpecial::Overlay
        ) {
            match backdrop {
                Some(target) => (
                    self.targets[target as usize].linear(),
                    [geom.width as f32, geom.height as f32],
                ),
                None => (white, [1.0, 1.0]),
            }
        } else if cmd.wipe_src_image_id.is_some() {
            (
                lookup(&cmd.wipe_src_image_id, true).unwrap_or(white),
                image_size(&cmd.wipe_src_image_id),
            )
        } else {
            (white, [1.0, 1.0])
        };
        let fog_size = image_size(&cmd.fog_image_id);

        let u = &cmd.vs_uniform;
        let effects = u.sprite_effects;
        let scissor = cmd.scissor.map_or(
            [
                geom.viewport.x,
                geom.viewport.y,
                geom.viewport.x + geom.viewport.w,
                geom.viewport.y + geom.viewport.h,
            ],
            |s| [s.x as f32, s.y as f32, (s.x + s.w) as f32, (s.y + s.h) as f32],
        );
        let blend = if cmd.pipeline_key.alpha_blend {
            Blend::sprite(cmd.pipeline_key.blend)
        } else {
            Blend::Replace
        };
        let depth = if cmd.pipeline_key.use_depth {
            Depth {
                test: true,
                write: cmd.pipeline_key.depth_write,
            }
        } else {
            Depth::OFF
        };
        let cull = if cmd.pipeline_key.cull_back {
            Cull::CounterClockwise
        } else {
            Cull::None
        };
        let plain = is_plain(cmd);
        let mut fu = [0.0f32; 72];
        let fragment = if plain {
            fu[0] = effects[0][0];
            fu[1] = effects[6][2];
            fu[4..8].copy_from_slice(&scissor);
            Program::PlainF
        } else {
            for (slot, value) in fu.chunks_exact_mut(4).zip(effects.iter()) {
                slot.copy_from_slice(value);
            }
            fu[44..48].copy_from_slice(&[base_size[0], base_size[1], aux_size[0], aux_size[1]]);
            fu[48..52].copy_from_slice(&[
                geom.viewport.x,
                geom.viewport.y,
                self.logical_w as f32 / geom.viewport.w,
                self.logical_h as f32 / geom.viewport.h,
            ]);
            fu[52..56].copy_from_slice(&u.camera_eye);
            fu[56..60].copy_from_slice(&u.single_light_pos_kind);
            fu[60..64].copy_from_slice(&u.camera_params);
            fu[64..68].copy_from_slice(&scissor);
            fu[68..72].copy_from_slice(&[
                1.0 / geom.width as f32,
                1.0 / geom.height as f32,
                fog_size[0],
                fog_size[1],
            ]);
            Program::SpriteF
        };
        let uniforms: &[f32] = if plain { &fu[..8] } else { &fu };
        let range = cmd.range.clone();
        let textures = [Some(base), Some(mask), Some(tone), Some(aux), Some(fog)];
        self.gpu.draw(&Draw {
            vertex: Program::SpriteV,
            fragment,
            blend,
            depth,
            stencil: Stencil::Off,
            cull,
            viewport: geom.viewport,
            textures: &textures,
            vertices: vertices
                .sprite
                .offset(range.start as usize * size_of::<VertexSprite2dData>()),
            vertex_count: range.end - range.start,
            vertex_uniforms: &[],
            fragment_uniforms: &[(c"fu", uniforms)],
            vertex_buffer: None,
        });
    }

    /// A 3D draw (mesh batch, or a shadow caster in the shadow pass):
    /// `vs_common`/`fs_common` or the shadow programs.
    fn draw_mesh(
        &mut self,
        images: &ImageManager,
        index: usize,
        geom: TargetGeom,
        vertices: PlanVertices,
        shadow_pass: bool,
    ) {
        let Some(full) = vertices.full else {
            return;
        };
        let cmd = &self.plan.draws[index];
        let white = self.white.linear();
        let image = |handle: &Option<ImageHandle>| -> (Option<Bound>, [f32; 2]) {
            let Some(handle) = handle.as_ref() else {
                return (None, [1.0, 1.0]);
            };
            let bound = [true, false].into_iter().find_map(|reduced| {
                self.textures
                    .get(&TexKey::Image(handle.key().0, reduced))
                    .map(Texture::linear)
            });
            let size = images
                .get(handle)
                .map_or([1.0, 1.0], |i| [i.width as f32, i.height as f32]);
            (bound, size)
        };
        let external = |path: &Option<PathBuf>| -> Option<(Bound, [f32; 2])> {
            let texture = self.textures.get(&TexKey::External(path.clone()?))?;
            Some((texture.linear(), [texture.width as f32, texture.height as f32]))
        };
        let (base, base_size) = match external(&cmd.mesh_texture_path) {
            Some((bound, size)) => (Some(bound), size),
            None => image(&cmd.image_id),
        };
        let base = base.unwrap_or(white);
        let normal = external(&cmd.mesh_normal_texture_path);
        let toon = external(&cmd.mesh_toon_texture_path);
        let (mask, _) = image(&cmd.mask_image_id);
        let tone = [false, true].into_iter().find_map(|reduced| {
            cmd.tonecurve_image_id.as_ref().and_then(|h| {
                self.textures
                    .get(&TexKey::Image(h.key().0, reduced))
                    .map(Texture::linear)
            })
        });
        let (fog, fog_size) = image(&cmd.fog_image_id);
        let (aux, aux_size) = image(&cmd.wipe_src_image_id);
        let shadow = if shadow_pass {
            None
        } else {
            self.shadow_map.as_ref().map(Texture::point)
        };
        let textures = [
            Some(base),
            Some(mask.unwrap_or(white)),
            Some(tone.unwrap_or(white)),
            Some(aux.unwrap_or(white)),
            Some(fog.unwrap_or(white)),
            Some(normal.map_or(white, |(b, _)| b)),
            Some(toon.map_or(white, |(b, _)| b)),
            Some(shadow.unwrap_or(white)),
        ];
        let normal_size = normal.map_or([1.0, 1.0], |(_, s)| s);
        let toon_size = toon.map_or([1.0, 1.0], |(_, s)| s);
        let mx: [f32; 12] = [
            normal_size[0],
            normal_size[1],
            toon_size[0],
            toon_size[1],
            fog_size[0],
            fog_size[1],
            base_size[0],
            base_size[1],
            aux_size[0],
            aux_size[1],
            0.0,
            0.0,
        ];
        let vs: &[f32] = bytemuck::cast_slice(std::slice::from_ref(&cmd.vs_uniform));
        let bones = match cmd.bone_uniform_index {
            Some(i) => self.plan.draw_bone_uniforms.get(i as usize).copied(),
            None => None,
        }
        .unwrap_or_else(BoneUniform::zero);
        let key = &cmd.pipeline_key;
        let (vertex, fragment, blend, depth) = if shadow_pass {
            (
                Program::ShadowV,
                Program::ShadowF,
                Blend::Replace,
                Depth {
                    test: true,
                    write: true,
                },
            )
        } else {
            (
                Program::MeshV,
                Program::MeshF,
                if key.alpha_blend {
                    Blend::sprite(key.blend)
                } else {
                    Blend::Replace
                },
                if key.use_depth {
                    Depth {
                        test: true,
                        write: key.depth_write,
                    }
                } else {
                    Depth::OFF
                },
            )
        };
        let cull = if key.cull_back {
            Cull::CounterClockwise
        } else {
            Cull::None
        };
        let range = cmd.range.clone();
        let Some(bone_buffer) = self.gpu.upload(std::slice::from_ref(&bones)) else {
            return;
        };
        const VS: &CStr = c"vs";
        const MX: &CStr = c"mx";
        let fragment_uniforms: &[(&CStr, &[f32])] = if shadow_pass {
            &[(VS, vs)]
        } else {
            &[(VS, vs), (MX, &mx)]
        };
        self.gpu.draw(&Draw {
            vertex,
            fragment,
            blend,
            depth,
            stencil: Stencil::Off,
            cull,
            viewport: geom.viewport,
            textures: &textures,
            vertices: full.offset(range.start as usize * size_of::<crate::render_plan::Vertex>()),
            vertex_count: range.end - range.start,
            vertex_uniforms: &[(VS, vs)],
            fragment_uniforms,
            vertex_buffer: Some(bone_buffer),
        });
    }

    /// Fills the whole target with `color`.
    fn clear(&mut self, geom: TargetGeom, color: [f32; 4]) {
        let full = Viewport {
            x: 0.0,
            y: 0.0,
            w: geom.width as f32,
            h: geom.height as f32,
        };
        self.solid(full, color, Blend::Replace, Stencil::Off);
    }

    /// A full-viewport triangle of one colour.
    fn solid(&mut self, viewport: Viewport, color: [f32; 4], blend: Blend, stencil: Stencil) {
        let corners: [[f32; 4]; 3] = [
            [-1.0, -1.0, 0.0, 1.0],
            [3.0, -1.0, 0.0, 1.0],
            [-1.0, 3.0, 0.0, 1.0],
        ];
        let Some(vertices) = self.gpu.upload(&corners) else {
            return;
        };
        self.gpu.draw(&Draw {
            vertex: Program::SolidV,
            fragment: Program::SolidF,
            blend,
            depth: Depth::OFF,
            stencil,
            cull: Cull::None,
            viewport,
            textures: &[],
            vertices,
            vertex_count: 3,
            vertex_uniforms: &[],
            fragment_uniforms: &[(c"color", &color)],
            vertex_buffer: None,
        });
    }

    /// Draws `source` over `viewport` (the copy passes).
    fn copy(&mut self, source: Bound, viewport: Viewport, blend: Blend) {
        let quad = fullscreen_quad();
        let Some(vertices) = self.gpu.upload(&quad) else {
            return;
        };
        let fu = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        self.gpu.draw(&Draw {
            vertex: Program::SpriteV,
            fragment: Program::PlainF,
            blend,
            depth: Depth::OFF,
            stencil: Stencil::Off,
            cull: Cull::None,
            viewport,
            textures: &[Some(source)],
            vertices,
            vertex_count: 6,
            vertex_uniforms: &[],
            fragment_uniforms: &[(c"fu", &fu)],
            vertex_buffer: None,
        });
    }

    /// `copy_internal_target`.
    fn copy_target(&mut self, src: Tgt, dst: Tgt) {
        let geom = self.target_geom();
        self.gpu.begin_scene(Some(&self.targets[dst as usize]));
        let source = self.targets[src as usize].linear();
        self.copy(source, geom.viewport, Blend::Replace);
        self.gpu.end_scene();
    }

    /// `render_wipe_composite`: under + the two scenes into the other scene
    /// target.
    fn render_wipe_composite(
        &mut self,
        images: &ImageManager,
        wipe: &WipeRenderPlan,
        under: Tgt,
    ) -> Result<Tgt> {
        let target = if matches!(under, Tgt::SceneA | Tgt::SceneB) {
            under.opposite()
        } else {
            Tgt::SceneB
        };
        let geom = self.target_geom();
        if matches!(wipe.wipe_type, 300 | 301) {
            // Page wipes: the under scene, then the turning pages over it.
            self.gpu.begin_scene(Some(&self.targets[target as usize]));
            let source = self.targets[under as usize].linear();
            self.copy(source, geom.viewport, Blend::Replace);
            let draws =
                build_page_wipe_draws(wipe, self.logical_w as f32, self.logical_h as f32);
            for draw in draws {
                let Some(vertices) = self.gpu.upload::<PageWipeVertex>(&draw.vertices) else {
                    continue;
                };
                let page = if draw.use_current { Tgt::WipeA } else { Tgt::WipeB };
                let texture = self.targets[page as usize].linear();
                self.gpu.draw(&Draw {
                    vertex: Program::PageV,
                    fragment: Program::PageF,
                    blend: Blend::ALPHA,
                    depth: Depth {
                        test: true,
                        write: true,
                    },
                    stencil: Stencil::Off,
                    cull: Cull::Clockwise,
                    viewport: geom.viewport,
                    textures: &[Some(texture)],
                    vertices,
                    vertex_count: draw.vertices.len() as u32,
                    vertex_uniforms: &[],
                    fragment_uniforms: &[],
                    vertex_buffer: None,
                });
            }
            self.gpu.end_scene();
            return Ok(target);
        }
        let mask = match wipe.mask_image_id.as_ref() {
            Some(id) => self.image_texture(images, id, true).map(|(bound, _)| bound),
            None => self.generated_wipe_mask(wipe),
        }
        .unwrap_or_else(|| self.white.linear());
        let uniform = wipe_uniform(wipe, self.logical_w as f32, self.logical_h as f32);
        let wu: [f32; 20] = bytemuck::cast(uniform);
        let corners = [
            WipeVertex {
                position: [-1.0, -1.0],
                uv: [0.0, 1.0],
            },
            WipeVertex {
                position: [3.0, -1.0],
                uv: [2.0, 1.0],
            },
            WipeVertex {
                position: [-1.0, 3.0],
                uv: [0.0, -1.0],
            },
        ];
        self.gpu.begin_scene(Some(&self.targets[target as usize]));
        let textures = [
            Some(self.targets[under as usize].linear()),
            Some(self.targets[Tgt::WipeA as usize].linear()),
            Some(self.targets[Tgt::WipeB as usize].linear()),
            Some(mask),
        ];
        if let Some(vertices) = self.gpu.upload(&corners) {
            self.gpu.draw(&Draw {
                vertex: Program::WipeV,
                fragment: Program::WipeF,
                blend: Blend::Replace,
                depth: Depth::OFF,
                stencil: Stencil::Off,
                cull: Cull::None,
                viewport: geom.viewport,
                textures: &textures,
                vertices,
                vertex_count: 3,
                vertex_uniforms: &[],
                fragment_uniforms: &[(c"wu", &wu)],
                vertex_buffer: None,
            });
        }
        self.gpu.end_scene();
        Ok(target)
    }

    /// `ensure_generated_wipe_mask`: the mask a mask wipe without an image
    /// uses, generated at the logical size once per wipe.
    fn generated_wipe_mask(&mut self, wipe: &WipeRenderPlan) -> Option<Bound> {
        let key = WipeMaskCacheKey {
            wipe_type: wipe.wipe_type,
            option: wipe.option.clone(),
            width: self.logical_w,
            height: self.logical_h,
            seed: wipe.random_seed,
        };
        if let Some((cached, texture)) = &self.wipe_mask
            && cached == &key
        {
            return Some(texture.linear());
        }
        if let Some((_, old)) = self.wipe_mask.take() {
            self.gpu.retire(old);
        }
        let image = generated_wipe_mask(wipe, self.logical_w, self.logical_h)?;
        let image = if u64::from(image.width) * u64::from(image.height) > REDUCE_ABOVE_PIXELS {
            half_size(&image)
        } else {
            image
        };
        let texture = self
            .gpu
            .create_texture(image.width, image.height, &image.rgba, (0, 0))?;
        let bound = texture.linear();
        self.wipe_mask = Some((key, texture));
        Some(bound)
    }

    /// Renders the frame's E-mote objects whose scene changed into their
    /// targets (`EmoteCompositor::prepare`).
    fn prepare_emotes(&mut self, frame: &RenderFrame) {
        let lists: Vec<&[RenderSprite]> = match frame.wipe.as_ref() {
            Some(wipe) => vec![&wipe.under, &wipe.current, &wipe.next, &wipe.over],
            None => vec![&frame.sprites],
        };
        let packets: Vec<&EmoteRenderPacket> = lists
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.sprite.emote_render.as_deref())
            .collect();
        let live: HashSet<u64> = packets.iter().map(|packet| packet.render_id).collect();
        let stale: Vec<u64> = self
            .emotes
            .keys()
            .filter(|id| !live.contains(id))
            .copied()
            .collect();
        for id in stale {
            if let Some(target) = self.emotes.remove(&id) {
                self.retire_emote(target);
            }
        }
        for packet in packets {
            self.prepare_emote(packet);
        }
    }

    fn retire_emote(&mut self, target: EmoteTarget) {
        self.gpu.retire(target.output);
        self.gpu.retire(target.feedback);
        for (_, texture) in target.resources {
            self.gpu.retire(texture);
        }
    }

    fn prepare_emote(&mut self, packet: &EmoteRenderPacket) {
        let width = packet.width.clamp(1, gpu::MAX_TEXTURE_SIDE);
        let height = packet.height.clamp(1, gpu::MAX_TEXTURE_SIDE);
        let recreate = self
            .emotes
            .get(&packet.render_id)
            .is_none_or(|t| t.output.width != width || t.output.height != height);
        if recreate {
            if let Some(old) = self.emotes.remove(&packet.render_id) {
                self.retire_emote(old);
            }
            let (Some(output), Some(feedback)) = (
                self.gpu.create_target(width, height),
                self.gpu.create_target(width, height),
            ) else {
                gpu::log_once("E-mote render target allocation failed");
                return;
            };
            let mut resources = HashMap::new();
            for (&index, source) in packet.textures.iter() {
                if let Some(texture) =
                    self.gpu
                        .create_texture(source.width, source.height, &source.rgba, (0, 0))
                {
                    resources.insert(index, texture);
                }
            }
            self.emotes.insert(
                packet.render_id,
                EmoteTarget {
                    output,
                    feedback,
                    feedback_valid: false,
                    resources,
                    version: u64::MAX,
                    hit_version: None,
                },
            );
        }
        let Some(target) = self.emotes.get_mut(&packet.render_id) else {
            return;
        };
        let frame = self.gpu.frame;
        target.output.last_frame = frame;
        if target.version != packet.version {
            // The previous composition becomes the feedback texture.
            std::mem::swap(&mut target.output, &mut target.feedback);
            target.feedback_valid = target.version != u64::MAX;
            target.version = packet.version;
            self.draw_emote(packet);
        }
        let Some(target) = self.emotes.get_mut(&packet.render_id) else {
            return;
        };
        if packet.alpha_readback
            && !packet.has_current_hit_surface()
            && target.hit_version != Some(packet.version)
        {
            target.hit_version = Some(packet.version);
            self.gpu.finish();
            let target = &self.emotes[&packet.render_id];
            let (data, stride) = target.output.pixels();
            let (w, h) = (target.output.width as usize, target.output.height as usize);
            let mut alpha = vec![0u8; w * h];
            for y in 0..h {
                for x in 0..w {
                    alpha[y * w + x] = unsafe { *data.add(y * stride + x * 4 + 3) };
                }
            }
            packet.publish_hit_alpha(alpha);
        }
    }

    /// Composes `packet` into its target (`EmoteCompositor::prepare`).
    fn draw_emote(&mut self, packet: &EmoteRenderPacket) {
        let draws = plan_emote_draws(packet);
        let Some(target) = self.emotes.get(&packet.render_id) else {
            return;
        };
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            w: target.output.width as f32,
            h: target.output.height as f32,
        };
        let resolve = |target: &EmoteTarget, texture: EmoteTexture| -> Option<Bound> {
            match texture {
                EmoteTexture::Resource(index) => target.resources.get(&index).map(Texture::point),
                EmoteTexture::Feedback => {
                    target.feedback_valid.then(|| target.feedback.point())
                }
            }
        };
        self.gpu.begin_scene(Some(&target.output));
        self.solid(viewport, [0.0; 4], Blend::Replace, Stencil::Off);
        for draw in &draws {
            let Some(target) = self.emotes.get(&packet.render_id) else {
                return;
            };
            let Some(texture) = resolve(target, draw.texture) else {
                continue;
            };
            let mut stencil = Stencil::Off;
            if !draw.stencil_groups.is_empty() {
                let mut reference = draw.stencil_initial_reference as u8;
                self.solid(
                    viewport,
                    [0.0; 4],
                    Blend::NoColor,
                    Stencil::Replace { reference },
                );
                let mut masks = Vec::new();
                for group in draw.stencil_groups.iter().filter(|g| g.phase == 1) {
                    for source in &group.sources {
                        masks.push((source, reference, vitasdk_sys::SCE_GXM_STENCIL_OP_INCR_WRAP));
                    }
                    reference = reference.saturating_add(1);
                }
                let final_reference = draw.stencil_final_reference as u8;
                for group in draw.stencil_groups.iter().filter(|g| g.phase == 2) {
                    for source in &group.sources {
                        masks.push((
                            source,
                            final_reference,
                            vitasdk_sys::SCE_GXM_STENCIL_OP_DECR_WRAP,
                        ));
                    }
                }
                for (source, reference, op) in masks {
                    let Some(target) = self.emotes.get(&packet.render_id) else {
                        return;
                    };
                    let Some(mask_texture) = resolve(target, source.texture) else {
                        continue;
                    };
                    let Some(vertices) = self.gpu.upload(&source.vertices) else {
                        continue;
                    };
                    self.gpu.draw(&Draw {
                        vertex: Program::EmoteV,
                        fragment: Program::EmoteF,
                        blend: Blend::NoColor,
                        depth: Depth::OFF,
                        stencil: Stencil::Equal { reference, op },
                        cull: Cull::None,
                        viewport,
                        textures: &[Some(mask_texture)],
                        vertices,
                        vertex_count: source.vertices.len() as u32,
                        vertex_uniforms: &[],
                        fragment_uniforms: &[],
                        vertex_buffer: None,
                    });
                }
                stencil = Stencil::Equal {
                    reference: final_reference,
                    op: vitasdk_sys::SCE_GXM_STENCIL_OP_KEEP,
                };
            }
            let Some(vertices) = self.gpu.upload(&draw.vertices) else {
                continue;
            };
            self.gpu.draw(&Draw {
                vertex: Program::EmoteV,
                fragment: Program::EmoteF,
                blend: Blend::emote(draw.blend_index),
                depth: Depth::OFF,
                stencil,
                cull: Cull::None,
                viewport,
                textures: &[Some(texture)],
                vertices,
                vertex_count: draw.vertices.len() as u32,
                vertex_uniforms: &[],
                fragment_uniforms: &[],
                vertex_buffer: None,
            });
        }
        self.gpu.end_scene();
    }
}

fn is_overlay(cmd: &DrawCommand) -> bool {
    matches!(cmd.pipeline_key.technique.special, TechniqueSpecial::Overlay)
}

/// A sprite every step of whose effect chain is the identity: the plain
/// program draws it the same (texture x alpha x tr, blend fades).
fn is_plain(cmd: &DrawCommand) -> bool {
    let e = &cmd.vs_uniform.sprite_effects;
    matches!(cmd.pipeline_key.program, EffectProgram::Sprite2D)
        && matches!(cmd.pipeline_key.technique.special, TechniqueSpecial::None)
        && e[0][1..] == [0.0; 3]
        && e[1] == [0.0; 4]
        && e[2][0] == 0.0
        && e[3] == [0.0; 4]
        && e[4][0] == 0.0
        && e[4][1] == 0.0
        && e[5][0] == 0.0
        && e[6][2] < 4.5
}

/// Two triangles covering the viewport, texture rows top-down.
fn fullscreen_quad() -> [VertexSprite2dData; 6] {
    let v = |x: f32, y: f32, u: f32, t: f32| VertexSprite2dData {
        pos: [x, y, 0.0],
        uv: [u, t],
        uv_aux: [0.0, 0.0],
        alpha: 1.0,
        world_pos: [0.0; 4],
        world_normal: [0.0; 4],
    };
    [
        v(-1.0, 1.0, 0.0, 0.0),
        v(1.0, 1.0, 1.0, 0.0),
        v(1.0, -1.0, 1.0, 1.0),
        v(-1.0, 1.0, 0.0, 0.0),
        v(1.0, -1.0, 1.0, 1.0),
        v(-1.0, -1.0, 0.0, 1.0),
    ]
}

/// A 2x2 box-filtered copy.
fn half_size(image: &RgbaImage) -> RgbaImage {
    let (w, h) = (image.width as usize, image.height as usize);
    let (hw, hh) = ((w / 2).max(1), (h / 2).max(1));
    let mut rgba = vec![0u8; hw * hh * 4];
    for y in 0..hh {
        let (y0, y1) = ((2 * y).min(h - 1), (2 * y + 1).min(h - 1));
        for x in 0..hw {
            let (x0, x1) = ((2 * x).min(w - 1), (2 * x + 1).min(w - 1));
            let out = (y * hw + x) * 4;
            for c in 0..4 {
                let sum = u32::from(image.rgba[(y0 * w + x0) * 4 + c])
                    + u32::from(image.rgba[(y0 * w + x1) * 4 + c])
                    + u32::from(image.rgba[(y1 * w + x0) * 4 + c])
                    + u32::from(image.rgba[(y1 * w + x1) * 4 + c]);
                rgba[out + c] = ((sum + 2) / 4) as u8;
            }
        }
    }
    RgbaImage {
        width: hw as u32,
        height: hh as u32,
        center_x: image.center_x,
        center_y: image.center_y,
        rgba,
    }
}

impl FrameCaptureBackend for Renderer {
    /// The frame drawn into a render target and read back.
    fn capture_render_frame(
        &mut self,
        images: &ImageManager,
        frame: &RenderFrame,
        logical_width: u32,
        logical_height: u32,
    ) -> Result<RgbaImage> {
        self.gpu.begin_frame();
        self.prepare_emotes(frame);
        let final_target = self.render_frame_to_targets(images, frame)?;
        self.gpu.finish();
        let target = &self.targets[final_target as usize];
        let (data, stride) = target.pixels();
        let (tw, th) = (target.width as usize, target.height as usize);
        let width = logical_width.max(1) as usize;
        let height = logical_height.max(1) as usize;
        // Scaled to the requested size (nearest texel).
        let mut rgba = vec![0u8; width * height * 4];
        for y in 0..height {
            let sy = (y * th / height).min(th - 1);
            for x in 0..width {
                let sx = (x * tw / width).min(tw - 1);
                let src = unsafe { std::slice::from_raw_parts(data.add(sy * stride + sx * 4), 4) };
                rgba[(y * width + x) * 4..][..4].copy_from_slice(src);
                rgba[(y * width + x) * 4 + 3] = 255;
            }
        }
        Ok(RgbaImage {
            width: width as u32,
            height: height as u32,
            center_x: 0,
            center_y: 0,
            rgba,
        })
    }
}
