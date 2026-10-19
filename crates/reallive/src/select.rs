//! Selections: options listed in a text window (`select`, `select_w`),
//! `#SELBTN` buttons (`select_s`) and button objects (`select_objbtn`).
//!
//! All five option effects are honoured: `colour(n)`, `title` (shown but
//! not selectable), `hide`, `blank` (an empty unselectable slot) and
//! `cursor` (initial highlight). Each applies when its condition is absent
//! or true. The store register receives the option's position in the
//! source, hidden options included.

use anyhow::Result;

use crate::bytecode::{Select, select_effect};
use crate::input::{Button, InputEvent, Key};
use crate::machine::{LongOp, Machine};
use crate::nls::cell_width;
use crate::surface::{Rect, Surface};
use crate::system::System;
use crate::textout;

/// An option after its conditions were evaluated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    /// Position in the source (the value stored on selection).
    pub index: usize,
    pub text: String,
    pub enabled: bool,
    /// Occupies a slot without text.
    pub blank: bool,
    pub colour: Option<i32>,
}

/// Evaluates options; returns the visible ones and the initial cursor.
pub fn evaluate(machine: &mut Machine, select: &Select) -> Result<(Vec<Choice>, Option<usize>)> {
    let mut choices = Vec::new();
    let mut cursor = None;
    for (index, option) in select.options.iter().enumerate() {
        let text = machine.eval_str(&option.text)?;
        let mut choice = Choice {
            index,
            text,
            enabled: true,
            blank: false,
            colour: None,
        };
        let mut hidden = false;
        for condition in &option.conditions {
            let applies = match &condition.condition {
                Some(expression) => machine.eval_int(expression)? != 0,
                None => true,
            };
            if !applies {
                continue;
            }
            let argument = match &condition.argument {
                Some(expression) => Some(machine.eval_int(expression)?),
                None => None,
            };
            match condition.effect {
                select_effect::COLOUR => choice.colour = Some(argument.unwrap_or(0)),
                select_effect::TITLE => {
                    choice.enabled = false;
                    if let Some(colour) = argument {
                        choice.colour = Some(colour);
                    }
                }
                select_effect::HIDE => hidden = true,
                select_effect::BLANK => {
                    choice.blank = true;
                    choice.enabled = false;
                }
                select_effect::CURSOR => cursor = Some(choices.len()),
                other => machine.note_unimplemented(format!("select effect {:?}", other as char)),
            }
        }
        if !hidden {
            choices.push(choice);
        }
    }
    Ok((choices, cursor))
}

/// One laid-out option.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub choice: Choice,
    pub rect: Rect,
}

/// `#SELBTN.nnn` presentation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ButtonStyle {
    pub name: Option<String>,
    pub back: Option<String>,
    pub moji_size: i32,
    pub default_colour: i32,
    pub select_colour: i32,
    /// (pattern, dx, dy) for normal, highlighted, pushed, disabled.
    pub frames: [(i32, i32, i32); 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    /// In a text window; rects are relative to its text origin.
    Window(usize),
    /// `#SELBTN` buttons in screen coordinates.
    Buttons(ButtonStyle),
}

/// What the renderer shows while a selection is running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub layout: Layout,
    pub items: Vec<Item>,
    pub highlighted: Option<usize>,
    pub pressed: bool,
}

impl Selection {
    fn hit(&self, sys: &mut System, (mx, my): (i32, i32)) -> Option<usize> {
        let origin = match self.layout {
            Layout::Window(window) => {
                let geometry = textout::geometry(sys, window);
                (geometry.text.x, geometry.text.y)
            }
            Layout::Buttons(_) => (0, 0),
        };
        self.items.iter().position(|item| {
            let r = item.rect;
            let (x, y) = (mx - origin.0, my - origin.1);
            x >= r.x && y >= r.y && x < r.right() && y < r.bottom()
        })
    }

