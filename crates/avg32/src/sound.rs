//! AVG32 audio: BGM, WAV effects and KOE voices.
//!
//! BGM is either CD-DA (`#CDTRACK`, played from a CloneCD image or ripped
//! track files) or "DirectSound" WAV files (`#MUSIC_TYPE=2`, `#DSTRACK`);
//! effects are WAV files on the `WAV` route; voices come from `KOEPAC`
//! archives (`Z###.KOE`), loose `Z#######.WAV` files, or the AFS archives of
//! the AIR voice patch.  Decoding lives here; playback goes through Kira on
//! desktop targets and is a silent no-op elsewhere.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::ini::Ini;
use crate::resource::Avg32Resources;

#[cfg(not(target_os = "horizon"))]
use kira::manager::backend::DefaultBackend;
#[cfg(not(target_os = "horizon"))]
use kira::manager::{AudioManager, AudioManagerSettings};
#[cfg(not(target_os = "horizon"))]
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};
#[cfg(not(target_os = "horizon"))]
use kira::tween::Tween;

/// A decoded, ready-to-play PCM clip (wrapped as an in-memory WAV).
#[derive(Debug, Clone)]
pub struct Clip {
    pub wav: Vec<u8>,
    /// Byte offset (into the PCM payload) that looping restarts from.
    pub loop_start: Option<usize>,
    pub byte_rate: u32,
}

#[cfg(not(target_os = "horizon"))]
type Handle = StaticSoundHandle;
#[cfg(target_os = "horizon")]
type Handle = ();

pub const CHANNEL_BGM: usize = 0;
pub const CHANNEL_WAV: usize = 1;
pub const CHANNEL_KOE: usize = 2;
pub const CHANNEL_SE: usize = 3;

pub struct Sound {
    #[cfg(not(target_os = "horizon"))]
    manager: Option<AudioManager<DefaultBackend>>,
    bgm: Option<Handle>,
    bgm_name: String,
    bgm_loop: bool,
    waves: BTreeMap<i32, Handle>,
    koe: Option<Handle>,
    se: Option<Handle>,
    movie: Option<Handle>,
    koe_enabled: bool,
    /// Per-channel volume (0-255) and mute state: BGM, WAV, KOE, SE.
    pub volumes: [i32; 4],
    pub muted: [bool; 4],
    koe_tables: BTreeMap<i32, Vec<KoeEntry>>,
    cd_tracks: Option<BTreeMap<usize, CdTrack>>,
    game_root: PathBuf,
}

impl std::fmt::Debug for Sound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sound")
            .field("bgm", &self.bgm_name)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy)]
struct KoeEntry {
    id: i32,
    blocks: usize,
    position: u32,
    rate: u32,
    end: u32,
}

#[derive(Debug, Clone)]
struct CdTrack {
    image: PathBuf,
    first_sector: usize,
    sector_count: usize,
}

impl Sound {
    pub fn new(game_root: &Path) -> Self {
        Self {
            #[cfg(not(target_os = "horizon"))]
            manager: if std::env::var_os("AVG32_NO_AUDIO").is_some() {
                None
            } else {
                AudioManager::new(AudioManagerSettings::default()).ok()
            },
            bgm: None,
            bgm_name: String::new(),
            bgm_loop: false,
            waves: BTreeMap::new(),
            koe: None,
            se: None,
            movie: None,
            koe_enabled: true,
            volumes: [255; 4],
            muted: [false; 4],
            koe_tables: BTreeMap::new(),
            cd_tracks: None,
            game_root: game_root.to_path_buf(),
        }
    }

    /// A silent instance (for headless tools and tests).
    pub fn silent(game_root: &Path) -> Self {
        let mut sound = Self::new_without_device(game_root);
        sound.koe_enabled = true;
        sound
    }

