//! AVG32 FMV playback.
//!
//! AVG32 titles ship cutscenes as Video-for-Windows AVI files (Cinepak video,
//! optionally uncompressed PCM audio). This module owns just enough of
//! RIFF/AVI to walk one linear play-through of a `movi` list -- no seeking,
//! no OpenDML/index chunks, no `rec ` interleave beyond flattening it -- since
//! AVG32 always plays a movie start-to-finish. Video frames are decoded with
//! `oxideav-cinepak`; presentation reuses the same 640x480 display buffer the
//! bytecode-driven graphics already render into, so a frontend needs no
//! separate video code path.

use std::time::Duration;
#[cfg(not(target_os = "horizon"))]
use web_time::Instant;

#[cfg(not(target_os = "horizon"))]
use anyhow::Context;
use anyhow::{Result, bail};
#[cfg(not(target_os = "horizon"))]
use oxideav_cinepak::{CinepakDecoder, CinepakFrame, CinepakPixelFormat};

#[cfg(not(target_os = "horizon"))]
use crate::buffer::PdtBuffer;
use crate::pdtmgr::{PdtManager, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamKind {
    Video,
    Audio,
    Other,
}

/// `(sample_rate, channels, bits_per_sample)` for a recognised PCM `auds` stream.
type AudioFormat = Option<(u32, u16, u16)>;
/// One `strl`'s kind plus its audio format, if it is a supported PCM `auds` stream.
type StreamInfo = (StreamKind, AudioFormat);

/// A decoded-enough AVI: raw per-frame Cinepak chunks plus, if present, the
/// audio track as already-interleaved little-endian PCM ready to wrap in a
/// WAV header.
#[derive(Debug, Clone, Default)]
pub struct AviMovie {
    pub frame_interval: Duration,
    pub video_frames: Vec<Vec<u8>>,
    pub audio: Option<AviAudio>,
}

#[derive(Debug, Clone)]
pub struct AviAudio {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub pcm: Vec<u8>,
}

impl AviMovie {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"AVI " {
            bail!("avg32: not a RIFF/AVI file");
        }
        let mut micro_sec_per_frame = 66_667u32; // ~15fps fallback.
        let mut streams: Vec<StreamInfo> = Vec::new();
        let mut video_frames = Vec::new();
        let mut audio_pcm = Vec::new();

        let mut pos = 12usize;
        while let Some((id, body, end)) = next_chunk(bytes, pos) {
            if id == b"LIST" && bytes.get(body..body + 4) == Some(b"hdrl") {
                let (rate, found) = parse_hdrl(&bytes[body + 4..end])?;
                micro_sec_per_frame = rate;
                streams = found;
            } else if id == b"LIST" && bytes.get(body..body + 4) == Some(b"movi") {
                parse_movi(
                    &bytes[body + 4..end],
                    &streams,
                    &mut video_frames,
                    &mut audio_pcm,
                )?;
            }
            pos = end + (end - body) % 2;
        }

        let audio = streams
            .iter()
            .find_map(|(kind, format)| {
                matches!(kind, StreamKind::Audio)
                    .then_some(*format)
                    .flatten()
            })
            .filter(|_| !audio_pcm.is_empty())
            .map(|(sample_rate, channels, bits_per_sample)| AviAudio {
                sample_rate,
                channels,
                bits_per_sample,
                pcm: audio_pcm,
            });

        Ok(Self {
            frame_interval: Duration::from_micros(u64::from(micro_sec_per_frame.max(1))),
            video_frames,
            audio,
        })
    }
}

/// Reads one RIFF chunk header at `pos` and returns `(id, body_start, body_end)`.
fn next_chunk(bytes: &[u8], pos: usize) -> Option<(&[u8], usize, usize)> {
    if pos + 8 > bytes.len() {
        return None;
    }
    let id = &bytes[pos..pos + 4];
    let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().expect("four bytes")) as usize;
    let body = pos + 8;
    let end = body.saturating_add(size).min(bytes.len());
    Some((id, body, end))
}

fn parse_hdrl(data: &[u8]) -> Result<(u32, Vec<StreamInfo>)> {
    let mut micro_sec_per_frame = 66_667u32;
    let mut streams = Vec::new();
    let mut pos = 0usize;
    while let Some((id, body, end)) = next_chunk(data, pos) {
        if id == b"avih" && end - body >= 4 {
            micro_sec_per_frame =
                u32::from_le_bytes(data[body..body + 4].try_into().expect("four bytes"));
        } else if id == b"LIST" && data.get(body..body + 4) == Some(b"strl") {
            streams.push(parse_strl(&data[body + 4..end])?);
        }
        pos = end + (end - body) % 2;
    }
    Ok((micro_sec_per_frame, streams))
}

