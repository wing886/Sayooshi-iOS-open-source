//! UK2 game-directory access.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use encoding_rs::SHIFT_JIS;

use crate::config::Uk2Config;
use crate::dlb::DlbArchive;
use crate::map::Uk2MapLayout;
use crate::mes::MesProgram;
use crate::music::{Uk2MusicFile, Uk2MusicKind};
use crate::pdt::{Pdt34Header, Pdt34Image, decode_pdt34};

#[derive(Debug, Clone)]
pub struct Uk2Game {
    root: PathBuf,
    pub config: Uk2Config,
    files: BTreeMap<String, PathBuf>,
    archives: Vec<DlbArchive>,
}

impl Uk2Game {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root
            .as_ref()
            .canonicalize()
            .with_context(|| format!("invalid UK2 game root {}", root.as_ref().display()))?;
        if !root.is_dir() {
            bail!("{} is not a directory", root.display());
        }
        let mut files = BTreeMap::new();
        for entry in std::fs::read_dir(&root)
            .with_context(|| format!("failed to list {}", root.display()))?
        {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
                continue;
            };
            files.insert(name.to_ascii_uppercase(), entry.path());
        }
        let cfg_path = files
            .get("UK2.CFG")
            .ok_or_else(|| anyhow::anyhow!("{} has no UK2.CFG", root.display()))?;
        let config = Uk2Config::parse_bytes(
            &std::fs::read(cfg_path)
                .with_context(|| format!("failed to read {}", cfg_path.display()))?,
        )?;
        let mut archives = Vec::new();
        for path in files
            .iter()
            .filter(|(name, _)| name.ends_with(".DLB"))
            .map(|(_, path)| path)
        {
            archives.push(DlbArchive::from_path(path)?);
        }
        Ok(Self {
            root,
            config,
            files,
            archives,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn resolve(&self, engine_name: &str) -> Option<&Path> {
        let mapped = map_virtual_extension(engine_name);
        self.files
            .get(&mapped.to_ascii_uppercase())
            .map(PathBuf::as_path)
    }

    pub fn contains_resource(&self, engine_name: &str) -> bool {
        let mapped = map_virtual_extension(engine_name);
        if self.files.contains_key(&mapped.to_ascii_uppercase()) {
            return true;
        }
        self.archives
            .iter()
            .any(|archive| archive.entry(&mapped).is_some())
    }

    pub fn read(&self, engine_name: &str) -> Result<Vec<u8>> {
        if let Some(path) = self.resolve(engine_name) {
            return std::fs::read(path)
                .with_context(|| format!("failed to read {}", path.display()));
        }
        let mapped = map_virtual_extension(engine_name);
        for archive in &self.archives {
            if archive.entry(&mapped).is_some() {
                return Ok(archive.read(&mapped)?.to_vec());
            }
        }
        bail!("uk2: resource {engine_name:?} was not found")
    }

    /// Reads a file of the game directory by its real (case-insensitive)
    /// name, bypassing virtual extensions and archives.
    pub fn read_raw_file(&self, name: &str) -> Result<Vec<u8>> {
        let path = self
            .files
            .get(&name.to_ascii_uppercase())
            .ok_or_else(|| anyhow::anyhow!("uk2: {} has no {name}", self.root.display()))?;
        std::fs::read(path).with_context(|| format!("failed to read {}", path.display()))
    }

    /// Writes a data file (save data, entity lists) into the game directory.
    pub fn write_file(&mut self, engine_name: &str, bytes: &[u8]) -> Result<()> {
        let mapped = map_virtual_extension(engine_name);
        let key = mapped.to_ascii_uppercase();
        let path = match self.files.get(&key) {
            Some(path) => path.clone(),
            None => self.root.join(&key),
        };
        std::fs::write(&path, bytes)
            .with_context(|| format!("failed to write {}", path.display()))?;
        self.files.insert(key, path);
        Ok(())
    }

    pub fn load_mes(&self, engine_name: &str) -> Result<MesProgram> {
        MesProgram::from_bytes(self.read(engine_name)?)
            .with_context(|| format!("failed to parse UK2 MES {engine_name:?}"))
    }

    pub fn load_pdt34(&self, engine_name: &str) -> Result<Pdt34Image> {
        decode_pdt34(&self.read(engine_name)?)
            .with_context(|| format!("failed to decode UK2 PDT {engine_name:?}"))
    }

    pub fn load_map(&self, engine_name: &str) -> Result<Uk2MapLayout> {
        Uk2MapLayout::from_bytes(self.read(engine_name)?)
            .with_context(|| format!("failed to decode UK2 MAP {engine_name:?}"))
    }

    pub fn find_map_for_resource(&self, resource_name: &str) -> Result<Option<Uk2MapLayout>> {
        let wanted = map_virtual_extension(resource_name);
        for (name, path) in &self.files {
            if name.ends_with(".MAP") {
                let map = Uk2MapLayout::from_bytes(
                    std::fs::read(path)
                        .with_context(|| format!("failed to read {}", path.display()))?,
                )
                .with_context(|| format!("failed to decode UK2 MAP {}", path.display()))?;
                if same_resource(&map.base, &wanted) || same_resource(&map.overlay, &wanted) {
                    return Ok(Some(map));
                }
            }
        }
        for archive in &self.archives {
            for entry in archive
                .entries()
                .iter()
                .filter(|entry| entry.name.to_ascii_uppercase().ends_with(".MAP"))
            {
                let map = Uk2MapLayout::from_bytes(archive.read(&entry.name)?.to_vec())
                    .with_context(|| format!("failed to decode DLB MAP {}", entry.name))?;
                if same_resource(&map.base, &wanted) || same_resource(&map.overlay, &wanted) {
                    return Ok(Some(map));
                }
            }
        }
        Ok(None)
    }

    pub fn preview_map(&self, map: &Uk2MapLayout) -> Result<Pdt34Image> {
        let mut base = self
            .read_optional(&map.base)
            .with_context(|| format!("failed to read base PDT {}", map.base))?
            .map(|bytes| decode_pdt34(&bytes))
            .transpose()
            .with_context(|| format!("failed to decode base PDT {}", map.base))?
            .unwrap_or_else(blank_pdt34);
        if let Some(bytes) = self.read_optional(&map.overlay)? {
            let mut overlay = decode_pdt34(&bytes)
                .with_context(|| format!("failed to decode overlay PDT {}", map.overlay))?;
            for (pixel_index, pixel) in overlay.rgba.chunks_exact_mut(4).enumerate() {
                let packed = overlay.indexed[pixel_index / 2];
                let colour = if pixel_index & 1 == 0 {
                    packed >> 4
                } else {
                    packed & 0x0f
                };
                if colour == 0 {
                    pixel[3] = 0;
                } else {
                    let destination_packed = &mut base.indexed[pixel_index / 2];
                    if pixel_index & 1 == 0 {
                        *destination_packed = (*destination_packed & 0x0f) | (colour << 4);
                    } else {
                        *destination_packed = (*destination_packed & 0xf0) | colour;
                    }
                }
            }
            alpha_blend_rgba(&mut base.rgba, &overlay.rgba);
        }
        Ok(base)
    }

    /// Draws the visible 40×25 cell viewport from the MAP's 16×16 chip atlas.
    /// The low nine bits of each map word select a chip; the remaining bits
    /// hold collision, layer, and dynamic state used by the actor renderer.
    pub fn render_map(
        &self,
        map: &Uk2MapLayout,
        scroll_x: u16,
        scroll_y: u16,
    ) -> Result<Pdt34Image> {
        const CHIP_SIZE: usize = 16;
        const SCREEN_WIDTH: usize = crate::pdt::PDT34_WIDTH as usize;
        const SCREEN_HEIGHT: usize = crate::pdt::PDT34_HEIGHT as usize;
        let bytes = self.read(&map.base)?;
        let atlas_header = Pdt34Header::parse(&bytes)?;
        let atlas = atlas_header.decode_image(&bytes)?;
        let atlas_left = usize::from(atlas_header.rect.left) * 8;
        let atlas_top = usize::from(atlas_header.rect.top);
        let atlas_width = (usize::from(atlas_header.rect.right - atlas_header.rect.left) + 1) * 8;
        let atlas_height = usize::from(atlas_header.rect.bottom - atlas_header.rect.top) + 1;
        if atlas_width % CHIP_SIZE != 0 || atlas_height % CHIP_SIZE != 0 {
            bail!(
                "uk2: MAP chip atlas {:?} is not aligned to 16-pixel cells",
                map.base
            );
        }
        let atlas_columns = atlas_width / CHIP_SIZE;
        let atlas_count = atlas_columns * (atlas_height / CHIP_SIZE);
        let mut frame = blank_pdt34();
        for screen_y in 0..SCREEN_HEIGHT / CHIP_SIZE {
            let map_y = usize::from(scroll_y) + screen_y;
            if map_y >= usize::from(map.height) {
                break;
            }
            for screen_x in 0..SCREEN_WIDTH / CHIP_SIZE {
                let map_x = usize::from(scroll_x) + screen_x;
                if map_x >= usize::from(map.width) {
                    break;
                }
                let word = map.tiles()[map_y * usize::from(map.width) + map_x];
                let chip = usize::from(word & 0x01ff);
                if chip >= atlas_count {
                    bail!(
                        "uk2: MAP chip {chip} exceeds {}-cell atlas {:?}",
                        atlas_count,
                        map.base
                    );
                }
                let source_x = atlas_left + chip % atlas_columns * CHIP_SIZE;
                let source_y = atlas_top + chip / atlas_columns * CHIP_SIZE;
                let target_x = screen_x * CHIP_SIZE;
                let target_y = screen_y * CHIP_SIZE;
                for row in 0..CHIP_SIZE {
                    let source_pixel = (source_y + row) * SCREEN_WIDTH + source_x;
                    let target_pixel = (target_y + row) * SCREEN_WIDTH + target_x;
                    let indexed_count = CHIP_SIZE / 2;
                    frame.indexed[target_pixel / 2..target_pixel / 2 + indexed_count]
                        .copy_from_slice(
                            &atlas.indexed[source_pixel / 2..source_pixel / 2 + indexed_count],
                        );
                    let rgba_count = CHIP_SIZE * 4;
                    frame.rgba[target_pixel * 4..target_pixel * 4 + rgba_count].copy_from_slice(
                        &atlas.rgba[source_pixel * 4..source_pixel * 4 + rgba_count],
                    );
                }
            }
        }
        Ok(frame)
    }

    pub fn load_mmd(&self, engine_name: &str) -> Result<Uk2MusicFile> {
        Uk2MusicFile::from_bytes(Uk2MusicKind::Mmd, self.read(engine_name)?)
            .with_context(|| format!("failed to load UK2 MMD {engine_name:?}"))
    }

    fn read_optional(&self, engine_name: &str) -> Result<Option<Vec<u8>>> {
        if self.contains_resource(engine_name) {
            self.read(engine_name).map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn load_mmm(&self, engine_name: &str) -> Result<Uk2MusicFile> {
        Uk2MusicFile::from_bytes(Uk2MusicKind::Mmm, self.read(engine_name)?)
            .with_context(|| format!("failed to load UK2 MMM {engine_name:?}"))
    }

    /// Loads an M0 resource using the media extensions selected by UK2.CFG.
    /// An explicit MMD/MMM extension wins; extensionless names try configured
    /// FM first, then MIDI, matching the reference configuration's priority.
    pub fn load_configured_music(&self, engine_name: &str) -> Result<Uk2MusicFile> {
        for (candidate, kind) in configured_music_candidates(&self.config, engine_name) {
            if !self.contains_resource(&candidate) {
                continue;
            }
            let bytes = self.read(&candidate)?;
            return Uk2MusicFile::from_bytes(kind, bytes)
                .with_context(|| format!("failed to parse UK2 music {candidate:?}"));
        }
        bail!("uk2: music resource {engine_name:?} was not found using UK2.CFG extensions")
    }

    /// Loads a MES using the raw Shift-JIS resource name supplied by UK2
    /// bytecode. This is the direct bridge needed by [`crate::vm::Uk2Host`].
    pub fn load_mes_engine_name(&self, engine_name: &[u8]) -> Result<MesProgram> {
        let engine_name = Self::decode_engine_name(engine_name)?;
        self.load_mes(&engine_name)
    }

    pub fn start_mes(&self) -> Result<MesProgram> {
        self.load_mes(&self.config.start)
    }

    pub fn decode_engine_name(bytes: &[u8]) -> Result<String> {
        let (name, _, had_errors) = SHIFT_JIS.decode(bytes);
        if had_errors {
            bail!("uk2: resource name contains invalid Shift-JIS");
        }
        Ok(name.into_owned())
    }
}

fn same_resource(a: &str, b: &str) -> bool {
    map_virtual_extension(a).eq_ignore_ascii_case(&map_virtual_extension(b))
}

fn configured_music_candidates(
    config: &Uk2Config,
    engine_name: &str,
) -> Vec<(String, Uk2MusicKind)> {
    let normalized = map_virtual_extension(engine_name);
    let extension = Path::new(&normalized)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default();
    let explicit_kind = if extension.eq_ignore_ascii_case("mmd")
        || extension.eq_ignore_ascii_case(&config.midi_ext)
    {
        Some(Uk2MusicKind::Mmd)
    } else if extension.eq_ignore_ascii_case("mmm")
        || extension.eq_ignore_ascii_case(&config.fm_ext)
    {
        Some(Uk2MusicKind::Mmm)
    } else {
        None
    };
    if let Some(kind) = explicit_kind {
        return vec![(normalized, kind)];
    }
    let mut fm = Path::new(&normalized).to_path_buf();
    fm.set_extension(&config.fm_ext);
    let mut midi = Path::new(&normalized).to_path_buf();
    midi.set_extension(&config.midi_ext);
    vec![
        (fm.to_string_lossy().into_owned(), Uk2MusicKind::Mmm),
        (midi.to_string_lossy().into_owned(), Uk2MusicKind::Mmd),
    ]
}

fn blank_pdt34() -> Pdt34Image {
    Pdt34Image {
        width: crate::pdt::PDT34_WIDTH,
        height: crate::pdt::PDT34_HEIGHT,
        indexed: vec![0; (crate::pdt::PDT34_WIDTH * crate::pdt::PDT34_HEIGHT / 2) as usize],
        rgba: vec![0; (crate::pdt::PDT34_WIDTH * crate::pdt::PDT34_HEIGHT * 4) as usize],
    }
}

fn alpha_blend_rgba(dst: &mut [u8], src: &[u8]) {
    for (dst_px, src_px) in dst.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
        let alpha = u16::from(src_px[3]);
        let inv = 255u16.saturating_sub(alpha);
        for i in 0..3 {
            dst_px[i] =
                (((u16::from(dst_px[i]) * inv) + (u16::from(src_px[i]) * alpha)) / 255) as u8;
        }
        dst_px[3] = 255;
    }
}

pub fn map_virtual_extension(name: &str) -> String {
    let path = Path::new(name);
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return name.to_owned();
    };
    if extension.len() > 1 && extension.ends_with('1') {
        let real_extension = &extension[..extension.len() - 1];
        let mut mapped = path.to_path_buf();
        mapped.set_extension(real_extension);
        return mapped.to_string_lossy().into_owned();
    }
    name.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_uk2_virtual_extensions() {
        assert_eq!(map_virtual_extension("start.mes1"), "start.mes");
        assert_eq!(map_virtual_extension("game.pdt1"), "game.pdt");
        assert_eq!(map_virtual_extension("map01.map1"), "map01.map");
        assert_eq!(map_virtual_extension("fight.mmd"), "fight.mmd");
    }

    #[test]
    fn resolves_music_candidates_from_config_and_virtual_extensions() {
        let config =
            Uk2Config::parse_text("MIDI_EXT=MMD\nFM_EXT=MMM\nLIB=OFF\nSTART=start.mes1\n").unwrap();
        assert_eq!(
            configured_music_candidates(&config, "title.mmm1"),
            vec![("title.mmm".to_owned(), Uk2MusicKind::Mmm)]
        );
        assert_eq!(
            configured_music_candidates(&config, "battle.mmd1"),
            vec![("battle.mmd".to_owned(), Uk2MusicKind::Mmd)]
        );
        assert_eq!(
            configured_music_candidates(&config, "battle"),
            vec![
                ("battle.MMM".to_owned(), Uk2MusicKind::Mmm),
                ("battle.MMD".to_owned(), Uk2MusicKind::Mmd),
            ]
        );
    }
}
