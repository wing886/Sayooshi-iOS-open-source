//! File access for the game engines.
//!
//! On native targets every function is a thin wrapper around `std::fs`.  On
//! the web (`wasm32-unknown-unknown`) there is no filesystem: the launcher
//! page indexes the folder the user picked and exposes it through the same
//! synchronous JavaScript functions the Siglus web port already uses
//! (`siglusFileExists`, `siglusReadFile`, `siglusListDir`), plus
//! `gameFsWrite`/`gameFsRemove` for save data.  Paths are then relative to
//! the selected game root and use `/` separators.

use std::io::{self, Read, Seek};
use std::path::{Path, PathBuf};

/// A readable, seekable file.
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

/// One directory entry, shaped like [`std::fs::DirEntry`].
#[derive(Debug, Clone)]
pub struct DirEntry {
    path: PathBuf,
    name: String,
    is_dir: bool,
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn file_name(&self) -> std::ffi::OsString {
        self.name.clone().into()
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir
    }
}

pub type ReadDir = std::vec::IntoIter<io::Result<DirEntry>>;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod imp {
    use super::*;
    use std::fs;

    pub fn read(path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    pub fn write(path: &Path, bytes: &[u8]) -> io::Result<()> {
        fs::write(path, bytes)
    }

    pub fn create_dir_all(path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    pub fn remove_file(path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    pub fn remove_dir_all(path: &Path) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    pub fn read_dir(path: &Path) -> io::Result<ReadDir> {
        let mut out = Vec::new();
        for entry in fs::read_dir(path)? {
            out.push(entry.map(|entry| DirEntry {
                path: entry.path(),
                name: entry.file_name().to_string_lossy().into_owned(),
                is_dir: entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
                    || entry.path().is_dir(),
            }));
        }
        Ok(out.into_iter())
    }

    pub fn is_file(path: &Path) -> bool {
        path.is_file()
    }

    pub fn is_dir(path: &Path) -> bool {
        path.is_dir()
    }

    pub fn file_len(path: &Path) -> io::Result<u64> {
        Ok(fs::metadata(path)?.len())
    }

    pub fn open(path: &Path) -> io::Result<Box<dyn ReadSeek>> {
        Ok(Box::new(fs::File::open(path)?))
    }

    pub fn canonicalize(path: &Path) -> io::Result<PathBuf> {
        fs::canonicalize(path)
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod imp {
    use super::*;
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = siglusFileExists)]
        fn js_exists(path: &str) -> bool;
        #[wasm_bindgen(catch, js_name = siglusReadFile)]
        fn js_read(path: &str) -> Result<Uint8Array, JsValue>;
        #[wasm_bindgen(js_name = siglusListDir)]
        fn js_list(path: &str) -> Array;
        #[wasm_bindgen(catch, js_name = gameFsWrite)]
        fn js_write(path: &str, bytes: &[u8]) -> Result<(), JsValue>;
        #[wasm_bindgen(catch, js_name = gameFsRemove)]
        fn js_remove(path: &str) -> Result<(), JsValue>;
    }

    /// Relative `/`-separated form used by the page's file index.
    pub(crate) fn key(path: &Path) -> String {
        let text = path.to_string_lossy().replace('\\', "/");
        let mut parts: Vec<&str> = Vec::new();
        for part in text.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    parts.pop();
                }
                other => parts.push(other),
            }
        }
        parts.join("/")
    }

    fn js_error(error: JsValue) -> io::Error {
        let text = error
            .as_string()
            .or_else(|| js_sys::Error::from(error).message().as_string())
            .unwrap_or_else(|| "JavaScript error".to_owned());
        io::Error::other(text)
    }

    fn not_found(path: &Path) -> io::Error {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} not found", path.display()),
        )
    }

    fn children(key: &str) -> Vec<String> {
        js_list(key)
            .iter()
            .filter_map(|value| value.as_string())
            .collect()
    }

    pub fn read(path: &Path) -> io::Result<Vec<u8>> {
        let key = key(path);
        if !js_exists(&key) {
            return Err(not_found(path));
        }
        Ok(js_read(&key).map_err(js_error)?.to_vec())
    }

    pub fn write(path: &Path, bytes: &[u8]) -> io::Result<()> {
        js_write(&key(path), bytes).map_err(js_error)
    }

    pub fn create_dir_all(_path: &Path) -> io::Result<()> {
        Ok(())
    }

    pub fn remove_file(path: &Path) -> io::Result<()> {
        js_remove(&key(path)).map_err(js_error)
    }

    pub fn remove_dir_all(path: &Path) -> io::Result<()> {
        let key = key(path);
        for child in children(&key) {
            let child = if key.is_empty() {
                child
            } else {
                format!("{key}/{child}")
            };
            let child = Path::new(&child);
            if is_dir(child) {
                remove_dir_all(child)?;
            } else {
                remove_file(child)?;
            }
        }
        Ok(())
    }

    pub fn read_dir(path: &Path) -> io::Result<ReadDir> {
        if !is_dir(path) {
            return Err(not_found(path));
        }
        let key = key(path);
        let out: Vec<_> = children(&key)
            .into_iter()
            .map(|name| {
                let child = path.join(&name);
                let is_dir = !js_exists(&super::imp::key(&child));
                Ok(DirEntry {
                    path: child,
                    name,
                    is_dir,
                })
            })
            .collect();
        Ok(out.into_iter())
    }

    pub fn is_file(path: &Path) -> bool {
        let key = key(path);
        !key.is_empty() && js_exists(&key)
    }

    pub fn is_dir(path: &Path) -> bool {
        let key = key(path);
        key.is_empty() || (!js_exists(&key) && js_list(&key).length() > 0)
    }

    pub fn file_len(path: &Path) -> io::Result<u64> {
        Ok(read(path)?.len() as u64)
    }

    pub fn open(path: &Path) -> io::Result<Box<dyn ReadSeek>> {
        Ok(Box::new(io::Cursor::new(read(path)?)))
    }

    pub fn canonicalize(path: &Path) -> io::Result<PathBuf> {
        let key = key(path);
        if key.is_empty() || js_exists(&key) || is_dir(path) {
            Ok(PathBuf::from(key))
        } else {
            Err(not_found(path))
        }
    }
}

