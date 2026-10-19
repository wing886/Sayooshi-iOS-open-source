//! Graphics primitives of `UK2.EXE` (seg018-seg057): page control,
//! palettes, planar buffer transfers, 16x16 cell ("chip") operations, glyph
//! rasterisation and PDT34 decoding into VRAM.

use anyhow::Result;

use super::Engine;
use super::ds::*;
use super::mem::{FarPtr, ds_ptr, off_of, ptr_add, seg_of};
use super::vram::{PLANE_BYTES, ROW_BYTES};

pub const GBUF_HEADER: usize = 16;
pub const CELL_BYTES: usize = 128;

impl Engine {
    // ---- pages ---------------------------------------------------------

    /// `sub_1F7DA(display, access)`.
    pub fn set_pages(&mut self, display: u16, access: u16) {
        self.vram.display = usize::from(display & 1);
        self.vram.access = usize::from(access & 1);
        self.set_w(DISPLAY_PAGE, display);
        self.set_w(ACCESS_PAGE, access);
        self.vram.dirty = true;
    }

    /// Draw into the hidden page while keeping the displayed one.
    pub fn access_back(&mut self) {
        let display = self.w(DISPLAY_PAGE);
        self.set_pages(display, display ^ 1);
    }

    pub fn access_front(&mut self) {
        let display = self.w(DISPLAY_PAGE);
        self.set_pages(display, display);
    }

    // ---- palette -------------------------------------------------------

    /// `sub_1F676`: load DS:67C into the analog palette.
    pub fn apply_palette(&mut self) -> Result<()> {
        if self.w(PALETTE_LOCK) == 1 {
            return Ok(());
        }
        self.wait_vsync()?;
        for index in 0..16u16 {
            let word = self.w(PALETTE + index * 2);
            let g = (word >> 8) & 0xf;
            let r = (word >> 4) & 0xf;
            let b = word & 0xf;
            self.vram.palette[index as usize] = (g << 8) | (r << 4) | b;
        }
        self.vram.dirty = true;
        Ok(())
    }

    /// `sub_1F6E8`: select a COLOR.TBL bank (entries 0x1000 are kept).
    pub fn select_palette_bank(&mut self, bank: u16) -> Result<()> {
        if bank < 1 || i32::from(bank) > i32::from(self.sw(PALETTE_BANK_COUNT)) {
            return Ok(());
        }
        for index in 0..16u16 {
            let value = self.w(PALETTE + bank * 32 + index * 2);
            if value != 0x1000 {
                self.set_w(PALETTE + index * 2, value);
            }
        }
        self.apply_palette()
    }

    /// `sub_1FCFD`: step every colour of DS:5AC0 towards DS:5AA0 by one per
    /// component, recording each intermediate palette.
    fn palette_steps(&mut self, steps: &mut [[u16; 16]; 16]) -> usize {
        const TARGET: u16 = 0x5AA0;
        const WORK: u16 = 0x5AC0;
        let mut count = 0usize;
        loop {
            let done = (0..16u16).all(|c| self.w(WORK + c * 2) == self.w(TARGET + c * 2));
            if done || count >= 15 {
                return count;
            }
            count += 1;
            for c in (0..16u16).rev() {
                let t = self.w(TARGET + c * 2);
                let v = self.w(WORK + c * 2);
                let step = |cur: i32, want: i32| {
                    if cur == want {
                        cur
                    } else if cur < want {
                        cur + 1
                    } else {
                        cur - 1
                    }
                };
                let tg = i32::from(((t & 0x0f00) as i16) >> 8);
                let tr = i32::from(((t & 0x00f0) as i16) >> 4);
                let tb = i32::from((t & 0x000f) as i16);
                let g = step(i32::from(((v & 0x0f00) as i16) >> 8), tg) & 0xf;
                let r = step(i32::from(((v & 0x00f0) as i16) >> 4), tr) & 0xf;
                let b = step(i32::from((v & 0x000f) as i16), tb) & 0xf;
                let word = ((g << 8) | (r << 4) | b) as u16;
                self.set_w(WORK + c * 2, word);
                steps[count - 1][c as usize] = word;
            }
        }
    }

