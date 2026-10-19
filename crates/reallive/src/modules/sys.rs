//! System functions (modules 0:4 and 1:4): timers, frame counters, waits,
//! arithmetic helpers, date and time, the call stack, skip and auto mode,
//! player settings, window settings, names and system commands.

use anyhow::{Result, bail};

use crate::bytecode::Command;
use crate::clock::{FrameCounter, FrameKind};
use crate::expr::Expr;
use crate::longop::{Wait, WaitEvent};
use crate::machine::{Frame, FrameKind as CallKind, IntTarget, Machine, Next};
use crate::settings::{WindowAttr, syscom};

/// Modtype 0: event-loop functions.
pub fn dispatch_event_loop(machine: &mut Machine, command: &Command) -> Result<Next> {
    match command.op.opcode {
        // SetInterrupt(scenario, entrypoint) / ClearInterrupt()
        120 => {
            let scene = machine.int_param(command, 0)?;
            let entrypoint = machine.int_param(command, 1)?;
            machine.interrupt = Some((scene, entrypoint));
        }
        121 => machine.interrupt = None,
        // rtlButton / rtlCancel / rtlSystem
        300..=302 => {
            machine.return_from_farcall()?;
            machine.sys.in_menu = false;
            return Ok(Next::Jumped);
        }
        // yield: return from an interrupt handler.
        303 => {
            if machine.in_interrupt {
                machine.return_from_farcall()?;
                machine.in_interrupt = false;
                return Ok(Next::Jumped);
            }
            machine.yield_frame = true;
        }
        // MsgBox(title, message[, ALERT_OKCANCEL])
        401 => {
            let title = machine.str_param(command, 0)?;
            let message = match command.params.get(1) {
                Some(param) if param.value.is_string() => machine.str_param(command, 1)?,
                Some(_) => machine.int_param(command, 1)?.to_string(),
                None => String::new(),
            };
            let op = crate::ui::MessageBox::new(title, message, command.op.overload == 1);
            machine.push_long_op(Box::new(op));
        }
        // ShowBackground: hide the text windows until a click.
        1000 => crate::modules::msg::show_background(machine)?,
        1100 => machine.sys.syscom.skip_mode = true,
        1101 => machine.sys.syscom.skip_mode = false,
        1102 => machine.store = i32::from(machine.sys.syscom.skip_mode),
        // CCOM_*: what stays visible while a script-drawn system menu is up.
        // CCOM_MESSAGEWINDOW_ON / _OFF([window | first, last])
        1200 | 1201 => {
            let off = command.op.opcode == 1201;
            let windows = window_range(machine, command)?;
            for index in windows {
                if let Some(flag) = machine.sys.text.display_off.get_mut(index) {
                    *flag = off;
                }
            }
        }
        // CCOM_BTNSEL_ON / _OFF
        1202 => machine.sys.gfx.ccom_hide_buttons = false,
        1203 => machine.sys.gfx.ccom_hide_buttons = true,
        // CCOM_OBJECT_ON / _OFF(buffer | first, last)
        1204 | 1205 => {
            let first = machine.int_param(command, 0)?;
            let last = machine.int_param_or(command, 1, first)?;
            for index in first..=last {
                if command.op.opcode == 1205 {
                    machine.sys.gfx.ccom_hidden_objects.insert(index);
                } else {
                    machine.sys.gfx.ccom_hidden_objects.remove(&index);
                }
            }
        }
        // CCOM_LOCAL_FLAG_EXCOPY(var): copies the variable's value at the
        // last savepoint into it (what a load would restore).
        2000 => {
            for index in 0..command.params.len() {
                let target = machine.int_target_param(command, index)?;
                if let IntTarget::Mem(reference) = target {
                    if let Some(value) = machine.memory.savepoint_int(reference) {
                        machine.set_target(target, value)?;
                    }
                }
            }
        }
        // CHECK_JUST_AFTER_LOAD
        3001 => machine.store = i32::from(std::mem::take(&mut machine.just_loaded)),
        _ => return crate::modules::sys_max::event_loop(machine, command),
    }
    Ok(Next::Advance)
}

/// `TONECURVE_BACKGROUND/FOREGROUND/PDTEXPAND/ALLOBJECT_ON/OFF` (700..707),
/// `TONECURVE_OBJECT_ON/OFF(buf[, curve])` (708/709), `..._ONS/OFFS(first,
/// last[, curve])` (710/711), `TONECURVE_FACE_ON/OFF` (712/713). A curve
/// given as a file name uses the first table.
fn tone_curve(machine: &mut Machine, command: &Command) -> Result<()> {
    let opcode = command.op.opcode;
    let curve_at = |machine: &mut Machine, index: usize| -> Result<Option<usize>> {
        let Some(param) = command.params.get(index) else {
            return Ok(Some(0).filter(|_| machine.sys.gfx.tone_curves.table(0).is_some()));
        };
        if param.value.is_string() {
            return Ok(machine.sys.gfx.tone_curves.table(0));
        }
        let number = machine.int_param(command, index)?;
        Ok(machine.sys.gfx.tone_curves.table(number))
    };
    let on = opcode % 2 == 0;
    match opcode {
        700..=707 | 712 | 713 => {
            let table = if on { curve_at(machine, 0)? } else { None };
            let gfx = &mut machine.sys.gfx;
            match opcode {
                700 | 701 => gfx.background_tone = table,
                702 | 703 => gfx.foreground_tone = table,
                712 | 713 => gfx.face_tone = table,
                // PDTEXPAND (images as they load) and ALLOBJECT: every
                // object.
                _ => gfx.all_objects_tone = table,
            }
        }
        708..=711 => {
            let (first, last, curve) = if opcode >= 710 {
                (
                    machine.int_param(command, 0)?,
                    machine.int_param(command, 1)?,
                    2,
                )
            } else {
                let buf = machine.int_param(command, 0)?;
                (buf, buf, 1)
            };
            let table = if on { curve_at(machine, curve)? } else { None };
            for buf in first.max(0)..=last.max(0) {
                match table {
                    Some(table) => machine.sys.gfx.object_tones.insert(buf as usize, table),
                    None => machine.sys.gfx.object_tones.remove(&(buf as usize)),
                };
            }
        }
        _ => {}
    }
    machine.sys.gfx.dirty = true;
    Ok(())
}

