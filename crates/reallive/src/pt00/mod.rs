//! A native port of Little Busters!' `PT00.DLL` (the baseball practice
//! mini-game), translated function by function from the decompiled DLL.
//!
//! The original works on C++ objects and on one of RealLive's integer
//! arrays through raw pointers (called `intF` here after the pointer
//! slot it reads; Little Busters!' scripts see it as `intD`), addressing fields by byte offset. To keep the
//! translation checkable line by line against the decompilation, this
//! port keeps that model: a flat, byte-addressed memory holds the DLL's
//! data sections (read from the game's own `PT00.dll` at load time, so no
//! game data lives in this source), the heap objects the DLL allocates,
//! RealLive's interface structure and a window onto `intF`. Methods are
//! named after the address of the function they translate (`f_10002710`
//! is `sub_10002710`). No x86 code is executed.
//!
//! Object layout used throughout:
//! * every "holder" object (`0x10022500`, `0x100224FC`, …) stores the
//!   address of `intF` in its first word;
//! * an actor (20 of them, one C++ class per role, the cats sharing one)
//!   is 313+ words: `[0]` vtable, `[1]` `intF`, `[2]` its 160-byte record
//!   in `intF` (`F[1000 + 40·n]`), `[3]`/`[4]` the ball state records,
//!   `[5..=7]` a delayed state change, `[8..=10]` a timer, `[11..=310]`
//!   the last 100 positions, `[312]` its number.

// Kept close to the decompilation: the DLL's own constants (its π is
// not `f64::consts::PI`) and its branch structure.
#![allow(
    clippy::approx_constant,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::manual_range_patterns
)]

mod actors;
mod base;
mod special;

use anyhow::{Context, Result, bail};

/// First address of the DLL image mapped into memory (`.rdata`).
const IMAGE_START: u32 = 0x1001_C000;
/// End of `.data` (virtual size) rounded up.
const IMAGE_END: u32 = 0x1002_4000;
/// Heap for objects created by `operator new`.
const HEAP: u32 = 0x1003_0000;
/// RealLive's DLL interface structure (`+20` points at `intF`).
const IFACE: u32 = 0x1004_0000;
/// `intF` itself.
pub const FBASE: u32 = 0x1005_0000;
/// Number of `intF` values mapped.
pub const F_LEN: usize = 2000;
const MEMORY_END: u32 = FBASE + F_LEN as u32 * 4;

/// `dword_100229F0`: holds the interface pointer given to `load`.
const HOLDER: u32 = 0x1002_29F0;
/// Objects whose first word is `intF`.
pub(crate) const FIELD: u32 = 0x1002_2500;
/// The ball object (`dword_10022510`): 100 predicted positions, then the
/// interface holder, a "dead ball" flag and its counters.
pub(crate) const BALL: u32 = 0x1002_2510;
/// `dword_100224A4`: the 20 actor pointers.
pub(crate) const ACTORS: u32 = 0x1002_24A4;
pub(crate) const ACTOR_COUNT: u32 = 20;

/// The DLL's data sections: (virtual address, raw bytes).
pub(crate) fn data_sections(dll: &[u8]) -> Result<Vec<(u32, Vec<u8>)>> {
    pe_sections(dll, &[b".rdata", b".data"])
}

