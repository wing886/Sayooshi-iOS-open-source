//! YM2608 (OPNA) sound chip model for the PC-9801-86 board.
//!
//! Written for the UK2 music driver port: six 4-operator FM channels, the
//! three-channel SSG and the six-voice rhythm section.  The FM core follows
//! the documented OPN behaviour (20-bit phase accumulators, detune and
//! multiple, the 10-bit attenuation envelope with the 64-rate increment
//! table, feedback and the eight algorithms).  The rhythm voices are
//! synthesised because the chip's internal sample ROM is not available.

use std::sync::OnceLock;

/// PC-9801-86 master clock.
pub const CLOCK: f64 = 7_987_200.0;
/// FM output sample rate (master clock / 144).
pub const RATE: f64 = CLOCK / 144.0;
/// SSG tone/noise base frequency (the tone period divides this).
const SSG_BASE: f64 = CLOCK / 128.0;

const SIN_BITS: usize = 10;
const SIN_LEN: usize = 1 << SIN_BITS;

struct Tables {
    sine: [f32; SIN_LEN],
    /// Linear gain for each attenuation step (0.09375 dB each).
    gain: Vec<f32>,
    rhythm: [Vec<f32>; 6],
}

fn tables() -> &'static Tables {
    static TABLES: OnceLock<Tables> = OnceLock::new();
    TABLES.get_or_init(|| {
        let mut sine = [0.0f32; SIN_LEN];
        for (i, slot) in sine.iter_mut().enumerate() {
            *slot = ((i as f64 + 0.5) * std::f64::consts::TAU / SIN_LEN as f64).sin() as f32;
        }
        let gain = (0..0x2000)
            .map(|att| 10f64.powf(-(att as f64) * 0.09375 / 20.0) as f32)
            .collect();
        Tables {
            sine,
            gain,
            rhythm: rhythm_samples(),
        }
    })
}

/// Envelope increment patterns (one nibble per step of an 8-step cycle).
const EG_INC: [u32; 64] = {
    let mut table = [0u32; 64];
    table[2] = 0x1010_1010;
    table[3] = 0x1010_1010;
    table[4] = 0x1010_1010;
    table[5] = 0x1010_1010;
    table[6] = 0x1110_1110;
    table[7] = 0x1110_1110;
    let mut rate = 8;
    while rate < 48 {
        table[rate] = 0x1010_1010;
        table[rate + 1] = 0x1011_1010;
        table[rate + 2] = 0x1110_1110;
        table[rate + 3] = 0x1111_1110;
        rate += 4;
    }
    let tail = [
        0x1111_1111,
        0x2111_2111,
        0x2121_2121,
        0x2221_2221,
        0x2222_2222,
        0x4222_4222,
        0x4242_4242,
        0x4442_4442,
        0x4444_4444,
        0x8444_8444,
        0x8484_8484,
        0x8884_8884,
        0x8888_8888,
        0x8888_8888,
        0x8888_8888,
        0x8888_8888,
    ];
    let mut i = 0;
    while i < 16 {
        table[48 + i] = tail[i];
        i += 1;
    }
    table
};

/// Detune offsets indexed by `[dt & 3][keycode]`.
const DETUNE: [[u8; 32]; 4] = [
    [0; 32],
    [
        0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 8, 8,
        8, 8,
    ],
    [
        1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 8, 8, 9, 10, 11, 12, 13, 14,
        16, 16, 16, 16,
    ],
    [
        2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 8, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19,
        20, 22, 22, 22, 22,
    ],
];

/// Hardware LFO frequencies (Hz) for register 22h.
const LFO_HZ: [f64; 8] = [3.98, 5.56, 6.02, 6.37, 6.88, 9.63, 48.1, 72.2];
/// Pitch modulation depth in cents per PMS value.
const PMS_CENTS: [f64; 8] = [0.0, 3.4, 6.7, 10.0, 14.0, 20.0, 40.0, 80.0];
/// Amplitude modulation depth (attenuation steps) per AMS value.
const AMS_STEPS: [f64; 4] = [0.0, 15.0, 63.0, 126.0];

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum EgState {
    Attack,
    Decay,
    Sustain,
    #[default]
    Release,
}

