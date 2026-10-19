//! Audio: BGM, PCM (`wav*`) channels, interface sounds and voices.
//!
//! Every file is decoded to 16-bit PCM here (NWA, Ogg/Vorbis, OWP, RIFF
//! WAVE, and the three voice archive formats: `KOEPAC`, NWK and OVK) and
//! handed to Kira as an in-memory WAVE. Whether something is still playing
//! is derived from the engine clock and the clip's length, so waits such as
//! `bgmWait` and `koeWait` behave identically with or without an audio
//! device (and deterministically under the virtual clock).

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::gameexe::Gameexe;
use crate::resource::{Kind, Resources};
use crate::settings::{Settings, channel};

#[cfg(not(target_os = "horizon"))]
use kira::manager::backend::DefaultBackend;
#[cfg(not(target_os = "horizon"))]
use kira::manager::{AudioManager, AudioManagerSettings};
#[cfg(not(target_os = "horizon"))]
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};
#[cfg(not(target_os = "horizon"))]
use kira::tween::Tween;

/// Decoded audio.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pcm {
    pub rate: u32,
    pub channels: u16,
    /// Interleaved samples.
    pub samples: Vec<i16>,
    /// Loop start in frames.
    pub loop_start: Option<usize>,
}

impl Pcm {
    pub fn frames(&self) -> usize {
        self.samples.len() / usize::from(self.channels.max(1))
    }

    pub fn duration_ms(&self) -> u64 {
        (self.frames() as u64 * 1000)
            .checked_div(u64::from(self.rate))
            .unwrap_or(0)
    }

    /// Keeps frames `from..to`.
    pub fn trim(&mut self, from: usize, to: usize) {
        let ch = usize::from(self.channels.max(1));
        let to = to.min(self.frames());
        let from = from.min(to);
        self.samples = self.samples[from * ch..to * ch].to_vec();
    }

    pub fn to_wav(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.samples.len() * 2);
        for sample in &self.samples {
            data.extend_from_slice(&sample.to_le_bytes());
        }
        let channels = self.channels.max(1);
        let block = channels * 2;
        let mut wav = Vec::with_capacity(44 + data.len());
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&((36 + data.len()) as u32).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&channels.to_le_bytes());
        wav.extend_from_slice(&self.rate.to_le_bytes());
        wav.extend_from_slice(&(self.rate * u32::from(block)).to_le_bytes());
        wav.extend_from_slice(&block.to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&(data.len() as u32).to_le_bytes());
        wav.extend_from_slice(&data);
        wav
    }
}

// ---- decoders ---------------------------------------------------------------

fn le_u16(bytes: &[u8], at: usize) -> Result<u16> {
    let b = bytes
        .get(at..at + 2)
        .context("reallive: truncated audio data")?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

fn le_u32(bytes: &[u8], at: usize) -> Result<u32> {
    let b = bytes
        .get(at..at + 4)
        .context("reallive: truncated audio data")?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// RIFF WAVE (PCM 8/16-bit).
pub fn decode_wav(bytes: &[u8]) -> Result<Pcm> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        bail!("reallive: not a RIFF WAVE file");
    }
    let mut at = 12;
    let mut format = None;
    while at + 8 <= bytes.len() {
        let id = &bytes[at..at + 4];
        let size = le_u32(bytes, at + 4)? as usize;
        let body = at + 8;
        let end = body.saturating_add(size).min(bytes.len());
        if id == b"fmt " {
            let channels = le_u16(bytes, body + 2)?;
            let rate = le_u32(bytes, body + 4)?;
            let bits = le_u16(bytes, body + 14)?;
            format = Some((channels, rate, bits));
        } else if id == b"data" {
            let (channels, rate, bits) = format.context("reallive: WAVE data before fmt")?;
            let data = &bytes[body..end];
            let samples = match bits {
                8 => data.iter().map(|&b| (i16::from(b) - 128) << 8).collect(),
                16 => data
                    .chunks_exact(2)
                    .map(|p| i16::from_le_bytes([p[0], p[1]]))
                    .collect(),
                other => bail!("reallive: {other}-bit WAVE files are not supported"),
            };
            return Ok(Pcm {
                rate,
                channels,
                samples,
                loop_start: None,
            });
        }
        at = body + size + (size & 1);
    }
    bail!("reallive: WAVE file has no data chunk")
}

pub fn decode_nwa(bytes: Vec<u8>) -> Result<Pcm> {
    let mut reader = siglus_assets::nwa::NwaReader::open_from_bytes(bytes)?;
    let header = reader.header().clone();
    let frames = header.frame_count();
    let data = reader.read_samples(frames)?;
    let samples = match header.bits_per_sample {
        8 => data.iter().map(|&b| (i16::from(b) - 128) << 8).collect(),
        _ => data
            .chunks_exact(2)
            .map(|p| i16::from_le_bytes([p[0], p[1]]))
            .collect(),
    };
    Ok(Pcm {
        rate: header.samples_per_sec,
        channels: header.channels,
        samples,
        loop_start: None,
    })
}

