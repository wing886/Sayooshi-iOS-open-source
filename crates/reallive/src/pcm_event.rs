//! Sound events (`PCMEVENT_*`): a list of effects played one after
//! another with random pauses between them — in order and repeating
//! (`LOOP`), in order once (`ONESHOT`) or picked by weight (`RANDOM`).

use crate::system::System;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Loop,
    Random,
    OneShot,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub name: String,
    /// `RANDOM` only: the relative chance of this entry.
    pub weight: i32,
    pub wait_min: i32,
    pub wait_max: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PcmEvent {
    mode: Mode,
    entries: Vec<Entry>,
    next: usize,
    /// The channel of the effect playing now.
    channel: Option<usize>,
    /// When to play the next effect (after a pause).
    due: Option<u64>,
    finished: bool,
}

impl PcmEvent {
    pub fn new(mode: Mode, entries: Vec<Entry>) -> Self {
        let finished = entries.is_empty();
        Self {
            mode,
            entries,
            next: 0,
            channel: None,
            due: Some(0),
            finished,
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Stops the event; with `stop_sound` the effect playing now too.
    pub fn stop(&mut self, sys: &mut System, stop_sound: bool) {
        if stop_sound && let Some(channel) = self.channel.take() {
            let now = sys.now();
            sys.sound.wav_stop(channel, 0, now);
        }
        self.finished = true;
    }

    fn pick(&mut self, sys: &mut System) -> Option<usize> {
        match self.mode {
            Mode::Loop => {
                let index = self.next % self.entries.len();
                self.next = index + 1;
                Some(index)
            }
            Mode::OneShot => {
                let index = self.next;
                self.next += 1;
                (index < self.entries.len()).then_some(index)
            }
            Mode::Random => {
                let total: i32 = self.entries.iter().map(|e| e.weight.max(0)).sum();
                if total <= 0 {
                    return Some(sys.random(0, self.entries.len() as i32 - 1) as usize);
                }
                let mut roll = sys.random(0, total - 1);
                self.entries.iter().position(|e| {
                    roll -= e.weight.max(0);
                    roll < 0
                })
            }
        }
    }

    /// Per frame.
    pub fn update(&mut self, sys: &mut System) {
        if self.finished {
            return;
        }
        let now = sys.now();
        if let Some(channel) = self.channel {
            if sys.sound.wav_playing(channel, now) {
                return;
            }
            // The effect ended: pause before the next one.
            self.channel = None;
            let entry = &self.entries[self.next.saturating_sub(1) % self.entries.len()];
            let (low, high) = (
                entry.wait_min.max(0),
                entry.wait_max.max(entry.wait_min).max(0),
            );
            let pause = sys.random(low, high);
            self.due = Some(now + pause as u64);
            if self.mode == Mode::OneShot && self.next >= self.entries.len() {
                self.finished = true;
            }
            return;
        }
        if self.due.is_some_and(|due| now < due) {
            return;
        }
        let Some(index) = self.pick(sys) else {
            self.finished = true;
            return;
        };
        if self.mode == Mode::Random {
            // `next` names the entry just played, for its pause.
            self.next = index + 1;
        }
        let name = self.entries[index].name.clone();
        let played = sys
            .sound
            .wav_play(&sys.resources, &sys.settings, now, &name, None, false, 0);
        match played {
            Ok(channel) => {
                self.channel = Some(channel);
                self.due = None;
            }
            // A missing file must not stop the other effects.
            Err(_) => {
                let entry = &self.entries[index];
                self.due = Some(now + entry.wait_max.max(0) as u64);
            }
        }
    }
}

/// Runs every event (from the engine's frame).
pub fn update_all(sys: &mut System) {
    let mut events = std::mem::take(&mut sys.pcm_events);
    for event in events.values_mut() {
        event.update(sys);
    }
    events.retain(|_, event| !event.finished());
    // Events started while updating (none can be) would be kept too.
    events.append(&mut sys.pcm_events);
    sys.pcm_events = events;
}
