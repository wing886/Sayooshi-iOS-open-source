//! Scenario compression: an LZ77 variant whose stream is XORed with a fixed
//! 256-byte mask, plus the "second-level" XOR that newer compilers
//! (versions 110002 and 1110002) apply to a region of the decompressed
//! bytecode with a per-title 16-byte key.
//!
//! Known keys are listed by `#REGNAME`. For titles that are not in the
//! list, [`derive_xor2_key`] recovers the key statistically from the
//! scenarios themselves.

use anyhow::{Result, bail};

const XOR_MASK: [u8; 256] = [
    0x8b, 0xe5, 0x5d, 0xc3, 0xa1, 0xe0, 0x30, 0x44, 0x00, 0x85, 0xc0, 0x74, 0x09, 0x5f, 0x5e, 0x33,
    0xc0, 0x5b, 0x8b, 0xe5, 0x5d, 0xc3, 0x8b, 0x45, 0x0c, 0x85, 0xc0, 0x75, 0x14, 0x8b, 0x55, 0xec,
    0x83, 0xc2, 0x20, 0x52, 0x6a, 0x00, 0xe8, 0xf5, 0x28, 0x01, 0x00, 0x83, 0xc4, 0x08, 0x89, 0x45,
    0x0c, 0x8b, 0x45, 0xe4, 0x6a, 0x00, 0x6a, 0x00, 0x50, 0x53, 0xff, 0x15, 0x34, 0xb1, 0x43, 0x00,
    0x8b, 0x45, 0x10, 0x85, 0xc0, 0x74, 0x05, 0x8b, 0x4d, 0xec, 0x89, 0x08, 0x8a, 0x45, 0xf0, 0x84,
    0xc0, 0x75, 0x78, 0xa1, 0xe0, 0x30, 0x44, 0x00, 0x8b, 0x7d, 0xe8, 0x8b, 0x75, 0x0c, 0x85, 0xc0,
    0x75, 0x44, 0x8b, 0x1d, 0xd0, 0xb0, 0x43, 0x00, 0x85, 0xff, 0x76, 0x37, 0x81, 0xff, 0x00, 0x00,
    0x04, 0x00, 0x6a, 0x00, 0x76, 0x43, 0x8b, 0x45, 0xf8, 0x8d, 0x55, 0xfc, 0x52, 0x68, 0x00, 0x00,
    0x04, 0x00, 0x56, 0x50, 0xff, 0x15, 0x2c, 0xb1, 0x43, 0x00, 0x6a, 0x05, 0xff, 0xd3, 0xa1, 0xe0,
    0x30, 0x44, 0x00, 0x81, 0xef, 0x00, 0x00, 0x04, 0x00, 0x81, 0xc6, 0x00, 0x00, 0x04, 0x00, 0x85,
    0xc0, 0x74, 0xc5, 0x8b, 0x5d, 0xf8, 0x53, 0xe8, 0xf4, 0xfb, 0xff, 0xff, 0x8b, 0x45, 0x0c, 0x83,
    0xc4, 0x04, 0x5f, 0x5e, 0x5b, 0x8b, 0xe5, 0x5d, 0xc3, 0x8b, 0x55, 0xf8, 0x8d, 0x4d, 0xfc, 0x51,
    0x57, 0x56, 0x52, 0xff, 0x15, 0x2c, 0xb1, 0x43, 0x00, 0xeb, 0xd8, 0x8b, 0x45, 0xe8, 0x83, 0xc0,
    0x20, 0x50, 0x6a, 0x00, 0xe8, 0x47, 0x28, 0x01, 0x00, 0x8b, 0x7d, 0xe8, 0x89, 0x45, 0xf4, 0x8b,
    0xf0, 0xa1, 0xe0, 0x30, 0x44, 0x00, 0x83, 0xc4, 0x08, 0x85, 0xc0, 0x75, 0x56, 0x8b, 0x1d, 0xd0,
    0xb0, 0x43, 0x00, 0x85, 0xff, 0x76, 0x49, 0x81, 0xff, 0x00, 0x00, 0x04, 0x00, 0x6a, 0x00, 0x76,
];

