//! Source-faithful grayscale mask generators for the built-in Siglus wipes.
//!
//! This module follows `eng_mask_00_4x4.cpp` through
//! `eng_mask_08_srect.cpp`.  The original engine builds an 8-bit grayscale
//! texture once when a wipe starts, then applies the current wipe progress in
//! the mask shader.  Keeping mask generation separate from per-frame
//! compositing also prevents random masks from changing every frame.

#[derive(Debug, Clone)]
pub struct GrayMask {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl GrayMask {
    fn new(width: u32, height: u32) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        Self {
            width,
            height,
            pixels: vec![0; width as usize * height as usize],
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some(self.pixels[y as usize * self.width as usize + x as usize])
    }

    fn set(&mut self, x: i32, y: i32, gray: u8, reverse: bool) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let gray = if reverse { 255 - gray } else { gray };
        self.pixels[y as usize * self.width as usize + x as usize] = gray;
    }

    fn fill_box(
        &mut self,
        mut x1: i32,
        mut y1: i32,
        mut x2: i32,
        mut y2: i32,
        gray: u8,
        reverse: bool,
    ) {
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }
        x1 = x1.max(0);
        y1 = y1.max(0);
        x2 = x2.min(self.width as i32 - 1);
        y2 = y2.min(self.height as i32 - 1);
        if x1 > x2 || y1 > y2 {
            return;
        }
        let gray = if reverse { 255 - gray } else { gray };
        for y in y1..=y2 {
            let row = y as usize * self.width as usize;
            self.pixels[row + x1 as usize..=row + x2 as usize].fill(gray);
        }
    }

    fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, gray: u8, reverse: bool) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.set(x0, y0, gray, reverse);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = err * 2;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn empty_box(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, gray: u8, reverse: bool) {
        self.line(x1, y1, x2, y1, gray, reverse);
        self.line(x1, y2, x2, y2, gray, reverse);
        self.line(x1, y1 + 1, x1, y2 - 1, gray, reverse);
        self.line(x2, y1 + 1, x2, y2 - 1, gray, reverse);
    }

    fn copy_from(&mut self, src: &GrayMask, dst_x: i32, dst_y: i32) {
        for y in 0..src.height as i32 {
            for x in 0..src.width as i32 {
                if let Some(gray) = src.get(x, y) {
                    self.set(dst_x + x, dst_y + y, gray, false);
                }
            }
        }
    }

    fn tiled_to(&self, width: u32, height: u32) -> GrayMask {
        let mut out = GrayMask::new(width, height);
        for y in 0..height {
            let sy = y % self.height;
            for x in 0..width {
                let sx = x % self.width;
                out.pixels[(y * width + x) as usize] = self.pixels[(sy * self.width + sx) as usize];
            }
        }
        out
    }
}

fn clamp_i32(value: i32, min: i32, max: i32) -> i32 {
    value.max(min).min(max)
}

/// Integer form of the original `linear_limit`.
fn linear_i32(value: i32, in_min: i32, out_min: i32, in_max: i32, out_max: i32) -> i32 {
    if in_max == in_min {
        return out_max;
    }
    let num = (out_max as i64 - out_min as i64) * (value as i64 - in_min as i64);
    (out_min as i64 + num / (in_max as i64 - in_min as i64)) as i32
}

fn linear_f64(value: f64, in_min: f64, out_min: f64, in_max: f64, out_max: f64) -> f64 {
    if (in_max - in_min).abs() <= f64::EPSILON {
        return out_max;
    }
    out_min + (out_max - out_min) * ((value - in_min) / (in_max - in_min))
}

fn opt(option: &[i32], index: usize, default: i32) -> i32 {
    option.get(index).copied().unwrap_or(default)
}

fn make_pattern<const N: usize>(
    cells: usize,
    pat_w: i32,
    pat_h: i32,
    reverse: bool,
    map: &[u8; N],
) -> GrayMask {
    let pat_w = pat_w.max(1);
    let pat_h = pat_h.max(1);
    let mut out = GrayMask::new((pat_w * cells as i32) as u32, (pat_h * cells as i32) as u32);
    for cy in 0..cells {
        for cx in 0..cells {
            let gray = map[cy * cells + cx];
            out.fill_box(
                (cx as i32) * pat_w,
                (cy as i32) * pat_h,
                (cx as i32 + 1) * pat_w - 1,
                (cy as i32 + 1) * pat_h - 1,
                gray,
                reverse,
            );
        }
    }
    out
}

fn make_direction(width: u32, height: u32, mut reverse: bool, dir: i32) -> GrayMask {
    let dir = dir.rem_euclid(4);
    if dir == 1 || dir == 3 {
        reverse = !reverse;
    }
    let mut out = GrayMask::new(
        if dir <= 1 { 1 } else { width },
        if dir <= 1 { height } else { 1 },
    );
    let count = if dir <= 1 {
        height as i32
    } else {
        width as i32
    };
    for i in 0..count {
        let pal = linear_i32(i, 0, 0, count.saturating_sub(1), 255);
        let gray = (255 - pal) as u8;
        if dir <= 1 {
            out.set(0, i, gray, reverse);
        } else {
            out.set(i, 0, gray, reverse);
        }
    }
    out.tiled_to(width, height)
}

fn make_direction_slice(
    width: u32,
    height: u32,
    mut reverse: bool,
    dir: i32,
    slice_len: i32,
) -> GrayMask {
    let dir = dir.rem_euclid(4);
    let slice_len = clamp_i32(slice_len, 2, 128);
    let len = if dir <= 1 {
        height as i32
    } else {
        width as i32
    };
    let count = (len + slice_len - 1) / slice_len;
    if dir == 1 || dir == 3 {
        reverse = !reverse;
    }
    let mut one = GrayMask::new(
        if dir <= 1 { 1 } else { width },
        if dir <= 1 { height } else { 1 },
    );
    for band in 0..count {
        let start_pal = linear_i32(band, 0, 0, count.saturating_sub(1), 127);
        let end_pal = start_pal + 128;
        for j in 0..slice_len {
            let pos = band * slice_len + j;
            if pos >= len {
                break;
            }
            let pal = linear_i32(j, 0, start_pal, slice_len - 1, end_pal);
            let gray = (255 - pal) as u8;
            if dir <= 1 {
                one.set(0, pos, gray, reverse);
            } else {
                one.set(pos, 0, gray, reverse);
            }
        }
    }
    one.tiled_to(width, height)
}

fn make_direction_blind(
    width: u32,
    height: u32,
    reverse: bool,
    dir: i32,
    blind_len: i32,
) -> GrayMask {
    let blind_len = clamp_i32(blind_len, 2, 128) as u32;
    if dir.rem_euclid(4) <= 1 {
        make_direction(width, blind_len, reverse, dir).tiled_to(width, height)
    } else {
        make_direction(blind_len, height, reverse, dir).tiled_to(width, height)
    }
}

fn make_direction_slant(width: u32, height: u32, mut reverse: bool, dir: i32) -> GrayMask {
    let dir = dir.rem_euclid(4);
    if dir == 2 || dir == 3 {
        reverse = !reverse;
    }
    let count = width as i32 + height as i32 - 1;
    let mut out = GrayMask::new(width, height);
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let i = if dir == 0 || dir == 3 {
                x + y
            } else {
                width as i32 - 1 - x + y
            };
            let pal = linear_i32(i, 0, 0, count.saturating_sub(1), 255);
            out.set(x, y, (255 - pal) as u8, reverse);
        }
    }
    out
}