    fn step(&mut self, sys: &mut System) -> Step {
        let count = self.items.len();
        let selectable = |items: &[Item], i: usize| items[i].choice.enabled;
        let move_by = |items: &[Item], from: Option<usize>, delta: isize| -> Option<usize> {
            let mut at = from.map_or(if delta > 0 { -1 } else { count as isize }, |i| i as isize);
            for _ in 0..count {
                at = (at + delta).rem_euclid(count as isize);
                if selectable(items, at as usize) {
                    return Some(at as usize);
                }
            }
            from
        };
        let mouse = sys.input.mouse;
        let hovered = self.hit(sys, mouse).filter(|&i| selectable(&self.items, i));
        if hovered.is_some() && hovered != self.highlighted {
            sys.play_se(0);
            self.highlighted = hovered;
        }
        let events = std::mem::take(&mut sys.input.events);
        let mut rest = Vec::new();
        let mut result = Step::Continue;
        for event in events {
            match event {
                InputEvent::KeyDown(Key::Up | Key::Left) => {
                    self.highlighted = move_by(&self.items, self.highlighted, -1);
                }
                InputEvent::KeyDown(Key::Down | Key::Right | Key::Tab) => {
                    self.highlighted = move_by(&self.items, self.highlighted, 1);
                }
                InputEvent::KeyDown(Key::Enter | Key::Space) => {
                    if let Some(i) = self.highlighted.filter(|&i| selectable(&self.items, i)) {
                        result = Step::Chosen(i);
                    }
                }
                InputEvent::KeyDown(Key::Char(c @ '1'..='9')) => {
                    let n = c as usize - '1' as usize;
                    let nth = (0..count).filter(|&i| selectable(&self.items, i)).nth(n);
                    if let Some(i) = nth {
                        result = Step::Chosen(i);
                    }
                }
                InputEvent::Press(Button::Left) => {
                    self.pressed = true;
                    if let Some(i) = hovered {
                        self.highlighted = Some(i);
                    }
                }
                InputEvent::Release(Button::Left) => {
                    self.pressed = false;
                    if let Some(i) = hovered {
                        result = Step::Chosen(i);
                    }
                }
                InputEvent::Press(Button::Right) | InputEvent::KeyDown(Key::Escape) => {
                    result = Step::Menu;
                }
                InputEvent::Press(Button::WheelUp) | InputEvent::KeyDown(Key::PageUp) => {
                    crate::backlog::open(sys);
                }
                other => rest.push(other),
            }
            if result != Step::Continue {
                break;
            }
        }
        sys.input.events = rest;
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Continue,
    Chosen(usize),
    Menu,
}

fn lay_out_in_window(sys: &mut System, window: usize, choices: Vec<Choice>) -> Vec<Item> {
    let config = sys.text.windows[window].clone();
    let line = textout::line_height(&config);
    let cell = config.moji_size + config.moji_rep.0;
    let (_, rows) = config.moji_cnt;
    // Options go below any text already shown; clear if they do not fit.
    let state = &sys.text.states[window];
    let mut row = if state.chars.is_empty() {
        0
    } else {
        state.line + i32::from(state.x > 0)
    };
    if row + choices.len() as i32 > rows.max(1) {
        textout::clear_window(sys, window);
        row = 0;
    }
    let indent = if config.selcom_use {
        config.selcom_mojipos.max(0)
    } else {
        0
    };
    choices
        .into_iter()
        .enumerate()
        .map(|(i, choice)| {
            let width: i32 = choice
                .text
                .chars()
                .map(|c| cell_width(c) as i32 * cell / 2)
                .sum();
            let y = config.luby_size + (row + i as i32) * line;
            Item {
                rect: Rect::new(indent, y - config.moji_rep.1 / 2, width.max(cell), line),
                choice,
            }
        })
        .collect()
}

fn button_style(sys: &mut System, set: i32) -> (ButtonStyle, (i32, i32), (i32, i32), (bool, bool)) {
    let exe = sys.gameexe.clone();
    let key = |part: &str| format!("SELBTN.{set:03}.{part}");
    let pair = |part: &str| {
        let values = exe.ints(&key(part));
        (
            values.first().copied().unwrap_or(0),
            values.get(1).copied().unwrap_or(0),
        )
    };
    let frame = |part: &str| {
        let values = exe.ints(&key(part));
        (
            values.first().copied().unwrap_or(0),
            values.get(1).copied().unwrap_or(0),
            values.get(2).copied().unwrap_or(0),
        )
    };
    let named = |part: &str| {
        exe.str(&key(part))
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
    };
    let default_colour = exe.int(&key("MOJIDEFAULTCOL")).unwrap_or(0);
    let mut select_colour = exe.int(&key("MOJISELECTCOL")).unwrap_or(1);
    if select_colour == default_colour {
        // Highlighting in the default colour would be invisible.
        select_colour = 1;
    }
    if select_colour < 0 {
        select_colour = default_colour;
    }
    let centering = pair("CENTERING");
    let style = ButtonStyle {
        name: named("NAME"),
        back: named("BACK"),
        moji_size: exe.int(&key("MOJISIZE")).unwrap_or(24).max(1),
        default_colour,
        select_colour,
        frames: [
            frame("NORMAL"),
            frame("SELECT"),
            frame("PUSH"),
            frame("DONTSEL"),
        ],
    };
    (
        style,
        pair("BASEPOS"),
        pair("REPPOS"),
        (centering.0 != 0, centering.1 != 0),
    )
}

fn lay_out_buttons(sys: &mut System, set: i32, choices: Vec<Choice>) -> (ButtonStyle, Vec<Item>) {
    let (style, base, rep, centre) = button_style(sys, set);
    let size = style
        .back
        .as_deref()
        .and_then(|name| textout::load_named(sys, name))
        .map(|image| (image.width(), image.height()))
        .or_else(|| {
            let image = textout::load_named(sys, style.name.as_deref()?)?;
            let region = image.region(style.frames[0].0);
            Some((region.width(), region.height()))
        })
        .unwrap_or((style.moji_size * 12, style.moji_size * 3 / 2));
    let count = choices.len() as i32;
    let x = if centre.0 {
        (sys.gfx.width - ((count - 1) * rep.0 + size.0)) / 2
    } else {
        base.0
    };
    let y = if centre.1 {
        (sys.gfx.height - ((count - 1) * rep.1 + size.1)) / 2
    } else {
        base.1
    };
    let items = choices
        .into_iter()
        .enumerate()
        .map(|(i, choice)| Item {
            rect: Rect::new(x + rep.0 * i as i32, y + rep.1 * i as i32, size.0, size.1),
            choice,
        })
        .collect();
    (style, items)
}

/// Runs a selection built from a select element.
#[derive(Debug)]
pub struct SelectOp {
    selection: Selection,
    /// `select_w`: the window to restore afterwards.
    restore_window: Option<usize>,
}

impl SelectOp {
    fn new(
        machine: &mut Machine,
        layout: Layout,
        items: Vec<Item>,
        cursor: Option<usize>,
        restore_window: Option<usize>,
    ) -> Self {
        let sys = &mut machine.sys;
        // Skip mode stops at choices.
        sys.syscom.skip_mode = false;
        let highlighted = cursor.or_else(|| items.iter().position(|item| item.choice.enabled));
        let selection = Selection {
            layout,
            items,
            highlighted: if sys.input.key_mouse || cursor.is_some() {
                highlighted
            } else {
                None
            },
            pressed: false,
        };
        sys.selection = Some(selection.clone());
        Self {
            selection,
            restore_window,
        }
    }
}

impl LongOp for SelectOp {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        if machine.sys.text.backlog_view.is_some() {
            crate::backlog::handle_input(&mut machine.sys);
            return Ok(false);
        }
        let step = self.selection.step(&mut machine.sys);
        machine.sys.selection = Some(self.selection.clone());
        match step {
            Step::Continue => Ok(false),
            Step::Menu => {
                if !crate::modules::menu::invoke_cancel_call(machine)? {
                    crate::modules::menu::open_context_menu(machine)?;
                }
                Ok(false)
            }
            Step::Chosen(i) => {
                let choice = self.selection.items[i].choice.clone();
                machine.sys.play_se(1);
                machine.sys.selection = None;
                if machine.selpoint_auto {
                    crate::save::remember_selection(machine);
                }
                if machine.sys.read_jump_cancel {
                    machine.sys.syscom.skip_mode = false;
                }
                machine.store = choice.index as i32;
                let sys = &mut machine.sys;
                sys.text.log.push(format!("→ {}", choice.text));
                sys.text
                    .current_page
                    .lines
                    .push(format!("→ {}", choice.text));
                sys.text.commit_page();
                match (&self.selection.layout, self.restore_window) {
                    (Layout::Window(window), Some(restore)) => {
                        textout::close_window(sys, *window);
                        sys.text.active = restore;
                    }
                    (Layout::Window(window), None) => textout::clear_window(sys, *window),
                    _ => {}
                }
                Ok(true)
            }
        }
    }

