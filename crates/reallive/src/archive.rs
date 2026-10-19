//! `SEEN.TXT`: the scenario archive.
//!
//! The file starts with a table of 10 000 `(offset, length)` pairs, one per
//! scenario number; a zero offset marks an absent scenario. Loose
//! `SEENnnnn.TXT` files next to the archive override its entries (patches
//! and translations ship them this way).

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::{Context, Result, anyhow};

use crate::compression::{self, Xor2Key};
use crate::nls::Nls;
use crate::scenario::{Header, Scenario};

const TOC_ENTRIES: usize = 10_000;

#[derive(Debug, Clone)]
enum Source {
    Packed { offset: usize, length: usize },
    Loose(PathBuf),
}

/// Where the second-level key came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeySource {
    NotNeeded,
    Known,
    Derived,
    Missing,
}

#[derive(Debug)]
pub struct Archive {
    path: PathBuf,
    data: Vec<u8>,
    entries: BTreeMap<i32, Source>,
    cache: RefCell<HashMap<i32, Rc<Scenario>>>,
    xor2: Option<Xor2Key>,
    pub key_source: KeySource,
    pub nls: Nls,
}

impl Archive {
    /// Opens `SEEN.TXT` (it may be missing when only loose files exist).
    pub fn open(path: &Path, regname: &str, nls: Nls) -> Result<Self> {
        let data = if game_fs::is_file(path) {
            game_fs::read(path).with_context(|| format!("failed to read {}", path.display()))?
        } else {
            Vec::new()
        };
        let mut entries = BTreeMap::new();
        if data.len() >= TOC_ENTRIES * 8 {
            for index in 0..TOC_ENTRIES {
                let at = index * 8;
                let offset = u32::from_le_bytes(data[at..at + 4].try_into().expect("4")) as usize;
                let length =
                    u32::from_le_bytes(data[at + 4..at + 8].try_into().expect("4")) as usize;
                if offset != 0 && offset + length <= data.len() {
                    entries.insert(index as i32, Source::Packed { offset, length });
                }
            }
        }
        if let Some(directory) = path.parent() {
            if let Ok(listing) = game_fs::read_dir(directory) {
                for entry in listing.flatten() {
                    let name = entry.file_name().to_string_lossy().to_ascii_uppercase();
                    if name.len() == 12
                        && name.starts_with("SEEN")
                        && name.ends_with(".TXT")
                        && name[4..8].bytes().all(|b| b.is_ascii_digit())
                    {
                        let number: i32 = name[4..8].parse().expect("digits");
                        entries.insert(number, Source::Loose(entry.path()));
                    }
                }
            }
        }
        if entries.is_empty() {
            anyhow::bail!("reallive: no scenarios found at {}", path.display());
        }
        let mut archive = Self {
            path: path.to_path_buf(),
            data,
            entries,
            cache: RefCell::new(HashMap::new()),
            xor2: None,
            key_source: KeySource::NotNeeded,
            nls,
        };
        archive.select_xor2_key(regname);
        Ok(archive)
    }

