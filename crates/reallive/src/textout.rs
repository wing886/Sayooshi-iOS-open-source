//! Text window layout, rendering and the text output / pause operations.
//!
//! Layout follows RealLive's fixed grid: a full-width character advances
//! `size + MOJI_REP.x`, a half-width one half of that; lines are
//! `MOJI_SIZE + MOJI_REP.y + LUBY_SIZE` apart. Closing punctuation may
//! squeeze into one extra cell at the end of a line; a character that
//! would leave such punctuation stranded at the start of the next line
//! moves down with it. `【name】` sets the speaker (it is not drawn) and,
//! like an opening quote at the start of a line, sets the indentation.

use std::rc::Rc;

use anyhow::Result;

use crate::image::Image;
use crate::input::{Button, InputEvent, Key};
use crate::machine::{LongOp, Machine};
use crate::nls::cell_width;
use crate::settings::WindowAttr;
use crate::surface::{Blend, Rect, Surface};
use crate::system::System;
use crate::text::{PlacedChar, Ruby, WindowConfig};

/// Characters that may not start a line.
pub fn is_kinsoku(c: char) -> bool {
    matches!(
        c,
        '、' | '。'
            | '，'
            | '．'
            | '：'
            | '；'
            | '？'
            | '！'
            | '゛'
            | '゜'
            | 'ヽ'
            | 'ヾ'
            | 'ゝ'
            | 'ゞ'
            | '々'
            | 'ー'
            | '）'
            | '］'
            | '｝'
            | '」'
            | '』'
            | '〕'
            | '〉'
            | '》'
            | '】'
            | '’'
            | '”'
            | 'ぁ'
            | 'ぃ'
            | 'ぅ'
            | 'ぇ'
            | 'ぉ'
            | 'っ'
            | 'ゃ'
            | 'ゅ'
            | 'ょ'
            | 'ゎ'
            | 'ァ'
            | 'ィ'
            | 'ゥ'
            | 'ェ'
            | 'ォ'
            | 'ッ'
            | 'ャ'
            | 'ュ'
            | 'ョ'
            | 'ヮ'
            | 'ヵ'
            | 'ヶ'
            | '・'
            | '…'
            | '‥'
            | '～'
            | '─'
            | ','
            | '.'
            | '!'
            | '?'
            | ')'
            | ']'
            | '}'
    )
}

pub fn is_opening_quote(c: char) -> bool {
    matches!(c, '「' | '『' | '（' | '“' | '〈' | '《' | '［' | '〔')
}

/// Pixel geometry of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometry {
    /// The whole window (frame included).
    pub window: Rect,
    /// The text area (insertion points are relative to its origin).
    pub text: Rect,
}

fn waku_key(config: &WindowConfig, waku_all: i32, part: &str) -> String {
    format!(
        "WAKU.{:03}.{:03}.{part}",
        config.waku_setno,
        config.waku_pattern(waku_all)
    )
}

pub(crate) fn load_named(sys: &mut System, name: &str) -> Option<Rc<Image>> {
    if name.is_empty() {
        return None;
    }
    let resources = &sys.resources;
    sys.gfx.load_image(resources, name).ok()
}

/// The frame images of a window: main, backing and whether it is a
/// nine-slice frame stretched around the text (types 3 and 4; type 3
/// also shrinks to what the window shows).
struct Waku {
    main: Option<Rc<Image>>,
    back: Option<Rc<Image>>,
    type4: bool,
    fit: bool,
    area: [i32; 4],
}

fn waku(sys: &mut System, config: &WindowConfig, name_box: bool) -> Waku {
    let waku_all = sys.settings.waku_all;
    let (setno, pattern) = if name_box {
        (config.name_waku_setno, 0)
    } else {
        (config.waku_setno, config.waku_pattern(waku_all))
    };
    let key = |part: &str| format!("WAKU.{setno:03}.{pattern:03}.{part}");
    let exe = sys.gameexe.clone();
    let main = exe.str(&key("NAME")).map(str::to_owned);
    let back = exe.str(&key("BACK")).map(str::to_owned);
    // Older games give the type per set (`#WAKU.002.TYPE`).
    let kind = exe
        .int(&key("TYPE"))
        .or_else(|| exe.int(&format!("WAKU.{setno:03}.TYPE")));
    let type4 = matches!(kind, Some(3 | 4));
    let fit = kind == Some(3);
    let mut area = [0; 4];
    for (slot, value) in area.iter_mut().zip(exe.ints(&key("AREA"))) {
        *slot = value;
    }
    let _ = waku_key;
    Waku {
        main: main.and_then(|name| load_named(sys, &name)),
        back: back.and_then(|name| load_named(sys, &name)),
        type4,
        fit,
        area,
    }
}

/// What a window shows, in pixels of its text area: the text placed so
/// far or the choices laid out in it.
fn content_size(sys: &System, index: usize, config: &WindowConfig) -> (i32, i32) {
    let line = line_height(config);
    let mut size = (0, 0);
    for ch in &sys.text.states[index].chars {
        let w = crate::nls::cell_width(ch.c) as i32 * ch.size / 2;
        size.0 = size.0.max(ch.x + w);
        size.1 = size.1.max(ch.y + line);
    }
    if let Some(selection) = &sys.selection
        && selection.layout == crate::select::Layout::Window(index)
    {
        for item in &selection.items {
            size.0 = size.0.max(item.rect.right());
            size.1 = size.1.max(item.rect.bottom());
        }
    }
    size
}

/// Size of the text area of a window.
pub fn text_area_size(config: &WindowConfig) -> (i32, i32) {
    let (cols, rows) = config.moji_cnt;
    let cell = config.moji_size + config.moji_rep.0;
    let line = line_height(config);
    // One extra cell for squeezed punctuation.
    (cols * cell + config.moji_size, rows * line)
}

