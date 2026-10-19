//! Frame-by-frame driver shared by the player and headless tools.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::{Context, Result};

use crate::archive::Archive;
use crate::gameexe::Gameexe;
use crate::input::InputEvent;
use crate::machine::{Frame, FrameKind, Machine};
use crate::nls::Nls;
use crate::surface::Surface;
use crate::system::{System, SystemOptions};
use crate::ui::Request;

#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub root: PathBuf,
    /// Text encoding; `None` uses the one RLdev recorded, else Shift-JIS.
    pub nls: Option<Nls>,
    pub virtual_clock: bool,
    pub audio: bool,
    pub persist: bool,
    pub save_dir: Option<PathBuf>,
    pub fonts: bool,
    /// Start here instead of `#SEEN_START`.
    pub start_scene: Option<i32>,
}

impl EngineOptions {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            nls: None,
            virtual_clock: false,
            audio: true,
            persist: true,
            save_dir: None,
            fonts: true,
            start_scene: None,
        }
    }

    /// Deterministic, silent and without side effects on disk.
    pub fn headless(root: impl Into<PathBuf>) -> Self {
        Self {
            virtual_clock: true,
            audio: false,
            persist: false,
            ..Self::new(root)
        }
    }
}

/// A file in `dir` by case-insensitive name.
fn find_case_insensitive(dir: &Path, name: &str) -> Option<PathBuf> {
    let wanted = name.to_ascii_lowercase();
    game_fs::read_dir(dir)
        .ok()?
        .flatten()
        .find(|entry| entry.file_name().to_string_lossy().to_ascii_lowercase() == wanted)
        .map(|entry| entry.path())
}

/// `SEEN.TXT`: `#FOLDNAME.TXT` names its folder and file; the game root is
/// the usual fallback.
fn find_archive(root: &Path, gameexe: &Gameexe) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(entry) = gameexe.get("FOLDNAME.TXT") {
        let strs = entry.strs();
        let folder = strs.first().copied().unwrap_or("");
        let file = strs.get(1).copied().unwrap_or("SEEN.TXT");
        if let Some(dir) = find_case_insensitive(root, folder) {
            candidates.push((dir, file.to_owned()));
        }
        candidates.push((root.to_path_buf(), file.to_owned()));
    }
    candidates.push((root.to_path_buf(), "SEEN.TXT".to_owned()));
    candidates
        .into_iter()
        .find_map(|(dir, file)| find_case_insensitive(&dir, &file))
}

pub struct Engine {
    pub machine: Machine,
    /// The last frame shown (manual drawing mode repeats it).
    shown: Option<Surface>,
    /// Game files the script asked to open (`shell`, e.g. its manual);
    /// the host decides what to do with them.
    pub files_to_open: Vec<PathBuf>,
}

impl std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("machine", &self.machine)
            .finish()
    }
}

impl Engine {
    pub fn open(options: EngineOptions) -> Result<Self> {
        let root = options.root.clone();
        let ini = find_case_insensitive(&root, "Gameexe.ini")
            .with_context(|| format!("reallive: no Gameexe.ini in {}", root.display()))?;
        let configured = options.nls.unwrap_or_default();
        let gameexe = Gameexe::open(&ini, configured)?;
        let seen = find_archive(&root, &gameexe)
            .with_context(|| format!("reallive: no SEEN.TXT under {}", root.display()))?;
        let regname = gameexe.str("REGNAME").unwrap_or("").to_owned();
        let mut archive = Archive::open(&seen, &regname, configured)?;
        let nls = match options.nls {
            Some(nls) => nls,
            None => archive.probable_encoding().unwrap_or(configured),
        };
        let gameexe = if nls != configured {
            archive = Archive::open(&seen, &regname, nls)?;
            Gameexe::open(&ini, nls)?
        } else {
            gameexe
        };
        let gameexe = Rc::new(gameexe);
        let sys = System::new(
            gameexe.clone(),
            root,
            SystemOptions {
                virtual_clock: options.virtual_clock,
                audio: options.audio,
                persist: options.persist,
                save_dir: options.save_dir,
                nls,
                fonts: options.fonts,
            },
        );
        let mut machine = Machine::new(Rc::new(archive), gameexe, sys)?;
        if let Some(scene) = options.start_scene {
            machine.jump(scene, 0)?;
        }
        Ok(Self {
            machine,
            shown: None,
            files_to_open: Vec::new(),
        })
    }