    fn name(&self) -> &'static str {
        "select"
    }
}

/// `select` / `select_w`: options in a text window.
pub fn window_select(machine: &mut Machine, select: &Select, separate_window: bool) -> Result<()> {
    let (choices, cursor) = evaluate(machine, select)?;
    let active = machine.sys.text.active;
    let mut restore = None;
    let mut window = active;
    if separate_window {
        let computed = match &select.window {
            Some(expression) => machine.eval_int(expression)?,
            None => -1,
        };
        let default = machine.gameexe.int("DEFAULT_SEL_WINDOW").unwrap_or(-1);
        let chosen = if computed >= 0 { computed } else { default };
        if chosen >= 0 && (chosen as usize) < machine.sys.text.states.len() {
            window = chosen as usize;
            restore = Some(active);
            for index in 0..machine.sys.text.states.len() {
                if index != window {
                    textout::close_window(&mut machine.sys, index);
                }
            }
            textout::clear_window(&mut machine.sys, window);
            machine.sys.text.active = window;
        }
    }
    textout::open_window(&mut machine.sys, window);
    let items = lay_out_in_window(&mut machine.sys, window, choices);
    let op = SelectOp::new(machine, Layout::Window(window), items, cursor, restore);
    machine.push_long_op(Box::new(op));
    Ok(())
}

