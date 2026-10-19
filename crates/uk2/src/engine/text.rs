//! Window text (seg007/seg009): `U2` formatting, the typewriter renderer
//! with its `\r \N \n \K \k \C \W` escapes, page scrolling and the Borland
//! `sprintf` used to format messages.

use anyhow::Result;

use super::Engine;
use super::ds::*;
use super::mem::{FarPtr, ptr_add};
use super::object::flag;

const TEXT_BUFFER: usize = 0x7d0;

/// Formatting argument as pushed on the original stack.
#[derive(Debug, Clone)]
pub enum PrintfArg {
    Word(u16),
    Far(FarPtr),
}

impl Engine {
    /// Borland `vsprintf` over a stack image of word / far-pointer pushes.
    pub fn sprintf(&self, format: &[u8], args: &[PrintfArg]) -> Vec<u8> {
        let mut stack = Vec::new();
        for arg in args {
            match arg {
                PrintfArg::Word(v) => stack.extend_from_slice(&v.to_le_bytes()),
                PrintfArg::Far(p) => {
                    stack.extend_from_slice(&(*p as u16).to_le_bytes());
                    stack.extend_from_slice(&((*p >> 16) as u16).to_le_bytes());
                }
            }
        }
        let mut at = 0usize;
        let mut word = |at: &mut usize| {
            let lo = stack.get(*at).copied().unwrap_or(0);
            let hi = stack.get(*at + 1).copied().unwrap_or(0);
            *at += 2;
            u16::from_le_bytes([lo, hi])
        };
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < format.len() && format[i] != 0 {
            let c = format[i];
            if c != b'%' {
                out.push(c);
                i += 1;
                continue;
            }
            i += 1;
            let mut left = false;
            let mut zero = false;
            let mut plus = false;
            let mut space = false;
            loop {
                match format.get(i) {
                    Some(b'-') => left = true,
                    Some(b'0') => zero = true,
                    Some(b'+') => plus = true,
                    Some(b' ') => space = true,
                    Some(b'#') => {}
                    _ => break,
                }
                i += 1;
            }
            let mut width: Option<usize> = None;
            if format.get(i) == Some(&b'*') {
                width = Some(word(&mut at) as i16 as usize);
                i += 1;
            } else {
                while let Some(d @ b'0'..=b'9') = format.get(i) {
                    width = Some(width.unwrap_or(0) * 10 + usize::from(d - b'0'));
                    i += 1;
                }
            }
            let mut precision: Option<usize> = None;
            if format.get(i) == Some(&b'.') {
                i += 1;
                let mut p = 0usize;
                if format.get(i) == Some(&b'*') {
                    p = word(&mut at) as usize;
                    i += 1;
                } else {
                    while let Some(d @ b'0'..=b'9') = format.get(i) {
                        p = p * 10 + usize::from(d - b'0');
                        i += 1;
                    }
                }
                precision = Some(p);
            }
            let mut long = false;
            while let Some(m @ (b'l' | b'h' | b'F' | b'N' | b'L')) = format.get(i) {
                if *m == b'l' {
                    long = true;
                }
                i += 1;
            }
            let Some(&conv) = format.get(i) else { break };
            i += 1;
            let mut body: Vec<u8> = Vec::new();
            let mut numeric = true;
            match conv {
                b'd' | b'i' => {
                    let v: i64 = if long {
                        let lo = word(&mut at);
                        let hi = word(&mut at);
                        i64::from(((u32::from(hi) << 16) | u32::from(lo)) as i32)
                    } else {
                        i64::from(word(&mut at) as i16)
                    };
                    let mut digits = v.unsigned_abs().to_string().into_bytes();
                    if let Some(p) = precision {
                        while digits.len() < p {
                            digits.insert(0, b'0');
                        }
                    }
                    if v < 0 {
                        body.push(b'-');
                    } else if plus {
                        body.push(b'+');
                    } else if space {
                        body.push(b' ');
                    }
                    body.extend_from_slice(&digits);
                }
                b'u' | b'x' | b'X' | b'o' => {
                    let v: u64 = if long {
                        let lo = word(&mut at);
                        let hi = word(&mut at);
                        u64::from((u32::from(hi) << 16) | u32::from(lo))
                    } else {
                        u64::from(word(&mut at))
                    };
                    let text = match conv {
                        b'u' => v.to_string(),
                        b'x' => format!("{v:x}"),
                        b'X' => format!("{v:X}"),
                        _ => format!("{v:o}"),
                    };
                    let mut digits = text.into_bytes();
                    if let Some(p) = precision {
                        while digits.len() < p {
                            digits.insert(0, b'0');
                        }
                    }
                    body = digits;
                }
                b'c' => {
                    numeric = false;
                    body.push(word(&mut at) as u8);
                }
                b's' => {
                    numeric = false;
                    let lo = word(&mut at);
                    let hi = word(&mut at);
                    let ptr = (u32::from(hi) << 16) | u32::from(lo);
                    let mut s = if ptr == 0 {
                        b"(null)".to_vec()
                    } else {
                        self.mem.cstr(ptr)
                    };
                    if let Some(p) = precision {
                        s.truncate(p);
                    }
                    body = s;
                }
                b'%' => {
                    numeric = false;
                    body.push(b'%');
                }
                other => {
                    numeric = false;
                    body.push(other);
                }
            }
            let width = width.unwrap_or(0);
            if body.len() < width {
                let pad = width - body.len();
                if left {
                    body.extend(std::iter::repeat_n(b' ', pad));
                } else if zero && numeric && precision.is_none() {
                    let sign = usize::from(matches!(body.first(), Some(b'-' | b'+' | b' ')));
                    for _ in 0..pad {
                        body.insert(sign, b'0');
                    }
                } else {
                    let mut padded = vec![b' '; pad];
                    padded.extend_from_slice(&body);
                    body = padded;
                }
            }
            out.extend_from_slice(&body);
        }
        out
    }

