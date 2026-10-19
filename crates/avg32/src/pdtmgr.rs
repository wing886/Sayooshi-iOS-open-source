//! AVG32's numbered PDT buffer bank and its drawing primitives
//! (copies, fills, fades, stretches and file loading).
//!
//! Buffer 0 is the display buffer.  The visible *screen* is a separate copy
//! that is only refreshed for regions a primitive "presents": scripts draw
//! into buffer 0 through destination `-1` precisely to update it without
//! presenting, then reveal the finished region later (`0x67:00` with a
//! source of 0).  Keeping that distinction is what makes those double
//! buffered sequences look right.

use crate::buffer::{
    AVG32_HEIGHT, AVG32_WIDTH, PdtBuffer, SCREEN_H, SCREEN_W, blend_pixel, blend_under, fade_pixel,
    fade_under,
};
use crate::pdt::PdtImage;

pub const MAXPDT: usize = 32;
pub const WAKUPDT: usize = MAXPDT - 1;
pub const MESWINPDT: usize = MAXPDT - 2;
pub const BACKUPPDT: usize = MAXPDT - 3;
pub const ANMPDT: usize = MAXPDT - 4;
pub const HIDEPDT: usize = MAXPDT - 5;
pub const EXFONTPDT: usize = MAXPDT - 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub const fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub const fn full() -> Self {
        Self::new(0, 0, SCREEN_W - 1, SCREEN_H - 1)
    }

    fn ordered(self) -> Self {
        Self {
            x1: self.x1.min(self.x2),
            y1: self.y1.min(self.y2),
            x2: self.x1.max(self.x2),
            y2: self.y1.max(self.y2),
        }
    }

    /// Clamps an in-place region to the 640x480 buffer bounds.
    fn clamped(self) -> Option<Self> {
        let rect = self.ordered();
        let rect = Self {
            x1: rect.x1.max(0),
            y1: rect.y1.max(0),
            x2: rect.x2.min(SCREEN_W - 1),
            y2: rect.y2.min(SCREEN_H - 1),
        };
        (rect.x1 <= rect.x2 && rect.y1 <= rect.y2).then_some(rect)
    }
}

/// Source rectangle plus destination origin after AVG32's
/// copy clipping (`sx1<0`, `dx<0`, and right/bottom overhang all trimmed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blit {
    pub source: Rect,
    pub dx: i32,
    pub dy: i32,
}

impl Blit {
    pub fn clip(source: Rect, dx: i32, dy: i32, source_size: (usize, usize)) -> Option<Self> {
        let mut rect = source.ordered();
        let (mut dx, mut dy) = (dx, dy);
        if rect.x1 < 0 {
            dx -= rect.x1;
            rect.x1 = 0;
        }
        if rect.y1 < 0 {
            dy -= rect.y1;
            rect.y1 = 0;
        }
        rect.x2 = rect.x2.min(source_size.0 as i32 - 1).min(SCREEN_W - 1);
        rect.y2 = rect.y2.min(source_size.1 as i32 - 1).min(SCREEN_H - 1);
        if dx < 0 {
            rect.x1 -= dx;
            dx = 0;
        }
        if dy < 0 {
            rect.y1 -= dy;
            dy = 0;
        }
        if dx + (rect.x2 - rect.x1) > SCREEN_W - 1 {
            rect.x2 -= dx + (rect.x2 - rect.x1) - (SCREEN_W - 1);
        }
        if dy + (rect.y2 - rect.y1) > SCREEN_H - 1 {
            rect.y2 -= dy + (rect.y2 - rect.y1) - (SCREEN_H - 1);
        }
        (rect.x1 <= rect.x2 && rect.y1 <= rect.y2).then_some(Self {
            source: rect,
            dx,
            dy,
        })
    }

    pub fn destination(&self) -> Rect {
        Rect::new(
            self.dx,
            self.dy,
            self.dx + self.source.x2 - self.source.x1,
            self.dy + self.source.y2 - self.source.y1,
        )
    }
}

