//! KAJA's PMD music driver (v4.8, PC-9801-86 / YM2608 configuration).
//!
//! UK2 plays its `.MMM` scores through the resident `PMD.COM`.  This module
//! interprets the compiled PMD "M" data the same way the driver does on every
//! Timer-B interrupt and drives the [`Opna`] model.  FM parts A-F, SSG parts
//! G-I and the rhythm part K are supported; the ADPCM part J (which needs a
//! separate sample bank that the game does not ship) is ignored, as are the
//! FM3 extended parts and sound effects.

use super::opna::{Opna, RATE};

const FNUM_DATA: [u16; 12] = [
    0x026a, 0x028f, 0x02b6, 0x02df, 0x030b, 0x0339, 0x036a, 0x039e, 0x03d5, 0x0410, 0x044e, 0x048f,
];
const PSG_TUNE_DATA: [u16; 12] = [
    0x0ee8, 0x0e12, 0x0d48, 0x0c89, 0x0bd5, 0x0b2b, 0x0a8a, 0x09f3, 0x0964, 0x08dd, 0x085e, 0x07e6,
];
/// Carrier slots per algorithm (bit 7 = slot 4 ... bit 4 = slot 1).
const CARRIER_TABLE: [u8; 8] = [0x80, 0x80, 0x80, 0x80, 0xa0, 0xe0, 0xe0, 0xf0];

const PART_COUNT: usize = 11;
const PART_J: usize = 9;
const PART_K: usize = 10;

#[derive(Clone, Copy, Default)]
struct Lfo {
    delay: u8,
    speed: u8,
    step: u8,
    time: u8,
    delay2: u8,
    speed2: u8,
    step2: u8,
    time2: u8,
    mdepth: u8,
    mdspd: u8,
    mdspd2: u8,
    wave: u8,
    mdc: u8,
    mdc2: u8,
    dat: u16,
}

#[derive(Clone, Copy, Default)]
struct Part {
    address: usize,
    partloop: usize,
    leng: u8,
    qdat: u8,
    qdata: u8,
    qdatb: u8,
    qdat2: u8,
    qdat3: u8,
    keyoff_flag: u8,
    onkai: u8,
    onkai_def: u8,
    fnum: u16,
    detune: u16,
    porta_num: u16,
    porta_num2: u16,
    porta_num3: u16,
    volume: u8,
    volpush: u8,
    shift: u8,
    shift_def: u8,
    voicenum: u8,
    alg_fb: u8,
    slot: [u8; 4],
    carrier: u8,
    slotmask: u8,
    neiromask: u8,
    volmask: u8,
    volmask2: u8,
    partmask: u8,
    lfoswi: u8,
    extendmode: u8,
    lfo: [Lfo; 2],
    fmpan: u8,
    hldelay: u8,
    hldelay_c: u8,
    sdelay: u8,
    sdelay_c: u8,
    sdelay_m: u8,
    psgpat: u8,
    envf: u8,
    eenv_ar: u8,
    eenv_arc: u8,
    eenv_dr: u8,
    eenv_drc: u8,
    eenv_sr: u8,
    eenv_src: u8,
    eenv_rr: u8,
    eenv_rrc: u8,
    eenv_sl: u8,
    eenv_al: u8,
    eenv_count: u8,
    eenv_volume: u8,
}

/// What the driver does after a command byte.
enum Flow {
    Continue,
    /// A portamento note was set up; finish it like a normal note.
    PortaNote,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Fm,
    Ssg,
    Rhythm,
}

pub struct Pmd {
    pub opna: Opna,
    md: Vec<u8>,
    parts: [Part; PART_COUNT],
    prgdat: Option<usize>,
    radtbl: usize,
    rhyadr: Option<usize>,
    playing: bool,
    // Current part context.
    port: usize,
    partb: u8,
    tempo_d: u8,
    tempo_48: u8,
    tempo_d_push: u8,
    tempo_48_push: u8,
    tieflag: u8,
    volpush_flag: u8,
    lfo_switch: u8,
    fm_key: [u8; 6],
    psnoi: u8,
    psnoi_last: u8,
    rdat: [u8; 6],
    rhyvol: u8,
    fm_voldown: u8,
    ssg_voldown: u8,
    rhythm_voldown: u8,
    fadeout_volume: u8,
    fadeout_speed: u8,
    status: u8,
    syousetu_lng: u8,
    seed: u16,
    timer: u32,
    timer_a: u32,
}

