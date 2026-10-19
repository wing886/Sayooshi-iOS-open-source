//! Identifies the engine behind a game directory.
//!
//! The decision is made from on-disk formats only (never from publisher or
//! title names):
//!
//! | Engine      | Evidence |
//! |-------------|----------|
//! | SiglusEngine | `Scene.pck` or `Gameexe.dat` |
//! | UK2         | `UK2.CFG`, or `.MES` files with the `<< UK2 TEXT Ver1.00 >>` header |
//! | AVG32       | `Gameexe.ini` plus a `PACL` `SEEN.TXT` or loose `TPC32` `SEEN###.TXT` scenes |
//! | RealLive    | `Gameexe.ini` plus a 10000-entry `SEEN.TXT` table of scenarios with the `0x1d0`/`0x1cc` header, or loose `SEEN####.TXT` scenes |
//!
//! RealLive's scenario archive is located through `#FOLDNAME.TXT` in
//! `Gameexe.ini`, falling back to the game root and `DAT/`.

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineKind {
    Siglus,
    RealLive,
    Avg32,
    Uk2,
    Unknown,
}

impl EngineKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Siglus => "SiglusEngine",
            Self::RealLive => "RealLive",
            Self::Avg32 => "AVG32",
            Self::Uk2 => "UK2",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for EngineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The detected engine and the files that identified it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameLayout {
    pub root: PathBuf,
    pub kind: EngineKind,
    /// `Gameexe.ini` (AVG32, RealLive; some Siglus games ship one too).
    pub gameexe_ini: Option<PathBuf>,
    /// `Gameexe.dat` (Siglus).
    pub gameexe_dat: Option<PathBuf>,
    /// `Scene.pck` (Siglus).
    pub scene_pck: Option<PathBuf>,
    /// `SEEN.TXT` scenario archive (AVG32, RealLive).
    pub seen_archive: Option<PathBuf>,
    /// Directory holding loose `SEEN*.TXT` scenes, when there is no archive.
    pub seen_directory: Option<PathBuf>,
    /// `PDT/` or `DAT/PDT/` image directory (AVG32).
    pub pdt_root: Option<PathBuf>,
    /// `UK2.CFG` (UK2).
    pub uk2_config: Option<PathBuf>,
    /// Human-readable reasons for the decision.
    pub evidence: Vec<String>,
}

const UK2_MES_MAGIC: &[u8] = b"<< UK2 TEXT Ver1.00 >>";
const AVG32_ARCHIVE_MAGIC: &[u8] = b"PACL";
const AVG32_SCENE_MAGIC: &[u8] = b"TPC32";
const REALLIVE_TOC_ENTRIES: usize = 10_000;
/// Scenario header sizes: RealLive (`0x1d0`) and its AVG2000 predecessor
/// (`0x1cc`).
const REALLIVE_HEADER_SIZES: [u32; 2] = [0x1d0, 0x1cc];

/// Detects the engine of the game installed at `root`.
pub fn detect_game_root(root: impl AsRef<Path>) -> io::Result<GameLayout> {
    let root = game_fs::canonicalize(root.as_ref())?;
    if !game_fs::is_dir(&root) {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!("{} is not a game directory", root.display()),
        ));
    }
    let mut layout = GameLayout {
        gameexe_ini: find_case_insensitive(&root, "Gameexe.ini"),
        gameexe_dat: find_case_insensitive(&root, "Gameexe.dat"),
        scene_pck: find_case_insensitive(&root, "Scene.pck"),
        seen_archive: None,
        seen_directory: None,
        pdt_root: ["PDT", "DAT/PDT"]
            .into_iter()
            .find_map(|path| find_case_insensitive(&root, path))
            .filter(|path| game_fs::is_dir(path)),
        uk2_config: find_case_insensitive(&root, "UK2.CFG"),
        evidence: Vec::new(),
        kind: EngineKind::Unknown,
        root,
    };
    layout.kind = classify(&mut layout);
    Ok(layout)
}

/// Shorthand for `detect_game_root(root)?.kind`.
pub fn detect(root: impl AsRef<Path>) -> io::Result<EngineKind> {
    Ok(detect_game_root(root)?.kind)
}