    fn new_without_device(game_root: &Path) -> Self {
        Self {
            #[cfg(not(target_os = "horizon"))]
            manager: None,
            bgm: None,
            bgm_name: String::new(),
            bgm_loop: false,
            waves: BTreeMap::new(),
            koe: None,
            se: None,
            movie: None,
            koe_enabled: true,
            volumes: [255; 4],
            muted: [false; 4],
            koe_tables: BTreeMap::new(),
            cd_tracks: None,
            game_root: game_root.to_path_buf(),
        }
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

    fn volume(&self, channel: usize) -> f64 {
        if self.muted[channel] {
            0.0
        } else {
            f64::from(self.volumes[channel].clamp(0, 255)) / 255.0
        }
    }

    #[cfg(not(target_os = "horizon"))]
    fn start(
        &mut self,
        clip: &Clip,
        looped: bool,
        channel: usize,
        fade_in_ms: u32,
    ) -> Option<Handle> {
        let volume = self.volume(channel);
        let manager = self.manager.as_mut()?;
        let data = StaticSoundData::from_cursor(std::io::Cursor::new(clip.wav.clone())).ok()?;
        let mut settings = StaticSoundSettings::new().volume(volume);
        if looped {
            let start = clip
                .loop_start
                .filter(|_| clip.byte_rate > 0)
                .map_or(0.0, |bytes| bytes as f64 / f64::from(clip.byte_rate));
            settings = settings.loop_region(start..);
        }
        if fade_in_ms > 0 {
            settings = settings.fade_in_tween(Tween {
                duration: std::time::Duration::from_millis(u64::from(fade_in_ms)),
                ..Default::default()
            });
        }
        manager.play(data.with_settings(settings)).ok()
    }

    #[cfg(target_os = "horizon")]
    fn start(
        &mut self,
        _clip: &Clip,
        _looped: bool,
        _channel: usize,
        _fade_in_ms: u32,
    ) -> Option<Handle> {
        None
    }

    fn stop_handle(handle: Option<Handle>, fade_ms: u32) {
        #[cfg(not(target_os = "horizon"))]
        if let Some(mut handle) = handle {
            handle.stop(Tween {
                duration: std::time::Duration::from_millis(u64::from(fade_ms)),
                ..Default::default()
            });
        }
        #[cfg(target_os = "horizon")]
        let _ = (handle, fade_ms);
    }

    fn playing(handle: Option<&Handle>) -> bool {
        #[cfg(not(target_os = "horizon"))]
        {
            handle.is_some_and(|handle| {
                matches!(
                    handle.state(),
                    kira::sound::PlaybackState::Playing | kira::sound::PlaybackState::Pausing
                )
            })
        }
        #[cfg(target_os = "horizon")]
        {
            let _ = handle;
            false
        }
    }

    // ---- BGM -------------------------------------------------------------

    pub fn current_bgm(&self) -> &str {
        &self.bgm_name
    }

    pub fn bgm_playing(&self) -> bool {
        Self::playing(self.bgm.as_ref())
    }

    /// Starts BGM playback, optionally fading in.
    pub fn bgm_play(
        &mut self,
        ini: &Ini,
        resources: &Avg32Resources,
        name: &str,
        looped: bool,
        fade_in_ms: u32,
    ) {
        self.bgm_stop(0);
        self.bgm_name = name.to_owned();
        self.bgm_loop = looped;
        let Some(clip) = self.bgm_clip(ini, resources, name) else {
            return;
        };
        self.bgm = self.start(&clip, looped, CHANNEL_BGM, fade_in_ms);
    }

    pub fn bgm_stop(&mut self, fade_ms: u32) {
        Self::stop_handle(self.bgm.take(), fade_ms);
        self.bgm_name.clear();
        self.bgm_loop = false;
    }

    fn bgm_clip(&mut self, ini: &Ini, resources: &Avg32Resources, name: &str) -> Option<Clip> {
        if ini.music_type == 2 {
            let track = ini.dsound.iter().find(|track| track.name == name)?;
            let bytes = find_file(
                &self.game_root,
                &ini.bgm_dir,
                &format!("{}.WAV", track.file),
            )
            .and_then(|path| game_fs::read(path).ok())?;
            let mut clip = wav_clip(&bytes, ini.music_linear != 0).ok()?;
            if track.cut_size > 0 {
                let cut = track.cut_size as usize;
                clip.loop_start = Some(if ini.music_linear != 0 {
                    cut / 2 * 2
                } else {
                    cut
                });
            }
            return Some(clip);
        }
        // CD-DA: the name maps to a track number through `#CDTRACK`.
        let track = ini
            .cd
            .iter()
            .find(|(_, track_name)| track_name.as_str() == name)
            .map(|(track, _)| *track)
            .or_else(|| name.trim().parse().ok())?;
        for candidate in [
            format!("{name}.WAV"),
            format!("{name}.OGG"),
            format!("TRACK{track:02}.WAV"),
            format!("TRACK{track:02}.OGG"),
            format!("{track:02}.WAV"),
            format!("{track:02}.OGG"),
        ] {
            let path = find_file(&self.game_root, &ini.bgm_dir, &candidate)
                .or_else(|| find_file(&self.game_root, "BGM", &candidate));
            if let Some(bytes) = path.and_then(|path| game_fs::read(path).ok()) {
                if let Ok(clip) = wav_clip(&bytes, false) {
                    return Some(clip);
                }
                return Some(Clip {
                    wav: bytes,
                    loop_start: None,
                    byte_rate: 0,
                });
            }
        }
        if let Ok(bytes) = resources.read("BGM", name) {
            return wav_clip(&bytes, false).ok();
        }
        let tracks = self
            .cd_tracks
            .get_or_insert_with(|| discover_cd_tracks(&self.game_root));
        tracks
            .get(&track)
            .and_then(|track| decode_cd_track(track).ok())
    }

    // ---- WAV --------------------------------------------------------------

    pub fn wav_play(
        &mut self,
        ini: &Ini,
        resources: &Avg32Resources,
        name: &str,
        looped: bool,
        channel: Option<i32>,
    ) {
        let key = channel.unwrap_or(-1);
        Self::stop_handle(self.waves.remove(&key), 0);
        let Ok(bytes) = resources.read("WAV", name) else {
            return;
        };
        let Ok(clip) = wav_clip(&bytes, ini.wav_linear != 0) else {
            return;
        };
        if let Some(handle) = self.start(&clip, looped, CHANNEL_WAV, 0) {
            self.waves.insert(key, handle);
        }
    }

    pub fn wav_stop(&mut self, channel: Option<i32>) {
        match channel {
            Some(channel) => Self::stop_handle(self.waves.remove(&channel), 0),
            None => {
                for (_, handle) in std::mem::take(&mut self.waves) {
                    Self::stop_handle(Some(handle), 0);
                }
            }
        }
    }

    pub fn wav_playing(&self) -> bool {
        self.waves
            .values()
            .any(|handle| Self::playing(Some(handle)))
    }

    /// `#SE.nnn` on its own channel so it never cuts an
    /// ambient loop short.
    pub fn se_play(&mut self, ini: &Ini, resources: &Avg32Resources, index: i32) {
        let Some(name) = usize::try_from(index)
            .ok()
            .and_then(|index| ini.se.get(index))
            .filter(|name| !name.is_empty())
            .cloned()
        else {
            return;
        };
        let Ok(bytes) = resources.read("WAV", &name) else {
            return;
        };
        let Ok(clip) = wav_clip(&bytes, ini.wav_linear != 0) else {
            return;
        };
        Self::stop_handle(self.se.take(), 0);
        self.se = self.start(&clip, false, CHANNEL_SE, 0);
    }

    // ---- KOE ----------------------------------------------------------------

    pub fn koe_enable(&mut self, enabled: bool) {
        self.koe_enabled = enabled;
    }

    pub fn koe_playing(&self) -> bool {
        Self::playing(self.koe.as_ref())
    }

    pub fn koe_stop(&mut self) {
        Self::stop_handle(self.koe.take(), 0);
    }

    /// `id` packs the archive (`Z###`) and entry number.
    pub fn koe_play(&mut self, ini: &Ini, version: i32, id: i32) {
        self.koe_stop();
        if !self.koe_enabled {
            return;
        }
        let (archive, entry) = if version >= 1714 {
            (id / 100_000, id % 100_000)
        } else {
            (id / 10_000, id % 10_000)
        };
        let clip = self
            .koe_from_pac(ini, archive, entry)
            .or_else(|| self.koe_from_wav(ini, archive, entry))
            .or_else(|| decode_afs_voice(&self.game_root, id as u32).ok());
        if let Some(clip) = clip {
            self.koe = self.start(&clip, false, CHANNEL_KOE, 0);
        }
    }

    fn koe_from_pac(&mut self, ini: &Ini, archive: i32, entry: i32) -> Option<Clip> {
        let path = find_file(&self.game_root, &ini.koe_dir, &format!("Z{archive:03}.KOE"))?;
        if !self.koe_tables.contains_key(&archive) {
            let table = read_koe_table(&path).ok()?;
            self.koe_tables.insert(archive, table);
        }
        let entry = *self.koe_tables[&archive]
            .iter()
            .find(|item| item.id == entry)?;
        let bytes = game_fs::read(&path).ok()?;
        let pcm = decode_koe_entry(&bytes, entry).ok()?;
        Some(pcm_clip(entry.rate, 1, &pcm))
    }

    fn koe_from_wav(&self, ini: &Ini, archive: i32, entry: i32) -> Option<Clip> {
        let name = format!("Z{archive:03}{entry:04}.WAV");
        let path = find_file(
            &self.game_root,
            &format!("{}/V/{archive:03}", ini.koe_dir),
            &name,
        )
        .or_else(|| find_file(&self.game_root, &ini.koe_dir, &name))?;
        wav_clip(&game_fs::read(path).ok()?, ini.koe_linear != 0).ok()
    }

    // ---- movie ------------------------------------------------------------------

    pub fn movie_play(&mut self, sample_rate: u32, channels: u16, bits: u16, pcm: &[u8]) {
        let clip = Clip {
            wav: wrap_pcm_as_wav(sample_rate, channels, bits, pcm),
            loop_start: None,
            byte_rate: 0,
        };
        Self::stop_handle(self.movie.take(), 0);
        self.movie = self.start(&clip, false, CHANNEL_WAV, 0);
    }

    pub fn movie_stop(&mut self) {
        Self::stop_handle(self.movie.take(), 0);
    }

    // ---- volume -----------------------------------------------------------

    pub fn set_volume(&mut self, channel: usize, volume: i32) {
        if channel < 4 {
            self.volumes[channel] = volume.clamp(0, 255);
            self.apply_volume(channel);
        }
    }

    pub fn set_mute(&mut self, channel: usize, muted: bool) {
        if channel < 4 {
            self.muted[channel] = muted;
            self.apply_volume(channel);
            if channel == CHANNEL_KOE {
                self.koe_enabled = !muted;
            }
        }
    }

    fn apply_volume(&mut self, channel: usize) {
        #[cfg(not(target_os = "horizon"))]
        {
            let volume = self.volume(channel);
            let mut handles: Vec<&mut Handle> = match channel {
                CHANNEL_BGM => self.bgm.iter_mut().collect(),
                CHANNEL_WAV => self
                    .waves
                    .values_mut()
                    .chain(self.movie.iter_mut())
                    .collect(),
                CHANNEL_KOE => self.koe.iter_mut().collect(),
                _ => self.se.iter_mut().collect(),
            };
            for handle in handles.iter_mut() {
                handle.set_volume(volume, Tween::default());
            }
        }
        #[cfg(target_os = "horizon")]
        let _ = channel;
    }

    pub fn stop_all(&mut self) {
        self.bgm_stop(0);
        self.wav_stop(None);
        self.koe_stop();
        self.movie_stop();
    }
}

/// Case-insensitive `root/dir/name` lookup (`dir` may contain `/`).
pub fn find_file(root: &Path, dir: &str, name: &str) -> Option<PathBuf> {
    let mut current = root.to_path_buf();
    for component in dir.split(['/', '\\', ':']).filter(|part| !part.is_empty()) {
        current = find_entry(&current, component)?;
    }
    find_entry(&current, name)
}

fn find_entry(directory: &Path, wanted: &str) -> Option<PathBuf> {
    crate::resource::find_component(directory, wanted)
}

/// AVG32's "pseudo 8-bit" samples expand quadratically.
fn linear_sample(byte: u8) -> i16 {
    let value = i32::from(byte) - 0x80;
    (value * 2 * value.abs()) as i16
}

/// Parses a RIFF WAVE (scanning for the `data` chunk) and
/// optionally expands pseudo-linear 8-bit PCM to 16-bit.
pub fn wav_clip(bytes: &[u8], linear: bool) -> Result<Clip> {
    if bytes.len() < 44 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        bail!("avg32: not a RIFF WAVE file");
    }
    let mut at = 12usize;
    let mut format = None;
    let mut data = None;
    while at + 8 <= bytes.len() {
        let id = &bytes[at..at + 4];
        let size =
            u32::from_le_bytes(bytes[at + 4..at + 8].try_into().expect("four bytes")) as usize;
        let body = at + 8;
        let end = body.saturating_add(size).min(bytes.len());
        if id == b"fmt " && end - body >= 16 {
            let channels = u16::from_le_bytes([bytes[body + 2], bytes[body + 3]]);
            let rate =
                u32::from_le_bytes(bytes[body + 4..body + 8].try_into().expect("four bytes"));
            let bits = u16::from_le_bytes([bytes[body + 14], bytes[body + 15]]);
            format = Some((channels, rate, bits));
        } else if id == b"data" {
            data = Some(&bytes[body..end]);
            break;
        }
        at = body + size + (size & 1);
    }
    let (channels, rate, bits) = format.context("avg32: WAVE file has no fmt chunk")?;
    let data = data.context("avg32: WAVE file has no data chunk")?;
    if bits == 8 && linear {
        let pcm: Vec<i16> = data.iter().map(|byte| linear_sample(*byte)).collect();
        return Ok(pcm_clip(rate, channels, &pcm));
    }
    let block = u32::from(channels.max(1)) * u32::from(bits.max(8) / 8);
    Ok(Clip {
        wav: wrap_pcm_as_wav(rate, channels, bits, data),
        loop_start: None,
        byte_rate: rate * block,
    })
}