/// Writes `values` to `target` and the variables after it.
fn write_consecutive(machine: &mut Machine, target: IntTarget, values: &[i32]) -> Result<()> {
    match target {
        IntTarget::Mem(reference) => {
            for (offset, &value) in values.iter().enumerate() {
                let next = crate::memory::IntRef {
                    index: reference.index + offset as i32,
                    ..reference
                };
                machine.set_target(IntTarget::Mem(next), value)?;
            }
        }
        IntTarget::Store => {
            if let Some(&value) = values.first() {
                machine.store = value;
            }
        }
    }
    Ok(())
}

/// Text windows named by `(window)`, `(first, last)` or nothing (all).
fn window_range(
    machine: &mut Machine,
    command: &Command,
) -> Result<std::ops::RangeInclusive<usize>> {
    let count = machine.sys.text.states.len();
    Ok(match command.params.len() {
        0 => 0..=count - 1,
        1 => {
            let w = machine.int_param(command, 0)?.clamp(0, count as i32 - 1) as usize;
            w..=w
        }
        _ => {
            let a = machine.int_param(command, 0)?.clamp(0, count as i32 - 1) as usize;
            let b = machine.int_param(command, 1)?.clamp(0, count as i32 - 1) as usize;
            a..=b
        }
    })
}

/// The `DefaultIntValue`-style optional counter parameter.
fn counter_param(machine: &mut Machine, command: &Command, index: usize) -> Result<i32> {
    machine.int_param_or(command, index, 0)
}

fn frame_layer(opcode: u16) -> usize {
    usize::from(matches!(opcode, 520..=534 | 620..=634))
}

/// Integer degrees → the RealLive fixed-point sine scale.
fn trig(value: i32, divisor: i32, f: fn(f64) -> f64) -> i32 {
    let result = f(f64::from(value).to_radians()) * 32640.0;
    let result = if divisor != 0 {
        result / f64::from(divisor)
    } else {
        result
    };
    result as i32
}

/// `index_series` interpolation: linear, accelerating or decelerating.
fn interpolate(start: i32, current: i32, end: i32, amount: i32, mode: i32) -> i32 {
    if end == start {
        return amount;
    }
    let p = f64::from(current - start) / f64::from(end - start);
    let shaped = match mode {
        1 => 2f64.powf(p) - 1.0,
        2 => (p + 1.0).log2(),
        3 => p * p,
        4 => 1.0 - (1.0 - p) * (1.0 - p),
        _ => p,
    };
    (shaped * f64::from(amount)) as i32
}

/// `TIMETABLE2` / `TIMETABLELEN2`: the value at `nowTime + repTime` of a
/// piecewise animation starting at `startNum` at `startTime`. With
/// `lengths`, the segment times are durations rather than end times.
fn timetable2(machine: &mut Machine, command: &Command, lengths: bool) -> Result<i32> {
    let now = machine.int_param(command, 0)? + machine.int_param(command, 1)?;
    let mut start = machine.int_param(command, 2)?;
    let mut value = machine.int_param(command, 3)?;
    let mut segments = Vec::new();
    let mut total = start;
    for param in command.params.iter().skip(4) {
        let Expr::Special { tag, pieces } = &param.value else {
            continue;
        };
        let mut v = pieces
            .iter()
            .map(|piece| machine.eval_int(piece))
            .collect::<Result<Vec<_>>>()?;
        let kind = *tag & 0xffff;
        // _SET has no time; the others start with one.
        if lengths && kind != 49 && !v.is_empty() {
            total += v[0];
            v[0] = total;
        }
        segments.push((kind, *tag >> 16, v));
    }
    if start > now {
        return Ok(value);
    }
    let arg = |v: &[i32], i: usize| v.get(i).copied().unwrap_or(0);
    for (kind, overload, v) in segments {
        let (end, target) = (arg(&v, 0), arg(&v, 1));
        let inside = now > start && now <= end;
        match kind {
            // _MOVE(endTime, endNum[, mod])
            48 => {
                let mode = if overload == 1 { arg(&v, 2) } else { 0 };
                if inside {
                    return Ok(value + interpolate(start, now, end, target - value, mode));
                }
                value = target;
                start = end;
            }
            // _SET(endNum)
            49 => value = arg(&v, 0),
            // _WAIT(endTime)
            50 => {
                if inside {
                    return Ok(value);
                }
                start = end;
            }
            // _TURN / _TURNUP / _LOOP / _JUMP(endTime, endNum, count)
            51..=54 => {
                let count = arg(&v, 2).max(1);
                if inside {
                    let cycle = ((end - start) / count).max(1);
                    let p = f64::from((now - start) % cycle) / f64::from(cycle);
                    let shape = match kind {
                        // there and back
                        51 => 1.0 - (p * 2.0 - 1.0).abs(),
                        // up, then straight back
                        52 | 53 => p,
                        // a parabola, as rlvm does
                        _ => 1.0 - (p * 2.0 - 1.0).powi(2),
                    };
                    return Ok(value + (f64::from(target - value) * shape) as i32);
                }
                if kind != 51 {
                    value = target;
                }
                start = end;
            }
            // _WAITSET(endTime, endNum)
            55 => {
                if inside {
                    return Ok(value);
                }
                value = target;
                start = end;
            }
            _ => {}
        }
    }
    Ok(value)
}

pub(crate) fn index_series(machine: &mut Machine, command: &Command, lengths: bool) -> Result<i32> {
    let index = machine.int_param(command, 0)?;
    let offset = machine.int_param(command, 1)?;
    let mut init = machine.int_param(command, 2)?;
    let mut value = init;
    let mut previous_finished = false;
    // TIMETABLELEN: each segment's start and end count from the previous
    // segment's end.
    let mut elapsed = 0;
    for param in command.params.iter().skip(3) {
        let Expr::Special { tag, pieces } = &param.value else {
            continue;
        };
        let values = pieces
            .iter()
            .map(|piece| machine.eval_int(piece))
            .collect::<Result<Vec<_>>>()?;
        match (tag, values.as_slice()) {
            (0, [val, ..]) => {
                if previous_finished {
                    value = *val;
                    init = *val;
                }
            }
            (1 | 2, [start, end, end_value, rest @ ..]) => {
                let mode = rest.first().copied().unwrap_or(0);
                let (start, end) = if lengths {
                    let start = elapsed + start;
                    elapsed = start + end;
                    (start, elapsed)
                } else {
                    (*start, *end)
                };
                let (start, end) = (start + offset, end + offset);
                if index > start && index < end {
                    value += interpolate(start, index, end, end_value - init, mode);
                    previous_finished = false;
                } else if index >= end {
                    value = *end_value;
                    init = *end_value;
                    previous_finished = true;
                }
            }
            _ => {}
        }
    }
    Ok(value)
}