impl Default for Pmd {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn word(md: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([
        md.get(at).copied().unwrap_or(0),
        md.get(at + 1).copied().unwrap_or(0),
    ])
}

impl Pmd {
    pub fn new() -> Self {
        let mut pmd = Self {
            opna: Opna::new(),
            md: vec![0x80],
            parts: [Part::default(); PART_COUNT],
            prgdat: None,
            radtbl: 0,
            rhyadr: None,
            playing: false,
            port: 0,
            partb: 1,
            tempo_d: 200,
            tempo_48: 0,
            tempo_d_push: 200,
            tempo_48_push: 0,
            tieflag: 0,
            volpush_flag: 0,
            lfo_switch: 0,
            fm_key: [0; 6],
            psnoi: 0,
            psnoi_last: 0,
            rdat: [0xcf; 6],
            rhyvol: 48,
            fm_voldown: 0,
            ssg_voldown: 0,
            rhythm_voldown: 0,
            fadeout_volume: 0,
            fadeout_speed: 0,
            status: 0,
            syousetu_lng: 96,
            seed: 0,
            timer: 0,
            timer_a: 0,
        };
        pmd.silence();
        pmd
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// `mstart`: begins playing a compiled PMD score (whole `.MMM` file).
    pub fn start(&mut self, file: &[u8]) {
        self.stop();
        if file.len() < 2 + 2 * PART_COUNT {
            return;
        }
        self.md = file[1..].to_vec();
        self.data_init();
        self.play_init();
        self.opn_init();
        self.tempo_d = 200;
        self.tempo_d_push = 200;
        self.calc_tb_tempo();
        self.syousetu_lng = 96;
        self.timer = 0;
        self.playing = true;
    }

    /// Driver function 2: fade out at `speed` volume steps per Timer-A tick.
    pub fn fade_out(&mut self, speed: u8) {
        if self.playing {
            self.fadeout_speed = speed;
        }
    }

    /// `mstop`.
    pub fn stop(&mut self) {
        self.playing = false;
        self.fadeout_speed = 0;
        self.fadeout_volume = 0xff;
        self.silence();
        self.fadeout_volume = 0;
    }

    /// Renders one stereo sample at [`RATE`], running Timer-B interrupts.
    pub fn sample(&mut self) -> (f32, f32) {
        if self.playing {
            if self.timer == 0 {
                self.mmain();
                self.timer = (256 - u32::from(self.tempo_d)) * 16;
            }
            self.timer -= 1;
            // Timer-A drives fade-outs (fixed 9216 us period).
            self.timer_a += 1;
            if self.timer_a >= (RATE * 0.009216) as u32 {
                self.timer_a = 0;
                self.fadeout();
            }
        }
        self.opna.sample()
    }

    // ---- chip access ----------------------------------------------------

    fn opnset(&mut self, reg: u8, value: u8) {
        self.opna.write(self.port, reg, value);
    }

    fn opnset44(&mut self, reg: u8, value: u8) {
        self.opna.write(0, reg, value);
    }

    fn silence(&mut self) {
        for port in 0..2 {
            for ch in 0..3u8 {
                for slot in 0..4u8 {
                    self.opna.write(port, 0x80 + slot * 4 + ch, 0xff);
                }
            }
        }
        for ch in [0u8, 1, 2, 4, 5, 6] {
            self.opna.write(0, 0x28, ch);
        }
        self.opna.write(0, 0x07, 0xbf);
        self.opna.write(0, 0x10, 0xff);
    }

    fn opn_init(&mut self) {
        self.opnset44(0x29, 0x83);
        self.psnoi = 0;
        self.opnset44(0x06, 0);
        self.psnoi_last = 0;
        for port in 0..2 {
            for reg in 0x90u8..0x9f {
                if reg & 3 != 3 {
                    self.opna.write(port, reg, 0);
                }
            }
            for ch in 0..3u8 {
                self.opna.write(port, 0xb4 + ch, 0xc0);
            }
        }
        self.opnset44(0x22, 0);
        self.rdat = [0xcf; 6];
        self.opnset44(0x10, 0xff);
        let mut level = 48u8;
        if self.rhythm_voldown != 0 {
            let scaled = u16::from(level << 2) * u16::from(self.rhythm_voldown.wrapping_neg());
            level = ((scaled >> 8) as u8) >> 2;
        }
        self.rhyvol = level;
        self.opnset44(0x11, level);
    }

    fn data_init(&mut self) {
        self.fadeout_volume = 0;
        self.fadeout_speed = 0;
        self.parts = [Part::default(); PART_COUNT];
        for part in &mut self.parts {
            part.onkai = 0xff;
            part.onkai_def = 0xff;
        }
        self.tieflag = 0;
        self.status = 0;
        self.fm_key = [0; 6];
        self.syousetu_lng = 96;
        self.volpush_flag = 0;
    }

    fn play_init(&mut self) {
        let table_size = 2 * (PART_COUNT + 1);
        self.prgdat = if usize::from(self.md[0]) != table_size {
            Some(usize::from(word(&self.md, table_size)))
        } else {
            None
        };
        for i in 0..PART_COUNT {
            let mut address = usize::from(word(&self.md, i * 2));
            if self.md.get(address).copied().unwrap_or(0x80) == 0x80 || i == PART_J {
                address = 0;
            }
            let part = &mut self.parts[i];
            part.address = address;
            part.leng = 1;
            part.keyoff_flag = 0xff;
            part.lfo[0].mdc = 0xff;
            part.lfo[0].mdc2 = 0xff;
            part.lfo[1].mdc = 0xff;
            part.lfo[1].mdc2 = 0xff;
            part.onkai = 0xff;
            part.onkai_def = 0xff;
            if i < 6 {
                part.volume = 108;
                part.fmpan = 0xc0;
                part.slotmask = 0xf0;
                part.neiromask = 0xff;
            } else if i < 9 {
                part.volume = 8;
                part.psgpat = 7;
                part.envf = 3;
            } else if i == PART_K {
                part.volume = 15;
            }
        }
        self.radtbl = usize::from(word(&self.md, PART_COUNT * 2));
        self.rhyadr = None;
    }

    // ---- main loop --------------------------------------------------------

    fn mmain(&mut self) {
        for i in 6..9 {
            self.select(i);
            self.psgmain(i);
        }
        for i in [3, 4, 5, 0, 1, 2] {
            self.select(i);
            self.fmmain(i);
        }
        self.port = 0;
        self.rhythmmain();
    }

    fn select(&mut self, i: usize) {
        self.partb = (i % 3) as u8 + 1;
        self.port = if (3..6).contains(&i) { 1 } else { 0 };
    }

    fn byte(&self, at: usize) -> u8 {
        self.md.get(at).copied().unwrap_or(0x80)
    }

    fn fmmain(&mut self, i: usize) {
        let mut si = self.parts[i].address;
        if si == 0 {
            return;
        }
        let p = &mut self.parts[i];
        p.leng = p.leng.wrapping_sub(1);
        let al = p.leng;
        if p.keyoff_flag & 3 == 0 && al <= p.qdat {
            self.keyoff(i);
            self.parts[i].keyoff_flag = 0xff;
        }
        if al != 0 {
            self.mpexit(i);
            return;
        }
        self.parts[i].lfoswi &= 0xf7;
        loop {
            let al = self.byte(si);
            si += 1;
            if al < 0x80 {
                let al = self.lfoinit(i, al);
                let al = self.oshift(i, al);
                self.fnumset(i, al);
                self.parts[i].leng = self.byte(si);
                si += 1;
                si = self.calc_q(i, si);
                self.porta_return(i, si);
                return;
            }
            if al == 0x80 {
                si -= 1;
                self.parts[i].address = si;
                self.parts[i].onkai = 0xff;
                let lp = self.parts[i].partloop;
                if lp == 0 {
                    self.mpexit(i);
                    return;
                }
                si = lp;
                continue;
            }
            match self.command(i, Kind::Fm, al, &mut si) {
                Flow::Continue => {}
                Flow::PortaNote => {
                    self.porta_return(i, si);
                    return;
                }
            }
        }
    }

    fn porta_return(&mut self, i: usize, si: usize) {
        self.volpush_check(i);
        self.volset(i);
        self.otodasi(i);
        self.keyon(i);
        self.note_started(i, si);
    }

    fn volpush_check(&mut self, i: usize) {
        let p = &mut self.parts[i];
        if p.volpush != 0 && p.onkai != 0xff {
            self.volpush_flag = self.volpush_flag.wrapping_sub(1);
            if self.volpush_flag != 0 {
                self.volpush_flag = 0;
                p.volpush = 0;
            }
        }
    }

    fn note_started(&mut self, i: usize, si: usize) {
        self.parts[i].address = si;
        self.tieflag = 0;
        self.volpush_flag = 0;
        self.parts[i].keyoff_flag = if self.byte(si) == 0xfb { 2 } else { 0 };
    }

    fn mpexit(&mut self, i: usize) {
        if self.parts[i].hldelay_c != 0 {
            self.parts[i].hldelay_c -= 1;
            if self.parts[i].hldelay_c == 0 {
                let pan = self.parts[i].fmpan;
                self.opnset(0xb4 + self.partb - 1, pan);
            }
        }
        if self.parts[i].sdelay_c != 0 {
            self.parts[i].sdelay_c -= 1;
            if self.parts[i].sdelay_c == 0 && self.parts[i].keyoff_flag & 1 == 0 {
                self.keyon(i);
            }
        }
        let cl = self.parts[i].lfoswi;
        if cl == 0 {
            if self.fadeout_speed != 0 {
                self.volset(i);
            }
            return;
        }
        self.run_lfos(i, cl);
        if self.lfo_switch & 0x19 != 0 {
            if self.lfo_switch & 8 != 0 {
                self.porta_calc(i);
            }
            self.otodasi(i);
        }
        if self.lfo_switch & 0x22 == 0 && self.fadeout_speed == 0 {
            return;
        }
        self.volset(i);
    }

    fn run_lfos(&mut self, i: usize, cl: u8) {
        self.lfo_switch = cl & 8;
        if cl & 3 != 0 && self.lfo(i) {
            self.lfo_switch |= cl & 3;
        }
        if cl & 0x30 != 0 {
            self.lfo_change(i);
            let changed = self.lfo(i);
            self.lfo_change(i);
            if changed {
                self.lfo_switch |= self.parts[i].lfoswi & 0x30;
            }
        }
    }

    fn calc_q(&mut self, i: usize, mut si: usize) -> usize {
        if self.byte(si) == 0xc1 {
            si += 1;
            self.parts[i].qdat = 0;
            return si;
        }
        let p = self.parts[i];
        let mut dl = p.qdata;
        if p.qdatb != 0 {
            dl = dl.wrapping_add(((u16::from(p.leng) * u16::from(p.qdatb)) >> 8) as u8);
        }
        if p.qdat3 != 0 {
            let r = self.rnd(u16::from(p.qdat3 & 0x7f) + 1) as u8;
            if p.qdat3 & 0x80 == 0 {
                dl = dl.wrapping_add(r);
            } else {
                dl = dl.saturating_sub(r);
            }
        }
        if p.qdat2 != 0 {
            let Some(dh) = p.leng.checked_sub(p.qdat2) else {
                self.parts[i].qdat = 0;
                return si;
            };
            dl = dl.min(dh);
        }
        self.parts[i].qdat = dl;
        si
    }

    fn psgmain(&mut self, i: usize) {
        let mut si = self.parts[i].address;
        if si == 0 {
            return;
        }
        let p = &mut self.parts[i];
        p.leng = p.leng.wrapping_sub(1);
        let al = p.leng;
        if p.keyoff_flag & 3 == 0 && al <= p.qdat {
            self.keyoffp(i);
            self.parts[i].keyoff_flag = 0xff;
        }
        if al != 0 {
            self.mpexitp(i);
            return;
        }
        self.parts[i].lfoswi &= 0xf7;
        loop {
            let al = self.byte(si);
            si += 1;
            if al < 0x80 {
                let al = self.lfoinitp(i, al);
                let al = self.oshift(i, al);
                self.fnumsetp(i, al);
                self.parts[i].leng = self.byte(si);
                si += 1;
                si = self.calc_q(i, si);
                self.porta_returnp(i, si);
                return;
            }
            if al == 0x80 {
                si -= 1;
                self.parts[i].address = si;
                self.parts[i].onkai = 0xff;
                let lp = self.parts[i].partloop;
                if lp == 0 {
                    self.mpexitp(i);
                    return;
                }
                si = lp;
                continue;
            }
            match self.command(i, Kind::Ssg, al, &mut si) {
                Flow::Continue => {}
                Flow::PortaNote => {
                    self.porta_returnp(i, si);
                    return;
                }
            }
        }
    }

    fn porta_returnp(&mut self, i: usize, si: usize) {
        self.volpush_check(i);
        self.volsetp(i);
        self.otodasip(i);
        self.keyonp(i);
        self.note_started(i, si);
    }

    fn mpexitp(&mut self, i: usize) {
        let cl = self.parts[i].lfoswi;
        self.lfo_switch = cl & 8;
        if cl != 0 {
            self.run_lfos(i, cl);
            if self.lfo_switch & 0x19 != 0 {
                if self.lfo_switch & 8 != 0 {
                    self.porta_calc(i);
                }
                self.otodasip(i);
            }
        }
        let changed = self.soft_env(i);
        if changed || self.lfo_switch & 0x22 != 0 || self.fadeout_speed != 0 {
            self.volsetp(i);
        }
    }

    fn rhythmmain(&mut self) {
        let i = PART_K;
        let mut si = self.parts[i].address;
        if si == 0 {
            return;
        }
        self.parts[i].leng = self.parts[i].leng.wrapping_sub(1);
        if self.parts[i].leng != 0 {
            return;
        }
        // Continue the current R pattern, if any.
        if let Some(bx) = self.rhyadr
            && self.rhythm_pattern(bx)
        {
            return;
        }
        loop {
            let al = self.byte(si);
            si += 1;
            if al == 0x80 {
                si -= 1;
                self.parts[i].address = si;
                let lp = self.parts[i].partloop;
                if lp == 0 {
                    self.rhyadr = None;
                    return;
                }
                si = lp;
                continue;
            }
            if al < 0x80 {
                self.parts[i].address = si;
                let at = self.radtbl + usize::from(al) * 2;
                let bx = usize::from(word(&self.md, at));
                if self.rhythm_pattern(bx) {
                    return;
                }
                continue;
            }
            let _ = self.command(i, Kind::Rhythm, al, &mut si);
            self.parts[i].address = si;
        }
    }

    /// Plays R pattern data from `bx`; returns `false` at its end marker.
    fn rhythm_pattern(&mut self, mut bx: usize) -> bool {
        loop {
            let al = self.byte(bx);
            bx += 1;
            if al == 0xff {
                self.rhyadr = None;
                return false;
            }
            if al & 0x80 != 0 {
                if al & 0x40 != 0 {
                    let _ = self.command(PART_K, Kind::Rhythm, al, &mut bx);
                    continue;
                }
                let shot = (u16::from(al & 0x3f) << 8) | u16::from(self.byte(bx));
                bx += 1;
                if shot != 0 && self.parts[PART_K].partmask == 0 {
                    let keys = (shot & 0x3f) as u8;
                    if keys != 0 {
                        for (n, level) in self.rdat.iter().enumerate() {
                            if keys & (1 << n) != 0 {
                                self.opna.write(0, 0x18 + n as u8, *level);
                            }
                        }
                        self.opnset44(0x10, keys);
                    }
                }
            }
            self.parts[PART_K].leng = self.byte(bx);
            bx += 1;
            self.rhyadr = Some(bx);
            return true;
        }
    }

    // ---- pitch --------------------------------------------------------------

    fn oshift(&self, i: usize, al: u8) -> u8 {
        if al == 0x0f {
            return al;
        }
        let p = &self.parts[i];
        let dl = p.shift.wrapping_add(p.shift_def) as i8;
        if dl == 0 {
            return al;
        }
        let mut bh = (al >> 4) as i32;
        let mut bl = i32::from(al & 0xf) + i32::from(dl);
        while bl < 0 {
            bh -= 1;
            bl += 12;
        }
        while bl >= 12 {
            bh += 1;
            bl -= 12;
        }
        (((bh as u8) & 0xf) << 4) | bl as u8
    }

    fn fnrest(&mut self, i: usize) {
        let p = &mut self.parts[i];
        p.onkai = 0xff;
        if p.lfoswi & 0x11 == 0 {
            p.fnum = 0;
        }
    }

    fn fnumset(&mut self, i: usize, al: u8) {
        if al & 0xf == 0xf {
            self.fnrest(i);
            return;
        }
        let p = &mut self.parts[i];
        p.onkai = al;
        let block = u16::from((al >> 1) & 0x38);
        let fnum = FNUM_DATA.get(usize::from(al & 0xf)).copied().unwrap_or(0);
        p.fnum = fnum | (block << 8);
    }

    fn fnumsetp(&mut self, i: usize, al: u8) {
        if al & 0xf == 0xf {
            self.fnrest(i);
            return;
        }
        let p = &mut self.parts[i];
        p.onkai = al;
        let octave = u32::from(al >> 4);
        let tune = PSG_TUNE_DATA
            .get(usize::from(al & 0xf))
            .copied()
            .unwrap_or(0);
        let round = octave > 0 && tune & (1 << (octave - 1)) != 0;
        let mut value = tune.checked_shr(octave).unwrap_or(0);
        if round {
            value += 1;
        }
        p.fnum = value;
    }

    fn otodasi(&mut self, i: usize) {
        let p = self.parts[i];
        if p.fnum == 0 || p.slotmask == 0 {
            return;
        }
        let mut cx = p.fnum & 0x3800;
        let mut ax = p.fnum & 0x7ff;
        ax = ax.wrapping_add(p.porta_num).wrapping_add(p.detune);
        if p.lfoswi & 1 != 0 {
            ax = ax.wrapping_add(p.lfo[0].dat);
        }
        if p.lfoswi & 0x10 != 0 {
            ax = ax.wrapping_add(p.lfo[1].dat);
        }
        fm_block_calc(&mut cx, &mut ax);
        let value = ax | cx;
        let partb = self.partb;
        self.opnset(0xa4 + partb - 1, (value >> 8) as u8);
        self.opnset(0xa0 + partb - 1, value as u8);
    }

    fn otodasip(&mut self, i: usize) {
        let p = self.parts[i];
        if p.fnum == 0 {
            return;
        }
        let mut ax = p.fnum.wrapping_add(p.porta_num);
        if p.extendmode & 1 == 0 {
            ax = ax.wrapping_sub(p.detune);
            if p.lfoswi & 1 != 0 {
                ax = ax.wrapping_sub(p.lfo[0].dat);
            }
            if p.lfoswi & 0x10 != 0 {
                ax = ax.wrapping_sub(p.lfo[1].dat);
            }
        } else {
            let scaled = |ax: u16, by: u16| -> u16 {
                let ans = (i32::from(ax as i16) * i32::from(by as i16)) << 4;
                let mut dx = (ans >> 16) as u16;
                dx = if ans >= 0 {
                    dx.wrapping_add(1)
                } else {
                    dx.wrapping_sub(1)
                };
                dx
            };
            if p.detune != 0 {
                ax = ax.wrapping_sub(scaled(ax, p.detune));
            }
            let mut dx = 0u16;
            if p.lfoswi & 1 != 0 {
                dx = p.lfo[0].dat;
            }
            if p.lfoswi & 0x10 != 0 {
                dx = dx.wrapping_add(p.lfo[1].dat);
            }
            if p.lfoswi & 0x11 != 0 && dx != 0 {
                ax = ax.wrapping_sub(scaled(ax, dx));
            }
        }
        if ax >= 0x1000 {
            ax = if ax & 0x8000 != 0 { 0 } else { 0xfff };
        }
        let reg = (self.partb - 1) * 2;
        self.opnset44(reg, ax as u8);
        self.opnset44(reg + 1, (ax >> 8) as u8);
    }

    fn porta_calc(&mut self, i: usize) {
        let p = &mut self.parts[i];
        p.porta_num = p.porta_num.wrapping_add(p.porta_num2);
        if p.porta_num3 == 0 {
            return;
        }
        if p.porta_num3 & 0x8000 != 0 {
            p.porta_num3 = p.porta_num3.wrapping_add(1);
            p.porta_num = p.porta_num.wrapping_sub(1);
        } else {
            p.porta_num3 -= 1;
            p.porta_num = p.porta_num.wrapping_add(1);
        }
    }

    // ---- volume -----------------------------------------------------------

    fn volset(&mut self, i: usize) {
        let p = self.parts[i];
        if p.slotmask == 0 {
            return;
        }
        let mut cl = if p.volpush != 0 {
            p.volpush - 1
        } else {
            p.volume
        };
        if self.fm_voldown != 0 {
            cl = ((u16::from(self.fm_voldown.wrapping_neg()) * u16::from(cl)) >> 8) as u8;
        }
        if self.fadeout_volume >= 2 {
            let al = (self.fadeout_volume >> 1).wrapping_neg();
            cl = ((u16::from(al) * u16::from(cl)) >> 8) as u8;
        }
        self.fmvs(i, cl);
    }

    fn fmvs(&mut self, i: usize, vol: u8) {
        let p = self.parts[i];
        let slotmask = p.slotmask;
        let mut vol_tbl = [0x80u8; 4];
        let cl = !vol;
        let carriers = slotmask & p.carrier;
        let mut bh = carriers;
        for (k, slot) in vol_tbl.iter_mut().enumerate() {
            if carriers & (0x80 >> k) != 0 {
                *slot = cl;
            }
        }
        if cl != 255 {
            let apply = |vol_tbl: &mut [u8; 4], mask: u8, al: u8| {
                for (k, slot) in vol_tbl.iter_mut().enumerate() {
                    if mask & (0x80 >> k) == 0 {
                        continue;
                    }
                    if al & 0x80 == 0 {
                        *slot = slot.saturating_sub(al);
                    } else {
                        let (value, borrow) = slot.overflowing_sub(al);
                        *slot = if borrow { value } else { 0xff };
                    }
                }
            };
            if p.lfoswi & 2 != 0 {
                let mask = p.volmask & slotmask;
                bh |= mask;
                apply(&mut vol_tbl, mask, p.lfo[0].dat as u8);
            }
            if p.lfoswi & 0x20 != 0 {
                let mask = p.volmask2 & slotmask;
                bh |= mask;
                apply(&mut vol_tbl, mask, p.lfo[1].dat as u8);
            }
        }
        let base = self.partb - 1;
        // slot4 (4Ch), slot3 (44h), slot2 (48h), slot1 (40h)
        let order = [(0x4c, 3usize), (0x44, 2), (0x48, 1), (0x40, 0)];
        for (k, (reg, slot)) in order.into_iter().enumerate() {
            if bh & (0x80 >> k) == 0 {
                continue;
            }
            let (sum, carry) = vol_tbl[k].overflowing_add(p.slot[slot]);
            let sum = if carry { 255 } else { sum };
            let value = sum.saturating_sub(0x80);
            self.opnset(reg + base, value);
        }
    }

    fn volsetp(&mut self, i: usize) {
        let p = self.parts[i];
        if p.envf == 3 || (p.envf == 0xff && p.eenv_count == 0) {
            return;
        }
        let mut dl = if p.volpush != 0 {
            p.volpush - 1
        } else {
            p.volume
        };
        if self.ssg_voldown != 0 {
            dl = ((u16::from(self.ssg_voldown.wrapping_neg()) * u16::from(dl)) >> 8) as u8;
        }
        if self.fadeout_volume != 0 {
            dl = ((u16::from(self.fadeout_volume.wrapping_neg()) * u16::from(dl)) >> 8) as u8;
        }
        'out: {
            if dl == 0 {
                break 'out;
            }
            if p.envf == 0xff {
                if p.eenv_volume == 0 {
                    dl = 0;
                    break 'out;
                }
                let product = u16::from(dl) * u16::from(p.eenv_volume + 1);
                let v = (product as u8) >> 3;
                dl = (v >> 1) + (v & 1);
            } else {
                let v = dl.wrapping_add(p.eenv_volume);
                if v & 0x80 != 0 || v == 0 {
                    dl = 0;
                    break 'out;
                }
                dl = v.min(15);
            }
            if p.lfoswi & 0x22 == 0 {
                break 'out;
            }
            let mut ax = 0u16;
            if p.lfoswi & 2 != 0 {
                ax = p.lfo[0].dat;
            }
            if p.lfoswi & 0x20 != 0 {
                ax = ax.wrapping_add(p.lfo[1].dat);
            }
            let dx = u16::from(dl).wrapping_add(ax);
            dl = if dx & 0x8000 != 0 {
                0
            } else if dx >= 16 {
                15
            } else {
                dx as u8
            };
        }
        let reg = 8 + self.partb - 1;
        self.opnset44(reg, dl);
    }

