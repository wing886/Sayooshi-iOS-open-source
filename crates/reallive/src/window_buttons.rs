//! Text window buttons: the boxes a window frame declares
//! (`#WAKU.set.no.CLEAR_BOX` and friends), drawn from the frame's `BTN`
//! image and clicked while the game waits for the player. Each button's
//! patterns start at a fixed base (eight per button); the pattern within
//! is its state: 0 normal, 1 under the pointer, 2 pressed, 3 on (for the
//! skip and auto mode toggles), 4 on and under the pointer. (The voice
//! replay button's patterns are a guess: no known frame draws one.)

use crate::surface::{Rect, Surface};
use crate::system::System;
use crate::textout::Geometry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Clear,
    MsgBk,
    MsgBkLeft,
    MsgBkRight,
    KoePlay,
    ReadJump,
    AutoMode,
    /// `#WBCALL.nnn`: a script routine.
    Extra(usize),
}

impl Kind {
    const ALL: [Kind; 15] = [
        Kind::Clear,
        Kind::MsgBk,
        Kind::MsgBkLeft,
        Kind::MsgBkRight,
        Kind::Extra(0),
        Kind::Extra(1),
        Kind::Extra(2),
        Kind::Extra(3),
        Kind::Extra(4),
        Kind::Extra(5),
        Kind::Extra(6),
        Kind::Extra(7),
        Kind::KoePlay,
        Kind::ReadJump,
        Kind::AutoMode,
    ];

    fn name(self) -> String {
        match self {
            Kind::Clear => "CLEAR".into(),
            Kind::MsgBk => "MSGBK".into(),
            Kind::MsgBkLeft => "MSGBKLEFT".into(),
            Kind::MsgBkRight => "MSGBKRIGHT".into(),
            Kind::KoePlay => "KOEPLAY".into(),
            Kind::ReadJump => "READJUMP".into(),
            Kind::AutoMode => "AUTOMODE".into(),
            Kind::Extra(n) => format!("EXBTN_{n:03}"),
        }
    }

    /// The first pattern of the button in the `BTN` image.
    fn base_pattern(self) -> i32 {
        match self {
            Kind::Clear => 8,
            Kind::MsgBk => 16,
            Kind::MsgBkLeft => 24,
            Kind::MsgBkRight => 32,
            Kind::Extra(n) => 40 + 8 * n as i32,
            Kind::KoePlay => 120,
            Kind::ReadJump => 104,
            Kind::AutoMode => 112,
        }
    }

