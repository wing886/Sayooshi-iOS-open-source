use crate::platform_time::{Duration, Instant};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
#[cfg(target_arch = "wasm32")]
use kira::sound::static_sound::{
    StaticSoundData as BgmSoundData, StaticSoundHandle as BgmSoundHandle,
};
#[cfg(not(target_arch = "wasm32"))]
type BgmSoundData = kira::sound::streaming::StreamingSoundData<kira::sound::FromFileError>;
#[cfg(not(target_arch = "wasm32"))]
type BgmSoundHandle = kira::sound::streaming::StreamingSoundHandle<kira::sound::FromFileError>;
use kira::Volume;
use kira::sound::{EndPosition, PlaybackPosition, Region};
use kira::tween::Tween;
use siglus_assets::gameexe::{GameexeConfig, GameexeDecodeOptions, decode_gameexe_dat_bytes};

use super::bgm::{BgmPlaybackFormat, decode_bgm_to_playback_bytes};
use super::{AudioHub, TrackKind};

const TNM_BGM_START_POS_INI: i64 = -1;
const TNM_BGM_PLAYER_CNT: usize = 2;

#[derive(Debug, Clone)]
struct WavPcmRegion {
    data_offset: usize,
    data_len: usize,
    data_len_field_offset: usize,
    riff_len_field_offset: usize,
    byte_rate: u32,
    block_align: usize,
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
    audio_format: u16,
}

impl WavPcmRegion {
    fn sample_count(&self) -> u64 {
        (self.data_len / self.block_align.max(1)) as u64
    }
}

fn wav_pcm_region(wav: &[u8]) -> Option<WavPcmRegion> {
    if wav.len() < 44 {
        return None;
    }
    if &wav[0..4] != b"RIFF" || &wav[8..12] != b"WAVE" {
        return None;
    }

    let mut pos = 12usize;
    let mut byte_rate: Option<u32> = None;
    let mut block_align: Option<usize> = None;
    let mut sample_rate: Option<u32> = None;
    let mut channels: Option<u16> = None;
    let mut bits_per_sample: Option<u16> = None;
    let mut audio_format: Option<u16> = None;
    let mut data_offset: Option<usize> = None;
    let mut data_len: Option<usize> = None;
    let mut data_len_field_offset: Option<usize> = None;

    while pos + 8 <= wav.len() {
        let id = &wav[pos..pos + 4];
        let sz =
            u32::from_le_bytes([wav[pos + 4], wav[pos + 5], wav[pos + 6], wav[pos + 7]]) as usize;
        let sz_field_off = pos + 4;
        pos += 8;
        if pos + sz > wav.len() {
            break;
        }
        if id == b"fmt " {
            if sz >= 16 {
                audio_format = Some(u16::from_le_bytes([wav[pos], wav[pos + 1]]));
                channels = Some(u16::from_le_bytes([wav[pos + 2], wav[pos + 3]]));
                sample_rate = Some(u32::from_le_bytes([
                    wav[pos + 4],
                    wav[pos + 5],
                    wav[pos + 6],
                    wav[pos + 7],
                ]));
                byte_rate = Some(u32::from_le_bytes([
                    wav[pos + 8],
                    wav[pos + 9],
                    wav[pos + 10],
                    wav[pos + 11],
                ]));
                block_align = Some(u16::from_le_bytes([wav[pos + 12], wav[pos + 13]]) as usize);
                bits_per_sample = Some(u16::from_le_bytes([wav[pos + 14], wav[pos + 15]]));
            }
        } else if id == b"data" {
            data_offset = Some(pos);
            data_len = Some(sz);
            data_len_field_offset = Some(sz_field_off);
        }
        pos += sz;
        if (sz & 1) != 0 {
            pos += 1;
        }
    }

    Some(WavPcmRegion {
        data_offset: data_offset?,
        data_len: data_len?,
        data_len_field_offset: data_len_field_offset?,
        riff_len_field_offset: 4,
        byte_rate: byte_rate?,
        block_align: block_align?.max(1),
        sample_rate: sample_rate?.max(1),
        channels: channels?.max(1),
        bits_per_sample: bits_per_sample?,
        audio_format: audio_format?,
    })
}

fn wav_slice_samples(wav: &[u8], start_sample: u64, end_sample: Option<u64>) -> Option<Vec<u8>> {
    let region = wav_pcm_region(wav)?;
    let total_samples = region.sample_count();
    let start_sample = start_sample.min(total_samples);
    let end_sample = end_sample
        .unwrap_or(total_samples)
        .min(total_samples)
        .max(start_sample);

    let start_byte = (start_sample as usize) * region.block_align;
    let end_byte = (end_sample as usize) * region.block_align;
    let src_begin = region.data_offset + start_byte;
    let src_end = region.data_offset + end_byte;
    if src_begin > wav.len() || src_end > wav.len() || src_begin > src_end {
        return None;
    }

    let mut out = wav.to_vec();
    out.splice(
        region.data_offset..region.data_offset + region.data_len,
        wav[src_begin..src_end].iter().copied(),
    );
    let new_data_len = (src_end - src_begin) as u32;
    out[region.data_len_field_offset..region.data_len_field_offset + 4]
        .copy_from_slice(&new_data_len.to_le_bytes());
    let riff_len = (out.len().saturating_sub(8)) as u32;
    out[region.riff_len_field_offset..region.riff_len_field_offset + 4]
        .copy_from_slice(&riff_len.to_le_bytes());
    Some(out)
}