    // ---- key on/off -----------------------------------------------------

    fn keyon(&mut self, i: usize) {
        let p = self.parts[i];
        if p.onkai == 0xff {
            return;
        }
        let ch = self.partb - 1;
        let index = usize::from(ch) + self.port * 3;
        let mut al = self.fm_key[index] | p.slotmask;
        if p.sdelay_c != 0 {
            al &= p.sdelay_m;
        }
        self.fm_key[index] = al;
        let port_bit = if self.port == 1 { 4 } else { 0 };
        self.opnset44(0x28, ch | al | port_bit);
    }

    fn keyoff(&mut self, i: usize) {
        if self.parts[i].onkai == 0xff {
            return;
        }
        self.kof1(i);
    }

    fn kof1(&mut self, i: usize) {
        let ch = self.partb - 1;
        let index = usize::from(ch) + self.port * 3;
        let al = self.fm_key[index] & !self.parts[i].slotmask;
        self.fm_key[index] = al;
        let port_bit = if self.port == 1 { 4 } else { 0 };
        self.opnset44(0x28, ch | al | port_bit);
    }

    fn psgmsk(&self) -> (u8, u8) {
        let bit = 1u8 << (self.partb - 1);
        (self.opna.reg(0, 7), bit | (bit << 3))
    }

