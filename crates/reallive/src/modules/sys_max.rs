//! The system functions RealLiveMax added to module 1:4 (names from its
//! SDK, via RLdev's function list): more waits, mouse and key queries,
//! read-flag and bit helpers, save-slot metadata, CG-table queries by
//! name/list/flag/group, quick save/load, and many switches.
//!
//! Switches that only change the original's own UI (hint icons, window
//! menus, Alt-key menu, MMX use, ...) are remembered so scripts read back
//! what they set.

use anyhow::Result;

use crate::bytecode::Command;
use crate::longop::{Wait, WaitEvent};
use crate::machine::{IntTarget, Machine, Next, StrTarget};
use crate::memory::{BANK_Z, IntRef};

/// Remembered switches: `(setter, getter, default)`; an `ON`/`OFF` pair
/// is written as setters `n` (1) and `n + 1` (0).
const REMEMBERED: &[(u16, u16, i32)] = &[
    // SET/GET_OWNER_AUTOMODE_MOJIWAIT, _MINWAIT
    (191, 196, 0),
    (192, 197, 0),
    // SET/GET_SELCOM_* modes
    (240, 241, 0),
    (242, 243, 0),
    (244, 245, 0),
    (246, 247, 0),
    (248, 249, 0),
    (229, 228, 0),
    // GET/SET_MOUSEMOVE_USE, _MOUSECURSOR_DISP
    (251, 250, 1),
    (253, 252, 1),
    // SET/GET_RETURNWAIT
    (341, 340, 0),
    // SET_RETURNCURSOR_DISP / GET_RETURNCURSOR_DISP
    (367, 360, 1),
    // SET/GET_HINT_AUTOMODE_DISP, _READJUMP_DISP
    (375, 374, 1),
    (377, 376, 1),
    // SET/GET_MMX_USE
    (380, 381, 0),
    // system flags (22xx setters, 23xx getters) with no counterpart here
    (2239, 2339, 255),
    (2280, 2380, 0),
    (2281, 2381, 0),
    (2282, 2382, 0),
    (2283, 2383, 0),
    (2284, 2384, 100),
];

/// `ON`/`OFF` setter pairs remembered under the first opcode.
const SWITCHES: &[(u16, u16)] = &[
    (170, 171),   // SET_ALTKEYMENU_ON / OFF
    (220, 221),   // SET_SELCOM_FLUSH_ON / OFF
    (222, 223),   // SET_SELCOM_WAIT_ON / OFF
    (224, 225),   // SET_SELCOM_MOUSESET_ON / OFF
    (226, 227),   // SET_SELCOM_WINDOWCLOSE_ON / OFF
    (230, 231),   // SET_SELCOM_WINDOWCLEAR_ON / OFF
    (343, 344),   // SET_RETURNWAITMOD_ON / OFF
    (361, 362),   // SET_RETURNCURSOR_ON / OFF
    (370, 371),   // SET_HINT_AUTOMODE_ON / OFF
    (372, 373),   // SET_HINT_READJUMP_ON / OFF
    (1250, 1251), // SET_WINDOWMENU_FILE_ON / OFF
    (1252, 1253), // SET_WINDOWMENU_SYSTEM_ON / OFF
    (1602, 1603), // CURSORAREA_ON / OFF
    (2400, 2500), // ENABLE / DISENABLE_SYSTEMMENU_OBJECT1
    (2401, 2501), // ... _OBJECT2
    (2403, 2503), // ... _WAIPKEYJUMP
    (2404, 2504), // ... _SYSBTNHIDE
];

fn remembered(machine: &Machine, key: u16, default: i32) -> i32 {
    machine.sys.remembered.get(&key).copied().unwrap_or(default)
}

fn write_ints(machine: &mut Machine, command: &Command, from: usize, values: &[i32]) -> Result<()> {
    for (offset, &value) in values.iter().enumerate() {
        if from + offset >= command.params.len() {
            break;
        }
        let target = machine.int_target_param(command, from + offset)?;
        machine.set_target(target, value)?;
    }
    Ok(())
}

fn write_str(machine: &mut Machine, command: &Command, index: usize, value: String) -> Result<()> {
    if index < command.params.len() {
        let target = machine.str_target_param(command, index)?;
        machine.write_string(target, value)?;
    }
    Ok(())
}

fn bank_z(flag: i32) -> IntRef {
    IntRef {
        bank: BANK_Z,
        width: 0,
        index: flag,
    }
}

