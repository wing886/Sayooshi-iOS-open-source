//! Window objects (seg005, seg006, seg016).
//!
//! Objects are 0x62-byte records in a fixed pool; `dword_23B72` is an array
//! of 70 far pointers ordered back-to-front and `word_2901C` counts the live
//! entries.  The screen is composed on VRAM page 0 on top of the background
//! kept in page 1.  Two 40x25 "cell owner" maps record which object owns each
//! 16x16 cell; drawing is clipped per cell to the owner, and cells that
//! return to the background are copied back from page 1.
//!
//! Record layout (offsets):
//! `+00` content draw routine, `+04` second draw routine, `+08` kind word,
//! `+0A/+0C/+0E/+10` content rectangle (byte columns / lines),
//! `+12/+14` content size, `+16..+1C` size limits, `+1E/+20` text origin,
//! `+22/+24` text cursor or map scroll, `+26` menu state, `+28` flags,
//! `+2A` frame chip base, `+2B` id, `+2C` title style, `+2D` frame gadgets,
//! `+2E/+32/+36` text buffer / page start / page end, `+3A` name,
//! `+4E` map, `+52` menu items, `+56` image buffer, `+5A` chip buffer.

use anyhow::Result;

use super::Engine;
use super::ds::*;
use super::mem::{FarPtr, far, ptr_add};

pub const OBJ_SIZE: usize = 0x62;
pub const MAX_OBJECTS: i32 = 70;

pub mod flag {
    pub const VISIBLE: u16 = 0x01;
    pub const HIDDEN_DRAW: u16 = 0x02;
    pub const OWN_IMAGE: u16 = 0x04;
    pub const NO_FREE: u16 = 0x08;
    pub const OWN_TEXT: u16 = 0x10;
    pub const FRAMELESS: u16 = 0x20;
    pub const RESIZABLE: u16 = 0x40;
    pub const FREE_POSITION: u16 = 0x80;
    pub const MENU_ACTIVE: u16 = 0x100;
    pub const INSET_HIT: u16 = 0x200;
}

/// Content routines stored in `+00` (the original stores far code pointers).
pub mod content {
    use super::super::mem::far;
    pub const CHIP_VIEW: u32 = 0xF000_0001; // sub_1C9E4 (F1)
    pub const MAP_VIEW: u32 = 0xF000_0002; // seg015:1A55 (F5)
    pub const IMAGE: u32 = 0xF000_0003; // sub_1E76B (W6)
    pub const MENU: u32 = 0xF000_0004; // sub_1A782 (W8)
    #[allow(dead_code)]
    pub fn code(value: u32) -> u32 {
        value - far(0xF000, 0)
    }
}

impl Engine {
    // ---- record access --------------------------------------------------

    #[inline]
    pub fn ow(&self, obj: FarPtr, off: i32) -> u16 {
        self.mem.rw(ptr_add(obj, off))
    }
    #[inline]
    pub fn osw(&self, obj: FarPtr, off: i32) -> i32 {
        i32::from(self.mem.rw(ptr_add(obj, off)) as i16)
    }
    #[inline]
    pub fn set_ow(&mut self, obj: FarPtr, off: i32, value: u16) {
        self.mem.ww(ptr_add(obj, off), value);
    }
    #[inline]
    pub fn set_osw(&mut self, obj: FarPtr, off: i32, value: i32) {
        self.mem.ww(ptr_add(obj, off), value as u16);
    }
    #[inline]
    pub fn ob(&self, obj: FarPtr, off: i32) -> u8 {
        self.mem.rb(ptr_add(obj, off))
    }
    #[inline]
    pub fn set_ob(&mut self, obj: FarPtr, off: i32, value: u8) {
        self.mem.wb(ptr_add(obj, off), value);
    }
    #[inline]
    pub fn od(&self, obj: FarPtr, off: i32) -> FarPtr {
        self.mem.rd(ptr_add(obj, off))
    }
    #[inline]
    pub fn set_od(&mut self, obj: FarPtr, off: i32, value: FarPtr) {
        self.mem.wd(ptr_add(obj, off), value);
    }

    pub fn obj_count(&self) -> i32 {
        i32::from(self.sw(OBJ_COUNT))
    }

    fn set_obj_count(&mut self, count: i32) {
        self.set_w(OBJ_COUNT, count as u16);
    }

    pub fn obj_at(&self, index: i32) -> FarPtr {
        let table = self.mem.d(OBJ_TABLE);
        self.mem.rd(ptr_add(table, index * 4))
    }

    /// `sub_20D33` on two adjacent table entries.
    pub(crate) fn obj_swap(&mut self, a: i32, b: i32) {
        let table = self.mem.d(OBJ_TABLE);
        self.mem
            .memswap(ptr_add(table, a * 4), ptr_add(table, b * 4), 4);
    }