    fn keyonp(&mut self, i: usize) {
        let p = self.parts[i];
        if p.onkai == 0xff {
            return;
        }
        let (al, ah) = self.psgmsk();
        let value = (al | ah) & !(ah & p.psgpat);
        self.opnset44(7, value);
        if self.psnoi != self.psnoi_last {
            self.opnset44(6, self.psnoi);
            self.psnoi_last = self.psnoi;
        }
    }

    fn keyoffp(&mut self, i: usize) {
        let p = &mut self.parts[i];
        if p.onkai == 0xff {
            return;
        }
        if p.envf == 0xff {
            p.eenv_count = 4;
        } else {
            p.envf = 2;
        }
    }

    // ---- voices -----------------------------------------------------------

    fn toneadr_calc(&self, voice: u8) -> Option<usize> {
        let mut bx = self.prgdat?;
        while bx + 26 <= self.md.len() {
            if self.md[bx] == voice {
                return Some(bx + 1);
            }
            bx += 26;
        }
        None
    }

    fn silence_fmpart(&mut self, i: usize) -> bool {
        let mask = self.parts[i].neiromask;
        if mask == 0 {
            return false;
        }
        let mut reg = 0x40 + self.partb - 1;
        for k in 0..4 {
            if mask & (0x80 >> k) != 0 {
                self.opnset(reg, 127);
                self.opnset(reg + 0x40, 127);
            }
            reg += 4;
        }
        self.kof1(i);
        true
    }

    fn neiroset(&mut self, i: usize, voice: u8) {
        let Some(bx) = self.toneadr_calc(voice) else {
            return;
        };
        if !self.silence_fmpart(i) {
            self.neiroset_tl(i, bx + 4);
            return;
        }
        let alg_fb = self.byte(bx + 24);
        self.opnset(0xb0 + self.partb - 1, alg_fb);
        self.parts[i].alg_fb = alg_fb;
        let carrier = CARRIER_TABLE[usize::from(alg_fb & 7)];
        let p = &mut self.parts[i];
        if p.volmask & 0xf == 0 {
            p.volmask = carrier;
        }
        if p.volmask2 & 0xf == 0 {
            p.volmask2 = carrier;
        }
        p.carrier = carrier;
        let mask = p.neiromask;
        let mut reg = 0x30 + self.partb - 1;
        for k in 0..24 {
            if mask & (0x80 >> (k % 8)) != 0 {
                let value = self.byte(bx + k);
                self.opnset(reg, value);
            }
            reg += 4;
        }
        self.neiroset_tl(i, bx + 4);
    }

    fn neiroset_tl(&mut self, i: usize, bx: usize) {
        let s1 = self.byte(bx);
        let s3 = self.byte(bx + 1);
        let s2 = self.byte(bx + 2);
        let s4 = self.byte(bx + 3);
        self.parts[i].slot = [s1, s2, s3, s4];
    }

    // ---- LFO ----------------------------------------------------------------

    fn lfo_change(&mut self, i: usize) {
        let p = &mut self.parts[i];
        p.lfo.swap(0, 1);
        p.lfoswi = p.lfoswi.rotate_left(4);
        p.extendmode = p.extendmode.rotate_left(4);
    }

    fn lfoinit_main(&mut self, i: usize) {
        let lfo = &mut self.parts[i].lfo[0];
        lfo.dat = 0;
        lfo.delay = lfo.delay2;
        lfo.speed = lfo.speed2;
        lfo.step = lfo.step2;
        lfo.time = lfo.time2;
        lfo.mdc = lfo.mdc2;
        if lfo.wave == 2 || lfo.wave == 3 {
            lfo.speed = 1;
        } else {
            lfo.speed = lfo.speed.wrapping_add(1);
        }
    }

    /// Runs LFO 1 once; returns whether its output changed.
    fn lfo(&mut self, i: usize) -> bool {
        let lfo = &mut self.parts[i].lfo[0];
        if lfo.delay != 0 {
            lfo.delay -= 1;
            return false;
        }
        let old = lfo.dat;
        self.lfo_main(i);
        self.parts[i].lfo[0].dat != old
    }

    fn lfo_main(&mut self, i: usize) {
        let mut rnd_value = None;
        {
            let lfo = &self.parts[i].lfo[0];
            if lfo.speed != 1 && lfo.wave == 3 {
                // handled below
            } else if lfo.speed == 1 && lfo.wave == 3 {
                let step = (lfo.step as i8).unsigned_abs();
                let ax = u16::from(step) * u16::from(lfo.time);
                rnd_value = Some(ax);
            }
        }
        let rnd = rnd_value.map(|ax| (ax, self.rnd(ax.wrapping_add(ax))));
        let lfo = &mut self.parts[i].lfo[0];
        if lfo.speed != 1 {
            if lfo.speed != 0xff {
                lfo.speed = lfo.speed.wrapping_sub(1);
            }
            return;
        }
        lfo.speed = lfo.speed2;
        let wave = lfo.wave;
        let mut md_inc = false;
        match wave {
            0 | 4 | 5 => {
                let delta = if wave == 5 {
                    let step = lfo.step as i8;
                    (i16::from(step) * i16::from(step.unsigned_abs())) as u16
                } else {
                    i16::from(lfo.step as i8) as u16
                };
                lfo.dat = lfo.dat.wrapping_add(delta);
                if lfo.dat == 0 {
                    md_inc = true;
                }
                let time = lfo.time;
                if time != 255 {
                    let t = time.wrapping_sub(1);
                    if t == 0 {
                        let mut reload = lfo.time2;
                        if wave != 4 {
                            reload = reload.wrapping_add(reload);
                        }
                        lfo.time = reload;
                        lfo.step = lfo.step.wrapping_neg();
                    } else {
                        lfo.time = t;
                    }
                }
            }
            1 => {
                lfo.dat = lfo.dat.wrapping_add(i16::from(lfo.step as i8) as u16);
                let time = lfo.time;
                let mut t = time;
                if time != 0xff {
                    t = time.wrapping_sub(1);
                    if t == 0 {
                        lfo.dat = lfo.dat.wrapping_neg();
                        md_inc = true;
                        t = lfo.time2.wrapping_add(lfo.time2);
                    }
                }
                lfo.time = t;
            }
            2 => {
                lfo.dat = (i16::from(lfo.step as i8) * i16::from(lfo.time as i8)) as u16;
                md_inc = true;
                lfo.step = lfo.step.wrapping_neg();
            }
            6 => {
                if lfo.time != 0 {
                    if lfo.time != 0xff {
                        lfo.time -= 1;
                    }
                    lfo.dat = lfo.dat.wrapping_add(i16::from(lfo.step as i8) as u16);
                }
            }
            _ => {
                if let Some((ax, value)) = rnd {
                    lfo.dat = value.wrapping_sub(ax);
                }
                md_inc = true;
            }
        }
        if md_inc {
            self.md_inc(i);
        }
    }