/// Handles `command` if it is one of these functions.
pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Option<Next>> {
    let opcode = command.op.opcode;
    if let Some(&(setter, getter, default)) = REMEMBERED
        .iter()
        .find(|&&(setter, getter, _)| opcode == setter || opcode == getter)
    {
        if opcode == setter {
            let value = machine.int_param_or(command, 0, default)?;
            machine.sys.remembered.insert(setter, value);
        } else {
            machine.store = remembered(machine, setter, default);
        }
        return Ok(Some(Next::Advance));
    }
    if let Some(&(on, _)) = SWITCHES
        .iter()
        .find(|&&(on, off)| opcode == on || opcode == off)
    {
        machine.sys.remembered.insert(on, i32::from(opcode == on));
        return Ok(Some(Next::Advance));
    }
    let now = machine.sys.now();
    match opcode {
        // SLEEP: give up the rest of the frame.
        1 => machine.yield_frame = true,
        // TIMEWAITKEY_NOMOUSECLEAR / DONTJUMP_TIMEWAIT(KEY)(_NOMOUSECLEAR):
        // timed waits a click ends (the KEY forms) that skip mode does not
        // shorten (the DONTJUMP forms).
        102..=105 => {
            let time = machine.int_param(command, 0)?;
            let mut wait = Wait::for_ms(machine, time);
            if opcode != 103 {
                wait = wait.cancellable();
            }
            if opcode >= 103 {
                wait = wait.unskippable();
            }
            machine.push_long_op(Box::new(wait));
        }
        // KEYWAIT_NOMOUSECLEAR
        107 => {
            let wait = Wait::event(WaitEvent::None).cancellable();
            machine.push_long_op(Box::new(wait));
        }
        // DBLCLKCLEAR / DBLCLKCHECK(_LEFT/_RIGHT)(x, y, clicked)
        134 => machine.sys.input.double_clicks = [None, None],
        135..=137 => {
            let clicks = std::mem::take(&mut machine.sys.input.double_clicks);
            let value = |click: Option<(i32, i32)>| match click {
                Some((x, y)) => [x, y, 1],
                None => [0, 0, 0],
            };
            let values: Vec<i32> = match opcode {
                135 => value(clicks[0])
                    .into_iter()
                    .chain(value(clicks[1]))
                    .collect(),
                136 => value(clicks[0]).to_vec(),
                _ => value(clicks[1]).to_vec(),
            };
            machine.store = i32::from(values.iter().skip(2).step_by(3).any(|&c| c != 0));
            write_ints(machine, command, 0, &values)?;
        }
        // MOUSEWHEELCLEAR / MOUSEWHEELCHECK (-1 up, 1 down) / ..CNT
        139 => machine.sys.input.wheel = 0,
        140 => machine.store = machine.sys.input.wheel.signum(),
        141 => machine.store = machine.sys.input.wheel,
        // SET_MOUSEAREA / CLR_MOUSEAREA: the pointer is confined there.
        160 => {
            let v = machine.int_params_from(command, 0)?;
            if let [x1, y1, x2, y2, ..] = v[..] {
                machine.sys.mouse_area = Some((x1, y1, x2, y2));
            }
        }
        161 => machine.sys.mouse_area = None,
        // AUTOMODEWAIT...: in auto mode, the pause auto mode gives the page
        // just shown (clicks end it for the KEY forms); otherwise nothing.
        180..=185 => {
            if machine.sys.settings.auto_mode {
                let chars = machine.sys.text.active_state().chars_since_pause;
                let s = &machine.sys.settings;
                let time = s.auto_base_time + s.auto_char_time * chars;
                let mut wait = Wait::for_ms(machine, time);
                if matches!(opcode, 181 | 182 | 184 | 185) {
                    wait = wait.cancellable();
                }
                if opcode >= 183 {
                    wait = wait.unskippable();
                }
                machine.push_long_op(Box::new(wait));
            } else {
                machine.store = 0;
            }
        }
        // SET/GET_WAIP_WINDOWCLOSE_MOD, SET/GET_GRPCOM_WINDOWCLOSE_MOD
        212 => machine.sys.wipe_closes_windows = machine.int_param(command, 0)? != 0,
        213 => machine.store = i32::from(machine.sys.wipe_closes_windows),
        217 => machine.sys.grp_closes_windows = machine.int_param(command, 0)? != 0,
        218 => machine.store = i32::from(machine.sys.grp_closes_windows),
        // ALLSET_READFLAG / ALLCLR_READFLAG: every line read / unread.
        310 => machine.memory.global.all_read = true,
        311 => {
            machine.memory.global.kidoku.clear();
            machine.memory.global.all_read = false;
        }
        // GET_JUMPSTATE / SET_JUMP_ENABLE / SET_JUMP_DISENABLE (skip mode)
        355 => machine.store = i32::from(machine.sys.syscom.skip_mode),
        356 => machine.sys.syscom.skip_mode_allowed = true,
        357 => machine.sys.syscom.skip_mode_allowed = false,
        // GET_R_CURSORNO / GET_P_CURSORNO
        363 | 365 => machine.store = machine.sys.key_cursor,
        // SET_WINDOW_WAKUMOD(window, mod) / SET_WINDOW_ATTRMOD(window, mod)
        417 | 428 => {
            let window = machine.int_param(command, 0)?;
            let value = machine.int_param(command, 1)?;
            if let Some(config) = machine.sys.text.window_mut(window) {
                if opcode == 417 {
                    config.waku_mod = value;
                } else {
                    config.attr_mod = value;
                }
            }
        }
        // GET/SET_WINDOW_EXBTN_MOD(button[, mod]): extra window buttons.
        458 => {
            let button = machine.int_param(command, 0)?;
            machine.store = remembered(machine, EXBTN_MOD + button.clamp(0, 9) as u16, 1);
        }
        459 => {
            let (button, value) = if command.params.len() >= 2 {
                (
                    machine.int_param(command, 0)?,
                    machine.int_param(command, 1)?,
                )
            } else {
                (-1, machine.int_param(command, 0)?)
            };
            let buttons = if button < 0 {
                0..10
            } else {
                button.min(9)..button.min(9) + 1
            };
            for b in buttons {
                machine.sys.remembered.insert(EXBTN_MOD + b as u16, value);
            }
        }
        // INIT_WINDOW_OPEN_ANIME / _CLOSE_ANIME([window]): back to the
        // Gameexe animation; GET/SET_WINDOW_ANIME_MOD(window[, mod]).
        470 | 471 => {
            let windows: Vec<i32> = match command.params.first() {
                Some(_) => vec![machine.int_param(command, 0)?],
                None => (0..machine.sys.text.windows.len() as i32).collect(),
            };
            for window in windows {
                let fresh = crate::text::WindowConfig::from_gameexe(
                    &machine.gameexe,
                    window.max(0) as usize,
                );
                if let Some(config) = machine.sys.text.window_mut(window) {
                    if opcode == 470 {
                        config.open_anm_mod = fresh.open_anm_mod;
                        config.open_anm_time = fresh.open_anm_time;
                    } else {
                        config.close_anm_mod = fresh.close_anm_mod;
                        config.close_anm_time = fresh.close_anm_time;
                    }
                }
            }
        }
        472 => {
            let window = machine.int_param(command, 0)?;
            machine.store = machine
                .sys
                .text
                .window(window)
                .map_or(0, |c| i32::from(c.anm_enabled));
        }
        473 => {
            let window = machine.int_param(command, 0)?;
            let value = machine.int_param(command, 1)?;
            if let Some(config) = machine.sys.text.window_mut(window) {
                config.anm_enabled = value != 0;
            }
        }
        // READJUMP_START_CHECK: skip mode is on.
        746 => machine.store = i32::from(machine.sys.syscom.skip_mode),
        // MESSAGE_ERASE(_CHECK): hide the windows until a click, as the
        // right button does.
        750 => crate::modules::msg::show_background(machine)?,
        751 => machine.store = i32::from(machine.sys.text.hidden_temporarily),
        // MSGBK_START(_CHECK): open the backlog.
        755 => crate::backlog::open(&mut machine.sys),
        756 => machine.store = i32::from(machine.sys.text.backlog_view.is_some()),
        // GET/SET_LAST_KOE_COMMAND(koe, ctrl), KOE_REPLAY(_CHECK),
        // GET_KOE_REPLAY(koe, ctrl): the last voice, for replaying.
        780 | 797 => {
            let (koe, ctrl) = machine.sys.last_koe;
            write_ints(machine, command, 0, &[koe, ctrl])?;
            machine.store = i32::from(koe >= 0);
        }
        781 => {
            let koe = machine.int_param(command, 0)?;
            let ctrl = machine.int_param_or(command, 1, 0)?;
            machine.sys.last_koe = (koe, ctrl);
        }
        795 => {
            let (koe, _) = machine.sys.last_koe;
            if koe >= 0 {
                crate::modules::sound::koe_play(machine, koe, None);
            }
        }
        796 => machine.store = i32::from(machine.sys.sound.koe_playing(now)),
        // TIMETABLELEN: index_series with lengths.
        801 => machine.store = crate::modules::sys::index_series(machine, command, true)?,
        // LOG2 / BITSET / BITCLR / BITCHECK(var, bit)
        1014 => {
            let n = machine.int_param(command, 0)?;
            machine.store = if n > 0 {
                31 - n.leading_zeros() as i32
            } else {
                0
            };
        }
        1015..=1017 => {
            let target = machine.int_target_param(command, 0)?;
            let bit = machine.int_param(command, 1)?.clamp(0, 31);
            let value = machine.get_target(target)?;
            let mask = 1i32 << bit;
            match opcode {
                1015 => machine.set_target(target, value | mask)?,
                1016 => machine.set_target(target, value & !mask)?,
                _ => {}
            }
            machine.store = i32::from(value & mask != 0);
        }
        // GET_LINENO
        1121 => machine.store = machine.line,
        // GET_RETURNWAIT (the second number for it)
        342 => machine.store = remembered(machine, 341, 0),
        // H/I/J_FLAG_ALLOC(count) / _FREE: RealLiveMax's extra variable
        // banks. Their sizes are kept; this interpreter's bytecode reader
        // has no bank codes for them, so no script here can address them.
        3600 | 3610 | 3620 => {
            let count = machine.int_param(command, 0)?;
            machine.sys.remembered.insert(opcode, count);
        }
        3601 | 3611 | 3621 => {
            machine.sys.remembered.remove(&(opcode - 1));
        }
        // SYSTEMMENUSUB: the system command menu.
        1217 => machine.sys.ui.request(crate::ui::Request::SyscomMenu),
        // GET_SELPOINTMOD / SELPOINT_CHECK
        1220 => machine.store = i32::from(machine.selpoint_auto),
        1232 => machine.store = i32::from(machine.previous_selection.is_some()),
        // NAMEGET_DEFAULT / NAMESET_DEFAULT(index) and local forms: the
        // names #NAME / #LOCALNAME give.
        1304 | 1305 | 1314 | 1315 => {
            let local = opcode >= 1314;
            let index = machine.int_param(command, 0)?.max(0) as usize;
            let key = format!("{}.{index:03}", if local { "LOCALNAME" } else { "NAME" });
            let default = machine.gameexe.str(&key).unwrap_or("").to_owned();
            if opcode % 10 == 4 {
                write_str(machine, command, 1, default)?;
            } else {
                machine.memory.set_name(local, index, default);
            }
        }
        // GET_NAME_MAXLEN
        1320 => machine.store = machine.gameexe.int("NAME_MAXLEN").unwrap_or(10),
        // GET_SAVE_YEAR..GET_SAVE_MILLISECOND(slot), GET_SAVE_TITLE
        1400..=1408 => {
            let slot = machine.int_param(command, 0)?;
            let header = crate::save::any_slot_header(machine, slot);
            match (header, opcode) {
                (Some(header), 1408) => {
                    write_str(machine, command, 1, header.title)?;
                    machine.store = 1;
                }
                (Some(header), _) => machine.store = header.date[usize::from(opcode - 1400)],
                (None, _) => machine.store = 0,
            }
        }
        // GET_SAVE_MAX
        1420 => machine.store = crate::ui::slot_count(machine),
        // GET_SAVE_ORDER(dest, count) (+ _WITH_QUICKDATA): slots, newest
        // first; the store is how many there are.
        1422 | 1426 => {
            let count = machine.int_param(command, 1)?.max(0) as usize;
            let mut slots: Vec<(Vec<i32>, i32)> = (0..crate::ui::slot_count(machine)
                + i32::from(opcode == 1426))
                .filter_map(|slot| {
                    let header = crate::save::any_slot_header(machine, slot)?;
                    Some((header.date.to_vec(), slot))
                })
                .collect();
            slots.sort_by(|a, b| b.0.cmp(&a.0));
            let order: Vec<i32> = slots.iter().map(|(_, slot)| *slot).take(count).collect();
            machine.store = order.len() as i32;
            if let IntTarget::Mem(first) = machine.int_target_param(command, 0)? {
                for (i, slot) in order.into_iter().enumerate() {
                    let target = IntTarget::Mem(IntRef {
                        index: first.index + i as i32,
                        ..first
                    });
                    machine.set_target(target, slot)?;
                }
            }
        }
        // GET_SAVE_NODATA_STR(str)
        1423 => {
            let text = machine.gameexe.str("SAVE_NODATA").unwrap_or("").to_owned();
            write_str(machine, command, 0, text)?;
        }
        // GET_SAVE_QUICKDATA_NO / GET_SAVE_NEW_WITH_QUICKDATA
        1424 => machine.store = crate::ui::quick_slot(machine),
        1425 => machine.store = machine.latest_save,
        // GET/SET/CLR_SAVE_COMMENT(1..3)(slot[, str])
        1450..=1458 => {
            let slot = machine.int_param(command, 0)?;
            let which = usize::from((opcode - 1450) / 3);
            let mut extra = crate::save::slot_extra(machine, slot);
            match (opcode - 1450) % 3 {
                0 => {
                    let comment = extra.as_ref().map(|e| e.comments[which].clone());
                    machine.store = i32::from(comment.as_ref().is_some_and(|c| !c.is_empty()));
                    write_str(machine, command, 1, comment.unwrap_or_default())?;
                }
                action => {
                    if let Some(extra) = &mut extra {
                        extra.comments[which] = if action == 1 {
                            machine.str_param(command, 1)?
                        } else {
                            String::new()
                        };
                        crate::save::set_slot_extra(machine, slot, extra)?;
                    }
                }
            }
        }
        // GET_SAVE_MESSAGE(slot, str) / GET_SAVE_MESSAGE_NOW(str)
        1459 => {
            let slot = machine.int_param(command, 0)?;
            let message = crate::save::slot_extra(machine, slot).map(|e| e.message);
            machine.store = i32::from(message.is_some());
            write_str(machine, command, 1, message.unwrap_or_default())?;
        }
        1462 => {
            let message = crate::save::current_message(machine);
            machine.store = 1;
            write_str(machine, command, 0, message)?;
        }
        // GET_SAVE_VALUE(slot, dest, count) / CLR_SAVE_VALUE(slot, count):
        // values kept with a save (the ones SET_SAVE_VALUE would store).
        1470 | 1471 => {
            let slot = machine.int_param(command, 0)?;
            let count = machine.int_param(command, 2)?.max(0) as usize;
            let values = crate::save::slot_extra(machine, slot)
                .map(|e| e.values)
                .unwrap_or_default();
            machine.store = i32::from(!values.is_empty());
            if let IntTarget::Mem(first) = machine.int_target_param(command, 1)? {
                for i in 0..count {
                    let target = IntTarget::Mem(IntRef {
                        index: first.index + i as i32,
                        ..first
                    });
                    machine.set_target(target, values.get(i).copied().unwrap_or(0))?;
                }
            }
        }
        1472 => {
            let slot = machine.int_param(command, 0)?;
            if let Some(mut extra) = crate::save::slot_extra(machine, slot) {
                extra.values.clear();
                crate::save::set_slot_extra(machine, slot, &extra)?;
            }
        }
        // CG table
        1505..=1510 | 1800..=1853 => cg_table(machine, command)?,
        // CURSORAREA_*: keyboard navigation areas of the original's UI.
        1600 | 1601 => {}
        1610 => {
            let (x, y) = machine.sys.input.mouse;
            write_ints(machine, command, 0, &[x, y])?;
            machine.store = -1;
        }
        1620 => machine.store = -1,
        // EDITBOX_*_GET: key events of text boxes.
        1712..=1715 => machine.store = 0,
        // SET_SYSTEM_FLAG_UNREADJUMP / _SYSBTNHIDE
        2060 | 2061 => {
            let value = machine.int_param(command, 0)?;
            machine.sys.remembered.insert(opcode, value);
        }
        // GET_SYSTEM_FLAG_WAKUMOD
        2376 => machine.store = machine.sys.settings.waku_all,
        // GET_SYSTEM_FLAG_DEFAULT_*
        2622..=2671 => machine.store = system_default(machine, opcode),
        // CHECK_SYSTEM_FLAG_DISPLAYMOD(mode): every mode is possible.
        2700 => machine.store = 1,
        // DATASAVE_SAVEPOINT(_SEPLAY)(slot[, message]), save
        3004 | 3005 | 3104 | 3105 => {
            let slot = machine.int_param(command, 0)?;
            crate::save::save_slot(machine, slot)?;
            machine.store = 1;
        }
        // DATALOAD(_SEPLAY)_SELKEEP(_NOWARNING)(slot)
        3010 | 3011 | 3110 | 3111 => {
            let slot = machine.int_param(command, 0)?;
            return load_keeping_selection(machine, slot).map(Some);
        }
        // QUICKSAVE...: the quick slot
        3024..=3027 | 3124..=3126 => {
            let slot = crate::ui::quick_slot(machine);
            crate::save::save_slot(machine, slot)?;
            machine.store = 1;
        }
        // QUICKLOAD...
        3028 | 3029 | 3128 | 3129 => {
            let slot = crate::ui::quick_slot(machine);
            if crate::save::slot_exists(machine, slot) {
                crate::save::load_slot(machine, slot)?;
                return Ok(Some(Next::Jumped));
            }
            machine.store = 0;
        }
        3030 | 3031 | 3130 => {
            let slot = crate::ui::quick_slot(machine);
            return load_keeping_selection(machine, slot).map(Some);
        }
        // MAKE_THUMBNAIL / CLEAR_THUMBNAIL / MAKE_THUMBNAIL_FILE: the save
        // thumbnail is the screen captured now.
        3200 | 3202 => {
            let frame = crate::screen::compose_layers(&mut machine.sys, false);
            machine.sys.gfx.thumbnail = Some(std::rc::Rc::new(frame));
        }
        3201 => machine.sys.gfx.thumbnail = None,
        // DELETE_SAVEDATA(slot), COPY_SAVEDATA / SWAP_SAVEDATA(from, to)
        3220 => {
            let slot = machine.int_param(command, 0)?;
            crate::save::delete_slot(machine, slot)?;
        }
        3221 | 3222 => {
            let from = machine.int_param(command, 0)?;
            let to = machine.int_param(command, 1)?;
            crate::save::copy_slot(machine, from, to, opcode == 3222)?;
        }
        _ => return Ok(None),
    }
    Ok(Some(Next::Advance))
}

