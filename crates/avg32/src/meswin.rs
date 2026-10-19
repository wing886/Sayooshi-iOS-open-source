//! The AVG32 message window and choice list.
//!
//! Text is kept as bytes in the game's encoding ([`crate::nls`]), laid out
//! on a grid of half-width cells (`#MSG_MOJI_SIZE`), and drawn straight into
//! display buffer 0 over the window frame built from the `#WAKUPDT` picture.
//! Every multi-byte character occupies two cells whatever its byte length.
//! The screen behind the window lives in `BACKUPPDT` so hiding the window
//! restores it exactly.

use crate::buffer::PdtBuffer;
use crate::nls;
use crate::pdtmgr::{BACKUPPDT, EXFONTPDT, MESWINPDT, Rect, WAKUPDT};
use crate::system::System;

#[derive(Debug, Clone, Default)]
pub struct MesWin {
    pub buf: Vec<u8>,
    pub ptr: usize,
    redraw: bool,
    hankaku: bool,
    indent: i32,
    indent_flag: i32,
    pub cur_x: i32,
    pub cur_y: i32,
    cur_time: u64,
    icon: i32,
    icon_base: u64,
    saved_geometry: Option<[i32; 4]>,
    pub sub_window: bool,
    /// `MesWinFlag`: the window is on screen.
    pub shown: bool,
    icon_shown: bool,
    pub style: i32,
    pub style_force: i32,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub double_text: bool,
    double_line: bool,
}

impl MesWin {
    pub fn reset(&mut self) {
        self.buf.clear();
        self.ptr = 0;
        self.cur_x = 0;
        self.cur_y = 0;
        self.indent_flag = 0;
        self.indent = 0;
        self.icon_shown = false;
        self.shown = false;
        self.redraw = false;
        self.icon = 0;
        self.double_text = false;
        self.double_line = false;
    }

    fn at(&self, offset: usize) -> u8 {
        self.buf.get(self.ptr + offset).copied().unwrap_or(0)
    }

    /// The character at `ptr + offset` and its byte length.
    fn char_at(&self, offset: usize) -> (char, usize) {
        nls::char_at(self.buf.get(self.ptr + offset..).unwrap_or(&[]))
    }

    pub fn has_pending_text(&self) -> bool {
        self.at(0) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.ptr == 0
    }
}

#[derive(Debug, Clone, Default)]
struct SelectItem {
    text: Vec<u8>,
    enable: i32,
    colour: i32,
    id: i32,
}

#[derive(Debug, Clone)]
pub struct Selection {
    items: Vec<SelectItem>,
    drawn: bool,
    old: i32,
    finish: i32,
    finish_count: i32,
    finish_base: u64,
    cur_id: i32,
    key_select: bool,
    last_mouse: (i32, i32),
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            drawn: false,
            old: -1,
            finish: 0,
            finish_count: 0,
            finish_base: 0,
            cur_id: 0,
            key_select: false,
            last_mouse: (-1, -1),
        }
    }
}

const MAX_SELECT: usize = 24;

/// Characters of Shift-JIS row 1 (`0x81xx`): full-width punctuation and
/// symbols.
fn is_symbol_row(character: char) -> bool {
    nls::sjis_code(character).is_some_and(|code| code >> 8 == 0x81)
}

/// Punctuation may overhang the right margin; opening brackets may not.
fn hangs_at_line_end(character: char) -> bool {
    match nls::sjis_code(character) {
        Some(code) if code >> 8 == 0x81 => {
            let low = code as u8;
            !((0x65..0x7a).contains(&low) && low & 1 != 0)
        }
        _ => false,
    }
}

impl System {
    fn novel(&self) -> bool {
        self.game.ini.novel_mode != 0
    }

    fn text_colour(&self) -> [u8; 3] {
        let index = self.game.ini.font_color.clamp(0, 15) as usize;
        self.game.ini.color_table[index].map(|channel| channel.clamp(0, 255) as u8)
    }

    fn table_colour(&self, index: i32) -> [u8; 3] {
        self.game.ini.color_table[index.clamp(0, 15) as usize]
            .map(|channel| channel.clamp(0, 255) as u8)
    }

    fn shadow_colour(&self) -> Option<[u8; 3]> {
        (self.game.ini.value("MOJI_KAGE") != 0)
            .then(|| self.table_colour(self.game.ini.value("KAGE_COLOR")))
    }

    /// Rasterises one character into `buffer` with its baseline at
    /// `baseline`.
    pub(crate) fn draw_char(
        &mut self,
        buffer: usize,
        x: i32,
        baseline: i32,
        character: char,
        size: i32,
        colour: [u8; 3],
    ) {
        if character == ' ' || character == '\u{3000}' {
            return;
        }
        let Some(font) = self.font.as_mut() else {
            return;
        };
        let Some(glyph) = font.glyph(character, size.max(1) as u32) else {
            return;
        };
        self.gfx.draw_coverage(
            buffer,
            x + glyph.left,
            baseline + glyph.top,
            glyph.width,
            glyph.height,
            &glyph.coverage,
            colour,
        );
    }

    /// Draws a character with the configured shadow.
    fn draw_text_char(
        &mut self,
        x: i32,
        baseline: i32,
        character: char,
        size: i32,
        colour: [u8; 3],
    ) {
        if let Some(shadow) = self.shadow_colour() {
            self.draw_char(0, x + 1, baseline + 1, character, size, shadow);
        }
        self.draw_char(0, x, baseline, character, size, colour);
    }

    fn window_rect(&self) -> Rect {
        Rect::new(self.mes.x1, self.mes.y1, self.mes.x2, self.mes.y2)
    }

    fn ensure_waku(&mut self) {
        if self.gfx.get(WAKUPDT).is_some() {
            return;
        }
        let name = self.game.ini.waku_file.clone();
        let image = if name.is_empty() {
            None
        } else {
            self.load_image(&name)
        };
        let mut buffer = PdtBuffer::screen();
        match image {
            Some(image) => buffer.copy_image(&image),
            None => {
                // No frame picture: a plain window whose whole area is the
                // (tinted) key colour.
                buffer.rgb.fill(0);
            }
        }
        self.gfx.set(WAKUPDT, Some(buffer));
    }

    fn waku_key(&self) -> [u8; 3] {
        self.gfx
            .get(WAKUPDT)
            .map_or([0, 0, 0], |waku| waku.pixel(0))
    }