fn pcm_clip(rate: u32, channels: u16, samples: &[i16]) -> Clip {
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    Clip {
        wav: wrap_pcm_as_wav(rate, channels, 16, &bytes),
        loop_start: None,
        byte_rate: rate * u32::from(channels.max(1)) * 2,
    }
}

pub fn wrap_pcm_as_wav(
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
    data: &[u8],
) -> Vec<u8> {
    let block_align = channels.max(1) * (bits_per_sample.max(8) / 8);
    let mut wav = Vec::with_capacity(44 + data.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&((36 + data.len()) as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&channels.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * u32::from(block_align)).to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&bits_per_sample.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(data.len() as u32).to_le_bytes());
    wav.extend_from_slice(data);
    wav
}

fn read_koe_table(path: &Path) -> Result<Vec<KoeEntry>> {
    let bytes = game_fs::read(path)?;
    if bytes.len() < 32 || &bytes[0..6] != b"KOEPAC" {
        bail!("avg32: {} is not a KOEPAC archive", path.display());
    }
    let count = le_u32(&bytes, 16)? as usize;
    let rate = match le_u32(&bytes, 24)? {
        0 => 22_050,
        rate => rate,
    };
    if count == 0 || count >= 1024 {
        bail!("avg32: KOEPAC entry count {count} is out of range");
    }
    let mut entries = Vec::with_capacity(count);
    for index in 0..count {
        let at = 32 + index * 8;
        let id = i16::from_le_bytes([bytes[at], bytes[at + 1]]);
        let blocks = u16::from_le_bytes([bytes[at + 2], bytes[at + 3]]);
        let position = le_u32(&bytes, at + 4)?;
        entries.push(KoeEntry {
            id: i32::from(id),
            blocks: usize::from(blocks),
            position,
            rate,
            end: 0,
        });
    }
    let file_end = bytes.len() as u32;
    for index in 0..count {
        entries[index].end = entries
            .get(index + 1)
            .map_or(file_end, |next| next.position);
    }
    Ok(entries)
}

