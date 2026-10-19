//! Mouse-driven window UI (seg014 tail): menus (`W8`), the click service
//! run before every interpreted instruction, window move/resize, and the
//! key/button waits of seg042-seg044.

use anyhow::Result;

use super::Engine;
use super::ds::*;
use super::mem::{FarPtr, ptr_add};
use super::object::{content, flag};

impl Engine {
    /// `sub_213D4`: clear click events and wait until nothing is held.
    pub fn wait_all_released(&mut self) -> Result<()> {
        self.set_w(LEFT_PRESSES, 0);
        self.set_w(RIGHT_PRESSES, 0);
        loop {
            if self.pressed_ascii() == 0 && self.w(LEFT_HELD) == 0 && self.w(RIGHT_HELD) == 0 {
                return Ok(());
            }
            self.idle()?;
        }
    }

    /// `sub_213B6`: wait for Space, Return or a left click.
    pub fn wait_key_or_click(&mut self) -> Result<()> {
        self.wait_all_released()?;
        loop {
            let key = self.pressed_ascii();
            if key == b' ' || key == b'\r' || self.w(LEFT_PRESSES) != 0 {
                return Ok(());
            }
            self.idle()?;
        }
    }

    /// `sub_21384`.
    pub fn wait_button_cycle(&mut self) -> Result<()> {
        while self.w(RIGHT_HELD) != 0 || self.w(LEFT_HELD) != 0 {
            self.idle()?;
        }
        loop {
            if self.key(super::scan::SPACE)
                || self.key(super::scan::RETURN)
                || self.w(RIGHT_HELD) != 0
                || self.w(LEFT_HELD) != 0
            {
                return Ok(());
            }
            self.idle()?;
        }
    }

    /// `sub_1B068`: W8 operand parsing and menu call.
    pub fn menu_command(&mut self) -> Result<()> {
        let dst = self.operand()?;
        let id = self.expr_word()?;
        let width = self.expr_word()?;
        let style = self.expr_word()?;
        let mut items: Vec<FarPtr> = Vec::new();
        loop {
            if items.len() >= 0x14 {
                return Err(self.fatal(0x0e));
            }
            let item = self.expression()?;
            if item == 0 {
                break;
            }
            items.push(item);
        }
        let result = self.menu_select(id, width, style, &mut items)?;
        self.mem.ww(dst, result);
        for item in items {
            if item != 0 {
                self.free_expr(item);
            }
        }
        Ok(())
    }

