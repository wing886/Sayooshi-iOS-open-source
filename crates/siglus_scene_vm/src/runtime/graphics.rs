//! Graphics runtime: bridges VM stage/object operations to `LayerManager` + `ImageManager`.
//!
//! This layer maps stage/object operations onto renderable sprites while preserving
//! stable sprite identities for the VM runtime.

use anyhow::{Context, Result, bail};
use std::collections::HashMap;

use crate::image_manager::{ImageHandle, ImageManager};
use crate::layer::{
    ClipRect, LayerId, LayerManager, Sprite, SpriteBlend, SpriteFit, SpriteId, SpriteSizeMode,
};

fn sg_cgm_coord_trace_enabled() -> bool {
    env_is_set!("SG_DEBUG")
}

/// Logs `format!(...)` arguments only when tracing (they are not built
/// otherwise: object_set_pos runs for moving objects every frame).
macro_rules! sg_cgm_coord_trace {
    ($($arg:tt)*) => {
        if sg_cgm_coord_trace_enabled() {
            eprintln!("[SG_DEBUG][CGM_COORD_TRACE][GFX] {}", format!($($arg)*));
        }
    };
}

fn cgm_file_interesting(file: Option<&str>) -> bool {
    file.map(|name| name.to_ascii_lowercase().contains("cgm_"))
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
struct ObjectState {
    // Every Siglus OBJECT, including BACK.OBJECT[0], owns an ordinary stage sprite.
    layer_id: Option<LayerId>,
    sprite_id: Option<SpriteId>,

    // Logical properties.
    is_mesh: bool,
    file: Option<String>,
    patno: i64,
    // Resource binding cached on the backing sprite. Script/render parameters
    // change every frame; the loaded album does not. Keeping these separate
    // mirrors C_elm_object::m_album + m_op.obp in the original engine.
    bound_file: Option<String>,
    bound_patno: i64,
    disp: bool,
    x: i64,
    y: i64,
    layer_no: i64,
    order: i64,
    alpha: i64,
    /// Stored but not used for sorting (Siglus draw order follows tree traversal).
    z: i64,
    center_x: i64,
    center_y: i64,
    scale_x: i64,
    scale_y: i64,
    rotate_z: i64,
    clip_use: i64,
    clip_left: i64,
    clip_top: i64,
    clip_right: i64,
    clip_bottom: i64,
    src_clip_use: i64,
    src_clip_left: i64,
    src_clip_top: i64,
    src_clip_right: i64,
    src_clip_bottom: i64,
    tr: i64,
    mono: i64,
    reverse: i64,
    bright: i64,
    dark: i64,
    color_rate: i64,
    color_add_r: i64,
    color_add_g: i64,
    color_add_b: i64,
    color_r: i64,
    color_g: i64,
    color_b: i64,
    blend: i64,
    light_no: i64,
    fog_use: i64,
}

impl Default for ObjectState {
    fn default() -> Self {
        Self {
            layer_id: None,
            sprite_id: None,
            is_mesh: false,
            file: None,
            patno: 0,
            bound_file: None,
            bound_patno: -1,
            disp: false,
            x: 0,
            y: 0,
            layer_no: 0,
            order: 0,
            alpha: 255,
            z: 0,
            center_x: 0,
            center_y: 0,
            scale_x: 1000,
            scale_y: 1000,
            rotate_z: 0,
            clip_use: 0,
            clip_left: 0,
            clip_top: 0,
            clip_right: 0,
            clip_bottom: 0,
            src_clip_use: 0,
            src_clip_left: 0,
            src_clip_top: 0,
            src_clip_right: 0,
            src_clip_bottom: 0,
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
            blend: 0,
            light_no: -1,
            fog_use: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DebugObjectSpriteBinding {
    pub stage: usize,
    pub obj_idx: usize,
    pub is_bg: bool,
    pub layer_id: Option<LayerId>,
    pub sprite_id: Option<SpriteId>,
    pub file: Option<String>,
    pub patno: i64,
    pub disp: bool,
    pub x: i64,
    pub y: i64,
    pub layer_no: i64,
    pub order: i64,
    pub alpha: i64,
    pub z: i64,
    pub tr: i64,
    pub clip_use: i64,
    pub clip_left: i64,
    pub clip_top: i64,
    pub clip_right: i64,
    pub clip_bottom: i64,
    pub scale_x: i64,
    pub scale_y: i64,
    pub rotate_z: i64,
}

#[derive(Debug, Clone, Default)]
struct StageState {
    layer_id: Option<LayerId>,
    objects: HashMap<usize, ObjectState>,
}

#[derive(Debug, Default)]
pub struct GfxRuntime {
    /// Current logical layer number selected by named commands (LAYER/LAYER_SET).
    /// Used as a default for CHR/object operations when scripts omit an explicit layer.
    pub current_layer: i32,
    stages: [StageState; 3],
}

impl GfxRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_stage(&mut self, stage: usize) -> &mut StageState {
        &mut self.stages[stage]
    }

    fn ensure_stage_layer(&mut self, layers: &mut LayerManager, stage: usize) -> LayerId {
        let st = self.ensure_stage(stage);
        if let Some(id) = st.layer_id {
            return id;
        }
        let id = layers.create_layer();
        st.layer_id = Some(id);
        id
    }

    /// Expose stage layer allocation for non-Gfx backends (e.g., movie sprites).
    pub fn ensure_stage_layer_id(
        &mut self,
        layers: &mut LayerManager,
        stage: i64,
    ) -> Option<LayerId> {
        if !(0..=2).contains(&stage) {
            return None;
        }
        Some(self.ensure_stage_layer(layers, stage as usize))
    }

    fn ensure_object_mut(&mut self, stage: usize, obj_idx: usize) -> &mut ObjectState {
        self.ensure_stage(stage).objects.entry(obj_idx).or_default()
    }

    fn object(&self, stage: usize, obj_idx: usize) -> Option<&ObjectState> {
        self.stages.get(stage)?.objects.get(&obj_idx)
    }

    fn reset_object_for_create(&mut self, layers: &mut LayerManager, stage: usize, obj_idx: usize) {
        let (layer_id, sprite_id) = {
            let obj = self.ensure_object_mut(stage, obj_idx);
            (obj.layer_id, obj.sprite_id)
        };

        {
            let obj = self.ensure_object_mut(stage, obj_idx);
            *obj = ObjectState::default();
            obj.layer_id = layer_id;
            obj.sprite_id = sprite_id;
        }

        if let (Some(lid), Some(sid)) = (layer_id, sprite_id)
            && let Some(sprite) = layers
                .layer_mut(lid)
                .and_then(|layer| layer.sprite_mut(sid))
        {
            *sprite = Sprite::default();
        }
    }

    pub fn debug_object_snapshot(
        &self,
        stage: usize,
        obj_idx: usize,
    ) -> Option<DebugObjectSpriteBinding> {
        let obj = self.object(stage, obj_idx)?;
        Some(DebugObjectSpriteBinding {
            stage,
            obj_idx,
            // Kept in the debug ABI for existing HUD callers. Siglus OBJECTs are never
            // redirected to LayerManager::bg; BACK.OBJECT[0] is an ordinary object.
            is_bg: false,
            layer_id: obj.layer_id,
            sprite_id: obj.sprite_id,
            file: obj.file.clone(),
            patno: obj.patno,
            disp: obj.disp,
            x: obj.x,
            y: obj.y,
            layer_no: obj.layer_no,
            order: obj.order,
            alpha: obj.alpha,
            z: obj.z,
            tr: obj.tr,
            clip_use: obj.clip_use,
            clip_left: obj.clip_left,
            clip_top: obj.clip_top,
            clip_right: obj.clip_right,
            clip_bottom: obj.clip_bottom,
            scale_x: obj.scale_x,
            scale_y: obj.scale_y,
            rotate_z: obj.rotate_z,
        })
    }

    pub fn object_sprite_binding(&self, stage: i64, obj_idx: i64) -> Option<(LayerId, SpriteId)> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let obj = self.object(stage_i as usize, obj_idx as usize)?;
        match (obj.layer_id, obj.sprite_id) {
            (Some(lid), Some(sid)) => Some((lid, sid)),
            _ => None,
        }
    }

    /// Cheap guard for the compatibility repair pass in `CommandContext`.
    ///
    /// Normal PCT objects keep their image binding for their whole lifetime.
    /// Walking every Stage/MWND/child tree just to discover that nothing is
    /// missing is much more expensive than checking the compact GfxRuntime
    /// object table first.  This deliberately mirrors the repair pass' scope:
    /// only Gfx objects with an existing sprite binding can be repaired there.
    pub fn has_missing_bound_object_image(&self, layers: &LayerManager) -> bool {
        self.stages.iter().any(|stage| {
            stage.objects.values().any(|obj| {
                let Some(file) = obj.file.as_deref() else {
                    return false;
                };
                if obj.is_mesh || file.is_empty() {
                    return false;
                }
                let (Some(layer_id), Some(sprite_id)) = (obj.layer_id, obj.sprite_id) else {
                    return false;
                };
                layers
                    .layer(layer_id)
                    .and_then(|layer| layer.sprite(sprite_id))
                    .is_some_and(|sprite| sprite.image_id.is_none())
            })
        })
    }

    fn load_any_image(images: &mut ImageManager, file: &str, patno: i64) -> Result<ImageHandle> {
        // Siglus delegates descriptors containing `|` to Tona3's composed-G00
        // loader. The whole descriptor is not a resource file name and composed
        // textures intentionally do not fall back to bg/png/jpeg resources.
        if file.contains('|') {
            if patno != 0 {
                bail!("composed g00 has one texture; invalid pattern {patno}");
            }
            return images
                .load_g00_composed(file)
                .with_context(|| format!("failed to load composed g00: {file}"));
        }

        // Engine preference: g00 first, then bg fallback.
        let pat_u32 = if patno < 0 { 0 } else { patno as u32 };
        match images.load_g00(file, pat_u32) {
            Ok(id) => Ok(id),
            Err(_) => images
                .load_bg(file)
                .with_context(|| format!("failed to load image as g00/bg: {file}")),
        }
    }

    fn ensure_bound_sprite(
        &mut self,
        layers: &mut LayerManager,
        stage: usize,
        obj_idx: usize,
    ) -> Result<(LayerId, SpriteId)> {
        let st_layer = self.ensure_stage_layer(layers, stage);
        let obj = self.ensure_object_mut(stage, obj_idx);

        if let (Some(lid), Some(sid)) = (obj.layer_id, obj.sprite_id) {
            return Ok((lid, sid));
        }

        let sid = {
            let layer = layers
                .layer_mut(st_layer)
                .context("stage layer not found")?;
            layer.create_sprite()
        };

        // Initialize with sane defaults.
        if let Some(layer) = layers.layer_mut(st_layer)
            && let Some(sprite) = layer.sprite_mut(sid)
        {
            sprite.visible = true;
            sprite.alpha = 255;
            sprite.fit = SpriteFit::PixelRect;
            sprite.size_mode = SpriteSizeMode::Intrinsic;
            sprite.x = 0;
            sprite.y = 0;
            sprite.order = 0;
        }

        obj.layer_id = Some(st_layer);
        obj.sprite_id = Some(sid);
        Ok((st_layer, sid))
    }

    fn sync_object_sprite(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: usize,
        obj_idx: usize,
    ) -> Result<()> {
        // Read in place (this runs for every object property write; a clone
        // copied its file names each time). The binding is written back last.
        self.ensure_object_mut(stage, obj_idx);
        let (lid, sid) = self.ensure_bound_sprite(layers, stage, obj_idx)?;
        let obj = self.object(stage, obj_idx).context("object not found")?;
        let sprite = layers
            .layer_mut(lid)
            .and_then(|l| l.sprite_mut(sid))
            .context("sprite not found")?;

        sprite.visible = obj.disp;
        sprite.x = obj.x as i32;
        sprite.y = obj.y as i32;
        sprite.alpha = obj.alpha.clamp(0, 255) as u8;
        sprite.scale_x = obj.scale_x as f32 / 1000.0;
        sprite.scale_y = obj.scale_y as f32 / 1000.0;
        sprite.rotate = obj.rotate_z as f32 * std::f32::consts::PI / 1800.0;
        sprite.pivot_x = obj.center_x as f32;
        sprite.pivot_y = obj.center_y as f32;
        sprite.dst_clip = clip_rect(
            obj.clip_use,
            obj.clip_left,
            obj.clip_top,
            obj.clip_right,
            obj.clip_bottom,
        );
        sprite.src_clip = clip_rect(
            obj.src_clip_use,
            obj.src_clip_left,
            obj.src_clip_top,
            obj.src_clip_right,
            obj.src_clip_bottom,
        );
        sprite.tr = obj.tr.clamp(0, 255) as u8;
        sprite.mono = obj.mono.clamp(0, 255) as u8;
        sprite.reverse = obj.reverse.clamp(0, 255) as u8;
        sprite.bright = obj.bright.clamp(0, 255) as u8;
        sprite.dark = obj.dark.clamp(0, 255) as u8;
        sprite.color_rate = obj.color_rate.clamp(0, 255) as u8;
        sprite.color_add_r = obj.color_add_r.clamp(0, 255) as u8;
        sprite.color_add_g = obj.color_add_g.clamp(0, 255) as u8;
        sprite.color_add_b = obj.color_add_b.clamp(0, 255) as u8;
        sprite.color_r = obj.color_r.clamp(0, 255) as u8;
        sprite.color_g = obj.color_g.clamp(0, 255) as u8;
        sprite.color_b = obj.color_b.clamp(0, 255) as u8;
        sprite.blend = SpriteBlend::from_i64(obj.blend);
        sprite.light_no = obj.light_no as i32;
        sprite.fog_use = obj.fog_use != 0;

        // Order: stage layer_no is treated as a coarse z, order as fine z.
        let coarse = obj.layer_no.clamp(-10000, 10000) as i32;
        let fine = obj.order.clamp(-100000, 100000) as i32;
        sprite.order = coarse.saturating_mul(1000).saturating_add(fine);

        if obj.is_mesh {
            sprite.image_id = None;
            sprite.mesh_file_name = obj.file.clone();
            sprite.mesh_kind = 1;
            sprite.camera_enabled = true;
            sprite.shadow_cast = true;
            sprite.shadow_receive = true;
            sprite.object_anchor = false;
            sprite.texture_center_x = 0.0;
            sprite.texture_center_y = 0.0;
            let (file, patno) = (obj.file.clone(), obj.patno);
            let state = self.ensure_object_mut(stage, obj_idx);
            state.bound_file = file;
            state.bound_patno = patno;
            return Ok(());
        }

        let Some(file) = obj.file.as_deref() else {
            return Ok(());
        };

        // C_elm_object owns a loaded C_d3d_album independently from its mutable
        // render parameters. X/Y/TR/ROTATE/etc. writes therefore must not run
        // resource lookup again. Only CREATE/CHANGE_FILE or a PATNO transition
        // changes the texture binding. For PATNO, select another cut directly
        // from the already-loaded album, matching m_album->get_texture(pat_no).
        let same_file = obj.bound_file.as_deref() == Some(file);
        if same_file && obj.bound_patno == obj.patno && sprite.image_id.is_some() {
            return Ok(());
        }

        let next_image = if same_file {
            sprite.image_id.as_ref().and_then(|image| {
                usize::try_from(obj.patno)
                    .ok()
                    .and_then(|cut| image.album_cut(cut))
            })
        } else {
            None
        };

        let image_id = match next_image {
            Some(image_id) => image_id,
            None => match Self::load_any_image(images, file, obj.patno) {
                Ok(image_id) => image_id,
                Err(err) if is_probable_mesh_path(file) => {
                    let _ = err;
                    sprite.image_id = None;
                    sprite.object_anchor = false;
                    sprite.texture_center_x = 0.0;
                    sprite.texture_center_y = 0.0;
                    return Ok(());
                }
                Err(err) => return Err(err),
            },
        };
        set_object_sprite_image(sprite, images, image_id);
        let (file, patno) = (file.to_string(), obj.patno);
        let state = self.ensure_object_mut(stage, obj_idx);
        state.bound_file = Some(file);
        state.bound_patno = patno;

        Ok(())
    }

    pub fn object_set_center(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        x: i64,
        y: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.center_x = x;
            obj.center_y = y;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_scale(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        x: i64,
        y: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.scale_x = x;
            obj.scale_y = y;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_rotate(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        z: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.rotate_z = z;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_clip(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        use_flag: i64,
        left: i64,
        top: i64,
        right: i64,
        bottom: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.clip_use = use_flag;
            obj.clip_left = left;
            obj.clip_top = top;
            obj.clip_right = right;
            obj.clip_bottom = bottom;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_src_clip(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        use_flag: i64,
        left: i64,
        top: i64,
        right: i64,
        bottom: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.src_clip_use = use_flag;
            obj.src_clip_left = left;
            obj.src_clip_top = top;
            obj.src_clip_right = right;
            obj.src_clip_bottom = bottom;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn stage_clear(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let mut indices: Vec<usize> = self.stages[stage_u].objects.keys().copied().collect();
        indices.sort_unstable();
        for idx in indices {
            {
                let obj = self.ensure_object_mut(stage_u, idx);
                obj.disp = false;
            }
            let _ = self.sync_object_sprite(images, layers, stage_u, idx);
        }
        Ok(())
    }

    fn object_create_impl(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        file: &str,
        disp: i64,
        x: i64,
        y: i64,
        patno: i64,
        reinit: bool,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) {
            bail!("invalid stage: {stage}");
        }
        if obj_idx < 0 {
            bail!("invalid obj idx: {obj_idx}");
        }

        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let current_layer = self.current_layer;

        if reinit {
            self.reset_object_for_create(layers, stage_u, obj_u);
        }

        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.is_mesh = false;
            obj.file = Some(file.to_string());
            obj.patno = patno;
            obj.disp = disp != 0;
            obj.x = x;
            obj.y = y;

            // Default layer number from current selection (can be overridden by scripts).
            if obj.layer_no == 0 {
                obj.layer_no = current_layer as i64;
            }
        }

        // C++ C_elm_object has no special BACK.OBJECT[0] background slot: every
        // picture object participates in the normal stage sprite tree.
        let _ = self.ensure_bound_sprite(layers, stage_u, obj_u)?;

        if cgm_file_interesting(Some(file)) || (30..=59).contains(&obj_u) {
            let obj = self.object(stage_u, obj_u);
            sg_cgm_coord_trace!(
                "object_create stage={} obj={} file={} disp={} x={} y={} patno={} reinit={} layer_no={:?} binding={:?}/{:?}",
                stage,
                obj_idx,
                file,
                disp,
                x,
                y,
                patno,
                reinit,
                obj.map(|o| o.layer_no),
                obj.and_then(|o| o.layer_id),
                obj.and_then(|o| o.sprite_id)
            );
        }

        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_create(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        file: &str,
        disp: i64,
        x: i64,
        y: i64,
        patno: i64,
    ) -> Result<()> {
        self.object_create_impl(
            images, layers, stage, obj_idx, file, disp, x, y, patno, true,
        )
    }

    pub fn object_create_billboard(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        file: &str,
        disp: i64,
        x: i64,
        y: i64,
        patno: i64,
    ) -> Result<()> {
        let result = self.object_create_impl(
            images, layers, stage, obj_idx, file, disp, x, y, patno, true,
        );
        if let Some((lid, sid)) = self.object_sprite_binding(stage, obj_idx)
            && let Some(sprite) = layers
                .layer_mut(lid)
                .and_then(|layer| layer.sprite_mut(sid))
        {
            sprite.billboard = true;
            sprite.camera_enabled = true;
        }
        result
    }

    /// Mirror of C++ `C_elm_object::restruct_pct` (and the `restruct_type`
    /// dispatch around it) executed at the tail of `C_elm_object::load`. After
    /// a save-file load, the per-object gfx runtime (sprite binding, image
    /// asset, transform/color state) is empty - the saved stream restores
    /// `globals::ObjectState` but the rendering side has no equivalent storage
    /// in the save format. This rebuilds the gfx side from the loaded globals
    /// so the next render frame sees the same picture the save captured.
    ///
    /// Caller filters: only invoke for Gfx-backed objects whose `file_name` is
    /// non-empty; everything else (mesh, movie, weather, number, string) needs
    /// its own backend-specific path and is no-op here.
    pub fn restore_gfx_object_from_globals(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        src: &crate::runtime::globals::ObjectState,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let file = match src.file_name.as_deref() {
            Some(f) if !f.is_empty() => f.to_string(),
            _ => return Ok(()),
        };
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;

        let current_layer = self.current_layer;
        self.reset_object_for_create(layers, stage_u, obj_u);
        {
            let pe = &src.runtime.prop_events;
            let dst = self.ensure_object_mut(stage_u, obj_u);
            dst.is_mesh = false;
            dst.file = Some(file);
            dst.patno = src.base.patno;
            dst.disp = src.base.disp != 0;
            dst.x = pe.x.get_total_value() as i64;
            dst.y = pe.y.get_total_value() as i64;
            dst.z = pe.z.get_total_value() as i64;
            dst.layer_no = src.base.layer;
            dst.order = src.base.order;
            dst.center_x = pe.center_x.get_total_value() as i64;
            dst.center_y = pe.center_y.get_total_value() as i64;
            dst.scale_x = pe.scale_x.get_total_value() as i64;
            dst.scale_y = pe.scale_y.get_total_value() as i64;
            dst.rotate_z = pe.rotate_z.get_total_value() as i64;
            dst.clip_use = src.base.clip_use;
            dst.clip_left = pe.clip_left.get_total_value() as i64;
            dst.clip_top = pe.clip_top.get_total_value() as i64;
            dst.clip_right = pe.clip_right.get_total_value() as i64;
            dst.clip_bottom = pe.clip_bottom.get_total_value() as i64;
            dst.src_clip_use = src.base.src_clip_use;
            dst.src_clip_left = pe.src_clip_left.get_total_value() as i64;
            dst.src_clip_top = pe.src_clip_top.get_total_value() as i64;
            dst.src_clip_right = pe.src_clip_right.get_total_value() as i64;
            dst.src_clip_bottom = pe.src_clip_bottom.get_total_value() as i64;
            let tr = pe.tr.get_total_value() as i64;
            dst.alpha = tr;
            dst.tr = tr;
            dst.mono = pe.mono.get_total_value() as i64;
            dst.reverse = pe.reverse.get_total_value() as i64;
            dst.bright = pe.bright.get_total_value() as i64;
            dst.dark = pe.dark.get_total_value() as i64;
            dst.color_rate = pe.color_rate.get_total_value() as i64;
            dst.color_add_r = pe.color_add_r.get_total_value() as i64;
            dst.color_add_g = pe.color_add_g.get_total_value() as i64;
            dst.color_add_b = pe.color_add_b.get_total_value() as i64;
            dst.color_r = pe.color_r.get_total_value() as i64;
            dst.color_g = pe.color_g.get_total_value() as i64;
            dst.color_b = pe.color_b.get_total_value() as i64;
            dst.blend = src.base.blend;
            dst.light_no = src.base.light_no;
            dst.fog_use = src.base.fog_use;
            if dst.layer_no == 0 {
                dst.layer_no = current_layer as i64;
            }
        }

        let _ = self.ensure_bound_sprite(layers, stage_u, obj_u)?;
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    /// Release only the type-owned visual resource while preserving the
    /// object's render parameters. This mirrors C_elm_object::free_type(false):
    /// CHANGE_FILE must not reset position/color/alpha/layer/order/etc.
    pub fn object_release_type_backing(
        &mut self,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let (layer_id, sprite_id) = {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.file = None;
            obj.is_mesh = false;
            obj.bound_file = None;
            obj.bound_patno = -1;
            (obj.layer_id, obj.sprite_id)
        };

        if let (Some(lid), Some(sid)) = (layer_id, sprite_id)
            && let Some(sprite) = layers
                .layer_mut(lid)
                .and_then(|layer| layer.sprite_mut(sid))
        {
            sprite.image_id = None;
            sprite.mesh_file_name = None;
            sprite.mesh_kind = 0;
            sprite.billboard = false;
            sprite.emote_render = None;
        }
        Ok(())
    }

    pub fn object_change_file(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        file: &str,
        disp: i64,
        x: i64,
        y: i64,
        patno: i64,
    ) -> Result<()> {
        self.object_create_impl(
            images, layers, stage, obj_idx, file, disp, x, y, patno, false,
        )
    }

    pub fn object_create_mesh(
        &mut self,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        file: &str,
        disp: i64,
        x: i64,
        y: i64,
        patno: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) {
            bail!("invalid stage: {stage}");
        }
        if obj_idx < 0 {
            bail!("invalid obj idx: {obj_idx}");
        }

        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let current_layer = self.current_layer;

        self.reset_object_for_create(layers, stage_u, obj_u);

        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.is_mesh = true;
            obj.file = Some(file.to_string());
            obj.patno = patno;
            obj.disp = disp != 0;
            obj.x = x;
            obj.y = y;
            if obj.layer_no == 0 {
                obj.layer_no = current_layer as i64;
            }
        }

        if cgm_file_interesting(Some(file)) || (30..=59).contains(&obj_u) {
            sg_cgm_coord_trace!(
                "object_create_mesh stage={} obj={} file={} disp={} x={} y={} patno={}",
                stage,
                obj_idx,
                file,
                disp,
                x,
                y,
                patno
            );
        }

        let (lid, sid) = self.ensure_bound_sprite(layers, stage_u, obj_u)?;
        let sprite = layers
            .layer_mut(lid)
            .and_then(|l| l.sprite_mut(sid))
            .context("mesh sprite not found")?;
        sprite.visible = disp != 0;
        sprite.image_id = None;
        sprite.x = x as i32;
        sprite.y = y as i32;
        sprite.alpha = 255;
        sprite.tr = 255;
        sprite.fit = SpriteFit::PixelRect;
        sprite.size_mode = SpriteSizeMode::Intrinsic;
        sprite.mesh_file_name = Some(file.to_string());
        sprite.mesh_kind = 1;
        sprite.camera_enabled = true;
        sprite.shadow_cast = true;
        sprite.shadow_receive = true;

        Ok(())
    }

    /// Mesh counterpart of object_change_file(): rebuild the mesh backing
    /// without resetting render parameters that survive free_type(false).
    pub fn object_change_mesh_file(
        &mut self,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        file: &str,
        disp: i64,
        x: i64,
        y: i64,
        patno: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) {
            bail!("invalid stage: {stage}");
        }
        if obj_idx < 0 {
            bail!("invalid obj idx: {obj_idx}");
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let current_layer = self.current_layer;

        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.is_mesh = true;
            obj.file = Some(file.to_string());
            obj.patno = patno;
            obj.disp = disp != 0;
            obj.x = x;
            obj.y = y;
            if obj.layer_no == 0 {
                obj.layer_no = current_layer as i64;
            }
        }

        let (lid, sid) = self.ensure_bound_sprite(layers, stage_u, obj_u)?;
        let sprite = layers
            .layer_mut(lid)
            .and_then(|l| l.sprite_mut(sid))
            .context("mesh sprite not found")?;
        sprite.visible = disp != 0;
        sprite.image_id = None;
        sprite.x = x as i32;
        sprite.y = y as i32;
        sprite.fit = SpriteFit::PixelRect;
        sprite.size_mode = SpriteSizeMode::Intrinsic;
        sprite.mesh_file_name = Some(file.to_string());
        sprite.mesh_kind = 1;
        sprite.billboard = false;
        sprite.camera_enabled = true;
        sprite.shadow_cast = true;
        sprite.shadow_receive = true;
        Ok(())
    }

    pub fn object_set_disp(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        disp: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.disp = disp != 0;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_pos(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        x: i64,
        y: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let current_layer = self.current_layer;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.x = x;
            obj.y = y;
            if obj.layer_no == 0 {
                obj.layer_no = current_layer as i64;
            }
        }
        if let Some(obj) = self.object(stage_u, obj_u)
            && (cgm_file_interesting(obj.file.as_deref()) || (30..=59).contains(&obj_u))
        {
            sg_cgm_coord_trace!(
                "object_set_pos stage={} obj={} file={:?} x={} y={} layer_no={} binding={:?}/{:?}",
                stage,
                obj_idx,
                obj.file.as_deref(),
                x,
                y,
                obj.layer_no,
                obj.layer_id,
                obj.sprite_id
            );
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_x(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        x: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.x = x;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_y(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        y: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.y = y;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_patno(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        patno: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.patno = patno;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_layer(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        layer_no: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.layer_no = layer_no;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_order(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        order: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.order = order;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_alpha(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        alpha: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.alpha = alpha;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_tr(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        tr: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.tr = tr;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_mono(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        mono: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.mono = mono;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_reverse(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        reverse: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.reverse = reverse;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_bright(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        bright: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.bright = bright;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_dark(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        dark: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.dark = dark;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_color_rate(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        rate: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.color_rate = rate;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_color_add(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        r: i64,
        g: i64,
        b: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.color_add_r = r;
            obj.color_add_g = g;
            obj.color_add_b = b;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_color(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        r: i64,
        g: i64,
        b: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.color_r = r;
            obj.color_g = g;
            obj.color_b = b;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_blend(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        blend: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.blend = blend;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_light_no(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        light_no: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.light_no = light_no;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_fog_use(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        fog_use: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.fog_use = fog_use;
        }
        self.sync_object_sprite(images, layers, stage_u, obj_u)
    }

    pub fn object_set_z(&mut self, stage: i64, obj_idx: i64, z: i64) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.ensure_object_mut(stage_u, obj_u);
        obj.z = z;
        Ok(())
    }

    pub fn object_clear(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
    ) -> Result<()> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return Ok(());
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let (layer_id, sprite_id) = {
            let obj = self.ensure_object_mut(stage_u, obj_u);
            obj.file = None;
            obj.patno = 0;
            obj.bound_file = None;
            obj.bound_patno = -1;
            obj.disp = false;
            obj.alpha = 255;
            (obj.layer_id, obj.sprite_id)
        };

        // free_type()/restruct_pct() in the original engine drops the album on
        // failure/free. Merely hiding our sprite is insufficient: retaining an
        // old image_id lets a later DISP write resurrect stale pixels even
        // though the logical object has no file/album anymore.
        if let (Some(lid), Some(sid)) = (layer_id, sprite_id)
            && let Some(sprite) = layers
                .layer_mut(lid)
                .and_then(|layer| layer.sprite_mut(sid))
        {
            *sprite = Sprite::default();
        }

        // Kept in the signature because object_clear is paired with the other
        // image-backed Gfx operations and callers already pass ImageManager.
        let _ = images;
        Ok(())
    }

    pub fn clear_objects_in_layer_no(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        layer_no: i64,
    ) -> Result<()> {
        for stage in 0..3usize {
            let mut indices: Vec<usize> = self.stages[stage].objects.keys().copied().collect();
            indices.sort_unstable();
            for obj_idx in indices {
                let matches = self
                    .object(stage, obj_idx)
                    .map(|o| o.layer_no == layer_no)
                    .unwrap_or(false);
                if !matches {
                    continue;
                }
                {
                    let obj = self.ensure_object_mut(stage, obj_idx);
                    obj.disp = false;
                }
                let _ = self.sync_object_sprite(images, layers, stage, obj_idx);
            }
        }
        Ok(())
    }

    pub fn object_get_pos(&self, stage: i64, obj_idx: i64) -> Option<(i64, i64)> {
        self.object_peek_pos(stage, obj_idx)
    }

    pub fn object_get_disp(&self, stage: i64, obj_idx: i64) -> Option<bool> {
        self.object_peek_disp(stage, obj_idx).map(|v| v != 0)
    }

    pub fn object_get_patno(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        self.object_peek_patno(stage, obj_idx)
    }

    pub fn object_get_layer(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        self.object_peek_layer(stage, obj_idx)
    }

    pub fn object_get_order(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        self.object_peek_order(stage, obj_idx)
    }

    pub fn object_get_alpha(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        self.object_peek_alpha(stage, obj_idx)
    }

    pub fn object_set_pat_no(
        &mut self,
        images: &mut ImageManager,
        layers: &mut LayerManager,
        stage: i64,
        obj_idx: i64,
        patno: i64,
    ) -> Result<()> {
        self.object_set_patno(images, layers, stage, obj_idx, patno)
    }

    pub fn object_peek_pos(&self, stage: i64, obj_idx: i64) -> Option<(i64, i64)> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.object(stage_u, obj_u)?;
        Some((obj.x, obj.y))
    }

    pub fn object_peek_disp(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.object(stage_u, obj_u)?;
        Some(if obj.disp { 1 } else { 0 })
    }

    pub fn object_peek_patno(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.object(stage_u, obj_u)?;
        Some(obj.patno)
    }

    pub fn object_peek_layer(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.object(stage_u, obj_u)?;
        Some(obj.layer_no)
    }

    pub fn object_peek_order(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.object(stage_u, obj_u)?;
        Some(obj.order)
    }

    pub fn object_peek_alpha(&self, stage: i64, obj_idx: i64) -> Option<i64> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.object(stage_u, obj_u)?;
        Some(obj.alpha)
    }

    pub fn object_peek_file(&self, stage: i64, obj_idx: i64) -> Option<String> {
        let stage_i = stage as isize;
        if !(0..3).contains(&stage_i) || obj_idx < 0 {
            return None;
        }
        let stage_u = stage_i as usize;
        let obj_u = obj_idx as usize;
        let obj = self.object(stage_u, obj_u)?;
        obj.file.clone()
    }
}

fn is_probable_mesh_path(file: &str) -> bool {
    let lower = file.to_ascii_lowercase();
    lower.ends_with(".x")
        || lower.ends_with(".obj")
        || lower.ends_with(".fbx")
        || lower.ends_with(".gltf")
        || lower.ends_with(".glb")
}

fn set_object_sprite_image(sprite: &mut Sprite, images: &ImageManager, image_id: ImageHandle) {
    sprite.image_id = Some(image_id.clone());
    if let Some(img) = images.get(&image_id) {
        sprite.object_anchor = true;
        sprite.texture_center_x = img.center_x as f32;
        sprite.texture_center_y = img.center_y as f32;
    } else {
        sprite.object_anchor = false;
        sprite.texture_center_x = 0.0;
        sprite.texture_center_y = 0.0;
    }
}

fn clip_rect(use_flag: i64, left: i64, top: i64, right: i64, bottom: i64) -> Option<ClipRect> {
    if use_flag == 0 {
        return None;
    }
    Some(ClipRect {
        left: left as i32,
        top: top as i32,
        right: right as i32,
        bottom: bottom as i32,
    })
}

#[cfg(test)]
mod tests {
    use super::GfxRuntime;
    use crate::layer::LayerManager;

    #[test]
    fn front_object_zero_mesh_gets_an_ordinary_sprite_binding() {
        let mut gfx = GfxRuntime::new();
        let mut layers = LayerManager::new();

        gfx.object_create_mesh(&mut layers, 1, 0, "room.x", 1, 10, 20, 0)
            .unwrap();

        let (layer_id, sprite_id) = gfx.object_sprite_binding(1, 0).unwrap();
        let sprite = layers.layer(layer_id).unwrap().sprite(sprite_id).unwrap();
        assert_eq!(sprite.mesh_kind, 1);
        assert_eq!(sprite.mesh_file_name.as_deref(), Some("room.x"));
        assert!(!gfx.has_missing_bound_object_image(&layers));
    }

    #[test]
    fn back_object_zero_pct_gets_an_ordinary_sprite_binding() {
        let project_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut gfx = GfxRuntime::new();
        let mut images = crate::image_manager::ImageManager::new(project_dir);
        let mut layers = LayerManager::new();

        // The image load is expected to fail for this synthetic path. The
        // important invariant is established first: BACK.OBJECT[0] is a normal
        // stage object and owns a stable sprite just like every other OBJECT.
        let _ = gfx.object_create(&mut images, &mut layers, 0, 0, "missing.g00", 1, 10, 20, 0);

        let (layer_id, sprite_id) = gfx.object_sprite_binding(0, 0).unwrap();
        let sprite = layers.layer(layer_id).unwrap().sprite(sprite_id).unwrap();
        assert!(!gfx.debug_object_snapshot(0, 0).unwrap().is_bg);
        assert!(!sprite.billboard);
    }

    #[test]
    fn back_object_zero_billboard_is_not_misclassified_as_background() {
        let project_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut gfx = GfxRuntime::new();
        let mut images = crate::image_manager::ImageManager::new(project_dir);
        let mut layers = LayerManager::new();

        // The image load is expected to fail for this synthetic path, but the
        // classification and binding happen before that error is returned.
        let _ = gfx.object_create_billboard(
            &mut images,
            &mut layers,
            0,
            0,
            "missing.g00",
            1,
            10,
            20,
            0,
        );

        let (layer_id, sprite_id) = gfx.object_sprite_binding(0, 0).unwrap();
        let sprite = layers.layer(layer_id).unwrap().sprite(sprite_id).unwrap();
        assert!(!gfx.debug_object_snapshot(0, 0).unwrap().is_bg);
        assert!(sprite.billboard);
    }
}