    /// Rebuilds the window picture in `MESWINPDT`.
    pub fn mes_setup(&mut self, style: i32) {
        let redraw = self.mes.shown;
        let redraw_icon = self.mes.icon_shown;
        self.mes.style = style;
        let had_waku = self.gfx.get(WAKUPDT).is_some();
        if had_waku && !self.novel() {
            self.mes_hide();
        }
        self.ensure_waku();
        if self.novel() {
            if redraw {
                self.gfx.copy(Rect::full(), MESWINPDT as i32, 0, 0, 0, 0);
                if self.sel.drawn {
                    self.sel.drawn = false;
                } else {
                    self.mes_reset_pos();
                    self.mes_print_all();
                }
            } else {
                self.mes.shown = false;
                self.mes.icon_shown = false;
            }
            return;
        }
        let mut pattern = style - 1;
        if self.mes.style_force != 0 {
            pattern = self.mes.style_force;
        }
        let pattern = pattern.clamp(0, 3);
        self.gfx.set(MESWINPDT, Some(PdtBuffer::screen()));
        self.gfx
            .fill_rect(Rect::full(), MESWINPDT as i32, [255, 255, 255]);
        let ini = &self.game.ini;
        let (font_x, font_y) = (ini.font_x, ini.font_y);
        let columns = if self.mes.sub_window {
            ini.mes_x
        } else {
            ini.mes_x + 1
        };
        let mut max_x = ini.win_x + ((columns * font_x * 2 + 31) & 0xfff0);
        let mut max_y = ini.win_y + ((ini.mes_y * font_y + 31) & 0xfff0);
        let (mut x1, mut y1) = (ini.win_x, ini.win_y);
        if max_x > 639 {
            x1 -= max_x - 640;
            max_x = 640;
        }
        if max_y > 479 {
            y1 -= max_y - 480;
            max_y = 480;
        }
        self.mes.x1 = x1;
        self.mes.y1 = y1;
        self.mes.x2 = max_x - 1;
        self.mes.y2 = max_y - 1;
        let base = pattern * 100;
        let waku = WAKUPDT as i32;
        let target = MESWINPDT as i32;
        let copy = |system: &mut Self, sx1: i32, sy1: i32, sx2: i32, sy2: i32, dx: i32, dy: i32| {
            system
                .gfx
                .copy(Rect::new(sx1, sy1, sx2, sy2), waku, dx, dy, target, 0);
        };
        copy(self, 8, 8 + base, 39, 31 + base, x1, y1);
        copy(self, 8, 48 + base, 39, 71 + base, x1, max_y - 24);
        for x in (x1 + 32)..(max_x - 32) {
            copy(self, 40, 8 + base, 40, 31 + base, x, y1);
            copy(self, 40, 48 + base, 40, 71 + base, x, max_y - 24);
        }
        let right = x1 + 32 + (max_x - x1 - 64).max(0);
        copy(self, 88, 8 + base, 119, 31 + base, right, y1);
        copy(self, 88, 48 + base, 119, 71 + base, right, max_y - 24);
        for y in (y1 + 24)..(max_y - 24) {
            copy(self, 8, 32 + base, 39, 32 + base, x1, y);
            copy(self, 88, 32 + base, 119, 32 + base, max_x - 32, y);
        }
        let key = self.waku_key();
        self.gfx.fill_rect(
            Rect::new(x1 + 8, y1 + 8, max_x - 8, max_y - 8),
            target,
            key.map(i32::from),
        );
        let area = self.game.ini.win_attr_area;
        let tint = self
            .game
            .ini
            .win_color
            .map(|channel| channel.clamp(0, 255) as u8);
        let opaque = self.game.ini.win_color_flag != 0;
        let window = self.gfx.get_mut(MESWINPDT).expect("created above");
        window.mask.fill(0);
        for y in y1.max(0)..max_y.min(480) {
            for x in x1.max(0)..max_x.min(640) {
                let at = window.at(x, y);
                if window.pixel(at) == key {
                    if x >= x1 + area[0]
                        && y >= y1 + area[1]
                        && x < max_x - area[2]
                        && y < max_y - area[3]
                    {
                        window.set_pixel(at, tint);
                        window.mask[at] = if opaque { 255 } else { 128 };
                    }
                } else {
                    window.mask[at] = 255;
                }
            }
        }
        if redraw {
            self.mes_draw();
            if self.sel.drawn {
                self.sel.drawn = false;
            } else {
                self.mes_reset_pos();
                self.mes_print_all();
                if redraw_icon {
                    self.mes_draw_icon(0);
                }
            }
        } else {
            self.mes.shown = false;
            self.mes.icon_shown = false;
        }
    }

    /// Composites the window over the saved
    /// background (translucent parts *multiply* the background).
    pub fn mes_draw_waku(&mut self) {
        let rect = self.window_rect();
        let (Some(window), Some(backup)) = (
            self.gfx.get(MESWINPDT).cloned(),
            self.gfx.get(BACKUPPDT).cloned(),
        ) else {
            return;
        };
        let display = self.gfx.display_mut();
        for y in rect.y1.max(0)..=rect.y2.min(479) {
            for x in rect.x1.max(0)..=rect.x2.min(639) {
                let at = display.at(x, y);
                let mask = window.mask[at];
                let back = backup.pixel(at);
                let colour = match mask {
                    0 => back,
                    255 => window.pixel(at),
                    _ => {
                        let front = window.pixel(at);
                        std::array::from_fn(|channel| {
                            ((u32::from(front[channel]) * u32::from(back[channel])) >> 8) as u8
                        })
                    }
                };
                display.set_pixel(at, colour);
            }
        }
        self.gfx.present(rect);
    }

    /// Shows the window.
    pub fn mes_draw(&mut self) {
        if self.mes.shown {
            return;
        }
        self.play_se(2);
        self.mes.shown = true;
        self.mes.icon_shown = false;
        if self.novel() {
            self.gfx.all_copy(0, BACKUPPDT as i32, 0);
            self.gfx.all_copy(BACKUPPDT as i32, MESWINPDT as i32, 0);
            let tint = self.game.ini.win_color;
            self.gfx.color_mask(Rect::full(), MESWINPDT as i32, tint);
            self.window_effect = crate::effect::Effect {
                sx2: 639,
                sy2: 479,
                cmd: 4,
                srcpdt: MESWINPDT as i32,
                dstpdt: 0,
                steptime: self.game.ini.default_fade_time.max(0) as u32,
                prevtime: self.now(),
                ..Default::default()
            };
        } else {
            let rect = self.window_rect();
            self.gfx
                .copy(rect, 0, rect.x1, rect.y1, BACKUPPDT as i32, 0);
            self.mes_draw_waku();
        }
    }

