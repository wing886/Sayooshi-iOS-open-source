//! Plays a movie through `MovieManager` on a simulated 16 ms frame clock and
//! reports which frame each poll returns (for checking stream decoding
//! without a renderer or device).
//!
//! ```text
//! movie_probe <project dir> <movie name> [seconds] [real ms per frame]
//! ```
use std::path::PathBuf;

use anyhow::{Context, Result};
use siglus_scene_vm::movie::MovieManager;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let project = PathBuf::from(
        args.next()
            .context("usage: movie_probe <project> <movie> [seconds] [ms]")?,
    );
    let name = args.next().context("missing movie name")?;
    let seconds: u64 = args.next().and_then(|v| v.parse().ok()).unwrap_or(20);
    let sleep_ms: u64 = args.next().and_then(|v| v.parse().ok()).unwrap_or(16);
    // `MOVIE_PROBE_START=ms`: start the clock mid-movie (as after a stream
    // restart).
    let start: u64 = std::env::var("MOVIE_PROBE_START")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let mut movies = MovieManager::new(project);
    let mut timer = start;
    let mut last = None;
    let mut distinct = 0usize;
    while timer < start + seconds * 1000 {
        match movies.poll_global_movie_frame(&name, timer)? {
            Some(frame) => {
                if last != Some(frame.frame_idx) {
                    distinct += 1;
                    last = Some(frame.frame_idx);
                }
                if (timer - start) % 2000 < 16 {
                    println!(
                        "timer {:6} ms: frame {} ({}x{}) fps {:?} total {:?} distinct {}",
                        timer,
                        frame.frame_idx,
                        frame.frame.width,
                        frame.frame.height,
                        frame.fps,
                        frame.total_ms,
                        distinct
                    );
                }
            }
            None => {
                if (timer - start) % 2000 < 16 {
                    println!("timer {timer:6} ms: no frame yet");
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
        timer += 16;
    }
    Ok(())
}