/// One XOR key segment: `length` bytes from `offset` of the decompressed
/// bytecode are XORed with `key` repeated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XorSegment {
    pub key: [u8; 16],
    pub offset: usize,
    pub length: usize,
}

/// A title's complete second-level key (one or more segments).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Xor2Key {
    pub segments: Vec<XorSegment>,
}

impl Xor2Key {
    pub fn single(key: [u8; 16]) -> Self {
        Self {
            segments: vec![XorSegment {
                key,
                offset: 256,
                length: 257,
            }],
        }
    }

    pub fn apply(&self, data: &mut [u8]) {
        for segment in &self.segments {
            for i in 0..segment.length {
                if let Some(byte) = data.get_mut(segment.offset + i) {
                    *byte ^= segment.key[i % 16];
                }
            }
        }
    }
}

/// Second-level keys of the titles known to use them, by `#REGNAME`
/// (decoded as Shift-JIS).
pub fn known_xor2_key(regname: &str) -> Option<Xor2Key> {
    let key = match regname {
        "KEY\\CLANNAD_FV" => Xor2Key::single([
            0xaf, 0x2f, 0xfb, 0x6b, 0xaf, 0x30, 0x77, 0x17, 0x87, 0x48, 0xfe, 0x2c, 0x68, 0x1a,
            0xb9, 0xf0,
        ]),
        "KEY\\リトルバスターズ！" => Xor2Key::single([
            0xa8, 0x28, 0xfd, 0x66, 0xa0, 0x23, 0x77, 0x69, 0xf9, 0x45, 0xf8, 0x2c, 0x7c, 0x00,
            0xad, 0xf4,
        ]),
        "KEY\\リトルバスターズ！ＥＸ" => Xor2Key {
            segments: vec![
                XorSegment {
                    key: [
                        0xa8, 0x28, 0xfd, 0x71, 0xb4, 0x23, 0x64, 0x15, 0x96, 0x48, 0x8a, 0x43,
                        0x62, 0x0e, 0xad, 0xf0,
                    ],
                    offset: 256,
                    length: 128,
                },
                XorSegment {
                    key: [
                        0xde, 0xd9, 0x4a, 0x18, 0xaf, 0x23, 0x1d, 0x9a, 0xac, 0x23, 0x25, 0x48,
                        0xd8, 0xd4, 0x8f, 0xa7,
                    ],
                    offset: 384,
                    length: 128,
                },
                XorSegment {
                    key: [
                        0xde, 0xf1, 0xb7, 0x69, 0x1b, 0x00, 0x79, 0x8f, 0x3a, 0x6b, 0xaf, 0x0b,
                        0xba, 0xda, 0x22, 0x57,
                    ],
                    offset: 512,
                    length: 16,
                },
                XorSegment {
                    key: [
                        0x76, 0xf1, 0xb7, 0x69, 0x1b, 0x00, 0x79, 0x8f, 0x3a, 0x6b, 0xaf, 0x0b,
                        0xba, 0xda, 0x22, 0x57,
                    ],
                    offset: 528,
                    length: 113,
                },
            ],
        },
        "StudioMebius\\SNOWSE" => Xor2Key::single([
            0xe4, 0xab, 0xa2, 0xc9, 0xec, 0x39, 0x36, 0x62, 0xc9, 0x03, 0xba, 0x6d, 0x2e, 0x9c,
            0xf2, 0x64,
        ]),
        "KEY\\クドわふたー" => Xor2Key::single([
            0x67, 0x1c, 0x21, 0xbe, 0x6f, 0xef, 0xb5, 0x16, 0x4a, 0x82, 0x39, 0x2b, 0xad, 0x3a,
            0x71, 0x3f,
        ]),
        "KEY\\クドわふたー【全年齢対象版】" => Xor2Key::single([
            0xaf, 0x3f, 0xe6, 0x63, 0xad, 0x3a, 0x69, 0x18, 0x85, 0x45, 0xe5, 0x40, 0x1e, 0x7e,
            0xb9, 0xe0,
        ]),
        _ => return None,
    };
    Some(key)
}