fn make_direction_block_blind(
    width: u32,
    height: u32,
    mut reverse: bool,
    dir: i32,
    blind_len: i32,
    block_len: i32,
) -> GrayMask {
    let dir = dir.rem_euclid(4);
    let blind = (clamp_i32(blind_len, 2, 128) / 2).max(1);
    let block = clamp_i32(block_len, 1, 128);
    let (pattern_w, pattern_h) = if dir <= 1 {
        (block * 2, blind * 2)
    } else {
        (blind * 2, block * 2)
    };
    if dir == 1 || dir == 3 {
        reverse = !reverse;
    }
    let mut pattern = GrayMask::new(pattern_w as u32, pattern_h as u32);
    if dir <= 1 {
        let count = pattern_h;
        let mut shifted = blind;
        for i in 0..count {
            let pal = linear_i32(i, 0, 0, count - 1, 255);
            pattern.fill_box(0, i, block - 1, i, (255 - pal) as u8, reverse);
            pattern.fill_box(
                block,
                shifted,
                block * 2 - 1,
                shifted,
                (255 - pal) as u8,
                reverse,
            );
            shifted = (shifted + 1) % count;
        }
    } else {
        let count = pattern_w;
        let mut shifted = blind;
        for i in 0..count {
            let pal = linear_i32(i, 0, 0, count - 1, 255);
            pattern.fill_box(i, 0, i, block - 1, (255 - pal) as u8, reverse);
            pattern.fill_box(
                shifted,
                block,
                shifted,
                block * 2 - 1,
                (255 - pal) as u8,
                reverse,
            );
            shifted = (shifted + 1) % count;
        }
    }
    pattern.tiled_to(width, height)
}

fn make_random_blocks(width: u32, height: u32, pat_w: i32, pat_h: i32, seed: u32) -> GrayMask {
    let pat_w = clamp_i32(pat_w, 1, 128) as u32;
    let pat_h = clamp_i32(pat_h, 1, 128) as u32;
    let cols = width.div_ceil(pat_w);
    let rows = height.div_ceil(pat_h);
    let count = (cols * rows).max(1) as usize;
    let mut values: Vec<u8> = (0..count)
        .map(|i| linear_i32(i as i32, 0, 0, count.saturating_sub(1) as i32, 255) as u8)
        .collect();

    // `std::random_shuffle` uses the process-global C RNG.  That sequence is
    // not portable across the platforms targeted by this port.  This is the
    // same Fisher-Yates operation with a stable per-wipe seed, and the mask is
    // generated once rather than once per frame.
    let mut rng = seed | 1;
    for i in (1..values.len()).rev() {
        rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
        let j = ((rng >> 16) as usize) % (i + 1);
        values.swap(i, j);
    }

    let mut out = GrayMask::new(width, height);
    for row in 0..rows {
        for col in 0..cols {
            let gray = values[(row * cols + col) as usize];
            out.fill_box(
                (col * pat_w) as i32,
                (row * pat_h) as i32,
                ((col + 1) * pat_w - 1) as i32,
                ((row + 1) * pat_h - 1) as i32,
                gray,
                false,
            );
        }
    }
    out
}

fn make_random_line(width: u32, height: u32, dir: i32, line_len: i32, seed: u32) -> GrayMask {
    if dir == 0 {
        make_random_blocks(width, 1, line_len, 1, seed).tiled_to(width, height)
    } else {
        make_random_blocks(1, height, 1, line_len, seed).tiled_to(width, height)
    }
}

fn make_random_slant_line(width: u32, height: u32, dir: i32, line_len: i32, seed: u32) -> GrayMask {
    let tmp_len = width + height.saturating_sub(1);
    let line = make_random_blocks(tmp_len, 1, line_len, 1, seed);
    let mut out = GrayMask::new(width, height);
    for y in 0..height as i32 {
        let offset = if dir == 0 { y } else { height as i32 - 1 - y };
        for x in 0..width as i32 {
            if let Some(gray) = line.get(x + offset, 0) {
                out.set(x, y, gray, false);
            }
        }
    }
    out
}

fn make_random_cross(width: u32, height: u32, pat_w: i32, pat_h: i32, seed: u32) -> GrayMask {
    let vertical = make_random_blocks(1, height, 1, pat_h, seed).tiled_to(width, height);
    let horizontal =
        make_random_blocks(width, 1, pat_w, 1, seed ^ 0x9e37_79b9).tiled_to(width, height);
    let mut out = GrayMask::new(width, height);
    for i in 0..out.pixels.len() {
        out.pixels[i] = vertical.pixels[i].max(horizontal.pixels[i]);
    }
    out
}

fn make_random_slant_cross(width: u32, height: u32, line_len: i32, seed: u32) -> GrayMask {
    let a = make_random_slant_line(width, height, 0, line_len, seed);
    let b = make_random_slant_line(width, height, 1, line_len, seed ^ 0x9e37_79b9);
    let mut out = GrayMask::new(width, height);
    for i in 0..out.pixels.len() {
        out.pixels[i] = a.pixels[i].max(b.pixels[i]);
    }
    out
}

fn make_both_direction(width: u32, height: u32, reverse: bool, dir: i32) -> GrayMask {
    let dir = dir.rem_euclid(2);
    let mut out = GrayMask::new(width, height);
    if dir == 0 {
        let upper = height / 2;
        let lower = height.div_ceil(2);
        out.copy_from(&make_direction(1, upper.max(1), reverse, 0), 0, 0);
        let lower_mask = make_direction(1, lower.max(1), reverse, 1);
        for x in 0..width as i32 {
            out.copy_from(&lower_mask, x, upper as i32);
        }
        let upper_mask = make_direction(1, upper.max(1), reverse, 0);
        for x in 0..width as i32 {
            out.copy_from(&upper_mask, x, 0);
        }
    } else {
        let left = width / 2;
        let right = width.div_ceil(2);
        let left_mask = make_direction(left.max(1), 1, reverse, 2);
        let right_mask = make_direction(right.max(1), 1, reverse, 3);
        for y in 0..height as i32 {
            out.copy_from(&left_mask, 0, y);
            out.copy_from(&right_mask, left as i32, y);
        }
    }
    out
}

fn copy_crop(dst: &mut GrayMask, src: &GrayMask, dx: i32, dy: i32) {
    dst.copy_from(src, dx, dy);
}

fn make_both_direction_slice(
    width: u32,
    height: u32,
    reverse: bool,
    dir: i32,
    slice_len: i32,
) -> GrayMask {
    let slice = clamp_i32(slice_len, 2, 128);
    let mut out = GrayMask::new(width, height);
    if dir.rem_euclid(2) == 0 {
        let upper = height as i32 / 2;
        let lower = (height as i32 + 1) / 2;
        let a = (slice - upper.rem_euclid(slice)).rem_euclid(slice);
        let upper_mask = make_direction_slice(1, (upper + a).max(1) as u32, reverse, 0, slice);
        for x in 0..width as i32 {
            copy_crop(&mut out, &upper_mask, x, -a);
        }
        let a = (slice - lower.rem_euclid(slice)).rem_euclid(slice);
        let lower_mask = make_direction_slice(1, (lower + a).max(1) as u32, reverse, 1, slice);
        for x in 0..width as i32 {
            copy_crop(&mut out, &lower_mask, x, upper);
        }
    } else {
        let left = width as i32 / 2;
        let right = (width as i32 + 1) / 2;
        let a = (slice - left.rem_euclid(slice)).rem_euclid(slice);
        let left_mask = make_direction_slice((left + a).max(1) as u32, 1, reverse, 2, slice);
        for y in 0..height as i32 {
            copy_crop(&mut out, &left_mask, -a, y);
        }
        let a = (slice - right.rem_euclid(slice)).rem_euclid(slice);
        let right_mask = make_direction_slice((right + a).max(1) as u32, 1, reverse, 3, slice);
        for y in 0..height as i32 {
            copy_crop(&mut out, &right_mask, left, y);
        }
    }
    out
}

