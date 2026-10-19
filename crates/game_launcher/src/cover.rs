//! Cover art for the game library.
//!
//! Lookup order:
//! 1. conventional cover files in the game folder (`cover.png`, ...);
//! 2. an `.ico` file in the game folder;
//! 3. the icon resource of the game's Windows executable;
//! 4. an engine-specific picture (UK2: the first opening image that is not
//!    mostly black).
//!
//! Icons are small, so they are presented on a generated card: the icon
//! scaled up by an integer factor on a gradient taken from its colours.

use std::io::Cursor;
use std::path::{Path, PathBuf};

use engine_detect::EngineKind;
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverKind {
    /// A picture meant to be shown full-bleed.
    Image,
    /// A card generated around the game's icon.
    Icon,
}

impl CoverKind {
    pub fn id(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Icon => "icon",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cover {
    pub png: Vec<u8>,
    pub kind: CoverKind,
    /// Human-readable origin (for diagnostics).
    pub source: String,
}

const COVER_FILES: &[&str] = &[
    "cover.png",
    "cover.jpg",
    "cover.jpeg",
    "thumbnail.png",
    "icon.png",
];

const CARD_WIDTH: u32 = 480;
const CARD_HEIGHT: u32 = 270;

pub fn resolve(root: &Path, engine: EngineKind) -> Option<Cover> {
    let files = list_files(root);
    for wanted in COVER_FILES {
        if let Some(path) = files.iter().find(|path| name_is(path, wanted))
            && let Some(image) = game_fs::read(path)
                .ok()
                .and_then(|bytes| image::load_from_memory(&bytes).ok())
        {
            return Some(cover(image, CoverKind::Image, path));
        }
    }
    let mut icons: Vec<&PathBuf> = files.iter().filter(|path| has_ext(path, "ico")).collect();
    icons.sort_by_key(|path| icon_rank(path));
    for path in icons {
        if let Some(icon) = game_fs::read(path)
            .ok()
            .and_then(|bytes| decode_ico(&bytes))
        {
            return Some(icon_card(icon, path));
        }
    }
    let mut exes: Vec<&PathBuf> = files
        .iter()
        .filter(|path| has_ext(path, "exe") && !is_helper_exe(path))
        .collect();
    exes.sort_by_key(|path| exe_rank(path, engine));
    for path in exes {
        let icon = game_fs::read(path)
            .ok()
            .and_then(|bytes| exe_icon_ico(&bytes))
            .and_then(|ico| decode_ico(&ico));
        if let Some(icon) = icon {
            return Some(icon_card(icon, path));
        }
    }
    engine_picture(root, engine)
}

fn cover(image: DynamicImage, kind: CoverKind, source: &Path) -> Cover {
    Cover {
        png: encode_png(&image),
        kind,
        source: source.display().to_string(),
    }
}

pub fn encode_png(image: &DynamicImage) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    let _ = image.write_to(&mut out, ImageFormat::Png);
    out.into_inner()
}

fn list_files(root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = game_fs::read_dir(root)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.is_file())
                .map(|entry| entry.path())
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    files
}

fn file_name_lower(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default()
}

fn name_is(path: &Path, wanted: &str) -> bool {
    file_name_lower(path) == wanted
}

fn has_ext(path: &Path, ext: &str) -> bool {
    path.extension()
        .is_some_and(|value| value.to_string_lossy().eq_ignore_ascii_case(ext))
}

fn icon_rank(path: &Path) -> u8 {
    let name = file_name_lower(path);
    if name.contains("icon") || name.contains("game") {
        0
    } else {
        1
    }
}

fn is_helper_exe(path: &Path) -> bool {
    let name = file_name_lower(path);
    [
        "unins", "setup", "install", "config", "update", "patch", "dxsetup", "vcredist",
    ]
    .iter()
    .any(|word| name.contains(word))
}

