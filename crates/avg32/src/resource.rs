//! AVG32 `#DIRC.*` resource routing.
//!
//! Every file type (`PDT`, `TXT`, `ANM`, `ARD`, `CUR`, `WAV`, and `***` for
//! everything else) names a directory and whether files are loose (`N`) or
//! packed into a PACL archive (`P:"ARCHIVE"`).  Archives are opened once and
//! cached.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::{Context, Result, bail};

use crate::archive::PaclArchive;
use crate::ini::Ini;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceRoute {
    pub directory: PathBuf,
    pub archive: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Avg32Resources {
    root: PathBuf,
    routes: BTreeMap<String, ResourceRoute>,
    archives: RefCell<BTreeMap<PathBuf, Rc<PaclArchive>>>,
}

impl Avg32Resources {
    pub fn from_ini(root: impl AsRef<Path>, ini: &Ini) -> Self {
        let root = root.as_ref().to_path_buf();
        let routes = ini
            .directories
            .iter()
            .filter(|(_, directory)| !directory.directory.is_empty())
            .map(|(kind, directory)| {
                (
                    kind.clone(),
                    ResourceRoute {
                        directory: resolve_dir(&root, &directory.directory),
                        archive: (directory.mode == 'P' && !directory.archive.is_empty())
                            .then(|| PathBuf::from(&directory.archive)),
                    },
                )
            })
            .collect();
        Self {
            root,
            routes,
            archives: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The route for a file type; unknown types use `***`.
    pub fn route(&self, kind: &str) -> Option<&ResourceRoute> {
        let kind = kind.trim_start_matches('.').to_ascii_uppercase();
        self.routes.get(&kind).or_else(|| self.routes.get("***"))
    }

    fn archive(&self, path: &Path) -> Result<Rc<PaclArchive>> {
        if let Some(archive) = self.archives.borrow().get(path) {
            return Ok(archive.clone());
        }
        let archive = Rc::new(PaclArchive::from_path(path)?);
        self.archives
            .borrow_mut()
            .insert(path.to_path_buf(), archive.clone());
        Ok(archive)
    }

    /// Reads `name` routed by `kind` (which also supplies the default
    /// extension).  Packed routes fall back to loose files of the same
    /// directory so partially-extracted installs keep working.
    pub fn read(&self, kind: &str, name: &str) -> Result<Vec<u8>> {
        let kind = kind.trim_start_matches('.').to_ascii_uppercase();
        let name = with_extension(name.trim(), &kind);
        let route = self
            .route(&kind)
            .ok_or_else(|| anyhow::anyhow!("avg32: no route for .{kind} files"))?;
        if let Some(archive_name) = &route.archive {
            if let Some(archive_path) = find_case_insensitive(&route.directory, archive_name) {
                let archive = self.archive(&archive_path)?;
                if archive.entry(&name).is_some() {
                    return archive.read(&name).with_context(|| {
                        format!("failed to open {name} in {}", archive_path.display())
                    });
                }
            }
        }
        let path = find_case_insensitive(&route.directory, Path::new(&name))
            .or_else(|| find_case_insensitive(&self.root, Path::new(&name)))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "avg32: resource {name} was not found in {}",
                    route.directory.display()
                )
            })?;
        game_fs::read(&path).with_context(|| format!("failed to read {}", path.display()))
    }

    pub fn exists(&self, kind: &str, name: &str) -> bool {
        self.read(kind, name).is_ok()
    }

    /// Path of a loose file (for streaming backends).
    pub fn loose_path(&self, kind: &str, name: &str) -> Result<PathBuf> {
        let kind = kind.trim_start_matches('.').to_ascii_uppercase();
        let route = self
            .route(&kind)
            .ok_or_else(|| anyhow::anyhow!("avg32: no route for .{kind} files"))?;
        if route.archive.is_some() {
            bail!("avg32: .{kind} is stored in a PACL archive, not a loose file");
        }
        let name = with_extension(name, &kind);
        find_case_insensitive(&route.directory, Path::new(&name))
            .ok_or_else(|| anyhow::anyhow!("avg32: resource {name} was not found"))
    }
}

fn resolve_dir(root: &Path, directory: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    for part in directory
        .split(['/', '\\', ':'])
        .filter(|part| !part.is_empty())
    {
        path = find_case_insensitive(&path, Path::new(part)).unwrap_or_else(|| path.join(part));
    }
    path
}

pub fn with_extension(name: &str, extension: &str) -> String {
    if extension == "***" || Path::new(name).extension().is_some() {
        name.to_owned()
    } else {
        format!("{name}.{extension}")
    }
}

pub fn find_case_insensitive(directory: &Path, wanted: &Path) -> Option<PathBuf> {
    let mut current = directory.to_path_buf();
    for component in wanted.components() {
        let component = component.as_os_str().to_str()?;
        current = find_component(&current, component)?;
    }
    Some(current)
}

/// One directory entry matched case-insensitively. A name that is not
/// found is retried with its Shift-JIS reading (see [`crate::nls`]).
pub fn find_component(directory: &Path, wanted: &str) -> Option<PathBuf> {
    let entries: Vec<_> = game_fs::read_dir(directory).ok()?.flatten().collect();
    let find = |wanted: &str| {
        entries.iter().find_map(|entry| {
            entry
                .file_name()
                .to_str()
                .filter(|name| name.eq_ignore_ascii_case(wanted))
                .map(|_| entry.path())
        })
    };
    find(wanted).or_else(|| find(&crate::nls::sjis_fallback(wanted)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_follow_the_ini_and_fall_back_to_the_other_route() {
        let ini = Ini::parse(
            "#DIRC.PDT=\"PDT\" =N:\"ALLPDT.PDL\"\n#DIRC.TXT=\"DAT\" =P:\"SEEN.TXT\"\n#DIRC.***=\"DAT\" =N\n",
            None,
        );
        let resources = Avg32Resources::from_ini("/game", &ini);
        assert_eq!(resources.route("PDT").unwrap().archive, None);
        assert_eq!(
            resources.route("TXT").unwrap().archive,
            Some(PathBuf::from("SEEN.TXT"))
        );
        assert_eq!(
            resources.route("CGM").unwrap().directory,
            PathBuf::from("/game/DAT")
        );
    }

    #[test]
    fn keeps_an_explicit_extension() {
        assert_eq!(with_extension("seen002", "TXT"), "seen002.TXT");
        assert_eq!(with_extension("seen002.txt", "TXT"), "seen002.txt");
        assert_eq!(with_extension("MODE.CGM", "***"), "MODE.CGM");
    }
}