pub fn decode_ogg(bytes: &[u8]) -> Result<Pcm> {
    let pcm = siglus_assets::vorbis::decode_ogg_vorbis_bytes(bytes)?;
    Ok(Pcm {
        rate: pcm.sample_rate,
        channels: pcm.channels,
        samples: pcm.samples,
        loop_start: None,
    })
}

/// Any supported file, by content.
pub fn decode(bytes: Vec<u8>) -> Result<Pcm> {
    if bytes.starts_with(b"RIFF") {
        return decode_wav(&bytes);
    }
    if bytes.starts_with(b"OggS") {
        return decode_ogg(&bytes);
    }
    // OWP: Ogg XORed with 0x39.
    if bytes.len() > 4 && bytes[..4].iter().zip(b"OggS").all(|(a, b)| a ^ 0x39 == *b) {
        let plain: Vec<u8> = bytes.iter().map(|b| b ^ 0x39).collect();
        return decode_ogg(&plain);
    }
    decode_nwa(bytes)
}

/// Expansion of 8-bit `KOEPAC` samples to 16 bits.
fn koe_sample(byte: u8) -> i16 {
    // A signed square law, saturated at both ends.
    match byte {
        0 => i16::MIN,
        128 => 0,
        255 => i16::MAX,
        _ => {
            let v = i32::from(byte) - 128;
            (v.signum() * (2 * v * v - 1)) as i16
        }
    }
}

/// DPCM step for a code (0, -1, +1, -2, +2, ...).
fn koe_delta(code: u8) -> u8 {
    if code & 1 == 0 {
        code >> 1
    } else {
        0xff - (code >> 1)
    }
}

/// Decodes one `KOEPAC` entry: a table of block sizes, then the blocks.
pub fn decode_koepac(bytes: &[u8], offset: usize, blocks: usize, rate: u32) -> Result<Pcm> {
    let mut sizes = Vec::with_capacity(blocks);
    for i in 0..blocks {
        sizes.push(usize::from(le_u16(bytes, offset + i * 2)?));
    }
    let mut at = offset + blocks * 2;
    let mut samples = Vec::with_capacity(blocks * 0x400);
    for size in sizes {
        let block = bytes
            .get(at..at + size)
            .context("reallive: truncated KOE block")?;
        match size {
            0 => samples.extend(std::iter::repeat_n(0, 0x400)),
            0x400 => samples.extend(block.iter().map(|&b| koe_sample(b))),
            _ => {
                let mut d = 0u8;
                let mut produced = 0;
                let mut j = 0;
                while j < size && produced < 0x400 {
                    let mut s = block[j];
                    if (s.wrapping_add(1)) & 0x0f != 0 {
                        d = d.wrapping_sub(koe_delta(s & 0x0f));
                    } else {
                        let low = (s >> 4) & 0x0f;
                        j += 1;
                        s = block.get(j).copied().unwrap_or(0);
                        d = d.wrapping_sub(koe_delta(low | ((s << 4) & 0xf0)));
                    }
                    samples.push(koe_sample(d));
                    s >>= 4;
                    if (s.wrapping_add(1)) & 0x0f != 0 {
                        d = d.wrapping_sub(koe_delta(s & 0x0f));
                    } else {
                        j += 1;
                        d = d.wrapping_sub(koe_delta(block.get(j).copied().unwrap_or(0)));
                    }
                    samples.push(koe_sample(d));
                    produced += 2;
                    j += 1;
                }
                samples.resize(samples.len() + (0x400 - produced.min(0x400)), 0);
            }
        }
        at += size;
    }
    Ok(Pcm {
        rate,
        channels: 1,
        samples,
        loop_start: None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchiveKind {
    Koepac { rate: u32 },
    Nwk,
    Ovk,
}

/// A voice archive `zNNNN.{koe,nwk,ovk}`: sample number → (offset, length).
#[derive(Debug)]
struct VoiceArchive {
    path: PathBuf,
    kind: ArchiveKind,
    entries: HashMap<i32, (usize, usize)>,
}

impl VoiceArchive {
    fn open(path: &Path) -> Result<Self> {
        let bytes =
            game_fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();
        let mut entries = HashMap::new();
        let kind = if bytes.starts_with(b"KOEPAC") {
            let count = le_u32(&bytes, 0x10)? as usize;
            let rate = match le_u32(&bytes, 0x18)? {
                0 => 22_050,
                rate => rate,
            };
            for i in 0..count {
                let at = 0x20 + i * 8;
                let number = i32::from(le_u16(&bytes, at)?);
                let blocks = usize::from(le_u16(&bytes, at + 2)?);
                let offset = le_u32(&bytes, at + 4)? as usize;
                entries.insert(number, (offset, blocks));
            }
            ArchiveKind::Koepac { rate }
        } else {
            let (kind, entry) = if extension == "ovk" {
                (ArchiveKind::Ovk, 16)
            } else {
                (ArchiveKind::Nwk, 12)
            };
            let count = le_u32(&bytes, 0)? as usize;
            for i in 0..count {
                let at = 4 + i * entry;
                let length = le_u32(&bytes, at)? as usize;
                let offset = le_u32(&bytes, at + 4)? as usize;
                let number = le_u32(&bytes, at + 8)? as i32;
                entries.insert(number, (offset, length));
            }
            kind
        };
        Ok(Self {
            path: path.to_path_buf(),
            kind,
            entries,
        })
    }

    fn decode(&self, sample: i32) -> Result<Pcm> {
        let &(offset, length) = self.entries.get(&sample).with_context(|| {
            format!("reallive: voice {sample} is not in {}", self.path.display())
        })?;
        let bytes = game_fs::read(&self.path)?;
        match self.kind {
            ArchiveKind::Koepac { rate } => decode_koepac(&bytes, offset, length, rate),
            ArchiveKind::Nwk | ArchiveKind::Ovk => {
                let data = bytes
                    .get(offset..offset + length)
                    .context("reallive: voice entry outside its archive")?
                    .to_vec();
                decode(data)
            }
        }
    }
}

// ---- playback ----------------------------------------------------------------

#[cfg(not(target_os = "horizon"))]
type Handle = StaticSoundHandle;
#[cfg(target_os = "horizon")]
type Handle = ();

/// A sound that was started.
struct Voice {
    handle: Option<Handle>,
    started: u64,
    duration: u64,
    looped: bool,
    /// Set by a fade-out: when it ends.
    ends_at: Option<u64>,
    paused_at: Option<u64>,
}

impl std::fmt::Debug for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Voice")
            .field("started", &self.started)
            .field("duration", &self.duration)
            .field("looped", &self.looped)
            .finish_non_exhaustive()
    }
}

/// Where a track starts (`DEBUG_MPLAY_*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartAt {
    Ms(u64),
    Frame(usize),
}