/// The KOEPAC DPCM voice decoder.
fn decode_koe_entry(bytes: &[u8], entry: KoeEntry) -> Result<Vec<i16>> {
    let table_at = entry.position as usize;
    let mut sizes = Vec::with_capacity(entry.blocks);
    for block in 0..entry.blocks {
        let at = table_at + block * 2;
        let pair = bytes
            .get(at..at + 2)
            .context("avg32: truncated KOE block table")?;
        sizes.push(usize::from(u16::from_le_bytes([pair[0], pair[1]])));
    }
    let data_at = table_at + entry.blocks * 2;
    let total: usize = sizes.iter().sum();
    if total > (entry.end.saturating_sub(entry.position)) as usize {
        bail!(
            "avg32: KOE entry {} is larger than its archive slot",
            entry.id
        );
    }
    let data = bytes
        .get(data_at..data_at + total)
        .context("avg32: truncated KOE sample data")?;
    let delta = |code: u8| -> u8 {
        let mut n = u32::from(code >> 1);
        if code & 1 != 0 {
            n ^= 0xff;
        }
        ((256 - n) & 0xff) as u8
    };
    let mut output = Vec::new();
    let mut at = 0usize;
    for size in sizes {
        match size {
            0 => output.extend(std::iter::repeat_n(0i16, 0x400)),
            0x400 => {
                output.extend(data[at..at + 0x400].iter().map(|byte| linear_sample(*byte)));
                at += 0x400;
            }
            _ => {
                let block = &data[at..at + size];
                let nibble = |index: usize| -> u8 {
                    block
                        .get(index >> 1)
                        .map_or(0, |byte| (byte >> ((index & 1) << 2)) & 15)
                };
                let mut sample = 0u8;
                let mut index = 0usize;
                while index < size * 2 {
                    let mut code = nibble(index);
                    index += 1;
                    if code == 15 {
                        code = nibble(index) | (nibble(index + 1) << 4);
                        index += 2;
                    }
                    sample = sample.wrapping_add(delta(code));
                    output.push(linear_sample(sample));
                }
                at += size;
            }
        }
    }
    Ok(output)
}

