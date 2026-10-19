//! Text rasterisation for AVG32 message windows, choices and buffer text.
//!
//! The original engine asked the OS to draw Shift-JIS text in the configured
//! system font.  Here a TrueType/OpenType face is rasterised with
//! `ab_glyph`; `NVL_SYSTEM` titles instead use the bitmap font shipped as
//! `FN.DAT` (24x24, 4 bits per pixel, JIS-indexed).

use std::collections::HashMap;
use std::rc::Rc;

use ab_glyph::{Font, FontVec, PxScale, point};
use encoding_rs::SHIFT_JIS;

// OFL-1.1 Noto Sans CJK SC Regular (OTF data; historical upstream filename).
const EMBEDDED_CJK_FONT: &[u8] = include_bytes!("../../siglus_scene_vm/assets/fonts/default.ttf");

/// One rasterised glyph: an 8-bit coverage map positioned relative to the
/// pen position on the baseline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlyphBitmap {
    pub width: usize,
    pub height: usize,
    pub left: i32,
    pub top: i32,
    pub coverage: Vec<u8>,
}

/// Anything that can rasterise characters at a pixel size.
pub trait GlyphSource {
    fn glyph(&mut self, character: char, pixels: u32) -> Option<Rc<GlyphBitmap>>;
}

pub struct TrueTypeFont {
    font: FontVec,
    fallback: Option<FontVec>,
    cache: HashMap<(char, u32), Option<Rc<GlyphBitmap>>>,
}

impl std::fmt::Debug for TrueTypeFont {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TrueTypeFont")
            .finish_non_exhaustive()
    }
}

impl TrueTypeFont {
    /// Loads the first face of a font file or collection that contains
    /// Japanese kana.
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        (0..16).find_map(|index| {
            let font = FontVec::try_from_vec_and_index(bytes.clone(), index).ok()?;
            (font.glyph_id('あ').0 != 0).then(|| Self {
                font,
                fallback: None,
                cache: HashMap::new(),
            })
        })
    }

    /// A Japanese system font, preferring `AVG32_FONT` when it is set.
    pub fn load_system() -> Option<Self> {
        if let Some(font) = game_fs::host_fonts()
            .into_iter()
            .find_map(|bytes| Self::from_bytes(bytes.as_ref().clone()))
        {
            return Some(font);
        }
        if let Some(path) = std::env::var_os("AVG32_FONT") {
            if let Some(font) = game_fs::read(path).ok().and_then(Self::from_bytes) {
                return Some(font);
            }
        }
        const CANDIDATES: &[&str] = &[
            // macOS
            "/System/Library/Fonts/ヒラギノ角ゴシック W4.ttc",
            "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
            "/System/Library/Fonts/Supplemental/Osaka.ttf",
            "/Library/Fonts/Osaka.ttf",
            "/System/Library/Fonts/Hiragino Sans GB.ttc",
            "/Library/Fonts/Arial Unicode.ttf",
            // Windows
            r"C:\Windows\Fonts\msgothic.ttc",
            r"C:\Windows\Fonts\YuGothM.ttc",
            r"C:\Windows\Fonts\meiryo.ttc",
            // Linux / BSD
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/OTF/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/truetype/fonts-japanese-gothic.ttf",
            "/usr/share/fonts/truetype/takao-gothic/TakaoGothic.ttf",
            "/usr/share/fonts/ipa-gothic/ipag.ttf",
            // Nintendo Switch romfs
            "romfs:/font.ttf",
        ];
        CANDIDATES
            .iter()
            .chain(game_fs::ANDROID_CJK_FONTS)
            .find_map(|path| game_fs::read(path).ok().and_then(Self::from_bytes))
            .or_else(|| Self::from_bytes(EMBEDDED_CJK_FONT.to_vec()))
    }
}

