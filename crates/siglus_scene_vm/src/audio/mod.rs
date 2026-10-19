//! Audio subsystem.

pub mod bgm;
pub mod engine;
pub mod jitan;
pub mod kira_hub;
#[cfg(not(target_arch = "wasm32"))]
pub mod nwa_stream;
pub mod sfx_engine;
#[cfg(any(target_os = "horizon", target_os = "vita"))]
pub mod switch_backend;

pub use engine::{
    BgmEngine, TNM_PLAYER_STATE_FADE_OUT, TNM_PLAYER_STATE_FREE, TNM_PLAYER_STATE_PAUSE,
    TNM_PLAYER_STATE_PLAY,
};
pub use kira_hub::{AudioHub, TrackKind};
pub use sfx_engine::{KoeEngine, PcmEngine, SeEngine};