pub fn line_height(config: &WindowConfig) -> i32 {
    config.moji_size + config.moji_rep.1 + config.luby_size
}

pub fn geometry(sys: &mut System, index: usize) -> Geometry {
    let config = sys.text.windows[index].clone();
    let (mut tw, mut th) = text_area_size(&config);
    let [top, bottom, left, right] = config.moji_pos;
    let frame = waku(sys, &config, false);
    if frame.fit {
        // Between `MOJI_MIN` and `MOJI_CNT` characters / lines.
        let cell = (config.moji_size + config.moji_rep.0).max(1);
        let line = line_height(&config).max(1);
        let (cw, ch) = content_size(sys, index, &config);
        let (min_cols, min_rows) = config.moji_min;
        // Choices longer than the window widen it (up to the screen).
        let widest = sys.gfx.width - left - right;
        tw = cw.max(min_cols * cell).min(widest.max(cell));
        th = ((ch + line - 1) / line * line).clamp((min_rows * line).min(th), th);
    }
    // A window sized to its content keeps as much room on the right as on
    // the left, and room for a glyph wider than the character pitch.
    let right = if frame.fit {
        right.max(left) + (-config.moji_rep.0).max(0)
    } else {
        right
    };
    let size = match (&frame.main, &frame.back) {
        _ if frame.type4 => (tw + left + right, th + top + bottom),
        (Some(main), _) => (main.width(), main.height()),
        (None, Some(back)) => (back.width(), back.height()),
        _ => (tw + left + right, th + top + bottom),
    };
    let (sw, sh) = (sys.gfx.width, sys.gfx.height);
    let pos = config.pos;
    let x = match pos.origin {
        1 | 3 => sw - size.0 - pos.x,
        _ => pos.x,
    };
    let y = match pos.origin {
        2 | 3 => sh - size.1 - pos.y,
        _ => pos.y,
    };
    // Keep the window on screen, as the interpreter does.
    let x = x.min(sw - size.0).max(0);
    let y = y.min(sh - size.1).max(0);
    Geometry {
        window: Rect::new(x, y, size.0, size.1),
        text: Rect::new(x + left, y + top, tw, th),
    }
}

pub(crate) fn colour_index(sys: &System, index: i32) -> [u8; 3] {
    sys.gfx
        .colours
        .get(index.max(0) as usize)
        .copied()
        .unwrap_or([255, 255, 255])
}

// ---- layout ----------------------------------------------------------------

/// Advance of `c` at `size` in a window.
fn advance(config: &WindowConfig, size: i32, c: char) -> i32 {
    let full = size + config.moji_rep.0;
    if cell_width(c) == 1 { full / 2 } else { full }
}

/// Starts a new line at the current indentation.
fn line_break(sys: &mut System, index: usize) {
    let config = &sys.text.windows[index];
    let height = line_height(config);
    let state = &mut sys.text.states[index];
    state.x = state.indent;
    state.y += height;
    state.line += 1;
    sys.text.current_page.lines.push(String::new());
}

pub fn is_full(sys: &System, index: usize) -> bool {
    let config = &sys.text.windows[index];
    sys.text.states[index].line >= config.moji_cnt.1.max(1)
}

/// Clears a window's text (a new page).
pub fn clear_window(sys: &mut System, index: usize) {
    let luby = sys.text.windows[index].luby_size;
    let state = &mut sys.text.states[index];
    state.chars.clear();
    state.rubies.clear();
    state.koe_markers.clear();
    state.x = 0;
    state.y = luby;
    state.indent = 0;
    state.line = 0;
    state.ruby_start = None;
    state.colour_override = None;
    state.size_override = None;
    state.last_was_name = false;
    // A new page starts without a speaker.
    state.namebox = None;
    state.name.clear();
    if index == sys.text.active {
        sys.text.commit_page();
    }
}

fn should_break(sys: &System, index: usize, c: char, rest: &[char]) -> bool {
    let config = &sys.text.windows[index];
    let state = &sys.text.states[index];
    let size = state.size_override.unwrap_or(config.moji_size);
    let width = advance(config, size, c);
    let normal = config.moji_cnt.0 * (config.moji_size + config.moji_rep.0);
    let extended = normal + config.moji_size;
    let squeeze = config.kinsoku_use && (is_kinsoku(c) || c == ' ');
    if squeeze && state.x + width <= extended {
        return false;
    }
    if state.x + width > normal {
        return true;
    }
    // Keep a run of following kinsoku characters with this one.
    if config.kinsoku_use && !squeeze {
        let mut end = state.x + width;
        for &next in rest {
            if !is_kinsoku(next) {
                break;
            }
            end += advance(config, size, next);
            if end > extended {
                return true;
            }
        }
    }
    false
}