/// Decompresses a scenario's code block: `src` starts with the 8-byte
/// block header (packed and unpacked size), `unpacked_len` comes from the
/// scenario header.
pub fn decompress(src: &[u8], unpacked_len: usize) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(unpacked_len);
    let mut at = 8usize;
    let mut mask = 8u8;
    let mut next = |at: &mut usize, mask: &mut u8| -> Option<u8> {
        let byte = *src.get(*at)? ^ XOR_MASK[usize::from(*mask)];
        *at += 1;
        *mask = mask.wrapping_add(1);
        Some(byte)
    };
    let Some(mut flag) = next(&mut at, &mut mask) else {
        bail!("reallive: empty compressed block");
    };
    let mut bit = 1u32;
    while at < src.len() && out.len() < unpacked_len {
        if bit == 256 {
            bit = 1;
            match next(&mut at, &mut mask) {
                Some(byte) => flag = byte,
                None => break,
            }
        }
        if u32::from(flag) & bit != 0 {
            match next(&mut at, &mut mask) {
                Some(byte) => out.push(byte),
                None => break,
            }
        } else {
            let (Some(low), Some(high)) = (next(&mut at, &mut mask), next(&mut at, &mut mask))
            else {
                break;
            };
            let count = usize::from(low) | usize::from(high) << 8;
            let distance = count >> 4;
            let length = (count & 0x0f) + 2;
            if distance == 0 || distance > out.len() {
                bail!("reallive: corrupt compressed data (distance {distance})");
            }
            let start = out.len() - distance;
            for i in 0..length {
                if out.len() >= unpacked_len {
                    break;
                }
                out.push(out[start + i]);
            }
        }
        bit <<= 1;
    }
    Ok(out)
}

/// Compresses `data` so that [`decompress`] restores it (literals only:
/// used by tests and the scenario writer, not size-optimal).
pub fn compress_literal(data: &[u8]) -> Vec<u8> {
    let mut raw = vec![0u8; 8];
    for chunk in data.chunks(8) {
        let flag = if chunk.len() == 8 {
            0xff
        } else {
            ((1u16 << chunk.len()) - 1) as u8
        };
        raw.push(flag);
        raw.extend_from_slice(chunk);
    }
    let packed = raw.len() as u32;
    raw[0..4].copy_from_slice(&packed.to_le_bytes());
    raw[4..8].copy_from_slice(&(data.len() as u32).to_le_bytes());
    for (i, byte) in raw.iter_mut().enumerate().skip(8) {
        *byte ^= XOR_MASK[i % 256];
    }
    raw
}

