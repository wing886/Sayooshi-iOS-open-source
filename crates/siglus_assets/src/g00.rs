use anyhow::{Result, anyhow, bail};
use image::ImageFormat;

use crate::lzss::{lzss_unpack, lzss_unpack32};

/// Fixed 256-byte XOR table used by Siglus G00 type 3 payloads.
///
/// Type 3 stores a JPEG stream after the 5-byte G00 header, with every
/// payload byte XORed against this table cyclically from index 0.
pub const G00_TYPE3_XOR_TABLE: [u8; 256] = [
    0x45, 0x0C, 0x85, 0xC0, 0x75, 0x14, 0xE5, 0x5D, 0x8B, 0x55, 0xEC, 0xC0, 0x5B, 0x8B, 0xC3, 0x8B,
    0x81, 0xFF, 0x00, 0x00, 0x04, 0x00, 0x85, 0xFF, 0x6A, 0x00, 0x76, 0xB0, 0x43, 0x00, 0x76, 0x49,
    0x00, 0x8B, 0x7D, 0xE8, 0x8B, 0x75, 0xA1, 0xE0, 0x0C, 0x85, 0xC0, 0xC0, 0x75, 0x78, 0x30, 0x44,
    0x00, 0x85, 0xFF, 0x76, 0x37, 0x81, 0x1D, 0xD0, 0xFF, 0x00, 0x00, 0x75, 0x44, 0x8B, 0xB0, 0x43,
    0x45, 0xF8, 0x8D, 0x55, 0xFC, 0x52, 0x00, 0x76, 0x68, 0x00, 0x00, 0x04, 0x00, 0x6A, 0x43, 0x8B,
    0xB1, 0x43, 0x00, 0x6A, 0x05, 0xFF, 0x50, 0xFF, 0xD3, 0xA1, 0xE0, 0x04, 0x00, 0x56, 0x15, 0x2C,
    0x44, 0x00, 0x85, 0xC0, 0x74, 0x09, 0xC3, 0xA1, 0x5F, 0x5E, 0x33, 0x8B, 0xE5, 0x5D, 0xE0, 0x30,
    0x04, 0x00, 0x81, 0xC6, 0x00, 0x00, 0x81, 0xEF, 0x04, 0x00, 0x85, 0x30, 0x44, 0x00, 0x00, 0x00,
    0x5D, 0xC3, 0x8B, 0x55, 0xF8, 0x8D, 0x5E, 0x5B, 0x4D, 0xFC, 0x51, 0xC4, 0x04, 0x5F, 0x8B, 0xE5,
    0x43, 0x00, 0xEB, 0xD8, 0x8B, 0x45, 0xFF, 0x15, 0xE8, 0x83, 0xC0, 0x57, 0x56, 0x52, 0x2C, 0xB1,
    0x01, 0x00, 0x8B, 0x7D, 0xE8, 0x89, 0x00, 0xE8, 0x45, 0xF4, 0x8B, 0x20, 0x50, 0x6A, 0x47, 0x28,
    0x00, 0x50, 0x53, 0xFF, 0x15, 0x34, 0xE4, 0x6A, 0xB1, 0x43, 0x00, 0x0C, 0x8B, 0x45, 0x00, 0x6A,
    0x8B, 0x4D, 0xEC, 0x89, 0x08, 0x8A, 0x85, 0xC0, 0x45, 0xF0, 0x84, 0x8B, 0x45, 0x10, 0x74, 0x05,
    0xF5, 0x28, 0x01, 0x00, 0x83, 0xC4, 0x52, 0x6A, 0x08, 0x89, 0x45, 0x83, 0xC2, 0x20, 0x00, 0xE8,
    0xE8, 0xF4, 0xFB, 0xFF, 0xFF, 0x8B, 0x8B, 0x5D, 0x45, 0x0C, 0x83, 0xC0, 0x74, 0xC5, 0xF8, 0x53,
    0xC4, 0x08, 0x85, 0xC0, 0x75, 0x56, 0x30, 0x44, 0x8B, 0x1D, 0xD0, 0xF0, 0xA1, 0xE0, 0x00, 0x83,
];

/// Apply/remove the G00 type 3 JPEG XOR layer in place.
///
/// XOR is symmetric, so this is used for both decoding and encoding.
pub fn xor_type3_jpeg_in_place(buf: &mut [u8]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte ^= G00_TYPE3_XOR_TABLE[i & 0xFF];
    }
}