fn load_keeping_selection(machine: &mut Machine, slot: i32) -> Result<Next> {
    if !crate::save::slot_exists(machine, slot) {
        machine.store = 0;
        return Ok(Next::Advance);
    }
    let selection = machine.previous_selection.clone();
    crate::save::load_slot(machine, slot)?;
    machine.previous_selection = selection;
    Ok(Next::Jumped)
}

/// `GET_SYSTEM_FLAG_DEFAULT_*` (2622..2671).
fn system_default(machine: &Machine, opcode: u16) -> i32 {
    let d = &machine.sys.defaults;
    match opcode {
        2622 => i32::from(d.show_object[0]),
        2623 => i32::from(d.show_object[1]),
        2624 => i32::from(d.show_weather),
        2625 => i32::from(d.classify_text),
        2626 => d.screen_mode,
        2627 => d.waku_all,
        2629 => d.koe_mode,
        2630..=2633 => d.volume[usize::from(opcode - 2630)],
        2634..=2637 => i32::from(d.enabled[usize::from(opcode - 2634)]),
        2650 => d.font_quality,
        2651 => d.font_weight,
        2652 => d.font_shadow,
        2661 => i32::from(!d.skip_animations),
        2663 => i32::from(d.confirm_save_load),
        2666 => d.sound_quality,
        // character voices on, full volume, normal speed, the rest off
        2628 | 2660 | 2664 | 2665 => 1,
        2639 => 255,
        2643 => 100,
        _ => 0,
    }
}