fn classify(layout: &mut GameLayout) -> EngineKind {
    if let Some(path) = &layout.scene_pck {
        layout.evidence.push(format!("{} present", file_name(path)));
    }
    if let Some(path) = &layout.gameexe_dat {
        layout.evidence.push(format!("{} present", file_name(path)));
    }
    if !layout.evidence.is_empty() {
        return EngineKind::Siglus;
    }

    if let Some(path) = &layout.uk2_config {
        layout.evidence.push(format!("{} present", file_name(path)));
        return EngineKind::Uk2;
    }
    if let Some(path) = find_file(&layout.root, |name, head| {
        name.ends_with(".MES") && head.starts_with(UK2_MES_MAGIC)
    }) {
        layout
            .evidence
            .push(format!("{} has the UK2 MES header", file_name(&path)));
        return EngineKind::Uk2;
    }

    let Some(gameexe) = layout.gameexe_ini.clone() else {
        layout
            .evidence
            .push("no Scene.pck, Gameexe.dat, UK2.CFG or Gameexe.ini".to_owned());
        return EngineKind::Unknown;
    };
    layout
        .evidence
        .push(format!("{} present", file_name(&gameexe)));

    for (directory, file) in scenario_locations(&layout.root, &gameexe) {
        if let Some(archive) = find_case_insensitive(&directory, &file) {
            let kind = classify_seen_archive(&archive);
            if kind != EngineKind::Unknown {
                layout.evidence.push(format!(
                    "{} is a {} scenario archive",
                    archive.display(),
                    kind
                ));
                layout.seen_archive = Some(archive);
                return kind;
            }
        }
        if let Some((kind, reason)) = classify_loose_scenes(&directory) {
            layout.evidence.push(reason);
            layout.seen_directory = Some(directory);
            return kind;
        }
    }
    layout
        .evidence
        .push("no recognisable SEEN.TXT scenarios".to_owned());
    EngineKind::Unknown
}

/// Directories and archive names to search for `SEEN.TXT`, most specific
/// first.
fn scenario_locations(root: &Path, gameexe: &Path) -> Vec<(PathBuf, String)> {
    let mut locations = Vec::new();
    if let Some((folder, file)) = game_fs::read(gameexe)
        .ok()
        .and_then(|bytes| foldname(&bytes))
    {
        if let Some(directory) = find_case_insensitive(root, &folder).filter(|p| game_fs::is_dir(p))
        {
            locations.push((directory, file.clone()));
        }
        locations.push((root.to_path_buf(), file));
    }
    for directory in ["DAT", ""] {
        let directory = if directory.is_empty() {
            Some(root.to_path_buf())
        } else {
            find_case_insensitive(root, directory).filter(|p| game_fs::is_dir(p))
        };
        if let Some(directory) = directory {
            locations.push((directory, "SEEN.TXT".to_owned()));
        }
    }
    let mut seen = Vec::new();
    locations.retain(|location| {
        let fresh = !seen.contains(location);
        seen.push(location.clone());
        fresh
    });
    locations
}

/// `#FOLDNAME.TXT = "DAT" = 0 : "SEEN.TXT"` → `("DAT", "SEEN.TXT")`.
fn foldname(gameexe: &[u8]) -> Option<(String, String)> {
    for line in gameexe.split(|&b| b == b'\n') {
        let line = trim_ascii(line);
        let Some(rest) = strip_prefix_ignore_case(line, b"#FOLDNAME.TXT") else {
            continue;
        };
        let quoted: Vec<String> = rest
            .split(|&b| b == b'"')
            .skip(1)
            .step_by(2)
            .map(|part| String::from_utf8_lossy(part).into_owned())
            .collect();
        let folder = quoted.first().cloned().unwrap_or_default();
        let file = quoted
            .get(1)
            .filter(|file| !file.is_empty())
            .cloned()
            .unwrap_or_else(|| "SEEN.TXT".to_owned());
        return Some((folder, file));
    }
    None
}

