//! Plays a movie without sound for a few seconds and saves frames.
//!
//! ```text
//! rl_movie <file.mpg> <out dir> [seconds]
//! ```
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use reallive::movie::Movie;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args
        .first()
        .context("usage: rl_movie <file.mpg> <out dir> [seconds]")?;
    let out = args
        .get(1)
        .context("usage: rl_movie <file.mpg> <out dir> [seconds]")?;
    let seconds: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
    std::fs::create_dir_all(out)?;
    let start = Instant::now();
    let now = || start.elapsed().as_millis() as u64;
    let mut movie = Movie::open(path.into(), None, false, now());
    let mut shown = 0;
    let mut next_shot = 0;
    while now() < seconds * 1000 {
        if !movie.update(now()) {
            println!("ended at {} ms", now());
            break;
        }
        if movie.started()
            && now() >= next_shot
            && let Some(frame) = &movie.current
        {
            let file = format!("{out}/{:05}.png", now());
            image::save_buffer(
                &file,
                &frame.rgba,
                frame.width as u32,
                frame.height as u32,
                image::ColorType::Rgba8,
            )?;
            shown += 1;
            next_shot = now() + seconds * 1000 / 4;
        }
        std::thread::sleep(Duration::from_millis(16));
    }
    println!(
        "saved {shown} frames; audio: {}",
        movie.audio_ready.as_ref().map_or(0, |p| p.samples.len())
    );
    Ok(())
}
