//! Full-width glyph source replacing the PC-98 kanji ROM.
//!
//! `UK2.EXE` reads 16x16 JIS X 0208 glyphs from the kanji ROM (`int 18h`
//! AH=14h / ports A1h-A5h).  A dump of a real ROM can be supplied as
//! `KANJI16.ROM` (94x94 JIS cells, 32 bytes each, row-major) in the game
//! directory or through `UK2_KANJI_ROM`; otherwise the embedded public-domain
//! Shinonome 16-dot font is used.  Half-width glyphs come from the game's own
//! `kana.pdt1`, exactly as in the original.

use std::collections::HashMap;
use std::path::Path;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont, point};
use anyhow::{Context, Result};

/// Encoding of the game's double-byte text.  The originals are Shift-JIS;
/// the others serve translation patches, whose double-byte characters are
/// rasterised from a system CJK font (the PC-98 kanji ROM only covers JIS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextEncoding {
    #[default]
    ShiftJis,
    Gbk,
    Big5,
    Korean,
}

impl TextEncoding {
    /// Whether `c` is a whole character on its own.
    pub fn is_single_byte(self, c: u8) -> bool {
        match self {
            Self::ShiftJis => c < 0x80 || (0xa0..0xe0).contains(&c),
            _ => c < 0x81 || c == 0xff,
        }
    }

    fn encoding(self) -> &'static encoding_rs::Encoding {
        match self {
            Self::ShiftJis => encoding_rs::SHIFT_JIS,
            Self::Gbk => encoding_rs::GBK,
            Self::Big5 => encoding_rs::BIG5,
            Self::Korean => encoding_rs::EUC_KR,
        }
    }
}

/// Marker for a double-byte character in a non-Shift-JIS encoding, placed
/// before its two bytes in the text pipeline.
pub const FOREIGN_GLYPH: u8 = 0xff;

/// 16x16 glyphs for non-Shift-JIS text, from a TrueType font.
pub struct ForeignFont {
    encoding: TextEncoding,
    font: Option<FontVec>,
    cache: HashMap<[u8; 2], [u8; 32]>,
}

const CJK_FONTS: &[&str] = &[
    "/System/Library/Fonts/PingFang.ttc",
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
    "/System/Library/Fonts/AppleSDGothicNeo.ttc",
    "/System/Library/Fonts/ヒラギノ角ゴシック W4.ttc",
    "/Library/Fonts/Arial Unicode.ttf",
    r"C:\Windows\Fonts\msyh.ttc",
    r"C:\Windows\Fonts\simsun.ttc",
    r"C:\Windows\Fonts\mingliub.ttc",
    r"C:\Windows\Fonts\malgun.ttf",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/OTF/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/wenquanyi/wqy-microhei/wqy-microhei.ttc",
    "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
    "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
];

impl ForeignFont {
    pub fn load(encoding: TextEncoding) -> Self {
        let probe = match encoding {
            TextEncoding::Korean => '한',
            TextEncoding::Big5 => '體',
            _ => '汉',
        };
        let from_bytes = |bytes: Vec<u8>| {
            (0..8).find_map(|index| {
                let font = FontVec::try_from_vec_and_index(bytes.clone(), index).ok()?;
                (font.glyph_id(probe).0 != 0).then_some(font)
            })
        };
        let font = game_fs::host_fonts()
            .into_iter()
            .find_map(|bytes| from_bytes(bytes.as_ref().clone()))
            .or_else(|| {
                CJK_FONTS
                    .iter()
                    .chain(game_fs::ANDROID_CJK_FONTS)
                    .find_map(|path| std::fs::read(path).ok().and_then(from_bytes))
            });
        if font.is_none() {
            log::warn!("uk2: no CJK font found for {encoding:?} text");
        }
        Self {
            encoding,
            font,
            cache: HashMap::new(),
        }
    }

    pub fn encoding(&self) -> TextEncoding {
        self.encoding
    }