    /// `sub_16F89(mode)`: U2 (mode 0) / D0 (mode 1).
    pub fn text_command(&mut self, mode: u16) -> Result<()> {
        let id = self.expr_word()?;
        let obj = if self.obj_index(id) == -1 {
            self.window_default(id)?
        } else {
            self.obj_at(self.obj_index(id))
        };
        let old = self.od(obj, 0x2e);
        self.mem.free(old);
        self.set_od(obj, 0x2e, 0);
        self.set_od(obj, 0x32, 0);
        self.set_od(obj, 0x36, 0);
        let flags = self.ow(obj, 0x28) & 0x00ef;
        self.set_ow(obj, 0x28, flags);
        self.set_w(TEXT_DONE, 0);
        let buffer = self.mem.alloc(TEXT_BUFFER);
        self.set_od(obj, 0x2e, buffer);
        self.set_od(obj, 0x32, buffer);
        self.set_od(obj, 0x36, buffer);
        let (ox, oy) = (self.ow(obj, 0x1e), self.ow(obj, 0x20));
        self.set_ow(obj, 0x22, ox);
        self.set_ow(obj, 0x24, oy);
        let mut strings: Vec<FarPtr> = Vec::new();
        let mut args: Vec<PrintfArg> = Vec::new();
        let mut count = 0;
        loop {
            if count > 0x1e {
                return Err(self.fatal(0x0c));
            }
            let value = self.expression()?;
            if value == 0 {
                break;
            }
            let kind = self.w(OPERAND_KIND);
            if kind == 1 || kind == 2 {
                strings.push(value);
                if count != 0 {
                    args.push(PrintfArg::Far(value));
                }
            } else {
                args.push(PrintfArg::Word(self.mem.rw(value)));
            }
            count += 1;
        }
        let format = strings
            .first()
            .map(|p| self.mem.cstr(*p))
            .unwrap_or_default();
        let text = self.sprintf(&format, &args);
        self.mem.strcpy_bytes(buffer, &text);
        if text.len() > TEXT_BUFFER {
            return Err(self.fatal(0x19));
        }
        for ptr in strings.into_iter().rev() {
            self.free_expr(ptr);
        }
        if mode != 0 {
            log::info!("uk2 D0: {}", String::from_utf8_lossy(&text));
            return Ok(());
        }
        if self.shift_state() & 1 == 0 && self.w(W_28FF0) != 0 {
            let speed = self.w(W_28FF0);
            self.set_w(TEXT_SPEED, speed);
        } else {
            self.set_w(TEXT_SPEED, 1000);
        }
        let obj = self.obj_ptr(id);
        if self.trace {
            eprintln!(
                "U2 id={id} flags={:04x} index={} count={}",
                self.ow(obj, 0x28),
                self.obj_index(id),
                self.obj_count()
            );
        }
        if self.w(W_27140) != 0 || self.ow(obj, 0x28) & flag::VISIBLE == 0 {
            if self.obj_raise(id)? == 0 {
                self.obj_redraw(-1)?;
            }
        } else {
            let saved = self.w(FORCE_REDRAW);
            self.set_w(FORCE_REDRAW, 0);
            let index = self.obj_index(id);
            self.obj_redraw(index)?;
            self.set_w(FORCE_REDRAW, saved);
        }
        self.set_w(TEXT_SPEED, 0);
        Ok(())
    }

