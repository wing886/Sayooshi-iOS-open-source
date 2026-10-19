//! Headless UK2 engine runner.
//!
//! ```text
//! uk2_run <game-dir> [--script FILE] [--out DIR] [--ticks N] [--trace]
//! ```
//!
//! Runs the engine on a virtual VSYNC clock (one tick per wait) and replays
//! scripted input.  Script lines are `<tick> <command> [args]`:
//! `move X Y`, `down`/`up` (left button), `rdown`/`rup`, `click X Y`,
//! `rclick`, `key NAME` (tap), `keydown NAME`, `keyup NAME`, `shot FILE`.
//! Key names: RETURN SPACE ESC UP DOWN LEFT RIGHT SHIFT CTRL or a hex scan
//! code.

use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use uk2::Uk2Game;
use uk2::engine::{Engine, HostEvent, MusicCommand, Platform, scan};

struct Headless {
    tick: u64,
    schedule: BTreeMap<u64, Vec<Action>>,
    frame: Vec<u8>,
    out: PathBuf,
    limit: u64,
    last_music: Option<String>,
}

#[derive(Debug, Clone)]
enum Action {
    Event(HostEvent),
    Shot(String),
}

impl Platform for Headless {
    fn ticks(&mut self) -> u64 {
        self.tick
    }

    fn wait(&mut self) {
        self.tick += 1;
    }

    fn poll_events(&mut self, events: &mut Vec<HostEvent>) {
        let due: Vec<u64> = self.schedule.range(..=self.tick).map(|(k, _)| *k).collect();
        for key in due {
            for action in self.schedule.remove(&key).unwrap_or_default() {
                match action {
                    Action::Event(event) => events.push(event),
                    Action::Shot(name) => {
                        if let Err(error) = save_png(&self.out.join(&name), &self.frame) {
                            eprintln!("screenshot {name}: {error:#}");
                        } else {
                            eprintln!("[tick {}] saved {name}", self.tick);
                        }
                    }
                }
            }
        }
        if self.tick >= self.limit {
            events.push(HostEvent::Quit);
        }
    }

    fn present(&mut self, rgba: &[u8]) {
        self.frame.clear();
        self.frame.extend_from_slice(rgba);
    }

    fn music(&mut self, command: MusicCommand) {
        if let MusicCommand::Load { name, .. } = &command {
            self.last_music = Some(name.clone());
            eprintln!("[tick {}] music {name}", self.tick);
        }
    }
}

fn save_png(path: &std::path::Path, rgba: &[u8]) -> Result<()> {
    if rgba.len() != 640 * 400 * 4 {
        bail!("no frame yet");
    }
    image::save_buffer(path, rgba, 640, 400, image::ColorType::Rgba8)?;
    Ok(())
}

fn key_code(name: &str) -> Result<u8> {
    Ok(match name.to_ascii_uppercase().as_str() {
        "RETURN" | "ENTER" => scan::RETURN,
        "SPACE" => scan::SPACE,
        "ESC" => scan::ESC,
        "UP" => scan::UP,
        "DOWN" => scan::DOWN,
        "LEFT" => scan::LEFT,
        "RIGHT" => scan::RIGHT,
        "SHIFT" => scan::SHIFT,
        "CTRL" => scan::CTRL,
        other => u8::from_str_radix(other.trim_start_matches("0X"), 16)
            .with_context(|| format!("unknown key {other}"))?,
    })
}

