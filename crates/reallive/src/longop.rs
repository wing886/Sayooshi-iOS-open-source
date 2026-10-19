//! General-purpose long operations.

use anyhow::Result;

use crate::machine::{IntTarget, LongOp, Machine};

/// What a [`Wait`] waits for, besides its optional deadline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitEvent {
    None,
    /// Until timer `counter` of `layer` exceeds `time`.
    Timer {
        layer: usize,
        counter: i32,
        time: i32,
    },
    /// Until the music has stopped.
    Bgm,
    /// Until a PCM channel is silent.
    Wav(usize),
    /// Until the voice has finished.
    Koe,
    /// Until a `PCMEVENT` has finished.
    PcmEvent(i32),
    /// Until a serial animation (or all of them) has finished.
    SerialPdt(Option<i32>),
    /// Until the screen flash has ended.
    Flash,
    /// Until the background has stopped scrolling.
    HaikeiScroll,
}

/// `wait`, `waitC`, `time`, `timeC`, `GetClick`, `WaitClick`.
///
/// When clicks break the wait, `store` receives 1 for a left click, -1 for
/// a right click and 0 when the wait ended by itself; with
/// `click_location` the cursor position is written to the two variables.
#[derive(Debug, Clone)]
pub struct Wait {
    pub until: Option<u64>,
    pub event: WaitEvent,
    pub break_on_click: bool,
    pub click_location: Option<(IntTarget, IntTarget)>,
    /// Skip mode does not cut it short (`DONTJUMP_*`).
    pub unskippable: bool,
}

impl Wait {
    pub fn for_ms(machine: &Machine, ms: i32) -> Self {
        Self {
            until: Some(machine.sys.now() + ms.max(0) as u64),
            event: WaitEvent::None,
            break_on_click: false,
            click_location: None,
            unskippable: false,
        }
    }

    pub fn event(event: WaitEvent) -> Self {
        Self {
            until: None,
            event,
            break_on_click: false,
            click_location: None,
            unskippable: false,
        }
    }

    pub fn unskippable(mut self) -> Self {
        self.unskippable = true;
        self
    }

    pub fn cancellable(mut self) -> Self {
        self.break_on_click = true;
        self
    }

    pub fn recording(mut self, x: IntTarget, y: IntTarget) -> Self {
        self.break_on_click = true;
        self.click_location = Some((x, y));
        self
    }

    fn record_location(&self, machine: &mut Machine) -> Result<()> {
        if let Some((x, y)) = self.click_location {
            let (mx, my) = machine.sys.input.mouse;
            machine.set_target(x, mx)?;
            machine.set_target(y, my)?;
        }
        Ok(())
    }
}

impl LongOp for Wait {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        if self.break_on_click {
            if let Some(button) = machine.sys.input.take_click() {
                self.record_location(machine)?;
                machine.store = button;
                return Ok(true);
            }
        }
        let now = machine.sys.now();
        let mut done = !self.unskippable && machine.sys.should_fast_forward();
        if let Some(until) = self.until {
            done |= now >= until;
        }
        if let WaitEvent::Timer {
            layer,
            counter,
            time,
        } = self.event
        {
            done |= machine.sys.timers.read(layer, counter, now) > time;
        }
        let sound = &machine.sys.sound;
        done |= match self.event {
            WaitEvent::Bgm => sound.bgm_status(now) == 0,
            WaitEvent::Wav(channel) => !sound.wav_playing(channel, now),
            WaitEvent::Koe => !sound.koe_playing(now),
            WaitEvent::PcmEvent(number) => !machine.sys.pcm_events.contains_key(&number),
            WaitEvent::SerialPdt(buf) => !machine.sys.gfx.snm.running(false, buf),
            WaitEvent::HaikeiScroll => machine.sys.gfx.haikei_scroll.is_none(),
            WaitEvent::Flash => machine
                .sys
                .gfx
                .flash
                .as_ref()
                .is_none_or(|f| f.finished(now)),
            _ => false,
        };
        if done && self.break_on_click {
            self.record_location(machine)?;
            machine.store = 0;
        }
        Ok(done)
    }

    fn name(&self) -> &'static str {
        "wait"
    }
}

/// A screen transition from `before` to `after`.
#[derive(Debug)]
pub struct TransitionOp {
    pub transition: crate::effects::Transition,
    pub before: std::rc::Rc<crate::surface::Surface>,
    pub after: std::rc::Rc<crate::surface::Surface>,
    pub start: u64,
}

