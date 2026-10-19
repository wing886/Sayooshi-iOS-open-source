//! Emulation of the DLLs games load with `LoadDLL` / `#DLL.nnn`.
//!
//! * `EF00` (Little Busters!): scattered-sprite effects. Function 0 seeds
//!   eight tracks of 32 random control points; function 1 evaluates a
//!   uniform cubic B-spline through them at `F[1162 + track]` / 1000 and
//!   projects the point, writing `F[1151..=1153]`.
//! * `DT00` (Tomoyo After's dungeon RPG): the translation in
//!   [`crate::dt00`], reading its tables from the game's `dt00.dll`.
//! * `PT00` (Little Busters! baseball): the native port in [`crate::pt00`].
//!   It reads its data tables from the game's own `PT00.dll`, loaded on
//!   the first call. The DLL's integer array is the script's `intD`,
//!   copied in before and out after each call.

use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::system::System;

/// Quadrant choices for the eight tracks (four patterns).
const DIRECTIONS: [usize; 32] = [
    0, 2, 1, 3, 0, 2, 1, 3, 1, 3, 2, 0, 1, 3, 2, 0, 0, 0, 0, 0, 3, 1, 2, 0, 3, 1, 3, 1, 0, 2, 3, 1,
];

const TRACK: usize = 0x60;

#[derive(Debug, Clone, PartialEq)]
pub enum Dll {
    Ef00(Vec<f64>),
    /// Built on the first call.
    Pt00(Option<Box<crate::pt00::Pt00>>),
    /// Built on the first call.
    Dt00(Option<Box<crate::dt00::Dt00>>),
    /// Loaded by name but unknown: calls return 0.
    Unknown(String),
}

impl Dll {
    pub fn named(name: &str) -> Self {
        let stem = name
            .trim()
            .trim_end_matches(".dll")
            .trim_end_matches(".DLL")
            .to_ascii_uppercase();
        match stem.as_str() {
            "EF00" => Dll::Ef00(Vec::new()),
            "PT00" => Dll::Pt00(None),
            "DT00" => Dll::Dt00(None),
            _ => Dll::Unknown(name.to_owned()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Dll::Ef00(_) => "EF00",
            Dll::Pt00(_) => "PT00",
            Dll::Dt00(_) => "DT00",
            Dll::Unknown(name) => name,
        }
    }
}

/// Integer memory the DLLs read and write: whole banks, by the bank
/// number of [`crate::memory`] (A–F are 0–5, G 6, Z 7).
pub trait DllMemory {
    fn bank(&mut self, bank: u8) -> Result<&mut [i32]>;

    fn get(&mut self, bank: u8, index: i32) -> Result<i32> {
        let words = self.bank(bank)?;
        usize::try_from(index)
            .ok()
            .and_then(|i| words.get(i).copied())
            .ok_or_else(|| anyhow::anyhow!("reallive: bad DLL memory index {index}"))
    }