    /// `sub_1FEFB(wait, bank)`: palette fade to a COLOR.TBL bank.
    pub fn fade_to_bank(&mut self, wait: u16, bank: u16) -> Result<()> {
        if bank < 1 || i32::from(bank) > i32::from(self.sw(PALETTE_BANK_COUNT)) {
            return Ok(());
        }
        // sub_1FE17
        let target = PALETTE_BANKS + (bank - 1) * 32;
        let mut lower = 0;
        let mut higher = 0;
        for c in 0..16u16 {
            let cur = self.sw(PALETTE + c * 2);
            let want = self.sw(target + c * 2);
            if cur < want {
                lower += 1;
            }
            if cur > want {
                higher += 1;
            }
        }
        let mut steps = [[0u16; 16]; 16];
        let (mut start, end, delta): (i32, i32, i32);
        if lower > higher {
            self.mem.memcpy(ds_ptr(0x5AA0), ds_ptr(PALETTE), 32);
            self.mem.memcpy(ds_ptr(0x5AC0), ds_ptr(target), 32);
            let n = self.palette_steps(&mut steps) as i32;
            start = n - 1;
            end = -1;
            delta = -1;
        } else if lower < higher {
            self.mem.memcpy(ds_ptr(0x5AC0), ds_ptr(PALETTE), 32);
            self.mem.memcpy(ds_ptr(0x5AA0), ds_ptr(target), 32);
            let n = self.palette_steps(&mut steps) as i32;
            start = 0;
            end = n;
            delta = 1;
        } else {
            start = 0;
            end = 0;
            delta = 0;
        }
        while start != end {
            self.wait_ticks(u32::from(wait))?;
            for c in 0..16u16 {
                self.set_w(PALETTE + c * 2, steps[start as usize][c as usize]);
            }
            self.apply_palette()?;
            start += delta;
        }
        self.select_palette_bank(bank)
    }

    // ---- planar buffers ("GBUF": 16-byte header + rows of 4 planes) -----

    /// `sub_1FA7C`: read an inclusive byte/line rectangle of `page` into `buf`.
    pub fn gbuf_get(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, page: u16, buf: FarPtr) {
        let width = (x2 - x1 + 1).max(0) as usize;
        let rows = (y2 - y1 + 1).max(0) as usize;
        self.mem.ww(ptr_add(buf, 6), y1 as u16);
        self.mem.ww(ptr_add(buf, 4), x1 as u16);
        let stride = usize::from(self.mem.rw(buf));
        let mut data = GBUF_HEADER;
        let mut skip = stride as i64 - width as i64;
        let sub_w = usize::from(self.mem.rw(ptr_add(buf, 8)));
        if sub_w > 0 {
            let sub_y = usize::from(self.mem.rw(ptr_add(buf, 14)));
            let sub_x = usize::from(self.mem.rw(ptr_add(buf, 12)));
            data += sub_y * sub_w + sub_x;
            skip -= sub_x as i64;
        }
        let page = usize::from(page & 1);
        let start = (y1.max(0) as usize) * ROW_BYTES + x1.max(0) as usize;
        let planes: Vec<Vec<u8>> = (0..4).map(|p| self.vram.plane(page, p).to_vec()).collect();
        let out = self.mem.slice_mut(buf);
        let mut offset = start;
        for _ in 0..rows {
            for plane in &planes {
                for x in 0..width {
                    let value = plane.get(offset + x).copied().unwrap_or(0);
                    if let Some(slot) = out.get_mut(data) {
                        *slot = value;
                    }
                    data += 1;
                }
                data = (data as i64 + skip) as usize;
            }
            offset += ROW_BYTES;
        }
    }