/// Return a de-obfuscated JPEG payload from a G00 type 3 payload.
pub fn decode_type3_jpeg_payload(payload: &[u8]) -> Vec<u8> {
    let mut out = payload.to_vec();
    xor_type3_jpeg_in_place(&mut out);
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G00Type {
    Type0,
    Type1,
    Type2,
    Type3,
}

impl G00Type {
    fn from_u8(v: u8) -> Result<Self> {
        match v {
            0 => Ok(Self::Type0),
            1 => Ok(Self::Type1),
            2 => Ok(Self::Type2),
            3 => Ok(Self::Type3),
            _ => bail!("g00: unknown type {}", v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct G00 {
    pub ty: G00Type,
    pub cuts: Vec<G00Cut>,
}

#[derive(Clone, Debug)]
pub struct G00Cut {
    pub width: u32,
    pub height: u32,
    pub center_x: i32,
    pub center_y: i32,
    pub disp_left: i32,
    pub disp_top: i32,
    pub disp_right: i32,
    pub disp_bottom: i32,
    pub chips: Vec<G00Chip>,
}

#[derive(Clone, Debug)]
pub struct G00Chip {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub sprite: bool,
    pub data: G00ChipData,
}

#[derive(Clone, Debug)]
pub enum G00ChipData {
    /// Type0: LZSS32 compressed 32bpp pixels.
    Type0Lzss32(Vec<u8>),
    /// Type1: LZSS compressed palette + indices.
    Type1LzssIndexed(Vec<u8>),
    /// Type2: Raw 32bpp pixels in the cut stream.
    RawBgra(Vec<u8>),
    /// Type3: JPEG bitstream (not decoded here).
    Jpeg(Vec<u8>),
}

struct Cur<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cur<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    fn ensure(&self, n: usize) -> Result<()> {
        if self.pos + n > self.buf.len() {
            bail!(
                "g00: unexpected EOF (need {}, have {})",
                n,
                self.remaining()
            );
        }
        Ok(())
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        self.ensure(n)?;
        let out = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(out)
    }

    fn skip(&mut self, n: usize) -> Result<()> {
        self.ensure(n)?;
        self.pos += n;
        Ok(())
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn read_u16_le(&mut self) -> Result<u16> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn read_u32_le(&mut self) -> Result<u32> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn read_i32_le(&mut self) -> Result<i32> {
        Ok(self.read_u32_le()? as i32)
    }
}

impl G00 {
    /// Parse a full `.g00` file.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cur = Cur::new(bytes);
        let ty = G00Type::from_u8(cur.read_u8()?)?;

        match ty {
            G00Type::Type0 | G00Type::Type1 | G00Type::Type3 => {
                let w = cur.read_u16_le()? as u32;
                let h = cur.read_u16_le()? as u32;
                let rest = cur.take(cur.remaining())?.to_vec();

                let chip_data = match ty {
                    G00Type::Type0 => G00ChipData::Type0Lzss32(rest),
                    G00Type::Type1 => G00ChipData::Type1LzssIndexed(rest),
                    G00Type::Type3 => G00ChipData::Jpeg(rest),
                    _ => unreachable!(),
                };

                let chip = G00Chip {
                    x: 0,
                    y: 0,
                    width: w,
                    height: h,
                    sprite: false,
                    data: chip_data,
                };

                let cut = G00Cut {
                    width: w,
                    height: h,
                    center_x: 0,
                    center_y: 0,
                    disp_left: 0,
                    disp_top: 0,
                    disp_right: w as i32,
                    disp_bottom: h as i32,
                    chips: vec![chip],
                };

                Ok(Self {
                    ty,
                    cuts: vec![cut],
                })
            }
            G00Type::Type2 => {
                // Header:
                // [u16 width][u16 height][i32 cut_cnt][G00_CUT_DATABASE * cut_cnt][lzss payload...]
                let _w = cur.read_u16_le()? as u32;
                let _h = cur.read_u16_le()? as u32;
                let cut_cnt = cur.read_i32_le()?;
                if cut_cnt < 0 {
                    bail!("g00: negative cut_cnt {}", cut_cnt);
                }

                // G00_CUT_DATABASE is 6 * i32.
                let db_bytes = (cut_cnt as usize)
                    .checked_mul(24)
                    .ok_or_else(|| anyhow!("g00: cut database size overflow"))?;
                cur.skip(db_bytes)?;

                let compressed = cur.take(cur.remaining())?;
                let decompressed = lzss_unpack(compressed)?;

                let mut dcur = Cur::new(&decompressed);
                let table_cut_cnt = dcur.read_u32_le()? as usize;

                // Table entries: (offset, size) pairs.
                let mut pairs: Vec<(usize, i32)> = Vec::with_capacity(table_cut_cnt);
                for _ in 0..table_cut_cnt {
                    let off = dcur.read_u32_le()? as usize;
                    let size = dcur.read_u32_le()? as i32;
                    pairs.push((off, size));
                }

                let mut cuts = Vec::new();
                for (off, size) in pairs {
                    if off == 0 || size <= 0 {
                        continue;
                    }
                    let size_u = size as usize;
                    let end = off
                        .checked_add(size_u)
                        .ok_or_else(|| anyhow!("g00: cut slice overflow"))?;
                    if end > decompressed.len() {
                        bail!(
                            "g00: cut slice out of bounds (off={}, size={}, len={})",
                            off,
                            size_u,
                            decompressed.len()
                        );
                    }
                    let cut_data = &decompressed[off..end];
                    cuts.push(parse_type2_cut(cut_data)?);
                }

                if cuts.is_empty() {
                    bail!("g00: type2 has no cuts");
                }

                Ok(Self { ty, cuts })
            }
        }
    }
}

fn parse_type2_cut(cut_data: &[u8]) -> Result<G00Cut> {
    // Cut header layout (MSVC default packing, 32-bit):
    //  u8 type
    //  u8 pad
    //  u16 count
    //  i32 x, y, disp_xl, disp_yl, xc, yc, cut_xl, cut_yl
    //  i32 keep[20]
    // Total: 116 bytes.
    let mut cur = Cur::new(cut_data);

    let _cut_type = cur.read_u8()?;
    cur.skip(1)?; // padding
    let chip_cnt = cur.read_u16_le()? as usize;

    let x = cur.read_i32_le()?;
    let y = cur.read_i32_le()?;
    let disp_xl = cur.read_i32_le()?;
    let disp_yl = cur.read_i32_le()?;
    let xc = cur.read_i32_le()?;
    let yc = cur.read_i32_le()?;
    let cut_xl = cur.read_i32_le()?;
    let cut_yl = cur.read_i32_le()?;

    cur.skip(20 * 4)?; // keep

    if cut_xl <= 0 || cut_yl <= 0 {
        bail!("g00: invalid cut size {}x{}", cut_xl, cut_yl);
    }

    let mut chips = Vec::with_capacity(chip_cnt);
    for _ in 0..chip_cnt {
        // Chip header layout (MSVC default packing, 32-bit):
        //  u16 x
        //  u16 y
        //  u8 type
        //  u8 pad
        //  u16 xl
        //  u16 yl
        //  u16 pad2 (to align i32)
        //  i32 keep[20]
        // Total: 92 bytes.
        let cx = cur.read_u16_le()? as u32;
        let cy = cur.read_u16_le()? as u32;
        let ctype = cur.read_u8()?;
        cur.skip(1)?; // padding
        let w = cur.read_u16_le()? as u32;
        let h = cur.read_u16_le()? as u32;
        cur.skip(2)?; // padding to i32
        cur.skip(20 * 4)?; // keep

        let pix_len = (w as usize)
            .checked_mul(h as usize)
            .and_then(|v| v.checked_mul(4))
            .ok_or_else(|| anyhow!("g00: chip pixel size overflow"))?;
        let pix = cur.take(pix_len)?.to_vec();

        chips.push(G00Chip {
            x: cx,
            y: cy,
            width: w,
            height: h,
            sprite: ctype == 1,
            data: G00ChipData::RawBgra(pix),
        });
    }

    Ok(G00Cut {
        width: cut_xl as u32,
        height: cut_yl as u32,
        center_x: xc,
        center_y: yc,
        disp_left: x,
        disp_top: y,
        disp_right: x.saturating_add(disp_xl),
        disp_bottom: y.saturating_add(disp_yl),
        chips,
    })
}

impl G00Chip {
    /// Decode chip pixels into a BGRA8 buffer (little-endian bytes).
    pub fn decode_bgra(&self) -> Result<Vec<u8>> {
        match &self.data {
            G00ChipData::Type0Lzss32(blob) => {
                let out = lzss_unpack32(blob)?;
                let expected = (self.width as usize)
                    .checked_mul(self.height as usize)
                    .and_then(|v| v.checked_mul(4))
                    .ok_or_else(|| anyhow!("g00: expected size overflow"))?;
                if out.len() != expected {
                    bail!(
                        "g00: type0 size mismatch (got={}, expected={})",
                        out.len(),
                        expected
                    );
                }
                Ok(out)
            }
            G00ChipData::Type1LzssIndexed(blob) => {
                let dec = lzss_unpack(blob)?;
                let mut cur = Cur::new(&dec);

                let pal_cnt = cur.read_u16_le()? as usize;
                if pal_cnt == 0 {
                    bail!("g00: type1 pal_cnt=0");
                }

                let pal_bytes = pal_cnt
                    .checked_mul(4)
                    .ok_or_else(|| anyhow!("g00: palette size overflow"))?;
                let pal_raw = cur.take(pal_bytes)?;

                let expected_px = (self.width as usize)
                    .checked_mul(self.height as usize)
                    .ok_or_else(|| anyhow!("g00: index size overflow"))?;

                if cur.remaining() < expected_px {
                    bail!(
                        "g00: type1 truncated indices (need={}, have={})",
                        expected_px,
                        cur.remaining()
                    );
                }

                let indices = cur.take(expected_px)?;

                // Palette entries are stored as u32 and copied directly to the output dwords in the original implementation.
                // On little-endian, u32::to_le_bytes gives BGRA bytes.
                let mut out: Vec<u8> = Vec::with_capacity(expected_px * 4);
                for &ix in indices {
                    let ix = ix as usize;
                    if ix >= pal_cnt {
                        bail!(
                            "g00: type1 palette index out of range (ix={}, pal_cnt={})",
                            ix,
                            pal_cnt
                        );
                    }
                    let base = ix * 4;
                    out.extend_from_slice(&pal_raw[base..base + 4]);
                }

                Ok(out)
            }
            G00ChipData::RawBgra(pix) => {
                let expected = (self.width as usize)
                    .checked_mul(self.height as usize)
                    .and_then(|v| v.checked_mul(4))
                    .ok_or_else(|| anyhow!("g00: expected size overflow"))?;
                if pix.len() != expected {
                    bail!(
                        "g00: raw size mismatch (got={}, expected={})",
                        pix.len(),
                        expected
                    );
                }
                Ok(pix.clone())
            }
            G00ChipData::Jpeg(bytes) => {
                let jpeg = decode_type3_jpeg_payload(bytes);
                if !jpeg.starts_with(&[0xFF, 0xD8]) {
                    bail!(
                        "g00: type3 XOR decode did not produce JPEG SOI (got={:02X?})",
                        &jpeg[..jpeg.len().min(2)]
                    );
                }
                let img = image::load_from_memory_with_format(&jpeg, ImageFormat::Jpeg)
                    .or_else(|_| image::load_from_memory(&jpeg))
                    .map_err(|e| anyhow!("g00: JPEG decode failed after type3 XOR: {e}"))?;
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                if w != self.width || h != self.height {
                    bail!(
                        "g00: JPEG size mismatch (got={}x{}, expected={}x{})",
                        w,
                        h,
                        self.width,
                        self.height
                    );
                }
                let mut out = rgba.into_raw();
                bgra_to_rgba_in_place(&mut out);
                // We swapped RGBA->BGRA in-place above; decode_bgra expects BGRA.
                Ok(out)
            }
        }
    }

    /// Decode chip pixels into an RGBA8 buffer.
    pub fn decode_rgba(&self) -> Result<Vec<u8>> {
        let mut bgra = self.decode_bgra()?;
        bgra_to_rgba_in_place(&mut bgra);
        Ok(bgra)
    }

    /// Return the stored type 3 payload bytes. These bytes are still XOR-obfuscated.
    pub fn jpeg_bytes(&self) -> Option<&[u8]> {
        match &self.data {
            G00ChipData::Jpeg(b) => Some(b),
            _ => None,
        }
    }
}

fn bgra_to_rgba_in_place(buf: &mut [u8]) {
    for px in buf.as_chunks_mut::<4>().0 {
        // BGRA -> RGBA
        px.swap(0, 2);
    }
}
