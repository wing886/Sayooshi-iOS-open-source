//! In-engine replacements for the interpreter's own dialogs: the system
//! command menu, save/load slot lists, name entry and text input boxes.
//!
//! Each dialog is a long operation that updates [`Ui::overlay`] every
//! frame; the renderer draws the overlay on top of the game screen and
//! stores the hit rectangles it used so the dialog can resolve clicks.

use std::collections::BTreeMap;

use anyhow::Result;

use crate::input::{Button, InputEvent, Key};
use crate::machine::{LongOp, Machine};
use crate::modules::menu::SlotMenuKind;

/// Something the engine should open at the next opportunity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    SyscomMenu,
    /// A settings dialog for a system command (message speed, volume, ...).
    SettingsDialog(i32),
    /// `shell`: a file of the game (its manual) for the host to open.
    OpenFile(std::path::PathBuf),
}

/// A text box created with `CreateInput`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InputBox {
    pub rect: (i32, i32, i32, i32),
    pub font_size: i32,
    pub background: [i32; 3],
    pub foreground: [i32; 3],
    pub text: String,
}

/// One row of a dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub text: String,
    pub enabled: bool,
}

/// What the renderer should draw for the active dialog.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Overlay {
    pub title: String,
    pub rows: Vec<Row>,
    pub selected: usize,
    /// First visible row (for scrolling lists).
    pub scroll: usize,
    /// Rows the renderer laid out: `(row index, x, y, w, h)`.
    pub hit_rects: Vec<(usize, i32, i32, i32, i32)>,
    /// Text being edited, with a caret.
    pub editing: Option<String>,
}

#[derive(Debug, Default)]
pub struct Ui {
    pub requests: Vec<Request>,
    pub overlay: Option<Overlay>,
    pub inputs: BTreeMap<i32, InputBox>,
    pub focused_input: Option<i32>,
}

impl Ui {
    pub fn request(&mut self, request: Request) {
        if !self.requests.contains(&request) {
            self.requests.push(request);
        }
    }

    pub fn create_input(&mut self, index: i32, values: &[i32]) {
        let get = |i: usize| values.get(i).copied().unwrap_or(0);
        self.inputs.insert(
            index,
            InputBox {
                rect: (get(0), get(1), get(2), get(3)),
                font_size: get(4),
                background: [get(5), get(6), get(7)],
                foreground: [get(8), get(9), get(10)],
                text: String::new(),
            },
        );
    }
}

/// Keyboard/mouse navigation shared by the list dialogs. Returns
/// `Some(Some(row))` when a row was chosen, `Some(None)` when cancelled.
fn navigate(machine: &mut Machine, overlay: &mut Overlay) -> Option<Option<usize>> {
    let count = overlay.rows.len();
    if count == 0 {
        return Some(None);
    }
    let (mx, my) = machine.sys.input.mouse;
    if let Some(&(row, ..)) = overlay
        .hit_rects
        .iter()
        .find(|(_, x, y, w, h)| mx >= *x && my >= *y && mx < x + w && my < y + h)
    {
        overlay.selected = row;
    }
    let events = std::mem::take(&mut machine.sys.input.events);
    let mut result = None;
    for event in events {
        match event {
            InputEvent::KeyDown(Key::Up) | InputEvent::Press(Button::WheelUp) => {
                overlay.selected = (overlay.selected + count - 1) % count;
            }
            InputEvent::KeyDown(Key::Down) | InputEvent::Press(Button::WheelDown) => {
                overlay.selected = (overlay.selected + 1) % count;
            }
            InputEvent::KeyDown(Key::PageUp) => {
                overlay.selected = overlay.selected.saturating_sub(10)
            }
            InputEvent::KeyDown(Key::PageDown) => {
                overlay.selected = (overlay.selected + 10).min(count - 1);
            }
            InputEvent::KeyDown(Key::Enter | Key::Space) | InputEvent::Press(Button::Left) => {
                let hit = matches!(event, InputEvent::Press(Button::Left))
                    .then(|| {
                        overlay
                            .hit_rects
                            .iter()
                            .find(|(_, x, y, w, h)| {
                                mx >= *x && my >= *y && mx < x + w && my < y + h
                            })
                            .map(|entry| entry.0)
                    })
                    .flatten();
                let chosen = hit.unwrap_or(overlay.selected);
                if matches!(event, InputEvent::Press(Button::Left)) && hit.is_none() {
                    continue;
                }
                if overlay.rows.get(chosen).is_some_and(|row| row.enabled) {
                    result = Some(Some(chosen));
                    break;
                }
            }
            InputEvent::KeyDown(Key::Escape) | InputEvent::Press(Button::Right) => {
                result = Some(None);
                break;
            }
            _ => {}
        }
    }
    // Keep the selection visible in a ten-row window.
    if overlay.selected < overlay.scroll {
        overlay.scroll = overlay.selected;
    } else if overlay.selected >= overlay.scroll + 10 {
        overlay.scroll = overlay.selected + 1 - 10;
    }
    result
}

