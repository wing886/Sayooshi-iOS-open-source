//! Graphics objects: management (60-62), creation (71/72), animation
//! (73/74), properties and `objEve*` (81/82), getters (84/85) and ranges
//! (90/91). Modtype 2 addresses child objects: the first parameter is the
//! parent.
//!
//! Property functions are organised as in the interpreter: base id `n`
//! gives `obj*` at 1000+n, `objEve*` at 2000+n (immediate with the plain
//! argument count, animated with duration, delay and curve appended),
//! `*Check` at 3000+n, `*Wait` at 4000+n, `*WaitC` at 5000+n and `*End`
//! at 6000+n.

use std::rc::Rc;

use anyhow::{Result, bail};

use crate::bytecode::Command;
use crate::gan::Gan;
use crate::graphics::ObjectRef;
use crate::machine::{LongOp, Machine, Next};
use crate::object::{
    AfterAnimation, Animation, CURVE_WAVE, Mutator, OBJECT_COUNT, Object, ObjectData, Property,
};
use crate::resource::Kind;
use crate::surface::Rect;

/// The objects a command addresses, and where its own arguments start.
struct Targets {
    refs: Vec<ObjectRef>,
    args: usize,
}

fn is_bg(module: u8) -> bool {
    matches!(module, 62 | 72 | 74 | 82 | 85 | 88 | 91 | 94)
}

fn targets(machine: &mut Machine, command: &Command, range: bool) -> Result<Targets> {
    let bg = is_bg(command.op.module);
    let child = command.op.modtype == 2;
    let mut at = 0;
    let parent = if child {
        at += 1;
        Some(machine.int_param(command, 0)?)
    } else {
        None
    };
    let make = |buf: i32| match parent {
        Some(parent) => ObjectRef {
            bg,
            buf: parent,
            child: Some(buf),
        },
        None => ObjectRef {
            bg,
            buf,
            child: None,
        },
    };
    let refs = if range {
        let min = machine.int_param(command, at)?;
        let max = machine.int_param(command, at + 1)?;
        at += 2;
        (min.max(0)..=max.min(OBJECT_COUNT as i32 - 1))
            .map(make)
            .collect()
    } else {
        let buf = machine.int_param(command, at)?;
        at += 1;
        vec![make(buf)]
    };
    Ok(Targets { refs, args: at })
}

/// Integer arguments from `from` on.
fn ints(machine: &mut Machine, command: &Command, from: usize) -> Result<Vec<i32>> {
    machine.int_params_from(command, from)
}

fn each(machine: &mut Machine, refs: &[ObjectRef], mut f: impl FnMut(&mut Object)) -> Result<()> {
    for &r in refs {
        f(machine.sys.gfx.object_mut(r)?);
    }
    Ok(())
}

// ---- property families ---------------------------------------------------------

type PropertyOf = fn(usize) -> Property;

/// Base id → (takes a repno, properties set in order).
fn family(base: u16) -> Option<(bool, &'static [PropertyOf])> {
    Some(match base {
        0 => (false, &[|_| Property::X, |_| Property::Y]),
        1 => (false, &[|_| Property::X]),
        2 => (false, &[|_| Property::Y]),
        3 => (false, &[|_| Property::Alpha]),
        4 => (false, &[|_| Property::Visible]),
        6 => (true, &[Property::AdjustX, Property::AdjustY]),
        7 => (true, &[Property::AdjustX]),
        8 => (true, &[Property::AdjustY]),
        9 => (false, &[|_| Property::Mono]),
        10 => (false, &[|_| Property::Invert]),
        11 => (false, &[|_| Property::Light]),
        12 => (
            false,
            &[
                |_| Property::TintR,
                |_| Property::TintG,
                |_| Property::TintB,
            ],
        ),
        13 => (false, &[|_| Property::TintR]),
        14 => (false, &[|_| Property::TintG]),
        15 => (false, &[|_| Property::TintB]),
        16 => (
            false,
            &[
                |_| Property::ColR,
                |_| Property::ColG,
                |_| Property::ColB,
                |_| Property::ColLevel,
            ],
        ),
        17 => (false, &[|_| Property::ColR]),
        18 => (false, &[|_| Property::ColG]),
        19 => (false, &[|_| Property::ColB]),
        20 => (false, &[|_| Property::ColLevel]),
        34 => (
            false,
            &[
                |_| Property::ClipX,
                |_| Property::ClipY,
                |_| Property::ClipW,
                |_| Property::ClipH,
            ],
        ),
        35 => (
            false,
            &[
                |_| Property::ClipX,
                |_| Property::ClipY,
                |_| Property::ClipRight,
                |_| Property::ClipBottom,
            ],
        ),
        5 => (
            false,
            &[
                |_| Property::ClipX,
                |_| Property::ClipY,
                |_| Property::ClipRight,
                |_| Property::ClipBottom,
            ],
        ),
        36 => (false, &[|_| Property::AdjustVert]),
        70 | 72 => (
            false,
            &[
                |_| Property::OwnClipX,
                |_| Property::OwnClipY,
                |_| Property::OwnClipRight,
                |_| Property::OwnClipBottom,
            ],
        ),
        71 => (
            false,
            &[
                |_| Property::OwnClipX,
                |_| Property::OwnClipY,
                |_| Property::OwnClipW,
                |_| Property::OwnClipH,
            ],
        ),
        40 => (true, &[Property::AdjustAlpha]),
        46 => (false, &[|_| Property::Width, |_| Property::Height]),
        47 => (false, &[|_| Property::Width]),
        48 => (false, &[|_| Property::Height]),
        49 => (false, &[|_| Property::Rotation]),
        50 => (false, &[|_| Property::RepOriginX, |_| Property::RepOriginY]),
        51 => (false, &[|_| Property::RepOriginX]),
        52 => (false, &[|_| Property::RepOriginY]),
        53 => (false, &[|_| Property::OriginX, |_| Property::OriginY]),
        54 => (false, &[|_| Property::OriginX]),
        55 => (false, &[|_| Property::OriginY]),
        61 => (false, &[|_| Property::HqWidth, |_| Property::HqHeight]),
        62 => (false, &[|_| Property::HqWidth]),
        63 => (false, &[|_| Property::HqHeight]),
        _ => return None,
    })
}

/// Splits `values` into (repno, property values, animation tail).
fn split_family(repno: bool, count: usize, values: &[i32]) -> Result<(usize, Vec<i32>, Vec<i32>)> {
    let skip = usize::from(repno);
    if values.len() < skip + count {
        bail!(
            "reallive: expected {} arguments, got {}",
            skip + count,
            values.len()
        );
    }
    let index = if repno {
        values[0].clamp(0, 7) as usize
    } else {
        0
    };
    let props = values[skip..skip + count].to_vec();
    let tail = values[skip + count..].to_vec();
    Ok((index, props, tail))
}

