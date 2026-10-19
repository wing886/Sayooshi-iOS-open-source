//! PC-98 640x400 16-colour graphics VRAM.
//!
//! Two pages of four bit planes each.  Plane order follows the segment order
//! used by `UK2.EXE`: `A800` (blue, bit 0), `B000` (red, bit 1), `B800`
//! (green, bit 2) and `E000` (intensity, bit 3).  The analog palette uses the
//! hardware `0xGRB` nibble order (ports A8h/AAh=G/ACh=R/AEh=B).

pub const ROW_BYTES: usize = 80;
pub const ROWS: usize = 400;
pub const PLANE_BYTES: usize = ROW_BYTES * ROWS;
pub const SCREEN_W: usize = 640;
pub const SCREEN_H: usize = 400;

/// Segment of each plane as used by the original code.
pub const PLANE_SEGS: [u16; 4] = [0xA800, 0xB000, 0xB800, 0xE000];

pub fn plane_index(seg: u16) -> usize {
    match seg {
        0xA800 => 0,
        0xB000 => 1,
        0xB800 => 2,
        _ => 3,
    }
}

#[derive(Clone)]
pub struct Vram {
    pub pages: Box<[[[u8; PLANE_BYTES]; 4]; 2]>,
    /// Page selected through port A6h (CPU access).
    pub access: usize,
    /// Page selected through port A4h (display).
    pub display: usize,
    /// Hardware palette (0xGRB words).
    pub palette: [u16; 16],
    pub graphics_on: bool,
    pub dirty: bool,
}

impl Default for Vram {
    fn default() -> Self {
        Self::new()
    }
}

impl Vram {
    pub fn new() -> Self {
        Self {
            pages: Box::new([[[0; PLANE_BYTES]; 4]; 2]),
            access: 0,
            display: 0,
            palette: [0; 16],
            graphics_on: true,
            dirty: true,
        }
    }

    #[inline]
    pub fn plane(&self, page: usize, plane: usize) -> &[u8; PLANE_BYTES] {
        &self.pages[page & 1][plane]
    }

    #[inline]
    pub fn plane_mut(&mut self, page: usize, plane: usize) -> &mut [u8; PLANE_BYTES] {
        if page & 1 == self.display {
            self.dirty = true;
        }
        &mut self.pages[page & 1][plane]
    }

    #[inline]
    pub fn get(&self, plane: usize, offset: usize) -> u8 {
        self.pages[self.access][plane]
            .get(offset)
            .copied()
            .unwrap_or(0)
    }

    #[inline]
    pub fn set(&mut self, plane: usize, offset: usize, value: u8) {
        let access = self.access;
        if access == self.display {
            self.dirty = true;
        }
        if let Some(slot) = self.pages[access][plane].get_mut(offset) {
            *slot = value;
        }
    }

    #[inline]
    pub fn get_page(&self, page: usize, plane: usize, offset: usize) -> u8 {
        self.pages[page & 1][plane]
            .get(offset)
            .copied()
            .unwrap_or(0)
    }

    #[inline]
    pub fn set_page(&mut self, page: usize, plane: usize, offset: usize, value: u8) {
        if page & 1 == self.display {
            self.dirty = true;
        }
        if let Some(slot) = self.pages[page & 1][plane].get_mut(offset) {
            *slot = value;
        }
    }

    /// Palette index of one pixel on a page.
    pub fn pixel(&self, page: usize, x: usize, y: usize) -> u8 {
        let offset = y * ROW_BYTES + x / 8;
        let bit = 7 - (x & 7);
        let mut colour = 0;
        for plane in 0..4 {
            colour |= ((self.pages[page & 1][plane][offset] >> bit) & 1) << plane;
        }
        colour
    }

    pub fn set_pixel(&mut self, page: usize, x: usize, y: usize, colour: u8) {
        let offset = y * ROW_BYTES + x / 8;
        let mask = 0x80u8 >> (x & 7);
        for plane in 0..4 {
            let slot = &mut self.pages[page & 1][plane][offset];
            if colour & (1 << plane) != 0 {
                *slot |= mask;
            } else {
                *slot &= !mask;
            }
        }
        if page & 1 == self.display {
            self.dirty = true;
        }
    }

    pub fn palette_rgb(word: u16) -> [u8; 3] {
        let g = ((word >> 8) & 0xf) as u8;
        let r = ((word >> 4) & 0xf) as u8;
        let b = (word & 0xf) as u8;
        [r * 0x11, g * 0x11, b * 0x11]
    }