    pub fn input(&mut self, event: InputEvent) {
        self.machine.sys.input.push(event);
    }

    /// Advances one frame. `elapsed` is only used by the virtual clock.
    pub fn tick(&mut self, elapsed_ms: u64) {
        let machine = &mut self.machine;
        machine.sys.clock.advance(elapsed_ms);
        machine.sys.input.now = machine.sys.clock.now();
        if let Some((x1, y1, x2, y2)) = machine.sys.mouse_area {
            let (x, y) = machine.sys.input.mouse;
            let clamped = (x.clamp(x1, x2.max(x1)), y.clamp(y1, y2.max(y1)));
            if clamped != (x, y) {
                machine.sys.input.mouse = clamped;
                machine.sys.warp_cursor = Some(clamped);
            }
        }
        for request in std::mem::take(&mut machine.sys.ui.requests) {
            match request {
                Request::SyscomMenu => {
                    machine.sys.in_menu = true;
                    let menu = crate::ui::SyscomMenu::new(machine);
                    machine.push_long_op(Box::new(menu));
                }
                Request::SettingsDialog(index) => {
                    machine.note_unimplemented(format!("settings dialog {index}"));
                }
                Request::OpenFile(path) => self.files_to_open.push(path),
            }
        }
        if let Some(mut polled) = machine.sys.polled_buttons.take() {
            polled.update(&mut machine.sys);
            machine.sys.polled_buttons = Some(polled);
        }
        self.call_interrupt();
        let machine = &mut self.machine;
        machine.run();
        let now = machine.sys.now();
        let sys = &mut machine.sys;
        sys.update_movie();
        crate::pcm_event::update_all(sys);
        sys.sound.update(&sys.resources, &sys.settings, now);
        sys.gfx.update(now);
        sys.input.end_frame();
    }

    /// `SetInterrupt`: the handler runs at the start of every frame and
    /// returns with `yield`.
    fn call_interrupt(&mut self) {
        let machine = &mut self.machine;
        let Some((scene, entrypoint)) = machine.interrupt else {
            return;
        };
        if machine.in_interrupt || machine.interrupt_suspended || machine.halted {
            return;
        }
        match machine.archive.scenario(scene) {
            Ok(scenario) => {
                let ip = scenario.entrypoint(entrypoint).unwrap_or(0);
                machine
                    .stack
                    .push(Frame::new(scenario, ip, FrameKind::Farcall));
                machine.in_interrupt = true;
            }
            Err(error) => {
                machine.report(format!("interrupt handler: {error:#}"));
                machine.interrupt = None;
            }
        }
    }

    pub fn render(&mut self) -> Surface {
        let gfx = &mut self.machine.sys.gfx;
        let manual = gfx.draw_mode == crate::graphics::DrawMode::Manual;
        let refresh = std::mem::take(&mut gfx.refresh_requested);
        if manual && !refresh && gfx.transition_frame.is_none() {
            if let Some(frame) = &self.shown {
                return frame.clone();
            }
        }
        let frame = crate::screen::compose(&mut self.machine.sys);
        self.shown = Some(frame.clone());
        frame
    }

    pub fn finished(&self) -> bool {
        self.machine.halted || self.machine.sys.quit_requested
    }

    /// Whether the game is waiting for the player (text or a choice).
    pub fn waiting_for_input(&self) -> bool {
        matches!(
            self.machine.current_long_op().map(|op| op.name()),
            Some("pause" | "select" | "select_objbtn" | "slot menu" | "system menu" | "name entry")
        )
    }
}
