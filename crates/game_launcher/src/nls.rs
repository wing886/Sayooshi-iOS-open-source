//! Text encodings ("NLS") the launcher lets the player choose.
//!
//! SiglusEngine scripts are Unicode, so the choice only applies to RealLive,
//! AVG32 and UK2.  Every engine defaults to Shift-JIS, the encoding of the
//! original releases; the others serve translation patches.

use engine_detect::EngineKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nls {
    Sjis,
    Gbk,
    Big5,
    Utf8,
    /// cp1252 (RLdev-compiled English/European patches).
    Western,
    /// cp949.
    Korean,
    /// RealLive only: use the encoding RLdev recorded in the scenarios.
    Auto,
}

impl Nls {
    pub fn id(self) -> &'static str {
        match self {
            Self::Sjis => "sjis",
            Self::Gbk => "gbk",
            Self::Big5 => "big5",
            Self::Utf8 => "utf8",
            Self::Western => "western",
            Self::Korean => "korean",
            Self::Auto => "auto",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Sjis => "Shift-JIS (Japanese)",
            Self::Gbk => "GBK (Simplified Chinese)",
            Self::Big5 => "Big5 (Traditional Chinese)",
            Self::Utf8 => "UTF-8",
            Self::Western => "Windows-1252 (Western)",
            Self::Korean => "CP949 (Korean)",
            Self::Auto => "Auto (as recorded by RLdev)",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim().to_ascii_lowercase();
        Some(match text.as_str() {
            "" | "sjis" | "shift-jis" | "shift_jis" | "cp932" => Self::Sjis,
            "gbk" | "gb2312" | "cp936" => Self::Gbk,
            "big5" | "cp950" => Self::Big5,
            "utf8" | "utf-8" => Self::Utf8,
            "western" | "cp1252" => Self::Western,
            "korean" | "cp949" | "euc-kr" => Self::Korean,
            "auto" => Self::Auto,
            _ => return None,
        })
    }

    /// Encodings an engine understands, default first.
    pub fn options(engine: EngineKind) -> &'static [Nls] {
        match engine {
            EngineKind::Avg32 => &[Self::Sjis, Self::Gbk, Self::Big5, Self::Utf8],
            EngineKind::RealLive => &[
                Self::Sjis,
                Self::Gbk,
                Self::Big5,
                Self::Utf8,
                Self::Western,
                Self::Korean,
                Self::Auto,
            ],
            EngineKind::Uk2 => &[Self::Sjis, Self::Gbk, Self::Big5, Self::Korean],
            EngineKind::Siglus | EngineKind::Unknown => &[],
        }
    }

    /// The encoding for `engine`: `requested` when the engine supports it,
    /// otherwise Shift-JIS.
    pub fn for_engine(engine: EngineKind, requested: Option<Nls>) -> Nls {
        requested
            .filter(|nls| Self::options(engine).contains(nls))
            .unwrap_or(Self::Sjis)
    }

    pub fn encoding(self) -> &'static encoding_rs::Encoding {
        match self {
            Self::Sjis | Self::Auto => encoding_rs::SHIFT_JIS,
            Self::Gbk => encoding_rs::GBK,
            Self::Big5 => encoding_rs::BIG5,
            Self::Utf8 => encoding_rs::UTF_8,
            Self::Western => encoding_rs::WINDOWS_1252,
            Self::Korean => encoding_rs::EUC_KR,
        }
    }

    pub fn to_avg32(self) -> avg32::Nls {
        match self {
            Self::Gbk => avg32::Nls::Gbk,
            Self::Big5 => avg32::Nls::Big5,
            Self::Utf8 => avg32::Nls::Utf8,
            _ => avg32::Nls::Sjis,
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub fn to_uk2(self) -> uk2::engine::font::TextEncoding {
        use uk2::engine::font::TextEncoding as T;
        match self {
            Self::Gbk => T::Gbk,
            Self::Big5 => T::Big5,
            Self::Korean => T::Korean,
            _ => T::ShiftJis,
        }
    }

    pub fn to_reallive(self) -> Option<reallive::nls::Nls> {
        use reallive::nls::Nls as R;
        Some(match self {
            Self::Sjis => R::Sjis,
            Self::Gbk => R::Gbk,
            Self::Big5 => R::Big5,
            Self::Utf8 => R::Utf8,
            Self::Western => R::Western,
            Self::Korean => R::Korean,
            Self::Auto => return None,
        })
    }
}