#[derive(Clone, Copy, Default)]
struct Operator {
    dt: u8,
    mul: u8,
    tl: u8,
    ks: u8,
    ar: u8,
    am: bool,
    dr: u8,
    sr: u8,
    sl: u8,
    rr: u8,
    phase: u32,
    incr: u32,
    att: i32,
    state: EgState,
    keyed: bool,
}

impl Operator {
    fn effective_rate(&self, rate: u8, keycode: u8) -> u8 {
        if rate == 0 {
            return 0;
        }
        let ksr = keycode >> (3 - self.ks);
        (rate * 2 + ksr).min(63)
    }

    fn key_on(&mut self, keycode: u8) {
        if self.keyed {
            return;
        }
        self.keyed = true;
        self.phase = 0;
        self.state = EgState::Attack;
        if self.effective_rate(self.ar, keycode) >= 62 {
            self.att = 0;
            self.state = EgState::Decay;
        }
    }

    fn key_off(&mut self) {
        if self.keyed {
            self.keyed = false;
            self.state = EgState::Release;
        }
    }

    fn clock_eg(&mut self, counter: u32, keycode: u8) {
        let rate = match self.state {
            EgState::Attack => self.ar,
            EgState::Decay => self.dr,
            EgState::Sustain => self.sr,
            EgState::Release => self.rr * 2 + 1,
        };
        let rate = self.effective_rate(rate, keycode);
        if self.state == EgState::Decay && self.att >= self.sustain_level() {
            self.state = EgState::Sustain;
        }
        if rate == 0 {
            return;
        }
        let shift = 11u32.saturating_sub(u32::from(rate >> 2));
        if counter & ((1 << shift) - 1) != 0 {
            return;
        }
        let index = (counter >> shift) & 7;
        let inc = ((EG_INC[rate as usize] >> (index * 4)) & 0xf) as i32;
        match self.state {
            EgState::Attack => {
                if rate >= 62 {
                    self.att = 0;
                } else {
                    self.att += ((!self.att) * inc) >> 4;
                }
                if self.att <= 0 {
                    self.att = 0;
                    self.state = EgState::Decay;
                }
            }
            _ => {
                self.att = (self.att + inc).min(0x3ff);
            }
        }
    }

    fn sustain_level(&self) -> i32 {
        let sl = if self.sl == 15 {
            31
        } else {
            i32::from(self.sl)
        };
        sl << 5
    }

    /// Output for a phase offset given in 1/1024 cycles.
    #[inline]
    fn output(&self, modulation: i32, am: i32, tables: &Tables) -> f32 {
        let mut att = self.att + (i32::from(self.tl) << 3);
        if self.am {
            att += am;
        }
        if att >= 0x3ff {
            return 0.0;
        }
        let index = ((self.phase >> 10) as i32 + modulation) as usize & (SIN_LEN - 1);
        tables.sine[index] * tables.gain[att as usize]
    }
}

#[derive(Clone, Copy, Default)]
struct FmChannel {
    ops: [Operator; 4],
    fnum: u16,
    block: u8,
    latch: u8,
    algorithm: u8,
    feedback: u8,
    left: bool,
    right: bool,
    ams: u8,
    pms: u8,
    fb: [f32; 2],
}

impl FmChannel {
    fn keycode(&self) -> u8 {
        let f = self.fnum;
        let n4 = (f >> 10) & 1;
        let n3 = if n4 != 0 {
            u16::from(f & 0x380 != 0)
        } else {
            u16::from(f & 0x380 == 0x380)
        };
        (self.block << 2) | (n4 << 1) as u8 | n3 as u8
    }

    fn update_increments(&mut self, pm_ratio: f64) {
        let keycode = self.keycode();
        let base = (u32::from(self.fnum) << self.block) >> 1;
        let base = if pm_ratio != 1.0 {
            (f64::from(base) * pm_ratio) as u32
        } else {
            base
        };
        for op in &mut self.ops {
            let dt = u32::from(DETUNE[usize::from(op.dt & 3)][usize::from(keycode)]);
            let mut incr = if op.dt & 4 != 0 {
                base.saturating_sub(dt)
            } else {
                base + dt
            } & 0x1ffff;
            incr = if op.mul == 0 {
                incr >> 1
            } else {
                incr * u32::from(op.mul)
            };
            op.incr = incr & 0xfffff;
        }
    }