/// The AIR voice patch (`vair_for_win_mixer`) keeps CRI ADX voices in AFS
/// archives beside the game directory.
pub fn decode_afs_voice(game_root: &Path, voice_id: u32) -> Result<Clip> {
    use std::io::{Read, Seek, SeekFrom};
    let group = (voice_id >> 16) & 0xff;
    let index = u64::from(voice_id & 0xffff);
    let name = format!("VOICE{group:02}.AFS");
    let path = crate::voicepatch::afs_directories(game_root)
        .into_iter()
        .find_map(|directory| find_file(&directory, "", &name))
        .with_context(|| format!("{name} not found"))?;
    let mut file =
        game_fs::open(&path).with_context(|| format!("open {}", path.display()))?;
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header[..3] != b"AFS" {
        bail!("AVG32 voice archive {} is not AFS", path.display());
    }
    let count = u64::from(u32::from_le_bytes(
        header[4..8].try_into().expect("four bytes"),
    ));
    if index >= count {
        bail!("AVG32 voice {voice_id:#x} is outside {count} entries");
    }
    let mut entry = [0u8; 8];
    file.seek(SeekFrom::Start(8 + index * 8))?;
    file.read_exact(&mut entry)?;
    let offset = u32::from_le_bytes(entry[..4].try_into().expect("four bytes"));
    let length = u32::from_le_bytes(entry[4..].try_into().expect("four bytes"));
    let mut adx = vec![0u8; length as usize];
    file.seek(SeekFrom::Start(u64::from(offset)))?;
    file.read_exact(&mut adx)
        .context("AVG32 AFS entry lies outside archive")?;
    let (rate, pcm) = decode_adx(&adx)?;
    Ok(pcm_clip(rate, 1, &pcm))
}