/// The CG-table functions by file name (1505..1510, 1800..1806), list
/// number (1810..1816), flag number (1820..1826) and group (1830..1853).
fn cg_table(machine: &mut Machine, command: &Command) -> Result<()> {
    use crate::cgtable::CgEntry;
    let opcode = command.op.opcode;
    let entries: Vec<CgEntry> = machine.sys.cg_table.entries();
    let flag_of = |machine: &Machine, flag: i32| machine.read_int(bank_z(flag)).unwrap_or(0);
    // The entries a function addresses, and where its other arguments
    // start.
    let (selected, rest): (Vec<CgEntry>, usize) = match opcode {
        1505 | 1506 | 1800..=1806 => {
            let name = machine.str_param(command, 0)?;
            let flag = machine.sys.cg_table.flag(&name);
            (
                entries
                    .iter()
                    .filter(|e| Some(e.flag) == flag)
                    .cloned()
                    .collect(),
                1,
            )
        }
        1507 | 1508 => (entries.clone(), 0),
        1810..=1816 => {
            let list = machine.int_param(command, 0)?;
            (
                entries.iter().filter(|e| e.list == list).cloned().collect(),
                1,
            )
        }
        1820..=1826 => {
            let flag = machine.int_param(command, 0)?;
            (
                entries.iter().filter(|e| e.flag == flag).cloned().collect(),
                1,
            )
        }
        1830..=1853 => {
            let groups = (0..5)
                .map(|i| machine.int_param(command, i))
                .collect::<Result<Vec<_>>>()?;
            // A negative group number matches anything below it.
            let matching = entries
                .iter()
                .filter(|e| {
                    groups
                        .iter()
                        .zip(&e.group)
                        .all(|(&want, &have)| want < 0 || want == have)
                })
                .cloned()
                .collect();
            (matching, 5)
        }
        // GET_CGTABLE(count, names, flags) / GET_CGTABLE_FILENAME(flag, name)
        1509 => {
            let count = machine.int_param(command, 0)?.max(0) as usize;
            let names = machine.str_target_param(command, 1)?;
            let flags = machine.int_target_param(command, 2)?;
            for (i, entry) in entries.iter().take(count).enumerate() {
                let value = flag_of(machine, entry.flag);
                machine.write_string(
                    StrTarget {
                        index: names.index + i as i32,
                        ..names
                    },
                    entry.name.clone(),
                )?;
                if let IntTarget::Mem(first) = flags {
                    machine.set_target(
                        IntTarget::Mem(IntRef {
                            index: first.index + i as i32,
                            ..first
                        }),
                        value,
                    )?;
                }
            }
            machine.store = entries.len().min(count) as i32;
            return Ok(());
        }
        _ => {
            let flag = machine.int_param(command, 0)?;
            let name = entries
                .iter()
                .find(|e| e.flag == flag)
                .map(|e| e.name.clone());
            machine.store = i32::from(name.is_some());
            write_str(machine, command, 1, name.unwrap_or_default())?;
            return Ok(());
        }
    };
    let first = selected.first().cloned();
    machine.store = i32::from(first.is_some());
    let kind = opcode % 10;
    match opcode {
        // set / clear flags
        1505 | 1507 | 1800 | 1810 | 1820 | 1830 | 1852 => {
            for entry in &selected {
                machine.write_int(bank_z(entry.flag), 1)?;
            }
        }
        1506 | 1508 | 1801 | 1811 | 1821 | 1831 | 1853 => {
            for entry in &selected {
                machine.write_int(bank_z(entry.flag), 0)?;
            }
        }
        // group counts: entries / entries seen
        1850 => machine.store = selected.len() as i32,
        1851 => {
            machine.store = selected
                .iter()
                .filter(|e| flag_of(machine, e.flag) != 0)
                .count() as i32
        }
        _ => {
            let Some(entry) = first else { return Ok(()) };
            match kind {
                // flag value / flag number
                2 => write_ints(machine, command, rest, &[flag_of(machine, entry.flag)])?,
                4 => write_ints(machine, command, rest, &[entry.flag])?,
                // name
                3 => write_str(machine, command, rest, entry.name.clone())?,
                // one code / all codes
                5 => {
                    let index = machine.int_param(command, rest)?.clamp(0, 4) as usize;
                    write_ints(machine, command, rest + 1, &[entry.code[index]])?;
                }
                _ => write_ints(machine, command, rest, &entry.code)?,
            }
        }
    }
    Ok(())
}

