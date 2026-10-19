//! Decoder for the PC-98 `PDT34` planar image format.
//!
//! The format is read from UK2's `sub_226B7` / `sub_225BE`: a 32-byte palette
//! block follows the tag, then two RLE marker bytes, an 8-byte rectangle, and
//! four separately encoded bitplanes. The rectangle is stored in 8-pixel
//! columns and pairs of scanlines, matching the PC-98 640x400 graphics planes.

use anyhow::{Context, Result, bail};

pub const PDT34_WIDTH: u32 = 640;
pub const PDT34_HEIGHT: u32 = 400;
pub const PDT34_PALETTE_OFFSET: usize = 1;
pub const PDT34_MARKERS_OFFSET: usize = 0x21;
pub const PDT34_RECT_OFFSET: usize = 0x23;
pub const PDT34_DATA_OFFSET: usize = 0x2b;
const PLANE_ROW_BYTES: usize = (PDT34_WIDTH as usize) / 8;
const PLANE_ROWS: usize = PDT34_HEIGHT as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pdt34Rect {
    /// Left edge in 8-pixel byte columns.
    pub left: u16,
    /// Top edge in scanlines.
    pub top: u16,
    /// Right edge in 8-pixel byte columns, inclusive.
    pub right: u16,
    /// Bottom edge in scanlines, inclusive.
    pub bottom: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pdt34Header {
    pub palette: [u16; 16],
    pub marker_alternating: u8,
    pub marker_solid: u8,
    pub rect: Pdt34Rect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pdt34Image {
    pub width: u32,
    pub height: u32,
    /// Packed 4bpp palette indices, two pixels per byte, row-major.
    pub indexed: Vec<u8>,
    /// Row-major RGBA image. MAP compositing applies transparency, since
    /// standalone PDT loads may use palette index zero as an opaque color.
    pub rgba: Vec<u8>,
}

impl Pdt34Header {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < PDT34_DATA_OFFSET {
            bail!("uk2: PDT34 is smaller than its {PDT34_DATA_OFFSET:#x}-byte header");
        }
        if bytes[0] != 0x34 {
            bail!("uk2: expected PDT34 tag 0x34, got {:#04x}", bytes[0]);
        }
        let mut palette = [0u16; 16];
        for (index, color) in palette.iter_mut().enumerate() {
            let at = PDT34_PALETTE_OFFSET + index * 2;
            *color = u16::from_le_bytes([bytes[at], bytes[at + 1]]);
            if *color > 0x0fff {
                bail!("uk2: PDT34 palette entry {index} exceeds 12 bits");
            }
        }
        let rect = Pdt34Rect {
            left: u16::from_le_bytes([bytes[PDT34_RECT_OFFSET], bytes[PDT34_RECT_OFFSET + 1]]),
            top: u16::from_le_bytes([bytes[PDT34_RECT_OFFSET + 2], bytes[PDT34_RECT_OFFSET + 3]]),
            right: u16::from_le_bytes([bytes[PDT34_RECT_OFFSET + 4], bytes[PDT34_RECT_OFFSET + 5]]),
            bottom: u16::from_le_bytes([
                bytes[PDT34_RECT_OFFSET + 6],
                bytes[PDT34_RECT_OFFSET + 7],
            ]),
        };
        validate_rect(rect)?;
        Ok(Self {
            palette,
            marker_alternating: bytes[PDT34_MARKERS_OFFSET],
            marker_solid: bytes[PDT34_MARKERS_OFFSET + 1],
            rect,
        })
    }

    pub fn payload<'a>(&self, bytes: &'a [u8]) -> Result<&'a [u8]> {
        bytes
            .get(PDT34_DATA_OFFSET..)
            .ok_or_else(|| anyhow::anyhow!("uk2: truncated PDT34 payload"))
    }

    /// Returns the RGB 4-bit channels of a palette word.  PDT34 stores the
    /// PC-98 analog palette order `0xGRB`, which `UK2.EXE` writes to the
    /// hardware unchanged.
    pub fn palette_nibbles(&self, index: usize) -> Option<[u8; 3]> {
        let value = *self.palette.get(index)?;
        Some([
            ((value >> 4) & 0x000f) as u8,
            ((value >> 8) & 0x000f) as u8,
            (value & 0x000f) as u8,
        ])
    }

    pub fn decode_image(&self, bytes: &[u8]) -> Result<Pdt34Image> {
        let payload = self.payload(bytes)?;
        let mut cursor = 0usize;
        let mut planes = vec![vec![0u8; PLANE_ROW_BYTES * PLANE_ROWS]; 4];
        for (plane_index, plane) in planes.iter_mut().enumerate() {
            decode_plane(self, payload, &mut cursor, plane)
                .with_context(|| format!("uk2: PDT34 plane {plane_index} decode failed"))?;
        }

        let mut indexed = vec![0u8; PDT34_WIDTH as usize * PDT34_HEIGHT as usize / 2];
        for y in 0..PLANE_ROWS {
            for x_byte in 0..PLANE_ROW_BYTES {
                let plane_bytes = [
                    planes[0][y * PLANE_ROW_BYTES + x_byte],
                    planes[1][y * PLANE_ROW_BYTES + x_byte],
                    planes[2][y * PLANE_ROW_BYTES + x_byte],
                    planes[3][y * PLANE_ROW_BYTES + x_byte],
                ];
                for bit in 0..8 {
                    let mut colour = 0u8;
                    for (plane, byte) in plane_bytes.iter().enumerate() {
                        colour |= ((byte >> (7 - bit)) & 1) << plane;
                    }
                    let pixel = y * PDT34_WIDTH as usize + x_byte * 8 + bit;
                    let packed = &mut indexed[pixel / 2];
                    if pixel & 1 == 0 {
                        *packed = colour << 4;
                    } else {
                        *packed |= colour;
                    }
                }
            }
        }
        let rgba = unpack_indexed_to_rgba(&indexed, &self.palette);
        Ok(Pdt34Image {
            width: PDT34_WIDTH,
            height: PDT34_HEIGHT,
            indexed,
            rgba,
        })
    }
}