fn decode_adx(adx: &[u8]) -> Result<(u32, Vec<i16>)> {
    if adx.get(..2) != Some(&[0x80, 0x00]) {
        bail!("AVG32 voice entry is not CRI ADX");
    }
    let data_offset = usize::from(be_u16(adx, 2)?) + 4;
    let sample_rate = be_u32(adx, 8)?;
    let sample_count = be_u32(adx, 12)? as usize;
    let mut output = Vec::with_capacity(sample_count);
    let mut cursor = data_offset;
    let (mut previous, mut previous_previous) = (0i32, 0i32);
    while output.len() < sample_count {
        let scale = i32::from(be_u16(adx, cursor)?);
        cursor += 2;
        let packed = adx
            .get(cursor..cursor + 16)
            .context("truncated ADX block")?;
        cursor += 16;
        for byte in packed {
            for nibble in [byte >> 4, byte & 0x0f] {
                let nibble = if nibble & 8 != 0 {
                    i32::from(nibble) - 16
                } else {
                    i32::from(nibble)
                };
                let sample = ((nibble * scale * 0x7f00 + 0x7298 * previous
                    - 0x3350 * previous_previous)
                    >> 14)
                    .clamp(i32::from(i16::MIN), i32::from(i16::MAX));
                previous_previous = previous;
                previous = sample;
                output.push(sample as i16);
                if output.len() == sample_count {
                    return Ok((sample_rate, output));
                }
            }
        }
    }
    Ok((sample_rate, output))
}

