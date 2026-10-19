//! Opcode modules, addressed by `(modtype, module)`.

pub mod grp;
pub mod jmp;
pub mod mem;
pub mod menu;
pub mod mov;
pub mod msg;
pub mod obj;
pub mod scr;
pub mod sel;
pub mod shk;
pub mod snm;
pub mod sound;
pub mod str;
pub mod sys;
pub mod sys_max;

use anyhow::Result;

use crate::bytecode::Command;
use crate::machine::{Machine, Next};

pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let op = command.op;
    match (op.modtype, op.module) {
        (0, 1 | 5 | 6) => jmp::dispatch(machine, command),
        (0, 2) => sel::dispatch(machine, command),
        (0, 3) => msg::dispatch(machine, command),
        (0, 4) => sys::dispatch_event_loop(machine, command),
        (1, 4) => sys::dispatch(machine, command),
        (2, 1) => dll(machine, command),
        (1, 10) => str::dispatch(machine, command),
        (1, 11) => mem::dispatch(machine, command),
        (1, 12 | 13) => shk::dispatch(machine, command),
        (1, 20) => sound::bgm(machine, command),
        (1, 21) => sound::pcm(machine, command),
        (1, 22) => sound::se(machine, command),
        (1, 23) => sound::koe(machine, command),
        (1, 26) => mov::dispatch(machine, command),
        (1, 5) => os(machine, command),
        (1, 14) => g00_buffers(machine, command),
        (1, 41) => flash(machine, command),
        (1, 34) => snm::dispatch(machine, command),
        (1, 30) => scr::scr(machine, command),
        // refresh: end the frame so the screen is drawn.
        (1, 31) => {
            machine.yield_frame = true;
            machine.sys.gfx.refresh_requested = true;
            Ok(Next::Advance)
        }
        (1, 40) => scr::bgr(machine, command),
        (1, 255) => scr::debug(machine, command),
        (1, 33) => grp::dispatch(machine, command),
        (1 | 2, 60..=62) => obj::management(machine, command),
        (1 | 2, 71 | 72) => obj::creation(machine, command),
        (1 | 2, 73 | 74) => obj::animation(machine, command),
        (1 | 2, 81 | 82 | 87 | 88 | 90 | 91 | 93 | 94) => obj::properties(machine, command),
        (1 | 2, 84 | 85) => obj::getters(machine, command),
        _ => machine.unimplemented(command),
    }
}

/// `G00BUF_LOAD(buf, file)` / `G00BUF_FREE(buf)` / `G00BUF_FREEALL` (1:14):
/// images held in memory ahead of use.
fn g00_buffers(machine: &mut Machine, command: &Command) -> Result<Next> {
    match command.op.opcode {
        1000 => {
            let buf = machine.int_param(command, 0)?;
            let name = machine.str_param(command, 1)?;
            let sys = &mut machine.sys;
            let image = sys.gfx.load_image(&sys.resources, &name)?;
            sys.gfx
                .preloaded
                .insert(buf, (name.trim().to_lowercase(), image));
        }
        1001 => {
            let buf = machine.int_param(command, 0)?;
            machine.sys.gfx.preloaded.remove(&buf);
        }
        1002 => machine.sys.gfx.preloaded.clear(),
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

/// The Box module (1:41): `BOXFLUSH`, `RECTFLUSH` (flash the screen or an
/// area), their `FADE` forms, `BOXPIKACHU` / `RECTPIKACHU` (blink `cnt`
/// times); `NEXT` forms do not wait. BOX areas are corners, RECT areas
/// position and size.
fn flash(machine: &mut Machine, command: &Command) -> Result<Next> {
    let opcode = command.op.opcode;
    // FLUSHSTOP / FLUSHWAIT / FLUSHCHECK, PIKACHUSTOP / _WAIT / _CHECK
    if matches!(opcode, 100..=102 | 110..=112) {
        let blink = opcode >= 110;
        let gfx = &mut machine.sys.gfx;
        let now = machine.sys.clock.now();
        let running = gfx
            .flash
            .as_ref()
            .is_some_and(|flash| flash.blink == blink && !flash.finished(now));
        match opcode % 10 {
            0 => {
                if running {
                    gfx.flash = None;
                    gfx.dirty = true;
                }
            }
            1 => {
                let wait = crate::longop::Wait::event(crate::longop::WaitEvent::Flash);
                machine.push_long_op(Box::new(wait));
            }
            _ => machine.store = i32::from(running),
        }
        return Ok(Next::Advance);
    }
    if opcode > 33 || opcode % 10 > 3 {
        return machine.unimplemented(command);
    }
    let rect_form = opcode % 2 == 1;
    let wait = opcode % 10 < 2;
    let fade = matches!(opcode / 10, 1 | 3);
    let blink = opcode >= 20;
    let v = machine.int_params_from(command, 0)?;
    let colour_at = if v.len() >= 7 { 4 } else { 0 };
    let area = (v.len() >= 7).then(|| {
        if rect_form {
            crate::surface::Rect::new(v[0], v[1], v[2], v[3])
        } else {
            crate::surface::Rect::from_corners(v[0], v[1], v[2], v[3])
        }
    });
    let byte = |i: usize| v.get(i).copied().unwrap_or(0).clamp(0, 255) as u8;
    let colour = [byte(colour_at), byte(colour_at + 1), byte(colour_at + 2)];
    let mut rest = v.get(colour_at + 3..).unwrap_or(&[]).iter().copied();
    let count = if blink {
        rest.next().unwrap_or(1).max(1) as u32
    } else {
        1
    };
    let time = rest.next().unwrap_or(if blink { 100 } else { 50 }).max(1) as u64;
    let now = machine.sys.now();
    machine.sys.gfx.flash = Some(crate::graphics::Flash {
        area,
        colour,
        start: now,
        time,
        count,
        fade,
        blink,
    });
    if wait {
        let until = now + time * u64::from(count);
        let wait = crate::longop::Wait::for_ms(machine, (until - now) as i32);
        machine.push_long_op(Box::new(wait));
    }
    Ok(Next::Advance)
}

/// The Os module (1:5).
fn os(machine: &mut Machine, command: &Command) -> Result<Next> {
    match command.op.opcode {
        // shell(file[, arguments]) / launch(file): only the game's own
        // files (its manual) are passed on, for the host to open.
        0 | 10 => {
            let name = machine.str_param(command, 0)?.replace('\\', "/");
            match machine.sys.resources.root_file(&name) {
                Some(path) => machine.sys.ui.request(crate::ui::Request::OpenFile(path)),
                None => machine.note_unimplemented(format!("shell {name}")),
            }
        }
        // DUMMYCHECK_DISC(file, key, code): the disc is always there.
        114 => machine.store = 1,
        // GET_COMPUTER_ID: a fixed identity (scripts only compare it with
        // the value they stored before).
        120 => machine.store = 1,
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

/// `LoadDLL`, `UnloadDLL`, `CallDLL` (2:1).
fn dll(machine: &mut Machine, command: &Command) -> Result<Next> {
    match command.op.opcode {
        10 => {
            let slot = machine.int_param(command, 0)?;
            let name = machine.str_param(command, 1)?;
            machine.dlls.load(slot, &name);
        }
        11 => {
            let slot = machine.int_param_or(command, 0, 0)?;
            machine.dlls.unload(slot);
        }
        12 => {
            let slot = machine.int_param(command, 0)?;
            let mut args = [0; 5];
            for (i, arg) in args.iter_mut().enumerate() {
                *arg = machine.int_param_or(command, i + 1, 0)?;
            }
            machine.store = machine.call_dll(slot, args)?;
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}