    fn sample(&mut self, am: i32, tables: &Tables) -> f32 {
        // Phase offsets are in 1/1024 cycle units; a full-scale operator
        // output modulates by +-4 cycles like the chip's `out >> 1`.
        const MOD: f32 = 4096.0;
        let [o1, o2, o3, o4] = &self.ops;
        let fb = if self.feedback != 0 {
            ((self.fb[0] + self.fb[1]) * 8192.0 / (1u32 << (10 - self.feedback)) as f32) as i32
        } else {
            0
        };
        let s1 = o1.output(fb, am, tables);
        self.fb = [self.fb[1], s1];
        let m = |v: f32| (v * MOD) as i32;
        let out = match self.algorithm {
            0 => {
                let s2 = o2.output(m(s1), am, tables);
                let s3 = o3.output(m(s2), am, tables);
                o4.output(m(s3), am, tables)
            }
            1 => {
                let s2 = o2.output(0, am, tables);
                let s3 = o3.output(m(s1 + s2), am, tables);
                o4.output(m(s3), am, tables)
            }
            2 => {
                let s2 = o2.output(0, am, tables);
                let s3 = o3.output(m(s2), am, tables);
                o4.output(m(s1 + s3), am, tables)
            }
            3 => {
                let s2 = o2.output(m(s1), am, tables);
                let s3 = o3.output(0, am, tables);
                o4.output(m(s2 + s3), am, tables)
            }
            4 => {
                let s2 = o2.output(m(s1), am, tables);
                let s3 = o3.output(0, am, tables);
                s2 + o4.output(m(s3), am, tables)
            }
            5 => {
                let mm = m(s1);
                o2.output(mm, am, tables) + o3.output(mm, am, tables) + o4.output(mm, am, tables)
            }
            6 => {
                let s2 = o2.output(m(s1), am, tables);
                s2 + o3.output(0, am, tables) + o4.output(0, am, tables)
            }
            _ => {
                s1 + o2.output(0, am, tables) + o3.output(0, am, tables) + o4.output(0, am, tables)
            }
        };
        for op in &mut self.ops {
            op.phase = (op.phase + op.incr) & 0xfffff;
        }
        out.clamp(-1.0, 1.0)
    }
}

#[derive(Clone, Copy, Default)]
struct SsgChannel {
    period: u16,
    phase: f64,
    high: bool,
    volume: u8,
}

#[derive(Clone, Copy, Default)]
struct RhythmVoice {
    pos: usize,
    playing: bool,
    level: u8,
    left: bool,
    right: bool,
}

pub struct Opna {
    regs: [[u8; 256]; 2],
    fm: [FmChannel; 6],
    ssg: [SsgChannel; 3],
    noise_phase: f64,
    noise_lfsr: u32,
    env_phase: f64,
    env_step: i32,
    env_hold: bool,
    rhythm: [RhythmVoice; 6],
    rhythm_total: u8,
    eg_counter: u32,
    eg_div: u32,
    lfo_phase: f64,
    dc: [f32; 2],
    dc_in: [f32; 2],
}

impl Default for Opna {
    fn default() -> Self {
        Self::new()
    }
}

impl Opna {
    pub fn new() -> Self {
        let mut opna = Self {
            regs: [[0; 256]; 2],
            fm: [FmChannel::default(); 6],
            ssg: [SsgChannel::default(); 3],
            noise_phase: 0.0,
            noise_lfsr: 1,
            env_phase: 0.0,
            env_step: 0,
            env_hold: true,
            rhythm: [RhythmVoice::default(); 6],
            rhythm_total: 0,
            eg_counter: 0,
            eg_div: 0,
            lfo_phase: 0.0,
            dc: [0.0; 2],
            dc_in: [0.0; 2],
        };
        for ch in &mut opna.fm {
            ch.left = true;
            ch.right = true;
            for op in &mut ch.ops {
                op.att = 0x3ff;
            }
        }
        opna.regs[0][7] = 0xbf;
        opna
    }

    pub fn reg(&self, port: usize, reg: u8) -> u8 {
        self.regs[port & 1][usize::from(reg)]
    }