fn discover_cd_tracks(game_root: &Path) -> BTreeMap<usize, CdTrack> {
    let mut directories = vec![game_root.to_path_buf()];
    if let Some(parent) = game_root.parent() {
        directories.push(parent.to_path_buf());
        if let Ok(entries) = game_fs::read_dir(parent) {
            directories.extend(
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| game_fs::is_dir(path)),
            );
        }
    }
    // A release may ship several discs (AIR: a data-only disc and the
    // music disc); take audio tracks from every image found, earlier
    // directories first.
    let mut tracks = BTreeMap::new();
    for directory in directories {
        for (number, track) in parse_ccd(&directory).unwrap_or_default() {
            tracks.entry(number).or_insert(track);
        }
    }
    tracks
}

fn parse_ccd(directory: &Path) -> Result<BTreeMap<usize, CdTrack>> {
    let ccd = game_fs::read_dir(directory)?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ccd"))
        })
        .context("no CloneCD descriptor")?;
    let image = ["img", "IMG"]
        .iter()
        .map(|extension| ccd.with_extension(extension))
        .find(|path| game_fs::is_file(path))
        .context("no CloneCD image")?;
    let text = game_fs::read_to_string(&ccd)?;
    let mut entries = BTreeMap::new();
    let mut data_tracks = std::collections::BTreeSet::new();
    let mut point = None;
    let mut control = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("[Entry ") {
            point = None;
            control = 0;
        } else if let Some(value) = line.strip_prefix("Point=0x") {
            point = usize::from_str_radix(value, 16).ok();
        } else if let Some(value) = line.strip_prefix("Control=0x") {
            control = u8::from_str_radix(value, 16).unwrap_or(0);
        } else if let Some(value) = line.strip_prefix("PLBA=") {
            if let (Some(point), Ok(lba)) = (point, value.parse::<usize>()) {
                entries.insert(point, lba);
                // Control bit 2 marks a data track.
                if control & 0x04 != 0 && (1..=99).contains(&point) {
                    data_tracks.insert(point);
                }
            }
        }
    }
    let lead_out = entries.get(&0xa2).copied().unwrap_or_default();
    let mut tracks = BTreeMap::new();
    for track in 1..=99usize {
        let Some(&start) = entries.get(&track) else {
            continue;
        };
        if data_tracks.contains(&track) {
            continue;
        }
        let end = ((track + 1)..=99)
            .find_map(|next| entries.get(&next).copied())
            .unwrap_or(lead_out);
        if end > start {
            tracks.insert(
                track,
                CdTrack {
                    image: image.clone(),
                    first_sector: start,
                    sector_count: end - start,
                },
            );
        }
    }
    Ok(tracks)
}

fn decode_cd_track(track: &CdTrack) -> Result<Clip> {
    use std::io::{Read, Seek, SeekFrom};
    const SECTOR: usize = 2352;
    let mut image = game_fs::open(&track.image)?;
    image.seek(SeekFrom::Start((track.first_sector * SECTOR) as u64))?;
    let mut data = vec![0; track.sector_count * SECTOR];
    image.read_exact(&mut data)?;
    Ok(Clip {
        wav: wrap_pcm_as_wav(44_100, 2, 16, &data),
        loop_start: None,
        byte_rate: 44_100 * 4,
    })
}

fn le_u32(bytes: &[u8], at: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(
        bytes
            .get(at..at + 4)
            .context("truncated little-endian u32")?
            .try_into()
            .expect("four bytes"),
    ))
}

fn be_u16(bytes: &[u8], at: usize) -> Result<u16> {
    Ok(u16::from_be_bytes(
        bytes
            .get(at..at + 2)
            .context("truncated big-endian u16")?
            .try_into()
            .expect("two bytes"),
    ))
}