fn exe_rank(path: &Path, engine: EngineKind) -> u8 {
    let name = file_name_lower(path);
    let engine_names: &[&str] = match engine {
        EngineKind::Siglus => &["siglusengine"],
        EngineKind::RealLive => &["reallive", "kinetic"],
        EngineKind::Avg32 => &["avg32", "avg2000"],
        EngineKind::Uk2 | EngineKind::Unknown => &[],
    };
    if engine_names.iter().any(|word| name.starts_with(word)) {
        1
    } else {
        // Titles often rename the engine after the game; prefer those over
        // generic tools, which `is_helper_exe` already dropped.
        0
    }
}

/// Decodes the largest image of an `.ico` file.
pub fn decode_ico(bytes: &[u8]) -> Option<RgbaImage> {
    if bytes.len() < 6 || bytes[..4] != [0, 0, 1, 0] {
        return None;
    }
    image::load_from_memory_with_format(bytes, ImageFormat::Ico)
        .ok()
        .map(|image| image.to_rgba8())
        .filter(|image| image.width() > 0 && image.height() > 0)
}

/// Rebuilds an `.ico` file from the largest icon group of a PE executable.
pub fn exe_icon_ico(pe: &[u8]) -> Option<Vec<u8>> {
    let resources = PeResources::parse(pe)?;
    const RT_ICON: u32 = 3;
    const RT_GROUP_ICON: u32 = 14;
    let groups = resources.entries_of_type(RT_GROUP_ICON);
    let mut best: Option<(u64, Vec<u8>)> = None;
    for group in groups {
        if group.len() < 6 {
            continue;
        }
        let count = usize::from(u16::from_le_bytes([group[4], group[5]]));
        let mut images = Vec::new();
        let mut area = 0u64;
        for index in 0..count {
            let at = 6 + index * 14;
            let Some(entry) = group.get(at..at + 14) else {
                break;
            };
            let id = u32::from(u16::from_le_bytes([entry[12], entry[13]]));
            let Some(data) = resources.entry(RT_ICON, id) else {
                continue;
            };
            let width = if entry[0] == 0 {
                256
            } else {
                u64::from(entry[0])
            };
            let height = if entry[1] == 0 {
                256
            } else {
                u64::from(entry[1])
            };
            area = area.max(width * height);
            images.push((entry[..12].to_vec(), data));
        }
        if images.is_empty()
            || best
                .as_ref()
                .is_some_and(|(best_area, _)| *best_area >= area)
        {
            continue;
        }
        let mut ico = vec![0, 0, 1, 0];
        ico.extend((images.len() as u16).to_le_bytes());
        let mut offset = 6 + images.len() * 16;
        for (header, data) in &images {
            ico.extend(&header[..8]);
            ico.extend((data.len() as u32).to_le_bytes());
            ico.extend((offset as u32).to_le_bytes());
            offset += data.len();
        }
        for (_, data) in &images {
            ico.extend(data);
        }
        best = Some((area, ico));
    }
    best.map(|(_, ico)| ico)
}

/// The resource section of a PE image, just enough to read icons.
struct PeResources<'a> {
    pe: &'a [u8],
    sections: Vec<(u32, u32, u32)>,
    base: usize,
}

impl<'a> PeResources<'a> {
    fn parse(pe: &'a [u8]) -> Option<Self> {
        let u16_at = |at: usize| pe.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
        let u32_at = |at: usize| {
            pe.get(at..at + 4)
                .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };
        if pe.get(..2)? != b"MZ" {
            return None;
        }
        let nt = u32_at(0x3c)? as usize;
        if pe.get(nt..nt + 4)? != b"PE\0\0" {
            return None;
        }
        let coff = nt + 4;
        let section_count = usize::from(u16_at(coff + 2)?);
        let optional_size = usize::from(u16_at(coff + 16)?);
        let optional = coff + 20;
        let directories = match u16_at(optional)? {
            0x10b => optional + 96,
            0x20b => optional + 112,
            _ => return None,
        };
        let resource_rva = u32_at(directories + 2 * 8)?;
        if resource_rva == 0 {
            return None;
        }
        let table = optional + optional_size;
        let mut sections = Vec::new();
        for index in 0..section_count {
            let at = table + index * 40;
            let virtual_size = u32_at(at + 8)?;
            let virtual_address = u32_at(at + 12)?;
            let raw_size = u32_at(at + 16)?;
            let raw_offset = u32_at(at + 20)?;
            sections.push((virtual_address, virtual_size.max(raw_size), raw_offset));
        }
        let mut resources = Self {
            pe,
            sections,
            base: 0,
        };
        resources.base = resources.rva_to_offset(resource_rva)?;
        Some(resources)
    }

