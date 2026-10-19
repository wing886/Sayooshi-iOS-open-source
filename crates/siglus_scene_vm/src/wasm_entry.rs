#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

//! Browser wasm entry points for the real Siglus host.
//!
//! The JavaScript launcher owns the selected directory File objects. Rust sees
//! only Siglus relative paths through `wasm_vfs`, and this module starts the
//! same host/renderer/VM pipeline used by native platforms.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::web::WindowAttributesWeb;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::host::{SiglusHost, SiglusHostConfig};
use crate::render::Renderer;
use crate::runtime::input::{VmKey, VmMouseButton};
use crate::wasm_vfs::{SiglusVfs, WasmDirectoryVfs};

#[wasm_bindgen]
pub fn siglus_wasm_start(canvas_id: &str) -> Result<(), JsValue> {
    start_siglus_from_directory(canvas_id.to_owned(), String::new())
}

#[wasm_bindgen]
pub fn start_siglus_from_directory(canvas_id: String, files_json: String) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let vfs = WasmDirectoryVfs::new();
    let file_count = vfs.known_file_count();
    if canvas_id.trim().is_empty() {
        return Err(JsValue::from_str("empty canvas id"));
    }
    if file_count == 0 {
        return Err(JsValue::from_str("no Siglus files are registered"));
    }
    if !vfs.exists("Gameexe.ini") && !vfs.exists("Gameexe.dat") && !vfs.exists("Scene.pck") {
        return Err(JsValue::from_str(
            "selected directory does not look like a Siglus game root: missing Gameexe.ini/Gameexe.dat/Scene.pck",
        ));
    }

    web_sys::console::log_1(&JsValue::from_str(&format!(
        "siglus_rs wasm starting: canvas={canvas_id}, files={file_count}, metadata_bytes={}",
        files_json.len()
    )));

    let event_loop = EventLoop::new()
        .map_err(|e| JsValue::from_str(&format!("create wasm event loop: {e:?}")))?;
    let proxy = event_loop.create_proxy();
    let app = WasmApp::new(canvas_id, proxy);
    event_loop
        .run_app(app)
        .map_err(|e| JsValue::from_str(&format!("run wasm event loop: {e:?}")))?;
    Ok(())
}

type HostInitResult = Result<Box<SiglusHost>, String>;

struct WasmApp {
    canvas_id: String,
    proxy: EventLoopProxy,
    pending_host: Rc<RefCell<Option<HostInitResult>>>,
    window: Option<&'static dyn Window>,
    window_id: Option<WindowId>,
    host: Option<Box<SiglusHost>>,
    init_started: bool,
    init_error: Option<String>,
    exit_requested: bool,
}

impl WasmApp {
    fn new(canvas_id: String, proxy: EventLoopProxy) -> Self {
        Self {
            canvas_id,
            proxy,
            pending_host: Rc::new(RefCell::new(None)),
            window: None,
            window_id: None,
            host: None,
            init_started: false,
            init_error: None,
            exit_requested: false,
        }
    }

