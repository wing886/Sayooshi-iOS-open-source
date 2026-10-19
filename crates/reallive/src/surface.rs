//! RGBA bitmaps (DCs and object images) and the pixel operations of the
//! `grp`/`rec` functions.
//!
//! Colour is straight (not premultiplied); the alpha channel is the mask
//! that `Mask` operations respect.

use crate::image::Image;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    /// From inclusive corners (the `grp` convention).
    pub fn from_corners(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));
        Self::new(x1, y1, x2 - x1 + 1, y2 - y1 + 1)
    }

    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    pub fn is_empty(&self) -> bool {
        self.w <= 0 || self.h <= 0
    }

    pub fn intersect(&self, other: &Rect) -> Rect {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let r = self.right().min(other.right());
        let b = self.bottom().min(other.bottom());
        Rect::new(x, y, (r - x).max(0), (b - y).max(0))
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && y >= self.y && x < self.right() && y < self.bottom()
    }
}

/// How source pixels are combined with the destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blend {
    /// Replace (blended by opacity).
    Copy,
    /// Alpha blend using the source mask.
    Mask,
    Add,
    /// Subtract the inverted source (`grpSub`).
    Sub,
    MaskAdd,
    MaskSub,
    And,
    Or,
}

impl Blend {
    pub fn uses_mask(self) -> bool {
        matches!(self, Blend::Mask | Blend::MaskAdd | Blend::MaskSub)
    }
}

/// An external mask applied during a blit (`...WithMask`, `...InvMask`).
#[derive(Debug, Clone, Copy)]
pub struct ExternalMask<'a> {
    pub mask: &'a Surface,
    pub invert: bool,
    pub offset: (i32, i32),
    /// Quantise to this many grey levels (0 = no quantisation).
    pub levels: i32,
    /// Scale each level by `threshold / 256` (0 = unchanged).
    pub threshold: i32,
}

impl ExternalMask<'_> {
    fn level(&self, x: i32, y: i32) -> u32 {
        let mx = x + self.offset.0;
        let my = y + self.offset.1;
        let mut value = if self.mask.width > 0 && self.mask.height > 0 {
            let mx = mx.rem_euclid(self.mask.width);
            let my = my.rem_euclid(self.mask.height);
            // Greyscale masks store the level in every channel.
            i32::from(self.mask.pixel(mx, my)[0])
        } else {
            255
        };
        if self.invert {
            value = 255 - value;
        }
        if self.levels > 1 {
            let step = 255.0 / f64::from(self.levels - 1);
            value = ((f64::from(value) / step).round() * step) as i32;
        }
        if self.threshold != 0 {
            value = value * self.threshold / 256;
        }
        value.clamp(0, 255) as u32
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Surface {
    pub width: i32,
    pub height: i32,
    pub rgba: Vec<u8>,
}

impl std::fmt::Debug for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Surface({}x{})", self.width, self.height)
    }
}

#[inline]
fn lerp(dst: u8, src: u8, alpha: u32) -> u8 {
    ((u32::from(dst) * (255 - alpha) + u32::from(src) * alpha + 127) / 255) as u8
}

impl Surface {
    /// A black, fully opaque surface.
    pub fn new(width: i32, height: i32) -> Self {
        let mut rgba = vec![0u8; (width.max(0) * height.max(0) * 4) as usize];
        for pixel in rgba.chunks_exact_mut(4) {
            pixel[3] = 255;
        }
        Self {
            width,
            height,
            rgba,
        }
    }

    pub fn from_image(image: &Image) -> Self {
        image.surface.clone()
    }

