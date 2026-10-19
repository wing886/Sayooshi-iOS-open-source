//! Saved games.
//!
//! A slot file stores the state of the last savepoint: local memory, the
//! call stack, and the state of each subsystem in named sections. The
//! global file stores global memory, read-text flags and settings.

use std::path::PathBuf;

use anyhow::{Context, Result, anyhow, bail};

use crate::bytecode::Command;
use crate::expr::Expr;
use crate::machine::{Frame, FrameKind, Machine};
use crate::memory::{LocalMemory, Memory};
use crate::serial::{Reader, Writer, fill};

const SLOT_MAGIC: &[u8; 8] = b"RLRSSAV1";
const GLOBAL_MAGIC: &[u8; 8] = b"RLRSGLB1";

/// Date and title of a save.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveHeader {
    pub title: String,
    /// y, m, d, weekday, hh, mm, ss, ms.
    pub date: [i32; 8],
    pub scene: i32,
    pub line: i32,
}

pub fn save_dir(machine: &Machine) -> PathBuf {
    if let Some(dir) = &machine.sys.options.save_dir {
        return dir.clone();
    }
    if let Some(dir) = std::env::var_os("REALLIVE_SAVE_DIR") {
        return PathBuf::from(dir);
    }
    machine.sys.root.join("savedata_rs")
}

pub fn slot_path(machine: &Machine, slot: i32) -> PathBuf {
    save_dir(machine).join(format!("save{slot:03}.sav"))
}

fn global_path(machine: &Machine) -> PathBuf {
    save_dir(machine).join("global.sav")
}

fn now_date() -> [i32; 8] {
    use chrono::{Datelike, Local, Timelike};
    let now = Local::now();
    [
        now.year(),
        now.month() as i32,
        now.day() as i32,
        now.weekday().num_days_from_sunday() as i32,
        now.hour() as i32,
        now.minute() as i32,
        now.second() as i32,
        (now.nanosecond() / 1_000_000).min(999) as i32,
    ]
}

fn write_local(w: &mut Writer, local: &LocalMemory) {
    for bank in &local.ints {
        w.i32s(bank);
    }
    w.strs(&local.str_s);
    w.strs(&local.local_names);
}

fn read_local(r: &mut Reader, local: &mut LocalMemory) -> Result<()> {
    for bank in &mut local.ints {
        fill(bank, &r.i32s()?);
    }
    fill(&mut local.str_s, &r.strs()?);
    fill(&mut local.local_names, &r.strs()?);
    Ok(())
}

fn write_stack(w: &mut Writer, stack: &[Frame]) {
    w.len(stack.len());
    for frame in stack {
        w.i32(frame.scenario.number);
        w.u32(frame.ip as u32);
        w.u8(match frame.kind {
            FrameKind::Root => 0,
            FrameKind::Gosub => 1,
            FrameKind::Farcall => 2,
        });
        w.i32s(&frame.vars.int_l);
        w.strs(&frame.vars.str_k);
    }
}

fn read_stack(r: &mut Reader, machine: &Machine) -> Result<Vec<Frame>> {
    let count = r.len()?;
    let mut stack = Vec::with_capacity(count);
    for _ in 0..count {
        let scene = r.i32()?;
        let ip = r.u32()? as usize;
        let kind = match r.u8()? {
            1 => FrameKind::Gosub,
            2 => FrameKind::Farcall,
            _ => FrameKind::Root,
        };
        let scenario = machine.archive.scenario(scene)?;
        let mut frame = Frame::new(scenario, ip, kind);
        fill(&mut frame.vars.int_l, &r.i32s()?);
        frame.vars.str_k = r.strs()?;
        stack.push(frame);
    }
    if stack.is_empty() {
        bail!("reallive: the save has an empty call stack");
    }
    Ok(stack)
}

/// Serializes the state of the last savepoint.
pub fn serialize_slot(machine: &Machine) -> Vec<u8> {
    let mut w = Writer::new();
    w.bytes.extend_from_slice(SLOT_MAGIC);
    w.section("header", |w| {
        w.str(&machine.sys.title);
        for value in now_date() {
            w.i32(value);
        }
        w.i32(machine.scene_number());
        w.i32(machine.line);
    });
    w.section("memory", |w| write_local(w, &machine.memory.savepoint));
    w.section("stack", |w| write_stack(w, &machine.savepoint_stack));
    w.section("system", |w| {
        w.str(&machine.sys.title);
        w.str(&machine.sys.default_grp);
        w.str(&machine.sys.default_bgr);
        w.bool(machine.sys.syscom.skip_mode);
    });
    machine.save_subsystems(&mut w);
    let extra = SlotExtra {
        message: current_message(machine),
        ..SlotExtra::default()
    };
    w.section("extra", |w| extra.write(w));
    if let Some(thumbnail) = &machine.sys.gfx.thumbnail {
        let small = thumbnail_of(thumbnail);
        w.section("thumbnail", |w| {
            w.i32(small.width);
            w.i32(small.height);
            w.bytes(&small.rgba);
        });
    }
    w.bytes
}