fn make_both_direction_blind_func(
    width: u32,
    height: u32,
    reverse: bool,
    dir: i32,
    blind_len: i32,
) -> GrayMask {
    let blind = clamp_i32(blind_len, 2, 128) as u32;
    let mut out = GrayMask::new(width, height);
    if dir.rem_euclid(4) <= 1 {
        let pattern = make_direction(width, blind, reverse, dir);
        if dir.rem_euclid(4) == 0 {
            let mut y = height as i32 - blind as i32;
            while y + blind as i32 > 0 {
                out.copy_from(&pattern, 0, y);
                y -= blind as i32;
            }
        } else {
            let mut y = 0;
            while y < height as i32 {
                out.copy_from(&pattern, 0, y);
                y += blind as i32;
            }
        }
    } else {
        let pattern = make_direction(blind, height, reverse, dir);
        if dir.rem_euclid(4) == 2 {
            let mut x = width as i32 - blind as i32;
            while x + blind as i32 > 0 {
                out.copy_from(&pattern, x, 0);
                x -= blind as i32;
            }
        } else {
            let mut x = 0;
            while x < width as i32 {
                out.copy_from(&pattern, x, 0);
                x += blind as i32;
            }
        }
    }
    out
}

fn make_both_direction_blind(
    width: u32,
    height: u32,
    reverse: bool,
    dir: i32,
    blind_len: i32,
) -> GrayMask {
    let mut out = GrayMask::new(width, height);
    if dir.rem_euclid(2) == 0 {
        let upper = height / 2;
        let lower = height.div_ceil(2);
        let a = make_both_direction_blind_func(1, upper.max(1), reverse, 0, blind_len);
        let b = make_both_direction_blind_func(1, lower.max(1), reverse, 1, blind_len);
        for x in 0..width as i32 {
            out.copy_from(&a, x, 0);
            out.copy_from(&b, x, upper as i32);
        }
    } else {
        let left = width / 2;
        let right = width.div_ceil(2);
        let a = make_both_direction_blind_func(left.max(1), 1, reverse, 2, blind_len);
        let b = make_both_direction_blind_func(right.max(1), 1, reverse, 3, blind_len);
        for y in 0..height as i32 {
            out.copy_from(&a, 0, y);
            out.copy_from(&b, left as i32, y);
        }
    }
    out
}

fn make_both_direction_stripe(
    width: u32,
    height: u32,
    reverse: bool,
    dir: i32,
    stripe_len: i32,
) -> GrayMask {
    let stripe = clamp_i32(stripe_len, 1, 128);
    let pitch = stripe * 2;
    let mut pattern = if dir.rem_euclid(2) == 0 {
        GrayMask::new(1, height)
    } else {
        GrayMask::new(width, 1)
    };
    if dir.rem_euclid(2) == 0 {
        let count = (height as i32 + pitch - 1) / pitch;
        let mut y1 = 0;
        for i in 0..count {
            let pal = linear_i32(i, 0, 0, count.saturating_sub(1), 255);
            pattern.fill_box(0, y1, 0, y1 + stripe - 1, (255 - pal) as u8, reverse);
            y1 += stripe;
            pattern.fill_box(0, y1, 0, y1 + stripe - 1, pal as u8, reverse);
            y1 += stripe;
        }
    } else {
        let count = (width as i32 + pitch - 1) / pitch;
        let mut x1 = 0;
        for i in 0..count {
            let pal = linear_i32(i, 0, 0, count.saturating_sub(1), 255);
            pattern.fill_box(x1, 0, x1 + stripe - 1, 0, (255 - pal) as u8, reverse);
            x1 += stripe;
            pattern.fill_box(x1, 0, x1 + stripe - 1, 0, pal as u8, reverse);
            x1 += stripe;
        }
    }
    pattern.tiled_to(width, height)
}

fn make_both_direction_stripe2(
    width: u32,
    height: u32,
    reverse: bool,
    dir: i32,
    stripe_len: i32,
) -> GrayMask {
    let stripe = clamp_i32(stripe_len, 1, 128);
    let mut pattern = if dir.rem_euclid(2) == 0 {
        GrayMask::new((stripe * 2) as u32, height)
    } else {
        GrayMask::new(width, (stripe * 2) as u32)
    };
    if dir.rem_euclid(2) == 0 {
        for y in 0..height as i32 {
            let pal = linear_i32(y, 0, 0, height as i32 - 1, 255);
            pattern.fill_box(0, y, stripe - 1, y, (255 - pal) as u8, reverse);
            pattern.fill_box(stripe, y, stripe * 2 - 1, y, pal as u8, reverse);
        }
    } else {
        for x in 0..width as i32 {
            let pal = linear_i32(x, 0, 0, width as i32 - 1, 255);
            pattern.fill_box(x, 0, x, stripe - 1, (255 - pal) as u8, reverse);
            pattern.fill_box(x, stripe, x, stripe * 2 - 1, pal as u8, reverse);
        }
    }
    pattern.tiled_to(width, height)
}

fn make_cross_direction(width: u32, height: u32, reverse: bool) -> GrayMask {
    let left = width / 2;
    let right = width.div_ceil(2);
    let upper = height / 2;
    let lower = height.div_ceil(2);
    let mut out = GrayMask::new(width, height);

    let q0 = make_direction(left.max(1), 1, reverse, 2);
    for y in 0..upper as i32 {
        out.copy_from(&q0, 0, y);
    }
    let q1 = make_direction(right.max(1), 1, reverse, 3);
    for y in upper as i32..height as i32 {
        out.copy_from(&q1, left as i32, y);
    }
    let q2 = make_direction(1, upper.max(1), reverse, 0);
    for x in left as i32..width as i32 {
        out.copy_from(&q2, x, 0);
    }
    let q3 = make_direction(1, lower.max(1), reverse, 1);
    for x in 0..left as i32 {
        out.copy_from(&q3, x, upper as i32);
    }
    out
}

fn make_cross_direction_slice(width: u32, height: u32, reverse: bool, slice_len: i32) -> GrayMask {
    let slice = clamp_i32(slice_len, 2, 128);
    let left = width as i32 / 2;
    let right = (width as i32 + 1) / 2;
    let upper = height as i32 / 2;
    let lower = (height as i32 + 1) / 2;
    let mut out = GrayMask::new(width, height);

    let a = (slice - left.rem_euclid(slice)).rem_euclid(slice);
    let q0 = make_direction_slice((left + a).max(1) as u32, 1, reverse, 2, slice);
    for y in 0..upper {
        out.copy_from(&q0, -a, y);
    }
    let a = (slice - right.rem_euclid(slice)).rem_euclid(slice);
    let q1 = make_direction_slice((right + a).max(1) as u32, 1, reverse, 3, slice);
    for y in upper..height as i32 {
        out.copy_from(&q1, left, y);
    }
    let a = (slice - upper.rem_euclid(slice)).rem_euclid(slice);
    let q2 = make_direction_slice(1, (upper + a).max(1) as u32, reverse, 0, slice);
    for x in left..width as i32 {
        out.copy_from(&q2, x, -a);
    }
    let a = (slice - lower.rem_euclid(slice)).rem_euclid(slice);
    let q3 = make_direction_slice(1, (lower + a).max(1) as u32, reverse, 1, slice);
    for x in 0..left {
        out.copy_from(&q3, x, upper);
    }
    out
}

fn make_cross_direction_blind(width: u32, height: u32, reverse: bool, blind_len: i32) -> GrayMask {
    let left = width / 2;
    let right = width.div_ceil(2);
    let upper = height / 2;
    let lower = height.div_ceil(2);
    let mut out = GrayMask::new(width, height);

    let q0 = make_both_direction_blind_func(left.max(1), 1, reverse, 2, blind_len);
    for y in 0..upper as i32 {
        out.copy_from(&q0, 0, y);
    }
    let q1 = make_both_direction_blind_func(right.max(1), 1, reverse, 3, blind_len);
    for y in upper as i32..height as i32 {
        out.copy_from(&q1, left as i32, y);
    }
    let q2 = make_both_direction_blind_func(1, upper.max(1), reverse, 0, blind_len);
    for x in left as i32..width as i32 {
        out.copy_from(&q2, x, 0);
    }
    let q3 = make_both_direction_blind_func(1, lower.max(1), reverse, 1, blind_len);
    for x in 0..left as i32 {
        out.copy_from(&q3, x, upper as i32);
    }
    out
}

