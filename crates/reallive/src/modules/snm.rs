//! The SerialPDT module (1:34): `snmPlay`, `snmStretch`, `snmScroll`, their
//! background (`snmBg*`), ready and paused forms, and the controls.

use anyhow::Result;

use crate::bytecode::Command;
use crate::longop::{Wait, WaitEvent};
use crate::machine::{Machine, Next};
use crate::serial_pdt::{SerialPdt, Shape};
use crate::surface::Rect;

fn frames(
    machine: &mut Machine,
    command: &Command,
    from: usize,
) -> Result<Vec<(std::rc::Rc<crate::image::Image>, u64)>> {
    let mut frames = Vec::new();
    for index in from..command.params.len() {
        let pieces = machine.complex_param(command, index)?;
        let Some(name) = pieces.first() else { continue };
        let name = machine.eval_str(name)?;
        let time = match pieces.get(1) {
            Some(time) => machine.eval_int(time)?.max(0) as u64,
            None => 0,
        };
        let image = machine.sys.gfx.load_image(&machine.sys.resources, &name)?;
        frames.push((image, time));
    }
    Ok(frames)
}

/// Starts (or readies) an animation; `n` is the opcode within its
/// hundred: 0–9 frames at a point, 10–19 stretched, 20–22 scrolling, with
/// the variant (play / wait / loop) in `n % 3`.
fn start(machine: &mut Machine, command: &Command, back: bool, n: i32, hold: bool) -> Result<()> {
    let buf = machine.int_param(command, 0)?;
    let variant = if n >= 20 { n - 20 } else { n % 10 % 3 };
    let looped = variant == 2;
    let (shape, frames) = if n >= 20 {
        let v = machine.int_params_from(command, 1)?;
        let (x1, y1, x2, y2) = (v[0], v[1], v[2], v[3]);
        let name = machine.str_param(command, 5)?;
        let tail: Vec<i32> = (6..11)
            .map(|i| machine.int_param_or(command, i, 0))
            .collect::<Result<_>>()?;
        let image = machine.sys.gfx.load_image(&machine.sys.resources, &name)?;
        let (w, h) = (image.surface.width, image.surface.height);
        // Never further than the image's own size.
        let dx = (tail[0] - tail[2]).clamp(-w, w);
        let dy = (tail[1] - tail[3]).clamp(-h, h);
        let area = Rect::from_corners(x1, y1, x2 + 1, y2 + 1);
        (
            Shape::Scroll { area, dx, dy },
            vec![(image, tail[4].max(0) as u64)],
        )
    } else if n >= 10 {
        let v: Vec<i32> = (1..5)
            .map(|i| machine.int_param(command, i))
            .collect::<Result<_>>()?;
        let area = Rect::from_corners(v[0], v[1], v[2] + 1, v[3] + 1);
        (Shape::Stretch(area), frames(machine, command, 5)?)
    } else {
        let (x, y) = (
            machine.int_param(command, 1)?,
            machine.int_param(command, 2)?,
        );
        (Shape::At(x, y), frames(machine, command, 3)?)
    };
    let now = machine.sys.now();
    let mut snm = SerialPdt::new(shape, frames, looped, now);
    if hold {
        snm = snm.hold();
    }
    machine.sys.gfx.snm.layer(back).insert(buf, snm);
    if variant == 1 && !back && !hold {
        wait(machine, Some(buf));
    }
    Ok(())
}

fn wait(machine: &mut Machine, buf: Option<i32>) {
    let mut wait = Wait::for_ms(machine, 0);
    wait.until = None;
    wait.event = WaitEvent::SerialPdt(buf);
    machine.push_long_op(Box::new(wait));
}

/// STOP / WAIT / CHECK / PAUSE / RESUME (`action` 0–4) on one buffer or,
/// with `buf` `None`, on all.
fn control(machine: &mut Machine, back: bool, action: i32, buf: Option<i32>) {
    let now = machine.sys.now();
    let snm = &mut machine.sys.gfx.snm;
    match action {
        0 => match buf {
            Some(buf) => {
                snm.layer(back).remove(&buf);
            }
            None => snm.layer(back).clear(),
        },
        1 => {
            if !back {
                wait(machine, buf);
            }
        }
        2 => machine.store = i32::from(snm.running(back, buf)),
        _ => {
            for (_, s) in snm
                .layer(back)
                .iter_mut()
                .filter(|(b, _)| buf.is_none_or(|x| **b == x))
            {
                if action == 3 {
                    s.pause(now);
                } else {
                    s.resume(now);
                }
            }
        }
    }
}

pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let opcode = i32::from(command.op.opcode);
    let back = opcode >= 3000;
    let local = opcode % 1000;
    let first = |machine: &mut Machine| -> Result<Option<i32>> {
        Ok(if command.params.is_empty() {
            None
        } else {
            Some(machine.int_param(command, 0)?)
        })
    };
    match (opcode, local) {
        (2000..=2004, _) => {
            let buf = first(machine)?;
            control(machine, false, opcode - 2000, buf);
        }
        (2020..=2024, _) => control(machine, false, opcode - 2020, None),
        // ERASE: both slots of the buffer.
        (2050, _) => {
            let buf = machine.int_param(command, 0)?;
            machine.sys.gfx.snm.fg.remove(&buf);
            machine.sys.gfx.snm.bg.remove(&buf);
        }
        (2054, _) => machine.sys.gfx.snm = Default::default(),
        // SWAP: exchange the foreground and background slots.
        (2055, _) => {
            let buf = machine.int_param(command, 0)?;
            let snm = &mut machine.sys.gfx.snm;
            let (f, b) = (snm.fg.remove(&buf), snm.bg.remove(&buf));
            if let Some(f) = f {
                snm.bg.insert(buf, f);
            }
            if let Some(b) = b {
                snm.fg.insert(buf, b);
            }
        }
        (2056, _) => {
            let snm = &mut machine.sys.gfx.snm;
            std::mem::swap(&mut snm.fg, &mut snm.bg);
        }
        (_, 100..=122) => start(machine, command, back, local - 100, false)?,
        (_, 200..=222) | (_, 300..=322) => start(machine, command, back, local % 100, true)?,
        // ADD(buf, (name, time)...): more frames for a buffer.
        (_, 800..=802) => {
            let buf = machine.int_param(command, 0)?;
            let frames = frames(machine, command, 1)?;
            if let Some(snm) = machine.sys.gfx.snm.layer(back).get_mut(&buf) {
                snm.add(frames);
            }
        }
        (_, 850..=854) => {
            let buf = first(machine)?;
            control(machine, back, local - 850, buf);
        }
        (_, 870..=874) => control(machine, back, local - 870, None),
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}