fn parse_strl(data: &[u8]) -> Result<StreamInfo> {
    let mut kind = StreamKind::Other;
    let mut audio_format = None;
    let mut pos = 0usize;
    while let Some((id, body, end)) = next_chunk(data, pos) {
        if id == b"strh" && end - body >= 4 {
            kind = match &data[body..body + 4] {
                b"vids" => StreamKind::Video,
                b"auds" => StreamKind::Audio,
                _ => StreamKind::Other,
            };
        } else if id == b"strf" && kind == StreamKind::Audio && end - body >= 16 {
            let format_tag =
                u16::from_le_bytes(data[body..body + 2].try_into().expect("two bytes"));
            let channels =
                u16::from_le_bytes(data[body + 2..body + 4].try_into().expect("two bytes"));
            let sample_rate =
                u32::from_le_bytes(data[body + 4..body + 8].try_into().expect("four bytes"));
            let bits_per_sample =
                u16::from_le_bytes(data[body + 14..body + 16].try_into().expect("two bytes"));
            // WAVE_FORMAT_PCM only; compressed audio tracks are left silent
            // rather than misdecoded.
            if format_tag == 1 {
                audio_format = Some((sample_rate, channels, bits_per_sample));
            }
        }
        pos = end + (end - body) % 2;
    }
    Ok((kind, audio_format))
}

fn parse_movi(
    data: &[u8],
    streams: &[StreamInfo],
    video_frames: &mut Vec<Vec<u8>>,
    audio_pcm: &mut Vec<u8>,
) -> Result<()> {
    let mut pos = 0usize;
    while let Some((id, body, end)) = next_chunk(data, pos) {
        if id == b"LIST" && data.get(body..body + 4) == Some(b"rec ") {
            parse_movi(&data[body + 4..end], streams, video_frames, audio_pcm)?;
        } else if let Some(index) = stream_index(id) {
            match &id[2..4] {
                b"dc" | b"db" if matches!(streams.get(index), Some((StreamKind::Video, _))) => {
                    video_frames.push(data[body..end].to_vec());
                }
                b"wb" if matches!(streams.get(index), Some((StreamKind::Audio, _))) => {
                    audio_pcm.extend_from_slice(&data[body..end]);
                }
                _ => {}
            }
        }
        pos = end + (end - body) % 2;
    }
    Ok(())
}

/// AVI stream-data chunk IDs are `NNxx` where `NN` are ASCII decimal digits
/// naming the stream (`strl` index) and `xx` names the payload kind
/// (`dc`/`db` video, `wb` audio).
fn stream_index(id: &[u8]) -> Option<usize> {
    let tens = id[0].checked_sub(b'0').filter(|d| *d <= 9)?;
    let ones = id[1].checked_sub(b'0').filter(|d| *d <= 9)?;
    Some(usize::from(tens) * 10 + usize::from(ones))
}

/// Plays an [`AviMovie`]'s video track into a rectangle of display buffer
/// 0 (`0x0e:50..55` stretch the movie to the requested area). Has no
/// fields (and [`Self::start`] always returns `None`) on `horizon`, which
/// lacks the `oxideav-cinepak` dependency.
pub struct MoviePlayer {
    #[cfg(not(target_os = "horizon"))]
    frames: Vec<Vec<u8>>,
    #[cfg(not(target_os = "horizon"))]
    decoder: CinepakDecoder,
    #[cfg(not(target_os = "horizon"))]
    next_frame: usize,
    #[cfg(not(target_os = "horizon"))]
    interval: Duration,
    #[cfg(not(target_os = "horizon"))]
    due: Instant,
    #[cfg(not(target_os = "horizon"))]
    looped: bool,
    #[cfg(not(target_os = "horizon"))]
    active: bool,
    #[cfg(not(target_os = "horizon"))]
    rect: Rect,
}

impl std::fmt::Debug for MoviePlayer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MoviePlayer")
            .finish_non_exhaustive()
    }
}

impl MoviePlayer {
    pub fn start(
        movie: &AviMovie,
        looped: bool,
        rect: Rect,
        gfx: &mut PdtManager,
    ) -> Result<Option<Self>> {
        #[cfg(not(target_os = "horizon"))]
        {
            if movie.video_frames.is_empty() {
                return Ok(None);
            }
            let rect = if rect.x2 <= rect.x1 || rect.y2 <= rect.y1 {
                Rect::full()
            } else {
                rect
            };
            let mut player = Self {
                frames: movie.video_frames.clone(),
                decoder: CinepakDecoder::new(),
                next_frame: 0,
                interval: movie.frame_interval,
                due: Instant::now(),
                looped,
                active: true,
                rect,
            };
            player.show_next(gfx)?;
            player.due = Instant::now() + player.interval;
            Ok(Some(player))
        }
        #[cfg(target_os = "horizon")]
        {
            let _ = (movie, looped, rect, gfx);
            Ok(None)
        }
    }

    pub fn active(&self) -> bool {
        #[cfg(not(target_os = "horizon"))]
        {
            self.active
        }
        #[cfg(target_os = "horizon")]
        {
            false
        }
    }

    pub fn stop(&mut self) {
        #[cfg(not(target_os = "horizon"))]
        {
            self.active = false;
        }
    }

