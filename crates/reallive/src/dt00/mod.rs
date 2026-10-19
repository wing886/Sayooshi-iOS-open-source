//! A native port of Tomoyo After's `dt00.dll` ("Dungeons & Takafumi",
//! the dungeon RPG).
//!
//! The DLL is plain integer C++ (no floating point), so unlike PT00 it is
//! machine-translated: every function of the decompilation became one
//! method in [`code`], keeping C's integer semantics exactly (wrapping
//! arithmetic, sign and width of every value). Calls whose `this` the
//! decompiler dropped were restored from the disassembly. Memory is the
//! same flat, byte-addressed model as PT00: the DLL's data sections (read
//! from the game's own `dt00.dll`, so no game data lives here), a stack
//! for locals whose address is taken, RealLive's interface structure and
//! the integer banks it points at.

mod code;
pub(crate) mod rt;

use anyhow::{Result, bail};

use crate::pt00::pe_sections;

/// `.text` (mapped too: switch tables live in it), `.rdata` from
/// `0x100270F0`, `.data` (with its BSS) up to `IMAGE_END`.
const IMAGE_START: u32 = 0x1000_1000;
const IMAGE_END: u32 = 0x1008_7000;
const RDATA: u32 = 0x1002_70F0;
/// Frames of functions with address-taken locals (grows down).
const STACK_LOW: u32 = 0x1009_0000;
const STACK_TOP: u32 = 0x100B_0000;
/// RealLive's `RealLiveState`.
const IFACE: u32 = 0x100B_0000;
/// intA–intF, intG, intZ, 0x2000 bytes apart.
const BANKS: u32 = 0x100C_0000;
const BANK_STRIDE: u32 = 0x2000;
pub const BANK_COUNT: usize = 8;
pub const BANK_LEN: usize = 2000;
const MEMORY_END: u32 = BANKS + BANK_COUNT as u32 * BANK_STRIDE;

#[derive(Clone, PartialEq)]
pub struct Dt00 {
    mem: Vec<u8>,
    sp: u32,
    /// MSVC `rand()` state.
    holdrand: u32,
}

impl std::fmt::Debug for Dt00 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dt00").finish_non_exhaustive()
    }
}

impl Dt00 {
    /// `reallive_dll_func_load`.
    pub fn load(dll: &[u8], seed: u32) -> Result<Self> {
        let mut dt = Self {
            mem: vec![0; (MEMORY_END - IMAGE_START) as usize],
            sp: STACK_TOP,
            holdrand: seed,
        };
        for (address, bytes) in pe_sections(dll, &[b".text", b".rdata", b".data"])? {
            let end = address + bytes.len() as u32;
            if address < IMAGE_START || end > IMAGE_END {
                bail!("DT00: section at {address:#x} outside the expected image");
            }
            let at = (address - IMAGE_START) as usize;
            dt.mem[at..at + bytes.len()].copy_from_slice(&bytes);
        }
        // RealLiveState: cbSize, hMainWindow, intA..intF, intG, intZ.
        dt.w32(IFACE, 0x100);
        for bank in 0..BANK_COUNT as u32 {
            dt.w32(IFACE + 8 + 4 * bank, BANKS + bank * BANK_STRIDE);
        }
        dt.construct(IFACE);
        Ok(dt)
    }