    fn md_inc(&mut self, i: usize) {
        let lfo = &mut self.parts[i].lfo[0];
        lfo.mdspd = lfo.mdspd.wrapping_sub(1);
        if lfo.mdspd != 0 {
            return;
        }
        lfo.mdspd = lfo.mdspd2;
        if lfo.mdc == 0 {
            return;
        }
        if lfo.mdc & 0x80 == 0 {
            lfo.mdc -= 1;
        }
        let step = lfo.step;
        let depth = lfo.mdepth;
        if step & 0x80 != 0 {
            let al = step.wrapping_neg().wrapping_add(depth);
            lfo.step = if al & 0x80 != 0 {
                if depth & 0x80 != 0 { 0 } else { 0x81 }
            } else {
                al.wrapping_neg()
            };
        } else {
            let al = step.wrapping_add(depth);
            lfo.step = if al & 0x80 != 0 {
                if depth & 0x80 != 0 { 0 } else { 0x7f }
            } else {
                al
            };
        }
    }

    fn rnd(&mut self, max: u16) -> u16 {
        let ax = 259u16.wrapping_mul(self.seed).wrapping_add(3) & 0x7fff;
        self.seed = ax;
        ((u32::from(ax) * u32::from(max)) / 32767) as u16
    }

    fn lfoinit(&mut self, i: usize, mut al: u8) -> u8 {
        let mut ah = al & 0xf;
        if ah == 0xc {
            al = self.parts[i].onkai_def;
            ah = al & 0xf;
        }
        self.parts[i].onkai_def = al;
        if ah == 0xf {
            self.lfo_exit(i);
            return al;
        }
        self.parts[i].porta_num = 0;
        if self.tieflag & 1 == 0 {
            self.lfin1(i);
        } else {
            self.lfo_exit(i);
        }
        al
    }

    fn lfoinitp(&mut self, i: usize, mut al: u8) -> u8 {
        let mut ah = al & 0xf;
        if ah == 0xc {
            al = self.parts[i].onkai_def;
            ah = al & 0xf;
        }
        self.parts[i].onkai_def = al;
        if ah != 0xf {
            self.parts[i].porta_num = 0;
            if self.tieflag & 1 == 0 {
                self.seinit(i);
                return al;
            }
        }
        self.soft_env(i);
        self.lfo_exit(i);
        al
    }

    fn lfo_exit(&mut self, i: usize) {
        if self.parts[i].lfoswi & 3 != 0 {
            self.lfo(i);
        }
        if self.parts[i].lfoswi & 0x30 != 0 {
            self.lfo_change(i);
            self.lfo(i);
            self.lfo_change(i);
        }
    }

    fn lfin1(&mut self, i: usize) {
        let p = &mut self.parts[i];
        p.hldelay_c = p.hldelay;
        if p.hldelay != 0 && i < 6 {
            let pan = p.fmpan & 0xc0;
            self.opnset(0xb4 + self.partb - 1, pan);
        }
        let p = &mut self.parts[i];
        p.sdelay_c = p.sdelay;
        let cl = p.lfoswi;
        if cl & 3 != 0 {
            if cl & 4 == 0 {
                self.lfoinit_main(i);
            }
            self.lfo(i);
        }
        if cl & 0x30 != 0 {
            self.lfo_change(i);
            if cl & 0x40 == 0 {
                self.lfoinit_main(i);
            }
            self.lfo(i);
            self.lfo_change(i);
        }
    }

    // ---- SSG software envelope --------------------------------------------

    fn seinit(&mut self, i: usize) {
        let p = &mut self.parts[i];
        if p.envf == 0xff {
            p.eenv_arc = p.eenv_ar.wrapping_sub(16);
            let dr = p.eenv_dr.wrapping_sub(16);
            p.eenv_drc = if dr & 0x80 != 0 {
                dr.wrapping_add(dr)
            } else {
                dr
            };
            let sr = p.eenv_sr.wrapping_sub(16);
            p.eenv_src = if sr & 0x80 != 0 {
                sr.wrapping_add(sr)
            } else {
                sr
            };
            p.eenv_rrc = p.eenv_rr.wrapping_add(p.eenv_rr).wrapping_sub(16);
            p.eenv_volume = p.eenv_al;
            p.eenv_count = 1;
            self.ext_ssgenv_main(i);
        } else {
            p.envf = 0;
            p.eenv_volume = 0;
            p.eenv_ar = p.eenv_arc;
            if p.eenv_ar == 0 {
                p.envf = 1;
                p.eenv_volume = p.eenv_dr;
            }
            p.eenv_sr = p.eenv_src;
            p.eenv_rr = p.eenv_rrc;
        }
        self.lfin1(i);
    }

    /// Returns whether the envelope level changed.
    fn soft_env(&mut self, i: usize) -> bool {
        if self.parts[i].envf == 0xff {
            return self.ext_ssgenv_main(i);
        }
        let p = &mut self.parts[i];
        let old = p.eenv_volume;
        let clamp = |p: &mut Part| {
            let v = p.eenv_volume;
            if (15..0xf1).contains(&v) {
                p.eenv_volume = 0xf1;
            }
        };
        match p.envf {
            0 => {
                p.eenv_ar = p.eenv_ar.wrapping_sub(1);
                if p.eenv_ar == 0 {
                    p.envf = 1;
                    p.eenv_volume = p.eenv_dr;
                }
            }
            2 => {
                if p.eenv_rr == 0 {
                    p.eenv_volume = 0xf1;
                } else {
                    p.eenv_rr -= 1;
                    if p.eenv_rr == 0 {
                        p.eenv_rr = p.eenv_rrc;
                        p.eenv_volume = p.eenv_volume.wrapping_sub(1);
                        clamp(p);
                    }
                }
            }
            _ => {
                if p.eenv_sr != 0 {
                    p.eenv_sr -= 1;
                    if p.eenv_sr == 0 {
                        p.eenv_sr = p.eenv_src;
                        p.eenv_volume = p.eenv_volume.wrapping_sub(1);
                        clamp(p);
                    }
                }
            }
        }
        p.eenv_volume != old
    }

    fn ext_ssgenv_main(&mut self, i: usize) -> bool {
        let p = &mut self.parts[i];
        if p.eenv_count == 0 {
            return false;
        }
        let old = p.eenv_volume;
        match p.eenv_count {
            1 => {
                let arc = p.eenv_arc;
                if arc.wrapping_sub(1) & 0x80 != 0 {
                    if p.eenv_ar != 0 {
                        p.eenv_arc = p.eenv_arc.wrapping_add(1);
                    }
                } else {
                    p.eenv_volume = p.eenv_volume.wrapping_add(arc);
                    if p.eenv_volume >= 15 {
                        p.eenv_volume = 15;
                        p.eenv_count += 1;
                        if p.eenv_sl == 15 {
                            p.eenv_count += 1;
                        }
                    } else {
                        p.eenv_arc = p.eenv_ar.wrapping_sub(16);
                    }
                }
            }
            2 => {
                let drc = p.eenv_drc;
                if drc.wrapping_sub(1) & 0x80 != 0 {
                    if p.eenv_dr != 0 {
                        p.eenv_drc = p.eenv_drc.wrapping_add(1);
                    }
                } else {
                    let (v, borrow) = p.eenv_volume.overflowing_sub(drc);
                    if borrow || v < p.eenv_sl {
                        p.eenv_volume = p.eenv_sl;
                        p.eenv_count += 1;
                    } else {
                        p.eenv_volume = v;
                        let dr = p.eenv_dr.wrapping_sub(16);
                        p.eenv_drc = if dr & 0x80 != 0 {
                            dr.wrapping_add(dr)
                        } else {
                            dr
                        };
                    }
                }
            }
            3 => {
                let src = p.eenv_src;
                if src.wrapping_sub(1) & 0x80 != 0 {
                    if p.eenv_sr != 0 {
                        p.eenv_src = p.eenv_src.wrapping_add(1);
                    }
                } else {
                    p.eenv_volume = p.eenv_volume.saturating_sub(src);
                    let sr = p.eenv_sr.wrapping_sub(16);
                    p.eenv_src = if sr & 0x80 != 0 {
                        sr.wrapping_add(sr)
                    } else {
                        sr
                    };
                }
            }
            _ => {
                let rrc = p.eenv_rrc;
                if rrc.wrapping_sub(1) & 0x80 != 0 {
                    if p.eenv_rr != 0 {
                        p.eenv_rrc = p.eenv_rrc.wrapping_add(1);
                    }
                } else {
                    p.eenv_volume = p.eenv_volume.saturating_sub(rrc);
                    p.eenv_rrc = p.eenv_rr.wrapping_add(p.eenv_rr).wrapping_sub(16);
                }
            }
        }
        p.eenv_volume != old
    }

