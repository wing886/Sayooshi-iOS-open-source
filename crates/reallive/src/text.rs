//! Text windows, pages and the backlog.
//!
//! This module holds the configuration and state of the (up to 64) text
//! windows defined by `#WINDOW.nnn.*` blocks; layout and rendering live in
//! [`crate::textout`].

use std::collections::BTreeMap;

use crate::gameexe::Gameexe;
use crate::settings::WindowAttr;

pub const WINDOW_COUNT: usize = 64;

/// `#WINDOW.nnn.POS = origin : x, y`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WindowPos {
    /// 0 top-left, 1 top-right, 2 bottom-left, 3 bottom-right.
    pub origin: i32,
    pub x: i32,
    pub y: i32,
}

/// A window's static configuration plus the parts scripts can change.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowConfig {
    pub index: usize,
    pub pos: WindowPos,
    pub default_pos: WindowPos,
    pub attr_mod: i32,
    pub attr: WindowAttr,
    pub waku_setno: i32,
    pub waku_mod: i32,
    pub waku_no: i32,
    /// `KEYCUR_MOD = type : x, y`.
    pub keycur: (i32, i32, i32),
    pub open_anm_mod: i32,
    pub open_anm_time: i32,
    pub close_anm_mod: i32,
    pub close_anm_time: i32,
    pub anm_enabled: bool,
    /// Characters per line and lines.
    pub moji_cnt: (i32, i32),
    /// Padding: top, bottom, left, right.
    pub moji_pos: [i32; 4],
    pub moji_size: i32,
    pub moji_rep: (i32, i32),
    pub luby_size: i32,
    pub indent_use: bool,
    pub moji_shadow: bool,
    pub kinsoku_use: bool,
    pub r_command_mod: i32,
    pub selcom_use: bool,
    pub moji_min: (i32, i32),
    pub selcom_setpos: [i32; 4],
    pub selcom_mojipos: i32,
    pub selcom_cursorselect: i32,
    pub selcom_cursorno: i32,
    pub selcom_mojidark: i32,
    pub selcom_mouseset: bool,
    pub name_mod: i32,
    pub name_pos: (i32, i32),
    pub name_moji_size: i32,
    pub name_moji_rep: i32,
    pub name_moji_pos: (i32, i32),
    pub name_moji_min: i32,
    pub name_waku_dir: i32,
    pub name_waku_setno: i32,
    pub name_centering: bool,
    pub msgbk_use: bool,
    /// `FACE.n = x, y, behind, ...`.
    pub faces: Vec<(i32, i32, i32)>,
    /// Button flags keyed by name (`CLEAR_USE`, `EXBTN_000_USE`, ...).
    pub buttons: BTreeMap<String, bool>,
}