fn now_parts() -> [i32; 8] {
    use chrono::{Datelike, Local, Timelike};
    let now = Local::now();
    [
        now.year(),
        now.month() as i32,
        now.day() as i32,
        now.weekday().num_days_from_sunday() as i32,
        now.hour() as i32,
        now.minute() as i32,
        now.second() as i32,
        (now.nanosecond() / 1_000_000).min(999) as i32,
    ]
}

fn write_targets(
    machine: &mut Machine,
    command: &Command,
    from: usize,
    values: &[i32],
) -> Result<()> {
    for (offset, value) in values.iter().enumerate() {
        if from + offset >= command.params.len() {
            break;
        }
        let target = machine.int_target_param(command, from + offset)?;
        machine.set_target(target, *value)?;
    }
    Ok(())
}

fn window_attr_values(attr: WindowAttr) -> [i32; 5] {
    [attr.r, attr.g, attr.b, attr.alpha, attr.filter]
}

fn set_attr_component(attr: &mut WindowAttr, component: usize, value: i32) {
    match component {
        0 => attr.r = value,
        1 => attr.g = value,
        2 => attr.b = value,
        3 => attr.alpha = value,
        _ => attr.filter = value,
    }
}

/// Modtype 1: the main system module.
pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let op = command.op;
    let now = machine.sys.now();
    match op.opcode {
        // title(text)
        0 => machine.sys.title = machine.str_param(command, 0)?,
        // GET_TITLE(str)
        2 => {
            let target = machine.str_target_param(command, 0)?;
            machine.write_string(target, machine.sys.title.clone())?;
        }
        // wait / waitC
        100 | 101 => {
            let time = machine.int_param(command, 0)?;
            let mut wait = Wait::for_ms(machine, time);
            if op.opcode == 101 {
                wait = wait.cancellable();
            }
            machine.push_long_op(Box::new(wait));
        }
        // ResetTimer / ResetExTimer
        110 | 120 => {
            let counter = counter_param(machine, command, 0)?;
            machine
                .sys
                .timers
                .set(usize::from(op.opcode == 120), counter, now, 0);
        }
        // time / timeC / timeEx / timeExC (and the undocumented *C2)
        111..=113 | 121..=123 => {
            let layer = usize::from(op.opcode >= 120);
            let time = machine.int_param(command, 0)?;
            let counter = counter_param(machine, command, 1)?;
            if machine.sys.timers.read(layer, counter, now) < time {
                let mut wait = Wait::event(WaitEvent::Timer {
                    layer,
                    counter,
                    time,
                });
                if matches!(op.opcode % 10, 2 | 3) {
                    wait = wait.cancellable();
                }
                machine.push_long_op(Box::new(wait));
            } else if matches!(op.opcode % 10, 2 | 3) {
                machine.store = 0;
            }
        }
        // Timer / ExTimer
        114 | 124 => {
            let counter = counter_param(machine, command, 0)?;
            machine.store = machine
                .sys
                .timers
                .read(usize::from(op.opcode == 124), counter, now);
        }
        // CmpTimer / CmpExTimer
        115 | 125 => {
            let time = machine.int_param(command, 0)?;
            let counter = counter_param(machine, command, 1)?;
            let value = machine
                .sys
                .timers
                .read(usize::from(op.opcode == 125), counter, now);
            machine.store = i32::from(value > time);
        }
        // SetTimer / SetExTimer
        116 | 126 => {
            let time = machine.int_param(command, 0)?;
            let counter = counter_param(machine, command, 1)?;
            machine
                .sys
                .timers
                .set(usize::from(op.opcode == 126), counter, now, time);
        }
        // KEYWAIT: until a click or key.
        106 => {
            let wait = Wait::event(WaitEvent::None).cancellable();
            machine.push_long_op(Box::new(wait));
        }
        // CLEAR_KEYTABLE([code...]): forget presses (of those keys).
        150 => {
            let codes = machine.int_params_from(command, 0)?;
            let input = &mut machine.sys.input;
            if codes.is_empty() {
                input.keys_pressed.clear();
            } else {
                for code in codes {
                    input.keys_pressed.remove(&code);
                }
            }
        }
        // GET_KEYTABLE_DATA / GET_KEYTABLE_REAL(buffer, code...): per key,
        // pressed since cleared / held now, into consecutive variables.
        151 | 152 => {
            let target = machine.int_target_param(command, 0)?;
            let codes = machine.int_params_from(command, 1)?;
            let input = &machine.sys.input;
            let values: Vec<i32> = codes
                .iter()
                .map(|code| {
                    let set = if op.opcode == 151 {
                        &input.keys_pressed
                    } else {
                        &input.keys_held
                    };
                    i32::from(set.contains(code))
                })
                .collect();
            write_consecutive(machine, target, &values)?;
        }
        // FlushClick
        130 => machine.sys.input.flush_clicks(),
        // GetClick(x, y) / WaitClick(time, x, y)
        131 => {
            let x = machine.int_target_param(command, 0)?;
            let y = machine.int_target_param(command, 1)?;
            let wait = Wait::event(WaitEvent::None).recording(x, y);
            machine.push_long_op(Box::new(wait));
        }
        132 => {
            let time = machine.int_param(command, 0)?;
            let x = machine.int_target_param(command, 1)?;
            let y = machine.int_target_param(command, 2)?;
            let wait = Wait::for_ms(machine, time).recording(x, y);
            machine.push_long_op(Box::new(wait));
        }
        // GetCursorPos(x, y, button1, button2) / GetCursorPos(x, y)
        133 | 202 | 138 => {
            let (x, y) = machine.sys.input.mouse;
            // 1 pressed, 2 released (a click) until `FlushClick`.
            let buttons = machine.sys.input.button_state;
            let mut values = vec![x, y];
            if op.opcode == 133 {
                values.extend(buttons);
            }
            write_targets(machine, command, 0, &values)?;
        }
        // SET_WAIP_WINDOWCLOSE_ON / _OFF, SET_GRPCOM_WINDOWCLOSE_ON / _OFF
        210 => machine.sys.wipe_closes_windows = true,
        211 => machine.sys.wipe_closes_windows = false,
        215 => machine.sys.grp_closes_windows = true,
        216 => machine.sys.grp_closes_windows = false,
        // CLEAR_MSGBK / SET_MSGBK_ON / SET_MSGBK_OFF
        300 => machine.sys.text.backlog.clear(),
        301 => machine.sys.text.backlog_enabled = true,
        302 => machine.sys.text.backlog_enabled = false,
        // SET_READJUMPCANCEL_ON / _OFF
        335 => machine.sys.read_jump_cancel = true,
        336 => machine.sys.read_jump_cancel = false,
        // SET_CTRLKEY_ENABLE / SET_CTRLKEY_DISENABLE
        358 => machine.sys.ctrl_key_skip = true,
        359 => machine.sys.ctrl_key_skip = false,
        // SET_P_CURSORNO(id)
        366 => machine.sys.key_cursor = machine.int_param(command, 0)?,
        // SET_WINDOW_DISP_ON / _OFF([window])
        480 | 481 => {
            let off = op.opcode == 481;
            for index in window_range(machine, command)? {
                if let Some(flag) = machine.sys.text.display_off.get_mut(index) {
                    *flag = off;
                }
            }
        }
        // TONECURVE_*: recolour parts of the screen with a tone curve.
        700..=713 => tone_curve(machine, command)?,
        // TIMETABLE2 / TIMETABLELEN2
        810 | 811 => machine.store = timetable2(machine, command, op.opcode == 811)?,
        // KeyMouseOn / KeyMouseOff
        200 => machine.sys.input.key_mouse = true,
        201 => machine.sys.input.key_mouse = false,
        // SetCursorPos(x, y)
        203 => {
            let x = machine.int_param(command, 0)?;
            let y = machine.int_param(command, 1)?;
            machine.sys.input.mouse = (x, y);
            machine.sys.warp_cursor = Some((x, y));
        }
        204 => machine.sys.cursor_visible = true,
        205 => machine.sys.cursor_visible = false,
        206 => machine.store = machine.sys.mouse_cursor,
        207 => machine.sys.mouse_cursor = machine.int_param(command, 0)?,
        // CallStackClear
        320 => {
            let mut top = machine.stack.pop().expect("a running frame");
            machine.stack.clear();
            top.kind = CallKind::Root;
            machine.stack.push(top);
        }
        // CallStackNop([count])
        321 => {
            let count = machine.int_param_or(command, 0, 1)?.max(0) as usize;
            let current = machine.frame().clone();
            let frame = Frame {
                ip: current.ip + 1,
                kind: CallKind::Gosub,
                ..current
            };
            let top = machine.stack.len() - 1;
            for _ in 0..count {
                machine.stack.insert(top, frame.clone());
            }
        }
        // CallStackPop([count]): returns `count` levels.
        322 => {
            let count = machine.int_param_or(command, 0, 1)?.max(0) as usize;
            let keep = machine.stack.len().saturating_sub(count).max(1);
            machine.stack.truncate(keep);
            return Ok(Next::Jumped);
        }
        323 => machine.store = machine.stack.len() as i32,
        // CallStackTrunc(length)
        324 => {
            let length = machine.int_param(command, 0)?.max(1) as usize;
            if length < machine.stack.len() {
                machine.stack.truncate(length);
                return Ok(Next::Jumped);
            }
        }
        // EnableSkipMode / DisableSkipMode
        330 => machine.sys.syscom.skip_mode_allowed = true,
        331 => machine.sys.syscom.skip_mode_allowed = false,
        // LocalSkipMode / SetLocalSkipMode / ClearLocalSkipMode
        332 => machine.store = i32::from(machine.sys.syscom.skip_mode),
        333 => machine.sys.syscom.skip_mode = true,
        334 => machine.sys.syscom.skip_mode = false,
        // CtrlKeySkip / On / Off / CtrlPressed / ShiftPressed
        350 => machine.store = i32::from(machine.sys.ctrl_key_skip),
        351 => machine.sys.ctrl_key_skip = true,
        352 => machine.sys.ctrl_key_skip = false,
        353 => machine.store = i32::from(machine.sys.input.ctrl),
        354 => machine.store = i32::from(machine.sys.input.shift),
        // PauseCursor(index)
        364 => machine.sys.key_cursor = machine.int_param(command, 0)?,
        // GetWindowPos(window, origin, x, y) / GetDefaultWindowPos
        400 | 403 => {
            let window = machine.int_param(command, 0)?;
            let config = machine
                .sys
                .text
                .window(window)
                .ok_or_else(|| anyhow::anyhow!("reallive: no text window {window}"))?;
            let pos = if op.opcode == 400 {
                config.pos
            } else {
                config.default_pos
            };
            write_targets(machine, command, 1, &[pos.origin, pos.x, pos.y])?;
        }
        // SetWindowPos(window, origin, x, y) / SetDefaultWindowPos
        401 | 404 => {
            let window = machine.int_param(command, 0)?;
            let origin = machine.int_param(command, 1)?;
            let x = machine.int_param(command, 2)?;
            let y = machine.int_param(command, 3)?;
            if let Some(config) = machine.sys.text.window_mut(window) {
                let pos = crate::text::WindowPos { origin, x, y };
                if op.opcode == 401 {
                    config.pos = pos;
                } else {
                    config.default_pos = pos;
                }
            }
        }
        // WindowResetPos(window) / DefaultWindowResetPos
        402 | 405 => {
            let window = machine.int_param(command, 0)?;
            let exe = machine.gameexe.clone();
            if let Some(config) = machine.sys.text.window_mut(window) {
                let fresh = crate::text::WindowConfig::from_gameexe(&exe, config.index);
                if op.opcode == 402 {
                    config.pos = config.default_pos;
                } else {
                    config.default_pos = fresh.default_pos;
                }
            }
        }
        // GetWakuAll / SetWakuAll
        410 => machine.store = machine.sys.settings.waku_all,
        411 => machine.sys.settings.waku_all = machine.int_param(command, 0)?,
        // GetWaku(window) / SetWaku(window, index)
        412 => {
            let window = machine.int_param(command, 0)?;
            let waku_all = machine.sys.settings.waku_all;
            machine.store = machine
                .sys
                .text
                .window(window)
                .map_or(0, |config| config.waku_pattern(waku_all));
        }
        413 => {
            let window = machine.int_param(command, 0)?;
            let index = machine.int_param(command, 1)?;
            if let Some(config) = machine.sys.text.window_mut(window) {
                config.waku_no = index;
            }
        }
        // GetWakuMod / SetWakuMod 0 / 1
        414 => {
            let window = machine.int_param(command, 0)?;
            machine.store = machine.sys.text.window(window).map_or(0, |c| c.waku_mod);
        }
        415 | 416 => {
            let window = machine.int_param(command, 0)?;
            if let Some(config) = machine.sys.text.window_mut(window) {
                config.waku_mod = i32::from(op.opcode == 416);
            }
        }
        // GetWindowAttr2 / SetWindowAttr2: the global attributes.
        420 => {
            let values = window_attr_values(machine.sys.settings.window_attr);
            write_targets(machine, command, 0, &values)?;
        }
        421 => {
            let values = machine.int_params_from(command, 0)?;
            if let Some(attr) = WindowAttr::from_slice(&values) {
                machine.sys.settings.window_attr = attr;
            }
        }
        // GetWindowModAttr(window, r, g, b, a, f) / SetWindowModAttr
        422 => {
            let window = machine.int_param(command, 0)?;
            let attr = machine
                .sys
                .text
                .window(window)
                .map(|c| c.attr)
                .unwrap_or_default();
            write_targets(machine, command, 1, &window_attr_values(attr))?;
        }
        423 => {
            let window = machine.int_param(command, 0)?;
            let values = machine.int_params_from(command, 1)?;
            if let (Some(config), Some(attr)) = (
                machine.sys.text.window_mut(window),
                WindowAttr::from_slice(&values),
            ) {
                config.attr = attr;
            }
        }
        // GetWindowAttrMod(window) / SetWindowAttrMod 0/1/2
        424 => {
            let window = machine.int_param(command, 0)?;
            machine.store = machine.sys.text.window(window).map_or(0, |c| c.attr_mod);
        }
        425..=427 => {
            let window = machine.int_param(command, 0)?;
            if let Some(config) = machine.sys.text.window_mut(window) {
                config.attr_mod = i32::from(op.opcode - 425);
            }
        }
        // Undocumented get/enable/disable flag triples.
        430..=457 => {
            let base = 430 + (op.opcode - 430) / 5 * 5;
            let slot = usize::from((op.opcode - 430) / 5);
            let flags = &mut machine.sys.misc_flags;
            if flags.len() <= slot {
                flags.resize(slot + 1, 0);
            }
            match op.opcode - base {
                0 => machine.store = flags[slot],
                1 => flags[slot] = 1,
                2 => flags[slot] = 0,
                _ => {}
            }
        }
        // EnableWindowAnm / DisableWindowAnm([window])
        460 | 461 => {
            for index in window_range(machine, command)? {
                if let Some(config) = machine.sys.text.window_mut(index as i32) {
                    config.anm_enabled = op.opcode == 460;
                }
            }
        }
        // Get/Set Open/Close AnmMod/AnmTime
        462..=469 => {
            let window = machine.int_param(command, 0)?;
            let setter = op.opcode % 2 == 1;
            let value = if setter {
                machine.int_param(command, 1)?
            } else {
                0
            };
            let Some(config) = machine.sys.text.window_mut(window) else {
                bail!("reallive: no text window {window}");
            };
            let field = match op.opcode {
                462 | 463 => &mut config.open_anm_mod,
                464 | 465 => &mut config.open_anm_time,
                466 | 467 => &mut config.close_anm_mod,
                _ => &mut config.close_anm_time,
            };
            if setter {
                *field = value;
            } else {
                machine.store = *field;
            }
        }
        // InitFrame* / InitExFrame*
        500..=504 | 520..=524 => {
            let counter = machine.int_param(command, 0)?;
            let from = machine.int_param(command, 1)?;
            let to = machine.int_param(command, 2)?;
            let time = machine.int_param(command, 3)?;
            let kind = FrameKind::from_opcode(op.opcode);
            machine.sys.frames.set(
                frame_layer(op.opcode),
                counter,
                FrameCounter::new(kind, from, to, time, now),
            );
        }
        // ReadFrame / FrameActive / AnyFrameActive (and Ex)
        510 | 530 => {
            let counter = machine.int_param(command, 0)?;
            machine.store = machine
                .sys
                .frames
                .read(frame_layer(op.opcode), counter, now);
        }
        511 | 531 => {
            let counter = machine.int_param(command, 0)?;
            machine.store = i32::from(machine.sys.frames.active(
                frame_layer(op.opcode),
                counter,
                now,
            ));
        }
        512 | 532 => {
            machine.store = i32::from(machine.sys.frames.any_active(frame_layer(op.opcode), now));
        }
        // ClearFrame(counter[, value]) / ClearAllFrames([value])
        513 | 533 => {
            let counter = machine.int_param(command, 0)?;
            let value = machine.int_param_or(command, 1, 0)?;
            if let Some(frame) = machine.sys.frames.get_mut(frame_layer(op.opcode), counter) {
                frame.stop(value);
            }
        }
        514 | 534 => {
            let value = machine.int_param_or(command, 0, 0)?;
            machine.sys.frames.clear_all(frame_layer(op.opcode), value);
        }
        // InitFrames*({counter, from, to, time}...)
        600..=604 | 620..=624 => {
            let kind = FrameKind::from_opcode(op.opcode);
            for index in 0..command.params.len() {
                let pieces = machine.complex_param(command, index)?;
                let values = pieces
                    .iter()
                    .map(|piece| machine.eval_int(piece))
                    .collect::<Result<Vec<_>>>()?;
                if let [counter, from, to, time, ..] = values[..] {
                    machine.sys.frames.set(
                        frame_layer(op.opcode),
                        counter,
                        FrameCounter::new(kind, from, to, time, now),
                    );
                }
            }
        }
        // ReadFrames({counter, var}...)
        610 | 630 => {
            let layer = frame_layer(op.opcode);
            let mut any = false;
            for index in 0..command.params.len() {
                let pieces = machine.complex_param(command, index)?;
                let (Some(counter), Some(target)) = (pieces.first(), pieces.get(1)) else {
                    continue;
                };
                let counter = machine.eval_int(counter)?;
                let value = machine.sys.frames.read(layer, counter, now);
                any |= machine.sys.frames.active(layer, counter, now);
                let target = machine.int_target(target)?;
                machine.set_target(target, value)?;
            }
            machine.store = i32::from(any);
        }
        800 => machine.store = index_series(machine, command, false)?,
        // rnd([min], max)
        1000 => {
            let values = machine.int_params_from(command, 0)?;
            machine.store = match values[..] {
                [max] => machine.sys.random(0, max),
                [min, max, ..] => machine.sys.random(min, max),
                [] => 0,
            };
        }
        // pcnt(n, d)
        1001 => {
            let n = machine.int_param(command, 0)?;
            let d = machine.int_param(command, 1)?;
            machine.store = if d == 0 {
                0
            } else {
                (f64::from(n) / f64::from(d) * 100.0).round() as i32
            };
        }
        1002 => machine.store = machine.int_param(command, 0)?.wrapping_abs(),
        // power(base[, exponent])
        1003 => {
            let base = machine.int_param(command, 0)?;
            let exponent = machine.int_param_or(command, 1, 2)?;
            machine.store = if exponent < 0 {
                0
            } else {
                base.wrapping_pow(exponent as u32)
            };
        }
        // sin / cos (value[, divisor]) and the undocumented variants
        1004 | 1010 | 1012 | 1013 => {
            let value = machine.int_param(command, 0)?;
            let divisor = machine.int_param_or(command, 1, 0)?;
            let f = if matches!(op.opcode, 1004 | 1012) {
                f64::sin
            } else {
                f64::cos
            };
            machine.store = trig(value, divisor, f);
        }
        // modulus(x1, y1, x2, y2) / angle(...)
        1005 | 1006 => {
            let v = machine.int_params_from(command, 0)?;
            let [x1, y1, x2, y2] = [0, 1, 2, 3].map(|i| f64::from(v.get(i).copied().unwrap_or(0)));
            let (dx, dy) = (x2 - x1, y2 - y1);
            machine.store = if op.opcode == 1005 {
                (dx * dx + dy * dy).sqrt() as i32
            } else {
                // 0 degrees points up, increasing clockwise.
                let degrees = dx.atan2(-dy).to_degrees();
                ((degrees + 360.0) % 360.0) as i32
            };
        }
        1007 => {
            let a = machine.int_param(command, 0)?;
            let b = machine.int_param(command, 1)?;
            machine.store = a.min(b);
        }
        1008 => {
            let a = machine.int_param(command, 0)?;
            let b = machine.int_param(command, 1)?;
            machine.store = a.max(b);
        }
        // constrain(min, value, max)
        1009 => {
            let min = machine.int_param(command, 0)?;
            let value = machine.int_param(command, 1)?;
            let max = machine.int_param(command, 2)?;
            machine.store = value.max(min).min(max.max(min));
        }
        1011 => machine.store = machine.int_param(command, 0)?.signum(),
        // GetYear .. GetMs
        1100..=1107 => machine.store = now_parts()[usize::from(op.opcode - 1100)],
        // GetDate / GetTime / GetDateTime
        1110 => write_targets(machine, command, 0, &now_parts()[..4])?,
        1111 => write_targets(machine, command, 0, &now_parts()[4..])?,
        1112 => write_targets(machine, command, 0, &now_parts())?,
        1120 => machine.store = machine.scene_number(),
        // DefaultGrp / SetDefaultGrp / DefaultBgr / SetDefaultBgr
        1130 | 1132 => {
            let target = machine.str_target_param(command, 0)?;
            let value = if op.opcode == 1130 {
                machine.sys.default_grp.clone()
            } else {
                machine.sys.default_bgr.clone()
            };
            machine.write_string(target, value)?;
        }
        1131 => machine.sys.default_grp = machine.str_param(command, 0)?,
        1133 => machine.sys.default_bgr = machine.str_param(command, 0)?,
        // end()
        1200 => {
            machine.sys.quit_requested = true;
            machine.halt();
            return Ok(Next::Jumped);
        }
        // MenuReturn / MenuReturn2 / ReturnMenu
        1201..=1203 => {
            crate::modules::menu::return_to_menu(machine, op.opcode != 1203)?;
            return Ok(Next::Jumped);
        }
        // ReturnPrevSelect / ReturnPrevSelect2
        1204 | 1205 => {
            if crate::save::return_to_previous_selection(machine)? {
                return Ok(Next::Jumped);
            }
        }
        // ContextMenu
        1210 => crate::modules::menu::open_context_menu(machine)?,
        // CALL_SYSTEMMENU_MOVIESUPPORT / CALL_SYSTEMMENU_MMXSUPPORT: the
        // original's hardware-support dialogs; nothing to configure here.
        1218 | 1260 => {}
        // OPEN_FULLSCREEN_SETTING_WINDOW
        1219 => machine
            .sys
            .ui
            .request(crate::ui::Request::SettingsDialog(syscom::DISPLAY_MODE)),
        // SET_SELPOINTMOD_ON / _OFF, SELPOINT, SET_SELPOINT_CLEAR
        1221 => machine.selpoint_auto = true,
        1222 => machine.selpoint_auto = false,
        1230 => crate::save::remember_selection(machine),
        1231 => machine.previous_selection = None,
        // EnableSyscom([n]) / HideSyscom([n]) / DisableSyscom(n)
        1211 | 1212 => {
            let state = if op.opcode == 1211 { 1 } else { 0 };
            match command.params.first() {
                Some(_) => {
                    let index = machine.int_param(command, 0)?;
                    machine.sys.syscom.set_state(index, state);
                }
                None => machine.sys.syscom.menu_enabled = state == 1,
            }
        }
        1213 => {
            let index = machine.int_param(command, 0)?;
            machine.sys.syscom.set_state(index, 2);
        }
        1214 => {
            let index = machine.int_param(command, 0)?;
            machine.store = machine.sys.syscom.state(index);
        }
        // InvokeSyscom(n[, value]) / ReadSyscom(n)
        1215 => {
            let index = machine.int_param(command, 0)?;
            let value = match command.params.get(1) {
                Some(_) => Some(machine.int_param(command, 1)?),
                None => None,
            };
            if crate::modules::menu::invoke_syscom(machine, index, value)? {
                return Ok(Next::Jumped);
            }
        }
        1216 => {
            let index = machine.int_param(command, 0)?;
            machine.store = read_syscom(machine, index);
        }
        // GetName / SetName / GetLocalName / SetLocalName
        1300 | 1310 => {
            let index = machine.int_param(command, 0)?.max(0) as usize;
            let target = machine.str_target_param(command, 1)?;
            let name = machine.memory.name(op.opcode == 1310, index).to_owned();
            machine.write_string(target, name)?;
        }
        1301 | 1311 => {
            let index = machine.int_param(command, 0)?.max(0) as usize;
            let name = machine.str_param(command, 1)?;
            machine.memory.set_name(op.opcode == 1311, index, name);
        }
        // nwSingle / nwMulti (+Local): name entry dialogs.
        1302 | 1303 | 1312 | 1313 => {
            let local = op.opcode >= 1310;
            let mut fields = Vec::new();
            if matches!(op.opcode, 1302 | 1312) {
                let index = machine.int_param(command, 0)?;
                let label = machine.str_param(command, 1)?;
                fields.push((index, label));
            } else {
                for index in 0..command.params.len() {
                    let pieces = machine.complex_param(command, index)?;
                    if let (Some(slot), Some(label)) = (pieces.first(), pieces.get(1)) {
                        let slot = machine.eval_int(slot)?;
                        let label = machine.eval_str(label)?;
                        fields.push((slot, label));
                    }
                }
            }
            crate::modules::menu::name_entry(machine, local, fields)?;
        }
        // Saved games
        1409..=1414 | 1421 => crate::save::sys_query(machine, command)?,
        // CG mode
        1500..=1504 => crate::cgtable::sys_query(machine, command)?,
        // CGTABLE_ON / CGTABLE_OFF
        1520 => machine.sys.cg_table_enabled = true,
        1521 => machine.sys.cg_table_enabled = false,
        // ENABLE_SYSTEMMENU_ANIME / DISENABLE_SYSTEMMENU_ANIME
        2402 => machine.sys.system_menu_animation = true,
        2502 => machine.sys.system_menu_animation = false,
        // Text input boxes
        1700..=1711 => crate::modules::menu::text_input(machine, command)?,
        // Miscellaneous flags: getters 2000..2009, setters 2050..2059
        2000..=2009 | 2050..=2059 => {
            let setter = op.opcode >= 2050;
            let which = op.opcode % 50;
            let value = if setter {
                machine.int_param(command, 0)?
            } else {
                0
            };
            let settings = &mut machine.sys.settings;
            let slot: Option<&mut bool> = match which {
                0 => Some(&mut settings.cursor_mono),
                1 => Some(&mut settings.skip_animations),
                2 => Some(&mut settings.low_priority),
                3 => Some(&mut settings.confirm_save_load),
                4 => Some(&mut settings.reduce_distortion),
                _ => None,
            };
            match (slot, setter) {
                (Some(flag), true) => *flag = value != 0,
                (Some(flag), false) => machine.store = i32::from(*flag),
                (None, true) if which == 9 => settings.sound_quality = value,
                (None, false) if which == 9 => machine.store = settings.sound_quality,
                // Undocumented flags (5..=8): remembered as set.
                (None, setter) => {
                    let flags = &mut machine.sys.misc_flags;
                    if flags.len() <= which as usize {
                        flags.resize(which as usize + 1, 0);
                    }
                    if setter {
                        flags[which as usize] = value;
                    } else {
                        machine.store = flags[which as usize];
                    }
                }
            }
        }
        // Setting setters (22xx), getters (23xx) and defaults (26xx).
        2221..=2276 | 2321..=2376 | 2600..=2621 => return settings_op(machine, command),
        // save/load menus and slots
        3000..=3003 | 3006..=3009 | 3100..=3103 | 3106..=3109 => {
            if crate::save::sys_save_load(machine, command)? {
                return Ok(Next::Jumped);
            }
        }
        // QUICKSAVE_NOWARNING / QUICKLOAD_SELKEEP_NOWARNING
        3127 => {
            let slot = crate::ui::quick_slot(machine);
            crate::save::save_slot(machine, slot)?;
        }
        3131 => {
            let slot = crate::ui::quick_slot(machine);
            let selection = machine.previous_selection.clone();
            crate::save::load_slot(machine, slot)?;
            machine.previous_selection = selection;
            return Ok(Next::Jumped);
        }
        // SAVEPOINT_CLEAR: forget the old savepoint; saving now keeps this
        // position.
        3503 => machine.mark_savepoint(),
        // Savepoint / EnableAutoSavepoints / DisableAutoSavepoints
        3500 => machine.mark_savepoint(),
        3501 => machine.mark_savepoints = true,
        3502 => machine.mark_savepoints = false,
        _ => {
            return match crate::modules::sys_max::dispatch(machine, command)? {
                Some(next) => Ok(next),
                None => machine.unimplemented(command),
            };
        }
    }
    Ok(Next::Advance)
}