fn parse_i64_like(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Ok(v) = s.parse::<i64>() {
        return Some(v);
    }
    if let Some(rest) = s.strip_prefix("0x") {
        return i64::from_str_radix(rest, 16).ok();
    }
    if let Some(rest) = s.strip_prefix("-0x") {
        return i64::from_str_radix(rest, 16).ok().map(|v| -v);
    }
    None
}

fn normalize_regist_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn clamp_sample_range(
    total_samples: u64,
    start_sample: i64,
    end_sample: i64,
    restart_sample: i64,
) -> (u64, u64, u64) {
    let total_samples_i64 = total_samples as i64;
    let end_sample = if end_sample < 0 {
        total_samples_i64
    } else {
        end_sample.clamp(0, total_samples_i64)
    };
    let start_sample = start_sample.clamp(0, end_sample);
    let restart_sample = restart_sample.clamp(0, end_sample);
    (
        start_sample as u64,
        end_sample as u64,
        restart_sample as u64,
    )
}

fn tween_ms(ms: i64) -> Tween {
    if ms > 0 {
        Tween {
            duration: Duration::from_millis(ms as u64),
            ..Tween::default()
        }
    } else {
        Tween::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingBgmActionKind {
    Stop,
    Pause,
}

#[derive(Debug, Clone, Copy)]
struct PendingBgmAction {
    kind: PendingBgmActionKind,
    at: Instant,
}

pub const TNM_PLAYER_STATE_FREE: i32 = 0;
pub const TNM_PLAYER_STATE_PLAY: i32 = 1;
pub const TNM_PLAYER_STATE_FADE_OUT: i32 = 2;
pub const TNM_PLAYER_STATE_PAUSE: i32 = 3;

#[derive(Debug, Clone)]
struct BgmScriptEntry {
    file_name: String,
    start_sample: i64,
    end_sample: i64,
    repeat_sample: i64,
}

#[derive(Debug, Default)]
struct BgmPlayerSlot {
    handle: Option<BgmSoundHandle>,
    source_bytes: Option<Arc<[u8]>>,
    /// The file an `NwaStream` source decodes from while playing.
    source_path: Option<PathBuf>,
    source_format: Option<BgmPlaybackFormat>,
    sample_rate_hz: u32,
    total_samples: u64,
    start_sample: u64,
    end_sample: u64,
    restart_sample: u64,
    current_segment_start_sample: u64,
    current_segment_samples: u64,
    start_time: Option<Instant>,
    paused_at: Option<Instant>,
    paused_total: Duration,
    pending: Option<PendingBgmAction>,
    fade_outing: bool,
    loop_flag: bool,
    name: Option<String>,
    file_name: Option<String>,
    ready_only: bool,
}

impl BgmPlayerSlot {
    fn reset_all(&mut self) {
        if let Some(mut h) = self.handle.take() {
            h.stop(Tween::default());
        }
        self.source_bytes = None;
        self.source_path = None;
        self.source_format = None;
        self.sample_rate_hz = 0;
        self.total_samples = 0;
        self.start_sample = 0;
        self.end_sample = 0;
        self.restart_sample = 0;
        self.current_segment_start_sample = 0;
        self.current_segment_samples = 0;
        self.start_time = None;
        self.paused_at = None;
        self.paused_total = Duration::from_millis(0);
        self.pending = None;
        self.fade_outing = false;
        self.loop_flag = false;
        self.name = None;
        self.file_name = None;
        self.ready_only = false;
    }

    fn clear_runtime_only(&mut self) {
        self.handle = None;
        self.current_segment_start_sample = self.start_sample;
        self.current_segment_samples = self.end_sample.saturating_sub(self.start_sample);
        self.start_time = None;
        self.paused_at = None;
        self.paused_total = Duration::from_millis(0);
        self.pending = None;
        self.fade_outing = false;
        self.ready_only = false;
    }

    fn elapsed_ms(&self) -> u64 {
        let Some(start) = self.start_time else {
            return 0;
        };
        let now = self.paused_at.unwrap_or_else(Instant::now);
        now.saturating_duration_since(start)
            .saturating_sub(self.paused_total)
            .as_millis() as u64
    }

    fn elapsed_samples(&self) -> u64 {
        if self.sample_rate_hz == 0 {
            return 0;
        }
        self.elapsed_ms().saturating_mul(self.sample_rate_hz as u64) / 1000
    }

    fn playback_window_samples(&self) -> u64 {
        self.end_sample.saturating_sub(self.start_sample)
    }

    fn loop_span_samples(&self) -> u64 {
        self.end_sample.saturating_sub(self.restart_sample)
    }

    fn has_loop_region(&self) -> bool {
        self.loop_flag && self.restart_sample < self.end_sample
    }

    fn playback_data(
        &self,
        effective_start: u64,
        amp: f64,
        fade_in_ms: i64,
    ) -> Result<BgmSoundData> {
        // The original player owns a C_sound_stream and decodes while playing.
        // Keep native playback streaming as well; the browser retains Kira's
        // static fallback because kira::sound::streaming is unavailable there.
        #[cfg(not(target_arch = "wasm32"))]
        let data = if let Some(path) = self
            .source_path
            .as_ref()
            .filter(|_| self.source_format == Some(BgmPlaybackFormat::NwaStream))
        {
            BgmSoundData::from_decoder(
                super::nwa_stream::NwaDecoder::open(path)
                    .with_context(|| format!("open NWA stream: {}", path.display()))?,
            )
        } else {
            let source = self
                .source_bytes
                .as_ref()
                .context("BGM slot not prepared")?;
            BgmSoundData::from_cursor(Cursor::new(Arc::clone(source)))
                .context("kira: prepare BGM playback stream")?
        };
        #[cfg(target_arch = "wasm32")]
        let data = {
            let source = self
                .source_bytes
                .as_ref()
                .context("BGM slot not prepared")?;
            BgmSoundData::from_cursor(Cursor::new(Arc::clone(source)))
                .context("kira: prepare BGM playback stream")?
        };
        let mut data = data
            .start_position(PlaybackPosition::Samples(effective_start as usize))
            .slice(Region {
                start: PlaybackPosition::Samples(0),
                end: EndPosition::Custom(PlaybackPosition::Samples(self.end_sample as usize)),
            })
            .volume(Volume::Amplitude(amp));
        if self.has_loop_region() {
            data = data.loop_region(Region {
                start: PlaybackPosition::Samples(self.restart_sample as usize),
                end: EndPosition::Custom(PlaybackPosition::Samples(self.end_sample as usize)),
            });
        }
        if fade_in_ms > 0 {
            data = data.fade_in_tween(tween_ms(fade_in_ms));
        }
        Ok(data)
    }

    fn play_pos_samples(&self) -> u64 {
        let elapsed = self.elapsed_samples();
        if self.has_loop_region() {
            let intro_samples = self.restart_sample.saturating_sub(self.start_sample);
            let loop_span = self.loop_span_samples();
            if loop_span == 0 {
                return self.end_sample;
            }
            if self.start_sample < self.restart_sample && elapsed < intro_samples {
                return self.start_sample + elapsed.min(intro_samples);
            }
            let loop_elapsed = if self.start_sample < self.restart_sample {
                elapsed.saturating_sub(intro_samples)
            } else {
                elapsed
            };
            return self.restart_sample + (loop_elapsed % loop_span);
        }
        self.start_sample + elapsed.min(self.playback_window_samples())
    }

    fn check_state(&self) -> i32 {
        if self.handle.is_none() {
            return TNM_PLAYER_STATE_FREE;
        }
        if self.paused_at.is_some() {
            return TNM_PLAYER_STATE_PAUSE;
        }
        if self.fade_outing {
            return TNM_PLAYER_STATE_FADE_OUT;
        }
        TNM_PLAYER_STATE_PLAY
    }

    fn is_playing(&self) -> bool {
        self.handle.is_some() && !self.fade_outing && self.paused_at.is_none()
    }
}

pub struct BgmEngine {
    project_dir: PathBuf,
    current_append_dir: String,
    game_volume_raw: u8,
    system_volume_raw: u8,
    current_name: Option<String>,
    current_player_id: Option<usize>,
    players: Vec<BgmPlayerSlot>,
    retired: Vec<(BgmSoundHandle, Instant)>,
    delay_deadline: Option<Instant>,
    delayed_fade_in_ms: i64,
    loop_flag: bool,
    pause_flag: bool,
}

impl std::fmt::Debug for BgmEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BgmEngine")
            .field("game_volume_raw", &self.game_volume_raw)
            .field("system_volume_raw", &self.system_volume_raw)
            .field("current_name", &self.current_name)
            .field("current_player_id", &self.current_player_id)
            .field("loop_flag", &self.loop_flag)
            .field("pause_flag", &self.pause_flag)
            .finish()
    }
}

