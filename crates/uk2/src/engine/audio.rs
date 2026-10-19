//! Audio output for the desktop player: streams the PMD/YM2608 model
//! through kira.

use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use kira::Frame;
use kira::OutputDestination;
use kira::clock::clock_info::ClockInfoProvider;
use kira::manager::backend::DefaultBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::modulator::value_provider::ModulatorValueProvider;
use kira::sound::{Sound, SoundData};

use super::MusicCommand;
use super::opna::RATE;
use super::pmd::Pmd;

struct Shared {
    pmd: Pmd,
    volume: f32,
}

struct PmdSound {
    shared: Arc<Mutex<Shared>>,
    time: f64,
    previous: (f32, f32),
    current: (f32, f32),
}

impl Sound for PmdSound {
    fn output_destination(&mut self) -> OutputDestination {
        OutputDestination::MAIN_TRACK
    }

    fn process(
        &mut self,
        dt: f64,
        _clock_info_provider: &ClockInfoProvider,
        _modulator_value_provider: &ModulatorValueProvider,
    ) -> Frame {
        let Ok(mut shared) = self.shared.lock() else {
            return Frame::ZERO;
        };
        self.time += dt * RATE;
        while self.time >= 1.0 {
            self.time -= 1.0;
            self.previous = self.current;
            self.current = shared.pmd.sample();
        }
        let t = self.time as f32;
        let volume = shared.volume;
        Frame {
            left: (self.previous.0 + (self.current.0 - self.previous.0) * t) * volume,
            right: (self.previous.1 + (self.current.1 - self.previous.1) * t) * volume,
        }
    }

    fn finished(&self) -> bool {
        false
    }
}

struct PmdSoundData(Arc<Mutex<Shared>>);

impl SoundData for PmdSoundData {
    type Error = ();
    type Handle = ();

    fn into_sound(self) -> Result<(Box<dyn Sound>, Self::Handle), Self::Error> {
        Ok((
            Box::new(PmdSound {
                shared: self.0,
                time: 0.0,
                previous: (0.0, 0.0),
                current: (0.0, 0.0),
            }),
            (),
        ))
    }
}

/// Plays the scores the engine hands to its music driver.
pub struct MusicPlayer {
    _manager: AudioManager<DefaultBackend>,
    shared: Arc<Mutex<Shared>>,
    score: Option<Vec<u8>>,
}

impl MusicPlayer {
    pub fn new() -> Result<Self> {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .context("open audio device")?;
        let shared = Arc::new(Mutex::new(Shared {
            pmd: Pmd::new(),
            volume: 1.0,
        }));
        manager
            .play(PmdSoundData(Arc::clone(&shared)))
            .map_err(|error| anyhow::anyhow!("start music stream: {error:?}"))?;
        Ok(Self {
            _manager: manager,
            shared,
            score: None,
        })
    }

    pub fn command(&mut self, command: MusicCommand) {
        let Ok(mut shared) = self.shared.lock() else {
            return;
        };
        match command {
            MusicCommand::Load { data, midi, .. } => {
                shared.pmd.stop();
                // Only the FM (PMD) driver is emulated.
                self.score = (!midi).then_some(data);
            }
            MusicCommand::Start => {
                if let Some(score) = &self.score {
                    shared.pmd.start(score);
                }
            }
            MusicCommand::Stop => shared.pmd.stop(),
            MusicCommand::Fade(speed) => shared.pmd.fade_out(speed),
        }
    }
}