/// `ReadSyscom(n)`: the setting associated with a system command.
pub fn read_syscom(machine: &Machine, index: i32) -> i32 {
    let s = &machine.sys.settings;
    match index {
        syscom::MESSAGE_SPEED => s.message_speed,
        syscom::DISPLAY_MODE => s.screen_mode,
        syscom::VOICE_SETTINGS => s.koe_mode,
        syscom::BGM_FADE => i32::from(s.bgm_koe_fade),
        syscom::WINDOW_DECORATION_STYLE => s.waku_all,
        syscom::SHOW_WEATHER => i32::from(s.show_weather),
        syscom::SHOW_OBJECT_1 => i32::from(s.show_object[0]),
        syscom::SHOW_OBJECT_2 => i32::from(s.show_object[1]),
        syscom::CLASSIFY_TEXT => i32::from(s.classify_text),
        syscom::GENERIC_1 => s.generic[0],
        syscom::GENERIC_2 => s.generic[1],
        syscom::SET_SKIP_MODE => i32::from(machine.sys.syscom.skip_mode),
        syscom::AUTO_MODE => i32::from(s.auto_mode),
        _ => 0,
    }
}

/// Applies a value to the setting behind a system command.
pub fn write_syscom(machine: &mut Machine, index: i32, value: i32) {
    let s = &mut machine.sys.settings;
    match index {
        syscom::MESSAGE_SPEED => s.message_speed = value,
        syscom::DISPLAY_MODE => s.screen_mode = value,
        syscom::VOICE_SETTINGS => s.koe_mode = value,
        syscom::BGM_FADE => s.bgm_koe_fade = value != 0,
        syscom::WINDOW_DECORATION_STYLE => s.waku_all = value,
        syscom::SHOW_WEATHER => s.show_weather = value != 0,
        syscom::SHOW_OBJECT_1 => s.show_object[0] = value != 0,
        syscom::SHOW_OBJECT_2 => s.show_object[1] = value != 0,
        syscom::CLASSIFY_TEXT => s.classify_text = value != 0,
        syscom::GENERIC_1 => s.generic[0] = value,
        syscom::GENERIC_2 => s.generic[1] = value,
        syscom::SET_SKIP_MODE => machine.sys.syscom.skip_mode = value != 0,
        syscom::AUTO_MODE => s.auto_mode = value != 0,
        _ => {}
    }
}

