//! Engine-provided menus: returning to the title, the system command menu,
//! save/load slot menus, name entry and text input boxes.

use anyhow::Result;

use crate::bytecode::Command;
use crate::machine::{Frame, FrameKind, Machine};
use crate::settings::syscom;

/// `ReturnMenu` / `MenuReturn`: restart at `#SEEN_MENU` with fresh local
/// state. `fade` fades the screen out first (the menu version).
pub fn return_to_menu(machine: &mut Machine, fade: bool) -> Result<()> {
    let scene = machine
        .gameexe
        .int("SEEN_MENU")
        .or_else(|| machine.gameexe.int("SEEN_START"))
        .or_else(|| machine.archive.first())
        .unwrap_or(1);
    let scenario = machine.archive.scenario(scene)?;
    let ip = scenario.entrypoint(0).unwrap_or(0);
    if fade {
        machine.sys.request_fade_out();
    }
    machine.reset_for_load();
    machine.memory.local = Default::default();
    machine.init_local_memory();
    machine.stack = vec![Frame::new(scenario, ip, FrameKind::Root)];
    machine.halted = false;
    machine.sys.in_menu = false;
    machine.mark_savepoint();
    Ok(())
}

/// `#CANCELCALL`: the scenario and entrypoint of a custom menu, if enabled.
pub fn cancel_call(machine: &Machine) -> Option<(i32, i32)> {
    if machine.gameexe.int("CANCELCALL_MOD").unwrap_or(0) == 0 {
        return None;
    }
    let values = machine.gameexe.ints("CANCELCALL");
    Some((*values.first()?, values.get(1).copied().unwrap_or(0)))
}

/// A right click or `ContextMenu()`: runs the `#CANCELCALL` routine or
/// opens the built-in system command menu.
pub fn open_context_menu(machine: &mut Machine) -> Result<()> {
    if !machine.sys.syscom.menu_enabled {
        return Ok(());
    }
    if cancel_call(machine).is_some() {
        // ContextMenu() has no effect when a custom menu is installed.
        return Ok(());
    }
    machine.sys.ui.request(crate::ui::Request::SyscomMenu);
    Ok(())
}

/// Invokes the custom menu (right click with `#CANCELCALL`).
pub fn invoke_cancel_call(machine: &mut Machine) -> Result<bool> {
    let Some((scene, entrypoint)) = cancel_call(machine) else {
        return Ok(false);
    };
    // The paused operation stays on the long-operation stack and resumes
    // when the menu returns with `rtlCancel`.
    let scenario = machine.archive.scenario(scene)?;
    let ip = scenario.entrypoint(entrypoint).unwrap_or(0);
    machine
        .stack
        .push(Frame::new(scenario, ip, FrameKind::Farcall));
    machine.sys.in_menu = true;
    Ok(true)
}

/// `InvokeSyscom(n[, value])`. Returns true when the machine jumped.
pub fn invoke_syscom(machine: &mut Machine, index: i32, value: Option<i32>) -> Result<bool> {
    match index {
        syscom::SAVE => open_slot_menu(machine, SlotMenuKind::Save)?,
        syscom::LOAD => open_slot_menu(machine, SlotMenuKind::Load)?,
        syscom::MENU_RETURN => {
            return_to_menu(machine, true)?;
            return Ok(true);
        }
        syscom::EXIT_GAME => {
            machine.sys.quit_requested = true;
            machine.halt();
            return Ok(true);
        }
        syscom::SHOW_BACKGROUND => crate::modules::msg::show_background(machine)?,
        syscom::RETURN_TO_PREVIOUS_SELECTION => {
            return crate::save::return_to_previous_selection(machine);
        }
        syscom::HIDE_MENU => {}
        syscom::SET_SKIP_MODE => {
            let on = value.map_or(!machine.sys.syscom.skip_mode, |value| value != 0);
            machine.sys.syscom.skip_mode = on;
        }
        syscom::AUTO_MODE => {
            let on = value.map_or(!machine.sys.settings.auto_mode, |value| value != 0);
            machine.sys.settings.auto_mode = on;
        }
        other => match value {
            Some(value) => crate::modules::sys::write_syscom(machine, other, value),
            None => machine
                .sys
                .ui
                .request(crate::ui::Request::SettingsDialog(other)),
        },
    }
    Ok(false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotMenuKind {
    /// `menu_save`: choose a slot and save.
    Save,
    /// `menu_load`: choose a slot and load.
    Load,
    /// `savemenu`: return the chosen slot in `store`.
    ChooseSave,
    /// `loadmenu`: return the chosen occupied slot in `store`.
    ChooseLoad,
}

impl SlotMenuKind {
    pub fn from_opcode(opcode: u16) -> Self {
        match opcode % 100 {
            0 => SlotMenuKind::Save,
            1 => SlotMenuKind::Load,
            2 => SlotMenuKind::ChooseSave,
            _ => SlotMenuKind::ChooseLoad,
        }
    }
}

pub fn open_slot_menu(machine: &mut Machine, kind: SlotMenuKind) -> Result<()> {
    machine.push_long_op(Box::new(crate::ui::SlotMenu::new(machine, kind)));
    Ok(())
}

/// `nwSingle`, `nwMulti` and the local variants.
pub fn name_entry(machine: &mut Machine, local: bool, fields: Vec<(i32, String)>) -> Result<()> {
    machine.push_long_op(Box::new(crate::ui::NameEntry::new(machine, local, fields)));
    Ok(())
}

/// `CreateInput`, `CloseInput`, `CloseAllInputs`, `FocusInput`,
/// `SetInput`, `GetInput`.
pub fn text_input(machine: &mut Machine, command: &Command) -> Result<()> {
    match command.op.opcode {
        1700 => {
            let values = machine.int_params_from(command, 0)?;
            let index = values.first().copied().unwrap_or(0);
            machine.sys.ui.create_input(index, &values[1..]);
        }
        1701 => {
            let index = machine.int_param(command, 0)?;
            machine.sys.ui.inputs.remove(&index);
        }
        1702 => machine.sys.ui.inputs.clear(),
        1703 => {
            let index = machine.int_param(command, 0)?;
            machine.sys.ui.focused_input = Some(index);
        }
        1710 => {
            let index = machine.int_param(command, 0)?;
            let text = machine.str_param(command, 1)?;
            if let Some(input) = machine.sys.ui.inputs.get_mut(&index) {
                input.text = text;
            }
        }
        1711 => {
            let index = machine.int_param(command, 0)?;
            let target = machine.str_target_param(command, 1)?;
            let text = machine
                .sys
                .ui
                .inputs
                .get(&index)
                .map(|input| input.text.clone())
                .unwrap_or_default();
            machine.write_string(target, text)?;
        }
        _ => {
            machine.unimplemented(command)?;
        }
    }
    Ok(())
}