impl Voice {
    /// Switches looping on or off while it plays; a sound that stops
    /// looping ends at the end of its current pass.
    fn set_looped(&mut self, looped: bool, now: u64) {
        if self.looped == looped {
            return;
        }
        #[cfg(not(target_os = "horizon"))]
        if let Some(handle) = self.handle.as_mut() {
            if looped {
                handle.set_loop_region(0.0..);
            } else {
                handle.set_loop_region(None);
            }
        }
        if !looped && self.duration > 0 {
            let played = now.saturating_sub(self.started);
            self.started = now - played % self.duration;
        }
        self.looped = looped;
    }

    fn playing(&self, now: u64) -> bool {
        if self.paused_at.is_some() {
            return true;
        }
        if self.ends_at.is_some_and(|end| now >= end) {
            return false;
        }
        self.looped || now < self.started + self.duration
    }
}

/// `#DSTRACK`: a named music track.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    pub file: String,
    pub from: i32,
    pub to: i32,
    pub looped_from: i32,
}

/// A volume moving towards a target.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Ramp {
    from: i32,
    to: i32,
    start: u64,
    duration: u64,
}

impl Ramp {
    fn fixed(value: i32) -> Self {
        Self {
            from: value,
            to: value,
            start: 0,
            duration: 0,
        }
    }

    fn value(&self, now: u64) -> i32 {
        if self.duration == 0 || now >= self.start + self.duration {
            return self.to;
        }
        let t = (now - self.start) as f64 / self.duration as f64;
        (f64::from(self.from) + f64::from(self.to - self.from) * t).round() as i32
    }

    fn retarget(&mut self, now: u64, to: i32, duration: u64) {
        *self = Self {
            from: self.value(now),
            to: to.clamp(0, 255),
            start: now,
            duration,
        };
    }
}

pub const WAV_CHANNELS: usize = 16;

pub struct Sound {
    #[cfg(not(target_os = "horizon"))]
    manager: Option<AudioManager<DefaultBackend>>,
    tracks: HashMap<String, Track>,
    /// `#SE.nnn`: file and channel.
    se_table: BTreeMap<i32, (String, i32)>,
    /// `#KOEONOFF`: voice character → `UseKoe` switch.
    pub koe_characters: HashMap<i32, i32>,
    bgm: Option<Voice>,
    pub bgm_name: String,
    bgm_looped: bool,
    /// A track waiting for the current one to fade out.
    queued_bgm: Option<(String, bool, u64, u64)>,
    wav: [Option<Voice>; WAV_CHANNELS],
    /// What each channel last played, and whether looped (`wavRewind`).
    wav_names: [(String, bool); WAV_CHANNELS],
    koe: Option<Voice>,
    se: Vec<Voice>,
    movie: Option<Voice>,
    /// Script volumes (`bgmSetVolume` etc.).
    bgm_volume: Ramp,
    wav_volume: [Ramp; WAV_CHANNELS],
    koe_volume: Ramp,
    /// `SEVOLSET` and friends.
    se_volume: Ramp,
    voices: HashMap<i32, VoiceArchive>,
    cache: HashMap<(Kind, String), Pcm>,
    /// Last applied device volumes, to avoid redundant updates.
    applied: Vec<f64>,
}