    /// `sub_1E70B`: table index of an object id, or -1.
    pub fn obj_index(&self, id: u16) -> i32 {
        for index in 0..self.obj_count() {
            if u16::from(self.ob(self.obj_at(index), 0x2b)) == id {
                return index;
            }
        }
        -1
    }

    /// `sub_1E73F`.
    pub fn obj_ptr(&self, id: u16) -> FarPtr {
        match self.obj_index(id) {
            -1 => 0,
            index => self.obj_at(index),
        }
    }

    /// `sub_1F505`: visible object at `index` (negative = topmost).
    pub fn obj_active(&self, index: &mut i32) -> FarPtr {
        let count = self.obj_count();
        if count == 0 {
            return 0;
        }
        if *index < 0 {
            *index = count - 1;
        }
        if *index >= count {
            return 0;
        }
        let obj = self.obj_at(*index);
        if self.ow(obj, 0x28) & flag::VISIBLE == 0 {
            return 0;
        }
        obj
    }

    // ---- cell ownership ----------------------------------------------------

    fn cell_map(&self, which: u16) -> FarPtr {
        self.mem.d(CELL_MAPS + (which & 1) * 4)
    }

    /// `sub_16A40(x, y, owner)`: 4 = off-screen, 2 = background, 0 = other
    /// owner, 3 = unchanged, 1 = newly owned.
    pub fn cell_state(&self, x: i32, y: i32, owner: u16) -> u16 {
        let cx = (x as u16) / 2;
        let cy = (y as u16) / 16;
        if cx > 0x27 || cy > 0x18 {
            return 4;
        }
        let cell = i32::from(cy) * 40 + i32::from(cx);
        let sel = self.w(CELL_MAP_SEL);
        let now = u16::from(self.mem.rb(ptr_add(self.cell_map(sel ^ 1), cell)));
        let before = u16::from(self.mem.rb(ptr_add(self.cell_map(sel), cell)));
        if now == 0xf0 {
            2
        } else if now != owner {
            0
        } else if now == before {
            3
        } else {
            1
        }
    }

    /// `sub_16AC9(x, y, buf, index, mode, owner)`: draw one chip if the
    /// cell belongs to `owner`.
    pub fn draw_chip(
        &mut self,
        x: i32,
        y: i32,
        buf: FarPtr,
        index: u16,
        mode: u16,
        owner: u16,
    ) -> bool {
        let state = self.cell_state(x, y, owner);
        if state == 4 || (state == 3 && self.w(FORCE_REDRAW) != 0) || state == 0 {
            return false;
        }
        if matches!(mode, 1..=3) {
            self.cell_put(x as u16, y as u16, buf, index, mode);
        }
        true
    }

    /// `sub_1DF70`: recompute cell ownership into the other map.
    pub fn compute_owners(&mut self) {
        let sel = self.w(CELL_MAP_SEL);
        let map = self.cell_map(sel);
        self.set_w(CELL_MAP_SEL, sel ^ 1);
        self.mem.memset(map, 0xf0, 1000);
        for index in 0..self.obj_count() {
            let obj = self.obj_at(index);
            let flags = self.ow(obj, 0x28);
            if flags & flag::VISIBLE == 0 {
                continue;
            }
            let mut x1 = self.osw(obj, 0x0a) / 2;
            let mut x2 = self.osw(obj, 0x0e) / 2;
            let mut y1 = self.osw(obj, 0x0c) / 16;
            let mut y2 = self.osw(obj, 0x10) / 16;
            if flags & flag::FRAMELESS == 0 {
                x1 -= 1;
                x2 += 1;
                y1 -= 1;
                y2 += 1;
            }
            x1 = x1.max(0);
            x2 = x2.min(40);
            y1 = y1.max(0);
            y2 = y2.min(25);
            let id = self.ob(obj, 0x2b);
            for cy in y1..y2 {
                for cx in x1..x2 {
                    self.mem.wb(ptr_add(map, cy * 40 + cx), id);
                }
            }
        }
    }

    /// `sub_1EB40(x1, y1, x2, y2)`: mark background cells of the current
    /// map as dirty (F1) inside a cell rectangle.
    pub fn mark_background_dirty(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let map = self.cell_map(self.w(CELL_MAP_SEL) ^ 1);
        for cy in 0..25 {
            if !(y1 <= cy && cy <= y2) {
                continue;
            }
            for cx in 0..40 {
                if x1 <= cx && cx <= x2 {
                    let p = ptr_add(map, cy * 40 + cx);
                    if self.mem.rb(p) == 0xf0 {
                        self.mem.wb(p, 0xf1);
                    }
                }
            }
        }
    }