/// `select_s`: `#SELBTN` buttons.
pub fn button_select(machine: &mut Machine, select: &Select) -> Result<()> {
    let (choices, cursor) = evaluate(machine, select)?;
    let requested = match &select.window {
        Some(expression) => machine.eval_int(expression)?,
        None => 0,
    };
    let set = if machine
        .gameexe
        .filter(&format!("SELBTN.{requested:03}"))
        .next()
        .is_some()
    {
        requested
    } else {
        0
    };
    let (style, items) = lay_out_buttons(&mut machine.sys, set, choices);
    let op = SelectOp::new(machine, Layout::Buttons(style), items, cursor, None);
    machine.push_long_op(Box::new(op));
    Ok(())
}

/// Draws the options of a window selection (called by the text renderer).
pub fn draw_window_items(
    sys: &mut System,
    frame: &mut Surface,
    window: usize,
    origin: (i32, i32),
    alpha: u8,
) {
    let Some(selection) = sys.selection.clone() else {
        return;
    };
    if selection.layout != Layout::Window(window) {
        return;
    }
    let config = sys.text.windows[window].clone();
    let size = config.moji_size;
    let cell = size + config.moji_rep.0;
    for (i, item) in selection.items.iter().enumerate() {
        if item.choice.blank {
            continue;
        }
        let highlighted = selection.highlighted == Some(i);
        let r = item.rect;
        if highlighted {
            let bar = Rect::new(origin.0 + r.x - 2, origin.1 + r.y, r.w + 4, r.h);
            let attr = crate::settings::WindowAttr {
                r: 255,
                g: 255,
                b: 255,
                alpha: 64,
                filter: 1,
            };
            textout::draw_backing(frame, bar, None, attr, alpha);
        }
        let colour_index = item.choice.colour.unwrap_or(0);
        let mut colour = textout::colour_index(sys, colour_index);
        if !item.choice.enabled
            || (config.selcom_mojidark != 0 && !highlighted && selection.highlighted.is_some())
        {
            colour = colour.map(|c| c / 2);
        }
        let mut x = origin.0 + r.x;
        let y = origin.1 + r.y + config.moji_rep.1 / 2;
        let shadow = config.moji_shadow.then_some([0, 0, 0]);
        for c in item.choice.text.chars() {
            textout::draw_glyph(frame, sys, c, x, y, size, colour, shadow, alpha);
            x += cell_width(c) as i32 * cell / 2;
        }
    }
}