    /// `sub_1FBDB`: allocate a buffer and read the rectangle into it.
    pub fn gbuf_alloc_get(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, page: u16) -> FarPtr {
        let width = (x2 - x1 + 1).max(0) as usize;
        let rows = (y2 - y1 + 1).max(0) as usize;
        let buf = self.mem.alloc(width * rows * 4 + GBUF_HEADER);
        self.mem.ww(buf, width as u16);
        self.mem.ww(ptr_add(buf, 2), rows as u16);
        self.gbuf_get(x1, y1, x2, y2, page, buf);
        buf
    }

    /// `sub_1FA2E`/`sub_1F8F9`: write a buffer at byte column `x`, line `y`.
    pub fn gbuf_put(&mut self, x: i32, y: i32, buf: FarPtr, page: u16) {
        if buf == 0 {
            return;
        }
        let header = self.mem.read_bytes(buf, GBUF_HEADER);
        let word = |at: usize| u16::from_le_bytes([header[at], header[at + 1]]) as usize;
        let (width, rows, skip, mut data) = if word(8) == 0 {
            (word(0), word(2), 0usize, GBUF_HEADER)
        } else {
            (
                word(8),
                word(10),
                word(0) - word(8),
                GBUF_HEADER + word(12) + word(14) * word(0) * 4,
            )
        };
        let source = self.mem.slice(buf).to_vec();
        let page = usize::from(page & 1);
        let mut offset = (y.max(0) as usize) * ROW_BYTES + x.max(0) as usize;
        for _ in 0..rows {
            for plane in 0..4 {
                for column in 0..width {
                    let value = source.get(data + column).copied().unwrap_or(0);
                    if offset + column < PLANE_BYTES {
                        self.vram.set_page(page, plane, offset + column, value);
                    }
                }
                data += width + skip;
            }
            offset += ROW_BYTES;
        }
    }

    /// `sub_2098E`: merge the access page (at the buffer's own position)
    /// into `buf`, treating colour 8 as transparent.
    pub fn gbuf_merge_sprite(&mut self, buf: FarPtr) {
        let width = usize::from(self.mem.rw(buf));
        let rows = usize::from(self.mem.rw(ptr_add(buf, 2)));
        let x = usize::from(self.mem.rw(ptr_add(buf, 4)));
        let y = usize::from(self.mem.rw(ptr_add(buf, 6)));
        let access = self.vram.access;
        let planes: Vec<Vec<u8>> = (0..4)
            .map(|p| self.vram.plane(access, p).to_vec())
            .collect();
        let out = self.mem.slice_mut(buf);
        let mut data = GBUF_HEADER;
        for row in 0..rows {
            let line = (y + row) * ROW_BYTES + x;
            for column in 0..width {
                let at = line + column;
                let p: [u8; 4] = std::array::from_fn(|k| planes[k].get(at).copied().unwrap_or(0));
                let mask = !(p[0] | p[1] | p[2]) & p[3];
                if mask == 0xff {
                    continue;
                }
                for k in 0..4 {
                    let slot = data + k * width + column;
                    let Some(dst) = out.get_mut(slot) else {
                        continue;
                    };
                    if mask == 0 {
                        *dst = p[k];
                    } else if k < 3 {
                        *dst = (*dst & mask) | p[k];
                    } else {
                        *dst = (*dst & mask) | (p[k] & !mask);
                    }
                }
            }
            data += width * 4;
        }
    }

    // ---- 16x16 cells -----------------------------------------------------

    /// `sub_234AC`: grab `w` byte columns x `h` lines of cells from the
    /// access page into consecutive 128-byte cells starting at `index`.
    pub fn cells_grab(&mut self, x: u16, y: u16, w: u16, h: u16, buf: FarPtr, index: u16) {
        let cols = w / 2;
        let cell_rows = h / 16;
        let mut cell = u32::from(index);
        for cy in 0..cell_rows {
            for cx in 0..cols {
                let base = (usize::from(y) + usize::from(cy) * 16) * ROW_BYTES
                    + usize::from(x)
                    + usize::from(cx) * 2;
                for (slot, plane) in [3usize, 0, 1, 2].into_iter().enumerate() {
                    for row in 0..16 {
                        let at = base + row * ROW_BYTES;
                        let b0 = self.vram.get(plane, at);
                        let b1 = self.vram.get(plane, at + 1);
                        let dst = ptr_add(buf, (cell * 128) as i32 + (row * 8 + slot * 2) as i32);
                        self.mem.wb(dst, b0);
                        self.mem.wb(ptr_add(dst, 1), b1);
                    }
                }
                cell += 1;
            }
        }
    }