    fn select_xor2_key(&mut self, regname: &str) {
        let needs = self.entries.keys().take(8).any(|&number| {
            self.raw(number)
                .ok()
                .and_then(|data| Header::parse(&data).ok())
                .is_some_and(|header| header.uses_xor2)
        });
        if !needs {
            return;
        }
        if let Some(key) = compression::known_xor2_key(regname) {
            self.xor2 = Some(key);
            self.key_source = KeySource::Known;
            return;
        }
        let codes: Vec<Vec<u8>> = self
            .entries
            .keys()
            .filter_map(|&number| {
                let data = self.raw(number).ok()?;
                Header::parse(&data).ok()?.decompress(&data).ok()
            })
            .collect();
        match compression::derive_xor2_key(&codes) {
            Some(key) => {
                self.xor2 = Some(key);
                self.key_source = KeySource::Derived;
            }
            None => self.key_source = KeySource::Missing,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn xor2_key(&self) -> Option<&Xor2Key> {
        self.xor2.as_ref()
    }

    pub fn numbers(&self) -> impl Iterator<Item = i32> + '_ {
        self.entries.keys().copied()
    }

    pub fn contains(&self, number: i32) -> bool {
        self.entries.contains_key(&number)
    }

    pub fn first(&self) -> Option<i32> {
        self.entries.keys().next().copied()
    }

    /// The raw (still compressed) scenario file.
    pub fn raw(&self, number: i32) -> Result<Vec<u8>> {
        match self
            .entries
            .get(&number)
            .ok_or_else(|| anyhow!("reallive: SEEN{number:04} does not exist"))?
        {
            Source::Packed { offset, length } => Ok(self.data[*offset..offset + length].to_vec()),
            Source::Loose(path) => {
                game_fs::read(path).with_context(|| format!("failed to read {}", path.display()))
            }
        }
    }

    pub fn scenario(&self, number: i32) -> Result<Rc<Scenario>> {
        if let Some(scenario) = self.cache.borrow().get(&number) {
            return Ok(scenario.clone());
        }
        let data = self.raw(number)?;
        let scenario = Rc::new(Scenario::parse(
            number,
            &data,
            self.xor2.as_ref(),
            self.nls,
        )?);
        self.cache.borrow_mut().insert(number, scenario.clone());
        Ok(scenario)
    }

    /// The encoding named by RLdev metadata in any scenario.
    pub fn probable_encoding(&self) -> Option<Nls> {
        self.entries.keys().find_map(|&number| {
            Header::parse(&self.raw(number).ok()?)
                .ok()?
                .rldev_encoding
                .filter(|nls| *nls != Nls::Sjis)
        })
    }
}

/// Assembles a `SEEN.TXT` archive (tests and tools).
pub fn build(scenarios: &[(i32, Vec<u8>)]) -> Vec<u8> {
    let mut out = vec![0u8; TOC_ENTRIES * 8];
    for (number, data) in scenarios {
        let offset = out.len() as u32;
        let at = *number as usize * 8;
        out[at..at + 4].copy_from_slice(&offset.to_le_bytes());
        out[at + 4..at + 8].copy_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(data);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The key recovered statistically decodes the installed game exactly
    /// like its known key (so unknown games work without a table entry).
    #[test]
    fn derived_key_matches_known_key() {
        let Some(root) = std::env::var_os("REALLIVE_TEST_GAME") else {
            return;
        };
        let root = Path::new(&root);
        let Some(seen) = game_fs::read_dir(root)
            .ok()
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .find(|path| {
                path.file_name()
                    .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case("seen.txt"))
            })
        else {
            return;
        };
        let ini = game_fs::read_dir(root)
            .unwrap()
            .flatten()
            .map(|entry| entry.path())
            .find(|path| {
                path.file_name()
                    .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case("gameexe.ini"))
            })
            .unwrap();
        let gameexe = crate::Gameexe::open(&ini, Nls::Sjis).unwrap();
        let regname = gameexe.str("REGNAME").unwrap_or("").to_owned();
        let archive = Archive::open(&seen, &regname, Nls::Sjis).unwrap();
        let Some(known) = compression::known_xor2_key(&regname) else {
            return;
        };
        let codes: Vec<Vec<u8>> = archive
            .entries
            .keys()
            .filter_map(|&number| {
                let data = archive.raw(number).ok()?;
                Header::parse(&data).ok()?.decompress(&data).ok()
            })
            .collect();
        let derived = compression::derive_xor2_key(&codes).expect("a key");
        for code in &codes {
            let (mut a, mut b) = (code.clone(), code.clone());
            known.apply(&mut a);
            derived.apply(&mut b);
            assert!(a == b, "derived key decodes differently");
        }
    }
}
