//! Game discovery: which engine a folder uses, whether this build can run
//! it, its title, cover and text-encoding choices.

use std::path::{Path, PathBuf};

use engine_detect::{EngineKind, GameLayout};

use crate::cover::{self, Cover};
use crate::json::Value;
use crate::nls::Nls;

/// Everything a launcher needs to list a game.
#[derive(Debug, Clone)]
pub struct GameInfo {
    pub root: PathBuf,
    pub engine: EngineKind,
    pub supported: bool,
    /// Why the game cannot be played on this platform, when it cannot.
    pub unsupported_reason: Option<String>,
    pub title: String,
    pub cover: Option<Cover>,
    pub nls: Nls,
    pub evidence: Vec<String>,
}

/// Engines this build can run.
pub fn platform_supports(engine: EngineKind) -> Result<(), String> {
    match engine {
        EngineKind::Siglus | EngineKind::RealLive | EngineKind::Avg32 => Ok(()),
        EngineKind::Uk2 => {
            if cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
                Err(
                    "UK2 games need threads, which the web version does not have. \
                     Use the desktop or mobile app."
                        .to_owned(),
                )
            } else {
                Ok(())
            }
        }
        EngineKind::Unknown => Err(
            "This folder is not a SiglusEngine, RealLive, AVG32 or UK2 game. \
             Choose the folder that contains the game's files (Gameexe.ini, \
             Scene.pck, SEEN.TXT or UK2.CFG)."
                .to_owned(),
        ),
    }
}

pub fn engine_id(engine: EngineKind) -> &'static str {
    match engine {
        EngineKind::Siglus => "siglus",
        EngineKind::RealLive => "reallive",
        EngineKind::Avg32 => "avg32",
        EngineKind::Uk2 => "uk2",
        EngineKind::Unknown => "unknown",
    }
}

pub fn engine_from_id(id: &str) -> EngineKind {
    match id.trim().to_ascii_lowercase().as_str() {
        "siglus" | "siglusengine" => EngineKind::Siglus,
        "reallive" => EngineKind::RealLive,
        "avg32" => EngineKind::Avg32,
        "uk2" => EngineKind::Uk2,
        _ => EngineKind::Unknown,
    }
}

/// Probes one folder.  `with_cover` controls the (slower) cover lookup.
pub fn probe(root: &Path, nls: Option<Nls>, with_cover: bool) -> std::io::Result<GameInfo> {
    let layout = engine_detect::detect_game_root(root)?;
    let engine = layout.kind;
    let nls = Nls::for_engine(engine, nls);
    let (supported, unsupported_reason) = match platform_supports(engine) {
        Ok(()) => (true, None),
        Err(reason) => (false, Some(reason)),
    };
    let title = title(&layout, nls);
    let cover = if with_cover && engine != EngineKind::Unknown {
        cover::resolve(&layout.root, engine)
    } else {
        None
    };
    Ok(GameInfo {
        root: layout.root.clone(),
        engine,
        supported,
        unsupported_reason,
        title,
        cover,
        nls,
        evidence: layout.evidence,
    })
}

/// Finds game folders at or below `path` (at most `depth` levels down).
/// A folder recognised as a game is not searched further.
pub fn scan(path: &Path, depth: usize) -> Vec<PathBuf> {
    let mut found = Vec::new();
    scan_into(path, depth, &mut found);
    found
}

fn scan_into(path: &Path, depth: usize, found: &mut Vec<PathBuf>) {
    let kind = engine_detect::detect(path).unwrap_or(EngineKind::Unknown);
    if kind != EngineKind::Unknown {
        found.push(game_fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()));
        return;
    }
    if depth == 0 {
        return;
    }
    let Ok(entries) = game_fs::read_dir(path) else {
        return;
    };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .filter(|entry| entry.is_dir())
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
        .map(|entry| entry.path())
        .collect();
    dirs.sort();
    for dir in dirs {
        scan_into(&dir, depth - 1, found);
    }
}

fn title(layout: &GameLayout, nls: Nls) -> String {
    let fallback = || folder_name(&layout.root);
    match layout.kind {
        EngineKind::Siglus => {
            let name =
                siglus_scene_vm::runtime::game_display_info::resolve_game_name_from_project_dir(
                    &layout.root,
                );
            if name.trim().is_empty() || name == "Siglus" {
                fallback()
            } else {
                name
            }
        }
        EngineKind::RealLive | EngineKind::Avg32 => layout
            .gameexe_ini
            .as_ref()
            .and_then(|path| game_fs::read(path).ok())
            .and_then(|bytes| gameexe_caption(&bytes, nls))
            .unwrap_or_else(fallback),
        EngineKind::Uk2 | EngineKind::Unknown => fallback(),
    }
}

fn folder_name(root: &Path) -> String {
    root.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "Game".to_owned())
}

