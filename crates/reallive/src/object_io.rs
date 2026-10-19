//! Saving and restoring objects. Parameters are stored in full; bitmaps
//! and GAN files are stored by name and reloaded.

use std::rc::Rc;

use anyhow::Result;

use crate::gan::Gan;
use crate::graphics::Graphics;
use crate::object::{
    AfterAnimation, Animation, Mutator, Object, ObjectData, ObjectParams, Property,
};
use crate::resource::{Kind, Resources};
use crate::serial::{Reader, Writer};
use crate::surface::Rect;

fn write_rect(w: &mut Writer, rect: Option<Rect>) {
    match rect {
        Some(r) => {
            w.bool(true);
            w.i32s(&[r.x, r.y, r.w, r.h]);
        }
        None => w.bool(false),
    }
}

fn read_rect(r: &mut Reader) -> Result<Option<Rect>> {
    if !r.bool()? {
        return Ok(None);
    }
    let v = r.i32s()?;
    Ok(match v[..] {
        [x, y, w, h] => Some(Rect::new(x, y, w, h)),
        _ => None,
    })
}

fn write_params(w: &mut Writer, p: &ObjectParams) {
    w.bool(p.visible);
    w.i32s(&[
        p.x,
        p.y,
        p.adjust_vert,
        p.origin.0,
        p.origin.1,
        p.rep_origin.0,
        p.rep_origin.1,
    ]);
    w.i32s(&p.adjust_x);
    w.i32s(&p.adjust_y);
    w.i32s(&[
        p.width,
        p.height,
        p.hq_width,
        p.hq_height,
        p.rotation,
        p.pattern,
        p.alpha,
    ]);
    w.i32s(&p.adjust_alpha);
    write_rect(w, p.clip);
    write_rect(w, p.own_clip);
    w.i32s(&[p.mono, p.invert, p.light, p.composite]);
    w.i32s(&p.tint);
    w.i32s(&p.colour);
    w.i32s(&[
        p.scroll_rate.0,
        p.scroll_rate.1,
        p.z_order,
        p.z_layer,
        p.z_depth,
        p.quarter_view,
    ]);
    w.bool(p.wipe_copy);
    let t = &p.text;
    w.str(&t.text);
    w.i32s(&[t.size, t.xspace, t.yspace, t.char_count, t.colour, t.shadow]);
    let d = &p.digits;
    w.i32s(&[d.value, d.digits, d.zero, d.sign, d.pack, d.space]);
    let f = &p.drift;
    w.i32s(&[
        f.count,
        f.use_animation,
        f.start_pattern,
        f.end_pattern,
        f.animation_time,
        f.y_time,
        f.period,
        f.amplitude,
        f.use_drift,
        f.unknown,
        f.drift_speed,
        f.area.x,
        f.area.y,
        f.area.w,
        f.area.h,
    ]);
    let b = &p.button;
    w.i32s(&[b.is_button, b.action, b.se, b.group, b.number, b.state]);
    w.i32s(&p.fade);
}

fn read_params(r: &mut Reader) -> Result<ObjectParams> {
    let mut p = ObjectParams {
        visible: r.bool()?,
        ..ObjectParams::default()
    };
    if let [x, y, vert, ox, oy, rx, ry] = r.i32s()?[..] {
        p.x = x;
        p.y = y;
        p.adjust_vert = vert;
        p.origin = (ox, oy);
        p.rep_origin = (rx, ry);
    }
    crate::serial::fill(&mut p.adjust_x, &r.i32s()?);
    crate::serial::fill(&mut p.adjust_y, &r.i32s()?);
    if let [w, h, hw, hh, rot, pat, a] = r.i32s()?[..] {
        p.width = w;
        p.height = h;
        p.hq_width = hw;
        p.hq_height = hh;
        p.rotation = rot;
        p.pattern = pat;
        p.alpha = a;
    }
    crate::serial::fill(&mut p.adjust_alpha, &r.i32s()?);
    p.clip = read_rect(r)?;
    p.own_clip = read_rect(r)?;
    if let [mono, invert, light, composite] = r.i32s()?[..] {
        p.mono = mono;
        p.invert = invert;
        p.light = light;
        p.composite = composite;
    }
    crate::serial::fill(&mut p.tint, &r.i32s()?);
    crate::serial::fill(&mut p.colour, &r.i32s()?);
    if let [sx, sy, order, layer, depth, qview] = r.i32s()?[..] {
        p.scroll_rate = (sx, sy);
        p.z_order = order;
        p.z_layer = layer;
        p.z_depth = depth;
        p.quarter_view = qview;
    }
    p.wipe_copy = r.bool()?;
    p.text.text = r.str()?;
    if let [size, xs, ys, count, colour, shadow] = r.i32s()?[..] {
        p.text.size = size;
        p.text.xspace = xs;
        p.text.yspace = ys;
        p.text.char_count = count;
        p.text.colour = colour;
        p.text.shadow = shadow;
    }
    if let [value, digits, zero, sign, pack, space] = r.i32s()?[..] {
        p.digits.value = value;
        p.digits.digits = digits;
        p.digits.zero = zero;
        p.digits.sign = sign;
        p.digits.pack = pack;
        p.digits.space = space;
    }
    let f = r.i32s()?;
    if f.len() == 15 {
        let d = &mut p.drift;
        d.count = f[0];
        d.use_animation = f[1];
        d.start_pattern = f[2];
        d.end_pattern = f[3];
        d.animation_time = f[4];
        d.y_time = f[5];
        d.period = f[6];
        d.amplitude = f[7];
        d.use_drift = f[8];
        d.unknown = f[9];
        d.drift_speed = f[10];
        d.area = Rect::new(f[11], f[12], f[13], f[14]);
    }
    if let [is_button, action, se, group, number, state] = r.i32s()?[..] {
        p.button.is_button = is_button;
        p.button.action = action;
        p.button.se = se;
        p.button.group = group;
        p.button.number = number;
        p.button.state = state;
    }
    crate::serial::fill(&mut p.fade, &r.i32s()?);
    Ok(p)
}