/// Places one character; `false` when the window is full.
pub fn place_char(sys: &mut System, index: usize, c: char, rest: &[char]) -> bool {
    if is_full(sys, index) {
        return false;
    }
    let config = sys.text.windows[index].clone();
    sys.text.states[index].visible = true;
    let indent_after = {
        let state = &sys.text.states[index];
        (state.last_was_name && is_opening_quote(c))
            || (state.x == 0 && state.line >= 0 && is_opening_quote(c) && config.indent_use)
    };
    if should_break(sys, index, c, rest) {
        line_break(sys, index);
        if is_full(sys, index) {
            return false;
        }
    }
    let size = sys.text.states[index]
        .size_override
        .unwrap_or(config.moji_size);
    let (text_colour, shadow_colour) = {
        let state = &sys.text.states[index];
        state.colour_override.or(state.colour).unwrap_or((0, -1))
    };
    let colour = colour_index(sys, text_colour);
    let shadow = if !config.moji_shadow || sys.settings.font_shadow == 0 {
        None
    } else if shadow_colour >= 0 {
        Some(colour_index(sys, shadow_colour))
    } else {
        Some([0, 0, 0])
    };
    let state = &mut sys.text.states[index];
    state.chars.push(PlacedChar {
        c,
        x: state.x,
        y: state.y + (config.moji_size - size),
        size,
        colour,
        shadow,
    });
    state.x += advance(&config, size, c);
    state.chars_since_pause += 1;
    if indent_after && config.indent_use {
        state.indent = state.x;
    }
    state.last_was_name = false;
    if index == sys.text.active {
        let page = &mut sys.text.current_page;
        if page.lines.is_empty() {
            page.lines.push(String::new());
        }
        page.lines.last_mut().expect("non-empty").push(c);
    }
    true
}

/// `【name】`: records the speaker and displays it as the window wants.
pub fn set_name(sys: &mut System, index: usize, name: &str, next: Option<char>) {
    let config = sys.text.windows[index].clone();
    sys.text.states[index].name = name.to_owned();
    if index == sys.text.active {
        sys.text.current_page.name = name.to_owned();
    }
    match config.name_mod {
        1 => {
            sys.text.states[index].namebox = Some(name.to_owned());
            sys.text.states[index].visible = true;
        }
        2 | 3 => {}
        _ => {
            let chars: Vec<char> = name.chars().collect();
            for (i, &c) in chars.iter().enumerate() {
                place_char(sys, index, c, &chars[i + 1..]);
            }
        }
    }
    // Indentation starts after the name: an opening quote that follows is
    // printed first; otherwise an implicit full-width space.
    if config.name_mod != 1 && !next.is_some_and(is_opening_quote) {
        if config.name_mod == 0 || config.name_mod == 2 {
            place_char(sys, index, '\u{3000}', &[]);
        }
        if config.indent_use {
            let state = &mut sys.text.states[index];
            state.indent = state.x;
        }
    }
    sys.text.states[index].last_was_name = true;
}

/// Replaces `＊Ａ`, `％Ａ` (and `＊ＡＢ`, and a trailing full-width digit
/// selecting one character) with name variables.
pub fn interpret_names(text: &str, name: impl Fn(bool, usize) -> String) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let letter = |c: char| ('Ａ'..='Ｚ').contains(&c);
    while i < chars.len() {
        let c = chars[i];
        if (c == '＊' || c == '％') && chars.get(i + 1).copied().is_some_and(letter) {
            let mut letters = String::from(chars[i + 1]);
            let mut j = i + 2;
            if chars.get(j).copied().is_some_and(letter) {
                letters.push(chars[j]);
                j += 1;
            }
            let value = crate::memory::Memory::name_index(&letters)
                .map(|index| name(c == '％', index))
                .unwrap_or_default();
            if let Some(digit) = chars.get(j).filter(|d| ('０'..='９').contains(d)) {
                let nth = *digit as usize - '０' as usize;
                out.extend(value.chars().nth(nth));
                j += 1;
            } else {
                out.push_str(&value);
            }
            i = j;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// Opens a window if it is closed (with its opening animation).
pub fn open_window(sys: &mut System, index: usize) {
    if sys.text.states[index].visible {
        return;
    }
    let config = &sys.text.windows[index];
    let now = sys.clock.now();
    let animate = config.anm_enabled && config.open_anm_mod != 0 && config.open_anm_time > 0;
    let state = &mut sys.text.states[index];
    state.visible = true;
    state.animation = animate.then_some((now, true));
}

/// Closes a window (with its closing animation).
pub fn close_window(sys: &mut System, index: usize) {
    if !sys.text.states[index].visible {
        return;
    }
    let config = &sys.text.windows[index];
    let now = sys.clock.now();
    let animate = config.anm_enabled && config.close_anm_mod != 0 && config.close_anm_time > 0;
    let luby = config.luby_size;
    if index == sys.text.active {
        sys.text.commit_page();
    }
    let state = &mut sys.text.states[index];
    if animate {
        state.animation = Some((now, false));
    } else {
        state.visible = false;
        state.animation = None;
    }
    state.chars.clear();
    state.rubies.clear();
    state.namebox = None;
    state.x = 0;
    state.y = luby;
    state.indent = 0;
    state.line = 0;
}

pub fn close_all_except(sys: &mut System, keep: usize) {
    for index in 0..sys.text.states.len() {
        if index != keep {
            close_window(sys, index);
        }
    }
}

/// What happens when a pause is dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseKind {
    /// `pause()`: page or paragraph depending on `R_COMMAND_MOD`.
    Pause,
    /// `pause_all()`: as pause, never closing other windows.
    PauseAll,
    /// `page()`: clear the window and close the others.
    Page,
    /// `spause()`: continue in place.
    Spause,
    /// Automatic pause when a window fills up: continue on a new page.
    PageFull,
}

fn finish_pause(sys: &mut System, kind: PauseKind) {
    let index = sys.text.active;
    let r_command = sys.text.windows[index].r_command_mod != 0;
    {
        let state = &mut sys.text.states[index];
        state.colour_override = None;
        state.size_override = None;
        state.chars_since_pause = 0;
    }
    sys.text.speed_override = None;
    match kind {
        PauseKind::Spause => {}
        PauseKind::PageFull => {
            // Continue the same speech on a fresh page, keeping the name.
            let name = sys.text.states[index].name.clone();
            let namebox = sys.text.states[index].namebox.clone();
            let indent = sys.text.states[index].indent;
            clear_window(sys, index);
            let state = &mut sys.text.states[index];
            state.name = name;
            state.namebox = namebox;
            state.indent = indent;
            state.x = indent;
        }
        PauseKind::Pause | PauseKind::PauseAll if r_command => {
            // The next paragraph names its own speaker.
            sys.text.states[index].namebox = None;
            line_break(sys, index);
            sys.text.states[index].indent = 0;
            sys.text.states[index].x = 0;
        }
        PauseKind::PauseAll => clear_window(sys, index),
        PauseKind::Pause | PauseKind::Page => {
            clear_window(sys, index);
            close_all_except(sys, index);
        }
    }
}

// ---- long operations ---------------------------------------------------------

/// Displays text one character at a time.
#[derive(Debug)]
pub struct TextoutOp {
    chars: Vec<char>,
    at: usize,
    last: u64,
    budget: f64,
    instant: bool,
}

impl TextoutOp {
    pub fn new(machine: &Machine, text: &str) -> Self {
        let sys = &machine.sys;
        let instant = sys.text.fast_text
            || sys.settings.message_no_wait
            || sys.should_fast_forward()
            || sys
                .text
                .speed_override
                .unwrap_or(sys.settings.message_speed)
                <= 0;
        Self {
            chars: text.chars().collect(),
            at: 0,
            last: sys.now(),
            budget: 1.0,
            instant,
        }
    }

    /// Displays up to `count` characters; returns true when all are shown,
    /// false when stopped (window full → a pause was pushed).
    fn display(&mut self, machine: &mut Machine, mut count: usize) -> bool {
        let index = machine.sys.text.active;
        while self.at < self.chars.len() && count > 0 {
            let c = self.chars[self.at];
            if c == '【' {
                if let Some(end) = self.chars[self.at..].iter().position(|&c| c == '】') {
                    let name: String = self.chars[self.at + 1..self.at + end].iter().collect();
                    let next = self.chars.get(self.at + end + 1).copied();
                    set_name(&mut machine.sys, index, &name, next);
                    self.at += end + 1;
                    continue;
                }
            }
            if !place_char(&mut machine.sys, index, c, &self.chars[self.at + 1..]) {
                machine.push_long_op(Box::new(PauseOp::new(machine, PauseKind::PageFull)));
                return false;
            }
            self.at += 1;
            count -= 1;
        }
        self.at >= self.chars.len()
    }
}

impl LongOp for TextoutOp {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let active = machine.sys.text.active;
        open_window(&mut machine.sys, active);
        // A click shows the rest of the text at once.
        if machine.sys.input.take_press(Button::Left) || machine.sys.should_fast_forward() {
            self.instant = true;
        }
        if self.instant {
            return Ok(self.display(machine, usize::MAX));
        }
        let now = machine.sys.now();
        let sys = &machine.sys;
        let speed = f64::from(
            sys.text
                .speed_override
                .unwrap_or(sys.settings.message_speed)
                .max(1),
        );
        self.budget += (now - self.last) as f64 / speed;
        self.last = now;
        let count = self.budget.floor() as usize;
        self.budget -= count as f64;
        Ok(self.display(machine, count))
    }

    fn name(&self) -> &'static str {
        "textout"
    }
}

