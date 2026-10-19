use std::path::Path;

use anyhow::Result;
use avg32::system::HostRequest;
use avg32::{AVG32_HEIGHT, AVG32_WIDTH, Avg32Engine, EngineOptions, Key};

use super::{FramebufferGame, GameKey, PointerButton};
use crate::nls::Nls;

pub struct Avg32Game {
    engine: Avg32Engine,
    frame: Vec<u8>,
    ended: bool,
}

impl Avg32Game {
    pub fn open(root: &Path, nls: Nls) -> Result<Self> {
        let engine = Avg32Engine::open(
            root,
            EngineOptions {
                nls: nls.to_avg32(),
                ..EngineOptions::default()
            },
        )?;
        Ok(Self {
            engine,
            frame: vec![0; (AVG32_WIDTH * AVG32_HEIGHT * 4) as usize],
            ended: false,
        })
    }
}

fn map_key(key: GameKey) -> Option<Key> {
    Some(match key {
        GameKey::Enter => Key::Enter,
        GameKey::Escape => Key::Escape,
        GameKey::Space => Key::Space,
        GameKey::Up => Key::Up,
        GameKey::Down => Key::Down,
        GameKey::Left => Key::Left,
        GameKey::Right => Key::Right,
        GameKey::PageUp => Key::PageUp,
        GameKey::PageDown => Key::PageDown,
        GameKey::Backspace => Key::Backspace,
        _ => return None,
    })
}

impl FramebufferGame for Avg32Game {
    fn size(&self) -> (u32, u32) {
        (AVG32_WIDTH, AVG32_HEIGHT)
    }

    fn step(&mut self, _dt_ms: u32) -> bool {
        if self.ended {
            return false;
        }
        self.engine.tick();
        for warning in self.engine.take_warnings() {
            log::warn!("avg32: {warning}");
        }
        for request in self.engine.take_requests() {
            if request == HostRequest::Quit {
                self.ended = true;
            }
        }
        if !self.engine.running() {
            self.ended = true;
        }
        self.frame = self.engine.frame_rgba();
        !self.ended
    }

    fn frame(&self) -> &[u8] {
        &self.frame
    }

    fn pointer_move(&mut self, x: i32, y: i32) {
        self.engine.mouse_move(x, y);
    }

    fn pointer_button(&mut self, button: PointerButton, pressed: bool) {
        // AVG32 acts on button release.
        if !pressed {
            self.engine.mouse_up(button == PointerButton::Right);
        }
    }

    fn wheel(&mut self, up: bool) {
        self.engine.wheel(up);
    }

    fn key(&mut self, key: GameKey, pressed: bool) {
        match key {
            GameKey::Ctrl | GameKey::Shift => self.engine.set_skip(pressed),
            _ if pressed => {
                if let Some(key) = map_key(key) {
                    self.engine.key_down(key);
                }
            }
            _ => {}
        }
    }

    fn text(&mut self, text: &str) {
        self.engine.text_input(text);
    }

    fn cursor_visible(&self) -> bool {
        self.engine.cursor_visible()
    }

    fn title(&self) -> String {
        self.engine.window_title().to_owned()
    }

    fn shutdown(&mut self) {
        self.engine.shutdown();
        self.ended = true;
    }
}