fn property_code(property: Property) -> (i32, i32) {
    match property {
        Property::X => (0, 0),
        Property::Y => (1, 0),
        Property::Alpha => (2, 0),
        Property::AdjustX(i) => (3, i as i32),
        Property::AdjustY(i) => (4, i as i32),
        Property::Mono => (5, 0),
        Property::Invert => (6, 0),
        Property::Light => (7, 0),
        Property::TintR => (8, 0),
        Property::TintG => (9, 0),
        Property::TintB => (10, 0),
        Property::ColR => (11, 0),
        Property::ColG => (12, 0),
        Property::ColB => (13, 0),
        Property::ColLevel => (14, 0),
        Property::AdjustVert => (15, 0),
        Property::AdjustAlpha(i) => (16, i as i32),
        Property::Width => (17, 0),
        Property::Height => (18, 0),
        Property::Rotation => (19, 0),
        Property::RepOriginX => (20, 0),
        Property::RepOriginY => (21, 0),
        Property::OriginX => (22, 0),
        Property::OriginY => (23, 0),
        Property::HqWidth => (24, 0),
        Property::HqHeight => (25, 0),
        Property::Visible => (26, 0),
        Property::ClipX => (27, 0),
        Property::ClipY => (28, 0),
        Property::ClipW => (29, 0),
        Property::ClipH => (30, 0),
        Property::ClipRight => (31, 0),
        Property::ClipBottom => (32, 0),
        Property::OwnClipX => (33, 0),
        Property::OwnClipY => (34, 0),
        Property::OwnClipW => (35, 0),
        Property::OwnClipH => (36, 0),
        Property::OwnClipRight => (37, 0),
        Property::OwnClipBottom => (38, 0),
    }
}

fn property_from_code(code: i32, index: i32) -> Option<Property> {
    let i = (index & 7) as usize;
    Some(match code {
        0 => Property::X,
        1 => Property::Y,
        2 => Property::Alpha,
        3 => Property::AdjustX(i),
        4 => Property::AdjustY(i),
        5 => Property::Mono,
        6 => Property::Invert,
        7 => Property::Light,
        8 => Property::TintR,
        9 => Property::TintG,
        10 => Property::TintB,
        11 => Property::ColR,
        12 => Property::ColG,
        13 => Property::ColB,
        14 => Property::ColLevel,
        15 => Property::AdjustVert,
        16 => Property::AdjustAlpha(i),
        17 => Property::Width,
        18 => Property::Height,
        19 => Property::Rotation,
        20 => Property::RepOriginX,
        21 => Property::RepOriginY,
        22 => Property::OriginX,
        23 => Property::OriginY,
        24 => Property::HqWidth,
        25 => Property::HqHeight,
        26 => Property::Visible,
        27 => Property::ClipX,
        28 => Property::ClipY,
        29 => Property::ClipW,
        30 => Property::ClipH,
        31 => Property::ClipRight,
        32 => Property::ClipBottom,
        33 => Property::OwnClipX,
        34 => Property::OwnClipY,
        35 => Property::OwnClipW,
        36 => Property::OwnClipH,
        37 => Property::OwnClipRight,
        38 => Property::OwnClipBottom,
        _ => return None,
    })
}