impl Drop for BgmEngine {
    fn drop(&mut self) {
        // Native streaming decoders can outlive their handles. Stop them while
        // CommandContext still owns the AudioHub that receives stop commands.
        let immediate = Tween {
            duration: Duration::ZERO,
            ..Tween::default()
        };
        for slot in &mut self.players {
            if let Some(handle) = slot.handle.as_mut() {
                handle.stop(immediate);
            }
        }
        for (handle, _) in &mut self.retired {
            handle.stop(immediate);
        }
    }
}

impl BgmEngine {
    pub fn new(project_dir: PathBuf) -> Self {
        Self {
            project_dir,
            current_append_dir: String::new(),
            game_volume_raw: 255,
            system_volume_raw: 255,
            current_name: None,
            current_player_id: None,
            players: (0..TNM_BGM_PLAYER_CNT)
                .map(|_| BgmPlayerSlot::default())
                .collect(),
            retired: Vec::new(),
            delay_deadline: None,
            delayed_fade_in_ms: 0,
            loop_flag: false,
            pause_flag: false,
        }
    }

    pub fn current_name(&self) -> Option<&str> {
        self.current_name.as_deref()
    }

    /// Bytes currently retained directly by BGM player slots.  This is a
    /// bookkeeping query only; it does not touch Kira or decode audio.  Shared
    /// Arc payloads are counted once.
    pub fn debug_source_memory(&self) -> (usize, usize) {
        let mut seen = [std::ptr::null::<u8>(); TNM_BGM_PLAYER_CNT];
        let mut seen_len = 0usize;
        let mut bytes = 0usize;
        let mut slots = 0usize;
        for slot in &self.players {
            let Some(source) = slot.source_bytes.as_ref() else {
                continue;
            };
            slots += 1;
            let ptr = source.as_ptr();
            if seen[..seen_len].contains(&ptr) {
                continue;
            }
            if seen_len < seen.len() {
                seen[seen_len] = ptr;
                seen_len += 1;
            }
            bytes = bytes.saturating_add(source.len());
        }
        (bytes, slots)
    }

