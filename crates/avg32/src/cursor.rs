//! AVG32 mouse cursors (`CUR16M` files, loaded with the click-area map by
//! `0x6c:02`).
//!
//! Layout: `"CUR16M\0\0"`, cursor count, payload size, 16 bytes of padding,
//! then the hot spot (x, y) followed by animation frames of 32x32 pixels:
//! 3072 bytes of BGR colour and a 128-byte AND mask (1 = transparent,
//! most-significant bit first).

use anyhow::{Result, bail};

pub const CURSOR_SIZE: usize = 32;
const FRAME_BYTES: usize = CURSOR_SIZE * CURSOR_SIZE * 3 + CURSOR_SIZE * CURSOR_SIZE / 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorFrame {
    /// RGB colour per pixel.
    pub rgb: Vec<[u8; 3]>,
    pub opaque: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub hot_x: i32,
    pub hot_y: i32,
    pub frames: Vec<CursorFrame>,
}

impl Cursor {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 0x28 || &bytes[..6] != b"CUR16M" {
            bail!("avg32: not a CUR16M cursor");
        }
        let int = |at: usize| i32::from_le_bytes(bytes[at..at + 4].try_into().expect("four bytes"));
        let (hot_x, hot_y) = (int(0x20), int(0x24));
        let frames = bytes[0x28..]
            .chunks_exact(FRAME_BYTES)
            .map(|frame| {
                let (colour, mask) = frame.split_at(CURSOR_SIZE * CURSOR_SIZE * 3);
                let rgb: Vec<[u8; 3]> = colour
                    .chunks_exact(3)
                    .map(|bgr| [bgr[2], bgr[1], bgr[0]])
                    .collect();
                // Pure red is a colour key: AIR's feather cursor carries a
                // red "X" guide over the artwork that is never shown.
                let opaque = (0..CURSOR_SIZE * CURSOR_SIZE)
                    .map(|index| {
                        mask[index / 8] & (0x80 >> (index % 8)) == 0 && rgb[index] != [255, 0, 0]
                    })
                    .collect();
                CursorFrame { rgb, opaque }
            })
            .collect::<Vec<_>>();
        if frames.is_empty() {
            bail!("avg32: cursor has no frames");
        }
        Ok(Self {
            hot_x: hot_x.clamp(0, CURSOR_SIZE as i32 - 1),
            hot_y: hot_y.clamp(0, CURSOR_SIZE as i32 - 1),
            frames,
        })
    }

    /// Draws animation frame `frame` with its hot spot at `(x, y)`.
    pub fn draw(&self, rgba: &mut [u8], x: i32, y: i32, frame: usize) {
        let frame = &self.frames[frame % self.frames.len()];
        for row in 0..CURSOR_SIZE {
            for column in 0..CURSOR_SIZE {
                let index = row * CURSOR_SIZE + column;
                if !frame.opaque[index] {
                    continue;
                }
                let (px, py) = (x - self.hot_x + column as i32, y - self.hot_y + row as i32);
                if !(0..640).contains(&px) || !(0..480).contains(&py) {
                    continue;
                }
                let at = (py as usize * 640 + px as usize) * 4;
                rgba[at..at + 3].copy_from_slice(&frame.rgb[index]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hot_spot_colour_and_mask() {
        let mut bytes = b"CUR16M\0\0".to_vec();
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&((8 + FRAME_BYTES) as u32).to_le_bytes());
        bytes.extend_from_slice(&[0; 16]);
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&30u32.to_le_bytes());
        let mut frame = vec![0u8; FRAME_BYTES];
        frame[..3].copy_from_slice(&[0x61, 0xad, 0xff]);
        frame[3072..].fill(0xff);
        frame[3072] = 0x7f;
        bytes.extend_from_slice(&frame);
        let cursor = Cursor::parse(&bytes).unwrap();
        assert_eq!((cursor.hot_x, cursor.hot_y), (1, 30));
        assert_eq!(cursor.frames[0].rgb[0], [0xff, 0xad, 0x61]);
        assert!(cursor.frames[0].opaque[0]);
        assert!(!cursor.frames[0].opaque[1]);
    }
}
