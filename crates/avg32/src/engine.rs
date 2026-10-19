//! The public AVG32 engine: a [`Scenario`] driving a [`System`], advanced
//! once per frame by the host.

use std::path::Path;

use anyhow::Result;

use crate::game::Avg32Game;
use crate::menu::SystemAction;
use crate::nls::Nls;
use crate::scenario::Scenario;
use crate::system::{Clock, HostRequest, Key, System};

#[derive(Debug, Clone, Copy)]
pub struct EngineOptions {
    /// Play audio through the default output device.
    pub audio: bool,
    /// Drive time from [`Avg32Engine::advance_clock`] instead of the wall
    /// clock (headless tools and tests).
    pub virtual_clock: bool,
    /// Read and write `SAVE.INI`.
    pub persist: bool,
    /// Text encoding of the scenario and configuration (Shift-JIS by
    /// default). It applies to the whole process.
    pub nls: Nls,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            audio: true,
            virtual_clock: false,
            persist: true,
            nls: Nls::default(),
        }
    }
}

#[derive(Debug)]
pub struct Avg32Engine {
    sys: System,
    scn: Scenario,
    menu_loading: bool,
}

impl Avg32Engine {
    pub fn open(root: impl AsRef<Path>, options: EngineOptions) -> Result<Self> {
        crate::nls::set(options.nls);
        let game = Avg32Game::open(root)?;
        let clock = if options.virtual_clock {
            Clock::Virtual(0)
        } else {
            Clock::Real(web_time::Instant::now())
        };
        let mut sys = System::new(game, clock, options.audio, options.persist);
        let start = sys.ini().start_seen;
        let scn = Scenario::new(&mut sys, start);
        Ok(Self {
            sys,
            scn,
            menu_loading: false,
        })
    }

    /// Starts at another scene (debugging and tools).
    pub fn jump_to_scene(&mut self, seen: i32) {
        self.sys.reset();
        self.scn.reset(&mut self.sys, seen, 0);
    }

    pub fn system(&self) -> &System {
        &self.sys
    }

    pub fn system_mut(&mut self) -> &mut System {
        &mut self.sys
    }

    pub fn scenario(&self) -> &Scenario {
        &self.scn
    }

    pub fn running(&self) -> bool {
        self.sys.running
    }

    pub fn seed_random(&mut self, seed: u64) {
        self.sys.seed_random(seed);
    }

    /// One frame of engine work.
    pub fn tick(&mut self) {
        if !self.sys.running {
            return;
        }
        if let Some(action) = self.sys.pending_action.take() {
            match action {
                SystemAction::Save(slot) => {
                    let (seen, pos) = (self.scn.last_save_seen, self.scn.last_save_pos);
                    self.sys.save(slot, seen, pos);
                }
                SystemAction::Load(slot) => {
                    if let Some((seen, pos)) = self.sys.load(slot) {
                        self.sys.reset();
                        self.scn.reset(&mut self.sys, seen, pos);
                        self.menu_loading = true;
                    }
                }
                SystemAction::ReturnToMenu => {
                    self.sys.reset();
                    self.sys.sound.bgm_stop(0);
                    let seen = self.sys.ini().menu_seen;
                    self.scn.reset(&mut self.sys, seen, 0);
                }
            }
        }
        crate::menu::menu_hover(&mut self.sys);
        let modal_name_input = self
            .sys
            .name_input
            .as_ref()
            .is_some_and(|input| input.inline_box.is_none());
        let picker = self
            .sys
            .menu
            .as_ref()
            .is_some_and(|menu| menu.kind == crate::menu::MenuKind::LoadPicker);
        if self.sys.menu.is_some() && !picker {
            return;
        }
        if modal_name_input || self.sys.hide_window || self.sys.backlog_view.is_some() {
            return;
        }
        if self.menu_loading {
            if self.sys.loading_proc() {
                self.menu_loading = false;
            }
            return;
        }
        self.scn.tick_background(&mut self.sys);
        self.scn.run(&mut self.sys);
    }

    /// The frame to present (640x480 RGBA8).
    pub fn frame_rgba(&mut self) -> Vec<u8> {
        self.sys.frame_rgba()
    }

    pub fn window_title(&self) -> &str {
        &self.sys.window_title
    }

    /// Whether the host should show its own pointer (hidden while the game
    /// hides the cursor or draws its own `CUR16M` cursor).
    pub fn cursor_visible(&self) -> bool {
        self.sys.mouse.visible && self.sys.cursor.is_none()
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.sys.mouse_move(x, y);
    }

    pub fn mouse_up(&mut self, right: bool) {
        self.sys.mouse_up(right);
    }

    pub fn key_down(&mut self, key: Key) {
        self.sys.key_down(key);
    }

    pub fn text_input(&mut self, text: &str) {
        self.sys.text_input(text);
    }

    /// Mouse wheel: up opens and scrolls back the message backlog.
    pub fn wheel(&mut self, up: bool) {
        self.sys.wheel(up);
    }

    /// Hold-to-skip (Shift/Ctrl in the original).
    pub fn set_skip(&mut self, held: bool) {
        self.sys.skip_key = held;
    }

    pub fn advance_clock(&mut self, milliseconds: u64) {
        self.sys.clock.advance(milliseconds);
    }

    pub fn take_requests(&mut self) -> Vec<HostRequest> {
        std::mem::take(&mut self.sys.requests)
    }

    pub fn take_warnings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.sys.warnings)
    }

    /// Saves to a slot (0-based) at the last save point.
    pub fn save(&mut self, slot: i32) {
        self.sys.pending_action = Some(SystemAction::Save(slot));
    }

    pub fn load(&mut self, slot: i32) {
        self.sys.pending_action = Some(SystemAction::Load(slot));
    }

    pub fn shutdown(&mut self) {
        self.sys.save_global_flags();
        self.sys.sound.stop_all();
        self.sys.running = false;
    }
}
