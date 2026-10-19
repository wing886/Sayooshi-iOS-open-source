//! `Gameexe.ini`: the per-title configuration of a RealLive game.
//!
//! Every line of the form `#KEY = value, value, ...` becomes an entry.
//! Values are integers or double-quoted strings; everything else between
//! them (`=`, `,`, `:`, `(`, `)` and spaces) is a separator, and a `-`
//! directly after a digit separates numbers (`0-99` is two values) while
//! one in front of a digit is a sign. Keys may repeat (`#DSTRACK`,
//! `#SE.nnn`, ...); lookups return the first occurrence and [`Gameexe::filter`]
//! walks all of them in file order. Keys are case-insensitive.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};

use crate::nls::Nls;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i32),
    Str(String),
}

impl Value {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Int(value) => Some(*value),
            Value::Str(_) => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(value) => Some(value),
            Value::Int(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The key without `#`, upper-cased.
    pub key: String,
    pub values: Vec<Value>,
}

impl Entry {
    /// `WINDOW.000.ATTR` → `["WINDOW", "000", "ATTR"]`.
    pub fn key_parts(&self) -> Vec<&str> {
        self.key.split('.').collect()
    }

    /// The numeric key part at `index` (`SE.012` → 12 for index 1).
    pub fn key_number(&self, index: usize) -> Option<i32> {
        self.key.split('.').nth(index)?.trim().parse().ok()
    }

    pub fn int(&self, index: usize) -> Option<i32> {
        self.values.get(index)?.as_int()
    }

    pub fn str(&self, index: usize) -> Option<&str> {
        self.values.get(index)?.as_str()
    }

    pub fn ints(&self) -> Vec<i32> {
        self.values.iter().filter_map(Value::as_int).collect()
    }