/// Number of save slots offered by the built-in menus.
pub fn slot_count(machine: &Machine) -> i32 {
    machine
        .gameexe
        .int("SAVE_CNT")
        .or_else(|| machine.gameexe.int("SAVEFILE_CNT"))
        .unwrap_or(100)
        .clamp(1, 1000)
}

/// Save/load slot list.
/// The quick-save slot: the one after the numbered slots.
pub fn quick_slot(machine: &Machine) -> i32 {
    slot_count(machine)
}

#[derive(Debug)]
pub struct SlotMenu {
    kind: SlotMenuKind,
    overlay: Overlay,
}

impl SlotMenu {
    pub fn new(machine: &Machine, kind: SlotMenuKind) -> Self {
        let saving = matches!(kind, SlotMenuKind::Save | SlotMenuKind::ChooseSave);
        let rows = (0..slot_count(machine))
            .map(|slot| match crate::save::slot_header(machine, slot) {
                Some(header) => Row {
                    text: format!(
                        "{:03}  {:04}/{:02}/{:02} {:02}:{:02}  {}",
                        slot + 1,
                        header.date[0],
                        header.date[1],
                        header.date[2],
                        header.date[4],
                        header.date[5],
                        header.title
                    ),
                    enabled: true,
                },
                None => Row {
                    text: format!("{:03}  ----", slot + 1),
                    enabled: saving,
                },
            })
            .collect();
        let latest = machine.latest_save.max(0) as usize;
        Self {
            kind,
            overlay: Overlay {
                title: if saving {
                    "セーブ".into()
                } else {
                    "ロード".into()
                },
                rows,
                selected: latest,
                ..Overlay::default()
            },
        }
    }
}

impl LongOp for SlotMenu {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        if let Some(overlay) = &machine.sys.ui.overlay {
            self.overlay.hit_rects.clone_from(&overlay.hit_rects);
        }
        let choice = navigate(machine, &mut self.overlay);
        let Some(choice) = choice else {
            machine.sys.ui.overlay = Some(self.overlay.clone());
            return Ok(false);
        };
        machine.sys.ui.overlay = None;
        let slot = choice.map_or(-1, |row| row as i32);
        machine.store = slot;
        if slot >= 0 {
            match self.kind {
                SlotMenuKind::Save => crate::save::save_slot(machine, slot)?,
                SlotMenuKind::Load => crate::save::load_slot(machine, slot)?,
                _ => {}
            }
        }
        Ok(true)
    }

    fn name(&self) -> &'static str {
        "slot menu"
    }
}

/// `nwSingle`/`nwMulti`: edits one or more name variables.
#[derive(Debug)]
pub struct NameEntry {
    local: bool,
    fields: Vec<(i32, String)>,
    values: Vec<String>,
    current: usize,
}

impl NameEntry {
    pub fn new(machine: &Machine, local: bool, fields: Vec<(i32, String)>) -> Self {
        let values = fields
            .iter()
            .map(|(index, _)| {
                machine
                    .memory
                    .name(local, (*index).max(0) as usize)
                    .to_owned()
            })
            .collect();
        Self {
            local,
            fields,
            values,
            current: 0,
        }
    }

    fn max_len(machine: &Machine) -> usize {
        machine.gameexe.int("NAME_MAXLEN").unwrap_or(6).clamp(6, 10) as usize
    }
}

impl LongOp for NameEntry {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        if self.fields.is_empty() {
            return Ok(true);
        }
        let max = Self::max_len(machine);
        let events = std::mem::take(&mut machine.sys.input.events);
        let mut done = false;
        for event in events {
            let value = &mut self.values[self.current];
            match event {
                InputEvent::Text(text) => {
                    for c in text.chars().filter(|c| !c.is_control()) {
                        if value.chars().count() < max {
                            value.push(c);
                        }
                    }
                }
                InputEvent::KeyDown(Key::Backspace) => {
                    value.pop();
                }
                InputEvent::KeyDown(Key::Up) => {
                    self.current = self.current.saturating_sub(1);
                }
                InputEvent::KeyDown(Key::Down | Key::Tab) => {
                    self.current = (self.current + 1).min(self.fields.len() - 1);
                }
                InputEvent::KeyDown(Key::Enter) => {
                    if self.current + 1 < self.fields.len() {
                        self.current += 1;
                    } else if self.values.iter().all(|value| !value.trim().is_empty()) {
                        done = true;
                    }
                }
                _ => {}
            }
        }
        if done {
            for ((index, _), value) in self.fields.iter().zip(&self.values) {
                machine.memory.set_name(
                    self.local,
                    (*index).max(0) as usize,
                    value.trim().to_owned(),
                );
            }
            machine.sys.ui.overlay = None;
            return Ok(true);
        }
        machine.sys.ui.overlay = Some(Overlay {
            title: "名前の入力".into(),
            rows: self
                .fields
                .iter()
                .zip(&self.values)
                .map(|((_, label), value)| Row {
                    text: format!("{label}  {value}"),
                    enabled: true,
                })
                .collect(),
            selected: self.current,
            editing: Some(self.values[self.current].clone()),
            ..Overlay::default()
        });
        Ok(false)
    }

    fn name(&self) -> &'static str {
        "name entry"
    }
}

