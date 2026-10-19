//! Locating game files.
//!
//! `#FOLDNAME.ext = "folder" = archive : "arcname"` maps a file type to a
//! directory under the game root. Names in scripts omit extensions and
//! are case-insensitive; a type may be stored in several formats (music as
//! `nwa`, `ogg`, `wav` or `mp3`, pictures as `g00` or `pdt`), which are
//! tried in order. Names that are not found are retried with their
//! Shift-JIS reading (translations usually keep the original file names).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::gameexe::Gameexe;
use crate::nls::Nls;

/// Resource kinds and the extensions tried for each.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Image,
    Anm,
    Gan,
    Hik,
    Bgm,
    Wav,
    Koe,
    Movie,
    Data,
    Ard,
    Cursor,
}

impl Kind {
    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Kind::Image => &["g00", "pdt", "png", "bmp", "jpg"],
            Kind::Anm => &["anm"],
            Kind::Gan => &["gan"],
            Kind::Hik => &["hik"],
            Kind::Bgm => &["nwa", "ogg", "wav", "mp3", "owp"],
            Kind::Wav => &["wav", "nwa", "ogg", "owp"],
            Kind::Koe => &["koe", "ovk", "nwk", "ogg", "wav", "nwa"],
            Kind::Movie => &["mpg", "mpeg", "avi", "wmv", "ogv", "mp4"],
            Kind::Data => &["dat", "cgm", "tcc", ""],
            Kind::Ard => &["ard"],
            Kind::Cursor => &["pdt", "g00"],
        }
    }

    /// The `#FOLDNAME` key and the conventional folder names.
    fn folders(self) -> (&'static [&'static str], &'static [&'static str]) {
        match self {
            Kind::Image => (&["G00", "PDT"], &["g00", "pdt"]),
            Kind::Anm => (&["ANM"], &["anm"]),
            Kind::Gan => (&["GAN"], &["gan"]),
            Kind::Hik => (&["HIK"], &["hik"]),
            Kind::Bgm => (&["BGM"], &["bgm"]),
            Kind::Wav => (&["WAV"], &["wav"]),
            Kind::Koe => (&["KOE"], &["koe"]),
            Kind::Movie => (&["MOV"], &["mov"]),
            Kind::Data => (&["DAT", "TXT"], &["dat"]),
            Kind::Ard => (&["ARD"], &["ard", "dat"]),
            Kind::Cursor => (&["PDT", "G00"], &["pdt", "g00"]),
        }
    }
}

#[derive(Debug)]
pub struct Resources {
    root: PathBuf,
    nls: Nls,
    /// Folders per kind, from `#FOLDNAME` or conventions.
    folders: HashMap<Kind, Vec<PathBuf>>,
    /// Lower-cased directory listings.
    listings: Mutex<HashMap<PathBuf, HashMap<String, PathBuf>>>,
}

impl Resources {
    pub fn new(root: &Path, gameexe: &Gameexe, nls: Nls) -> Self {
        let mut folders = HashMap::new();
        let all = [
            Kind::Image,
            Kind::Anm,
            Kind::Gan,
            Kind::Hik,
            Kind::Bgm,
            Kind::Wav,
            Kind::Koe,
            Kind::Movie,
            Kind::Data,
            Kind::Ard,
            Kind::Cursor,
        ];
        for kind in all {
            let (keys, defaults) = kind.folders();
            let mut dirs: Vec<PathBuf> = keys
                .iter()
                .filter_map(|key| gameexe.str(&format!("FOLDNAME.{key}")))
                .filter(|folder| !folder.is_empty())
                .map(|folder| root.join(folder.replace('\\', "/")))
                .collect();
            dirs.extend(defaults.iter().map(|folder| root.join(folder)));
            dirs.push(root.to_path_buf());
            dirs.dedup();
            folders.insert(kind, dirs);
        }
        Self {
            root: root.to_path_buf(),
            nls,
            folders,
            listings: Mutex::new(HashMap::new()),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn listing_lookup(&self, directory: &Path, lower_name: &str) -> Option<PathBuf> {
        let mut listings = self.listings.lock().ok()?;
        let listing = listings.entry(directory.to_path_buf()).or_insert_with(|| {
            let mut map = HashMap::new();
            if let Ok(entries) = game_fs::read_dir(directory) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_lowercase();
                    map.insert(name, entry.path());
                }
            }
            // Case-insensitive directory resolution for the folder itself.
            if map.is_empty() {
                if let (Some(parent), Some(leaf)) = (directory.parent(), directory.file_name()) {
                    if let Ok(entries) = game_fs::read_dir(parent) {
                        let leaf = leaf.to_string_lossy().to_lowercase();
                        for entry in entries.flatten() {
                            if entry.file_name().to_string_lossy().to_lowercase() == leaf {
                                if let Ok(inner) = game_fs::read_dir(entry.path()) {
                                    for file in inner.flatten() {
                                        let name =
                                            file.file_name().to_string_lossy().to_lowercase();
                                        map.insert(name, file.path());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            map
        });
        listing.get(lower_name).cloned()
    }

    fn find_in(&self, directory: &Path, name: &str, kind: Kind) -> Option<PathBuf> {
        let lower = name.to_lowercase();
        let has_extension = Path::new(&lower)
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| kind.extensions().contains(&ext) || ext.len() == 3);
        if has_extension {
            if let Some(path) = self.listing_lookup(directory, &lower) {
                return Some(path);
            }
        }
        for extension in kind.extensions() {
            let candidate = if extension.is_empty() {
                lower.clone()
            } else {
                format!("{lower}.{extension}")
            };
            if let Some(path) = self.listing_lookup(directory, &candidate) {
                return Some(path);
            }
        }
        None
    }

    /// Finds a resource by script name.
    pub fn find(&self, kind: Kind, name: &str) -> Option<PathBuf> {
        // Scripts may append `?nnn` to a name ("CGMS12?030"); only the
        // part before it names a file.
        let name = name
            .split('?')
            .next()
            .unwrap_or("")
            .trim()
            .replace('\\', "/");
        if name.is_empty() {
            return None;
        }
        let dirs = self.folders.get(&kind)?;
        for candidate in std::iter::once(name.clone()).chain(self.nls.sjis_fallback(&name)) {
            // Names may include a sub-directory.
            let (sub, leaf) = match candidate.rsplit_once('/') {
                Some((sub, leaf)) => (Some(sub.to_owned()), leaf.to_owned()),
                None => (None, candidate.clone()),
            };
            for dir in dirs {
                let dir = match &sub {
                    Some(sub) => dir.join(sub),
                    None => dir.clone(),
                };
                if let Some(path) = self.find_in(&dir, &leaf, kind) {
                    return Some(path);
                }
            }
        }
        None
    }

    pub fn read(&self, kind: Kind, name: &str) -> Option<Vec<u8>> {
        game_fs::read(self.find(kind, name)?).ok()
    }

    /// A file relative to the game root (case-insensitive).
    pub fn root_file(&self, name: &str) -> Option<PathBuf> {
        // Case-insensitive at every level of a relative path.
        let mut path = self.root.clone();
        for part in name
            .split(['/', '\\'])
            .filter(|p| !p.is_empty() && *p != ".")
        {
            if part == ".." {
                return None;
            }
            path = self.listing_lookup(&path, &part.to_lowercase())?;
        }
        (path != self.root).then_some(path)
    }
}