pub fn decode_pdt34(bytes: &[u8]) -> Result<Pdt34Image> {
    let header = Pdt34Header::parse(bytes)?;
    header.decode_image(bytes)
}

/// Composites a PDT34 source rectangle as a transparent sprite. UK2's
/// renderer stores the rectangle's horizontal coordinates in 8-pixel byte
/// columns; `1000` preserves the source rectangle coordinate on that axis.
pub fn composite_pdt34_sprite(
    destination: &mut Pdt34Image,
    source: &Pdt34Image,
    rect: Pdt34Rect,
    x: u16,
    y: u16,
) -> Result<()> {
    validate_rect(rect)?;
    if destination.width != PDT34_WIDTH
        || destination.height != PDT34_HEIGHT
        || source.width != PDT34_WIDTH
        || source.height != PDT34_HEIGHT
    {
        bail!("uk2: PDT34 sprite composition requires 640x400 images");
    }
    let width = (usize::from(rect.right - rect.left) + 1) * 8;
    let height = usize::from(rect.bottom - rect.top) + 1;
    let source_x = usize::from(rect.left) * 8;
    let source_y = usize::from(rect.top);
    let target_x = if x == 1000 {
        source_x
    } else {
        usize::from(x) * 8
    };
    let target_y = if y == 1000 { source_y } else { usize::from(y) };
    let max_x = (target_x + width).min(destination.width as usize);
    let max_y = (target_y + height).min(destination.height as usize);
    let source_width = source.width as usize;
    let destination_width = destination.width as usize;
    for dst_y in target_y.min(max_y)..max_y {
        let src_y = source_y + dst_y - target_y;
        for dst_x in target_x.min(max_x)..max_x {
            let src_x = source_x + dst_x - target_x;
            let source_pixel = src_y * source_width + src_x;
            let packed = source.indexed[source_pixel / 2];
            let palette_index = if source_pixel & 1 == 0 {
                packed >> 4
            } else {
                packed & 0x0f
            };
            if palette_index == 0 {
                continue;
            }
            let source_rgba = source_pixel * 4;
            let destination_rgba = (dst_y * destination_width + dst_x) * 4;
            destination.rgba[destination_rgba..destination_rgba + 4]
                .copy_from_slice(&source.rgba[source_rgba..source_rgba + 4]);
            let destination_pixel = dst_y * destination_width + dst_x;
            let destination_packed = &mut destination.indexed[destination_pixel / 2];
            if destination_pixel & 1 == 0 {
                *destination_packed = (*destination_packed & 0x0f) | (palette_index << 4);
            } else {
                *destination_packed = (*destination_packed & 0xf0) | palette_index;
            }
        }
    }
    Ok(())
}