    /// Hides the window.
    pub fn mes_hide(&mut self) {
        if !self.mes.shown {
            return;
        }
        self.mes.shown = false;
        self.mes.icon_shown = false;
        if self.novel() {
            self.window_effect = crate::effect::Effect {
                sx2: 639,
                sy2: 479,
                cmd: 4,
                srcpdt: BACKUPPDT as i32,
                dstpdt: 0,
                steptime: self.game.ini.default_fade_time.max(0) as u32,
                prevtime: self.now(),
                ..Default::default()
            };
        } else {
            let rect = self.window_rect();
            self.gfx
                .copy(rect, BACKUPPDT as i32, rect.x1, rect.y1, 0, 0);
        }
    }

    /// The player's "hide window" toggle.
    pub fn mes_hide_temp(&mut self) {
        if !self.mes.shown {
            return;
        }
        if self.novel() {
            self.gfx.all_copy(BACKUPPDT as i32, 0, 0);
        } else {
            let rect = self.window_rect();
            self.gfx
                .copy(rect, BACKUPPDT as i32, rect.x1, rect.y1, 0, 0);
        }
    }

    /// Clears the current message.
    pub fn mes_clear(&mut self) {
        let page = std::mem::take(&mut self.mes.buf);
        self.record_backlog(&page);
        self.mes.buf.clear();
        self.mes.ptr = 0;
        self.mes.cur_x = 0;
        self.mes.cur_y = 0;
        self.mes.indent_flag = 0;
        self.mes.indent = 0;
        self.mes.icon_shown = false;
        self.mes.double_line = false;
        self.redraw_window_background();
    }

    fn redraw_window_background(&mut self) {
        if self.mes.shown {
            if self.novel() {
                self.gfx.copy(Rect::full(), MESWINPDT as i32, 0, 0, 0, 0);
            } else {
                self.mes_draw_waku();
            }
        }
    }

    /// A page break inside one message.
    pub fn mes_clear_window(&mut self) {
        let shown = self.mes.buf[..self.mes.ptr.min(self.mes.buf.len())].to_vec();
        self.record_backlog(&shown);
        let rest = self.mes.buf[self.mes.ptr.min(self.mes.buf.len())..].to_vec();
        self.mes.buf = rest;
        self.mes.ptr = 0;
        self.mes.cur_x = self.mes.indent;
        self.mes.cur_y = 0;
        self.mes.icon_shown = false;
        self.mes.double_line = false;
        self.redraw_window_background();
    }

    /// Appends one text packet (`0xFE`
    /// half-width, `0xFF` full-width, or `0x0D` newline) with `＊Ａ`-style
    /// name and `＊００` external-glyph substitution.
    pub fn mes_set(&mut self, packet: &[u8]) {
        self.mouse.start_pdt_draw();
        let Some((&kind, text)) = packet.split_first() else {
            return;
        };
        let text = &text[..text
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(text.len())];
        self.mes.buf.push(kind);
        match kind {
            0xfe => self.mes.buf.extend_from_slice(text),
            0xff => {
                let rest = |at: usize| text.get(at..).unwrap_or(&[]);
                let mut at = 0;
                while at < text.len() {
                    let (character, length) = nls::char_at(rest(at));
                    let length = length.max(1);
                    if character == '＊' {
                        // `＊` + full-width letter/digit.
                        let (code, code_length) = nls::char_at(rest(at + length));
                        if ('０'..='９').contains(&code) {
                            let (ones, ones_length) = nls::char_at(rest(at + length + code_length));
                            let tens = code as i32 - '０' as i32;
                            let ones = if ('０'..='９').contains(&ones) {
                                ones as i32 - '０' as i32
                            } else {
                                0
                            };
                            self.mes.buf.extend_from_slice(&text[at..at + length]);
                            self.mes.buf.push((tens * 10 + ones).clamp(0, 255) as u8);
                            at += length + code_length + ones_length;
                        } else {
                            let name = self.name(code as i32 - 'Ａ' as i32).to_vec();
                            self.mes.buf.extend_from_slice(&name);
                            at += length + code_length;
                        }
                    } else {
                        self.mes
                            .buf
                            .extend_from_slice(&text[at..(at + length).min(text.len())]);
                        at += length;
                    }
                }
            }
            _ => {}
        }
        self.mes.cur_time = self.now();
    }

    pub fn mes_reset_pos(&mut self) {
        self.mes.ptr = 0;
        self.mes.cur_x = 0;
        self.mes.cur_y = 0;
        self.mes.indent_flag = 0;
        self.mes.indent = 0;
        self.mes.double_text = false;
        self.mes.double_line = false;
    }

    fn new_line(&mut self, to: i32) {
        self.mes.cur_x = to;
        self.mes.cur_y += 1;
        if self.mes.double_line {
            self.mes.cur_y += 1;
            self.mes.double_line = false;
        }
    }