fn quadrant_and_local_angle(x: f64, y: f64, cx: f64, cy: f64) -> (usize, f64) {
    let mut angle = (y - cy).atan2(x - cx).to_degrees();
    if angle < 0.0 {
        angle += 360.0;
    }
    if angle < 90.0 {
        (2, angle)
    } else if angle < 180.0 {
        (3, angle - 90.0)
    } else if angle < 270.0 {
        (0, angle - 180.0)
    } else {
        (1, angle - 270.0)
    }
}

fn gray_from_palette(index: i32, reverse: bool) -> u8 {
    let gray = (255 - clamp_i32(index, 0, 255)) as u8;
    if reverse { 255 - gray } else { gray }
}

fn make_around_one(width: u32, height: u32, reverse: bool, dir: i32) -> GrayMask {
    let dir = dir.rem_euclid(4) as usize;
    let cx = width as f64 / 2.0 - 0.5;
    let cy = height as f64 / 2.0 - 0.5;
    let mut out = GrayMask::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let (quadrant, local) = quadrant_and_local_angle(x as f64, y as f64, cx, cy);
            let segment = (quadrant + 4 - dir) % 4;
            let local_pal = linear_f64(local, 0.0, 0.0, 90.0, 63.0).round() as i32;
            out.pixels[(y * width + x) as usize] =
                gray_from_palette(segment as i32 * 64 + local_pal, reverse);
        }
    }
    out
}

fn make_around_half(width: u32, height: u32, reverse: bool, dir: i32) -> GrayMask {
    let dir = dir.rem_euclid(2);
    let cx = width as f64 / 2.0 - 0.5;
    let cy = height as f64 / 2.0 - 0.5;
    let mut out = GrayMask::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let (q, local) = quadrant_and_local_angle(x as f64, y as f64, cx, cy);
            let high = if dir == 0 {
                q == 1 || q == 3
            } else {
                q == 0 || q == 2
            };
            let base = if high { 128 } else { 0 };
            let pal = base + linear_f64(local, 0.0, 0.0, 90.0, 127.0).round() as i32;
            out.pixels[(y * width + x) as usize] = gray_from_palette(pal, reverse);
        }
    }
    out
}

fn make_around_divide(width: u32, height: u32, reverse: bool, divide_mod: i32) -> GrayMask {
    let divide = 1_i32 << clamp_i32(divide_mod, 0, 4);
    let cx = width as f64 / 2.0 - 0.5;
    let cy = height as f64 / 2.0 - 0.5;
    let sector = 90.0 / divide as f64;
    let mut out = GrayMask::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let (_, local) = quadrant_and_local_angle(x as f64, y as f64, cx, cy);
            let within = local % sector;
            let pal = linear_f64(within, 0.0, 0.0, sector, 255.0).round() as i32;
            out.pixels[(y * width + x) as usize] = gray_from_palette(pal, reverse);
        }
    }
    out
}

fn clockwise_angle(start_x: f64, start_y: f64, vx: f64, vy: f64) -> f64 {
    let dot = start_x * vx + start_y * vy;
    let cross = start_x * vy - start_y * vx;
    let mut angle = cross.atan2(dot).to_degrees();
    if angle < 0.0 {
        angle += 360.0;
    }
    angle
}

fn fan_mask(
    width: u32,
    height: u32,
    reverse: bool,
    origin: (f64, f64),
    start: (f64, f64),
    pal_start: i32,
    pal_end: i32,
) -> GrayMask {
    let mut min_angle: f64 = 90.0;
    let mut max_angle: f64 = 0.0;
    for (x, y) in [
        (0.0, 0.0),
        (width.saturating_sub(1) as f64, 0.0),
        (0.0, height.saturating_sub(1) as f64),
        (
            width.saturating_sub(1) as f64,
            height.saturating_sub(1) as f64,
        ),
    ] {
        let angle = clockwise_angle(start.0, start.1, x - origin.0, y - origin.1);
        if angle <= 90.0 {
            min_angle = min_angle.min(angle);
            max_angle = max_angle.max(angle);
        }
    }
    if max_angle <= min_angle {
        min_angle = 0.0;
        max_angle = 90.0;
    }
    let mut out = GrayMask::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let angle = clockwise_angle(start.0, start.1, x as f64 - origin.0, y as f64 - origin.1);
            if angle <= 90.0 {
                let pal = linear_f64(
                    angle,
                    min_angle,
                    pal_start as f64,
                    max_angle,
                    pal_end as f64,
                )
                .round() as i32;
                out.pixels[(y * width + x) as usize] = gray_from_palette(pal, reverse);
            }
        }
    }
    out
}

fn make_oogi_center(width: u32, height: u32, reverse: bool) -> GrayMask {
    let cx = width as f64 / 2.0 - 0.5;
    let cy = height as f64 / 2.0 - 0.5;
    let mut out = GrayMask::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let (q, local) = quadrant_and_local_angle(x as f64, y as f64, cx, cy);
            let local_reverse = if q == 1 || q == 3 { !reverse } else { reverse };
            let pal = linear_f64(local, 0.0, 0.0, 90.0, 255.0).round() as i32;
            out.pixels[(y * width + x) as usize] = gray_from_palette(pal, local_reverse);
        }
    }
    out
}

fn corner_origin(width: u32, height: u32, dir: i32, distant: bool) -> ((f64, f64), (f64, f64)) {
    let d = if distant {
        width.min(height) as f64
    } else {
        0.0
    };
    match dir.rem_euclid(4) {
        0 => ((-d, -d), (1.0, 0.0)),
        1 => ((width as f64 - 1.0 + d, -d), (0.0, 1.0)),
        2 => ((-d, height as f64 - 1.0 + d), (0.0, -1.0)),
        _ => (
            (width as f64 - 1.0 + d, height as f64 - 1.0 + d),
            (-1.0, 0.0),
        ),
    }
}

fn make_oogi_corner(width: u32, height: u32, reverse: bool, dir: i32, distant: bool) -> GrayMask {
    let (origin, start) = corner_origin(width, height, dir, distant);
    fan_mask(width, height, reverse, origin, start, 0, 255)
}

fn edge_fans(width: u32, height: u32, dir: i32, distant: bool) -> [((f64, f64), (f64, f64)); 2] {
    match dir.rem_euclid(4) {
        0 => {
            let d = if distant { height as f64 } else { 0.0 };
            [
                ((width as f64 / 2.0, -d), (1.0, 0.0)),
                ((width as f64 / 2.0 - 1.0, -d), (0.0, 1.0)),
            ]
        }
        1 => {
            let d = if distant { height as f64 } else { 0.0 };
            [
                (
                    (width as f64 / 2.0 - 1.0, height as f64 - 1.0 + d),
                    (-1.0, 0.0),
                ),
                ((width as f64 / 2.0, height as f64 - 1.0 + d), (0.0, -1.0)),
            ]
        }
        2 => {
            let d = if distant { width as f64 } else { 0.0 };
            [
                ((-d, height as f64 / 2.0 - 1.0), (0.0, -1.0)),
                ((-d, height as f64 / 2.0), (1.0, 0.0)),
            ]
        }
        _ => {
            let d = if distant { width as f64 } else { 0.0 };
            [
                ((width as f64 - 1.0 + d, height as f64 / 2.0), (0.0, 1.0)),
                (
                    (width as f64 - 1.0 + d, height as f64 / 2.0 - 1.0),
                    (-1.0, 0.0),
                ),
            ]
        }
    }
}

