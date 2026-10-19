//! The graphics system: device contexts (DCs), object layers, the image
//! cache and screen composition.
//!
//! DCs are copy-on-write (`Rc<Surface>`), so the state at a savepoint is
//! captured by cloning the handles; saves store the pixels themselves
//! rather than replaying drawing commands.

use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{Context, Result, anyhow};

use crate::font::FontSet;
use crate::gameexe::Gameexe;
use crate::image::Image;
use crate::object::{OBJECT_COUNT, Object, ObjectData, RenderContext};
use crate::resource::{Kind, Resources};
use crate::serial::{Reader, Writer};
use crate::surface::{Rect, Surface};

pub const DC_COUNT: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMode {
    Auto,
    SemiAuto,
    Manual,
}

/// `#OBJECT.nnn = layer, space_key, obj_on_off, time_mod, disp_sort,
/// init_mod, weather_on_off`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ObjectSettings {
    pub layer: i32,
    pub space_key: i32,
    pub obj_on_off: i32,
    pub time_mod: i32,
    pub disp_sort: i32,
    pub init_mod: i32,
    pub weather_on_off: i32,
}

impl ObjectSettings {
    fn from_values(values: &[i32]) -> Self {
        let get = |i: usize| values.get(i).copied().unwrap_or(0);
        Self {
            layer: get(0),
            space_key: get(1),
            obj_on_off: get(2),
            time_mod: get(3),
            disp_sort: get(4),
            init_mod: get(5),
            weather_on_off: get(6),
        }
    }
}

/// Which object slot an operation addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectRef {
    /// false = foreground, true = background.
    pub bg: bool,
    pub buf: i32,
    /// A child of `buf` (modtype 2 functions).
    pub child: Option<i32>,
}

impl ObjectRef {
    pub fn fg(buf: i32) -> Self {
        Self {
            bg: false,
            buf,
            child: None,
        }
    }
}

/// A flash of colour over the screen or part of it.
/// A background scroll to (x, y) (either may stay) over `time` ms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HaikeiMove {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub time: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HaikeiScroll {
    pub from: (i32, i32),
    pub to: (i32, i32),
    pub start: u64,
    pub time: u64,
}

impl HaikeiScroll {
    pub fn start(from: (i32, i32), m: HaikeiMove, now: u64) -> Self {
        Self {
            from,
            to: (m.x.unwrap_or(from.0), m.y.unwrap_or(from.1)),
            start: now,
            time: m.time,
        }
    }

    pub fn at(&self, now: u64) -> (i32, i32) {
        if self.time == 0 || now >= self.start + self.time {
            return self.to;
        }
        let t = (now - self.start) as f64 / self.time as f64;
        let lerp = |a: i32, b: i32| a + (f64::from(b - a) * t).round() as i32;
        (lerp(self.from.0, self.to.0), lerp(self.from.1, self.to.1))
    }

