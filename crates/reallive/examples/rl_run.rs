//! Runs a game headlessly, clicking through text and taking the first
//! choice, and reports what the engine could not do.
//!
//! ```text
//! rl_run <game dir> [frames] [--shots DIR] [--every N] [--scene N] [--click FRAME:X:Y]...
//!        [--choices 4,1,...] [--ctrl FROM:TO] [--skip]
//! ```
use std::collections::BTreeMap;

use anyhow::{Context, Result};
use reallive::engine::{Engine, EngineOptions};
use reallive::input::{Button, InputEvent, Key};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = args
        .first()
        .context("usage: rl_run <game dir> [frames] [--shots DIR] [--every N] [--scene N]")?;
    let value = |flag: &str| {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let frames: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(20_000);
    let shots = value("--shots");
    let every: usize = value("--every").and_then(|v| v.parse().ok()).unwrap_or(600);
    // `--choices 4,1,2`: the options to pick, in order (then the first).
    let mut choices: std::collections::VecDeque<char> = value("--choices")
        .map(|v| {
            v.split(',')
                .filter_map(|c| c.trim().chars().next())
                .collect()
        })
        .unwrap_or_default();
    let from: usize = value("--from").and_then(|v| v.parse().ok()).unwrap_or(0);
    // `--ctrl FROM:TO`: Ctrl held over those frames (repeating as the OS
    // repeats a held key).
    let ctrl: Option<(usize, usize)> = value("--ctrl").and_then(|v| {
        let (a, b) = v.split_once(':')?;
        Some((a.parse().ok()?, b.parse().ok()?))
    });
    let mut options = EngineOptions::headless(root);
    options.fonts = shots.is_some();
    options.start_scene = value("--scene").and_then(|v| v.parse().ok());
    // Scripted clicks for menus the game draws itself.
    let clicks: Vec<(usize, i32, i32)> = args
        .windows(2)
        .filter(|w| w[0] == "--click")
        .filter_map(|w| {
            let mut parts = w[1].split(':').map(|p| p.parse::<i64>().ok());
            Some((
                parts.next()?? as usize,
                parts.next()?? as i32,
                parts.next()?? as i32,
            ))
        })
        .collect();
    // `--monkey SEED`: click at random places (for exercising game UIs).
    let mut monkey: Option<u64> = value("--monkey").and_then(|v| v.parse().ok());
    let mut engine = Engine::open(options)?;
    // `--skip`: everything counts as read and skip mode is on.
    let skip = args.iter().any(|a| a == "--skip");
    if skip {
        engine.machine.memory.global.all_read = true;
    }
    if let Some(dir) = &shots {
        std::fs::create_dir_all(dir)?;
    }
    let mut scenes = BTreeMap::new();
    let mut last_scene = -1;
    let mut waited = 0;
    for frame in 0..frames {
        if engine.finished() {
            println!("halted at frame {frame}");
            break;
        }
        let scene = engine.machine.scene_number();
        *scenes.entry(scene).or_insert(0usize) += 1;
        if scene != last_scene {
            println!("frame {frame:6}: SEEN{scene:04}");
            last_scene = scene;
        }
        if let Some((start, end)) = ctrl {
            if (start..end).contains(&frame) {
                engine.input(InputEvent::KeyDown(Key::Ctrl));
            } else if frame == end {
                engine.input(InputEvent::KeyUp(Key::Ctrl));
            }
        }
        if skip {
            engine.machine.sys.syscom.skip_mode = true;
        }
        for &(at, x, y) in &clicks {
            if at == frame {
                engine.machine.sys.input.mouse = (x, y);
            } else if at + 2 == frame {
                engine.input(InputEvent::Press(Button::Left));
            } else if at + 4 == frame {
                engine.input(InputEvent::Release(Button::Left));
            }
        }
        if let Some(state) = monkey.as_mut()
            && frame % 40 == 0
        {
            *state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let x = ((*state >> 33) % 800) as i32;
            let y = ((*state >> 13) % 600) as i32;
            engine.machine.sys.input.mouse = (x, y);
        }
        // Scripts that poll the mouse themselves get a click now and then.
        if engine.machine.current_long_op().is_none() && frame > 7000 && frame % 90 == 0 {
            engine.input(InputEvent::Press(Button::Left));
        } else if engine.machine.current_long_op().is_none() && frame > 7000 && frame % 90 == 2 {
            engine.input(InputEvent::Release(Button::Left));
        }
        if engine.waiting_for_input() {
            waited += 1;
            // Give the screen a moment, then answer.
            if waited > 3 {
                waited = 0;
                match engine.machine.current_long_op().map(|op| op.name()) {
                    Some("select") => {
                        let pick = choices.pop_front().unwrap_or('1');
                        engine.input(InputEvent::KeyDown(Key::Char(pick)))
                    }
                    Some("name entry") => engine.input(InputEvent::KeyDown(Key::Enter)),
                    Some("system menu" | "slot menu") => {
                        engine.input(InputEvent::KeyDown(Key::Escape))
                    }
                    _ => {
                        engine.input(InputEvent::Press(Button::Left));
                        engine.input(InputEvent::Release(Button::Left));
                    }
                }
            }
        }
        // Movies decode on a worker thread in real time: give it a moment
        // per frame so the simulated clock does not outrun it.
        if engine.machine.sys.movie.is_some() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        engine.tick(16);
        if let Some(dir) = &shots
            && frame >= from
            && frame % every == every - 1
        {
            let surface = engine.render();
            let path = format!("{dir}/{frame:06}.png");
            image::save_buffer(
                &path,
                &surface.rgba,
                surface.width as u32,
                surface.height as u32,
                image::ColorType::Rgba8,
            )?;
        }
    }
    let d = &engine.machine.diagnostics;
    let mut unimplemented: Vec<_> = d.unimplemented.iter().collect();
    unimplemented.sort_by(|a, b| b.1.cmp(a.1));
    println!("\nunimplemented ({}):", unimplemented.len());
    for (name, count) in unimplemented.iter().take(80) {
        println!("  {count:6}  {name}");
    }
    println!("\nerrors ({}):", d.errors.len());
    for error in d.errors.iter().take(30) {
        println!("  {error}");
    }
    // `--dump D:0:8` prints a memory range at the end.
    for w in args.windows(2).filter(|w| w[0] == "--dump") {
        let mut parts = w[1].split(':');
        let (Some(bank), Some(Ok(from)), Some(Ok(count))) = (
            parts.next(),
            parts.next().map(str::parse::<i32>),
            parts.next().map(str::parse::<i32>),
        ) else {
            continue;
        };
        let machine = &engine.machine;
        let values: Vec<String> = (from..from + count)
            .map(|i| {
                reallive::memory::IntRef::named(bank, i)
                    .zip(machine.stack.last())
                    .and_then(|(r, f)| machine.memory.int(r, &f.vars).ok())
                    .map_or("?".into(), |v| v.to_string())
            })
            .collect();
        println!("{bank}[{from}..]: {}", values.join(" "));
    }
    println!(
        "\nat SEEN{:04} line {}, stack depth {}",
        engine.machine.scene_number(),
        engine.machine.line,
        engine.machine.stack.len()
    );
    println!(
        "\nlong op: {:?}",
        engine.machine.current_long_op().map(|op| op.name())
    );
    if let Some(movie) = &engine.machine.sys.movie {
        println!(
            "movie: {} (started {}, showing a frame {})",
            movie.path.display(),
            movie.started(),
            movie.current.is_some()
        );
    }
    Ok(())
}