    // ---- tempo / fade ---------------------------------------------------------

    fn calc_tb_tempo(&mut self) {
        let bl = 0u8.wrapping_sub(self.tempo_d);
        let al = if bl < 18 {
            255
        } else {
            let quotient = (0x112c / u16::from(bl)) as u8;
            let remainder = (0x112c % u16::from(bl)) as u8;
            if remainder & 0x80 != 0 {
                quotient.wrapping_add(1)
            } else {
                quotient
            }
        };
        self.tempo_48 = al;
        self.tempo_48_push = al;
    }

    fn calc_tempo_tb(&mut self) {
        let bl = self.tempo_48;
        let al = if bl < 18 {
            0
        } else {
            let quotient = (0x112c / u16::from(bl)) as u8;
            let remainder = (0x112c % u16::from(bl)) as u8;
            let value = quotient.wrapping_neg();
            if remainder & 0x80 != 0 {
                value.wrapping_sub(1)
            } else {
                value
            }
        };
        self.tempo_d = al;
        self.tempo_d_push = al;
    }

    fn comt(&mut self, si: &mut usize) {
        let al = self.byte(*si);
        *si += 1;
        if al < 251 {
            self.tempo_d = al;
            self.tempo_d_push = al;
            self.calc_tb_tempo();
            return;
        }
        let arg = self.byte(*si);
        *si += 1;
        match al {
            0xff => {
                self.tempo_48 = arg.max(18);
                self.tempo_48_push = self.tempo_48;
                self.calc_tempo_tb();
            }
            0xfe => {
                let base = i16::from(self.tempo_d_push);
                let value = (base + i16::from(arg as i8)).clamp(0, 250) as u8;
                self.tempo_d = value;
                self.tempo_d_push = value;
                self.calc_tb_tempo();
            }
            _ => {
                let base = i16::from(self.tempo_48_push);
                let value = (base + i16::from(arg as i8)).clamp(18, 255) as u8;
                self.tempo_48 = value;
                self.tempo_48_push = value;
                self.calc_tempo_tb();
            }
        }
    }

    fn fadeout(&mut self) {
        let speed = self.fadeout_speed;
        if speed == 0 {
            return;
        }
        let (value, carry) = speed.overflowing_add(self.fadeout_volume);
        if speed & 0x80 == 0 {
            if carry {
                self.fadeout_volume = 255;
                self.fadeout_speed = 0;
                // PMD.COM defaults to stopping the music after a fade-out.
                self.stop();
            } else {
                self.fadeout_volume = value;
            }
        } else if carry {
            self.fadeout_volume = value;
        } else {
            self.fadeout_volume = 0;
            self.fadeout_speed = 0;
            self.volset2r(self.rhyvol);
        }
    }

    // ---- commands -------------------------------------------------------------

    fn arg(&self, si: &mut usize) -> u8 {
        let value = self.byte(*si);
        *si += 1;
        value
    }

    fn arg_word(&self, si: &mut usize) -> u16 {
        let value = word(&self.md, *si);
        *si += 2;
        value
    }

