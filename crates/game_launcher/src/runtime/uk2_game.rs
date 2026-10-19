//! UK2 runs like the original DOS program: on its own thread, busy-waiting
//! on emulated interrupts.  This adapter feeds it host input and hands the
//! latest presented frame to the frame-based host.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use uk2::engine::audio::MusicPlayer;
use uk2::engine::vram::{SCREEN_H, SCREEN_W};
use uk2::engine::{Engine, HostEvent, MusicCommand, Platform, VSYNC_HZ, scan};

use super::{FramebufferGame, GameKey, PointerButton};
use crate::nls::Nls;

#[derive(Default)]
struct Shared {
    frame: Vec<u8>,
    events: VecDeque<HostEvent>,
    finished: bool,
    error: Option<String>,
}

struct ThreadPlatform {
    start: Instant,
    shared: Arc<Mutex<Shared>>,
    music: Option<MusicPlayer>,
}

impl ThreadPlatform {
    fn tick_at(&self, instant: Instant) -> u64 {
        (instant.duration_since(self.start).as_secs_f64() * VSYNC_HZ) as u64
    }
}

impl Platform for ThreadPlatform {
    fn ticks(&mut self) -> u64 {
        self.tick_at(Instant::now())
    }

    fn wait(&mut self) {
        let next = (self.tick_at(Instant::now()) + 1) as f64 / VSYNC_HZ;
        let target = self.start + Duration::from_secs_f64(next);
        let now = Instant::now();
        if target > now {
            std::thread::sleep((target - now).min(Duration::from_millis(20)));
        }
    }

    fn poll_events(&mut self, events: &mut Vec<HostEvent>) {
        if let Ok(mut shared) = self.shared.lock() {
            events.extend(shared.events.drain(..));
        }
    }

    fn present(&mut self, rgba: &[u8]) {
        if let Ok(mut shared) = self.shared.lock() {
            shared.frame.clear();
            shared.frame.extend_from_slice(rgba);
        }
    }

    fn music(&mut self, command: MusicCommand) {
        if let Some(music) = &mut self.music {
            music.command(command);
        }
    }
}

pub struct Uk2FramebufferGame {
    shared: Arc<Mutex<Shared>>,
    frame: Vec<u8>,
    thread: Option<JoinHandle<()>>,
    ended: bool,
    title: String,
}

impl Uk2FramebufferGame {
    pub fn open(root: &Path, nls: Nls) -> Result<Self> {
        let game = uk2::Uk2Game::open(root)?;
        let title = root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "UK2".to_owned());
        let shared = Arc::new(Mutex::new(Shared {
            frame: vec![0; SCREEN_W * SCREEN_H * 4],
            ..Shared::default()
        }));
        let thread_shared = Arc::clone(&shared);
        let text_nls = nls.to_uk2();
        let thread = std::thread::Builder::new()
            .name("uk2-engine".to_owned())
            .spawn(move || {
                let music = MusicPlayer::new()
                    .map_err(|error| log::warn!("uk2: audio disabled: {error:#}"))
                    .ok();
                let platform = ThreadPlatform {
                    start: Instant::now(),
                    shared: Arc::clone(&thread_shared),
                    music,
                };
                let result = Engine::new(game, Box::new(platform)).and_then(|mut engine| {
                    engine.set_text_encoding(text_nls);
                    engine.run()
                });
                let mut shared = thread_shared.lock().expect("uk2 state poisoned");
                if let Err(error) = result
                    && !Engine::quit_requested(&error)
                {
                    shared.error = Some(format!("{error:#}"));
                }
                shared.finished = true;
            })
            .context("spawn UK2 engine thread")?;
        Ok(Self {
            shared,
            frame: vec![0; SCREEN_W * SCREEN_H * 4],
            thread: Some(thread),
            ended: false,
            title,
        })
    }

    fn send(&self, event: HostEvent) {
        if let Ok(mut shared) = self.shared.lock() {
            shared.events.push_back(event);
        }
    }
}

fn scan_code(key: GameKey) -> Option<u8> {
    Some(match key {
        GameKey::Enter => scan::RETURN,
        GameKey::Escape => scan::ESC,
        GameKey::Space => scan::SPACE,
        GameKey::Up => scan::UP,
        GameKey::Down => scan::DOWN,
        GameKey::Left => scan::LEFT,
        GameKey::Right => scan::RIGHT,
        GameKey::Ctrl => scan::CTRL,
        GameKey::Shift => scan::SHIFT,
        GameKey::Backspace => 0x0e,
        GameKey::Tab => 0x0f,
        GameKey::PageUp => 0x36,
        GameKey::PageDown => 0x37,
        GameKey::Home => 0x3e,
        GameKey::F(n @ 1..=10) => 0x61 + n,
        GameKey::Char(c) => {
            const ROWS: [(&str, u8); 4] = [
                ("1234567890", 0x01),
                ("qwertyuiop", 0x10),
                ("asdfghjkl", 0x1d),
                ("zxcvbnm", 0x29),
            ];
            let c = c.to_ascii_lowercase();
            return ROWS
                .iter()
                .find_map(|(row, base)| row.find(c).map(|index| base + index as u8));
        }
        _ => return None,
    })
}

impl FramebufferGame for Uk2FramebufferGame {
    fn size(&self) -> (u32, u32) {
        (SCREEN_W as u32, SCREEN_H as u32)
    }

    fn step(&mut self, _dt_ms: u32) -> bool {
        if self.ended {
            return false;
        }
        let Ok(shared) = self.shared.lock() else {
            self.ended = true;
            return false;
        };
        if shared.frame.len() == self.frame.len() {
            self.frame.copy_from_slice(&shared.frame);
        }
        if shared.finished {
            if let Some(error) = &shared.error {
                log::error!("uk2: {error}");
            }
            drop(shared);
            self.ended = true;
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
            return false;
        }
        true
    }

    fn frame(&self) -> &[u8] {
        &self.frame
    }

    fn pointer_move(&mut self, x: i32, y: i32) {
        self.send(HostEvent::MouseMove {
            x: x.clamp(0, SCREEN_W as i32 - 1),
            y: y.clamp(0, SCREEN_H as i32 - 1),
        });
    }

    fn pointer_button(&mut self, button: PointerButton, pressed: bool) {
        self.send(HostEvent::MouseButton {
            left: button == PointerButton::Left,
            pressed,
        });
    }

    fn wheel(&mut self, _up: bool) {}

    fn key(&mut self, key: GameKey, pressed: bool) {
        if let Some(code) = scan_code(key) {
            self.send(HostEvent::Key { code, pressed });
        }
    }

    fn text(&mut self, _text: &str) {}

    fn cursor_visible(&self) -> bool {
        // The engine draws the PC-98 software cursor itself.
        false
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn shutdown(&mut self) {
        self.send(HostEvent::Quit);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
        self.ended = true;
    }
}

impl Drop for Uk2FramebufferGame {
    fn drop(&mut self) {
        self.shutdown();
    }
}
