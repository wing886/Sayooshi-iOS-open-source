//! String operations with the same indexing model as the original Windows
//! `TSTR` implementation.
//!
//! Siglus uses `std::basic_string<TCHAR>` and indexes strings by UTF-16 code
//! units in its Unicode build. Rust strings are UTF-8, so byte offsets and
//! Unicode-scalar indexes are not interchangeable with script-visible TSTR
//! indexes.

#[inline]
pub fn ascii_lower(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_lowercase()).collect()
}

#[inline]
pub fn ascii_upper(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

#[inline]
pub fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

pub fn utf16_slice(s: &str, start: usize, len: Option<usize>) -> String {
    let units: Vec<u16> = s.encode_utf16().collect();
    let start = start.min(units.len());
    let end = len
        .map(|n| start.saturating_add(n).min(units.len()))
        .unwrap_or(units.len());
    String::from_utf16_lossy(&units[start..end])
}

pub fn utf16_left(s: &str, len: usize) -> String {
    utf16_slice(s, 0, Some(len))
}

pub fn utf16_right(s: &str, len: usize) -> String {
    let total = utf16_len(s);
    utf16_slice(s, total.saturating_sub(len), None)
}

pub fn utf16_code_unit(s: &str, index: usize) -> Option<u16> {
    s.encode_utf16().nth(index)
}

fn utf16_index_for_byte_offset(s: &str, byte_offset: usize) -> usize {
    s.get(..byte_offset)
        .map(utf16_len)
        .unwrap_or_else(|| utf16_len(s))
}

pub fn search_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    // tona3's str_to_lower() only folds ASCII A-Z, so this transformation is
    // length preserving in UTF-8 as well as UTF-16.
    let hay = ascii_lower(haystack);
    let needle = ascii_lower(needle);
    hay.find(&needle)
        .map(|byte_offset| utf16_index_for_byte_offset(&hay, byte_offset))
}

pub fn rsearch_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    let hay = ascii_lower(haystack);
    let needle = ascii_lower(needle);
    hay.rfind(&needle)
        .map(|byte_offset| utf16_index_for_byte_offset(&hay, byte_offset))
}

#[inline]
pub fn is_hankaku(ch: char) -> bool {
    // tona3 classifies characters by their CP932 multibyte width. These are
    // the script-relevant single-column ranges: ASCII and halfwidth katakana.
    ch.is_ascii() || matches!(ch as u32, 0x00A5 | 0x203E | 0xFF61..=0xFF9F)
}

#[inline]
pub fn display_width_char(ch: char) -> usize {
    if is_hankaku(ch) { 1 } else { 2 }
}

pub fn display_width(s: &str) -> usize {
    s.chars().map(display_width_char).sum()
}

pub fn left_by_display_width(s: &str, limit: usize) -> String {
    let mut width = 0usize;
    let mut out = String::new();
    for ch in s.chars() {
        let w = display_width_char(ch);
        if width + w > limit {
            break;
        }
        width += w;
        out.push(ch);
    }
    out
}

pub fn right_by_display_width(s: &str, limit: usize) -> String {
    let mut width = 0usize;
    let mut out = Vec::new();
    for ch in s.chars().rev() {
        let w = display_width_char(ch);
        if width + w > limit {
            break;
        }
        width += w;
        out.push(ch);
    }
    out.into_iter().rev().collect()
}

pub fn mid_by_display_width(s: &str, start_width: usize, len_width: Option<usize>) -> String {
    let mut width = 0usize;
    let mut out = String::new();
    let end_width = len_width.map(|len| start_width.saturating_add(len));
    for ch in s.chars() {
        let w = display_width_char(ch);
        if width >= start_width {
            width = width.saturating_add(w);
            if end_width.is_some_and(|end| width > end) {
                break;
            }
            out.push(ch);
        } else {
            width = width.saturating_add(w);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_script_indexes_are_not_utf8_offsets() {
        assert_eq!(search_ascii_case_insensitive("あいう", "う"), Some(2));
        assert_eq!(utf16_len("A😀B"), 4);
        assert_eq!(utf16_code_unit("A😀B", 1), Some(0xD83D));
        assert_eq!(utf16_code_unit("A😀B", 2), Some(0xDE00));
    }

    #[test]
    fn halfwidth_katakana_counts_as_one_column() {
        assert_eq!(display_width("Aｱあ"), 4);
    }

    #[test]
    fn mid_len_uses_absolute_display_positions_like_tstr() {
        assert_eq!(mid_by_display_width("あいう", 1, Some(2)), "");
        assert_eq!(mid_by_display_width("あいう", 2, Some(2)), "い");
    }
}
