//! The machine's services: time, input, settings, graphics, text, sound.

use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;

use crate::cgtable::CgTable;
use crate::clock::{Clock, FrameCounters, Timers};
use crate::font::FontSet;
use crate::gameexe::Gameexe;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::nls::Nls;
use crate::resource::{Kind, Resources};
use crate::serial::Writer;
use crate::settings::{Settings, Syscom};
use crate::text::TextSystem;
use crate::ui::Ui;

/// Options chosen by the host.
#[derive(Debug, Clone)]
pub struct SystemOptions {
    pub virtual_clock: bool,
    pub audio: bool,
    /// Read and write save files.
    pub persist: bool,
    pub save_dir: Option<PathBuf>,
    /// Encoding of configuration files and file names.
    pub nls: Nls,
    /// Load system fonts (headless tests skip this).
    pub fonts: bool,
}

impl Default for SystemOptions {
    fn default() -> Self {
        Self {
            virtual_clock: false,
            audio: true,
            persist: true,
            save_dir: None,
            nls: Nls::Sjis,
            fonts: true,
        }
    }
}

#[derive(Debug)]
pub struct System {
    pub gameexe: Rc<Gameexe>,
    pub root: PathBuf,
    pub clock: Clock,
    pub timers: Timers,
    pub frames: FrameCounters,
    pub input: Input,
    pub settings: Settings,
    pub defaults: Settings,
    pub syscom: Syscom,
    pub text: TextSystem,
    pub gfx: Graphics,
    pub rng: u64,
    /// `title()`: the save title.
    pub title: String,
    pub cursor_visible: bool,
    pub mouse_cursor: i32,
    /// Fast-forward (Ctrl held, or skip mode through read text).
    pub fast_forward: bool,
    /// `CtrlKeySkipOn` / `CtrlKeySkipOff`.
    pub ctrl_key_skip: bool,
    /// The game asked to exit (`end()` or syscom 29).
    pub quit_requested: bool,
    pub options: SystemOptions,
    /// Bytecode is running on behalf of a menu (`#CANCELCALL`, `#WBCALL`).
    pub in_menu: bool,
    /// `SetCursorPos`: the host should move the pointer.
    pub warp_cursor: Option<(i32, i32)>,
    /// `PauseCursor(index)`.
    pub key_cursor: i32,
    /// Undocumented `Sys 430..457` flag triples.
    pub misc_flags: Vec<i32>,
    /// `SET_WAIP_WINDOWCLOSE_*` (`#WAIP_WINDOWCLOSE`): screen transitions
    /// close the text windows first.
    pub wipe_closes_windows: bool,
    /// `SET_GRPCOM_WINDOWCLOSE_*` (`#GRPCOM_WINDOWCLOSE`): graphics
    /// commands that update the screen close the text windows first.
    pub grp_closes_windows: bool,
    /// `SET_READJUMPCANCEL_*`: selections end skip mode.
    pub read_jump_cancel: bool,
    /// `CGTABLE_ON` / `CGTABLE_OFF`: whether viewed CGs are recorded.
    pub cg_table_enabled: bool,
    /// `ENABLE_SYSTEMMENU_ANIME` / `DISENABLE_SYSTEMMENU_ANIME`.
    pub system_menu_animation: bool,
    pub default_grp: String,
    pub default_bgr: String,
    /// Volume or channel settings changed; the mixer should re-read them.
    pub volumes_changed: bool,
    pub resources: Resources,
    pub cg_table: CgTable,
    pub ui: Ui,
    /// `MenuReturn`: fade the screen out before the next scene starts.
    pub fade_out_requested: bool,
    /// The running selection, for the renderer.
    pub selection: Option<crate::select::Selection>,
    /// RealLiveMax switches with no effect here, remembered for scripts
    /// that read them back (see `modules::sys_max`).
    pub remembered: std::collections::BTreeMap<u16, i32>,
    /// `SET_MOUSEAREA`: the pointer is kept inside (x1, y1, x2, y2).
    pub mouse_area: Option<(i32, i32, i32, i32)>,
    /// The last voice played: (id, character) (`KOE_REPLAY`).
    pub last_koe: (i32, i32),
    /// `CCOM_SET_WINDOW_EXBTN_OFF`: extra window buttons hidden.
    pub exbtn_hidden: std::collections::BTreeSet<usize>,
    /// `#DATABASE.nnn` tables, loaded on first use (`None`: missing).
    pub databases: std::collections::HashMap<i32, Option<Rc<siglus_assets::dbs::DbsDatabase>>>,
    /// Running `PCMEVENT_*` sound events, by number.
    pub pcm_events: std::collections::BTreeMap<i32, crate::pcm_event::PcmEvent>,
    /// A polled button-object selection (`select_btnobjstart`).
    pub polled_buttons: Option<crate::select::PolledButtons>,
    pub sound: crate::sound::Sound,
    pub movie: Option<crate::movie::Movie>,
}