    /// The body of `reallive_dll_func_load` (0x100012A0), with the object
    /// each call runs on taken from the disassembly.
    fn construct(&mut self, iface: u32) {
        const ROOT: u32 = 0x1008_59EC;
        self.f_10002330(ROOT);
        self.f_10001c70(ROOT, iface as i32);
        self.f_10002330(0x1008_59E4);
        self.f_10001c70(0x1008_59E4, ROOT as i32);
        self.f_10002330(0x1008_59C8);
        self.f_10002350(0x1008_59C8, ROOT as i32);
        self.f_100048b0(0x1008_2160);
        self.f_100048e0(0x1008_2160, ROOT as i32);
        self.f_10006120(0x1008_2160, 0x1008_59C8);
        self.f_100048f0(0x1008_2160, 0x1008_2150);
        self.f_100048b0(0x1008_2150);
        self.f_10001c70(0x1008_2150, ROOT as i32);
        self.f_10006120(0x1008_2150, 0x1008_59C8);
        self.f_100048f0(0x1008_2150, 0x1008_2160);
        self.f_10002330(0x1008_59D8);
        self.f_10005b80(0x1008_59D8, ROOT as i32);
        self.f_10002330(0x1008_59E8);
        self.f_10001c70(0x1008_59E8, ROOT as i32);
        self.f_10001e80(0x1008_59C0);
        self.f_10001c70(0x1008_59C0, ROOT as i32);
        self.f_10006120(0x1008_59C0, 0x1008_2060);
        self.f_10002330(0x1008_59B8);
        self.f_10001c70(0x1008_59B8, ROOT as i32);
        self.f_100048b0(0x1008_2060);
        self.f_10006110(0x1008_2060, ROOT as i32);
        self.f_10006120(0x1008_2060, 0x1008_59C8);
        self.f_100048f0(0x1008_2060, 0x1008_2160);
    }