pub fn write(w: &mut Writer, object: &Object) {
    write_params(w, &object.params);
    match &object.data {
        None => w.u8(0),
        Some(ObjectData::File { name, .. }) => {
            w.u8(1);
            w.str(name);
        }
        Some(ObjectData::Gan {
            image_name,
            gan_name,
            ..
        }) => {
            w.u8(2);
            w.str(image_name);
            w.str(gan_name);
        }
        Some(ObjectData::Text) => w.u8(3),
        Some(ObjectData::Digits { name, .. }) => {
            w.u8(4);
            w.str(name);
        }
        Some(ObjectData::Drift { name, .. }) => {
            w.u8(5);
            w.str(name);
        }
        Some(ObjectData::Rect(rect)) => {
            w.u8(6);
            w.i32s(&[rect.x, rect.y, rect.w, rect.h]);
        }
        Some(ObjectData::Parent(children)) => {
            w.u8(7);
            let present: Vec<(usize, &Object)> = children
                .iter()
                .enumerate()
                .filter_map(|(i, c)| c.as_ref().map(|c| (i, c)))
                .collect();
            w.len(present.len());
            for (index, child) in present {
                w.u32(index as u32);
                write(w, child);
            }
        }
    }
    match &object.animation {
        Some(animation) => {
            w.bool(true);
            w.u64(animation.start);
            w.u8(match animation.after {
                AfterAnimation::Stop => 0,
                AfterAnimation::Clear => 1,
                AfterAnimation::Loop => 2,
                AfterAnimation::Blink => 3,
            });
            w.i32(animation.parameter);
            w.bool(animation.finished);
        }
        None => w.bool(false),
    }
    w.len(object.mutators.len());
    for mutator in &object.mutators {
        w.i32s(&[
            mutator.id,
            mutator.repno,
            mutator.duration,
            mutator.delay,
            mutator.curve,
        ]);
        w.u64(mutator.start);
        w.len(mutator.targets.len());
        for &(property, from, to) in &mutator.targets {
            let (code, index) = property_code(property);
            w.i32s(&[code, index, from, to]);
        }
    }
}

pub fn read(r: &mut Reader, graphics: &mut Graphics, resources: &Resources) -> Result<Object> {
    let params = read_params(r)?;
    let data = match r.u8()? {
        1 => {
            let name = r.str()?;
            graphics
                .load_image(resources, &name)
                .ok()
                .map(|image| ObjectData::File { name, image })
        }
        2 => {
            let image_name = r.str()?;
            let gan_name = r.str()?;
            let image = graphics.load_image(resources, &image_name).ok();
            let gan = resources
                .read(Kind::Gan, &gan_name)
                .and_then(|bytes| Gan::parse(&bytes).ok());
            match (image, gan) {
                (Some(image), Some(gan)) => Some(ObjectData::Gan {
                    image_name,
                    image,
                    gan_name,
                    gan: Rc::new(gan),
                }),
                _ => None,
            }
        }
        3 => Some(ObjectData::Text),
        4 => {
            let name = r.str()?;
            graphics
                .load_image(resources, &name)
                .ok()
                .map(|image| ObjectData::Digits { name, image })
        }
        5 => {
            let name = r.str()?;
            graphics
                .load_image(resources, &name)
                .ok()
                .map(|image| ObjectData::Drift { name, image })
        }
        6 => match r.i32s()?[..] {
            [x, y, w, h] => Some(ObjectData::Rect(Rect::new(x, y, w, h))),
            _ => None,
        },
        7 => {
            let mut children = vec![None; crate::object::OBJECT_COUNT];
            let count = r.len()?;
            for _ in 0..count {
                let index = r.u32()? as usize;
                let child = read(r, graphics, resources)?;
                if index < children.len() {
                    children[index] = Some(child);
                }
            }
            Some(ObjectData::Parent(children))
        }
        _ => None,
    };
    let mut object = match data {
        Some(data) => Object::new(data),
        None => Object::empty(),
    };
    object.params = params;
    if r.bool()? {
        let start = r.u64()?;
        let after = match r.u8()? {
            1 => AfterAnimation::Clear,
            2 => AfterAnimation::Loop,
            3 => AfterAnimation::Blink,
            _ => AfterAnimation::Stop,
        };
        object.animation = Some(Animation {
            start,
            after,
            parameter: r.i32()?,
            finished: r.bool()?,
        });
    }
    let mutators = r.len()?;
    for _ in 0..mutators {
        let values = r.i32s()?;
        let start = r.u64()?;
        let targets_len = r.len()?;
        let mut targets = Vec::new();
        for _ in 0..targets_len {
            if let [code, index, from, to] = r.i32s()?[..] {
                if let Some(property) = property_from_code(code, index) {
                    targets.push((property, from, to));
                }
            }
        }
        if let [id, repno, duration, delay, curve] = values[..] {
            object.mutators.push(Mutator {
                id,
                repno,
                targets,
                start,
                duration,
                delay,
                curve,
            });
        }
    }
    Ok(object)
}
