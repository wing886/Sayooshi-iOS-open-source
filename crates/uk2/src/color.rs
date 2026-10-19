//! Parser and helpers for UK2's `COLOR.TBL` hardware-palette banks.

use anyhow::{Context, Result, bail};

pub const COLOR_BANK_COLORS: usize = 16;
pub const COLOR_BANK_BYTES: usize = COLOR_BANK_COLORS * 2;
pub const MAX_COLOR_BANKS: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorTable {
    banks: Vec<[u16; COLOR_BANK_COLORS]>,
}

impl ColorTable {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() || bytes.len() % COLOR_BANK_BYTES != 0 {
            bail!("uk2: COLOR.TBL length must be a nonzero multiple of {COLOR_BANK_BYTES}");
        }
        let count = bytes.len() / COLOR_BANK_BYTES;
        if count > MAX_COLOR_BANKS {
            bail!("uk2: COLOR.TBL has {count} banks; UK2 supports at most {MAX_COLOR_BANKS}");
        }
        let mut banks = Vec::with_capacity(count);
        for (bank_index, bank_bytes) in bytes.chunks_exact(COLOR_BANK_BYTES).enumerate() {
            let mut bank = [0u16; COLOR_BANK_COLORS];
            for (color_index, color) in bank.iter_mut().enumerate() {
                let at = color_index * 2;
                *color = u16::from_le_bytes([bank_bytes[at], bank_bytes[at + 1]]);
                if *color > 0x0fff {
                    bail!("uk2: COLOR.TBL bank {bank_index} color {color_index} exceeds 12 bits");
                }
            }
            banks.push(bank);
        }
        Ok(Self { banks })
    }

    /// UK2 scripts address banks starting at one; zero is invalid.
    pub fn bank(&self, one_based_index: u16) -> Result<&[u16; COLOR_BANK_COLORS]> {
        let index = usize::from(one_based_index)
            .checked_sub(1)
            .ok_or_else(|| anyhow::anyhow!("uk2: COLOR.TBL bank index is one-based"))?;
        self.banks.get(index).with_context(|| {
            format!(
                "uk2: COLOR.TBL bank {one_based_index} is missing ({} banks loaded)",
                self.banks.len()
            )
        })
    }

    pub fn len(&self) -> usize {
        self.banks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.banks.is_empty()
    }
}

/// Converts a PC-98 analog palette word (`0xGRB`) to RGB.
pub fn color_word_to_rgb(color: u16) -> [u8; 3] {
    [
        (((color >> 4) & 0x0f) as u8) * 17,
        (((color >> 8) & 0x0f) as u8) * 17,
        ((color & 0x0f) as u8) * 17,
    ]
}

pub fn transition_rgba(
    rgba: &[u8],
    indexed: &[u8],
    target: &[u16; COLOR_BANK_COLORS],
    numerator: u8,
    denominator: u8,
) -> Result<Vec<u8>> {
    if denominator == 0
        || numerator > denominator
        || rgba.len() % 4 != 0
        || rgba.len() != indexed.len() * 8
    {
        bail!("uk2: invalid RGBA palette transition parameters");
    }
    let target = target.map(color_word_to_rgb);
    let scale = u32::from(denominator);
    let step = u32::from(numerator);
    let mut output = rgba.to_vec();
    for (pixel, (source, destination)) in rgba
        .chunks_exact(4)
        .zip(output.chunks_exact_mut(4))
        .enumerate()
    {
        let packed = indexed[pixel / 2];
        let index = usize::from(if pixel & 1 == 0 {
            packed >> 4
        } else {
            packed & 0x0f
        });
        for channel in 0..3 {
            let from = u32::from(source[channel]);
            let to = u32::from(target[index][channel]);
            destination[channel] = ((from * (scale - step) + to * step) / scale) as u8;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_little_endian_banks_and_uses_one_based_indices() {
        let table = ColorTable::parse(&[0x23, 0x01].repeat(COLOR_BANK_BYTES / 2)).unwrap();
        assert_eq!(table.len(), 1);
        assert_eq!(table.bank(1).unwrap()[0], 0x0123);
        assert!(table.bank(0).is_err());
        assert!(table.bank(2).is_err());
    }

    #[test]
    fn rejects_incomplete_or_non_rgb12_data() {
        assert!(ColorTable::parse(&[0; COLOR_BANK_BYTES - 1]).is_err());
        let mut bytes = vec![0; COLOR_BANK_BYTES];
        bytes[1] = 0x10;
        assert!(ColorTable::parse(&bytes).is_err());
    }

    #[test]
    fn expands_grb_nibbles_to_eight_bit_channels() {
        assert_eq!(color_word_to_rgb(0x0f8), [255, 0, 136]);
        assert_eq!(color_word_to_rgb(0xf00), [0, 255, 0]);
    }

    #[test]
    fn palette_transition_reaches_selected_bank_colors() {
        let mut target = [0u16; COLOR_BANK_COLORS];
        target[0] = 0xf00;
        let pixels = [0u8, 0, 0, 255, 10, 10, 10, 255];
        let transitioned = transition_rgba(&pixels, &[0], &target, 16, 16).unwrap();
        assert_eq!(&transitioned[..4], &[0, 255, 0, 255]);
        assert_eq!(&transitioned[4..], &[0, 255, 0, 255]);
        assert!(transition_rgba(&pixels, &[], &target, 16, 16).is_err());
    }
}