    pub fn finished(&self, now: u64) -> bool {
        now >= self.start + self.time
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flash {
    pub area: Option<Rect>,
    pub colour: [u8; 3],
    pub start: u64,
    /// Length of one flash, in ms.
    pub time: u64,
    /// How many times it flashes.
    pub count: u32,
    /// Fades out instead of cutting off.
    pub fade: bool,
    /// Started by a `*PIKACHU*` (blink) command.
    pub blink: bool,
}

impl Flash {
    pub fn finished(&self, now: u64) -> bool {
        now >= self.start + self.time * u64::from(self.count.max(1))
    }

    /// The opacity at `now` (0 when off).
    pub fn alpha(&self, now: u64) -> u8 {
        if now < self.start || self.finished(now) || self.time == 0 {
            return 0;
        }
        let t = (now - self.start) % self.time;
        let blinking = self.count > 1;
        // Blinks are on for the first half of each period.
        let on = if blinking { self.time / 2 } else { self.time };
        if t >= on.max(1) {
            return 0;
        }
        if self.fade {
            (255.0 * (1.0 - t as f64 / on.max(1) as f64)) as u8
        } else {
            255
        }
    }
}

/// Graphics state captured at a savepoint.
#[derive(Debug, Clone, Default)]
pub struct GraphicsSnapshot {
    pub dcs: Vec<Option<Rc<Surface>>>,
    pub fg: Vec<Option<Object>>,
    pub bg: Vec<Option<Object>>,
    pub stack: Vec<String>,
    /// The HIK background: (file, position).
    pub hik: Option<(String, (i32, i32))>,
}

#[derive(Debug)]
pub struct Graphics {
    pub width: i32,
    pub height: i32,
    pub dcs: Vec<Option<Rc<Surface>>>,
    /// `grpLoadMask` buffers.
    pub masks: HashMap<i32, Rc<Surface>>,
    pub fg: Vec<Option<Object>>,
    pub bg: Vec<Option<Object>>,
    pub images: HashMap<String, Rc<Image>>,
    /// `#COLOR_TABLE`.
    pub colours: Vec<[u8; 3]>,
    pub object_settings: HashMap<i32, ObjectSettings>,
    pub default_object_settings: ObjectSettings,
    pub draw_mode: DrawMode,
    /// `refresh` was called (manual drawing mode shows a new frame).
    pub refresh_requested: bool,
    /// The screen needs redrawing.
    pub dirty: bool,
    /// Entries of the graphics stack (`stackSize` and friends).
    pub stack: Vec<String>,
    /// Whole-screen shake (`ShakeScreen`, `ShakeSpec`, `shake`).
    pub screen_shake: Option<crate::shake::Shake>,
    /// Layer shake (`ShakeLayers*`).
    pub layer_shake: Option<crate::shake::Shake>,
    /// While a transition runs, the frame to show instead of composing.
    pub transition_frame: Option<Rc<Surface>>,
    /// `ShowBackground` / space key: hide the interface.
    pub interface_hidden: bool,
    /// `CCOM_OBJECT_OFF`: foreground objects hidden while a system menu
    /// is up.
    pub ccom_hidden_objects: std::collections::BTreeSet<i32>,
    /// `CCOM_BTNSEL_OFF`: button selections hidden while a menu is up.
    pub ccom_hide_buttons: bool,
    /// `OBJFRONTANM_PAUSE`: foreground objects whose pattern animation is
    /// frozen, with the time it stopped at.
    pub paused_animations: HashMap<usize, u64>,
    /// `*GANANM_NEXT_*`: animations to start when the running one ends
    /// (a looping one at the end of its cycle): (object, when, animation).
    pub queued_animations: Vec<(ObjectRef, Option<u64>, crate::object::Animation)>,
    /// `#TONECURVE_FILENAME` and which parts use which table
    /// (`TONECURVE_*`).
    pub tone_curves: crate::tone_curve::ToneCurves,
    pub object_tones: HashMap<usize, usize>,
    pub all_objects_tone: Option<usize>,
    pub background_tone: Option<usize>,
    pub foreground_tone: Option<usize>,
    pub face_tone: Option<usize>,
    /// An animated HIK background shown over DC 0.
    pub hik: Option<crate::hik::HikRenderer>,
    /// `HAIKEI_SET_POS`: the background's scroll position.
    pub haikei_pos: (i32, i32),
    /// The picture `bgrLoadHaikei` showed (redrawn when it scrolls).
    pub haikei_image: Option<Rc<Image>>,
    /// `HAIKEI_SCROLL_POS*`: the running scroll, those queued after it
    /// (`_NEXT`) and those waiting for `_READY_SYNC`.
    pub haikei_scroll: Option<HaikeiScroll>,
    pub haikei_queue: Vec<HaikeiMove>,
    pub haikei_ready: Vec<HaikeiMove>,
    /// A screen flash (`BOXFLUSH` and friends, module 1:41).
    pub flash: Option<Flash>,
    /// `CAPTURE*` without a bank: the last screen capture.
    pub capture: Option<Rc<Surface>>,
    /// `MAKE_THUMBNAIL`: the picture the next save keeps.
    pub thumbnail: Option<Rc<Surface>>,
    /// `SET_SCREENZOOM*`: the screen magnified by a percentage around a
    /// point.
    pub screen_zoom: Option<(i32, i32, i32)>,
    /// `G00BUF_LOAD`: images kept in the cache, by buffer number.
    pub preloaded: HashMap<i32, (String, Rc<Image>)>,
    /// Serial animations drawn onto DC 0 (module 1:34).
    pub snm: crate::serial_pdt::SerialPdts,
    pub fonts: FontSet,
    pub snapshot: GraphicsSnapshot,
    /// Names of images loaded since the last query (CG tracking).
    pub loaded_images: Vec<String>,
}

impl Graphics {
    pub fn new(gameexe: &Gameexe, fonts: FontSet) -> Self {
        let (width, height) = screen_size(gameexe);
        let mut colours = vec![[255, 255, 255]; 256];
        for entry in gameexe.filter("COLOR_TABLE.") {
            let values = entry.ints();
            if let (Some(index), [r, g, b, ..]) = (entry.key_number(1), &values[..]) {
                if let Some(slot) = colours.get_mut(index.max(0) as usize) {
                    *slot = [*r, *g, *b].map(|c| c.clamp(0, 255) as u8);
                }
            }
        }
        let mut object_settings = HashMap::new();
        let default_object_settings = gameexe
            .get("OBJECT.999")
            .map(|entry| ObjectSettings::from_values(&entry.ints()))
            .unwrap_or_default();
        for entry in gameexe.filter("OBJECT.") {
            let parts = entry.key_parts();
            let Some(range) = parts.get(1) else {
                continue;
            };
            // `OBJECT.nnn` or `OBJECT.nnn:mmm` ranges.
            let bounds: Vec<i32> = range
                .split([':', '-'])
                .filter_map(|part| part.trim().parse().ok())
                .collect();
            let settings = ObjectSettings::from_values(&entry.ints());
            match bounds[..] {
                [single] => {
                    object_settings.insert(single, settings);
                }
                [from, to] => {
                    for index in from..=to {
                        object_settings.insert(index, settings);
                    }
                }
                _ => {}
            }
        }
        let mut dcs = vec![None; DC_COUNT];
        dcs[0] = Some(Rc::new(Surface::new(width, height)));
        dcs[1] = Some(Rc::new(Surface::new(width, height)));
        Self {
            width,
            height,
            dcs,
            masks: HashMap::new(),
            fg: vec![None; OBJECT_COUNT],
            bg: vec![None; OBJECT_COUNT],
            images: HashMap::new(),
            colours,
            object_settings,
            default_object_settings,
            draw_mode: DrawMode::Auto,
            refresh_requested: false,
            dirty: true,
            stack: Vec::new(),
            screen_shake: None,
            layer_shake: None,
            transition_frame: None,
            interface_hidden: false,
            ccom_hidden_objects: Default::default(),
            ccom_hide_buttons: false,
            paused_animations: HashMap::new(),
            queued_animations: Vec::new(),
            preloaded: HashMap::new(),
            capture: None,
            flash: None,
            hik: None,
            haikei_pos: (0, 0),
            tone_curves: Default::default(),
            object_tones: HashMap::new(),
            all_objects_tone: None,
            background_tone: None,
            foreground_tone: None,
            face_tone: None,
            fonts,
            snapshot: GraphicsSnapshot::default(),
            snm: Default::default(),
            thumbnail: None,
            haikei_image: None,
            haikei_scroll: None,
            haikei_queue: Vec::new(),
            haikei_ready: Vec::new(),
            screen_zoom: None,
            loaded_images: Vec::new(),
        }
    }

    pub fn screen_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    // ---- DCs --------------------------------------------------------------

    fn dc_index(dc: i32) -> Result<usize> {
        usize::try_from(dc)
            .ok()
            .filter(|&index| index < DC_COUNT)
            .ok_or_else(|| anyhow!("reallive: invalid DC {dc}"))
    }

    /// A DC, allocated at screen size if it was empty.
    pub fn dc(&mut self, dc: i32) -> Result<Rc<Surface>> {
        let index = Self::dc_index(dc)?;
        let (width, height) = (self.width, self.height);
        Ok(self.dcs[index]
            .get_or_insert_with(|| Rc::new(Surface::new(width, height)))
            .clone())
    }

    pub fn dc_mut(&mut self, dc: i32) -> Result<&mut Surface> {
        let index = Self::dc_index(dc)?;
        let (width, height) = (self.width, self.height);
        if index == 0 {
            self.dirty = true;
        }
        let slot = self.dcs[index].get_or_insert_with(|| Rc::new(Surface::new(width, height)));
        Ok(Rc::make_mut(slot))
    }

    pub fn set_dc(&mut self, dc: i32, surface: Surface) -> Result<()> {
        let index = Self::dc_index(dc)?;
        if index == 0 {
            self.dirty = true;
        }
        self.dcs[index] = Some(Rc::new(surface));
        Ok(())
    }

    /// `allocDC(dc, w, h)`; DC 1 is never smaller than the screen.
    pub fn alloc_dc(&mut self, dc: i32, width: i32, height: i32) -> Result<()> {
        let (mut width, mut height) = (width.max(1), height.max(1));
        if dc == 1 {
            width = width.max(self.width);
            height = height.max(self.height);
        }
        if dc == 0 {
            return Ok(());
        }
        self.set_dc(dc, Surface::new(width, height))
    }

    /// `freeDC(dc)`: DC 1 is blanked instead.
    pub fn free_dc(&mut self, dc: i32) -> Result<()> {
        let index = Self::dc_index(dc)?;
        match index {
            0 => {}
            1 => {
                let (w, h) = (self.width, self.height);
                self.dcs[1] = Some(Rc::new(Surface::new(w, h)));
            }
            _ => self.dcs[index] = None,
        }
        Ok(())
    }

    /// Grows a DC to at least `width` x `height`, keeping its pixels.
    pub fn ensure_dc_size(&mut self, dc: i32, width: i32, height: i32) -> Result<()> {
        let current = self.dc(dc)?;
        if current.width >= width && current.height >= height {
            return Ok(());
        }
        let mut grown = Surface::new(current.width.max(width), current.height.max(height));
        grown.blit(
            &current,
            current.rect(),
            0,
            0,
            255,
            crate::surface::Blend::Copy,
            None,
        );
        self.set_dc(dc, grown)
    }

    // ---- images -----------------------------------------------------------

    /// Loads (and caches) a bitmap by script name.
    pub fn load_image(&mut self, resources: &Resources, name: &str) -> Result<Rc<Image>> {
        let key = name.trim().to_lowercase();
        if let Some(image) = self.images.get(&key) {
            self.loaded_images.push(name.to_owned());
            return Ok(image.clone());
        }
        let bytes = resources
            .read(Kind::Image, name)
            .ok_or_else(|| anyhow!("reallive: image {name:?} was not found"))?;
        let image = Rc::new(
            crate::image::decode(&bytes).with_context(|| format!("decoding image {name:?}"))?,
        );
        if self.images.len() > 256 {
            self.images.clear();
            for (name, image) in self.preloaded.values() {
                self.images.insert(name.clone(), image.clone());
            }
        }
        self.images.insert(key, image.clone());
        self.loaded_images.push(name.to_owned());
        Ok(image)
    }

    // ---- objects ----------------------------------------------------------

    fn layer_mut(&mut self, bg: bool) -> &mut Vec<Option<Object>> {
        if bg { &mut self.bg } else { &mut self.fg }
    }

    pub fn object(&self, r: ObjectRef) -> Option<&Object> {
        let layer = if r.bg { &self.bg } else { &self.fg };
        let parent = layer.get(usize::try_from(r.buf).ok()?)?.as_ref()?;
        match r.child {
            None => Some(parent),
            Some(child) => match parent.data.as_ref()? {
                ObjectData::Parent(children) => {
                    children.get(usize::try_from(child).ok()?)?.as_ref()
                }
                _ => None,
            },
        }
    }

    /// The object at `r`, created empty if absent (a parent is created for
    /// child references).
    pub fn object_mut(&mut self, r: ObjectRef) -> Result<&mut Object> {
        let index = usize::try_from(r.buf)
            .ok()
            .filter(|&i| i < OBJECT_COUNT)
            .ok_or_else(|| anyhow!("reallive: invalid object {}", r.buf))?;
        self.dirty = true;
        let slot = &mut self.layer_mut(r.bg)[index];
        let parent = slot.get_or_insert_with(Object::empty);
        let Some(child) = r.child else {
            return Ok(parent);
        };
        let child = usize::try_from(child)
            .ok()
            .filter(|&i| i < OBJECT_COUNT)
            .ok_or_else(|| anyhow!("reallive: invalid child object {child}"))?;
        if !matches!(parent.data, Some(ObjectData::Parent(_))) {
            parent.data = Some(ObjectData::Parent(vec![None; OBJECT_COUNT]));
        }
        let Some(ObjectData::Parent(children)) = &mut parent.data else {
            unreachable!("just made a parent");
        };
        Ok(children[child].get_or_insert_with(Object::empty))
    }

    /// Replaces the data of an object, resetting its parameters.
    pub fn set_object(&mut self, r: ObjectRef, data: ObjectData) -> Result<&mut Object> {
        let object = self.object_mut(r)?;
        *object = Object::new(data);
        Ok(object)
    }

    /// Removes an object's data (`objDelete`) or everything (`objClear`).
    pub fn delete_object(&mut self, r: ObjectRef, clear_params: bool) -> Result<()> {
        let object = self.object_mut(r)?;
        object.data = None;
        object.animation = None;
        object.mutators.clear();
        if clear_params {
            object.params = Default::default();
        }
        Ok(())
    }

    /// Layer promotion (`grpOpenBg`, `grpDisplay`, ...): foreground objects
    /// without the wipe-copy flag are deleted and background objects move
    /// to the foreground.
    pub fn promote_objects(&mut self) {
        self.snm.promote();
        for index in 0..OBJECT_COUNT {
            let background = self.bg[index].take();
            match background {
                Some(object) if object.data.is_some() => self.fg[index] = Some(object),
                _ => {
                    if self.fg[index]
                        .as_ref()
                        .is_some_and(|object| !object.params.wipe_copy)
                    {
                        self.fg[index] = None;
                    }
                }
            }
        }
        self.dirty = true;
    }

    pub fn settings_for(&self, buf: usize) -> ObjectSettings {
        self.object_settings
            .get(&(buf as i32))
            .copied()
            .unwrap_or(self.default_object_settings)
    }

    /// Advances animations and mutators.
    /// Moves the background (`HAIKEI_*_POS`), redrawing a picture
    /// background on DC 0.
    pub fn set_haikei_pos(&mut self, pos: (i32, i32)) {
        if pos == self.haikei_pos && self.hik.is_none() {
            return;
        }
        self.haikei_pos = pos;
        if let Some(hik) = &mut self.hik {
            hik.offset = pos;
        }
        self.draw_haikei();
    }

    /// Draws the picture background onto DC 0 at the scroll position.
    pub fn draw_haikei(&mut self) {
        let Some(image) = self.haikei_image.clone() else {
            return;
        };
        let (x, y) = self.haikei_pos;
        if let Ok(dc0) = self.dc_mut(0) {
            let rect = dc0.rect();
            dc0.fill(rect, [0, 0, 0, 255], 255);
            let src = Rect::new(x, y, rect.w, rect.h).intersect(&image.surface.rect());
            dc0.blit(
                &image.surface,
                src,
                src.x - x,
                src.y - y,
                255,
                crate::surface::Blend::Mask,
                None,
            );
        }
    }

    pub fn update(&mut self, now: u64) {
        let mut changed = false;
        if self.hik.is_some() {
            changed = true;
        }
        if let Some(scroll) = self.haikei_scroll {
            self.set_haikei_pos(scroll.at(now));
            changed = true;
            if scroll.finished(now) {
                self.haikei_scroll = (!self.haikei_queue.is_empty())
                    .then(|| HaikeiScroll::start(scroll.to, self.haikei_queue.remove(0), now));
            }
        }
        if !self.snm.fg.is_empty() {
            let mut snm = std::mem::take(&mut self.snm);
            if let Ok(dc) = self.dc_mut(0) {
                changed |= snm.update(dc, now);
            }
            snm.fg.retain(|_, s| !s.finished);
            self.snm = snm;
        }
        if self.flash.as_ref().is_some_and(|flash| flash.finished(now)) {
            self.flash = None;
            changed = true;
        } else if self.flash.is_some() {
            changed = true;
        }
        for (r, due, mut animation) in std::mem::take(&mut self.queued_animations) {
            let Ok(object) = self.object_mut(r) else {
                continue;
            };
            let ready = match due {
                Some(due) => now >= due,
                None => !object.is_animating(),
            };
            if ready {
                animation.start = due.unwrap_or(now).min(now);
                object.animation = Some(animation);
                changed = true;
            } else {
                self.queued_animations.push((r, due, animation));
            }
        }
        for layer in [&mut self.fg, &mut self.bg] {
            for object in layer.iter_mut().flatten() {
                changed |= object.update(now);
            }
        }
        if changed {
            self.dirty = true;
        }
    }

    // ---- composition ------------------------------------------------------

    /// The current offsets of the screen shake and of each shaken layer.
    pub fn shake_offsets(
        &mut self,
        now: u64,
    ) -> (
        crate::shake::Offset,
        crate::shake::Layers,
        crate::shake::Offset,
    ) {
        let screen = match &self.screen_shake {
            Some(shake) => match shake.offset(now) {
                Some(offset) => offset,
                None => {
                    self.screen_shake = None;
                    Default::default()
                }
            },
            None => Default::default(),
        };
        let (layers, layer) = match &self.layer_shake {
            Some(shake) => match shake.offset(now) {
                Some(offset) => (shake.layers, offset),
                None => {
                    self.layer_shake = None;
                    (crate::shake::Layers::ALL, Default::default())
                }
            },
            None => (crate::shake::Layers::ALL, Default::default()),
        };
        (screen, layers, layer)
    }

    pub fn is_shaking(&self) -> bool {
        self.screen_shake.is_some() || self.layer_shake.is_some()
    }

    pub fn draw_objects(&mut self, frame: &mut Surface, now: u64, show: ShowFlags) {
        self.draw_objects_offset(frame, now, show, (0, 0));
    }

    pub fn draw_objects_offset(
        &mut self,
        frame: &mut Surface,
        now: u64,
        show: ShowFlags,
        offset: (i32, i32),
    ) {
        let mut order: Vec<(i32, i32, i32, usize)> = Vec::new();
        for (index, object) in self.fg.iter().enumerate() {
            let Some(object) = object else {
                continue;
            };
            if !object.params.visible
                || object.data.is_none()
                || self.ccom_hidden_objects.contains(&(index as i32))
            {
                continue;
            }
            let settings = self.settings_for(index);
            let hidden = (settings.obj_on_off == 1 && !show.object1)
                || (settings.obj_on_off == 2 && !show.object2)
                || (settings.weather_on_off != 0 && !show.weather)
                || (settings.space_key != 0 && self.interface_hidden);
            if hidden {
                continue;
            }
            let p = &object.params;
            order.push((p.z_order, p.z_layer, p.z_depth, index));
        }
        order.sort();
        let screen = self.screen_rect();
        let mut ctx = RenderContext {
            fonts: &mut self.fonts,
            colours: &self.colours,
            now,
            screen,
        };
        for (.., index) in order {
            let Some(object) = &self.fg[index] else {
                continue;
            };
            ctx.now = self.paused_animations.get(&index).copied().unwrap_or(now);
            let tone = self
                .object_tones
                .get(&index)
                .copied()
                .or(self.all_objects_tone);
            if let Some(table) = tone {
                let mut toned = object.clone();
                if let Some(image) = toned.image_mut() {
                    *image = self.tone_curves.apply(image, table);
                }
                toned.params.x += offset.0;
                toned.params.y += offset.1;
                toned.render(frame, &mut ctx, None);
            } else if offset == (0, 0) {
                object.render(frame, &mut ctx, None);
            } else {
                let mut shifted = object.clone();
                shifted.params.x += offset.0;
                shifted.params.y += offset.1;
                shifted.render(frame, &mut ctx, None);
            }
        }
    }

    /// `OBJFRONTANM_PAUSE` / `_PAUSEALL`.
    pub fn pause_animation(&mut self, index: usize, now: u64) {
        self.paused_animations.entry(index).or_insert(now);
    }

    /// `OBJFRONTANM_RESUME` / `_RESUMEALL`: continue from where it stopped.
    pub fn resume_animation(&mut self, index: usize, now: u64) {
        if let Some(since) = self.paused_animations.remove(&index)
            && let Some(Some(object)) = self.fg.get_mut(index)
            && let Some(animation) = &mut object.animation
        {
            animation.start += now.saturating_sub(since);
        }
    }

    // ---- savepoints and saves --------------------------------------------

    pub fn take_savepoint(&mut self) {
        self.snapshot = GraphicsSnapshot {
            dcs: self.dcs.clone(),
            fg: self.fg.clone(),
            bg: self.bg.clone(),
            stack: self.stack.clone(),
            hik: self.hik.as_ref().map(|hik| (hik.name.clone(), hik.offset)),
        };
    }

    pub fn reset(&mut self) {
        let (w, h) = (self.width, self.height);
        self.dcs = vec![None; DC_COUNT];
        self.dcs[0] = Some(Rc::new(Surface::new(w, h)));
        self.dcs[1] = Some(Rc::new(Surface::new(w, h)));
        self.fg = vec![None; OBJECT_COUNT];
        self.bg = vec![None; OBJECT_COUNT];
        self.stack.clear();
        self.screen_shake = None;
        self.layer_shake = None;
        self.transition_frame = None;
        self.interface_hidden = false;
        self.hik = None;
        self.snm = Default::default();
        self.haikei_image = None;
        self.haikei_scroll = None;
        self.haikei_queue.clear();
        self.haikei_ready.clear();
        self.flash = None;
        self.dirty = true;
    }

    /// After a load the clock has a different origin: finish running
    /// property animations and restart pattern animations now.
    pub fn rebase_times(&mut self, now: u64) {
        fn rebase(object: &mut Object, now: u64) {
            for mutator in std::mem::take(&mut object.mutators) {
                mutator.finish(&mut object.params);
            }
            if let Some(animation) = &mut object.animation {
                animation.start = now;
            }
            if let Some(ObjectData::Parent(children)) = &mut object.data {
                for child in children.iter_mut().flatten() {
                    rebase(child, now);
                }
            }
        }
        for layer in [&mut self.fg, &mut self.bg] {
            for object in layer.iter_mut().flatten() {
                rebase(object, now);
            }
        }
    }

    /// Writes the savepoint snapshot.
    pub fn save(&self, w: &mut Writer) {
        let snapshot = &self.snapshot;
        w.len(snapshot.dcs.len());
        for dc in &snapshot.dcs {
            match dc {
                Some(surface) => {
                    w.bool(true);
                    w.i32(surface.width);
                    w.i32(surface.height);
                    w.bytes(&compress(&surface.rgba));
                }
                None => w.bool(false),
            }
        }
        w.strs(&snapshot.stack);
        for layer in [&snapshot.fg, &snapshot.bg] {
            let present: Vec<(usize, &Object)> = layer
                .iter()
                .enumerate()
                .filter_map(|(i, o)| o.as_ref().map(|o| (i, o)))
                .collect();
            w.len(present.len());
            for (index, object) in present {
                w.u32(index as u32);
                crate::object_io::write(w, object);
            }
        }
        // Added later: older saves end here.
        let (name, (x, y)) = snapshot.hik.clone().unwrap_or_default();
        w.str(&name);
        w.i32(x);
        w.i32(y);
    }

    pub fn load(&mut self, r: &mut Reader, resources: &Resources) -> Result<()> {
        let count = r.len()?;
        let mut dcs = vec![None; DC_COUNT];
        for index in 0..count {
            if r.bool()? {
                let width = r.i32()?;
                let height = r.i32()?;
                let rgba = decompress(r.bytes()?)?;
                if index < DC_COUNT && rgba.len() == (width * height * 4) as usize {
                    dcs[index] = Some(Rc::new(Surface {
                        width,
                        height,
                        rgba,
                    }));
                }
            }
        }
        if dcs[0].is_none() {
            dcs[0] = Some(Rc::new(Surface::new(self.width, self.height)));
        }
        self.dcs = dcs;
        self.stack = r.strs()?;
        for bg in [false, true] {
            let mut layer = vec![None; OBJECT_COUNT];
            let present = r.len()?;
            for _ in 0..present {
                let index = r.u32()? as usize;
                let object = crate::object_io::read(r, self, resources)?;
                if index < OBJECT_COUNT {
                    layer[index] = Some(object);
                }
            }
            if bg {
                self.bg = layer;
            } else {
                self.fg = layer;
            }
        }
        self.hik = None;
        if !r.is_empty() {
            let name = r.str()?;
            let offset = (r.i32()?, r.i32()?);
            if !name.is_empty()
                && let Some(bytes) = resources.read(crate::resource::Kind::Hik, &name)
            {
                let script = crate::hik::HikScript::parse(&bytes, |frame| {
                    self.load_image(resources, frame)
                })?;
                let mut renderer = crate::hik::HikRenderer::new(&name, Rc::new(script), 0);
                renderer.offset = offset;
                self.hik = Some(renderer);
            }
        }
        self.transition_frame = None;
        self.dirty = true;
        self.take_savepoint();
        Ok(())
    }
}

/// Which optional object groups are enabled (`ShowObject1`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShowFlags {
    pub object1: bool,
    pub object2: bool,
    pub weather: bool,
}

impl Default for ShowFlags {
    fn default() -> Self {
        Self {
            object1: true,
            object2: true,
            weather: true,
        }
    }
}

/// `#SCREENSIZE_MOD`: 0 is 640x480, 1 is 800x600; later interpreters
/// accept larger sizes.
pub fn screen_size(gameexe: &Gameexe) -> (i32, i32) {
    match gameexe.ints("SCREENSIZE_MOD")[..] {
        [1, ..] => (800, 600),
        [2, ..] => (1024, 768),
        [3, ..] => (1280, 720),
        [999, w, h, ..] if w > 0 && h > 0 => (w, h),
        _ => (640, 480),
    }
}

/// Run-length compression for DC pixels in saves (screens are mostly
/// runs of similar pixels; this keeps saves small without a codec
/// dependency).
pub fn compress(data: &[u8]) -> Vec<u8> {
    // Encode 4-byte pixels as (run length, pixel) pairs, runs up to 255.
    let mut out = Vec::with_capacity(data.len() / 4);
    let pixels = data.chunks_exact(4);
    let mut current: Option<&[u8]> = None;
    let mut run = 0u8;
    for pixel in pixels {
        match current {
            Some(previous) if previous == pixel && run < 255 => run += 1,
            _ => {
                if let Some(previous) = current {
                    out.push(run);
                    out.extend_from_slice(previous);
                }
                current = Some(pixel);
                run = 1;
            }
        }
    }
    if let Some(previous) = current {
        out.push(run);
        out.extend_from_slice(previous);
    }
    out
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(data.len() * 2);
    for chunk in data.chunks(5) {
        let [run, r, g, b, a] = chunk[..] else {
            return Err(anyhow!("reallive: corrupt pixel data in save"));
        };
        for _ in 0..run {
            out.extend_from_slice(&[r, g, b, a]);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod haikei_tests {
    use super::*;

    #[test]
    fn scrolls_interpolate_and_keep_unnamed_axes() {
        let m = HaikeiMove {
            x: Some(100),
            y: None,
            time: 1000,
        };
        let scroll = HaikeiScroll::start((0, 40), m, 0);
        assert_eq!(scroll.at(500), (50, 40));
        assert_eq!(scroll.at(2000), (100, 40));
        assert!(scroll.finished(1000));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_compression_round_trips() {
        let mut data = Vec::new();
        for i in 0..1000u32 {
            let v = (i / 300) as u8;
            data.extend_from_slice(&[v, v, 1, 255]);
        }
        assert_eq!(decompress(&compress(&data)).unwrap(), data);
        assert!(compress(&data).len() < data.len() / 10);
    }

    #[test]
    fn promotion_respects_wipe_copy() {
        let mut graphics = Graphics::new(&Gameexe::default(), FontSet::empty());
        graphics
            .set_object(ObjectRef::fg(1), ObjectData::Text)
            .unwrap();
        graphics
            .set_object(ObjectRef::fg(2), ObjectData::Text)
            .unwrap()
            .params
            .wipe_copy = true;
        graphics
            .set_object(
                ObjectRef {
                    bg: true,
                    buf: 3,
                    child: None,
                },
                ObjectData::Text,
            )
            .unwrap();
        graphics.promote_objects();
        assert!(graphics.fg[1].is_none());
        assert!(graphics.fg[2].is_some());
        assert!(graphics.fg[3].is_some());
        assert!(graphics.bg[3].is_none());
    }
}