/// Saved pictures are a quarter of the screen's size.
fn thumbnail_of(screen: &crate::surface::Surface) -> crate::surface::Surface {
    use crate::surface::{Blend, Rect, Surface};
    let (w, h) = ((screen.width / 4).max(1), (screen.height / 4).max(1));
    let mut small = Surface::new(w, h);
    small.stretch_blit(
        screen,
        Rect::new(0, 0, screen.width, screen.height),
        Rect::new(0, 0, w, h),
        255,
        Blend::Copy,
    );
    small
}

/// The picture saved with slot `slot` (`OBJ*LOADTHUMBNAIL`).
pub fn slot_thumbnail(machine: &Machine, slot: i32) -> Option<crate::surface::Surface> {
    let bytes = slot_bytes(machine, slot)?;
    let sections = slot_sections(&bytes).ok()?;
    let (_, body) = sections.into_iter().find(|(name, _)| name == "thumbnail")?;
    let mut r = Reader::new(body);
    let (width, height) = (r.i32().ok()?, r.i32().ok()?);
    let rgba = r.bytes().ok()?.to_vec();
    (rgba.len() == (width.max(0) * height.max(0) * 4) as usize).then_some(crate::surface::Surface {
        width,
        height,
        rgba,
    })
}

/// What RealLiveMax keeps with a save besides the game: three comments,
/// the text on screen when it was made and some script values
/// (`SET_SAVE_COMMENT*`, `GET_SAVE_MESSAGE`, `GET_SAVE_VALUE`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlotExtra {
    pub comments: [String; 3],
    pub message: String,
    pub values: Vec<i32>,
}

impl SlotExtra {
    fn write(&self, w: &mut Writer) {
        for comment in &self.comments {
            w.str(comment);
        }
        w.str(&self.message);
        w.i32s(&self.values);
    }

    fn read(r: &mut Reader) -> Result<Self> {
        Ok(Self {
            comments: [r.str()?, r.str()?, r.str()?],
            message: r.str()?,
            values: r.i32s()?,
        })
    }
}

/// The text of the page being read (`GET_SAVE_MESSAGE_NOW`).
pub fn current_message(machine: &Machine) -> String {
    machine.sys.text.current_page.lines.join("")
}

pub fn slot_extra(machine: &Machine, slot: i32) -> Option<SlotExtra> {
    let bytes = slot_bytes(machine, slot)?;
    let sections = slot_sections(&bytes).ok()?;
    match sections.into_iter().find(|(name, _)| name == "extra") {
        Some((_, body)) => SlotExtra::read(&mut Reader::new(body)).ok(),
        None => Some(SlotExtra::default()),
    }
}

fn write_slot_bytes(machine: &mut Machine, slot: i32, bytes: Vec<u8>) -> Result<()> {
    if !machine.sys.options.persist {
        machine.saved_in_memory.insert(slot, bytes);
        return Ok(());
    }
    let path = slot_path(machine, slot);
    game_fs::create_dir_all(path.parent().expect("slot files live in a directory"))?;
    game_fs::write(&path, bytes).with_context(|| format!("failed to write {}", path.display()))
}

/// Replaces the extra data of an existing save.
pub fn set_slot_extra(machine: &mut Machine, slot: i32, extra: &SlotExtra) -> Result<()> {
    let Some(bytes) = slot_bytes(machine, slot) else {
        return Ok(());
    };
    let mut w = Writer::new();
    w.bytes.extend_from_slice(SLOT_MAGIC);
    for (name, body) in slot_sections(&bytes)? {
        if name != "extra" {
            w.str(&name);
            w.bytes(body);
        }
    }
    w.section("extra", |w| extra.write(w));
    write_slot_bytes(machine, slot, w.bytes)
}

/// `DELETE_SAVEDATA`.
pub fn delete_slot(machine: &mut Machine, slot: i32) -> Result<()> {
    machine.saved_in_memory.remove(&slot);
    let path = slot_path(machine, slot);
    if machine.sys.options.persist && game_fs::is_file(&path) {
        game_fs::remove_file(path)?;
    }
    Ok(())
}