/// Draws `#SELBTN` buttons.
pub fn draw_buttons(sys: &mut System, frame: &mut Surface) {
    if sys.gfx.ccom_hide_buttons {
        return;
    }
    let Some(selection) = sys.selection.clone() else {
        return;
    };
    let Layout::Buttons(style) = &selection.layout else {
        return;
    };
    let attr = sys.settings.window_attr;
    let name = style
        .name
        .as_deref()
        .and_then(|name| textout::load_named(sys, name));
    let back = style
        .back
        .as_deref()
        .and_then(|back| textout::load_named(sys, back));
    for (i, item) in selection.items.iter().enumerate() {
        if item.choice.blank {
            continue;
        }
        let r = item.rect;
        let highlighted = selection.highlighted == Some(i);
        let state = if !item.choice.enabled {
            3
        } else if highlighted && selection.pressed {
            2
        } else if highlighted {
            1
        } else {
            0
        };
        if let Some(back) = &back {
            textout::draw_backing(frame, r, Some(&back.surface), attr, 255);
        }
        if let Some(name) = &name {
            let (pattern, dx, dy) = style.frames[state];
            textout::draw_image(frame, name, pattern, r.x + dx, r.y + dy, 255);
        } else if back.is_none() {
            textout::draw_backing(frame, r, None, attr, 255);
        }
        let colour_index = match item.choice.colour {
            Some(colour) if !highlighted || !item.choice.enabled => colour,
            _ if highlighted => style.select_colour,
            _ => style.default_colour,
        };
        let colour = textout::colour_index(sys, colour_index);
        let shadow = textout::colour_index(sys, 255);
        let size = style.moji_size;
        let width: i32 = item
            .choice
            .text
            .chars()
            .map(|c| cell_width(c) as i32 * size / 2)
            .sum();
        let mut x = r.x + (r.w - width) / 2;
        let y = r.y + (r.h - size) / 2;
        for c in item.choice.text.chars() {
            textout::draw_glyph(frame, sys, c, x, y, size, colour, Some(shadow), 255);
            x += cell_width(c) as i32 * size / 2;
        }
    }
}

/// `select_objbtn(group)`: waits for a click on a button object.
#[derive(Debug)]
pub struct ObjectButtonSelect {
    group: i32,
    cancelable: bool,
    hovered: Option<(usize, i32)>,
    pressed: bool,
}

impl ObjectButtonSelect {
    pub fn new(group: i32, cancelable: bool) -> Self {
        Self {
            group,
            cancelable,
            hovered: None,
            pressed: false,
        }
    }

    fn buttons(&self, sys: &mut System) -> Vec<(usize, i32, Rect)> {
        group_buttons(sys, self.group)
    }

    fn set_state(sys: &mut System, slot: usize, state: i32) {
        set_button_state(sys, slot, state);
    }
}

/// Button objects of a group, topmost first: (object slot, button number,
/// bounds).
fn group_buttons(sys: &mut System, group: i32) -> Vec<(usize, i32, Rect)> {
    let now = sys.now();
    let gfx = &mut sys.gfx;
    let mut out = Vec::new();
    for (slot, object) in gfx.fg.iter().enumerate() {
        let Some(object) = object else { continue };
        let b = &object.params.button;
        if b.is_button == 0 || b.group != group || !object.params.visible {
            continue;
        }
        let bounds = object.bounds(now, &mut gfx.fonts, &gfx.colours, (0, 0));
        out.push((slot, b.number, bounds));
    }
    out.sort_by_key(|&(slot, ..)| {
        let p = &gfx.fg[slot].as_ref().expect("present").params;
        std::cmp::Reverse((p.z_order, p.z_layer, p.z_depth, slot))
    });
    out
}

fn set_button_state(sys: &mut System, slot: usize, state: i32) {
    if let Some(object) = sys.gfx.fg.get_mut(slot).and_then(Option::as_mut) {
        object.params.button.state = state;
    }
}

/// `select_btnobjstart`..`select_btnobjend`: a button-object selection
/// the script polls (`select_btnobjnow_hit`, `select_btnobjnow_decide`)
/// instead of waiting for it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PolledButtons {
    pub group: i32,
    hovered: Option<(usize, i32)>,
    /// The button held down, and whether with the right button.
    pushed: Option<(i32, bool)>,
    /// A click on a button not yet collected by `now_decide`, and whether
    /// it was a right click.
    decided: Option<(i32, bool)>,
    /// `select_btnobjstart_with_right_click`: right clicks work buttons too.
    pub right_click: bool,
}

impl PolledButtons {
    pub fn new(group: i32) -> Self {
        Self {
            group,
            ..Self::default()
        }
    }