    pub fn set_current_append_dir(&mut self, append_dir: impl Into<String>) {
        let append_dir = append_dir.into();
        if self.current_append_dir != append_dir {
            self.current_append_dir = append_dir;
        }
    }

    pub fn set_current_append_dir_ref(&mut self, append_dir: &str) {
        if self.current_append_dir != append_dir {
            self.current_append_dir.clear();
            self.current_append_dir.push_str(append_dir);
        }
    }

    pub fn volume_raw(&self) -> u8 {
        self.game_volume_raw
    }
    pub fn save_loop_flag(&self) -> bool {
        self.loop_flag
    }

    pub fn save_pause_flag(&self) -> bool {
        self.pause_flag
    }

    pub fn save_delay_time_ms(&self) -> i64 {
        self.delay_deadline
            .map(|deadline| {
                deadline
                    .saturating_duration_since(Instant::now())
                    .as_millis()
                    .min(i64::MAX as u128) as i64
            })
            .unwrap_or(0)
    }

    fn total_gain_amplitude(&self) -> f64 {
        (self.game_volume_raw as f64 / 255.0) * (self.system_volume_raw as f64 / 255.0)
    }

    fn apply_all_active_volumes(&mut self, fade_ms: i64) {
        let amp = self.total_gain_amplitude();
        let tween = tween_ms(fade_ms);
        for slot in &mut self.players {
            if let Some(h) = &mut slot.handle {
                h.set_volume(Volume::Amplitude(amp), tween);
            }
        }
    }

    pub fn set_volume_raw(&mut self, _audio: &mut AudioHub, volume_raw: u8) -> Result<()> {
        self.game_volume_raw = volume_raw;
        self.apply_all_active_volumes(0);
        Ok(())
    }

    pub fn set_volume_raw_fade(
        &mut self,
        _audio: &mut AudioHub,
        volume_raw: u8,
        fade_ms: i64,
    ) -> Result<()> {
        self.game_volume_raw = volume_raw;
        self.apply_all_active_volumes(fade_ms);
        Ok(())
    }

    pub fn set_system_volume_raw(&mut self, volume_raw: u8) {
        self.system_volume_raw = volume_raw;
        self.apply_all_active_volumes(0);
    }

    pub fn set_looping(&mut self, looping: bool) -> Result<()> {
        self.loop_flag = looping;
        Ok(())
    }

    pub fn check_state(&self) -> i32 {
        self.current_slot()
            .map(|slot| slot.check_state())
            .unwrap_or(TNM_PLAYER_STATE_FREE)
    }

    pub fn is_playing(&self) -> bool {
        self.current_slot()
            .map(|slot| slot.is_playing())
            .unwrap_or(false)
    }

    pub fn is_fade_out_doing(&self) -> bool {
        self.check_state() == TNM_PLAYER_STATE_FADE_OUT
    }

    pub fn can_wait(&self) -> bool {
        self.is_playing() && !self.loop_flag
    }

    fn current_slot(&self) -> Option<&BgmPlayerSlot> {
        self.current_player_id.and_then(|id| self.players.get(id))
    }

    fn current_slot_mut(&mut self) -> Option<&mut BgmPlayerSlot> {
        self.current_player_id
            .and_then(|id| self.players.get_mut(id))
    }

    fn lookup_gameexe_bgm_entry(&self, regist_name: &str) -> Option<BgmScriptEntry> {
        let cfg = load_gameexe_config(&self.project_dir)?;
        let target = normalize_regist_name(regist_name);
        let cnt = cfg.indexed_count("BGM");
        for i in 0..cnt {
            let Some(key_name) = cfg.get_indexed_item_unquoted("BGM", i, 0) else {
                continue;
            };
            if normalize_regist_name(key_name) != target {
                continue;
            }
            let file_name = cfg
                .get_indexed_item_unquoted("BGM", i, 1)?
                .trim()
                .to_string();
            if file_name.is_empty() {
                continue;
            }
            let start_sample = cfg
                .get_indexed_item_unquoted("BGM", i, 2)
                .and_then(parse_i64_like)
                .unwrap_or(0);
            let end_sample = cfg
                .get_indexed_item_unquoted("BGM", i, 3)
                .and_then(parse_i64_like)
                .unwrap_or(-1);
            let repeat_sample = cfg
                .get_indexed_item_unquoted("BGM", i, 4)
                .and_then(parse_i64_like)
                .unwrap_or(0);
            return Some(BgmScriptEntry {
                file_name,
                start_sample,
                end_sample,
                repeat_sample,
            });
        }
        None
    }

    fn resolve_bgm_script(&self, regist_name: &str) -> Result<(BgmScriptEntry, PathBuf)> {
        if let Some(entry) = self.lookup_gameexe_bgm_entry(regist_name) {
            let (path, _ty) = crate::resource::find_audio_path_with_append_dir(
                &self.project_dir,
                &self.current_append_dir,
                "bgm",
                &entry.file_name,
            )
            .map_err(|err| {
                anyhow!(
                    "BGM file not found for regist name {}: {}; {}",
                    regist_name,
                    entry.file_name,
                    err
                )
            })?;
            return Ok((entry, path));
        }

        let direct_name = regist_name.trim();
        let (path, _ty) = crate::resource::find_audio_path_with_append_dir(
            &self.project_dir,
            &self.current_append_dir,
            "bgm",
            direct_name,
        )
        .map_err(|err| {
            anyhow!("BGM regist name not found in script table: {regist_name}; {err}")
        })?;
        Ok((
            BgmScriptEntry {
                file_name: direct_name.to_string(),
                start_sample: 0,
                end_sample: -1,
                repeat_sample: 0,
            },
            path,
        ))
    }