/// `COPY_SAVEDATA` / `SWAP_SAVEDATA`.
pub fn copy_slot(machine: &mut Machine, from: i32, to: i32, swap: bool) -> Result<()> {
    let source = slot_bytes(machine, from);
    let dest = slot_bytes(machine, to);
    match source {
        Some(bytes) => write_slot_bytes(machine, to, bytes)?,
        None => delete_slot(machine, to)?,
    }
    if swap {
        match dest {
            Some(bytes) => write_slot_bytes(machine, from, bytes)?,
            None => delete_slot(machine, from)?,
        }
    }
    Ok(())
}

/// The header of a slot, saved to disk or kept in memory.
pub fn any_slot_header(machine: &Machine, slot: i32) -> Option<SaveHeader> {
    read_header(&slot_bytes(machine, slot)?).ok()
}

fn slot_sections(bytes: &[u8]) -> Result<Vec<(String, &[u8])>> {
    let body = bytes
        .strip_prefix(SLOT_MAGIC.as_slice())
        .ok_or_else(|| anyhow!("reallive: not a save file"))?;
    Reader::new(body).sections()
}

pub fn read_header(bytes: &[u8]) -> Result<SaveHeader> {
    for (name, body) in slot_sections(bytes)? {
        if name == "header" {
            let mut r = Reader::new(body);
            let title = r.str()?;
            let mut date = [0; 8];
            for value in &mut date {
                *value = r.i32()?;
            }
            return Ok(SaveHeader {
                title,
                date,
                scene: r.i32()?,
                line: r.i32()?,
            });
        }
    }
    bail!("reallive: the save has no header")
}

pub fn slot_header(machine: &Machine, slot: i32) -> Option<SaveHeader> {
    let bytes = game_fs::read(slot_path(machine, slot)).ok()?;
    read_header(&bytes).ok()
}

/// The local memory stored in a slot (`GetSaveFlag`).
pub fn slot_memory(machine: &Machine, slot: i32) -> Option<LocalMemory> {
    let bytes = game_fs::read(slot_path(machine, slot)).ok()?;
    let sections = slot_sections(&bytes).ok()?;
    let (_, body) = sections.into_iter().find(|(name, _)| name == "memory")?;
    let mut local = LocalMemory::default();
    read_local(&mut Reader::new(body), &mut local).ok()?;
    Some(local)
}

pub fn save_slot(machine: &mut Machine, slot: i32) -> Result<()> {
    // Without `MAKE_THUMBNAIL`, the picture is the screen as it is now.
    let made = machine.sys.gfx.thumbnail.is_some();
    if !made {
        let frame = crate::screen::compose_layers(&mut machine.sys, true);
        machine.sys.gfx.thumbnail = Some(std::rc::Rc::new(frame));
    }
    let result = save_slot_inner(machine, slot);
    if !made {
        machine.sys.gfx.thumbnail = None;
    }
    result
}

fn save_slot_inner(machine: &mut Machine, slot: i32) -> Result<()> {
    if !machine.sys.options.persist {
        machine
            .saved_in_memory
            .insert(slot, serialize_slot(machine));
        machine.latest_save = slot;
        return Ok(());
    }
    let path = slot_path(machine, slot);
    game_fs::create_dir_all(path.parent().expect("slot files live in a directory"))?;
    game_fs::write(&path, serialize_slot(machine))
        .with_context(|| format!("failed to write {}", path.display()))?;
    machine.latest_save = slot;
    save_global(machine)?;
    Ok(())
}

fn slot_bytes(machine: &Machine, slot: i32) -> Option<Vec<u8>> {
    if let Some(bytes) = machine.saved_in_memory.get(&slot) {
        return Some(bytes.clone());
    }
    game_fs::read(slot_path(machine, slot)).ok()
}

pub fn slot_exists(machine: &Machine, slot: i32) -> bool {
    machine.saved_in_memory.contains_key(&slot) || game_fs::is_file(slot_path(machine, slot))
}

/// Restores a serialized game. Execution resumes at the savepoint.
pub fn restore(machine: &mut Machine, bytes: &[u8]) -> Result<()> {
    let sections = slot_sections(bytes)?;
    let mut local = LocalMemory::default();
    let mut stack = None;
    for (name, body) in &sections {
        let mut r = Reader::new(body);
        match name.as_str() {
            "memory" => read_local(&mut r, &mut local)?,
            "stack" => stack = Some(read_stack(&mut r, machine)?),
            "system" => {
                machine.sys.title = r.str()?;
                machine.sys.default_grp = r.str()?;
                machine.sys.default_bgr = r.str()?;
                machine.sys.syscom.skip_mode = r.bool()?;
            }
            _ => {}
        }
    }
    let stack = stack.ok_or_else(|| anyhow!("reallive: the save has no call stack"))?;
    machine.reset_for_load();
    machine.memory.local = local;
    machine.stack = stack;
    machine.halted = false;
    machine.load_subsystems(&sections)?;
    machine.mark_savepoint();
    machine.just_loaded = true;
    Ok(())
}

