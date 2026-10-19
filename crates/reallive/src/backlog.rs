//! The message backlog: earlier pages of text, browsed with the wheel.

use crate::input::{Button, InputEvent, Key};
use crate::system::System;

/// Opens the backlog on the most recent committed page.
pub fn open(sys: &mut System) {
    if sys.text.backlog.is_empty() {
        return;
    }
    let msgbk = sys
        .text
        .windows
        .get(sys.text.active)
        .is_none_or(|config| config.msgbk_use);
    if msgbk {
        sys.text.backlog_view = Some(sys.text.backlog.len() - 1);
    }
}

pub fn close(sys: &mut System) {
    sys.text.backlog_view = None;
}

/// Handles the input while the backlog is open.
pub fn handle_input(sys: &mut System) {
    let Some(mut page) = sys.text.backlog_view else {
        return;
    };
    let last = sys.text.backlog.len().saturating_sub(1);
    let events = std::mem::take(&mut sys.input.events);
    for event in events {
        match event {
            InputEvent::Press(Button::WheelUp) | InputEvent::KeyDown(Key::Up | Key::PageUp) => {
                page = page.saturating_sub(1);
            }
            InputEvent::Press(Button::WheelDown)
            | InputEvent::KeyDown(Key::Down | Key::PageDown) => {
                if page >= last {
                    sys.text.backlog_view = None;
                    return;
                }
                page += 1;
            }
            InputEvent::KeyDown(Key::Home) => page = 0,
            InputEvent::KeyDown(Key::End) => page = last,
            InputEvent::Press(Button::Right | Button::Left)
            | InputEvent::KeyDown(Key::Escape | Key::Enter) => {
                sys.text.backlog_view = None;
                return;
            }
            _ => {}
        }
    }
    sys.text.backlog_view = Some(page.min(last));
}
