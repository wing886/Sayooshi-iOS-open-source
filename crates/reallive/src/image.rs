//! Bitmaps: RealLive `g00` (types 0, 1 and 2), AVG32 `pdt`, and common
//! formats as a fallback for patched games.
//!
//! `g00` layout (little-endian):
//!
//! * type 0: `type:u8 w:u16 h:u16 packed:u32 unpacked:u32` then LZ data
//!   whose literals are 3-byte BGR pixels;
//! * type 1: same header; LZ bytes holding `count:u16`, `count` BGRA
//!   palette entries and one index per pixel;
//! * type 2: `type:u8 w:u16 h:u16 regions:u32` + 24 bytes per region
//!   (`x1 y1 x2 y2 origin_x origin_y`), then `packed:u32 unpacked:u32` and
//!   LZ bytes with, per region, blocks of BGRA pixels. Regions are the
//!   "patterns" objects select with `objPattNo`.
//!
//! Both LZ variants read flag bits least-significant first; a clear bit
//! is a back reference `d:u16` copying `(d & 15) + k` units from `d >> 4`
//! units back.

use anyhow::{Result, anyhow, bail};

use crate::surface::Surface;

/// A pattern: a sub-rectangle of the bitmap with its own origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub origin_x: i32,
    pub origin_y: i32,
}

impl Region {
    pub fn full(width: i32, height: i32) -> Self {
        Self {
            x1: 0,
            y1: 0,
            x2: width - 1,
            y2: height - 1,
            origin_x: 0,
            origin_y: 0,
        }
    }

    pub fn width(&self) -> i32 {
        (self.x2 - self.x1 + 1).max(0)
    }

    pub fn height(&self) -> i32 {
        (self.y2 - self.y1 + 1).max(0)
    }
}

/// A decoded bitmap in straight (non-premultiplied) RGBA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub surface: Surface,
    pub regions: Vec<Region>,
    /// The file carried an alpha channel.
    pub has_alpha: bool,
}

impl Image {
    /// A transparent image.
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            surface: Surface {
                width,
                height,
                rgba: vec![0; (width.max(0) * height.max(0) * 4) as usize],
            },
            regions: vec![Region::full(width, height)],
            has_alpha: false,
        }
    }

    pub fn from_surface(surface: Surface) -> Self {
        let regions = vec![Region::full(surface.width, surface.height)];
        Self {
            surface,
            regions,
            has_alpha: true,
        }
    }

    pub fn width(&self) -> i32 {
        self.surface.width
    }

    pub fn height(&self) -> i32 {
        self.surface.height
    }

    pub fn region(&self, pattern: i32) -> Region {
        usize::try_from(pattern)
            .ok()
            .and_then(|index| self.regions.get(index))
            .copied()
            .unwrap_or_else(|| Region::full(self.width(), self.height()))
    }
}

/// Decodes any supported bitmap.
pub fn decode(bytes: &[u8]) -> Result<Image> {
    if bytes.starts_with(b"PDT10") || bytes.starts_with(b"PDT11") {
        return decode_pdt(bytes);
    }
    if bytes.first().is_some_and(|&t| t <= 2) && bytes.len() >= 9 {
        if let Ok(image) = decode_g00(bytes) {
            return Ok(image);
        }
    }
    decode_common(bytes)
}