    /// `sub_171C6`: draw the current page of an object's text.
    pub fn draw_text(&mut self, index: i32) -> Result<()> {
        let mut i = index;
        let obj = self.obj_active(&mut i);
        if obj == 0 {
            return Ok(());
        }
        let index = i;
        let base = self.od(obj, 0x2e);
        let page = self.od(obj, 0x36);
        if base == 0 || self.mem.rb(page) == 0 {
            self.set_w(TEXT_DONE, 1);
            return Ok(());
        }
        let mut raw = false;
        let mut digits = false;
        let cursor = self.w(CURSOR_SHOWN);
        self.set_od(obj, 0x32, page);
        let origin_x = self.ow(obj, 0x1e);
        let mut p = page;
        let mut char_start = page;
        loop {
            let c = self.mem.rb(p);
            if c == 0 {
                break;
            }
            if c == b'\\' {
                let command = self.mem.rb(ptr_add(p, 1));
                let live_page = self.od(obj, 0x36);
                match command {
                    b'r' => {
                        if live_page <= p {
                            self.set_ow(obj, 0x22, origin_x);
                            let y = self.ow(obj, 0x24).wrapping_add(16);
                            self.set_ow(obj, 0x24, y);
                        }
                    }
                    b'N' => digits = true,
                    b'n' => digits = false,
                    b'K' => raw = false,
                    b'k' => raw = true,
                    b'C' => {
                        p = ptr_add(p, 1);
                        let d = self.mem.rb(ptr_add(p, 1));
                        let colour = if d >= b'A' {
                            u16::from(d).wrapping_sub(0x37)
                        } else {
                            u16::from(d).wrapping_sub(0x30)
                        };
                        self.set_w(TEXT_COLOUR, colour);
                    }
                    b'W' => {
                        // The delay is disabled in this build (`test al, 0`).
                        p = ptr_add(p, 3);
                    }
                    _ => {}
                }
                p = ptr_add(p, 2);
                continue;
            }
            char_start = p;
            let glyph: Vec<u8>;
            if c.is_ascii_digit() && digits {
                glyph = vec![0x82, 0x4f + (c - b'0')];
            } else if self.text_encoding.is_single_byte(c) {
                if raw {
                    glyph = vec![c];
                } else {
                    let table = self.mem.d(ZENKAKU_TABLE);
                    let entry = ptr_add(table, i32::from(c) * 2);
                    glyph = vec![self.mem.rb(entry), self.mem.rb(ptr_add(entry, 1))];
                }
            } else if self.text_encoding != super::font::TextEncoding::ShiftJis {
                glyph = vec![super::font::FOREIGN_GLYPH, c, self.mem.rb(ptr_add(p, 1))];
                p = ptr_add(p, 1);
            } else {
                glyph = vec![c, self.mem.rb(ptr_add(p, 1))];
                p = ptr_add(p, 1);
            }
            p = ptr_add(p, 1);
            if self.od(obj, 0x36) > char_start {
                continue;
            }
            let result = self.draw_text_char(&glyph, index)?;
            if result != 0 {
                if self.w(W_28FF2) == 0 || self.w(TEXT_SPEED) == 0 {
                    self.set_od(obj, 0x36, char_start);
                    self.cursor_show(cursor);
                    return Ok(());
                }
                self.text_scroll_line(obj)?;
                self.set_od(obj, 0x36, char_start);
                self.set_od(obj, 0x32, char_start);
            }
            let speed = self.w(TEXT_SPEED);
            if speed < 1000 && self.shift_state() & 1 == 0 {
                self.delay_ms(u32::from(speed))?;
            }
        }
        let _ = char_start;
        let start = self.od(obj, 0x32);
        self.set_od(obj, 0x36, start);
        self.set_w(TEXT_DONE, 1);
        self.cursor_show(cursor);
        Ok(())
    }