/// Waits for the player at the end of a paragraph or page.
#[derive(Debug)]
pub struct PauseOp {
    kind: PauseKind,
    start: u64,
    skip_start: Option<u64>,
}

impl PauseOp {
    pub fn new(machine: &Machine, kind: PauseKind) -> Self {
        Self {
            kind,
            start: machine.sys.now(),
            skip_start: None,
        }
    }
}

impl LongOp for PauseOp {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let now = machine.sys.now();
        machine.sys.text.waiting_since.get_or_insert(self.start);
        let sys = &mut machine.sys;
        // Hidden windows: any click shows them again.
        if sys.text.hidden_temporarily {
            if sys.input.take_click().is_some() || sys.input.take_key(Key::Space) {
                sys.text.hidden_temporarily = false;
            }
            return Ok(false);
        }
        // The backlog, when open, takes the input.
        if sys.text.backlog_view.is_some() {
            crate::backlog::handle_input(sys);
            return Ok(false);
        }
        // A click on a window button does what the button does.
        if let Some(position) = sys
            .input
            .events
            .iter()
            .position(|e| *e == InputEvent::Press(Button::Left))
            && let Some(kind) = crate::window_buttons::hit(sys)
        {
            sys.input.events.remove(position);
            crate::window_buttons::press(machine, kind)?;
            return Ok(false);
        }
        let sys = &mut machine.sys;
        let mut done = false;
        let events = std::mem::take(&mut sys.input.events);
        let mut rest = Vec::new();
        for event in events {
            match event {
                InputEvent::Press(Button::Left) | InputEvent::KeyDown(Key::Enter) => done = true,
                InputEvent::Press(Button::WheelDown) | InputEvent::KeyDown(Key::PageDown) => {
                    done = true
                }
                InputEvent::Press(Button::WheelUp) | InputEvent::KeyDown(Key::PageUp) => {
                    crate::backlog::open(sys);
                }
                InputEvent::KeyDown(Key::Space) => sys.text.hidden_temporarily = true,
                other => rest.push(other),
            }
        }
        sys.input.events = rest;
        // Right click / Escape: the menu.
        if !done && sys.input.take_press(Button::Right) {
            if crate::modules::menu::invoke_cancel_call(machine)? {
                return Ok(false);
            }
            crate::modules::menu::open_context_menu(machine)?;
            return Ok(false);
        }
        let sys = &mut machine.sys;
        if sys.should_fast_forward() {
            done = true;
        }
        // Skip mode fast-forwards through text already read.
        if sys.syscom.skip_mode && sys.text.kidoku_read() {
            done = true;
        }
        if sys.settings.auto_mode {
            let chars = sys.text.states[sys.text.active].chars_since_pause;
            let wait = sys.settings.auto_base_time + sys.settings.auto_char_time * chars;
            let voice_busy = sys.voice_playing();
            if !voice_busy && now >= self.start + wait.max(0) as u64 {
                done = true;
            }
        }
        let _ = self.skip_start;
        if done {
            sys.text.waiting_since = None;
            finish_pause(sys, self.kind);
        }
        Ok(done)
    }

    fn name(&self) -> &'static str {
        "pause"
    }
}