    /// Lays out and draws the next character,
    /// applying the bracket-indent rules (`【name】「…」`).
    fn mes_put_char(&mut self) {
        if self.novel() {
            self.mes_put_char_novel();
            return;
        }
        let mut special_indent = 0;
        loop {
            match self.mes.at(0) {
                0 => return,
                0x0d => {
                    self.mes.ptr += 1;
                    let indent = self.mes.indent;
                    self.new_line(indent);
                    return;
                }
                0xfe => {
                    self.mes.hankaku = true;
                    self.mes.ptr += 1;
                    continue;
                }
                0xff => {
                    self.mes.hankaku = false;
                    self.mes.ptr += 1;
                    continue;
                }
                _ => {}
            }
            let (character, length) = self.mes.char_at(0);
            match character {
                '【' => {
                    self.mes.indent_flag = 0;
                    self.mes.indent = 0;
                    self.mes.cur_x = 0;
                    self.mes.ptr += length;
                }
                '】' => {
                    // Following full-width spaces set the indent.
                    if self.mes.char_at(length).0 == '\u{3000}' {
                        self.mes.indent = self.mes.cur_x;
                        self.mes.indent_flag = 1;
                        let mut step = length;
                        while self.mes.char_at(step).0 == '\u{3000}' {
                            let space = self.mes.char_at(step).1;
                            self.mes.ptr += step;
                            self.mes.indent += 2;
                            step = space;
                        }
                    } else {
                        self.mes.indent_flag = 0;
                        self.mes.ptr += length;
                    }
                }
                '「' | '『' => {
                    if self.mes.indent_flag == 0 {
                        self.mes.indent = self.mes.cur_x + 2;
                    }
                    self.mes.indent_flag += 1;
                    break;
                }
                '」' | '』' => {
                    self.mes.indent_flag -= 1;
                    if self.mes.indent_flag <= 0 {
                        special_indent = self.mes.indent;
                        self.mes.indent = 0;
                    }
                    break;
                }
                _ => {
                    if self.mes.indent_flag == 0 {
                        self.mes.indent_flag = 99;
                    }
                    break;
                }
            }
        }
        let ini = &self.game.ini;
        let (mes_x, mes_y, font_x, font_y, font_size) =
            (ini.mes_x, ini.mes_y, ini.font_x, ini.font_y, ini.font_size);
        if self.mes.cur_x >= mes_x * 2 - 2 {
            let wrap = self.mes.cur_x >= mes_x * 2 || !hangs_at_line_end(self.mes.char_at(0).0);
            if wrap {
                let to = if special_indent != 0 {
                    special_indent
                } else {
                    self.mes.indent
                };
                self.new_line(to);
                if self.mes.cur_y == mes_y {
                    return;
                }
            }
        }
        let (x1, y1, x2, y2) = (self.mes.x1, self.mes.y1, self.mes.x2, self.mes.y2);
        let mut x = self.mes.cur_x * font_x
            + x1
            + (font_size - font_x * 2) / 2
            + (x2 - x1 - font_x * mes_x * 2 + font_x) / 2;
        let mut y = y1 + self.mes.cur_y * font_y + font_size + (y2 - y1 - font_y * mes_y) / 2;
        let mut size = font_size;
        if self.mes.double_text {
            size = font_size * 2;
            y += font_y;
            self.mes.double_line = true;
        }
        let colour = self.text_colour();
        let (character, length) = self.mes.char_at(0);
        if self.mes.hankaku && length == 1 {
            self.draw_text_char(x, y, character, size, colour);
            self.mes.ptr += 1;
            self.mes.cur_x += if self.mes.double_text { 2 } else { 1 };
        } else if character == '＊' {
            x = self.mes.cur_x * font_x
                + x1
                + (font_size - font_x * 2) / 2
                + (x2 - x1 - font_x * mes_x * 2 + font_x) / 2;
            y = y1 + self.mes.cur_y * font_y + (y2 - y1 - font_y * mes_y) / 2;
            let glyph = i32::from(self.mes.at(length));
            self.put_exfont(x, y, glyph);
            self.mes.ptr += length + 1;
            self.mes.cur_x += 2;
        } else {
            self.draw_text_char(x, y, character, size, colour);
            self.mes.ptr += length.max(1);
            self.mes.cur_x += if self.mes.double_text { 4 } else { 2 };
        }
    }

    /// Novel-mode variant of `mes_put_char`.
    fn mes_put_char_novel(&mut self) {
        let ini = &self.game.ini;
        let (mes_x, mes_y, font_x, font_y, win_x, win_y) = (
            ini.mes_x, ini.mes_y, ini.font_x, ini.font_y, ini.win_x, ini.win_y,
        );
        if self.mes.cur_y >= mes_y {
            return;
        }
        loop {
            match self.mes.at(0) {
                0 => return,
                0x0d => {
                    self.mes.ptr += 1;
                    self.mes.cur_x = self.mes.indent;
                    self.mes.cur_y += 1;
                    return;
                }
                0xff | 0xfe => {
                    self.mes.hankaku = false;
                    self.mes.ptr += 1;
                    continue;
                }
                _ => {}
            }
            let (character, length) = self.mes.char_at(0);
            match character {
                '【' => {
                    self.mes.indent_flag = 0;
                    self.mes.indent = 0;
                    self.mes.cur_x = 0;
                    self.mes.ptr += length;
                }
                '】' => {
                    self.mes.indent_flag = 1000;
                    self.mes.ptr += length;
                }
                '「' | '『' => {
                    if self.mes.indent_flag == 1000 {
                        self.mes.indent = self.mes.cur_x;
                    }
                    self.mes.indent_flag += 1;
                    break;
                }
                '」' | '』' => {
                    self.mes.indent_flag -= 1;
                    if self.mes.indent_flag <= 0 {
                        self.mes.indent = 0;
                    } else if self.mes.indent_flag == 1000 {
                        self.mes.indent = 0;
                        self.mes.indent_flag = 0;
                    }
                    break;
                }
                _ => {
                    if self.mes.indent_flag == 0 {
                        self.mes.indent_flag = 99;
                    }
                    break;
                }
            }
        }
        let (character, length) = self.mes.char_at(0);
        if self.mes.cur_x >= mes_x * 2 - 2 {
            let next = self.mes.char_at(length).0;
            let wrap =
                self.mes.cur_x >= mes_x * 2 || !hangs_at_line_end(character) || is_symbol_row(next);
            if wrap {
                self.mes.cur_x = self.mes.indent;
                self.mes.cur_y += 1;
                if self.mes.cur_y >= mes_y {
                    return;
                }
            }
        }
        let x = self.mes.cur_x * font_x + win_x;
        let y = self.mes.cur_y * font_y + win_y;
        let code = nls::current().novel_code(self.mes.buf.get(self.mes.ptr..).unwrap_or(&[]));
        self.put_novel_char(x, y, character, code);
        self.mes.ptr += length.max(1);
        self.mes.cur_x += 2;
        if self.mes.at(0) == 0x0d {
            self.mes.cur_x = 0;
            self.mes.cur_y += 1;
            self.mes.ptr += 1;
            self.mes.indent_flag = 0;
            self.mes.indent = 0;
        }
    }

    /// A 24x24 `FN.DAT` glyph with a
    /// two-pixel drop shadow.
    fn put_novel_char(&mut self, x: i32, y: i32, character: char, code: Option<u16>) {
        let colour = self.text_colour().map(|channel| i32::from(channel) + 1);
        let glyph = code.and_then(|code| self.novel_font.as_ref()?.glyph(code));
        let glyph = match glyph {
            Some(glyph) => glyph.to_vec(),
            None => {
                // No FN.DAT, or a character it lacks: use the outline font.
                let size = self.game.ini.font_size;
                let colour = self.text_colour();
                self.draw_char(0, x + 2, y + size + 2, character, size, [0, 0, 0]);
                self.draw_char(0, x, y + size, character, size, colour);
                return;
            }
        };
        let display = self.gfx.display_mut();
        for pass in 0..2 {
            let (ox, oy) = if pass == 0 { (2, 2) } else { (0, 0) };
            for row in 0..24 {
                for column in 0..24 {
                    let byte = glyph[row * 12 + column / 2];
                    let nibble = if column & 1 == 0 {
                        byte & 0x0f
                    } else {
                        byte >> 4
                    };
                    let value = i32::from(nibble | (nibble << 4));
                    let (px, py) = (x + ox + column as i32, y + oy + row as i32);
                    if !display.contains(px, py) {
                        continue;
                    }
                    let at = display.at(px, py);
                    let mut pixel = display.pixel(at).map(i32::from);
                    if pass == 0 {
                        let level = 256 - value;
                        for channel in &mut pixel {
                            *channel -= (*channel * level) >> 8;
                        }
                    } else {
                        let level = 255 - value;
                        for (channel, target) in pixel.iter_mut().zip(colour) {
                            *channel += ((target - *channel) * level) >> 8;
                        }
                    }
                    display.set_pixel(at, pixel.map(|channel| channel.clamp(0, 255) as u8));
                }
            }
        }
        let size = (self.game.ini.font_x * 2, self.game.ini.font_y);
        self.gfx
            .present(Rect::new(x, y, x + size.0 + 1, y + size.1 + 1));
    }