    /// The global switch (`#WINDOW_CLEAR_USE`...).
    fn global_use(self) -> &'static str {
        match self {
            Kind::Clear => "WINDOW_CLEAR_USE",
            Kind::MsgBk => "WINDOW_MSGBK_USE",
            Kind::MsgBkLeft => "WINDOW_MSGBKLEFT_USE",
            Kind::MsgBkRight => "WINDOW_MSGBKRIGHT_USE",
            Kind::KoePlay => "WINDOW_KOEPLAY_USE",
            Kind::ReadJump => "WINDOW_READJUMP_USE",
            Kind::AutoMode => "WINDOW_AUTOMODE_USE",
            Kind::Extra(_) => "WINDOW_EXBTN_USE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowButton {
    pub kind: Kind,
    pub rect: Rect,
}

/// Whether an extra button is switched on (`SET_WINDOW_EXBTN_MOD`,
/// `CCOM_SET_WINDOW_EXBTN_ON/OFF`).
pub fn extra_enabled(sys: &System, n: usize) -> bool {
    let key = crate::modules::sys_max::EXBTN_MOD + n as u16;
    sys.remembered.get(&key).copied().unwrap_or(1) != 0 && !sys.exbtn_hidden.contains(&n)
}

/// The buttons of window `index` (placed on `geometry`).
pub fn buttons(sys: &System, index: usize, geometry: &Geometry) -> Vec<WindowButton> {
    let Some(config) = sys.text.windows.get(index) else {
        return Vec::new();
    };
    let exe = &sys.gameexe;
    let prefix = format!(
        "WAKU.{:03}.{:03}.",
        config.waku_setno,
        config.waku_pattern(sys.settings.waku_all)
    );
    if exe.str(&format!("{prefix}BTN")).is_none_or(str::is_empty) {
        return Vec::new();
    }
    let window = geometry.window;
    let mut out = Vec::new();
    for kind in Kind::ALL {
        if exe.int(kind.global_use()).unwrap_or(1) == 0 {
            continue;
        }
        if config.buttons.get(&format!("{}_USE", kind.name())) == Some(&false) {
            continue;
        }
        if let Kind::Extra(n) = kind
            && !extra_enabled(sys, n)
        {
            continue;
        }
        let values = exe.ints(&format!("{prefix}{}_BOX", kind.name()));
        let [origin, x, y, w, h] = values[..] else {
            continue;
        };
        if w <= 0 || h <= 0 {
            continue;
        }
        // The box is measured from a corner of the window.
        let bx = match origin {
            1 | 3 => window.right() - x - w,
            _ => window.x + x,
        };
        let by = match origin {
            2 | 3 => window.bottom() - y - h,
            _ => window.y + y,
        };
        out.push(WindowButton {
            kind,
            rect: Rect::new(bx, by, w, h),
        });
    }
    out
}

fn inside(rect: Rect, (x, y): (i32, i32)) -> bool {
    x >= rect.x && y >= rect.y && x < rect.right() && y < rect.bottom()
}

fn toggled(sys: &System, kind: Kind) -> bool {
    match kind {
        Kind::ReadJump => sys.syscom.skip_mode,
        Kind::AutoMode => sys.settings.auto_mode,
        _ => false,
    }
}

/// Draws the buttons of window `index` at `offset`.
pub fn draw(
    sys: &mut System,
    frame: &mut Surface,
    index: usize,
    geometry: &Geometry,
    offset: (i32, i32),
    alpha: u8,
) {
    let list = buttons(sys, index, geometry);
    if list.is_empty() {
        return;
    }
    let config = &sys.text.windows[index];
    let key = format!(
        "WAKU.{:03}.{:03}.BTN",
        config.waku_setno,
        config.waku_pattern(sys.settings.waku_all)
    );
    let Some(name) = sys.gameexe.str(&key).map(str::to_owned) else {
        return;
    };
    let Some(image) = crate::textout::load_named(sys, &name) else {
        return;
    };
    let mouse = sys.input.mouse;
    let held = sys.input.keys_held.contains(&100);
    for button in list {
        let over = inside(button.rect, mouse);
        let state = match (toggled(sys, button.kind), over) {
            (true, true) => 4,
            (true, false) => 3,
            (false, true) if held => 2,
            (false, true) => 1,
            (false, false) => 0,
        };
        let pattern = button.kind.base_pattern() + state;
        // Patterns a frame leaves empty fall back to the plain one.
        let region = image.region(pattern);
        let pattern = if region.x2 > region.x1 {
            pattern
        } else {
            button.kind.base_pattern()
        };
        crate::textout::draw_image(
            frame,
            &image,
            pattern,
            button.rect.x + offset.0,
            button.rect.y + offset.1,
            alpha,
        );
    }
}

/// The button under the pointer in a visible window, if any.
pub fn hit(sys: &mut System) -> Option<Kind> {
    if sys.text.hidden_temporarily || sys.gfx.interface_hidden {
        return None;
    }
    let mouse = sys.input.mouse;
    for index in 0..sys.text.states.len() {
        if !sys.text.states[index].visible || sys.text.display_off[index] {
            continue;
        }
        let geometry = crate::textout::geometry(sys, index);
        if let Some(button) = buttons(sys, index, &geometry)
            .into_iter()
            .find(|b| inside(b.rect, mouse))
        {
            return Some(button.kind);
        }
    }
    None
}

/// Carries out a click on a button. Returns true when the machine jumped
/// (an extra button's routine was called).
pub fn press(machine: &mut crate::machine::Machine, kind: Kind) -> anyhow::Result<bool> {
    let sys = &mut machine.sys;
    match kind {
        Kind::Clear => sys.text.hidden_temporarily = true,
        Kind::MsgBk | Kind::MsgBkLeft => crate::backlog::open(sys),
        Kind::MsgBkRight => {}
        Kind::KoePlay => {
            let (koe, _) = sys.last_koe;
            if koe >= 0 {
                crate::modules::sound::koe_play(machine, koe, None);
            }
        }
        Kind::ReadJump => sys.syscom.skip_mode = !sys.syscom.skip_mode,
        Kind::AutoMode => sys.settings.auto_mode = !sys.settings.auto_mode,
        Kind::Extra(n) => {
            let values = machine.gameexe.ints(&format!("WBCALL.{n:03}"));
            let Some(&scene) = values.first() else {
                return Ok(false);
            };
            let entrypoint = values.get(1).copied().unwrap_or(0);
            let scenario = machine.archive.scenario(scene)?;
            let ip = scenario.entrypoint(entrypoint).unwrap_or(0);
            // The paused wait resumes when the routine returns
            // (`rtlButton`).
            machine.stack.push(crate::machine::Frame::new(
                scenario,
                ip,
                crate::machine::FrameKind::Farcall,
            ));
            machine.sys.in_menu = true;
            return Ok(true);
        }
    }
    Ok(false)
}