// ---- rendering -----------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_glyph(
    frame: &mut Surface,
    sys: &mut System,
    c: char,
    x: i32,
    y: i32,
    size: i32,
    colour: [u8; 3],
    shadow: Option<[u8; 3]>,
    alpha: u8,
) {
    if c == ' ' || c == '\u{3000}' {
        return;
    }
    let bold = sys.settings.font_weight != 0;
    let ascent = sys.gfx.fonts.ascent(size as u32).round() as i32;
    let Some(glyph) = sys.gfx.fonts.glyph(c, size as u32, bold) else {
        return;
    };
    let passes: &[(i32, [u8; 3])] = match shadow {
        Some(shadow) => &[(1, shadow), (0, colour)],
        None => &[(0, colour)],
    };
    for &(offset, rgb) in passes {
        for gy in 0..glyph.height {
            for gx in 0..glyph.width {
                let coverage = glyph.coverage[gy * glyph.width + gx];
                if coverage == 0 {
                    continue;
                }
                let tx = x + glyph.left + gx as i32 + offset;
                let ty = y + ascent + glyph.top + gy as i32 + offset;
                if tx < 0 || ty < 0 || tx >= frame.width || ty >= frame.height {
                    continue;
                }
                let a = u32::from(coverage) * u32::from(alpha) / 255;
                let at = ((ty * frame.width + tx) * 4) as usize;
                for ch in 0..3 {
                    let d = u32::from(frame.rgba[at + ch]);
                    frame.rgba[at + ch] = ((d * (255 - a) + u32::from(rgb[ch]) * a) / 255) as u8;
                }
            }
        }
    }
}

/// Draws the translucent backing: filter 0 subtracts, 1 blends.
pub(crate) fn draw_backing(
    frame: &mut Surface,
    rect: Rect,
    mask: Option<&Surface>,
    attr: WindowAttr,
    alpha: u8,
) {
    let rect = rect.intersect(&frame.rect());
    let colour = [attr.r, attr.g, attr.b].map(|c| c.clamp(0, 255) as u32);
    let strength = attr.alpha.clamp(0, 255) as u32 * u32::from(alpha) / 255;
    for y in rect.y..rect.bottom() {
        for x in rect.x..rect.right() {
            let m = match mask {
                Some(mask) => u32::from(mask.pixel(x - rect.x, y - rect.y)[3]) * strength / 255,
                None => strength,
            };
            if m == 0 {
                continue;
            }
            let at = ((y * frame.width + x) * 4) as usize;
            for c in 0..3 {
                let d = u32::from(frame.rgba[at + c]);
                frame.rgba[at + c] = if attr.filter == 0 {
                    // Subtractive: darken by the mask, then add the colour.
                    (d.saturating_sub(m) + colour[c] * m / 255).min(255) as u8
                } else {
                    ((d * (255 - m) + colour[c] * m) / 255) as u8
                };
            }
        }
    }
}

pub(crate) fn draw_image(
    frame: &mut Surface,
    image: &Image,
    pattern: i32,
    x: i32,
    y: i32,
    alpha: u8,
) {
    let region = image.region(pattern);
    let src = Rect::from_corners(region.x1, region.y1, region.x2, region.y2);
    frame.blit(&image.surface, src, x, y, alpha, Blend::Mask, None);
}

