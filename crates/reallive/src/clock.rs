//! Time: the engine clock, RealLive timers and frame counters.

use web_time::Instant;

/// Milliseconds since the interpreter started, either from the wall clock
/// or advanced explicitly (headless tools and tests).
#[derive(Debug, Clone)]
pub enum Clock {
    Real(Instant),
    Virtual(u64),
}

impl Default for Clock {
    fn default() -> Self {
        Clock::Real(Instant::now())
    }
}

impl Clock {
    pub fn now(&self) -> u64 {
        match self {
            Clock::Real(start) => start.elapsed().as_millis() as u64,
            Clock::Virtual(now) => *now,
        }
    }

    pub fn advance(&mut self, ms: u64) {
        if let Clock::Virtual(now) = self {
            *now += ms;
        }
    }
}

pub const TIMER_COUNT: usize = 256;

/// Two layers (basic and "Ex") of 255 timers. A timer that was never set
/// reads the time since the interpreter started.
#[derive(Debug, Clone)]
pub struct Timers {
    /// `(time it was set, value it was set to)`.
    slots: [Vec<(u64, i64)>; 2],
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| vec![(0, 0); TIMER_COUNT]),
        }
    }
}

impl Timers {
    fn slot(&mut self, layer: usize, counter: i32) -> Option<&mut (u64, i64)> {
        self.slots
            .get_mut(layer)?
            .get_mut(usize::try_from(counter).ok()?)
    }

    pub fn set(&mut self, layer: usize, counter: i32, now: u64, value: i32) {
        if let Some(slot) = self.slot(layer, counter) {
            *slot = (now, i64::from(value));
        }
    }

    pub fn read(&self, layer: usize, counter: i32, now: u64) -> i32 {
        let Some(&(set_at, value)) = self
            .slots
            .get(layer)
            .and_then(|slots| slots.get(usize::try_from(counter).ok()?))
        else {
            return 0;
        };
        (value + now.saturating_sub(set_at) as i64).clamp(0, i64::from(i32::MAX)) as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
    Simple,
    Loop,
    Turn,
    Accel,
    Decel,
}

impl FrameKind {
    /// `InitFrame` opcodes 500–504 / 520–524 / 600–604 / 620–624.
    pub fn from_opcode(opcode: u16) -> Self {
        match opcode % 10 {
            1 => FrameKind::Loop,
            2 => FrameKind::Turn,
            3 => FrameKind::Accel,
            4 => FrameKind::Decel,
            _ => FrameKind::Simple,
        }
    }
}

/// A frame counter counting from `from` to `to` over `duration` ms.
///
/// The value is a pure function of the elapsed time, so it does not depend
/// on how often the game polls it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameCounter {
    kind: FrameKind,
    from: i32,
    to: i32,
    duration: u64,
    start: u64,
    /// Value while stopped.
    value: i32,
    active: bool,
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self {
            kind: FrameKind::Simple,
            from: 0,
            to: 0,
            duration: 0,
            start: 0,
            value: 0,
            active: false,
        }
    }
}

impl FrameCounter {
    pub fn new(kind: FrameKind, from: i32, to: i32, duration: i32, now: u64) -> Self {
        let duration = duration.max(0) as u64;
        Self {
            kind,
            from,
            to,
            duration,
            start: now,
            value: if duration == 0 { to } else { from },
            active: duration > 0 && from != to,
        }
    }

    fn progress(&self, now: u64) -> f64 {
        now.saturating_sub(self.start) as f64 / self.duration as f64
    }

