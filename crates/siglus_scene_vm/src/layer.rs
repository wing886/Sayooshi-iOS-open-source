use crate::image_manager::ImageHandle;

pub type LayerId = usize;
pub type SpriteId = usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpriteFit {
    /// Stretch to the entire framebuffer.
    FullScreen,
    /// Positioned in pixel coordinates with size controlled by `size_mode`.
    PixelRect,
}

#[cfg(test)]
mod tests {
    use super::LayerManager;

    #[test]
    fn resetting_transient_effects_preserves_mesh_payload() {
        let mut layers = LayerManager::new();
        let layer_id = layers.create_layer();
        let sprite_id = layers.layer_mut(layer_id).unwrap().create_sprite();
        let sprite = layers
            .layer_mut(layer_id)
            .unwrap()
            .sprite_mut(sprite_id)
            .unwrap();
        sprite.mesh_kind = 1;
        sprite.mesh_file_name = Some("room.x".to_owned());
        sprite.shadow_cast = true;
        sprite.shadow_receive = true;

        layers.reset_runtime_effects();

        let sprite = layers.layer(layer_id).unwrap().sprite(sprite_id).unwrap();
        assert_eq!(sprite.mesh_kind, 1);
        assert_eq!(sprite.mesh_file_name.as_deref(), Some("room.x"));
        assert!(sprite.shadow_cast);
        assert!(sprite.shadow_receive);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClipRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpriteSizeMode {
    /// Use the intrinsic image size.
    Intrinsic,
    /// Use an explicit size.
    Explicit { width: u32, height: u32 },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum SpriteBlend {
    #[default]
    Normal,
    Add,
    Sub,
    Mul,
    Screen,
    Overlay,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpriteRuntimeLight {
    pub id: i32,
    pub kind: i32,
    pub diffuse: [f32; 4],
    pub ambient: [f32; 4],
    pub specular: [f32; 4],
    pub pos: [f32; 4],
    pub dir: [f32; 4],
    pub atten: [f32; 4],
    pub cone: [f32; 4],
}

impl Default for SpriteRuntimeLight {
    fn default() -> Self {
        Self {
            id: -1,
            kind: -1,
            diffuse: [0.0, 0.0, 0.0, 1.0],
            ambient: [0.0, 0.0, 0.0, 1.0],
            specular: [0.0, 0.0, 0.0, 1.0],
            pos: [0.0, 0.0, 0.0, 1.0],
            dir: [0.0, 0.0, -1.0, 0.0],
            atten: [1.0, 0.0, 0.0, 5000.0],
            cone: [0.0, 0.0, 1.0, 0.0],
        }
    }
}
impl SpriteBlend {
    pub fn from_i64(v: i64) -> Self {
        // C_elm_object clamps OBJECT.BLEND to TNM_BLEND_NORMAL..TNM_BLEND_SCREEN
        // before converting it to a render blend mode. Overlay is an internal
        // renderer mode and is not addressable through the Siglus property.
        match v.clamp(0, 4) {
            1 => SpriteBlend::Add,
            2 => SpriteBlend::Sub,
            3 => SpriteBlend::Mul,
            4 => SpriteBlend::Screen,
            _ => SpriteBlend::Normal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub image_id: Option<ImageHandle>,
    /// Dynamic texture produced by the Eluna Emote compositor.
    /// The outer Siglus object still uses the normal sprite pipeline.
    pub emote_render: Option<std::sync::Arc<crate::emote::EmoteRenderPacket>>,
    pub mask_image_id: Option<ImageHandle>,
    pub mask_offset_x: i32,
    pub mask_offset_y: i32,
    pub tonecurve_image_id: Option<ImageHandle>,
    pub tonecurve_row: f32,
    pub tonecurve_sat: f32,
    pub fit: SpriteFit,
    pub size_mode: SpriteSizeMode,
    pub visible: bool,
    pub alpha: u8,
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub rotate: f32,
    pub rotate_x: f32,
    pub rotate_y: f32,
    pub pivot_x: f32,
    pub pivot_y: f32,
    pub pivot_z: f32,
    /// Siglus object sprites use OBJECT.X/Y as the render anchor. For G00/PCT
    /// object resources, vertices are offset by OBJECT.CENTER plus the texture
    /// center stored in the cut metadata. Runtime helper sprites keep this off
    /// and continue to use x/y as a top-left pixel rectangle.
    pub object_anchor: bool,
    pub texture_center_x: f32,
    pub texture_center_y: f32,
    pub camera_enabled: bool,
    pub camera_eye: [f32; 3],
    pub camera_target: [f32; 3],
    pub camera_up: [f32; 3],
    pub camera_view_angle_deg: f32,
    pub culling: bool,
    pub alpha_test: bool,
    pub alpha_blend: bool,
    pub fog_use: bool,
    pub light_no: i32,
    pub light_enabled: bool,
    pub light_diffuse: [f32; 4],
    pub light_ambient: [f32; 4],
    pub light_specular: [f32; 4],
    pub light_factor: f32,
    pub light_kind: i32,
    pub light_pos: [f32; 4],
    pub light_dir: [f32; 4],
    pub light_atten: [f32; 4],
    pub light_cone: [f32; 4],
    pub mesh_runtime_lights: Vec<SpriteRuntimeLight>,
    pub fog_enabled: bool,
    pub fog_color: [f32; 4],
    pub fog_near: f32,
    pub fog_far: f32,
    pub fog_scroll_x: f32,
    pub fog_texture_image_id: Option<ImageHandle>,
    pub world_no: i32,
    pub billboard: bool,
    pub mesh_file_name: Option<String>,
    pub mesh_animation: crate::mesh3d::MeshAnimationState,
    /// 0 = regular sprite or 2D camera quad, 1 = static mesh, 2 = billboard mesh, 3 = skinned mesh.
    pub mesh_kind: u8,
    pub shadow_cast: bool,
    pub shadow_receive: bool,
    /// Runtime wipe/effect family applied by the renderer.
    /// 0=none, 1=mosaic, 2=raster_h, 3=raster_v, 4=explosion_blur,
    /// 5=shimi, 6=shimi_inv, 10=cross_mosaic, 11=cross_raster_h,
    /// 12=cross_raster_v, 13=cross_explosion_blur.
    pub wipe_fx_mode: u8,
    pub wipe_fx_params: [f32; 4],
    /// Optional secondary source texture for dual-source wipe/effect composition.
    pub wipe_src_image_id: Option<ImageHandle>,
    pub tr: u8,
    pub mono: u8,
    pub reverse: u8,
    pub bright: u8,
    pub dark: u8,
    pub color_rate: u8,
    pub color_add_r: u8,
    pub color_add_g: u8,
    pub color_add_b: u8,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    /// 0 = off, 1 = use luminance as alpha, 2 = use texture alpha.
    pub mask_mode: u8,
    pub blend: SpriteBlend,
    pub dst_clip: Option<ClipRect>,
    pub src_clip: Option<ClipRect>,
    /// Render order within a layer (ascending).
    pub order: i32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            image_id: None,
            emote_render: None,
            mask_image_id: None,
            mask_offset_x: 0,
            mask_offset_y: 0,
            tonecurve_image_id: None,
            tonecurve_row: 0.0,
            tonecurve_sat: 0.0,
            fit: SpriteFit::PixelRect,
            size_mode: SpriteSizeMode::Intrinsic,
            visible: false,
            alpha: 255,
            x: 0,
            y: 0,
            z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
            rotate: 0.0,
            rotate_x: 0.0,
            rotate_y: 0.0,
            pivot_x: 0.0,
            pivot_y: 0.0,
            pivot_z: 0.0,
            object_anchor: false,
            texture_center_x: 0.0,
            texture_center_y: 0.0,
            camera_enabled: false,
            camera_eye: [0.0, 0.0, -1000.0],
            camera_target: [0.0, 0.0, 0.0],
            camera_up: [0.0, 1.0, 0.0],
            camera_view_angle_deg: 45.0,
            culling: false,
            alpha_test: false,
            alpha_blend: true,
            fog_use: false,
            light_no: -1,
            light_enabled: false,
            light_diffuse: [1.0, 1.0, 1.0, 1.0],
            light_ambient: [0.0, 0.0, 0.0, 1.0],
            light_specular: [0.0, 0.0, 0.0, 1.0],
            light_factor: 0.0,
            light_kind: -1,
            light_pos: [0.0, 0.0, 0.0, 0.0],
            light_dir: [0.0, 0.0, -1.0, 0.0],
            light_atten: [1.0, 0.0, 0.0, 5000.0],
            light_cone: [0.0, 0.0, 1.0, 0.0],
            mesh_runtime_lights: Vec::new(),
            fog_enabled: false,
            fog_color: [0.0, 0.0, 0.0, 1.0],
            fog_near: 0.0,
            fog_far: 0.0,
            fog_scroll_x: 0.0,
            fog_texture_image_id: None,
            world_no: -1,
            billboard: false,
            mesh_file_name: None,
            mesh_animation: crate::mesh3d::MeshAnimationState::default(),
            mesh_kind: 0,
            shadow_cast: false,
            shadow_receive: false,
            wipe_fx_mode: 0,
            wipe_fx_params: [0.0; 4],
            wipe_src_image_id: None,
            tr: 255,
            mono: 0,
            reverse: 0,
            bright: 0,
            dark: 0,
            color_rate: 0,
            color_add_r: 0,
            color_add_g: 0,
            color_add_b: 0,
            color_r: 0,
            color_g: 0,
            color_b: 0,
            mask_mode: 0,
            blend: SpriteBlend::Normal,
            dst_clip: None,
            src_clip: None,
            order: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    sprites: Vec<Sprite>,
}

impl Default for Layer {
    fn default() -> Self {
        Self::new()
    }
}

impl Layer {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    pub fn create_sprite(&mut self) -> SpriteId {
        let id = self.sprites.len();
        self.sprites.push(Sprite::default());
        id
    }

    pub fn sprite(&self, id: SpriteId) -> Option<&Sprite> {
        self.sprites.get(id)
    }

    pub fn sprite_mut(&mut self, id: SpriteId) -> Option<&mut Sprite> {
        self.sprites.get_mut(id)
    }

    pub fn clear_all_sprites(&mut self) {
        // Keep sprite IDs stable: do not shrink the sprite vector.
        for s in &mut self.sprites {
            s.image_id = None;
            s.emote_render = None;
            s.mask_image_id = None;
            s.mask_offset_x = 0;
            s.mask_offset_y = 0;
            s.tonecurve_image_id = None;
            s.tonecurve_row = 0.0;
            s.tonecurve_sat = 0.0;
            s.visible = false;
            s.alpha = 255;
            s.x = 0;
            s.y = 0;
            s.z = 0.0;
            s.order = 0;
            s.fit = SpriteFit::PixelRect;
            s.size_mode = SpriteSizeMode::Intrinsic;
            s.scale_x = 1.0;
            s.scale_y = 1.0;
            s.scale_z = 1.0;
            s.rotate = 0.0;
            s.rotate_x = 0.0;
            s.rotate_y = 0.0;
            s.pivot_x = 0.0;
            s.pivot_y = 0.0;
            s.pivot_z = 0.0;
            s.camera_enabled = false;
            s.camera_eye = [0.0, 0.0, -1000.0];
            s.camera_target = [0.0, 0.0, 0.0];
            s.camera_up = [0.0, 1.0, 0.0];
            s.camera_view_angle_deg = 45.0;
            s.culling = false;
            s.alpha_test = false;
            s.alpha_blend = true;
            s.fog_use = false;
            s.light_no = -1;
            s.light_enabled = false;
            s.light_diffuse = [1.0, 1.0, 1.0, 1.0];
            s.light_ambient = [0.0, 0.0, 0.0, 1.0];
            s.light_specular = [0.0, 0.0, 0.0, 1.0];
            s.light_factor = 0.0;
            s.light_kind = -1;
            s.light_pos = [0.0, 0.0, 0.0, 0.0];
            s.light_dir = [0.0, 0.0, -1.0, 0.0];
            s.light_atten = [1.0, 0.0, 0.0, 5000.0];
            s.light_cone = [0.0, 0.0, 1.0, 0.0];
            s.mesh_runtime_lights.clear();
            s.fog_enabled = false;
            s.fog_color = [0.0, 0.0, 0.0, 1.0];
            s.fog_near = 0.0;
            s.fog_far = 0.0;
            s.fog_scroll_x = 0.0;
            s.fog_texture_image_id = None;
            s.world_no = -1;
            s.billboard = false;
            s.mesh_kind = 0;
            s.mesh_file_name = None;
            s.mesh_animation = crate::mesh3d::MeshAnimationState::default();
            s.shadow_cast = false;
            s.shadow_receive = false;
            s.wipe_fx_mode = 0;
            s.wipe_fx_params = [0.0; 4];
            s.wipe_src_image_id = None;
            s.tr = 255;
            s.mono = 0;
            s.reverse = 0;
            s.bright = 0;
            s.dark = 0;
            s.color_rate = 0;
            s.color_add_r = 0;
            s.color_add_g = 0;
            s.color_add_b = 0;
            s.color_r = 0;
            s.color_g = 0;
            s.color_b = 0;
            s.blend = SpriteBlend::Normal;
            s.dst_clip = None;
            s.src_clip = None;
        }
    }

    fn sprite_ids_sorted(&self) -> Vec<SpriteId> {
        let mut ids: Vec<SpriteId> = (0..self.sprites.len()).collect();
        ids.sort_by(|&a, &b| {
            let oa = self.sprites[a].order;
            let ob = self.sprites[b].order;
            oa.cmp(&ob).then(a.cmp(&b))
        });
        ids
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderFrame {
    /// Ordinary painter-ordered submission when no stage wipe is active.
    pub sprites: Vec<RenderSprite>,
    /// Original-engine two-render-target wipe submission.
    pub wipe: Option<WipeRenderPlan>,
}

impl RenderFrame {
    pub fn ordinary(sprites: Vec<RenderSprite>) -> Self {
        Self {
            sprites,
            wipe: None,
        }
    }

    pub fn submitted_sprite_count(&self) -> usize {
        self.wipe.as_ref().map_or(self.sprites.len(), |wipe| {
            wipe.under.len() + wipe.current.len() + wipe.next.len() + wipe.over.len()
        })
    }

    /// Debug-only flattening. Actual wipe composition is always performed by wgpu.
    pub fn debug_flatten(&self) -> Vec<RenderSprite> {
        if let Some(wipe) = self.wipe.as_ref() {
            let mut out = Vec::with_capacity(self.submitted_sprite_count());
            out.extend(wipe.under.iter().cloned());
            out.extend(wipe.current.iter().cloned());
            out.extend(wipe.next.iter().cloned());
            out.extend(wipe.over.iter().cloned());
            out
        } else {
            self.sprites.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct WipeRenderPlan {
    pub under: Vec<RenderSprite>,
    pub current: Vec<RenderSprite>,
    pub next: Vec<RenderSprite>,
    pub over: Vec<RenderSprite>,
    pub wipe_type: i32,
    pub option: Vec<i32>,
    pub progress: f32,
    pub mask_image_id: Option<ImageHandle>,
    /// One RNG sample is chosen when WIPE starts and stays fixed for its lifetime.
    pub random_seed: u32,
}

#[derive(Debug, Clone)]
pub struct RenderSprite {
    pub layer_id: Option<LayerId>,
    pub sprite_id: Option<SpriteId>,
    /// Original Siglus C++ S_tnm_sorter.order. This is separate from
    /// Sprite::order because Sprite::order is still used by older backend
    /// storage paths and debug output. Final scene ordering, wipe ranges,
    /// effects, and quake ranges must use this pair.
    pub sorter_order: i32,
    /// Original Siglus C++ S_tnm_sorter.layer. This can be much larger than
    /// 1023, so it must not be packed into Sprite::order.
    pub sorter_layer: i32,
    /// Sorter of the top-level stage entity used by C_elm_stage::get_sprite_tree()
    /// when selecting WIPE ranges. Descendant sprites retain their own sorter
    /// for draw ordering/effects, but the whole object subtree is selected by
    /// its top-level parent sorter in the original engine.
    pub wipe_sorter_order: i32,
    pub wipe_sorter_layer: i32,
    /// Stage-form owner used to apply SCREEN/STAGE effects exactly as the
    /// original two-stage render preparation does. `None` is reserved for
    /// non-stage overlays such as message-back projections and the cursor.
    pub stage_form_id: Option<u32>,
    pub sprite: Sprite,
}

impl RenderSprite {
    pub fn new(layer_id: Option<LayerId>, sprite_id: Option<SpriteId>, sprite: Sprite) -> Self {
        let (sorter_order, sorter_layer) = unpack_sprite_order(sprite.order);
        Self {
            layer_id,
            sprite_id,
            sorter_order,
            sorter_layer,
            wipe_sorter_order: sorter_order,
            wipe_sorter_layer: sorter_layer,
            stage_form_id: None,
            sprite,
        }
    }

    pub fn with_sorter(
        layer_id: Option<LayerId>,
        sprite_id: Option<SpriteId>,
        sorter_order: i32,
        sorter_layer: i32,
        sprite: Sprite,
    ) -> Self {
        Self {
            layer_id,
            sprite_id,
            sorter_order,
            sorter_layer,
            wipe_sorter_order: sorter_order,
            wipe_sorter_layer: sorter_layer,
            stage_form_id: None,
            sprite,
        }
    }

    pub fn set_sorter(&mut self, order: i64, layer: i64) {
        self.sorter_order = order.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        self.sorter_layer = layer.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        self.wipe_sorter_order = self.sorter_order;
        self.wipe_sorter_layer = self.sorter_layer;
    }

    pub fn set_wipe_sorter(&mut self, order: i64, layer: i64) {
        self.wipe_sorter_order = order.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        self.wipe_sorter_layer = layer.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    }

    pub fn set_stage_form_owner(&mut self, form_id: u32) {
        self.stage_form_id = Some(form_id);
    }
}

fn unpack_sprite_order(order: i32) -> (i32, i32) {
    if order.abs() >= 1024 {
        (order.div_euclid(1024), order.rem_euclid(1024))
    } else {
        (0, order)
    }
}

#[derive(Debug, Default)]
pub struct LayerManager {
    bg: Sprite,
    layers: Vec<Layer>,
}

impl LayerManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bg_mut(&mut self) -> &mut Sprite {
        &mut self.bg
    }

    pub fn set_bg_image(&mut self, image_id: ImageHandle) {
        self.bg.image_id = Some(image_id);
        self.bg.mask_image_id = None;
        self.bg.mask_offset_x = 0;
        self.bg.mask_offset_y = 0;
        self.bg.tonecurve_image_id = None;
        self.bg.tonecurve_row = 0.0;
        self.bg.tonecurve_sat = 0.0;
        self.bg.wipe_fx_mode = 0;
        self.bg.wipe_fx_params = [0.0; 4];
        self.bg.wipe_src_image_id = None;
        self.bg.fit = SpriteFit::FullScreen;
        self.bg.size_mode = SpriteSizeMode::Intrinsic;
        self.bg.visible = true;
        self.bg.order = i32::MIN;
        self.bg.scale_x = 1.0;
        self.bg.scale_y = 1.0;
        self.bg.scale_z = 1.0;
        self.bg.rotate = 0.0;
        self.bg.rotate_x = 0.0;
        self.bg.rotate_y = 0.0;
        self.bg.pivot_x = 0.0;
        self.bg.pivot_y = 0.0;
        self.bg.pivot_z = 0.0;
        self.bg.z = 0.0;
        self.bg.camera_enabled = false;
        self.bg.camera_eye = [0.0, 0.0, -1000.0];
        self.bg.camera_target = [0.0, 0.0, 0.0];
        self.bg.camera_up = [0.0, 1.0, 0.0];
        self.bg.camera_view_angle_deg = 45.0;
        self.bg.culling = false;
        self.bg.alpha_test = false;
        self.bg.alpha_blend = true;
        self.bg.fog_use = false;
        self.bg.light_no = -1;
        self.bg.light_enabled = false;
        self.bg.light_diffuse = [1.0, 1.0, 1.0, 1.0];
        self.bg.light_ambient = [0.0, 0.0, 0.0, 1.0];
        self.bg.light_specular = [0.0, 0.0, 0.0, 1.0];
        self.bg.light_factor = 0.0;
        self.bg.light_kind = -1;
        self.bg.light_pos = [0.0, 0.0, 0.0, 0.0];
        self.bg.light_dir = [0.0, 0.0, -1.0, 0.0];
        self.bg.light_atten = [1.0, 0.0, 0.0, 5000.0];
        self.bg.light_cone = [0.0, 0.0, 1.0, 0.0];
        self.bg.mesh_runtime_lights.clear();
        self.bg.fog_enabled = false;
        self.bg.fog_color = [0.0, 0.0, 0.0, 1.0];
        self.bg.fog_near = 0.0;
        self.bg.fog_far = 0.0;
        self.bg.fog_scroll_x = 0.0;
        self.bg.fog_texture_image_id = None;
        self.bg.world_no = -1;
        self.bg.billboard = false;
        self.bg.mesh_kind = 0;
        self.bg.mesh_file_name = None;
        self.bg.mesh_animation = crate::mesh3d::MeshAnimationState::default();
        self.bg.shadow_cast = false;
        self.bg.shadow_receive = false;
        self.bg.wipe_fx_mode = 0;
        self.bg.wipe_fx_params = [0.0; 4];
        self.bg.wipe_src_image_id = None;
        self.bg.tr = 255;
        self.bg.mono = 0;
        self.bg.reverse = 0;
        self.bg.bright = 0;
        self.bg.dark = 0;
        self.bg.color_rate = 0;
        self.bg.color_add_r = 0;
        self.bg.color_add_g = 0;
        self.bg.color_add_b = 0;
        self.bg.color_r = 0;
        self.bg.color_g = 0;
        self.bg.color_b = 0;
        self.bg.mask_mode = 0;
        self.bg.blend = SpriteBlend::Normal;
        self.bg.dst_clip = None;
        self.bg.src_clip = None;
    }

    pub fn clear_bg(&mut self) {
        self.bg.image_id = None;
        self.bg.mask_image_id = None;
        self.bg.mask_offset_x = 0;
        self.bg.mask_offset_y = 0;
        self.bg.tonecurve_image_id = None;
        self.bg.tonecurve_row = 0.0;
        self.bg.tonecurve_sat = 0.0;
        self.bg.wipe_fx_mode = 0;
        self.bg.wipe_fx_params = [0.0; 4];
        self.bg.wipe_src_image_id = None;
        self.bg.light_enabled = false;
        self.bg.light_diffuse = [1.0, 1.0, 1.0, 1.0];
        self.bg.light_ambient = [0.0, 0.0, 0.0, 1.0];
        self.bg.light_specular = [0.0, 0.0, 0.0, 1.0];
        self.bg.light_factor = 0.0;
        self.bg.light_kind = -1;
        self.bg.light_pos = [0.0, 0.0, 0.0, 0.0];
        self.bg.light_dir = [0.0, 0.0, -1.0, 0.0];
        self.bg.light_atten = [1.0, 0.0, 0.0, 5000.0];
        self.bg.light_cone = [0.0, 0.0, 1.0, 0.0];
        self.bg.mesh_runtime_lights.clear();
        self.bg.fog_enabled = false;
        self.bg.fog_color = [0.0, 0.0, 0.0, 1.0];
        self.bg.fog_near = 0.0;
        self.bg.fog_far = 0.0;
        self.bg.fog_scroll_x = 0.0;
        self.bg.fog_texture_image_id = None;
        self.bg.mesh_kind = 0;
        self.bg.mesh_file_name = None;
        self.bg.mesh_animation = crate::mesh3d::MeshAnimationState::default();
        self.bg.shadow_cast = false;
        self.bg.shadow_receive = false;
        self.bg.visible = false;
    }

    pub fn create_layer(&mut self) -> LayerId {
        let id = self.layers.len();
        self.layers.push(Layer::new());
        id
    }

    pub fn layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.get(id)
    }

    pub fn layer_mut(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.get_mut(id)
    }

    pub fn clear_layer(&mut self, id: LayerId) {
        if let Some(layer) = self.layers.get_mut(id) {
            layer.clear_all_sprites();
        }
    }

    pub fn clear_all(&mut self) {
        self.clear_bg();
        for layer in &mut self.layers {
            layer.clear_all_sprites();
        }
    }

    pub fn reset_runtime_effects(&mut self) {
        self.bg.mask_image_id = None;
        self.bg.mask_offset_x = 0;
        self.bg.mask_offset_y = 0;
        self.bg.tonecurve_image_id = None;
        self.bg.tonecurve_row = 0.0;
        self.bg.tonecurve_sat = 0.0;
        self.bg.wipe_fx_mode = 0;
        self.bg.wipe_fx_params = [0.0; 4];
        self.bg.wipe_src_image_id = None;
        self.bg.light_enabled = false;
        self.bg.light_diffuse = [1.0, 1.0, 1.0, 1.0];
        self.bg.light_ambient = [0.0, 0.0, 0.0, 1.0];
        self.bg.light_specular = [0.0, 0.0, 0.0, 1.0];
        self.bg.light_factor = 0.0;
        self.bg.light_kind = -1;
        self.bg.light_pos = [0.0, 0.0, 0.0, 0.0];
        self.bg.light_dir = [0.0, 0.0, -1.0, 0.0];
        self.bg.light_atten = [1.0, 0.0, 0.0, 5000.0];
        self.bg.light_cone = [0.0, 0.0, 1.0, 0.0];
        self.bg.mesh_runtime_lights.clear();
        self.bg.fog_enabled = false;
        self.bg.fog_color = [0.0, 0.0, 0.0, 1.0];
        self.bg.fog_near = 0.0;
        self.bg.fog_far = 0.0;
        self.bg.fog_scroll_x = 0.0;
        self.bg.fog_texture_image_id = None;
        for layer in &mut self.layers {
            for s in &mut layer.sprites {
                s.mask_image_id = None;
                s.mask_offset_x = 0;
                s.mask_offset_y = 0;
                s.tonecurve_image_id = None;
                s.tonecurve_row = 0.0;
                s.tonecurve_sat = 0.0;
                s.wipe_fx_mode = 0;
                s.wipe_fx_params = [0.0; 4];
                s.wipe_src_image_id = None;
                s.light_enabled = false;
                s.light_diffuse = [1.0, 1.0, 1.0, 1.0];
                s.light_ambient = [0.0, 0.0, 0.0, 1.0];
                s.light_factor = 0.0;
                s.fog_enabled = false;
                s.fog_color = [0.0, 0.0, 0.0, 1.0];
                s.fog_near = 0.0;
                s.fog_far = 0.0;
                s.fog_scroll_x = 0.0;
                s.fog_texture_image_id = None;
            }
        }
    }

    pub fn render_list(&self) -> Vec<RenderSprite> {
        self.render_list_excluding(|_, _| false)
    }

    /// Build the generic layer-backed submission while skipping sprite bindings
    /// that are owned by the Siglus OBJECT tree.  OBJECT sprites are rebuilt from
    /// the object tree later, so cloning them here only to discard them is wasted
    /// frame memory and work.
    pub fn render_list_excluding<F>(&self, mut exclude: F) -> Vec<RenderSprite>
    where
        F: FnMut(LayerId, SpriteId) -> bool,
    {
        let mut out = Vec::new();

        if self.bg.visible
            && self.bg.alpha > 0
            && self.bg.tr > 0
            && let Some(ref img) = self.bg.image_id
        {
            let mut bg = self.bg.clone();
            bg.image_id = Some(img.clone());
            out.push(RenderSprite::new(None, None, bg));
        }

        for (layer_id, layer) in self.layers.iter().enumerate() {
            for sprite_id in layer.sprite_ids_sorted() {
                if exclude(layer_id, sprite_id) {
                    continue;
                }
                let s = &layer.sprites[sprite_id];
                if !s.visible {
                    continue;
                }
                if s.image_id.is_none() || s.alpha == 0 || s.tr == 0 {
                    continue;
                }
                out.push(RenderSprite::new(
                    Some(layer_id),
                    Some(sprite_id),
                    s.clone(),
                ));
            }
        }

        out
    }
}