fn make_oogi_edge(
    width: u32,
    height: u32,
    reverse: bool,
    dir: i32,
    mode: i32,
    distant: bool,
) -> GrayMask {
    let [a, b] = edge_fans(width, height, dir, distant);
    let a_mask = if mode == 0 {
        fan_mask(width, height, reverse, a.0, a.1, 0, 127)
    } else {
        fan_mask(width, height, reverse, a.0, a.1, 0, 255)
    };
    let b_mask = if mode == 0 {
        fan_mask(width, height, reverse, b.0, b.1, 128, 255)
    } else {
        fan_mask(width, height, !reverse, b.0, b.1, 0, 255)
    };
    let mut out = a_mask;
    for i in 0..out.pixels.len() {
        if b_mask.pixels[i] != 0 {
            out.pixels[i] = b_mask.pixels[i];
        }
    }
    out
}

fn make_square(width: u32, height: u32, reverse: bool) -> GrayMask {
    let mut out = GrayMask::new(width, height);
    let count = width.max(height).div_ceil(2).max(1) as i32;
    let ax2 = width as i32 - 1;
    let ay2 = height as i32 - 1;
    let bx1 = width as i32 / 2 - 1;
    let by1 = height as i32 / 2 - 1;
    let bx2 = bx1 + 1;
    let by2 = by1 + 1;
    for i in 0..count {
        let pal = linear_i32(i, 0, 0, count - 1, 255);
        out.empty_box(
            linear_i32(i, 0, 0, count - 1, bx1),
            linear_i32(i, 0, 0, count - 1, by1),
            linear_i32(i, 0, ax2, count - 1, bx2),
            linear_i32(i, 0, ay2, count - 1, by2),
            (255 - pal) as u8,
            reverse,
        );
    }
    out
}

fn make_rhombus(width: u32, height: u32, reverse: bool) -> GrayMask {
    let mut out = GrayMask::new(width, height);
    let quadrants = [
        (0, 0, width as i32 / 2, height as i32 / 2, 1, 1),
        (
            width as i32 - 1,
            0,
            (width as i32 + 1) / 2,
            height as i32 / 2,
            -1,
            1,
        ),
        (
            0,
            height as i32 - 1,
            width as i32 / 2,
            (height as i32 + 1) / 2,
            1,
            -1,
        ),
        (
            width as i32 - 1,
            height as i32 - 1,
            (width as i32 + 1) / 2,
            (height as i32 + 1) / 2,
            -1,
            -1,
        ),
    ];
    for (start_x, start_y, x_len, y_len, x_dir, y_dir) in quadrants {
        if x_len <= 0 || y_len <= 0 {
            continue;
        }
        let count = x_len + y_len - 1;
        let mut x1 = start_x;
        let mut y1 = start_y;
        let mut x2 = start_x;
        let mut y2 = start_y;
        for i in 0..count {
            let pal = linear_i32(i, 0, 0, count - 1, 255);
            out.line(x1, y1, x2, y2, (255 - pal) as u8, reverse);
            if (x1 - start_x).abs() < x_len - 1 {
                x1 += x_dir;
            } else {
                y1 += y_dir;
            }
            if (y2 - start_y).abs() < y_len - 1 {
                y2 += y_dir;
            } else {
                x2 += x_dir;
            }
        }
    }
    out
}

fn make_jyuuji(width: u32, height: u32, reverse: bool) -> GrayMask {
    let center_x = width as i32 / 2 - 1;
    let center_y = height as i32 / 2 - 1;
    let len = center_x
        .abs()
        .max((width as i32 - center_x).abs())
        .max(center_y.abs())
        .max((height as i32 - center_y).abs());
    let count = len.max(1);
    let mut out = GrayMask::new(width, height);
    let starts = [
        (center_x - len, center_y - len, center_x, center_y, -1, -1),
        (
            center_x + 1 + len,
            center_y - len,
            center_x + 1,
            center_y,
            1,
            -1,
        ),
        (
            center_x - len,
            center_y + 1 + len,
            center_x,
            center_y + 1,
            -1,
            1,
        ),
        (
            center_x + 1 + len,
            center_y + 1 + len,
            center_x + 1,
            center_y + 1,
            1,
            1,
        ),
    ];
    for (x1, y1, mut x2, mut y2, dx, dy) in starts {
        for i in 0..count {
            let pal = linear_i32(i, 0, 0, count - 1, 255) as u8;
            out.empty_box(x1, y1, x2, y2, pal, reverse);
            x2 += dx;
            y2 += dy;
        }
    }
    out
}

fn make_television(width: u32, height: u32, reverse: bool) -> GrayMask {
    let mut out = GrayMask::new(width, height);
    let mut count = (height as i32) / 16;
    if count == 0 {
        count = 1;
    }
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = width as i32 - 1;
    let mut y2 = height as i32 - 1;
    for i in 0..count {
        let pal = linear_i32(i, 0, 0, count - 1, 15);
        out.line(x1, y1, x2, y1, (255 - pal) as u8, reverse);
        out.line(x1, y2, x2, y2, (255 - pal) as u8, reverse);
        y1 += 1;
        y2 -= 1;
    }
    let y_alpha = (height as i32 - count * 2) / 2;
    count = width as i32 / 2;
    if count & 1 != 0 {
        count += 1;
    }
    for i in 0..count.max(1) {
        let pal = linear_i32(i, 0, 16, count.max(1) - 1, 255);
        let y_add = linear_i32(i * 3, 0, 0, count.max(1) - 1, y_alpha);
        out.fill_box(x1, y1 + y_add, x2, y2 - y_add, (255 - pal) as u8, reverse);
        x1 += 1;
        x2 -= 1;
    }
    out
}

fn with_gap(base: GrayMask, pat_w: u32, pat_h: u32) -> GrayMask {
    let mut out = GrayMask::new(pat_w, pat_h * 2);
    out.copy_from(&base, 0, 0);
    out.copy_from(&base, -(pat_w as i32 / 2), pat_h as i32);
    out.copy_from(&base, pat_w as i32 / 2, pat_h as i32);
    out
}

fn prepare_srect(mut pat_w: i32, mut pat_h: i32, gap: bool) -> (u32, u32) {
    pat_w = clamp_i32(pat_w, 4, 128);
    pat_h = clamp_i32(pat_h, 4, 128);
    if gap {
        pat_w &= !1;
        pat_h &= !1;
    }
    (pat_w as u32, pat_h as u32)
}

fn make_srect(
    width: u32,
    height: u32,
    pat_w: i32,
    pat_h: i32,
    gap: bool,
    build: impl FnOnce(u32, u32) -> GrayMask,
) -> GrayMask {
    let (pw, ph) = prepare_srect(pat_w, pat_h, gap);
    let base = build(pw, ph);
    let pattern = if gap { with_gap(base, pw, ph) } else { base };
    pattern.tiled_to(width, height)
}

fn pattern_selection(all: &[u8], cells: usize, mode: i32) -> &[u8] {
    let mode = clamp_i32(mode, 0, 7) as usize;
    let size = cells * cells;
    &all[mode * size..(mode + 1) * size]
}

fn make_dynamic_pattern(
    width: u32,
    height: u32,
    cells: usize,
    pat_w: i32,
    pat_h: i32,
    reverse: bool,
    map: &[u8],
) -> GrayMask {
    let max = if cells == 8 { 64 } else { 128 };
    let pat_w = clamp_i32(pat_w, 1, max);
    let pat_h = clamp_i32(pat_h, 1, max);
    let mut pattern = GrayMask::new((pat_w * cells as i32) as u32, (pat_h * cells as i32) as u32);
    for y in 0..cells {
        for x in 0..cells {
            pattern.fill_box(
                x as i32 * pat_w,
                y as i32 * pat_h,
                (x as i32 + 1) * pat_w - 1,
                (y as i32 + 1) * pat_h - 1,
                map[y * cells + x],
                reverse,
            );
        }
    }
    pattern.tiled_to(width, height)
}