    /// `sub_16455(mode)`: redraw all windows and restore uncovered cells
    /// from the background page.
    pub fn compose(&mut self, mode: u16) -> Result<()> {
        self.compute_owners();
        self.set_w(FORCE_REDRAW, u16::from(mode != 1));
        let cursor = self.cursor_show(0);
        let count = self.obj_count();
        let mut index = 0;
        while index < self.obj_count() {
            let obj = self.obj_at(index);
            if self.ow(obj, 0x28) & 3 == 1 {
                if mode == 0 && index == count - 1 {
                    self.set_w(FORCE_REDRAW, 0);
                }
                self.obj_redraw(index)?;
            }
            index += 1;
        }
        self.set_w(FORCE_REDRAW, 0);
        let display = self.w(DISPLAY_PAGE);
        let back = usize::from(display ^ 1) & 1;
        let front = usize::from(display) & 1;
        let sel = self.w(CELL_MAP_SEL);
        let before = self.cell_map(sel);
        let now = self.cell_map(sel ^ 1);
        for cell in 0..1000i32 {
            let n = self.mem.rb(ptr_add(now, cell));
            if n == 0xf0 && n != self.mem.rb(ptr_add(before, cell)) {
                let x = (cell % 40) as usize * 2;
                let y = (cell / 40) as usize * 16;
                for plane in 0..4 {
                    for row in 0..16 {
                        let at = (y + row) * 80 + x;
                        for k in 0..2 {
                            let value = self.vram.get_page(back, plane, at + k);
                            self.vram.set_page(front, plane, at + k, value);
                        }
                    }
                }
            }
        }
        self.set_pages(display, display);
        self.cursor_show(cursor);
        let top = self.obj_at(self.obj_count() - 1);
        let id = if self.obj_count() > 0 {
            u16::from(self.ob(top, 0x2b))
        } else {
            0
        };
        self.set_w(TOP_OBJECT, id);
        Ok(())
    }

    // ---- drawing ------------------------------------------------------------

    /// `sub_15E4B`.
    pub fn obj_redraw(&mut self, index: i32) -> Result<bool> {
        let mut i = index;
        if self.obj_active(&mut i) != 0 {
            self.set_w(W_28FC6, 1);
            self.obj_draw_frame(i)?;
            self.obj_draw_content(i)?;
            self.set_w(W_28FC6, 0);
        }
        Ok(true)
    }

    /// `sub_15E82`: draw text / content routines.
    pub fn obj_draw_content(&mut self, index: i32) -> Result<()> {
        let mut i = index;
        let obj = self.obj_active(&mut i);
        if obj == 0 {
            return Ok(());
        }
        let index = i;
        if self.od(obj, 0x2e) != 0 {
            self.set_w(TEXT_DONE, 0);
            let start = self.od(obj, 0x32);
            self.set_od(obj, 0x36, start);
            self.obj_clear(index)?;
            self.draw_text(index)?;
        }
        let first = self.od(obj, 0);
        if first != 0 {
            self.call_content(first, index)?;
        }
        let second = self.od(obj, 4);
        if second != 0 {
            self.call_content(second, index)?;
        }
        if self.od(obj, 0) == 0 && self.od(obj, 4) == 0 && self.od(obj, 0x2e) == 0 {
            self.obj_clear(index)?;
        }
        Ok(())
    }

    fn call_content(&mut self, routine: FarPtr, index: i32) -> Result<()> {
        match routine {
            content::CHIP_VIEW => self.chip_view_draw(index),
            content::MAP_VIEW => self.map_view_draw(index),
            content::IMAGE => self.image_window_draw(index),
            content::MENU => self.menu_draw(index),
            _ => Ok(()),
        }
    }

    /// `sub_17583`: fill the content area with the frame's centre chip and
    /// reset the text cursor.
    pub fn obj_clear(&mut self, index: i32) -> Result<()> {
        let mut i = index;
        let obj = self.obj_active(&mut i);
        if obj == 0 {
            return Ok(());
        }
        let index = i;
        let flags = self.ow(obj, 0x28);
        if flags & flag::FRAMELESS == 0 {
            let cursor = self.w(CURSOR_SHOWN);
            let chip = u16::from(self.ob(obj, 0x2a)) + 4;
            let owner = u16::from(self.ob(obj, 0x2b));
            let chips = self.mem.d(WIN_CHIPS);
            let mut y = self.osw(obj, 0x0c) + self.osw(obj, 0x20);
            while y < self.osw(obj, 0x10) {
                let mut x = self.osw(obj, 0x0a) + self.osw(obj, 0x1e);
                while x < self.osw(obj, 0x0e) {
                    self.draw_chip(x, y, chips, chip, 2, owner);
                    x += 2;
                }
                y += 16;
            }
            self.cursor_show(cursor);
            let (ox, oy) = (self.ow(obj, 0x1e), self.ow(obj, 0x20));
            self.set_ow(obj, 0x22, ox);
            self.set_ow(obj, 0x24, oy);
        }
        Ok(())
    }