    /// Per frame: hover highlighting and clicks.
    pub fn update(&mut self, sys: &mut System) {
        let buttons = group_buttons(sys, self.group);
        let (mx, my) = sys.input.mouse;
        let hit = buttons
            .iter()
            .find(|(_, _, r)| mx >= r.x && my >= r.y && mx < r.right() && my < r.bottom())
            .map(|&(slot, number, _)| (slot, number));
        if hit != self.hovered {
            if let Some((slot, _)) = self.hovered {
                set_button_state(sys, slot, 0);
            }
            if let Some((slot, _)) = hit {
                sys.play_se(0);
                set_button_state(sys, slot, 1);
            }
            self.hovered = hit;
        }
        let Some((slot, number)) = hit else {
            self.pushed = None;
            return;
        };
        if self.pushed.is_some_and(|(n, _)| n != number) {
            self.pushed = None;
        }
        let mut rest = Vec::new();
        for event in std::mem::take(&mut sys.input.events) {
            let right = match event {
                InputEvent::Press(Button::Left) | InputEvent::Release(Button::Left) => false,
                InputEvent::Press(Button::Right) | InputEvent::Release(Button::Right)
                    if self.right_click =>
                {
                    true
                }
                other => {
                    rest.push(other);
                    continue;
                }
            };
            if matches!(event, InputEvent::Press(_)) {
                set_button_state(sys, slot, 2);
                self.pushed = Some((number, right));
            } else if self.decided.is_none() {
                set_button_state(sys, slot, 1);
                sys.play_se(1);
                self.pushed = None;
                self.decided = Some((number, right));
            }
        }
        sys.input.events = rest;
    }

    pub fn hovered(&self) -> i32 {
        self.hovered.map_or(-1, |(_, number)| number)
    }

    /// The button held down (`None`: either mouse button, else only the
    /// left or right one), or -1.
    pub fn pushed(&self, right: Option<bool>) -> i32 {
        match self.pushed {
            Some((number, r)) if right.is_none_or(|want| want == r) => number,
            _ => -1,
        }
    }

    /// Collects a click (`None`: with either button), or -1.
    pub fn take_decided(&mut self, right: Option<bool>) -> i32 {
        match self.decided {
            Some((number, r)) if right.is_none_or(|want| want == r) => {
                self.decided = None;
                number
            }
            _ => -1,
        }
    }

    /// Returns the highlighted button to normal.
    pub fn finish(&mut self, sys: &mut System) {
        if let Some((slot, _)) = self.hovered.take() {
            set_button_state(sys, slot, 0);
        }
    }
}

impl LongOp for ObjectButtonSelect {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let sys = &mut machine.sys;
        let buttons = self.buttons(sys);
        let (mx, my) = sys.input.mouse;
        let hit = buttons
            .iter()
            .find(|(_, _, r)| mx >= r.x && my >= r.y && mx < r.right() && my < r.bottom())
            .map(|&(slot, number, _)| (slot, number));
        if hit != self.hovered {
            if let Some((slot, _)) = self.hovered {
                Self::set_state(sys, slot, 0);
            }
            if let Some((slot, _)) = hit {
                sys.play_se(0);
                Self::set_state(sys, slot, 1);
            }
            self.hovered = hit;
        }
        let events = std::mem::take(&mut sys.input.events);
        let mut rest = Vec::new();
        let mut result = None;
        for event in events {
            match event {
                InputEvent::Press(Button::Left) => {
                    self.pressed = true;
                    if let Some((slot, _)) = hit {
                        Self::set_state(sys, slot, 2);
                    }
                }
                InputEvent::Release(Button::Left) => {
                    self.pressed = false;
                    if let Some((slot, number)) = hit {
                        Self::set_state(sys, slot, 0);
                        result = Some(number);
                    }
                }
                InputEvent::Press(Button::Right) | InputEvent::KeyDown(Key::Escape)
                    if self.cancelable =>
                {
                    result = Some(-1);
                }
                other => rest.push(other),
            }
            if result.is_some() {
                break;
            }
        }
        sys.input.events = rest;
        match result {
            Some(number) => {
                if number >= 0 {
                    sys.play_se(1);
                }
                if let Some((slot, _)) = self.hovered {
                    Self::set_state(sys, slot, 0);
                }
                machine.store = number;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn name(&self) -> &'static str {
        "select_objbtn"
    }
}