pub fn generate(
    wipe_type: i32,
    option: &[i32],
    width: u32,
    height: u32,
    random_seed: u32,
) -> Option<GrayMask> {
    let width = width.max(1);
    let height = height.max(1);
    let mask = match wipe_type {
        5 | 101 => {
            make_direction_slice(width, height, false, opt(option, 1, 0), opt(option, 2, 16))
        }
        10 => make_dynamic_pattern(
            width,
            height,
            4,
            opt(option, 1, 1),
            opt(option, 2, 1),
            false,
            &JIWA9_4,
        ),
        11 => make_dynamic_pattern(
            width,
            height,
            4,
            opt(option, 1, 1),
            opt(option, 2, 1),
            false,
            &JIWA7_4,
        ),
        20 => make_dynamic_pattern(
            width,
            height,
            4,
            opt(option, 1, 1),
            opt(option, 2, 1),
            opt(option, 4, 0) != 0,
            pattern_selection(&TURN_AROUND_4, 4, opt(option, 3, 0)),
        ),
        21 => make_dynamic_pattern(
            width,
            height,
            4,
            opt(option, 1, 1),
            opt(option, 2, 1),
            opt(option, 4, 0) != 0,
            pattern_selection(&TURN_RET_4, 4, opt(option, 3, 0)),
        ),
        22 => make_dynamic_pattern(
            width,
            height,
            4,
            opt(option, 1, 1),
            opt(option, 2, 1),
            opt(option, 4, 0) != 0,
            pattern_selection(&TURN_DOWN_4, 4, opt(option, 3, 0)),
        ),
        30 => make_dynamic_pattern(
            width,
            height,
            8,
            opt(option, 1, 1),
            opt(option, 2, 1),
            opt(option, 4, 0) != 0,
            pattern_selection(&TURN_AROUND_8, 8, opt(option, 3, 0)),
        ),
        31 => make_dynamic_pattern(
            width,
            height,
            8,
            opt(option, 1, 1),
            opt(option, 2, 1),
            opt(option, 4, 0) != 0,
            pattern_selection(&TURN_RET_8, 8, opt(option, 3, 0)),
        ),
        32 => make_dynamic_pattern(
            width,
            height,
            8,
            opt(option, 1, 1),
            opt(option, 2, 1),
            opt(option, 4, 0) != 0,
            pattern_selection(&TURN_DOWN_8, 8, opt(option, 3, 0)),
        ),
        40 => make_random_blocks(
            width,
            height,
            opt(option, 1, 1),
            opt(option, 2, 1),
            random_seed,
        ),
        41 => make_random_line(
            width,
            height,
            opt(option, 1, 0),
            opt(option, 2, 1),
            random_seed,
        ),
        42 => make_random_slant_line(
            width,
            height,
            opt(option, 1, 0),
            opt(option, 2, 1),
            random_seed,
        ),
        43 => make_random_cross(
            width,
            height,
            opt(option, 1, 1),
            opt(option, 2, 1),
            random_seed,
        ),
        44 => make_random_slant_cross(width, height, opt(option, 1, 1), random_seed),
        60 => make_around_one(width, height, opt(option, 2, 0) != 0, opt(option, 1, 0)),
        61 => make_around_half(width, height, opt(option, 2, 0) != 0, opt(option, 1, 0)),
        62 => make_around_divide(width, height, opt(option, 2, 0) != 0, opt(option, 1, 0)),
        63 => make_oogi_center(width, height, opt(option, 1, 0) != 0),
        64 => make_oogi_corner(
            width,
            height,
            opt(option, 2, 0) != 0,
            opt(option, 1, 0),
            false,
        ),
        65 | 66 => make_oogi_edge(
            width,
            height,
            opt(option, 2, 0) != 0,
            opt(option, 1, 0),
            wipe_type - 65,
            false,
        ),
        67 => make_oogi_corner(
            width,
            height,
            opt(option, 2, 0) != 0,
            opt(option, 1, 0),
            true,
        ),
        68 | 69 => make_oogi_edge(
            width,
            height,
            opt(option, 2, 0) != 0,
            opt(option, 1, 0),
            wipe_type - 68,
            true,
        ),
        70..=79 => {
            let gap = opt(option, 5, 0) != 0;
            let reverse = if wipe_type == 73 {
                opt(option, 4, 0) != 0
            } else {
                gap
            };
            let p1 = opt(option, 1, 64);
            let p2 = opt(option, 2, 64);
            let p3 = opt(option, 3, 0);
            let p4 = opt(option, 4, 0);
            make_srect(width, height, p1, p2, p3 != 0, |pw, ph| match wipe_type {
                70 => make_around_one(pw, ph, reverse, p4),
                71 => make_around_half(pw, ph, reverse, p4),
                72 => make_around_divide(pw, ph, reverse, p4),
                73 => make_oogi_center(pw, ph, reverse),
                74 => make_oogi_corner(pw, ph, reverse, p4, false),
                75 | 76 => make_oogi_edge(pw, ph, reverse, p4, wipe_type - 75, false),
                77 => make_oogi_corner(pw, ph, reverse, p4, true),
                78 | 79 => make_oogi_edge(pw, ph, reverse, p4, wipe_type - 78, true),
                _ => unreachable!(),
            })
        }
        80 => make_square(width, height, opt(option, 1, 0) != 0),
        81 => make_rhombus(width, height, opt(option, 1, 0) != 0),
        82 => make_jyuuji(width, height, opt(option, 1, 0) != 0),
        83 => make_television(width, height, opt(option, 1, 0) != 0),
        90..=93 => {
            let reverse = opt(option, 4, 0) != 0;
            make_srect(
                width,
                height,
                opt(option, 1, 64),
                opt(option, 2, 64),
                opt(option, 3, 0) != 0,
                |pw, ph| match wipe_type {
                    90 => make_square(pw, ph, reverse),
                    91 => make_rhombus(pw, ph, reverse),
                    92 => make_jyuuji(pw, ph, reverse),
                    93 => make_television(pw, ph, reverse),
                    _ => unreachable!(),
                },
            )
        }
        100 => make_direction(width, height, false, opt(option, 1, 0)),
        102 => make_direction_blind(width, height, false, opt(option, 1, 0), opt(option, 2, 16)),
        110 => make_direction_slant(width, height, false, opt(option, 1, 0)),
        111 => make_direction_block_blind(
            width,
            height,
            false,
            opt(option, 1, 0),
            opt(option, 2, 16),
            opt(option, 3, 16),
        ),
        120 => make_both_direction(width, height, opt(option, 2, 0) != 0, opt(option, 1, 0)),
        121 => make_both_direction_slice(
            width,
            height,
            opt(option, 3, 0) != 0,
            opt(option, 1, 0),
            opt(option, 2, 16),
        ),
        122 => make_both_direction_blind(
            width,
            height,
            opt(option, 3, 0) != 0,
            opt(option, 1, 0),
            opt(option, 2, 16),
        ),
        130 => {
            make_both_direction_stripe(width, height, false, opt(option, 1, 0), opt(option, 2, 16))
        }
        131 => {
            make_both_direction_stripe2(width, height, false, opt(option, 1, 0), opt(option, 2, 16))
        }
        132 => {
            let dir = opt(option, 1, 0);
            let blind = opt(option, 2, 16);
            let block = opt(option, 3, 16);
            if dir == 0 {
                make_both_direction_stripe2(width, clamp_i32(blind, 2, 128) as u32, false, 0, block)
                    .tiled_to(width, height)
            } else {
                make_both_direction_stripe2(
                    clamp_i32(blind, 2, 128) as u32,
                    height,
                    false,
                    1,
                    block,
                )
                .tiled_to(width, height)
            }
        }
        140 => make_cross_direction(width, height, opt(option, 1, 0) != 0),
        141 => {
            make_cross_direction_slice(width, height, opt(option, 2, 0) != 0, opt(option, 1, 16))
        }
        142 => {
            make_cross_direction_blind(width, height, opt(option, 2, 0) != 0, opt(option, 1, 16))
        }
        150..=152 => {
            let mut pw = opt(option, 1, 64);
            let mut ph = opt(option, 2, 64);
            let (prepared_w, prepared_h) = prepare_srect(pw, ph, false);
            pw = (prepared_w * 2) as i32;
            ph = (prepared_h * 2) as i32;
            let pattern = match wipe_type {
                150 => make_cross_direction(pw as u32, ph as u32, opt(option, 3, 0) != 0),
                151 => make_cross_direction_slice(
                    pw as u32,
                    ph as u32,
                    opt(option, 4, 0) != 0,
                    opt(option, 3, 16),
                ),
                152 => make_cross_direction_blind(
                    pw as u32,
                    ph as u32,
                    opt(option, 4, 0) != 0,
                    opt(option, 3, 16),
                ),
                _ => unreachable!(),
            };
            pattern.tiled_to(width, height)
        }
        _ => return None,
    };
    Some(mask)
}