    fn command(&mut self, i: usize, kind: Kind, cmd: u8, si: &mut usize) -> Flow {
        use Kind::*;
        let fm = kind == Fm;
        let ssg = kind == Ssg;
        let rhythm = kind == Rhythm;
        let skip = |si: &mut usize, n: usize| *si += n;
        match cmd {
            0xff if fm => {
                let voice = self.arg(si);
                self.parts[i].voicenum = voice;
                if self.parts[i].partmask != 0 {
                    if let Some(bx) = self.toneadr_calc(voice) {
                        self.parts[i].alg_fb = self.byte(bx + 24);
                        self.neiroset_tl(i, bx + 4);
                    }
                } else {
                    self.neiroset(i, voice);
                }
            }
            0xfe if !rhythm => {
                let q = self.arg(si);
                self.parts[i].qdata = q;
                self.parts[i].qdat3 = 0;
            }
            0xfd => self.parts[i].volume = self.arg(si),
            0xfc => self.comt(si),
            0xfb => self.tieflag |= 1,
            0xfa => self.parts[i].detune = self.arg_word(si),
            0xf9 => {
                let at = usize::from(self.arg_word(si)) + 1;
                if let Some(slot) = self.md.get_mut(at) {
                    *slot = 0;
                }
            }
            0xf8 => {
                let count = self.arg(si);
                let jump = if count == 0 {
                    *si += 1;
                    true
                } else {
                    let counter = self.byte(*si).wrapping_add(1);
                    if let Some(slot) = self.md.get_mut(*si) {
                        *slot = counter;
                    }
                    *si += 1;
                    counter != count
                };
                if jump {
                    let target = usize::from(self.arg_word(si)) + 2;
                    *si = target;
                } else {
                    *si += 2;
                }
            }
            0xf7 => {
                let bx = usize::from(self.arg_word(si));
                let count = self.byte(bx).wrapping_sub(1);
                if count == self.byte(bx + 1) {
                    *si = bx + 4;
                }
            }
            0xf6 => self.parts[i].partloop = *si,
            0xf5 if !rhythm => self.parts[i].shift = self.arg(si),
            0xf4 => {
                let p = &mut self.parts[i];
                p.volume = if fm {
                    p.volume.wrapping_add(4).min(127)
                } else {
                    p.volume.wrapping_add(1).min(15)
                };
            }
            0xf3 => {
                let p = &mut self.parts[i];
                p.volume = p.volume.saturating_sub(if fm { 4 } else { 1 });
            }
            0xf2 if !rhythm => {
                self.lfoset(i, si);
            }
            0xf1 if !rhythm => {
                let mut al = self.arg(si);
                if al & 0xf8 != 0 {
                    al = 1;
                }
                let p = &mut self.parts[i];
                p.lfoswi = (p.lfoswi & 0xf8) | (al & 7);
                self.lfoinit_main(i);
            }
            0xf0 if ssg => {
                let p = &mut self.parts[i];
                let ar = self.md.get(*si).copied().unwrap_or(0);
                let dr = self.md.get(*si + 1).copied().unwrap_or(0);
                let sr = self.md.get(*si + 2).copied().unwrap_or(0);
                let rr = self.md.get(*si + 3).copied().unwrap_or(0);
                *si += 4;
                p.eenv_ar = ar;
                p.eenv_arc = ar;
                p.eenv_dr = dr;
                p.eenv_sr = sr;
                p.eenv_src = sr;
                p.eenv_rr = rr;
                p.eenv_rrc = rr;
                if p.envf == 0xff {
                    p.envf = 2;
                    p.eenv_volume = 0xf1;
                }
            }
            0xef => {
                let reg = self.arg(si);
                let value = self.arg(si);
                self.opnset(reg, value);
            }
            0xee if ssg => self.psnoi = self.arg(si),
            0xed if ssg => self.parts[i].psgpat = self.arg(si),
            0xec if fm => {
                let al = self.arg(si);
                self.panset(i, al);
            }
            0xeb => {
                let al = self.arg(si);
                self.rhykey(al);
            }
            0xea => {
                let al = self.arg(si);
                self.rhythm_level(al, 0xc0, al & 0x1f);
            }
            0xe9 => {
                let al = self.arg(si);
                self.rhythm_level(al, 0x1f, (al & 3) << 6);
            }
            0xe8 => {
                let mut dl = self.arg(si);
                if self.rhythm_voldown != 0 {
                    dl = ((u16::from(self.rhythm_voldown.wrapping_neg()) * u16::from(dl)) >> 8)
                        as u8;
                }
                self.volset2r(dl);
            }
            0xe7 if !rhythm => {
                let al = self.arg(si);
                self.parts[i].shift = self.parts[i].shift.wrapping_add(al);
            }
            0xe6 => {
                let al = self.arg(si) as i8;
                let value = (i16::from(self.rhyvol) + i16::from(al)).clamp(0, 63) as u8;
                self.volset2r(value);
            }
            0xe5 => {
                let index = usize::from(self.arg(si)).clamp(1, 6) - 1;
                let delta = self.arg(si) as i8;
                let level = (i16::from(self.rdat[index] & 0x1f) + i16::from(delta)).clamp(0, 31);
                self.rdat[index] = (self.rdat[index] & 0xe0) | level as u8;
                self.opnset44(0x18 + index as u8, self.rdat[index]);
            }
            0xe4 if fm => self.parts[i].hldelay = self.arg(si),
            0xe3 => {
                let al = self.arg(si);
                let p = &mut self.parts[i];
                let max = if fm { 127 } else { 15 };
                p.volume = p.volume.wrapping_add(al).min(max);
            }
            0xe2 => {
                let al = self.arg(si);
                let p = &mut self.parts[i];
                p.volume = p.volume.saturating_sub(al);
            }
            0xe1 if fm => {
                let al = self.arg(si);
                let p = &mut self.parts[i];
                p.fmpan = (p.fmpan & 0xc0) | al;
                if p.partmask == 0 {
                    let value = self.calc_panout(i);
                    self.opnset(0xb4 + self.partb - 1, value);
                }
            }
            0xe0 if fm => {
                let al = self.arg(si);
                self.opnset44(0x22, al);
            }
            0xdf => self.syousetu_lng = self.arg(si),
            0xde => {
                let al = self.arg(si);
                let p = &mut self.parts[i];
                let max = if fm { 127 } else { 15 };
                let value = al.wrapping_add(p.volume);
                let value = if value > max { max } else { value };
                p.volpush = value + 1;
                self.volpush_flag = 1;
            }
            0xdd => {
                let al = self.arg(si);
                let p = &mut self.parts[i];
                let value = match p.volume.checked_sub(al) {
                    Some(v) if v < 255 => v,
                    Some(_) => 254,
                    None => 0,
                };
                p.volpush = value + 1;
                self.volpush_flag = 1;
            }
            0xdc => self.status = self.arg(si),
            0xdb => {
                let al = self.arg(si);
                self.status = self.status.wrapping_add(al);
            }
            0xda if fm => return self.porta(i, si),
            0xda if ssg => return self.portap(i, si),
            0xd6 if !rhythm => {
                let speed = self.arg(si);
                let depth = self.arg(si);
                let lfo = &mut self.parts[i].lfo[0];
                lfo.mdspd = speed;
                lfo.mdspd2 = speed;
                lfo.mdepth = depth;
            }
            0xd5 => {
                let delta = self.arg_word(si);
                let p = &mut self.parts[i];
                p.detune = p.detune.wrapping_add(delta);
            }
            0xd2 => {
                self.fadeout_speed = self.arg(si);
            }
            0xd0 if ssg => {
                let al = self.arg(si) as i8;
                self.psnoi = (i16::from(self.psnoi) + i16::from(al)).clamp(0, 31) as u8;
            }
            0xcf if fm => {
                let al = self.arg(si);
                self.slotmask_set(i, al);
            }
            0xce | 0xc6 => skip(si, 6),
            0xcd if ssg => {
                let ar = self.arg(si) & 0x1f;
                let dr = self.arg(si) & 0x1f;
                let sr = self.arg(si) & 0x1f;
                let rr_sl = self.arg(si);
                let al = self.arg(si) & 0xf;
                let p = &mut self.parts[i];
                p.eenv_ar = ar;
                p.eenv_dr = dr;
                p.eenv_sr = sr;
                p.eenv_rr = rr_sl & 0xf;
                p.eenv_sl = ((rr_sl >> 4) & 0xf) ^ 0xf;
                p.eenv_al = al;
                if p.envf != 0xff {
                    p.envf = 0xff;
                    p.eenv_count = 4;
                    p.eenv_volume = 0;
                }
            }
            0xcd => skip(si, 5),
            0xcc if ssg => {
                let al = self.arg(si) & 1;
                let p = &mut self.parts[i];
                p.extendmode = (p.extendmode & 0xfe) | al;
            }
            0xcb if !rhythm => self.parts[i].lfo[0].wave = self.arg(si),
            0xca if !rhythm => {
                let al = (self.arg(si) & 1) << 1;
                let p = &mut self.parts[i];
                p.extendmode = (p.extendmode & 0xfd) | al;
            }
            0xc9 if ssg => {
                let al = (self.arg(si) & 1) << 2;
                let p = &mut self.parts[i];
                p.extendmode = (p.extendmode & 0xfb) | al;
            }
            0xc8 | 0xc7 => skip(si, 3),
            0xc5 if fm => {
                let al = self.arg(si);
                let p = &mut self.parts[i];
                p.volmask = if al & 0xf == 0 {
                    p.carrier
                } else {
                    ((al & 0xf) << 4) | 0xf
                };
            }
            0xc4 if !rhythm => self.parts[i].qdatb = self.arg(si),
            0xc3 if fm => {
                let al = self.arg(si);
                *si += 1;
                let pan = if al == 0 {
                    3
                } else if al & 0x80 != 0 {
                    1
                } else {
                    2
                };
                self.panset(i, pan);
            }
            0xc3 => skip(si, 2),
            0xc2 if !rhythm => {
                let delay = self.arg(si);
                let lfo = &mut self.parts[i].lfo[0];
                lfo.delay = delay;
                lfo.delay2 = delay;
                self.lfoinit_main(i);
            }
            0xc1 => {}
            0xc0 => {
                let al = self.arg(si);
                if al >= 2 {
                    // Voldown and other 0C0h specials carry one byte.
                    let value = self.arg(si);
                    match al {
                        0xff => self.fm_voldown = value,
                        0xfd => self.ssg_voldown = value,
                        0xf9 => self.rhythm_voldown = value,
                        _ => {}
                    }
                }
            }
            0xbf if !rhythm => {
                self.lfo_change(i);
                self.lfoset(i, si);
                self.lfo_change(i);
            }
            0xbe if !rhythm => {
                let al = (self.arg(si) & 7) << 4;
                let p = &mut self.parts[i];
                p.lfoswi = (p.lfoswi & 0x8f) | al;
                self.lfo_change(i);
                self.lfoinit_main(i);
                self.lfo_change(i);
            }
            0xbd if !rhythm => {
                let speed = self.arg(si);
                let depth = self.arg(si);
                let lfo = &mut self.parts[i].lfo[1];
                lfo.mdspd = speed;
                lfo.mdspd2 = speed;
                lfo.mdepth = depth;
            }
            0xbc if !rhythm => self.parts[i].lfo[1].wave = self.arg(si),
            0xbb if !rhythm => {
                let al = (self.arg(si) & 1) << 5;
                let p = &mut self.parts[i];
                p.extendmode = (p.extendmode & 0xdf) | al;
            }
            0xba if fm => {
                let al = self.arg(si);
                let p = &mut self.parts[i];
                p.volmask2 = if al & 0xf == 0 {
                    p.carrier
                } else {
                    ((al & 0xf) << 4) | 0xf
                };
            }
            0xb9 if !rhythm => {
                let delay = self.arg(si);
                self.lfo_change(i);
                let lfo = &mut self.parts[i].lfo[0];
                lfo.delay = delay;
                lfo.delay2 = delay;
                self.lfoinit_main(i);
                self.lfo_change(i);
            }
            0xb8 if fm => self.tl_set(i, si),
            0xb5 if fm => {
                let mask = self.arg(si) & 0xf;
                let delay = self.arg(si);
                let p = &mut self.parts[i];
                p.sdelay_m = (mask ^ 0xf).rotate_right(4);
                p.sdelay = delay;
                p.sdelay_c = delay;
            }
            0xb8 | 0xb5 | 0xd6 | 0xbd => skip(si, 2),
            0xbf => skip(si, 4),
            0xb7 if !rhythm => {
                let al = self.arg(si);
                let (index, mut value) = if al & 0x80 != 0 {
                    (1, al & 0x7f)
                } else {
                    (0, al)
                };
                if value == 0 {
                    value = 0xff;
                }
                let lfo = &mut self.parts[i].lfo[index];
                lfo.mdc = value;
                lfo.mdc2 = value;
            }
            0xb6 if fm => {
                let al = self.arg(si);
                self.fb_set(i, al);
            }
            0xb4 => skip(si, 16),
            0xb3 if !rhythm => self.parts[i].qdat2 = self.arg(si),
            0xb2 if !rhythm => self.parts[i].shift_def = self.arg(si),
            0xb1 if !rhythm => self.parts[i].qdat3 = self.arg(si),
            0xf2 | 0xf0 => skip(si, 4),
            _ if cmd < 0xb1 => {
                // Unknown command: the driver turns it into a part end.
                *si -= 1;
                if let Some(slot) = self.md.get_mut(*si) {
                    *slot = 0x80;
                }
            }
            // Everything else takes a single ignored byte.
            _ => skip(si, 1),
        }
        Flow::Continue
    }

    fn lfoset(&mut self, i: usize, si: &mut usize) {
        let delay = self.arg(si);
        let speed = self.arg(si);
        let step = self.arg(si);
        let time = self.arg(si);
        let lfo = &mut self.parts[i].lfo[0];
        lfo.delay = delay;
        lfo.delay2 = delay;
        lfo.speed = speed;
        lfo.speed2 = speed;
        lfo.step = step;
        lfo.step2 = step;
        lfo.time = time;
        lfo.time2 = time;
        self.lfoinit_main(i);
    }

