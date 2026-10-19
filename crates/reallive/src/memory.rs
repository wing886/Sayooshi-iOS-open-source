//! RealLive variables.
//!
//! | bank | scope | size |
//! | --- | --- | --- |
//! | `intA`–`intF` | local (saved with a game) | 2000 |
//! | `intG`, `intZ` | global (shared by all saves) | 2000 |
//! | `intL` | per call frame (`*_with` arguments) | 40 |
//! | `strS` | local | 2000 |
//! | `strM` | global | 2000 |
//! | `strK` | per call frame | 3 |
//!
//! Integer banks can also be addressed as packed arrays of 1-, 2-, 4- or
//! 8-bit values (`intAb`, `intA2b`, `intA4b`, `intA8b`): in bytecode the
//! bank byte is `bank + 26 * width_code`. Strings are stored decoded; the
//! local and global name tables (`％Ａ`, `＊Ａ`) have 702 entries each
//! (`Ａ`–`Ｚ` and `ＡＡ`–`ＺＺ`).
//!
//! A savepoint snapshots local memory so that saving always stores the
//! state of the last savepoint rather than the current one.

use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::expr::bank;

pub const BANK_SIZE: usize = 2000;
pub const INT_L_SIZE: usize = 40;
pub const STR_K_SIZE: usize = 3;
pub const NAME_COUNT: usize = 702;

/// A decoded integer memory reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntRef {
    /// 0–6 = A–G, 7 = Z, 8 = L.
    pub bank: u8,
    /// 0 = 32-bit, 1 = 1-bit, 2 = 2-bit, 3 = 4-bit, 4 = 8-bit.
    pub width: u8,
    pub index: i32,
}

pub const BANK_Z: u8 = 7;
pub const BANK_L: u8 = 8;

impl IntRef {
    /// Decodes a bytecode bank byte.
    pub fn from_bytecode(byte: u8, index: i32) -> Option<Self> {
        let width = byte / 26;
        let bank = match byte % 26 {
            b @ 0..=6 => b,
            bank::INT_Z => BANK_Z,
            bank::INT_L => BANK_L,
            _ => return None,
        };
        (width <= 4).then_some(Self { bank, width, index })
    }

    /// `A`–`G`, `Z` or `L`, optionally with a width suffix (`"b"`, `"2b"`,
    /// `"4b"`, `"8b"`).
    pub fn named(name: &str, index: i32) -> Option<Self> {
        let mut chars = name.chars();
        let bank = match chars.next()?.to_ascii_uppercase() {
            c @ 'A'..='G' => c as u8 - b'A',
            'Z' => BANK_Z,
            'L' => BANK_L,
            _ => return None,
        };
        let width = match chars.as_str() {
            "" => 0,
            "b" | "1b" => 1,
            "2b" => 2,
            "4b" => 3,
            "8b" => 4,
            _ => return None,
        };
        Some(Self { bank, width, index })
    }

    pub fn to_bytecode(self) -> u8 {
        let base = match self.bank {
            BANK_Z => bank::INT_Z,
            BANK_L => bank::INT_L,
            other => other,
        };
        base + 26 * self.width
    }

    pub fn name(self) -> String {
        let bank = match self.bank {
            0..=6 => char::from(b'A' + self.bank).to_string(),
            BANK_Z => "Z".into(),
            _ => "L".into(),
        };
        let width = ["", "b", "2b", "4b", "8b"][usize::from(self.width.min(4))];
        format!("int{bank}{width}[{}]", self.index)
    }

    /// Bits per element.
    pub fn bits(self) -> u32 {
        match self.width {
            0 => 32,
            w => 1 << (w - 1),
        }
    }

    /// Number of addressable elements in a bank of `words` 32-bit words.
    pub fn capacity(self, words: usize) -> usize {
        words * 32 / self.bits() as usize
    }

    /// Reads from a bank's 32-bit words.
    pub fn read(self, words: &[i32]) -> Result<i32> {
        let index = self.checked_index(words.len())?;
        if self.width == 0 {
            return Ok(words[index]);
        }
        let bits = self.bits();
        let per_word = (32 / bits) as usize;
        let word = words[index / per_word] as u32;
        let shift = (index % per_word) as u32 * bits;
        Ok(((word >> shift) & ((1u32 << bits) - 1)) as i32)
    }

    /// Writes into a bank's 32-bit words.
    pub fn write(self, words: &mut [i32], value: i32) -> Result<()> {
        let index = self.checked_index(words.len())?;
        if self.width == 0 {
            words[index] = value;
            return Ok(());
        }
        let bits = self.bits();
        let per_word = (32 / bits) as usize;
        let mask = (1u32 << bits) - 1;
        let shift = (index % per_word) as u32 * bits;
        let word = &mut words[index / per_word];
        *word = ((*word as u32 & !(mask << shift)) | ((value as u32 & mask) << shift)) as i32;
        Ok(())
    }