/// The 22xx/23xx/26xx setting families: `Set*`, getter, `Def*`.
fn settings_op(machine: &mut Machine, command: &Command) -> Result<Next> {
    let opcode = command.op.opcode;
    let (kind, which) = match opcode {
        2221..=2299 => (0, opcode - 2200),
        2321..=2399 => (1, opcode - 2300),
        _ => (2, opcode),
    };
    // Defaults use their own numbering.
    let which = if kind == 2 {
        match opcode {
            2600 => 23,
            2601 => 24,
            2602 => 26,
            2603 => 27,
            2604 => 50,
            2605 => 51,
            2606 => 52,
            2610..=2614 => 60 + (opcode - 2610),
            2615..=2617 => 65 + (opcode - 2615),
            2620 => 21,
            2621 => 22,
            _ => return machine.unimplemented(command),
        }
    } else {
        which
    };
    let value = if kind == 0 {
        match which {
            // Window attribute tuples take several values.
            65..=67 => 0,
            74 => machine.int_param(command, 1)?,
            _ => machine.int_param(command, 0)?,
        }
    } else {
        0
    };
    let defaults = machine.sys.defaults.clone();
    let source = if kind == 2 {
        &defaults
    } else {
        &machine.sys.settings
    };
    let current: Option<i32> = match which {
        21 => Some(source.generic[0]),
        22 => Some(source.generic[1]),
        23 => Some(source.message_speed),
        24 => Some(i32::from(source.message_no_wait)),
        25 => Some(source.koe_mode),
        26 => Some(source.bgm_koe_fade_vol),
        27 => Some(i32::from(source.bgm_koe_fade)),
        30..=33 => Some(source.volume[usize::from(which - 30)]),
        40..=43 => Some(i32::from(source.enabled[usize::from(which - 40)])),
        50 => Some(i32::from(source.auto_mode)),
        51 => Some(source.auto_char_time),
        52 => Some(source.auto_base_time),
        55 => Some(source.font_quality),
        56 => Some(source.font_weight),
        57 => Some(source.font_shadow),
        60..=64 => Some(window_attr_values(source.window_attr)[usize::from(which - 60)]),
        70 => Some(i32::from(source.show_object[0])),
        71 => Some(i32::from(source.show_object[1])),
        72 => Some(i32::from(source.show_weather)),
        73 => Some(i32::from(source.classify_text)),
        75 => Some(source.screen_mode),
        _ => None,
    };
    match kind {
        // Getters and defaults.
        1 | 2 => match which {
            65..=67 => {
                let values = window_attr_values(source.window_attr);
                let count = [3, 4, 5][usize::from(which - 65)];
                write_targets(machine, command, 0, &values[..count])?;
            }
            74 => {
                let character = machine.int_param(command, 0)?;
                machine.store = i32::from(
                    *machine
                        .sys
                        .settings
                        .use_koe
                        .get(&character)
                        .unwrap_or(&true),
                );
            }
            _ => match current {
                Some(value) => machine.store = value,
                None => {
                    return match crate::modules::sys_max::dispatch(machine, command)? {
                        Some(next) => Ok(next),
                        None => machine.unimplemented(command),
                    };
                }
            },
        },
        // Setters.
        _ => {
            let s = &mut machine.sys.settings;
            match which {
                21 => s.generic[0] = value,
                22 => s.generic[1] = value,
                23 => s.message_speed = value,
                24 => s.message_no_wait = value != 0,
                25 => s.koe_mode = value,
                26 => s.bgm_koe_fade_vol = value,
                27 => s.bgm_koe_fade = value != 0,
                30..=33 => s.volume[usize::from(which - 30)] = value.clamp(0, 255),
                40..=43 => s.enabled[usize::from(which - 40)] = value != 0,
                50 => s.auto_mode = value != 0,
                51 => s.auto_char_time = value,
                52 => s.auto_base_time = value,
                55 => s.font_quality = value,
                56 => s.font_weight = value,
                57 => s.font_shadow = value,
                60..=64 => set_attr_component(&mut s.window_attr, usize::from(which - 60), value),
                65..=67 => {
                    let count = [3, 4, 5][usize::from(which - 65)];
                    let values = machine.int_params_from(command, 0)?;
                    let attr = &mut machine.sys.settings.window_attr;
                    for (component, value) in values.iter().take(count).enumerate() {
                        set_attr_component(attr, component, *value);
                    }
                }
                70 => s.show_object[0] = value != 0,
                71 => s.show_object[1] = value != 0,
                72 => s.show_weather = value != 0,
                73 => s.classify_text = value != 0,
                74 => {
                    let character = machine.int_param(command, 0)?;
                    machine.sys.settings.use_koe.insert(character, value != 0);
                }
                75 => s.screen_mode = value,
                _ => {
                    return match crate::modules::sys_max::dispatch(machine, command)? {
                        Some(next) => Ok(next),
                        None => machine.unimplemented(command),
                    };
                }
            }
            if (30..=33).contains(&which) || (40..=43).contains(&which) {
                machine.sys.volumes_changed = true;
            }
        }
    }
    Ok(Next::Advance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolation_modes() {
        assert_eq!(interpolate(0, 5, 10, 100, 0), 50);
        assert!(interpolate(0, 5, 10, 100, 1) < 50);
        assert!(interpolate(0, 5, 10, 100, 2) > 50);
        assert_eq!(trig(90, 0, f64::sin), 32640);
        assert_eq!(trig(90, 32640, f64::sin), 1);
    }
}