    /// An external glyph from `#EXFONT_N_NAME`.
    fn put_exfont(&mut self, x: i32, y: i32, index: i32) {
        let ini = &self.game.ini;
        let (width, height, columns, size) = (
            ini.exfont_x,
            ini.exfont_y,
            ini.exfont_max_x.max(1),
            ini.font_size,
        );
        let colour = self.text_colour().map(i32::from);
        let Some(font) = self.gfx.get(EXFONTPDT).cloned() else {
            return;
        };
        let (gx, gy) = ((index % columns) * width, (index / columns) * height);
        let display = self.gfx.display_mut();
        for pass in 0..2 {
            let (ox, oy, w, h) = if pass == 0 {
                (2, 2, size, size)
            } else {
                (1, 1, width, height)
            };
            for row in 0..h {
                for column in 0..w {
                    let (sx, sy) = (gx + column, gy + row);
                    let (px, py) = (x + ox + column, y + oy + row);
                    if !font.contains(sx, sy) || !display.contains(px, py) {
                        continue;
                    }
                    let source = font.pixel(font.at(sx, sy)).map(i32::from);
                    if source == [0, 0, 0] {
                        continue;
                    }
                    let at = display.at(px, py);
                    let mut pixel = display.pixel(at).map(i32::from);
                    for channel in 0..3 {
                        let weight = source[2 - channel];
                        if pass == 0 {
                            pixel[channel] -= (pixel[channel] * weight) >> 8;
                        } else {
                            pixel[channel] +=
                                ((colour[channel] + 1 - pixel[channel]) * weight) >> 8;
                        }
                    }
                    display.set_pixel(at, pixel.map(|channel| channel.clamp(0, 255) as u8));
                }
            }
        }
    }

    /// 0 when done, -1 on a full page.
    pub fn mes_print_all(&mut self) -> i32 {
        if !self.mes.has_pending_text() {
            return 0;
        }
        if !self.mes.shown {
            self.mes_draw();
            if self.novel() {
                self.mes.redraw = true;
            }
            return 1;
        }
        if self.novel() && self.mes.redraw {
            self.mes.redraw = false;
            self.mes_reset_pos();
        }
        let mut result = 1;
        while self.mes.has_pending_text() {
            self.mes_put_char();
            if !self.mes.has_pending_text() {
                result = 0;
                break;
            }
            if self.mes.cur_y >= self.game.ini.mes_y {
                result = -1;
                break;
            }
        }
        let rect = self.window_rect();
        self.gfx.present(rect);
        result
    }

    /// One character per `#MSG_SPEED` tick.
    pub fn mes_print(&mut self) -> i32 {
        if self.game.ini.mes_wait == 0 {
            return self.mes_print_all();
        }
        if !self.mes.has_pending_text() {
            return 0;
        }
        let now = self.now();
        if now.saturating_sub(self.mes.cur_time) < self.game.ini.mes_wait as u64 {
            return 1;
        }
        self.mes.cur_time = now;
        if !self.mes.shown {
            self.mes_draw();
            if self.novel() {
                self.mes.redraw = true;
            }
            return 1;
        }
        if self.novel() && self.mes.redraw {
            self.mes.redraw = false;
            let saved = std::mem::take(&mut self.mes.buf);
            self.mes.buf = saved[..self.mes.ptr.min(saved.len())].to_vec();
            self.mes_reset_pos();
            self.mes_print_all();
            self.mes.buf = saved;
        }
        self.mes_put_char();
        let rect = self.window_rect();
        self.gfx.present(rect);
        if !self.mes.has_pending_text() {
            return 0;
        }
        if self.mes.cur_y >= self.game.ini.mes_y {
            return -1;
        }
        1
    }

    pub fn change_font_color(&mut self, colour: i32) {
        self.game.ini.font_color = colour;
    }

    pub fn change_window_style(&mut self, style: i32) {
        self.mes.style_force = style;
        let current = self.mes.style;
        self.mes_setup(current);
    }

    /// The animated "click to continue" icon.
    pub fn mes_draw_icon(&mut self, kind: i32) {
        if !self.mes.shown {
            return;
        }
        if self.novel() {
            let (mut x, mut y) = (self.mes.cur_x, self.mes.cur_y);
            if x >= self.game.ini.mes_x * 2 {
                x = 0;
                y += 1;
            }
            let ini = &self.game.ini;
            let (px, py) = (x * ini.font_x + ini.win_x, y * ini.font_y + ini.win_y);
            self.draw_novel_icon(px, py, kind);
            return;
        }
        let ini = &self.game.ini;
        let (icon_w, icon_h, wait, count) = (
            ini.mes_icon_x,
            ini.mes_icon_y,
            ini.mes_icon_wait,
            ini.mes_icon_count.max(1),
        );
        let (ix, iy) = (self.mes.x2 - icon_w - 7, self.mes.y2 - icon_h - 7);
        let now = self.now();
        if !self.mes.icon_shown {
            // Keep the screen under the icon in the frame picture's
            // bottom-right corner, and drop any click made before the icon.
            self.gfx.copy(
                Rect::new(ix, iy, self.mes.x2 - 8, self.mes.y2 - 8),
                0,
                640 - icon_w,
                480 - icon_h,
                WAKUPDT as i32,
                0,
            );
            self.mes.icon_shown = true;
            self.mes.icon_base = now;
            self.mouse.button(now);
        }
        if now < self.mes.icon_base + wait.max(0) as u64 {
            return;
        }
        self.mes.icon_base = now;
        let Some(waku) = self.gfx.get(WAKUPDT).cloned() else {
            return;
        };
        let key = waku.pixel(0);
        let display = self.gfx.display_mut();
        for y in 0..icon_h {
            for x in 0..icon_w {
                let (sx, sy) = (160 + self.mes.icon * icon_w + x, 8 + y);
                let (bx, by) = (640 - icon_w + x, 480 - icon_h + y);
                let (dx, dy) = (ix + x, iy + y);
                if !waku.contains(sx, sy) || !waku.contains(bx, by) || !display.contains(dx, dy) {
                    continue;
                }
                let source = waku.pixel(waku.at(sx, sy));
                let colour = if source == key {
                    waku.pixel(waku.at(bx, by))
                } else {
                    source
                };
                let at = display.at(dx, dy);
                display.set_pixel(at, colour);
            }
        }
        self.gfx
            .present(Rect::new(ix, iy, self.mes.x2 - 8, self.mes.y2 - 8));
        self.mes.icon = (self.mes.icon + 1) % count;
    }