impl GlyphSource for TrueTypeFont {
    fn glyph(&mut self, character: char, pixels: u32) -> Option<Rc<GlyphBitmap>> {
        if let Some(cached) = self.cache.get(&(character, pixels)) {
            return cached.clone();
        }
        let scale = PxScale::from(pixels.max(1) as f32);
        let glyph = self
            .font
            .glyph_id(character)
            .with_scale_and_position(scale, point(0.0, 0.0));
        let mut outline = if glyph.id.0 != 0 { self.font.outline_glyph(glyph) } else { None };
        // Host Japanese fonts may lack simplified Chinese menu/title glyphs.
        // Keep the host face when it works, and fall back per character.
        if outline.is_none() && !character.is_whitespace() {
            if self.fallback.is_none() {
                self.fallback = FontVec::try_from_vec(EMBEDDED_CJK_FONT.to_vec()).ok();
            }
            if let Some(font) = &self.fallback {
                let id = font.glyph_id(character);
                if id.0 != 0 { outline = font.outline_glyph(id.with_scale_and_position(scale, point(0.0, 0.0))); }
            }
        }
        let rendered = outline.map(|outline| {
            let bounds = outline.px_bounds();
            let width = bounds.width().ceil().max(0.0) as usize;
            let height = bounds.height().ceil().max(0.0) as usize;
            let mut coverage = vec![0u8; width * height];
            outline.draw(|x, y, value| {
                let (x, y) = (x as usize, y as usize);
                if x < width && y < height {
                    coverage[y * width + x] = (value.clamp(0.0, 1.0) * 255.0).round() as u8;
                }
            });
            Rc::new(GlyphBitmap {
                width,
                height,
                left: bounds.min.x.floor() as i32,
                top: bounds.min.y.floor() as i32,
                coverage,
            })
        });
        self.cache.insert((character, pixels), rendered.clone());
        rendered
    }
}

/// `FN.DAT`: the `NVL_SYSTEM` bitmap font.
#[derive(Debug, Clone)]
pub struct NovelFont {
    data: Vec<u8>,
}

const NOVEL_FONT_BYTES: usize = 12 * 24;
const NOVEL_FONT_SIZE: usize = 2_544_768;

impl NovelFont {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Shift-JIS to JIS X 0208 row/cell code.
    fn sjis_to_jis(sjis: u16) -> Option<u16> {
        let mut low = i32::from(sjis & 0xff);
        let mut high = (i32::from(sjis) - 0x8100) & 0xff00;
        let mut base = 0x2100;
        if low > 0x7f {
            if low >= 0x9f {
                if low > 0xfc {
                    return None;
                }
                base += 0x100;
                low -= 0x5f;
            } else {
                low -= 1;
            }
        } else if low == 0x7f || low < 0x40 {
            return None;
        }
        low -= 0x1f;
        if high >= 0x1f00 {
            if !(0x5f00..=0x6e00).contains(&high) {
                return None;
            }
            high -= 0x5f00;
            base += 0x5f00 - 0x2100;
        }
        Some(((high << 1) + base + low) as u16)
    }

    /// The 12x24-byte (24x24 nibble) glyph for a Shift-JIS code.
    pub fn glyph(&self, sjis: u16) -> Option<&[u8]> {
        let jis = i32::from(Self::sjis_to_jis(sjis)?) - 0x2121;
        let (high, low) = (jis >> 8, jis & 0xff);
        let mut position = ((high * 0x5e + low) * NOVEL_FONT_BYTES as i32).max(0) as usize;
        if position > NOVEL_FONT_SIZE - NOVEL_FONT_BYTES
            || position + NOVEL_FONT_BYTES > self.data.len()
        {
            position = 0;
        }
        self.data.get(position..position + NOVEL_FONT_BYTES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_font_replacement_rasterizes_cjk() {
        let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../siglus_scene_vm/assets/fonts/default.ttf")).unwrap();
        let mut font = TrueTypeFont::from_bytes(bytes).expect("embedded OFL font must parse");
        for ch in "中文あA保存档案读取文字速度音量设定画面模式略过返回标题消除继续游戏".chars() {
            let glyph = font.glyph(ch, 22).unwrap_or_else(|| panic!("outline missing for {ch}"));
            assert!(glyph.coverage.iter().any(|alpha| *alpha > 0));
        }
    }

    #[test]
    fn missing_host_glyph_uses_embedded_cjk_fallback() {
        let mut font = TrueTypeFont::from_bytes(include_bytes!("../tests/fixtures/latin-kana-fixture.otf").to_vec()).unwrap();
        assert_eq!(font.font.glyph_id('中').0, 0);
        assert!(font.glyph('A', 22).is_some());
        assert!(font.fallback.is_none());
        let glyph = font.glyph('中', 22).expect("missing host character must render");
        assert!(glyph.coverage.iter().any(|alpha| *alpha > 0));
        assert!(font.fallback.is_some());
    }

    #[test]
    fn converts_sjis_to_jis_like_the_reference() {
        // "あ" is SJIS 0x82A0, JIS 0x2422.
        assert_eq!(NovelFont::sjis_to_jis(0x82a0), Some(0x2422));
        // "亜" is SJIS 0x889F, JIS 0x3021.
        assert_eq!(NovelFont::sjis_to_jis(0x889f), Some(0x3021));
    }
}