/// The named sections of a DLL based at `0x10000000`.
pub(crate) fn pe_sections(dll: &[u8], wanted: &[&[u8]]) -> Result<Vec<(u32, Vec<u8>)>> {
    let u16_at = |at: usize| -> Result<u16> {
        Ok(u16::from_le_bytes(
            dll.get(at..at + 2)
                .context("DLL: truncated PE")?
                .try_into()?,
        ))
    };
    let u32_at = |at: usize| -> Result<u32> {
        Ok(u32::from_le_bytes(
            dll.get(at..at + 4)
                .context("DLL: truncated PE")?
                .try_into()?,
        ))
    };
    if dll.get(..2) != Some(b"MZ") {
        bail!("DLL: not a PE file");
    }
    let pe = u32_at(0x3c)? as usize;
    if dll.get(pe..pe + 4) != Some(b"PE\0\0") {
        bail!("DLL: not a PE file");
    }
    let sections = u16_at(pe + 6)? as usize;
    let optional = u16_at(pe + 20)? as usize;
    let image_base = u32_at(pe + 24 + 28)?;
    if image_base != 0x1000_0000 {
        bail!("DLL: unexpected image base {image_base:#x}");
    }
    let mut out = Vec::new();
    for i in 0..sections {
        let at = pe + 24 + optional + 40 * i;
        let name = dll.get(at..at + 8).context("DLL: truncated PE")?;
        let virtual_address = image_base + u32_at(at + 12)?;
        let raw_size = u32_at(at + 16)? as usize;
        let raw_at = u32_at(at + 20)? as usize;
        let name = &name[..name.iter().position(|&b| b == 0).unwrap_or(8)];
        if wanted.contains(&name) {
            let raw = dll
                .get(raw_at..raw_at + raw_size)
                .context("DLL: truncated section")?;
            out.push((virtual_address, raw.to_vec()));
        }
    }
    Ok(out)
}

/// Converts like the compiler's `__ftol`: truncation towards zero.
#[inline]
pub(crate) fn ftol(x: f64) -> i32 {
    x as i64 as i32
}

#[derive(Clone, PartialEq)]
pub struct Pt00 {
    mem: Vec<u8>,
    heap_top: u32,
    /// MSVC `rand()` state.
    holdrand: u32,
}

impl std::fmt::Debug for Pt00 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pt00").finish_non_exhaustive()
    }
}

impl Pt00 {
    /// `reallive_dll_func_load`: maps the DLL's data and builds the actors.
    pub fn load(dll: &[u8], seed: u32) -> Result<Self> {
        let mut pt = Self {
            mem: vec![0; (MEMORY_END - IMAGE_START) as usize],
            heap_top: HEAP,
            holdrand: seed,
        };
        for (address, bytes) in data_sections(dll)? {
            let end = address + bytes.len() as u32;
            if address < IMAGE_START || end > IMAGE_END {
                bail!("PT00: section at {address:#x} outside the expected image");
            }
            let at = (address - IMAGE_START) as usize;
            pt.mem[at..at + bytes.len()].copy_from_slice(&bytes);
        }
        // Static initialisers (sub_10004A10).
        pt.si(0x1002_2A00, 2100);
        pt.si(0x1002_29F8, 0);
        pt.si(0x1002_29FC, 0);
        // The interface structure: +20 is intF.
        pt.si(IFACE + 20, FBASE as i32);
        pt.construct();
        Ok(pt)
    }

    fn construct(&mut self) {
        // Actor classes by vtable, and their object sizes.
        const CLASSES: [(u32, u32); 11] = [
            (0x1001_C260, 0x4E4),
            (0x1001_C23C, 0x4E4),
            (0x1001_C218, 0x4E4),
            (0x1001_C1F4, 0x4E4),
            (0x1001_C1D0, 0x4E4),
            (0x1001_C1AC, 0x4E4),
            (0x1001_C548, 0x4E4),
            (0x1001_C188, 0x4F4),
            (0x1001_C164, 0x4F4),
            (0x1001_C140, 0x4E4),
            (0x1001_C11C, 0x4F0),
        ];
        for (i, &(vtable, size)) in CLASSES.iter().enumerate() {
            let object = self.alloc(size);
            self.si(object, vtable as i32);
            if i == 6 {
                self.f_1000cae0(object);
            }
            self.si(ACTORS + 4 * i as u32, object as i32);
        }
        for i in 11..ACTOR_COUNT {
            let object = self.alloc(0x4E4);
            self.si(object, 0x1001_C0F8u32 as i32);
            self.si(ACTORS + 4 * i, object as i32);
        }
        self.si(HOLDER, IFACE as i32);
        // Objects that keep a pointer to the holder.
        for object in [0x1002_29E0, 0x1002_2490, 0x1002_24F8] {
            self.si(object, HOLDER as i32);
        }
        // The ball keeps it at +1200.
        self.si(BALL + 1200, HOLDER as i32);
        // Objects that keep intF itself.
        for object in [
            FIELD,
            0x1002_24FC,
            0x1002_29D0,
            0x1002_29E8,
            0x1002_24F4,
            0x1002_2498,
        ] {
            self.si(object, FBASE as i32);
        }
        for i in 0..ACTOR_COUNT {
            let actor = self.actor(i);
            self.f_10004a70(actor, i as i32);
        }
    }