    /// `sub_1AAF9(id, width, style, items)`: show a menu and wait for a
    /// choice.  Returns the one-based source index or 0 when cancelled.
    pub fn menu_select(
        &mut self,
        id: u16,
        width: u16,
        style: u16,
        items: &mut [FarPtr],
    ) -> Result<u16> {
        let list = self.mem.alloc(0x78);
        let map = self.mem.alloc(0x3c);
        let mut count = 0i32;
        for (k, item) in items.iter_mut().enumerate() {
            let a = self.mem.rb(*item);
            let b = self.mem.rb(ptr_add(*item, 1));
            if a == b'\\' && b == b'0' {
                continue;
            }
            count += 1;
            self.mem.wd(ptr_add(list, (count - 1) * 4), *item);
            self.mem.wd(ptr_add(list, count * 4), 0);
            *item = 0;
            self.mem.ww(ptr_add(map, (count - 1) * 2), k as u16);
        }
        let mut cx = 0i32;
        let mut cy = 0i32;
        let mut obj = self.obj_ptr(id);
        if obj != 0 {
            cx = self.osw(obj, 0x0a) / 2;
            cy = self.osw(obj, 0x0c) / 16;
            self.menu_free(id);
            self.set_ow(obj, 0x26, 0);
        }
        let row_h = if style != 0 { 16 } else { 24 };
        let w_cells = i32::from(width);
        let h_cells = (count * row_h + 15) / 16;
        if self.w(W_28FFC) != 0 && obj != 0 {
            let mx = i32::from(self.sw(MOUSE_X)) / 8 + i32::from(self.sw(W_28FFA)) * 2;
            if self.osw(obj, 0x0a) > mx || self.osw(obj, 0x0a) + w_cells * 2 < mx {
                cx = mx / 2 - 1;
            }
            let my = i32::from(self.sw(W_28FF8)) * 16 + i32::from(self.sw(MOUSE_Y));
            if self.osw(obj, 0x0c) > my || self.osw(obj, 0x0c) + h_cells * 16 < my {
                cy = my / 16 - 1;
            }
        }
        let reuse = obj != 0
            && w_cells * 2 == self.osw(obj, 0x0e) - self.osw(obj, 0x0a)
            && h_cells * 16 == self.osw(obj, 0x10) - self.osw(obj, 0x0c)
            && self.ow(obj, 0x28) & flag::VISIBLE != 0;
        if !reuse {
            let style_saved = self.w(FRAME_STYLE);
            self.set_w(FRAME_STYLE, 0x15);
            let free_saved = self.w(W_29004);
            self.set_w(W_29004, 0);
            obj = self.window_define(cx, cy, cx + w_cells, cy + h_cells, 0x10, 0, id)?;
            self.obj_raise(id)?;
            self.set_w(W_29004, free_saved);
            self.set_w(FRAME_STYLE, style_saved);
            let f = self.ow(obj, 0x28) | flag::VISIBLE;
            self.set_ow(obj, 0x28, f);
        }
        self.set_w(W_2900C, (self.osw(obj, 0x0a) / 2) as u16);
        self.set_w(W_2900A, (self.osw(obj, 0x0c) / 16) as u16);
        self.set_od(obj, 0, content::MENU);
        let bits = (style & 1) | ((self.w(W_28FCC) & 1) << 1);
        let state = self.ow(obj, 0x26) | bits;
        self.set_ow(obj, 0x26, state);
        if style & 1 != 0 {
            let f = self.ow(obj, 0x28) | flag::MENU_ACTIVE;
            self.set_ow(obj, 0x28, f);
        }
        self.set_od(obj, 0x52, list);
        self.set_w(W_28FC6, 1);
        let cursor = self.cursor_show(0);
        self.obj_redraw(-1)?;
        self.cursor_show(cursor);
        self.set_w(W_28FC6, 0);
        while self.w(RIGHT_HELD) != 0 || self.w(LEFT_HELD) != 0 {
            self.idle()?;
        }
        self.set_b(WAIT_OBJECT, id as u8);
        let mut choice = 0u16;
        loop {
            if self.w(RIGHT_HELD) != 0 {
                if self.w(TOP_OBJECT) != id {
                    self.obj_raise(id)?;
                    let cursor = self.cursor_show(0);
                    self.menu_draw(-1)?;
                    self.cursor_show(cursor);
                    while self.w(RIGHT_HELD) != 0 {
                        self.idle()?;
                    }
                    self.set_w(RIGHT_PRESSES, 0);
                    continue;
                }
                choice = 0;
                break;
            }
            if self.w(LEFT_HELD) != 0 {
                let top = self.obj_at(self.obj_count() - 1);
                let mut hit = false;
                if u16::from(self.ob(top, 0x2b)) == id {
                    let left = self.osw(obj, 0x0a);
                    let right = self.osw(obj, 0x0e) - 1;
                    let mx = i32::from(self.sw(MOUSE_X));
                    let my = i32::from(self.sw(MOUSE_Y));
                    if left * 8 <= mx && right * 8 >= mx {
                        let compact = self.ow(obj, 0x26) & 1 != 0;
                        let (mut y, span, step) = if compact {
                            (self.osw(obj, 0x20) + self.osw(obj, 0x0c) + 1, 14, 16)
                        } else {
                            (self.osw(obj, 0x0c) + 6, 19, 24)
                        };
                        let mut row = 0i32;
                        loop {
                            if self.mem.rd(ptr_add(list, row * 4)) == 0 {
                                break;
                            }
                            if my >= y && y + span >= my {
                                if !compact {
                                    let cursor = self.cursor_show(0);
                                    let bottom = y + 19;
                                    self.vram.frame_rect_lines(left, y, right, bottom);
                                    self.cursor_show(cursor);
                                    while self.w(LEFT_HELD) != 0 {
                                        self.idle()?;
                                    }
                                }
                                choice = self.mem.rw(ptr_add(map, row * 2)) + 1;
                                hit = true;
                                break;
                            }
                            y += step;
                            row += 1;
                        }
                    }
                }
                if hit {
                    break;
                }
                self.mouse_service()?;
            }
            if self.ow(obj, 0x28) & flag::MENU_ACTIVE != 0 {
                self.mouse_service()?;
            }
            if choice != 0 {
                break;
            }
            self.idle()?;
        }
        self.mem.free(map);
        let state = self.ow(obj, 0x26) | 4;
        self.set_ow(obj, 0x26, state);
        let f = self.ow(obj, 0x28) & !flag::MENU_ACTIVE;
        self.set_ow(obj, 0x28, f);
        if self.ow(obj, 0x26) & 2 == 0 {
            self.obj_close(id, 0)?;
        }
        self.set_b(WAIT_OBJECT, 0);
        Ok(choice)
    }