    /// Renders the displayed page to RGBA.
    pub fn render_rgba(&self, out: &mut Vec<u8>) {
        out.resize(SCREEN_W * SCREEN_H * 4, 0);
        let colours: Vec<[u8; 3]> = self.palette.iter().map(|w| Self::palette_rgb(*w)).collect();
        let page = &self.pages[self.display];
        for offset in 0..PLANE_BYTES {
            let p0 = page[0][offset];
            let p1 = page[1][offset];
            let p2 = page[2][offset];
            let p3 = page[3][offset];
            for bit in 0..8 {
                let shift = 7 - bit;
                let index = ((p0 >> shift) & 1)
                    | (((p1 >> shift) & 1) << 1)
                    | (((p2 >> shift) & 1) << 2)
                    | (((p3 >> shift) & 1) << 3);
                let rgb = if self.graphics_on {
                    colours[index as usize]
                } else {
                    [0, 0, 0]
                };
                let at = (offset * 8 + bit) * 4;
                out[at..at + 3].copy_from_slice(&rgb);
                out[at + 3] = 255;
            }
        }
    }

    /// `sub_1FF77`/`sub_1FF8D`: clear the selected planes of the access page.
    pub fn clear_planes(&mut self, mask: u16) {
        let access = self.access;
        for plane in 0..4 {
            if mask & (1 << plane) != 0 {
                self.plane_mut(access, plane).fill(0);
            }
        }
    }

    /// `sub_20536`: copy `rows` rows of `width` bytes inside one page
    /// (all planes) from `src` to `dst` byte offsets.
    pub fn copy_rows(&mut self, page: usize, dst: usize, src: usize, width: usize, rows: usize) {
        for row in 0..rows {
            for plane in 0..4 {
                for x in 0..width {
                    let s = src + row * ROW_BYTES + x;
                    let d = dst + row * ROW_BYTES + x;
                    if s < PLANE_BYTES && d < PLANE_BYTES {
                        let value = self.pages[page & 1][plane][s];
                        self.set_page(page, plane, d, value);
                    }
                }
            }
        }
    }

    /// `sub_20B56`: invert the planes in `planes` over an inclusive byte/line
    /// rectangle on the access page.
    pub fn invert(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, planes: u16) {
        for y in y1.max(0)..=y2.min(ROWS as i32 - 1) {
            for x in x1.max(0)..=x2.min(ROW_BYTES as i32 - 1) {
                let offset = y as usize * ROW_BYTES + x as usize;
                for plane in 0..4 {
                    if planes & (1 << plane) != 0 {
                        let value = !self.get(plane, offset);
                        self.set(plane, offset, value);
                    }
                }
            }
        }
    }

    /// `sub_20644`: draw a rectangular frame (byte-column horizontal
    /// coordinates) in `colour` on the access page.  The four-pixel inset of
    /// the vertical edges matches the executable's edge masks.
    pub fn frame_rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, colour: u8) {
        if !(0..=15).contains(&colour)
            || x2 < x1
            || !(0..=0x4f).contains(&x1)
            || x2 > 0x4f
            || y2 < y1
            || !(0..=0x18f).contains(&y1)
            || y2 > 0x18f
        {
            return;
        }
        // DS:822 maps a logical GRB colour to the plane bit order.
        const MAP: [u8; 16] = [0, 4, 2, 6, 1, 5, 3, 7, 8, 12, 10, 14, 9, 13, 11, 15];
        let colour = MAP[colour as usize];
        for y in y1..=y2 {
            let edge_row = y == y1 || y == y2;
            for x in x1..=x2 {
                let mask = if x == x1 {
                    if edge_row && x1 < x2 { 0x0f } else { 0x08 }
                } else if x == x2 {
                    if edge_row && x1 < x2 { 0xf0 } else { 0x10 }
                } else if edge_row && x1 < x2 {
                    0xff
                } else {
                    continue;
                };
                let offset = y as usize * ROW_BYTES + x as usize;
                for plane in 0..4 {
                    let value = self.get(plane, offset);
                    let value = if colour & (1 << plane) != 0 {
                        value | mask
                    } else {
                        value & !mask
                    };
                    self.set(plane, offset, value);
                }
            }
        }
    }

    /// `sub_20578`: GDC complement-mode rectangle outline (drag/resize
    /// feedback) drawn on the display page.  Drawing it twice restores VRAM.
    pub fn xor_outline(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let clamp_x = |v: i32| v.clamp(0, SCREEN_W as i32 - 1);
        let clamp_y = |v: i32| v.clamp(0, SCREEN_H as i32 - 1);
        let (x1, y1, x2, y2) = (clamp_x(x1), clamp_y(y1), clamp_x(x2), clamp_y(y2));
        if x1 == x2 || y1 == y2 {
            return;
        }
        let page = self.display;
        let mut flip = |x: i32, y: i32, vram: &mut Self| {
            let offset = y as usize * ROW_BYTES + x as usize / 8;
            let mask = 0x80u8 >> (x as usize & 7);
            for plane in 0..3 {
                let value = vram.pages[page][plane][offset] ^ mask;
                vram.set_page(page, plane, offset, value);
            }
        };
        let (lx, hx) = (x1.min(x2), x1.max(x2));
        let (ly, hy) = (y1.min(y2), y1.max(y2));
        for x in lx..=hx {
            flip(x, ly, self);
            flip(x, hy, self);
        }
        for y in ly + 1..hy {
            flip(lx, y, self);
            flip(hx, y, self);
        }
    }
}