impl Default for System {
    fn default() -> Self {
        Self::new(
            Rc::new(Gameexe::default()),
            PathBuf::new(),
            SystemOptions {
                virtual_clock: true,
                audio: false,
                persist: false,
                save_dir: None,
                nls: Nls::Sjis,
                fonts: false,
            },
        )
    }
}

impl System {
    pub fn new(gameexe: Rc<Gameexe>, root: PathBuf, options: SystemOptions) -> Self {
        let settings = Settings::from_gameexe(&gameexe);
        let ctrl_key_skip = gameexe.int("CTRL_USE").unwrap_or(1) != 0;
        let resources = Resources::new(&root, &gameexe, options.nls);
        let tone_curves = gameexe
            .str("TONECURVE_FILENAME")
            .filter(|name| !name.is_empty())
            .and_then(|name| resources.read(crate::resource::Kind::Data, name))
            .map(|bytes| crate::tone_curve::ToneCurves::parse(&bytes))
            .unwrap_or_default();
        let wipe_closes_windows = gameexe.int("WAIP_WINDOWCLOSE").unwrap_or(0) != 0;
        let grp_closes_windows = gameexe.int("GRPCOM_WINDOWCLOSE").unwrap_or(0) != 0;
        let cg_table = gameexe
            .str("CGTABLE_FILENAME")
            .and_then(|name| resources.read(Kind::Data, name))
            .and_then(|bytes| CgTable::load(&bytes).ok())
            .unwrap_or_default();
        let clock = if options.virtual_clock {
            Clock::Virtual(0)
        } else {
            Clock::default()
        };
        let seed = if options.virtual_clock {
            0x853c_49e6_748f_ea9b
        } else {
            web_time::SystemTime::now()
                .duration_since(web_time::UNIX_EPOCH)
                .map_or(1, |d| d.as_nanos() as u64)
                | 1
        };
        let fonts = if options.fonts {
            FontSet::load_system()
        } else {
            FontSet::empty()
        };
        let sound = crate::sound::Sound::new(&gameexe, options.audio);
        Self {
            sound,
            movie: None,
            gfx: Graphics {
                tone_curves,
                ..Graphics::new(&gameexe, fonts)
            },
            syscom: Syscom::from_gameexe(&gameexe),
            text: TextSystem::new(&gameexe),
            defaults: settings.clone(),
            settings,
            gameexe,
            root,
            clock,
            timers: Timers::default(),
            frames: FrameCounters::default(),
            input: Input::default(),
            rng: seed,
            title: String::new(),
            cursor_visible: true,
            mouse_cursor: 0,
            fast_forward: false,
            ctrl_key_skip,
            quit_requested: false,
            options,
            in_menu: false,
            warp_cursor: None,
            key_cursor: 0,
            misc_flags: Vec::new(),
            wipe_closes_windows,
            grp_closes_windows,
            read_jump_cancel: false,
            cg_table_enabled: true,
            system_menu_animation: true,
            default_grp: String::new(),
            default_bgr: String::new(),
            volumes_changed: true,
            resources,
            cg_table,
            ui: Ui::default(),
            fade_out_requested: false,
            selection: None,
            polled_buttons: None,
            pcm_events: Default::default(),
            remembered: Default::default(),
            mouse_area: None,
            last_koe: (-1, 0),
            exbtn_hidden: Default::default(),
            databases: Default::default(),
        }
    }