    /// `sub_2342D`: allocate a cell buffer for a rectangle and grab it.
    pub fn cells_alloc_grab(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) -> FarPtr {
        let w = x2.wrapping_sub(x1).wrapping_add(1);
        let h = y2.wrapping_sub(y1).wrapping_add(1);
        let buf = self.mem.alloc(usize::from(w) * usize::from(h) * 4);
        self.cells_grab(x1, y1, w, h, buf, 0);
        buf
    }

    /// Put one cell on the access page. `mode`: 1 = `sub_23158` (keep the
    /// intensity plane), 2 = `sub_230C4` (all planes), 3 = `sub_231DE`
    /// (intensity plane cleared).
    pub fn cell_put(&mut self, x: u16, y: u16, buf: FarPtr, index: u16, mode: u16) {
        let base = usize::from(y) * ROW_BYTES + usize::from(x);
        let src = ptr_add(buf, i32::from(index) * 128);
        for (slot, plane) in [3usize, 0, 1, 2].into_iter().enumerate() {
            if plane == 3 && mode == 1 {
                continue;
            }
            for row in 0..16 {
                let at = base + row * ROW_BYTES;
                let (b0, b1) = if plane == 3 && mode == 3 {
                    (0, 0)
                } else {
                    let p = ptr_add(src, (row * 8 + slot * 2) as i32);
                    (self.mem.rb(p), self.mem.rb(ptr_add(p, 1)))
                };
                self.vram.set(plane, at, b0);
                self.vram.set(plane, at + 1, b1);
            }
        }
    }

    /// `sub_233DD`: copy a 128-byte cell.
    pub fn cell_copy(&mut self, dst: FarPtr, src: FarPtr, index: u16) {
        self.mem
            .memcpy(dst, ptr_add(src, i32::from(index) * 128), CELL_BYTES);
    }

    /// `sub_23272`: overlay with the source intensity plane as keep-mask.
    pub fn cell_overlay_mask(&mut self, dst: FarPtr, src: FarPtr, index: u16) {
        let src = ptr_add(src, i32::from(index) * 128);
        for row in 0..16i32 {
            let s = ptr_add(src, row * 8);
            let d = ptr_add(dst, row * 8);
            let mask = self.mem.rw(s);
            let e = self.mem.rw(d) & mask;
            self.mem.ww(d, e);
            for k in 1..4 {
                let v = (self.mem.rw(ptr_add(d, k * 2)) & mask) | self.mem.rw(ptr_add(s, k * 2));
                self.mem.ww(ptr_add(d, k * 2), v);
            }
        }
    }

    /// `sub_232E7`: take the source where either intensity plane is set.
    pub fn cell_overlay_either(&mut self, dst: FarPtr, src: FarPtr, index: u16) {
        let src = ptr_add(src, i32::from(index) * 128);
        for row in 0..16i32 {
            let s = ptr_add(src, row * 8);
            let d = ptr_add(dst, row * 8);
            let mask = self.mem.rw(s) | self.mem.rw(d);
            for k in 1..4 {
                let v = (self.mem.rw(ptr_add(s, k * 2)) & mask)
                    | (self.mem.rw(ptr_add(d, k * 2)) & !mask);
                self.mem.ww(ptr_add(d, k * 2), v);
            }
        }
    }