    pub fn tick(&mut self, gfx: &mut PdtManager) -> Result<()> {
        #[cfg(not(target_os = "horizon"))]
        {
            if !self.active || Instant::now() < self.due {
                return Ok(());
            }
            self.show_next(gfx)?;
            self.due += self.interval;
            if self.due < Instant::now() {
                self.due = Instant::now() + self.interval;
            }
        }
        #[cfg(target_os = "horizon")]
        {
            let _ = gfx;
        }
        Ok(())
    }

    #[cfg(not(target_os = "horizon"))]
    fn show_next(&mut self, gfx: &mut PdtManager) -> Result<()> {
        if self.next_frame >= self.frames.len() {
            if self.looped {
                self.next_frame = 0;
                self.decoder.reset();
            } else {
                self.active = false;
                return Ok(());
            }
        }
        let frame = self
            .decoder
            .decode_frame(&self.frames[self.next_frame], None)
            .map_err(|err| anyhow::anyhow!("avg32: failed to decode FMV frame: {err}"))
            .with_context(|| format!("FMV frame {}", self.next_frame))?;
        let rgba = expand_to_rgba(&frame);
        let mut buffer = PdtBuffer::new(frame.width as usize, frame.height as usize);
        for (index, pixel) in rgba.chunks_exact(4).enumerate() {
            buffer.set_pixel(index, [pixel[0], pixel[1], pixel[2]]);
        }
        gfx.stretch_from(
            &buffer,
            Rect::new(0, 0, frame.width as i32 - 1, frame.height as i32 - 1),
            self.rect,
            0,
        );
        self.next_frame += 1;
        Ok(())
    }
}

#[cfg(not(target_os = "horizon"))]
fn expand_to_rgba(frame: &CinepakFrame) -> Vec<u8> {
    let pixels = frame.pixels();
    let count = (frame.width as usize) * (frame.height as usize);
    let mut rgba = vec![0u8; count * 4];
    match frame.pixel_format {
        CinepakPixelFormat::Rgb24 => {
            for index in 0..count {
                rgba[index * 4] = pixels[index * 3];
                rgba[index * 4 + 1] = pixels[index * 3 + 1];
                rgba[index * 4 + 2] = pixels[index * 3 + 2];
                rgba[index * 4 + 3] = 255;
            }
        }
        CinepakPixelFormat::Gray8 => {
            for index in 0..count {
                let luminance = pixels[index];
                rgba[index * 4] = luminance;
                rgba[index * 4 + 1] = luminance;
                rgba[index * 4 + 2] = luminance;
                rgba[index * 4 + 3] = 255;
            }
        }
    }
    rgba
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(id: &[u8; 4], body: Vec<u8>) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + body.len() + 1);
        out.extend_from_slice(id);
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        if body.len() % 2 == 1 {
            out.push(0);
        }
        out
    }

    fn list(list_type: &[u8; 4], body: Vec<u8>) -> Vec<u8> {
        let mut payload = list_type.to_vec();
        payload.extend_from_slice(&body);
        chunk(b"LIST", payload)
    }

    fn strl(fcc_type: &[u8; 4], strf: Vec<u8>) -> Vec<u8> {
        list(
            b"strl",
            [chunk(b"strh", fcc_type.to_vec()), chunk(b"strf", strf)].concat(),
        )
    }

    fn synthetic_avi() -> Vec<u8> {
        let audio_strf = {
            let mut bytes = vec![0u8; 16];
            bytes[0..2].copy_from_slice(&1u16.to_le_bytes()); // WAVE_FORMAT_PCM
            bytes[2..4].copy_from_slice(&2u16.to_le_bytes()); // stereo
            bytes[4..8].copy_from_slice(&22_050u32.to_le_bytes());
            bytes[14..16].copy_from_slice(&16u16.to_le_bytes());
            bytes
        };
        let hdrl = list(
            b"hdrl",
            [
                chunk(b"avih", 66_667u32.to_le_bytes().to_vec()),
                strl(b"vids", vec![0; 4]),
                strl(b"auds", audio_strf),
            ]
            .concat(),
        );
        let movi = list(
            b"movi",
            [
                chunk(b"00dc", vec![1, 2, 3, 4]),
                chunk(b"01wb", vec![5, 6, 7, 8]),
            ]
            .concat(),
        );
        let mut body = b"AVI ".to_vec();
        body.extend_from_slice(&hdrl);
        body.extend_from_slice(&movi);
        chunk(b"RIFF", body)
    }

    #[test]
    fn demuxes_synthetic_container() {
        let movie = AviMovie::parse(&synthetic_avi()).unwrap();
        assert_eq!(movie.frame_interval, Duration::from_micros(66_667));
        assert_eq!(movie.video_frames, vec![vec![1, 2, 3, 4]]);
        let audio = movie.audio.expect("synthetic container carries PCM audio");
        assert_eq!(audio.sample_rate, 22_050);
        assert_eq!(audio.channels, 2);
        assert_eq!(audio.bits_per_sample, 16);
        assert_eq!(audio.pcm, vec![5, 6, 7, 8]);
    }

    #[test]
    fn rejects_non_riff_data() {
        assert!(AviMovie::parse(b"not a riff file").is_err());
    }
}