impl WindowConfig {
    pub fn from_gameexe(exe: &Gameexe, index: usize) -> Self {
        let key = |name: &str| format!("WINDOW.{index:03}.{name}");
        let ints = |name: &str| exe.ints(&key(name));
        let int = |name: &str, default: i32| exe.int(&key(name)).unwrap_or(default);
        let pair = |name: &str, default: (i32, i32)| {
            let values = ints(name);
            (
                values.first().copied().unwrap_or(default.0),
                values.get(1).copied().unwrap_or(default.1),
            )
        };
        let pos_values = ints("POS");
        let pos = WindowPos {
            origin: pos_values.first().copied().unwrap_or(0),
            x: pos_values.get(1).copied().unwrap_or(0),
            y: pos_values.get(2).copied().unwrap_or(0),
        };
        let moji_pos_values = ints("MOJI_POS");
        let mut moji_pos = [0; 4];
        for (slot, value) in moji_pos.iter_mut().zip(&moji_pos_values) {
            *slot = *value;
        }
        let keycur_values = ints("KEYCUR_MOD");
        let setpos_values = ints("SELCOM_SETPOS");
        let mut selcom_setpos = [0; 4];
        for (slot, value) in selcom_setpos.iter_mut().zip(&setpos_values) {
            *slot = *value;
        }
        let mut faces = Vec::new();
        for face in 0..8 {
            let values = exe.ints(&format!("WINDOW.{index:03}.FACE.{face:03}"));
            if values.len() >= 2 {
                faces.push((values[0], values[1], values.get(2).copied().unwrap_or(0)));
            }
        }
        let prefix = format!("WINDOW.{index:03}.");
        let buttons = exe
            .filter(&prefix)
            .filter_map(|entry| {
                let name = entry.key.strip_prefix(&prefix)?;
                (name.ends_with("_USE") && name != "INDENT_USE" && name != "KINSOKU_USE")
                    .then(|| (name.to_owned(), entry.int(0).unwrap_or(0) != 0))
            })
            .collect();
        let moji_size = int("MOJI_SIZE", 25);
        Self {
            index,
            pos,
            default_pos: pos,
            attr_mod: int("ATTR_MOD", 0),
            attr: WindowAttr::from_slice(&ints("ATTR")).unwrap_or_default(),
            waku_setno: int("WAKU_SETNO", 0),
            waku_mod: int("WAKU_MOD", 0),
            waku_no: int("WAKU_NO", 0),
            keycur: (
                keycur_values.first().copied().unwrap_or(0),
                keycur_values.get(1).copied().unwrap_or(0),
                keycur_values.get(2).copied().unwrap_or(0),
            ),
            open_anm_mod: int("OPEN_ANM_MOD", 0),
            open_anm_time: int("OPEN_ANM_TIME", 0),
            close_anm_mod: int("CLOSE_ANM_MOD", 0),
            close_anm_time: int("CLOSE_ANM_TIME", 0),
            anm_enabled: true,
            moji_cnt: pair("MOJI_CNT", (20, 3)),
            moji_pos,
            moji_size,
            moji_rep: pair("MOJI_REP", (0, 0)),
            luby_size: int("LUBY_SIZE", 0),
            indent_use: int("INDENT_USE", 1) != 0,
            moji_shadow: int("MOJI_SHADOW", 1) != 0,
            kinsoku_use: int("KINSOKU_USE", 1) != 0,
            r_command_mod: int("R_COMMAND_MOD", 0),
            selcom_use: int("SELCOM_USE", 1) != 0,
            moji_min: pair("MOJI_MIN", (0, 0)),
            selcom_setpos,
            selcom_mojipos: int("SELCOM_MOJIPOS", 0),
            selcom_cursorselect: int("SELCOM_CURSORSELECT", 0),
            selcom_cursorno: int("SELCOM_CURSORNO", 0),
            selcom_mojidark: int("SELCOM_MOJIDARK", 64),
            selcom_mouseset: int("SELCOM_MOUSESET", 0) != 0,
            name_mod: int("NAME_MOD", 0),
            name_pos: pair("NAME_POS", (0, 0)),
            name_moji_size: int("NAME_MOJI_SIZE", moji_size),
            name_moji_rep: int("NAME_MOJI_REP", 0),
            name_moji_pos: pair("NAME_MOJI_POS", (0, 0)),
            name_moji_min: int("NAME_MOJI_MIN", 0),
            name_waku_dir: int("NAME_WAKU_DIR", 0),
            name_waku_setno: int("NAME_WAKU_SETNO", 0),
            name_centering: int("NAME_CENTERING", 0) != 0,
            msgbk_use: int("MSGBK_USE", 1) != 0,
            faces,
            buttons,
        }
    }

    /// The effective waku pattern given the global `SetWakuAll` value.
    pub fn waku_pattern(&self, waku_all: i32) -> i32 {
        if self.waku_mod != 0 {
            self.waku_no
        } else {
            waku_all
        }
    }

    pub fn effective_attr(&self, global: WindowAttr) -> WindowAttr {
        if self.attr_mod != 0 {
            self.attr
        } else {
            global
        }
    }
}

/// A character placed in a window.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedChar {
    pub c: char,
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub colour: [u8; 3],
    pub shadow: Option<[u8; 3]>,
}

/// A ruby gloss over `x1..x2` of the line at `y`.
#[derive(Debug, Clone, PartialEq)]
pub struct Ruby {
    pub text: String,
    pub x1: i32,
    pub x2: i32,
    pub y: i32,
}

/// What a window currently shows.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WindowState {
    pub visible: bool,
    pub chars: Vec<PlacedChar>,
    pub rubies: Vec<Ruby>,
    /// The speaker set by `【name】`.
    pub name: String,
    /// Text of the separate name box (`NAME_MOD` 1).
    pub namebox: Option<String>,
    /// Insertion point in pixels, relative to the text area.
    pub x: i32,
    pub y: i32,
    pub indent: i32,
    pub line: i32,
    /// `FontSize()` until the next pause.
    pub size_override: Option<i32>,
    /// `FontColour()` until the next pause: (text, shadow) colour indices.
    pub colour_override: Option<(i32, i32)>,
    /// `SetFontColour()`: (text, shadow) colour indices.
    pub colour: Option<(i32, i32)>,
    pub ruby_start: Option<i32>,
    pub last_was_name: bool,
    /// Characters since the last pause (auto mode timing).
    pub chars_since_pause: i32,
    pub faces: [Option<String>; 8],
    /// Open/close animation: (start time, opening).
    pub animation: Option<(u64, bool)>,
    /// Voice replay markers: (x, y, koe id).
    pub koe_markers: Vec<(i32, i32, i32)>,
}

/// One screen of text kept for the message backlog.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BacklogPage {
    pub name: String,
    pub lines: Vec<String>,
    pub voices: Vec<i32>,
}

pub const BACKLOG_PAGES: usize = 256;