/// The built-in system command menu (right click without `#CANCELCALL`).
#[derive(Debug)]
pub struct SyscomMenu {
    commands: Vec<i32>,
    overlay: Overlay,
}

impl SyscomMenu {
    pub fn new(machine: &Machine) -> Self {
        use crate::settings::syscom;
        let order = [
            syscom::SAVE,
            syscom::LOAD,
            syscom::RETURN_TO_PREVIOUS_SELECTION,
            syscom::SET_SKIP_MODE,
            syscom::AUTO_MODE,
            syscom::SHOW_BACKGROUND,
            syscom::MESSAGE_SPEED,
            syscom::VOLUME_SETTINGS,
            syscom::MENU_RETURN,
            syscom::EXIT_GAME,
        ];
        let mut commands = Vec::new();
        let mut rows = Vec::new();
        for index in order {
            // State 0 hides, 1 shows, 2 shows disabled.
            let state = machine.sys.syscom.state(index);
            if state == 0 {
                continue;
            }
            let label = machine
                .gameexe
                .str(&format!("SYSCOM.{index:03}"))
                .filter(|label| !label.is_empty())
                .map_or_else(
                    || crate::settings::syscom::default_label(index).to_owned(),
                    str::to_owned,
                );
            let enabled = state == 1
                && (index != syscom::RETURN_TO_PREVIOUS_SELECTION
                    || machine.previous_selection.is_some());
            commands.push(index);
            rows.push(Row {
                text: label,
                enabled,
            });
        }
        Self {
            commands,
            overlay: Overlay {
                title: "システム".into(),
                rows,
                ..Overlay::default()
            },
        }
    }
}

impl LongOp for SyscomMenu {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        if let Some(overlay) = &machine.sys.ui.overlay {
            self.overlay.hit_rects.clone_from(&overlay.hit_rects);
        }
        let Some(choice) = navigate(machine, &mut self.overlay) else {
            machine.sys.ui.overlay = Some(self.overlay.clone());
            return Ok(false);
        };
        machine.sys.ui.overlay = None;
        machine.sys.in_menu = false;
        if let Some(row) = choice {
            crate::modules::menu::invoke_syscom(machine, self.commands[row], None)?;
        }
        Ok(true)
    }

    fn name(&self) -> &'static str {
        "system menu"
    }
}

/// `MsgBox`: a message with OK (and Cancel) buttons; `store` receives 1
/// for OK and 0 for Cancel.
#[derive(Debug)]
pub struct MessageBox {
    overlay: Overlay,
    results: Vec<i32>,
}

impl MessageBox {
    pub fn new(title: String, message: String, cancel: bool) -> Self {
        if cancel {
            Self::with_buttons(title, message, &[("OK", 1), ("キャンセル", 0)])
        } else {
            Self::with_buttons(title, message, &[("OK", 1)])
        }
    }

    /// A box with the given buttons and what each stores.
    pub fn with_buttons(title: String, message: String, buttons: &[(&str, i32)]) -> Self {
        let rows = buttons
            .iter()
            .map(|(text, _)| Row {
                text: (*text).into(),
                enabled: true,
            })
            .collect();
        let title = if title.is_empty() {
            message
        } else {
            format!("{title}\n{message}")
        };
        Self {
            overlay: Overlay {
                title,
                rows,
                ..Overlay::default()
            },
            results: buttons.iter().map(|b| b.1).collect(),
        }
    }
}

impl LongOp for MessageBox {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        if let Some(overlay) = &machine.sys.ui.overlay {
            self.overlay.hit_rects.clone_from(&overlay.hit_rects);
        }
        let Some(choice) = navigate(machine, &mut self.overlay) else {
            machine.sys.ui.overlay = Some(self.overlay.clone());
            return Ok(false);
        };
        machine.sys.ui.overlay = None;
        // Cancelling picks the last button.
        let index = choice.unwrap_or(self.results.len() - 1);
        machine.store = self.results.get(index).copied().unwrap_or(0);
        Ok(true)
    }

    fn name(&self) -> &'static str {
        "message box"
    }
}