    /// `sub_15F2D`: clamp the window on screen and draw its frame.
    pub fn obj_draw_frame(&mut self, index: i32) -> Result<()> {
        let mut i = index;
        let obj = self.obj_active(&mut i);
        if obj == 0 {
            return Ok(());
        }
        let index = i;
        let flags = self.ow(obj, 0x28);
        let id = u16::from(self.ob(obj, 0x2b));
        if flags & flag::FREE_POSITION == 0 {
            let mut clamped = false;
            let (mut x1, mut y1) = (self.osw(obj, 0x0a), self.osw(obj, 0x0c));
            let (w, h) = (self.osw(obj, 0x12), self.osw(obj, 0x14));
            let (min_x, min_y, max_x, max_y) = if flags & flag::FRAMELESS != 0 {
                (0, 0, 0x50, 0x190)
            } else {
                (2, 0x10, 0x4e, 0x180)
            };
            if x1 < min_x {
                x1 = min_x;
                clamped = true;
            }
            if y1 < min_y {
                y1 = min_y;
                clamped = true;
            }
            let mut x2 = x1 + w;
            let mut y2 = y1 + h;
            if x2 > max_x {
                x2 = max_x;
                clamped = true;
            }
            if y2 > max_y {
                y2 = max_y;
                clamped = true;
            }
            x1 = x2 - w;
            y1 = y2 - h;
            self.set_osw(obj, 0x0a, x1);
            self.set_osw(obj, 0x0e, x2);
            self.set_osw(obj, 0x0c, y1);
            self.set_osw(obj, 0x10, y2);
            if clamped {
                self.obj_close(id, 0)?;
                self.obj_raise(id)?;
            }
        }
        if self.osw(obj, 0x22) < 0 {
            self.set_ow(obj, 0x22, 0);
        }
        if self.osw(obj, 0x24) < 0 {
            self.set_ow(obj, 0x24, 0);
        }
        if flags & flag::FRAMELESS != 0 {
            return Ok(());
        }
        let left = self.osw(obj, 0x0a) - 2;
        let top = self.osw(obj, 0x0c) - 16;
        let bottom = self.osw(obj, 0x10) + 16;
        let right_inner = self.osw(obj, 0x0e);
        let cursor = self.w(CURSOR_SHOWN);
        let chips = self.mem.d(WIN_CHIPS);
        let mut chip = u16::from(self.ob(obj, 0x2a));
        let mut y = top;
        let mut x = right_inner;
        while y < bottom {
            self.draw_chip(left, y, chips, chip, 2, id);
            chip += 1;
            if y == top || bottom - 16 == y {
                x = self.osw(obj, 0x0a);
                while x < self.osw(obj, 0x0e) {
                    self.draw_chip(x, y, chips, chip, 2, id);
                    x += 2;
                }
            }
            chip += 1;
            self.draw_chip(x, y, chips, chip, 2, id);
            chip += 1;
            if y > top && bottom - 32 > y {
                chip -= 3;
            }
            y += 16;
        }
        self.obj_draw_gadgets(index)?;
        self.cursor_show(cursor);
        Ok(())
    }

    /// `sub_15C80`: title decoration and frame gadgets.
    pub fn obj_draw_gadgets(&mut self, index: i32) -> Result<()> {
        let mut i = index;
        let obj = self.obj_active(&mut i);
        if obj == 0 || self.ow(obj, 0x28) & flag::FRAMELESS != 0 {
            return Ok(());
        }
        let index = i;
        let id = u16::from(self.ob(obj, 0x2b));
        let mut chip = u16::from(self.ob(obj, 0x2c)) * 8;
        if self.obj_count() - 1 == index {
            chip += 4;
        }
        let right = self.osw(obj, 0x0e) + 2;
        let top = self.osw(obj, 0x0c) - 16;
        let bottom = self.osw(obj, 0x10) + 16;
        let mut x = self.osw(obj, 0x0a);
        let parts = self.mem.d(WIN_DISP_CHIPS);
        for _ in 0..4 {
            self.draw_chip(x, top, parts, chip, 2, id);
            x += 2;
            chip += 1;
        }
        let base = u16::from(self.ob(obj, 0x2a));
        let gadgets = self.ob(obj, 0x2d);
        let chips = self.mem.d(WIN_CHIPS);
        if gadgets & 1 != 0 {
            self.draw_chip(right - 4, top, chips, base + 9, 2, id);
        }
        if gadgets & 2 != 0 {
            self.draw_chip(right - 4, bottom - 16, chips, base + 0x0e, 2, id);
        }
        if gadgets & 4 != 0 {
            self.draw_chip(right - 8, bottom - 16, chips, base + 0x0c, 2, id);
            self.draw_chip(right - 6, bottom - 16, chips, base + 0x0d, 2, id);
        }
        if gadgets & 8 != 0 {
            self.draw_chip(right - 12, bottom - 16, chips, base + 0x0a, 2, id);
            self.draw_chip(right - 10, bottom - 16, chips, base + 0x0b, 2, id);
        }
        Ok(())
    }