fn parse_script(text: &str) -> Result<BTreeMap<u64, Vec<Action>>> {
    let mut schedule: BTreeMap<u64, Vec<Action>> = BTreeMap::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let tick: u64 = parts[0]
            .parse()
            .with_context(|| format!("line {}: bad tick", number + 1))?;
        let arg = |i: usize| -> Result<&str> {
            parts
                .get(i)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("line {}: missing argument", number + 1))
        };
        let mut push = |t: u64, action: Action| schedule.entry(t).or_default().push(action);
        match parts.get(1).copied().unwrap_or("") {
            "move" => push(
                tick,
                Action::Event(HostEvent::MouseMove {
                    x: arg(2)?.parse()?,
                    y: arg(3)?.parse()?,
                }),
            ),
            "down" => push(
                tick,
                Action::Event(HostEvent::MouseButton {
                    left: true,
                    pressed: true,
                }),
            ),
            "up" => push(
                tick,
                Action::Event(HostEvent::MouseButton {
                    left: true,
                    pressed: false,
                }),
            ),
            "rdown" => push(
                tick,
                Action::Event(HostEvent::MouseButton {
                    left: false,
                    pressed: true,
                }),
            ),
            "rup" => push(
                tick,
                Action::Event(HostEvent::MouseButton {
                    left: false,
                    pressed: false,
                }),
            ),
            "click" => {
                push(
                    tick,
                    Action::Event(HostEvent::MouseMove {
                        x: arg(2)?.parse()?,
                        y: arg(3)?.parse()?,
                    }),
                );
                push(
                    tick + 1,
                    Action::Event(HostEvent::MouseButton {
                        left: true,
                        pressed: true,
                    }),
                );
                push(
                    tick + 4,
                    Action::Event(HostEvent::MouseButton {
                        left: true,
                        pressed: false,
                    }),
                );
            }
            "rclick" => {
                push(
                    tick,
                    Action::Event(HostEvent::MouseButton {
                        left: false,
                        pressed: true,
                    }),
                );
                push(
                    tick + 4,
                    Action::Event(HostEvent::MouseButton {
                        left: false,
                        pressed: false,
                    }),
                );
            }
            "key" => {
                let code = key_code(arg(2)?)?;
                push(
                    tick,
                    Action::Event(HostEvent::Key {
                        code,
                        pressed: true,
                    }),
                );
                push(
                    tick + 3,
                    Action::Event(HostEvent::Key {
                        code,
                        pressed: false,
                    }),
                );
            }
            "keydown" => {
                let code = key_code(arg(2)?)?;
                push(
                    tick,
                    Action::Event(HostEvent::Key {
                        code,
                        pressed: true,
                    }),
                );
            }
            "keyup" => {
                let code = key_code(arg(2)?)?;
                push(
                    tick,
                    Action::Event(HostEvent::Key {
                        code,
                        pressed: false,
                    }),
                );
            }
            "shot" => push(tick, Action::Shot(arg(2)?.to_owned())),
            other => bail!("line {}: unknown command {other}", number + 1),
        }
    }
    Ok(schedule)
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let root = PathBuf::from(args.next().context("usage: uk2_run <game-dir> [options]")?);
    let mut script = String::new();
    let mut out = PathBuf::from(".");
    let mut limit = 20_000u64;
    let mut trace = false;
    let mut trace_mes = false;
    let mut start = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--script" => {
                let path = args.next().context("--script FILE")?;
                script = std::fs::read_to_string(&path).with_context(|| format!("read {path}"))?;
            }
            "--out" => out = PathBuf::from(args.next().context("--out DIR")?),
            "--ticks" => limit = args.next().context("--ticks N")?.parse()?,
            "--trace" => trace = true,
            "--trace-mes" => trace_mes = true,
            "--start" => start = Some(args.next().context("--start NAME")?.into_bytes()),
            other => bail!("unknown option {other}"),
        }
    }
    std::fs::create_dir_all(&out)?;
    let platform = Headless {
        tick: 0,
        schedule: parse_script(&script)?,
        frame: Vec::new(),
        out: out.clone(),
        limit,
        last_music: None,
    };
    let game = Uk2Game::open(&root)?;
    let mut engine = Engine::new(game, Box::new(platform))?;
    engine.trace = trace;
    engine.trace_mes = trace_mes;
    engine.start_override = start;
    let result = engine.run();
    engine.present();
    let _ = save_png(&out.join("final.png"), &{
        let mut rgba = Vec::new();
        engine.vram.render_rgba(&mut rgba);
        rgba
    });
    eprintln!("{}", engine.debug_dump());
    eprintln!(
        "instructions: {}, mes: {}",
        engine.instruction_count,
        String::from_utf8_lossy(
            &engine
                .mem
                .cstr(uk2::engine::mem::ds_ptr(uk2::engine::ds::CUR_MES_NAME))
        )
    );
    result
}