    /// `sub_1AFD9`: free a finished menu's item list.
    pub fn menu_free(&mut self, id: u16) {
        let obj = self.obj_ptr(id);
        if obj == 0 || self.ow(obj, 0x26) & 4 == 0 {
            return;
        }
        let list = self.od(obj, 0x52);
        if list != 0 {
            let mut i = 0;
            loop {
                let item = self.mem.rd(ptr_add(list, i * 4));
                if item == 0 {
                    break;
                }
                self.mem.free(item);
                i += 1;
            }
            self.mem.free(list);
        }
        self.set_od(obj, 0x52, 0);
        self.set_od(obj, 4, 0);
        self.set_od(obj, 0, 0);
    }

    /// `sub_1A782`: draw menu rows and the hover highlight.
    pub fn menu_draw(&mut self, index: i32) -> Result<()> {
        let mut idx = index;
        let obj = self.obj_active(&mut idx);
        if obj == 0 {
            return Ok(());
        }
        let index = idx;
        let x1 = self.osw(obj, 0x0a);
        let y1 = self.osw(obj, 0x0c);
        let x2 = self.osw(obj, 0x0e);
        let y2 = self.osw(obj, 0x10);
        let flags = self.ow(obj, 0x28);
        let id = u16::from(self.ob(obj, 0x2b));
        let (fx1, fx2, fy1, fy2) = if flags & flag::FRAMELESS == 0 {
            (x1 - 2, x2 + 2, y1 - 16, y2 + 16)
        } else {
            (x1, x2, y1, y2)
        };
        let mut visible = 0;
        let mut x = fx1;
        while x < fx2 {
            let mut y = fy1;
            while y < fy2 {
                let state = self.cell_state(x, y, id);
                if state == 1 || state == 2 {
                    visible += 1;
                }
                y += 16;
            }
            x += 2;
        }
        let state = self.ow(obj, 0x26);
        let mut hover = 0i32;
        let mut previous = 0i32;
        if state & 5 != 0 {
            previous = i32::from((state & 0xff) >> 3);
            if state & 4 == 0 && self.obj_count() - 1 == idx {
                let mx = i32::from(self.sw(MOUSE_X)) / 8;
                let my = i32::from(self.sw(MOUSE_Y)) & !0xf;
                if x1 <= mx && x2 > mx && y1 <= my && y2 > my {
                    hover = (my - y1) / 16 + 1;
                }
            }
        }
        if visible != 0 && self.w(W_28FC6) != 0 {
            let list = self.od(obj, 0x52);
            if list == 0 {
                return Err(self.fatal(0x1a));
            }
            let style = self.w(GLYPH_STYLE);
            self.set_w(GLYPH_STYLE, 0);
            self.set_ow(obj, 0x22, 0);
            self.set_ow(obj, 0x24, 0);
            let left = x1;
            let right = x2 - 1;
            let mut top = y1 + 6;
            let mut bottom = top + 0x13;
            self.set_w(FORCE_REDRAW, 0);
            self.obj_clear(idx)?;
            let compact = state & 1 != 0;
            let mut row = 0;
            loop {
                let item = self.mem.rd(ptr_add(list, row * 4));
                if item == 0 {
                    break;
                }
                if !compact {
                    let y = self.ow(obj, 0x24).wrapping_add(8);
                    self.set_ow(obj, 0x24, y);
                    let x = self.ow(obj, 0x22).wrapping_add(1);
                    self.set_ow(obj, 0x22, x);
                }
                self.set_od(obj, 0x36, item);
                self.set_od(obj, 0x2e, item);
                self.draw_text(idx)?;
                self.set_od(obj, 0x36, 0);
                self.set_od(obj, 0x2e, 0);
                self.set_od(obj, 0x32, 0);
                if !compact {
                    self.vram.frame_rect(left, top, right, top, 7);
                    self.vram.frame_rect(left, top, left, bottom, 7);
                    self.vram.frame_rect(right, top, right, bottom, 0);
                    self.vram.frame_rect(left, bottom, right, bottom, 0);
                    top += 24;
                    bottom += 24;
                } else {
                    top += 16;
                    bottom += 16;
                }
                row += 1;
            }
            self.set_w(GLYPH_STYLE, style);
            let map = self.mem.d(CELL_MAPS + (self.w(CELL_MAP_SEL) & 1) * 4);
            let mut x = fx1;
            while x < fx2 {
                let mut y = fy1;
                while y < fy2 {
                    self.mem.wb(ptr_add(map, (y / 16) * 40 + x / 2), 0xf0);
                    y += 16;
                }
                x += 2;
            }
            hover = 0;
        }
        if previous != hover && visible != 0 {
            let cursor = self.cursor_show(0);
            let right_cell = self.osw(obj, 0x12) / 2 - 1;
            if previous != 0 {
                self.invert_cells(0, previous - 1, right_cell, previous - 1, id, 0x0f);
            }
            if hover != 0 {
                self.invert_cells(0, hover - 1, right_cell, hover - 1, id, 0x0f);
            }
            self.cursor_show(cursor);
            if state & 4 == 0 {
                let new_state = (self.ow(obj, 0x26) & 0xff07).wrapping_add((hover * 8) as u16);
                self.set_ow(obj, 0x26, new_state);
            }
        }
        Ok(())
    }