    /// Updates the stopped state and returns the current value.
    pub fn read(&mut self, now: u64) -> i32 {
        if !self.active {
            return self.value;
        }
        let span = f64::from(self.to - self.from);
        let t = self.progress(now);
        let fraction = match self.kind {
            FrameKind::Simple => t,
            FrameKind::Loop => t.fract(),
            FrameKind::Turn => {
                let phase = t % 2.0;
                if phase <= 1.0 { phase } else { 2.0 - phase }
            }
            // Starts slowly and speeds up / the opposite; the rate changes
            // linearly from 0.9x to 1.1x (and back) of the mean speed.
            FrameKind::Accel => {
                5.0 * (1.1 / (1.1 - 0.2 * t.min(1.0))).ln() / (5.0 * (1.1f64 / 0.9).ln())
            }
            FrameKind::Decel => {
                5.0 * ((0.9 + 0.2 * t.min(1.0)) / 0.9).ln() / (5.0 * (1.1f64 / 0.9).ln())
            }
        };
        let finished = matches!(
            self.kind,
            FrameKind::Simple | FrameKind::Accel | FrameKind::Decel
        ) && t >= 1.0;
        if finished {
            self.active = false;
            self.value = self.to;
            return self.value;
        }
        let value = f64::from(self.from) + span * fraction;
        // Truncate towards the starting value, as integer counters do.
        value.trunc() as i32
    }

    pub fn is_active(&mut self, now: u64) -> bool {
        self.read(now);
        self.active
    }

    pub fn stop(&mut self, value: i32) {
        self.active = false;
        self.value = value;
    }
}

pub const FRAME_COUNTER_COUNT: usize = 256;

/// Two layers (basic and "Ex") of frame counters.
#[derive(Debug, Clone)]
pub struct FrameCounters {
    layers: [Vec<FrameCounter>; 2],
}

impl Default for FrameCounters {
    fn default() -> Self {
        Self {
            layers: std::array::from_fn(|_| vec![FrameCounter::default(); FRAME_COUNTER_COUNT]),
        }
    }
}

impl FrameCounters {
    pub fn get_mut(&mut self, layer: usize, counter: i32) -> Option<&mut FrameCounter> {
        self.layers
            .get_mut(layer)?
            .get_mut(usize::try_from(counter).ok()?)
    }

    pub fn set(&mut self, layer: usize, counter: i32, value: FrameCounter) {
        if let Some(slot) = self.get_mut(layer, counter) {
            *slot = value;
        }
    }

    pub fn read(&mut self, layer: usize, counter: i32, now: u64) -> i32 {
        self.get_mut(layer, counter).map_or(0, |c| c.read(now))
    }

    pub fn active(&mut self, layer: usize, counter: i32, now: u64) -> bool {
        self.get_mut(layer, counter)
            .is_some_and(|c| c.is_active(now))
    }

    pub fn any_active(&mut self, layer: usize, now: u64) -> bool {
        self.layers
            .get_mut(layer)
            .is_some_and(|layer| layer.iter_mut().any(|c| c.is_active(now)))
    }

    pub fn clear_all(&mut self, layer: usize, value: i32) {
        if let Some(layer) = self.layers.get_mut(layer) {
            for counter in layer {
                counter.stop(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timers_read_elapsed_time() {
        let mut timers = Timers::default();
        assert_eq!(timers.read(0, 3, 500), 500);
        timers.set(0, 3, 500, 100);
        assert_eq!(timers.read(0, 3, 700), 300);
    }

    #[test]
    fn frame_counters_are_functions_of_time() {
        let mut simple = FrameCounter::new(FrameKind::Simple, 0, 100, 1000, 0);
        assert_eq!(simple.read(500), 50);
        assert!(simple.is_active(999));
        assert_eq!(simple.read(1000), 100);
        assert!(!simple.is_active(1000));
        let mut down = FrameCounter::new(FrameKind::Simple, 100, 0, 1000, 0);
        assert_eq!(down.read(250), 75);
        let mut looped = FrameCounter::new(FrameKind::Loop, 0, 10, 100, 0);
        assert_eq!(looped.read(150), 5);
        assert!(looped.is_active(10_000));
        let mut turn = FrameCounter::new(FrameKind::Turn, 0, 10, 100, 0);
        assert_eq!(turn.read(150), 5);
        assert_eq!(turn.read(100), 10);
        let mut accel = FrameCounter::new(FrameKind::Accel, 0, 1000, 1000, 0);
        let mut decel = FrameCounter::new(FrameKind::Decel, 0, 1000, 1000, 0);
        assert!(accel.read(500) < 500 && decel.read(500) > 500);
        assert_eq!(accel.read(1000), 1000);
        let mut instant = FrameCounter::new(FrameKind::Simple, 0, 7, 0, 0);
        assert_eq!(instant.read(0), 7);
    }
}
