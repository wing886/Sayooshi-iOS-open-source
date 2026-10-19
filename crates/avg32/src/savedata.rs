//! AVG32 `SAVE.INI` files: slots and global flags.
//!
//! The file starts with a global header (the `#SAVETITLE` banner, the
//! persistent variables 1000-1999, the window style and the character
//! names) followed by fixed-size slots whose layout depends on the engine
//! revision.  Keeping the original layout means saves written by the
//! Windows engine load here and vice versa.

use std::path::Path;

use anyhow::Result;

use crate::flags::{Flags, STRING_BYTES, STRING_COUNT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub header: usize,
    pub size: usize,
    pub cd: usize,
    pub stack: usize,
    pub macros: usize,
    pub pad: usize,
}

impl Layout {
    pub fn for_version(version: i32) -> Self {
        let (header, size, cd, stack, macros, pad) = match version {
            1604 => (0x01454, 0x1f400, 0x14426, 0x1447a, 0x1585e, 0),
            1613 => (0x01468, 0x20000, 0x14426, 0x1447a, 0x1587e, 0),
            1704 => (0x01468, 0x20000, 0x14426, 0x1448a, 0x1587e, 0),
            1714 => (0x048c0, 0x253f8, 0x1444e, 0x164be, 0x178c2, 4),
            _ => (0x01468, 0x21fa0, 0x1444e, 0x164be, 0x178c2, 4),
        };
        Self {
            header,
            size,
            cd,
            stack,
            macros,
            pad,
        }
    }

    /// The global header size only depends on whether the revision is
    /// newer than 17M.
    pub fn global_header(version: i32) -> usize {
        if version > 1713 { 0x048c0 } else { 0x01468 }
    }
}

/// One screen-restoration record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroItem {
    pub cmd: i32,
    pub files: Vec<Vec<u8>>,
    pub args: [i32; 90],
}

impl MacroItem {
    pub fn new(cmd: i32) -> Self {
        Self {
            cmd,
            files: Vec::new(),
            args: [0; 90],
        }
    }
}