    /// `sub_2336E`: AND the intensity planes, OR the source colour planes
    /// where the destination intensity was set.
    pub fn cell_overlay_and(&mut self, dst: FarPtr, src: FarPtr, index: u16) {
        let src = ptr_add(src, i32::from(index) * 128);
        for row in 0..16i32 {
            let s = ptr_add(src, row * 8);
            let d = ptr_add(dst, row * 8);
            let keep = self.mem.rw(d);
            let e = self.mem.rw(s) & keep;
            self.mem.ww(d, e);
            for k in 1..4 {
                let v = self.mem.rw(ptr_add(d, k * 2)) | (self.mem.rw(ptr_add(s, k * 2)) & keep);
                self.mem.ww(ptr_add(d, k * 2), v);
            }
        }
    }

    // ---- whole-page effects ----------------------------------------------

    /// `sub_20CA7`: copy planes from one page to another.
    pub fn copy_page(&mut self, src: u16, dst: u16, planes: u16) {
        for plane in 0..4 {
            if planes & (1 << plane) != 0 {
                let data = *self.vram.plane(usize::from(src), plane);
                *self.vram.plane_mut(usize::from(dst), plane) = data;
            }
        }
    }

    /// `sub_20039`: interlaced dissolve copy of the back page to the front.
    pub fn dissolve(&mut self, width: u16, height: u16, dst: u16, src: u16) -> Result<()> {
        let front = usize::from(self.w(DISPLAY_PAGE) & 1);
        let back = front ^ 1;
        let mut table = 0x7F0u16;
        for _pass in 0..8 {
            for odd in 0..2usize {
                let first = self.b(table) as i8 as i32;
                table += 1;
                let mut row = first;
                while row < i32::from(height) {
                    for plane in 0..4 {
                        let mut column = odd;
                        while column < usize::from(width) {
                            let s = usize::from(src) + row as usize * ROW_BYTES + column;
                            let d = usize::from(dst) + row as usize * ROW_BYTES + column;
                            let value = self.vram.get_page(back, plane, s);
                            self.vram.set_page(front, plane, d, value);
                            column += 2;
                        }
                    }
                    row += 8;
                }
                self.present();
                self.wait_ticks(2)?;
            }
        }
        let display = self.w(DISPLAY_PAGE);
        self.set_pages(display, display);
        Ok(())
    }

    // ---- glyphs -----------------------------------------------------------

    /// `sub_21FA5(str, x, y, colour, bold)`: draw one character at pixel
    /// position `x`, line `y`.  Returns 1 for a half-width character.
    pub fn draw_glyph(&mut self, text: &[u8], x: i32, y: i32, colour: u16, bold: i32) -> u16 {
        let c = text.first().copied().unwrap_or(0);
        let foreign = c == super::font::FOREIGN_GLYPH && text.len() >= 3;
        let half = !foreign && (c < 0x80 || (0xa0..0xc0).contains(&c));
        self.set_w(GLYPH_HALF, u16::from(half));
        let mut glyph = [0u8; 32];
        if half && self.mem.d(ANK_FONT) != 0 {
            let index = if c < 0xa0 {
                u16::from(c).wrapping_sub(0x20)
            } else {
                u16::from(c) - 0x40
            };
            let src = ANK_FONT_DATA.wrapping_add(index.wrapping_mul(16));
            for row in 0..16u16 {
                glyph[row as usize] = self.b(src.wrapping_add(row));
            }
        } else if half {
            // ANK ROM fallback (only reachable before kana.pdt1 is loaded).
            glyph[..16].fill(0);
        } else if foreign {
            glyph = self.foreign_glyph(text[1], text[2]);
        } else {
            let trail = text.get(1).copied().unwrap_or(0);
            glyph = self.kanji.sjis_glyph(c, trail);
        }
        self.raster_glyph(&glyph, x, y, colour, bold);
        u16::from(half)
    }