impl TransitionOp {
    pub fn new(
        machine: &Machine,
        transition: crate::effects::Transition,
        before: crate::surface::Surface,
        after: crate::surface::Surface,
    ) -> Self {
        Self {
            transition,
            before: std::rc::Rc::new(before),
            after: std::rc::Rc::new(after),
            start: machine.sys.now(),
        }
    }
}

impl LongOp for TransitionOp {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let now = machine.sys.now();
        let time = self.transition.time.max(0) as u64;
        let skip = machine.sys.should_fast_forward() || machine.sys.settings.skip_animations;
        if skip || time == 0 || now >= self.start + time {
            machine.sys.gfx.transition_frame = None;
            machine.sys.gfx.dirty = true;
            return Ok(true);
        }
        let t = (now - self.start) as f64 / time as f64;
        let t = crate::effects::eased(self.transition.style, t);
        let frame = crate::effects::render(&self.transition, t, &self.before, &self.after);
        machine.sys.gfx.transition_frame = Some(std::rc::Rc::new(frame));
        Ok(false)
    }

    fn name(&self) -> &'static str {
        "transition"
    }
}

/// Progress of a timed operation, `0..=1`.
fn progress(machine: &Machine, start: u64, time: i32) -> f64 {
    if time <= 0 || machine.sys.should_fast_forward() {
        return 1.0;
    }
    ((machine.sys.now() - start) as f64 / f64::from(time)).clamp(0.0, 1.0)
}

fn lerp_rect(a: crate::surface::Rect, b: crate::surface::Rect, t: f64) -> crate::surface::Rect {
    let mix = |x: i32, y: i32| (f64::from(x) + f64::from(y - x) * t).round() as i32;
    crate::surface::Rect::new(mix(a.x, b.x), mix(a.y, b.y), mix(a.w, b.w), mix(a.h, b.h))
}

/// `grpZoom`: stretches an area of a DC that moves from `from` to `to`
/// into `dest` on DC 0.
#[derive(Debug)]
pub struct ZoomOp {
    source: std::rc::Rc<crate::surface::Surface>,
    from: crate::surface::Rect,
    to: crate::surface::Rect,
    dest: crate::surface::Rect,
    time: i32,
    start: u64,
}

impl ZoomOp {
    pub fn new(
        machine: &mut Machine,
        dc: i32,
        from: crate::surface::Rect,
        to: crate::surface::Rect,
        dest: crate::surface::Rect,
        time: i32,
    ) -> Result<Self> {
        let source = machine.sys.gfx.dc(dc)?;
        Ok(Self {
            source: std::rc::Rc::new((*source).clone()),
            from,
            to,
            dest,
            time,
            start: machine.sys.now(),
        })
    }
}

impl LongOp for ZoomOp {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let t = progress(machine, self.start, self.time);
        let area = lerp_rect(self.from, self.to, t);
        machine.sys.gfx.dc_mut(0)?.stretch_blit(
            &self.source,
            area,
            self.dest,
            255,
            crate::surface::Blend::Copy,
        );
        Ok(t >= 1.0)
    }

    fn name(&self) -> &'static str {
        "zoom"
    }
}

/// `grpFlash`: fills an area with a colour briefly, then shows the scene
/// again (DC 0 is not modified).
#[derive(Debug)]
pub struct FlashOp {
    rect: Option<crate::surface::Rect>,
    colour: [u8; 4],
    time: i32,
    start: u64,
}

impl FlashOp {
    pub fn new(
        machine: &Machine,
        rect: Option<crate::surface::Rect>,
        colour: [u8; 4],
        time: i32,
    ) -> Self {
        Self {
            rect,
            colour,
            time,
            start: machine.sys.now(),
        }
    }
}

impl LongOp for FlashOp {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        if progress(machine, self.start, self.time) >= 1.0 {
            machine.sys.gfx.transition_frame = None;
            machine.sys.gfx.dirty = true;
            return Ok(true);
        }
        let mut frame = crate::screen::compose_scene(&mut machine.sys);
        let rect = self.rect.unwrap_or(frame.rect());
        frame.fill(rect, self.colour, 255);
        machine.sys.gfx.transition_frame = Some(std::rc::Rc::new(frame));
        Ok(false)
    }

    fn name(&self) -> &'static str {
        "flash"
    }
}