/// `SET_WINDOW_EXBTN_MOD` values, by button, in `remembered`.
pub const EXBTN_MOD: u16 = 4580;
/// `SET_MOUSEACTIONCALL_ON/OFF` (and the `CCOM_` forms), by id.
const MOUSEACTIONCALL: u16 = 21200;
/// `CCOM_QUAKE_ON/OFF` by buffer, `CCOM_FULLQUAKE`, `CCOM_SYSBTN`,
/// `CCOM_HINT`, `CCOM_SELBTN`, `LOADEVENT_G_FLAG_SET`.
const CCOM_QUAKE: u16 = 12060;
const CCOM_SWITCHES: u16 = 12080;
const LOADEVENT: u16 = 13000;

/// The ids an `(id)` / `()` pair of forms names: one, or all of `count`.
fn ids(machine: &mut Machine, command: &Command, count: i32) -> Result<std::ops::Range<i32>> {
    Ok(match command.params.first() {
        Some(_) => {
            let id = machine.int_param(command, 0)?;
            id..id + 1
        }
        None => 0..count,
    })
}

/// RealLiveMax additions to module 0:4 (interrupt and call controls,
/// system-menu (`CCOM_*`) switches, window extra buttons).
pub fn event_loop(machine: &mut Machine, command: &Command) -> Result<Next> {
    let opcode = command.op.opcode;
    match opcode {
        // INTERRUPT_CHANGE(scenario, entrypoint)
        122 => {
            let scene = machine.int_param(command, 0)?;
            let entrypoint = machine.int_param_or(command, 1, 0)?;
            machine.interrupt = Some((scene, entrypoint));
        }
        // INTERRUPT_GET_STATE(scenario, entrypoint, set)
        123 => {
            let (scene, entry) = machine.interrupt.unwrap_or((-1, -1));
            let set = i32::from(machine.interrupt.is_some());
            write_ints(machine, command, 0, &[scene, entry, set])?;
        }
        // INTERRUPTCOM_GET_PROCESS / _DISAPPEAR / _APPEAR
        130 => machine.store = i32::from(machine.in_interrupt),
        131 => machine.interrupt_suspended = true,
        132 => machine.interrupt_suspended = false,
        // PAGE: wait, then start a new page.
        163 => {
            return Ok(crate::modules::msg::pause(
                machine,
                crate::textout::PauseKind::Page,
            ));
        }
        // CANCELCALL: the right-click routine, as if right-clicked.
        200 => {
            if crate::modules::menu::invoke_cancel_call(machine)? {
                return Ok(Next::Jumped);
            }
            crate::modules::menu::open_context_menu(machine)?;
        }
        201 => machine.store = i32::from(crate::modules::menu::cancel_call(machine).is_some()),
        // GET_MOUSEACTIONCALL_AREA(id, x1, y1, x2, y2): `#MOUSEACTIONCALL`.
        210 => {
            let id = machine.int_param(command, 0)?;
            let mut area = machine.gameexe.ints(&format!("MOUSEACTIONCALL.{id:03}"));
            area.resize(4, 0);
            write_ints(machine, command, 1, &area[..4])?;
        }
        // SET_MOUSEACTIONCALL(id) / _ON / _OFF; CCOM_SET_MOUSEACTIONCALL_*.
        211 | 212 | 2030 | 2032 | 2034 => {
            for id in ids(machine, command, 16)? {
                machine
                    .sys
                    .remembered
                    .insert(MOUSEACTIONCALL + id.clamp(0, 99) as u16, 1);
            }
        }
        213 | 2031 | 2033 | 2035 => {
            for id in ids(machine, command, 16)? {
                machine
                    .sys
                    .remembered
                    .insert(MOUSEACTIONCALL + id.clamp(0, 99) as u16, 0);
            }
        }
        // MPLAY(name): music.
        289 => {
            let name = machine.str_param(command, 0)?;
            let now = machine.sys.now();
            let sys = &mut machine.sys;
            let result = sys
                .sound
                .bgm_play(&sys.resources, &sys.settings, now, &name, true, 0, 0);
            if let Err(error) = result {
                machine.note_unimplemented(format!("MPLAY {name}: {error:#}"));
            }
        }
        // LOADCALL_ / MOUSEACTIONCALL_ / EXAFTERCALL_FARRETURN: return
        // from a routine the system called.
        304..=306 => {
            machine.return_from_farcall()?;
            machine.sys.in_menu = false;
            return Ok(Next::Jumped);
        }
        // GET_EXCALL_MOD / GET_LOADCALL_MOD / GET_EXAFTERCALL_MOD
        320 | 324 | 326 => {
            let key = match opcode {
                320 => "EXCALL_MOD",
                324 => "LOADCALL_MOD",
                _ => "EXAFTERCALL_MOD",
            };
            machine.store = machine.gameexe.int(key).unwrap_or(0);
        }
        // PCMPLAY(name, channel)
        371 => {
            let name = machine.str_param(command, 0)?;
            let channel = machine.int_param_or(command, 1, 0)?.max(0) as usize;
            let now = machine.sys.now();
            let sys = &mut machine.sys;
            let result = sys.sound.wav_play(
                &sys.resources,
                &sys.settings,
                now,
                &name,
                Some(channel),
                false,
                0,
            );
            if let Err(error) = result {
                machine.note_unimplemented(format!("PCMPLAY {name}: {error:#}"));
            }
        }
        // MsgBox(title, text) with OK, Yes/No, Yes/No/Cancel buttons.
        400 | 402 | 403 => {
            let title = machine.str_param(command, 0)?;
            let message = match command.params.get(1) {
                Some(param) if param.value.is_string() => machine.str_param(command, 1)?,
                Some(_) => machine.int_param(command, 1)?.to_string(),
                None => String::new(),
            };
            let buttons: &[(&str, i32)] = match opcode {
                400 => &[("OK", 1)],
                402 => &[("はい", 1), ("いいえ", 0)],
                _ => &[("はい", 1), ("いいえ", 0), ("キャンセル", -1)],
            };
            let op = crate::ui::MessageBox::with_buttons(title, message, buttons);
            machine.push_long_op(Box::new(op));
        }
        // CCOM_MSGBK_START_READY: the backlog opens on demand here.
        1010 => {}
        // CCOM_QUAKE_ON / _OFF([buf | first, last])
        1206 | 1207 => {
            let v = machine.int_params_from(command, 0)?;
            let (first, last) = match v[..] {
                [] => (0, 31),
                [one] => (one, one),
                [first, last, ..] => (first, last),
            };
            for buf in first.max(0)..=last.min(99) {
                machine
                    .sys
                    .remembered
                    .insert(CCOM_QUAKE + buf as u16, i32::from(opcode == 1206));
            }
        }
        // CCOM_FULLQUAKE / _SYSBTN / _HINT / _SELBTN _ON / _OFF
        1208..=1215 => {
            let key = CCOM_SWITCHES + (opcode - 1208) / 2;
            machine
                .sys
                .remembered
                .insert(key, i32::from(opcode.is_multiple_of(2)));
        }
        // CCOM_LOCAL_NAME_EXCOPY(index): the local name as it was at the
        // savepoint; _NOW keeps the current one (a no-op here).
        2001 | 2005 => {
            let index = machine.int_param(command, 0)?.max(0) as usize;
            let memory = &mut machine.memory;
            if let Some(saved) = memory.savepoint.local_names.get(index).cloned()
                && let Some(name) = memory.local.local_names.get_mut(index)
            {
                *name = saved;
            }
        }
        2003 => {}
        // CCOM_LOCAL_FLAG_EXCOPY_NOW(var) keeps the current value;
        // _SAVEPOINT restores the savepoint's.
        2002 => {}
        2004 => {
            for index in 0..command.params.len() {
                let target = machine.int_target_param(command, index)?;
                if let IntTarget::Mem(reference) = target
                    && let Some(value) = machine.memory.savepoint_int(reference)
                {
                    machine.set_target(target, value)?;
                }
            }
        }
        // CCOM_SET_WINDOW_EXBTN_ON / _OFF (_NOW, _SAVEPOINT)([button])
        2010..=2015 => {
            let on = opcode.is_multiple_of(2);
            for b in ids(machine, command, 8)? {
                let b = b.max(0) as usize;
                if on {
                    machine.sys.exbtn_hidden.remove(&b);
                } else {
                    machine.sys.exbtn_hidden.insert(b);
                }
            }
        }
        // CCOM_SET_WINDOW_EXBTN_MOD (_NOW, _SAVEPOINT)([button,] mod)
        2020..=2022 => {
            let v = machine.int_params_from(command, 0)?;
            let (buttons, value) = match v[..] {
                [b, value, ..] => (b..b + 1, value),
                [value] => (0..10, value),
                [] => (0..10, 1),
            };
            for b in buttons {
                machine
                    .sys
                    .remembered
                    .insert(EXBTN_MOD + b.clamp(0, 9) as u16, value);
            }
        }
        // LOADEVENT_G_FLAG_SET(use[, flag, id])
        3000 => {
            let v = machine.int_params_from(command, 0)?;
            for (i, value) in v.iter().enumerate().take(3) {
                machine.sys.remembered.insert(LOADEVENT + i as u16, *value);
            }
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}