    // ---- lifetime -------------------------------------------------------------

    /// `sub_16234(id, mode)`: 0 hide, 1 destroy, 2 detach.
    pub fn obj_close(&mut self, id: u16, mode: u16) -> Result<()> {
        let mut index = self.obj_index(id);
        if index == -1 {
            return Ok(());
        }
        let obj = self.obj_at(index);
        let flags = self.ow(obj, 0x28);
        let owns = flags & flag::NO_FREE == 0;
        if flags & flag::VISIBLE == 0 && mode == 0 {
            return Ok(());
        }
        while self.obj_count() - 1 > index {
            self.obj_swap(index, index + 1);
            index += 1;
        }
        if flags & flag::OWN_IMAGE != 0 && mode != 2 && owns {
            let image = self.od(obj, 0x56);
            self.mem.free(image);
            self.set_od(obj, 0x56, 0);
        }
        if flags & flag::OWN_TEXT != 0 && mode != 2 && owns {
            let text = self.od(obj, 0x2e);
            self.mem.free(text);
            self.set_od(obj, 0x2e, 0);
            self.set_od(obj, 0x32, 0);
            self.set_od(obj, 0x36, 0);
        }
        self.menu_free(id);
        let was_visible = self.ow(obj, 0x28) & flag::VISIBLE;
        if mode == 0 {
            let f = self.ow(obj, 0x28) & 0x00fe;
            self.set_ow(obj, 0x28, f);
        }
        if mode == 1 {
            if owns {
                for off in [0x5a, 0x2e, 0x56] {
                    let p = self.od(obj, off);
                    self.mem.free(p);
                }
                self.free_obj_map(obj);
            }
            self.set_obj_count(self.obj_count() - 1);
        }
        if mode == 2 && !owns {
            self.set_obj_count(self.obj_count() - 1);
        }
        let count = self.obj_count();
        if count > 0 {
            let top = self.obj_at(count - 1);
            if self.ow(top, 0x28) & flag::VISIBLE == 0 {
                let mut si = count - 1;
                while si >= 0 {
                    if self.ow(self.obj_at(si), 0x28) & flag::VISIBLE != 0 {
                        break;
                    }
                    si -= 1;
                }
                if si >= 0 {
                    let table = self.mem.d(OBJ_TABLE);
                    self.mem
                        .memswap(ptr_add(table, si * 4), ptr_add(table, (count - 1) * 4), 4);
                }
            }
        }
        if was_visible != 0 && self.w(W_29016) != 0 {
            let saved = self.w(W_27152);
            self.set_w(W_27152, 0);
            self.compose(2)?;
            self.set_w(W_27152, saved);
        }
        Ok(())
    }

    /// `sub_16177(mode)`: close every object (WD, load, quit).
    pub fn obj_close_all(&mut self, mode: u16) -> Result<()> {
        let passes_from = if mode == 1 { 2 } else { mode };
        let saved = self.w(W_29016);
        self.set_w(W_29016, 0);
        let mut pass = passes_from;
        while pass >= mode {
            let ids: Vec<u16> = (0..self.obj_count())
                .map(|i| u16::from(self.ob(self.obj_at(i), 0x2b)))
                .collect();
            for id in ids {
                self.obj_close(id, pass)?;
            }
            if pass == 0 {
                break;
            }
            pass -= 1;
        }
        self.set_w(W_29016, saved);
        self.compose(0)
    }