pub fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    imp::read(path.as_ref())
}

pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<String> {
    String::from_utf8(read(path)?)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> io::Result<()> {
    imp::write(path.as_ref(), bytes.as_ref())
}

pub fn create_dir_all(path: impl AsRef<Path>) -> io::Result<()> {
    imp::create_dir_all(path.as_ref())
}

pub fn remove_file(path: impl AsRef<Path>) -> io::Result<()> {
    imp::remove_file(path.as_ref())
}

pub fn remove_dir_all(path: impl AsRef<Path>) -> io::Result<()> {
    imp::remove_dir_all(path.as_ref())
}

pub fn read_dir(path: impl AsRef<Path>) -> io::Result<ReadDir> {
    imp::read_dir(path.as_ref())
}

pub fn is_file(path: impl AsRef<Path>) -> bool {
    imp::is_file(path.as_ref())
}

pub fn is_dir(path: impl AsRef<Path>) -> bool {
    imp::is_dir(path.as_ref())
}

pub fn exists(path: impl AsRef<Path>) -> bool {
    is_file(path.as_ref()) || is_dir(path.as_ref())
}

pub fn file_len(path: impl AsRef<Path>) -> io::Result<u64> {
    imp::file_len(path.as_ref())
}

pub fn open(path: impl AsRef<Path>) -> io::Result<Box<dyn ReadSeek>> {
    imp::open(path.as_ref())
}

pub fn canonicalize(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    imp::canonicalize(path.as_ref())
}

static HOST_FONTS: std::sync::Mutex<Vec<std::sync::Arc<Vec<u8>>>> =
    std::sync::Mutex::new(Vec::new());

/// Registers a font file supplied by the host app (for example the system
/// CJK font on iOS, or a font the web page downloaded).  Engines try host
/// fonts before their built-in lists of system font paths.
pub fn add_host_font(bytes: Vec<u8>) {
    if let Ok(mut fonts) = HOST_FONTS.lock() {
        fonts.push(std::sync::Arc::new(bytes));
    }
}

/// Registers a host font by path (read with `std::fs`, not the game index).
pub fn add_host_font_path(path: impl AsRef<Path>) -> io::Result<()> {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        add_host_font(std::fs::read(path)?);
        Ok(())
    }
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        add_host_font(read(path)?);
        Ok(())
    }
}

/// Fonts registered with [`add_host_font`], oldest first.
pub fn host_fonts() -> Vec<std::sync::Arc<Vec<u8>>> {
    HOST_FONTS
        .lock()
        .map(|fonts| fonts.clone())
        .unwrap_or_default()
}

/// Well-known CJK system font files on Android.
pub const ANDROID_CJK_FONTS: &[&str] = &[
    "/system/fonts/NotoSansCJK-Regular.ttc",
    "/system/fonts/NotoSerifCJK-Regular.ttc",
    "/system/fonts/DroidSansJapanese.ttf",
    "/system/fonts/DroidSansFallbackFull.ttf",
    "/system/fonts/DroidSansFallback.ttf",
];

/// Reads at most `len` bytes from the start of a file.
pub fn read_head(path: impl AsRef<Path>, len: usize) -> Vec<u8> {
    let mut head = Vec::with_capacity(len);
    if let Ok(file) = open(path) {
        let _ = file.take(len as u64).read_to_end(&mut head);
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_round_trip() {
        let dir = std::env::temp_dir().join(format!("game-fs-{}", std::process::id()));
        let _ = remove_dir_all(&dir);
        create_dir_all(dir.join("sub")).unwrap();
        write(dir.join("sub/a.bin"), b"hello").unwrap();
        assert!(is_dir(dir.join("sub")));
        assert!(is_file(dir.join("sub/a.bin")));
        assert_eq!(read_head(dir.join("sub/a.bin"), 2), b"he");
        let names: Vec<_> = read_dir(dir.join("sub"))
            .unwrap()
            .flatten()
            .map(|e| e.file_name())
            .collect();
        assert_eq!(names, vec![std::ffi::OsString::from("a.bin")]);
        remove_dir_all(&dir).unwrap();
        assert!(!exists(&dir));
    }
}
