//! Graphics objects: retained sprites composited over DC 0.
//!
//! Geometry follows the RealLive model: the object's origin (its own
//! `objOrigin`, or the origin stored with the g00 pattern) is placed at
//! `(x, y)` plus the eight adjustment offsets; scaling and rotation happen
//! around the origin plus the "rep origin".

use std::cell::RefCell;
use std::rc::Rc;

use crate::font::FontSet;
use crate::gan::Gan;
use crate::image::{Image, Region};
use crate::surface::{Blend, Rect, Surface, Transform, apply_tint};

pub const OBJECT_COUNT: usize = 256;

#[derive(Debug, Clone, PartialEq)]
pub struct TextParams {
    pub text: String,
    pub size: i32,
    pub xspace: i32,
    pub yspace: i32,
    /// Characters per line (0 = unlimited); `objTextOpts`' vertical flag
    /// makes this 1.
    pub char_count: i32,
    pub colour: i32,
    /// Shadow colour index, or -1 for none.
    pub shadow: i32,
}

impl Default for TextParams {
    fn default() -> Self {
        Self {
            text: String::new(),
            size: 14,
            xspace: 0,
            yspace: 0,
            char_count: 0,
            colour: 0,
            shadow: -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DigitParams {
    pub value: i32,
    pub digits: i32,
    pub zero: i32,
    pub sign: i32,
    pub pack: i32,
    pub space: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftParams {
    pub count: i32,
    pub use_animation: i32,
    pub start_pattern: i32,
    pub end_pattern: i32,
    pub animation_time: i32,
    pub y_time: i32,
    pub period: i32,
    pub amplitude: i32,
    pub use_drift: i32,
    pub unknown: i32,
    pub drift_speed: i32,
    pub area: Rect,
}

impl Default for DriftParams {
    fn default() -> Self {
        Self {
            count: 1,
            use_animation: 0,
            start_pattern: 0,
            end_pattern: 0,
            animation_time: 0,
            y_time: 1000,
            period: 0,
            amplitude: 0,
            use_drift: 0,
            unknown: 0,
            drift_speed: 0,
            area: Rect::new(0, 0, 640, 480),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ButtonParams {
    pub is_button: i32,
    pub action: i32,
    pub se: i32,
    pub group: i32,
    pub number: i32,
    /// 0 normal, 1 hover, 2 pressed (pattern offsets).
    pub state: i32,
}

/// Everything scripts can set on an object.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectParams {
    pub visible: bool,
    pub x: i32,
    pub y: i32,
    pub adjust_x: [i32; 8],
    pub adjust_y: [i32; 8],
    pub adjust_vert: i32,
    pub origin: (i32, i32),
    pub rep_origin: (i32, i32),
    /// Percent.
    pub width: i32,
    pub height: i32,
    /// Thousandths.
    pub hq_width: i32,
    pub hq_height: i32,
    /// Tenths of a degree, clockwise.
    pub rotation: i32,
    pub pattern: i32,
    pub alpha: i32,
    pub adjust_alpha: [i32; 8],
    pub clip: Option<Rect>,
    pub own_clip: Option<Rect>,
    pub mono: i32,
    pub invert: i32,
    pub light: i32,
    pub tint: [i32; 3],
    /// r, g, b, level.
    pub colour: [i32; 4],
    /// 0 normal, 1 add, 2 subtract.
    pub composite: i32,
    pub scroll_rate: (i32, i32),
    pub z_order: i32,
    pub z_layer: i32,
    pub z_depth: i32,
    pub quarter_view: i32,
    pub wipe_copy: bool,
    pub text: TextParams,
    pub digits: DigitParams,
    pub drift: DriftParams,
    pub button: ButtonParams,
    /// `objFadeOpts` values (min alpha, max alpha, ...).
    pub fade: [i32; 5],
}

impl Default for ObjectParams {
    fn default() -> Self {
        Self {
            visible: false,
            x: 0,
            y: 0,
            adjust_x: [0; 8],
            adjust_y: [0; 8],
            adjust_vert: 0,
            origin: (0, 0),
            rep_origin: (0, 0),
            width: 100,
            height: 100,
            hq_width: 1000,
            hq_height: 1000,
            rotation: 0,
            pattern: 0,
            alpha: 255,
            adjust_alpha: [255; 8],
            clip: None,
            own_clip: None,
            mono: 0,
            invert: 0,
            light: 0,
            tint: [0; 3],
            colour: [0; 4],
            composite: 0,
            scroll_rate: (0, 0),
            z_order: 0,
            z_layer: 0,
            z_depth: 0,
            quarter_view: 0,
            wipe_copy: false,
            text: TextParams::default(),
            digits: DigitParams::default(),
            drift: DriftParams::default(),
            button: ButtonParams::default(),
            fade: [0, 255, 0, 0, 0],
        }
    }
}

impl ObjectParams {
    pub fn adjust_sum(&self) -> (i32, i32) {
        (
            self.adjust_x.iter().sum(),
            self.adjust_y.iter().sum::<i32>() + self.adjust_vert,
        )
    }

    pub fn scale(&self) -> (f64, f64) {
        (
            f64::from(self.width) / 100.0 * f64::from(self.hq_width) / 1000.0,
            f64::from(self.height) / 100.0 * f64::from(self.hq_height) / 1000.0,
        )
    }

    pub fn computed_alpha(&self) -> i32 {
        self.adjust_alpha
            .iter()
            .fold(self.alpha.clamp(0, 255), |alpha, adjust| {
                alpha * (*adjust).clamp(0, 255) / 255
            })
    }

    /// Whether any colour effect applies.
    fn has_effects(&self) -> bool {
        self.mono != 0
            || self.invert != 0
            || self.light != 0
            || self.tint != [0; 3]
            || self.colour[3] != 0
    }

    /// The per-pixel colour pipeline: colour mix, tint, light, mono, invert.
    pub fn shade(&self, pixel: [u8; 4]) -> [u8; 4] {
        let mut rgb = [pixel[0], pixel[1], pixel[2]];
        let level = self.colour[3].clamp(0, 255) as u32;
        if level != 0 {
            for c in 0..3 {
                let target = self.colour[c].clamp(0, 255) as u32;
                rgb[c] = ((u32::from(rgb[c]) * (255 - level) + target * level) / 255) as u8;
            }
        }
        if self.tint != [0; 3] {
            rgb = apply_tint(rgb, self.tint);
        }
        if self.light != 0 {
            rgb = apply_tint(rgb, [self.light; 3]);
        }
        if self.mono != 0 {
            let grey = ((u32::from(rgb[0]) * 77 + u32::from(rgb[1]) * 151 + u32::from(rgb[2]) * 28)
                >> 8) as u8;
            let level = self.mono.clamp(0, 255) as u32;
            for channel in &mut rgb {
                *channel =
                    ((u32::from(*channel) * (255 - level) + u32::from(grey) * level) / 255) as u8;
            }
        }
        if self.invert != 0 {
            let level = self.invert.clamp(0, 255) as u32;
            for channel in &mut rgb {
                let inverted = 255 - u32::from(*channel);
                *channel = ((u32::from(*channel) * (255 - level) + inverted * level) / 255) as u8;
            }
        }
        [rgb[0], rgb[1], rgb[2], pixel[3]]
    }
}

/// How a pattern or GAN animation ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AfterAnimation {
    /// Stay on the last frame (`Play`).
    Stop,
    /// Delete the object (`PlayOnce`).
    Clear,
    /// Start again (`Loop`).
    Loop,
    /// Play forwards and back, pause about three seconds, repeat (`Blink`).
    Blink,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    pub start: u64,
    pub after: AfterAnimation,
    /// Pattern animations: ms per pattern. GAN animations: the set index.
    pub parameter: i32,
    pub finished: bool,
}

/// Pause between blinks.
const BLINK_PAUSE: u64 = 3000;

#[derive(Debug, Clone)]
pub enum ObjectData {
    File {
        name: String,
        image: Rc<Image>,
    },
    Gan {
        image_name: String,
        image: Rc<Image>,
        gan_name: String,
        gan: Rc<Gan>,
    },
    Text,
    Digits {
        name: String,
        image: Rc<Image>,
    },
    Drift {
        name: String,
        image: Rc<Image>,
    },
    /// A filter area (`objOfArea`, `objOfRect`).
    Rect(Rect),
    /// A parent holding child objects.
    Parent(Vec<Option<Object>>),
}

/// A property that `objEve*` functions can animate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Property {
    X,
    Y,
    Alpha,
    AdjustX(usize),
    AdjustY(usize),
    Mono,
    Invert,
    Light,
    TintR,
    TintG,
    TintB,
    ColR,
    ColG,
    ColB,
    ColLevel,
    AdjustVert,
    AdjustAlpha(usize),
    Width,
    Height,
    Rotation,
    RepOriginX,
    RepOriginY,
    OriginX,
    OriginY,
    HqWidth,
    HqHeight,
    Visible,
    /// The clip rectangle (`objRectEria`, `objBoxEria`): left, top,
    /// width, height, right, bottom.
    ClipX,
    ClipY,
    ClipW,
    ClipH,
    ClipRight,
    ClipBottom,
    /// The object's own clip (`objOwnEria` and friends), likewise.
    OwnClipX,
    OwnClipY,
    OwnClipW,
    OwnClipH,
    OwnClipRight,
    OwnClipBottom,
}

/// An unset clip, as a rectangle: large enough not to clip.
const NO_CLIP: Rect = Rect {
    x: 0,
    y: 0,
    w: 0x4000,
    h: 0x4000,
};

impl Property {
    pub fn get(self, p: &ObjectParams) -> i32 {
        match self {
            Property::X => p.x,
            Property::Y => p.y,
            Property::Alpha => p.alpha,
            Property::AdjustX(i) => p.adjust_x[i & 7],
            Property::AdjustY(i) => p.adjust_y[i & 7],
            Property::Mono => p.mono,
            Property::Invert => p.invert,
            Property::Light => p.light,
            Property::TintR => p.tint[0],
            Property::TintG => p.tint[1],
            Property::TintB => p.tint[2],
            Property::ColR => p.colour[0],
            Property::ColG => p.colour[1],
            Property::ColB => p.colour[2],
            Property::ColLevel => p.colour[3],
            Property::AdjustVert => p.adjust_vert,
            Property::AdjustAlpha(i) => p.adjust_alpha[i & 7],
            Property::Width => p.width,
            Property::Height => p.height,
            Property::Rotation => p.rotation,
            Property::RepOriginX => p.rep_origin.0,
            Property::RepOriginY => p.rep_origin.1,
            Property::OriginX => p.origin.0,
            Property::OriginY => p.origin.1,
            Property::HqWidth => p.hq_width,
            Property::HqHeight => p.hq_height,
            Property::Visible => i32::from(p.visible),
            Property::ClipX => p.clip.unwrap_or(NO_CLIP).x,
            Property::ClipY => p.clip.unwrap_or(NO_CLIP).y,
            Property::ClipW => p.clip.unwrap_or(NO_CLIP).w,
            Property::ClipH => p.clip.unwrap_or(NO_CLIP).h,
            Property::ClipRight => p.clip.unwrap_or(NO_CLIP).right(),
            Property::ClipBottom => p.clip.unwrap_or(NO_CLIP).bottom(),
            Property::OwnClipX => p.own_clip.unwrap_or(NO_CLIP).x,
            Property::OwnClipY => p.own_clip.unwrap_or(NO_CLIP).y,
            Property::OwnClipW => p.own_clip.unwrap_or(NO_CLIP).w,
            Property::OwnClipH => p.own_clip.unwrap_or(NO_CLIP).h,
            Property::OwnClipRight => p.own_clip.unwrap_or(NO_CLIP).right(),
            Property::OwnClipBottom => p.own_clip.unwrap_or(NO_CLIP).bottom(),
        }
    }

    pub fn set(self, p: &mut ObjectParams, value: i32) {
        match self {
            Property::X => p.x = value,
            Property::Y => p.y = value,
            Property::Alpha => p.alpha = value,
            Property::AdjustX(i) => p.adjust_x[i & 7] = value,
            Property::AdjustY(i) => p.adjust_y[i & 7] = value,
            Property::Mono => p.mono = value,
            Property::Invert => p.invert = value,
            Property::Light => p.light = value,
            Property::TintR => p.tint[0] = value,
            Property::TintG => p.tint[1] = value,
            Property::TintB => p.tint[2] = value,
            Property::ColR => p.colour[0] = value,
            Property::ColG => p.colour[1] = value,
            Property::ColB => p.colour[2] = value,
            Property::ColLevel => p.colour[3] = value,
            Property::AdjustVert => p.adjust_vert = value,
            Property::AdjustAlpha(i) => p.adjust_alpha[i & 7] = value,
            Property::Width => p.width = value,
            Property::Height => p.height = value,
            Property::Rotation => p.rotation = value,
            Property::RepOriginX => p.rep_origin.0 = value,
            Property::RepOriginY => p.rep_origin.1 = value,
            Property::OriginX => p.origin.0 = value,
            Property::OriginY => p.origin.1 = value,
            Property::HqWidth => p.hq_width = value,
            Property::HqHeight => p.hq_height = value,
            Property::Visible => p.visible = value != 0,
            Property::ClipX
            | Property::ClipY
            | Property::ClipW
            | Property::ClipH
            | Property::ClipRight
            | Property::ClipBottom => {
                let r = p.clip.get_or_insert(NO_CLIP);
                match self {
                    Property::ClipX => r.x = value,
                    Property::ClipY => r.y = value,
                    Property::ClipW => r.w = value,
                    Property::ClipH => r.h = value,
                    Property::ClipRight => r.w = value - r.x,
                    _ => r.h = value - r.y,
                }
            }
            Property::OwnClipX
            | Property::OwnClipY
            | Property::OwnClipW
            | Property::OwnClipH
            | Property::OwnClipRight
            | Property::OwnClipBottom => {
                let r = p.own_clip.get_or_insert(NO_CLIP);
                match self {
                    Property::OwnClipX => r.x = value,
                    Property::OwnClipY => r.y = value,
                    Property::OwnClipW => r.w = value,
                    Property::OwnClipH => r.h = value,
                    Property::OwnClipRight => r.w = value - r.x,
                    _ => r.h = value - r.y,
                }
            }
        }
    }
}

/// An `objEve*` animation of one or two properties.
#[derive(Debug, Clone, PartialEq)]
pub struct Mutator {
    /// The unified property id (base id of the `obj*` function family).
    pub id: i32,
    pub repno: i32,
    pub targets: Vec<(Property, i32, i32)>,
    pub start: u64,
    pub duration: i32,
    pub delay: i32,
    /// 0 linear, 1 accelerating, 2 decelerating; 3 is a damped wave:
    /// targets are `(property, amplitude, cycles)` and settle at 0.
    /// `CURVE_WAVE_AROUND + n` is a damped wave of `n` cycles around a
    /// base: targets are `(property, base, amplitude)`.
    pub curve: i32,
}

pub const CURVE_WAVE: i32 = 3;
/// A mutator id that matches every running `objEve*` animation
/// (`*EVE_ALLEVE_*`).
pub const ALL_MUTATORS: i32 = -1;
pub const CURVE_WAVE_AROUND: i32 = 1000;
/// `CURVE_FLASH + n`: `n` damped pulses from the base towards base +
/// amplitude and back.
pub const CURVE_FLASH: i32 = 2000;
/// `CURVE_BLINK + n`: `n` times base + amplitude, then base (on/off).
pub const CURVE_BLINK: i32 = 3000;
/// `CURVE_REPEAT + n`: `n` undamped trips to base + amplitude and back.
pub const CURVE_REPEAT: i32 = 4000;

impl Mutator {
    fn value_at(&self, from: i32, to: i32, now: u64) -> (i32, bool) {
        let begin = self.start + self.delay.max(0) as u64;
        if now < begin {
            return (from, false);
        }
        if self.duration <= 0 || now >= begin + self.duration as u64 {
            return (to, true);
        }
        let t = (now - begin) as f64 / f64::from(self.duration);
        if self.curve >= CURVE_REPEAT {
            let cycles = f64::from(self.curve - CURVE_REPEAT);
            let phase = (t * cycles).fract();
            let tri = 1.0 - (phase * 2.0 - 1.0).abs();
            return (
                (f64::from(from) + f64::from(to) * tri).round() as i32,
                false,
            );
        }
        if self.curve >= CURVE_BLINK {
            let cycles = f64::from(self.curve - CURVE_BLINK);
            let on = (t * cycles).fract() < 0.5;
            return (if on { from + to } else { from }, false);
        }
        if self.curve >= CURVE_FLASH {
            let cycles = f64::from(self.curve - CURVE_FLASH);
            let pulse = (t * cycles * std::f64::consts::PI).sin().abs() * (1.0 - t);
            return (
                (f64::from(from) + f64::from(to) * pulse).round() as i32,
                false,
            );
        }
        if self.curve >= CURVE_WAVE_AROUND {
            let cycles = f64::from(self.curve - CURVE_WAVE_AROUND);
            let wave = (t * cycles * std::f64::consts::TAU).sin() * (1.0 - t);
            return (
                (f64::from(from) + f64::from(to) * wave).round() as i32,
                false,
            );
        }
        if self.curve == CURVE_WAVE {
            let wave = (t * f64::from(to) * std::f64::consts::TAU).sin() * (1.0 - t);
            return ((f64::from(from) * wave).round() as i32, false);
        }
        let shaped = match self.curve {
            1 => t * t,
            2 => 1.0 - (1.0 - t) * (1.0 - t),
            _ => t,
        };
        (
            (f64::from(from) + f64::from(to - from) * shaped).round() as i32,
            false,
        )
    }

    /// Applies the value for `now`; true when finished.
    pub fn apply(&self, params: &mut ObjectParams, now: u64) -> bool {
        let mut done = true;
        for &(property, from, to) in &self.targets {
            let (value, finished) = self.value_at(from, to, now);
            if finished && self.curve >= CURVE_WAVE_AROUND {
                property.set(params, from);
            } else if finished {
                self.finish_one(params, property, to);
            } else if property == Property::Visible {
                // Shown while the effect runs, whatever it ends with.
                property.set(params, 1);
            } else {
                property.set(params, value);
            }
            done &= finished;
        }
        done
    }

    fn finish_one(&self, params: &mut ObjectParams, property: Property, to: i32) {
        let value = if self.curve == CURVE_WAVE { 0 } else { to };
        property.set(params, value);
    }

    pub fn finish(&self, params: &mut ObjectParams) {
        for &(property, from, to) in &self.targets {
            if self.curve >= CURVE_WAVE_AROUND {
                property.set(params, from);
            } else {
                self.finish_one(params, property, to);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub params: ObjectParams,
    pub data: Option<ObjectData>,
    pub animation: Option<Animation>,
    pub mutators: Vec<Mutator>,
    /// Rendered text (key: text parameters) for text objects.
    text_cache: RefCell<Option<(TextParams, Rc<Surface>)>>,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params
            && self.animation == other.animation
            && self.mutators == other.mutators
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new(ObjectData::Text)
    }
}

/// Resources the renderer needs.
pub struct RenderContext<'a> {
    pub fonts: &'a mut FontSet,
    pub colours: &'a [[u8; 3]],
    pub now: u64,
    pub screen: Rect,
}

impl Object {
    pub fn new(data: ObjectData) -> Self {
        Self {
            params: ObjectParams::default(),
            data: Some(data),
            animation: None,
            mutators: Vec::new(),
            text_cache: RefCell::new(None),
        }
    }

    pub fn empty() -> Self {
        Self {
            params: ObjectParams::default(),
            data: None,
            animation: None,
            mutators: Vec::new(),
            text_cache: RefCell::new(None),
        }
    }

    pub fn image_mut(&mut self) -> Option<&mut Rc<Image>> {
        match self.data.as_mut()? {
            ObjectData::File { image, .. }
            | ObjectData::Gan { image, .. }
            | ObjectData::Digits { image, .. }
            | ObjectData::Drift { image, .. } => Some(image),
            _ => None,
        }
    }

    pub fn image(&self) -> Option<&Rc<Image>> {
        match self.data.as_ref()? {
            ObjectData::File { image, .. }
            | ObjectData::Gan { image, .. }
            | ObjectData::Digits { image, .. }
            | ObjectData::Drift { image, .. } => Some(image),
            _ => None,
        }
    }

    /// Advances animations and mutators. Returns true if anything changed.
    pub fn update(&mut self, now: u64) -> bool {
        let mut changed = false;
        if !self.mutators.is_empty() {
            let params = &mut self.params;
            self.mutators.retain(|mutator| !mutator.apply(params, now));
            changed = true;
        }
        if let Some(animation) = &mut self.animation {
            if !animation.finished {
                changed = true;
                let frames = match self.data.as_ref() {
                    Some(ObjectData::Gan { gan, .. }) => gan
                        .sets
                        .get(animation.parameter.max(0) as usize)
                        .map_or(0, |set| set.len()),
                    Some(ObjectData::File { image, .. }) => image.regions.len(),
                    _ => 0,
                };
                if frames == 0 {
                    animation.finished = true;
                } else if let Some(ObjectData::Gan { gan, .. }) = self.data.as_ref() {
                    let set = &gan.sets[animation.parameter.max(0) as usize];
                    let total: u64 = set.iter().map(|f| f.time.max(1) as u64).sum();
                    let elapsed = now.saturating_sub(animation.start);
                    match animation.after {
                        AfterAnimation::Loop | AfterAnimation::Blink => {}
                        _ if elapsed >= total => animation.finished = true,
                        _ => {}
                    }
                } else {
                    let per = animation.parameter.max(1) as u64;
                    let elapsed = now.saturating_sub(animation.start);
                    if matches!(
                        animation.after,
                        AfterAnimation::Stop | AfterAnimation::Clear
                    ) && elapsed >= per * frames as u64
                    {
                        animation.finished = true;
                    }
                }
                if animation.finished && animation.after == AfterAnimation::Clear {
                    self.data = None;
                }
            }
        }
        if let Some(ObjectData::Parent(children)) = &mut self.data {
            for child in children.iter_mut().flatten() {
                changed |= child.update(now);
            }
        }
        changed
    }

    /// Length of one cycle of the running pattern animation, in ms.
    pub fn animation_cycle(&self) -> Option<u64> {
        let animation = self.animation.as_ref()?;
        match self.data.as_ref()? {
            ObjectData::Gan { gan, .. } => {
                let set = gan.sets.get(animation.parameter.max(0) as usize)?;
                Some(set.iter().map(|f| f.time.max(1) as u64).sum())
            }
            ObjectData::File { image, .. } => {
                Some(image.regions.len().max(1) as u64 * animation.parameter.max(1) as u64)
            }
            _ => None,
        }
    }

    pub fn is_animating(&self) -> bool {
        self.animation.as_ref().is_some_and(|a| {
            !a.finished && matches!(a.after, AfterAnimation::Stop | AfterAnimation::Clear)
        })
    }

    pub fn has_mutator(&self, id: i32, repno: Option<i32>) -> bool {
        self.mutators
            .iter()
            .any(|m| (id == ALL_MUTATORS || m.id == id) && repno.is_none_or(|r| r == m.repno))
    }

    /// Finishes matching mutators immediately.
    pub fn end_mutators(&mut self, id: i32, repno: Option<i32>) {
        let params = &mut self.params;
        self.mutators.retain(|m| {
            let matches = (id == ALL_MUTATORS || m.id == id) && repno.is_none_or(|r| r == m.repno);
            if matches {
                m.finish(params);
            }
            !matches
        });
    }

    /// The pattern and GAN offset shown at `now`.
    pub fn current_frame(&self, now: u64) -> (i32, (i32, i32), i32) {
        let pattern = self.params.pattern;
        let Some(animation) = &self.animation else {
            return (pattern, (0, 0), 255);
        };
        let elapsed = now.saturating_sub(animation.start);
        match self.data.as_ref() {
            Some(ObjectData::Gan { gan, .. }) => {
                let Some(set) = gan.sets.get(animation.parameter.max(0) as usize) else {
                    return (pattern, (0, 0), 255);
                };
                if set.is_empty() {
                    return (pattern, (0, 0), 255);
                }
                let total: u64 = set.iter().map(|f| f.time.max(1) as u64).sum();
                let mut t = match animation.after {
                    AfterAnimation::Loop => elapsed % total.max(1),
                    AfterAnimation::Blink => {
                        let cycle = total + BLINK_PAUSE;
                        let phase = elapsed % cycle;
                        if phase >= total { total } else { phase }
                    }
                    _ => elapsed.min(total),
                };
                // `GAN_CUTNO_REP` shifts the GAN's patterns (digit objects
                // animated by one GAN).
                for frame in set {
                    let time = frame.time.max(1) as u64;
                    if t < time {
                        return (frame.pattern + pattern, (frame.x, frame.y), frame.alpha);
                    }
                    t -= time;
                }
                let last = set.last().expect("non-empty");
                (last.pattern + pattern, (last.x, last.y), last.alpha)
            }
            Some(ObjectData::File { image, .. }) => {
                let frames = image.regions.len().max(1) as u64;
                let per = animation.parameter.max(1) as u64;
                let step = elapsed / per;
                let index = match animation.after {
                    AfterAnimation::Loop => step % frames,
                    AfterAnimation::Blink => {
                        // 0..n-1..0, then a pause on frame 0.
                        let sweep = frames * 2 - 1;
                        let pause = BLINK_PAUSE / per;
                        let phase = step % (sweep + pause);
                        if phase < frames {
                            phase
                        } else if phase < sweep {
                            sweep - 1 - phase
                        } else {
                            0
                        }
                    }
                    _ => step.min(frames - 1),
                };
                (index as i32, (0, 0), 255)
            }
            _ => (pattern, (0, 0), 255),
        }
    }

    /// Size of the current pattern (for `objGetDims`).
    pub fn dimensions(&self, now: u64, fonts: &mut FontSet, colours: &[[u8; 3]]) -> (i32, i32) {
        let (scale_x, scale_y) = self.params.scale();
        let (w, h) = match self.data.as_ref() {
            Some(ObjectData::File { image, .. } | ObjectData::Gan { image, .. }) => {
                let (pattern, ..) = self.current_frame(now);
                let region = image.region(pattern);
                (region.width(), region.height())
            }
            Some(ObjectData::Text) => {
                let surface = self.text_surface(fonts, colours);
                (surface.width, surface.height)
            }
            Some(ObjectData::Rect(rect)) => (rect.w, rect.h),
            Some(ObjectData::Digits { image, .. } | ObjectData::Drift { image, .. }) => {
                let region = image.region(0);
                (region.width(), region.height())
            }
            _ => (0, 0),
        };
        (
            (f64::from(w) * scale_x).round() as i32,
            (f64::from(h) * scale_y).round() as i32,
        )
    }

    /// Unrotated screen bounds (button hit testing). `offset` is the
    /// parent's position for child objects.
    pub fn bounds(
        &self,
        now: u64,
        fonts: &mut FontSet,
        colours: &[[u8; 3]],
        offset: (i32, i32),
    ) -> Rect {
        let p = &self.params;
        let (w, h) = self.dimensions(now, fonts, colours);
        let pattern_origin = match self.data.as_ref() {
            Some(ObjectData::File { image, .. } | ObjectData::Gan { image, .. }) => {
                let region = image.region(self.current_frame(now).0);
                (region.origin_x, region.origin_y)
            }
            _ => (0, 0),
        };
        let anchor = if p.origin != (0, 0) {
            p.origin
        } else {
            pattern_origin
        };
        let (sx, sy) = p.scale();
        let pivot = (anchor.0 + p.rep_origin.0, anchor.1 + p.rep_origin.1);
        let (ax, ay) = p.adjust_sum();
        let x = f64::from(p.x + ax + offset.0 + pivot.0 - anchor.0) - f64::from(pivot.0) * sx;
        let y = f64::from(p.y + ay + offset.1 + pivot.1 - anchor.1) - f64::from(pivot.1) * sy;
        Rect::new(x.round() as i32, y.round() as i32, w, h)
    }

    fn text_surface(&self, fonts: &mut FontSet, colours: &[[u8; 3]]) -> Rc<Surface> {
        let key = self.params.text.clone();
        if let Some((cached_key, surface)) = self.text_cache.borrow().as_ref() {
            if *cached_key == key {
                return surface.clone();
            }
        }
        let surface = Rc::new(render_text_object(&key, fonts, colours));
        *self.text_cache.borrow_mut() = Some((key, surface.clone()));
        surface
    }

    /// Draws the object onto `dst`.
    pub fn render(
        &self,
        dst: &mut Surface,
        ctx: &mut RenderContext,
        parent: Option<&ObjectParams>,
    ) {
        let p = &self.params;
        if !p.visible {
            return;
        }
        let Some(data) = self.data.as_ref() else {
            return;
        };
        let (px, py, parent_alpha, parent_scale) = match parent {
            Some(parent) => {
                let (ax, ay) = parent.adjust_sum();
                (
                    parent.x + ax,
                    parent.y + ay,
                    parent.computed_alpha(),
                    parent.scale(),
                )
            }
            None => (0, 0, 255, (1.0, 1.0)),
        };
        let mut alpha = p.computed_alpha() * parent_alpha / 255;
        let mut clip = p.clip.unwrap_or(ctx.screen);
        if let Some(parent) = parent {
            if let Some(own) = parent.own_clip {
                clip = clip.intersect(&Rect::new(px + own.x, py + own.y, own.w, own.h));
            }
        }
        match data {
            ObjectData::Rect(area) => {
                // A filter: shade what lies beneath.
                let area = Rect::new(area.x + px, area.y + py, area.w, area.h).intersect(&clip);
                if p.has_effects() {
                    let opacity = alpha.clamp(0, 255) as u8;
                    let params = p.clone();
                    dst.map_pixels(area, opacity, |rgb| {
                        let shaded = params.shade([rgb[0], rgb[1], rgb[2], 255]);
                        [shaded[0], shaded[1], shaded[2]]
                    });
                }
            }
            ObjectData::Parent(children) => {
                let mut combined = p.clone();
                combined.x += px;
                combined.y += py;
                combined.alpha = alpha;
                combined.adjust_alpha = [255; 8];
                for child in children.iter().flatten() {
                    child.render(dst, ctx, Some(&combined));
                }
            }
            ObjectData::Digits { image, .. } => {
                let (ax, ay) = p.adjust_sum();
                let mut x = p.x + ax + px;
                let y = p.y + ay + py;
                for pattern in digit_patterns(&p.digits) {
                    let region = image.region(pattern);
                    if pattern >= 0 {
                        self.blit_region(
                            dst,
                            image,
                            region,
                            (x, y),
                            (0, 0),
                            alpha,
                            clip,
                            parent_scale,
                        );
                    }
                    x += if p.digits.space > 0 {
                        p.digits.space
                    } else {
                        region.width()
                    };
                }
            }
            ObjectData::Drift { image, .. } => {
                self.render_drift(dst, image, ctx.now, alpha, clip, (px, py));
            }
            ObjectData::Text => {
                let surface = self.text_surface(ctx.fonts, ctx.colours);
                let image = Image::from_surface((*surface).clone());
                let region = image.region(0);
                let (ax, ay) = p.adjust_sum();
                self.blit_region(
                    dst,
                    &image,
                    region,
                    (p.x + ax + px, p.y + ay + py),
                    (0, 0),
                    alpha,
                    clip,
                    parent_scale,
                );
            }
            ObjectData::File { image, .. } | ObjectData::Gan { image, .. } => {
                let (pattern, offset, frame_alpha) = self.current_frame(ctx.now);
                if pattern < 0 {
                    return;
                }
                let pattern = pattern + p.button.state.max(0) * i32::from(p.button.is_button != 0);
                alpha = alpha * frame_alpha.clamp(0, 255) / 255;
                let region = image.region(pattern);
                let (ax, ay) = p.adjust_sum();
                self.blit_region(
                    dst,
                    image,
                    region,
                    (p.x + ax + px + offset.0, p.y + ay + py + offset.1),
                    (region.origin_x, region.origin_y),
                    alpha,
                    clip,
                    parent_scale,
                );
            }
        }
    }

    /// Draws one pattern with the object's transform and colour effects.
    #[allow(clippy::too_many_arguments)]
    fn blit_region(
        &self,
        dst: &mut Surface,
        image: &Image,
        mut region: Region,
        position: (i32, i32),
        pattern_origin: (i32, i32),
        alpha: i32,
        clip: Rect,
        parent_scale: (f64, f64),
    ) {
        let p = &self.params;
        if alpha <= 0 || region.width() <= 0 || region.height() <= 0 {
            return;
        }
        if let Some(own) = p.own_clip {
            // Own clipping is expressed relative to the pattern.
            region.x1 = (region.x1 + own.x).max(region.x1);
            region.y1 = (region.y1 + own.y).max(region.y1);
            region.x2 = region.x2.min(region.x1 + own.w - 1);
            region.y2 = region.y2.min(region.y1 + own.h - 1);
        }
        let anchor = if p.origin != (0, 0) {
            p.origin
        } else {
            pattern_origin
        };
        let (sx, sy) = p.scale();
        let scale = (sx * parent_scale.0, sy * parent_scale.1);
        let pivot = (anchor.0 + p.rep_origin.0, anchor.1 + p.rep_origin.1);
        // Pattern-local coordinates → bitmap coordinates.
        let to_bitmap = |(x, y): (i32, i32)| (f64::from(x + region.x1), f64::from(y + region.y1));
        let origin = to_bitmap(pivot);
        let dest = (
            f64::from(position.0 + pivot.0 - anchor.0),
            f64::from(position.1 + pivot.1 - anchor.1),
        );
        let transform = Transform::new(origin, dest, f64::from(p.rotation) / 10.0, scale);
        let bitmap = &image.surface;
        let blend = match p.composite {
            1 => Blend::MaskAdd,
            2 => Blend::MaskSub,
            _ => Blend::Mask,
        };
        let src_rect = Rect::from_corners(region.x1, region.y1, region.x2, region.y2);
        if p.has_effects() {
            let params = p.clone();
            let shade = move |px: [u8; 4]| params.shade(px);
            dst.draw_transformed(
                bitmap,
                src_rect,
                &transform,
                clip,
                alpha as u8,
                blend,
                Some(&shade),
            );
        } else {
            dst.draw_transformed(bitmap, src_rect, &transform, clip, alpha as u8, blend, None);
        }
    }

    fn render_drift(
        &self,
        dst: &mut Surface,
        image: &Image,
        now: u64,
        alpha: i32,
        clip: Rect,
        offset: (i32, i32),
    ) {
        let d = &self.params.drift;
        let area = Rect::new(d.area.x + offset.0, d.area.y + offset.1, d.area.w, d.area.h);
        let bitmap = &image.surface;
        let count = d.count.clamp(0, 256);
        let fall_time = d.y_time.max(1) as u64;
        for i in 0..count {
            // Deterministic pseudo-random placement per particle.
            let seed = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ 0x5851_f42d;
            let rand = |k: u64| {
                let mut z = seed.wrapping_add(k.wrapping_mul(0xbf58_476d_1ce4_e5b9));
                z = (z ^ (z >> 31)).wrapping_mul(0x94d0_49bb_1331_11eb);
                z ^ (z >> 29)
            };
            let phase = rand(1) % fall_time;
            let t = (now + phase) % fall_time;
            let cycle = (now + phase) / fall_time;
            let x0 = area.x + (rand(2 + cycle) % area.w.max(1) as u64) as i32;
            let y = area.y + (t as i64 * i64::from(area.h) / fall_time as i64) as i32;
            let sway = if d.use_drift != 0 && d.amplitude != 0 && d.period > 0 {
                let angle = (now + phase) as f64 / f64::from(d.period) * std::f64::consts::TAU;
                (angle.sin() * f64::from(d.amplitude)) as i32
            } else {
                0
            };
            let pattern = if d.use_animation != 0 && d.end_pattern >= d.start_pattern {
                let frames = (d.end_pattern - d.start_pattern + 1) as u64;
                let per = (d.animation_time.max(1) as u64 / frames).max(1);
                d.start_pattern + ((now + phase) / per % frames) as i32
            } else {
                self.params.pattern
            };
            let region = image.region(pattern);
            let src = Rect::from_corners(region.x1, region.y1, region.x2, region.y2);
            let transform =
                Transform::translation(f64::from(x0 + sway - src.x), f64::from(y - src.y));
            dst.draw_transformed(
                bitmap,
                src,
                &transform,
                clip,
                alpha.clamp(0, 255) as u8,
                Blend::Mask,
                None,
            );
        }
    }
}

/// Patterns to draw for a digits object: 0-9 digits, 10 `+`, 11 `-`,
/// 12 `±`; -1 is a blank cell.
pub fn digit_patterns(d: &DigitParams) -> Vec<i32> {
    let negative = d.value < 0;
    let digits: Vec<i32> = d
        .value
        .unsigned_abs()
        .to_string()
        .bytes()
        .map(|b| i32::from(b - b'0'))
        .collect();
    let sign = if negative {
        Some(11)
    } else if d.sign != 0 {
        Some(if d.value == 0 { 12 } else { 10 })
    } else {
        None
    };
    let width = d.digits.max(0) as usize;
    let counted_sign = usize::from(sign.is_some() && d.sign == 0);
    let pad_len = width.saturating_sub(digits.len() + counted_sign);
    let pad_value = if d.zero != 0 { 0 } else { -1 };
    let mut out = Vec::new();
    if d.zero != 0 || d.pack == 0 {
        // Sign first, then padding.
        out.extend(sign);
        out.extend(std::iter::repeat_n(pad_value, pad_len));
    } else {
        // Padding first, sign next to the number.
        out.extend(std::iter::repeat_n(pad_value, pad_len));
        out.extend(sign);
    }
    out.extend(digits);
    out
}

/// Parses the text-object control syntax (`#c`, `#d`, `#s`, `#x`, `#y`,
/// `##`) and renders the text.
pub fn render_text_object(
    params: &TextParams,
    fonts: &mut FontSet,
    colours: &[[u8; 3]],
) -> Surface {
    #[derive(Clone, Copy)]
    struct Placed {
        c: char,
        x: i32,
        y: i32,
        size: i32,
        colour: usize,
    }
    let base_size = params.size.max(1);
    let mut size = base_size;
    let mut colour = params.colour.max(0) as usize;
    let mut x = 0;
    let mut y = 0;
    let mut placed = Vec::new();
    let mut on_line = 0;
    let chars: Vec<char> = params.text.chars().collect();
    let mut i = 0;
    let read_number = |i: &mut usize| -> Option<i32> {
        let start = *i;
        let mut end = start;
        if chars.get(end) == Some(&'-') {
            end += 1;
        }
        while chars.get(end).is_some_and(|c| c.is_ascii_digit()) {
            end += 1;
        }
        let text: String = chars[start..end].iter().collect();
        let value = text.parse().ok();
        if value.is_some() {
            *i = end;
        }
        value
    };
    let line_height = |size: i32| size + params.yspace;
    while i < chars.len() {
        let c = chars[i];
        if c == '#' {
            match chars.get(i + 1).map(|c| c.to_ascii_lowercase()) {
                Some('#') => {
                    i += 2;
                    continue;
                }
                Some('d') => {
                    i += 2;
                    x = 0;
                    y += line_height(size);
                    on_line = 0;
                    continue;
                }
                Some('c') => {
                    i += 2;
                    colour = read_number(&mut i)
                        .map_or(params.colour.max(0) as usize, |v| v.max(0) as usize);
                    continue;
                }
                Some('s') => {
                    i += 2;
                    size = read_number(&mut i).unwrap_or(base_size).max(1);
                    continue;
                }
                Some('x') => {
                    i += 2;
                    x = read_number(&mut i).unwrap_or(0);
                    continue;
                }
                Some('y') => {
                    i += 2;
                    y = read_number(&mut i).unwrap_or(0);
                    continue;
                }
                _ => {}
            }
        }
        let cells = crate::nls::cell_width(c) as i32;
        let width = cells * size / 2;
        // `char_count` is measured in full-width characters.
        if params.char_count > 0 && on_line > 0 && on_line + cells > params.char_count * 2 {
            x = 0;
            y += line_height(size);
            on_line = 0;
        }
        placed.push(Placed {
            c,
            x,
            y,
            size,
            colour,
        });
        x += width + params.xspace;
        on_line += cells;
        i += 1;
    }
    let shadow = params.shadow >= 0;
    let width = placed
        .iter()
        .map(|p| p.x + crate::nls::cell_width(p.c) as i32 * p.size / 2)
        .max()
        .unwrap_or(0)
        + i32::from(shadow)
        + 2;
    let height = placed.iter().map(|p| p.y + p.size).max().unwrap_or(0) + i32::from(shadow) + 2;
    let mut surface = Surface {
        width: width.max(1),
        height: height.max(1),
        rgba: vec![0; (width.max(1) * height.max(1) * 4) as usize],
    };
    let colour_of = |index: usize| colours.get(index).copied().unwrap_or([255, 255, 255]);
    for item in &placed {
        let Some(glyph) = fonts.glyph(item.c, item.size as u32, false) else {
            continue;
        };
        let baseline = item.y + fonts.ascent(item.size as u32).round() as i32;
        let passes: &[(i32, [u8; 3])] = if shadow {
            &[
                (1, colour_of(params.shadow as usize)),
                (0, colour_of(item.colour)),
            ]
        } else {
            &[(0, colour_of(item.colour))]
        };
        for &(offset, rgb) in passes {
            for gy in 0..glyph.height {
                for gx in 0..glyph.width {
                    let coverage = glyph.coverage[gy * glyph.width + gx];
                    if coverage == 0 {
                        continue;
                    }
                    let tx = item.x + glyph.left + gx as i32 + offset;
                    let ty = baseline + glyph.top + gy as i32 + offset;
                    if tx < 0 || ty < 0 || tx >= surface.width || ty >= surface.height {
                        continue;
                    }
                    let at = ((ty * surface.width + tx) * 4) as usize;
                    let a = u32::from(coverage);
                    let old_a = u32::from(surface.rgba[at + 3]);
                    // Composite over whatever was drawn (shadow).
                    let out_a = a + old_a * (255 - a) / 255;
                    for c in 0..3 {
                        let old = u32::from(surface.rgba[at + c]) * old_a * (255 - a) / 255;
                        let new = u32::from(rgb[c]) * a;
                        surface.rgba[at + c] = (new + old)
                            .checked_div(out_a)
                            .map_or(0, |v| v.min(255) as u8);
                    }
                    surface.rgba[at + 3] = out_a.min(255) as u8;
                }
            }
        }
    }
    surface
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_follow_padding_rules() {
        let d = |value, digits, zero, sign, pack| DigitParams {
            value,
            digits,
            zero,
            sign,
            pack,
            space: 0,
        };
        assert_eq!(digit_patterns(&d(42, 4, 1, 0, 0)), vec![0, 0, 4, 2]);
        assert_eq!(digit_patterns(&d(42, 4, 0, 0, 0)), vec![-1, -1, 4, 2]);
        assert_eq!(digit_patterns(&d(-5, 3, 0, 0, 1)), vec![-1, 11, 5]);
        assert_eq!(digit_patterns(&d(-5, 3, 0, 0, 0)), vec![11, -1, 5]);
        assert_eq!(digit_patterns(&d(0, 2, 1, 1, 0)), vec![12, 0, 0]);
    }

    #[test]
    fn mutators_interpolate_and_finish() {
        let mut params = ObjectParams::default();
        let mutator = Mutator {
            id: 0,
            repno: 0,
            targets: vec![(Property::X, 0, 100)],
            start: 0,
            duration: 100,
            delay: 50,
            curve: 0,
        };
        assert!(!mutator.apply(&mut params, 10));
        assert_eq!(params.x, 0);
        assert!(!mutator.apply(&mut params, 100));
        assert_eq!(params.x, 50);
        assert!(mutator.apply(&mut params, 150));
        assert_eq!(params.x, 100);
    }

    #[test]
    fn origin_places_the_pattern() {
        let mut image = Image::new(4, 4);
        for pixel in image.surface.rgba.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[200, 0, 0, 255]);
        }
        let mut object = Object::new(ObjectData::File {
            name: "a".into(),
            image: Rc::new(image),
        });
        object.params.visible = true;
        object.params.x = 10;
        object.params.y = 10;
        object.params.origin = (2, 2);
        let mut dst = Surface::new(20, 20);
        let mut fonts = FontSet::empty();
        let mut ctx = RenderContext {
            fonts: &mut fonts,
            colours: &[],
            now: 0,
            screen: Rect::new(0, 0, 20, 20),
        };
        object.render(&mut dst, &mut ctx, None);
        assert_eq!(dst.pixel(8, 8)[0], 200);
        assert_eq!(dst.pixel(11, 11)[0], 200);
        assert_eq!(dst.pixel(12, 12)[0], 0);
        assert_eq!(dst.pixel(7, 7)[0], 0);
    }
}
