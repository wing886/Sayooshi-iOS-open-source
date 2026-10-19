//! Runtime support for voice patches in the style of *vair for win*
//! (AIR voiced with the Dreamcast release's `VOICE??.AFS` archives).
//!
//! The patch ships `voicepat.txt`, one line per voiced message:
//! `GG\IIII <tab> SEENnnn.TXT <tab> offset`, where `GG` is the decimal AFS
//! archive number, `IIII` the hexadecimal entry, and `offset` the file
//! offset of the position word that follows a `0xFF` text opcode.  The
//! original tool rewrote `SEEN.TXT`; here the table is applied while the
//! scenario runs, leaving the game files untouched.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::game::Avg32Game;

#[derive(Debug, Clone, Default)]
pub struct VoicePatch {
    voices: HashMap<(i32, u32), i32>,
}

fn parse(text: &str) -> Vec<(i32, i32, u32)> {
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let voice = fields.next()?;
            let scene = fields.next()?;
            let offset = fields.next()?.parse().ok()?;
            let (group, entry) = voice.split_once(['\\', '¥'])?;
            let group: i32 = group.parse().ok()?;
            let entry = i32::from_str_radix(entry, 16).ok()?;
            let seen = scene
                .to_ascii_uppercase()
                .strip_prefix("SEEN")?
                .strip_suffix(".TXT")?
                .parse()
                .ok()?;
            Some(((group << 16) | entry, seen, offset))
        })
        .collect()
}

impl VoicePatch {
    /// Finds the best-matching `voicepat*.txt` next to or inside the game.
    pub fn discover(game: &Avg32Game) -> Option<Self> {
        let root = &game.layout.root;
        let mut directories: Vec<PathBuf> = vec![root.clone(), root.join("DAT"), root.join("dat")];
        if let Some(parent) = root.parent() {
            directories.push(parent.join("vair_for_win_mixer"));
            directories.push(parent.to_path_buf());
        }
        let mut scenes: HashMap<i32, Option<Vec<u8>>> = HashMap::new();
        let mut best: Option<(usize, Vec<(i32, i32, u32)>)> = None;
        for directory in directories {
            let Ok(entries) = game_fs::read_dir(&directory) else {
                continue;
            };
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
                if !(name.starts_with("voicepat") && name.ends_with(".txt")) {
                    continue;
                }
                let Ok(bytes) = game_fs::read(entry.path()) else {
                    continue;
                };
                let lines = parse(&crate::game::decode_text(&bytes));
                let score = Self::score(game, &lines, &mut scenes);
                if best
                    .as_ref()
                    .is_none_or(|(best_score, _)| score > *best_score)
                {
                    best = Some((score, lines));
                }
            }
        }
        let (score, lines) = best?;
        if score == 0 {
            return None;
        }
        Some(Self {
            voices: lines
                .into_iter()
                .map(|(voice, seen, offset)| ((seen, offset), voice))
                .collect(),
        })
    }

    /// How many sampled entries really point just past a text opcode.
    fn score(
        game: &Avg32Game,
        lines: &[(i32, i32, u32)],
        scenes: &mut HashMap<i32, Option<Vec<u8>>>,
    ) -> usize {
        let step = (lines.len() / 256).max(1);
        lines
            .iter()
            .step_by(step)
            .filter(|(_, seen, offset)| {
                let scene = scenes
                    .entry(*seen)
                    .or_insert_with(|| game.read_seen(*seen).ok());
                scene.as_ref().is_some_and(|bytes| {
                    (*offset as usize)
                        .checked_sub(1)
                        .and_then(|at| bytes.get(at))
                        .is_some_and(|byte| matches!(byte, 0xfe | 0xff))
                })
            })
            .count()
    }

    pub fn voice_at(&self, seen: i32, file_offset: u32) -> Option<i32> {
        self.voices.get(&(seen, file_offset)).copied()
    }

    pub fn len(&self) -> usize {
        self.voices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.voices.is_empty()
    }
}

/// Where AFS voice archives may live for a game root.
pub fn afs_directories(root: &Path) -> Vec<PathBuf> {
    let mut directories = vec![root.to_path_buf(), root.join("DAT"), root.join("KOE")];
    if let Some(parent) = root.parent() {
        directories.push(parent.join("vair_for_win_mixer"));
        directories.push(parent.to_path_buf());
    }
    directories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_voicepat_lines() {
        let lines = parse("01\\000a\tSEEN163.TXT\t2011\r\n25\\0001\tseen700.txt\t42\n");
        assert_eq!(
            lines,
            vec![((1 << 16) | 0x0a, 163, 2011), ((25 << 16) | 1, 700, 42)]
        );
    }
}