    fn prepare_slot(
        &mut self,
        slot_id: usize,
        regist_name: &str,
        loop_flag: bool,
        start_pos_sample: i64,
        ready_only: bool,
    ) -> Result<()> {
        let (script_entry, path) = self.resolve_bgm_script(regist_name)?;

        let decoded = decode_bgm_to_playback_bytes(&path, None)
            .with_context(|| format!("prepare BGM playback: {}", path.display()))?;

        let total_samples = decoded.total_samples;

        let script_start = script_entry.start_sample;
        let script_end = script_entry.end_sample;
        let script_repeat = script_entry.repeat_sample;
        let effective_start = if start_pos_sample == TNM_BGM_START_POS_INI {
            script_start
        } else {
            start_pos_sample
        };
        let (start_sample, end_sample, restart_sample) =
            clamp_sample_range(total_samples, effective_start, script_end, script_repeat);

        if env_is_set!("SG_AUDIO_TRACE") {
            eprintln!(
                "[SG_AUDIO_TRACE] bgm.prepare name={} file={} source_format={:?} container={:?} channels={} sample_rate={} total_frames={} start={} end={} repeat={}",
                regist_name,
                path.display(),
                decoded.format,
                decoded.container,
                decoded.channels,
                decoded.sample_rate,
                total_samples,
                start_sample,
                end_sample,
                restart_sample
            );
        }

        let slot = &mut self.players[slot_id];
        slot.reset_all();
        slot.source_bytes = (decoded.stream_path.is_none()).then(|| decoded.bytes.into());
        slot.source_path = decoded.stream_path;
        slot.source_format = Some(decoded.format);
        slot.sample_rate_hz = decoded.sample_rate;
        slot.total_samples = total_samples;
        slot.start_sample = start_sample;
        slot.end_sample = end_sample;
        slot.restart_sample = restart_sample;
        slot.current_segment_start_sample = start_sample;
        slot.current_segment_samples = end_sample.saturating_sub(start_sample);
        slot.loop_flag = loop_flag;
        slot.name = Some(regist_name.to_string());
        slot.file_name = Some(path.to_string_lossy().to_string());
        slot.ready_only = ready_only;
        Ok(())
    }

    fn start_slot_internal(
        &mut self,
        audio: &mut AudioHub,
        slot_id: usize,
        fade_in_ms: i64,
        start_paused: bool,
    ) -> Result<()> {
        let amp = self.total_gain_amplitude();
        let slot = &mut self.players[slot_id];
        if slot.source_bytes.is_none() && slot.source_path.is_none() {
            return Err(anyhow!("BGM slot not prepared"));
        }

        let mut effective_start = slot.start_sample;
        if effective_start >= slot.end_sample {
            if slot.has_loop_region() {
                effective_start = slot.restart_sample;
            } else {
                return Err(anyhow!("invalid BGM range: start >= end"));
            }
        }
        if effective_start >= slot.end_sample {
            return Err(anyhow!("invalid BGM range after restart clamp"));
        }

        if !audio.is_enabled() {
            slot.handle = None;
            slot.current_segment_start_sample = effective_start;
            slot.current_segment_samples = slot.end_sample.saturating_sub(effective_start);
            slot.start_time = Some(Instant::now());
            slot.paused_at = if start_paused { slot.start_time } else { None };
            slot.paused_total = Duration::from_millis(0);
            slot.pending = None;
            slot.fade_outing = false;
            slot.start_sample = effective_start;
            slot.ready_only = start_paused;
            return Ok(());
        }

        let data = slot.playback_data(
            effective_start,
            amp,
            if start_paused { 0 } else { fade_in_ms },
        )?;
        #[cfg(not(target_arch = "wasm32"))]
        let mut handle = audio.play_streaming(TrackKind::Bgm, data)?;
        #[cfg(target_arch = "wasm32")]
        let mut handle = audio.play_static(TrackKind::Bgm, data)?;

        if start_paused {
            handle.pause(Tween::default());
        }

        slot.handle = Some(handle);
        slot.current_segment_start_sample = effective_start;
        slot.current_segment_samples = slot.end_sample.saturating_sub(effective_start);
        slot.start_time = Some(Instant::now());
        slot.paused_at = if start_paused { slot.start_time } else { None };
        slot.paused_total = Duration::from_millis(0);
        slot.pending = None;
        slot.fade_outing = false;
        slot.start_sample = effective_start;
        slot.ready_only = start_paused;
        Ok(())
    }

    fn start_slot(&mut self, audio: &mut AudioHub, slot_id: usize, fade_in_ms: i64) -> Result<()> {
        self.start_slot_internal(audio, slot_id, fade_in_ms, false)
    }

    fn ready_slot(&mut self, audio: &mut AudioHub, slot_id: usize) -> Result<()> {
        self.start_slot_internal(audio, slot_id, 0, true)
    }

    fn handoff_current_to_retired(&mut self, fade_out_ms: i64) {
        let Some(cur_id) = self.current_player_id else {
            return;
        };
        let slot = &mut self.players[cur_id];
        if let Some(mut h) = slot.handle.take() {
            if fade_out_ms > 0 {
                h.stop(tween_ms(fade_out_ms));
                self.retired.push((
                    h,
                    Instant::now() + Duration::from_millis(fade_out_ms as u64),
                ));
            } else {
                h.stop(Tween::default());
            }
        }
        slot.clear_runtime_only();
    }