const JIWA9_4: [u8; 16] = [
    255, 64, 192, 32, 96, 160, 128, 160, 192, 0, 224, 64, 128, 160, 96, 160,
];

const JIWA7_4: [u8; 16] = [
    255, 43, 213, 0, 85, 170, 128, 170, 213, 0, 255, 43, 128, 170, 85, 170,
];

const TURN_AROUND_4: [u8; 128] = [
    255, 238, 221, 204, 68, 51, 34, 187, 85, 0, 17, 170, 102, 119, 136, 153, 102, 85, 68, 255, 119,
    0, 51, 238, 136, 17, 34, 221, 153, 170, 187, 204, 153, 136, 119, 102, 170, 17, 0, 85, 187, 34,
    51, 68, 204, 221, 238, 255, 204, 187, 170, 153, 221, 34, 17, 136, 238, 51, 0, 119, 255, 68, 85,
    102, 204, 221, 238, 255, 187, 34, 51, 68, 170, 17, 0, 85, 153, 136, 119, 102, 255, 68, 85, 102,
    238, 51, 0, 119, 221, 34, 17, 136, 204, 187, 170, 153, 102, 119, 136, 153, 85, 0, 17, 170, 68,
    51, 34, 187, 255, 238, 221, 204, 153, 170, 187, 204, 136, 17, 34, 221, 119, 0, 51, 238, 102,
    85, 68, 255,
];

const TURN_RET_4: [u8; 128] = [
    255, 238, 221, 204, 187, 170, 153, 136, 119, 102, 85, 68, 51, 34, 17, 0, 204, 221, 238, 255,
    136, 153, 170, 187, 68, 85, 102, 119, 0, 17, 34, 51, 51, 34, 17, 0, 119, 102, 85, 68, 187, 170,
    153, 136, 255, 238, 221, 204, 0, 17, 34, 51, 68, 85, 102, 119, 136, 153, 170, 187, 204, 221,
    238, 255, 255, 187, 119, 51, 238, 170, 102, 34, 221, 153, 85, 17, 204, 136, 68, 0, 204, 136,
    68, 0, 221, 153, 85, 17, 238, 170, 102, 34, 255, 187, 119, 51, 51, 119, 187, 255, 34, 102, 170,
    238, 17, 85, 153, 221, 0, 68, 136, 204, 0, 68, 136, 204, 17, 85, 153, 221, 34, 102, 170, 238,
    51, 119, 187, 255,
];

const TURN_DOWN_4: [u8; 128] = [
    255, 238, 221, 204, 136, 153, 170, 187, 119, 102, 85, 68, 0, 17, 34, 51, 204, 221, 238, 255,
    187, 170, 153, 136, 68, 85, 102, 119, 51, 34, 17, 0, 0, 17, 34, 51, 119, 102, 85, 68, 136, 153,
    170, 187, 255, 238, 221, 204, 51, 34, 17, 0, 68, 85, 102, 119, 187, 170, 153, 136, 204, 221,
    238, 255, 255, 136, 119, 0, 238, 153, 102, 17, 221, 170, 85, 34, 204, 187, 68, 51, 204, 187,
    68, 51, 221, 170, 85, 34, 238, 153, 102, 17, 255, 136, 119, 0, 0, 119, 136, 255, 17, 102, 153,
    238, 34, 85, 170, 221, 51, 68, 187, 204, 51, 68, 187, 204, 34, 85, 170, 221, 17, 102, 153, 238,
    0, 119, 136, 255,
];

const TURN_AROUND_8: [u8; 512] = [
    255, 251, 247, 243, 239, 235, 231, 227, 146, 142, 138, 134, 130, 126, 121, 223, 150, 65, 61,
    56, 52, 48, 117, 219, 154, 69, 16, 12, 8, 44, 113, 215, 158, 73, 20, 0, 4, 40, 109, 211, 162,
    77, 24, 28, 32, 36, 105, 207, 166, 81, 85, 89, 93, 97, 101, 203, 170, 174, 178, 182, 186, 191,
    195, 199, 170, 166, 162, 158, 154, 150, 146, 255, 174, 81, 77, 73, 69, 65, 142, 251, 178, 85,
    24, 20, 16, 61, 138, 247, 182, 89, 28, 0, 12, 56, 134, 243, 186, 93, 32, 4, 8, 52, 130, 239,
    191, 97, 36, 40, 44, 48, 126, 235, 195, 101, 105, 109, 113, 117, 121, 231, 199, 203, 207, 211,
    215, 219, 223, 227, 199, 195, 191, 186, 182, 178, 174, 170, 203, 101, 97, 93, 89, 85, 81, 166,
    207, 105, 36, 32, 28, 24, 77, 162, 211, 109, 40, 4, 0, 20, 73, 158, 215, 113, 44, 8, 12, 16,
    69, 154, 219, 117, 48, 52, 56, 61, 65, 150, 223, 121, 126, 130, 134, 138, 142, 146, 227, 231,
    235, 239, 243, 247, 251, 255, 227, 223, 219, 215, 211, 207, 203, 199, 231, 121, 117, 113, 109,
    105, 101, 195, 235, 126, 48, 44, 40, 36, 97, 191, 239, 130, 52, 8, 4, 32, 93, 186, 243, 134,
    56, 12, 0, 28, 89, 182, 247, 138, 61, 16, 20, 24, 85, 178, 251, 142, 65, 69, 73, 77, 81, 174,
    255, 146, 150, 154, 158, 162, 166, 170, 227, 231, 235, 239, 243, 247, 251, 255, 223, 121, 126,
    130, 134, 138, 142, 146, 219, 117, 48, 52, 56, 61, 65, 150, 215, 113, 44, 8, 12, 16, 69, 154,
    211, 109, 40, 4, 0, 20, 73, 158, 207, 105, 36, 32, 28, 24, 77, 162, 203, 101, 97, 93, 89, 85,
    81, 166, 199, 195, 191, 186, 182, 178, 174, 170, 255, 146, 150, 154, 158, 162, 166, 170, 251,
    142, 65, 69, 73, 77, 81, 174, 247, 138, 61, 16, 20, 24, 85, 178, 243, 134, 56, 12, 0, 28, 89,
    182, 239, 130, 52, 8, 4, 32, 93, 186, 235, 126, 48, 44, 40, 36, 97, 191, 231, 121, 117, 113,
    109, 105, 101, 195, 227, 223, 219, 215, 211, 207, 203, 199, 170, 174, 178, 182, 186, 191, 195,
    199, 166, 81, 85, 89, 93, 97, 101, 203, 162, 77, 24, 28, 32, 36, 105, 207, 158, 73, 20, 0, 4,
    40, 109, 211, 154, 69, 16, 12, 8, 44, 113, 215, 150, 65, 61, 56, 52, 48, 117, 219, 146, 142,
    138, 134, 130, 126, 121, 223, 255, 251, 247, 243, 239, 235, 231, 227, 199, 203, 207, 211, 215,
    219, 223, 227, 195, 101, 105, 109, 113, 117, 121, 231, 191, 97, 36, 40, 44, 48, 126, 235, 186,
    93, 32, 4, 8, 52, 130, 239, 182, 89, 28, 0, 12, 56, 134, 243, 178, 85, 24, 20, 16, 61, 138,
    247, 174, 81, 77, 73, 69, 65, 142, 251, 170, 166, 162, 158, 154, 150, 146, 255,
];