    fn ensure_created(&mut self, elwt: &dyn ActiveEventLoop) {
        if self.window.is_some() || self.init_started || self.init_error.is_some() {
            return;
        }
        self.init_started = true;

        let canvas = match lookup_canvas(&self.canvas_id) {
            Ok(c) => c,
            Err(e) => {
                self.init_error = Some(e.clone());
                log_js_error(&e);
                elwt.exit();
                return;
            }
        };

        let css_w = canvas.client_width().max(1) as u32;
        let css_h = canvas.client_height().max(1) as u32;
        if canvas.width() == 0 || canvas.height() == 0 {
            let dpr = web_sys::window()
                .map(|w| w.device_pixel_ratio())
                .unwrap_or(1.0)
                .max(1.0);
            canvas.set_width(((css_w as f64) * dpr).round().max(1.0) as u32);
            canvas.set_height(((css_h as f64) * dpr).round().max(1.0) as u32);
        }

        let attrs = WindowAttributes::default()
            .with_title("siglus_rs")
            .with_surface_size(LogicalSize::new(css_w as f64, css_h as f64))
            .with_platform_attributes(Box::new(
                WindowAttributesWeb::default()
                    .with_canvas(Some(canvas))
                    .with_prevent_default(true)
                    .with_focusable(true)
                    .with_append(false),
            ));

        let window = match elwt.create_window(attrs) {
            Ok(w) => w,
            Err(e) => {
                let msg = format!("create wasm window: {e:?}");
                self.init_error = Some(msg.clone());
                log_js_error(&msg);
                elwt.exit();
                return;
            }
        };
        let window: &'static dyn Window = Box::leak(window);
        self.window_id = Some(window.id());
        self.window = Some(window);

        let proxy = self.proxy.clone();
        let pending_host = self.pending_host.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = async move {
                let renderer = Renderer::new(window)
                    .await
                    .map_err(|e| format!("renderer init: {e:#}"))?;
                let mut config = SiglusHostConfig::new(PathBuf::from("."));
                config.width = Some(1280);
                config.height = Some(720);
                let host = SiglusHost::new_with_renderer(config, renderer)
                    .await
                    .map_err(|e| format!("host init: {e:#}"))?;
                Ok(Box::new(host))
            }
            .await;
            *pending_host.borrow_mut() = Some(result);
            proxy.wake_up();
        });
    }

    fn handle_window_event(&mut self, event: WindowEvent, elwt: &dyn ActiveEventLoop) {
        let Some(host) = self.host.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                self.exit_requested = true;
                elwt.exit();
            }
            WindowEvent::SurfaceResized(size) => {
                let sf = self.window.map(|w| w.scale_factor() as f32).unwrap_or(1.0);
                host.resize(size.width.max(1), size.height.max(1), sf);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(code),
                        text,
                        ..
                    },
                ..
            } => {
                if let Some(k) = map_keycode(code) {
                    host.key_down(k);
                }
                if host.vm_mut().ctx.editbox_accepts_direct_text() {
                    if let Some(text) = text.as_deref() {
                        host.text_input(text);
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        physical_key: PhysicalKey::Code(code),
                        ..
                    },
                ..
            } => {
                if let Some(k) = map_keycode(code) {
                    host.key_up(k);
                }
            }
            WindowEvent::Ime(winit::event::Ime::Preedit(text, cursor)) => {
                host.ime_preedit(&text, cursor)
            }
            WindowEvent::Ime(winit::event::Ime::Commit(text)) => host.text_input(&text),
            WindowEvent::Ime(winit::event::Ime::Disabled) => host.ime_disabled(),
            WindowEvent::Ime(winit::event::Ime::Enabled) => {}
            WindowEvent::PointerMoved {
                position,
                primary: true,
                ..
            }
            | WindowEvent::PointerEntered {
                position,
                primary: true,
                ..
            } => {
                let (x, y) = if let Some(w) = self.window {
                    let p = position.to_logical::<f64>(w.scale_factor());
                    (p.x, p.y)
                } else {
                    (position.x, position.y)
                };
                host.mouse_move(x, y);
            }
            WindowEvent::PointerButton {
                state,
                button,
                position,
                primary: true,
                ..
            } => {
                let Some(button) = button.mouse_button() else {
                    return;
                };
                let point = position
                    .to_logical::<f64>(self.window.map(|w| w.scale_factor()).unwrap_or(1.0));
                host.mouse_move(point.x, point.y);
                if let Some(b) = map_mouse_button(button) {
                    match (state, b) {
                        (ElementState::Pressed, VmMouseButton::Left) => {
                            let (x, y) = current_mouse_pos(host);
                            host.touch(0, x, y);
                        }
                        (ElementState::Released, VmMouseButton::Left) => {
                            let (x, y) = current_mouse_pos(host);
                            host.touch(2, x, y);
                        }
                        (ElementState::Pressed, b) => host.mouse_down(b),
                        (ElementState::Released, b) => host.mouse_up(b),
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => (y * 120.0) as i32,
                    MouseScrollDelta::PixelDelta(p) => p.y.round() as i32,
                    _ => 0,
                };
                host.mouse_wheel(dy);
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window {
                    apply_ime_window_state(window, host);
                }
                match host.step(16) {
                    Ok(true) => {
                        self.exit_requested = true;
                        elwt.exit();
                    }
                    Ok(false) => {
                        if let Some(w) = self.window {
                            w.request_redraw();
                        }
                    }
                    Err(e) => {
                        let msg = format!("siglus wasm step failed: {e:#}");
                        log_js_error(&msg);
                    }
                }
            }
            _ => {}
        }
    }
}