fn validate_rect(rect: Pdt34Rect) -> Result<()> {
    if rect.right < rect.left {
        bail!("uk2: PDT34 rectangle has right edge before left edge");
    }
    if rect.bottom < rect.top {
        bail!("uk2: PDT34 rectangle has bottom edge before top edge");
    }
    if rect.right as usize >= PLANE_ROW_BYTES {
        bail!("uk2: PDT34 rectangle exceeds the 80-byte screen row");
    }
    if rect.bottom as usize >= PLANE_ROWS {
        bail!("uk2: PDT34 rectangle exceeds the 400-line screen");
    }
    if rect.top & 1 != 0 || rect.bottom & 1 != 1 {
        bail!("uk2: PDT34 rectangle must start on an even line and end on an odd line");
    }
    Ok(())
}

fn decode_plane(
    header: &Pdt34Header,
    input: &[u8],
    cursor: &mut usize,
    plane: &mut [u8],
) -> Result<()> {
    let width = usize::from(header.rect.right - header.rect.left) + 1;
    let row_pairs = usize::from((header.rect.bottom - header.rect.top) / 2) + 1;
    let mut column = 0usize;
    while column < width {
        let x = usize::from(header.rect.left) + column;
        let mut row_pair = 0usize;
        while row_pair < row_pairs {
            let (repeat, top, bottom) = match read_byte(input, cursor)? {
                marker if marker == header.marker_alternating => {
                    let repeat = usize::from(read_byte(input, cursor)?);
                    let top = read_byte(input, cursor)?;
                    let bottom = read_byte(input, cursor)?;
                    (repeat, top, bottom)
                }
                marker if marker == header.marker_solid => {
                    let repeat = usize::from(read_byte(input, cursor)?);
                    let colour = read_byte(input, cursor)?;
                    (repeat, colour, colour)
                }
                first => (1, first, read_byte(input, cursor)?),
            };
            if repeat == 0 || row_pair + repeat > row_pairs {
                bail!("PDT34 run length {repeat} exceeds remaining rectangle rows");
            }
            for offset in 0..repeat {
                let line = usize::from(header.rect.top) + (row_pair + offset) * 2;
                plane[line * PLANE_ROW_BYTES + x] = top;
                plane[(line + 1) * PLANE_ROW_BYTES + x] = bottom;
            }
            row_pair += repeat;
        }
        column += 1;
    }
    Ok(())
}

fn read_byte(input: &[u8], cursor: &mut usize) -> Result<u8> {
    let byte = *input
        .get(*cursor)
        .ok_or_else(|| anyhow::anyhow!("truncated PDT34 plane stream"))?;
    *cursor += 1;
    Ok(byte)
}