    /// `reallive_dll_func_call`. Returns 1, as the DLL does.
    pub fn call(&mut self, func: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> i32 {
        match func {
            20 => {
                self.f_10002380(0x100859C8, a2, a3);
            }
            21 => {
                self.f_100027d0(0x100859C8, a2, a3);
            }
            22 => {
                self.f_10002970(0x100859C8, a2, a3, a4, a5);
            }
            23 => {
                self.f_10002b10(0x100859C8, a2, a3);
            }
            24 => {
                self.f_10002cc0(0x100859C8);
            }
            25 => {
                self.f_10002d20(0x100859C8, a2, a3);
            }
            26 => {
                self.f_10003180(0x100859C8, a2, a3);
            }
            27 => {
                self.f_10003340(0x100859C8, a2, a3);
            }
            30 => {
                self.f_10004960(0x10082160);
            }
            31 => {
                self.f_100049d0(0x10082160, a2);
            }
            32 => {
                self.f_10004a10(0x10082160, a2);
            }
            33 => {
                self.f_10004aa0(0x10082160, a2);
            }
            34 => {
                self.f_10004b50(0x10082160, a2, a3);
            }
            35 => {
                self.f_100053d0(0x10082160, a2, a3, a4);
            }
            36 => {
                self.f_100055a0(0x10082160);
            }
            37 => {
                self.f_100055f0(0x10082160);
            }
            38 => {
                self.f_10005650(0x10082160);
            }
            39 => {
                self.f_10005690(0x10082160, a2, a3);
            }
            40 => {
                self.f_100057e0(0x10082160, a2, a3);
            }
            41 => {
                self.f_100059d0(0x10082160, a2 as u32);
            }
            42 => {
                self.f_10005b10(0x10082160);
            }
            50 => {
                self.f_100035a0(0x10082150);
            }
            51 => {
                self.f_100035f0(0x10082150, a2, a3);
            }
            52 => {
                self.f_10003720(0x10082150, a2, a3);
            }
            53 => {
                self.f_10003aa0(0x10082150, a2, a3, a4);
            }
            54 => {
                self.f_10003b60(0x10082150, a2);
            }
            55 => {
                self.f_10003c50(0x10082150, a2);
            }
            56 => {
                self.f_10003ca0(0x10082150, a2);
            }
            57 => {
                self.f_10003cf0(0x10082150, a2, a3, a4);
            }
            58 => {
                self.f_10003f60(0x10082150);
            }
            100 => {
                self.f_10005bb0(0x100859D8, a2);
            }
            110 => {
                self.f_10001c80(0x100859E8);
            }
            111 => {
                self.f_10001d00(0x100859E8);
            }
            112 => {
                self.f_10001d80(0x100859E8);
            }
            113 => {
                self.f_10001dc0(0x100859E8, a2);
            }
            114 => {
                self.f_10001e10(0x100859E8, a2);
            }
            120 => {
                self.f_10001eb0(0x100859C0);
            }
            121 => {
                self.f_10001f00(0x100859C0);
            }
            122 => {
                self.f_10002060(0x100859C0);
            }
            123 => {
                self.f_10002120(0x100859C0);
            }
            124 => {
                self.f_100021c0(0x100859C0);
            }
            125 => {
                self.f_10002210(0x100859C0);
            }
            130 => {
                self.f_10005da0(0x100859B8);
            }
            131 => {
                self.f_10006080(0x100859B8, a2, a3);
            }
            132 => {
                self.f_100077a0(0x10082060, a2, a3);
            }
            200 | 220 | 224..=227 | 230 => {
                self.f_100163e0(0x10082060);
            }
            201 => {
                self.f_1000a480(0x10082060);
            }
            202 => {
                self.f_1000a5f0(0x10082060);
            }
            203 => {
                self.f_1000c1c0(0x10082060);
            }
            204 => {
                self.f_1000c530(0x10082060);
            }
            205 => {
                self.f_1000c5b0(0x10082060);
            }
            206 => {
                self.f_1000cf80(0x10082060);
            }
            207 => {
                self.f_1000dee0(0x10082060);
            }
            208 => {
                self.f_10013c40(0x10082060);
            }
            209 => {
                self.f_10013f70(0x10082060);
            }
            210 => {
                self.f_100145a0(0x10082060);
            }
            221 => {
                self.f_10014970(0x10082060);
            }
            222 => {
                self.f_10014a20(0x10082060);
            }
            223 => {
                self.f_10016130(0x10082060);
            }
            228 => {
                self.f_10016400(0x10082060);
            }
            229 => {
                self.f_100164b0(0x10082060);
            }
            _ => {}
        }
        1
    }

    // ---- RealLive memory ------------------------------------------------------

    /// Copies an integer bank (0–5 = A–F, 6 = G, 7 = Z) in before a call.
    pub fn write_bank(&mut self, bank: usize, values: &[i32]) {
        let base = BANKS + bank as u32 * BANK_STRIDE;
        for (i, &v) in values.iter().take(BANK_LEN).enumerate() {
            self.w32(base + 4 * i as u32, v as u32);
        }
    }

    /// Copies a bank back out after a call.
    pub fn read_bank(&self, bank: usize, values: &mut [i32]) {
        let base = BANKS + bank as u32 * BANK_STRIDE;
        for (i, v) in values.iter_mut().take(BANK_LEN).enumerate() {
            *v = self.r32(base + 4 * i as u32) as i32;
        }
    }

    /// Resets the `rand()` state (for differential testing).
    #[doc(hidden)]
    pub fn set_rand(&mut self, seed: u32) {
        self.holdrand = seed;
    }

    /// The whole flat memory, from `.text` up (for differential testing).
    #[doc(hidden)]
    pub fn memory_mut(&mut self) -> (u32, &mut [u8]) {
        (IMAGE_START, &mut self.mem)
    }

    /// The DLL's `.rdata` and `.data`.
    pub fn image(&self) -> &[u8] {
        &self.mem[(RDATA - IMAGE_START) as usize..(IMAGE_END - IMAGE_START) as usize]
    }

    /// FNV-1a over the DLL's sections (for comparing with the original).
    pub fn image_hash(&self) -> u64 {
        // .rdata and .data (the import table before them is not mapped)
        let (start, end) = (
            (RDATA - IMAGE_START) as usize,
            (IMAGE_END - IMAGE_START) as usize,
        );
        self.mem[start..end]
            .iter()
            .fold(0xcbf2_9ce4_8422_2325, |h, &b| {
                (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3)
            })
    }

    // ---- flat memory ------------------------------------------------------------

    #[inline]
    fn offset(&self, address: u32, len: u32) -> Option<usize> {
        (address >= IMAGE_START && address.checked_add(len)? <= MEMORY_END)
            .then(|| (address - IMAGE_START) as usize)
    }

    #[inline]
    pub(crate) fn r8(&self, address: u32) -> u8 {
        self.offset(address, 1).map_or(0, |at| self.mem[at])
    }

    #[inline]
    pub(crate) fn r16(&self, address: u32) -> u16 {
        self.offset(address, 2).map_or(0, |at| {
            u16::from_le_bytes(self.mem[at..at + 2].try_into().expect("2 bytes"))
        })
    }

    #[inline]
    pub(crate) fn r32(&self, address: u32) -> u32 {
        self.offset(address, 4).map_or(0, |at| {
            u32::from_le_bytes(self.mem[at..at + 4].try_into().expect("4 bytes"))
        })
    }

    #[inline]
    pub(crate) fn r64(&self, address: u32) -> u64 {
        u64::from(self.r32(address)) | u64::from(self.r32(address.wrapping_add(4))) << 32
    }

    #[inline]
    pub(crate) fn w8(&mut self, address: u32, value: u8) {
        if let Some(at) = self.offset(address, 1) {
            self.mem[at] = value;
        }
    }

    #[inline]
    pub(crate) fn w16(&mut self, address: u32, value: u16) {
        if let Some(at) = self.offset(address, 2) {
            self.mem[at..at + 2].copy_from_slice(&value.to_le_bytes());
        }
    }

    #[inline]
    pub(crate) fn w32(&mut self, address: u32, value: u32) {
        if let Some(at) = self.offset(address, 4) {
            self.mem[at..at + 4].copy_from_slice(&value.to_le_bytes());
        }
    }

    #[inline]
    pub(crate) fn w64(&mut self, address: u32, value: u64) {
        self.w32(address, value as u32);
        self.w32(address.wrapping_add(4), (value >> 32) as u32);
    }

    /// `memset`; returns the destination.
    pub(crate) fn memset(&mut self, dest: u32, value: u8, len: u32) -> u32 {
        for i in 0..len {
            self.w8(dest.wrapping_add(i), value);
        }
        dest
    }

    /// `qmemcpy` (forward copy); returns the destination.
    pub(crate) fn memcpy(&mut self, dest: u32, src: u32, len: u32) -> u32 {
        for i in 0..len {
            let b = self.r8(src.wrapping_add(i));
            self.w8(dest.wrapping_add(i), b);
        }
        dest
    }

    /// Allocates a stack frame.
    pub(crate) fn enter(&mut self, size: u32) -> u32 {
        assert!(self.sp - size >= STACK_LOW, "DT00: stack overflow");
        self.sp -= size;
        self.memset(self.sp, 0, size);
        self.sp
    }

    pub(crate) fn leave(&mut self, size: u32) {
        self.sp += size;
    }

    /// MSVC `rand()`.
    pub(crate) fn rand(&mut self) -> i32 {
        self.holdrand = self.holdrand.wrapping_mul(214_013).wrapping_add(2_531_011);
        ((self.holdrand >> 16) & 0x7fff) as i32
    }

    /// The DLL reports internal errors with `MessageBoxA`.
    pub(crate) fn message_box(
        &mut self,
        _window: u32,
        text: u32,
        caption: u32,
        _flags: u32,
    ) -> i32 {
        let read = |dt: &Self, mut at: u32| {
            let mut bytes = Vec::new();
            while bytes.len() < 256 {
                let b = dt.r8(at);
                if b == 0 {
                    break;
                }
                bytes.push(b);
                at += 1;
            }
            encoding_rs::SHIFT_JIS.decode(&bytes).0.into_owned()
        };
        eprintln!("DT00: {}: {}", read(self, caption), read(self, text));
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_dll() -> Option<Vec<u8>> {
        let root = std::env::var("REALLIVE_TEST_TOMOYO").ok()?;
        game_fs::read(std::path::Path::new(&root).join("dt00.dll")).ok()
    }

    #[test]
    fn rejects_non_pe() {
        assert!(Dt00::load(b"not a dll", 1).is_err());
    }

    /// Calls every entry point on the real DLL's tables.
    #[test]
    fn game_dll_runs() {
        let Some(dll) = game_dll() else { return };
        let mut dt = Dt00::load(&dll, 1).expect("load");
        for func in [
            20, 21, 22, 23, 24, 25, 26, 27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 50,
            51, 52, 53, 54, 55, 56, 57, 58, 100, 110, 111, 112, 113, 114, 120, 121, 122, 123, 124,
            125, 130, 131, 132, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 221, 222,
            223, 228, 229, 230,
        ] {
            dt.call(func, 0, 0, 0, 0);
        }
    }
}