    fn checked_index(self, words: usize) -> Result<usize> {
        if self.index < 0 || self.index as usize >= self.capacity(words) {
            bail!("reallive: {} is out of range", self.name());
        }
        Ok(self.index as usize)
    }
}

/// Memory saved with a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalMemory {
    /// `intA`–`intF`.
    pub ints: [Vec<i32>; 6],
    pub str_s: Vec<String>,
    pub local_names: Vec<String>,
}

impl Default for LocalMemory {
    fn default() -> Self {
        Self {
            ints: std::array::from_fn(|_| vec![0; BANK_SIZE]),
            str_s: vec![String::new(); BANK_SIZE],
            local_names: vec![String::new(); NAME_COUNT],
        }
    }
}

/// Memory shared by every save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalMemory {
    pub int_g: Vec<i32>,
    pub int_z: Vec<i32>,
    pub str_m: Vec<String>,
    pub global_names: Vec<String>,
    /// Scenario number → read kidoku markers (bit set).
    pub kidoku: HashMap<i32, Vec<u64>>,
    /// `ALLSET_READFLAG`: everything counts as read.
    pub all_read: bool,
}

impl Default for GlobalMemory {
    fn default() -> Self {
        Self {
            int_g: vec![0; BANK_SIZE],
            int_z: vec![0; BANK_SIZE],
            str_m: vec![String::new(); BANK_SIZE],
            global_names: vec![String::new(); NAME_COUNT],
            kidoku: HashMap::new(),
            all_read: false,
        }
    }
}

/// Per-call-frame variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameMemory {
    pub int_l: Vec<i32>,
    pub str_k: Vec<String>,
}

impl Default for FrameMemory {
    fn default() -> Self {
        Self {
            int_l: vec![0; INT_L_SIZE],
            str_k: vec![String::new(); STR_K_SIZE],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub local: LocalMemory,
    pub global: GlobalMemory,
    /// Local memory at the last savepoint.
    pub savepoint: LocalMemory,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    fn words<'a>(&'a self, bank: u8, frame: &'a FrameMemory) -> Result<&'a [i32]> {
        Ok(match bank {
            0..=5 => &self.local.ints[usize::from(bank)],
            6 => &self.global.int_g,
            BANK_Z => &self.global.int_z,
            BANK_L => &frame.int_l,
            _ => bail!("reallive: invalid integer bank {bank}"),
        })
    }

    pub(crate) fn words_mut<'a>(
        &'a mut self,
        bank: u8,
        frame: &'a mut FrameMemory,
    ) -> Result<&'a mut [i32]> {
        Ok(match bank {
            0..=5 => &mut self.local.ints[usize::from(bank)],
            6 => &mut self.global.int_g,
            BANK_Z => &mut self.global.int_z,
            BANK_L => &mut frame.int_l,
            _ => bail!("reallive: invalid integer bank {bank}"),
        })
    }

    pub fn int(&self, reference: IntRef, frame: &FrameMemory) -> Result<i32> {
        reference.read(self.words(reference.bank, frame)?)
    }

    pub fn set_int(
        &mut self,
        reference: IntRef,
        value: i32,
        frame: &mut FrameMemory,
    ) -> Result<()> {
        reference.write(self.words_mut(reference.bank, frame)?, value)
    }

