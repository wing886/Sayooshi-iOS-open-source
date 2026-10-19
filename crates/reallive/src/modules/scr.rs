//! Screen settings and the graphics stack (1:30), backgrounds (1:40) and
//! debugging functions (1:255).

use std::rc::Rc;

use anyhow::Result;

use crate::bytecode::Command;
use crate::effects::Transition;
use crate::graphics::DrawMode;
use crate::machine::{Machine, Next};
use crate::resource::Kind;
use crate::surface::Blend;

pub fn scr(machine: &mut Machine, command: &Command) -> Result<Next> {
    let gfx = &mut machine.sys.gfx;
    match command.op.opcode {
        0 => gfx.stack.clear(),
        1 => {
            let count = machine.int_param(command, 0)?.max(0);
            for _ in 0..count {
                machine.sys.gfx.stack.push(String::new());
            }
        }
        2 => {
            let count = machine.int_param(command, 0)?.max(0) as usize;
            let stack = &mut machine.sys.gfx.stack;
            stack.truncate(stack.len().saturating_sub(count));
        }
        3 => machine.store = gfx.stack.len() as i32,
        4 => {
            let length = machine.int_param(command, 0)?.max(0) as usize;
            machine.sys.gfx.stack.truncate(length);
        }
        20 => gfx.draw_mode = DrawMode::Auto,
        21 => gfx.draw_mode = DrawMode::SemiAuto,
        22 => gfx.draw_mode = DrawMode::Manual,
        // ModeToScreenSize(mode, width, height)
        30 => {
            let mode = machine.int_param(command, 0)?;
            let (w, h) = match mode {
                1 => (800, 600),
                2 => (1024, 768),
                3 => (1280, 960),
                4 => (1600, 1200),
                _ => (640, 480),
            };
            let tw = machine.int_target_param(command, 1)?;
            let th = machine.int_target_param(command, 2)?;
            machine.set_target(tw, w)?;
            machine.set_target(th, h)?;
        }
        // CAPTUREBANK(dc) / _WITH_MESSAGEWINDOW(dc) / _WITH_SYSBTN(dc),
        // CAPTURE / _WITH_MESSAGEWINDOW / _WITH_SYSBTN: the screen into a
        // DC, or kept aside.
        60..=65 => {
            let windows = !matches!(command.op.opcode, 60 | 62);
            let frame = crate::screen::compose_layers(&mut machine.sys, windows);
            if matches!(command.op.opcode, 60 | 61 | 64) {
                let dc = machine.int_param(command, 0)?;
                machine.sys.gfx.set_dc(dc, frame)?;
            } else {
                machine.sys.gfx.capture = Some(std::rc::Rc::new(frame));
            }
        }
        // WAIP_WITH_MESSAGEWINDOW: the text windows are part of every
        // transition here already.
        40 => {}
        // ZEROBANKUPDATEBOX / _RECT(area): show DC 0 (in manual drawing
        // mode, the whole screen is shown).
        50 | 51 => gfx.refresh_requested = true,
        // THUMBNAILEXPAND(slot, dc, x, y): a save's picture onto a DC.
        70 => {
            let slot = machine.int_param(command, 0)?;
            let dc = machine.int_param(command, 1)?;
            let x = machine.int_param_or(command, 2, 0)?;
            let y = machine.int_param_or(command, 3, 0)?;
            if let Some(thumbnail) = crate::save::slot_thumbnail(machine, slot) {
                machine.sys.gfx.dc_mut(dc)?.blit(
                    &thumbnail,
                    thumbnail.rect(),
                    x,
                    y,
                    255,
                    crate::surface::Blend::Copy,
                    None,
                );
            }
        }
        // SET_SCREENZOOM(percent, x, y) / _PERCENT(percent) / _SIZE(percent)
        // / _POS(x, y); CLR_SCREENZOOM
        80..=83 => {
            let v = machine.int_params_from(command, 0)?;
            let (w, h) = (machine.sys.gfx.width, machine.sys.gfx.height);
            let (mut percent, mut x, mut y) =
                machine.sys.gfx.screen_zoom.unwrap_or((100, w / 2, h / 2));
            match (command.op.opcode, &v[..]) {
                (80, [p, cx, cy, ..]) => (percent, x, y) = (*p, *cx, *cy),
                (81 | 82, [p, ..]) => percent = *p,
                (83, [cx, cy, ..]) => (x, y) = (*cx, *cy),
                _ => {}
            }
            machine.sys.gfx.screen_zoom = Some((percent, x, y));
            machine.sys.gfx.dirty = true;
        }
        84 => {
            gfx.screen_zoom = None;
            gfx.dirty = true;
        }
        // GetDCPixel(x, y, dc, r, g, b)
        31 => {
            let x = machine.int_param(command, 0)?;
            let y = machine.int_param(command, 1)?;
            let dc = machine.int_param(command, 2)?;
            let pixel = machine.sys.gfx.dc(dc)?.pixel(x, y);
            for (i, value) in pixel.iter().take(3).enumerate() {
                let target = machine.int_target_param(command, 3 + i)?;
                machine.set_target(target, i32::from(*value))?;
            }
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

/// `bgrLoadHaikei([file,] sel)`: a full-screen background with a
/// transition; objects are promoted as with `grpOpenBg`.
pub fn bgr(machine: &mut Machine, command: &Command) -> Result<Next> {
    match command.op.opcode {
        // bgrLoadHaikei([file,] sel) / HAIKEICHANGE (objects stay where
        // they are); HAIKEI_TIME / HAIKEICHANGE_TIME / HAIKEICHR_TIME with
        // a transition time last.
        10 | 11 | 20 | 21 | 101 => {
            let opcode = command.op.opcode;
            let mut count = Machine::param_count(command);
            let time = if matches!(opcode, 20 | 21 | 101) && count >= 2 {
                count -= 1;
                Some(machine.int_param(command, count)?)
            } else {
                None
            };
            let (name, sel) = if count <= 1 {
                (None, machine.int_param_or(command, 0, 0)?)
            } else {
                (
                    Some(machine.str_param(command, 0)?),
                    machine.int_param(command, 1)?,
                )
            };
            let mut values = crate::modules::grp::sel_values(machine, sel, false);
            if let Some(time) = time {
                values[6] = time;
            }
            let promote = !matches!(opcode, 11 | 21);
            load_haikei(machine, name, &values, promote)?;
        }
        // HAIKEI_FULL / HAIKEICHANGE_FULL(file, x, y, zero x, zero y, time,
        // effect, direction, op1..op5, alpha, level): the transition given
        // in full.
        30 | 31 => {
            let name = machine.str_param(command, 0)?;
            let v: Vec<i32> = (1..15)
                .map(|i| machine.int_param_or(command, i, 0))
                .collect::<Result<_>>()?;
            let (w, h) = (machine.sys.gfx.width, machine.sys.gfx.height);
            let mut values = vec![0, 0, w - 1, h - 1, v[0], v[1]];
            values.extend_from_slice(&v[4..14]);
            values.resize(16, 0);
            load_haikei(machine, Some(name), &values, command.op.opcode == 30)?;
            let pos = (v[2], v[3]);
            machine.sys.gfx.set_haikei_pos(pos);
        }
        // bgrMulti(file, (file[, x, y])...): a background with pictures
        // laid over it.
        100 => {
            let name = machine.str_param(command, 0)?;
            let values = crate::modules::grp::sel_values(machine, 0, false);
            load_haikei(machine, Some(name), &values, true)?;
            for index in 1..command.params.len() {
                let pieces = machine.complex_param(command, index)?;
                let Some(first) = pieces.first() else {
                    continue;
                };
                let file = machine.eval_str(first)?;
                let x = match pieces.get(1) {
                    Some(e) => machine.eval_int(e)?,
                    None => 0,
                };
                let y = match pieces.get(2) {
                    Some(e) => machine.eval_int(e)?,
                    None => 0,
                };
                let sys = &mut machine.sys;
                let image = sys.gfx.load_image(&sys.resources, &file)?;
                sys.gfx.dc_mut(0)?.blit(
                    &image.surface,
                    image.surface.rect(),
                    x,
                    y,
                    255,
                    Blend::Mask,
                    None,
                );
            }
        }
        // HAIKEIBUF_LOAD(buf, file) / _FREE(buf) / _FREEALL: preloading.
        2000 => {
            let buf = machine.int_param(command, 0)?;
            let name = machine.str_param(command, 1)?;
            let sys = &mut machine.sys;
            let image = sys.gfx.load_image(&sys.resources, &name)?;
            sys.gfx.preloaded.insert(10_000 + buf, (name, image));
        }
        2001 => {
            let buf = machine.int_param(command, 0)?;
            machine.sys.gfx.preloaded.remove(&(10_000 + buf));
        }
        2002 => machine.sys.gfx.preloaded.retain(|&buf, _| buf < 10_000),
        // HAIKEI_SCROLL_POS / _POSX / _POSY(..., time): scroll now; _NEXT
        // after the running scroll; _READY when _READY_SYNC is called.
        3000..=3008 => {
            use crate::graphics::{HaikeiMove, HaikeiScroll};
            let v = machine.int_params_from(command, 0)?;
            let get = |i: usize| v.get(i).copied().unwrap_or(0);
            let m = match (command.op.opcode - 3000) % 3 {
                0 => HaikeiMove {
                    x: Some(get(0)),
                    y: Some(get(1)),
                    time: get(2).max(0) as u64,
                },
                1 => HaikeiMove {
                    x: Some(get(0)),
                    y: None,
                    time: get(1).max(0) as u64,
                },
                _ => HaikeiMove {
                    x: None,
                    y: Some(get(0)),
                    time: get(1).max(0) as u64,
                },
            };
            let now = machine.sys.now();
            let gfx = &mut machine.sys.gfx;
            match (command.op.opcode - 3000) / 3 {
                0 => {
                    gfx.haikei_queue.clear();
                    gfx.haikei_scroll = Some(HaikeiScroll::start(gfx.haikei_pos, m, now));
                }
                1 if gfx.haikei_scroll.is_some() => gfx.haikei_queue.push(m),
                1 => gfx.haikei_scroll = Some(HaikeiScroll::start(gfx.haikei_pos, m, now)),
                _ => gfx.haikei_ready.push(m),
            }
        }
        3009 => {
            let now = machine.sys.now();
            let gfx = &mut machine.sys.gfx;
            let mut ready = std::mem::take(&mut gfx.haikei_ready);
            if gfx.haikei_scroll.is_none() && !ready.is_empty() {
                let first = ready.remove(0);
                gfx.haikei_scroll = Some(crate::graphics::HaikeiScroll::start(
                    gfx.haikei_pos,
                    first,
                    now,
                ));
            }
            gfx.haikei_queue.extend(ready);
        }
        // HAIKEI_SCROLL_WAIT / _CHECK / _STOP
        3100 => {
            let wait = crate::longop::Wait::event(crate::longop::WaitEvent::HaikeiScroll);
            machine.push_long_op(Box::new(wait));
        }
        3101 => machine.store = i32::from(machine.sys.gfx.haikei_scroll.is_some()),
        3102 => {
            let gfx = &mut machine.sys.gfx;
            gfx.haikei_scroll = None;
            gfx.haikei_queue.clear();
            gfx.haikei_ready.clear();
        }
        // HAIKEI_NEXT: the HIK layers move on to their next animation.
        1000 => {
            let now = machine.sys.now();
            if let Some(hik) = &mut machine.sys.gfx.hik {
                hik.next_animation(now);
            }
        }
        // HAIKEI_WAIT / HAIKEI_CHECK: background changes run to completion
        // before the script continues, so none is ever pending.
        1001 => {}
        1002 => machine.store = 0,
        // HAIKEI_GET_POS(x, y) / _GET_POSX / _GET_POSY
        1100 => {
            let (x, y) = machine.sys.gfx.haikei_pos;
            for (index, value) in [x, y].into_iter().enumerate() {
                let target = machine.int_target_param(command, index)?;
                machine.set_target(target, value)?;
            }
        }
        1101 => machine.store = machine.sys.gfx.haikei_pos.0,
        1102 => machine.store = machine.sys.gfx.haikei_pos.1,
        // HAIKEI_SET_POS(x, y) / _X / _Y, HAIKEI_ADD_POS(x, y) / _X / _Y
        1103..=1108 => {
            let v = machine.int_params_from(command, 0)?;
            let get = |i: usize| v.get(i).copied().unwrap_or(0);
            let mut pos = machine.sys.gfx.haikei_pos;
            match command.op.opcode {
                1103 => pos = (get(0), get(1)),
                1104 => pos.0 = get(0),
                1105 => pos.1 = get(0),
                1106 => pos = (pos.0 + get(0), pos.1 + get(1)),
                1107 => pos.0 += get(0),
                _ => pos.1 += get(0),
            }
            machine.sys.gfx.set_haikei_pos(pos);
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

/// Shows a background: `name` an image or HIK animation (`None`: black,
/// `???`: the default), with the transition of the 16 `#SEL` values.
fn load_haikei(
    machine: &mut Machine,
    name: Option<String>,
    values: &[i32],
    promote: bool,
) -> Result<()> {
    // '???' (compiled as "?" by some tools): the default bgr.
    let name = name.map(|name| {
        if !name.is_empty() && name.chars().all(|c| c == '?') {
            machine.sys.default_bgr.clone()
        } else {
            name
        }
    });
    let before = crate::screen::compose_scene(&mut machine.sys);
    let now = machine.sys.now();
    let mut image = None;
    machine.sys.gfx.hik = None;
    if let Some(name) = &name {
        machine.sys.default_bgr.clone_from(name);
        let sys = &mut machine.sys;
        match sys.resources.read(Kind::Hik, name) {
            Some(bytes) => {
                let script = {
                    let (gfx, resources) = (&mut sys.gfx, &sys.resources);
                    crate::hik::HikScript::parse(&bytes, |frame| gfx.load_image(resources, frame))?
                };
                let mut renderer = crate::hik::HikRenderer::new(name, Rc::new(script), now);
                renderer.offset = sys.gfx.haikei_pos;
                sys.gfx.hik = Some(renderer);
            }
            None => {
                image = Some(sys.gfx.load_image(&sys.resources, name)?);
                crate::cgtable::mark_viewed(machine, name);
            }
        }
    }
    let gfx = &mut machine.sys.gfx;
    gfx.stack.clear();
    gfx.haikei_image = image;
    let dc0 = gfx.dc_mut(0)?;
    let rect = dc0.rect();
    dc0.fill(rect, [0, 0, 0, 255], 255);
    gfx.draw_haikei();
    if promote {
        gfx.promote_objects();
    }
    let after = crate::screen::compose_scene(&mut machine.sys);
    let transition = Transition::from_sel(values);
    crate::modules::grp::start_transition(machine, transition, before, after);
    Ok(())
}

/// Debugging functions: messages go to the diagnostics log; queries answer
/// "not debugging".
pub fn debug(machine: &mut Machine, command: &Command) -> Result<Next> {
    match command.op.opcode {
        0..=2 | 10 | 11 | 12 => {
            let message = match Machine::param_count(command) {
                0 => String::new(),
                _ => machine
                    .str_param(command, 0)
                    .or_else(|_| machine.int_param(command, 0).map(|v| v.to_string()))?,
            };
            machine.report(format!(
                "SEEN{:04} line {}: debug message: {message}",
                machine.scene_number(),
                machine.line
            ));
            // OK / Yes.
            machine.store = 1;
        }
        // __DebugMsgBox YESNOCANCEL: yes.
        3 => machine.store = 1,
        20 | 21 => machine.store = 0,
        40 | 41 => machine.store = 0,
        // The rest only matter in the original's debug build: message
        // suppression (DEBUG_MESSAGE_NONE_*), window-caption rewriting,
        // debug keys, timing reports and buffer dumps (__SaveBuffer*).
        13..=17 | 32 | 50 | 100 | 101 => {}
        // __DebugInputStr(prompt, buf) / __DebugInputInt(prompt, num): as
        // if nothing was typed.
        30 | 31 => {
            if command.params.len() > 1 {
                if command.op.opcode == 30 {
                    let target = machine.str_target_param(command, 1)?;
                    machine.write_string(target, String::new())?;
                } else {
                    let target = machine.int_target_param(command, 1)?;
                    machine.set_target(target, 0)?;
                }
            }
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}
