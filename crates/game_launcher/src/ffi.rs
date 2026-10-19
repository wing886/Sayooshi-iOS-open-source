//! C API for the native launchers (macOS, iOS, Android).
//!
//! Strings returned by this module are UTF-8, NUL-terminated and must be
//! freed with [`game_string_free`].  Metadata is returned as JSON (see
//! `GameInfo::to_json`).  Games of the software-rendered engines are driven
//! frame by frame through the `game_fb_*` functions; SiglusEngine games keep
//! using the `siglus_*` host API.

use std::ffi::{CStr, CString, c_char};
use std::path::{Path, PathBuf};
use std::ptr;

use crate::json::Value;
use crate::nls::Nls;
use crate::probe;
use crate::runtime::{self, FramebufferGame, GameKey, PointerButton};

unsafe fn str_arg<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .ok()
        .filter(|text| !text.is_empty())
}

fn into_c(text: String) -> *mut c_char {
    CString::new(text.replace('\0', ""))
        .map(CString::into_raw)
        .unwrap_or(ptr::null_mut())
}

/// Frees a string returned by this API. Null is ignored.
///
/// # Safety
///
/// `ptr` must be null or a string returned by this module, not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}

fn probe_value(root: &Path, nls: Option<Nls>, cache: Option<&Path>) -> Value {
    match probe::probe(root, nls, true) {
        Ok(info) => {
            let cover = cache
                .and_then(|cache| info.write_cover(cache))
                .map(|path| path.to_string_lossy().into_owned());
            info.to_json(Value::opt_string(cover))
        }
        Err(error) => Value::Object(vec![
            ("root", Value::string(root.to_string_lossy())),
            ("engine", Value::string("unknown")),
            ("supported", Value::Bool(false)),
            ("unsupported_reason", Value::string(error.to_string())),
            ("title", Value::string(folder_name(root))),
            ("cover", Value::Null),
        ]),
    }
}

fn folder_name(root: &Path) -> String {
    root.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Describes one game folder as JSON.  `nls` (optional) selects the text
/// encoding used for the title; `cover_cache_dir` (optional) receives the
/// cover PNG, whose path is reported as `cover`.
///
/// # Safety
///
/// Pointer arguments must be null or NUL-terminated strings valid for the
/// duration of the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_probe_json(
    root: *const c_char,
    nls: *const c_char,
    cover_cache_dir: *const c_char,
) -> *mut c_char {
    let Some(root) = (unsafe { str_arg(root) }) else {
        return ptr::null_mut();
    };
    let nls = unsafe { str_arg(nls) }.and_then(Nls::parse);
    let cache = unsafe { str_arg(cover_cache_dir) }.map(PathBuf::from);
    into_c(probe_value(Path::new(root), nls, cache.as_deref()).to_json())
}

/// Finds every game at or below `path` (up to `depth` folder levels) and
/// describes each as in [`game_probe_json`], as a JSON array.  An empty
/// array means nothing recognisable was found.
///
/// # Safety
///
/// As for [`game_probe_json`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_scan_json(
    path: *const c_char,
    depth: i32,
    cover_cache_dir: *const c_char,
) -> *mut c_char {
    let Some(path) = (unsafe { str_arg(path) }) else {
        return ptr::null_mut();
    };
    let cache = unsafe { str_arg(cover_cache_dir) }.map(PathBuf::from);
    let roots = probe::scan(Path::new(path), depth.clamp(0, 8) as usize);
    let items = roots
        .iter()
        .map(|root| probe_value(root, None, cache.as_deref()))
        .collect();
    into_c(Value::Array(items).to_json())
}

/// Registers a font file (for example the system CJK font the app found)
/// for the engines' text.  Returns 0 on success.
///
/// # Safety
///
/// `path` must be null or a NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_add_font_file(path: *const c_char) -> i32 {
    match unsafe { str_arg(path) } {
        Some(path) => i32::from(game_fs::add_host_font_path(path).is_err()),
        None => 1,
    }
}

pub struct FbHandle {
    game: Box<dyn FramebufferGame>,
    title: Option<CString>,
}

/// Opens a RealLive, AVG32 or UK2 game for frame-by-frame hosting.
/// `engine` may be null to detect it.  On failure returns null and, when
/// `error_out` is non-null, stores a message there (free it with
/// [`game_string_free`]).
///
/// # Safety
///
/// String arguments must be null or NUL-terminated; `error_out` must be
/// null or writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_open(
    root: *const c_char,
    engine: *const c_char,
    nls: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut FbHandle {
    let report = |message: String| {
        log::error!("game_fb_open: {message}");
        if !error_out.is_null() {
            unsafe { *error_out = into_c(message) };
        }
        ptr::null_mut()
    };
    let Some(root) = (unsafe { str_arg(root) }) else {
        return report("no game folder given".to_owned());
    };
    let engine = unsafe { str_arg(engine) }
        .map(probe::engine_from_id)
        .unwrap_or(engine_detect::EngineKind::Unknown);
    let nls = unsafe { str_arg(nls) }.and_then(Nls::parse);
    match runtime::open(Path::new(root), engine, nls) {
        Ok(game) => Box::into_raw(Box::new(FbHandle { game, title: None })),
        Err(error) => report(format!("{error:#}")),
    }
}

