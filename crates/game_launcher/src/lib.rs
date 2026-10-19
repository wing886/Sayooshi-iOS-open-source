//! Shared launcher layer for the app builds (macOS bundle, iOS, Android and
//! the web).
//!
//! * [`probe`]: engine detection, titles, covers and text-encoding choices
//!   for one game folder, and discovery of every game below a folder;
//! * [`runtime`]: one frame-based interface over the software-rendered
//!   engines (RealLive, AVG32, UK2);
//! * [`ffi`]: the C API the native apps call;
//! * `desktop`: a window runner for every engine (macOS app, desktop);
//! * `web`: the wasm-bindgen API for the browser page.
//!
//! SiglusEngine keeps its own platform hosts in `siglus_scene_vm`, which this
//! crate links, so one library serves every engine.

pub mod cover;
pub mod ffi;
pub mod json;
pub mod nls;
pub mod probe;
pub mod runtime;

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub mod desktop;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod web;

pub use engine_detect::EngineKind;
pub use nls::Nls;
pub use probe::{GameInfo, probe, scan};
