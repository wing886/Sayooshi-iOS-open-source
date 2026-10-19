//! Headless AVG32 driver: runs the engine on a simulated clock with
//! automatic input, reporting warnings and optionally writing screenshots.
//!
//! ```text
//! avg32_probe <game-root> [--seen N] [--frames N] [--shots DIR] [--every N] [--sweep] [--nls sjis|gbk|big5|utf8]
//! ```

use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use avg32::{Avg32Engine, EngineOptions, Key, Nls};

struct Args {
    root: PathBuf,
    seen: Option<i32>,
    frames: usize,
    shots: Option<PathBuf>,
    every: usize,
    sweep: bool,
    click_every: usize,
    pos: Vec<(i32, i32)>,
    nls: Nls,
}

fn parse_args() -> Result<Args> {
    let mut raw = std::env::args().skip(1);
    let mut args = Args {
        root: raw
            .next()
            .context("usage: avg32_probe <game-root> [options]")?
            .into(),
        seen: None,
        frames: 20_000,
        shots: None,
        every: 60,
        sweep: false,
        click_every: 1,
        pos: vec![(320, 240)],
        nls: Nls::default(),
    };
    while let Some(flag) = raw.next() {
        match flag.as_str() {
            "--seen" => args.seen = raw.next().and_then(|value| value.parse().ok()),
            "--frames" => {
                args.frames = raw
                    .next()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(args.frames)
            }
            "--shots" => args.shots = raw.next().map(PathBuf::from),
            "--every" => {
                args.every = raw
                    .next()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(60)
            }
            "--click-every" => {
                args.click_every = raw.next().and_then(|value| value.parse().ok()).unwrap_or(1)
            }
            "--sweep" => args.sweep = true,
            "--nls" => args.nls = raw.next().context("--nls sjis|gbk|big5|utf8")?.parse()?,
            "--pos" => {
                let value = raw.next().unwrap_or_default();
                args.pos.clear();
                for pair in value.split(';') {
                    let (x, y) = pair.split_once(',').context("--pos x,y[;x,y...]")?;
                    args.pos.push((x.parse()?, y.parse()?));
                }
            }
            other => anyhow::bail!("unknown option {other}"),
        }
    }
    Ok(args)
}

fn write_ppm(path: &std::path::Path, rgba: &[u8]) -> Result<()> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(file, "P6\n640 480\n255\n")?;
    for pixel in rgba.as_chunks::<4>().0 {
        file.write_all(&pixel[..3])?;
    }
    Ok(())
}

fn options(nls: Nls) -> EngineOptions {
    EngineOptions {
        audio: false,
        virtual_clock: true,
        persist: false,
        nls,
    }
}

/// Runs frames with automatic clicking; returns the number of frames run.
fn drive(
    engine: &mut Avg32Engine,
    frames: usize,
    click_every: usize,
    pos: &[(i32, i32)],
    shots: Option<(&PathBuf, usize)>,
    stop_on_scene_change: bool,
) -> Result<usize> {
    let start = engine.scenario().seen;
    for frame in 0..frames {
        if !engine.running() {
            return Ok(frame);
        }
        if stop_on_scene_change && engine.scenario().seen != start {
            return Ok(frame);
        }
        if click_every > 0 && frame % (click_every * 7) == 1 {
            engine.key_down(Key::Down);
        }
        if click_every > 0 && frame % click_every == 0 {
            let (x, y) = pos[(frame / click_every.max(1)).min(pos.len() - 1)];
            engine.mouse_move(x, y);
            engine.key_down(Key::Enter);
        }
        if let Ok(value) = std::env::var("AVG32_PROBE_RCLICK") {
            if value.parse() == Ok(frame) {
                engine.mouse_up(true);
            }
        }
        if let Ok(value) = std::env::var("AVG32_PROBE_WHEEL") {
            if value.parse() == Ok(frame) {
                engine.wheel(true);
            }
        }
        engine.tick();
        engine.advance_clock(17);
        if let Some((directory, every)) = shots
            && frame % every == 0
        {
            let rgba = engine.frame_rgba();
            write_ppm(&directory.join(format!("{frame:06}.ppm")), &rgba)?;
        }
        for warning in engine.take_warnings() {
            println!("  frame {frame}: {warning}");
        }
    }
    Ok(frames)
}

fn main() -> Result<()> {
    let args = parse_args()?;
    if args.sweep {
        let game = avg32::Avg32Game::open(&args.root)?;
        let mut scenes: Vec<i32> = game
            .scene_names()
            .iter()
            .filter_map(|name| {
                name.to_ascii_uppercase()
                    .strip_prefix("SEEN")?
                    .strip_suffix(".TXT")?
                    .parse()
                    .ok()
            })
            .collect();
        scenes.sort_unstable();
        drop(game);
        for seen in scenes {
            let mut engine = Avg32Engine::open(&args.root, options(args.nls))?;
            engine.take_warnings();
            engine.jump_to_scene(seen);
            println!("SEEN{seen:03}");
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                drive(
                    &mut engine,
                    args.frames,
                    args.click_every,
                    &args.pos,
                    None,
                    true,
                )
            }));
            match result {
                Ok(Ok(frames)) => println!(
                    "  -> {frames} frames, now SEEN{:03} at {:#x}{}",
                    engine.scenario().seen,
                    engine.scenario().pos,
                    if engine.running() { "" } else { " (stopped)" }
                ),
                Ok(Err(error)) => println!("  -> error: {error:#}"),
                Err(_) => println!("  -> PANIC"),
            }
        }
        return Ok(());
    }
    let mut engine = Avg32Engine::open(&args.root, options(args.nls))?;
    if let Some(seen) = args.seen {
        engine.jump_to_scene(seen);
    }
    if let Some(directory) = &args.shots {
        std::fs::create_dir_all(directory)?;
    }
    let frames = drive(
        &mut engine,
        args.frames,
        args.click_every,
        &args.pos,
        args.shots
            .as_ref()
            .map(|directory| (directory, args.every.max(1))),
        false,
    )?;
    println!(
        "ran {frames} frames; SEEN{:03} at {:#x}; title {:?}",
        engine.scenario().seen,
        engine.scenario().pos,
        engine.window_title()
    );
    Ok(())
}
