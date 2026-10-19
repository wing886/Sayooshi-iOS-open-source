//! Decoder and scheduler data for AVG32 `ANM32` animations.

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationCell {
    pub source_rect: [i32; 4],
    pub destination: [i32; 2],
    pub wait_microseconds: u32,
}

#[derive(Debug, Clone)]
pub struct Avg32Animation {
    bytes: Vec<u8>,
    pub source_pdt: String,
    cells: Vec<AnimationCell>,
    stream_base: usize,
    scene_base: usize,
    stream_count: usize,
    scene_count: usize,
}

impl Avg32Animation {
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        if bytes.get(..5) != Some(b"ANM32") {
            bail!("avg32: expected ANM32 animation");
        }
        if le_u32(&bytes, 6)? != 0x0100_0000 {
            bail!("avg32: unsupported ANM32 version");
        }
        let source_pdt = c_string(&bytes, 0x1c)?;
        let cell_count = le_u32(&bytes, 0x8c)? as usize;
        let stream_count = le_u32(&bytes, 0x90)? as usize;
        let scene_count = le_u32(&bytes, 0x94)? as usize;
        let stream_base = 0xb8usize
            .checked_add(
                cell_count
                    .checked_mul(0x60)
                    .ok_or_else(|| anyhow::anyhow!("avg32: ANM cell table overflows"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("avg32: ANM stream table overflows"))?;
        let scene_base = stream_base
            .checked_add(
                stream_count
                    .checked_mul(0x68)
                    .ok_or_else(|| anyhow::anyhow!("avg32: ANM stream table overflows"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("avg32: ANM scene table overflows"))?;
        let end = scene_base
            .checked_add(
                scene_count
                    .checked_mul(0x78)
                    .ok_or_else(|| anyhow::anyhow!("avg32: ANM scene table overflows"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("avg32: ANM scene table overflows"))?;
        if end > bytes.len() {
            bail!("avg32: truncated ANM32 tables");
        }
        let mut cells = Vec::with_capacity(cell_count);
        for index in 0..cell_count {
            let at = 0xb8 + index * 0x60;
            cells.push(AnimationCell {
                source_rect: [
                    le_i32(&bytes, at)?,
                    le_i32(&bytes, at + 4)?,
                    le_i32(&bytes, at + 8)?,
                    le_i32(&bytes, at + 12)?,
                ],
                destination: [le_i32(&bytes, at + 16)?, le_i32(&bytes, at + 20)?],
                wait_microseconds: le_u32(&bytes, at + 0x38)?,
            });
        }
        Ok(Self {
            bytes,
            source_pdt,
            cells,
            stream_base,
            scene_base,
            stream_count,
            scene_count,
        })
    }

    pub fn scene_stream_count(&self, scene: usize) -> Result<usize> {
        self.scene_at(scene, 4).map(|value| value as usize)
    }

    pub fn stream_frames(&self, scene: usize, stream: usize) -> Result<Vec<AnimationCell>> {
        if stream >= self.scene_stream_count(scene)? {
            bail!("avg32: ANM stream {stream} is outside scene {scene}");
        }
        let stream_index = self.scene_at(scene, 8 + stream * 4)? as usize;
        if stream_index >= self.stream_count {
            bail!("avg32: ANM stream table index {stream_index} is out of range");
        }
        let at = self.stream_base + stream_index * 0x68;
        let frame_count = le_u32(&self.bytes, at + 4)? as usize;
        let mut frames = Vec::with_capacity(frame_count);
        for frame in 0..frame_count {
            let cell = le_u32(&self.bytes, at + 8 + frame * 4)? as usize;
            frames.push(
                *self.cells.get(cell).ok_or_else(|| {
                    anyhow::anyhow!("avg32: ANM cell index {cell} is out of range")
                })?,
            );
        }
        Ok(frames)
    }

    fn scene_at(&self, scene: usize, offset: usize) -> Result<u32> {
        if scene >= self.scene_count {
            bail!("avg32: ANM scene {scene} is out of range");
        }
        le_u32(&self.bytes, self.scene_base + scene * 0x78 + offset)
    }
}

fn c_string(bytes: &[u8], at: usize) -> Result<String> {
    let tail = bytes
        .get(at..)
        .ok_or_else(|| anyhow::anyhow!("avg32: truncated ANM32 source PDT"))?;
    let length = tail
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| anyhow::anyhow!("avg32: unterminated ANM32 source PDT"))?;
    Ok(crate::nls::decode_name(&tail[..length]))
}

fn le_u32(bytes: &[u8], at: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(
        bytes
            .get(at..at + 4)
            .ok_or_else(|| anyhow::anyhow!("avg32: truncated ANM32 u32 at {at:#x}"))?
            .try_into()
            .expect("four bytes"),
    ))
}

fn le_i32(bytes: &[u8], at: usize) -> Result<i32> {
    Ok(le_u32(bytes, at)? as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a synthetic ANM32 buffer with 2 cells, 2 single-frame streams,
    /// and one scene referencing both streams in order — enough to exercise
    /// multi-stream playback, which the engine
    /// advances through
    /// in full before looping or finishing.
    fn sample_anm32() -> Vec<u8> {
        const CELL_SIZE: usize = 0x60;
        const STREAM_SIZE: usize = 0x68;
        const SCENE_SIZE: usize = 0x78;
        let cell_count = 2usize;
        let stream_count = 2usize;
        let scene_count = 1usize;
        let stream_base = 0xb8 + cell_count * CELL_SIZE;
        let scene_base = stream_base + stream_count * STREAM_SIZE;
        let total = scene_base + scene_count * SCENE_SIZE;
        let mut bytes = vec![0u8; total];
        bytes[0..5].copy_from_slice(b"ANM32");
        bytes[6..10].copy_from_slice(&0x0100_0000u32.to_le_bytes());
        bytes[0x1c..0x1c + 4].copy_from_slice(b"BG\0\0");
        bytes[0x8c..0x90].copy_from_slice(&(cell_count as u32).to_le_bytes());
        bytes[0x90..0x94].copy_from_slice(&(stream_count as u32).to_le_bytes());
        bytes[0x94..0x98].copy_from_slice(&(scene_count as u32).to_le_bytes());

        let put_cell = |bytes: &mut [u8], index: usize, wait: u32| {
            let at = 0xb8 + index * CELL_SIZE;
            bytes[at..at + 4].copy_from_slice(&(index as i32).to_le_bytes());
            bytes[at + 0x38..at + 0x3c].copy_from_slice(&wait.to_le_bytes());
        };
        put_cell(&mut bytes, 0, 1_000);
        put_cell(&mut bytes, 1, 2_000);

        let put_stream = |bytes: &mut [u8], index: usize, cell: u32| {
            let at = stream_base + index * STREAM_SIZE;
            bytes[at + 4..at + 8].copy_from_slice(&1u32.to_le_bytes()); // frame_count
            bytes[at + 8..at + 12].copy_from_slice(&cell.to_le_bytes());
        };
        put_stream(&mut bytes, 0, 0);
        put_stream(&mut bytes, 1, 1);

        let scene_at = scene_base;
        bytes[scene_at + 4..scene_at + 8].copy_from_slice(&(stream_count as u32).to_le_bytes());
        bytes[scene_at + 8..scene_at + 12].copy_from_slice(&0u32.to_le_bytes());
        bytes[scene_at + 12..scene_at + 16].copy_from_slice(&1u32.to_le_bytes());
        bytes
    }

    #[test]
    fn parses_source_pdt_and_scene_stream_layout() {
        let animation = Avg32Animation::parse(sample_anm32()).unwrap();
        assert_eq!(animation.source_pdt, "BG");
        assert_eq!(animation.scene_stream_count(0).unwrap(), 2);
    }

    #[test]
    fn each_stream_resolves_its_own_single_frame() {
        let animation = Avg32Animation::parse(sample_anm32()).unwrap();
        let first = animation.stream_frames(0, 0).unwrap();
        let second = animation.stream_frames(0, 1).unwrap();
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].wait_microseconds, 1_000);
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].wait_microseconds, 2_000);
    }

    #[test]
    fn stream_beyond_the_scene_is_rejected() {
        let animation = Avg32Animation::parse(sample_anm32()).unwrap();
        assert!(animation.stream_frames(0, 2).is_err());
    }
}