fn u16_at(data: &[u8], at: usize) -> Result<u16> {
    data.get(at..at + 2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or_else(|| anyhow!("reallive: truncated bitmap"))
}

fn i32_at(data: &[u8], at: usize) -> Result<i32> {
    data.get(at..at + 4)
        .map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or_else(|| anyhow!("reallive: truncated bitmap"))
}

/// The LZ decoder shared by both g00 variants. `unit` is the literal size
/// (3 for type 0, 1 otherwise) and `min` the smallest copy length in
/// units.
fn lz_extract(src: &[u8], unpacked: usize, unit: usize, min: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(unpacked);
    let mut at = 0;
    while at < src.len() && out.len() < unpacked {
        let flag = src[at];
        at += 1;
        for bit in 0..8 {
            if at >= src.len() || out.len() >= unpacked {
                break;
            }
            if flag & (1 << bit) != 0 {
                let end = (at + unit).min(src.len());
                out.extend_from_slice(&src[at..end]);
                at = end;
            } else {
                if at + 2 > src.len() {
                    at = src.len();
                    break;
                }
                let d = usize::from(u16::from_le_bytes([src[at], src[at + 1]]));
                at += 2;
                let (distance, length) = if unit == 3 {
                    ((d >> 4) * 3, ((d & 0x0f) + 1) * 3)
                } else {
                    (d >> 4, (d & 0x0f) + min)
                };
                if distance == 0 || distance > out.len() {
                    // Corrupt reference: stop rather than read garbage.
                    return out;
                }
                let start = out.len() - distance;
                for i in 0..length {
                    let byte = out[start + i];
                    out.push(byte);
                }
            }
        }
    }
    out.truncate(unpacked);
    out
}

pub fn decode_g00(data: &[u8]) -> Result<Image> {
    let kind = data[0];
    let width = i32::from(u16_at(data, 1)?);
    let mut height = i32::from(u16_at(data, 3)?);
    match kind {
        0 | 1 => {
            let unpacked = i32_at(data, 9)?.max(0) as usize;
            let src = data
                .get(13..)
                .ok_or_else(|| anyhow!("reallive: truncated g00"))?;
            let mut image = Image::new(width, height);
            if kind == 0 {
                let bgr = lz_extract(src, unpacked, 3, 1);
                for (pixel, source) in image
                    .surface
                    .rgba
                    .chunks_exact_mut(4)
                    .zip(bgr.chunks_exact(3))
                {
                    pixel.copy_from_slice(&[source[2], source[1], source[0], 255]);
                }
            } else {
                let bytes = lz_extract(src, unpacked + 1, 1, 2);
                let count = usize::from(u16_at(&bytes, 0)?).min(256);
                let palette: Vec<[u8; 4]> = (0..count)
                    .map(|i| {
                        let at = 2 + i * 4;
                        let b = bytes.get(at..at + 4).unwrap_or(&[0, 0, 0, 255]);
                        [b[2], b[1], b[0], b[3]]
                    })
                    .collect();
                let indices = bytes.get(2 + count * 4..).unwrap_or(&[]);
                let mut has_alpha = false;
                for (pixel, &index) in image.surface.rgba.chunks_exact_mut(4).zip(indices) {
                    let colour = palette
                        .get(usize::from(index))
                        .copied()
                        .unwrap_or([0, 0, 0, 255]);
                    has_alpha |= colour[3] != 255;
                    pixel.copy_from_slice(&colour);
                }
                image.has_alpha = has_alpha;
            }
            Ok(image)
        }
        2 => {
            let count = i32_at(data, 5)?.max(0) as usize;
            if count == 0 || 9 + count * 24 > data.len() {
                bail!("reallive: invalid g00 region table");
            }
            let mut regions = Vec::with_capacity(count);
            for i in 0..count {
                let at = 9 + i * 24;
                let mut region = Region {
                    x1: i32_at(data, at)?,
                    y1: i32_at(data, at + 4)?,
                    x2: i32_at(data, at + 8)?,
                    y2: i32_at(data, at + 12)?,
                    origin_x: i32_at(data, at + 16)?,
                    origin_y: i32_at(data, at + 20)?,
                };
                region.x1 = region.x1.clamp(0, width - 1);
                region.x2 = region.x2.clamp(0, width - 1);
                region.y1 = region.y1.clamp(0, height - 1);
                region.y2 = region.y2.clamp(0, height - 1);
                regions.push(region);
            }
            // Newer files stack same-sized regions on top of each other:
            // give each its own band of the canvas.
            let real: Vec<&Region> = regions
                .iter()
                .filter(|r| r.width() > 0 && r.height() > 0)
                .collect();
            let stacked = real.len() > 1 && real.iter().all(|r| *r == real[0]);
            if stacked {
                for (i, region) in regions.iter_mut().enumerate() {
                    region.y1 += i as i32 * height;
                    region.y2 += i as i32 * height;
                }
                height *= count as i32;
            }
            let head = 9 + count * 24;
            let unpacked = i32_at(data, head + 4)?.max(0) as usize;
            let bytes = lz_extract(data.get(head + 8..).unwrap_or(&[]), unpacked, 1, 2);
            let mut image = Image::new(width, height);
            image.has_alpha = true;
            let stored = (i32_at(&bytes, 0).unwrap_or(0).max(0) as usize).min(count);
            for (index, region) in regions.iter().enumerate().take(stored) {
                let offset = i32_at(&bytes, index * 8 + 4)?.max(0) as usize;
                let length = i32_at(&bytes, index * 8 + 8)?.max(0) as usize;
                let mut at = offset + 0x74;
                let end = (offset + length).min(bytes.len());
                while at + 0x5c <= end {
                    let x = i32::from(u16_at(&bytes, at)?) + region.x1;
                    let y = i32::from(u16_at(&bytes, at + 2)?) + region.y1;
                    let w = i32::from(u16_at(&bytes, at + 6)?);
                    let h = i32::from(u16_at(&bytes, at + 8)?);
                    at += 0x5c;
                    for row in 0..h {
                        let dy = y + row;
                        for column in 0..w {
                            let dx = x + column;
                            let source = at + ((row * w + column) * 4) as usize;
                            let Some(px) = bytes.get(source..source + 4) else {
                                break;
                            };
                            if dx < 0 || dy < 0 || dx >= width || dy >= height {
                                continue;
                            }
                            let target = ((dy * width + dx) * 4) as usize;
                            image.surface.rgba[target..target + 4]
                                .copy_from_slice(&[px[2], px[1], px[0], px[3]]);
                        }
                    }
                    at += (w * h * 4) as usize;
                }
            }
            image.regions = regions;
            Ok(image)
        }
        other => bail!("reallive: unsupported g00 type {other}"),
    }
}

fn decode_pdt(bytes: &[u8]) -> Result<Image> {
    let pdt = avg32::decode_pdt(bytes)?;
    let (width, height) = (pdt.width as i32, pdt.height as i32);
    let mut rgba = pdt.rgba;
    // AVG32 decoding premultiplies colour by the mask; undo it.
    for pixel in rgba.chunks_exact_mut(4) {
        let alpha = u32::from(pixel[3]);
        if alpha != 0 && alpha != 255 {
            for channel in &mut pixel[..3] {
                *channel = ((u32::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8;
            }
        }
    }
    Ok(Image {
        surface: Surface {
            width,
            height,
            rgba,
        },
        regions: vec![Region::full(width, height)],
        has_alpha: pdt.has_mask,
    })
}

fn decode_common(bytes: &[u8]) -> Result<Image> {
    let decoded = image::load_from_memory(bytes)
        .map_err(|error| anyhow!("reallive: unsupported bitmap: {error}"))?;
    let has_alpha = decoded.color().has_alpha();
    let rgba = decoded.to_rgba8();
    let (width, height) = (rgba.width() as i32, rgba.height() as i32);
    Ok(Image {
        surface: Surface {
            width,
            height,
            rgba: rgba.into_raw(),
        },
        regions: vec![Region::full(width, height)],
        has_alpha,
    })
}

/// Encodes an image as a type 0 or type 2 g00 (tests and tools); the LZ
/// stream holds literals only.
pub fn encode_g00(image: &Image, with_regions: bool) -> Vec<u8> {
    fn literal_lz(data: &[u8], unit: usize) -> Vec<u8> {
        let mut out = Vec::new();
        for chunk in data.chunks(8 * unit) {
            let units = chunk.len().div_ceil(unit);
            out.push(((1u16 << units) - 1) as u8);
            out.extend_from_slice(chunk);
        }
        out
    }
    let mut out = Vec::new();
    if !with_regions {
        let bgr: Vec<u8> = image
            .surface
            .rgba
            .chunks_exact(4)
            .flat_map(|p| [p[2], p[1], p[0]])
            .collect();
        let lz = literal_lz(&bgr, 3);
        out.push(0);
        out.extend_from_slice(&(image.width() as u16).to_le_bytes());
        out.extend_from_slice(&(image.height() as u16).to_le_bytes());
        out.extend_from_slice(&((lz.len() + 8) as i32).to_le_bytes());
        out.extend_from_slice(&(bgr.len() as i32).to_le_bytes());
        out.extend(lz);
        return out;
    }
    let mut body = Vec::new();
    body.extend_from_slice(&(image.regions.len() as i32).to_le_bytes());
    let index_len = 4 + image.regions.len() * 8;
    let mut blocks = Vec::new();
    for region in &image.regions {
        let mut block = vec![0u8; 0x74];
        let mut header = vec![0u8; 0x5c];
        header[6..8].copy_from_slice(&(region.width() as u16).to_le_bytes());
        header[8..10].copy_from_slice(&(region.height() as u16).to_le_bytes());
        block.extend(header);
        for y in region.y1..=region.y2 {
            for x in region.x1..=region.x2 {
                let at = ((y * image.width() + x) * 4) as usize;
                let p = &image.surface.rgba[at..at + 4];
                block.extend_from_slice(&[p[2], p[1], p[0], p[3]]);
            }
        }
        blocks.push(block);
    }
    let mut offset = index_len;
    for block in &blocks {
        body.extend_from_slice(&(offset as i32).to_le_bytes());
        body.extend_from_slice(&(block.len() as i32).to_le_bytes());
        offset += block.len();
    }
    for block in blocks {
        body.extend(block);
    }
    let lz = literal_lz(&body, 1);
    out.push(2);
    out.extend_from_slice(&(image.width() as u16).to_le_bytes());
    out.extend_from_slice(&(image.height() as u16).to_le_bytes());
    out.extend_from_slice(&(image.regions.len() as i32).to_le_bytes());
    for region in &image.regions {
        for value in [
            region.x1,
            region.y1,
            region.x2,
            region.y2,
            region.origin_x,
            region.origin_y,
        ] {
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
    out.extend_from_slice(&((lz.len() + 8) as i32).to_le_bytes());
    out.extend_from_slice(&(body.len() as i32).to_le_bytes());
    out.extend(lz);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Image {
        let mut image = Image::new(4, 2);
        for (i, pixel) in image.surface.rgba.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&[i as u8 * 10, 20, 30, 255 - i as u8]);
        }
        image
    }

    #[test]
    fn type0_round_trips_colour() {
        let image = sample();
        let decoded = decode(&encode_g00(&image, false)).unwrap();
        for (a, b) in decoded
            .surface
            .rgba
            .chunks_exact(4)
            .zip(image.surface.rgba.chunks_exact(4))
        {
            assert_eq!(&a[..3], &b[..3]);
            assert_eq!(a[3], 255);
        }
    }

    #[test]
    fn type2_round_trips_regions_and_alpha() {
        let mut image = sample();
        image.regions = vec![
            Region {
                x1: 0,
                y1: 0,
                x2: 1,
                y2: 1,
                origin_x: 1,
                origin_y: 2,
            },
            Region {
                x1: 2,
                y1: 0,
                x2: 3,
                y2: 1,
                origin_x: 0,
                origin_y: 0,
            },
        ];
        let decoded = decode(&encode_g00(&image, true)).unwrap();
        assert_eq!(decoded.regions, image.regions);
        assert_eq!(decoded.surface.rgba, image.surface.rgba);
    }

    #[test]
    fn lz_back_references_repeat_units() {
        // flag: literal then reference (distance 1 unit, length 2 units).
        let mut src = vec![0b01, 1, 2, 3];
        let d: u16 = (1 << 4) | 1;
        src.extend_from_slice(&d.to_le_bytes());
        assert_eq!(lz_extract(&src, 9, 3, 1), vec![1, 2, 3, 1, 2, 3, 1, 2, 3]);
    }
}