/// `#CAPTION = "..."` (RealLive and AVG32 `Gameexe.ini`).
pub fn gameexe_caption(bytes: &[u8], nls: Nls) -> Option<String> {
    let encoding = nls.encoding();
    for line in bytes.split(|&b| b == b'\n') {
        let trimmed = trim(line);
        if trimmed.len() < 8 || !trimmed[..8].eq_ignore_ascii_case(b"#CAPTION") {
            continue;
        }
        let rest = &trimmed[8..];
        let start = rest.iter().position(|&b| b == b'"')? + 1;
        let end = rest[start..].iter().position(|&b| b == b'"')? + start;
        let (text, _, _) = encoding.decode(&rest[start..end]);
        let text = text.trim().to_owned();
        if !text.is_empty() {
            return Some(text);
        }
    }
    None
}

fn trim(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .map_or(start, |i| i + 1);
    &bytes[start..end]
}

/// Stable identifier for library entries.
pub fn stable_id(root: &Path) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in root.to_string_lossy().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

impl GameInfo {
    /// `cover_path` is where the cover PNG was written, if it was.
    pub fn to_json(&self, cover_value: Value) -> Value {
        let nls_options = Nls::options(self.engine)
            .iter()
            .map(|nls| {
                Value::Object(vec![
                    ("id", Value::string(nls.id())),
                    ("label", Value::string(nls.label())),
                ])
            })
            .collect();
        Value::Object(vec![
            ("id", Value::string(stable_id(&self.root))),
            ("root", Value::string(self.root.to_string_lossy())),
            ("engine", Value::string(engine_id(self.engine))),
            ("engine_name", Value::string(self.engine.name())),
            ("supported", Value::Bool(self.supported)),
            (
                "unsupported_reason",
                Value::opt_string(self.unsupported_reason.clone()),
            ),
            ("title", Value::string(self.title.clone())),
            ("cover", cover_value),
            (
                "cover_kind",
                Value::opt_string(self.cover.as_ref().map(|cover| cover.kind.id())),
            ),
            (
                "nls",
                if Nls::options(self.engine).is_empty() {
                    Value::Null
                } else {
                    Value::string(self.nls.id())
                },
            ),
            ("nls_options", Value::Array(nls_options)),
            (
                "evidence",
                Value::Array(
                    self.evidence
                        .iter()
                        .map(|e| Value::string(e.clone()))
                        .collect(),
                ),
            ),
        ])
    }

    /// Writes the cover PNG into `cache_dir` and returns its path.
    pub fn write_cover(&self, cache_dir: &Path) -> Option<PathBuf> {
        let cover = self.cover.as_ref()?;
        std::fs::create_dir_all(cache_dir).ok()?;
        let path = cache_dir.join(format!("cover-{}.png", stable_id(&self.root)));
        std::fs::write(&path, &cover.png).ok()?;
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_caption_in_the_chosen_encoding() {
        let (sjis, _, _) = encoding_rs::SHIFT_JIS.encode("#CAPTION = \"リトルバスターズ！\"\r\n");
        let text = "#REGNAME=\"x\"\r\n".to_string();
        let mut bytes = text.into_bytes();
        bytes.extend_from_slice(&sjis);
        assert_eq!(
            gameexe_caption(&bytes, Nls::Sjis).as_deref(),
            Some("リトルバスターズ！")
        );
        let (gbk, _, _) = encoding_rs::GBK.encode("#CAPTION=\"中文标题\"\n");
        assert_eq!(gameexe_caption(&gbk, Nls::Gbk).as_deref(), Some("中文标题"));
    }

    #[test]
    fn scan_finds_nested_games_and_probe_reports_them() {
        let base = std::env::temp_dir().join(format!("launcher-scan-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("a/uk2game")).unwrap();
        std::fs::create_dir_all(base.join("b")).unwrap();
        std::fs::create_dir_all(base.join("unknown")).unwrap();
        std::fs::write(base.join("a/uk2game/UK2.CFG"), b"START = start.mes1\n").unwrap();
        std::fs::write(base.join("b/Gameexe.ini"), b"#CAPTION=\"Test\"\n").unwrap();
        std::fs::write(base.join("b/SEEN001.TXT"), b"TPC32\0\0\0").unwrap();
        let found = scan(&base, 3);
        assert_eq!(found.len(), 2);
        let info = probe(&found[1], None, false).unwrap();
        assert_eq!(info.engine, EngineKind::Avg32);
        assert_eq!(info.title, "Test");
        assert!(info.supported);
        let json = info.to_json(Value::Null).to_json();
        assert!(json.contains("\"engine\":\"avg32\""), "{json}");
        let unknown = probe(&base.join("unknown"), None, true).unwrap();
        assert!(!unknown.supported);
        std::fs::remove_dir_all(&base).unwrap();
    }
}