    pub fn strs(&self) -> Vec<&str> {
        self.values.iter().filter_map(Value::as_str).collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Gameexe {
    entries: Vec<Entry>,
    index: HashMap<String, usize>,
}

impl Gameexe {
    pub fn open(path: &Path, nls: Nls) -> Result<Self> {
        let bytes =
            game_fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
        Ok(Self::parse(&decode_config(&bytes, nls)))
    }

    pub fn parse(text: &str) -> Self {
        let mut gameexe = Self::default();
        for line in text.lines() {
            if let Some(entry) = parse_line(line) {
                gameexe.push(entry);
            }
        }
        gameexe
    }

    fn push(&mut self, entry: Entry) {
        self.index
            .entry(entry.key.clone())
            .or_insert(self.entries.len());
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn get(&self, key: &str) -> Option<&Entry> {
        let key = key.trim_start_matches('#').to_ascii_uppercase();
        self.index.get(&key).map(|&index| &self.entries[index])
    }

    pub fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn int(&self, key: &str) -> Option<i32> {
        self.get(key)?.int(0)
    }

    pub fn int_or(&self, key: &str, default: i32) -> i32 {
        self.int(key).unwrap_or(default)
    }

    pub fn int_at(&self, key: &str, index: usize) -> Option<i32> {
        self.get(key)?.int(index)
    }

    pub fn ints(&self, key: &str) -> Vec<i32> {
        self.get(key).map(Entry::ints).unwrap_or_default()
    }

    pub fn str(&self, key: &str) -> Option<&str> {
        self.get(key)?.str(0)
    }

    pub fn str_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.str(key).unwrap_or(default)
    }

    /// Every entry whose key starts with `prefix`, in file order.
    pub fn filter<'a>(&'a self, prefix: &str) -> impl Iterator<Item = &'a Entry> + 'a {
        let prefix = prefix.trim_start_matches('#').to_ascii_uppercase();
        self.entries
            .iter()
            .filter(move |entry| entry.key.starts_with(&prefix))
    }

    /// Adds or replaces an entry (used for defaults and runtime changes).
    pub fn set(&mut self, key: &str, values: Vec<Value>) {
        let key = key.trim_start_matches('#').to_ascii_uppercase();
        match self.index.get(&key) {
            Some(&index) => self.entries[index].values = values,
            None => self.push(Entry { key, values }),
        }
    }
}

/// Decodes a configuration file: UTF-8 when valid, then `nls`, then
/// Shift-JIS.
pub fn decode_config(bytes: &[u8], nls: Nls) -> String {
    if let Ok(text) = std::str::from_utf8(bytes) {
        return text.trim_start_matches('\u{feff}').to_owned();
    }
    nls.decode_strict(bytes)
        .unwrap_or_else(|| Nls::Sjis.decode(bytes))
}

fn parse_line(line: &str) -> Option<Entry> {
    let hash = line.find('#')?;
    // A `#` inside a comment or string further along would be matched
    // too; the original only honours a leading one.
    if !line[..hash].trim().is_empty() {
        return None;
    }
    let rest = &line[hash + 1..];
    let (key, value) = match rest.find('=') {
        Some(equal) => (&rest[..equal], &rest[equal + 1..]),
        None => (rest, ""),
    };
    let key = key.trim().to_ascii_uppercase();
    if key.is_empty() {
        return None;
    }
    Some(Entry {
        key,
        values: tokenize(value),
    })
}

fn tokenize(value: &str) -> Vec<Value> {
    let chars: Vec<char> = value.chars().collect();
    let is_num = |c: char| c == '-' || c.is_ascii_digit();
    let mut values = Vec::new();
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if c == '"' {
            let start = at + 1;
            let mut end = start;
            while end < chars.len() && chars[end] != '"' {
                end += 1;
            }
            values.push(Value::Str(chars[start..end].iter().collect()));
            at = end + 1;
        } else if is_num(c) {
            let mut token = String::new();
            let mut last = '\0';
            while at < chars.len() {
                let c = chars[at];
                if c == '-' {
                    if last.is_ascii_digit() {
                        at += 1;
                        break;
                    }
                    token.push(c);
                } else if c.is_ascii_digit() {
                    token.push(c);
                } else {
                    break;
                }
                last = c;
                at += 1;
            }
            if token != "-" && !token.is_empty() {
                values.push(Value::Int(parse_int(&token)));
            }
        } else {
            at += 1;
        }
    }
    values
}

fn parse_int(token: &str) -> i32 {
    // Leading zeros are common (`#SEEN_START=0001`); huge values saturate.
    let negative = token.starts_with('-');
    let digits = token.trim_start_matches('-');
    let magnitude = digits.bytes().fold(0i64, |acc, digit| {
        (acc * 10 + i64::from(digit - b'0')).min(i64::from(i32::MAX) + 1)
    });
    let value = if negative { -magnitude } else { magnitude };
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_numbers_ranges_and_strings() {
        let exe = Gameexe::parse(
            "#CLANNADDSTRACK = 00000000 - 99999999 - 00269364 = \"BGM01\"  = \"BGM01\"\n\
             #DCDSTRACK=00000000-10998934-00000000=\"dcbgm000\"=\"dcbgm000\"\n\
             #WINDOW.000.MOJI_SIZE=025\n\
             #SEL.000=000,000,639,479:000,000,-01,000\n\
             #CAPTION=\"CLANNAD\"\n",
        );
        let track = exe.get("CLANNADDSTRACK").unwrap();
        assert_eq!(track.ints(), vec![0, 99999999, 269364]);
        assert_eq!(track.strs(), vec!["BGM01", "BGM01"]);
        assert_eq!(exe.ints("DCDSTRACK"), vec![0, 10998934, 0]);
        assert_eq!(exe.int("window.000.moji_size"), Some(25));
        assert_eq!(exe.ints("SEL.000"), vec![0, 0, 639, 479, 0, 0, -1, 0]);
        assert_eq!(exe.str("#CAPTION"), Some("CLANNAD"));
        assert_eq!(
            exe.get("WINDOW.000.MOJI_SIZE").unwrap().key_number(1),
            Some(0)
        );
    }

    #[test]
    fn keeps_repeated_keys_in_order() {
        let exe = Gameexe::parse("#SE.001=\"a\"\n#SE.002=\"b\"\n#SE.001=\"c\"\n");
        assert_eq!(exe.str("SE.001"), Some("a"));
        let all: Vec<_> = exe
            .filter("SE.")
            .map(|entry| entry.str(0).unwrap())
            .collect();
        assert_eq!(all, vec!["a", "b", "c"]);
    }
}