    pub fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    #[inline]
    pub fn pixel(&self, x: i32, y: i32) -> [u8; 4] {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return [0, 0, 0, 0];
        }
        let at = ((y * self.width + x) * 4) as usize;
        [
            self.rgba[at],
            self.rgba[at + 1],
            self.rgba[at + 2],
            self.rgba[at + 3],
        ]
    }

    #[inline]
    fn at(&self, x: i32, y: i32) -> usize {
        ((y * self.width + x) * 4) as usize
    }

    pub fn fill(&mut self, rect: Rect, colour: [u8; 4], opacity: u8) {
        let rect = rect.intersect(&self.rect());
        let alpha = u32::from(opacity);
        for y in rect.y..rect.bottom() {
            for x in rect.x..rect.right() {
                let at = self.at(x, y);
                let pixel = &mut self.rgba[at..at + 4];
                if alpha == 255 {
                    pixel.copy_from_slice(&colour);
                } else {
                    for c in 0..3 {
                        pixel[c] = lerp(pixel[c], colour[c], alpha);
                    }
                }
            }
        }
    }

    /// Draws a one-pixel outline (`grpOutline`).
    pub fn outline(&mut self, rect: Rect, colour: [u8; 4], opacity: u8) {
        if rect.is_empty() {
            return;
        }
        self.fill(Rect::new(rect.x, rect.y, rect.w, 1), colour, opacity);
        self.fill(
            Rect::new(rect.x, rect.bottom() - 1, rect.w, 1),
            colour,
            opacity,
        );
        self.fill(
            Rect::new(rect.x, rect.y + 1, 1, rect.h - 2),
            colour,
            opacity,
        );
        self.fill(
            Rect::new(rect.right() - 1, rect.y + 1, 1, rect.h - 2),
            colour,
            opacity,
        );
    }

    /// Applies `f` to every pixel of `rect`, blending the result by
    /// `opacity`.
    pub fn map_pixels(&mut self, rect: Rect, opacity: u8, f: impl Fn([u8; 3]) -> [u8; 3]) {
        let rect = rect.intersect(&self.rect());
        let alpha = u32::from(opacity);
        for y in rect.y..rect.bottom() {
            for x in rect.x..rect.right() {
                let at = self.at(x, y);
                let old = [self.rgba[at], self.rgba[at + 1], self.rgba[at + 2]];
                let new = f(old);
                for c in 0..3 {
                    self.rgba[at + c] = lerp(old[c], new[c], alpha);
                }
            }
        }
    }

    pub fn invert(&mut self, rect: Rect, opacity: u8) {
        self.map_pixels(rect, opacity, |p| p.map(|c| 255 - c));
    }

    pub fn mono(&mut self, rect: Rect, opacity: u8) {
        self.map_pixels(rect, opacity, |p| {
            let grey =
                ((u32::from(p[0]) * 77 + u32::from(p[1]) * 151 + u32::from(p[2]) * 28) >> 8) as u8;
            [grey; 3]
        });
    }

    /// `grpColour`/`grpLight`: positive components screen, negative ones
    /// multiply.
    pub fn colour(&mut self, rect: Rect, rgb: [i32; 3], opacity: u8) {
        self.map_pixels(rect, opacity, |p| apply_tint(p, rgb));
    }

    /// Copies `src_rect` of `src` to `(dx, dy)`.
    pub fn blit(
        &mut self,
        src: &Surface,
        src_rect: Rect,
        dx: i32,
        dy: i32,
        opacity: u8,
        blend: Blend,
        mask: Option<ExternalMask>,
    ) {
        // Clip against both surfaces.
        let requested = src_rect;
        let mut src_rect = src_rect.intersect(&src.rect());
        let mut dx = dx + (src_rect.x - requested.x);
        let mut dy = dy + (src_rect.y - requested.y);
        if dx < 0 {
            src_rect.x -= dx;
            src_rect.w += dx;
            dx = 0;
        }
        if dy < 0 {
            src_rect.y -= dy;
            src_rect.h += dy;
            dy = 0;
        }
        src_rect.w = src_rect.w.min(self.width - dx);
        src_rect.h = src_rect.h.min(self.height - dy);
        if src_rect.is_empty() || opacity == 0 {
            return;
        }
        let op = u32::from(opacity);
        for row in 0..src_rect.h {
            let sy = src_rect.y + row;
            let ty = dy + row;
            for column in 0..src_rect.w {
                let sx = src_rect.x + column;
                let tx = dx + column;
                let s = src.pixel(sx, sy);
                let mut alpha = op;
                if blend.uses_mask() {
                    alpha = alpha * u32::from(s[3]) / 255;
                }
                if let Some(mask) = &mask {
                    alpha = alpha * mask.level(tx, ty) / 255;
                }
                if alpha == 0 {
                    continue;
                }
                let at = self.at(tx, ty);
                let d = &mut self.rgba[at..at + 4];
                combine(d, s, alpha, blend, op);
            }
        }
    }

    /// Swaps `src_rect` of `other` with the same-sized area at `(dx, dy)`.
    pub fn swap_with(&mut self, other: &mut Surface, src_rect: Rect, dx: i32, dy: i32) {
        for row in 0..src_rect.h {
            for column in 0..src_rect.w {
                let (sx, sy) = (src_rect.x + column, src_rect.y + row);
                let (tx, ty) = (dx + column, dy + row);
                if !other.rect().contains(sx, sy) || !self.rect().contains(tx, ty) {
                    continue;
                }
                let a = other.at(sx, sy);
                let b = self.at(tx, ty);
                for c in 0..4 {
                    std::mem::swap(&mut other.rgba[a + c], &mut self.rgba[b + c]);
                }
            }
        }
    }

    /// Scales `src_rect` of `src` into `dst_rect` (bilinear).
    pub fn stretch_blit(
        &mut self,
        src: &Surface,
        src_rect: Rect,
        dst_rect: Rect,
        opacity: u8,
        blend: Blend,
    ) {
        if src_rect.is_empty() || dst_rect.is_empty() {
            return;
        }
        let clip = dst_rect.intersect(&self.rect());
        let sx_scale = f64::from(src_rect.w) / f64::from(dst_rect.w);
        let sy_scale = f64::from(src_rect.h) / f64::from(dst_rect.h);
        let op = u32::from(opacity);
        for ty in clip.y..clip.bottom() {
            for tx in clip.x..clip.right() {
                let fx =
                    f64::from(src_rect.x) + (f64::from(tx - dst_rect.x) + 0.5) * sx_scale - 0.5;
                let fy =
                    f64::from(src_rect.y) + (f64::from(ty - dst_rect.y) + 0.5) * sy_scale - 0.5;
                let s = sample_bilinear(src, fx, fy, src_rect);
                let mut alpha = op;
                if blend.uses_mask() {
                    alpha = alpha * u32::from(s[3]) / 255;
                }
                if alpha == 0 {
                    continue;
                }
                let at = self.at(tx, ty);
                combine(&mut self.rgba[at..at + 4], s, alpha, blend, op);
            }
        }
    }

    /// `grpRotate`: scales `src_rect` by `scale` (percent), rotates it by
    /// `angle` tenths of a degree clockwise around `(ox, oy)` (source
    /// coordinates) and places that point at `(dox, doy)`, clipped to
    /// `clip`.
    #[allow(clippy::too_many_arguments)]
    pub fn rotate_blit(
        &mut self,
        src: &Surface,
        src_rect: Rect,
        origin: (f64, f64),
        dest_origin: (f64, f64),
        clip: Rect,
        angle_tenths: i32,
        scale: (f64, f64),
        opacity: u8,
        blend: Blend,
    ) {
        let transform = Transform::new(
            origin,
            dest_origin,
            f64::from(angle_tenths) / 10.0,
            (scale.0 / 100.0, scale.1 / 100.0),
        );
        self.draw_transformed(src, src_rect, &transform, clip, opacity, blend, None);
    }

    /// Draws `src_rect` of `src` through an affine transform; `tint`
    /// optionally post-processes each source pixel.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_transformed(
        &mut self,
        src: &Surface,
        src_rect: Rect,
        transform: &Transform,
        clip: Rect,
        opacity: u8,
        blend: Blend,
        tint: Option<&dyn Fn([u8; 4]) -> [u8; 4]>,
    ) {
        let src_rect = src_rect.intersect(&src.rect());
        if src_rect.is_empty() || opacity == 0 {
            return;
        }
        let bounds = transform
            .bounds(src_rect)
            .intersect(&clip)
            .intersect(&self.rect());
        let op = u32::from(opacity);
        let identity = transform.is_translation();
        for ty in bounds.y..bounds.bottom() {
            for tx in bounds.x..bounds.right() {
                let (fx, fy) = transform.inverse(f64::from(tx) + 0.5, f64::from(ty) + 0.5);
                let s = if identity {
                    let (sx, sy) = (fx.floor() as i32, fy.floor() as i32);
                    if !src_rect.contains(sx, sy) {
                        continue;
                    }
                    src.pixel(sx, sy)
                } else {
                    if fx < f64::from(src_rect.x) - 0.5
                        || fy < f64::from(src_rect.y) - 0.5
                        || fx > f64::from(src_rect.right()) + 0.5
                        || fy > f64::from(src_rect.bottom()) + 0.5
                    {
                        continue;
                    }
                    sample_bilinear(src, fx - 0.5, fy - 0.5, src_rect)
                };
                let s = match tint {
                    Some(f) => f(s),
                    None => s,
                };
                let mut alpha = op;
                if blend.uses_mask() {
                    alpha = alpha * u32::from(s[3]) / 255;
                }
                if alpha == 0 {
                    continue;
                }
                let at = self.at(tx, ty);
                combine(&mut self.rgba[at..at + 4], s, alpha, blend, op);
            }
        }
    }

    /// Pixels of `rect` as a new surface.
    pub fn crop(&self, rect: Rect) -> Surface {
        let mut out = Surface::new(rect.w.max(0), rect.h.max(0));
        out.blit(self, rect, 0, 0, 255, Blend::Copy, None);
        out
    }
}