    /// `sub_1669B(x1, y1, x2, y2, gadgets, title, id)`: create a (hidden)
    /// window in 16x16 cell coordinates.
    pub fn window_create(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        gadgets: u16,
        title: u16,
        id: u16,
    ) -> Result<FarPtr> {
        if self.obj_count() >= MAX_OBJECTS {
            return Err(self.fatal(0x18));
        }
        let previous = self.obj_index(id);
        self.obj_close(id, 1)?;
        let obj = self.obj_at(self.obj_count());
        self.mem.memset(obj, 0, OBJ_SIZE);
        self.set_obj_count(self.obj_count() + 1);
        let (x1, x2) = ((x1 * 2) as i16 as i32, (x2 * 2) as i16 as i32);
        let (y1, y2) = ((y1 * 16) as i16 as i32, (y2 * 16) as i16 as i32);
        if x1 > x2 || y1 > y2 {
            return Err(self.fatal(0x16));
        }
        let mut w = (x2 - x1) as u16;
        let mut h = (y2 - y1) as u16;
        if !(10..=80).contains(&w) {
            w = 10;
        }
        if !(16..=400).contains(&h) {
            h = 16;
        }
        self.set_osw(obj, 0x0a, x1);
        self.set_osw(obj, 0x0c, y1);
        self.set_osw(obj, 0x0e, x1 + i32::from(w));
        self.set_osw(obj, 0x10, y1 + i32::from(h));
        self.set_ow(obj, 0x12, w);
        self.set_ow(obj, 0x14, h);
        self.set_ow(obj, 0x16, 0x0a);
        self.set_ow(obj, 0x18, 0x10);
        self.set_ow(obj, 0x1a, 0x50);
        self.set_ow(obj, 0x1c, 0x190);
        if previous == -1 {
            let templates = self.mem.d(TEMPLATES);
            for t in 0..i32::from(self.w(TEMPLATE_COUNT)) {
                let template = ptr_add(templates, t * OBJ_SIZE as i32);
                if u16::from(self.ob(template, 0x2b)) == id {
                    for off in (0x0a..=0x1c).step_by(2) {
                        let value = self.ow(template, off);
                        self.set_ow(obj, off, value);
                    }
                    break;
                }
            }
        }
        self.set_od(obj, 0, 0);
        self.set_od(obj, 4, 0);
        self.set_ow(obj, 0x1e, 0);
        self.set_ow(obj, 0x20, 0);
        self.set_ob(obj, 0x2a, self.w(FRAME_STYLE) as u8);
        self.set_ob(obj, 0x2b, id as u8);
        self.set_ow(obj, 0x28, flag::VISIBLE);
        self.set_ob(obj, 0x2d, gadgets as u8);
        self.set_ob(obj, 0x2c, title as u8);
        if self.w(W_29008) == 0 {
            let f = self.ow(obj, 0x28) | flag::FRAMELESS;
            self.set_ow(obj, 0x28, f);
            let g = self.ob(obj, 0x2d) & 0x11;
            self.set_ob(obj, 0x2d, g);
        }
        if self.w(W_29004) != 0 && self.od(obj, 0x56) == 0 {
            let f = self.ow(obj, 0x28) | flag::FREE_POSITION;
            self.set_ow(obj, 0x28, f);
        }
        if self.w(W_29002) != 0 {
            let f = self.ow(obj, 0x28) | flag::RESIZABLE;
            self.set_ow(obj, 0x28, f);
        }
        self.obj_close(id, 0)?;
        if previous != -1 && self.ow(self.obj_at(previous), 0x28) & flag::VISIBLE != 0 {
            self.obj_raise(id)?;
        }
        Ok(self.obj_ptr(id))
    }

    /// `sub_1690E`: W1 window definition (optionally mouse-relative).
    pub fn window_define(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        gadgets: u16,
        title: u16,
        id: u16,
    ) -> Result<FarPtr> {
        let (mut x1, mut y1) = (x1, y1);
        let w = x2 - x1;
        let h = y2 - y1;
        if self.w(W_28FFC) != 0 {
            let mut obj = self.obj_ptr(id);
            let mut cx = (i32::from(self.sw(MOUSE_X)) / 8) & !1;
            let mut cy = i32::from(self.sw(MOUSE_Y)) & !0xf;
            if obj != 0 {
                if self.osw(obj, 0x0c) > cy
                    || self.osw(obj, 0x10) < cy
                    || self.osw(obj, 0x0a) > cx
                    || self.osw(obj, 0x0e) < cx
                {
                    obj = 0;
                } else {
                    x1 = self.osw(obj, 0x0a) / 2;
                    y1 = self.osw(obj, 0x0c) / 16;
                }
            }
            if obj == 0 {
                cx += i32::from(self.sw(W_28FFA)) * 2;
                cy += i32::from(self.sw(W_28FF8)) * 16;
                x1 = (cx / 2).max(0);
                y1 = (cy / 16).max(0);
            }
        }
        self.window_create(x1, y1, x1 + w, y1 + h, gadgets, title, id)
    }

