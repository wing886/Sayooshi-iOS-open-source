//! GAN animation files: sets of frames that select patterns of a `g00`.
//!
//! Layout: `10000 10000 10100 name_len name\0 20000 set_count`, then per
//! set `30000 frame_count` and per frame tag/value pairs ended by
//! `999999` (30100 pattern, 30101 x, 30102 y, 30103 time, 30104 alpha,
//! 30105 other).

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GanFrame {
    pub pattern: i32,
    pub x: i32,
    pub y: i32,
    /// Duration in ms.
    pub time: i32,
    pub alpha: i32,
    pub other: i32,
}

impl Default for GanFrame {
    fn default() -> Self {
        Self {
            pattern: -1,
            x: 0,
            y: 0,
            time: 0,
            alpha: 255,
            other: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Gan {
    pub image_name: String,
    pub sets: Vec<Vec<GanFrame>>,
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl Cursor<'_> {
    fn i32(&mut self) -> Result<i32> {
        let bytes = self
            .data
            .get(self.pos..self.pos + 4)
            .ok_or_else(|| anyhow::anyhow!("reallive: truncated GAN file"))?;
        self.pos += 4;
        Ok(i32::from_le_bytes(bytes.try_into().expect("4")))
    }
}

impl Gan {
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut c = Cursor { data, pos: 0 };
        if (c.i32()?, c.i32()?, c.i32()?) != (10000, 10000, 10100) {
            bail!("reallive: not a GAN file");
        }
        let name_len = c.i32()?.max(0) as usize;
        let name_bytes = data.get(c.pos..c.pos + name_len).unwrap_or(&[]);
        let image_name = crate::nls::Nls::Sjis.decode(
            &name_bytes[..name_bytes
                .iter()
                .position(|b| *b == 0)
                .unwrap_or(name_bytes.len())],
        );
        c.pos += name_len;
        if c.i32()? != 20000 {
            bail!("reallive: GAN data section missing");
        }
        let set_count = c.i32()?.max(0);
        let mut sets = Vec::new();
        for _ in 0..set_count {
            if c.i32()? != 30000 {
                bail!("reallive: GAN set marker missing");
            }
            let frames = c.i32()?.max(0);
            let mut set = Vec::with_capacity(frames as usize);
            for _ in 0..frames {
                let mut frame = GanFrame::default();
                loop {
                    let tag = c.i32()?;
                    if tag == 999_999 {
                        break;
                    }
                    let value = c.i32()?;
                    match tag {
                        30100 => frame.pattern = value,
                        30101 => frame.x = value,
                        30102 => frame.y = value,
                        30103 => frame.time = value,
                        30104 => frame.alpha = value,
                        30105 => frame.other = value,
                        _ => {}
                    }
                }
                set.push(frame);
            }
            sets.push(set);
        }
        Ok(Self { image_name, sets })
    }

    /// Serializes (tests and tools).
    pub fn to_bytes(&self) -> Vec<u8> {
        fn put(out: &mut Vec<u8>, value: i32) {
            out.extend_from_slice(&value.to_le_bytes());
        }
        let mut out = Vec::new();
        for magic in [10000, 10000, 10100] {
            put(&mut out, magic);
        }
        let name = crate::nls::Nls::Sjis.encode(&self.image_name);
        put(&mut out, name.len() as i32 + 1);
        out.extend_from_slice(&name);
        out.push(0);
        put(&mut out, 20000);
        put(&mut out, self.sets.len() as i32);
        for set in &self.sets {
            put(&mut out, 30000);
            put(&mut out, set.len() as i32);
            for frame in set {
                for (tag, value) in [
                    (30100, frame.pattern),
                    (30101, frame.x),
                    (30102, frame.y),
                    (30103, frame.time),
                    (30104, frame.alpha),
                    (30105, frame.other),
                ] {
                    put(&mut out, tag);
                    put(&mut out, value);
                }
                put(&mut out, 999_999);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let gan = Gan {
            image_name: "BG01".into(),
            sets: vec![vec![
                GanFrame {
                    pattern: 1,
                    time: 100,
                    ..GanFrame::default()
                },
                GanFrame {
                    pattern: 2,
                    x: -4,
                    time: 50,
                    ..GanFrame::default()
                },
            ]],
        };
        assert_eq!(Gan::parse(&gan.to_bytes()).unwrap(), gan);
    }
}
