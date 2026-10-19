//! Glyph rasterisation for text windows, text objects and `grpTextout`.
//!
//! A [`FontSet`] holds a primary face and fallbacks: a character missing
//! from the primary face (a hanzi in a Chinese translation, a hangul
//! syllable, ...) is drawn from the first fallback that has it.

use std::collections::HashMap;
use std::rc::Rc;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont, point};

/// One glyph as an 8-bit coverage map relative to the pen position on the
/// baseline.
#[derive(Debug, Clone, PartialEq)]
pub struct Glyph {
    pub width: usize,
    pub height: usize,
    pub left: i32,
    pub top: i32,
    pub coverage: Vec<u8>,
    /// Horizontal advance in pixels.
    pub advance: f32,
}

pub struct FontSet {
    faces: Vec<FontVec>,
    cache: HashMap<(char, u32, bool), Option<Rc<Glyph>>>,
}

impl std::fmt::Debug for FontSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FontSet({} faces)", self.faces.len())
    }
}

/// Fonts tried in order; the first that loads becomes primary and the
/// others are fallbacks.
const CANDIDATES: &[&str] = &[
    // macOS
    "/System/Library/Fonts/ヒラギノ角ゴシック W4.ttc",
    "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
    "/System/Library/Fonts/PingFang.ttc",
    "/System/Library/Fonts/AppleSDGothicNeo.ttc",
    "/System/Library/Fonts/Supplemental/Osaka.ttf",
    "/Library/Fonts/Arial Unicode.ttf",
    // Windows
    r"C:\Windows\Fonts\msgothic.ttc",
    r"C:\Windows\Fonts\YuGothM.ttc",
    r"C:\Windows\Fonts\meiryo.ttc",
    r"C:\Windows\Fonts\msyh.ttc",
    r"C:\Windows\Fonts\mingliub.ttc",
    r"C:\Windows\Fonts\malgun.ttf",
    // Linux / BSD
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/OTF/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/fonts-japanese-gothic.ttf",
    "/usr/share/fonts/truetype/takao-gothic/TakaoGothic.ttf",
    "/usr/share/fonts/ipa-gothic/ipag.ttf",
    "/usr/share/fonts/wenquanyi/wqy-microhei/wqy-microhei.ttc",
    "romfs:/font.ttf",
];

fn load_faces(bytes: Vec<u8>) -> Vec<FontVec> {
    // Collections: take the first face that has kana, else the first.
    let faces: Vec<FontVec> = (0..8)
        .filter_map(|index| FontVec::try_from_vec_and_index(bytes.clone(), index).ok())
        .collect();
    let preferred = faces
        .iter()
        .position(|face| face.glyph_id('あ').0 != 0)
        .unwrap_or(0);
    faces.into_iter().nth(preferred).into_iter().collect()
}

impl FontSet {
    pub fn empty() -> Self {
        Self {
            faces: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            faces: load_faces(bytes),
            cache: HashMap::new(),
        }
    }

    /// `REALLIVE_FONT` (a path, or several separated by the platform path
    /// separator) followed by the system CJK fonts.
    pub fn load_system() -> Self {
        let mut faces = Vec::new();
        for bytes in game_fs::host_fonts() {
            faces.extend(load_faces(bytes.as_ref().clone()));
        }
        if let Some(paths) = std::env::var_os("REALLIVE_FONT") {
            for path in std::env::split_paths(&paths) {
                if let Ok(bytes) = game_fs::read(&path) {
                    faces.extend(load_faces(bytes));
                }
            }
        }
        for path in CANDIDATES.iter().chain(game_fs::ANDROID_CJK_FONTS) {
            if faces.len() >= 4 {
                break;
            }
            if let Ok(bytes) = game_fs::read(path) {
                faces.extend(load_faces(bytes));
            }
        }
        Self {
            faces,
            cache: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    fn face_for(&self, character: char) -> Option<&FontVec> {
        self.faces
            .iter()
            .find(|face| face.glyph_id(character).0 != 0)
            .or_else(|| self.faces.first())
    }

    /// Rasterises `character` at `pixels` (the full-width cell size).
    pub fn glyph(&mut self, character: char, pixels: u32, bold: bool) -> Option<Rc<Glyph>> {
        let key = (character, pixels, bold);
        if let Some(cached) = self.cache.get(&key) {
            return cached.clone();
        }
        let rendered = self.rasterise(character, pixels, bold).map(Rc::new);
        if self.cache.len() > 8192 {
            self.cache.clear();
        }
        self.cache.insert(key, rendered.clone());
        rendered
    }

    fn rasterise(&self, character: char, pixels: u32, bold: bool) -> Option<Glyph> {
        let face = self.face_for(character)?;
        let scale = PxScale::from(pixels.max(1) as f32);
        let scaled = face.as_scaled(scale);
        let id = face.glyph_id(character);
        let advance = scaled.h_advance(id);
        // Place the baseline so that the em box spans the cell height.
        let glyph = id.with_scale_and_position(scale, point(0.0, 0.0));
        let Some(outline) = face.outline_glyph(glyph) else {
            return Some(Glyph {
                width: 0,
                height: 0,
                left: 0,
                top: 0,
                coverage: Vec::new(),
                advance,
            });
        };
        let bounds = outline.px_bounds();
        let extra = usize::from(bold);
        let width = bounds.width().ceil().max(0.0) as usize + extra;
        let height = bounds.height().ceil().max(0.0) as usize;
        let mut coverage = vec![0u8; width * height];
        outline.draw(|x, y, value| {
            let (x, y) = (x as usize, y as usize);
            if x < width && y < height {
                coverage[y * width + x] = (value.clamp(0.0, 1.0) * 255.0).round() as u8;
            }
        });
        if bold {
            // Synthetic emboldening: widen strokes by one pixel.
            for row in coverage.chunks_mut(width) {
                for x in (1..width).rev() {
                    row[x] = row[x].max(row[x - 1]);
                }
            }
        }
        Some(Glyph {
            width,
            height,
            left: bounds.min.x.floor() as i32,
            top: bounds.min.y.floor() as i32,
            coverage,
            advance,
        })
    }

    /// Distance from the top of a `pixels`-high cell to the baseline.
    pub fn ascent(&self, pixels: u32) -> f32 {
        match self.faces.first() {
            Some(face) => {
                let scaled = face.as_scaled(PxScale::from(pixels.max(1) as f32));
                let (ascent, descent) = (scaled.ascent(), -scaled.descent());
                // Fit ascent + descent into the cell.
                pixels as f32 * ascent / (ascent + descent).max(1.0)
            }
            None => pixels as f32 * 0.88,
        }
    }
}