    /// `sub_16A0E`: default text window.
    pub fn window_default(&mut self, id: u16) -> Result<FarPtr> {
        let obj = self.window_define(10, 3, 30, 15, 0x17, 0, id)?;
        self.obj_raise(id)?;
        Ok(obj)
    }

    /// `sub_1EA69`: show and raise an object.  Returns the object or null
    /// when it already was the visible top object.
    pub fn obj_raise(&mut self, id: u16) -> Result<FarPtr> {
        let index = self.obj_index(id);
        if index == -1 {
            return Ok(0);
        }
        let obj = self.obj_at(index);
        if self.obj_count() - 1 == index && self.ow(obj, 0x28) & flag::VISIBLE != 0 {
            return Ok(0);
        }
        self.set_w(FORCE_REDRAW, 0);
        let f = self.ow(obj, 0x28) | flag::VISIBLE;
        self.set_ow(obj, 0x28, f);
        let mut i = index;
        while self.obj_count() - 1 > i {
            self.obj_swap(i, i + 1);
            i += 1;
        }
        let cursor = self.cursor_show(0);
        let below = self.obj_count() - 2;
        self.obj_draw_gadgets(below)?;
        self.compute_owners();
        let saved = self.w(W_27152);
        self.set_w(W_27152, 0);
        self.obj_redraw(-1)?;
        self.set_w(W_27152, saved);
        self.cursor_show(cursor);
        self.set_w(TOP_OBJECT, id);
        Ok(obj)
    }

    /// `sub_1E07C(x, y, gadgets?, title, new_id, src_id)`: clone a window
    /// (W9 / WK).  Returns the new object.
    pub fn window_clone(
        &mut self,
        x: u16,
        y: u16,
        gadgets: u16,
        title: u16,
        new_id: u16,
        src_id: u16,
    ) -> Result<FarPtr> {
        let src = self.obj_ptr(src_id);
        if src == 0 || self.obj_count() >= MAX_OBJECTS {
            return Ok(0);
        }
        let mut new_id = new_id;
        if new_id == 1000 {
            let mut candidate = 0x96u16;
            while candidate <= 0xef {
                if self.obj_index(candidate) == -1 {
                    break;
                }
                candidate += 1;
            }
            if candidate > 0xef {
                return Ok(0);
            }
            new_id = candidate;
        }
        let snapshot = self.mem.read_bytes(src, OBJ_SIZE);
        let word = |at: usize| i32::from(i16::from_le_bytes([snapshot[at], snapshot[at + 1]]));
        let x1 = if x == 1000 {
            word(0x0a)
        } else {
            i32::from(x) * 2
        };
        let y1 = if y == 1000 {
            word(0x0c)
        } else {
            i32::from(y) * 16
        };
        let x2 = x1 + word(0x0e) - word(0x0a);
        let y2 = y1 + word(0x10) - word(0x0c);
        let title = if title == 1000 {
            u16::from(snapshot[0x2c])
        } else {
            title
        };
        let gadgets = if gadgets == 1000 { 0x1f } else { gadgets };
        let saved = self.w(W_29008);
        if i32::from(snapshot[0x28]) & i32::from(flag::FRAMELESS) != 0 {
            self.set_w(W_29008, 0);
        }
        let obj = self.window_define(x1 / 2, y1 / 16, x2 / 2, y2 / 16, gadgets, title, new_id)?;
        self.set_w(W_29008, saved);
        self.mem.write_bytes(obj, &snapshot);
        self.set_osw(obj, 0x0a, x1);
        self.set_osw(obj, 0x0e, x2);
        self.set_osw(obj, 0x0c, y1);
        self.set_osw(obj, 0x10, y2);
        let f = self.ow(obj, 0x28) | 9;
        self.set_ow(obj, 0x28, f);
        self.set_ob(obj, 0x2b, new_id as u8);
        self.set_ob(obj, 0x2d, gadgets as u8);
        self.set_ob(obj, 0x2c, title as u8);
        self.set_w(W_27152, 0);
        self.obj_raise(new_id)?;
        Ok(obj)
    }

