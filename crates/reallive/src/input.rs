//! Keyboard and mouse state, fed by the host once per frame.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Left,
    Right,
    WheelUp,
    WheelDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Enter,
    Space,
    Escape,
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    Ctrl,
    Shift,
    Tab,
    Backspace,
    F(u8),
    Char(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    Press(Button),
    Release(Button),
    KeyDown(Key),
    KeyUp(Key),
    /// Committed text from an input method or the keyboard.
    Text(String),
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub mouse: (i32, i32),
    pub left_held: bool,
    pub right_held: bool,
    pub ctrl: bool,
    pub shift: bool,
    /// Events of the current frame, oldest first; long operations and
    /// handlers take the ones they use.
    pub events: Vec<InputEvent>,
    /// `KeyMouseOn` / `KeyMouseOff`: whether the cursor keys move the mouse.
    pub key_mouse: bool,
    /// Last click position for `GetClick`-style queries.
    pub last_click: Option<(i32, i32, Button)>,
    /// `GetCursorPos` button states (left, right): 0 nothing, 1 pressed,
    /// 2 released since the last `FlushClick`.
    pub button_state: [i32; 2],
    /// RealLive key-table codes (see [`key_code`]) held now.
    pub keys_held: std::collections::BTreeSet<i32>,
    /// Key-table codes pressed since `CLEAR_KEYTABLE` cleared them.
    pub keys_pressed: std::collections::BTreeSet<i32>,
    /// Wheel movement since `MOUSEWHEELCLEAR`: up negative, down positive.
    pub wheel: i32,
    /// Double clicks (left, right) since `DBLCLKCLEAR`, where they were.
    pub double_clicks: [Option<(i32, i32)>; 2],
    /// Time of the frame, for double-click detection (set by the engine).
    pub now: u64,
    /// The last press of each button: (time, position).
    last_press: [Option<(u64, (i32, i32))>; 2],
}

/// Two presses within this many ms, close together, are a double click.
const DOUBLE_CLICK_MS: u64 = 400;

/// The key-table code of an input (`GET_KEYTABLE_DATA` and friends).
///
/// The codes are RealLive's own, not Windows virtual keys. They are
/// inferred from how scripts use them: 0–3 are the cursor keys (scripts
/// pair 0/3 against 2/1 for "previous"/"next"), 4 and 5 decide and cancel,
/// 49 the space bar, 65/66 the mouse wheel, 80–89 the digit keys 0–9,
/// 100/101 the mouse buttons.
pub fn key_code(event: &InputEvent) -> Option<i32> {
    Some(match event {
        InputEvent::KeyDown(key) | InputEvent::KeyUp(key) => match key {
            Key::Up => 0,
            Key::Down => 1,
            Key::Right => 2,
            Key::Left => 3,
            Key::Enter => 4,
            Key::Escape | Key::Backspace => 5,
            Key::Space => 49,
            Key::PageUp => 65,
            Key::PageDown => 66,
            Key::Char(c @ '0'..='9') => 80 + (*c as i32 - '0' as i32),
            _ => return None,
        },
        InputEvent::Press(button) | InputEvent::Release(button) => match button {
            Button::WheelUp => 65,
            Button::WheelDown => 66,
            Button::Left => 100,
            Button::Right => 101,
        },
        InputEvent::Text(_) => return None,
    })
}

impl Input {
    pub fn push(&mut self, event: InputEvent) {
        match &event {
            InputEvent::Press(Button::Left) => {
                self.left_held = true;
                self.button_state = [1, 0];
            }
            InputEvent::Release(Button::Left) => {
                self.left_held = false;
                self.button_state[0] = 2;
            }
            InputEvent::Press(Button::Right) => {
                self.right_held = true;
                self.button_state = [0, 1];
            }
            InputEvent::Release(Button::Right) => {
                self.right_held = false;
                self.button_state[1] = 2;
            }
            InputEvent::KeyDown(Key::Ctrl) => self.ctrl = true,
            InputEvent::KeyUp(Key::Ctrl) => self.ctrl = false,
            InputEvent::KeyDown(Key::Shift) => self.shift = true,
            InputEvent::KeyUp(Key::Shift) => self.shift = false,
            _ => {}
        }
        if let InputEvent::Press(button) = event {
            self.last_click = Some((self.mouse.0, self.mouse.1, button));
            match button {
                Button::WheelUp => self.wheel -= 1,
                Button::WheelDown => self.wheel += 1,
                Button::Left | Button::Right => {
                    let side = usize::from(button == Button::Right);
                    let here = self.mouse;
                    let double = self.last_press[side].is_some_and(|(at, (x, y))| {
                        self.now.saturating_sub(at) <= DOUBLE_CLICK_MS
                            && (x - here.0).abs() <= 4
                            && (y - here.1).abs() <= 4
                    });
                    if double {
                        self.double_clicks[side] = Some(here);
                        self.last_press[side] = None;
                    } else {
                        self.last_press[side] = Some((self.now, here));
                    }
                }
            }
        }
        if let Some(code) = key_code(&event) {
            match event {
                InputEvent::KeyDown(_) | InputEvent::Press(_) => {
                    self.keys_held.insert(code);
                    self.keys_pressed.insert(code);
                    // Wheel "presses" have no release.
                    if matches!(code, 65 | 66) {
                        self.keys_held.remove(&code);
                    }
                }
                _ => {
                    self.keys_held.remove(&code);
                }
            }
        }
        self.events.push(event);
    }

    /// Takes the first press of `button` (Enter and Space count as a left
    /// click, Escape as a right click).
    pub fn take_press(&mut self, button: Button) -> bool {
        let position = self.events.iter().position(|event| match (event, button) {
            (InputEvent::Press(pressed), _) => *pressed == button,
            (InputEvent::KeyDown(Key::Enter | Key::Space), Button::Left) => true,
            (InputEvent::KeyDown(Key::Escape), Button::Right) => true,
            (InputEvent::KeyDown(Key::PageUp), Button::WheelUp) => true,
            (InputEvent::KeyDown(Key::PageDown), Button::WheelDown) => true,
            _ => false,
        });
        match position {
            Some(index) => {
                self.events.remove(index);
                true
            }
            None => false,
        }
    }

    /// Takes the first left or right click: `1` left, `-1` right.
    pub fn take_click(&mut self) -> Option<i32> {
        let position = self.events.iter().position(|event| {
            matches!(
                event,
                InputEvent::Press(Button::Left | Button::Right)
                    | InputEvent::KeyDown(Key::Enter | Key::Space | Key::Escape)
            )
        })?;
        let event = self.events.remove(position);
        Some(match event {
            InputEvent::Press(Button::Right) | InputEvent::KeyDown(Key::Escape) => -1,
            _ => 1,
        })
    }

    pub fn take_key(&mut self, key: Key) -> bool {
        match self
            .events
            .iter()
            .position(|event| *event == InputEvent::KeyDown(key))
        {
            Some(index) => {
                self.events.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn has_pending(&self) -> bool {
        !self.events.is_empty()
    }

    /// Drops all pending clicks (`FlushClick`).
    pub fn flush_clicks(&mut self) {
        self.events.retain(|event| {
            !matches!(
                event,
                InputEvent::Press(_) | InputEvent::KeyDown(Key::Enter | Key::Space | Key::Escape)
            )
        });
        self.last_click = None;
        self.button_state = [0, 0];
    }

    /// End of frame: unconsumed events expire.
    pub fn end_frame(&mut self) {
        self.events.clear();
    }
}