pub fn load_slot(machine: &mut Machine, slot: i32) -> Result<()> {
    let bytes =
        slot_bytes(machine, slot).ok_or_else(|| anyhow!("reallive: slot {slot} is empty"))?;
    restore(machine, &bytes)
}

pub fn save_global(machine: &Machine) -> Result<()> {
    if !machine.sys.options.persist {
        return Ok(());
    }
    let mut w = Writer::new();
    w.bytes.extend_from_slice(GLOBAL_MAGIC);
    let global = &machine.memory.global;
    w.section("memory", |w| {
        w.i32s(&global.int_g);
        w.i32s(&global.int_z);
        w.strs(&global.str_m);
        w.strs(&global.global_names);
    });
    w.section("kidoku", |w| {
        w.len(global.kidoku.len());
        for (scene, bits) in &global.kidoku {
            w.i32(*scene);
            w.len(bits.len());
            for word in bits {
                w.u64(*word);
            }
        }
    });
    w.section("misc", |w| w.i32(machine.latest_save));
    w.section("settings", |w| crate::settings_io::write(w, &machine.sys));
    let path = global_path(machine);
    game_fs::create_dir_all(path.parent().expect("a directory"))?;
    game_fs::write(&path, w.bytes).with_context(|| format!("failed to write {}", path.display()))
}

pub fn load_global(machine: &mut Machine) -> Result<()> {
    if !machine.sys.options.persist {
        return Ok(());
    }
    let Ok(bytes) = game_fs::read(global_path(machine)) else {
        return Ok(());
    };
    let body = bytes
        .strip_prefix(GLOBAL_MAGIC.as_slice())
        .ok_or_else(|| anyhow!("reallive: not a global save file"))?;
    for (name, body) in Reader::new(body).sections()? {
        let mut r = Reader::new(body);
        let global = &mut machine.memory.global;
        match name.as_str() {
            "memory" => {
                fill(&mut global.int_g, &r.i32s()?);
                fill(&mut global.int_z, &r.i32s()?);
                fill(&mut global.str_m, &r.strs()?);
                fill(&mut global.global_names, &r.strs()?);
            }
            "kidoku" => {
                let count = r.len()?;
                for _ in 0..count {
                    let scene = r.i32()?;
                    let words = r.len()?;
                    let bits = (0..words).map(|_| r.u64()).collect::<Result<Vec<_>>>()?;
                    global.kidoku.insert(scene, bits);
                }
            }
            "misc" => machine.latest_save = r.i32()?,
            "settings" => crate::settings_io::read(&mut r, &mut machine.sys)?,
            _ => {}
        }
    }
    Ok(())
}

/// Saves the selection point for `ReturnPrevSelect`.
pub fn remember_selection(machine: &mut Machine) {
    machine.previous_selection = Some(serialize_slot(machine));
}

/// `ReturnPrevSelect`: returns true when a jump took place.
pub fn return_to_previous_selection(machine: &mut Machine) -> Result<bool> {
    match machine.previous_selection.clone() {
        Some(bytes) => {
            restore(machine, &bytes)?;
            Ok(true)
        }
        None => Ok(false),
    }
}

fn write_ints(machine: &mut Machine, command: &Command, from: usize, values: &[i32]) -> Result<()> {
    for (offset, value) in values.iter().enumerate() {
        if from + offset >= command.params.len() {
            break;
        }
        let target = machine.int_target_param(command, from + offset)?;
        machine.set_target(target, *value)?;
    }
    Ok(())
}