fn set_family(machine: &mut Machine, t: &Targets, base: u16, values: &[i32]) -> Result<()> {
    let (repno, props) = family(base).expect("checked by caller");
    let (index, props_values, _) = split_family(repno, props.len(), values)?;
    each(machine, &t.refs, |object| {
        for (property, &value) in props.iter().zip(&props_values) {
            property(index).set(&mut object.params, value);
        }
    })
}

fn animate_family(machine: &mut Machine, t: &Targets, base: u16, values: &[i32]) -> Result<()> {
    let (repno, props) = family(base).expect("checked by caller");
    let (index, props_values, tail) = split_family(repno, props.len(), values)?;
    if tail.is_empty() {
        return set_family(machine, t, base, values);
    }
    let now = machine.sys.now();
    let duration = tail.first().copied().unwrap_or(0);
    let delay = tail.get(1).copied().unwrap_or(0);
    let curve = tail.get(2).copied().unwrap_or(0);
    each(machine, &t.refs, |object| {
        let targets = props
            .iter()
            .zip(&props_values)
            .map(|(property, &to)| {
                let property = property(index);
                (property, property.get(&object.params), to)
            })
            .collect();
        // A new animation of the same property replaces the old one.
        object.end_mutators(i32::from(base), repno.then_some(index as i32));
        object.mutators.push(Mutator {
            id: i32::from(base),
            repno: if repno { index as i32 } else { -1 },
            targets,
            start: now,
            duration,
            delay,
            curve,
        });
    })
}

/// `objEveDisplay` with an `#OBJDISP` style description: fade, slide,
/// spin, stretch and a damped vertical wave, all at once.
#[allow(clippy::too_many_arguments)]
fn display_mutators(
    object: &Object,
    display: bool,
    duration: i32,
    delay: i32,
    now: u64,
    spec: &[i32],
) -> Vec<Mutator> {
    let get = |i: usize| spec.get(i).copied().unwrap_or(0);
    let p = &object.params;
    let (tr, mv, (dx, dy)) = (get(1), get(2), (get(3), get(4)));
    let (rotate, turns) = (get(5), get(6));
    let (scale_x, percent_x, scale_y, percent_y) = (get(7), get(8), get(9), get(10));
    let (sin, sin_len, sin_count) = (get(11), get(12), get(13));
    // (value when hidden, value when shown)
    let span = |hidden: i32, shown: i32| {
        if display {
            (hidden, shown)
        } else {
            (shown, hidden)
        }
    };
    let mut targets = vec![(Property::Visible, 1, i32::from(display))];
    if tr != 0 {
        let (from, to) = span(0, 255);
        targets.push((Property::Alpha, from, to));
    }
    if mv != 0 {
        let (x, y) = if display {
            (p.x - dx, p.y - dy)
        } else {
            (p.x + dx, p.y + dy)
        };
        let (to_x, to_y) = if display { (p.x, p.y) } else { (x, y) };
        let (from_x, from_y) = if display { (x, y) } else { (p.x, p.y) };
        targets.push((Property::X, from_x, to_x));
        targets.push((Property::Y, from_y, to_y));
    }
    if rotate != 0 {
        let (from, to) = span(p.rotation - 3600 * turns, p.rotation);
        targets.push((Property::Rotation, from, to));
    }
    if scale_x != 0 {
        let (from, to) = span(percent_x, p.width);
        targets.push((Property::Width, from, to));
    }
    if scale_y != 0 {
        let (from, to) = span(percent_y, p.height);
        targets.push((Property::Height, from, to));
    }
    let mut mutators = vec![Mutator {
        id: 4,
        repno: -1,
        targets,
        start: now,
        duration,
        delay,
        curve: 0,
    }];
    if sin != 0 && sin_len != 0 {
        mutators.push(Mutator {
            id: 4,
            repno: -1,
            targets: vec![(Property::AdjustVert, sin_len, sin_count.max(1))],
            start: now,
            duration,
            delay,
            curve: CURVE_WAVE,
        });
    }
    mutators
}

fn eve_display(machine: &mut Machine, t: &Targets, values: &[i32]) -> Result<()> {
    let display = values.first().copied().unwrap_or(0) != 0;
    if values.len() <= 1 {
        return each(machine, &t.refs, |object| object.params.visible = display);
    }
    let duration = values.get(1).copied().unwrap_or(0);
    let delay = values.get(2).copied().unwrap_or(0);
    let spec: Vec<i32> = if values.len() == 4 {
        // Overload 1: a preset from `#OBJDISP.nnn`.
        machine.gameexe.ints(&format!("OBJDISP.{:03}", values[3]))
    } else {
        values[3..].to_vec()
    };
    let now = machine.sys.now();
    each(machine, &t.refs, |object| {
        object.end_mutators(4, None);
        let mutators = display_mutators(object, display, duration, delay, now, &spec);
        object.mutators.extend(mutators);
    })
}

/// Waits until the given objects have finished a mutator family (or all
/// animations, for `id` None).
#[derive(Debug)]
struct MutatorWait {
    refs: Vec<ObjectRef>,
    id: Option<i32>,
    repno: Option<i32>,
    clickable: bool,
}

impl MutatorWait {
    fn busy(&self, machine: &Machine) -> bool {
        self.refs.iter().any(|&r| {
            machine
                .sys
                .gfx
                .object(r)
                .is_some_and(|object| match self.id {
                    Some(id) => object.has_mutator(id, self.repno),
                    None => object.is_animating(),
                })
        })
    }

    fn finish(&self, machine: &mut Machine) {
        for &r in &self.refs {
            if let Ok(object) = machine.sys.gfx.object_mut(r) {
                match self.id {
                    Some(id) => object.end_mutators(id, self.repno),
                    None => {
                        if let Some(animation) = &mut object.animation {
                            animation.start = 0;
                        }
                    }
                }
            }
        }
    }
}

impl LongOp for MutatorWait {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let skip = machine.sys.should_fast_forward()
            || (self.clickable && machine.sys.input.take_click().is_some());
        if skip {
            self.finish(machine);
            return Ok(true);
        }
        Ok(!self.busy(machine))
    }

    fn name(&self) -> &'static str {
        "object wait"
    }
}

