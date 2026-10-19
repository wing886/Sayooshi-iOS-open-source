//! NWA music decoded while it plays: a Kira streaming decoder over an
//! `NwaReader`, so a track costs one compressed unit in memory instead of
//! its whole PCM (up to ~50 MB for a long BGM).

use std::path::Path;

use kira::Frame;
use kira::sound::FromFileError;
use kira::sound::streaming::Decoder;
use siglus_assets::nwa::NwaReader;

/// Frames decoded per request from the streaming thread.
const CHUNK_FRAMES: u32 = 4096;

pub struct NwaDecoder {
    reader: NwaReader,
    channels: u16,
    bits: u16,
    sample_rate: u32,
    frames: usize,
}

fn io_error(error: anyhow::Error) -> FromFileError {
    FromFileError::IoError(std::io::Error::other(format!("{error:#}")))
}

impl NwaDecoder {
    pub fn open(path: &Path) -> Result<Self, FromFileError> {
        let reader = NwaReader::open(path).map_err(io_error)?;
        let header = reader.header();
        let (channels, bits) = (header.channels, header.bits_per_sample);
        if !matches!(channels, 1 | 2) {
            return Err(FromFileError::UnsupportedChannelConfiguration);
        }
        if !matches!(bits, 8 | 16) {
            return Err(io_error(anyhow::anyhow!("NWA: {bits}-bit samples")));
        }
        Ok(Self {
            channels,
            bits,
            sample_rate: header.samples_per_sec.max(1),
            frames: header.frame_count() as usize,
            reader,
        })
    }
}

impl Decoder for NwaDecoder {
    type Error = FromFileError;

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn num_frames(&self) -> usize {
        self.frames
    }

    fn decode(&mut self) -> Result<Vec<Frame>, Self::Error> {
        let bytes = self.reader.read_samples(CHUNK_FRAMES).map_err(io_error)?;
        let sample = |i: usize| -> f32 {
            if self.bits == 16 {
                f32::from(i16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]])) / 32768.0
            } else {
                (f32::from(bytes[i]) - 128.0) / 128.0
            }
        };
        let width = usize::from(self.bits / 8) * usize::from(self.channels);
        let count = bytes.len() / width;
        let frames = (0..count)
            .map(|f| {
                if self.channels == 2 {
                    Frame::new(sample(f * 2), sample(f * 2 + 1))
                } else {
                    Frame::from_mono(sample(f))
                }
            })
            .collect();
        Ok(frames)
    }

    fn seek(&mut self, index: usize) -> Result<usize, Self::Error> {
        let index = index.min(self.frames);
        self.reader.set_read_sample_pos(index as u32);
        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Streams a real NWA (`SIGLUS_TEST_NWA`, or RewriteHF's smallest BGM
    /// when present) and compares it with the whole-file decode.
    #[test]
    fn streamed_samples_match_the_full_decode() {
        let path = std::env::var_os("SIGLUS_TEST_NWA")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| "/Users/xmoe/Documents/RewriteHF/bgm/BGM001.nwa".into());
        if !path.is_file() {
            return;
        }
        let mut decoder = NwaDecoder::open(&path).unwrap();
        let wav = NwaReader::open(&path).unwrap().to_wav_bytes().unwrap();
        let pcm = &wav[44..];
        let mut streamed = Vec::new();
        loop {
            let frames = decoder.decode().unwrap();
            if frames.is_empty() {
                break;
            }
            streamed.extend(frames);
        }
        assert_eq!(streamed.len(), decoder.num_frames());
        for (i, frame) in streamed.iter().enumerate().step_by(997) {
            let left = i16::from_le_bytes([pcm[i * 4], pcm[i * 4 + 1]]);
            assert_eq!(frame.left, f32::from(left) / 32768.0, "frame {i}");
        }
        // Seeking lands where asked.
        let middle = decoder.num_frames() / 2;
        assert_eq!(decoder.seek(middle).unwrap(), middle);
        let frame = decoder.decode().unwrap()[0];
        let left = i16::from_le_bytes([pcm[middle * 4], pcm[middle * 4 + 1]]);
        assert_eq!(frame.left, f32::from(left) / 32768.0);
    }
}
