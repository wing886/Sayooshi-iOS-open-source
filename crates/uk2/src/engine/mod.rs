//! Port of the PC-98 `UK2.EXE` engine (AyPio "SOSAR SYSTEM").
//!
//! The port follows the reference executable function by function.  Engine
//! globals live in a real data segment ([`mem::Mem`]) initialised from the
//! game's own `UK2.EXE`, graphics are drawn into emulated PC-98 bit-plane
//! VRAM ([`vram::Vram`]), and the original's interrupt handlers (VSYNC timer,
//! bus mouse, keyboard) are emulated at the points where the original code
//! polls their results.  Function names in comments refer to the IDA listing.

pub mod audio;
pub mod battle;
pub mod ds;
pub mod font;
pub mod gfx;
pub mod interp;
pub mod map;
pub mod mem;
pub mod object;
pub mod opna;
pub mod pmd;
pub mod system;
pub mod text;
pub mod ui;
pub mod vram;

use std::collections::VecDeque;

use anyhow::{Result, bail};

use crate::game::Uk2Game;
use ds::*;
use mem::{FarPtr, Mem, ds_ptr};
use vram::Vram;

/// PC-98 400-line VSYNC frequency.
pub const VSYNC_HZ: f64 = 56.4229;

/// PC-98 keyboard scan codes used by the engine.
pub mod scan {
    pub const ESC: u8 = 0x00;
    pub const RETURN: u8 = 0x1C;
    pub const SPACE: u8 = 0x34;
    pub const UP: u8 = 0x3A;
    pub const LEFT: u8 = 0x3B;
    pub const RIGHT: u8 = 0x3C;
    pub const DOWN: u8 = 0x3D;
    pub const PAD8: u8 = 0x43;
    pub const PAD4: u8 = 0x46;
    pub const PAD6: u8 = 0x48;
    pub const PAD2: u8 = 0x4B;
    pub const VF1: u8 = 0x52;
    pub const SHIFT: u8 = 0x70;
    pub const CAPS: u8 = 0x71;
    pub const GRPH: u8 = 0x73;
    pub const CTRL: u8 = 0x74;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostEvent {
    /// Absolute pointer position in 640x400 screen pixels.
    MouseMove {
        x: i32,
        y: i32,
    },
    MouseButton {
        left: bool,
        pressed: bool,
    },
    /// PC-98 scan code; repeats are delivered as further presses.
    Key {
        code: u8,
        pressed: bool,
    },
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusicCommand {
    /// Load a score (`M0`); `kind` is the driver chosen by the engine.
    Load {
        name: String,
        data: Vec<u8>,
        midi: bool,
    },
    Start,
    Stop,
    /// `M1`: driver fade-out (PMD/MMD function 2) with the given speed.
    Fade(u8),
}

/// Host services required by the engine thread.
pub trait Platform {
    /// Monotonic VSYNC tick count.
    fn ticks(&mut self) -> u64;
    /// Block for a short while (the engine is waiting on an interrupt).
    fn wait(&mut self);
    fn poll_events(&mut self, events: &mut Vec<HostEvent>);
    /// RGBA 640x400 frame of the displayed page, cursor included.
    fn present(&mut self, rgba: &[u8]);
    fn music(&mut self, _command: MusicCommand) {}
    /// Resident music drivers reported to the game: bit 0 = MMD (MIDI),
    /// bit 1 = PMD (FM).  The port only synthesises PMD scores.
    fn music_drivers(&self) -> u16 {
        2
    }
}

/// Returned through `anyhow` when the host asked the engine to stop.
#[derive(Debug)]
pub struct QuitRequested;

impl std::fmt::Display for QuitRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("uk2: quit requested")
    }
}

impl std::error::Error for QuitRequested {}