    pub fn write(&mut self, port: usize, reg: u8, value: u8) {
        let port = port & 1;
        self.regs[port][usize::from(reg)] = value;
        if port == 0 && reg < 0x10 {
            self.write_ssg(reg, value);
            return;
        }
        if port == 0 && reg < 0x20 {
            self.write_rhythm(reg, value);
            return;
        }
        if port == 0 && reg == 0x28 {
            let ch = usize::from(value & 3);
            if ch == 3 {
                return;
            }
            let ch = ch + if value & 4 != 0 { 3 } else { 0 };
            let keycode = self.fm[ch].keycode();
            for (slot, op) in self.fm[ch].ops.iter_mut().enumerate() {
                if value & (0x10 << slot) != 0 {
                    op.key_on(keycode);
                } else {
                    op.key_off();
                }
            }
            return;
        }
        if reg < 0x30 {
            return;
        }
        let ch_in_port = usize::from(reg & 3);
        if ch_in_port == 3 {
            return;
        }
        let ch = ch_in_port + port * 3;
        if reg < 0xa0 {
            // Register offsets +0/+4/+8/+C address slots 1/3/2/4.
            let slot = [0, 2, 1, 3][usize::from((reg >> 2) & 3)];
            let op = &mut self.fm[ch].ops[slot];
            match reg & 0xf0 {
                0x30 => {
                    op.dt = (value >> 4) & 7;
                    op.mul = value & 0xf;
                }
                0x40 => op.tl = value & 0x7f,
                0x50 => {
                    op.ks = value >> 6;
                    op.ar = value & 0x1f;
                }
                0x60 => {
                    op.am = value & 0x80 != 0;
                    op.dr = value & 0x1f;
                }
                0x70 => op.sr = value & 0x1f,
                0x80 => {
                    op.sl = value >> 4;
                    op.rr = value & 0xf;
                }
                _ => {}
            }
            self.fm[ch].update_increments(1.0);
            return;
        }
        let channel = &mut self.fm[ch];
        match reg & 0xfc {
            0xa4 => channel.latch = value,
            0xa0 => {
                channel.fnum = (u16::from(channel.latch & 7) << 8) | u16::from(value);
                channel.block = (channel.latch >> 3) & 7;
                channel.update_increments(1.0);
            }
            0xb0 => {
                channel.algorithm = value & 7;
                channel.feedback = (value >> 3) & 7;
            }
            0xb4 => {
                channel.left = value & 0x80 != 0;
                channel.right = value & 0x40 != 0;
                channel.ams = (value >> 4) & 3;
                channel.pms = value & 7;
            }
            _ => {}
        }
    }

    fn write_ssg(&mut self, reg: u8, value: u8) {
        match reg {
            0..=5 => {
                let ch = usize::from(reg / 2);
                let lo = self.regs[0][usize::from(ch as u8 * 2)];
                let hi = self.regs[0][usize::from(ch as u8 * 2 + 1)] & 0xf;
                self.ssg[ch].period = (u16::from(hi) << 8) | u16::from(lo);
            }
            8..=10 => self.ssg[usize::from(reg - 8)].volume = value & 0x1f,
            13 => {
                self.env_step = 0;
                self.env_phase = 0.0;
                self.env_hold = false;
            }
            _ => {}
        }
    }

    fn write_rhythm(&mut self, reg: u8, value: u8) {
        match reg {
            0x10 => {
                for (i, voice) in self.rhythm.iter_mut().enumerate() {
                    if value & (1 << i) == 0 {
                        continue;
                    }
                    if value & 0x80 != 0 {
                        voice.playing = false;
                    } else {
                        voice.playing = true;
                        voice.pos = 0;
                    }
                }
            }
            0x11 => self.rhythm_total = value & 0x3f,
            0x18..=0x1d => {
                let voice = &mut self.rhythm[usize::from(reg - 0x18)];
                voice.level = value & 0x1f;
                voice.left = value & 0x80 != 0;
                voice.right = value & 0x40 != 0;
            }
            _ => {}
        }
    }

