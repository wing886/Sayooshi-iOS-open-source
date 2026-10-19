#![cfg(target_arch = "wasm32")]

//! Browser directory-backed virtual file access for the Siglus wasm port.
//!
//! The browser cannot open native paths from wasm. The JavaScript launcher must
//! let the user select the original game directory, build a path -> File object
//! index, and expose the synchronous functions imported below. Rust only passes
//! normalized Siglus relative paths such as `Gameexe.ini`, `Scene.pck`,
//! `bgm/BGM016.nwa`, or `g00/sys10_tt01.g00`.
//!
//! This module intentionally uses path-only requests. File ids, offsets, and
//! browser handles are private to the JavaScript side. It does not package the
//! game and does not read file contents during directory scanning.

use anyhow::{Context, Result, anyhow, bail};
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = siglusFileExists)]
    fn siglus_file_exists(path: &str) -> bool;

    #[wasm_bindgen(js_name = siglusReadFile)]
    fn siglus_read_file(path: &str) -> Uint8Array;

    #[wasm_bindgen(js_name = siglusListDir)]
    fn siglus_list_dir(path: &str) -> Array;

    #[wasm_bindgen(js_name = siglusKnownFileCount)]
    fn siglus_known_file_count() -> u32;
}

/// Path-based filesystem surface used by the wasm port.
///
/// The first wasm implementation deliberately supports whole-file reads only.
/// That matches the current Siglus resource model where `Scene.pck`, G00, NWA,
/// OMV and other resources are individual files. If a specific large resource
/// later proves too expensive as a whole-file read, streaming/range I/O should
/// be added behind this trait without changing engine-level resource semantics.
pub trait SiglusVfs {
    fn exists(&self, path: &str) -> bool;
    fn read_all(&self, path: &str) -> Result<Vec<u8>>;
    fn list_dir(&self, path: &str) -> Result<Vec<String>>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WasmDirectoryVfs;

impl WasmDirectoryVfs {
    pub fn new() -> Self {
        Self
    }

    pub fn known_file_count(&self) -> u32 {
        siglus_known_file_count()
    }
}

impl SiglusVfs for WasmDirectoryVfs {
    fn exists(&self, path: &str) -> bool {
        siglus_file_exists(&normalize_siglus_path(path))
    }

    fn read_all(&self, path: &str) -> Result<Vec<u8>> {
        let normalized = normalize_siglus_path(path);
        if normalized.is_empty() {
            bail!("empty wasm resource path");
        }
        if !siglus_file_exists(&normalized) {
            bail!("wasm resource not found: {normalized}");
        }

        let arr = siglus_read_file(&normalized);
        let len = arr.length() as usize;
        let mut out = vec![0u8; len];
        arr.copy_to(&mut out);
        Ok(out)
    }

    fn list_dir(&self, path: &str) -> Result<Vec<String>> {
        let normalized = normalize_siglus_path(path);
        let array = siglus_list_dir(&normalized);
        let mut out = Vec::with_capacity(array.length() as usize);
        for value in array.iter() {
            let Some(s) = value.as_string() else {
                return Err(anyhow!(
                    "siglusListDir returned a non-string entry for {normalized}"
                ));
            };
            out.push(s);
        }
        Ok(out)
    }
}

/// Convenience export for JavaScript-side diagnostics.
#[wasm_bindgen]
pub fn siglus_wasm_vfs_file_count() -> u32 {
    WasmDirectoryVfs::new().known_file_count()
}

/// Convenience export for JavaScript-side diagnostics.
#[wasm_bindgen]
pub fn siglus_wasm_vfs_exists(path: &str) -> bool {
    WasmDirectoryVfs::new().exists(path)
}

/// Convenience export for JavaScript-side diagnostics.
#[wasm_bindgen]
pub fn siglus_wasm_vfs_read_len(path: &str) -> std::result::Result<u32, JsValue> {
    let bytes = WasmDirectoryVfs::new()
        .read_all(path)
        .with_context(|| format!("read wasm resource {path}"))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(bytes.len() as u32)
}

pub fn normalize_siglus_path(path: &str) -> String {
    path.replace('\\', "/")
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect::<Vec<_>>()
        .join("/")
}
