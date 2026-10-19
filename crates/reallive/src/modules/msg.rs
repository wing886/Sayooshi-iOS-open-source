//! Text output (module 0:3) and plain text elements.

use anyhow::Result;

use crate::bytecode::Command;
use crate::machine::{Machine, Next, SavepointKind};
use crate::text::Ruby;
use crate::textout::{self, PauseKind, PauseOp, TextoutOp};

/// Displays text from a text element or `strout`.
pub fn textout(machine: &mut Machine, text: &str) -> Result<()> {
    let text = {
        let memory = &machine.memory;
        textout::interpret_names(text, |local, index| memory.name(local, index).to_owned())
    };
    if text.is_empty() {
        return Ok(());
    }
    if machine.sys.text.page_is_empty() && machine.should_set_savepoint(SavepointKind::Message) {
        machine.mark_savepoint();
    }
    machine.sys.text.log.push(text.clone());
    let op = TextoutOp::new(machine, &text);
    machine.push_long_op(Box::new(op));
    Ok(())
}

pub(crate) fn pause(machine: &mut Machine, kind: PauseKind) -> Next {
    let op = PauseOp::new(machine, kind);
    machine.push_long_op(Box::new(op));
    Next::Advance
}

/// `par`: a new paragraph (line break, indentation reset).
fn paragraph(machine: &mut Machine) {
    let index = machine.sys.text.active;
    let config = machine.sys.text.windows[index].clone();
    let state = &mut machine.sys.text.states[index];
    state.indent = 0;
    if state.x > 0 || !state.chars.is_empty() {
        state.x = 0;
        state.y += textout::line_height(&config);
        state.line += 1;
        machine.sys.text.current_page.lines.push(String::new());
    }
    machine.sys.text.states[index].last_was_name = false;
}

fn hard_break(machine: &mut Machine) {
    let index = machine.sys.text.active;
    let config = machine.sys.text.windows[index].clone();
    let state = &mut machine.sys.text.states[index];
    state.x = state.indent;
    state.y += textout::line_height(&config);
    state.line += 1;
    machine.sys.text.current_page.lines.push(String::new());
}

fn colour_param(machine: &mut Machine, command: &Command) -> Result<(i32, i32)> {
    let text = machine.int_param_or(command, 0, 0)?;
    let shadow = machine.int_param_or(command, 1, -1)?;
    Ok((text, shadow))
}

fn active_windows(machine: &Machine) -> Vec<usize> {
    (0..machine.sys.text.states.len())
        .filter(|&index| machine.sys.text.states[index].visible)
        .collect()
}

pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let active = machine.sys.text.active;
    match command.op.opcode {
        3 | 202 => paragraph(machine),
        17 => return Ok(pause(machine, PauseKind::Pause)),
        15 | 205 | 206 => return Ok(pause(machine, PauseKind::Spause)),
        207 => return Ok(pause(machine, PauseKind::PauseAll)),
        210 => return Ok(pause(machine, PauseKind::Page)),
        // SetFontColour: the window's default colour.
        100 => {
            let colour = colour_param(machine, command)?;
            machine.sys.text.states[active].colour = Some(colour);
        }
        101 => {
            let size = match Machine::param_count(command) {
                0 => None,
                _ => Some(machine.int_param(command, 0)?),
            };
            machine.sys.text.states[active].size_override = size;
        }
        102 => {
            let window = machine.int_param_or(command, 0, 0)?;
            let count = machine.sys.text.states.len() as i32;
            machine.sys.text.active = window.clamp(0, count - 1) as usize;
        }
        103 => machine.sys.text.fast_text = true,
        104 => machine.sys.text.fast_text = false,
        // FontColour: until the next pause.
        105 => {
            let colour = colour_param(machine, command)?;
            machine.sys.text.states[active].colour_override = Some(colour);
        }
        106 => {
            let colour = colour_param(machine, command)?;
            for state in &mut machine.sys.text.states {
                state.colour = Some(colour);
            }
        }
        107 => {
            let size = match Machine::param_count(command) {
                0 => None,
                _ => Some(machine.int_param(command, 0)?),
            };
            for state in &mut machine.sys.text.states {
                state.size_override = size;
            }
        }
        109 => machine.sys.settings.message_no_wait = true,
        110 => machine.sys.settings.message_no_wait = false,
        111 => machine.store = active as i32,
        // `\ruby{base}={ruby}`: begin, then display the ruby text.
        120 if Machine::param_count(command) == 0 => {
            let state = &mut machine.sys.text.states[active];
            state.ruby_start = Some(state.x);
        }
        120 => {
            let ruby = machine.str_param(command, 0)?;
            let state = &mut machine.sys.text.states[active];
            if let Some(start) = state.ruby_start.take() {
                let y = state.y;
                let x2 = state.x;
                state.rubies.push(Ruby {
                    text: ruby,
                    x1: start,
                    x2,
                    y,
                });
            }
        }
        151 => {
            textout::close_window(&mut machine.sys, active);
        }
        152 => textout::clear_window(&mut machine.sys, active),
        161 => {
            for index in active_windows(machine) {
                textout::close_window(&mut machine.sys, index);
            }
        }
        162 => {
            for index in active_windows(machine) {
                textout::clear_window(&mut machine.sys, index);
            }
        }
        170 => show_background(machine)?,
        201 => hard_break(machine),
        300 => {
            let state = &mut machine.sys.text.states[active];
            state.indent = state.x;
        }
        301 => machine.sys.text.states[active].indent = 0,
        310 => {
            let x = machine.int_param(command, 0)?;
            let y = machine.int_param(command, 1)?;
            let luby = machine.sys.text.windows[active].luby_size;
            let state = &mut machine.sys.text.states[active];
            state.x = x;
            state.y = y + luby;
        }
        311 => machine.sys.text.states[active].x = machine.int_param(command, 0)?,
        312 => {
            let y = machine.int_param(command, 0)?;
            let luby = machine.sys.text.windows[active].luby_size;
            machine.sys.text.states[active].y = y + luby;
        }
        320 => {
            let x = machine.int_param(command, 0)?;
            let y = machine.int_param(command, 1)?;
            let state = &mut machine.sys.text.states[active];
            state.x += x;
            state.y += y;
        }
        321 => machine.sys.text.states[active].x += machine.int_param(command, 0)?,
        322 => machine.sys.text.states[active].y += machine.int_param(command, 0)?,
        330 => {
            let luby = machine.sys.text.windows[active].luby_size;
            let (x, y) = {
                let state = &machine.sys.text.states[active];
                (state.x, state.y - luby)
            };
            if Machine::param_count(command) >= 2 {
                let tx = machine.int_target_param(command, 0)?;
                let ty = machine.int_target_param(command, 1)?;
                machine.set_target(tx, x)?;
                machine.set_target(ty, y)?;
            } else {
                // With one argument it receives Y.
                let ty = machine.int_target_param(command, 0)?;
                machine.set_target(ty, y)?;
            }
        }
        340 => {
            let window = machine.int_param_or(command, 0, active as i32)?;
            machine.store = machine
                .sys
                .text
                .states
                .get(window.max(0) as usize)
                .map_or(0, |state| state.chars.len() as i32);
        }
        341 => {
            machine.store = machine
                .sys
                .text
                .states
                .iter()
                .map(|s| s.chars.len() as i32)
                .sum();
        }
        1000 => {
            let file = machine.str_param(command, 0)?;
            let slot = machine.int_param_or(command, 1, 0)?.clamp(0, 7) as usize;
            machine.sys.text.states[active].faces[slot] = Some(file);
        }
        1001 => {
            let slot = machine.int_param_or(command, 0, 0)?.clamp(0, 7) as usize;
            machine.sys.text.states[active].faces[slot] = None;
        }
        // MESSAGE_SPEED(speed): this message's speed.
        108 => machine.sys.text.speed_override = Some(machine.int_param(command, 0)?),
        // MULTI_MESSAGE(window...): the text goes to the first window named.
        112 => {
            let window = machine.int_param(command, 0)?;
            let count = machine.sys.text.states.len() as i32;
            machine.sys.text.active = window.clamp(0, count - 1) as usize;
        }
        // SYSBTN_OPEN / _CLOSE([time]): the floating system buttons, which
        // this interface does not have.
        400 | 401 => {
            machine
                .sys
                .remembered
                .insert(SYSBTN, i32::from(command.op.opcode == 400));
        }
        // GET/SET_SYSBTN_PATNO_USER/_OWNER, GET_SYSBTN_PATMOD,
        // SET_SYSBTN_PATMOD_USER/_OWNER/(mod), and the same for the
        // full-screen backlog (502..509).
        402..=409 | 502..=509 => {
            let opcode = command.op.opcode;
            let base = if opcode >= 500 {
                MSGBK_PATTERNS
            } else {
                SYSBTN + 1
            };
            match opcode % 100 {
                2 | 4 | 6 => {
                    let key = base + (opcode % 100 - 2) / 2;
                    machine.store = machine.sys.remembered.get(&key).copied().unwrap_or(0);
                }
                3 | 5 => {
                    let value = machine.int_param(command, 0)?;
                    machine
                        .sys
                        .remembered
                        .insert(base + (opcode % 100 - 3) / 2, value);
                }
                // PATMOD: 0 the user's pattern, 1 the owner's.
                7 => {
                    machine.sys.remembered.insert(base + 2, 0);
                }
                8 => {
                    machine.sys.remembered.insert(base + 2, 1);
                }
                _ => {
                    let value = machine.int_param(command, 0)?;
                    machine.sys.remembered.insert(base + 2, value);
                }
            }
        }
        // FLAGINDEX(var): a debugging aid of the original's message
        // window; nothing to show here.
        2000 => {}
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

/// `remembered` keys: the system button bar's state, then its pattern
/// numbers (user, owner) and mode; the full-screen backlog's patterns.
const SYSBTN: u16 = 30400;
const MSGBK_PATTERNS: u16 = 30500;

/// Hides the text windows until the player clicks.
#[derive(Debug)]
struct ShowBackground;

impl crate::machine::LongOp for ShowBackground {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let input = &mut machine.sys.input;
        let done = input.take_click().is_some()
            || input.take_key(crate::input::Key::Enter)
            || input.take_key(crate::input::Key::Space)
            || machine.sys.should_fast_forward();
        machine.sys.text.hidden_temporarily = !done;
        Ok(done)
    }

    fn name(&self) -> &'static str {
        "show background"
    }
}

/// `ShowBackground()` / `msgHideAllTemp`.
pub fn show_background(machine: &mut Machine) -> Result<()> {
    machine.sys.text.hidden_temporarily = true;
    machine.push_long_op(Box::new(ShowBackground));
    Ok(())
}
