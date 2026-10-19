//! VisualArt's AVG32 engine (the pre-RealLive system behind Kanon, AIR and
//! many other titles).
//!
//! * [`scenario`] interprets `TPC32` bytecode from `SEEN.TXT`;
//! * [`system`], [`meswin`] and [`menu`] provide the message window,
//!   choices, menus, save/load, animation and input;
//! * [`pdtmgr`] and [`effect`] implement the 32 PDT buffers and every
//!   screen transition;
//! * [`sound`] plays CD/DirectSound BGM, WAV effects and KOE voices.

pub mod animation;
pub mod archive;
pub mod ard;
pub mod buffer;
pub mod cgmode;
pub mod cursor;
pub mod effect;
pub mod engine;
pub mod flags;
pub mod font;
pub mod game;
pub mod ini;
pub mod menu;
pub mod meswin;
pub mod movie;
pub mod nls;
pub mod pdt;
pub mod pdtmgr;
pub mod resource;
pub mod savedata;
pub mod scenario;
pub mod scene;
pub mod sound;
pub mod system;
pub mod voicepatch;

pub use archive::{PaclArchive, PaclEntry};
pub use buffer::{AVG32_HEIGHT, AVG32_WIDTH};
pub use engine::{Avg32Engine, EngineOptions};
pub use game::{Avg32Game, EngineKind, GameLayout, detect_game_root};
pub use nls::Nls;
pub use pdt::{PdtImage, decode_pdt};
pub use scene::{Avg32SceneHeader, SceneMenu, SceneSubmenu, SceneValue, ValueKind};
pub use system::Key;