    fn rva_to_offset(&self, rva: u32) -> Option<usize> {
        self.sections
            .iter()
            .find(|(start, size, _)| rva >= *start && rva < start.saturating_add(*size))
            .map(|(start, _, raw)| (rva - start + raw) as usize)
    }

    fn u32_at(&self, at: usize) -> Option<u32> {
        self.pe
            .get(at..at + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// `(id or name offset, is_directory, target offset)` of a directory.
    fn directory(&self, offset: usize) -> Vec<(u32, bool, usize)> {
        let at = self.base + offset;
        let Some(counts) = self.pe.get(at + 12..at + 16) else {
            return Vec::new();
        };
        let count = usize::from(u16::from_le_bytes([counts[0], counts[1]]))
            + usize::from(u16::from_le_bytes([counts[2], counts[3]]));
        (0..count.min(4096))
            .filter_map(|index| {
                let entry = at + 16 + index * 8;
                let name = self.u32_at(entry)?;
                let target = self.u32_at(entry + 4)?;
                Some((
                    name,
                    target & 0x8000_0000 != 0,
                    (target & 0x7fff_ffff) as usize,
                ))
            })
            .collect()
    }

    fn leaf(&self, offset: usize) -> Option<Vec<u8>> {
        let mut offset = offset;
        // Descend through any remaining levels (language) to the data entry.
        for _ in 0..4 {
            let at = self.base + offset;
            let entries = self.directory(offset);
            if entries.is_empty() {
                let rva = self.u32_at(at)?;
                let size = self.u32_at(at + 4)? as usize;
                let start = self.rva_to_offset(rva)?;
                return self.pe.get(start..start + size).map(<[u8]>::to_vec);
            }
            let (_, is_dir, target) = entries[0];
            if !is_dir {
                let at = self.base + target;
                let rva = self.u32_at(at)?;
                let size = self.u32_at(at + 4)? as usize;
                let start = self.rva_to_offset(rva)?;
                return self.pe.get(start..start + size).map(<[u8]>::to_vec);
            }
            offset = target;
        }
        None
    }

    fn entries_of_type(&self, kind: u32) -> Vec<Vec<u8>> {
        let Some((_, _, types)) = self
            .directory(0)
            .into_iter()
            .find(|(id, is_dir, _)| *id == kind && *is_dir)
        else {
            return Vec::new();
        };
        self.directory(types)
            .into_iter()
            .filter_map(
                |(_, is_dir, target)| {
                    if is_dir { self.leaf(target) } else { None }
                },
            )
            .collect()
    }

    fn entry(&self, kind: u32, id: u32) -> Option<Vec<u8>> {
        let (_, _, types) = self
            .directory(0)
            .into_iter()
            .find(|(entry_id, is_dir, _)| *entry_id == kind && *is_dir)?;
        let (_, is_dir, target) = self
            .directory(types)
            .into_iter()
            .find(|(entry_id, _, _)| *entry_id == id)?;
        is_dir.then(|| self.leaf(target)).flatten()
    }
}

/// Places a small icon on a card-sized gradient.
pub fn icon_card(icon: RgbaImage, source: &Path) -> Cover {
    let (sum, count) = icon
        .pixels()
        .fold(([0u64; 3], 0u64), |(mut sum, count), pixel| {
            if pixel[3] < 128 {
                return (sum, count);
            }
            for channel in 0..3 {
                sum[channel] += u64::from(pixel[channel]);
            }
            (sum, count + 1)
        });
    let average = if count == 0 {
        [96.0, 96.0, 110.0]
    } else {
        [
            sum[0] as f32 / count as f32,
            sum[1] as f32 / count as f32,
            sum[2] as f32 / count as f32,
        ]
    };
    let mut card = RgbaImage::new(CARD_WIDTH, CARD_HEIGHT);
    for (x, y, pixel) in card.enumerate_pixels_mut() {
        let t = y as f32 / CARD_HEIGHT as f32;
        let glow = 1.0 - ((x as f32 / CARD_WIDTH as f32) - 0.5).abs() * 0.6;
        let top = 0.55 * glow;
        let bottom = 0.18;
        let shade = top + (bottom - top) * t;
        *pixel = Rgba([
            (average[0] * shade + 12.0) as u8,
            (average[1] * shade + 12.0) as u8,
            (average[2] * shade + 16.0) as u8,
            255,
        ]);
    }
    let scale = (CARD_HEIGHT * 2 / 3 / icon.height().max(1))
        .min(CARD_WIDTH * 2 / 3 / icon.width().max(1))
        .max(1);
    let (w, h) = (icon.width() * scale, icon.height() * scale);
    let scaled = image::imageops::resize(&icon, w, h, image::imageops::FilterType::Nearest);
    let x = i64::from((CARD_WIDTH - w.min(CARD_WIDTH)) / 2);
    let y = i64::from((CARD_HEIGHT - h.min(CARD_HEIGHT)) / 2);
    image::imageops::overlay(&mut card, &scaled, x, y);
    cover(DynamicImage::ImageRgba8(card), CoverKind::Icon, source)
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn engine_picture(root: &Path, engine: EngineKind) -> Option<Cover> {
    match engine {
        EngineKind::Uk2 => uk2_picture(root),
        _ => None,
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn engine_picture(_root: &Path, _engine: EngineKind) -> Option<Cover> {
    None
}

/// UK2 (PC-98) games have no icons: use the first image the opening
/// scripts show that is not mostly black.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn uk2_picture(root: &Path) -> Option<Cover> {
    use std::collections::VecDeque;
    use uk2::value::{InlineStringPart, Operand};
    use uk2::{InstructionArg, InstructionKind, TwoOpcode};

    fn literals(operand: &Operand, out: &mut Vec<Vec<u8>>) {
        if let Operand::InlineString(parts) = operand {
            let mut text = Vec::new();
            for part in parts {
                if let InlineStringPart::Literal(bytes) = part {
                    text.extend_from_slice(bytes);
                }
            }
            out.push(text);
        }
    }

    let game = uk2::Uk2Game::open(root).ok()?;
    let mut queue: VecDeque<uk2::MesProgram> = VecDeque::from([game.start_mes().ok()?]);
    let mut seen_mes: Vec<Vec<u8>> = Vec::new();
    let mut seen_images: Vec<Vec<u8>> = Vec::new();
    let mut best: Option<(f32, DynamicImage, String)> = None;
    let mut scripts = 0;
    while let Some(program) = queue.pop_front() {
        scripts += 1;
        if scripts > 48 || seen_images.len() >= 24 {
            break;
        }
        let Ok(instructions) = uk2::disassemble_reachable(&program) else {
            continue;
        };
        for instruction in instructions.values() {
            let InstructionKind::Command { opcode, arguments } = &instruction.kind else {
                continue;
            };
            let mut names = Vec::new();
            for argument in arguments {
                match argument {
                    InstructionArg::Direct(operand) => literals(operand, &mut names),
                    InstructionArg::ResourceList(resources) => {
                        for resource in resources {
                            for term in &resource.resource.terms {
                                literals(&term.operand, &mut names);
                            }
                        }
                    }
                    _ => {}
                }
            }
            for name in names {
                let lower = name.to_ascii_lowercase();
                let is_mes = lower.windows(4).any(|w| w == b".mes");
                let is_pdt = lower.windows(4).any(|w| w == b".pdt");
                if is_mes && *opcode == TwoOpcode::J2 {
                    if !seen_mes.contains(&lower) {
                        seen_mes.push(lower);
                        if let Ok(program) = game.load_mes_engine_name(&name) {
                            queue.push_back(program);
                        }
                    }
                } else if is_pdt
                    && matches!(opcode, TwoOpcode::W5 | TwoOpcode::UE | TwoOpcode::U3)
                    && !seen_images.contains(&lower)
                    && seen_images.len() < 24
                {
                    seen_images.push(lower);
                    let Ok(engine_name) = uk2::Uk2Game::decode_engine_name(&name) else {
                        continue;
                    };
                    let Ok(image) = game.load_pdt34(&engine_name) else {
                        continue;
                    };
                    let score = picture_score(&image.rgba);
                    if std::env::var_os("GAME_LAUNCHER_COVER_DEBUG").is_some() {
                        eprintln!("uk2 cover candidate {engine_name}: {score:.2}");
                    }
                    if best
                        .as_ref()
                        .is_none_or(|(best_score, _, _)| score > *best_score)
                        && let Some(buffer) =
                            RgbaImage::from_raw(image.width, image.height, image.rgba.clone())
                    {
                        best = Some((score, DynamicImage::ImageRgba8(buffer), engine_name));
                    }
                }
            }
        }
    }
    let (_, image, name) = best?;
    let image = crop_to_content(image);
    Some(Cover {
        png: encode_png(&image),
        kind: CoverKind::Image,
        source: format!("{} ({name})", root.display()),
    })
}

/// Trims black borders (PDT images often cover part of the screen).
fn crop_to_content(image: DynamicImage) -> DynamicImage {
    let rgba = image.to_rgba8();
    let lit = |p: &Rgba<u8>| u32::from(p[0]) + u32::from(p[1]) + u32::from(p[2]) > 24;
    let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0, 0);
    for (x, y, pixel) in rgba.enumerate_pixels() {
        if lit(pixel) {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
    }
    if x0 > x1 || y0 > y1 || (x1 - x0) < 32 || (y1 - y0) < 32 {
        return image;
    }
    image.crop_imm(x0, y0, x1 - x0 + 1, y1 - y0 + 1)
}

/// Share of bright pixels, weighted by colour variety.
fn picture_score(rgba: &[u8]) -> f32 {
    let mut lit = 0usize;
    let mut colours = std::collections::HashSet::new();
    let total = rgba.len() / 4;
    for pixel in rgba.chunks_exact(4) {
        if u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2]) > 90 {
            lit += 1;
        }
        colours.insert([pixel[0], pixel[1], pixel[2]]);
    }
    lit as f32 / total.max(1) as f32 * (colours.len().min(16) as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_ico() -> Vec<u8> {
        let mut image = RgbaImage::new(16, 16);
        for pixel in image.pixels_mut() {
            *pixel = Rgba([200, 40, 40, 255]);
        }
        let mut out = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(image)
            .write_to(&mut out, ImageFormat::Ico)
            .unwrap();
        out.into_inner()
    }

    #[test]
    fn decodes_ico_and_builds_card() {
        let icon = decode_ico(&tiny_ico()).unwrap();
        assert_eq!(icon.dimensions(), (16, 16));
        let card = icon_card(icon, Path::new("x.ico"));
        let decoded = image::load_from_memory(&card.png).unwrap();
        assert_eq!(
            (decoded.width(), decoded.height()),
            (CARD_WIDTH, CARD_HEIGHT)
        );
        // The icon sits in the middle at an integer scale.
        let centre = decoded
            .to_rgba8()
            .get_pixel(CARD_WIDTH / 2, CARD_HEIGHT / 2)
            .0;
        assert_eq!(centre, [200, 40, 40, 255]);
    }

    #[test]
    fn extracts_icon_from_synthetic_pe() {
        // A minimal PE32 with one section holding a resource tree:
        // RT_GROUP_ICON(14)/1/lang -> group, RT_ICON(3)/1/lang -> image.
        let ico = tiny_ico();
        let image_data = ico[22..].to_vec();
        let mut group = vec![0, 0, 1, 0, 1, 0];
        group.extend(&ico[6..14]);
        group.extend((image_data.len() as u32).to_le_bytes());
        group.extend(1u16.to_le_bytes());

        let section_rva = 0x1000u32;
        let mut rsrc = Vec::new();
        let dir = |entries: &[(u32, u32)]| {
            let mut out = vec![0u8; 12];
            out.extend(0u16.to_le_bytes());
            out.extend((entries.len() as u16).to_le_bytes());
            for (id, target) in entries {
                out.extend(id.to_le_bytes());
                out.extend(target.to_le_bytes());
            }
            out
        };
        // Layout offsets (relative to the section start).
        let root = 0u32;
        let icon_types = 0x40u32;
        let group_types = 0x60u32;
        let icon_lang = 0x80u32;
        let group_lang = 0xa0u32;
        let icon_entry = 0xc0u32;
        let group_entry = 0xd0u32;
        let icon_data = 0x100u32;
        let group_data = icon_data + image_data.len() as u32;
        rsrc.resize((group_data as usize) + group.len(), 0);
        let put = |rsrc: &mut Vec<u8>, at: u32, bytes: &[u8]| {
            rsrc[at as usize..at as usize + bytes.len()].copy_from_slice(bytes);
        };
        put(
            &mut rsrc,
            root,
            &dir(&[
                (3, 0x8000_0000 | icon_types),
                (14, 0x8000_0000 | group_types),
            ]),
        );
        put(&mut rsrc, icon_types, &dir(&[(1, 0x8000_0000 | icon_lang)]));
        put(
            &mut rsrc,
            group_types,
            &dir(&[(1, 0x8000_0000 | group_lang)]),
        );
        put(&mut rsrc, icon_lang, &dir(&[(0x411, icon_entry)]));
        put(&mut rsrc, group_lang, &dir(&[(0x411, group_entry)]));
        let data_entry = |rva: u32, size: usize| {
            let mut out = rva.to_le_bytes().to_vec();
            out.extend((size as u32).to_le_bytes());
            out.extend([0u8; 8]);
            out
        };
        put(
            &mut rsrc,
            icon_entry,
            &data_entry(section_rva + icon_data, image_data.len()),
        );
        put(
            &mut rsrc,
            group_entry,
            &data_entry(section_rva + group_data, group.len()),
        );
        put(&mut rsrc, icon_data, &image_data);
        put(&mut rsrc, group_data, &group);

        let mut pe = vec![0u8; 0x200];
        pe[..2].copy_from_slice(b"MZ");
        pe[0x3c..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        pe[0x80..0x84].copy_from_slice(b"PE\0\0");
        let coff = 0x84;
        pe[coff + 2..coff + 4].copy_from_slice(&1u16.to_le_bytes());
        pe[coff + 16..coff + 18].copy_from_slice(&224u16.to_le_bytes());
        let optional = coff + 20;
        pe[optional..optional + 2].copy_from_slice(&0x10bu16.to_le_bytes());
        let res_dir = optional + 96 + 16;
        pe[res_dir..res_dir + 4].copy_from_slice(&section_rva.to_le_bytes());
        pe[res_dir + 4..res_dir + 8].copy_from_slice(&(rsrc.len() as u32).to_le_bytes());
        let section = optional + 224;
        pe[section..section + 5].copy_from_slice(b".rsrc");
        pe[section + 8..section + 12].copy_from_slice(&(rsrc.len() as u32).to_le_bytes());
        pe[section + 12..section + 16].copy_from_slice(&section_rva.to_le_bytes());
        pe[section + 16..section + 20].copy_from_slice(&(rsrc.len() as u32).to_le_bytes());
        pe[section + 20..section + 24].copy_from_slice(&0x200u32.to_le_bytes());
        pe.extend(&rsrc);

        let rebuilt = exe_icon_ico(&pe).expect("icon group");
        let icon = decode_ico(&rebuilt).expect("decodable ico");
        assert_eq!(icon.dimensions(), (16, 16));
    }
}