impl std::fmt::Debug for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sound")
            .field("bgm", &self.bgm_name)
            .finish_non_exhaustive()
    }
}

impl Sound {
    pub fn new(gameexe: &Gameexe, device: bool) -> Self {
        let mut tracks = HashMap::new();
        for entry in gameexe.filter("DSTRACK") {
            let ints = entry.ints();
            let strs = entry.strs();
            if let (Some(file), Some(name)) = (strs.first(), strs.get(1).or(strs.first())) {
                tracks.insert(
                    name.to_lowercase(),
                    Track {
                        file: (*file).to_owned(),
                        from: ints.first().copied().unwrap_or(0),
                        to: ints.get(1).copied().unwrap_or(0),
                        looped_from: ints.get(2).copied().unwrap_or(0),
                    },
                );
            }
        }
        let mut se_table = BTreeMap::new();
        for entry in gameexe.filter("SE.") {
            if let Some(number) = entry.key_number(1) {
                let file = entry
                    .strs()
                    .first()
                    .map(|s| (*s).to_owned())
                    .unwrap_or_default();
                let channel = entry.ints().first().copied().unwrap_or(-1);
                se_table.insert(number, (file, channel));
            }
        }
        let mut koe_characters = HashMap::new();
        for entry in gameexe.filter("KOEONOFF.") {
            let parts = entry.key_parts();
            let (Some(usekoe), Some(ids)) = (
                parts.get(1).and_then(|p| p.parse::<i32>().ok()),
                parts.get(2),
            ) else {
                continue;
            };
            for id in ids.trim_matches(|c| c == '(' || c == ')').split(',') {
                if let Ok(id) = id.trim().parse::<i32>() {
                    koe_characters.insert(id, usekoe);
                }
            }
        }
        Self {
            #[cfg(not(target_os = "horizon"))]
            manager: if device && std::env::var_os("REALLIVE_NO_AUDIO").is_none() {
                AudioManager::new(AudioManagerSettings::default()).ok()
            } else {
                None
            },
            tracks,
            se_table,
            koe_characters,
            bgm: None,
            bgm_name: String::new(),
            bgm_looped: false,
            queued_bgm: None,
            wav: Default::default(),
            koe: None,
            se: Vec::new(),
            movie: None,
            bgm_volume: Ramp::fixed(255),
            wav_volume: [Ramp::fixed(255); WAV_CHANNELS],
            koe_volume: Ramp::fixed(255),
            se_volume: Ramp::fixed(255),
            wav_names: std::array::from_fn(|_| (String::new(), false)),
            voices: HashMap::new(),
            cache: HashMap::new(),
            applied: Vec::new(),
        }
    }

    pub fn silent() -> Self {
        Self::new(&Gameexe::default(), false)
    }

    pub fn has_device(&self) -> bool {
        #[cfg(not(target_os = "horizon"))]
        {
            self.manager.is_some()
        }
        #[cfg(target_os = "horizon")]
        {
            false
        }
    }

    fn load(&mut self, resources: &Resources, kind: Kind, name: &str) -> Result<Pcm> {
        let key = (kind, name.to_lowercase());
        if let Some(pcm) = self.cache.get(&key) {
            return Ok(pcm.clone());
        }
        let path = resources
            .find(kind, name)
            .with_context(|| format!("reallive: sound {name:?} not found"))?;
        let pcm = decode(game_fs::read(&path)?)
            .with_context(|| format!("reallive: cannot decode {}", path.display()))?;
        // Keep short effects; music is decoded again when needed.
        if pcm.samples.len() < 2_000_000 {
            if self.cache.len() > 64 {
                self.cache.clear();
            }
            self.cache.insert(key, pcm.clone());
        }
        Ok(pcm)
    }

    #[allow(unused_variables)]
    fn start(&mut self, pcm: &Pcm, looped: bool, volume: f64, fade_in: u64, now: u64) -> Voice {
        #[cfg(not(target_os = "horizon"))]
        let handle = self.manager.as_mut().and_then(|manager| {
            let data = StaticSoundData::from_cursor(std::io::Cursor::new(pcm.to_wav())).ok()?;
            let mut settings = StaticSoundSettings::new().volume(volume);
            if looped {
                let start = pcm.loop_start.unwrap_or(0) as f64 / f64::from(pcm.rate.max(1));
                settings = settings.loop_region(start..);
            }
            if fade_in > 0 {
                settings = settings.fade_in_tween(tween(fade_in));
            }
            manager.play(data.with_settings(settings)).ok()
        });
        #[cfg(target_os = "horizon")]
        let handle = None;
        Voice {
            handle,
            started: now,
            duration: pcm.duration_ms(),
            looped,
            ends_at: None,
            paused_at: None,
        }
    }