/// Recovers an unknown second-level key from the first-level-decompressed
/// code of many scenarios.
///
/// The key covers a fixed range of every scenario (starting at byte 256),
/// so each byte position is XORed with the same value in every file. The
/// part of each file past that range is plain bytecode, whose byte
/// distribution is very uneven (`$`, `#`, `\xff`, `\0`, text lead bytes).
/// For every position the XOR value that makes the samples of all files
/// look most like that distribution is chosen; positions whose best value
/// is 0 are unencrypted. Returns `None` when too few scenarios are long
/// enough for a reliable estimate.
pub fn derive_xor2_key(codes: &[Vec<u8>]) -> Option<Xor2Key> {
    const START: usize = 256;
    const END: usize = 1024;
    // Background distribution from the unencrypted tails.
    let mut counts = [1.0f64; 256];
    for code in codes {
        for &byte in code.iter().skip(END) {
            counts[usize::from(byte)] += 1.0;
        }
    }
    let total: f64 = counts.iter().sum();
    if total < 10_000.0 {
        return None;
    }
    let log_p: Vec<f64> = counts.iter().map(|count| (count / total).ln()).collect();
    let mut xor = vec![0u8; END - START];
    let mut any = false;
    for position in START..END {
        let samples: Vec<u8> = codes
            .iter()
            .filter_map(|code| code.get(position).copied())
            .collect();
        if samples.len() < 16 {
            break;
        }
        let score = |x: u8| -> f64 {
            samples
                .iter()
                .map(|&byte| log_p[usize::from(byte ^ x)])
                .sum()
        };
        let (best, best_score) = (0..=255u8)
            .map(|x| (x, score(x)))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .expect("256 candidates");
        // Prefer "not encrypted" unless another value is clearly better.
        if best != 0 && best_score - score(0) > samples.len() as f64 * 0.5 {
            xor[position - START] = best;
            any = true;
        }
    }
    if !any {
        return None;
    }
    // Smooth with the key's 16-byte period: an isolated disagreement with
    // both neighbours one period away is almost certainly noise.
    for i in 16..xor.len().saturating_sub(16) {
        if xor[i - 16] == xor[i + 16] && xor[i] != xor[i - 16] {
            xor[i] = xor[i - 16];
        }
    }
    // Pack runs of 16-periodic bytes into segments.
    let mut segments = Vec::new();
    let mut i = 0;
    while i < xor.len() {
        if xor[i] == 0 && (i + 16 >= xor.len() || xor[i + 16] == 0) {
            i += 1;
            continue;
        }
        let start = i;
        let mut key = [0u8; 16];
        for (k, slot) in key.iter_mut().enumerate() {
            *slot = xor.get(start + k).copied().unwrap_or(0);
        }
        let mut end = start;
        while end < xor.len() && xor[end] == key[(end - start) % 16] {
            end += 1;
        }
        // Trailing positions that are plain zero bytes of the key are
        // indistinguishable from unencrypted ones; that is harmless.
        segments.push(XorSegment {
            key,
            offset: START + start,
            length: end - start,
        });
        i = end.max(start + 1);
    }
    Some(Xor2Key { segments })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_compression_round_trips() {
        let data: Vec<u8> = (0..1000u32).map(|i| (i * 7 % 251) as u8).collect();
        assert_eq!(
            decompress(&compress_literal(&data), data.len()).unwrap(),
            data
        );
    }

    #[test]
    fn back_references_copy_overlapping_runs() {
        // flag 0b10: literal 'a' then a back reference (distance 1, length 5).
        let mut raw = vec![0u8; 8];
        raw.push(0b01);
        raw.push(b'a');
        let count: u16 = (1 << 4) | 3;
        raw.extend_from_slice(&count.to_le_bytes());
        for (i, byte) in raw.iter_mut().enumerate().skip(8) {
            *byte ^= XOR_MASK[i % 256];
        }
        assert_eq!(decompress(&raw, 6).unwrap(), b"aaaaaa");
    }

    #[test]
    fn derives_a_key_from_many_scenarios() {
        // Fake bytecode with a skewed byte distribution.
        let mut state = 12345u32;
        let mut random = move || {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            (state >> 16) as u8
        };
        let alphabet = b"$\xff\x00\x00\x00\x00#\x01\x02\n\x0a@$$$\x82\x82\xa0\xa9(),";
        let key = Xor2Key::single(*b"0123456789abcdef");
        let codes: Vec<Vec<u8>> = (0..200)
            .map(|_| {
                let mut code: Vec<u8> = (0..3000)
                    .map(|_| alphabet[usize::from(random()) % alphabet.len()])
                    .collect();
                key.apply(&mut code);
                code
            })
            .collect();
        let derived = derive_xor2_key(&codes).unwrap();
        let mut probe = vec![0u8; 1024];
        key.apply(&mut probe);
        let mut again = vec![0u8; 1024];
        derived.apply(&mut again);
        assert_eq!(probe, again);
    }
}