const TURN_RET_8: [u8; 512] = [
    255, 251, 247, 243, 239, 235, 231, 227, 223, 219, 215, 211, 207, 203, 199, 195, 191, 186, 182,
    178, 174, 170, 166, 162, 158, 154, 150, 146, 142, 138, 134, 130, 126, 121, 117, 113, 109, 105,
    101, 97, 93, 89, 85, 81, 77, 73, 69, 65, 61, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8,
    4, 0, 227, 231, 235, 239, 243, 247, 251, 255, 195, 199, 203, 207, 211, 215, 219, 223, 162, 166,
    170, 174, 178, 182, 186, 191, 130, 134, 138, 142, 146, 150, 154, 158, 97, 101, 105, 109, 113,
    117, 121, 126, 65, 69, 73, 77, 81, 85, 89, 93, 32, 36, 40, 44, 48, 52, 56, 61, 0, 4, 8, 12, 16,
    20, 24, 28, 28, 24, 20, 16, 12, 8, 4, 0, 61, 56, 52, 48, 44, 40, 36, 32, 93, 89, 85, 81, 77,
    73, 69, 65, 126, 121, 117, 113, 109, 105, 101, 97, 158, 154, 150, 146, 142, 138, 134, 130, 191,
    186, 182, 178, 174, 170, 166, 162, 223, 219, 215, 211, 207, 203, 199, 195, 255, 251, 247, 243,
    239, 235, 231, 227, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 61, 65, 69, 73,
    77, 81, 85, 89, 93, 97, 101, 105, 109, 113, 117, 121, 126, 130, 134, 138, 142, 146, 150, 154,
    158, 162, 166, 170, 174, 178, 182, 186, 191, 195, 199, 203, 207, 211, 215, 219, 223, 227, 231,
    235, 239, 243, 247, 251, 255, 255, 223, 191, 158, 126, 93, 61, 28, 251, 219, 186, 154, 121, 89,
    56, 24, 247, 215, 182, 150, 117, 85, 52, 20, 243, 211, 178, 146, 113, 81, 48, 16, 239, 207,
    174, 142, 109, 77, 44, 12, 235, 203, 170, 138, 105, 73, 40, 8, 231, 199, 166, 134, 101, 69, 36,
    4, 227, 195, 162, 130, 97, 65, 32, 0, 227, 195, 162, 130, 97, 65, 32, 0, 231, 199, 166, 134,
    101, 69, 36, 4, 235, 203, 170, 138, 105, 73, 40, 8, 239, 207, 174, 142, 109, 77, 44, 12, 243,
    211, 178, 146, 113, 81, 48, 16, 247, 215, 182, 150, 117, 85, 52, 20, 251, 219, 186, 154, 121,
    89, 56, 24, 255, 223, 191, 158, 126, 93, 61, 28, 28, 61, 93, 126, 158, 191, 223, 255, 24, 56,
    89, 121, 154, 186, 219, 251, 20, 52, 85, 117, 150, 182, 215, 247, 16, 48, 81, 113, 146, 178,
    211, 243, 12, 44, 77, 109, 142, 174, 207, 239, 8, 40, 73, 105, 138, 170, 203, 235, 4, 36, 69,
    101, 134, 166, 199, 231, 0, 32, 65, 97, 130, 162, 195, 227, 0, 32, 65, 97, 130, 162, 195, 227,
    4, 36, 69, 101, 134, 166, 199, 231, 8, 40, 73, 105, 138, 170, 203, 235, 12, 44, 77, 109, 142,
    174, 207, 239, 16, 48, 81, 113, 146, 178, 211, 243, 20, 52, 85, 117, 150, 182, 215, 247, 24,
    56, 89, 121, 154, 186, 219, 251, 28, 61, 93, 126, 158, 191, 223, 255,
];

const TURN_DOWN_8: [u8; 512] = [
    255, 251, 247, 243, 239, 235, 231, 227, 195, 199, 203, 207, 211, 215, 219, 223, 191, 186, 182,
    178, 174, 170, 166, 162, 130, 134, 138, 142, 146, 150, 154, 158, 126, 121, 117, 113, 109, 105,
    101, 97, 65, 69, 73, 77, 81, 85, 89, 93, 61, 56, 52, 48, 44, 40, 36, 32, 0, 4, 8, 12, 16, 20,
    24, 28, 227, 231, 235, 239, 243, 247, 251, 255, 223, 219, 215, 211, 207, 203, 199, 195, 162,
    166, 170, 174, 178, 182, 186, 191, 158, 154, 150, 146, 142, 138, 134, 130, 97, 101, 105, 109,
    113, 117, 121, 126, 93, 89, 85, 81, 77, 73, 69, 65, 32, 36, 40, 44, 48, 52, 56, 61, 28, 24, 20,
    16, 12, 8, 4, 0, 0, 4, 8, 12, 16, 20, 24, 28, 61, 56, 52, 48, 44, 40, 36, 32, 65, 69, 73, 77,
    81, 85, 89, 93, 126, 121, 117, 113, 109, 105, 101, 97, 130, 134, 138, 142, 146, 150, 154, 158,
    191, 186, 182, 178, 174, 170, 166, 162, 195, 199, 203, 207, 211, 215, 219, 223, 255, 251, 247,
    243, 239, 235, 231, 227, 28, 24, 20, 16, 12, 8, 4, 0, 32, 36, 40, 44, 48, 52, 56, 61, 93, 89,
    85, 81, 77, 73, 69, 65, 97, 101, 105, 109, 113, 117, 121, 126, 158, 154, 150, 146, 142, 138,
    134, 130, 162, 166, 170, 174, 178, 182, 186, 191, 223, 219, 215, 211, 207, 203, 199, 195, 227,
    231, 235, 239, 243, 247, 251, 255, 255, 195, 191, 130, 126, 65, 61, 0, 251, 199, 186, 134, 121,
    69, 56, 4, 247, 203, 182, 138, 117, 73, 52, 8, 243, 207, 178, 142, 113, 77, 48, 12, 239, 211,
    174, 146, 109, 81, 44, 16, 235, 215, 170, 150, 105, 85, 40, 20, 231, 219, 166, 154, 101, 89,
    36, 24, 227, 223, 162, 158, 97, 93, 32, 28, 227, 223, 162, 158, 97, 93, 32, 28, 231, 219, 166,
    154, 101, 89, 36, 24, 235, 215, 170, 150, 105, 85, 40, 20, 239, 211, 174, 146, 109, 81, 44, 16,
    243, 207, 178, 142, 113, 77, 48, 12, 247, 203, 182, 138, 117, 73, 52, 8, 251, 199, 186, 134,
    121, 69, 56, 4, 255, 195, 191, 130, 126, 65, 61, 0, 0, 61, 65, 126, 130, 191, 195, 255, 4, 56,
    69, 121, 134, 186, 199, 251, 8, 52, 73, 117, 138, 182, 203, 247, 12, 48, 77, 113, 142, 178,
    207, 243, 16, 44, 81, 109, 146, 174, 211, 239, 20, 40, 85, 105, 150, 170, 215, 235, 24, 36, 89,
    101, 154, 166, 219, 231, 28, 32, 93, 97, 158, 162, 223, 227, 28, 32, 93, 97, 158, 162, 223,
    227, 24, 36, 89, 101, 154, 166, 219, 231, 20, 40, 85, 105, 150, 170, 215, 235, 16, 44, 81, 109,
    146, 174, 211, 239, 12, 48, 77, 113, 142, 178, 207, 243, 8, 52, 73, 117, 138, 182, 203, 247, 4,
    56, 69, 121, 134, 186, 199, 251, 0, 61, 65, 126, 130, 191, 195, 255,
];