fn property_function(machine: &mut Machine, command: &Command, t: &Targets) -> Result<bool> {
    let opcode = command.op.opcode;
    let (kind, base) = (opcode / 1000, opcode % 1000);
    let event = (200..=216).contains(&base);
    if !(1..=6).contains(&kind) || (family(base).is_none() && !matches!(base, 4 | 21) && !event) {
        return Ok(false);
    }
    if (base == 21 || event) && !(3..=6).contains(&kind) {
        return Ok(false);
    }
    // objRectEria / objBoxEria themselves also clear, and take shorter forms.
    if kind == 1 && matches!(base, 5 | 34 | 35 | 70 | 71 | 72) {
        return Ok(false);
    }
    let values = ints(machine, command, t.args)?;
    let repno = family(base).is_some_and(|(repno, _)| repno);
    let repno_arg = if repno { values.first().copied() } else { None };
    // *EVE_ALLEVE_*: every running animation of the object.
    let id = if base == 21 {
        crate::object::ALL_MUTATORS
    } else if event {
        2000 + i32::from(base)
    } else {
        i32::from(base)
    };
    match kind {
        1 => set_family(machine, t, base, &values)?,
        2 if base == 4 => eve_display(machine, t, &values)?,
        2 => animate_family(machine, t, base, &values)?,
        3 => {
            machine.store = i32::from(t.refs.iter().any(|&r| {
                machine
                    .sys
                    .gfx
                    .object(r)
                    .is_some_and(|object| object.has_mutator(id, repno_arg))
            }));
        }
        4 | 5 => machine.push_long_op(Box::new(MutatorWait {
            refs: t.refs.clone(),
            id: Some(id),
            repno: repno_arg,
            clickable: kind == 5,
        })),
        _ => each(machine, &t.refs, |object| {
            object.end_mutators(id, repno_arg)
        })?,
    }
    Ok(true)
}

fn grp_rect(v: &[i32]) -> Rect {
    Rect::from_corners(v[0], v[1], v[2], v[3])
}