    /// `sub_220A6`: shift a 16-line glyph into place and apply the raster op.
    fn raster_glyph(&mut self, glyph: &[u8; 32], x: i32, y: i32, colour: u16, bold: i32) {
        let mut glyph = *glyph;
        if self.w(GLYPH_HALF) != 0 {
            let half: [u8; 16] = glyph[..16].try_into().unwrap();
            for row in 0..16 {
                glyph[row * 2] = half[row];
                glyph[row * 2 + 1] = 0;
            }
        }
        let column = x.div_euclid(8);
        let mut shift = x.rem_euclid(8) as u32;
        let colour = self.b(GLYPH_COLOUR_MAP + (colour & 0xf));
        let mut bold = bold;
        loop {
            let mut rows = [[0u8; 3]; 16];
            for row in 0..16 {
                let w = (u32::from(glyph[row * 2]) << 8) | u32::from(glyph[row * 2 + 1]);
                let shifted = (w << 8) >> shift;
                rows[row] = [(shifted >> 16) as u8, (shifted >> 8) as u8, shifted as u8];
            }
            let op = self.w(GLYPH_OP);
            let bytes = if op == 2 || op == 4 { 2 } else { 3 };
            if (1..=4).contains(&op) {
                for plane in 0..4 {
                    let set = colour & (1 << plane) != 0;
                    for row in 0..16 {
                        let line = y + row as i32;
                        if !(0..400).contains(&line) {
                            continue;
                        }
                        for k in 0..bytes {
                            let col = column + k as i32;
                            if !(0..80).contains(&col) {
                                continue;
                            }
                            let at = line as usize * ROW_BYTES + col as usize;
                            let bits = rows[row][k];
                            let old = self.vram.get(plane, at);
                            let new = match op {
                                3 => {
                                    if set {
                                        old
                                    } else {
                                        old | bits
                                    }
                                }
                                _ => {
                                    if set {
                                        old | bits
                                    } else {
                                        old & !bits
                                    }
                                }
                            };
                            self.vram.set(plane, at, new);
                        }
                    }
                }
            }
            if bold <= 0 {
                break;
            }
            bold -= 1;
            shift += 1;
        }
    }

    // ---- PDT34 --------------------------------------------------------------

    /// `sub_2276B`: read a whole resource into a new heap block.
    pub fn load_file(&mut self, name: &[u8]) -> Result<FarPtr> {
        let bytes = self.read_resource(name)?;
        Ok(self.mem.alloc_bytes(&bytes))
    }

    /// `sub_22893`: decode a PDT held in memory into the access page and
    /// publish its rectangle.
    pub fn pdt_decode(&mut self, pdt: FarPtr) {
        if self.mem.rb(pdt) == 0x34 {
            self.pdt_decode_body(ptr_add(pdt, 1));
        }
        self.publish_pdt_rect();
    }

    fn publish_pdt_rect(&mut self) {
        let rect = self.mem.d(super::ds::l(0x29F3A));
        let left = self.mem.rw(rect);
        let top = self.mem.rw(ptr_add(rect, 2));
        let right = self.mem.rw(ptr_add(rect, 4));
        let bottom = self.mem.rw(ptr_add(rect, 6));
        self.set_w(PDT_LEFT, left);
        self.set_w(PDT_RIGHT, right);
        self.set_w(PDT_TOP, top);
        self.set_w(PDT_BOTTOM, bottom);
    }