pub struct Engine {
    pub mem: Mem,
    pub vram: Vram,
    pub game: Uk2Game,
    pub platform: Box<dyn Platform>,
    pub kanji: font::KanjiRom,
    pub(crate) text_encoding: font::TextEncoding,
    foreign_font: Option<font::ForeignFont>,
    pub(crate) programs: Vec<interp::Program>,
    pub(crate) last_map_name: Vec<u8>,
    last_tick: u64,
    instructions_since_tick: u32,
    rng: u32,
    host_mouse: (i32, i32),
    host_buttons: u8,
    events: Vec<HostEvent>,
    rgba: Vec<u8>,
    quit: bool,
    last_present_tick: u64,
    pub trace: bool,
    /// Log every MES file the interpreter enters.
    pub trace_mes: bool,
    /// Debugging aid: MES file to run instead of the configured `START`.
    pub start_override: Option<Vec<u8>>,
    pub instruction_count: u64,
    pub max_instructions: Option<u64>,
    pending_music: VecDeque<MusicCommand>,
}

/// Instructions the emulated CPU executes per VSYNC before it is throttled.
const INSTRUCTIONS_PER_TICK: u32 = 3000;

impl Engine {
    pub fn new(game: Uk2Game, platform: Box<dyn Platform>) -> Result<Self> {
        let exe = game.read_raw_file("UK2.EXE")?;
        let mem = Mem::from_exe(&exe)?;
        let kanji = font::KanjiRom::load(game.root())?;
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(1);
        let mut engine = Self {
            mem,
            vram: Vram::new(),
            game,
            platform,
            kanji,
            text_encoding: font::TextEncoding::default(),
            foreign_font: None,
            programs: Vec::new(),
            last_map_name: Vec::new(),
            last_tick: 0,
            instructions_since_tick: 0,
            rng: seed,
            host_mouse: (0x27f, 0x18f),
            host_buttons: 0,
            events: Vec::new(),
            rgba: Vec::new(),
            quit: false,
            last_present_tick: 0,
            trace: false,
            trace_mes: false,
            start_override: None,
            instruction_count: 0,
            max_instructions: None,
            pending_music: VecDeque::new(),
        };
        engine.last_tick = engine.platform.ticks();
        Ok(engine)
    }

    // ---- small helpers -------------------------------------------------

    #[inline]
    pub fn w(&self, off: u16) -> u16 {
        self.mem.w(off)
    }
    #[inline]
    pub fn sw(&self, off: u16) -> i16 {
        self.mem.sw(off)
    }
    #[inline]
    pub fn set_w(&mut self, off: u16, value: u16) {
        self.mem.set_w(off, value);
    }
    #[inline]
    pub fn b(&self, off: u16) -> u8 {
        self.mem.b(off)
    }
    #[inline]
    pub fn set_b(&mut self, off: u16, value: u8) {
        self.mem.set_b(off, value);
    }

    /// Borland C `rand()`.
    pub fn rand(&mut self) -> u16 {
        self.rng = self.rng.wrapping_mul(0x015A_4E35).wrapping_add(1);
        ((self.rng >> 16) & 0x7fff) as u16
    }

    /// The executable's `rand() * n / 0x8000` idiom.
    pub fn rand_n(&mut self, n: i32) -> i32 {
        (i32::from(self.rand() as i16) * n) / 0x8000
    }

    /// `sub_1ECC8`: fatal engine error.
    pub fn fatal(&self, code: u16) -> anyhow::Error {
        let name = self.mem.cstr(ds_ptr(CUR_MES_NAME));
        let next = self.mem.cstr(ds_ptr(NEXT_MES_NAME));
        anyhow::anyhow!(
            "uk2: engine error no={code} count={} file={}|{}",
            self.w(MES_PC),
            String::from_utf8_lossy(&name).trim_end(),
            String::from_utf8_lossy(&next).trim_end()
        )
    }

    // ---- interrupt emulation --------------------------------------------