/// `grpPan`, `grpShift`, `grpSlide`: scroll an area of a DC through a
/// window of DC 0.
#[derive(Debug)]
pub struct PanOp {
    kind: u16,
    source: std::rc::Rc<crate::surface::Surface>,
    /// Pan: start and end positions. Shift/slide: source position and
    /// block size.
    a: (i32, i32),
    b: (i32, i32),
    window: crate::surface::Rect,
    direction: i32,
    time: i32,
    start: u64,
    /// DC 0 inside the window when the operation started.
    underneath: crate::surface::Surface,
}

impl PanOp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        machine: &mut Machine,
        kind: u16,
        rec: bool,
        dc: i32,
        a: (i32, i32),
        b: (i32, i32),
        window: crate::surface::Rect,
        direction: i32,
        time: i32,
    ) -> Result<Self> {
        let source = machine.sys.gfx.dc(dc)?;
        let underneath = machine.sys.gfx.dc(0)?.crop(window);
        // grpShift/grpSlide give the block as corners; rec as a size.
        let b = if kind != 406 && !rec {
            (b.0 - a.0 + 1, b.1 - a.1 + 1)
        } else {
            b
        };
        Ok(Self {
            kind,
            source: std::rc::Rc::new((*source).clone()),
            a,
            b,
            window,
            direction,
            time,
            start: machine.sys.now(),
            underneath,
        })
    }
}

impl LongOp for PanOp {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        use crate::surface::{Blend, Rect};
        let t = progress(machine, self.start, self.time);
        let w = self.window;
        let dc0 = machine.sys.gfx.dc_mut(0)?;
        if self.kind == 406 {
            let x = (f64::from(self.a.0) + f64::from(self.b.0 - self.a.0) * t).round() as i32;
            let y = (f64::from(self.a.1) + f64::from(self.b.1 - self.a.1) * t).round() as i32;
            dc0.blit(
                &self.source,
                Rect::new(x, y, w.w, w.h),
                w.x,
                w.y,
                255,
                Blend::Copy,
                None,
            );
        } else {
            // Direction: 0 down, 1 up, 2 right, 3 left.
            let (horizontal, positive) = match self.direction {
                0 => (false, true),
                1 => (false, false),
                2 => (true, true),
                _ => (true, false),
            };
            let span = if horizontal { self.b.0 } else { self.b.1 };
            let shown = (f64::from(span) * t).round() as i32;
            let block = if horizontal {
                Rect::new(self.a.0, self.a.1, self.b.0, w.h)
            } else {
                Rect::new(self.a.0, self.a.1, w.w, self.b.1)
            };
            let (bx, by) = match (horizontal, positive) {
                (false, true) => (w.x, w.y - span + shown),
                (false, false) => (w.x, w.y + w.h - shown),
                (true, true) => (w.x - span + shown, w.y),
                (true, false) => (w.x + w.w - shown, w.y),
            };
            if self.kind == 407 {
                // Shift: the old contents move out ahead of the new block.
                let (ox, oy) = match (horizontal, positive) {
                    (false, true) => (w.x, w.y + shown),
                    (false, false) => (w.x, w.y - shown),
                    (true, true) => (w.x + shown, w.y),
                    (true, false) => (w.x - shown, w.y),
                };
                let old = self.underneath.clone();
                let mut canvas = crate::surface::Surface::new(w.w, w.h);
                canvas.blit(&old, old.rect(), ox - w.x, oy - w.y, 255, Blend::Copy, None);
                canvas.blit(
                    &self.source,
                    block,
                    bx - w.x,
                    by - w.y,
                    255,
                    Blend::Copy,
                    None,
                );
                dc0.blit(&canvas, canvas.rect(), w.x, w.y, 255, Blend::Copy, None);
            } else {
                // Slide: the new block moves in over the old contents.
                let old = self.underneath.clone();
                let mut canvas = crate::surface::Surface::new(w.w, w.h);
                canvas.blit(&old, old.rect(), 0, 0, 255, Blend::Copy, None);
                canvas.blit(
                    &self.source,
                    block,
                    bx - w.x,
                    by - w.y,
                    255,
                    Blend::Copy,
                    None,
                );
                dc0.blit(&canvas, canvas.rect(), w.x, w.y, 255, Blend::Copy, None);
            }
        }
        Ok(t >= 1.0)
    }

    fn name(&self) -> &'static str {
        "pan"
    }
}