    /// 16x16 glyph (two bytes per row) for a double-byte character.
    pub fn glyph(&mut self, lead: u8, trail: u8) -> [u8; 32] {
        if let Some(glyph) = self.cache.get(&[lead, trail]) {
            return *glyph;
        }
        let mut out = [0u8; 32];
        let bytes = [lead, trail];
        let (text, _, _) = self.encoding.encoding().decode(&bytes);
        if let (Some(font), Some(c)) = (&self.font, text.chars().next()) {
            let scale = PxScale::from(16.0);
            let scaled = font.as_scaled(scale);
            let glyph = font
                .glyph_id(c)
                .with_scale_and_position(scale, point(0.0, scaled.ascent() - 1.0));
            if let Some(outline) = font.outline_glyph(glyph) {
                let bounds = outline.px_bounds();
                let dx = ((16.0 - bounds.width()) / 2.0 - bounds.min.x).round() as i32;
                let dy = bounds.min.y.round() as i32;
                outline.draw(|x, y, coverage| {
                    let px = x as i32 + bounds.min.x as i32 + dx;
                    let py = y as i32 + dy;
                    if coverage >= 0.45 && (0..16).contains(&px) && (0..16).contains(&py) {
                        out[(py * 2 + px / 8) as usize] |= 0x80 >> (px % 8);
                    }
                });
            }
        }
        self.cache.insert([lead, trail], out);
        out
    }
}

static SHINONOME: &[u8] = include_bytes!("../../assets/shinonome_jis16.bin");
const CELL_COUNT: usize = 94 * 94;

pub struct KanjiRom {
    data: Vec<u8>,
}

impl KanjiRom {
    pub fn load(game_root: &Path) -> Result<Self> {
        let candidate = std::env::var_os("UK2_KANJI_ROM")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                let path = game_root.join("KANJI16.ROM");
                path.is_file().then_some(path)
            });
        if let Some(path) = candidate {
            let data = std::fs::read(&path)
                .with_context(|| format!("uk2: failed to read kanji ROM {}", path.display()))?;
            if data.len() >= CELL_COUNT * 32 {
                return Ok(Self { data });
            }
            log::warn!(
                "uk2: {} is not a 94x94x32 kanji dump; using the embedded font",
                path.display()
            );
        }
        Ok(Self::embedded())
    }

    pub fn embedded() -> Self {
        Self {
            data: SHINONOME.to_vec(),
        }
    }

    /// 16x16 glyph (two bytes per row) for a JIS code `0xHHLL`.
    pub fn jis_glyph(&self, jis: u16) -> [u8; 32] {
        let row = (jis >> 8) as usize;
        let col = (jis & 0xff) as usize;
        let mut out = [0u8; 32];
        if (0x21..=0x7e).contains(&row) && (0x21..=0x7e).contains(&col) {
            let at = ((row - 0x21) * 94 + (col - 0x21)) * 32;
            out.copy_from_slice(&self.data[at..at + 32]);
        }
        out
    }

    /// Converts a Shift-JIS double-byte code (lead, trail) to JIS.
    pub fn sjis_to_jis(lead: u8, trail: u8) -> u16 {
        let mut lead = u16::from(lead);
        let trail = u16::from(trail);
        if lead >= 0xe0 {
            lead -= 0x40;
        }
        let mut row = (lead - 0x81) * 2 + 0x21;
        let col = if trail >= 0x9f {
            row += 1;
            trail - 0x7e
        } else {
            trail - 0x1f - u16::from(trail >= 0x80)
        };
        (row << 8) | col
    }

    pub fn sjis_glyph(&self, lead: u8, trail: u8) -> [u8; 32] {
        self.jis_glyph(Self::sjis_to_jis(lead, trail))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gbk_text_uses_single_bytes_for_ascii_only() {
        assert!(TextEncoding::Gbk.is_single_byte(b'A'));
        assert!(!TextEncoding::Gbk.is_single_byte(0xb0));
        assert!(TextEncoding::ShiftJis.is_single_byte(0xb0));
        let mut font = ForeignFont::load(TextEncoding::Gbk);
        if font.font.is_some() {
            // 汉 in GBK.
            let glyph = font.glyph(0xba, 0xba);
            assert_ne!(glyph, [0; 32]);
            for row in glyph.chunks(2) {
                eprintln!(
                    "{}",
                    (0..16)
                        .map(|x| if row[x / 8] & (0x80 >> (x % 8)) != 0 {
                            '#'
                        } else {
                            '.'
                        })
                        .collect::<String>()
                );
            }
        }
    }

    #[test]
    fn converts_shift_jis_to_jis() {
        assert_eq!(KanjiRom::sjis_to_jis(0x88, 0x9f), 0x3021); // 亜
        assert_eq!(KanjiRom::sjis_to_jis(0x82, 0xa0), 0x2422); // あ
        assert_eq!(KanjiRom::sjis_to_jis(0x81, 0x40), 0x2121);
        let rom = KanjiRom::embedded();
        assert_ne!(rom.sjis_glyph(0x88, 0x9f), [0; 32]);
    }
}