fn classify_seen_archive(path: &Path) -> EngineKind {
    let Ok(data) = game_fs::read(path) else {
        return EngineKind::Unknown;
    };
    if data.starts_with(AVG32_ARCHIVE_MAGIC) {
        return EngineKind::Avg32;
    }
    if data.len() >= REALLIVE_TOC_ENTRIES * 8 {
        let mut valid = 0;
        for index in 0..REALLIVE_TOC_ENTRIES {
            let at = index * 8;
            let offset = u32_at(&data, at) as usize;
            let length = u32_at(&data, at + 4) as usize;
            if offset == 0 || length < 4 || offset.saturating_add(length) > data.len() {
                continue;
            }
            if !REALLIVE_HEADER_SIZES.contains(&u32_at(&data, offset)) {
                return EngineKind::Unknown;
            }
            valid += 1;
            if valid >= 4 {
                break;
            }
        }
        if valid > 0 {
            return EngineKind::RealLive;
        }
    }
    EngineKind::Unknown
}

/// Classifies loose `SEEN*.TXT` files by content, then by name width
/// (AVG32 uses `SEEN###.TXT`, RealLive `SEEN####.TXT`).
fn classify_loose_scenes(directory: &Path) -> Option<(EngineKind, String)> {
    let mut by_name = None;
    for entry in game_fs::read_dir(directory).ok()?.flatten() {
        let Some(name) = entry.file_name().to_str().map(str::to_ascii_uppercase) else {
            continue;
        };
        let digits = name
            .strip_prefix("SEEN")
            .and_then(|rest| rest.strip_suffix(".TXT"))
            .filter(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()));
        let Some(digits) = digits else {
            continue;
        };
        let head = game_fs::read_head(entry.path(), 8);
        if head.starts_with(AVG32_SCENE_MAGIC) {
            return Some((EngineKind::Avg32, format!("{name} is a TPC32 scene")));
        }
        if head.len() >= 4 && REALLIVE_HEADER_SIZES.contains(&u32_at(&head, 0)) {
            return Some((
                EngineKind::RealLive,
                format!("{name} has a RealLive scenario header"),
            ));
        }
        by_name = by_name.or(match digits.len() {
            3 => Some((
                EngineKind::Avg32,
                format!("{name} is named like an AVG32 scene"),
            )),
            4 => Some((
                EngineKind::RealLive,
                format!("{name} is named like a RealLive scenario"),
            )),
            _ => None,
        });
    }
    by_name
}

fn find_file(directory: &Path, accept: impl Fn(&str, &[u8]) -> bool) -> Option<PathBuf> {
    let mut entries: Vec<_> = game_fs::read_dir(directory).ok()?.flatten().collect();
    entries.sort_by_key(|entry| entry.file_name());
    entries.into_iter().find_map(|entry| {
        let name = entry.file_name().to_str()?.to_ascii_uppercase();
        let path = entry.path();
        entry
            .is_file()
            .then(|| game_fs::read_head(&path, 32))
            .filter(|head| accept(&name, head))
            .map(|_| path)
    })
}

/// Resolves a relative path one component at a time, ignoring ASCII case.
pub fn find_case_insensitive(directory: &Path, wanted: impl AsRef<Path>) -> Option<PathBuf> {
    let mut current = directory.to_path_buf();
    for component in wanted.as_ref().components() {
        let wanted = component.as_os_str().to_str()?;
        current = game_fs::read_dir(&current)
            .ok()?
            .flatten()
            .find_map(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .filter(|name| name.eq_ignore_ascii_case(wanted))
                    .map(|_| entry.path())
            })?;
    }
    Some(current)
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

fn u32_at(data: &[u8], at: usize) -> u32 {
    data.get(at..at + 4)
        .map(|bytes| u32::from_le_bytes(bytes.try_into().expect("4 bytes")))
        .unwrap_or(0)
}

