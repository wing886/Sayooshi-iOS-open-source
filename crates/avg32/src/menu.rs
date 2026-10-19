//! In-engine menus and dialogs:
//! the right-click system menu (`#SYSCOM` items: save, load, text speed,
//! window background and frame, fast-forward, hide window, return to menu,
//! quit), the scenario-driven load picker (`0x58:04`) and name entry
//! (`0x61`).  They are drawn over the presented frame, never into the PDT
//! buffers, so closing them leaves the game screen untouched.

use crate::nls;
use crate::system::{HostRequest, Key, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuKind {
    Root,
    Save,
    Load,
    LoadPicker,
    Speed,
    Background,
    Frame,
    Volume,
    ConfirmMenu,
    ConfirmQuit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    Open(MenuKind),
    Save(i32),
    Load(i32),
    Pick(i32),
    Speed(i32),
    Background(i32),
    Frame(i32),
    Volume(usize, i32),
    FastForward,
    HideWindow,
    Fullscreen(bool),
    ReturnToMenu,
    Quit,
    Close,
}

/// Work a menu hands to the engine because it needs the scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemAction {
    Save(i32),
    Load(i32),
    ReturnToMenu,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuState {
    pub kind: MenuKind,
    pub title: String,
    pub items: Vec<MenuItem>,
    pub selected: usize,
    parents: Vec<MenuKind>,
}

const PANEL_WIDTH: i32 = 360;
const ROW: i32 = 22;
const TEXT: u32 = 16;

fn item(label: impl Into<String>, action: MenuAction, enabled: bool) -> MenuItem {
    MenuItem {
        label: label.into(),
        action,
        enabled,
    }
}

impl MenuState {
    fn build(sys: &System, kind: MenuKind) -> Self {
        let syscom = |index: usize, fallback: &str| {
            sys.ini()
                .syscom
                .get(&index)
                .filter(|entry| entry.enabled && !entry.name.is_empty())
                .map_or_else(|| fallback.to_owned(), |entry| entry.name.clone())
        };
        let sub = |index: usize, item: usize, fallback: &str| {
            sys.ini()
                .syscom
                .get(&index)
                .and_then(|entry| entry.items.get(&item))
                .filter(|text| !text.is_empty())
                .cloned()
                .unwrap_or_else(|| fallback.to_owned())
        };
        let slot_label = |slot: usize| {
            let summary = &sys.slots[slot];
            if summary.valid {
                format!(
                    "{:02}/{:02}({:02}:{:02}) {}",
                    summary.month,
                    summary.day,
                    summary.hour,
                    summary.minute,
                    sys.decode_title(&summary.title)
                )
            } else {
                let empty = sys.ini().save_no_title.trim().to_owned();
                if empty.is_empty() {
                    "--".to_owned()
                } else {
                    empty
                }
            }
        };
        let (title, items) = match kind {
            MenuKind::Root => {
                let mut items = Vec::new();
                let has = |index: usize| {
                    sys.ini()
                        .syscom
                        .get(&index)
                        .is_none_or(|entry| entry.enabled)
                };
                if has(0) {
                    items.push(item(
                        syscom(0, "セーブ"),
                        MenuAction::Open(MenuKind::Save),
                        sys.menu_enabled(0),
                    ));
                }
                if has(1) {
                    items.push(item(
                        syscom(1, "ロード"),
                        MenuAction::Open(MenuKind::Load),
                        sys.menu_enabled(1),
                    ));
                }
                if has(2) {
                    items.push(item(
                        syscom(2, "文字速度"),
                        MenuAction::Open(MenuKind::Speed),
                        sys.menu_enabled(2),
                    ));
                }
                if has(3) {
                    items.push(item(
                        syscom(3, "ウィンドウ背景"),
                        MenuAction::Open(MenuKind::Background),
                        sys.menu_enabled(3),
                    ));
                }
                if has(4) {
                    items.push(item(
                        syscom(4, "音量の設定"),
                        MenuAction::Open(MenuKind::Volume),
                        sys.menu_enabled(4),
                    ));
                }
                if has(5) {
                    items.push(item(
                        syscom(5, "画面モード"),
                        MenuAction::Fullscreen(true),
                        sys.menu_enabled(5),
                    ));
                }
                if has(12) {
                    items.push(item(
                        syscom(12, "ウィンドウ枠"),
                        MenuAction::Open(MenuKind::Frame),
                        sys.menu_enabled(12),
                    ));
                }
                if has(25) {
                    let label = format!(
                        "{}{}",
                        syscom(25, "メッセージ早送り"),
                        if sys.skip_mode { " ✓" } else { "" }
                    );
                    items.push(item(label, MenuAction::FastForward, sys.menu_enabled(25)));
                }
                if has(31) {
                    items.push(item(
                        syscom(31, "ウィンドウを消す"),
                        MenuAction::HideWindow,
                        sys.menu_enabled(31) && sys.mes.shown,
                    ));
                }
                if has(28) {
                    items.push(item(
                        syscom(28, "メニューに戻る"),
                        MenuAction::Open(MenuKind::ConfirmMenu),
                        sys.menu_enabled(28),
                    ));
                }
                if has(29) {
                    items.push(item(
                        syscom(29, "ゲームを終わる"),
                        MenuAction::Open(MenuKind::ConfirmQuit),
                        true,
                    ));
                }
                items.push(item(syscom(30, "ゲームに戻る"), MenuAction::Close, true));
                (sys.ini().caption.clone(), items)
            }
            MenuKind::Save | MenuKind::Load | MenuKind::LoadPicker => {
                let items = (0..sys.slot_count())
                    .map(|slot| {
                        let action = match kind {
                            MenuKind::Save => MenuAction::Save(slot as i32),
                            MenuKind::Load => MenuAction::Load(slot as i32),
                            _ => MenuAction::Pick(slot as i32 + 1),
                        };
                        let enabled = kind == MenuKind::Save || sys.slots[slot].valid;
                        item(
                            format!("{:2}: {}", slot + 1, slot_label(slot)),
                            action,
                            enabled,
                        )
                    })
                    .chain(std::iter::once(item(
                        sub(if kind == MenuKind::Save { 0 } else { 1 }, 1, "キャンセル"),
                        if kind == MenuKind::LoadPicker {
                            MenuAction::Pick(0)
                        } else {
                            MenuAction::Close
                        },
                        true,
                    )))
                    .collect();
                let title = if kind == MenuKind::Save {
                    syscom(0, "セーブ")
                } else {
                    syscom(1, "ロード")
                };
                (title, items)
            }
            MenuKind::Speed => {
                let current = sys.ini().mes_wait;
                let items = [
                    (0, "瞬間"),
                    (20, "速い"),
                    (40, "普通"),
                    (70, "遅い"),
                    (110, "とても遅い"),
                ]
                .into_iter()
                .map(|(speed, label)| {
                    let wait = if speed == 0 { 0 } else { speed + 8 };
                    let mark = if wait == current { " ✓" } else { "" };
                    item(format!("{label}{mark}"), MenuAction::Speed(speed), true)
                })
                .collect();
                (syscom(2, "文字速度"), items)
            }
            MenuKind::Background => {
                let opaque = sys.ini().win_color_flag != 0;
                let items = vec![
                    item(
                        format!("半透明{}", if opaque { "" } else { " ✓" }),
                        MenuAction::Background(0),
                        true,
                    ),
                    item(
                        format!("不透明{}", if opaque { " ✓" } else { "" }),
                        MenuAction::Background(1),
                        true,
                    ),
                ];
                (syscom(3, "ウィンドウ背景"), items)
            }
            MenuKind::Frame => {
                let current = sys.mes.style;
                let items = (1..=3)
                    .map(|style| {
                        let label = sub(12, style as usize - 1, &format!("パターン{style}"));
                        let mark = if style == current { " ✓" } else { "" };
                        item(format!("{label}{mark}"), MenuAction::Frame(style), true)
                    })
                    .collect();
                (syscom(12, "ウィンドウ枠"), items)
            }
            MenuKind::Volume => {
                let names = [sub(4, 0, "BGM"), sub(4, 1, "効果音"), "音声".to_owned()];
                let mut items = Vec::new();
                for (channel, name) in names.into_iter().enumerate() {
                    let volume = sys.sound.volumes[channel];
                    items.push(item(
                        format!("{name} {}%  ‐", volume * 100 / 255),
                        MenuAction::Volume(channel, -32),
                        true,
                    ));
                    items.push(item(
                        format!("{name} {}%  ＋", volume * 100 / 255),
                        MenuAction::Volume(channel, 32),
                        true,
                    ));
                }
                (syscom(4, "音量の設定"), items)
            }
            MenuKind::ConfirmMenu => (
                syscom(28, "メニューに戻る"),
                vec![
                    item(sub(28, 0, "やっぱり続ける"), MenuAction::Close, true),
                    item(sub(28, 1, "メニューに戻る"), MenuAction::ReturnToMenu, true),
                ],
            ),
            MenuKind::ConfirmQuit => (
                syscom(29, "ゲームを終わる"),
                vec![
                    item(sub(29, 0, "やっぱり続ける"), MenuAction::Close, true),
                    item(sub(29, 1, "ゲームを終わる"), MenuAction::Quit, true),
                ],
            ),
        };
        let selected = items.iter().position(|item| item.enabled).unwrap_or(0);
        Self {
            kind,
            title,
            items,
            selected,
            parents: Vec::new(),
        }
    }

    pub fn context(sys: &System) -> Self {
        Self::build(sys, MenuKind::Root)
    }

    pub fn load_picker(sys: &System) -> Self {
        Self::build(sys, MenuKind::LoadPicker)
    }

    fn geometry(&self) -> (i32, i32, i32) {
        let height = (self.items.len() as i32 + 1) * ROW + 12;
        let left = (640 - PANEL_WIDTH) / 2;
        let top = ((480 - height) / 2).max(4);
        (left, top, height)
    }

    fn item_at(&self, x: i32, y: i32) -> Option<usize> {
        let (left, top, _) = self.geometry();
        if x < left || x >= left + PANEL_WIDTH {
            return None;
        }
        let row = (y - top - ROW - 6).div_euclid(ROW);
        (0..self.items.len() as i32)
            .contains(&row)
            .then_some(row as usize)
    }
}

fn activate(sys: &mut System, index: usize) {
    let Some(menu) = sys.menu.as_ref() else {
        return;
    };
    let Some(item) = menu.items.get(index).cloned() else {
        return;
    };
    if !item.enabled {
        return;
    }
    sys.play_se(1);
    match item.action {
        MenuAction::Open(kind) => {
            let mut next = MenuState::build(sys, kind);
            let mut parents = sys
                .menu
                .as_ref()
                .map(|menu| menu.parents.clone())
                .unwrap_or_default();
            parents.push(sys.menu.as_ref().map_or(MenuKind::Root, |menu| menu.kind));
            next.parents = parents;
            sys.menu = Some(next);
        }
        MenuAction::Save(slot) => {
            sys.pending_action = Some(SystemAction::Save(slot));
            sys.menu = None;
        }
        MenuAction::Load(slot) => {
            sys.pending_action = Some(SystemAction::Load(slot));
            sys.menu = None;
        }
        MenuAction::Pick(slot) => {
            sys.menu_result = Some(slot);
            sys.menu = None;
        }
        MenuAction::Speed(speed) => {
            sys.ini_mut().mes_wait = if speed == 0 { 0 } else { speed + 8 };
            sys.menu = None;
        }
        MenuAction::Background(flag) => {
            let colour = sys.ini().win_color;
            sys.set_window_colour(flag, colour);
            sys.menu = None;
        }
        MenuAction::Frame(style) => {
            sys.mes_setup(style);
            sys.menu = None;
        }
        MenuAction::Volume(channel, delta) => {
            let volume = sys.sound.volumes[channel] + delta;
            sys.sound.set_volume(channel, volume);
            let selected = index;
            let parents = sys
                .menu
                .as_ref()
                .map(|menu| menu.parents.clone())
                .unwrap_or_default();
            let mut next = MenuState::build(sys, MenuKind::Volume);
            next.selected = selected;
            next.parents = parents;
            sys.menu = Some(next);
        }
        MenuAction::FastForward => {
            sys.skip_mode = !sys.skip_mode;
            sys.menu = None;
        }
        MenuAction::HideWindow => {
            sys.menu = None;
            if sys.mes.shown {
                sys.hide_window = true;
                sys.mes_hide_temp();
            }
        }
        MenuAction::Fullscreen(on) => {
            sys.requests.push(HostRequest::ToggleFullscreen(on));
            sys.menu = None;
        }
        MenuAction::ReturnToMenu => {
            sys.pending_action = Some(SystemAction::ReturnToMenu);
            sys.menu = None;
        }
        MenuAction::Quit => {
            sys.requests.push(HostRequest::Quit);
            sys.running = false;
            sys.menu = None;
        }
        MenuAction::Close => sys.menu = None,
    }
}

fn back(sys: &mut System) {
    let Some(menu) = sys.menu.take() else {
        return;
    };
    if menu.kind == MenuKind::LoadPicker {
        sys.menu_result = Some(0);
        return;
    }
    let mut parents = menu.parents;
    if let Some(parent) = parents.pop() {
        let mut previous = MenuState::build(sys, parent);
        previous.parents = parents;
        sys.menu = Some(previous);
    }
}

pub fn menu_click(sys: &mut System, right: bool) {
    if right {
        back(sys);
        return;
    }
    let (x, y) = (sys.mouse.x, sys.mouse.y);
    let Some(index) = sys.menu.as_ref().and_then(|menu| menu.item_at(x, y)) else {
        return;
    };
    activate(sys, index);
}

pub fn menu_key(sys: &mut System, key: Key) {
    let Some(menu) = sys.menu.as_mut() else {
        return;
    };
    let count = menu.items.len();
    match key {
        Key::Up | Key::Left => {
            for step in 1..=count {
                let index = (menu.selected + count - step) % count;
                if menu.items[index].enabled {
                    menu.selected = index;
                    break;
                }
            }
        }
        Key::Down | Key::Right => {
            for step in 1..=count {
                let index = (menu.selected + step) % count;
                if menu.items[index].enabled {
                    menu.selected = index;
                    break;
                }
            }
        }
        Key::Enter | Key::Space => {
            let selected = menu.selected;
            activate(sys, selected);
        }
        Key::Escape | Key::Backspace => back(sys),
        _ => {}
    }
}

/// Keeps the highlighted row under the mouse.
pub fn menu_hover(sys: &mut System) {
    let (x, y) = (sys.mouse.x, sys.mouse.y);
    if let Some(menu) = sys.menu.as_mut() {
        if let Some(index) = menu.item_at(x, y) {
            if menu.items[index].enabled {
                menu.selected = index;
            }
        }
    }
}

// ---- name entry ------------------------------------------------------------------

/// Names are at most 12 Shift-JIS bytes.
fn clamp_name(text: &str) -> String {
    let mut out = String::new();
    for character in text.chars() {
        let mut candidate = out.clone();
        candidate.push(character);
        if nls::encode(&candidate).len() > 12 {
            break;
        }
        out = candidate;
    }
    out
}

pub fn name_input_text(sys: &mut System, text: &str) {
    let Some(input) = sys.name_input.as_mut() else {
        return;
    };
    if input
        .inline_box
        .as_ref()
        .is_some_and(|inline| inline.target.is_none())
    {
        return;
    }
    let focus = input.focus.min(1);
    let appended = format!("{}{}", input.values[focus], text.replace(['\r', '\n'], ""));
    input.values[focus] = clamp_name(&appended);
}

pub fn name_input_key(sys: &mut System, key: Key) {
    let Some(input) = sys.name_input.as_mut() else {
        return;
    };
    let inline = input.inline_box.is_some();
    match key {
        Key::Backspace => {
            let focus = input.focus.min(1);
            input.values[focus].pop();
        }
        Key::Up | Key::Down if !inline => {
            if input.indices[1] > 0 && input.indices[0] > 0 {
                input.focus = 1 - input.focus.min(1);
            }
        }
        Key::Enter => {
            if inline {
                // The in-scene box is read back by the script (`0x61:02`)
                // after it sees the confirming click.
                let now = sys.now();
                sys.mouse.flush();
                let _ = now;
                sys.mouse_click_for_script();
                return;
            }
            let input = sys.name_input.take().expect("checked above");
            for field in 0..2 {
                if input.indices[field] > 0 && !input.values[field].trim().is_empty() {
                    let name = nls::encode(input.values[field].trim());
                    sys.set_name(input.indices[field] - 1, &name);
                }
            }
            sys.save_global_flags();
        }
        Key::Escape if !inline => sys.name_input = None,
        _ => {}
    }
}

// ---- backlog ----------------------------------------------------------------------

pub fn backlog_key(sys: &mut System, key: Key) {
    match key {
        Key::PageUp | Key::Up => sys.wheel(true),
        Key::PageDown | Key::Down => sys.wheel(false),
        Key::Escape | Key::Backspace => sys.backlog_view = None,
        Key::Enter | Key::Space => backlog_click(sys, false),
        _ => {}
    }
}

pub fn backlog_click(sys: &mut System, right: bool) {
    if right {
        sys.backlog_view = None;
        return;
    }
    let voice = sys
        .backlog_view
        .and_then(|page| sys.backlog.get(page))
        .and_then(|page| page.voice);
    match voice {
        Some(voice) => sys.replay_voice(voice),
        None => sys.backlog_view = None,
    }
}

/// Splits a message-buffer page into display lines.
fn backlog_lines(page: &[u8], width: usize) -> Vec<String> {
    let mut lines = vec![String::new()];
    let mut at = 0;
    while at < page.len() {
        let byte = page[at];
        match byte {
            0xfe | 0xff | 0x00 => at += 1,
            0x0d => {
                lines.push(String::new());
                at += 1;
            }
            _ => {
                let (character, length) = nls::char_at(&page[at..]);
                let length = length.max(1);
                let line = lines.last_mut().expect("never empty");
                if line.chars().count() >= width {
                    lines.push(String::new());
                }
                lines.last_mut().expect("never empty").push(character);
                at += length;
            }
        }
    }
    lines
}

// ---- drawing ----------------------------------------------------------------------

fn fill(rgba: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32, colour: [u8; 3], alpha: u32) {
    for y in y1.max(0)..y2.min(480) {
        for x in x1.max(0)..x2.min(640) {
            let at = (y as usize * 640 + x as usize) * 4;
            for channel in 0..3 {
                let under = u32::from(rgba[at + channel]);
                rgba[at + channel] =
                    ((u32::from(colour[channel]) * alpha + under * (255 - alpha)) / 255) as u8;
            }
        }
    }
}

fn text(sys: &mut System, rgba: &mut [u8], x: i32, baseline: i32, label: &str, colour: [u8; 3]) {
    let Some(font) = sys.font.as_mut() else {
        return;
    };
    let mut pen = x;
    for character in label.chars() {
        if let Some(glyph) = font.glyph(character, TEXT) {
            for row in 0..glyph.height {
                for column in 0..glyph.width {
                    let alpha = u32::from(glyph.coverage[row * glyph.width + column]);
                    let (px, py) = (
                        pen + glyph.left + column as i32,
                        baseline + glyph.top + row as i32,
                    );
                    if alpha == 0 || !(0..640).contains(&px) || !(0..480).contains(&py) {
                        continue;
                    }
                    let at = (py as usize * 640 + px as usize) * 4;
                    for channel in 0..3 {
                        let under = u32::from(rgba[at + channel]);
                        rgba[at + channel] = ((u32::from(colour[channel]) * alpha
                            + under * (255 - alpha))
                            / 255) as u8;
                    }
                }
            }
        }
        pen += if character.is_ascii() {
            TEXT as i32 / 2 + 1
        } else {
            TEXT as i32
        };
        if pen > x + PANEL_WIDTH - 24 {
            break;
        }
    }
}

fn text_wide(
    sys: &mut System,
    rgba: &mut [u8],
    x: i32,
    baseline: i32,
    label: &str,
    colour: [u8; 3],
) {
    let Some(font) = sys.font.as_mut() else {
        return;
    };
    let mut pen = x;
    for character in label.chars() {
        if let Some(glyph) = font.glyph(character, 22) {
            for row in 0..glyph.height {
                for column in 0..glyph.width {
                    let alpha = u32::from(glyph.coverage[row * glyph.width + column]);
                    let (px, py) = (
                        pen + glyph.left + column as i32,
                        baseline + glyph.top + row as i32,
                    );
                    if alpha == 0 || !(0..640).contains(&px) || !(0..480).contains(&py) {
                        continue;
                    }
                    let at = (py as usize * 640 + px as usize) * 4;
                    for channel in 0..3 {
                        let under = u32::from(rgba[at + channel]);
                        rgba[at + channel] = ((u32::from(colour[channel]) * alpha
                            + under * (255 - alpha))
                            / 255) as u8;
                    }
                }
            }
        }
        pen += if character.is_ascii() { 12 } else { 22 };
    }
}

/// Lay out raw character slices so replacement-font slots survive history rendering.
fn novel_backlog_cells(page: &[u8], encoding: nls::Nls) -> Vec<(usize, usize, usize, usize)> {
    let (mut at, mut column, mut row) = (0, 0, 0);
    let mut cells = Vec::new();
    while at < page.len() && row < 14 {
        match page[at] {
            0xfe | 0xff | 0 => at += 1,
            0x0d => { column = 0; row += 1; at += 1; }
            byte => {
                if column >= 25 { column = 0; row += 1; }
                if row >= 14 { break; }
                let length = encoding.char_len(byte).min(page.len() - at).max(1);
                cells.push((at, length, column, row));
                at += length;
                column += 1;
            }
        }
    }
    cells
}

fn novel_backlog(sys: &mut System, rgba: &mut [u8], page: &[u8]) {
    let encoding = nls::current();
    for (at, length, column, row) in novel_backlog_cells(page, encoding) {
        let raw = &page[at..at + length];
        let glyph = encoding.novel_code(raw)
            .and_then(|code| sys.novel_font.as_ref()?.glyph(code)).map(|g| g.to_vec());
        let (x, y) = (40 + column * 22, 60 + row * 28);
        if let Some(glyph) = glyph {
            for dy in 0..22 {
                for dx in 0..22 {
                    let (sx, sy) = (dx * 24 / 22, dy * 24 / 22);
                    let byte = glyph[sy * 12 + sx / 2];
                    let nibble = if sx & 1 == 0 { byte & 15 } else { byte >> 4 };
                    let alpha = 255 - u32::from(nibble) * 17;
                    let pixel = ((y + dy) * 640 + x + dx) * 4;
                    for channel in 0..3 {
                        rgba[pixel + channel] = ((255 * alpha + u32::from(rgba[pixel + channel]) * (255 - alpha)) / 255) as u8;
                    }
                }
            }
        } else {
            text_wide(sys, rgba, x as i32, y as i32 + 22, &encoding.decode(raw), [255, 255, 255]);
        }
    }
}

#[cfg(test)]
mod backlog_tests {
    use super::*;
    #[test]
    fn history_keeps_raw_slots_and_wraps_inside_viewport() {
        let page = [0xfe, 0x85, 0xf2, 0x0d, 0x85, 0xf3, 0xff];
        let cells = novel_backlog_cells(&page, nls::Nls::Sjis);
        assert_eq!(cells, vec![(1, 2, 0, 0), (4, 2, 0, 1)]);
        assert_eq!(nls::Nls::Sjis.novel_code(&page[1..3]), Some(0x85f2));
        let cells = novel_backlog_cells(&vec![b'A'; 400], nls::Nls::Sjis);
        assert_eq!(cells.len(), 350);
        assert_eq!(cells[25], (25, 1, 0, 1));
        assert_eq!(cells.last(), Some(&(349, 1, 24, 13)));
    }
}

pub fn draw_overlays(sys: &mut System, rgba: &mut [u8]) {
    if let Some(index) = sys.backlog_view {
        fill(rgba, 0, 0, 640, 480, [0, 0, 16], 200);
        let total = sys.backlog.len();
        if let Some(page) = sys.backlog.get(index).cloned() {
            let header = format!(
                "BACKLOG  {}/{}{}",
                index + 1,
                total,
                if page.voice.is_some() { "  ♪" } else { "" }
            );
            text(sys, rgba, 24, 36, &header, [160, 176, 255]);
            if sys.ini().novel_mode != 0 && sys.novel_font.is_some() {
                novel_backlog(sys, rgba, &page.text);
            } else {
                let mut baseline = 80;
                for line in backlog_lines(&page.text, 25) {
                    text_wide(sys, rgba, 40, baseline, &line, [255, 255, 255]);
                    baseline += 28;
                    if baseline > 450 { break; }
                }
            }
        }
        text(
            sys,
            rgba,
            24,
            468,
            "↑↓:page  click:voice  R:close",
            [128, 128, 160],
        );
        return;
    }
    if let Some(menu) = sys.menu.clone() {
        let (left, top, height) = menu.geometry();
        fill(
            rgba,
            left,
            top,
            left + PANEL_WIDTH,
            top + height,
            [16, 24, 48],
            220,
        );
        fill(
            rgba,
            left,
            top,
            left + PANEL_WIDTH,
            top + ROW + 4,
            [48, 64, 112],
            255,
        );
        text(
            sys,
            rgba,
            left + 10,
            top + ROW - 4,
            &menu.title,
            [255, 255, 255],
        );
        for (index, item) in menu.items.iter().enumerate() {
            let row_top = top + ROW + 6 + index as i32 * ROW;
            if index == menu.selected {
                fill(
                    rgba,
                    left + 4,
                    row_top,
                    left + PANEL_WIDTH - 4,
                    row_top + ROW,
                    [96, 128, 208],
                    200,
                );
            }
            let colour = if item.enabled {
                [255, 255, 255]
            } else {
                [128, 128, 128]
            };
            text(sys, rgba, left + 14, row_top + ROW - 5, &item.label, colour);
        }
    }
    if let Some(input) = sys.name_input.clone() {
        if let Some(inline) = &input.inline_box {
            if inline.target.is_none() {
                return;
            }
            let [x1, y1, x2, y2] = inline.rect;
            let background = inline.background.map(|channel| channel.clamp(0, 255) as u8);
            let foreground = inline.foreground.map(|channel| channel.clamp(0, 255) as u8);
            fill(rgba, x1, y1, x2 + 1, y2 + 1, background, 255);
            let caret = format!("{}_", input.values[0]);
            text(sys, rgba, x1 + 2, y2 - 4, &caret, foreground);
            return;
        }
        let rows = input
            .indices
            .iter()
            .filter(|index| **index > 0)
            .count()
            .max(1) as i32;
        let height = rows * ROW * 2 + ROW + 16;
        let (left, top) = ((640 - PANEL_WIDTH) / 2, (480 - height) / 2);
        fill(
            rgba,
            left,
            top,
            left + PANEL_WIDTH,
            top + height,
            [16, 24, 48],
            230,
        );
        let mut row = top + 8;
        for field in 0..2 {
            if input.indices[field] <= 0 {
                continue;
            }
            text(
                sys,
                rgba,
                left + 12,
                row + ROW - 5,
                &input.titles[field],
                [200, 200, 255],
            );
            row += ROW;
            let focused = field == input.focus.min(1);
            fill(
                rgba,
                left + 12,
                row,
                left + PANEL_WIDTH - 12,
                row + ROW,
                if focused { [64, 80, 140] } else { [40, 48, 80] },
                255,
            );
            let value = if focused {
                format!("{}_", input.values[field])
            } else {
                input.values[field].clone()
            };
            text(sys, rgba, left + 16, row + ROW - 5, &value, [255, 255, 255]);
            row += ROW;
        }
        text(
            sys,
            rgba,
            left + 12,
            row + ROW - 2,
            "Enter: OK",
            [160, 160, 160],
        );
    }
}