    /// `sub_1B2BC`: classify the last left click (see `mouse_service`).
    fn hit_test(&mut self) -> u16 {
        let col = i32::from(self.sw(MOUSE_X)) / 8;
        let y = i32::from(self.sw(MOUSE_Y));
        let count = self.obj_count();
        let mut di = count;
        let mut rect = (0, 0, 0, 0);
        let mut obj = 0;
        while di > 0 {
            obj = self.obj_at(di - 1);
            let flags = self.ow(obj, 0x28);
            let (mut x1, mut y1, mut x2, mut y2) = (
                self.osw(obj, 0x0a),
                self.osw(obj, 0x0c),
                self.osw(obj, 0x0e),
                self.osw(obj, 0x10),
            );
            if flags & flag::FRAMELESS == 0 {
                x1 -= 2;
                x2 += 2;
                y1 -= 16;
                y2 += 16;
            } else if flags & flag::INSET_HIT != 0 {
                x1 += 2;
                x2 -= 2;
                y1 += 16;
                y2 -= 16;
            }
            rect = (x1, y1, x2, y2);
            if flags & flag::VISIBLE != 0 && col >= x1 && col < x2 && y >= y1 && y < y2 {
                break;
            }
            di -= 1;
        }
        if self.w(W_2901A) != 0 && di != count {
            di = 0;
        }
        if di == 0 {
            return 0;
        }
        self.set_w(W_29018, (di - 1) as u16);
        let (x1, top, x2, bottom) = rect;
        let chip = u16::from(self.ob(obj, 0x2a));
        let gadgets = self.ob(obj, 0x2d);
        let id = u16::from(self.ob(obj, 0x2b));
        let chips = self.mem.d(WIN_CHIPS);
        let press = |engine: &mut Engine, x: i32, yy: i32, index: u16| {
            let cursor = engine.w(CURSOR_SHOWN);
            engine.draw_chip(x, yy, chips, index, 1, id);
            engine.cursor_show(cursor);
        };
        if top <= y && top + 16 >= y && x2 - 2 > col && x2 - 4 <= col && gadgets & 1 != 0 {
            press(self, x2 - 4, top, chip + 0x0f);
            return 1;
        }
        if top <= y && top + 16 >= y {
            if x1 + 2 <= col && x1 + 10 >= col {
                return if gadgets & 0x10 != 0 { 2 } else { 0 };
            }
            return if di == count { 0 } else { 9 };
        }
        if bottom >= y && bottom - 16 <= y && x2 - 2 > col && x2 - 4 <= col && gadgets & 2 != 0 {
            press(self, x2 - 4, bottom - 16, chip + 0x14);
            return 3;
        }
        if count - 1 == i32::from(self.w(W_29018)) {
            let bottom_row = bottom >= y && bottom - 16 <= y;
            let arrows = [
                (x2 - 4, x2 - 6, 4u8, 0x13u16, 5u16),
                (x2 - 6, x2 - 8, 4, 0x12, 6),
                (x2 - 8, x2 - 10, 8, 0x11, 7),
                (x2 - 10, x2 - 12, 8, 0x10, 8),
            ];
            let mut result = None;
            for (hi, lo, bit, index, code) in arrows {
                if bottom_row && hi > col && lo <= col && gadgets & bit != 0 {
                    result = Some((lo, index, code));
                }
            }
            if let Some((x, index, code)) = result {
                press(self, x, bottom - 16, chip + index);
                return code;
            }
            return 4;
        }
        9
    }