fn trim_ascii(bytes: &[u8]) -> &[u8] {
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

fn strip_prefix_ignore_case<'a>(bytes: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    (bytes.len() >= prefix.len() && bytes[..prefix.len()].eq_ignore_ascii_case(prefix))
        .then(|| &bytes[prefix.len()..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "engine-detect-{name}-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn write(&self, name: &str, bytes: &[u8]) {
            let path = self.0.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn reallive_seen() -> Vec<u8> {
        let mut data = vec![0u8; REALLIVE_TOC_ENTRIES * 8];
        let offset = data.len() as u32;
        let mut scenario = 0x1d0u32.to_le_bytes().to_vec();
        scenario.extend(10002u32.to_le_bytes());
        scenario.resize(0x1d0, 0);
        data[8..12].copy_from_slice(&offset.to_le_bytes());
        data[12..16].copy_from_slice(&(scenario.len() as u32).to_le_bytes());
        data.extend(scenario);
        data
    }

    #[test]
    fn detects_siglus() {
        let dir = TempDir::new("siglus");
        dir.write("Scene.pck", b"\0");
        dir.write("Gameexe.dat", b"\0");
        let layout = detect_game_root(&dir.0).unwrap();
        assert_eq!(layout.kind, EngineKind::Siglus);
        assert!(layout.scene_pck.is_some());
    }

    #[test]
    fn detects_uk2_by_config_or_mes_header() {
        let dir = TempDir::new("uk2cfg");
        dir.write("uk2.cfg", b"START = start.mes1\n");
        assert_eq!(detect(&dir.0).unwrap(), EngineKind::Uk2);

        let dir = TempDir::new("uk2mes");
        let mut mes = UK2_MES_MAGIC.to_vec();
        mes.push(0);
        dir.write("START.MES", &mes);
        assert_eq!(detect(&dir.0).unwrap(), EngineKind::Uk2);
    }

    #[test]
    fn detects_avg32_archive_and_loose_scenes() {
        let dir = TempDir::new("avg32pacl");
        dir.write("GAMEEXE.INI", b"#CAPTION=\"x\"\n");
        dir.write("DAT/SEEN.TXT", b"PACL\0\0\0\0");
        let layout = detect_game_root(&dir.0).unwrap();
        assert_eq!(layout.kind, EngineKind::Avg32);
        assert!(layout.seen_archive.is_some());

        let dir = TempDir::new("avg32loose");
        dir.write("Gameexe.ini", b"");
        dir.write("SEEN001.TXT", b"TPC32\0\0\0");
        assert_eq!(detect(&dir.0).unwrap(), EngineKind::Avg32);
    }

    #[test]
    fn detects_reallive_through_foldname() {
        let dir = TempDir::new("reallive");
        dir.write(
            "Gameexe.ini",
            b"#REGNAME = \"x\"\r\n#FOLDNAME.TXT = \"SCN\" = 0 : \"SEEN.TXT\"\r\n",
        );
        dir.write("SCN/SEEN.TXT", &reallive_seen());
        let layout = detect_game_root(&dir.0).unwrap();
        assert_eq!(layout.kind, EngineKind::RealLive);
        assert!(layout.seen_archive.unwrap().ends_with("SCN/SEEN.TXT"));
    }

    #[test]
    fn detects_reallive_loose_scenarios() {
        let dir = TempDir::new("reallive-loose");
        dir.write("Gameexe.ini", b"");
        dir.write("SEEN0001.TXT", b"compressed?");
        assert_eq!(detect(&dir.0).unwrap(), EngineKind::RealLive);
    }

    #[test]
    fn unknown_without_markers() {
        let dir = TempDir::new("unknown");
        dir.write("readme.txt", b"hello");
        assert_eq!(detect(&dir.0).unwrap(), EngineKind::Unknown);
        let dir = TempDir::new("unknown-ini");
        dir.write("Gameexe.ini", b"");
        assert_eq!(detect(&dir.0).unwrap(), EngineKind::Unknown);
    }

    #[test]
    fn parses_foldname_defaults() {
        assert_eq!(
            foldname(b"#foldname.txt = \"DAT\" = 0 : \"\"\n"),
            Some(("DAT".to_owned(), "SEEN.TXT".to_owned()))
        );
        assert_eq!(foldname(b"#REGNAME = \"x\"\n"), None);
    }
}
