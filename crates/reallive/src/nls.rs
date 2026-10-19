//! Text encodings of RealLive scenarios, configuration files and saves.
//!
//! Japanese releases use Shift-JIS (cp932). Translations re-encode their
//! scenarios as GBK, Big5, UTF-8, or (when compiled with RLdev) record the
//! encoding in the scenario metadata: cp932, cp936, "western" (cp1252) or
//! cp949. Text stays as raw bytes inside the bytecode and is decoded when
//! it reaches the interpreter; the byte-level scanners of the bytecode
//! parser use [`Nls::char_len`] so that trail bytes such as `@` (0x40) or
//! `\` (0x5c) are never mistaken for syntax.

use std::fmt;
use std::str::FromStr;

use encoding_rs::{BIG5, EUC_KR, Encoding, GBK, SHIFT_JIS, UTF_8, WINDOWS_1252};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Nls {
    #[default]
    Sjis,
    Gbk,
    Big5,
    Utf8,
    /// cp1252, used by RLdev-compiled English/European translations.
    Western,
    /// cp949.
    Korean,
}

impl Nls {
    pub const ALL: [Nls; 6] = [
        Nls::Sjis,
        Nls::Gbk,
        Nls::Big5,
        Nls::Utf8,
        Nls::Western,
        Nls::Korean,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Nls::Sjis => "sjis",
            Nls::Gbk => "gbk",
            Nls::Big5 => "big5",
            Nls::Utf8 => "utf8",
            Nls::Western => "western",
            Nls::Korean => "korean",
        }
    }

    pub fn encoding(self) -> &'static Encoding {
        match self {
            Nls::Sjis => SHIFT_JIS,
            Nls::Gbk => GBK,
            Nls::Big5 => BIG5,
            Nls::Utf8 => UTF_8,
            Nls::Western => WINDOWS_1252,
            Nls::Korean => EUC_KR,
        }
    }

    /// The encoding recorded in an RLdev metadata block.
    pub fn from_rldev(code: u8) -> Option<Nls> {
        match code {
            0 => Some(Nls::Sjis),
            1 => Some(Nls::Gbk),
            2 => Some(Nls::Western),
            3 => Some(Nls::Korean),
            _ => None,
        }
    }

    /// Byte length of the character that starts with `byte`.
    pub fn char_len(self, byte: u8) -> usize {
        match self {
            Nls::Sjis => {
                if (0x81..=0x9f).contains(&byte) || (0xe0..=0xfc).contains(&byte) {
                    2
                } else {
                    1
                }
            }
            Nls::Gbk | Nls::Big5 | Nls::Korean => {
                if (0x81..=0xfe).contains(&byte) {
                    2
                } else {
                    1
                }
            }
            Nls::Utf8 => match byte {
                0xc0..=0xdf => 2,
                0xe0..=0xef => 3,
                0xf0..=0xf7 => 4,
                _ => 1,
            },
            Nls::Western => 1,
        }
    }

    /// True when `byte` starts a multi-byte character.
    pub fn is_lead(self, byte: u8) -> bool {
        self.char_len(byte) > 1
    }

    pub fn decode(self, bytes: &[u8]) -> String {
        self.encoding()
            .decode_without_bom_handling(bytes)
            .0
            .into_owned()
    }

    /// Strict decoding: `None` if `bytes` is not valid in this encoding.
    pub fn decode_strict(self, bytes: &[u8]) -> Option<String> {
        self.encoding()
            .decode_without_bom_handling_and_without_replacement(bytes)
            .map(|text| text.into_owned())
    }

    pub fn encode(self, text: &str) -> Vec<u8> {
        self.encoding().encode(text).0.into_owned()
    }

    /// Decodes a file name: this encoding when valid, otherwise Shift-JIS.
    pub fn decode_name(self, bytes: &[u8]) -> String {
        self.decode_strict(bytes)
            .unwrap_or_else(|| Nls::Sjis.decode(bytes))
    }

    /// The Shift-JIS reading of a file name that was decoded with this
    /// encoding, when it differs (translations usually keep the original
    /// resource names).
    pub fn sjis_fallback(self, name: &str) -> Option<String> {
        if self == Nls::Sjis || name.is_ascii() {
            return None;
        }
        let bytes = self.encode(name);
        let fallback = Nls::Sjis.decode_strict(&bytes)?;
        (fallback != name).then_some(fallback)
    }
}

