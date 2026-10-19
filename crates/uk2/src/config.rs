//! Parser for `UK2.CFG`.

use std::collections::BTreeMap;

use anyhow::{Result, bail};
use encoding_rs::SHIFT_JIS;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uk2Config {
    pub midi_ext: String,
    pub fm_ext: String,
    pub use_lib: bool,
    pub start: String,
    pub font: String,
    pub ems: bool,
    pub mouse: bool,
    pub extra: BTreeMap<String, String>,
}

impl Uk2Config {
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self> {
        let bytes = bytes.strip_suffix(&[0x1a]).unwrap_or(bytes);
        let (text, _, had_errors) = SHIFT_JIS.decode(bytes);
        if had_errors {
            bail!("uk2: UK2.CFG contains invalid Shift-JIS");
        }
        Self::parse_text(&text)
    }

    pub fn parse_text(text: &str) -> Result<Self> {
        let mut values = BTreeMap::new();
        for raw_line in text.lines() {
            let line = raw_line.split('#').next().unwrap_or_default().trim();
            if line.is_empty() {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            values.insert(key.trim().to_ascii_uppercase(), value.trim().to_owned());
        }

        let midi_ext = take_required(&mut values, "MIDI_EXT")?;
        let fm_ext = take_required(&mut values, "FM_EXT")?;
        let use_lib = parse_on_off(&take_required(&mut values, "LIB")?, "LIB")?;
        let start = take_required(&mut values, "START")?;
        let font = values
            .remove("FONT")
            .unwrap_or_else(|| "font.tab".to_owned());
        let ems = values
            .remove("EMS")
            .map(|value| parse_on_off(&value, "EMS"))
            .transpose()?
            .unwrap_or(false);
        let mouse = values
            .remove("MOUSE")
            .map(|value| parse_on_off(&value, "MOUSE"))
            .transpose()?
            .unwrap_or(false);

        Ok(Self {
            midi_ext,
            fm_ext,
            use_lib,
            start,
            font,
            ems,
            mouse,
            extra: values,
        })
    }
}

fn take_required(values: &mut BTreeMap<String, String>, key: &str) -> Result<String> {
    values
        .remove(key)
        .ok_or_else(|| anyhow::anyhow!("uk2: UK2.CFG is missing {key}"))
}

fn parse_on_off(value: &str, key: &str) -> Result<bool> {
    match value.trim().to_ascii_uppercase().as_str() {
        "ON" => Ok(true),
        "OFF" => Ok(false),
        other => bail!("uk2: {key} expects ON or OFF, got {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_reference_style_config() {
        let cfg = Uk2Config::parse_text(
            "MIDI_EXT=MMD\nFM_EXT=MMM\nLIB=OFF\nSTART=start.mes1\nFONT=font.tab1\nEMS=ON\nMOUSE=OFF\n",
        )
        .unwrap();
        assert_eq!(cfg.start, "start.mes1");
        assert!(cfg.ems);
        assert!(!cfg.use_lib);
    }
}