    pub fn string<'a>(
        &'a self,
        bank_byte: u8,
        index: i32,
        frame: &'a FrameMemory,
    ) -> Result<&'a str> {
        let slot = usize::try_from(index)
            .ok()
            .filter(|&index| index < BANK_SIZE)
            .ok_or_else(|| anyhow::anyhow!("reallive: string index {index} is out of range"))?;
        Ok(match bank_byte {
            bank::STR_S => &self.local.str_s[slot],
            bank::STR_M => &self.global.str_m[slot],
            bank::STR_K => frame.str_k.get(slot).map_or("", String::as_str),
            other => bail!("reallive: invalid string bank 0x{other:02x}"),
        })
    }

    pub fn set_string(
        &mut self,
        bank_byte: u8,
        index: i32,
        value: String,
        frame: &mut FrameMemory,
    ) -> Result<()> {
        let slot = usize::try_from(index)
            .ok()
            .filter(|&index| index < BANK_SIZE)
            .ok_or_else(|| anyhow::anyhow!("reallive: string index {index} is out of range"))?;
        match bank_byte {
            bank::STR_S => self.local.str_s[slot] = value,
            bank::STR_M => self.global.str_m[slot] = value,
            bank::STR_K => {
                if frame.str_k.len() <= slot {
                    frame.str_k.resize(slot + 1, String::new());
                }
                frame.str_k[slot] = value;
            }
            other => bail!("reallive: invalid string bank 0x{other:02x}"),
        }
        Ok(())
    }

    /// A local integer's value at the last savepoint (global and call
    /// frame variables have none).
    pub fn savepoint_int(&self, reference: IntRef) -> Option<i32> {
        let words = self.savepoint.ints.get(usize::from(reference.bank))?;
        reference.read(words).ok()
    }

    pub fn take_savepoint(&mut self) {
        self.savepoint.clone_from(&self.local);
    }

    pub fn has_been_read(&self, scene: i32, kidoku: i32) -> bool {
        if self.global.all_read {
            return true;
        }
        let Ok(kidoku) = usize::try_from(kidoku) else {
            return false;
        };
        self.global
            .kidoku
            .get(&scene)
            .and_then(|bits| bits.get(kidoku / 64))
            .is_some_and(|word| word & (1 << (kidoku % 64)) != 0)
    }

    pub fn record_kidoku(&mut self, scene: i32, kidoku: i32) {
        let Ok(kidoku) = usize::try_from(kidoku) else {
            return;
        };
        let bits = self.global.kidoku.entry(scene).or_default();
        if bits.len() <= kidoku / 64 {
            bits.resize(kidoku / 64 + 1, 0);
        }
        bits[kidoku / 64] |= 1 << (kidoku % 64);
    }

    /// Name table slot for `Ａ`..`Ｚ`, `ＡＡ`..`ＺＺ` (0-based).
    pub fn name_index(letters: &str) -> Option<usize> {
        let digits: Vec<usize> = letters
            .chars()
            .map(|c| match c {
                'Ａ'..='Ｚ' => Some(c as usize - 'Ａ' as usize),
                'A'..='Z' => Some(c as usize - 'A' as usize),
                _ => None,
            })
            .collect::<Option<_>>()?;
        match digits[..] {
            [a] => Some(a),
            [a, b] => Some(26 + a * 26 + b),
            _ => None,
        }
    }

    pub fn name(&self, local: bool, index: usize) -> &str {
        let table = if local {
            &self.local.local_names
        } else {
            &self.global.global_names
        };
        table.get(index).map_or("", String::as_str)
    }

    pub fn set_name(&mut self, local: bool, index: usize, value: String) {
        let table = if local {
            &mut self.local.local_names
        } else {
            &mut self.global.global_names
        };
        if let Some(slot) = table.get_mut(index) {
            *slot = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_bit_access_shares_words() {
        let mut memory = Memory::new();
        let mut frame = FrameMemory::default();
        let a = |index| IntRef::named("A", index).unwrap();
        let a8 = |index| IntRef::named("A8b", index).unwrap();
        let a1 = |index| IntRef::named("Ab", index).unwrap();
        memory.set_int(a8(1), 0x1ff, &mut frame).unwrap();
        assert_eq!(memory.int(a(0), &frame).unwrap(), 0xff00);
        assert_eq!(memory.int(a1(8), &frame).unwrap(), 1);
        assert_eq!(memory.int(a1(16), &frame).unwrap(), 0);
        memory.set_int(a1(31), 1, &mut frame).unwrap();
        assert_eq!(
            memory.int(a(0), &frame).unwrap(),
            0xff00u32 as i32 | i32::MIN
        );
        assert!(memory.int(a1(64000), &frame).is_err());
        assert!(memory.int(a1(63999), &frame).is_ok());
    }

    #[test]
    fn bytecode_banks() {
        let z = IntRef::from_bytecode(25, 3).unwrap();
        assert_eq!((z.bank, z.width), (BANK_Z, 0));
        let l2 = IntRef::from_bytecode(11 + 26 * 2, 3).unwrap();
        assert_eq!((l2.bank, l2.width), (BANK_L, 2));
        assert_eq!(l2.to_bytecode(), 11 + 52);
    }

    #[test]
    fn names_and_kidoku() {
        assert_eq!(Memory::name_index("Ａ"), Some(0));
        assert_eq!(Memory::name_index("ＡＡ"), Some(26));
        assert_eq!(Memory::name_index("ＺＺ"), Some(701));
        let mut memory = Memory::new();
        assert!(!memory.has_been_read(3, 70));
        memory.record_kidoku(3, 70);
        assert!(memory.has_been_read(3, 70));
    }
}