    /// Draws the novel-mode click icon.
    fn draw_novel_icon(&mut self, x: i32, y: i32, kind: i32) {
        if !self.mes.shown {
            return;
        }
        let now = self.now();
        let wait = self.game.ini.mes_icon_wait.max(0) as u64;
        if !self.mes.icon_shown {
            self.mes.icon_shown = true;
            self.mes.icon_base = now.saturating_sub(wait);
        }
        if now < self.mes.icon_base + wait {
            return;
        }
        self.mes.icon_base = now;
        let (Some(waku), Some(back)) = (
            self.gfx.get(WAKUPDT).cloned(),
            self.gfx.get(MESWINPDT).cloned(),
        ) else {
            return;
        };
        let key = waku.pixel(0);
        let display = self.gfx.display_mut();
        for row in 0..24 {
            for column in 0..32 {
                let (sx, sy) = (160 + self.mes.icon * 32 + column, kind * 48 + 48 + row);
                let (dx, dy) = (x + column, y + row);
                if !waku.contains(sx, sy) || !display.contains(dx, dy) {
                    continue;
                }
                let source = waku.pixel(waku.at(sx, sy));
                let at = display.at(dx, dy);
                let colour = if source == key {
                    back.pixel(at)
                } else {
                    source
                };
                display.set_pixel(at, colour);
            }
        }
        self.gfx.present(Rect::new(x, y, x + 31, y + 23));
        self.mes.icon = (self.mes.icon + 1) % 8;
    }

    /// Removes the click icon.
    pub fn mes_hide_icon(&mut self) {
        if !(self.mes.icon_shown && self.mes.shown) {
            return;
        }
        self.mes.icon_shown = false;
        let ini = &self.game.ini;
        if self.novel() {
            let (mut x, mut y) = (self.mes.cur_x, self.mes.cur_y);
            if x >= ini.mes_x * 2 {
                x = 0;
                y += 1;
            }
            let (px, py) = (x * ini.font_x + ini.win_x, y * ini.font_y + ini.win_y);
            self.gfx.copy(
                Rect::new(px, py, px + 31, py + 23),
                MESWINPDT as i32,
                px,
                py,
                0,
                0,
            );
        } else {
            let (w, h) = (ini.mes_icon_x, ini.mes_icon_y);
            self.gfx.copy(
                Rect::new(640 - w, 480 - h, 639, 479),
                WAKUPDT as i32,
                self.mes.x2 - w - 7,
                self.mes.y2 - h - 7,
                0,
                0,
            );
        }
    }

    /// What happens after a click in a message.
    pub fn mes_line_feed(&mut self) -> i32 {
        if !self.mes.shown {
            return 0;
        }
        if self.novel() {
            self.mes.cur_x = 0;
            self.mes.cur_y += 1;
            self.mes.buf.truncate(self.mes.ptr);
            self.mes.buf.push(0x0d);
            self.mes.ptr += 1;
            self.mes.indent_flag = 0;
            self.mes.indent = 0;
            if self.mes.cur_y >= self.game.ini.mes_y {
                return -1;
            }
        } else {
            let page = std::mem::take(&mut self.mes.buf);
            self.record_backlog(&page);
            self.mes.ptr = 0;
            self.mes.cur_x = 0;
            self.mes.cur_y = 0;
            self.mes.indent_flag = 0;
            self.mes.indent = 0;
            self.mes.double_text = false;
            self.mes_draw_waku();
        }
        0
    }

    /// Saving is only offered at a clean page.
    pub fn check_novel_save(&self) -> bool {
        !self.novel() || self.mes.is_empty()
    }

    pub fn set_novel_mode(&mut self, enabled: i32) {
        self.game.ini.novel_mode = enabled;
        let style = self.mes.style;
        self.mes_setup(style);
    }

    pub fn mes_set_pos(&mut self, x: i32, y: i32) {
        if x != 0 || y != 0 {
            self.mes_hide();
            self.game.ini.win_x = x;
            self.game.ini.win_y = y;
            let style = self.mes.style;
            self.mes_setup(style);
        }
    }

    pub fn mes_set_size(&mut self, width: i32, height: i32) {
        self.mes_hide();
        self.game.ini.mes_x = width;
        self.game.ini.mes_y = height;
        let style = self.mes.style;
        self.mes_setup(style);
    }

    pub fn set_font_size(&mut self, width: i32, height: i32) {
        self.game.ini.font_x = width;
        self.game.ini.font_y = height;
        let style = self.mes.style;
        self.mes_setup(style);
    }

    pub fn set_window_colour(&mut self, flag: i32, colour: [i32; 3]) {
        self.game.ini.win_color_flag = flag;
        self.game.ini.win_color = colour;
        let style = self.mes.style;
        self.mes_setup(style);
    }

    // ---- choices ---------------------------------------------------------------------

    /// Adds a choice. `enable`: 1 selectable, 0 shown but
    /// disabled, -1 hidden (it still consumes an id).
    pub fn select_add_item(&mut self, text: &[u8], enable: i32, colour: i32) {
        self.sel.cur_id += 1;
        if enable == -1 || self.sel.items.len() >= MAX_SELECT {
            return;
        }
        let mut item = Vec::new();
        let mut at = 0;
        while at < text.len() && text[at] != 0 {
            let (character, length) = nls::char_at(&text[at..]);
            let length = length.max(1);
            if character == '＊' {
                let (letter, letter_length) = nls::char_at(text.get(at + length..).unwrap_or(&[]));
                let index = if letter == '\0' {
                    0
                } else {
                    letter as i32 - 'Ａ' as i32
                };
                item.extend_from_slice(self.name(index));
                at += length + letter_length;
            } else {
                item.extend_from_slice(&text[at..(at + length).min(text.len())]);
                at += length;
            }
        }
        self.sel.items.push(SelectItem {
            text: item,
            enable,
            colour,
            id: self.sel.cur_id,
        });
        self.sel.drawn = false;
    }

