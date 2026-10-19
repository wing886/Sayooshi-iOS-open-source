//! Counters for the Vita player's periodic and slow-frame logs.

use std::sync::atomic::{AtomicU64, Ordering};

use super::gpu::{Gpu, GpuCounters};

/// Host phases timed per frame: script pump, element tick, render-frame
/// build (the renderer itself is `RENDER_US`).
pub const PUMP: usize = 0;
pub const TICK: usize = 1;
pub const BUILD: usize = 2;
const PHASE_NAMES: [&str; 3] = ["pump", "tick", "build"];
static PHASE_US: [AtomicU64; 3] = [const { AtomicU64::new(0) }; 3];
static PHASE_MAX_US: [AtomicU64; 3] = [const { AtomicU64::new(0) }; 3];

/// This frame's times (pump, tick, build, render), for the slow-frame log;
/// cleared by `take_frame_detail`.
static FRAME_US: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];

static FRAMES: AtomicU64 = AtomicU64::new(0);
static RENDER_US: AtomicU64 = AtomicU64::new(0);
static OFFSCREEN_FRAMES: AtomicU64 = AtomicU64::new(0);
static TEXTURE_BYTES: AtomicU64 = AtomicU64::new(0);

/// GPU work since the last summary, and in the current frame.
static PERIOD: std::sync::Mutex<GpuCounters> = std::sync::Mutex::new(GpuCounters {
    uploads: 0,
    upload_bytes: 0,
    created: 0,
    draws: 0,
    scenes: 0,
    wait_us: 0,
});
static FRAME: std::sync::Mutex<GpuCounters> = std::sync::Mutex::new(GpuCounters {
    uploads: 0,
    upload_bytes: 0,
    created: 0,
    draws: 0,
    scenes: 0,
    wait_us: 0,
});

pub fn phase(index: usize, start: crate::platform_time::Instant) {
    let us = start.elapsed().as_micros() as u64;
    PHASE_US[index].fetch_add(us, Ordering::Relaxed);
    PHASE_MAX_US[index].fetch_max(us, Ordering::Relaxed);
    FRAME_US[index].fetch_add(us, Ordering::Relaxed);
}

/// A frame drawn through render targets (wipe or overlay).
pub(super) fn offscreen_frame() {
    OFFSCREEN_FRAMES.fetch_add(1, Ordering::Relaxed);
}

fn add(into: &mut GpuCounters, from: &GpuCounters) {
    into.uploads += from.uploads;
    into.upload_bytes += from.upload_bytes;
    into.created += from.created;
    into.draws += from.draws;
    into.scenes += from.scenes;
    into.wait_us += from.wait_us;
}

/// Records a rendered frame (and takes the GPU's counters).
pub(super) fn record_frame(gpu: &mut Gpu, start: crate::platform_time::Instant) {
    let us = start.elapsed().as_micros() as u64;
    FRAME_US[3].fetch_add(us, Ordering::Relaxed);
    FRAMES.fetch_add(1, Ordering::Relaxed);
    RENDER_US.fetch_add(us, Ordering::Relaxed);
    TEXTURE_BYTES.store(gpu.texture_bytes as u64, Ordering::Relaxed);
    let counters = std::mem::take(&mut gpu.counters);
    if let Ok(mut period) = PERIOD.lock() {
        add(&mut period, &counters);
    }
    if let Ok(mut frame) = FRAME.lock() {
        add(&mut frame, &counters);
    }
}

/// The current frame's phase times and GPU work, then reset.
pub fn take_frame_detail() -> String {
    let [pump, tick, build, render] =
        std::array::from_fn(|i| FRAME_US[i].swap(0, Ordering::Relaxed));
    let gpu = FRAME.lock().map(|mut f| std::mem::take(&mut *f)).unwrap_or_default();
    format!(
        "pump {pump} us, tick {tick} us, build {build} us, render {render} us; gpu wait {} us, draws {}, scenes {}, uploads {} ({} bytes)",
        gpu.wait_us, gpu.draws, gpu.scenes, gpu.uploads, gpu.upload_bytes
    )
}

/// A summary since the last call, then reset.
pub fn take_summary() -> String {
    let frames = FRAMES.swap(0, Ordering::Relaxed);
    let render_us = RENDER_US.swap(0, Ordering::Relaxed);
    let phases: Vec<String> = PHASE_NAMES
        .iter()
        .enumerate()
        .map(|(i, name)| {
            format!(
                "{name} avg {} max {}",
                PHASE_US[i].swap(0, Ordering::Relaxed) / frames.max(1),
                PHASE_MAX_US[i].swap(0, Ordering::Relaxed)
            )
        })
        .collect();
    let gpu = PERIOD.lock().map(|mut p| std::mem::take(&mut *p)).unwrap_or_default();
    format!(
        "phases [{}] render frames={frames} (avg {} us, offscreen {}) gpu wait avg {} us, draws {}, scenes {}; textures uploads={} upload-bytes={} created={} cached-bytes={}",
        phases.join(", "),
        render_us.checked_div(frames).unwrap_or(0),
        OFFSCREEN_FRAMES.swap(0, Ordering::Relaxed),
        gpu.wait_us.checked_div(frames).unwrap_or(0),
        gpu.draws,
        gpu.scenes,
        gpu.uploads,
        gpu.upload_bytes,
        gpu.created,
        TEXTURE_BYTES.load(Ordering::Relaxed)
    )
}