/// Screen for positive components, multiply for negative ones.
pub fn apply_tint(p: [u8; 3], rgb: [i32; 3]) -> [u8; 3] {
    let mut out = p;
    for c in 0..3 {
        let v = rgb[c].clamp(-255, 255);
        let x = i32::from(p[c]);
        out[c] = if v >= 0 {
            (255 - (255 - x) * (255 - v) / 255) as u8
        } else {
            (x * (255 + v) / 255) as u8
        };
    }
    out
}

#[inline]
fn combine(d: &mut [u8], s: [u8; 4], alpha: u32, blend: Blend, opacity: u32) {
    match blend {
        Blend::Copy => {
            if alpha == 255 {
                d.copy_from_slice(&s);
            } else {
                for c in 0..3 {
                    d[c] = lerp(d[c], s[c], alpha);
                }
                d[3] = lerp(d[3], s[3], opacity);
            }
        }
        Blend::Mask => {
            for c in 0..3 {
                d[c] = lerp(d[c], s[c], alpha);
            }
            d[3] = (u32::from(d[3]) + (255 - u32::from(d[3])) * alpha / 255).min(255) as u8;
        }
        Blend::Add | Blend::MaskAdd => {
            for c in 0..3 {
                d[c] = (u32::from(d[c]) + u32::from(s[c]) * alpha / 255).min(255) as u8;
            }
        }
        Blend::Sub | Blend::MaskSub => {
            for c in 0..3 {
                let inverted = 255 - u32::from(s[c]);
                d[c] = u32::from(d[c]).saturating_sub(inverted * alpha / 255) as u8;
            }
        }
        Blend::And => {
            for c in 0..3 {
                d[c] = lerp(d[c], d[c] & s[c], alpha);
            }
        }
        Blend::Or => {
            for c in 0..3 {
                d[c] = lerp(d[c], d[c] | s[c], alpha);
            }
        }
    }
}