    fn set(&mut self, bank: u8, index: i32, value: i32) -> Result<()> {
        let words = self.bank(bank)?;
        let slot = usize::try_from(index)
            .ok()
            .and_then(|i| words.get_mut(i))
            .ok_or_else(|| anyhow::anyhow!("reallive: bad DLL memory index {index}"))?;
        *slot = value;
        Ok(())
    }
}

const BANK_D: u8 = 3;
const BANK_F: u8 = 5;

fn ef00_configure(params: &mut Vec<f64>, sys: &mut System, args: [i32; 4]) {
    if params.is_empty() {
        params.resize(TRACK * 8, 0.0);
    }
    let (top, size) = if args[0] == 1 {
        (0, 0x20)
    } else {
        let top = args[1].clamp(0, 0x20);
        (top, args[2].min(0x20 - top).max(0))
    };
    let mut random = |modulus: i32| f64::from(sys.random(0, modulus - 1));
    for track in 0..8 {
        let at = track * TRACK + top as usize * 3;
        for point in 0..size as usize {
            params[at + point * 3] = random(800) - 400.0;
            params[at + point * 3 + 1] = random(600) - 300.0;
            params[at + point * 3 + 2] = random(700) - 350.0;
        }
    }
    if args[3] != 1 {
        return;
    }
    let pattern = sys.random(0, 3) as usize * 8;
    for track in 0..8 {
        let p = &mut params[track * TRACK..];
        let mut x = f64::from(sys.random(0, 599)) - 300.0;
        let mut y = f64::from(sys.random(0, 479)) - 240.0;
        x += if x < 0.0 { -80.0 } else { 80.0 };
        y += if y < 0.0 { -80.0 } else { 80.0 };
        // Force the start into one quadrant.
        let (sx, sy) = match DIRECTIONS[pattern + track] {
            0 => (1.0, 1.0),
            1 => (-1.0, 1.0),
            2 => (1.0, -1.0),
            _ => (-1.0, -1.0),
        };
        x = x.abs() * sx;
        y = y.abs() * sy;
        p[9] = x * 1.2;
        p[10] = y * 1.2;
        p[11] *= 1.2;
        p[12] *= -0.08;
        p[13] *= -0.08;
        p[14] *= -0.08;
        p[15] = -p[9];
        p[16] = -p[10];
        p[17] = -p[11];
    }
}

fn ef00_calculate(params: &[f64], memory: &mut dyn DllMemory, track: i32) -> Result<()> {
    if params.is_empty() {
        bail!("reallive: EF00 calculation before configuration");
    }
    let track = track.clamp(0, 7);
    let step = memory.get(BANK_F, 1154 + track)?;
    let point = |offset: i32| ((((step + offset) & 0x1f) + track * 0x20) * 3) as usize;
    let (j, k, l, m) = (point(0), point(1), point(2), point(3));
    let x = f64::from(memory.get(BANK_F, 1162 + track)?) * 0.001;
    // Uniform cubic B-spline basis.
    let va = x * x * x / 6.0;
    let vb = (-x * x * x + 3.0 * x * x - 3.0 * x + 1.0) / 6.0;
    let vc = (3.0 * x * x * x - 6.0 * x * x + 4.0) / 6.0;
    let vd = (-3.0 * x * x * x + 3.0 * x * x + 3.0 * x + 1.0) / 6.0;
    // The DLL reads each control point one element late; kept for
    // fidelity with the original.
    let at = |i: usize| params.get(i).copied().unwrap_or(0.0);
    let blend = |o: usize| va * at(m + o) + vd * at(l + o) + vc * at(k + o) + vb * at(j + o);
    let depth = blend(3);
    let (mut px, mut py) = (blend(2), blend(1));
    if depth != 400.0 {
        px = px * 800.0 / (400.0 - depth);
        py = py * 700.0 / (400.0 - depth);
    }
    memory.set(BANK_F, 1151, px as i32)?;
    memory.set(BANK_F, 1152, py as i32)?;
    memory.set(BANK_F, 1153, depth as i32)?;
    Ok(())
}

fn pt00_call(
    state: &mut Option<Box<crate::pt00::Pt00>>,
    sys: &mut System,
    memory: &mut dyn DllMemory,
    args: [i32; 5],
) -> Result<i32> {
    use crate::pt00::{F_LEN, Pt00};
    let pt = match state {
        Some(pt) => pt,
        None => {
            let path = sys.resources.root_file("PT00.dll").ok_or_else(|| {
                anyhow::anyhow!("reallive: PT00.dll not found in the game directory")
            })?;
            let dll = game_fs::read(&path)?;
            let seed = sys.random(0, i32::MAX) as u32;
            state.insert(Box::new(Pt00::load(&dll, seed)?))
        }
    };
    let words = memory.bank(BANK_D)?;
    let len = words.len().min(F_LEN);
    pt.write_f(&words[..len]);
    let result = pt.call(args[0], args[1], args[2], args[3], args[4]);
    pt.read_f(&mut memory.bank(BANK_D)?[..len]);
    Ok(result)
}

fn dt00_call(
    state: &mut Option<Box<crate::dt00::Dt00>>,
    sys: &mut System,
    memory: &mut dyn DllMemory,
    args: [i32; 5],
) -> Result<i32> {
    use crate::dt00::{BANK_COUNT, Dt00};
    let dt = match state {
        Some(dt) => dt,
        None => {
            let path = sys.resources.root_file("dt00.dll").ok_or_else(|| {
                anyhow::anyhow!("reallive: dt00.dll not found in the game directory")
            })?;
            let dll = game_fs::read(&path)?;
            let seed = sys.random(0, i32::MAX) as u32;
            dt00_trace(|out| {
                out.extend_from_slice(b"DT00TRC1");
                out.extend_from_slice(&seed.to_le_bytes());
            });
            state.insert(Box::new(Dt00::load(&dll, seed)?))
        }
    };
    // intA–intF, intG, intZ, in the order of RealLiveState.
    let mut record = Vec::new();
    for arg in args {
        record.extend_from_slice(&arg.to_le_bytes());
    }
    for bank in 0..BANK_COUNT {
        let words = memory.bank(bank as u8)?;
        record.extend(
            words
                .iter()
                .take(crate::dt00::BANK_LEN)
                .flat_map(|v| v.to_le_bytes()),
        );
        dt.write_bank(bank, words);
    }
    let result = dt.call(args[0], args[1], args[2], args[3], args[4]);
    for bank in 0..BANK_COUNT {
        let words = memory.bank(bank as u8)?;
        dt.read_bank(bank, words);
        record.extend(
            words
                .iter()
                .take(crate::dt00::BANK_LEN)
                .flat_map(|v| v.to_le_bytes()),
        );
    }
    record.extend_from_slice(&dt.image_hash().to_le_bytes());
    dt00_trace(|out| out.extend_from_slice(&record));
    if let Some(dir) = std::env::var_os("RL_DT00_IMAGES") {
        static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let _ = game_fs::write(
            std::path::Path::new(&dir).join(format!("image_{n}.bin")),
            dt.image(),
        );
    }
    Ok(result)
}

/// With `RL_DT00_TRACE=<file>`, every DT00 call is appended to the file
/// (arguments, banks before and after, a hash of the DLL's data), for
/// checking the translation against the original DLL.
fn dt00_trace(write: impl FnOnce(&mut Vec<u8>)) {
    use std::io::Write;
    let Some(path) = std::env::var_os("RL_DT00_TRACE") else {
        return;
    };
    let mut out = Vec::new();
    write(&mut out);
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = file.write_all(&out);
    }
}