    /// `sub_226B7` / `sub_225BE`.
    fn pdt_decode_body(&mut self, data: FarPtr) {
        if self.w(PDT_PALETTE) == 1 {
            self.mem.memcpy(ds_ptr(PALETTE_BANKS), data, 32);
        }
        let marker4 = self.mem.rb(ptr_add(data, 0x20));
        let marker3 = self.mem.rb(ptr_add(data, 0x21));
        let rect = ptr_add(data, 0x22);
        self.mem.set_d(super::ds::l(0x29F3A), rect);
        let left = self.mem.rw(rect);
        let top = self.mem.rw(ptr_add(rect, 2));
        let right = self.mem.rw(ptr_add(rect, 4));
        let bottom = self.mem.rw(ptr_add(rect, 6));
        let width = u8::try_from(right.wrapping_sub(left).wrapping_add(1) & 0xff).unwrap_or(0);
        let pairs = ((i32::from(bottom as i16) - i32::from(top as i16)) / 2 + 1) as u8;
        let start = usize::from(top) * ROW_BYTES + usize::from(left);
        let bytes = self.mem.slice(ptr_add(data, 0x2a)).to_vec();
        let mut cursor = 0usize;
        let mut next = |cursor: &mut usize| {
            let value = bytes.get(*cursor).copied().unwrap_or(0);
            *cursor += 1;
            value
        };
        for plane in 0..4 {
            for column in 0..usize::from(width) {
                let mut at = start + column;
                let mut done = 0u32;
                while done < u32::from(pairs) {
                    let first = next(&mut cursor);
                    let (mut run, top_byte, bottom_byte) = if first == marker4 {
                        let run = next(&mut cursor);
                        let a = next(&mut cursor);
                        let b = next(&mut cursor);
                        (run, a, b)
                    } else if first == marker3 {
                        let run = next(&mut cursor);
                        let a = next(&mut cursor);
                        (run, a, a)
                    } else {
                        (1, first, next(&mut cursor))
                    };
                    while run > 0 {
                        run -= 1;
                        self.vram.set(plane, at, top_byte);
                        self.vram.set(plane, at + ROW_BYTES, bottom_byte);
                        at += ROW_BYTES * 2;
                        done += 1;
                    }
                }
            }
        }
    }

    /// `sub_22952`: load and display a PDT on the access page.
    pub fn pdt_show(&mut self, name: &[u8]) -> Result<()> {
        if name.first().copied().unwrap_or(0) == 0 {
            return Ok(());
        }
        let buf = self.load_file(name)?;
        self.pdt_decode(buf);
        self.mem.free(buf);
        Ok(())
    }

    /// `sub_228D0`: read only a PDT's rectangle.
    pub fn pdt_rect(&mut self, name: &[u8]) -> Result<()> {
        let bytes = self.read_resource(name)?;
        if bytes.first() == Some(&0x34) && bytes.len() >= 0x2b {
            let word = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
            self.set_w(PDT_LEFT, word(0x23));
            self.set_w(PDT_RIGHT, word(0x27));
            self.set_w(PDT_TOP, word(0x25));
            self.set_w(PDT_BOTTOM, word(0x29));
        }
        Ok(())
    }

    /// `sub_1EDC4(name, buffer)`: show a PDT on the access page and cut its
    /// rectangle into 16x16 cells (allocating when `buffer` is null).
    pub fn pdt_to_cells(&mut self, name: &[u8], buffer: FarPtr) -> Result<FarPtr> {
        if name.first().copied().unwrap_or(0) == 0 {
            return Ok(0);
        }
        let cursor = self.cursor_show(0);
        self.pdt_show(name)?;
        let (left, right, top, bottom) = (
            self.w(PDT_LEFT),
            self.w(PDT_RIGHT),
            self.w(PDT_TOP),
            self.w(PDT_BOTTOM),
        );
        let buffer = if buffer == 0 {
            self.cells_alloc_grab(left, top, right, bottom)
        } else {
            self.cells_grab(
                left,
                top,
                right.wrapping_sub(left).wrapping_add(1),
                bottom.wrapping_sub(top).wrapping_add(1),
                buffer,
                0,
            );
            buffer
        };
        self.cursor_show(cursor);
        Ok(buffer)
    }

    /// `sub_21A38`: show (1) / hide (0) the software cursor; returns the
    /// previous state.
    pub fn cursor_show(&mut self, state: u16) -> u16 {
        let previous = self.w(CURSOR_SHOWN);
        if self.w(CURSOR_ENABLED) != 0 && state != previous {
            self.set_w(CURSOR_SHOWN, state);
            self.set_w(CURSOR_SHOWN2, state);
            self.vram.dirty = true;
        }
        previous
    }
}

#[allow(dead_code)]
fn _assert_ptr_helpers(ptr: FarPtr) -> (u16, u16) {
    (seg_of(ptr), off_of(ptr))
}