    fn alloc(&mut self, size: u32) -> u32 {
        let object = self.heap_top;
        self.heap_top = (self.heap_top + size + 15) & !15;
        assert!(self.heap_top <= IFACE, "PT00: heap exhausted");
        object
    }

    pub(crate) fn actor(&self, index: u32) -> u32 {
        self.p(ACTORS + 4 * index)
    }

    // ---- memory -----------------------------------------------------------

    #[inline]
    fn offset(&self, address: u32, len: u32) -> Option<usize> {
        (address >= IMAGE_START && address.checked_add(len)? <= MEMORY_END)
            .then(|| (address - IMAGE_START) as usize)
    }

    #[inline]
    pub(crate) fn i(&self, address: u32) -> i32 {
        match self.offset(address, 4) {
            Some(at) => i32::from_le_bytes(self.mem[at..at + 4].try_into().expect("4 bytes")),
            None => 0,
        }
    }

    /// A pointer-valued word.
    #[inline]
    pub(crate) fn p(&self, address: u32) -> u32 {
        self.i(address) as u32
    }

    #[inline]
    pub(crate) fn si(&mut self, address: u32, value: i32) {
        if let Some(at) = self.offset(address, 4) {
            self.mem[at..at + 4].copy_from_slice(&value.to_le_bytes());
        }
    }

    #[inline]
    pub(crate) fn add(&mut self, address: u32, value: i32) {
        let v = self.i(address).wrapping_add(value);
        self.si(address, v);
    }

    #[inline]
    pub(crate) fn b(&self, address: u32) -> u8 {
        self.offset(address, 1).map_or(0, |at| self.mem[at])
    }

    #[inline]
    pub(crate) fn sb(&mut self, address: u32, value: u8) {
        if let Some(at) = self.offset(address, 1) {
            self.mem[at] = value;
        }
    }

    pub(crate) fn d(&self, address: u32) -> f64 {
        let lo = self.i(address) as u32 as u64;
        let hi = self.i(address + 4) as u32 as u64;
        f64::from_bits(lo | (hi << 32))
    }

    pub(crate) fn sd(&mut self, address: u32, value: f64) {
        let bits = value.to_bits();
        self.si(address, bits as u32 as i32);
        self.si(address + 4, (bits >> 32) as u32 as i32);
    }

    pub(crate) fn fill(&mut self, address: u32, len: u32, value: u8) {
        if let Some(at) = self.offset(address, len) {
            self.mem[at..at + len as usize].fill(value);
        }
    }

    /// `intF[index]`.
    #[inline]
    pub(crate) fn f(&self, index: u32) -> i32 {
        self.i(FBASE + 4 * index)
    }

    #[inline]
    pub(crate) fn sf(&mut self, index: u32, value: i32) {
        self.si(FBASE + 4 * index, value);
    }

    /// Copies RealLive's `intF` in before a call.
    pub fn write_f(&mut self, values: &[i32]) {
        for (i, &v) in values.iter().take(F_LEN).enumerate() {
            self.sf(i as u32, v);
        }
    }

    /// Copies `intF` back out after a call.
    pub fn read_f(&self, values: &mut [i32]) {
        for (i, v) in values.iter_mut().take(F_LEN).enumerate() {
            *v = self.f(i as u32);
        }
    }

    // ---- C runtime --------------------------------------------------------

    /// MSVC `rand()`.
    pub(crate) fn rand(&mut self) -> i32 {
        self.holdrand = self.holdrand.wrapping_mul(214_013).wrapping_add(2_531_011);
        ((self.holdrand >> 16) & 0x7fff) as i32
    }