/// `SaveExists`, `SaveDate`, `SaveTime`, `SaveDateTime`, `SaveInfo`,
/// `GetSaveFlag`, `LatestSave`.
pub fn sys_query(machine: &mut Machine, command: &Command) -> Result<()> {
    let opcode = command.op.opcode;
    if opcode == 1421 {
        machine.store = machine.latest_save;
        return Ok(());
    }
    let slot = machine.int_param(command, 0)?;
    if opcode == 1409 {
        machine.store = i32::from(slot_exists(machine, slot));
        return Ok(());
    }
    if opcode == 1414 {
        let Some(local) = slot_bytes(machine, slot).and_then(|bytes| {
            let sections = slot_sections(&bytes).ok()?;
            let (_, body) = sections.into_iter().find(|(name, _)| name == "memory")?;
            let mut local = LocalMemory::default();
            read_local(&mut Reader::new(body), &mut local).ok()?;
            Some(local)
        }) else {
            machine.store = 0;
            return Ok(());
        };
        let saved = Memory {
            local,
            ..Memory::default()
        };
        for param in command.params.iter().skip(1) {
            let Expr::Special { tag, pieces } = &param.value else {
                continue;
            };
            let (Some(src), Some(dst), Some(count)) =
                (pieces.first(), pieces.get(1), pieces.get(2))
            else {
                continue;
            };
            let count = machine.eval_int(count)?.max(0);
            if *tag == 1 {
                let src = machine.str_target(src)?;
                let dst = machine.str_target(dst)?;
                for i in 0..count {
                    let frame = crate::memory::FrameMemory::default();
                    let value = saved
                        .string(src.bank, src.index + i, &frame)
                        .unwrap_or("")
                        .to_owned();
                    machine.write_string(
                        crate::machine::StrTarget {
                            bank: dst.bank,
                            index: dst.index + i,
                        },
                        value,
                    )?;
                }
            } else {
                let src = machine.int_target(src)?;
                let dst = machine.int_target(dst)?;
                let sources = Machine::int_range(src, count as usize);
                let dests = Machine::int_range(dst, count as usize);
                for (source, dest) in sources.into_iter().zip(dests) {
                    let value = match source {
                        crate::machine::IntTarget::Mem(reference) if reference.bank <= 5 => {
                            let frame = crate::memory::FrameMemory::default();
                            saved.int(reference, &frame).unwrap_or(0)
                        }
                        other => machine.get_target(other)?,
                    };
                    machine.set_target(dest, value)?;
                }
            }
        }
        machine.store = 1;
        return Ok(());
    }
    let header = slot_bytes(machine, slot).and_then(|bytes| read_header(&bytes).ok());
    let Some(header) = header else {
        machine.store = 0;
        return Ok(());
    };
    match opcode {
        1410 => write_ints(machine, command, 1, &header.date[..4])?,
        1411 => write_ints(machine, command, 1, &header.date[4..])?,
        1412 => write_ints(machine, command, 1, &header.date)?,
        _ => {
            write_ints(machine, command, 1, &header.date)?;
            if command.params.len() > 9 {
                let target = machine.str_target_param(command, 9)?;
                machine.write_string(target, header.title.clone())?;
            }
        }
    }
    machine.store = 1;
    Ok(())
}

/// `menu_save`, `menu_load`, `savemenu`, `loadmenu`, `save`, `load` and
/// the `_always` variants. Returns true when the machine jumped (a load).
pub fn sys_save_load(machine: &mut Machine, command: &Command) -> Result<bool> {
    let opcode = command.op.opcode;
    match opcode % 100 {
        // menu_save / menu_load / savemenu / loadmenu
        0..=3 => {
            let kind = crate::modules::menu::SlotMenuKind::from_opcode(opcode);
            crate::modules::menu::open_slot_menu(machine, kind)?;
            Ok(false)
        }
        // save2 / save / save_always2 / save_always
        6 | 7 => {
            let slot = machine.int_param(command, 0)?;
            save_slot(machine, slot)?;
            Ok(false)
        }
        // load2 / load / load_always2 / load_always
        8 | 9 => {
            let slot = machine.int_param(command, 0)?;
            load_slot(machine, slot)?;
            Ok(true)
        }
        _ => {
            machine.unimplemented(command)?;
            Ok(false)
        }
    }
}

impl Machine {
    /// Clears execution and presentation state before a load or a return
    /// to the menu.
    pub fn reset_for_load(&mut self) {
        self.clear_long_ops();
        self.in_interrupt = false;
        self.sys.reset_presentation();
    }

    pub fn save_subsystems(&self, w: &mut Writer) {
        self.sys.save_state(w);
    }

    pub fn load_subsystems(&mut self, sections: &[(String, &[u8])]) -> Result<()> {
        self.sys.load_state(sections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_memory_round_trips() {
        let mut local = LocalMemory::default();
        local.ints[2][5] = 42;
        local.str_s[3] = "テスト".into();
        local.local_names[1] = "名".into();
        let mut w = Writer::new();
        write_local(&mut w, &local);
        let mut back = LocalMemory::default();
        read_local(&mut Reader::new(&w.bytes), &mut back).unwrap();
        assert_eq!(back, local);
    }
}