/// Draws a type 4 frame: corners, stretched edges, stretched backing.
fn draw_type4(
    frame: &mut Surface,
    image: &Image,
    window: Rect,
    attr: WindowAttr,
    area: [i32; 4],
    alpha: u8,
) {
    // Type 4 frames carry their pieces as patterns; a single picture
    // (type 3) is cut into a 3 × 3 grid, corners a third of its height.
    let sliced = image.regions.len() < 12;
    let (w, h) = (image.surface.width, image.surface.height);
    let corner = (w.min(h) / 3).max(1);
    let r = |i: i32| {
        if !sliced {
            return image.region(i);
        }
        let (col, row) = (i % 3, i / 3);
        let xs = [0, corner, w - corner, w];
        let ys = [0, corner, h - corner, h];
        // Patterns 0..11 are rows of three: top, left/right edges (3, 6)
        // and bottom (9..11).
        let row = match row {
            0 => 0,
            3 => 2,
            _ => 1,
        };
        crate::image::Region {
            x1: xs[col as usize],
            y1: ys[row],
            x2: xs[col as usize + 1] - 1,
            y2: ys[row + 1] - 1,
            origin_x: 0,
            origin_y: 0,
        }
    };
    let (tl, tc, tr) = (r(0), r(1), r(2));
    let (ls, rs) = if sliced { (r(3), r(5)) } else { (r(3), r(6)) };
    let (bl, bc, br) = (r(9), r(10), r(11));
    let inner = Rect::new(
        window.x + ls.width() - area[2],
        window.y + tc.height() - area[0],
        window.w - ls.width() - rs.width() + area[2] + area[3],
        window.h - tc.height() - bc.height() + area[0] + area[1],
    );
    draw_backing(frame, inner, None, attr, alpha);
    let blit = |frame: &mut Surface, region: crate::image::Region, dest: Rect| {
        let src = Rect::new(region.x1, region.y1, region.width(), region.height());
        frame.stretch_blit(&image.surface, src, dest, alpha, Blend::Mask);
    };
    let (x, y, w, h) = (window.x, window.y, window.w, window.h);
    blit(frame, tl, Rect::new(x, y, tl.width(), tl.height()));
    blit(
        frame,
        tr,
        Rect::new(x + w - tr.width(), y, tr.width(), tr.height()),
    );
    blit(
        frame,
        bl,
        Rect::new(x, y + h - bl.height(), bl.width(), bl.height()),
    );
    blit(
        frame,
        br,
        Rect::new(
            x + w - br.width(),
            y + h - br.height(),
            br.width(),
            br.height(),
        ),
    );
    blit(
        frame,
        tc,
        Rect::new(x + tl.width(), y, w - tl.width() - tr.width(), tc.height()),
    );
    blit(
        frame,
        bc,
        Rect::new(
            x + bl.width(),
            y + h - bc.height(),
            w - bl.width() - br.width(),
            bc.height(),
        ),
    );
    blit(
        frame,
        ls,
        Rect::new(
            x,
            y + tl.height(),
            ls.width(),
            h - tl.height() - bl.height(),
        ),
    );
    blit(
        frame,
        rs,
        Rect::new(
            x + w - rs.width(),
            y + tr.height(),
            rs.width(),
            h - tr.height() - br.height(),
        ),
    );
}

/// Opening/closing animation: (alpha, dx, dy, scale). Returns `None` when
/// a closing animation has ended.
fn animation_state(
    sys: &mut System,
    index: usize,
    geometry: &Geometry,
) -> Option<(u8, i32, i32, f64)> {
    let now = sys.clock.now();
    let config = &sys.text.windows[index];
    let Some((start, opening)) = sys.text.states[index].animation else {
        return Some((255, 0, 0, 1.0));
    };
    let (mode, time) = if opening {
        (config.open_anm_mod, config.open_anm_time)
    } else {
        (config.close_anm_mod, config.close_anm_time)
    };
    let t = ((now - start) as f64 / f64::from(time.max(1))).clamp(0.0, 1.0);
    if t >= 1.0 {
        let state = &mut sys.text.states[index];
        state.animation = None;
        if !opening {
            state.visible = false;
            return None;
        }
        return Some((255, 0, 0, 1.0));
    }
    // Progress towards being fully shown.
    let shown = if opening { t } else { 1.0 - t };
    let (sw, sh) = (f64::from(sys.gfx.width), f64::from(sys.gfx.height));
    let w = geometry.window;
    let hidden_offset = |mode: i32| -> (f64, f64) {
        let up = -(f64::from(w.y + w.h));
        let down = sh - f64::from(w.y);
        let left = -(f64::from(w.x + w.w));
        let right = sw - f64::from(w.x);
        match mode {
            2 => (0.0, up),
            3 => (0.0, down),
            4 => (left, 0.0),
            5 => (right, 0.0),
            6 => (0.0, if -up < down { up } else { down }),
            7 => (if -left < right { left } else { right }, 0.0),
            _ => {
                let vertical = if -up < down { up } else { down };
                let horizontal = if -left < right { left } else { right };
                if vertical.abs() < horizontal.abs() {
                    (0.0, vertical)
                } else {
                    (horizontal, 0.0)
                }
            }
        }
    };
    Some(match mode {
        1 => ((shown * 255.0) as u8, 0, 0, 1.0),
        2..=8 => {
            let (hx, hy) = hidden_offset(mode);
            (
                255,
                (hx * (1.0 - shown)) as i32,
                (hy * (1.0 - shown)) as i32,
                1.0,
            )
        }
        9..=16 => (255, 0, 0, shown.max(0.01)),
        _ => (255, 0, 0, 1.0),
    })
}

/// Draws every visible text window onto `frame`.
pub fn draw_windows(
    sys: &mut System,
    frame: &mut Surface,
    window_offset: (i32, i32),
    text_offset: (i32, i32),
) {
    if sys.text.hidden_temporarily || sys.gfx.interface_hidden {
        return;
    }
    for index in 0..sys.text.states.len() {
        if !sys.text.states[index].visible || sys.text.display_off[index] {
            continue;
        }
        let geometry = geometry(sys, index);
        let Some((alpha, dx, dy, scale)) = animation_state(sys, index, &geometry) else {
            continue;
        };
        if scale < 1.0 {
            // Expanding windows: draw into a layer and scale it.
            let mut layer = frame.clone();
            draw_window(
                sys,
                &mut layer,
                index,
                &geometry,
                window_offset,
                text_offset,
                255,
            );
            let w = geometry.window;
            let dest = Rect::new(
                w.x + (f64::from(w.w) * (1.0 - scale) / 2.0) as i32,
                w.y + (f64::from(w.h) * (1.0 - scale) / 2.0) as i32,
                (f64::from(w.w) * scale).max(1.0) as i32,
                (f64::from(w.h) * scale).max(1.0) as i32,
            );
            frame.stretch_blit(&layer, w, dest, alpha, Blend::Copy);
        } else {
            let shifted = (window_offset.0 + dx, window_offset.1 + dy);
            let text_shift = (text_offset.0 + dx, text_offset.1 + dy);
            draw_window(sys, frame, index, &geometry, shifted, text_shift, alpha);
        }
    }
}