impl ApplicationHandler for WasmApp {
    fn can_create_surfaces(&mut self, elwt: &dyn ActiveEventLoop) {
        self.ensure_created(elwt);
    }

    fn proxy_wake_up(&mut self, elwt: &dyn ActiveEventLoop) {
        let Some(result) = self.pending_host.borrow_mut().take() else {
            return;
        };
        match result {
            Ok(host) => {
                self.host = Some(host);
                if let Some(w) = self.window {
                    w.request_redraw();
                }
            }
            Err(e) => {
                self.init_error = Some(e.clone());
                log_js_error(&e);
                elwt.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        elwt: &dyn ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.window_id == Some(window_id) {
            self.handle_window_event(event, elwt);
        }
    }

    fn about_to_wait(&mut self, elwt: &dyn ActiveEventLoop) {
        if self.exit_requested {
            elwt.exit();
            return;
        }
        if let Some(w) = self.window {
            w.request_redraw();
        }
        elwt.set_control_flow(ControlFlow::Poll);
    }
}

fn lookup_canvas(id: &str) -> Result<web_sys::HtmlCanvasElement, String> {
    let window = web_sys::window().ok_or_else(|| "window is unavailable".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "document is unavailable".to_string())?;
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| format!("canvas element not found: {id}"))?;
    element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| format!("element is not a canvas: {id}"))
}

fn log_js_error(msg: &str) {
    web_sys::console::error_1(&JsValue::from_str(msg));
}

fn apply_ime_window_state(window: &dyn Window, host: &mut SiglusHost) {
    if let Some((x, y, width, height)) = host.vm_mut().ctx.focused_editbox_ime_area() {
        crate::ime::enable_ime(
            window,
            LogicalPosition::new(x.max(0) as f64, y.max(0) as f64).into(),
            LogicalSize::new(width.max(1) as f64, height.max(1) as f64).into(),
        );
    } else {
        crate::ime::disable_ime(window);
    }
}

fn current_mouse_pos(host: &mut SiglusHost) -> (f64, f64) {
    let input = &host.vm_mut().ctx.input;
    (input.mouse_x as f64, input.mouse_y as f64)
}

fn map_mouse_button(button: MouseButton) -> Option<VmMouseButton> {
    match button {
        MouseButton::Left => Some(VmMouseButton::Left),
        MouseButton::Right => Some(VmMouseButton::Right),
        MouseButton::Middle => Some(VmMouseButton::Middle),
        _ => None,
    }
}