    pub fn play_name_script(
        &mut self,
        audio: &mut AudioHub,
        name: &str,
        loop_flag: bool,
        fade_in_ms: i64,
        fade_out_ms: i64,
        start_pos_sample: i64,
        ready_only: bool,
        delay_time_ms: i64,
    ) -> Result<()> {
        let regist_name = normalize_regist_name(name);
        if self.current_name.as_deref() == Some(regist_name.as_str()) && self.loop_flag && loop_flag
        {
            return Ok(());
        }

        self.handoff_current_to_retired(fade_out_ms);
        let next_id = match self.current_player_id {
            Some(id) => (id + 1) % TNM_BGM_PLAYER_CNT,
            None => 0,
        };
        let total_ready_only = ready_only || delay_time_ms > 0;
        self.prepare_slot(
            next_id,
            &regist_name,
            loop_flag,
            start_pos_sample,
            total_ready_only,
        )?;
        self.current_player_id = Some(next_id);
        self.current_name = Some(regist_name);
        self.loop_flag = loop_flag;
        self.pause_flag = ready_only;
        self.delayed_fade_in_ms = fade_in_ms;
        self.delay_deadline = if delay_time_ms > 0 {
            Some(Instant::now() + Duration::from_millis(delay_time_ms.max(0) as u64))
        } else {
            None
        };

        if total_ready_only {
            self.ready_slot(audio, next_id)?;
        } else {
            self.start_slot(audio, next_id, fade_in_ms)?;
        }
        Ok(())
    }

    pub fn ready_name(
        &mut self,
        audio: &mut AudioHub,
        name: &str,
        start_pos_sample: i64,
    ) -> Result<()> {
        self.play_name_script(audio, name, self.loop_flag, 0, 0, start_pos_sample, true, 0)
    }

    pub fn play_name_with_options(
        &mut self,
        audio: &mut AudioHub,
        name: &str,
        start_pos_sample: i64,
        fade_in_ms: i64,
    ) -> Result<()> {
        self.play_name_script(
            audio,
            name,
            self.loop_flag,
            fade_in_ms,
            0,
            start_pos_sample,
            false,
            0,
        )
    }

    pub fn play_name(&mut self, audio: &mut AudioHub, name: &str) -> Result<()> {
        self.play_name_script(audio, name, true, 0, 0, TNM_BGM_START_POS_INI, false, 0)
    }

    pub fn play_pos_samples(&self) -> u64 {
        self.current_slot()
            .map(|s| s.play_pos_samples())
            .unwrap_or(0)
    }

    pub fn pause_fade(&mut self, _audio: &mut AudioHub, fade_ms: i64) -> Result<()> {
        self.delay_deadline = None;
        self.pause_flag = true;
        let amp = self.total_gain_amplitude();
        let Some(slot) = self.current_slot_mut() else {
            return Ok(());
        };
        if slot.handle.is_none() || slot.paused_at.is_some() {
            return Ok(());
        }
        if fade_ms > 0 {
            if let Some(h) = &mut slot.handle {
                h.set_volume(Volume::Amplitude(amp), Tween::default());
                h.set_volume(Volume::Amplitude(0.0), tween_ms(fade_ms));
            }
            slot.fade_outing = true;
            slot.pending = Some(PendingBgmAction {
                kind: PendingBgmActionKind::Pause,
                at: Instant::now() + Duration::from_millis(fade_ms as u64),
            });
        } else {
            if let Some(h) = &mut slot.handle {
                h.pause(Tween::default());
            }
            slot.paused_at = Some(Instant::now());
        }
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        self.delay_deadline = None;
        self.pause_flag = true;
        let Some(slot) = self.current_slot_mut() else {
            return Ok(());
        };
        if slot.handle.is_none() || slot.paused_at.is_some() {
            return Ok(());
        }
        if let Some(h) = &mut slot.handle {
            h.pause(Tween::default());
        }
        slot.paused_at = Some(Instant::now());
        Ok(())
    }

    pub fn resume_script(
        &mut self,
        audio: &mut AudioHub,
        fade_in_ms: i64,
        delay_time_ms: i64,
    ) -> Result<()> {
        if delay_time_ms > 0 {
            self.delay_deadline =
                Some(Instant::now() + Duration::from_millis(delay_time_ms as u64));
            self.delayed_fade_in_ms = fade_in_ms;
            self.pause_flag = false;
            return Ok(());
        }

        let amp = self.total_gain_amplitude();
        let Some(cur_id) = self.current_player_id else {
            return Ok(());
        };
        if self.players[cur_id].handle.is_none() {
            self.start_slot(audio, cur_id, fade_in_ms)?;
            self.pause_flag = false;
            self.delay_deadline = None;
            return Ok(());
        }

        let slot = &mut self.players[cur_id];
        if let Some(p) = slot.paused_at.take() {
            slot.paused_total += Instant::now().saturating_duration_since(p);
        }
        if let Some(h) = &mut slot.handle {
            h.resume(Tween::default());
            if fade_in_ms > 0 {
                h.set_volume(Volume::Amplitude(0.0), Tween::default());
                h.set_volume(Volume::Amplitude(amp), tween_ms(fade_in_ms));
            } else {
                h.set_volume(Volume::Amplitude(amp), Tween::default());
            }
        }
        slot.fade_outing = false;
        slot.pending = None;
        slot.ready_only = false;
        self.pause_flag = false;
        self.delay_deadline = None;
        Ok(())
    }