/// Resolves a script buffer number: negative numbers address buffer 0
/// *without* presenting.
pub fn resolve(index: i32) -> Option<(usize, bool)> {
    if index < 0 {
        Some((0, false))
    } else if (index as usize) < MAXPDT {
        Some((index as usize, true))
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct PdtManager {
    slots: Vec<Option<PdtBuffer>>,
    screen: Vec<u8>,
    getput: Option<PdtBuffer>,
    /// Cleared while a load restores the screen behind a black fade, so the
    /// intermediate reconstruction never becomes visible.
    pub present_enabled: bool,
}

impl Default for PdtManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PdtManager {
    pub fn new() -> Self {
        let mut slots = vec![None; MAXPDT];
        slots[0] = Some(PdtBuffer::screen());
        Self {
            slots,
            screen: vec![0; (AVG32_WIDTH * AVG32_HEIGHT * 3) as usize],
            getput: None,
            present_enabled: true,
        }
    }

    pub fn get(&self, index: usize) -> Option<&PdtBuffer> {
        self.slots.get(index).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut PdtBuffer> {
        self.slots.get_mut(index).and_then(Option::as_mut)
    }

    pub fn set(&mut self, index: usize, buffer: Option<PdtBuffer>) {
        if index < MAXPDT {
            self.slots[index] = buffer;
        }
    }

    /// Temporarily removes a buffer (callers must [`Self::set`] it back).
    pub fn take(&mut self, index: usize) -> Option<PdtBuffer> {
        self.slots.get_mut(index).and_then(Option::take)
    }

    pub fn ensure(&mut self, index: usize) -> &mut PdtBuffer {
        self.slots[index].get_or_insert_with(PdtBuffer::screen)
    }

    pub fn display(&self) -> &PdtBuffer {
        self.slots[0].as_ref().expect("display buffer is permanent")
    }

    pub fn display_mut(&mut self) -> &mut PdtBuffer {
        self.slots[0].as_mut().expect("display buffer is permanent")
    }

    /// The presented RGB screen.
    pub fn screen(&self) -> &[u8] {
        &self.screen
    }

    /// Copies an inclusive region of buffer 0 onto the visible screen.
    pub fn present(&mut self, rect: Rect) {
        if !self.present_enabled {
            return;
        }
        let Some(rect) = rect.clamped() else {
            return;
        };
        let display = self.slots[0].as_ref().expect("display buffer is permanent");
        let row = (rect.x2 - rect.x1 + 1) as usize * 3;
        for y in rect.y1..=rect.y2 {
            let at = (y as usize * AVG32_WIDTH as usize + rect.x1 as usize) * 3;
            self.screen[at..at + row].copy_from_slice(&display.rgb[at..at + row]);
        }
    }

    pub fn present_all(&mut self) {
        self.present(Rect::full());
    }

    fn after_draw(&mut self, index: usize, update: bool, rect: Rect) {
        if index == 0 && update {
            self.present(rect);
        }
    }

    /// Hands a clipped blit to `draw`.  The source is borrowed out of the
    /// bank, or snapshotted when it is also the destination.
    fn blit_with(
        &mut self,
        source: i32,
        rect: Rect,
        dx: i32,
        dy: i32,
        destination: i32,
        mut draw: impl FnMut(&PdtBuffer, &mut PdtBuffer, Blit),
    ) {
        let Some((source, _)) = resolve(source) else {
            return;
        };
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        let Some(size) = self.get(source).map(|buffer| (buffer.width, buffer.height)) else {
            return;
        };
        let Some(blit) = Blit::clip(rect, dx, dy, size) else {
            return;
        };
        if source == destination {
            let snapshot = self.get(source).cloned().expect("checked above");
            draw(&snapshot, self.ensure(destination), blit);
        } else {
            let source_buffer = self.slots[source].take().expect("checked above");
            draw(&source_buffer, self.ensure(destination), blit);
            self.slots[source] = Some(source_buffer);
        }
        self.after_draw(destination, update, blit.destination());
    }

    /// Plain copy. A non-zero `fade` cross-fades the copied pixels over
    /// the destination by `(fade + 1) / 256`. The mask plane is copied.
    pub fn copy(&mut self, rect: Rect, source: i32, dx: i32, dy: i32, destination: i32, fade: i32) {
        let fade = fade.clamp(0, 255) as u32;
        self.blit_with(source, rect, dx, dy, destination, |src, dst, blit| {
            for_each(blit, |sx, sy, tx, ty| {
                let s = src.at(sx, sy);
                let d = dst.at(tx, ty);
                if fade != 0 {
                    let under = dst.pixel(d);
                    fade_pixel(
                        src.pixel(s),
                        under,
                        &mut dst.rgb[d * 3..d * 3 + 3],
                        256,
                        fade + 1,
                    );
                } else {
                    dst.set_pixel(d, src.pixel(s));
                }
                dst.mask[d] = src.mask[s];
            });
        });
    }

    /// Composites through the source mask.
    pub fn mask_copy(
        &mut self,
        rect: Rect,
        source: i32,
        dx: i32,
        dy: i32,
        destination: i32,
        fade: i32,
    ) {
        let fade = fade.clamp(0, 255) as u32;
        self.blit_with(source, rect, dx, dy, destination, |src, dst, blit| {
            mask_blit(src, dst, blit, fade);
        });
    }

    /// `MaskCopy` from a free-standing buffer (the ending scroller).
    pub fn mask_copy_from(
        &mut self,
        src: &PdtBuffer,
        rect: Rect,
        dx: i32,
        dy: i32,
        destination: i32,
        fade: i32,
    ) {
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        let Some(blit) = Blit::clip(rect, dx, dy, (src.width, src.height)) else {
            return;
        };
        let fade = fade.clamp(0, 255) as u32;
        mask_blit(
            src,
            self.ensure(destination),
            blit,
            if fade != 0 { fade + 1 } else { 0 },
        );
        self.after_draw(destination, update, blit.destination());
    }

    /// Draws the source *under* the destination,
    /// then replaces the destination mask by the source mask.
    pub fn copy_with_mask(
        &mut self,
        rect: Rect,
        source: i32,
        dx: i32,
        dy: i32,
        destination: i32,
        fade: i32,
    ) {
        let fade = fade.clamp(0, 255) as u32;
        self.blit_with(source, rect, dx, dy, destination, |src, dst, blit| {
            for_each(blit, |sx, sy, tx, ty| {
                let s = src.at(sx, sy);
                let d = dst.at(tx, ty);
                let under_mask = dst.mask[d];
                dst.mask[d] = src.mask[s];
                if fade != 0 {
                    fade_under(
                        src.pixel(s),
                        &mut dst.rgb[d * 3..d * 3 + 3],
                        under_mask,
                        fade,
                    );
                } else {
                    blend_under(src.pixel(s), &mut dst.rgb[d * 3..d * 3 + 3], under_mask);
                }
            });
        });
    }

    /// A colour-keyed copy — every pixel except the key
    /// colour is transferred (the mask plane is left alone).
    pub fn color_key_copy(
        &mut self,
        rect: Rect,
        source: i32,
        dx: i32,
        dy: i32,
        destination: i32,
        key: [i32; 3],
    ) {
        let key = key.map(|channel| channel.clamp(0, 255) as u8);
        self.blit_with(source, rect, dx, dy, destination, |src, dst, blit| {
            for_each(blit, |sx, sy, tx, ty| {
                let pixel = src.pixel(src.at(sx, sy));
                if pixel != key {
                    let d = dst.at(tx, ty);
                    dst.set_pixel(d, pixel);
                }
            });
        });
    }

    /// Exchanges RGB (not mask) between two regions.
    pub fn swap(&mut self, rect: Rect, source: i32, dx: i32, dy: i32, destination: i32) {
        let Some((source, _)) = resolve(source) else {
            return;
        };
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        if self.get(source).is_none() {
            return;
        }
        self.ensure(destination);
        let Some(blit) = Blit::clip(rect, dx, dy, (AVG32_WIDTH as usize, AVG32_HEIGHT as usize))
        else {
            return;
        };
        if source == destination {
            let buffer = self.get_mut(source).expect("checked above");
            for_each(blit, |sx, sy, tx, ty| {
                let a = buffer.at(sx, sy);
                let b = buffer.at(tx, ty);
                let (pa, pb) = (buffer.pixel(a), buffer.pixel(b));
                buffer.set_pixel(a, pb);
                buffer.set_pixel(b, pa);
            });
            self.after_draw(source, update, blit.source);
            self.after_draw(source, update, blit.destination());
            return;
        }
        let mut src = self.slots[source].take().expect("checked above");
        {
            let dst = self.slots[destination].as_mut().expect("ensured");
            for_each(blit, |sx, sy, tx, ty| {
                let a = src.at(sx, sy);
                let b = dst.at(tx, ty);
                let (pa, pb) = (src.pixel(a), dst.pixel(b));
                src.set_pixel(a, pb);
                dst.set_pixel(b, pa);
            });
        }
        self.slots[source] = Some(src);
        if source == 0 {
            self.after_draw(0, true, blit.source);
        }
        self.after_draw(destination, update, blit.destination());
    }

    /// Whole-buffer copy that leaves the destination mask
    /// fully opaque.
    pub fn all_copy(&mut self, source: i32, destination: i32, fade: i32) {
        let Some((source, _)) = resolve(source) else {
            return;
        };
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        let Some(src) = self.get(source).cloned() else {
            return;
        };
        let fade = fade.clamp(0, 255) as u32;
        let dst = self.ensure(destination);
        let width = src.width.min(dst.width);
        let height = src.height.min(dst.height);
        for y in 0..height {
            for x in 0..width {
                let s = y * src.width + x;
                let d = y * dst.width + x;
                if fade != 0 {
                    let under = dst.pixel(d);
                    fade_pixel(
                        src.pixel(s),
                        under,
                        &mut dst.rgb[d * 3..d * 3 + 3],
                        256,
                        fade + 1,
                    );
                } else {
                    dst.set_pixel(d, src.pixel(s));
                }
            }
        }
        dst.mask.fill(255);
        self.after_draw(destination, update, Rect::full());
    }

    /// RGB copy of `rect` from `source` to the
    /// same position on buffer 0; with source 0 it simply presents `rect`.
    pub fn copy_back_buffer(&mut self, rect: Rect, source: i32, update: bool) {
        let Some((source, _)) = resolve(source) else {
            return;
        };
        let Some(rect) = rect.clamped() else {
            return;
        };
        if source != 0 {
            let Some(src) = self.get(source).cloned() else {
                return;
            };
            let dst = self.display_mut();
            for y in rect.y1..=rect.y2.min(src.height as i32 - 1) {
                for x in rect.x1..=rect.x2.min(src.width as i32 - 1) {
                    let s = src.at(x, y);
                    let d = dst.at(x, y);
                    dst.set_pixel(d, src.pixel(s));
                }
            }
        }
        if update {
            self.present(rect);
        }
    }

    /// Fills colour and makes the mask opaque.
    pub fn fill_rect(&mut self, rect: Rect, buffer: i32, colour: [i32; 3]) {
        self.paint(rect, buffer, true, |dst, index| {
            dst.set_pixel(index, colour.map(|c| c as u8));
            dst.mask[index] = 255;
        });
    }

    /// Fills colour only.
    pub fn clear_rect(&mut self, rect: Rect, buffer: i32, colour: [i32; 3]) {
        self.paint(rect, buffer, true, |dst, index| {
            dst.set_pixel(index, colour.map(|c| c as u8));
        });
    }

    pub fn draw_rect_line(&mut self, rect: Rect, buffer: i32, colour: [i32; 3]) {
        let Some((index, update)) = resolve(buffer) else {
            return;
        };
        let Some(rect) = rect.clamped() else {
            return;
        };
        let colour = colour.map(|c| c as u8);
        let dst = self.ensure(index);
        for y in rect.y1..=rect.y2 {
            for x in rect.x1..=rect.x2 {
                if y == rect.y1 || y == rect.y2 || x == rect.x1 || x == rect.x2 {
                    let at = dst.at(x, y);
                    dst.set_pixel(at, colour);
                }
            }
        }
        self.after_draw(index, update, rect);
    }

    /// Moves every pixel `(count + 1) / 256` of the way
    /// towards `colour`.
    pub fn fade_color(&mut self, rect: Rect, buffer: i32, colour: [i32; 3], count: i32) {
        let count = count.clamp(0, 255) + 1;
        self.paint(rect, buffer, false, |dst, index| {
            let pixel = dst.pixel(index);
            let faded = std::array::from_fn(|channel| {
                let value = i32::from(pixel[channel]);
                (value + (((colour[channel] - value) * count) >> 8)) as u8
            });
            dst.set_pixel(index, faded);
        });
    }

    /// Multiplies each channel by `(c + 1) / 256`.
    pub fn color_mask(&mut self, rect: Rect, buffer: i32, colour: [i32; 3]) {
        let factor = colour.map(|c| c.clamp(0, 255) as u32 + 1);
        self.paint(rect, buffer, false, |dst, index| {
            let pixel = dst.pixel(index);
            let tinted = std::array::from_fn(|channel| {
                ((u32::from(pixel[channel]) * factor[channel]) >> 8) as u8
            });
            dst.set_pixel(index, tinted);
        });
    }

    pub fn monochrome(&mut self, rect: Rect, buffer: i32) {
        self.paint(rect, buffer, false, |dst, index| {
            let [r, g, b] = dst.pixel(index).map(u32::from);
            let grey = ((r * 299 + g * 587 + b * 114) / 1000) as u8;
            dst.set_pixel(index, [grey; 3]);
        });
    }

    pub fn invert(&mut self, rect: Rect, buffer: i32) {
        self.paint(rect, buffer, false, |dst, index| {
            let pixel = dst.pixel(index);
            dst.set_pixel(index, pixel.map(|channel| !channel));
        });
    }

    /// In-place per-pixel edit of an existing (or, when `create`, newly
    /// allocated) buffer within a clamped rectangle.
    fn paint(
        &mut self,
        rect: Rect,
        buffer: i32,
        create: bool,
        mut edit: impl FnMut(&mut PdtBuffer, usize),
    ) {
        let Some((index, update)) = resolve(buffer) else {
            return;
        };
        let Some(rect) = rect.clamped() else {
            return;
        };
        let dst = if create {
            self.ensure(index)
        } else {
            match self.get_mut(index) {
                Some(dst) => dst,
                None => return,
            }
        };
        let x2 = rect.x2.min(dst.width as i32 - 1);
        let y2 = rect.y2.min(dst.height as i32 - 1);
        for y in rect.y1..=y2 {
            for x in rect.x1..=x2 {
                let at = dst.at(x, y);
                edit(dst, at);
            }
        }
        self.after_draw(index, update, rect);
    }

    /// Scaled copy (nearest neighbour).
    pub fn stretch_copy(
        &mut self,
        source_rect: Rect,
        source: i32,
        destination_rect: Rect,
        destination: i32,
    ) {
        let Some((source, _)) = resolve(source) else {
            return;
        };
        let Some(src) = self.get(source).cloned() else {
            return;
        };
        self.stretch_from(&src, source_rect, destination_rect, destination);
    }

    pub fn stretch_from(
        &mut self,
        src: &PdtBuffer,
        source_rect: Rect,
        destination_rect: Rect,
        destination: i32,
    ) {
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        let s = source_rect.ordered();
        let d = destination_rect.ordered();
        let (sw, sh) = (s.x2 - s.x1 + 1, s.y2 - s.y1 + 1);
        let (dw, dh) = (d.x2 - d.x1 + 1, d.y2 - d.y1 + 1);
        if sw <= 0 || sh <= 0 || dw <= 0 || dh <= 0 {
            return;
        }
        let Some(clip) = d.clamped() else {
            return;
        };
        let dst = self.ensure(destination);
        for y in clip.y1..=clip.y2.min(dst.height as i32 - 1) {
            let sy = (y - d.y1) * sh / dh + s.y1;
            if sy < 0 || sy >= src.height as i32 {
                continue;
            }
            for x in clip.x1..=clip.x2.min(dst.width as i32 - 1) {
                let sx = (x - d.x1) * sw / dw + s.x1;
                if sx < 0 || sx >= src.width as i32 {
                    continue;
                }
                let at = dst.at(x, y);
                dst.set_pixel(at, src.pixel(src.at(sx, sy)));
            }
        }
        self.after_draw(destination, update, clip);
    }

    /// Captures a region into the private get/put buffer.
    pub fn get_region(&mut self, rect: Rect, source: i32) {
        let Some((source, _)) = resolve(source) else {
            return;
        };
        let Some(src) = self.get(source) else {
            return;
        };
        let Some(rect) = rect.clamped() else {
            return;
        };
        let width = (rect.x2 - rect.x1 + 1) as usize;
        let height = (rect.y2 - rect.y1 + 1) as usize;
        let mut captured = PdtBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let s = src.at(rect.x1 + x as i32, rect.y1 + y as i32);
                captured.set_pixel(y * width + x, src.pixel(s));
            }
        }
        self.getput = Some(captured);
    }

    /// Writes the captured region back at `(dx, dy)`.
    pub fn put_region(&mut self, dx: i32, dy: i32, destination: i32) {
        let Some(captured) = self.getput.clone() else {
            return;
        };
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        let Some(blit) = Blit::clip(
            Rect::new(0, 0, captured.width as i32 - 1, captured.height as i32 - 1),
            dx,
            dy,
            (captured.width, captured.height),
        ) else {
            return;
        };
        let dst = self.ensure(destination);
        for_each(blit, |sx, sy, tx, ty| {
            let at = dst.at(tx, ty);
            dst.set_pixel(at, captured.pixel(captured.at(sx, sy)));
        });
        self.after_draw(destination, update, blit.destination());
    }

    /// `None` (the `*`/`?` names) copies buffer 1 with
    /// its mask; an image is copied into the destination's top-left corner.
    pub fn load_file(&mut self, image: Option<&PdtImage>, destination: i32) {
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        match image {
            None => {
                if destination != 1 {
                    let working = self.ensure(1).clone();
                    *self.ensure(destination) = working;
                }
            }
            Some(image) => self.ensure(destination).copy_image(image),
        }
        self.after_draw(destination, update, Rect::full());
    }

    /// Like `load_file`, but the result is opaque.
    pub fn load_base_file(&mut self, image: Option<&PdtImage>, destination: i32) {
        self.load_file(image, destination);
        if let Some((destination, _)) = resolve(destination) {
            if image.is_some() || destination != 1 {
                self.ensure(destination).mask.fill(255);
            }
        }
    }

    /// Composites a decoded PDT (through its own mask)
    /// onto a buffer.
    pub fn load_copy(&mut self, image: &PdtImage, rect: Rect, dx: i32, dy: i32, destination: i32) {
        let Some((destination, update)) = resolve(destination) else {
            return;
        };
        let source = PdtBuffer::from_image(image);
        let Some(blit) = Blit::clip(rect, dx, dy, (source.width, source.height)) else {
            return;
        };
        let dst = self.ensure(destination);
        for_each(blit, |sx, sy, tx, ty| {
            let s = source.at(sx, sy);
            let d = dst.at(tx, ty);
            if image.has_mask {
                blend_pixel(
                    source.pixel(s),
                    &mut dst.rgb[d * 3..d * 3 + 3],
                    source.mask[s],
                );
            } else {
                dst.set_pixel(d, source.pixel(s));
            }
        });
        self.after_draw(destination, update, blit.destination());
    }

    /// Draws coverage-mapped glyph pixels onto a buffer (used by text output).
    pub fn draw_coverage(
        &mut self,
        buffer: usize,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        coverage: &[u8],
        colour: [u8; 3],
    ) {
        let Some(dst) = self.get_mut(buffer) else {
            return;
        };
        for row in 0..height {
            for column in 0..width {
                let alpha = u32::from(coverage[row * width + column]);
                if alpha == 0 {
                    continue;
                }
                let (px, py) = (x + column as i32, y + row as i32);
                if !dst.contains(px, py) {
                    continue;
                }
                let at = dst.at(px, py);
                let pixel = dst.pixel(at);
                let blended = std::array::from_fn(|channel| {
                    ((u32::from(colour[channel]) * alpha
                        + u32::from(pixel[channel]) * (255 - alpha))
                        / 255) as u8
                });
                dst.set_pixel(at, blended);
            }
        }
    }
}

fn mask_blit(src: &PdtBuffer, dst: &mut PdtBuffer, blit: Blit, fade: u32) {
    for_each(blit, |sx, sy, tx, ty| {
        let s = src.at(sx, sy);
        let d = dst.at(tx, ty);
        if fade != 0 {
            let under = dst.pixel(d);
            fade_pixel(
                src.pixel(s),
                under,
                &mut dst.rgb[d * 3..d * 3 + 3],
                u32::from(src.mask[s]),
                fade,
            );
        } else {
            blend_pixel(src.pixel(s), &mut dst.rgb[d * 3..d * 3 + 3], src.mask[s]);
        }
    });
}

/// Visits every `(source x, source y, destination x, destination y)` of a
/// clipped blit in row-major order.
pub fn for_each(blit: Blit, mut visit: impl FnMut(i32, i32, i32, i32)) {
    for y in blit.source.y1..=blit.source.y2 {
        for x in blit.source.x1..=blit.source.x2 {
            visit(
                x,
                y,
                blit.dx + x - blit.source.x1,
                blit.dy + y - blit.source.y1,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_destination_draws_to_display_without_presenting() {
        let mut manager = PdtManager::new();
        manager.fill_rect(Rect::new(0, 0, 9, 9), -1, [255, 0, 0]);
        assert_eq!(manager.display().pixel(0), [255, 0, 0]);
        assert_eq!(&manager.screen()[..3], &[0, 0, 0]);
        manager.copy_back_buffer(Rect::new(0, 0, 9, 9), 0, true);
        assert_eq!(&manager.screen()[..3], &[255, 0, 0]);
    }

    #[test]
    fn copy_clips_negative_source_origin_like_the_reference() {
        let blit = Blit::clip(Rect::new(-10, 0, 99, 9), 0, 0, (640, 480)).unwrap();
        assert_eq!(blit.source, Rect::new(0, 0, 99, 9));
        assert_eq!((blit.dx, blit.dy), (10, 0));
    }

    #[test]
    fn swap_exchanges_rgb_between_buffers() {
        let mut manager = PdtManager::new();
        manager.fill_rect(Rect::full(), 2, [1, 2, 3]);
        manager.fill_rect(Rect::full(), 3, [4, 5, 6]);
        manager.swap(Rect::new(0, 0, 0, 0), 2, 0, 0, 3);
        assert_eq!(manager.get(2).unwrap().pixel(0), [4, 5, 6]);
        assert_eq!(manager.get(3).unwrap().pixel(0), [1, 2, 3]);
    }
}