    /// Scroll a text window up one line (part of `sub_171C6`).
    fn text_scroll_line(&mut self, obj: FarPtr) -> Result<()> {
        let mut x1 = self.osw(obj, 0x0a) + self.osw(obj, 0x1e);
        let mut x2 = self.osw(obj, 0x0e);
        let mut y1 = self.osw(obj, 0x0c) + self.osw(obj, 0x20);
        let mut y2 = self.osw(obj, 0x10);
        x1 = x1.max(0);
        x2 = x2.min(0x4f);
        y1 = y1.max(0);
        y2 = y2.min(0x27f);
        if x2 <= x1 || y2 <= y1 {
            return Ok(());
        }
        self.cursor_show(0);
        let page = self.w(DISPLAY_PAGE) as usize;
        let width = (x2 - x1 + 1) as usize;
        let rows = (y2 - (y1 + 16) + 1).max(0) as usize;
        self.vram.copy_rows(
            page,
            (y1 as usize) * 80 + x1 as usize,
            ((y1 + 16) as usize) * 80 + x1 as usize,
            width,
            rows,
        );
        let y = self.ow(obj, 0x14).wrapping_sub(self.ow(obj, 0x20));
        self.set_ow(obj, 0x24, y);
        let owner = u16::from(self.ob(obj, 0x2b));
        let chip = u16::from(self.ob(obj, 0x2a)) + 4;
        let chips = self.mem.d(WIN_CHIPS);
        let bottom = y2 - 16;
        let mut x = x1;
        while x < x2 {
            self.draw_chip(x, bottom, chips, chip, 2, owner);
            x += 2;
        }
        if self.shift_state() & 1 != 0 {
            let wait = self.w(W_28FEC);
            self.delay_ms(u32::from(wait))?;
        }
        Ok(())
    }

    /// `sub_16BA9(glyph, index)`: draw one character at the text cursor.
    fn draw_text_char(&mut self, glyph: &[u8], index: i32) -> Result<i32> {
        let obj = self.obj_at(index);
        let w = self.osw(obj, 0x12);
        let h = self.osw(obj, 0x14);
        let ox = self.osw(obj, 0x1e);
        let oy = self.osw(obj, 0x20);
        if self.osw(obj, 0x22) >= w - ox {
            let y = self.ow(obj, 0x24).wrapping_add(16);
            self.set_ow(obj, 0x24, y);
            self.set_osw(obj, 0x22, ox);
        }
        if self.osw(obj, 0x24) >= h - oy {
            return Ok(-1);
        }
        let first = glyph.first().copied().unwrap_or(0);
        let width = if first < 0x80 || (0xa0..0xe0).contains(&first) {
            1
        } else {
            2
        };
        let col = self.osw(obj, 0x0a) + self.osw(obj, 0x22);
        let y = self.osw(obj, 0x0c) + self.osw(obj, 0x24);
        let owner = u16::from(self.ob(obj, 0x2b));
        let s1 = self.cell_state(col, y, owner);
        let s2 = self.cell_state(col + 1, y, owner);
        let mut draw = !(s1 == 0 || s1 == 4 || s2 == 0 || s2 == 4);
        if draw && self.w(FORCE_REDRAW) != 0 && s1 == 3 && s2 == 3 {
            draw = false;
        }
        if draw {
            let x = col * 8;
            let style = self.w(GLYPH_STYLE);
            let mut variant = i32::from(s1);
            match style {
                0..=3 => {
                    self.set_w(GLYPH_OP, 1);
                    variant = i32::from(style);
                }
                4..=7 => {
                    self.set_w(GLYPH_OP, 3);
                    variant = i32::from(style) - 4;
                }
                8 => {
                    self.set_w(GLYPH_OP, 2);
                    variant = i32::from(style) - 7;
                }
                _ => {}
            }
            if self.osw(obj, 0x22) + width >= w {
                match self.w(GLYPH_OP) {
                    3 => self.set_w(GLYPH_OP, 4),
                    1 => self.set_w(GLYPH_OP, 2),
                    _ => {}
                }
            }
            let shadow = self.w(TEXT_SHADOW);
            let outline = self.w(TEXT_OUTLINE);
            let colour = self.w(TEXT_COLOUR);
            match variant {
                0 => {
                    self.draw_glyph(glyph, x - 1, y - 1, shadow, 0);
                    self.draw_glyph(glyph, x, y, outline, 0);
                }
                1 => {
                    self.draw_glyph(glyph, x, y, colour, 1);
                }
                2 => {
                    self.draw_glyph(glyph, x, y, colour, 0);
                }
                3 => {
                    for (dx, dy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                        self.draw_glyph(glyph, x + dx, y + dy, shadow, 0);
                    }
                    self.draw_glyph(glyph, x, y, outline, 0);
                }
                _ => {}
            }
        }
        let cx = self.osw(obj, 0x22) + width;
        self.set_osw(obj, 0x22, cx);
        Ok(if h - oy > self.osw(obj, 0x24) { 0 } else { -1 })
    }