    /// Services pending host events and elapsed VSYNC interrupts.
    pub fn poll(&mut self) -> Result<()> {
        if self.quit {
            return Err(QuitRequested.into());
        }
        let mut events = std::mem::take(&mut self.events);
        events.clear();
        self.platform.poll_events(&mut events);
        let mut mouse_changed = false;
        for event in events.drain(..) {
            match event {
                HostEvent::Quit => {
                    self.quit = true;
                }
                HostEvent::MouseMove { x, y } => {
                    self.host_mouse = (x, y);
                    mouse_changed = true;
                }
                HostEvent::MouseButton { left, pressed } => {
                    let bit = if left { 1 } else { 2 };
                    if pressed {
                        self.host_buttons |= bit;
                    } else {
                        self.host_buttons &= !bit;
                    }
                    self.mouse_irq();
                    mouse_changed = false;
                }
                HostEvent::Key { code, pressed } => self.keyboard_irq(code, pressed),
            }
        }
        self.events = events;
        if mouse_changed {
            self.mouse_irq();
        }
        let now = self.platform.ticks();
        while self.last_tick < now {
            self.last_tick += 1;
            self.instructions_since_tick = 0;
            self.vsync_irq();
            self.mouse_irq();
        }
        if self.vram.dirty && self.last_present_tick != self.last_tick {
            self.present();
        }
        if self.quit {
            return Err(QuitRequested.into());
        }
        Ok(())
    }

    /// Called from busy-wait loops of the original code.
    pub fn idle(&mut self) -> Result<()> {
        if self.vram.dirty {
            self.present();
        }
        self.platform.wait();
        self.poll()
    }

    /// Called once per interpreted MES instruction; throttles the emulated
    /// CPU to a plausible PC-98 speed.
    pub fn instruction_tick(&mut self) -> Result<()> {
        self.instruction_count += 1;
        if let Some(limit) = self.max_instructions
            && self.instruction_count > limit
        {
            bail!("uk2: instruction limit {limit} reached");
        }
        self.instructions_since_tick += 1;
        if self.instructions_since_tick >= INSTRUCTIONS_PER_TICK {
            let tick = self.last_tick;
            while self.last_tick == tick {
                self.idle()?;
            }
        } else {
            self.poll()?;
        }
        Ok(())
    }

    /// `sub_1F8AD(1)`: wait for the start of the next VSYNC.
    pub fn wait_vsync(&mut self) -> Result<()> {
        let tick = self.last_tick;
        while self.last_tick == tick {
            self.idle()?;
        }
        Ok(())
    }

    pub fn wait_ticks(&mut self, count: u32) -> Result<()> {
        for _ in 0..count {
            self.wait_vsync()?;
        }
        Ok(())
    }

    /// `sub_12574`: delay in milliseconds.
    pub fn delay_ms(&mut self, ms: u32) -> Result<()> {
        let ticks = ((f64::from(ms) * VSYNC_HZ) / 1000.0).ceil() as u32;
        self.wait_ticks(ticks)
    }

    /// `sub_2146C`: VSYNC interrupt body.
    fn vsync_irq(&mut self) {
        let ticks = self.w(TICKS).wrapping_add(1);
        self.set_w(TICKS, ticks);
        for timer in [TIMER_A, FRAME_TIMER] {
            let value = self.w(timer);
            if value != 0 {
                self.set_w(timer, value - 1);
            }
        }
        if self.w(CURSOR_ANIM_RATE) != 0 {
            let count = self.w(CURSOR_ANIM_COUNT);
            if count != 0 {
                self.set_w(CURSOR_ANIM_COUNT, count - 1);
            } else {
                // sub_215B1: advance the animated cursor frame.
                let mut frame = self.w(CURSOR_FRAME).wrapping_add(1);
                if self.w(CURSOR_FRAMES) < frame {
                    frame = 0;
                }
                self.set_w(CURSOR_FRAME, frame);
                let rate = self.w(CURSOR_ANIM_RATE);
                self.set_w(CURSOR_ANIM_COUNT, rate);
                self.vram.dirty = true;
            }
        }
        let press = self.w(PRESS_TIMER);
        if press != 0 {
            self.set_w(PRESS_TIMER, press - 1);
            if press == 1 {
                self.set_w(RIGHT_PRESSES, 0);
                self.set_w(LEFT_PRESSES, 0);
            }
        }
        let release = self.w(RELEASE_TIMER);
        if release != 0 {
            self.set_w(RELEASE_TIMER, release - 1);
            if release == 1 {
                self.set_w(RIGHT_RELEASES, 0);
                self.set_w(LEFT_RELEASES, 0);
            }
        }
    }

