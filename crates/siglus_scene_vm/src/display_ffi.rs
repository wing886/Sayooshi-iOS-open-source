//! C ABI helpers for bundle/mobile launchers that need Siglus display metadata.

use std::ffi::{CStr, CString, c_char};
use std::path::PathBuf;

use crate::runtime::game_display_info::{
    resolve_game_cover_from_project_dir, resolve_game_name_from_project_dir,
};

unsafe fn path_from_cstr(ptr: *const c_char) -> Option<PathBuf> {
    if ptr.is_null() {
        return None;
    }
    let s = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string();
    if s.is_empty() {
        None
    } else {
        Some(PathBuf::from(s))
    }
}

fn into_c_string_ptr(s: String) -> *mut c_char {
    CString::new(s)
        .unwrap_or_else(|_| CString::new("Siglus").unwrap())
        .into_raw()
}

/// Free a string returned by the display metadata functions. A null pointer is ignored.
///
/// # Safety
///
/// If non-null, `ptr` must be an allocation returned by one of this module's
/// metadata functions, with its original NUL terminator and length preserved.
/// It must not have been freed. No other access may overlap this call, and the
/// pointer must not be used after this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn siglus_string_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    drop(unsafe { CString::from_raw(ptr) });
}

/// Return the game name, or the default name if the path is null or unavailable.
/// Free a non-null result with `siglus_string_free`.
///
/// # Safety
///
/// If non-null, `game_root_utf8` must point to a NUL-terminated string in a
/// single readable allocation of at most `isize::MAX` bytes, including the
/// terminator. The bytes must remain valid and unchanged for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn siglus_game_name_from_dir(game_root_utf8: *const c_char) -> *mut c_char {
    let Some(path) = (unsafe { path_from_cstr(game_root_utf8) }) else {
        return into_c_string_ptr("Siglus".to_string());
    };
    into_c_string_ptr(resolve_game_name_from_project_dir(path))
}

/// Return the game cover path, or null if no cover is available.
/// Free a non-null result with `siglus_string_free`.
///
/// # Safety
///
/// If non-null, `game_root_utf8` must point to a NUL-terminated string in a
/// single readable allocation of at most `isize::MAX` bytes, including the
/// terminator. The bytes must remain valid and unchanged for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn siglus_game_cover_path_from_dir(
    game_root_utf8: *const c_char,
) -> *mut c_char {
    let Some(path) = (unsafe { path_from_cstr(game_root_utf8) }) else {
        return std::ptr::null_mut();
    };
    let Some(cover) = resolve_game_cover_from_project_dir(path) else {
        return std::ptr::null_mut();
    };
    into_c_string_ptr(cover.source_path.to_string_lossy().to_string())
}

/// Return the game cover MIME type, or null if no cover is available.
/// Free a non-null result with `siglus_string_free`.
///
/// # Safety
///
/// If non-null, `game_root_utf8` must point to a NUL-terminated string in a
/// single readable allocation of at most `isize::MAX` bytes, including the
/// terminator. The bytes must remain valid and unchanged for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn siglus_game_cover_mime_from_dir(
    game_root_utf8: *const c_char,
) -> *mut c_char {
    let Some(path) = (unsafe { path_from_cstr(game_root_utf8) }) else {
        return std::ptr::null_mut();
    };
    let Some(cover) = resolve_game_cover_from_project_dir(path) else {
        return std::ptr::null_mut();
    };
    into_c_string_ptr(cover.mime)
}