/// Modules 81/82 and ranges 90/91.
pub fn properties(machine: &mut Machine, command: &Command) -> Result<Next> {
    let range = matches!(command.op.module, 90 | 91 | 93 | 94);
    // OBJFRONTADD(S) / OBJBACKADD(S) (87, 88, 93, 94): add to the value;
    // their 12xx functions stop object events where they are.
    if matches!(command.op.module, 87 | 88 | 93 | 94) {
        let t = targets(machine, command, range)?;
        return add_or_stop(machine, command, &t);
    }
    let t = targets(machine, command, range)?;
    if property_function(machine, command, &t)? {
        return Ok(Next::Advance);
    }
    let v = ints_lenient(machine, command, t.args)?;
    let get = |i: usize| v.get(i).copied().unwrap_or(0);
    if (2200..=2216).contains(&command.op.opcode) {
        object_event(machine, &t, command.op.opcode, &v)?;
        return Ok(Next::Advance);
    }
    match command.op.opcode {
        1005 if v.len() >= 4 => each(machine, &t.refs, |o| o.params.clip = Some(grp_rect(&v)))?,
        1005 | 1034 | 1035 if v.is_empty() => each(machine, &t.refs, |o| o.params.clip = None)?,
        1034 => each(machine, &t.refs, |o| {
            o.params.clip = Some(Rect::new(get(0), get(1), get(2), get(3)))
        })?,
        1035 if v.len() >= 4 => each(machine, &t.refs, |o| o.params.clip = Some(grp_rect(&v)))?,
        1035 => each(machine, &t.refs, |o| {
            o.params.clip = Some(Rect::from_corners(0, 0, get(0), get(1)))
        })?,
        1021 => each(machine, &t.refs, |o| o.params.composite = get(0))?,
        1022 => {
            let area = if v.len() >= 4 {
                grp_rect(&v)
            } else {
                Rect::new(0, 0, machine.sys.gfx.width, machine.sys.gfx.height)
            };
            each(machine, &t.refs, |o| {
                if let Some(ObjectData::Rect(rect)) = &mut o.data {
                    *rect = area;
                }
            })?;
        }
        1024 => {
            let text = if Machine::param_count(command) > t.args {
                machine.str_param(command, t.args)?
            } else {
                String::new()
            };
            each(machine, &t.refs, |o| o.params.text.text = text.clone())?;
        }
        1025 => each(machine, &t.refs, |o| {
            let text = &mut o.params.text;
            text.size = get(0);
            text.xspace = get(1);
            text.yspace = get(2);
            text.char_count = get(3);
            text.colour = get(4);
            text.shadow = v.get(5).copied().unwrap_or(-1);
        })?,
        1026 => each(machine, &t.refs, |o| o.params.z_layer = get(0))?,
        1027 => each(machine, &t.refs, |o| o.params.z_depth = get(0))?,
        1028 => each(machine, &t.refs, |o| {
            o.params.scroll_rate = (get(0), get(1))
        })?,
        1029 => each(machine, &t.refs, |o| o.params.scroll_rate.0 = get(0))?,
        1030 => each(machine, &t.refs, |o| o.params.scroll_rate.1 = get(0))?,
        1031 => each(machine, &t.refs, |o| {
            let d = &mut o.params.drift;
            d.count = get(0);
            d.use_animation = get(1);
            d.start_pattern = get(2);
            d.end_pattern = get(3);
            d.animation_time = get(4);
            d.y_time = get(5);
            d.period = get(6);
            d.amplitude = get(7);
            d.use_drift = get(8);
            d.unknown = get(9);
            d.drift_speed = get(10);
            if v.len() >= 15 {
                d.area = grp_rect(&v[11..15]);
            }
        })?,
        1032 => each(machine, &t.refs, |o| o.params.z_order = get(0))?,
        1033 => each(machine, &t.refs, |o| o.params.quarter_view = get(0))?,
        1037 => each(machine, &t.refs, |o| o.params.digits.value = get(0))?,
        1038 => each(machine, &t.refs, |o| {
            let d = &mut o.params.digits;
            d.digits = get(0);
            d.zero = get(1);
            d.sign = get(2);
            d.pack = get(3);
            d.space = get(4);
        })?,
        1039 => each(machine, &t.refs, |o| o.params.pattern = get(0))?,
        // objAdjustAll(x, y) / X / Y: every repno's offset.
        1041 => each(machine, &t.refs, |o| {
            o.params.adjust_x = [0; 8];
            o.params.adjust_y = [0; 8];
            o.params.adjust_x[0] = get(0);
            o.params.adjust_y[0] = get(1);
        })?,
        1042 => each(machine, &t.refs, |o| {
            o.params.adjust_x = [0; 8];
            o.params.adjust_x[0] = get(0);
        })?,
        1043 => each(machine, &t.refs, |o| {
            o.params.adjust_y = [0; 8];
            o.params.adjust_y[0] = get(0);
        })?,
        // OBJFRONTSET_ALLREPTR(alpha): every repno's alpha.
        1044 => each(machine, &t.refs, |o| o.params.adjust_alpha = [get(0); 8])?,
        // OBJFRONTSET_GANPATNO(set, pattern): show one GAN frame.
        1045 => {
            let now = machine.sys.now();
            let (set, pattern) = (get(0), get(1));
            each(machine, &t.refs, |o| {
                if let Some(ObjectData::Gan { gan, .. }) = &o.data {
                    let offset: i32 = gan
                        .sets
                        .get(set.max(0) as usize)
                        .map(|frames| {
                            frames
                                .iter()
                                .take(pattern.max(0) as usize)
                                .map(|f| f.time.max(1))
                                .sum()
                        })
                        .unwrap_or(0);
                    o.animation = Some(Animation {
                        start: now.saturating_sub(offset as u64),
                        after: AfterAnimation::Stop,
                        parameter: set,
                        finished: false,
                    });
                }
            })?;
            // Frozen on that frame.
            for r in &t.refs {
                if !r.bg && r.child.is_none() && r.buf >= 0 {
                    machine.sys.gfx.pause_animation(r.buf as usize, now);
                }
            }
        }
        // OBJFRONTSET_WEATHER_REPCNT(count) / _WEATHERPARAM_TYPEB(...)
        1057 => each(machine, &t.refs, |o| o.params.drift.count = get(0))?,
        1059 => each(machine, &t.refs, |o| {
            let d = &mut o.params.drift;
            d.count = get(0);
            d.use_animation = get(1);
            d.start_pattern = get(2);
            d.end_pattern = get(3);
            d.animation_time = get(4);
            d.y_time = get(5);
            d.period = get(6);
            d.amplitude = get(7);
            if v.len() >= 17 {
                d.area = Rect::from_corners(get(13), get(14), get(15), get(16));
            }
        })?,
        // OBJFRONTSET_FILTER_MOD / _MMX_USE / _QUAKE_USE: rendering
        // switches of the original with no counterpart here.
        1060 | 1073 | 1074 => {}
        // OBJFRONTSET_BTNOFF / _BTNPUSHKEEP / _BTNACTIONNO
        1065 => each(machine, &t.refs, |o| o.params.button.is_button = 0)?,
        1067 => each(machine, &t.refs, |o| {
            o.params.button.state = if get(0) != 0 { 2 } else { 0 }
        })?,
        1068 => each(machine, &t.refs, |o| o.params.button.action = get(0))?,
        // OBJFRONTSET_ERIA / _OWNERIA / _BOXOWNERIA(x1, y1, x2, y2),
        // _RECTOWNERIA(x, y, w, h); no arguments clear.
        1072 if v.is_empty() => each(machine, &t.refs, |o| o.params.own_clip = None)?,
        1072 => each(machine, &t.refs, |o| o.params.own_clip = Some(grp_rect(&v)))?,
        // OBJFRONTSET_RECT(x, y, w, h): a rectangle object's area.
        1023 => {
            let area = if v.len() >= 4 {
                Rect::new(get(0), get(1), get(2), get(3))
            } else {
                Rect::new(0, 0, machine.sys.gfx.width, machine.sys.gfx.height)
            };
            each(machine, &t.refs, |o| {
                if let Some(ObjectData::Rect(rect)) = &mut o.data {
                    *rect = area;
                }
            })?;
        }
        // OBJFRONTSET_GAN_CUTNO_REP(n): the pattern offset of a GAN object.
        1058 => each(machine, &t.refs, |o| o.params.pattern = get(0))?,
        1056 => each(machine, &t.refs, |o| {
            for (slot, value) in o.params.fade.iter_mut().zip(&v) {
                *slot = *value;
            }
        })?,
        1064 => each(machine, &t.refs, |o| {
            let b = &mut o.params.button;
            b.is_button = 1;
            b.action = get(0);
            b.se = get(1);
            b.group = get(2);
            b.number = get(3);
        })?,
        1066 => each(machine, &t.refs, |o| o.params.button.state = get(0))?,
        1070 | 1071 if v.is_empty() => each(machine, &t.refs, |o| o.params.own_clip = None)?,
        1070 => each(machine, &t.refs, |o| o.params.own_clip = Some(grp_rect(&v)))?,
        1071 => each(machine, &t.refs, |o| {
            o.params.own_clip = Some(Rect::new(get(0), get(1), get(2), get(3)))
        })?,
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

/// OBJFRONTADD* (base ids 0..2, 6..8, 36): add to the properties;
/// OBJFRONTEVE_*_STOP (1200..1216): stop an object event where it is.
fn add_or_stop(machine: &mut Machine, command: &Command, t: &Targets) -> Result<Next> {
    let opcode = command.op.opcode;
    let values = ints(machine, command, t.args)?;
    if (1200..=1216).contains(&opcode) {
        let id = 2000 + i32::from(opcode - 1000);
        each(machine, &t.refs, |o| o.mutators.retain(|m| m.id != id))?;
        return Ok(Next::Advance);
    }
    let base = opcode % 1000;
    let Some((repno, props)) = family(base).filter(|_| opcode < 2000) else {
        return machine.unimplemented(command);
    };
    let (index, deltas, _) = split_family(repno, props.len(), &values)?;
    each(machine, &t.refs, |object| {
        for (property, &delta) in props.iter().zip(&deltas) {
            let property = property(index);
            let value = property.get(&object.params);
            property.set(&mut object.params, value.wrapping_add(delta));
        }
    })?;
    Ok(Next::Advance)
}

/// `OBJFRONTEVE_JUMP`, `_QUAKE*`, `_FLUSH*`, `_BLINK*` and `_REPEAT*` (2200..2216): damped
/// oscillations of a property around its current value. The arguments
/// start with (time, delay); the bound and direction modes are not
/// modelled.
fn object_event(machine: &mut Machine, t: &Targets, opcode: u16, v: &[i32]) -> Result<()> {
    use crate::object::{CURVE_FLASH, CURVE_WAVE_AROUND};
    let get = |i: usize| v.get(i).copied().unwrap_or(0);
    let (time, delay) = (get(0), get(1));
    // (property, amplitude) pairs, cycle count, flash or wave.
    let (moves, count, flash): (Vec<(Property, i32)>, i32, bool) = match opcode {
        2200 => (vec![(Property::AdjustY(7), -get(2))], get(3), true),
        2201 | 2202 => (
            vec![
                (Property::AdjustX(7), get(2)),
                (Property::AdjustY(7), get(3)),
            ],
            get(4),
            false,
        ),
        2203 => (
            vec![(Property::Width, get(2)), (Property::Height, get(3))],
            get(4),
            false,
        ),
        2204 => (
            vec![(Property::HqWidth, get(2)), (Property::HqHeight, get(3))],
            get(4),
            false,
        ),
        2205 => (vec![(Property::Rotation, get(2))], get(3), false),
        2206 => (
            vec![
                (Property::TintR, get(2)),
                (Property::TintG, get(3)),
                (Property::TintB, get(4)),
            ],
            get(5),
            true,
        ),
        2207 => {
            let colour = [get(2), get(3), get(4)];
            each(machine, &t.refs, |o| {
                o.params.colour[..3].copy_from_slice(&colour)
            })?;
            (vec![(Property::ColLevel, 255)], get(5), true)
        }
        2208 => (vec![(Property::Invert, 255)], get(2), true),
        2209 => (vec![(Property::Alpha, get(2))], get(3), true),
        // BLINK_PAL / _COL / _REVERSE / _TR: on and off, undamped.
        2210 => (
            vec![
                (Property::TintR, get(2)),
                (Property::TintG, get(3)),
                (Property::TintB, get(4)),
            ],
            get(5),
            true,
        ),
        2211 => {
            let colour = [get(2), get(3), get(4)];
            each(machine, &t.refs, |o| {
                o.params.colour[..3].copy_from_slice(&colour)
            })?;
            (vec![(Property::ColLevel, 255)], get(5), true)
        }
        2212 => (vec![(Property::Invert, 255)], get(2), true),
        2213 => (vec![(Property::Alpha, get(2))], get(3), true),
        // REPEAT_MOVE / _ROTATE / _SCALE: out and back, again and again.
        2214 => (
            vec![
                (Property::AdjustX(7), get(2)),
                (Property::AdjustY(7), get(3)),
            ],
            get(4),
            false,
        ),
        2215 => (vec![(Property::Rotation, get(2))], get(3), false),
        _ => (
            vec![(Property::Width, get(2)), (Property::Height, get(3))],
            get(4),
            false,
        ),
    };
    let now = machine.sys.now();
    let id = i32::from(opcode);
    let blink_or_repeat = opcode >= 2210;
    // A count of 0 repeats (practically) for ever.
    let count = if blink_or_repeat && count <= 0 {
        100_000
    } else {
        count.max(1)
    };
    let curve = match opcode {
        2210..=2213 => crate::object::CURVE_BLINK,
        2214..=2216 => crate::object::CURVE_REPEAT,
        _ if flash => CURVE_FLASH,
        _ => CURVE_WAVE_AROUND,
    } + count;
    // Blinks and repeats take `time` per cycle.
    let time = if blink_or_repeat {
        time.saturating_mul(count)
    } else {
        time
    };
    each(machine, &t.refs, |object| {
        object.end_mutators(id, None);
        let targets = moves
            .iter()
            .map(|&(property, amplitude)| {
                let base = property.get(&object.params);
                // Flashes and blinks towards a value: the amplitude is the
                // distance.
                let amplitude = if flash && !matches!(property, Property::AdjustY(_)) {
                    amplitude - base
                } else {
                    amplitude
                };
                (property, base, amplitude)
            })
            .collect();
        object.mutators.push(Mutator {
            id,
            repno: -1,
            targets,
            start: now,
            duration: time,
            delay,
            curve,
        });
    })
}

/// Writes `values` to the variables passed from parameter `from` on.
fn write_params(
    machine: &mut Machine,
    command: &Command,
    from: usize,
    values: &[i32],
) -> Result<()> {
    for (offset, &value) in values.iter().enumerate() {
        if from + offset >= Machine::param_count(command) {
            break;
        }
        let target = machine.int_target_param(command, from + offset)?;
        machine.set_target(target, value)?;
    }
    Ok(())
}

/// The mutator id of `OBJFRONTEVE_QUAKE`.
const QUAKE_ID: i32 = 2201;

/// Integers, skipping arguments that are not integers (text functions).
fn ints_lenient(machine: &mut Machine, command: &Command, from: usize) -> Result<Vec<i32>> {
    let mut out = Vec::new();
    for index in from..Machine::param_count(command) {
        match machine.int_param(command, index) {
            Ok(value) => out.push(value),
            Err(_) => break,
        }
    }
    Ok(out)
}

// ---- creation -------------------------------------------------------------------

fn load_gan(machine: &mut Machine, name: &str) -> Result<Rc<Gan>> {
    let bytes = machine
        .sys
        .resources
        .read(Kind::Gan, name)
        .ok_or_else(|| anyhow::anyhow!("reallive: GAN file {name:?} not found"))?;
    Ok(Rc::new(Gan::parse(&bytes)?))
}

/// Replaces an object's contents; its parameters stay (scripts may set
/// them before loading).
fn install(machine: &mut Machine, r: ObjectRef, data: ObjectData) -> Result<&mut Object> {
    let object = machine.sys.gfx.object_mut(r)?;
    object.data = Some(data);
    object.animation = None;
    object.mutators.clear();
    Ok(object)
}

/// Applies the optional `visible, x, y, pattern` arguments.
fn common_args(object: &mut Object, v: &[i32], with_pattern: bool) {
    if let Some(&visible) = v.first() {
        object.params.visible = visible != 0;
    }
    if v.len() >= 3 {
        object.params.x = v[1];
        object.params.y = v[2];
    }
    if with_pattern {
        if let Some(&pattern) = v.get(3) {
            object.params.pattern = pattern;
        }
    }
}

pub fn creation(machine: &mut Machine, command: &Command) -> Result<Next> {
    let t = targets(machine, command, false)?;
    let r = t.refs[0];
    let a = t.args;
    match command.op.opcode {
        1000..=1002 | 1300 | 1400 => {
            let name = machine.str_param(command, a)?;
            let v = ints_lenient(machine, command, a + 1)?;
            let image = {
                let sys = &mut machine.sys;
                sys.gfx.load_image(&sys.resources, &name)?
            };
            let data = match command.op.opcode {
                1300 => ObjectData::Drift { name, image },
                1400 => ObjectData::Digits { name, image },
                _ => ObjectData::File { name, image },
            };
            let with_pattern = command.op.opcode < 1300;
            common_args(install(machine, r, data)?, &v, with_pattern);
        }
        // LOADTHUMBNAIL(slot): the picture saved with a game.
        1005 => {
            let slot = machine.int_param(command, a)?;
            let data = match crate::save::slot_thumbnail(machine, slot) {
                Some(surface) => ObjectData::File {
                    name: format!("#thumbnail{slot}"),
                    image: Rc::new(crate::image::Image::from_surface(surface)),
                },
                None => ObjectData::File {
                    name: String::new(),
                    image: Rc::new(crate::image::Image::new(1, 1)),
                },
            };
            install(machine, r, data)?;
        }
        // objOfFileGan; LOADGANCOPYPARAM (1004) keeps the parameters too,
        // as every load does here.
        1003 | 1004 => {
            let image_name = machine.str_param(command, a)?;
            let gan_name = machine.str_param(command, a + 1)?;
            let v = ints_lenient(machine, command, a + 2)?;
            let image = {
                let sys = &mut machine.sys;
                sys.gfx.load_image(&sys.resources, &image_name)?
            };
            let gan = load_gan(machine, &gan_name)?;
            let data = ObjectData::Gan {
                image_name,
                image,
                gan_name,
                gan,
            };
            common_args(install(machine, r, data)?, &v, true);
        }
        1100 | 1101 => {
            let v = ints_lenient(machine, command, a)?;
            let area = match v.len() {
                0 | 1 => Rect::new(0, 0, machine.sys.gfx.width, machine.sys.gfx.height),
                2 | 3 if command.op.opcode == 1101 => {
                    Rect::new(v[0], v[1], machine.sys.gfx.width, machine.sys.gfx.height)
                }
                _ if command.op.opcode == 1100 => grp_rect(&v),
                _ => Rect::new(v[0], v[1], v[2], v[3]),
            };
            let visible = v.get(4).copied();
            let object = install(machine, r, ObjectData::Rect(area))?;
            if let Some(visible) = visible {
                object.params.visible = visible != 0;
            }
        }
        1200 => {
            let text = machine.str_param(command, a)?;
            let v = ints_lenient(machine, command, a + 1)?;
            let object = install(machine, r, ObjectData::Text)?;
            object.params.text.text = text;
            common_args(object, &v, false);
        }
        1500 => {
            let v = ints_lenient(machine, command, a)?;
            let count = v
                .first()
                .copied()
                .unwrap_or(0)
                .clamp(0, OBJECT_COUNT as i32) as usize;
            // The file names after the count are not used.
            let rest: Vec<i32> = (a + 3..Machine::param_count(command))
                .map(|i| machine.int_param(command, i))
                .collect::<Result<_>>()?;
            let object = install(machine, r, ObjectData::Parent(vec![None; count.max(1)]))?;
            object.params.visible = rest.first().is_none_or(|&visible| visible != 0);
            if rest.len() >= 3 {
                object.params.x = rest[1];
                object.params.y = rest[2];
            }
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

// ---- animation (73/74) ------------------------------------------------------------

pub fn animation(machine: &mut Machine, command: &Command) -> Result<Next> {
    let opcode = command.op.opcode;
    let now = machine.sys.now();
    // The *ALL forms (100..106): every object of the layer.
    if (100..=106).contains(&opcode) && opcode != 104 {
        let gfx = &mut machine.sys.gfx;
        let objects: Vec<usize> = (0..OBJECT_COUNT)
            .filter(|&i| gfx.fg.get(i).is_some_and(Option::is_some))
            .collect();
        match opcode {
            100 => {
                for &i in &objects {
                    if let Some(Some(o)) = gfx.fg.get_mut(i) {
                        o.animation = None;
                    }
                }
            }
            101 => objects.iter().for_each(|&i| gfx.pause_animation(i, now)),
            102 => objects.iter().for_each(|&i| gfx.resume_animation(i, now)),
            103 => {
                machine.store = i32::from(
                    !objects
                        .iter()
                        .any(|&i| gfx.fg[i].as_ref().is_some_and(Object::is_animating)),
                );
            }
            // KEEPALL / RELEASEALL: nothing is discarded between scenes here.
            _ => {}
        }
        return Ok(Next::Advance);
    }
    if opcode == 104 {
        // objWaitAll: every animating object of both layers.
        let bg = is_bg(command.op.module);
        let refs = (0..OBJECT_COUNT as i32)
            .map(|buf| ObjectRef {
                bg,
                buf,
                child: None,
            })
            .collect();
        machine.push_long_op(Box::new(MutatorWait {
            refs,
            id: None,
            repno: None,
            clickable: false,
        }));
        return Ok(Next::Advance);
    }
    let t = targets(machine, command, false)?;
    let v = ints(machine, command, t.args)?;
    match opcode {
        // OBJFRONTANM_PAUSE / _RESUME
        1 | 2 => {
            for r in &t.refs {
                if r.child.is_none() && !r.bg && r.buf >= 0 {
                    if opcode == 1 {
                        machine.sys.gfx.pause_animation(r.buf as usize, now);
                    } else {
                        machine.sys.gfx.resume_animation(r.buf as usize, now);
                    }
                }
            }
        }
        // OBJFRONTANM_KEEP / _RELEASE
        5 | 6 => {}
        // *GANANM_NEXT_LOOP / _ONESHOT / _ONESHOTFREE / _ONESHOTWAIT /
        // _ONESHOTWAITFREE / _EYE / _KOESYNC(set): after the running one.
        3101..=3109 => {
            let after = match opcode {
                3101 | 3102 => AfterAnimation::Loop,
                3103 | 3104 | 3106 => AfterAnimation::Stop,
                3105 | 3107 => AfterAnimation::Clear,
                _ => AfterAnimation::Blink,
            };
            let parameter = v.first().copied().unwrap_or(0);
            for &r in &t.refs {
                let due = machine.sys.gfx.object(r).and_then(|object| {
                    let animation = object.animation.as_ref()?;
                    if object.is_animating() || animation.after != AfterAnimation::Loop {
                        return None;
                    }
                    let cycle = object.animation_cycle()?.max(1);
                    let elapsed = now.saturating_sub(animation.start);
                    Some(animation.start + elapsed.div_ceil(cycle) * cycle)
                });
                let animation = Animation {
                    start: now,
                    after,
                    parameter,
                    finished: false,
                };
                machine.sys.gfx.queued_animations.retain(|(q, ..)| *q != r);
                machine.sys.gfx.queued_animations.push((r, due, animation));
            }
            if matches!(opcode, 3106 | 3107) {
                machine.push_long_op(Box::new(MutatorWait {
                    refs: t.refs.clone(),
                    id: None,
                    repno: None,
                    clickable: false,
                }));
            }
        }
        // *GANANM_ONESHOTRELEASE / _ONESHOTWAITRELEASE(set)
        3050 | 3051 => {
            let parameter = v.first().copied().unwrap_or(0);
            each(machine, &t.refs, |o| {
                o.animation = Some(Animation {
                    start: now,
                    after: AfterAnimation::Clear,
                    parameter,
                    finished: false,
                });
                o.params.visible = true;
            })?;
            if opcode == 3051 {
                machine.push_long_op(Box::new(MutatorWait {
                    refs: t.refs.clone(),
                    id: None,
                    repno: None,
                    clickable: false,
                }));
            }
        }
        0 | 1000 => each(machine, &t.refs, |o| {
            o.animation = None;
            if let Some(&pattern) = v.first() {
                o.params.pattern = pattern;
            }
        })?,
        3 => {
            machine.store = i32::from(
                !t.refs
                    .iter()
                    .any(|&r| machine.sys.gfx.object(r).is_some_and(Object::is_animating)),
            );
        }
        4 => machine.push_long_op(Box::new(MutatorWait {
            refs: t.refs.clone(),
            id: None,
            repno: None,
            clickable: false,
        })),
        _ => {
            let (after, wait) = match opcode % 1000 {
                1 | 2 => (AfterAnimation::Loop, false),
                3 | 4 => (AfterAnimation::Stop, false),
                5 => (AfterAnimation::Clear, false),
                6 => (AfterAnimation::Stop, true),
                7 => (AfterAnimation::Clear, true),
                8 | 9 => (AfterAnimation::Blink, false),
                _ => return machine.unimplemented(command),
            };
            let parameter = v.first().copied().unwrap_or(0);
            each(machine, &t.refs, |o| {
                o.animation = Some(Animation {
                    start: now,
                    after,
                    parameter,
                    finished: false,
                });
                o.params.visible = true;
            })?;
            if wait {
                machine.push_long_op(Box::new(MutatorWait {
                    refs: t.refs.clone(),
                    id: None,
                    repno: None,
                    clickable: false,
                }));
            }
        }
    }
    Ok(Next::Advance)
}

// ---- getters (84/85) ---------------------------------------------------------------

pub fn getters(machine: &mut Machine, command: &Command) -> Result<Next> {
    let t = targets(machine, command, false)?;
    let r = t.refs[0];
    let a = t.args;
    let now = machine.sys.now();
    let object = machine
        .sys
        .gfx
        .object(r)
        .cloned()
        .unwrap_or_else(Object::empty);
    let p = &object.params;
    match command.op.opcode {
        1000 => {
            let x = machine.int_target_param(command, a)?;
            let y = machine.int_target_param(command, a + 1)?;
            machine.set_target(x, p.x)?;
            machine.set_target(y, p.y)?;
        }
        1001 => machine.store = p.x,
        1002 => machine.store = p.y,
        1003 => machine.store = p.alpha,
        1004 => machine.store = i32::from(p.visible),
        1006 => {
            let index = machine.int_param(command, a)?.clamp(0, 7) as usize;
            let x = machine.int_target_param(command, a + 1)?;
            let y = machine.int_target_param(command, a + 2)?;
            machine.set_target(x, p.adjust_x[index])?;
            machine.set_target(y, p.adjust_y[index])?;
        }
        1007 | 1008 => {
            let index = machine.int_param(command, a)?.clamp(0, 7) as usize;
            machine.store = if command.op.opcode == 1007 {
                p.adjust_x[index]
            } else {
                p.adjust_y[index]
            };
        }
        1009 => machine.store = p.mono,
        1010 => machine.store = p.invert,
        1011 => machine.store = p.light,
        // GET_ERIA / GET_BOX / GET_BOXERIA / GET_BOXOWNERIA(x1, y1, x2, y2),
        // GET_RECT / GET_RECTERIA / GET_RECTOWNERIA(x, y, w, h)
        1005 | 1022 | 1035 | 1070 | 1072 | 1023 | 1034 | 1071 => {
            let rect = match command.op.opcode {
                1070..=1072 => p.own_clip,
                1022 | 1023 => match &object.data {
                    Some(ObjectData::Rect(rect)) => Some(*rect),
                    _ => p.clip,
                },
                _ => p.clip,
            }
            .unwrap_or(Rect::new(0, 0, 0, 0));
            let values = if matches!(command.op.opcode, 1023 | 1034 | 1071) {
                [rect.x, rect.y, rect.w, rect.h]
            } else {
                [rect.x, rect.y, rect.right(), rect.bottom()]
            };
            write_params(machine, command, a, &values)?;
        }
        1012 => write_params(machine, command, a, &p.tint)?,
        1013..=1015 => machine.store = p.tint[usize::from(command.op.opcode - 1013)],
        1016 => write_params(machine, command, a, &p.colour)?,
        1017..=1020 => machine.store = p.colour[usize::from(command.op.opcode - 1017)],
        1021 => machine.store = p.composite,
        // GET_MOJI(str): a text object's text.
        1024 => {
            let target = machine.str_target_param(command, a)?;
            machine.write_string(target, p.text.text.clone())?;
        }
        1025 => {
            let t = &p.text;
            let values = [t.size, t.xspace, t.yspace, t.char_count, t.colour, t.shadow];
            write_params(machine, command, a, &values)?;
        }
        1026 => machine.store = p.z_layer,
        1027 => machine.store = p.z_depth,
        1028 => write_params(machine, command, a, &[p.scroll_rate.0, p.scroll_rate.1])?,
        1029 => machine.store = p.scroll_rate.0,
        1030 => machine.store = p.scroll_rate.1,
        1032 => machine.store = p.z_order,
        1033 => machine.store = p.quarter_view,
        1036 => machine.store = p.adjust_vert,
        1037 => machine.store = p.digits.value,
        1038 => {
            let d = &p.digits;
            write_params(
                machine,
                command,
                a,
                &[d.digits, d.zero, d.sign, d.pack, d.space],
            )?;
        }
        1039 => machine.store = p.pattern,
        1040 => {
            let index = machine.int_param(command, a)?.clamp(0, 7) as usize;
            machine.store = p.adjust_alpha[index];
        }
        1045 => {
            let set = match (&object.animation, &object.data) {
                (Some(animation), Some(ObjectData::Gan { .. })) => animation.parameter,
                _ => 0,
            };
            let pattern = object.current_frame(now).0;
            write_params(machine, command, a, &[set, pattern])?;
        }
        1046 => write_params(machine, command, a, &[p.width, p.height])?,
        1047 => machine.store = p.width,
        1048 => machine.store = p.height,
        1049 => machine.store = p.rotation,
        1050 => write_params(machine, command, a, &[p.rep_origin.0, p.rep_origin.1])?,
        1051 => machine.store = p.rep_origin.0,
        1052 => machine.store = p.rep_origin.1,
        1053 => write_params(machine, command, a, &[p.origin.0, p.origin.1])?,
        1054 => machine.store = p.origin.0,
        1055 => machine.store = p.origin.1,
        1061 => write_params(machine, command, a, &[p.hq_width, p.hq_height])?,
        1062 => machine.store = p.hq_width,
        1063 => machine.store = p.hq_height,
        1064 => {
            let b = &p.button;
            write_params(machine, command, a, &[b.action, b.se, b.group, b.number])?;
        }
        1066 => machine.store = p.button.state,
        1067 => machine.store = i32::from(p.button.state == 2),
        // GET_MMX_USE / GET_QUAKE_USE / GET_FILTER_MOD: nothing special here.
        1060 | 1073 => machine.store = 0,
        1058 => machine.store = p.pattern,
        1074 => machine.store = i32::from(object.has_mutator(QUAKE_ID, None)),
        // GET_TYPE: 0 none, then by kind of contents.
        1101 => {
            machine.store = match &object.data {
                None => 0,
                Some(ObjectData::File { .. }) => 1,
                Some(ObjectData::Gan { .. }) => 2,
                Some(ObjectData::Text) => 3,
                Some(ObjectData::Digits { .. }) => 4,
                Some(ObjectData::Rect(_)) => 5,
                Some(ObjectData::Drift { .. }) => 6,
                Some(_) => 7,
            };
        }
        // GET_SELCENTER(x, y): the centre of the object on screen.
        1102 => {
            let x = machine.int_target_param(command, a)?;
            let y = machine.int_target_param(command, a + 1)?;
            let parent = match r.child {
                Some(_) => {
                    let parent = ObjectRef { child: None, ..r };
                    machine
                        .sys
                        .gfx
                        .object(parent)
                        .map_or((0, 0), |o| (o.params.x, o.params.y))
                }
                None => (0, 0),
            };
            let bounds = {
                let gfx = &mut machine.sys.gfx;
                object.bounds(now, &mut gfx.fonts, &gfx.colours, parent)
            };
            machine.set_target(x, bounds.x + bounds.w / 2)?;
            machine.set_target(y, bounds.y + bounds.h / 2)?;
        }
        // EVE_DISP_ENDALL(speed up): every running change ends at once.
        6004 => {
            if let Ok(object) = machine.sys.gfx.object_mut(r) {
                object.end_mutators(crate::object::ALL_MUTATORS, None);
            }
        }
        1100 => {
            let w = machine.int_target_param(command, a)?;
            let h = machine.int_target_param(command, a + 1)?;
            let (width, height) = {
                let gfx = &mut machine.sys.gfx;
                object.dimensions(now, &mut gfx.fonts, &gfx.colours)
            };
            machine.set_target(w, width)?;
            machine.set_target(h, height)?;
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

// ---- management (60-62) ----------------------------------------------------------------

fn free(object: &mut Object, reset_params: bool) {
    object.data = None;
    object.animation = None;
    object.mutators.clear();
    if reset_params {
        object.params = Default::default();
    }
}

fn reset_params(object: &mut Object) {
    object.params = Default::default();
    object.mutators.clear();
}

pub fn management(machine: &mut Machine, command: &Command) -> Result<Next> {
    let module = command.op.module;
    let opcode = command.op.opcode;
    // Module 60 addresses both layers.
    let layers: &[bool] = match module {
        60 => &[false, true],
        62 => &[true],
        _ => &[false],
    };
    let everything = matches!(opcode, 100..=102 | 110..=113 | 115 | 116);
    let child = command.op.modtype == 2;
    let parent = if child {
        Some(machine.int_param(command, 0)?)
    } else {
        None
    };
    let first = usize::from(child);
    let count = Machine::param_count(command);
    let refs_on = |bg: bool, bufs: &[i32]| -> Vec<ObjectRef> {
        bufs.iter()
            .map(|&buf| match parent {
                Some(parent) => ObjectRef {
                    bg,
                    buf: parent,
                    child: Some(buf),
                },
                None => ObjectRef {
                    bg,
                    buf,
                    child: None,
                },
            })
            .collect()
    };
    match opcode {
        // SAVECHILD_ON / _OFF: children are always saved here.
        6 | 7 => {}
        // GET_FILENAME / GET_GAN_FILENAME(buf, var); the _ALL forms have
        // nothing to write to.
        15 | 16 => {
            let buf = machine.int_param(command, first)?;
            let target = machine.str_target_param(command, first + 1)?;
            let r = refs_on(layers[0], &[buf])[0];
            let name = match machine.sys.gfx.object(r).and_then(|o| o.data.as_ref()) {
                Some(ObjectData::File { name, .. } | ObjectData::Digits { name, .. })
                | Some(ObjectData::Drift { name, .. }) => name.clone(),
                Some(ObjectData::Gan {
                    image_name,
                    gan_name,
                    ..
                }) => {
                    if opcode == 15 {
                        image_name.clone()
                    } else {
                        gan_name.clone()
                    }
                }
                _ => String::new(),
            };
            let name = if opcode == 16
                && !matches!(
                    machine.sys.gfx.object(r).and_then(|o| o.data.as_ref()),
                    Some(ObjectData::Gan { .. })
                ) {
                String::new()
            } else {
                name
            };
            machine.write_string(target, name)?;
        }
        115 | 116 => {}
        // OBJCOPYALL: every foreground object to the background.
        102 => {
            let gfx = &mut machine.sys.gfx;
            gfx.bg.clone_from(&gfx.fg);
            gfx.dirty = true;
        }
        // objCopy family: (source, destination).
        2 | 3 | 14 => {
            let from = machine.int_param(command, first)?;
            // objCopyFgToBg(buf): the same number on the other layer.
            let to = machine.int_param_or(command, first + 1, from)?;
            let (from_bg, to_bg) = match (module, opcode) {
                (60, _) => (false, true),
                (61, 2) | (_, 14) if module == 61 => (false, false),
                (61, _) => (false, true),
                (62, 2) => (true, false),
                _ => (true, true),
            };
            let source = refs_on(from_bg, &[from])[0];
            let object = machine.sys.gfx.object(source).cloned();
            let dest = refs_on(to_bg, &[to])[0];
            match object {
                Some(object) => *machine.sys.gfx.object_mut(dest)? = object,
                None => free(machine.sys.gfx.object_mut(dest)?, true),
            }
        }
        0 | 1 | 4 | 5 | 10..=13 | 100 | 101 | 110..=113 => {
            let bufs: Vec<i32> = if everything || (module == 60 && opcode == 1 && count <= first) {
                (0..OBJECT_COUNT as i32).collect()
            } else if count >= first + 2 {
                let min = machine.int_param(command, first)?;
                let max = machine.int_param(command, first + 1)?;
                (min.max(0)..=max.min(OBJECT_COUNT as i32 - 1)).collect()
            } else {
                vec![machine.int_param(command, first)?]
            };
            for &bg in layers {
                for r in refs_on(bg, &bufs) {
                    if machine.sys.gfx.object(r).is_none() && !matches!(opcode, 4 | 5) {
                        continue;
                    }
                    let object = machine.sys.gfx.object_mut(r)?;
                    match opcode {
                        0 | 100 => free(object, false),
                        1 | 101 => object.params.wipe_copy = false,
                        4 => object.params.wipe_copy = true,
                        5 => object.params.wipe_copy = false,
                        10 | 110 => reset_params(object),
                        // INITREP: the repetition origin back to zero.
                        12 | 112 => object.params.rep_origin = (0, 0),
                        // WAIPERASE_INITPARAM
                        13 | 113 => {
                            reset_params(object);
                            object.params.wipe_copy = false;
                        }
                        _ => free(object, true),
                    }
                }
            }
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}