fn draw_window(
    sys: &mut System,
    frame: &mut Surface,
    index: usize,
    geometry: &Geometry,
    window_offset: (i32, i32),
    text_offset: (i32, i32),
    alpha: u8,
) {
    let config = sys.text.windows[index].clone();
    let attr = config.effective_attr(sys.settings.window_attr);
    let window = Rect::new(
        geometry.window.x + window_offset.0,
        geometry.window.y + window_offset.1,
        geometry.window.w,
        geometry.window.h,
    );
    let frame_images = waku(sys, &config, false);
    // Faces behind the window.
    draw_faces(sys, frame, index, geometry, true, alpha);
    if frame_images.type4 {
        if let Some(main) = &frame_images.main {
            draw_type4(frame, main, window, attr, frame_images.area, alpha);
        }
    } else {
        match &frame_images.back {
            Some(back) => draw_backing(
                frame,
                Rect::new(window.x, window.y, back.width(), back.height()),
                Some(&back.surface),
                attr,
                alpha,
            ),
            None if frame_images.main.is_none() => draw_backing(frame, window, None, attr, alpha),
            None => {}
        }
        if let Some(main) = &frame_images.main {
            draw_image(frame, main, 0, window.x, window.y, alpha);
        }
    }
    crate::window_buttons::draw(
        sys,
        frame,
        index,
        geometry,
        (window.x - geometry.window.x, window.y - geometry.window.y),
        alpha,
    );
    draw_faces(sys, frame, index, geometry, false, alpha);
    let origin = (
        geometry.text.x + text_offset.0,
        geometry.text.y + text_offset.1,
    );
    let chars = sys.text.states[index].chars.clone();
    for ch in &chars {
        draw_glyph(
            frame,
            sys,
            ch.c,
            origin.0 + ch.x,
            origin.1 + ch.y,
            ch.size,
            ch.colour,
            ch.shadow,
            alpha,
        );
    }
    crate::select::draw_window_items(sys, frame, index, origin, alpha);
    let rubies = sys.text.states[index].rubies.clone();
    for ruby in &rubies {
        draw_ruby(sys, frame, &config, ruby, origin, alpha);
    }
    if config.name_mod == 1 {
        if let Some(name) = sys.text.states[index].namebox.clone() {
            draw_name_box(sys, frame, &config, window, &name, alpha);
        }
    }
    if index == sys.text.active {
        if let Some(since) = sys.text.waiting_since {
            draw_key_cursor(sys, frame, &config, geometry, origin, since);
        }
    }
}

fn draw_ruby(
    sys: &mut System,
    frame: &mut Surface,
    config: &WindowConfig,
    ruby: &Ruby,
    origin: (i32, i32),
    alpha: u8,
) {
    let size = config.luby_size.max(1);
    let chars: Vec<char> = ruby.text.chars().collect();
    if chars.is_empty() {
        return;
    }
    let total: i32 = chars.iter().map(|&c| cell_width(c) as i32 * size / 2).sum();
    let span = ruby.x2 - ruby.x1;
    let gap = if chars.len() > 1 {
        ((span - total).max(0)) / chars.len() as i32
    } else {
        0
    };
    let mut x = ruby.x1 + (span - total - gap * (chars.len() as i32 - 1)).max(0) / 2;
    let colour = colour_index(sys, 0);
    for c in chars {
        draw_glyph(
            frame,
            sys,
            c,
            origin.0 + x,
            origin.1 + ruby.y - size,
            size,
            colour,
            Some([0, 0, 0]),
            alpha,
        );
        x += cell_width(c) as i32 * size / 2 + gap;
    }
}

fn draw_name_box(
    sys: &mut System,
    frame: &mut Surface,
    config: &WindowConfig,
    window: Rect,
    name: &str,
    alpha: u8,
) {
    let size = config.name_moji_size.max(1);
    let (pad_x, pad_y) = config.name_moji_pos;
    let text_width: i32 = name
        .chars()
        .map(|c| cell_width(c) as i32 * (size + config.name_moji_rep) / 2)
        .sum();
    let min_width = config.name_moji_min * (size + config.name_moji_rep);
    let inner = text_width.max(min_width);
    let box_images = waku(sys, config, true);
    let (bw, bh) = match (&box_images.main, &box_images.back) {
        (Some(main), _) => (main.width(), main.height()),
        (None, Some(back)) => (back.width(), back.height()),
        _ => (inner + pad_x * 2, size + pad_y * 2),
    };
    let x = if config.name_waku_dir != 0 {
        config.name_pos.0
    } else {
        window.x + config.name_pos.0
    };
    let y = window.y + config.name_pos.1 - bh;
    let attr = config.effective_attr(sys.settings.window_attr);
    match &box_images.back {
        Some(back) => draw_backing(
            frame,
            Rect::new(x, y, bw, bh),
            Some(&back.surface),
            attr,
            alpha,
        ),
        None if box_images.main.is_none() => {
            draw_backing(frame, Rect::new(x, y, bw, bh), None, attr, alpha)
        }
        None => {}
    }
    if let Some(main) = &box_images.main {
        draw_image(frame, main, 0, x, y, alpha);
    }
    let mut tx = if config.name_centering {
        x + (bw - text_width) / 2
    } else {
        x + pad_x
    };
    let ty = y + (bh - size) / 2;
    let colour = colour_index(sys, 0);
    for c in name.chars() {
        draw_glyph(frame, sys, c, tx, ty, size, colour, Some([0, 0, 0]), alpha);
        tx += cell_width(c) as i32 * (size + config.name_moji_rep) / 2;
    }
}