    /// A window sized to the choices.
    pub fn select_sub_window_setup(&mut self) {
        if !self.novel() {
            self.mes_hide();
        }
        let ini = &self.game.ini;
        self.mes.saved_geometry = Some([ini.mes_x, ini.mes_y, ini.win_x, ini.win_y]);
        let longest = self
            .sel
            .items
            .iter()
            .map(|item| item.text.len() as i32)
            .fold(ini.sub_win_min, i32::max);
        let count = self.sel.items.len() as i32;
        let (sub_x, sub_y) = (ini.sub_win_x, ini.sub_win_y);
        let ini = &mut self.game.ini;
        ini.mes_x = (longest + 1) / 2;
        ini.mes_y = count.max(1);
        ini.win_x = sub_x;
        ini.win_y = sub_y;
        self.mes.sub_window = true;
        let style = self.mes.style;
        self.mes_setup(style);
    }

    pub fn select_sub_window_close(&mut self) {
        if !self.novel() {
            self.mes_hide();
        }
        if let Some([mes_x, mes_y, win_x, win_y]) = self.mes.saved_geometry.take() {
            let ini = &mut self.game.ini;
            ini.mes_x = mes_x;
            ini.mes_y = mes_y;
            ini.win_x = win_x;
            ini.win_y = win_y;
        }
        self.mes.sub_window = false;
        let style = self.mes.style;
        self.mes_setup(style);
    }

    fn select_geometry(&self) -> (i32, i32, i32) {
        let ini = &self.game.ini;
        let (x1, y1, x2, y2) = (self.mes.x1, self.mes.y1, self.mes.x2, self.mes.y2);
        let left =
            x1 + (ini.font_size - ini.font_x * 2) / 2 + (x2 - x1 - ini.font_x * ini.mes_x * 2) / 2;
        let right = left + ini.mes_x * 2 * ini.font_x - (ini.font_size - ini.font_x * 2) / 2;
        let top =
            y1 + (ini.font_size - ini.font_y) / 2 + 4 + (y2 - y1 - ini.font_y * ini.mes_y) / 2;
        (left, right, top)
    }

    fn select_row_rect(&self, index: i32) -> Rect {
        let ini = &self.game.ini;
        let (mut left, mut right, top) = self.select_geometry();
        let half = (self.mes.x2 - self.mes.x1) >> 1;
        let rows = ini.mes_y.max(1);
        let row = if self.sel.items.len() as i32 > rows {
            if index >= rows {
                left += half;
            } else {
                right -= half - 1;
            }
            index % rows
        } else {
            index
        };
        Rect::new(
            left,
            row * ini.font_y + top,
            right - 1,
            (row + 1) * ini.font_y + top - 1,
        )
    }

    fn invert_row(&mut self, index: i32) {
        let rect = self.select_row_rect(index);
        self.gfx.invert(rect, 0);
    }

    /// -1 while choosing, otherwise the chosen item id
    /// (hidden items count towards the numbering).
    pub fn select(&mut self) -> i32 {
        if self.novel() {
            return self.select_novel();
        }
        if self.sel.finish != 0 {
            return self.select_finish();
        }
        if self.sel.items.is_empty() {
            return 0;
        }
        let ini_rows = self.game.ini.mes_y.max(1);
        if !self.sel.drawn {
            self.mes_draw();
            self.sel.drawn = true;
            self.sel.old = -1;
            let ini = &self.game.ini;
            let (font_x, font_y, font_size, mes_x) =
                (ini.font_x, ini.font_y, ini.font_size, ini.mes_x);
            let (x1, y1, x2, y2) = (self.mes.x1, self.mes.y1, self.mes.x2, self.mes.y2);
            let base_colour = self.text_colour();
            for (index, item) in self.sel.items.clone().into_iter().enumerate() {
                let index = index as i32;
                let mut x =
                    x1 + (font_size - font_x * 2) / 2 + (x2 - x1 - font_x * mes_x * 2 + font_x) / 2;
                let row = if index >= ini_rows {
                    x += (x2 - x1) >> 1;
                    index % ini_rows
                } else {
                    index
                };
                let y = y1 + row * font_y + font_size + (y2 - y1 - font_y * ini_rows) / 2;
                let colour = if item.enable != 0 {
                    base_colour
                } else {
                    self.table_colour(item.colour)
                };
                self.draw_string_at(0, x, y, &item.text, font_size, colour, true);
            }
            let rect = self.window_rect();
            self.gfx.present(rect);
            let now = self.now();
            self.mouse.button(now);
            self.sel.key_select = false;
        }
        let now = self.now();
        let (x, y, button) = self.mouse.state(now);
        if (x, y) != self.sel.last_mouse {
            self.sel.last_mouse = (x, y);
            self.sel.key_select = false;
        }
        let (left, right, top) = self.select_geometry();
        let count = self.sel.items.len() as i32;
        let mut choice = if self.sel.key_select {
            self.sel.old
        } else {
            -1
        };
        match self.key_input() {
            0x1e => {
                choice = self.sel.old;
                if self.sel.old != 0 {
                    choice = if self.sel.old == -1 {
                        count - 1
                    } else {
                        self.sel.old - 1
                    };
                    while choice >= 0 && self.sel.items[choice as usize].enable == 0 {
                        choice -= 1;
                    }
                }
                self.sel.key_select = true;
            }
            0x1f => {
                choice = self.sel.old;
                if choice + 1 < count {
                    loop {
                        choice += 1;
                        if choice >= count {
                            choice = -1;
                            break;
                        }
                        if self.sel.items[choice as usize].enable != 0 {
                            break;
                        }
                    }
                }
                self.sel.key_select = true;
            }
            _ => {}
        }
        let font_y = self.game.ini.font_y.max(1);
        if !self.sel.key_select
            && x >= left
            && x < right
            && y >= top
            && y < font_y * count + top
            && y < font_y * ini_rows + top
        {
            choice = (y - top) / font_y;
            if count > ini_rows && x >= ((self.mes.x2 - self.mes.x1) >> 1) + left {
                choice += ini_rows;
            }
            if choice >= count || self.sel.items[choice as usize].enable == 0 {
                choice = -1;
            }
        }
        if choice != self.sel.old {
            if self.sel.old != -1 {
                self.invert_row(self.sel.old);
            }
            if choice != -1 {
                self.invert_row(choice);
                self.play_se(0);
            }
            self.sel.old = choice;
        }
        let clicked = button == 0 || (self.sel.key_select && self.mouse.button(now));
        if clicked && self.sel.old != -1 {
            self.mouse.flush();
            self.sel.finish = self.sel.old + 1;
            self.sel.finish_count = 0;
            self.sel.finish_base = now;
            self.sel.old = 0;
            self.sel.drawn = false;
            self.play_se(1);
        }
        -1
    }