    fn stop(voice: Option<Voice>, fade: u64) {
        #[cfg(not(target_os = "horizon"))]
        if let Some(mut handle) = voice.and_then(|v| v.handle) {
            handle.stop(tween(fade));
        }
        #[cfg(target_os = "horizon")]
        let _ = (voice, fade);
    }

    fn fade_out(voice: &mut Option<Voice>, now: u64, fade: u64) {
        if fade == 0 {
            Self::stop(voice.take(), 0);
            return;
        }
        if let Some(v) = voice {
            #[cfg(not(target_os = "horizon"))]
            if let Some(handle) = v.handle.as_mut() {
                handle.stop(tween(fade));
            }
            v.ends_at = Some(now + fade);
        }
    }

    // ---- BGM ------------------------------------------------------------

    fn bgm_pcm(&mut self, resources: &Resources, name: &str) -> Result<Pcm> {
        if let Some(track) = self.tracks.get(&name.to_lowercase()).cloned() {
            let mut pcm = self.load(resources, Kind::Bgm, &track.file)?;
            let from = track.from.max(0) as usize;
            let to = if track.to <= 0 {
                usize::MAX
            } else {
                track.to as usize
            };
            pcm.trim(from, to);
            pcm.loop_start = Some((track.looped_from.max(0) as usize).saturating_sub(from));
            return Ok(pcm);
        }
        self.load(resources, Kind::Bgm, name)
    }

    /// `bgmPlay` / `bgmLoop`. With `fade_out`, the current track fades
    /// out first and the new one starts afterwards.
    #[allow(clippy::too_many_arguments)]
    pub fn bgm_play(
        &mut self,
        resources: &Resources,
        settings: &Settings,
        now: u64,
        name: &str,
        looped: bool,
        fade_in: u64,
        fade_out: u64,
    ) -> Result<()> {
        self.bgm_play_from(
            resources, settings, now, name, looped, fade_in, fade_out, None,
        )
    }

    /// `bgm_play`, starting `from` into the track (`DEBUG_MPLAY_*`).
    #[allow(clippy::too_many_arguments)]
    pub fn bgm_play_from(
        &mut self,
        resources: &Resources,
        settings: &Settings,
        now: u64,
        name: &str,
        looped: bool,
        fade_in: u64,
        fade_out: u64,
        from: Option<StartAt>,
    ) -> Result<()> {
        if fade_out > 0 && self.bgm_playing(now) {
            Self::fade_out(&mut self.bgm, now, fade_out);
            self.queued_bgm = Some((name.to_owned(), looped, fade_in, now + fade_out));
            return Ok(());
        }
        Self::stop(self.bgm.take(), 0);
        self.queued_bgm = None;
        self.bgm_name = name.to_owned();
        self.bgm_looped = looped;
        let mut pcm = self.bgm_pcm(resources, name)?;
        if let Some(from) = from {
            let frame = match from {
                StartAt::Ms(ms) => (ms as f64 * f64::from(pcm.rate) / 1000.0) as usize,
                StartAt::Frame(frame) => frame,
            };
            let frames = pcm.frames();
            pcm.trim(frame, frames);
            pcm.loop_start = pcm.loop_start.map(|start| start.saturating_sub(frame));
        }
        let volume = self.volume_of(settings, channel::BGM, now, self.bgm_volume.value(now));
        self.bgm = Some(self.start(&pcm, looped, volume, fade_in, now));
        Ok(())
    }

    /// `MCHANGELOOP` / `MCHANGEONESHOT`: whether the playing track loops.
    pub fn set_bgm_looped(&mut self, looped: bool, now: u64) {
        self.bgm_looped = looped;
        if let Some(voice) = self.bgm.as_mut() {
            voice.set_looped(looped, now);
        }
    }

    /// `PCMCHANGELOOP` / `PCMCHANGEONESHOT`.
    pub fn set_wav_looped(&mut self, channel: usize, looped: bool, now: u64) {
        if let Some(voice) = self.wav.get_mut(channel).and_then(Option::as_mut) {
            voice.set_looped(looped, now);
        }
        if let Some(entry) = self.wav_names.get_mut(channel) {
            entry.1 = looped;
        }
    }

    /// `wavRewind`: the channel's sound again from the start.
    pub fn wav_rewind(
        &mut self,
        resources: &Resources,
        settings: &Settings,
        now: u64,
        channel: usize,
    ) -> Result<()> {
        let Some((name, looped)) = self.wav_names.get(channel).cloned() else {
            return Ok(());
        };
        if name.is_empty() || !self.wav_playing(channel, now) {
            return Ok(());
        }
        self.wav_play(resources, settings, now, &name, Some(channel), looped, 0)?;
        Ok(())
    }

