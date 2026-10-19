//! AVG32 PDT buffers.
//!
//! AVG32 keeps every numbered graphics buffer as packed RGB plus a separate
//! 8-bit mask plane.  Colour data coming out of masked PDT files is already
//! premultiplied by that mask, so compositing is `dst = src + dst * (256 -
//! mask) / 256` rather than a
//! straight-alpha blend.

use crate::pdt::PdtImage;

pub const AVG32_WIDTH: u32 = 640;
pub const AVG32_HEIGHT: u32 = 480;
pub const SCREEN_W: i32 = AVG32_WIDTH as i32;
pub const SCREEN_H: i32 = AVG32_HEIGHT as i32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdtBuffer {
    pub width: usize,
    pub height: usize,
    /// Packed `R, G, B` triples, row-major.
    pub rgb: Vec<u8>,
    /// One mask byte per pixel; `255` is opaque.
    pub mask: Vec<u8>,
}

impl PdtBuffer {
    /// A freshly allocated buffer is black with a fully opaque mask, exactly
    /// like a newly created AVG32 buffer.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rgb: vec![0; width * height * 3],
            mask: vec![255; width * height],
        }
    }

    pub fn screen() -> Self {
        Self::new(AVG32_WIDTH as usize, AVG32_HEIGHT as usize)
    }

    pub fn from_image(image: &PdtImage) -> Self {
        let width = image.width as usize;
        let height = image.height as usize;
        let mut buffer = Self::new(width, height);
        for (index, pixel) in image.rgba.chunks_exact(4).enumerate() {
            buffer.rgb[index * 3..index * 3 + 3].copy_from_slice(&pixel[..3]);
            buffer.mask[index] = pixel[3];
        }
        buffer
    }

    #[inline]
    pub fn at(&self, x: i32, y: i32) -> usize {
        y as usize * self.width + x as usize
    }

    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    #[inline]
    pub fn pixel(&self, index: usize) -> [u8; 3] {
        [
            self.rgb[index * 3],
            self.rgb[index * 3 + 1],
            self.rgb[index * 3 + 2],
        ]
    }

    #[inline]
    pub fn set_pixel(&mut self, index: usize, colour: [u8; 3]) {
        self.rgb[index * 3..index * 3 + 3].copy_from_slice(&colour);
    }

    /// Copies an image's pixels into the top-left of this buffer, clipped
    /// to both sizes; everything outside the image is left untouched.
    pub fn copy_image(&mut self, image: &PdtImage) {
        let has_mask = image.has_mask;
        let width = (image.width as usize).min(self.width);
        let height = (image.height as usize).min(self.height);
        for y in 0..height {
            for x in 0..width {
                let source = (y * image.width as usize + x) * 4;
                let target = y * self.width + x;
                self.rgb[target * 3..target * 3 + 3]
                    .copy_from_slice(&image.rgba[source..source + 3]);
                self.mask[target] = if has_mask {
                    image.rgba[source + 3]
                } else {
                    255
                };
            }
        }
    }

    /// Converts the colour plane to opaque RGBA8 for presentation.
    pub fn to_rgba(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.width * self.height * 4);
        for pixel in self.rgb.chunks_exact(3) {
            out.extend_from_slice(&[pixel[0], pixel[1], pixel[2], 255]);
        }
        out
    }
}

/// Composites a premultiplied source pixel through `mask`.
#[inline]
pub fn blend_pixel(source: [u8; 3], destination: &mut [u8], mask: u8) {
    if mask == 0 {
        return;
    }
    let inverse = 256 - u32::from(mask);
    for channel in 0..3 {
        let value = u32::from(source[channel]) + ((inverse * u32::from(destination[channel])) >> 8);
        destination[channel] = value.min(255) as u8;
    }
}

/// Composites through `mask`, then cross-fades the result
/// against the previous destination by `fade / 256`.
#[inline]
pub fn fade_pixel(source: [u8; 3], under: [u8; 3], destination: &mut [u8], mask: u32, fade: u32) {
    if mask == 0 {
        return;
    }
    let inverse = 256u32.saturating_sub(mask);
    for channel in 0..3 {
        let composed =
            (u32::from(source[channel]) + ((inverse * u32::from(under[channel])) >> 8)).min(255);
        let value = (composed * fade + (256 - fade.min(256)) * u32::from(under[channel])) >> 8;
        destination[channel] = value.min(255) as u8;
    }
}

/// Composites the *destination* over the source ("draw
/// underneath"), used by `copy_with_mask`.
#[inline]
pub fn blend_under(source: [u8; 3], destination: &mut [u8], destination_mask: u8) {
    if destination_mask == 0 {
        destination[..3].copy_from_slice(&source);
        return;
    }
    let inverse = 256 - u32::from(destination_mask);
    for channel in 0..3 {
        let value = u32::from(destination[channel]) + ((inverse * u32::from(source[channel])) >> 8);
        destination[channel] = value.min(255) as u8;
    }
}

/// Fading variant of [`blend_under`].
#[inline]
pub fn fade_under(source: [u8; 3], destination: &mut [u8], destination_mask: u8, fade: u32) {
    if destination_mask == 0 {
        destination[..3].copy_from_slice(&source);
        return;
    }
    let inverse = 256 - u32::from(destination_mask);
    for channel in 0..3 {
        let composed = (u32::from(destination[channel])
            + ((inverse * u32::from(source[channel])) >> 8))
            .min(255);
        let value = (composed * fade + (256 - fade.min(256)) * u32::from(source[channel])) >> 8;
        destination[channel] = value.min(255) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn premultiplied_blend_matches_reference_formula() {
        let mut destination = [200, 100, 50];
        blend_pixel([64, 64, 64], &mut destination, 128);
        assert_eq!(destination, [164, 114, 89]);
        let mut untouched = [1, 2, 3];
        blend_pixel([255, 255, 255], &mut untouched, 0);
        assert_eq!(untouched, [1, 2, 3]);
    }
}