    /// `sub_2184F`: mouse event handler.
    fn mouse_irq(&mut self) {
        if self.w(CURSOR_MODE) == 2 {
            return;
        }
        let buttons = self.host_buttons;
        self.set_w(RIGHT_HELD, u16::from(buttons & 2 != 0));
        self.set_w(LEFT_HELD, u16::from(buttons & 1 != 0));
        let lifetime = u16::from(self.b(EVENT_LIFETIME));
        let (mx, my) = (self.w(MOUSE_X), self.w(MOUSE_Y));
        match self.w(MOUSE_PREV_RIGHT) + self.w(RIGHT_HELD) {
            1 => {
                self.set_w(RIGHT_PRESS_X, mx);
                self.set_w(RIGHT_PRESS_Y, my);
                if self.w(PRESS_TIMER) == 0 {
                    self.set_w(PRESS_TIMER, lifetime);
                }
                let v = self.w(RIGHT_PRESSES).wrapping_add(1);
                self.set_w(RIGHT_PRESSES, v);
            }
            2 => {
                self.set_w(RIGHT_RELEASE_X, mx);
                self.set_w(RIGHT_RELEASE_Y, my);
                if self.w(RELEASE_TIMER) == 0 {
                    self.set_w(RELEASE_TIMER, lifetime);
                }
                let v = self.w(RIGHT_RELEASES).wrapping_add(1);
                self.set_w(RIGHT_RELEASES, v);
            }
            _ => {}
        }
        match self.w(MOUSE_PREV_LEFT) + self.w(LEFT_HELD) {
            1 => {
                self.set_w(LEFT_PRESS_X, mx);
                self.set_w(LEFT_PRESS_Y, my);
                if self.w(PRESS_TIMER) == 0 {
                    self.set_w(PRESS_TIMER, lifetime);
                }
                let v = self.w(LEFT_PRESSES).wrapping_add(1);
                self.set_w(LEFT_PRESSES, v);
            }
            2 => {
                self.set_w(LEFT_RELEASE_X, mx);
                self.set_w(LEFT_RELEASE_Y, my);
                if self.w(RELEASE_TIMER) == 0 {
                    self.set_w(RELEASE_TIMER, lifetime);
                }
                let v = self.w(LEFT_RELEASES).wrapping_add(1);
                self.set_w(LEFT_RELEASES, v);
            }
            _ => {}
        }
        let right = self.w(RIGHT_HELD);
        let left = self.w(LEFT_HELD);
        self.set_w(MOUSE_PREV_RIGHT, right * 2);
        self.set_w(MOUSE_PREV_LEFT, left * 2);
        if self.w(CURSOR_MODE) != 0 {
            return;
        }
        // The host reports absolute positions; feed the delta through the
        // original scaling and clamping.
        let dx = self.host_mouse.0 - i32::from(self.sw(MOUSE_X));
        let dy = self.host_mouse.1 - i32::from(self.sw(MOUSE_Y));
        if dx == 0 && dy == 0 {
            return;
        }
        let mut x = i32::from(self.sw(MOUSE_X)) + dx;
        let mut y = i32::from(self.sw(MOUSE_Y)) + dy;
        x = x
            .max(i32::from(self.sw(MOUSE_MIN_X)))
            .min(i32::from(self.sw(MOUSE_MAX_X)));
        y = y
            .max(i32::from(self.sw(MOUSE_MIN_Y)))
            .min(i32::from(self.sw(MOUSE_MAX_Y)));
        if x != i32::from(self.sw(MOUSE_X)) || y != i32::from(self.sw(MOUSE_Y)) {
            self.set_w(MOUSE_X, x as u16);
            self.set_w(MOUSE_Y, y as u16);
            if self.w(CURSOR_SHOWN) != 0 {
                self.vram.dirty = true;
            }
        }
    }