    /// `PCMFADECHECK`: the channel is fading out.
    pub fn wav_fading(&self, channel: usize, now: u64) -> bool {
        self.wav
            .get(channel)
            .and_then(Option::as_ref)
            .is_some_and(|v| v.playing(now) && v.ends_at.is_some())
    }

    pub fn set_se_volume(&mut self, now: u64, volume: i32, fade: u64) {
        self.se_volume.retarget(now, volume, fade);
    }

    pub fn se_volume(&self, now: u64) -> i32 {
        self.se_volume.value(now)
    }

    pub fn bgm_stop(&mut self) {
        Self::stop(self.bgm.take(), 0);
        self.queued_bgm = None;
        self.bgm_name.clear();
    }

    pub fn bgm_fade_out(&mut self, now: u64, fade: u64) {
        self.queued_bgm = None;
        Self::fade_out(&mut self.bgm, now, fade);
    }

    pub fn bgm_looped(&self) -> bool {
        self.bgm_looped
    }

    pub fn bgm_playing(&self, now: u64) -> bool {
        self.bgm.as_ref().is_some_and(|v| v.playing(now))
    }

    /// `bgmStatus`: 0 stopped, 1 playing, 2 fading out.
    pub fn bgm_status(&self, now: u64) -> i32 {
        match &self.bgm {
            Some(v) if v.playing(now) && v.ends_at.is_some() => 2,
            Some(v) if v.playing(now) => 1,
            _ => 0,
        }
    }

    pub fn bgm_pause(&mut self, now: u64) {
        if let Some(v) = self.bgm.as_mut() {
            #[cfg(not(target_os = "horizon"))]
            if let Some(handle) = v.handle.as_mut() {
                handle.pause(tween(0));
            }
            v.paused_at.get_or_insert(now);
        }
    }

    pub fn bgm_resume(&mut self, now: u64) {
        if let Some(v) = self.bgm.as_mut() {
            #[cfg(not(target_os = "horizon"))]
            if let Some(handle) = v.handle.as_mut() {
                handle.resume(tween(0));
            }
            if let Some(paused) = v.paused_at.take() {
                v.started += now - paused;
            }
        }
    }

    pub fn bgm_rewind(
        &mut self,
        resources: &Resources,
        settings: &Settings,
        now: u64,
    ) -> Result<()> {
        if self.bgm_name.is_empty() {
            return Ok(());
        }
        let name = self.bgm_name.clone();
        let looped = self.bgm_looped;
        self.bgm_play(resources, settings, now, &name, looped, 0, 0)
    }

    pub fn set_bgm_volume(&mut self, now: u64, volume: i32, fade: u64) {
        self.bgm_volume.retarget(now, volume, fade);
    }

    pub fn bgm_volume(&self, now: u64) -> i32 {
        self.bgm_volume.value(now)
    }

    // ---- PCM channels ---------------------------------------------------

    /// `wavPlay` family. `channel` None picks a free channel.
    #[allow(clippy::too_many_arguments)]
    pub fn wav_play(
        &mut self,
        resources: &Resources,
        settings: &Settings,
        now: u64,
        name: &str,
        channel: Option<usize>,
        looped: bool,
        fade_in: u64,
    ) -> Result<usize> {
        let index = match channel {
            Some(c) => c.min(WAV_CHANNELS - 1),
            None => (0..WAV_CHANNELS)
                .find(|&c| !self.wav[c].as_ref().is_some_and(|v| v.playing(now)))
                .unwrap_or(0),
        };
        Self::stop(self.wav[index].take(), 0);
        let pcm = self.load(resources, Kind::Wav, name)?;
        let volume = self.volume_of(
            settings,
            channel::PCM,
            now,
            self.wav_volume[index].value(now),
        );
        self.wav[index] = Some(self.start(&pcm, looped, volume, fade_in, now));
        self.wav_names[index] = (name.to_owned(), looped);
        Ok(index)
    }

    pub fn wav_stop(&mut self, channel: usize, fade: u64, now: u64) {
        if let Some(slot) = self.wav.get_mut(channel) {
            Self::fade_out(slot, now, fade);
        }
    }

    pub fn wav_stop_all(&mut self, fade: u64, now: u64) {
        for slot in &mut self.wav {
            Self::fade_out(slot, now, fade);
        }
    }

    pub fn wav_playing(&self, channel: usize, now: u64) -> bool {
        self.wav
            .get(channel)
            .and_then(Option::as_ref)
            .is_some_and(|v| v.playing(now))
    }

    pub fn set_wav_volume(&mut self, channel: usize, now: u64, volume: i32, fade: u64) {
        if let Some(ramp) = self.wav_volume.get_mut(channel) {
            ramp.retarget(now, volume, fade);
        }
    }

