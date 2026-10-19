//! MPEG-1/2 movie playback (`movPlay` and friends).
//!
//! Video is decoded on a worker thread and handed over through a small
//! bounded channel, so memory stays flat however long the movie is. The
//! audio track is decoded separately (MPEG audio decodes far faster than
//! real time) and the movie clock starts when it begins to play, keeping
//! picture and sound together.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError, sync_channel};

use crate::sound::Pcm;
use crate::surface::{Rect, Surface};

const CHUNK: usize = 256 * 1024;
/// Longest wait for the audio track before starting without it.
const AUDIO_WAIT_MS: u64 = 3000;

struct VideoFrame {
    pts_ms: i64,
    surface: Surface,
}

pub struct Movie {
    pub path: PathBuf,
    /// Where the picture goes (the whole screen when empty).
    pub dest: Option<Rect>,
    pub looped: bool,
    video: Receiver<VideoFrame>,
    audio: Option<Receiver<Option<Pcm>>>,
    stop: Arc<AtomicBool>,
    created: u64,
    started: Option<u64>,
    pending: Option<VideoFrame>,
    first_pts: Option<i64>,
    pub current: Option<Rc<Surface>>,
    video_done: bool,
    /// The audio track, once decoded, for the sound system to start.
    pub audio_ready: Option<Pcm>,
}

impl std::fmt::Debug for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Movie")
            .field("path", &self.path)
            .field("started", &self.started)
            .finish_non_exhaustive()
    }
}

/// Frame duration and picture size from the sequence header (decoded
/// frames are padded to whole macroblocks).
fn stream_info(path: &Path) -> (f64, Option<(i32, i32)>) {
    let mut head = vec![0; 64 * 1024];
    let read = game_fs::open(path)
        .and_then(|mut f| f.read(&mut head))
        .unwrap_or(0);
    let header = siglus_assets::mpeg2::find_sequence_header(&head[..read]);
    let duration = header
        .as_ref()
        .and_then(|h| siglus_assets::mpeg2::fps_from_frame_rate_code(h.frame_rate_code))
        .map_or(1000.0 / 29.97, |fps| 1000.0 / f64::from(fps));
    (
        duration,
        header.map(|h| (i32::from(h.width), i32::from(h.height))),
    )
}

fn spawn_video(path: PathBuf, stop: Arc<AtomicBool>) -> Receiver<VideoFrame> {
    let (tx, rx) = sync_channel(4);
    run_worker(move || {
        let (per_frame, size) = stream_info(&path);
        let Ok(mut file) = game_fs::open(&path) else {
            return;
        };
        let mut pipeline = na_mpeg2_decoder::MpegVideoPipeline::new();
        let mut index = 0u64;
        let mut buffer = vec![0; CHUNK];
        let mut send = |frame: &na_mpeg2_decoder::Frame| -> bool {
            let (w, h) = (frame.width, frame.height);
            let mut surface = Surface::new(w as i32, h as i32);
            na_mpeg2_decoder::frame_to_rgba_bt601_limited(frame, &mut surface.rgba);
            if let Some((width, height)) =
                size.filter(|&(w, h)| w < surface.width || h < surface.height)
            {
                surface = surface.crop(Rect::new(0, 0, width, height));
            }
            let pts_ms = frame
                .pts_90k
                .map_or((index as f64 * per_frame) as i64, |pts| pts / 90);
            index += 1;
            tx.send(VideoFrame { pts_ms, surface }).is_ok()
        };
        let mut alive = true;
        while alive && !stop.load(Ordering::Relaxed) {
            let read = match file.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(read) => read,
            };
            let result = pipeline.push_with(&buffer[..read], None, |frame| {
                alive &= send(&frame);
            });
            if result.is_err() {
                break;
            }
        }
        if alive && !stop.load(Ordering::Relaxed) {
            let _ = pipeline.flush_with(|frame| {
                send(&frame);
            });
        }
    });
    rx
}

fn spawn_audio(path: PathBuf, stop: Arc<AtomicBool>) -> Receiver<Option<Pcm>> {
    let (tx, rx) = sync_channel(1);
    run_worker(move || {
        let Ok(mut file) = game_fs::open(&path) else {
            let _ = tx.send(None);
            return;
        };
        let mut pipeline = na_mpeg2_decoder::MpegAudioPipeline::new();
        let mut buffer = vec![0; CHUNK];
        let mut pcm = Pcm::default();
        loop {
            if stop.load(Ordering::Relaxed) {
                return;
            }
            let read = match file.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(read) => read,
            };
            let _ = pipeline.push_with(&buffer[..read], None, |chunk| {
                pcm.rate = chunk.sample_rate;
                pcm.channels = chunk.channels;
                pcm.samples.extend(
                    chunk
                        .samples
                        .iter()
                        .map(|&s| (s.clamp(-1.0, 1.0) * 32767.0) as i16),
                );
            });
        }
        let _ = tx.send((!pcm.samples.is_empty()).then_some(pcm));
    });
    rx
}

/// Movies decode on worker threads.  The browser build has none, so there
/// the worker is dropped and the movie ends as soon as it starts.
fn run_worker(work: impl FnOnce() + Send + 'static) {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    std::thread::spawn(work);
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    drop(work);
}

impl Movie {
    pub fn open(path: PathBuf, dest: Option<Rect>, looped: bool, now: u64) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        Self {
            video: spawn_video(path.clone(), stop.clone()),
            audio: Some(spawn_audio(path.clone(), stop.clone())),
            path,
            dest,
            looped,
            stop,
            created: now,
            started: None,
            pending: None,
            first_pts: None,
            current: None,
            video_done: false,
            audio_ready: None,
        }
    }

    pub fn started(&self) -> bool {
        self.started.is_some()
    }

    /// Advances to `now`; false once the movie has ended.
    pub fn update(&mut self, now: u64) -> bool {
        if self.started.is_none() {
            let audio = self.audio.as_ref().map(Receiver::try_recv);
            match audio {
                Some(Ok(pcm)) => {
                    self.audio = None;
                    self.audio_ready = pcm;
                    self.started = Some(now);
                }
                Some(Err(TryRecvError::Empty)) if now < self.created + AUDIO_WAIT_MS => {
                    return true;
                }
                _ => {
                    self.audio = None;
                    self.started = Some(now);
                }
            }
        }
        let elapsed = (now - self.started.expect("started")) as i64;
        loop {
            if self.pending.is_none() {
                match self.video.try_recv() {
                    Ok(frame) => self.pending = Some(frame),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        self.video_done = true;
                        break;
                    }
                }
            }
            let Some(frame) = &self.pending else { break };
            let first = *self.first_pts.get_or_insert(frame.pts_ms);
            if frame.pts_ms - first > elapsed {
                break;
            }
            let frame = self.pending.take().expect("pending");
            self.current = Some(Rc::new(frame.surface));
        }
        !(self.video_done && self.pending.is_none())
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for Movie {
    fn drop(&mut self) {
        self.stop();
    }
}
