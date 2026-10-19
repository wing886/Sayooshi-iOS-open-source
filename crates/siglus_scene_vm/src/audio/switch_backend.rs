//! Native PCM sink for the existing Kira mixer.
//!
//! Kira continues to decode, schedule and mix every existing Siglus sound.
//! Platform shells pull stereo PCM from this backend without owning the mixer.

use std::sync::{Mutex, OnceLock};

use kira::manager::backend::{Backend, Renderer};

static RENDERER: OnceLock<Mutex<Option<Renderer>>> = OnceLock::new();

fn renderer_slot() -> &'static Mutex<Option<Renderer>> {
    RENDERER.get_or_init(|| Mutex::new(None))
}

#[derive(Debug, Default)]
pub struct SwitchBackend;

impl Backend for SwitchBackend {
    type Settings = ();
    type Error = &'static str;

    fn setup(_: Self::Settings) -> Result<(Self, u32), Self::Error> {
        Ok((Self, 48_000))
    }

    fn start(&mut self, renderer: Renderer) -> Result<(), Self::Error> {
        let mut slot = renderer_slot()
            .lock()
            .map_err(|_| "audio renderer mutex poisoned")?;
        *slot = Some(renderer);
        Ok(())
    }
}

/// Fill interleaved signed 16-bit stereo samples for a native audio device.
pub unsafe fn render_stereo_i16(dst: *mut i16, frames: usize) {
    if dst.is_null() || frames == 0 {
        return;
    }
    let output = unsafe { std::slice::from_raw_parts_mut(dst, frames.saturating_mul(2)) };
    output.fill(0);
    let Ok(mut slot) = renderer_slot().lock() else {
        return;
    };
    let Some(renderer) = slot.as_mut() else {
        return;
    };
    renderer.on_start_processing();
    for stereo in output.chunks_exact_mut(2) {
        let frame = renderer.process();
        stereo[0] = (frame.left.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        stereo[1] = (frame.right.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
    }
}

/// Stable ABI used by the existing Switch shell.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn siglus_switch_audio_render_i16(dst: *mut i16, frames: usize) {
    unsafe { render_stereo_i16(dst, frames) }
}