    pub fn resume_fade(&mut self, audio: &mut AudioHub, fade_ms: i64) -> Result<()> {
        self.resume_script(audio, fade_ms, 0)
    }

    pub fn resume(&mut self) -> Result<()> {
        let amp = self.total_gain_amplitude();
        let Some(slot) = self.current_slot_mut() else {
            return Ok(());
        };
        if let Some(p) = slot.paused_at.take() {
            slot.paused_total += Instant::now().saturating_duration_since(p);
            if let Some(h) = &mut slot.handle {
                h.resume(Tween::default());
                h.set_volume(Volume::Amplitude(amp), Tween::default());
            }
        }
        slot.ready_only = false;
        self.pause_flag = false;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.stop_current_internal(0)
    }

    fn stop_current_internal(&mut self, fade_out_ms: i64) -> Result<()> {
        self.delay_deadline = None;
        if let Some(slot) = self.current_slot_mut() {
            if slot.handle.is_none() {
                slot.clear_runtime_only();
            } else if fade_out_ms > 0 {
                if let Some(h) = &mut slot.handle {
                    h.stop(tween_ms(fade_out_ms));
                }
                slot.fade_outing = true;
                slot.pending = Some(PendingBgmAction {
                    kind: PendingBgmActionKind::Stop,
                    at: Instant::now() + Duration::from_millis(fade_out_ms as u64),
                });
            } else {
                if let Some(mut h) = slot.handle.take() {
                    h.stop(Tween::default());
                }
                slot.clear_runtime_only();
            }
        }
        self.current_name = None;
        self.loop_flag = false;
        self.pause_flag = false;
        Ok(())
    }

    pub fn stop_fade(&mut self, fade_out_ms: i64) -> Result<()> {
        self.stop_current_internal(fade_out_ms)
    }

    pub fn tick(&mut self, audio: &mut AudioHub) -> Result<()> {
        let now = Instant::now();
        self.retired.retain_mut(|(h, deadline)| {
            if now >= *deadline {
                h.stop(Tween::default());
                false
            } else {
                true
            }
        });

        if let Some(deadline) = self.delay_deadline
            && now >= deadline
        {
            self.delay_deadline = None;
            self.resume_script(audio, self.delayed_fade_in_ms, 0)?;
        }

        let Some(cur_id) = self.current_player_id else {
            return Ok(());
        };

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(err) = self.players[cur_id]
            .handle
            .as_mut()
            .and_then(|h| h.pop_error())
        {
            self.stop_current_internal(0)?;
            return Err(err).context("kira: decode BGM stream");
        }

        let pending = self.players[cur_id].pending;
        if let Some(pending) = pending
            && now >= pending.at
        {
            let slot = &mut self.players[cur_id];
            slot.pending = None;
            match pending.kind {
                PendingBgmActionKind::Stop => {
                    if let Some(mut h) = slot.handle.take() {
                        h.stop(Tween::default());
                    }
                    slot.clear_runtime_only();
                }
                PendingBgmActionKind::Pause => {
                    if let Some(h) = &mut slot.handle {
                        h.pause(Tween::default());
                    }
                    slot.paused_at = Some(now);
                    slot.fade_outing = false;
                }
            }
        }

        let (paused, has_handle, segment_samples, elapsed_samples, has_loop_region) = {
            let slot = &self.players[cur_id];
            (
                slot.paused_at.is_some(),
                slot.handle.is_some(),
                slot.playback_window_samples(),
                slot.elapsed_samples(),
                slot.has_loop_region(),
            )
        };
        if paused || !has_handle {
            return Ok(());
        }
        if segment_samples == 0 {
            return Ok(());
        }
        if elapsed_samples < segment_samples {
            return Ok(());
        }
        if has_loop_region {
            return Ok(());
        } else {
            let slot = &mut self.players[cur_id];
            if let Some(mut h) = slot.handle.take() {
                h.stop(Tween::default());
            }
            slot.clear_runtime_only();
        }
        Ok(())
    }
}

fn path_is_file(path: &Path) -> bool {
    crate::resource::game_file_exists(path)
}

fn load_gameexe_decode_options(project_dir: &Path) -> Result<GameexeDecodeOptions> {
    crate::resource::load_gameexe_decode_options(project_dir)
}

fn find_gameexe_path(project_dir: &Path) -> Option<PathBuf> {
    const CANDIDATES: &[&str] = &[
        "Gameexe.dat",
        "Gameexe.ini",
        "gameexe.dat",
        "gameexe.ini",
        "GameexeEN.dat",
        "GameexeEN.ini",
        "GameexeZH.dat",
        "GameexeZH.ini",
        "GameexeZHTW.dat",
        "GameexeZHTW.ini",
        "GameexeDE.dat",
        "GameexeDE.ini",
        "GameexeES.dat",
        "GameexeES.ini",
        "GameexeFR.dat",
        "GameexeFR.ini",
        "GameexeID.dat",
        "GameexeID.ini",
    ];
    for name in CANDIDATES {
        let p = project_dir.join(name);
        if path_is_file(&p) {
            return Some(p);
        }
    }
    None
}