    /// `sub_1B115`: handle clicks on windows (close boxes, drag, resize,
    /// scroll arrows, text paging).  Called before every instruction.
    pub fn mouse_service(&mut self) -> Result<u16> {
        if self.obj_count() > 0 {
            let top = self.obj_at(self.obj_count() - 1);
            if self.ow(top, 0x28) & 0x101 == 0x101 {
                self.obj_draw_content(-1)?;
            }
        }
        let mut result = 0u16;
        if self.w(LEFT_PRESSES) != 0 {
            let mut si = self.hit_test();
            result = si;
            let obj = self.obj_at(i32::from(self.w(W_29018)));
            let wait = u16::from(self.b(WAIT_OBJECT));
            match si {
                0 | 4 | 9 if si != 9 || wait != 0 => {
                    if wait != 0 {
                        let target = self.obj_ptr(wait);
                        if wait != self.w(TOP_OBJECT) || self.ow(target, 0x28) & flag::VISIBLE == 0
                        {
                            si = 1;
                            self.obj_raise(wait)?;
                        } else if self.od(target, 0x52) == 0 {
                            si = 1;
                            if self.w(TEXT_DONE) != 0 {
                                self.set_b(WAIT_OBJECT, 0);
                                self.set_b(WAIT_FLAG, 0);
                            } else {
                                let cursor = self.cursor_show(0);
                                self.text_scroll_request(5)?;
                                self.cursor_show(cursor);
                            }
                        } else {
                            si = 0;
                        }
                    } else {
                        si = 0;
                    }
                }
                9 => {
                    let id = u16::from(self.ob(obj, 0x2b));
                    if id != self.w(TOP_OBJECT) {
                        self.obj_raise(id)?;
                    }
                }
                1 => {
                    let id = u16::from(self.ob(obj, 0x2b));
                    self.obj_close(id, 0)?;
                }
                2 | 3 => {
                    let index = i32::from(self.w(W_29018));
                    self.window_drag(si, index)?;
                }
                5..=8 => {
                    let direction = si;
                    si = 0;
                    loop {
                        if si == 0 {
                            si = self.text_scroll_request(direction)?;
                        }
                        if self.w(LEFT_HELD) == 0 {
                            break;
                        }
                        self.idle()?;
                    }
                    let cursor = self.cursor_show(0);
                    self.obj_draw_frame(-1)?;
                    self.cursor_show(cursor);
                }
                _ => {}
            }
            if si != 0 {
                while self.w(LEFT_HELD) != 0 {
                    self.idle()?;
                }
            }
        }
        if result != 0 {
            self.set_w(W_28FFE, result);
        }
        self.set_w(LEFT_PRESSES, 0);
        Ok(result)
    }

