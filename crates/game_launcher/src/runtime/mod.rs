//! A common frame-based runtime over the software-rendered engines
//! (RealLive, AVG32 and UK2).
//!
//! Hosts call [`FramebufferGame::step`] once per display frame, draw the RGBA
//! frame scaled to fit, and forward input in game coordinates.  SiglusEngine
//! renders with the GPU and keeps its own platform hosts.

mod avg32_game;
mod reallive_game;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod uk2_game;

use std::path::Path;

use anyhow::{Result, bail};
use engine_detect::EngineKind;

use crate::nls::Nls;

/// Keys hosts can send.  The numeric codes are part of the C/JS API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameKey {
    Enter,
    Escape,
    Space,
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    Backspace,
    Tab,
    Ctrl,
    Shift,
    F(u8),
    Char(char),
}

impl GameKey {
    /// Decodes the API key code: 1-15 are named keys, 0x100+n is F(n),
    /// and 0x10000+c is the character `c`.
    pub fn from_code(code: u32) -> Option<Self> {
        Some(match code {
            1 => Self::Enter,
            2 => Self::Escape,
            3 => Self::Space,
            4 => Self::Up,
            5 => Self::Down,
            6 => Self::Left,
            7 => Self::Right,
            8 => Self::PageUp,
            9 => Self::PageDown,
            10 => Self::Home,
            11 => Self::End,
            12 => Self::Backspace,
            13 => Self::Tab,
            14 => Self::Ctrl,
            15 => Self::Shift,
            0x101..=0x10c => Self::F((code - 0x100) as u8),
            0x10000.. => Self::Char(char::from_u32(code - 0x10000)?),
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    Left,
    Right,
}

pub trait FramebufferGame {
    /// Frame size in pixels.
    fn size(&self) -> (u32, u32);
    /// Advances the game by one display frame; `false` once it has ended.
    fn step(&mut self, dt_ms: u32) -> bool;
    /// The current RGBA8 frame, `size().0 * size().1 * 4` bytes.
    fn frame(&self) -> &[u8];
    fn pointer_move(&mut self, x: i32, y: i32);
    fn pointer_button(&mut self, button: PointerButton, pressed: bool);
    fn wheel(&mut self, up: bool);
    fn key(&mut self, key: GameKey, pressed: bool);
    fn text(&mut self, text: &str);
    /// Whether the host should draw its own pointer.
    fn cursor_visible(&self) -> bool {
        true
    }
    fn title(&self) -> String;
    /// Saves persistent data and stops audio.
    fn shutdown(&mut self);
}

/// Opens a game with the runtime of its engine.
pub fn open(root: &Path, engine: EngineKind, nls: Option<Nls>) -> Result<Box<dyn FramebufferGame>> {
    let engine = if engine == EngineKind::Unknown {
        engine_detect::detect(root)?
    } else {
        engine
    };
    if let Err(reason) = crate::probe::platform_supports(engine) {
        bail!("{reason}");
    }
    let nls = Nls::for_engine(engine, nls);
    Ok(match engine {
        EngineKind::Avg32 => Box::new(avg32_game::Avg32Game::open(root, nls)?),
        EngineKind::RealLive => Box::new(reallive_game::RealLiveGame::open(root, nls)?),
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        EngineKind::Uk2 => Box::new(uk2_game::Uk2FramebufferGame::open(root, nls)?),
        EngineKind::Siglus => {
            bail!("SiglusEngine games run through the Siglus platform host")
        }
        other => bail!("{} games are not supported", other.name()),
    })
}