/// DLL slots of a running game.
#[derive(Debug, Default, Clone)]
pub struct Dlls {
    pub slots: HashMap<i32, Dll>,
}

impl Dlls {
    pub fn load(&mut self, slot: i32, name: &str) {
        self.slots.insert(slot, Dll::named(name));
    }

    pub fn unload(&mut self, slot: i32) {
        self.slots.remove(&slot);
    }

    pub fn call(
        &mut self,
        slot: i32,
        sys: &mut System,
        memory: &mut dyn DllMemory,
        args: [i32; 5],
    ) -> Result<i32> {
        let Some(dll) = self.slots.get_mut(&slot) else {
            bail!("reallive: no DLL is loaded in slot {slot}");
        };
        match dll {
            Dll::Ef00(params) => match args[0] {
                0 => ef00_configure(params, sys, [args[1], args[2], args[3], args[4]]),
                1 => ef00_calculate(params, memory, args[1])?,
                other => bail!("reallive: EF00 has no function {other}"),
            },
            Dll::Pt00(state) => return pt00_call(state, sys, memory, args),
            Dll::Dt00(state) => return dt00_call(state, sys, memory, args),
            Dll::Unknown(_) => return Ok(0),
        }
        Ok(0)
    }
}

/// The local integer banks of a machine's memory.
pub struct Banks<'a> {
    pub memory: &'a mut crate::memory::Memory,
    pub frame: &'a mut crate::memory::FrameMemory,
}

impl DllMemory for Banks<'_> {
    fn bank(&mut self, bank: u8) -> Result<&mut [i32]> {
        self.memory.words_mut(bank, self.frame)
    }
}
