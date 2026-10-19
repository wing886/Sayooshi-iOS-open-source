//! Timing helpers for original Siglus stage wipes.
//!
//! Scene rendering and WIPE composition are performed exclusively by the wgpu
//! render graph.  The only CPU-side work retained by the original engine is
//! construction of the small 8-bit grayscale mask textures in `wipe_mask`.

pub fn eased_progress(raw: f32, speed_mode: i32) -> f32 {
    let progress = raw.clamp(0.0, 1.0);
    match speed_mode {
        1 => progress * progress,
        2 => 1.0 - (1.0 - progress) * (1.0 - progress),
        3 => progress * progress * (3.0 - 2.0 * progress),
        _ => progress,
    }
}

#[cfg(test)]
mod tests {
    use super::eased_progress;

    #[test]
    fn endpoints_are_preserved_for_every_speed_mode() {
        for mode in 0..=3 {
            assert_eq!(eased_progress(0.0, mode), 0.0);
            assert_eq!(eased_progress(1.0, mode), 1.0);
        }
    }
}
