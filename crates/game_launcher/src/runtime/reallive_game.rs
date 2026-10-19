use std::path::Path;

use anyhow::Result;
use reallive::engine::{Engine, EngineOptions};
use reallive::input::{Button, InputEvent, Key};

use super::{FramebufferGame, GameKey, PointerButton};
use crate::nls::Nls;

pub struct RealLiveGame {
    engine: Engine,
    frame: Vec<u8>,
    size: (u32, u32),
    ended: bool,
}

impl RealLiveGame {
    pub fn open(root: &Path, nls: Nls) -> Result<Self> {
        let mut options = EngineOptions::new(root);
        options.nls = nls.to_reallive();
        let engine = Engine::open(options)?;
        let size = (
            engine.machine.sys.gfx.width.max(1) as u32,
            engine.machine.sys.gfx.height.max(1) as u32,
        );
        Ok(Self {
            engine,
            frame: vec![0; (size.0 * size.1 * 4) as usize],
            size,
            ended: false,
        })
    }
}

fn map_key(key: GameKey) -> Key {
    match key {
        GameKey::Enter => Key::Enter,
        GameKey::Escape => Key::Escape,
        GameKey::Space => Key::Space,
        GameKey::Up => Key::Up,
        GameKey::Down => Key::Down,
        GameKey::Left => Key::Left,
        GameKey::Right => Key::Right,
        GameKey::PageUp => Key::PageUp,
        GameKey::PageDown => Key::PageDown,
        GameKey::Home => Key::Home,
        GameKey::End => Key::End,
        GameKey::Backspace => Key::Backspace,
        GameKey::Tab => Key::Tab,
        GameKey::Ctrl => Key::Ctrl,
        GameKey::Shift => Key::Shift,
        GameKey::F(n) => Key::F(n),
        GameKey::Char(c) => Key::Char(c),
    }
}

impl FramebufferGame for RealLiveGame {
    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn step(&mut self, dt_ms: u32) -> bool {
        if self.ended {
            return false;
        }
        self.engine.tick(u64::from(dt_ms));
        for error in self.engine.machine.diagnostics.errors.drain(..) {
            log::warn!("reallive: {error}");
        }
        if self.engine.finished() {
            self.shutdown();
            return false;
        }
        let surface = self.engine.render();
        self.size = (surface.width.max(1) as u32, surface.height.max(1) as u32);
        self.frame = surface.rgba;
        true
    }

    fn frame(&self) -> &[u8] {
        &self.frame
    }

    fn pointer_move(&mut self, x: i32, y: i32) {
        self.engine.machine.sys.input.mouse = (x, y);
    }

    fn pointer_button(&mut self, button: PointerButton, pressed: bool) {
        let button = match button {
            PointerButton::Left => Button::Left,
            PointerButton::Right => Button::Right,
        };
        self.engine.input(if pressed {
            InputEvent::Press(button)
        } else {
            InputEvent::Release(button)
        });
    }

    fn wheel(&mut self, up: bool) {
        self.engine.input(InputEvent::Press(if up {
            Button::WheelUp
        } else {
            Button::WheelDown
        }));
    }

    fn key(&mut self, key: GameKey, pressed: bool) {
        let key = map_key(key);
        self.engine.input(if pressed {
            InputEvent::KeyDown(key)
        } else {
            InputEvent::KeyUp(key)
        });
    }

    fn text(&mut self, text: &str) {
        self.engine.input(InputEvent::Text(text.to_owned()));
    }

    fn cursor_visible(&self) -> bool {
        self.engine.machine.sys.cursor_visible
    }

    fn title(&self) -> String {
        let sys = &self.engine.machine.sys;
        if sys.title.is_empty() {
            sys.gameexe.str("CAPTION").unwrap_or("RealLive").to_owned()
        } else {
            sys.title.clone()
        }
    }

    fn shutdown(&mut self) {
        if !self.ended {
            reallive::save::save_global(&self.engine.machine).ok();
        }
        self.ended = true;
    }
}