    fn ssg_envelope_level(&self) -> u8 {
        let shape = self.regs[0][13];
        let step = self.env_step.clamp(0, 31) as u8;
        let attack = shape & 4 != 0;
        let first_cycle = self.env_step < 32;
        let level = if first_cycle {
            if attack { step } else { 31 - step }
        } else if shape & 8 == 0 {
            0
        } else if shape & 1 != 0 {
            // hold
            let last = if attack { 31 } else { 0 };
            if shape & 2 != 0 { 31 - last } else { last }
        } else {
            let cycle = (self.env_step / 32) as u8;
            let mut up = attack;
            if shape & 2 != 0 && cycle & 1 == 1 {
                up = !up;
            }
            let s = (self.env_step % 32) as u8;
            if up { s } else { 31 - s }
        };
        level >> 1
    }

    /// Generates one stereo sample at [`RATE`].
    pub fn sample(&mut self) -> (f32, f32) {
        let tables = tables();
        // Envelope generator runs at a third of the sample rate.
        self.eg_div += 1;
        if self.eg_div >= 3 {
            self.eg_div = 0;
            self.eg_counter = self.eg_counter.wrapping_add(1);
            let counter = self.eg_counter;
            for ch in &mut self.fm {
                let keycode = ch.keycode();
                for op in &mut ch.ops {
                    op.clock_eg(counter, keycode);
                }
            }
        }
        // Hardware LFO.
        let lfo_reg = self.regs[0][0x22];
        let (am_value, pm_value) = if lfo_reg & 8 != 0 {
            self.lfo_phase += LFO_HZ[usize::from(lfo_reg & 7)] / RATE;
            if self.lfo_phase >= 1.0 {
                self.lfo_phase -= 1.0;
            }
            let tri = if self.lfo_phase < 0.5 {
                self.lfo_phase * 2.0
            } else {
                2.0 - self.lfo_phase * 2.0
            };
            (tri, (self.lfo_phase * std::f64::consts::TAU).sin())
        } else {
            (0.0, 0.0)
        };
        let mut left = 0.0f32;
        let mut right = 0.0f32;
        for ch in &mut self.fm {
            let am = (AMS_STEPS[usize::from(ch.ams)] * am_value) as i32;
            if lfo_reg & 8 != 0 && ch.pms != 0 {
                let cents = PMS_CENTS[usize::from(ch.pms)] * pm_value;
                ch.update_increments(2f64.powf(cents / 1200.0));
            }
            let out = ch.sample(am, tables) * 0.25;
            if ch.left {
                left += out;
            }
            if ch.right {
                right += out;
            }
        }
        // SSG.
        let noise_period = f64::from((self.regs[0][6] & 0x1f).max(1));
        self.noise_phase += SSG_BASE / noise_period / RATE;
        while self.noise_phase >= 1.0 {
            self.noise_phase -= 1.0;
            let bit = (self.noise_lfsr ^ (self.noise_lfsr >> 3)) & 1;
            self.noise_lfsr = (self.noise_lfsr >> 1) | (bit << 16);
        }
        let env_period = u16::from_le_bytes([self.regs[0][11], self.regs[0][12]]).max(1);
        if !self.env_hold {
            self.env_phase += SSG_BASE * 2.0 / f64::from(env_period) / 16.0 / RATE;
            while self.env_phase >= 1.0 {
                self.env_phase -= 1.0;
                self.env_step += 1;
                if self.env_step >= 64 {
                    self.env_step = 32;
                }
            }
        }
        let env_level = self.ssg_envelope_level();
        let noise = self.noise_lfsr & 1 != 0;
        let mixer = self.regs[0][7];
        let mut ssg_out = 0.0f32;
        for (i, ch) in self.ssg.iter_mut().enumerate() {
            let period = f64::from(ch.period.max(1));
            ch.phase += SSG_BASE / period / 2.0 / RATE;
            while ch.phase >= 1.0 {
                ch.phase -= 1.0;
                ch.high = !ch.high;
            }
            let tone_on = mixer & (1 << i) == 0;
            let noise_on = mixer & (8 << i) == 0;
            let gate = (ch.high || !tone_on) && (noise || !noise_on);
            if !gate || (!tone_on && !noise_on) {
                continue;
            }
            let level = if ch.volume & 0x10 != 0 {
                env_level
            } else {
                ch.volume & 0xf
            };
            if level == 0 {
                continue;
            }
            ssg_out += 2f32.powf((f32::from(level) - 15.0) / 2.0) * 0.14;
        }
        left += ssg_out;
        right += ssg_out;
        // Rhythm.
        let total = 63 - i32::from(self.rhythm_total);
        for (voice, sample) in self.rhythm.iter_mut().zip(&tables.rhythm) {
            if !voice.playing {
                continue;
            }
            let Some(value) = sample.get(voice.pos) else {
                voice.playing = false;
                continue;
            };
            voice.pos += 1;
            let att = (total + 31 - i32::from(voice.level)) * 8;
            let out = value * tables.gain[att.clamp(0, 0x1fff) as usize] * 0.5;
            if voice.left {
                left += out;
            }
            if voice.right {
                right += out;
            }
        }
        // Remove the DC offset of the unipolar SSG output.
        let mut out = [left, right];
        for (i, value) in out.iter_mut().enumerate() {
            let filtered = *value - self.dc_in[i] + 0.9995 * self.dc[i];
            self.dc_in[i] = *value;
            self.dc[i] = filtered;
            *value = filtered;
        }
        (out[0], out[1])
    }
}