    /// `sub_1E90B(id, name, keep)`: window showing a PDT image (W6).
    pub fn image_window(&mut self, id: u16, name: &[u8], keep: u16) -> Result<()> {
        let mut obj = self.obj_ptr(id);
        self.pdt_rect(name)?;
        let w = ((self
            .w(PDT_RIGHT)
            .wrapping_sub(self.w(PDT_LEFT))
            .wrapping_add(1))
            & 0xfffe) as i32;
        let h = ((self
            .w(PDT_BOTTOM)
            .wrapping_sub(self.w(PDT_TOP))
            .wrapping_add(1))
            & 0xfff0) as i32;
        if obj == 0 {
            obj = self.window_define(1, 1, 1 + w / 2, 1 + h / 16, 0x17, 0, id)?;
            self.obj_raise(id)?;
        }
        let mut stored = name.to_vec();
        stored.truncate(13);
        self.mem.strcpy_bytes(ptr_add(obj, 0x3a), &stored);
        self.set_ow(obj, 0x08, 3);
        self.set_od(obj, 0, content::IMAGE);
        let image = self.od(obj, 0x56);
        self.mem.free(image);
        self.set_od(obj, 0x56, 0);
        self.set_osw(obj, 0x1e, w);
        self.set_ow(obj, 0x20, 0);
        self.set_osw(obj, 0x16, w);
        self.set_osw(obj, 0x1c, h);
        self.set_osw(obj, 0x18, h);
        if keep != 0 {
            let f = self.ow(obj, 0x28) | flag::OWN_IMAGE;
            self.set_ow(obj, 0x28, f);
        }
        if !(self.osw(obj, 0x12) >= w
            && self.osw(obj, 0x14) == h
            && self.osw(obj, 0x0a) >= 2
            && self.osw(obj, 0x0c) >= 0x10)
        {
            self.obj_close(id, 0)?;
            self.set_ow(obj, 0x0a, 2);
            self.set_osw(obj, 0x0e, w + 2);
            self.set_ow(obj, 0x0c, 0x10);
            self.set_osw(obj, 0x10, h + 0x10);
            self.set_osw(obj, 0x12, w);
            self.set_osw(obj, 0x14, h);
        }
        self.obj_raise(id)?;
        let cursor = self.cursor_show(0);
        self.image_window_draw(-1)?;
        self.cursor_show(cursor);
        Ok(())
    }

    /// `sub_1E76B`: draw the image of a W6 window.
    pub fn image_window_draw(&mut self, index: i32) -> Result<()> {
        let mut i = index;
        let obj = self.obj_active(&mut i);
        if obj == 0 {
            return Ok(());
        }
        let index = i;
        let owner = u16::from(self.ob(obj, 0x2b));
        let (x1, y1, x2, y2) = (
            self.osw(obj, 0x0a),
            self.osw(obj, 0x0c),
            self.osw(obj, 0x0e),
            self.osw(obj, 0x10),
        );
        let mut touched = 0;
        let mut total = 0;
        let mut x = x1;
        while x <= x2 {
            let mut y = y1;
            while y <= y2 {
                let state = self.cell_state(x, y, owner);
                if state == 1 || (state == 3 && self.w(FORCE_REDRAW) == 0) {
                    touched += 1;
                }
                total += 1;
                y += 16;
            }
            x += 2;
        }
        if touched == 0 {
            return Ok(());
        }
        let mut image = self.od(obj, 0x56);
        if image == 0 {
            let name = self.mem.cstr(ptr_add(obj, 0x3a));
            image = self.load_file(&name)?;
            self.set_od(obj, 0x56, image);
        }
        let rect = ptr_add(image, 0x23);
        let rw = self
            .mem
            .rw(ptr_add(rect, 4))
            .wrapping_sub(self.mem.rw(rect));
        let rh = self
            .mem
            .rw(ptr_add(rect, 6))
            .wrapping_sub(self.mem.rw(ptr_add(rect, 2)));
        self.mem.ww(rect, x1 as u16);
        self.mem.ww(ptr_add(rect, 2), y1 as u16);
        self.mem.ww(ptr_add(rect, 4), (x1 as u16).wrapping_add(rw));
        self.mem.ww(ptr_add(rect, 6), (y1 as u16).wrapping_add(rh));
        let saved = self.w(PDT_PALETTE);
        self.set_w(PDT_PALETTE, 0);
        self.pdt_decode(image);
        self.set_w(PDT_PALETTE, saved);
        let flags = self.ow(obj, 0x28);
        let (mut cx1, mut cx2, mut cy1, mut cy2) = (x1, x2, y1, y2);
        if flags & flag::FRAMELESS == 0 {
            cx1 -= 2;
            cx2 += 2;
            cy1 -= 16;
            cy2 += 16;
        }
        let (cx1, cx2, cy1, cy2) = (cx1 / 2, cx2 / 2, cy1 / 16, cy2 / 16);
        if total > touched {
            let map = self.cell_map(self.w(CELL_MAP_SEL));
            for cy in cy1..cy2 {
                for cx in cx1..cx2 {
                    self.mem.wb(ptr_add(map, cy * 40 + cx), 0xf0);
                }
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn _unused(seg: u16) -> FarPtr {
    far(seg, 0)
}