    // ---- virtual calls ---------------------------------------------------------

    /// Calls slot `offset / 4` of `object`'s vtable.
    pub(crate) fn vcall(&mut self, object: u32, offset: u32, a: i32, b: i32) -> i32 {
        let vtable = self.p(object);
        let function = self.p(vtable + offset);
        self.dispatch(function, object, a, b)
    }

    // ---- entry point -----------------------------------------------------------

    /// `reallive_dll_func_call`. Returns 1, as the DLL does.
    pub fn call(&mut self, func: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32 {
        let actor = |pt: &Self, n: i32| {
            (0..ACTOR_COUNT as i32)
                .contains(&n)
                .then(|| pt.actor(n as u32))
        };
        match func {
            10 => self.f_10002710(a2, a3, a4, a5),
            11 => self.f_10002710(a2 / 2, a3, a4, a5),
            12 => self.f_10002990(a2, a3, a4, a5),
            30 => self.f_10002a30(),
            31 => self.f_10002cd0(),
            50 => self.f_10003b50(),
            60 => {
                self.f_10004310(a2, a3, None);
            }
            61 => self.f_10004400(a2, a3, a4 == 1),
            70 => {
                if let Some(object) = actor(self, a2) {
                    self.vcall(object, 8, 0, 0);
                }
            }
            71 => {
                if let Some(object) = actor(self, a2) {
                    self.vcall(object, 12, 0, 0);
                }
            }
            72 => {
                if let Some(object) = actor(self, a2) {
                    self.f_10004ad0(object);
                }
            }
            100 => {
                let object = self.actor(0);
                self.f_1000cb40(object);
            }
            101 => {
                let object = self.actor(0);
                self.vcall(object, 20, 2, 0);
            }
            102 => {
                let object = self.actor(0);
                self.vcall(object, 20, 11, 0);
            }
            103 => {
                let object = self.actor(0);
                self.vcall(object, 20, 4, 0);
            }
            190 => {
                let object = self.actor(9);
                self.vcall(object, 20, 2, 0);
            }
            900 => self.f_100135a0(),
            901 => self.f_10013600(),
            910 => self.f_10013a20(),
            911 => self.f_10013af0(),
            920 => self.f_10013bf0(),
            921 => self.f_10013d10(),
            930 => self.f_10013ff0(),
            931 => self.f_10014000(),
            _ => {}
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_dll() -> Option<Vec<u8>> {
        let root = std::env::var("REALLIVE_TEST_GAME").ok()?;
        game_fs::read(std::path::Path::new(&root).join("PT00.dll")).ok()
    }

    #[test]
    fn rejects_non_pe() {
        assert!(Pt00::load(b"not a dll", 1).is_err());
    }

    /// Runs every entry point for a while on the real DLL's tables.
    #[test]
    fn game_dll_runs() {
        let Some(dll) = game_dll() else { return };
        let mut pt = Pt00::load(&dll, 1).expect("load");
        let mut f = vec![0; F_LEN];
        pt.call(900, 0, 0, 0, 0);
        pt.call(910, 0, 0, 0, 0);
        pt.call(920, 0, 0, 0, 0);
        pt.call(930, 0, 0, 0, 0);
        for n in 0..ACTOR_COUNT as i32 {
            pt.call(72, n, 0, 0, 0);
        }
        pt.call(101, 0, 0, 0, 0);
        for frame in 0..2000 {
            if frame % 300 == 0 {
                pt.call(103, 0, 0, 0, 0);
            }
            pt.call(30, 0, 0, 0, 0);
            pt.call(50, 0, 0, 0, 0);
            for n in 0..ACTOR_COUNT as i32 {
                pt.call(70, n, 0, 0, 0);
                pt.call(71, n, 0, 0, 0);
            }
            pt.call(31, 0, 0, 0, 0);
            for func in [901, 911, 921, 931] {
                pt.call(func, 0, 0, 0, 0);
            }
            pt.call(10, 100, 200, 300, 0);
        }
        pt.read_f(&mut f);
    }
}