fn handle<'a>(ptr: *mut FbHandle) -> Option<&'a mut FbHandle> {
    unsafe { ptr.as_mut() }
}

/// Advances one display frame. Returns 0 while running, 1 once the game
/// has ended (the host should close it).
///
/// # Safety
///
/// `game` must be null or a live handle from [`game_fb_open`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_step(game: *mut FbHandle, dt_ms: u32) -> i32 {
    match handle(game) {
        Some(handle) => i32::from(!handle.game.step(dt_ms)),
        None => 1,
    }
}

/// The current RGBA8 frame (tightly packed rows).  The pointer stays valid
/// until the next call on this handle.
///
/// # Safety
///
/// `game` must be a live handle; `width`/`height` must be writable or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_frame(
    game: *mut FbHandle,
    width: *mut u32,
    height: *mut u32,
) -> *const u8 {
    let Some(handle) = handle(game) else {
        return ptr::null();
    };
    let (w, h) = handle.game.size();
    let frame = handle.game.frame();
    if frame.len() < (w * h * 4) as usize {
        return ptr::null();
    }
    unsafe {
        if !width.is_null() {
            *width = w;
        }
        if !height.is_null() {
            *height = h;
        }
    }
    frame.as_ptr()
}

/// Pointer position in frame pixels.
///
/// # Safety
///
/// `game` must be null or a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_pointer_move(game: *mut FbHandle, x: i32, y: i32) {
    if let Some(handle) = handle(game) {
        handle.game.pointer_move(x, y);
    }
}

/// `button`: 0 left, 1 right.
///
/// # Safety
///
/// `game` must be null or a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_pointer_button(game: *mut FbHandle, button: i32, pressed: i32) {
    if let Some(handle) = handle(game) {
        let button = if button == 1 {
            PointerButton::Right
        } else {
            PointerButton::Left
        };
        handle.game.pointer_button(button, pressed != 0);
    }
}

/// # Safety
///
/// `game` must be null or a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_wheel(game: *mut FbHandle, up: i32) {
    if let Some(handle) = handle(game) {
        handle.game.wheel(up != 0);
    }
}

/// Key codes: 1 Enter, 2 Escape, 3 Space, 4 Up, 5 Down, 6 Left, 7 Right,
/// 8 PageUp, 9 PageDown, 10 Home, 11 End, 12 Backspace, 13 Tab, 14 Ctrl,
/// 15 Shift, 0x100+n F-n, 0x10000+c the Unicode character c.
///
/// # Safety
///
/// `game` must be null or a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_key(game: *mut FbHandle, code: u32, pressed: i32) {
    if let (Some(handle), Some(key)) = (handle(game), GameKey::from_code(code)) {
        handle.game.key(key, pressed != 0);
    }
}

/// Committed text input.
///
/// # Safety
///
/// `game` must be null or a live handle; `text` null or NUL-terminated.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_text(game: *mut FbHandle, text: *const c_char) {
    if let (Some(handle), Some(text)) = (handle(game), unsafe { str_arg(text) }) {
        handle.game.text(text);
    }
}

/// 1 when the host should show its pointer.
///
/// # Safety
///
/// `game` must be null or a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_cursor_visible(game: *mut FbHandle) -> i32 {
    handle(game).map_or(0, |handle| i32::from(handle.game.cursor_visible()))
}

/// Window title; valid until the next call on this handle (do not free).
///
/// # Safety
///
/// `game` must be null or a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_title(game: *mut FbHandle) -> *const c_char {
    let Some(handle) = handle(game) else {
        return ptr::null();
    };
    handle.title = CString::new(handle.game.title().replace('\0', "")).ok();
    handle
        .title
        .as_ref()
        .map_or(ptr::null(), |title| title.as_ptr())
}

/// Saves persistent data, stops audio and frees the handle.
///
/// # Safety
///
/// `game` must be null or a live handle, which is invalid afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_fb_close(game: *mut FbHandle) {
    if game.is_null() {
        return;
    }
    let mut handle = unsafe { Box::from_raw(game) };
    handle.game.shutdown();
}

/// Runs any supported game in its own window until it ends (desktop and
/// the macOS app).  SiglusEngine games use the Siglus desktop host.
/// Returns 0 on a normal exit.
///
/// # Safety
///
/// String arguments must be null or NUL-terminated.  Call on the main
/// thread.
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn game_run_entry(root: *const c_char, nls: *const c_char) -> i32 {
    let Some(root_text) = (unsafe { str_arg(root) }) else {
        return 1;
    };
    let nls = unsafe { str_arg(nls) }.and_then(Nls::parse);
    crate::desktop::run(Path::new(root_text), nls)
}