fn unpack_indexed_to_rgba(indexed: &[u8], palette: &[u16; 16]) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(indexed.len() * 2 * 4);
    for &byte in indexed {
        for nibble in [byte >> 4, byte & 0x0f] {
            let colour = palette[usize::from(nibble)];
            let red = (((colour >> 4) & 0x000f) * 0x11) as u8;
            let green = (((colour >> 8) & 0x000f) * 0x11) as u8;
            let blue = ((colour & 0x000f) * 0x11) as u8;
            rgba.extend_from_slice(&[red, green, blue, 255]);
        }
    }
    rgba
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_header(rect: Pdt34Rect, marker_alternating: u8, marker_solid: u8) -> Vec<u8> {
        let mut bytes = vec![0; PDT34_DATA_OFFSET];
        bytes[0] = 0x34;
        for index in 0..16u16 {
            bytes[1 + usize::from(index) * 2..3 + usize::from(index) * 2]
                .copy_from_slice(&(index * 0x111).to_le_bytes());
        }
        bytes[PDT34_MARKERS_OFFSET] = marker_alternating;
        bytes[PDT34_MARKERS_OFFSET + 1] = marker_solid;
        bytes[PDT34_RECT_OFFSET..PDT34_RECT_OFFSET + 2].copy_from_slice(&rect.left.to_le_bytes());
        bytes[PDT34_RECT_OFFSET + 2..PDT34_RECT_OFFSET + 4]
            .copy_from_slice(&rect.top.to_le_bytes());
        bytes[PDT34_RECT_OFFSET + 4..PDT34_RECT_OFFSET + 6]
            .copy_from_slice(&rect.right.to_le_bytes());
        bytes[PDT34_RECT_OFFSET + 6..PDT34_RECT_OFFSET + 8]
            .copy_from_slice(&rect.bottom.to_le_bytes());
        bytes
    }

    #[test]
    fn parses_palette_markers_and_rect() {
        let rect = Pdt34Rect {
            left: 2,
            top: 4,
            right: 7,
            bottom: 11,
        };
        let bytes = make_header(rect, 0xf0, 0xf1);
        let header = Pdt34Header::parse(&bytes).unwrap();
        assert_eq!(header.palette[15], 0x0fff);
        assert_eq!(header.palette_nibbles(1), Some([1, 1, 1]));
        assert_eq!(header.marker_alternating, 0xf0);
        assert_eq!(header.marker_solid, 0xf1);
        assert_eq!(header.rect, rect);
    }

    #[test]
    fn expands_palette_words_in_grb_channel_order() {
        let indexed = [0x12, 0x30];
        let palette = [
            0, 0x00f0, 0x0f00, 0x000f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let rgba = unpack_indexed_to_rgba(&indexed, &palette);
        assert_eq!(
            &rgba[..16],
            &[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 0, 0, 0, 255,]
        );
    }

    #[test]
    fn decodes_literal_column_pairs_in_four_planes() {
        let rect = Pdt34Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 1,
        };
        let mut bytes = make_header(rect, 0xf0, 0xf1);
        // Four one-column planes, each holding one literal pair of scanlines.
        bytes.extend_from_slice(&[0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00]);
        let image = decode_pdt34(&bytes).unwrap();
        assert_eq!((image.width, image.height), (640, 400));
        // Pixel zero has the high bit set in every plane, yielding colour 15.
        assert_eq!(image.indexed[0] >> 4, 15);
        assert_eq!(image.rgba[3], 255);
    }

    #[test]
    fn composites_nonzero_sprite_pixels_at_pc98_coordinates() {
        let mut destination = Pdt34Image {
            width: PDT34_WIDTH,
            height: PDT34_HEIGHT,
            indexed: vec![0; (PDT34_WIDTH * PDT34_HEIGHT / 2) as usize],
            rgba: vec![7; (PDT34_WIDTH * PDT34_HEIGHT * 4) as usize],
        };
        let mut source = Pdt34Image {
            width: PDT34_WIDTH,
            height: PDT34_HEIGHT,
            indexed: vec![0; (PDT34_WIDTH * PDT34_HEIGHT / 2) as usize],
            rgba: vec![0; (PDT34_WIDTH * PDT34_HEIGHT * 4) as usize],
        };
        source.indexed[0] = 0x01;
        source.rgba[4..8].copy_from_slice(&[21, 31, 41, 255]);
        composite_pdt34_sprite(
            &mut destination,
            &source,
            Pdt34Rect {
                left: 0,
                top: 0,
                right: 0,
                bottom: 1,
            },
            1,
            0,
        )
        .unwrap();
        let transparent = 8 * 4;
        let drawn = 9 * 4;
        assert_eq!(&destination.rgba[transparent..transparent + 4], &[7; 4]);
        assert_eq!(&destination.rgba[drawn..drawn + 4], &[21, 31, 41, 255]);
        assert_eq!(destination.indexed[4] & 0x0f, 1);
    }
}