fn draw_faces(
    sys: &mut System,
    frame: &mut Surface,
    index: usize,
    geometry: &Geometry,
    behind: bool,
    alpha: u8,
) {
    let config = sys.text.windows[index].clone();
    let faces = sys.text.states[index].faces.clone();
    for (slot, file) in faces.iter().enumerate() {
        let (Some(file), Some(&(fx, fy, is_behind))) = (file, config.faces.get(slot)) else {
            continue;
        };
        if (is_behind != 0) != behind {
            continue;
        }
        if let Some(mut image) = load_named(sys, file) {
            if let Some(table) = sys.gfx.face_tone {
                image = sys.gfx.tone_curves.apply(&image, table);
            }
            draw_image(
                frame,
                &image,
                0,
                geometry.text.x + fx,
                geometry.text.y + fy,
                alpha,
            );
        }
    }
}

fn draw_key_cursor(
    sys: &mut System,
    frame: &mut Surface,
    config: &WindowConfig,
    geometry: &Geometry,
    origin: (i32, i32),
    since: u64,
) {
    let exe = sys.gameexe.clone();
    let cursor = sys.key_cursor;
    let key = |part: &str| format!("CURSOR.{cursor:03}.{part}");
    let Some(name) = exe.str(&key("NAME")).map(str::to_owned) else {
        return;
    };
    let Some(image) = load_named(sys, &name) else {
        return;
    };
    let size = exe.ints(&key("SIZE"));
    let (w, h) = (
        size.first().copied().unwrap_or(image.height()),
        size.get(1).copied().unwrap_or(image.height()),
    );
    let frames = exe.int(&key("CONT")).unwrap_or(1).max(1);
    let speed = exe.int(&key("SPEED")).unwrap_or(1000).max(1);
    let elapsed = sys.clock.now().saturating_sub(since) as i32;
    let current = (elapsed / (speed / frames).max(1)) % frames;
    let state = &sys.text.states[sys.text.active];
    let (x, y) = match config.keycur.0 {
        1 => (origin.0 + state.x, origin.1 + state.y),
        2 => (origin.0 + config.keycur.1, origin.1 + config.keycur.2),
        _ => (geometry.text.right() - w, geometry.text.bottom() - h),
    };
    frame.blit(
        &image.surface,
        Rect::new(current * w, 0, w, h),
        x,
        y,
        255,
        Blend::Mask,
        None,
    );
}

/// `grpTextout`: draws text into a DC.
pub fn draw_string_to_dc(
    sys: &mut System,
    dc: i32,
    x: i32,
    y: i32,
    text: &str,
    size: i32,
    colour: [u8; 4],
) -> Result<()> {
    let mut surface = (*sys.gfx.dc(dc)?).clone();
    let mut pen = x;
    for c in text.chars() {
        draw_glyph(
            &mut surface,
            sys,
            c,
            pen,
            y,
            size.max(1),
            [colour[0], colour[1], colour[2]],
            None,
            255,
        );
        pen += cell_width(c) as i32 * size.max(1) / 2;
    }
    sys.gfx.set_dc(dc, surface)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_interpreted() {
        let name = |local: bool, index: usize| match (local, index) {
            (false, 0) => "朋也".to_owned(),
            (false, 27) => "岡崎".to_owned(),
            (true, 2) => "渚".to_owned(),
            _ => String::new(),
        };
        assert_eq!(interpret_names("＊Ａ「％Ｃ」", name), "朋也「渚」");
        assert_eq!(interpret_names("＊ＡＢさん", name), "岡崎さん");
        assert_eq!(interpret_names("＊Ａ１", name), "也");
    }

    #[test]
    fn layout_wraps_and_squeezes_punctuation() {
        let exe = crate::gameexe::Gameexe::parse(
            "#WINDOW.000.MOJI_CNT=4,3\n#WINDOW.000.MOJI_SIZE=20\n#WINDOW.000.MOJI_REP=0,0\n",
        );
        let mut sys = System::new(
            Rc::new(exe),
            std::path::PathBuf::new(),
            crate::system::SystemOptions {
                virtual_clock: true,
                audio: false,
                persist: false,
                save_dir: None,
                nls: crate::nls::Nls::Sjis,
                fonts: false,
            },
        );
        let text: Vec<char> = "あいうえ。かき".chars().collect();
        for (i, &c) in text.iter().enumerate() {
            assert!(place_char(&mut sys, 0, c, &text[i + 1..]));
        }
        let chars = &sys.text.states[0].chars;
        // The full stop squeezes onto the first line.
        assert_eq!((chars[4].c, chars[4].y), ('。', 0));
        assert_eq!((chars[5].c, chars[5].x, chars[5].y), ('か', 0, 20));
    }

    #[test]
    fn names_indent_following_lines() {
        let exe = crate::gameexe::Gameexe::parse(
            "#WINDOW.000.MOJI_CNT=6,3\n#WINDOW.000.MOJI_SIZE=10\n#WINDOW.000.MOJI_REP=0,0\n",
        );
        let mut sys = System::new(
            Rc::new(exe),
            std::path::PathBuf::new(),
            crate::system::SystemOptions {
                virtual_clock: true,
                audio: false,
                persist: false,
                save_dir: None,
                nls: crate::nls::Nls::Sjis,
                fonts: false,
            },
        );
        set_name(&mut sys, 0, "名", Some('「'));
        let text: Vec<char> = "「あいうえおか".chars().collect();
        for (i, &c) in text.iter().enumerate() {
            place_char(&mut sys, 0, c, &text[i + 1..]);
        }
        let chars = &sys.text.states[0].chars;
        let wrapped = chars.iter().find(|p| p.y > 0).unwrap();
        // Indented to just after the opening quote (name + quote).
        assert_eq!(wrapped.x, 20);
    }
}