    /// `sub_1E204(direction)`: page (5 next, 6 first) a text window or
    /// scroll a map / chip view (5 down, 6 up, 7 right, 8 left).  Returns 1
    /// when the top object has text.
    pub fn text_scroll_request(&mut self, direction: u16) -> Result<u16> {
        let obj = self.obj_at(self.obj_count() - 1);
        let text = self.od(obj, 0x2e);
        if text != 0 {
            if self.w(W_28FF0) != 0 {
                let speed = self.w(W_28FF0);
                self.set_w(TEXT_SPEED, speed);
            }
            let start = self.od(obj, 0x32);
            if direction == 6 && start != text {
                self.set_od(obj, 0x32, text);
                self.obj_draw_content(-1)?;
            }
            let end = self.od(obj, 0x36);
            if direction == 5 && start != end {
                self.set_od(obj, 0x32, end);
                self.obj_draw_content(-1)?;
            }
            self.set_w(TEXT_SPEED, 0);
            return Ok(1);
        }
        let step: i32 = if self.shift_state() & 1 != 0 { 4 } else { 1 };
        let sx = self.osw(obj, 0x22);
        let sy = self.osw(obj, 0x24);
        let right = sx + self.osw(obj, 0x12) / 2;
        let bottom = sy + self.osw(obj, 0x14) / 16;
        let map = self.od(obj, 0x4e);
        if map != 0 {
            let (mw, mh) = (
                i32::from(self.mem.rw(map) as i16),
                i32::from(self.mem.rw(ptr_add(map, 2)) as i16),
            );
            let moved = match direction {
                5 if mh >= bottom + step => {
                    self.set_osw(obj, 0x24, sy + step);
                    true
                }
                6 if sy - step >= 0 => {
                    self.set_osw(obj, 0x24, sy - step);
                    true
                }
                7 if right + step <= mw => {
                    self.set_osw(obj, 0x22, sx + step);
                    true
                }
                8 if sx - step >= 0 => {
                    self.set_osw(obj, 0x22, sx - step);
                    true
                }
                5..=8 => return Ok(0),
                _ => true,
            };
            if moved {
                self.obj_draw_content(-1)?;
            }
        }
        if self.od(obj, 0x5a) != 0 {
            let lim_h = self.osw(obj, 0x1c) / 16;
            let lim_w = self.osw(obj, 0x1a) / 2;
            let sx = self.osw(obj, 0x22);
            let sy = self.osw(obj, 0x24);
            let moved = match direction {
                5 if lim_h >= bottom + step => {
                    self.set_osw(obj, 0x24, sy + step);
                    true
                }
                6 if sy - step >= 0 => {
                    self.set_osw(obj, 0x24, sy - step);
                    true
                }
                7 if lim_w >= right + step => {
                    self.set_osw(obj, 0x22, sx + step);
                    true
                }
                8 if sx - step >= 0 => {
                    self.set_osw(obj, 0x22, sx - step);
                    true
                }
                5..=8 => return Ok(0),
                _ => true,
            };
            if moved {
                self.obj_draw_content(-1)?;
            }
        }
        Ok(0)
    }

    /// `sub_17622(x1, y1, x2, y2, id, planes)`: invert a cell rectangle
    /// (relative to a window's content unless `id` is 1000).
    pub fn invert_cells(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, id: u16, planes: u16) {
        let cursor = self.w(CURSOR_SHOWN);
        let mut ax = x1 * 2;
        let mut bx = x2 * 2 + 1;
        let mut ay = y1 * 16;
        let mut by = y2 * 16 + 15;
        if id == 1000 {
            self.vram.invert(ax, ay, bx, by, planes);
            self.cursor_show(cursor);
            return;
        }
        let obj = self.obj_ptr(id);
        if obj == 0 {
            return;
        }
        ax += self.osw(obj, 0x0a);
        bx += self.osw(obj, 0x0a);
        ay += self.osw(obj, 0x0c);
        by += self.osw(obj, 0x0c);
        let mut y = ay;
        while y < by {
            let mut x = ax;
            while x < bx {
                let state = self.cell_state(x, y, id);
                if state == 1 || state == 3 {
                    self.vram.invert(x, y, x + 1, y + 15, planes);
                }
                x += 2;
            }
            y += 16;
        }
        self.cursor_show(cursor);
    }
}