    /// `sub_1B60E(mode, index)`: interactive window move (2) / resize (3).
    fn window_drag(&mut self, mode: u16, index: i32) -> Result<()> {
        let index = if index == -1 {
            self.obj_count() - 1
        } else {
            index
        };
        if index < 0 || index >= self.obj_count() {
            return Ok(());
        }
        let obj = self.obj_at(index);
        let snap = self.mem.read_bytes(obj, super::object::OBJ_SIZE);
        let sw = |at: usize| i32::from(i16::from_le_bytes([snap[at], snap[at + 1]]));
        let flags = sw(0x28) as u16;
        let id = u16::from(snap[0x2b]);
        if flags & flag::RESIZABLE != 0 && mode == 3 {
            if sw(0x12) == sw(0x1a) {
                self.set_osw(obj, 0x0e, sw(0x0a) + sw(0x16));
                self.set_osw(obj, 0x10, sw(0x0c) + sw(0x18));
            } else {
                self.set_osw(obj, 0x0e, sw(0x0a) + sw(0x1a));
                self.set_osw(obj, 0x10, sw(0x0c) + sw(0x1c));
                if flags & flag::FREE_POSITION != 0 {
                    if self.osw(obj, 0x0e) > 0x4e {
                        let x = self.osw(obj, 0x0a) - (self.osw(obj, 0x0e) - 0x4e);
                        self.set_osw(obj, 0x0a, x);
                    }
                    if sw(0x0a) < 2 {
                        self.set_ow(obj, 0x0a, 2);
                    }
                    if self.osw(obj, 0x10) > 0x180 {
                        let y = self.osw(obj, 0x0c) - (self.osw(obj, 0x10) - 0x180);
                        self.set_osw(obj, 0x0c, y);
                    }
                    if sw(0x0c) < 0x10 {
                        self.set_ow(obj, 0x0c, 0x10);
                    }
                }
            }
            if id != self.w(TOP_OBJECT) {
                self.obj_raise(id)?;
            } else {
                self.compute_owners();
                self.obj_redraw(-1)?;
            }
            return Ok(());
        }
        let cursor = self.cursor_show(0);
        let mut left = sw(0x0a) * 8;
        let mut right = sw(0x0e) * 8;
        let mut top = sw(0x0c);
        let mut bottom = sw(0x10);
        let start_mouse = (i32::from(self.sw(MOUSE_X)), i32::from(self.sw(MOUSE_Y)));
        if flags & flag::FRAMELESS == 0 {
            left -= 16;
            right += 16;
            top -= 16;
            bottom += 16;
        }
        self.vram.xor_outline(left, top, right - 1, bottom - 1);
        let limits = (
            self.w(MOUSE_MIN_X),
            self.w(MOUSE_MAX_X),
            self.w(MOUSE_MIN_Y),
            self.w(MOUSE_MAX_Y),
        );
        let (mut hold_w, mut hold_h) = (0, 0);
        if mode == 2 {
            let mx = i32::from(self.sw(MOUSE_X));
            let my = i32::from(self.sw(MOUSE_Y));
            if flags & flag::FREE_POSITION != 0 {
                let lc = left / 8;
                left = lc;
                self.set_w(MOUSE_MIN_X, (mx - (lc + 6) * 8) as u16);
                self.set_w(MOUSE_MIN_Y, (my - top) as u16);
                self.set_w(MOUSE_MAX_X, (0x27f - ((lc + 4) * 8 - mx)) as u16);
                self.set_w(MOUSE_MAX_Y, (0x18f - (top + 16 - my)) as u16);
                left = lc * 8;
            } else {
                self.set_w(MOUSE_MIN_X, (mx - left) as u16);
                self.set_w(MOUSE_MIN_Y, (my - top) as u16);
                self.set_w(MOUSE_MAX_X, (0x27f - (right - mx)) as u16);
                self.set_w(MOUSE_MAX_Y, (0x18f - (bottom - my)) as u16);
            }
        } else {
            hold_w = right - i32::from(self.sw(MOUSE_X)) + 1;
            hold_h = bottom - i32::from(self.sw(MOUSE_Y)) + 1;
            self.set_w(MOUSE_MIN_X, (left + sw(0x16) * 8 + 0x20 - hold_w) as u16);
            self.set_w(MOUSE_MIN_Y, (top + sw(0x18) + 0x20 - hold_h) as u16);
            self.set_w(MOUSE_MAX_X, (left + sw(0x1a) * 8 + 0x20 - hold_w) as u16);
            self.set_w(MOUSE_MAX_Y, (top + sw(0x1c) + 0x20 - hold_h) as u16);
        }
        if self.sw(MOUSE_MIN_X) < 0 {
            self.set_w(MOUSE_MIN_X, 0);
        }
        if 0x27f - hold_w < i32::from(self.sw(MOUSE_MAX_X)) {
            self.set_w(MOUSE_MAX_X, (0x27f - hold_w) as u16);
        }
        if self.sw(MOUSE_MIN_Y) < 0 {
            self.set_w(MOUSE_MIN_Y, 0);
        }
        if 0x18f - hold_h < i32::from(self.sw(MOUSE_MAX_Y)) {
            self.set_w(MOUSE_MAX_Y, (0x18f - hold_h) as u16);
        }
        if self.sw(MOUSE_MAX_X) < self.sw(MOUSE_MIN_X) {
            let v = self.w(MOUSE_MIN_X);
            self.set_w(MOUSE_MAX_X, v);
        }
        if self.sw(MOUSE_MAX_Y) < self.sw(MOUSE_MIN_Y) {
            let v = self.w(MOUSE_MIN_Y);
            self.set_w(MOUSE_MAX_Y, v);
        }
        let mut last = start_mouse;
        loop {
            let now = (i32::from(self.sw(MOUSE_X)), i32::from(self.sw(MOUSE_Y)));
            if self.w(RIGHT_HELD) != 0 || self.w(LEFT_HELD) == 0 {
                break;
            }
            if now != last {
                let (dx, dy) = (now.0 - last.0, now.1 - last.1);
                self.vram.xor_outline(left, top, right - 1, bottom - 1);
                right += dx;
                bottom += dy;
                if mode == 2 {
                    left += dx;
                    top += dy;
                }
                self.vram.xor_outline(left, top, right - 1, bottom - 1);
                last = now;
            }
            self.idle()?;
        }
        self.vram.xor_outline(left, top, right - 1, bottom - 1);
        left += 8;
        right += 8;
        top += 8;
        bottom += 8;
        let left = (left / 8) & !1;
        let right = (right / 8) & !1;
        let top = top & !0xf;
        let bottom = bottom & !0xf;
        self.set_w(MOUSE_MIN_X, limits.0);
        self.set_w(MOUSE_MAX_X, limits.1);
        self.set_w(MOUSE_MIN_Y, limits.2);
        self.set_w(MOUSE_MAX_Y, limits.3);
        let mut finish_compose = true;
        if mode == 2 {
            if self.osw(obj, 0x0a) == left && self.osw(obj, 0x0c) == top {
                self.obj_raise(id)?;
                finish_compose = false;
            } else {
                let left = left.min(0x4e);
                let top = top.min(0x180);
                self.set_osw(obj, 0x0a, left);
                self.set_osw(obj, 0x0c, top);
                self.set_osw(obj, 0x0e, left + sw(0x12));
                self.set_osw(obj, 0x10, top + sw(0x14));
                if self.ow(obj, 0x28) & flag::FRAMELESS == 0 {
                    for (off, d) in [(0x0a, 2), (0x0e, 2), (0x0c, 16), (0x10, 16)] {
                        let v = self.osw(obj, off) + d;
                        self.set_osw(obj, off, v);
                    }
                }
            }
        }
        if mode == 3 && finish_compose {
            let w = right - left - 4;
            let h = bottom - top - 0x20;
            if self.osw(obj, 0x12) == w && self.osw(obj, 0x14) == h {
                if self.obj_raise(id)? == 0 {
                    self.obj_draw_frame(-1)?;
                }
                self.cursor_show(cursor);
                return Ok(());
            }
            self.set_osw(obj, 0x12, w);
            self.set_osw(obj, 0x0e, left + w + 2);
            self.set_osw(obj, 0x14, h);
            self.set_osw(obj, 0x10, top + h + 16);
            let map = self.od(obj, 0x4e);
            if map != 0 {
                let cols = (w - 4) / 2;
                if self.osw(obj, 0x22) + cols >= i32::from(self.mem.rw(map) as i16) {
                    let v = i32::from(self.mem.rw(map) as i16) - cols;
                    self.set_osw(obj, 0x22, v);
                }
                let rows = (h - 0x20) / 16;
                if self.osw(obj, 0x24) + rows >= i32::from(self.mem.rw(ptr_add(map, 2)) as i16) {
                    let v = i32::from(self.mem.rw(ptr_add(map, 2)) as i16) - rows;
                    self.set_osw(obj, 0x24, v);
                }
                if sw(0x28) as u16 & flag::NO_FREE == 0 {
                    let player = self.mem.d(PLAYER);
                    if player != 0
                        && self.mem.rb(ptr_add(map, 0x27)) == self.mem.rb(ptr_add(player, 0x0c))
                    {
                        let px = self.mem.rw(player);
                        let py = self.mem.rw(ptr_add(player, 2));
                        self.map_center_view(obj, px, py);
                    }
                }
            }
            if self.od(obj, 0x5a) != 0 {
                if self.osw(obj, 0x22) * 2 + w >= sw(0x1a) {
                    let v = ((sw(0x1a) - w) / 2).max(0);
                    self.set_osw(obj, 0x22, v);
                }
                if self.osw(obj, 0x24) * 16 + h >= sw(0x1c) {
                    let v = ((sw(0x1c) - h) / 16).max(0);
                    self.set_osw(obj, 0x24, v);
                }
            }
        }
        if finish_compose {
            let cancelled = self.w(RIGHT_HELD) != 0;
            if cancelled {
                let mut i = self.obj_count() - 2;
                while i >= 0 {
                    self.obj_swap(i, i + 1);
                    i -= 1;
                }
            } else {
                let mut i = index;
                while self.obj_count() - 1 > i {
                    self.obj_swap(i, i + 1);
                    i += 1;
                }
            }
            let below = self.obj_count() - 2;
            self.obj_draw_gadgets(below)?;
            self.set_w(W_27152, 0);
            self.compose(u16::from(cancelled))?;
        }
        self.cursor_show(cursor);
        Ok(())
    }
}

impl super::vram::Vram {
    /// Pressed-button outline drawn when a menu row is clicked.
    pub fn frame_rect_lines(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.frame_rect(left, top, right, top, 0);
        self.frame_rect(left, top, left, bottom, 0);
        self.frame_rect(right, top, right, bottom, 7);
        self.frame_rect(left, bottom, right, bottom, 7);
    }
}