    pub fn request_fade_out(&mut self) {
        self.fade_out_requested = true;
    }

    /// Clears what is on screen and playing (load, return to menu).
    pub fn reset_presentation(&mut self) {
        self.ui.overlay = None;
        self.ui.requests.clear();
        self.text.reset();
        self.gfx.reset();
    }

    /// Writes the savepoint state of every subsystem.
    pub fn save_state(&self, w: &mut Writer) {
        w.section("text", |w| self.text.save(w));
        w.section("graphics", |w| self.gfx.save(w));
    }

    pub fn load_state(&mut self, sections: &[(String, &[u8])]) -> Result<()> {
        for (name, body) in sections {
            let mut r = crate::serial::Reader::new(body);
            match name.as_str() {
                "text" => self.text.load(&mut r)?,
                "graphics" => {
                    self.gfx.load(&mut r, &self.resources)?;
                    let now = self.clock.now();
                    self.gfx.rebase_times(now);
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn now(&self) -> u64 {
        self.clock.now()
    }

    /// A uniformly distributed number in `min..=max` (either order).
    pub fn random(&mut self, min: i32, max: i32) -> i32 {
        let (low, high) = if min <= max { (min, max) } else { (max, min) };
        // splitmix64
        self.rng = self.rng.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        let span = (i64::from(high) - i64::from(low) + 1) as u64;
        (i64::from(low) + (z % span) as i64) as i32
    }

    /// Whether text and waits should be skipped right now.
    /// Text, waits and transitions go by at once: the host's fast
    /// forward, Ctrl held (`#CTRL_USE`), or skip mode over text already
    /// read.
    pub fn should_fast_forward(&self) -> bool {
        self.fast_forward
            || (self.ctrl_key_skip && self.input.ctrl)
            || (self.syscom.skip_mode && self.syscom.skip_mode_allowed && self.text.kidoku_read())
    }

    pub fn take_savepoint(&mut self) {
        self.text.take_savepoint();
        self.gfx.take_savepoint();
    }

    /// Whether a voice is playing (auto mode waits for it).
    pub fn voice_playing(&self) -> bool {
        self.sound.koe_playing(self.now())
    }

    /// Plays interface sound `#SE.nnn` (0 hover, 1 decide, ...).
    pub fn play_se(&mut self, index: i32) {
        let now = self.now();
        if self.should_fast_forward() {
            return;
        }
        // A missing sound file is not worth interrupting the game for.
        let _ = self
            .sound
            .se_play(&self.resources, &self.settings, now, index);
    }

    /// Advances the movie; its sound starts with the picture.
    pub fn update_movie(&mut self) {
        let now = self.now();
        let Some(movie) = self.movie.as_mut() else {
            return;
        };
        let was_started = movie.started();
        let running = movie.update(now);
        if !was_started && movie.started() {
            if let Some(pcm) = movie.audio_ready.take() {
                self.sound.movie_play(&self.settings, now, &pcm);
            }
        }
        if !running {
            let movie = self.movie.take().expect("present");
            self.sound.movie_stop();
            if movie.looped {
                self.movie = Some(crate::movie::Movie::open(
                    movie.path.clone(),
                    movie.dest,
                    true,
                    now,
                ));
            }
        }
    }

    pub fn stop_movie(&mut self) {
        self.movie = None;
        self.sound.movie_stop();
    }
}