    /// Novel-mode variant of `select`.
    fn select_novel(&mut self) -> i32 {
        if self.sel.finish != 0 {
            return self.select_finish();
        }
        if self.sel.items.is_empty() {
            return 0;
        }
        let ini = &self.game.ini;
        let (win_x, win_y, font_x, font_y) = (ini.win_x, ini.win_y, ini.font_x, ini.font_y.max(1));
        if !self.sel.drawn {
            self.mes_draw();
            if self.window_effect.active() {
                return -1;
            }
            self.mes_clear();
            self.sel.drawn = true;
            self.sel.old = 0;
            for (index, item) in self.sel.items.clone().into_iter().enumerate() {
                let mut x = win_x;
                let y = win_y + index as i32 * font_y;
                let mut at = 0;
                while at < item.text.len() {
                    let (character, length) = nls::char_at(&item.text[at..]);
                    let code = nls::current().novel_code(&item.text[at..]);
                    at += length.max(1);
                    self.put_novel_char(x, y, character, code);
                    x += font_x * 2;
                }
            }
            let now = self.now();
            self.mouse.button(now);
            self.mes.icon = 0;
            self.mes.icon_base = now.saturating_sub(self.game.ini.mes_icon_wait.max(0) as u64);
            self.draw_novel_icon(win_x - 32, win_y, 2);
            self.sel.key_select = false;
        }
        let now = self.now();
        let (x, y, button) = self.mouse.state(now);
        if (x, y) != self.sel.last_mouse {
            self.sel.last_mouse = (x, y);
            self.sel.key_select = false;
        }
        let count = self.sel.items.len() as i32;
        let mut choice = self.sel.old;
        match self.key_input() {
            0x1e => {
                choice = self.sel.old - 1;
                while choice >= 0 && self.sel.items[choice as usize].enable == 0 {
                    choice -= 1;
                }
                if choice == -1 {
                    choice = self.sel.old;
                }
                self.sel.key_select = true;
            }
            0x1f => {
                loop {
                    choice += 1;
                    if choice >= count {
                        choice = self.sel.old;
                        break;
                    }
                    if self.sel.items[choice as usize].enable != 0 {
                        break;
                    }
                }
                self.sel.key_select = true;
            }
            _ => {}
        }
        if !self.sel.key_select && y >= win_y && y < font_y * count + win_y {
            choice = (y - win_y) / font_y;
            if self.sel.items[choice as usize].enable == 0 {
                choice = self.sel.old;
            }
        }
        if choice != self.sel.old {
            let (x1, y1) = (win_x - 32, win_y);
            let row = self.sel.old * font_y + y1;
            self.gfx.copy(
                Rect::new(x1, row, x1 + 31, row + 23),
                MESWINPDT as i32,
                x1,
                row,
                0,
                0,
            );
            self.sel.old = choice;
            self.mes.icon = 0;
            self.mes.icon_base = now.saturating_sub(self.game.ini.mes_icon_wait.max(0) as u64);
            self.play_se(0);
        }
        let clicked = button == 0 || (self.sel.key_select && self.mouse.button(now));
        if clicked {
            self.mouse.flush();
            self.sel.finish = self.sel.old + 1;
            self.sel.finish_count = 0;
            self.sel.finish_base = now;
            self.sel.drawn = false;
            self.play_se(1);
        }
        let old = self.sel.old;
        self.draw_novel_icon(win_x - 32, old * font_y + win_y, 2);
        -1
    }

    /// Blinks the chosen row, then reports it.
    fn select_finish(&mut self) -> i32 {
        self.sel.cur_id = 0;
        let now = self.now();
        let mut result = -1;
        let blink = self.game.ini.sel_blink_time.max(0) as u64;
        if now.saturating_sub(self.sel.finish_base) >= blink || self.check_skip() {
            self.sel.finish_base = now;
            if self.novel() {
                let ini = &self.game.ini;
                let (x1, row) = (
                    ini.win_x - 32,
                    (self.sel.finish - 1) * ini.font_y + ini.win_y,
                );
                if self.sel.finish_count & 1 != 0 {
                    self.mes.icon = 0;
                    self.mes.icon_base =
                        now.saturating_sub(self.game.ini.mes_icon_wait.max(0) as u64);
                    self.draw_novel_icon(x1, row, 2);
                } else {
                    self.gfx.copy(
                        Rect::new(x1, row, x1 + 31, row + 23),
                        MESWINPDT as i32,
                        x1,
                        row,
                        0,
                        0,
                    );
                }
            } else {
                self.invert_row(self.sel.finish - 1);
            }
            self.sel.finish_count += 1;
            if self.sel.finish_count > self.game.ini.sel_blink_count * 2 || self.check_skip() {
                let chosen = self.sel.finish;
                self.sel.finish = 0;
                let id = self
                    .sel
                    .items
                    .get((chosen - 1) as usize)
                    .map_or(chosen, |item| item.id);
                self.sel.items.clear();
                result = id;
            }
        }
        result
    }

    pub fn select_clear(&mut self) {
        self.sel = Selection::default();
    }

    /// Draws a Shift-JIS string with the message font (choices, `0x66`).
    pub(crate) fn draw_string_at(
        &mut self,
        buffer: usize,
        x: i32,
        baseline: i32,
        text: &[u8],
        size: i32,
        colour: [u8; 3],
        shadow: bool,
    ) {
        let mut pen = x;
        let mut at = 0;
        let unit = self.game.ini.font_x.max(1);
        while at < text.len() && text[at] != 0 {
            let (character, length) = nls::char_at(&text[at..]);
            let width = if length > 1 { 2 } else { 1 };
            if shadow {
                if let Some(shadow_colour) = self.shadow_colour() {
                    self.draw_char(
                        buffer,
                        pen + 1,
                        baseline + 1,
                        character,
                        size,
                        shadow_colour,
                    );
                }
            }
            self.draw_char(buffer, pen, baseline, character, size, colour);
            pen += unit * width;
            at += length.max(1);
        }
    }

    /// `0x66`: text straight into a PDT buffer.
    pub fn draw_buffer_string(
        &mut self,
        x: i32,
        y: i32,
        buffer: i32,
        colour: [i32; 3],
        text: &[u8],
    ) {
        let Some((index, update)) = crate::pdtmgr::resolve(buffer) else {
            return;
        };
        self.gfx.ensure(index);
        let size = self.game.ini.font_size;
        let colour = colour.map(|channel| channel.clamp(0, 255) as u8);
        self.draw_string_at(index, x, y + size, text, size, colour, false);
        if index == 0 && update {
            let width = (text.len() as i32 + 1) * (self.game.ini.font_x + 1);
            self.gfx
                .present(Rect::new(x, y, x + width, y + self.game.ini.font_y));
        }
    }
}