    /// Keyboard interrupt handler (seg055).
    fn keyboard_irq(&mut self, code: u8, pressed: bool) {
        if code >= 0x80 {
            return;
        }
        self.set_b(KEY_TABLE + u16::from(code), u8::from(pressed));
        if self.key(scan::SHIFT) && self.key(scan::GRPH) && self.key(scan::CTRL) {
            self.set_w(ABORT, 1);
        }
        if self.w(KEYS_ENABLED) == 0 {
            // byte_29FAB/AC/AA/AD, 29FA4, 29F70, 29F8C, 29FB6/B8/BB/B3, 29FC2
            for off in [
                0x3B, 0x3C, 0x3A, 0x3D, 0x34, 0x00, 0x1C, 0x46, 0x48, 0x4B, 0x43, 0x52,
            ] {
                self.set_b(KEY_TABLE + off, 0);
            }
        }
        if self.w(CURSOR_MODE) != 2 {
            if self.key(scan::RETURN) || self.key(scan::SPACE) {
                self.set_w(LEFT_HELD, 1);
                self.set_w(LEFT_PRESSES, 1);
            }
            if self.key(scan::ESC) || self.key(scan::VF1) {
                self.set_w(RIGHT_HELD, 1);
                self.set_w(RIGHT_PRESSES, 1);
            }
        }
    }

    #[inline]
    pub fn key(&self, code: u8) -> bool {
        self.b(KEY_TABLE + u16::from(code)) != 0
    }

    /// `sub_21339`: shift-key state bits (1 SHIFT, 2 CAPS, 8 GRPH, 0x10 CTRL).
    pub fn shift_state(&self) -> u8 {
        let mut state = 0;
        if self.key(scan::SHIFT) {
            state |= 1;
        }
        if self.key(scan::CTRL) {
            state |= 0x10;
        }
        if self.key(scan::GRPH) {
            state |= 8;
        }
        if self.key(scan::CAPS) {
            state |= 2;
        }
        state
    }

    /// `sub_213F4`: cursor/numeric-pad direction bits (1 up, 2 down, 4 left,
    /// 8 right).
    pub fn direction_keys(&self) -> u16 {
        let mut bits = 0;
        if self.key(scan::PAD8) || self.key(scan::UP) {
            bits |= 1;
        }
        if self.key(scan::PAD4) || self.key(scan::LEFT) {
            bits |= 4;
        }
        if self.key(scan::PAD2) || self.key(scan::DOWN) {
            bits |= 2;
        }
        if self.key(scan::PAD6) || self.key(scan::RIGHT) {
            bits |= 8;
        }
        bits
    }

    /// `sub_2144B`: ASCII code of the first pressed key.
    pub fn pressed_ascii(&self) -> u8 {
        for code in 0..0x80u16 {
            if self.b(KEY_TABLE + code) != 0 {
                return self.b(KEY_ASCII + code);
            }
        }
        0
    }

    // ---- presentation --------------------------------------------------

    pub fn present(&mut self) {
        let mut rgba = std::mem::take(&mut self.rgba);
        self.vram.render_rgba(&mut rgba);
        self.draw_cursor(&mut rgba);
        self.platform.present(&rgba);
        self.rgba = rgba;
        self.vram.dirty = false;
        self.last_present_tick = self.last_tick;
    }