fn load_gameexe_config(project_dir: &Path) -> Option<GameexeConfig> {
    let path = find_gameexe_path(project_dir)?;
    let raw = crate::resource::read_file_bytes(&path).ok()?;
    if path
        .extension()
        .and_then(|s| s.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("ini"))
    {
        let text = String::from_utf8(raw).ok()?;
        return Some(GameexeConfig::from_text(&text));
    }
    let opt = load_gameexe_decode_options(project_dir).ok()?;
    let (text, _report) = decode_gameexe_dat_bytes(&raw, &opt).ok()?;
    Some(GameexeConfig::from_text(&text))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod streaming_tests {
    use super::*;

    fn wav_source() -> Arc<[u8]> {
        let data_len = 48000u32 * 2;
        let mut wav = Vec::new();
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(36 + data_len).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&48000u32.to_le_bytes());
        wav.extend_from_slice(&96000u32.to_le_bytes());
        wav.extend_from_slice(&2u16.to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_len.to_le_bytes());
        wav.resize(44 + data_len as usize, 0);
        wav.into()
    }

    #[test]
    fn native_bgm_keeps_shared_encoded_source_and_sample_ranges() {
        let source = wav_source();
        let slot = BgmPlayerSlot {
            source_bytes: Some(Arc::clone(&source)),
            end_sample: 40000,
            restart_sample: 12000,
            loop_flag: true,
            ..Default::default()
        };
        let data = slot.playback_data(6000, 0.5, 250).unwrap();
        assert_eq!(Arc::strong_count(&source), 3);
        assert_eq!(
            data.settings.start_position,
            PlaybackPosition::Samples(6000)
        );
        assert_eq!(data.slice, Some((0, 40000)));
        assert_eq!(data.num_frames(), 40000);
        assert_eq!(
            data.settings.loop_region,
            Some(Region {
                start: PlaybackPosition::Samples(12000),
                end: EndPosition::Custom(PlaybackPosition::Samples(40000)),
            })
        );
        assert_eq!(
            data.settings.fade_in_tween.unwrap().duration,
            Duration::from_millis(250)
        );
        drop(data);
        assert_eq!(Arc::strong_count(&source), 2);
    }

    #[test]
    fn oneshot_stops_at_script_end_without_loop_or_implicit_fade() {
        let slot = BgmPlayerSlot {
            source_bytes: Some(wav_source()),
            end_sample: 24000,
            ..Default::default()
        };
        let data = slot.playback_data(12000, 1.0, 0).unwrap();
        assert_eq!(data.slice, Some((0, 24000)));
        assert!(data.settings.loop_region.is_none());
        assert!(data.settings.fade_in_tween.is_none());
    }

    #[test]
    fn streaming_bgm_preserves_ready_pause_resume_and_retire_lifecycle() {
        let mut audio = AudioHub::new();
        let mut bgm = BgmEngine::new(std::env::temp_dir());
        bgm.players[0] = BgmPlayerSlot {
            source_bytes: Some(wav_source()),
            sample_rate_hz: 48000,
            total_samples: 48000,
            end_sample: 48000,
            loop_flag: true,
            ..Default::default()
        };
        bgm.current_player_id = Some(0);
        bgm.current_name = Some("test".to_owned());
        bgm.loop_flag = true;

        bgm.ready_slot(&mut audio, 0).unwrap();
        assert_eq!(bgm.check_state(), TNM_PLAYER_STATE_PAUSE);
        assert_eq!(bgm.play_pos_samples(), 0);
        bgm.resume_script(&mut audio, 20, 0).unwrap();
        assert_eq!(bgm.check_state(), TNM_PLAYER_STATE_PLAY);
        bgm.pause_fade(&mut audio, 20).unwrap();
        assert_eq!(bgm.check_state(), TNM_PLAYER_STATE_FADE_OUT);
        bgm.players[0].pending.as_mut().unwrap().at = Instant::now();
        bgm.tick(&mut audio).unwrap();
        assert_eq!(bgm.check_state(), TNM_PLAYER_STATE_PAUSE);
        bgm.resume_script(&mut audio, 0, 0).unwrap();
        bgm.handoff_current_to_retired(20);
        assert_eq!(bgm.retired.len(), 1);
        assert_eq!(bgm.check_state(), TNM_PLAYER_STATE_FREE);
        bgm.retired[0].1 = Instant::now();
        bgm.tick(&mut audio).unwrap();
        assert!(bgm.retired.is_empty());
        bgm.start_slot(&mut audio, 0, 0).unwrap();
        bgm.stop().unwrap();
        assert_eq!(bgm.check_state(), TNM_PLAYER_STATE_FREE);
        assert!(bgm.current_name().is_none());
    }

    fn check_context_releases_streaming_decoder(restart: bool) {
        let source = wav_source();
        let mut ctx = crate::runtime::CommandContext::new(std::env::temp_dir());
        ctx.bgm.players[0] = BgmPlayerSlot {
            source_bytes: Some(Arc::clone(&source)),
            sample_rate_hz: 48000,
            total_samples: 48000,
            end_sample: 48000,
            loop_flag: true,
            ..Default::default()
        };
        ctx.bgm.current_player_id = Some(0);
        ctx.bgm.start_slot(&mut ctx.audio, 0, 0).unwrap();
        std::thread::sleep(Duration::from_millis(20));
        if restart {
            ctx.reset_for_scene_restart();
        } else {
            drop(ctx);
        }
        let deadline = Instant::now() + Duration::from_millis(500);
        while Arc::strong_count(&source) > 1 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(
            Arc::strong_count(&source),
            1,
            "old BGM decoder leaked after shutdown"
        );
    }

    #[test]
    fn restarting_context_releases_old_streaming_decoder() {
        check_context_releases_streaming_decoder(true);
    }

    #[test]
    fn dropping_context_releases_old_streaming_decoder() {
        check_context_releases_streaming_decoder(false);
    }
}