fn be_u32(bytes: &[u8], at: usize) -> Result<u32> {
    Ok(u32::from_be_bytes(
        bytes
            .get(at..at + 4)
            .context("truncated big-endian u32")?
            .try_into()
            .expect("four bytes"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_disc(dir: &Path, entries: &[(u8, u8, usize)]) {
        game_fs::create_dir_all(dir).unwrap();
        let mut ccd = String::from("[CloneCD]\nVersion=3\n");
        for (i, (point, control, lba)) in entries.iter().enumerate() {
            ccd += &format!("[Entry {i}]\nSession=1\nPoint=0x{point:02x}\nControl=0x{control:02x}\nPLBA={lba}\n");
        }
        game_fs::write(dir.join("IMAGE.CCD"), ccd).unwrap();
        game_fs::write(dir.join("IMAGE.img"), []).unwrap();
    }

    /// AIR ships a data-only disc next to the music disc; the data disc
    /// must not hide the music.
    #[test]
    fn cd_tracks_skip_data_discs() {
        let root = std::env::temp_dir().join(format!("avg32-ccd-{}", std::process::id()));
        let _ = game_fs::remove_dir_all(&root);
        write_disc(&root.join("blue"), &[(0xa2, 0x04, 1000), (0x01, 0x04, 0)]);
        write_disc(
            &root.join("orange"),
            &[(0xa2, 0x00, 900), (0x01, 0x04, 0), (0x02, 0x00, 600), (0x03, 0x00, 700)],
        );
        game_fs::create_dir_all(root.join("game")).unwrap();
        let tracks = discover_cd_tracks(&root.join("game"));
        let _ = game_fs::remove_dir_all(&root);
        assert_eq!(tracks.keys().copied().collect::<Vec<_>>(), vec![2, 3]);
        assert!(tracks[&2].image.ends_with("orange/IMAGE.img"));
        assert_eq!((tracks[&2].first_sector, tracks[&2].sector_count), (600, 100));
        assert_eq!(tracks[&3].sector_count, 200);
    }

    /// With `AVG32_GAME_ROOT` set to a CD-audio title (AIR), the music
    /// tracks must be found and decode to sound.
    #[test]
    fn installed_game_cd_tracks_decode() {
        let Some(root) = std::env::var_os("AVG32_GAME_ROOT") else {
            return;
        };
        let tracks = discover_cd_tracks(Path::new(&root));
        let track = tracks.get(&2).expect("CD track 2 (the first music track)");
        eprintln!("{} tracks; track 2 from {}", tracks.len(), track.image.display());
        let clip = decode_cd_track(track).unwrap();
        let pcm = &clip.wav[44..];
        assert!(pcm.iter().any(|&b| b != 0), "track {} is silent", track.image.display());
    }

    #[test]
    fn linear_table_matches_reference() {
        assert_eq!(linear_sample(0x80), 0);
        assert_eq!(linear_sample(0x81), 2);
        assert_eq!(linear_sample(0x7f), -2);
        assert_eq!(linear_sample(0xff), 127 * 254);
    }

    #[test]
    fn decodes_koe_dpcm_blocks() {
        // One silent block, then a 1-byte DPCM block holding nibbles 1 and 2:
        // code 1 => +1, code 2 => -1.
        let mut bytes = vec![0u8; 16];
        let entry = KoeEntry {
            id: 1,
            blocks: 2,
            position: 0,
            rate: 22_050,
            end: 64,
        };
        bytes[0..2].copy_from_slice(&0u16.to_le_bytes());
        bytes[2..4].copy_from_slice(&1u16.to_le_bytes());
        bytes[4] = 0x21;
        let pcm = decode_koe_entry(&bytes, entry).unwrap();
        assert_eq!(pcm.len(), 0x400 + 2);
        assert_eq!(pcm[0x400], linear_sample(1));
        assert_eq!(pcm[0x401], linear_sample(0));
    }

    #[test]
    fn wav_parser_finds_data_after_extra_chunks() {
        let mut wav = wrap_pcm_as_wav(11_025, 1, 8, &[0x80, 0x90]);
        // Insert a LIST chunk before data.
        let data_at = wav.len() - 2 - 8;
        let mut extra = b"LIST".to_vec();
        extra.extend_from_slice(&2u32.to_le_bytes());
        extra.extend_from_slice(&[0, 0]);
        wav.splice(data_at..data_at, extra);
        let clip = wav_clip(&wav, true).unwrap();
        assert!(clip.wav.len() > 44);
    }
}