pub const MACRO_BYTES: usize = 0x470;
pub const MAX_MACROS: usize = 32;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SlotSummary {
    pub valid: bool,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub title: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SlotData {
    pub summary: SlotSummary,
    pub year: i32,
    pub values: Vec<i32>,
    pub bits: Vec<bool>,
    pub strings: Vec<Vec<u8>>,
    pub seen: i32,
    pub position: i32,
    pub bgm: Vec<u8>,
    pub stack: Vec<(i32, i32)>,
    pub macros: Vec<MacroItem>,
    pub window_style_force: i32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GlobalData {
    pub values: Vec<i32>,
    pub bits: Vec<bool>,
    pub window_style: i32,
    pub names: Option<[Vec<u8>; 26]>,
}

fn get_i32(bytes: &[u8], at: usize) -> i32 {
    bytes.get(at..at + 4).map_or(0, |slice| {
        i32::from_le_bytes(slice.try_into().expect("four bytes"))
    })
}

fn put_i32(bytes: &mut [u8], at: usize, value: i32) {
    if let Some(slice) = bytes.get_mut(at..at + 4) {
        slice.copy_from_slice(&value.to_le_bytes());
    }
}

/// `LoadStr`: a fixed field whose last byte is forced to NUL.
fn get_str(bytes: &[u8], at: usize, len: usize) -> Vec<u8> {
    let Some(field) = bytes.get(at..at + len) else {
        return Vec::new();
    };
    let field = &field[..len - 1];
    let end = field
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(field.len());
    field[..end].to_vec()
}

fn put_str(bytes: &mut [u8], at: usize, value: &[u8], len: usize) {
    if let Some(field) = bytes.get_mut(at..at + len) {
        field.fill(0);
        let count = value.len().min(len - 1);
        field[..count].copy_from_slice(&value[..count]);
    }
}

fn get_bits(bytes: &[u8], at: usize, count: usize) -> Vec<bool> {
    (0..count)
        .map(|index| {
            bytes
                .get(at + index / 8)
                .is_some_and(|byte| byte & (0x80 >> (index % 8)) != 0)
        })
        .collect()
}

fn put_bits(bytes: &mut [u8], at: usize, bits: impl Iterator<Item = bool>) {
    for (index, bit) in bits.enumerate() {
        if let Some(byte) = bytes.get_mut(at + index / 8) {
            if bit {
                *byte |= 0x80 >> (index % 8);
            } else {
                *byte &= !(0x80 >> (index % 8));
            }
        }
    }
}

fn read_file(path: &Path) -> Vec<u8> {
    game_fs::read(path).unwrap_or_default()
}

/// Reads the global header (global flags and names).
pub fn load_global(path: &Path, version: i32, banner: &[u8]) -> Option<GlobalData> {
    let bytes = read_file(path);
    let header = Layout::global_header(version);
    if bytes.len() < header || !bytes.starts_with(banner) {
        return None;
    }
    let values = (0..1000)
        .map(|index| get_i32(&bytes, 0x80 + index * 4))
        .collect();
    let bits = get_bits(&bytes, 0x1020, 1000);
    let window_style = get_i32(&bytes, 0x11d0) + 1;
    let names_present = (0..26).any(|index| bytes[0x11e4 + index * 16] != 0);
    let names = names_present
        .then(|| std::array::from_fn(|index| get_str(&bytes, 0x11e4 + index * 16, 16)));
    Some(GlobalData {
        values,
        bits,
        window_style,
        names,
    })
}

/// Rewrites only the global header.
pub fn save_global(
    path: &Path,
    version: i32,
    banner: &[u8],
    flags: &Flags,
    window_style: i32,
    names: &[Vec<u8>; 26],
) -> Result<()> {
    let header = Layout::global_header(version);
    let mut bytes = read_file(path);
    if bytes.len() < header {
        bytes.resize(header, 0);
    }
    put_str(&mut bytes, 0, banner, 128);
    for index in 0..1000 {
        put_i32(
            &mut bytes,
            0x80 + index * 4,
            flags.value(1000 + index as i32),
        );
    }
    put_bits(
        &mut bytes,
        0x1020,
        (0..1000).map(|index| flags.bit(1000 + index)),
    );
    put_i32(&mut bytes, 0x11d0, window_style - 1);
    for (index, name) in names.iter().enumerate() {
        let at = 0x11e4 + index * 16;
        bytes[at..at + 16].fill(0);
        let count = name.len().min(16);
        bytes[at..at + count].copy_from_slice(&name[..count]);
    }
    write(path, &bytes)
}

fn write(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        game_fs::create_dir_all(parent)?;
    }
    game_fs::write(path, bytes)?;
    Ok(())
}

fn slot_bytes(bytes: &[u8], layout: Layout, slot: usize) -> Option<&[u8]> {
    let start = layout.header + layout.size * slot;
    bytes.get(start..start + layout.size)
}

fn summary(slot: &[u8], layout: Layout) -> SlotSummary {
    let pad = layout.pad;
    let month = get_i32(slot, pad + 4);
    let day = get_i32(slot, pad + 8);
    let hour = get_i32(slot, pad + 12);
    let minute = get_i32(slot, pad + 16);
    let title = get_str(slot, pad + 0x14, 64);
    let valid = get_i32(slot, 0) == 1
        && (1..=12).contains(&month)
        && (1..=31).contains(&day)
        && (0..=23).contains(&hour)
        && (0..=59).contains(&minute)
        && !title.is_empty();
    SlotSummary {
        valid,
        month,
        day,
        hour,
        minute,
        title,
    }
}

/// Date/title of every slot.
pub fn list_slots(path: &Path, version: i32, count: usize) -> Vec<SlotSummary> {
    let bytes = read_file(path);
    let layout = Layout::for_version(version);
    (0..count)
        .map(|slot| {
            slot_bytes(&bytes, layout, slot)
                .map_or_else(SlotSummary::default, |slot| summary(slot, layout))
        })
        .collect()
}

/// Reads one save slot.
pub fn load_slot(path: &Path, version: i32, slot: usize) -> Option<SlotData> {
    let bytes = read_file(path);
    let layout = Layout::for_version(version);
    let data = slot_bytes(&bytes, layout, slot)?;
    let summary = summary(data, layout);
    if !summary.valid {
        return None;
    }
    let pad = layout.pad;
    let values = (0..1000)
        .map(|index| get_i32(data, pad + 0x10a38 + index * 4))
        .collect();
    let bits = get_bits(data, pad + 0x12978, 1000);
    let strings = (0..STRING_COUNT)
        .map(|index| get_str(data, pad + 0x12a72 + index * STRING_BYTES, STRING_BYTES))
        .collect();
    let stack_count = get_i32(data, layout.stack).clamp(0, 256) as usize;
    let stack = (0..stack_count)
        .map(|index| {
            (
                get_i32(data, layout.stack + 4 + index * 20),
                get_i32(data, layout.stack + 20 + index * 20),
            )
        })
        .collect();
    let macro_count = get_i32(data, layout.macros).clamp(0, MAX_MACROS as i32) as usize;
    let macros = (0..macro_count)
        .map(|index| {
            let at = layout.macros + 4 + index * MACRO_BYTES;
            let file_count = get_i32(data, at + 4).clamp(0, 16) as usize;
            let names = data.get(at + 8..at + 8 + 512).unwrap_or(&[]);
            let files = names
                .split(|byte| *byte == 0)
                .take(file_count.max(1))
                .map(<[u8]>::to_vec)
                .collect();
            let mut args = [0i32; 90];
            for (argument, slot) in args.iter_mut().enumerate() {
                *slot = get_i32(data, at + 520 + argument * 4);
            }
            MacroItem {
                cmd: get_i32(data, at),
                files,
                args,
            }
        })
        .collect();
    Some(SlotData {
        year: get_i32(data, 4),
        values,
        bits,
        strings,
        seen: get_i32(data, pad + 0x14372),
        position: get_i32(data, pad + 0x14382),
        bgm: get_str(data, layout.cd, 64),
        stack,
        macros,
        window_style_force: get_i32(data, 0x143aa),
        summary,
    })
}

/// Writes one save slot.
pub fn save_slot(
    path: &Path,
    version: i32,
    banner: &[u8],
    slot: usize,
    flags: &Flags,
    data: &SlotData,
) -> Result<()> {
    let layout = Layout::for_version(version);
    let mut bytes = read_file(path);
    let end = layout.header + layout.size * (slot + 1);
    if bytes.len() < layout.header {
        bytes.resize(layout.header, 0);
        put_str(&mut bytes, 0, banner, 128);
    }
    if bytes.len() < end {
        bytes.resize(end, 0);
    }
    let start = layout.header + layout.size * slot;
    let buffer = &mut bytes[start..end];
    buffer.fill(0);
    let pad = layout.pad;
    put_i32(buffer, 0, 1);
    put_i32(buffer, 4, data.year);
    put_i32(buffer, pad + 4, data.summary.month);
    put_i32(buffer, pad + 8, data.summary.day);
    put_i32(buffer, pad + 12, data.summary.hour);
    put_i32(buffer, pad + 16, data.summary.minute);
    let title: &[u8] = if data.summary.title.is_empty() {
        b"No Title"
    } else {
        &data.summary.title
    };
    put_str(buffer, pad + 0x14, title, 32);
    for index in 0..2000 {
        put_i32(buffer, pad + 0x10a38 + index * 4, flags.value(index as i32));
    }
    put_bits(
        buffer,
        pad + 0x12978,
        (0..2000).map(|index| flags.bit(index)),
    );
    for index in 0..STRING_COUNT {
        put_str(
            buffer,
            pad + 0x12a72 + index * STRING_BYTES,
            flags.string(index as i32),
            STRING_BYTES,
        );
    }
    put_i32(buffer, pad + 0x14372, data.seen);
    put_i32(buffer, pad + 0x14382, data.position);
    put_str(buffer, layout.cd, &data.bgm, 64);
    put_i32(buffer, layout.stack, data.stack.len() as i32);
    for (index, (seen, position)) in data.stack.iter().enumerate() {
        put_i32(buffer, layout.stack + 4 + index * 20, *seen);
        put_i32(buffer, layout.stack + 20 + index * 20, *position);
    }
    put_i32(
        buffer,
        layout.macros,
        data.macros.len().min(MAX_MACROS) as i32,
    );
    for (index, item) in data.macros.iter().take(MAX_MACROS).enumerate() {
        let at = layout.macros + 4 + index * MACRO_BYTES;
        put_i32(buffer, at, item.cmd);
        put_i32(buffer, at + 4, item.files.len() as i32);
        let mut names = Vec::new();
        for file in &item.files {
            names.extend_from_slice(file);
            names.push(0);
        }
        names.truncate(511);
        if let Some(field) = buffer.get_mut(at + 8..at + 8 + names.len()) {
            field.copy_from_slice(&names);
        }
        for (argument, value) in item.args.iter().enumerate() {
            put_i32(buffer, at + 520 + argument * 4, *value);
        }
    }
    put_i32(buffer, 0x143aa, data.window_style_force);
    write(path, &bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_round_trip_through_the_original_layout() {
        let directory = std::env::temp_dir().join(format!("avg32-save-{}", std::process::id()));
        let path = directory.join("SAVE.INI");
        let mut flags = Flags::default();
        flags.set_value(12, 34);
        flags.set_bit(7, true);
        flags.set_string(2, b"abc");
        let mut data = SlotData {
            seen: 5,
            position: 0x123,
            bgm: b"002".to_vec(),
            stack: vec![(3, 0x40)],
            macros: vec![MacroItem {
                cmd: 2,
                files: vec![b"BG001".to_vec()],
                args: [7; 90],
            }],
            ..SlotData::default()
        };
        data.summary = SlotSummary {
            valid: true,
            month: 9,
            day: 23,
            hour: 12,
            minute: 30,
            title: b"test".to_vec(),
        };
        for version in [1613, 1714] {
            save_slot(&path, version, b"banner", 2, &flags, &data).unwrap();
            let loaded = load_slot(&path, version, 2).unwrap();
            assert_eq!(loaded.values[12], 34);
            assert!(loaded.bits[7]);
            assert_eq!(loaded.strings[2], b"abc");
            assert_eq!((loaded.seen, loaded.position), (5, 0x123));
            assert_eq!(loaded.stack, vec![(3, 0x40)]);
            assert_eq!(loaded.macros, data.macros);
            assert_eq!(list_slots(&path, version, 3)[2].title, b"test");
            assert!(!list_slots(&path, version, 3)[0].valid);
            game_fs::remove_file(&path).unwrap();
        }
        let _ = game_fs::remove_dir_all(directory);
    }
}