fn sample_bilinear(src: &Surface, fx: f64, fy: f64, bounds: Rect) -> [u8; 4] {
    let x0 = fx.floor();
    let y0 = fy.floor();
    let tx = fx - x0;
    let ty = fy - y0;
    let clamp_x = |x: i32| x.clamp(bounds.x, bounds.right() - 1);
    let clamp_y = |y: i32| y.clamp(bounds.y, bounds.bottom() - 1);
    let (x0, y0) = (x0 as i32, y0 as i32);
    let p00 = src.pixel(clamp_x(x0), clamp_y(y0));
    let p10 = src.pixel(clamp_x(x0 + 1), clamp_y(y0));
    let p01 = src.pixel(clamp_x(x0), clamp_y(y0 + 1));
    let p11 = src.pixel(clamp_x(x0 + 1), clamp_y(y0 + 1));
    let mut out = [0u8; 4];
    // Interpolate premultiplied values so transparent edges do not bleed.
    let weights = [
        (1.0 - tx) * (1.0 - ty),
        tx * (1.0 - ty),
        (1.0 - tx) * ty,
        tx * ty,
    ];
    let pixels = [p00, p10, p01, p11];
    let alpha: f64 = pixels
        .iter()
        .zip(weights)
        .map(|(p, w)| f64::from(p[3]) * w)
        .sum();
    for c in 0..3 {
        let premultiplied: f64 = pixels
            .iter()
            .zip(weights)
            .map(|(p, w)| f64::from(p[c]) * f64::from(p[3]) * w)
            .sum();
        out[c] = if alpha > 0.0 {
            (premultiplied / alpha).round().clamp(0.0, 255.0) as u8
        } else {
            0
        };
    }
    out[3] = alpha.round().clamp(0.0, 255.0) as u8;
    out
}

/// An affine map from source to destination coordinates: translate the
/// origin to 0, scale, rotate (clockwise), translate to the destination.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    origin: (f64, f64),
    dest: (f64, f64),
    cos: f64,
    sin: f64,
    scale: (f64, f64),
}