impl fmt::Display for Nls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Nls {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> anyhow::Result<Self> {
        let key: String = text
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        Ok(match key.as_str() {
            "sjis" | "shiftjis" | "cp932" | "ms932" | "ja" => Nls::Sjis,
            "gbk" | "gb2312" | "gb18030" | "cp936" | "chs" | "zhcn" => Nls::Gbk,
            "big5" | "cp950" | "cht" | "zhtw" => Nls::Big5,
            "utf8" | "utf" | "unicode" => Nls::Utf8,
            "western" | "cp1252" | "latin1" | "en" => Nls::Western,
            "korean" | "cp949" | "euckr" | "ko" => Nls::Korean,
            _ => anyhow::bail!(
                "unknown NLS encoding {text:?} (expected sjis, gbk, big5, utf8, western or korean)"
            ),
        })
    }
}

/// Display width of a character in half-width cells: 1 for ASCII and
/// half-width katakana, 2 for everything else. RealLive measures string
/// lengths and window columns this way.
pub fn cell_width(character: char) -> usize {
    match character {
        '\u{0}'..='\u{7f}' | '\u{ff61}'..='\u{ff9f}' => 1,
        // Latin-1 and other alphabetic scripts are half-width in the
        // western encodings.
        '\u{80}'..='\u{24f}' => 1,
        _ => 2,
    }
}

/// Width of `text` in half-width cells.
pub fn text_width(text: &str) -> usize {
    text.chars().map(cell_width).sum()
}

/// Converts ASCII (and half-width katakana) to their full-width forms.
pub fn han_to_zen(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            ' ' => '\u{3000}',
            '!'..='~' => char::from_u32(c as u32 - 0x21 + 0xff01).unwrap_or(c),
            _ => HALF_KANA
                .iter()
                .position(|&half| half == c)
                .map_or(c, |index| FULL_KANA[index]),
        })
        .collect()
}

/// Converts full-width ASCII (and katakana) to half-width forms.
pub fn zen_to_han(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\u{3000}' => ' ',
            '\u{ff01}'..='\u{ff5e}' => char::from_u32(c as u32 - 0xff01 + 0x21).unwrap_or(c),
            _ => FULL_KANA
                .iter()
                .position(|&full| full == c)
                .map_or(c, |index| HALF_KANA[index]),
        })
        .collect()
}

const HALF_KANA: [char; 63] = [
    '｡', '｢', '｣', '､', '･', 'ｦ', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ', 'ｰ', 'ｱ', 'ｲ', 'ｳ',
    'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ', 'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ', 'ﾅ', 'ﾆ',
    'ﾇ', 'ﾈ', 'ﾉ', 'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ', 'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ', 'ﾘ', 'ﾙ',
    'ﾚ', 'ﾛ', 'ﾜ', 'ﾝ', 'ﾞ', 'ﾟ',
];
const FULL_KANA: [char; 63] = [
    '。', '「', '」', '、', '・', 'ヲ', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ', 'ッ', 'ー',
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
    'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ',
    'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ン', '゛', '゜',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lead_bytes_follow_the_encoding() {
        assert_eq!(Nls::Sjis.char_len(0x82), 2);
        assert_eq!(Nls::Sjis.char_len(0xa0), 1);
        assert_eq!(Nls::Gbk.char_len(0xa0), 2);
        assert_eq!(Nls::Utf8.char_len(0xe3), 3);
        assert_eq!(Nls::Western.char_len(0xe9), 1);
    }

    #[test]
    fn width_and_conversions() {
        assert_eq!(text_width("aあ"), 3);
        assert_eq!(han_to_zen("A1 ｱ"), "Ａ１\u{3000}ア");
        assert_eq!(zen_to_han("Ａ１\u{3000}ア"), "A1 ｱ");
    }

    #[test]
    fn falls_back_to_sjis_names() {
        let gbk = Nls::Gbk.decode(&Nls::Sjis.encode("背景"));
        assert_eq!(Nls::Gbk.sjis_fallback(&gbk).as_deref(), Some("背景"));
    }
}