    fn porta(&mut self, i: usize, si: &mut usize) -> Flow {
        if self.parts[i].partmask != 0 {
            *si += 1;
            return Flow::Continue;
        }
        let al = self.arg(si);
        let al = self.lfoinit(i, al);
        let al = self.oshift(i, al);
        self.fnumset(i, al);
        let from = self.parts[i].fnum;
        let onkai = self.parts[i].onkai;
        let al = self.arg(si);
        let al = self.oshift(i, al);
        self.fnumset(i, al);
        let to = self.parts[i].fnum;
        self.parts[i].onkai = onkai;
        self.parts[i].fnum = from;
        let octave_diff =
            ((((to >> 8) as u8 & 0x38).wrapping_sub((from >> 8) as u8 & 0x38)) as i8) >> 3;
        let mut ax = i32::from(octave_diff) * 0x26a;
        ax += i32::from(to & 0x7ff) - i32::from(from & 0x7ff);
        let leng = self.arg(si);
        self.parts[i].leng = leng;
        *si = self.calc_q(i, *si);
        self.set_porta(i, ax, leng);
        Flow::PortaNote
    }

    fn portap(&mut self, i: usize, si: &mut usize) -> Flow {
        if self.parts[i].partmask != 0 {
            *si += 1;
            return Flow::Continue;
        }
        let al = self.arg(si);
        let al = self.lfoinitp(i, al);
        let al = self.oshift(i, al);
        self.fnumsetp(i, al);
        let from = self.parts[i].fnum;
        let onkai = self.parts[i].onkai;
        let al = self.arg(si);
        let al = self.oshift(i, al);
        self.fnumsetp(i, al);
        let to = self.parts[i].fnum;
        self.parts[i].onkai = onkai;
        self.parts[i].fnum = from;
        let ax = i32::from(to as i16) - i32::from(from as i16);
        let leng = self.arg(si);
        self.parts[i].leng = leng;
        *si = self.calc_q(i, *si);
        self.set_porta(i, ax, leng);
        Flow::PortaNote
    }

    fn set_porta(&mut self, i: usize, ax: i32, leng: u8) {
        let ax = i32::from(ax as i16);
        let divisor = i32::from(leng.max(1));
        let p = &mut self.parts[i];
        p.porta_num2 = (ax / divisor) as u16;
        p.porta_num3 = (ax % divisor) as u16;
        p.lfoswi |= 8;
    }

    fn calc_panout(&self, i: usize) -> u8 {
        let p = &self.parts[i];
        if p.hldelay_c != 0 {
            p.fmpan & 0xc0
        } else {
            p.fmpan
        }
    }

    fn panset(&mut self, i: usize, al: u8) {
        let pan = al.rotate_right(2) & 0xc0;
        let p = &mut self.parts[i];
        p.fmpan = (p.fmpan & 0x3f) | pan;
        if p.partmask == 0 {
            let value = self.calc_panout(i);
            self.opnset(0xb4 + self.partb - 1, value);
        }
    }

    fn rhykey(&mut self, al: u8) {
        if al == 0 {
            return;
        }
        if al & 0x80 == 0 {
            for n in 0..6 {
                if al & (1 << n) != 0 {
                    self.opnset44(0x18 + n as u8, self.rdat[n]);
                }
            }
        }
        self.opnset44(0x10, al);
    }

    /// `rs002`: updates one rhythm voice's level or pan (top 3 bits select it).
    fn rhythm_level(&mut self, al: u8, keep: u8, value: u8) {
        let index = usize::from(al >> 5);
        if !(1..=6).contains(&index) {
            return;
        }
        let slot = &mut self.rdat[index - 1];
        *slot = (*slot & keep) | value;
        let data = *slot;
        self.opnset44(0x18 + index as u8 - 1, data);
    }

    fn volset2r(&mut self, dl: u8) {
        self.rhyvol = dl;
        let mut value = dl;
        if self.fadeout_volume != 0 {
            value = ((u16::from(!self.fadeout_volume) * u16::from(dl)) >> 8) as u8;
        }
        self.opnset44(0x11, value);
    }

    fn slotmask_set(&mut self, i: usize, al: u8) {
        let p = &mut self.parts[i];
        if al & 0xf != 0 {
            p.carrier = (al & 0xf).rotate_left(4);
        } else {
            p.carrier = CARRIER_TABLE[usize::from(p.alg_fb & 7)];
        }
        let mask = al & 0xf0;
        if p.slotmask == mask {
            return;
        }
        p.slotmask = mask;
        if mask == 0 {
            p.partmask |= 0x20;
        } else {
            p.partmask &= 0xdf;
        }
        let mut neiromask = 0u8;
        if mask & 0x80 != 0 {
            neiromask |= 0x11;
        }
        if mask & 0x40 != 0 {
            neiromask |= 0x44;
        }
        if mask & 0x20 != 0 {
            neiromask |= 0x22;
        }
        if mask & 0x10 != 0 {
            neiromask |= 0x88;
        }
        p.neiromask = neiromask;
    }

    fn tl_set(&mut self, i: usize, si: &mut usize) {
        let al = self.arg(si);
        let dl = self.arg(si);
        let slots = al & 0xf & (self.parts[i].slotmask >> 4);
        let write = self.parts[i].partmask == 0;
        // Slot order in the operand: bit0=slot1, bit1=slot2, bit2=slot3, bit3=slot4.
        let regs = [(0u8, 0usize), (8, 1), (4, 2), (12, 3)];
        let base = 0x40 + self.partb - 1;
        for (bit, (offset, slot)) in regs.into_iter().enumerate() {
            if slots & (1 << bit) == 0 {
                continue;
            }
            let value = if al & 0x80 == 0 {
                dl & 0x7f
            } else {
                let v = self.parts[i].slot[slot].wrapping_add(dl);
                if v & 0x80 != 0 {
                    if dl & 0x80 != 0 { 0 } else { 127 }
                } else {
                    v
                }
            };
            self.parts[i].slot[slot] = value;
            if write {
                self.opnset(base + offset, value);
            }
        }
    }

    fn fb_set(&mut self, i: usize, al: u8) {
        let reg = 0xb0 + self.partb - 1;
        let current = self.parts[i].alg_fb;
        let fb = if al & 0x80 == 0 {
            (al & 7) << 3
        } else {
            let delta = if al & 0x40 != 0 {
                al as i8
            } else {
                (al & 7) as i8
            };
            let value = i16::from((current >> 3) & 7) + i16::from(delta);
            (value.clamp(0, 7) as u8) << 3
        };
        let value = (current & 7) | fb;
        self.opnset(reg, value);
        self.parts[i].alg_fb = value;
    }
}

/// `fm_block_calc`: normalises an F-number that left the base octave.
fn fm_block_calc(cx: &mut u16, ax: &mut u16) {
    loop {
        if *ax & 0x8000 == 0 && *ax >= 0x26a {
            if *ax < 0x26a * 2 {
                return;
            }
            *cx += 0x800;
            if *cx == 0x4000 {
                *cx = 0x3800;
                if *ax >= 0x800 {
                    *ax = 0x7ff;
                }
                return;
            }
            *ax = ax.wrapping_sub(0x26a);
        } else {
            if *cx < 0x800 {
                *cx = 0;
                if *ax & 0x8000 != 0 || *ax < 8 {
                    *ax = 8;
                }
                return;
            }
            *cx -= 0x800;
            *ax = ax.wrapping_add(0x26a);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_calc_moves_octaves() {
        let (mut cx, mut ax) = (0x2000u16, 0x26a * 2 + 5);
        fm_block_calc(&mut cx, &mut ax);
        assert_eq!((cx, ax), (0x2800, 0x26a + 5));
        let (mut cx, mut ax) = (0x2000u16, 0x200);
        fm_block_calc(&mut cx, &mut ax);
        assert_eq!((cx, ax), (0x1800, 0x46a));
    }

    #[test]
    fn plays_a_minimal_score() {
        // Part A: @0 V127 o4 A (len 24), end; voice 0 is a sine carrier.
        let mut md = vec![0u8; 26];
        let part_a = 26usize;
        md.extend_from_slice(&[0xff, 0x00, 0xfd, 0x7f, 0x49, 24, 0x80]);
        let empty = md.len();
        md.push(0x80);
        let voices = md.len();
        let mut voice = vec![0u8];
        voice.extend_from_slice(&[1, 1, 1, 1, 127, 127, 127, 0, 31, 31, 31, 31]);
        voice.extend_from_slice(&[0; 8]);
        voice.extend_from_slice(&[0x0f, 0x0f, 0x0f, 0x0f, 7]);
        md.extend_from_slice(&voice);
        md[0..2].copy_from_slice(&(part_a as u16).to_le_bytes());
        for k in 1..11 {
            md[k * 2..k * 2 + 2].copy_from_slice(&(empty as u16).to_le_bytes());
        }
        md[22..24].copy_from_slice(&(empty as u16).to_le_bytes());
        md[24..26].copy_from_slice(&(voices as u16).to_le_bytes());
        let mut file = vec![0u8];
        file.extend_from_slice(&md);
        let mut pmd = Pmd::new();
        pmd.start(&file);
        let mut energy = 0.0f32;
        for _ in 0..(RATE as usize / 4) {
            let (l, _) = pmd.sample();
            energy += l * l;
        }
        assert!(energy > 1.0, "{energy}");
    }
}