    pub fn wav_volume(&self, channel: usize, now: u64) -> i32 {
        self.wav_volume.get(channel).map_or(0, |r| r.value(now))
    }

    // ---- interface sounds -------------------------------------------------

    pub fn has_se(&self, number: i32) -> bool {
        self.se_table
            .get(&number)
            .is_some_and(|(file, _)| !file.is_empty())
    }

    /// `sePlay(n)` and interface sounds: `#SE.nnn`.
    pub fn se_play(
        &mut self,
        resources: &Resources,
        settings: &Settings,
        now: u64,
        number: i32,
    ) -> Result<()> {
        let Some((file, _)) = self.se_table.get(&number).cloned() else {
            return Ok(());
        };
        if file.is_empty() {
            return Ok(());
        }
        let pcm = self.load(resources, Kind::Wav, &file)?;
        let volume = self.volume_of(settings, channel::SE, now, self.se_volume.value(now));
        self.se.retain(|v| v.playing(now));
        let voice = self.start(&pcm, false, volume, 0, now);
        self.se.push(voice);
        Ok(())
    }

    // ---- voices -------------------------------------------------------------

    fn voice_pcm(&mut self, resources: &Resources, id: i32) -> Result<Pcm> {
        let file = id / 100_000;
        let sample = id % 100_000;
        if let std::collections::hash_map::Entry::Vacant(slot) = self.voices.entry(file) {
            if let Some(path) = resources.find(Kind::Koe, &format!("z{file:04}")) {
                slot.insert(VoiceArchive::open(&path)?);
            }
        }
        if let Some(archive) = self.voices.get(&file) {
            return archive.decode(sample);
        }
        // Loose files, optionally in per-archive folders.
        let loose = format!("z{file:04}{sample:05}");
        let path = resources
            .find(Kind::Koe, &loose)
            .or_else(|| resources.find(Kind::Koe, &format!("{file:04}/{loose}")))
            .with_context(|| format!("reallive: voice {id} not found"))?;
        decode(game_fs::read(path)?)
    }

    /// Whether a voice of `character` should play (`#KOEONOFF` / `UseKoe`).
    pub fn character_enabled(&self, settings: &Settings, character: i32) -> bool {
        match self.koe_characters.get(&character) {
            Some(usekoe) => settings.use_koe.get(usekoe).copied().unwrap_or(true),
            None => true,
        }
    }

    pub fn koe_play(
        &mut self,
        resources: &Resources,
        settings: &Settings,
        now: u64,
        id: i32,
    ) -> Result<()> {
        Self::stop(self.koe.take(), 0);
        // Mode 2 is "text only".
        if settings.koe_mode == 2 || !settings.enabled[channel::KOE] {
            return Ok(());
        }
        let pcm = self.voice_pcm(resources, id)?;
        let volume = self.volume_of(settings, channel::KOE, now, self.koe_volume.value(now));
        self.koe = Some(self.start(&pcm, false, volume, 0, now));
        Ok(())
    }

    pub fn koe_stop(&mut self) {
        Self::stop(self.koe.take(), 0);
    }

    pub fn koe_playing(&self, now: u64) -> bool {
        self.koe.as_ref().is_some_and(|v| v.playing(now))
    }

    pub fn set_koe_volume(&mut self, now: u64, volume: i32, fade: u64) {
        self.koe_volume.retarget(now, volume, fade);
    }

    pub fn koe_volume(&self, now: u64) -> i32 {
        self.koe_volume.value(now)
    }

    // ---- movies ----------------------------------------------------------------

    /// A movie's sound track (at the music volume).
    pub fn movie_play(&mut self, settings: &Settings, now: u64, pcm: &Pcm) {
        Self::stop(self.movie.take(), 0);
        let volume = self.volume_of(settings, channel::BGM, now, 255);
        self.movie = Some(self.start(pcm, false, volume, 0, now));
    }

    pub fn movie_stop(&mut self) {
        Self::stop(self.movie.take(), 0);
    }

    // ---- housekeeping ---------------------------------------------------------

    fn volume_of(&self, settings: &Settings, index: usize, now: u64, script: i32) -> f64 {
        if !settings.enabled[index] {
            return 0.0;
        }
        let mut volume = f64::from(settings.volume[index].clamp(0, 255)) / 255.0
            * f64::from(script.clamp(0, 255))
            / 255.0;
        // Music ducks under voices.
        if index == channel::BGM && settings.bgm_koe_fade && self.koe_playing(now) {
            volume *= f64::from(settings.bgm_koe_fade_vol.clamp(0, 255)) / 255.0;
        }
        volume
    }