impl Transform {
    pub fn new(origin: (f64, f64), dest: (f64, f64), degrees: f64, scale: (f64, f64)) -> Self {
        let radians = degrees.to_radians();
        Self {
            origin,
            dest,
            cos: radians.cos(),
            sin: radians.sin(),
            scale,
        }
    }

    pub fn translation(dx: f64, dy: f64) -> Self {
        Self::new((0.0, 0.0), (dx, dy), 0.0, (1.0, 1.0))
    }

    fn is_translation(&self) -> bool {
        self.sin == 0.0 && self.cos == 1.0 && self.scale == (1.0, 1.0)
    }

    pub fn forward(&self, x: f64, y: f64) -> (f64, f64) {
        let (x, y) = (
            (x - self.origin.0) * self.scale.0,
            (y - self.origin.1) * self.scale.1,
        );
        (
            x * self.cos - y * self.sin + self.dest.0,
            x * self.sin + y * self.cos + self.dest.1,
        )
    }

    pub fn inverse(&self, x: f64, y: f64) -> (f64, f64) {
        let (x, y) = (x - self.dest.0, y - self.dest.1);
        let (rx, ry) = (x * self.cos + y * self.sin, -x * self.sin + y * self.cos);
        let sx = if self.scale.0 != 0.0 {
            rx / self.scale.0
        } else {
            f64::INFINITY
        };
        let sy = if self.scale.1 != 0.0 {
            ry / self.scale.1
        } else {
            f64::INFINITY
        };
        (sx + self.origin.0, sy + self.origin.1)
    }

    /// Destination bounding box of `rect`.
    pub fn bounds(&self, rect: Rect) -> Rect {
        let corners = [
            (f64::from(rect.x), f64::from(rect.y)),
            (f64::from(rect.right()), f64::from(rect.y)),
            (f64::from(rect.x), f64::from(rect.bottom())),
            (f64::from(rect.right()), f64::from(rect.bottom())),
        ]
        .map(|(x, y)| self.forward(x, y));
        let min_x = corners.iter().map(|c| c.0).fold(f64::INFINITY, f64::min);
        let min_y = corners.iter().map(|c| c.1).fold(f64::INFINITY, f64::min);
        let max_x = corners
            .iter()
            .map(|c| c.0)
            .fold(f64::NEG_INFINITY, f64::max);
        let max_y = corners
            .iter()
            .map(|c| c.1)
            .fold(f64::NEG_INFINITY, f64::max);
        if !min_x.is_finite() || !max_x.is_finite() {
            return Rect::new(0, 0, 0, 0);
        }
        let x = min_x.floor() as i32;
        let y = min_y.floor() as i32;
        Rect::new(x, y, max_x.ceil() as i32 - x, max_y.ceil() as i32 - y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_copy_respects_alpha_and_opacity() {
        let mut dst = Surface::new(2, 1);
        let mut src = Surface::new(2, 1);
        src.rgba = vec![255, 0, 0, 255, 255, 0, 0, 0];
        dst.blit(&src, src.rect(), 0, 0, 255, Blend::Mask, None);
        assert_eq!(dst.pixel(0, 0)[0], 255);
        assert_eq!(dst.pixel(1, 0)[0], 0);
        let mut dst = Surface::new(1, 1);
        dst.blit(&src, Rect::new(0, 0, 1, 1), 0, 0, 128, Blend::Copy, None);
        assert_eq!(dst.pixel(0, 0)[0], 128);
    }

    #[test]
    fn clipping_negative_destinations() {
        let mut dst = Surface::new(4, 4);
        let mut src = Surface::new(4, 4);
        src.fill(src.rect(), [9, 9, 9, 255], 255);
        dst.blit(&src, src.rect(), -2, -3, 255, Blend::Copy, None);
        assert_eq!(dst.pixel(1, 0)[0], 9);
        assert_eq!(dst.pixel(2, 0)[0], 0);
        assert_eq!(dst.pixel(0, 1)[0], 0);
    }

    #[test]
    fn transforms_invert_each_other() {
        let t = Transform::new((10.0, 20.0), (100.0, 50.0), 30.0, (2.0, 0.5));
        let (x, y) = t.forward(13.0, 27.0);
        let (bx, by) = t.inverse(x, y);
        assert!((bx - 13.0).abs() < 1e-9 && (by - 27.0).abs() < 1e-9);
    }

    #[test]
    fn tint_screens_and_multiplies() {
        assert_eq!(apply_tint([100, 100, 100], [255, 0, -255]), [255, 100, 0]);
    }
}