#[derive(Debug, Clone)]
pub struct TextSystem {
    pub windows: Vec<WindowConfig>,
    pub states: Vec<WindowState>,
    pub active: usize,
    /// `FastText()`.
    pub fast_text: bool,
    /// `MESSAGE_SPEED(speed)`: ms per character until the next pause.
    pub speed_override: Option<i32>,
    kidoku_read: bool,
    /// Every piece of text output, in order (headless tools and tests).
    pub log: Vec<String>,
    pub backlog: Vec<BacklogPage>,
    pub current_page: BacklogPage,
    /// The index of the backlog page being viewed, if any.
    pub backlog_view: Option<usize>,
    /// Waiting for a click: when the key cursor started animating.
    pub waiting_since: Option<u64>,
    /// Windows hidden by `msgHideAllTemp` / `ShowBackground`.
    pub hidden_temporarily: bool,
    /// `SET_WINDOW_DISP_OFF` / `CCOM_MESSAGEWINDOW_OFF`: windows not drawn.
    pub display_off: Vec<bool>,
    /// `SET_MSGBK_ON` / `SET_MSGBK_OFF`: whether pages enter the backlog.
    pub backlog_enabled: bool,
    /// Window positions at the last savepoint.
    savepoint_positions: Vec<WindowPos>,
}

impl Default for TextSystem {
    fn default() -> Self {
        Self::new(&Gameexe::default())
    }
}

impl TextSystem {
    pub fn new(exe: &Gameexe) -> Self {
        let windows: Vec<WindowConfig> = (0..WINDOW_COUNT)
            .map(|index| WindowConfig::from_gameexe(exe, index))
            .collect();
        let savepoint_positions = windows.iter().map(|w| w.pos).collect();
        Self {
            windows,
            states: vec![WindowState::default(); WINDOW_COUNT],
            active: 0,
            fast_text: false,
            speed_override: None,
            kidoku_read: false,
            log: Vec::new(),
            backlog: Vec::new(),
            current_page: BacklogPage::default(),
            backlog_view: None,
            waiting_since: None,
            hidden_temporarily: false,
            display_off: vec![false; WINDOW_COUNT],
            backlog_enabled: true,
            savepoint_positions,
        }
    }

    pub fn window(&self, index: i32) -> Option<&WindowConfig> {
        self.windows.get(usize::try_from(index).ok()?)
    }

    pub fn window_mut(&mut self, index: i32) -> Option<&mut WindowConfig> {
        self.windows.get_mut(usize::try_from(index).ok()?)
    }

    pub fn state(&self, index: usize) -> &WindowState {
        &self.states[index.min(WINDOW_COUNT - 1)]
    }

    pub fn state_mut(&mut self, index: usize) -> &mut WindowState {
        &mut self.states[index.min(WINDOW_COUNT - 1)]
    }

    pub fn active_state(&mut self) -> &mut WindowState {
        let active = self.active;
        self.state_mut(active)
    }

    /// No text on the active window's page yet (savepoints happen here).
    pub fn page_is_empty(&self) -> bool {
        self.states[self.active].chars.is_empty()
    }

    pub fn set_kidoku_read(&mut self, read: bool) {
        self.kidoku_read = read;
    }

    pub fn kidoku_read(&self) -> bool {
        self.kidoku_read
    }

    /// Moves the current page into the backlog.
    pub fn commit_page(&mut self) {
        let page = std::mem::take(&mut self.current_page);
        if self.backlog_enabled && !page.lines.iter().all(|line| line.is_empty()) {
            self.backlog.push(page);
            if self.backlog.len() > BACKLOG_PAGES {
                self.backlog.remove(0);
            }
        }
    }

    pub fn take_savepoint(&mut self) {
        self.savepoint_positions = self.windows.iter().map(|w| w.pos).collect();
    }

    pub fn reset(&mut self) {
        for state in &mut self.states {
            *state = WindowState::default();
        }
        self.active = 0;
        self.current_page = BacklogPage::default();
        self.backlog_view = None;
        self.waiting_since = None;
        self.hidden_temporarily = false;
    }

    pub fn save(&self, w: &mut crate::serial::Writer) {
        w.len(self.savepoint_positions.len());
        for pos in &self.savepoint_positions {
            w.i32s(&[pos.origin, pos.x, pos.y]);
        }
        w.u32(self.active as u32);
    }

    pub fn load(&mut self, r: &mut crate::serial::Reader) -> anyhow::Result<()> {
        let count = r.len()?;
        for index in 0..count {
            let pos = r.i32s()?;
            if let (Some(window), [origin, x, y]) = (self.windows.get_mut(index), &pos[..]) {
                window.pos = WindowPos {
                    origin: *origin,
                    x: *x,
                    y: *y,
                };
            }
        }
        self.active = (r.u32()? as usize).min(WINDOW_COUNT - 1);
        Ok(())
    }
}
