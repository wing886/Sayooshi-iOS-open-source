//! Decoder for the `PDT10` and `PDT11` bitmap formats used by AVG32.

use anyhow::{Result, bail};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdtImage {
    pub width: u32,
    pub height: u32,
    /// Pixels in conventional RGBA8 order.  Colour is premultiplied by the
    /// mask, which is stored as alpha (255 when the file has no mask).
    pub rgba: Vec<u8>,
    pub has_mask: bool,
}

pub fn decode_pdt(bytes: &[u8]) -> Result<PdtImage> {
    if bytes.len() < 0x20 {
        bail!("avg32: PDT file is smaller than its header");
    }
    let kind = &bytes[..5];
    if kind != b"PDT10" && kind != b"PDT11" {
        bail!("avg32: expected PDT10 or PDT11 magic");
    }
    if read_u32(bytes, 8)? as usize != bytes.len() {
        bail!("avg32: PDT header size does not match input");
    }
    let width = read_u32(bytes, 0x0c)?;
    let height = read_u32(bytes, 0x10)?;
    let pixels = usize::try_from(width)
        .ok()
        .and_then(|width| {
            usize::try_from(height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .ok_or_else(|| anyhow::anyhow!("avg32: PDT dimensions overflow"))?;
    let mask_at = read_u32(bytes, 0x1c)? as usize;
    if mask_at != 0 && (mask_at < 0x20 || mask_at > bytes.len()) {
        bail!("avg32: PDT mask offset is outside input");
    }

    let mut rgba = if kind == b"PDT10" {
        let colour_end = if mask_at == 0 { bytes.len() } else { mask_at };
        decode_pdt10_colours(&bytes[0x20..colour_end], pixels)?
    } else {
        decode_pdt11_colours(bytes, pixels, mask_at)?
    };

    if mask_at != 0 {
        let alpha = unpack_mask(&bytes[mask_at..], pixels)?;
        for (pixel, alpha) in rgba.chunks_exact_mut(4).zip(alpha) {
            pixel[3] = alpha;
        }
    }
    Ok(PdtImage {
        width,
        height,
        rgba,
        has_mask: mask_at != 0,
    })
}

fn decode_pdt10_colours(input: &[u8], pixels: usize) -> Result<Vec<u8>> {
    let mut at = 0usize;
    let mut bgr = Vec::with_capacity(pixels * 3);
    while bgr.len() < pixels * 3 {
        let flags = next(input, &mut at, "PDT10 flags")?;
        for bit in 0..8 {
            if bgr.len() == pixels * 3 {
                break;
            }
            if flags & (0x80 >> bit) != 0 {
                bgr.extend_from_slice(&[
                    next(input, &mut at, "PDT10 literal")?,
                    next(input, &mut at, "PDT10 literal")?,
                    next(input, &mut at, "PDT10 literal")?,
                ]);
            } else {
                let token = u16::from_le_bytes([
                    next(input, &mut at, "PDT10 match")?,
                    next(input, &mut at, "PDT10 match")?,
                ]);
                let count = usize::from((token & 0x0f) + 1);
                let distance = usize::from(token >> 4) + 1;
                if distance * 3 > bgr.len() {
                    bail!("avg32: PDT10 match refers before image start");
                }
                for _ in 0..count {
                    if bgr.len() == pixels * 3 {
                        break;
                    }
                    let src = bgr.len() - distance * 3;
                    let pixel = [bgr[src], bgr[src + 1], bgr[src + 2]];
                    bgr.extend_from_slice(&pixel);
                }
            }
        }
    }
    let mut rgba = Vec::with_capacity(pixels * 4);
    for pixel in bgr.chunks_exact(3) {
        rgba.extend_from_slice(&[pixel[2], pixel[1], pixel[0], 255]);
    }
    Ok(rgba)
}

fn decode_pdt11_colours(bytes: &[u8], pixels: usize, mask_at: usize) -> Result<Vec<u8>> {
    if bytes.len() < 0x460 {
        bail!("avg32: PDT11 file is smaller than palette/index tables");
    }
    let colour_end = if mask_at == 0 { bytes.len() } else { mask_at };
    if colour_end < 0x460 {
        bail!("avg32: PDT11 colour stream overlaps its header");
    }
    let mut table = [0u32; 16];
    for (index, slot) in table.iter_mut().enumerate() {
        *slot = read_u32(bytes, 0x420 + index * 4)?;
    }
    let indices = unpack_pdt11_indices(&bytes[0x460..colour_end], pixels, &table)?;
    let mut rgba = Vec::with_capacity(pixels * 4);
    for index in indices {
        let colour = read_u32(bytes, 0x20 + usize::from(index) * 4)?;
        let [blue, green, red, _] = colour.to_le_bytes();
        rgba.extend_from_slice(&[red, green, blue, 255]);
    }
    Ok(rgba)
}

fn unpack_pdt11_indices(input: &[u8], pixels: usize, table: &[u32; 16]) -> Result<Vec<u8>> {
    let mut at = 0usize;
    let mut output = Vec::with_capacity(pixels);
    while output.len() < pixels {
        let flags = next(input, &mut at, "PDT11 flags")?;
        for bit in 0..8 {
            if output.len() == pixels {
                break;
            }
            if flags & (0x80 >> bit) != 0 {
                output.push(next(input, &mut at, "PDT11 literal")?);
            } else {
                let token = next(input, &mut at, "PDT11 match")?;
                let count = usize::from(token >> 4) + 2;
                let distance = table[usize::from(token & 0x0f)] as usize;
                if distance == 0 || distance > output.len() {
                    bail!("avg32: PDT11 match refers before image start");
                }
                for _ in 0..count {
                    if output.len() == pixels {
                        break;
                    }
                    output.push(output[output.len() - distance]);
                }
            }
        }
    }
    Ok(output)
}

fn unpack_mask(input: &[u8], pixels: usize) -> Result<Vec<u8>> {
    let mut at = 0usize;
    let mut output = Vec::with_capacity(pixels);
    while output.len() < pixels {
        let flags = next(input, &mut at, "PDT mask flags")?;
        for bit in 0..8 {
            if output.len() == pixels {
                break;
            }
            if flags & (0x80 >> bit) != 0 {
                output.push(next(input, &mut at, "PDT mask literal")?);
            } else {
                let low = next(input, &mut at, "PDT mask match")?;
                let high = next(input, &mut at, "PDT mask match")?;
                let count = usize::from(low) + 2;
                let distance = usize::from(high) + 1;
                if distance > output.len() {
                    bail!("avg32: PDT mask match refers before output start");
                }
                for _ in 0..count {
                    if output.len() == pixels {
                        break;
                    }
                    output.push(output[output.len() - distance]);
                }
            }
        }
    }
    Ok(output)
}

fn next(input: &[u8], at: &mut usize, what: &str) -> Result<u8> {
    let byte = *input
        .get(*at)
        .ok_or_else(|| anyhow::anyhow!("avg32: truncated {what}"))?;
    *at += 1;
    Ok(byte)
}

fn read_u32(bytes: &[u8], at: usize) -> Result<u32> {
    let slice = bytes
        .get(at..at + 4)
        .ok_or_else(|| anyhow::anyhow!("avg32: truncated u32 at {at:#x}"))?;
    Ok(u32::from_le_bytes(
        slice.try_into().expect("slice has four bytes"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_literal_pdt10_pixel() {
        let mut pdt = vec![0u8; 0x20];
        pdt[..5].copy_from_slice(b"PDT10");
        pdt[0x0c..0x10].copy_from_slice(&1u32.to_le_bytes());
        pdt[0x10..0x14].copy_from_slice(&1u32.to_le_bytes());
        pdt.extend_from_slice(&[0x80, 0x11, 0x22, 0x33]); // B, G, R
        let size = pdt.len() as u32;
        pdt[8..12].copy_from_slice(&size.to_le_bytes());
        assert_eq!(decode_pdt(&pdt).unwrap().rgba, [0x33, 0x22, 0x11, 255]);
    }
}