    /// Called once per frame: starts queued music, applies volume ramps.
    pub fn update(&mut self, resources: &Resources, settings: &Settings, now: u64) {
        if let Some((name, looped, fade_in, at)) = self.queued_bgm.clone() {
            if now >= at {
                self.queued_bgm = None;
                let _ = self.bgm_play(resources, settings, now, &name, looped, fade_in, 0);
            }
        }
        if self.bgm.as_ref().is_some_and(|v| !v.playing(now)) {
            self.bgm = None;
        }
        for slot in &mut self.wav {
            if slot.as_ref().is_some_and(|v| !v.playing(now)) {
                *slot = None;
            }
        }
        if self.koe.as_ref().is_some_and(|v| !v.playing(now)) {
            self.koe = None;
        }
        self.se.retain(|v| v.playing(now));
        self.apply_volumes(settings, now);
    }

    #[allow(unused_variables)]
    fn apply_volumes(&mut self, settings: &Settings, now: u64) {
        #[cfg(not(target_os = "horizon"))]
        {
            if self.manager.is_none() {
                return;
            }
            let mut wanted =
                vec![self.volume_of(settings, channel::BGM, now, self.bgm_volume.value(now))];
            for c in 0..WAV_CHANNELS {
                wanted.push(self.volume_of(
                    settings,
                    channel::PCM,
                    now,
                    self.wav_volume[c].value(now),
                ));
            }
            wanted.push(self.volume_of(settings, channel::KOE, now, self.koe_volume.value(now)));
            if wanted == self.applied {
                return;
            }
            let set = |voice: &mut Option<Voice>, volume: f64| {
                if let Some(handle) = voice.as_mut().and_then(|v| v.handle.as_mut()) {
                    handle.set_volume(volume, tween(30));
                }
            };
            set(&mut self.bgm, wanted[0]);
            for c in 0..WAV_CHANNELS {
                set(&mut self.wav[c], wanted[1 + c]);
            }
            set(&mut self.koe, wanted[1 + WAV_CHANNELS]);
            self.applied = wanted;
        }
    }

    pub fn stop_all(&mut self) {
        self.bgm_stop();
        for slot in &mut self.wav {
            Self::stop(slot.take(), 0);
        }
        self.koe_stop();
        for voice in self.se.drain(..) {
            Self::stop(Some(voice), 0);
        }
        self.movie_stop();
    }

    /// Music to resume after loading a save: (name, looped).
    pub fn current_bgm(&self, now: u64) -> Option<(String, bool)> {
        (self.bgm_playing(now) && !self.bgm_name.is_empty())
            .then(|| (self.bgm_name.clone(), self.bgm_looped))
    }
}

#[cfg(not(target_os = "horizon"))]
fn tween(ms: u64) -> Tween {
    Tween {
        duration: std::time::Duration::from_millis(ms),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn koe_tables_match_the_interpreter() {
        // Spot checks against the interpreter's 8-bit expansion table.
        assert_eq!(koe_sample(0) as u16, 0x8000);
        assert_eq!(koe_sample(1) as u16, 0x81ff);
        assert_eq!(koe_sample(127) as u16, 0xffff);
        assert_eq!(koe_sample(128) as u16, 0x0000);
        assert_eq!(koe_sample(129) as u16, 0x0001);
        assert_eq!(koe_sample(130) as u16, 0x0007);
        assert_eq!(koe_sample(254) as u16, 0x7c07);
        assert_eq!(koe_sample(255) as u16, 0x7fff);
        assert_eq!(koe_delta(0), 0x00);
        assert_eq!(koe_delta(1), 0xff);
        assert_eq!(koe_delta(2), 0x01);
        assert_eq!(koe_delta(255), 0x80);
    }

    #[test]
    fn wav_round_trip() {
        let pcm = Pcm {
            rate: 22050,
            channels: 2,
            samples: vec![0, 1, -1, 300, i16::MIN, i16::MAX],
            loop_start: None,
        };
        let back = decode_wav(&pcm.to_wav()).unwrap();
        assert_eq!(back, pcm);
        assert_eq!(back.frames(), 3);
    }

    #[test]
    fn koepac_blocks() {
        // One silent block, one raw block and one DPCM block.
        let mut data = Vec::new();
        for size in [0u16, 0x400, 2] {
            data.extend_from_slice(&size.to_le_bytes());
        }
        data.extend(std::iter::repeat_n(128u8, 0x400));
        data.extend_from_slice(&[0x21, 0x00]);
        let pcm = decode_koepac(&data, 0, 3, 22050).unwrap();
        assert_eq!(pcm.samples.len(), 0x400 * 3);
        assert!(pcm.samples[..0x800].iter().all(|&s| s == 0));
        // 0x21: low nibble 1 → d = 0 - 0xff = 1; high nibble 2 → d = 1 - 1 = 0.
        assert_eq!(pcm.samples[0x800], koe_sample(1));
        assert_eq!(pcm.samples[0x801], koe_sample(0));
    }

    #[test]
    fn ramps_interpolate() {
        let mut ramp = Ramp::fixed(255);
        ramp.retarget(1000, 0, 1000);
        assert_eq!(ramp.value(1500), 128);
        assert_eq!(ramp.value(2000), 0);
    }
}