fn map_keycode(code: KeyCode) -> Option<VmKey> {
    match code {
        KeyCode::Enter | KeyCode::NumpadEnter => Some(VmKey::Enter),
        KeyCode::Space => Some(VmKey::Space),
        KeyCode::Escape => Some(VmKey::Escape),
        KeyCode::Backspace => Some(VmKey::Backspace),
        KeyCode::Delete => Some(VmKey::Delete),
        KeyCode::Tab => Some(VmKey::Tab),
        KeyCode::Home => Some(VmKey::Home),
        KeyCode::End => Some(VmKey::End),
        KeyCode::ArrowUp => Some(VmKey::ArrowUp),
        KeyCode::ArrowDown => Some(VmKey::ArrowDown),
        KeyCode::ArrowLeft => Some(VmKey::ArrowLeft),
        KeyCode::ArrowRight => Some(VmKey::ArrowRight),
        KeyCode::ShiftLeft | KeyCode::ShiftRight => Some(VmKey::Shift),
        KeyCode::ControlLeft | KeyCode::ControlRight => Some(VmKey::Control),
        KeyCode::MetaLeft | KeyCode::MetaRight => Some(VmKey::Meta),
        KeyCode::AltLeft | KeyCode::AltRight => Some(VmKey::Alt),
        KeyCode::Digit0 => Some(VmKey::Digit(0)),
        KeyCode::Digit1 => Some(VmKey::Digit(1)),
        KeyCode::Digit2 => Some(VmKey::Digit(2)),
        KeyCode::Digit3 => Some(VmKey::Digit(3)),
        KeyCode::Digit4 => Some(VmKey::Digit(4)),
        KeyCode::Digit5 => Some(VmKey::Digit(5)),
        KeyCode::Digit6 => Some(VmKey::Digit(6)),
        KeyCode::Digit7 => Some(VmKey::Digit(7)),
        KeyCode::Digit8 => Some(VmKey::Digit(8)),
        KeyCode::Digit9 => Some(VmKey::Digit(9)),
        KeyCode::KeyA => Some(VmKey::Letter('A')),
        KeyCode::KeyB => Some(VmKey::Letter('B')),
        KeyCode::KeyC => Some(VmKey::Letter('C')),
        KeyCode::KeyD => Some(VmKey::Letter('D')),
        KeyCode::KeyE => Some(VmKey::Letter('E')),
        KeyCode::KeyF => Some(VmKey::Letter('F')),
        KeyCode::KeyG => Some(VmKey::Letter('G')),
        KeyCode::KeyH => Some(VmKey::Letter('H')),
        KeyCode::KeyI => Some(VmKey::Letter('I')),
        KeyCode::KeyJ => Some(VmKey::Letter('J')),
        KeyCode::KeyK => Some(VmKey::Letter('K')),
        KeyCode::KeyL => Some(VmKey::Letter('L')),
        KeyCode::KeyM => Some(VmKey::Letter('M')),
        KeyCode::KeyN => Some(VmKey::Letter('N')),
        KeyCode::KeyO => Some(VmKey::Letter('O')),
        KeyCode::KeyP => Some(VmKey::Letter('P')),
        KeyCode::KeyQ => Some(VmKey::Letter('Q')),
        KeyCode::KeyR => Some(VmKey::Letter('R')),
        KeyCode::KeyS => Some(VmKey::Letter('S')),
        KeyCode::KeyT => Some(VmKey::Letter('T')),
        KeyCode::KeyU => Some(VmKey::Letter('U')),
        KeyCode::KeyV => Some(VmKey::Letter('V')),
        KeyCode::KeyW => Some(VmKey::Letter('W')),
        KeyCode::KeyX => Some(VmKey::Letter('X')),
        KeyCode::KeyY => Some(VmKey::Letter('Y')),
        KeyCode::KeyZ => Some(VmKey::Letter('Z')),
        KeyCode::F1 => Some(VmKey::F(1)),
        KeyCode::F2 => Some(VmKey::F(2)),
        KeyCode::F3 => Some(VmKey::F(3)),
        KeyCode::F4 => Some(VmKey::F(4)),
        KeyCode::F5 => Some(VmKey::F(5)),
        KeyCode::F6 => Some(VmKey::F(6)),
        KeyCode::F7 => Some(VmKey::F(7)),
        KeyCode::F8 => Some(VmKey::F(8)),
        KeyCode::F9 => Some(VmKey::F(9)),
        KeyCode::F10 => Some(VmKey::F(10)),
        KeyCode::F11 => Some(VmKey::F(11)),
        KeyCode::F12 => Some(VmKey::F(12)),
        _ => None,
    }
}