/// Procedural stand-ins for the YM2608 rhythm ROM (BD, SD, TOP, HH, TOM, RIM).
fn rhythm_samples() -> [Vec<f32>; 6] {
    let rate = RATE;
    let mut seed = 0x1234_5678u32;
    let mut noise = move || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        (seed as f32 / u32::MAX as f32) * 2.0 - 1.0
    };
    let render = |seconds: f64, f: &mut dyn FnMut(f64) -> f32| -> Vec<f32> {
        (0..(seconds * rate) as usize)
            .map(|i| f(i as f64 / rate))
            .collect()
    };
    let mut phase = 0.0f64;
    let bd = render(0.3, &mut |t| {
        let freq = 50.0 + 120.0 * (-t * 30.0).exp();
        phase += freq / rate;
        ((phase * std::f64::consts::TAU).sin() * (-t * 12.0).exp()) as f32
    });
    let mut phase = 0.0f64;
    let mut lp = 0.0f32;
    let sd = render(0.2, &mut |t| {
        phase += 190.0 / rate;
        let n = noise();
        lp += (n - lp) * 0.6;
        let tone = (phase * std::f64::consts::TAU).sin() * (-t * 30.0).exp();
        (tone * 0.5) as f32 + lp * 0.7 * (-t * 18.0).exp() as f32
    });
    let mut prev = 0.0f32;
    let top = render(0.8, &mut |t| {
        let n = noise();
        let hp = n - prev;
        prev = n;
        hp * 0.5 * (-t * 5.0).exp() as f32
    });
    let mut prev = 0.0f32;
    let hh = render(0.1, &mut |t| {
        let n = noise();
        let hp = n - prev;
        prev = n;
        hp * 0.5 * (-t * 45.0).exp() as f32
    });
    let mut phase = 0.0f64;
    let tom = render(0.35, &mut |t| {
        let freq = 110.0 + 60.0 * (-t * 15.0).exp();
        phase += freq / rate;
        ((phase * std::f64::consts::TAU).sin() * (-t * 9.0).exp()) as f32
    });
    let mut phase = 0.0f64;
    let rim = render(0.05, &mut |t| {
        phase += 1700.0 / rate;
        ((phase * std::f64::consts::TAU).sin() * (-t * 90.0).exp()) as f32
    });
    [bd, sd, top, hh, tom, rim]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4_fnum_produces_440_hz() {
        let mut opna = Opna::new();
        // Pure sine: algorithm 7, only slot 4 audible, MUL=1.
        opna.write(0, 0xb0, 7);
        for slot_reg in [0x30u8, 0x34, 0x38, 0x3c] {
            opna.write(0, slot_reg, 1);
            opna.write(0, slot_reg + 0x10, 0x7f);
            opna.write(0, slot_reg + 0x20, 0x1f);
            opna.write(0, slot_reg + 0x50, 0x0f);
        }
        opna.write(0, 0x4c, 0);
        opna.write(0, 0xa4, (4 << 3) | 0x04);
        opna.write(0, 0xa0, 0x10);
        opna.write(0, 0x28, 0x80);
        let samples: Vec<f32> = (0..RATE as usize).map(|_| opna.sample().0).collect();
        let crossings = samples
            .windows(2)
            .filter(|w| w[0] <= 0.0 && w[1] > 0.0)
            .count();
        assert!((435..=445).contains(&crossings), "{crossings}");
    }
}