    /// Software cursor (`sub_21696`), drawn over the presented frame.
    fn draw_cursor(&self, rgba: &mut [u8]) {
        if self.w(CURSOR_SHOWN) == 0 || self.vram.display != 0 {
            return;
        }
        let shape = u32::from(self.w(CURSOR_SHAPE)) + u32::from(self.w(CURSOR_FRAME));
        let base = u32::from(CURSOR_DATA) + shape * u32::from(CURSOR_SIZE);
        let x0 = i32::from(self.sw(MOUSE_X));
        let y0 = i32::from(self.sw(MOUSE_Y));
        for row in 0..32i32 {
            let y = y0 + row;
            if !(0..400).contains(&y) {
                continue;
            }
            for col in 0..32i32 {
                let x = x0 + col;
                if !(0..640).contains(&x) {
                    continue;
                }
                let pixel = (y as usize) * 640 + x as usize;
                let mut colour = 0u8;
                let mut current = 0u8;
                // reconstruct the pixel from VRAM, then apply mask/data
                let vram_colour = self.vram.pixel(self.vram.display, x as usize, y as usize);
                let mut changed = false;
                for plane in 0..4u32 {
                    let rec = base + (row as u32 * 4 + plane) * 10;
                    let byte = (col / 8) as u32;
                    let bit = 7 - (col % 8) as u32;
                    let mask = (self.mem.ds[(rec + byte) as usize & 0xffff] >> bit) & 1;
                    let data = (self.mem.ds[(rec + 5 + byte) as usize & 0xffff] >> bit) & 1;
                    let mut value = (vram_colour >> plane) & 1;
                    if mask != 0 {
                        value = 0;
                        changed = true;
                    }
                    if data != 0 {
                        value = 1;
                        changed = true;
                    }
                    colour |= value << plane;
                    current |= ((vram_colour >> plane) & 1) << plane;
                }
                if changed && colour != current {
                    let rgb = Vram::palette_rgb(self.vram.palette[colour as usize]);
                    rgba[pixel * 4..pixel * 4 + 3].copy_from_slice(&rgb);
                }
            }
        }
    }

    // ---- music ---------------------------------------------------------

    /// Encoding of the game's double-byte text (Shift-JIS by default).
    pub fn set_text_encoding(&mut self, encoding: font::TextEncoding) {
        self.text_encoding = encoding;
        if self
            .foreign_font
            .as_ref()
            .is_some_and(|font| font.encoding() != encoding)
        {
            self.foreign_font = None;
        }
    }

    pub(crate) fn foreign_glyph(&mut self, lead: u8, trail: u8) -> [u8; 32] {
        let encoding = self.text_encoding;
        self.foreign_font
            .get_or_insert_with(|| font::ForeignFont::load(encoding))
            .glyph(lead, trail)
    }

    pub fn music(&mut self, command: MusicCommand) {
        self.platform.music(command);
    }
}

impl Engine {
    pub fn quit_requested(error: &anyhow::Error) -> bool {
        error.downcast_ref::<QuitRequested>().is_some()
    }

    pub(crate) fn heap_free(&mut self, ptr: FarPtr) {
        self.mem.free(ptr);
    }
}

impl Engine {
    /// Human-readable engine state for diagnostics.
    pub fn debug_dump(&self) -> String {
        use std::fmt::Write;
        let mut out = String::new();
        let _ = writeln!(out, "palette: {:03x?}", self.vram.palette);
        let _ = writeln!(
            out,
            "pages display={} access={} mouse=({}, {}) cursor={}",
            self.vram.display,
            self.vram.access,
            self.sw(MOUSE_X),
            self.sw(MOUSE_Y),
            self.w(CURSOR_SHOWN)
        );
        for i in 0..self.obj_count() {
            let o = self.obj_at(i);
            let _ = writeln!(
                out,
                "obj[{i}] id={} flags={:04x} rect=({},{})-({},{}) size={}x{} style={} text={:08x} fn={:08x} map={:08x}",
                self.ob(o, 0x2b),
                self.ow(o, 0x28),
                self.osw(o, 0x0a),
                self.osw(o, 0x0c),
                self.osw(o, 0x0e),
                self.osw(o, 0x10),
                self.osw(o, 0x12),
                self.osw(o, 0x14),
                self.ob(o, 0x2a),
                self.od(o, 0x2e),
                self.od(o, 0),
                self.od(o, 0x4e)
            );
            let text = self.od(o, 0x2e);
            if text != 0 {
                let bytes = self.mem.cstr(text);
                let (decoded, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
                let _ = writeln!(out, "    text: {decoded:?}");
            }
        }
        let mut colours = [0usize; 16];
        for y in 0..400 {
            for x in 0..640 {
                colours[self.vram.pixel(0, x, y) as usize] += 1;
            }
        }
        let _ = writeln!(out, "page0 colour histogram: {colours:?}");
        out
    }
}
