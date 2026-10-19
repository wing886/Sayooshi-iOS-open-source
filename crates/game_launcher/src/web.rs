//! Browser API (wasm-bindgen) for the web launcher page.
//!
//! The page registers the folder the user picked with the `game_fs` index
//! (the `siglusFileExists`/`siglusReadFile`/`siglusListDir` functions) and
//! then probes games and runs RealLive/AVG32 games frame by frame here.
//! SiglusEngine games keep using `start_siglus_from_directory`.

use std::path::Path;

use wasm_bindgen::prelude::*;

use crate::json::Value;
use crate::nls::Nls;
use crate::probe;
use crate::runtime::{self, FramebufferGame, GameKey, PointerButton};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(message: &str);
}

/// Routes Rust panics to the browser console (wasm aborts without a message).
fn install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            console_error(&format!("game_launcher panic: {info}"));
            previous(info);
        }));
    });
}

fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for (index, shift) in [18, 12, 6, 0].into_iter().enumerate() {
            if index <= chunk.len() {
                out.push(TABLE[((n >> shift) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn probe_value(root: &str, nls: Option<Nls>) -> Value {
    match probe::probe(Path::new(root), nls, true) {
        Ok(info) => {
            let cover = info
                .cover
                .as_ref()
                .map(|cover| format!("data:image/png;base64,{}", base64(&cover.png)));
            info.to_json(Value::opt_string(cover))
        }
        Err(error) => Value::Object(vec![
            ("root", Value::string(root)),
            ("engine", Value::string("unknown")),
            ("supported", Value::Bool(false)),
            ("unsupported_reason", Value::string(error.to_string())),
            ("title", Value::string(root)),
            ("cover", Value::Null),
        ]),
    }
}

/// Describes the game at `root` (a path in the registered folder index).
#[wasm_bindgen(js_name = launcherProbe)]
pub fn launcher_probe(root: &str, nls: Option<String>) -> String {
    install_panic_hook();
    let nls = nls.as_deref().and_then(Nls::parse);
    probe_value(root, nls).to_json()
}

/// Finds and describes every game at or below `path`.
#[wasm_bindgen(js_name = launcherScan)]
pub fn launcher_scan(path: &str, depth: u32) -> String {
    install_panic_hook();
    let roots = probe::scan(Path::new(path), depth.min(8) as usize);
    Value::Array(
        roots
            .iter()
            .map(|root| probe_value(&root.to_string_lossy(), None))
            .collect(),
    )
    .to_json()
}

/// Supplies a font for game text (browsers expose no system fonts).
#[wasm_bindgen(js_name = launcherAddFont)]
pub fn launcher_add_font(bytes: &[u8]) {
    game_fs::add_host_font(bytes.to_vec());
}

/// A RealLive or AVG32 game running in the page.
#[wasm_bindgen]
pub struct WebGame {
    game: Box<dyn FramebufferGame>,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new(root: &str, engine: &str, nls: Option<String>) -> Result<WebGame, JsValue> {
        install_panic_hook();
        let nls = nls.as_deref().and_then(Nls::parse);
        runtime::open(Path::new(root), probe::engine_from_id(engine), nls)
            .map(|game| WebGame { game })
            .map_err(|error| JsValue::from_str(&format!("{error:#}")))
    }

    /// Advances one frame; false once the game has ended.
    pub fn step(&mut self, dt_ms: u32) -> bool {
        self.game.step(dt_ms)
    }

    pub fn width(&self) -> u32 {
        self.game.size().0
    }

    pub fn height(&self) -> u32 {
        self.game.size().1
    }

    /// The RGBA frame (`width * height * 4` bytes).
    pub fn frame(&self) -> Vec<u8> {
        self.game.frame().to_vec()
    }

    #[wasm_bindgen(js_name = pointerMove)]
    pub fn pointer_move(&mut self, x: i32, y: i32) {
        self.game.pointer_move(x, y);
    }

    /// `button`: 0 left, 1 right.
    #[wasm_bindgen(js_name = pointerButton)]
    pub fn pointer_button(&mut self, button: u32, pressed: bool) {
        let button = if button == 1 {
            PointerButton::Right
        } else {
            PointerButton::Left
        };
        self.game.pointer_button(button, pressed);
    }

    pub fn wheel(&mut self, up: bool) {
        self.game.wheel(up);
    }

    /// Key codes as in the C API (`game_fb_key`).
    pub fn key(&mut self, code: u32, pressed: bool) {
        if let Some(key) = GameKey::from_code(code) {
            self.game.key(key, pressed);
        }
    }

    pub fn text(&mut self, text: &str) {
        self.game.text(text);
    }

    #[wasm_bindgen(js_name = cursorVisible)]
    pub fn cursor_visible(&self) -> bool {
        self.game.cursor_visible()
    }

    pub fn title(&self) -> String {
        self.game.title()
    }

    pub fn close(&mut self) {
        self.game.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::base64;

    #[test]
    fn encodes_base64() {
        assert_eq!(base64(b"Man"), "TWFu");
        assert_eq!(base64(b"Ma"), "TWE=");
        assert_eq!(base64(b"M"), "TQ==");
    }
}
