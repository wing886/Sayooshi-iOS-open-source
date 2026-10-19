//! The RealLive virtual machine: call stack, instruction dispatch and
//! expression evaluation.
//!
//! Execution is cooperative. [`Machine::run`] executes instructions until
//! a long operation (text output waiting for a click, a timed wait, a
//! transition, a selection, ...) reports that it needs another frame.
//! Long operations live on their own stack above the call stack; they are
//! popped when they finish.

use std::rc::Rc;

use anyhow::{Context, Result, anyhow, bail};

use crate::archive::Archive;
use crate::bytecode::{Command, CommandKind, Element, Param};
use crate::expr::{self, Expr, bank, op};
use crate::gameexe::Gameexe;
use crate::memory::{FrameMemory, IntRef, Memory};
use crate::nls::Nls;
use crate::scenario::Scenario;
use crate::system::System;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
    Root,
    Gosub,
    Farcall,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub scenario: Rc<Scenario>,
    pub ip: usize,
    pub kind: FrameKind,
    pub vars: FrameMemory,
}

impl Frame {
    pub fn new(scenario: Rc<Scenario>, ip: usize, kind: FrameKind) -> Self {
        Self {
            scenario,
            ip,
            kind,
            vars: FrameMemory::default(),
        }
    }
}

/// An operation that spans several frames.
pub trait LongOp: std::fmt::Debug {
    /// Advances the operation; `true` when it has finished.
    fn step(&mut self, machine: &mut Machine) -> Result<bool>;

    fn name(&self) -> &'static str;
}

/// What an opcode did to the instruction pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Next {
    /// Continue with the following instruction.
    Advance,
    /// The opcode moved the instruction pointer itself.
    Jumped,
}

/// A writable integer location: memory or the store register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntTarget {
    Store,
    Mem(IntRef),
}

/// A writable string location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrTarget {
    pub bank: u8,
    pub index: i32,
}

#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    /// Unimplemented opcodes and how often they were hit.
    pub unimplemented: std::collections::BTreeMap<String, usize>,
    /// Runtime errors with their location.
    pub errors: Vec<String>,
}

pub struct Machine {
    pub archive: Rc<Archive>,
    pub gameexe: Rc<Gameexe>,
    pub memory: Memory,
    pub stack: Vec<Frame>,
    /// Long operations with the call-stack depth they were started at. The
    /// last one runs while no frame has been pushed above it (a
    /// `#CANCELCALL` menu, for example, runs on top of a paused text wait
    /// and the wait resumes when the menu returns).
    pub long_ops: Vec<(usize, Box<dyn LongOp>)>,
    pub store: i32,
    pub halted: bool,
    pub line: i32,
    pub sys: System,
    pub diagnostics: Diagnostics,
    /// Call stack at the last savepoint.
    pub savepoint_stack: Vec<Frame>,
    pub mark_savepoints: bool,
    /// Set by opcodes that must end the current frame (e.g. `refresh`).
    pub yield_frame: bool,
    /// Stop on the first runtime error instead of skipping the instruction.
    pub halt_on_error: bool,
    /// `SetInterrupt(scenario, entrypoint)`: called once per frame.
    pub interrupt: Option<(i32, i32)>,
    /// An interrupt handler is running.
    pub in_interrupt: bool,
    /// `INTERRUPTCOM_DISAPPEAR`: the handler is not called for now.
    pub interrupt_suspended: bool,
    /// Slot saves kept in memory when persistence is disabled.
    pub saved_in_memory: std::collections::HashMap<i32, Vec<u8>>,
    /// `LatestSave()`: the slot most recently saved to, or -1.
    pub latest_save: i32,
    /// State at the last selection (`ReturnPrevSelect`).
    pub previous_selection: Option<Vec<u8>>,
    /// `SET_SELPOINTMOD_*` (`#INIT_SELPOINT_USE`): selections record the
    /// point `ReturnPrevSelect` goes back to.
    pub selpoint_auto: bool,
    /// A game was just loaded (`CHECK_JUST_AFTER_LOAD`).
    pub just_loaded: bool,
    /// Emulated DLLs (`LoadDLL`, `#DLL.nnn`).
    pub dlls: crate::dll::Dlls,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field("scene", &self.scene_number())
            .field("ip", &self.stack.last().map(|frame| frame.ip))
            .field("halted", &self.halted)
            .finish_non_exhaustive()
    }
}

/// Instructions executed per [`Machine::run`] before yielding regardless.
const INSTRUCTION_BUDGET: usize = 200_000;

impl Machine {
    pub fn new(archive: Rc<Archive>, gameexe: Rc<Gameexe>, sys: System) -> Result<Self> {
        let start = gameexe
            .int("SEEN_START")
            .filter(|&scene| archive.contains(scene))
            .or_else(|| archive.first())
            .ok_or_else(|| anyhow!("reallive: the archive has no scenarios"))?;
        let scenario = archive.scenario(start)?;
        let selpoint_auto = gameexe.int("INIT_SELPOINT_USE").unwrap_or(1) != 0;
        let mut machine = Self {
            archive,
            gameexe,
            memory: Memory::new(),
            stack: vec![Frame::new(scenario, 0, FrameKind::Root)],
            long_ops: Vec::new(),
            store: 0,
            halted: false,
            line: 0,
            sys,
            diagnostics: Diagnostics::default(),
            savepoint_stack: Vec::new(),
            mark_savepoints: true,
            yield_frame: false,
            halt_on_error: false,
            interrupt: None,
            in_interrupt: false,
            interrupt_suspended: false,
            saved_in_memory: Default::default(),
            latest_save: -1,
            previous_selection: None,
            selpoint_auto,
            just_loaded: false,
            dlls: crate::dll::Dlls::default(),
        };
        for entry in machine.gameexe.clone().filter("DLL.") {
            if let (Some(slot), Some(name)) = (entry.key_number(1), entry.str(0)) {
                machine.dlls.load(slot, name);
            }
        }
        machine.init_local_memory();
        machine.init_global_memory();
        crate::save::load_global(&mut machine)?;
        machine.mark_savepoint();
        Ok(machine)
    }

    // ---- state ----------------------------------------------------------

    /// Applies `#A[...]`, `#S[...]` and `#LOCALNAME.x` initial values.
    pub fn init_local_memory(&mut self) {
        self.apply_memory_initializers(true);
    }

    /// Applies `#G[...]`, `#Z[...]`, `#M[...]` and `#NAME.x` initial values
    /// (overwritten by the global save when one exists).
    pub fn init_global_memory(&mut self) {
        self.apply_memory_initializers(false);
    }

    fn apply_memory_initializers(&mut self, local: bool) {
        let gameexe = self.gameexe.clone();
        for entry in gameexe.entries() {
            let key = entry.key.as_str();
            if let Some(letters) = key.strip_prefix(if local { "LOCALNAME." } else { "NAME." }) {
                if let (Some(index), Some(value)) = (Memory::name_index(letters), entry.str(0)) {
                    self.memory.set_name(local, index, value.to_owned());
                }
                continue;
            }
            let Some((bank, rest)) = key.split_once('[') else {
                continue;
            };
            let Some(index) = rest
                .strip_suffix(']')
                .and_then(|index| index.trim().parse::<i32>().ok())
            else {
                continue;
            };
            match bank {
                "S" | "M" => {
                    if (bank == "S") != local {
                        continue;
                    }
                    let target = StrTarget {
                        bank: if bank == "S" {
                            bank::STR_S
                        } else {
                            bank::STR_M
                        },
                        index,
                    };
                    let value = entry.str(0).unwrap_or("").to_owned();
                    if let Err(error) = self.memory.set_string(
                        target.bank,
                        target.index,
                        value,
                        &mut FrameMemory::default(),
                    ) {
                        self.report(format!("Gameexe #{key}: {error:#}"));
                    }
                }
                _ => {
                    // `A`, `AB`, `A2B`, `A4B`, `A8B` (keys are upper-cased).
                    let Some(reference) = bank
                        .get(..1)
                        .map(|letter| format!("{letter}{}", bank[1..].to_ascii_lowercase()))
                        .and_then(|name| IntRef::named(&name, index))
                    else {
                        continue;
                    };
                    let is_local = reference.bank <= 5;
                    if is_local != local || reference.bank == crate::memory::BANK_L {
                        continue;
                    }
                    let value = entry.int(0).unwrap_or(0);
                    if let Err(error) =
                        self.memory
                            .set_int(reference, value, &mut FrameMemory::default())
                    {
                        self.report(format!("Gameexe #{key}: {error:#}"));
                    }
                }
            }
        }
    }

    pub fn frame(&self) -> &Frame {
        self.stack
            .last()
            .expect("the call stack is never empty while running")
    }

    /// `CallDLL(slot, args...)`.
    pub fn call_dll(&mut self, slot: i32, args: [i32; 5]) -> Result<i32> {
        let frame = self
            .stack
            .last_mut()
            .ok_or_else(|| anyhow!("reallive: no call frame"))?;
        let mut memory = crate::dll::Banks {
            memory: &mut self.memory,
            frame: &mut frame.vars,
        };
        self.dlls.call(slot, &mut self.sys, &mut memory, args)
    }

    pub fn frame_mut(&mut self) -> &mut Frame {
        self.stack
            .last_mut()
            .expect("the call stack is never empty while running")
    }

    pub fn scenario(&self) -> &Rc<Scenario> {
        &self.frame().scenario
    }

    pub fn scene_number(&self) -> i32 {
        self.stack.last().map_or(-1, |frame| frame.scenario.number)
    }

    /// Encoding of the running scenario's text.
    pub fn nls(&self) -> Nls {
        self.stack
            .last()
            .map_or(self.archive.nls, |frame| frame.scenario.nls)
    }

    /// Starts a long operation above the current frame.
    pub fn push_long_op(&mut self, op: Box<dyn LongOp>) {
        self.long_ops.push((self.stack.len(), op));
    }

    /// The running long operation, if one is on top.
    pub fn current_long_op(&self) -> Option<&dyn LongOp> {
        self.long_ops
            .last()
            .filter(|(started, _)| *started == self.stack.len())
            .map(|(_, op)| op.as_ref())
    }

    pub fn clear_long_ops(&mut self) {
        self.long_ops.clear();
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    // ---- control flow ---------------------------------------------------

    pub fn advance(&mut self) {
        let frame = self.frame_mut();
        frame.ip += 1;
        if frame.ip >= frame.scenario.script.elements.len() {
            self.halted = true;
        }
    }

    pub fn goto(&mut self, index: usize) {
        let frame = self.frame_mut();
        frame.ip = index;
        if index >= frame.scenario.script.elements.len() {
            self.halted = true;
        }
    }

    pub fn gosub(&mut self, index: usize) {
        // The caller resumes after the gosub.
        self.frame_mut().ip += 1;
        let scenario = self.scenario().clone();
        self.stack
            .push(Frame::new(scenario, index, FrameKind::Gosub));
    }

    pub fn return_from_gosub(&mut self) -> Result<()> {
        if self.frame().kind != FrameKind::Gosub {
            bail!("reallive: ret without a matching gosub");
        }
        self.stack.pop();
        Ok(())
    }

    fn resolve_entrypoint(&self, scene: i32, entrypoint: i32) -> Result<(Rc<Scenario>, usize)> {
        let scenario = self
            .archive
            .scenario(scene)
            .with_context(|| format!("jump to SEEN{scene:04}"))?;
        let ip = scenario
            .entrypoint(entrypoint)
            .ok_or_else(|| anyhow!("reallive: SEEN{scene:04} has no entrypoint {entrypoint}"))?;
        Ok((scenario, ip))
    }

    pub fn jump(&mut self, scene: i32, entrypoint: i32) -> Result<()> {
        let (scenario, ip) = self.resolve_entrypoint(scene, entrypoint)?;
        let frame = self.frame_mut();
        frame.scenario = scenario;
        frame.ip = ip;
        Ok(())
    }

    pub fn farcall(&mut self, scene: i32, entrypoint: i32) -> Result<()> {
        let (scenario, ip) = self.resolve_entrypoint(scene, entrypoint)?;
        self.frame_mut().ip += 1;
        if entrypoint == 0 && self.should_set_savepoint(SavepointKind::SeenTop) {
            self.mark_savepoint();
        }
        self.stack
            .push(Frame::new(scenario, ip, FrameKind::Farcall));
        Ok(())
    }

    pub fn return_from_farcall(&mut self) -> Result<()> {
        if self.frame().kind != FrameKind::Farcall {
            bail!("reallive: rtl without a matching farcall");
        }
        self.stack.pop();
        Ok(())
    }

    /// Writes `*_with` arguments into the (new) current frame.
    pub fn write_with_arguments(&mut self, integers: &[i32], strings: &[String]) {
        let vars = &mut self.frame_mut().vars;
        for (slot, value) in vars.int_l.iter_mut().zip(integers) {
            *slot = *value;
        }
        for (index, value) in strings.iter().enumerate() {
            if vars.str_k.len() <= index {
                vars.str_k.resize(index + 1, String::new());
            }
            vars.str_k[index] = value.clone();
        }
    }

    /// `pushStringValueUp`: sets `strK[index]` of the calling frame.
    pub fn push_string_value_up(&mut self, index: i32, value: String) -> Result<()> {
        if !(0..=2).contains(&index) {
            bail!("reallive: invalid index {index} in pushStringValueUp");
        }
        let depth = self.stack.len();
        if depth >= 2 {
            let vars = &mut self.stack[depth - 2].vars;
            let index = index as usize;
            if vars.str_k.len() <= index {
                vars.str_k.resize(index + 1, String::new());
            }
            vars.str_k[index] = value;
        }
        Ok(())
    }

    /// `RETURN_L_FLAG_SET`: sets the caller's `intL[index]`.
    pub fn push_int_value_up(&mut self, index: i32, value: i32) -> Result<()> {
        let depth = self.stack.len();
        if depth >= 2 {
            let vars = &mut self.stack[depth - 2].vars;
            let slot = usize::try_from(index)
                .ok()
                .and_then(|index| vars.int_l.get_mut(index))
                .ok_or_else(|| {
                    anyhow::anyhow!("reallive: invalid index {index} in RETURN_L_FLAG_SET")
                })?;
            *slot = value;
        }
        Ok(())
    }

    // ---- savepoints -----------------------------------------------------

    pub fn mark_savepoint(&mut self) {
        self.savepoint_stack = self.stack.clone();
        self.memory.take_savepoint();
        self.sys.take_savepoint();
    }

    pub fn should_set_savepoint(&self, kind: SavepointKind) -> bool {
        if !self.mark_savepoints {
            return false;
        }
        let header = &self.scenario().header;
        let (attribute, key) = match kind {
            SavepointKind::Message => (header.savepoint_message, "SAVEPOINT_MESSAGE"),
            SavepointKind::Selcom => (header.savepoint_selcom, "SAVEPOINT_SELCOM"),
            SavepointKind::SeenTop => (header.savepoint_seentop, "SAVEPOINT_SEENTOP"),
        };
        match attribute {
            1 => true,
            2 => false,
            _ => self.gameexe.int(key) != Some(0),
        }
    }

    // ---- execution ------------------------------------------------------

    /// Runs until a long operation needs another frame, the machine halts,
    /// or the instruction budget is spent.
    pub fn run(&mut self) {
        self.yield_frame = false;
        for _ in 0..INSTRUCTION_BUDGET {
            if self.halted || self.yield_frame {
                return;
            }
            let depth = self.stack.len();
            if self
                .long_ops
                .last()
                .is_some_and(|(started, _)| *started >= depth)
            {
                let slot = self.long_ops.len() - 1;
                let (started, mut op) = self.long_ops.pop().expect("checked");
                if started > depth {
                    // Its frame has been popped (e.g. by a menu that
                    // cleared the call stack).
                    continue;
                }
                match op.step(self) {
                    Ok(true) => {}
                    Ok(false) => {
                        // Operations pushed while stepping stay above it.
                        self.long_ops.insert(slot, (started, op));
                        if self.stack.len() == depth {
                            return;
                        }
                    }
                    Err(error) => self.report(format!("[{}] {error:#}", op.name())),
                }
                continue;
            }
            self.step();
        }
    }

    /// Executes one instruction.
    pub fn step(&mut self) {
        if self.halted {
            return;
        }
        let frame = self.frame();
        let Some(element) = frame.scenario.script.elements.get(frame.ip) else {
            self.halted = true;
            return;
        };
        // Elements are immutable; clone the scenario handle, not the element.
        let scenario = frame.scenario.clone();
        let element = &scenario.script.elements[frame.ip];
        let result = self.execute(element);
        match result {
            Ok(Next::Advance) => self.advance(),
            Ok(Next::Jumped) => {}
            Err(error) => {
                let location = format!("SEEN{:04} line {}", self.scene_number(), self.line);
                self.report(format!("{location}: {error:#}"));
                if self.halt_on_error {
                    self.halted = true;
                } else {
                    self.advance();
                }
            }
        }
    }

    pub fn report(&mut self, message: String) {
        if self.diagnostics.errors.len() < 10_000 {
            self.diagnostics.errors.push(message);
        }
    }

    fn execute(&mut self, element: &Element) -> Result<Next> {
        match element {
            Element::Comma | Element::Entrypoint(_) => Ok(Next::Advance),
            Element::Line(line) => {
                self.line = *line;
                Ok(Next::Advance)
            }
            Element::Kidoku(kidoku) => {
                self.on_kidoku(*kidoku);
                Ok(Next::Advance)
            }
            Element::Expression(expression) => {
                self.eval_int(expression)?;
                Ok(Next::Advance)
            }
            Element::Textout(raw) => {
                let nls = self.nls();
                let text = Element::text(raw, nls);
                if text.starts_with(&nls.encode("ＳｅｅｎＥｎｄ")) {
                    self.halted = true;
                    return Ok(Next::Jumped);
                }
                let text = nls.decode(&text);
                crate::modules::msg::textout(self, &text)?;
                Ok(Next::Advance)
            }
            Element::Command(command) => self.execute_command(command),
        }
    }

    fn on_kidoku(&mut self, kidoku: i32) {
        if self.should_set_savepoint(SavepointKind::Message) && self.sys.text.page_is_empty() {
            self.mark_savepoint();
        }
        let scene = self.scene_number();
        let read = self.memory.has_been_read(scene, kidoku);
        self.sys.text.set_kidoku_read(read);
        // Skip mode stops at text not read before.
        if !read {
            self.sys.syscom.skip_mode = false;
        }
        self.memory.record_kidoku(scene, kidoku);
    }

    fn execute_command(&mut self, command: &Command) -> Result<Next> {
        // `REALLIVE_TRACE[=scene]`: every command as it runs (debugging
        // aid), optionally only in one scene.
        thread_local! {
            static TRACE: Option<Option<i32>> = std::env::var("REALLIVE_TRACE")
                .ok()
                .map(|v| v.parse().ok());
        }
        if TRACE.with(|t| t.is_some_and(|scene| scene.is_none_or(|s| s == self.scene_number()))) {
            eprintln!(
                "[{}] SEEN{:04} line {}: {}",
                self.sys.now(),
                self.scene_number(),
                self.line,
                crate::opcodes::name(command.op)
                    .map_or_else(|| command.op.to_string(), |n| format!("{n} {}", command.op)),
            );
        }
        match &command.kind {
            CommandKind::Plain => crate::modules::dispatch(self, command),
            _ => crate::modules::jmp::flow(self, command),
        }
    }

    pub fn unimplemented(&mut self, command: &Command) -> Result<Next> {
        let name = crate::opcodes::name(command.op).map_or_else(
            || command.op.to_string(),
            |name| format!("{name} {}", command.op),
        );
        self.note_unimplemented(name);
        Ok(Next::Advance)
    }

    /// Records a feature the engine could not provide.
    pub fn note_unimplemented(&mut self, what: String) {
        *self.diagnostics.unimplemented.entry(what).or_default() += 1;
    }

    // ---- expressions ----------------------------------------------------

    pub fn int_ref(&mut self, bank_byte: u8, index: &Expr) -> Result<IntRef> {
        let index = self.eval_int(index)?;
        IntRef::from_bytecode(bank_byte, index)
            .ok_or_else(|| anyhow!("reallive: invalid integer bank 0x{bank_byte:02x}"))
    }

    pub fn read_int(&self, reference: IntRef) -> Result<i32> {
        self.memory.int(reference, &self.frame().vars)
    }

    pub fn write_int(&mut self, reference: IntRef, value: i32) -> Result<()> {
        let frame = self
            .stack
            .last_mut()
            .ok_or_else(|| anyhow!("reallive: no call frame"))?;
        self.memory.set_int(reference, value, &mut frame.vars)
    }

    pub fn read_string(&self, target: StrTarget) -> Result<String> {
        Ok(self
            .memory
            .string(target.bank, target.index, &self.frame().vars)?
            .to_owned())
    }

    pub fn write_string(&mut self, target: StrTarget, value: String) -> Result<()> {
        let frame = self
            .stack
            .last_mut()
            .ok_or_else(|| anyhow!("reallive: no call frame"))?;
        self.memory
            .set_string(target.bank, target.index, value, &mut frame.vars)
    }

    pub fn eval_int(&mut self, expression: &Expr) -> Result<i32> {
        match expression {
            Expr::Int(value) => Ok(*value),
            Expr::StoreRegister => Ok(self.store),
            Expr::Mem { bank, index } => {
                if bank::is_string(*bank) {
                    bail!("reallive: a string was used as an integer");
                }
                let reference = self.int_ref(*bank, index)?;
                self.read_int(reference)
            }
            Expr::Unary(operation, operand) => {
                let value = self.eval_int(operand)?;
                Ok(if *operation == 0x01 {
                    value.wrapping_neg()
                } else {
                    value
                })
            }
            Expr::Binary(operation, left, right) => {
                let operation = *operation;
                if operation == op::ASSIGN {
                    let value = self.eval_int(right)?;
                    self.assign_int(left, value)?;
                    Ok(value)
                } else if (op::ASSIGN_BASE..op::ASSIGN).contains(&operation) {
                    let target = self.int_target(left)?;
                    let current = self.get_target(target)?;
                    let rhs = self.eval_int(right)?;
                    let value = expr::binary_op(operation, current, rhs).unwrap_or(current);
                    self.set_target(target, value)?;
                    Ok(value)
                } else {
                    let lhs = self.eval_int(left)?;
                    let rhs = self.eval_int(right)?;
                    expr::binary_op(operation, lhs, rhs)
                        .ok_or_else(|| anyhow!("reallive: unknown operator 0x{operation:02x}"))
                }
            }
            Expr::Str(_) | Expr::Print(_) => bail!("reallive: a string was used as an integer"),
            Expr::Complex(pieces) | Expr::Special { pieces, .. } => match pieces.first() {
                Some(first) => self.eval_int(first),
                None => Ok(0),
            },
        }
    }

    fn assign_int(&mut self, target: &Expr, value: i32) -> Result<()> {
        if let Expr::Mem { bank, index } = target {
            if bank::is_string(*bank) {
                bail!("reallive: an integer was assigned to a string");
            }
        }
        let target = self.int_target(target)?;
        self.set_target(target, value)
    }

    /// The writable location an expression names.
    pub fn int_target(&mut self, expression: &Expr) -> Result<IntTarget> {
        match expression {
            Expr::StoreRegister => Ok(IntTarget::Store),
            Expr::Mem { bank, index } if !bank::is_string(*bank) => {
                Ok(IntTarget::Mem(self.int_ref(*bank, index)?))
            }
            other => bail!("reallive: {other:?} is not an integer variable"),
        }
    }

    pub fn get_target(&self, target: IntTarget) -> Result<i32> {
        match target {
            IntTarget::Store => Ok(self.store),
            IntTarget::Mem(reference) => self.read_int(reference),
        }
    }

    pub fn set_target(&mut self, target: IntTarget, value: i32) -> Result<()> {
        match target {
            IntTarget::Store => {
                self.store = value;
                Ok(())
            }
            IntTarget::Mem(reference) => self.write_int(reference, value),
        }
    }

    pub fn str_target(&mut self, expression: &Expr) -> Result<StrTarget> {
        match expression {
            Expr::Mem { bank, index } if bank::is_string(*bank) => Ok(StrTarget {
                bank: *bank,
                index: self.eval_int(index)?,
            }),
            other => bail!("reallive: {other:?} is not a string variable"),
        }
    }

    pub fn eval_str(&mut self, expression: &Expr) -> Result<String> {
        match expression {
            Expr::Str(bytes) => Ok(self.nls().decode(bytes)),
            Expr::Print(inner) => self.eval_str(inner),
            Expr::Mem { bank, .. } if bank::is_string(*bank) => {
                let target = self.str_target(expression)?;
                self.read_string(target)
            }
            Expr::Complex(pieces) | Expr::Special { pieces, .. } => match pieces.first() {
                Some(first) => self.eval_str(first),
                None => Ok(String::new()),
            },
            // Numbers print as themselves (`###PRINT(intA[0])`).
            other => Ok(self.eval_int(other)?.to_string()),
        }
    }

    // ---- parameters -----------------------------------------------------

    fn param<'c>(&self, command: &'c Command, index: usize) -> Result<&'c Param> {
        command.params.get(index).ok_or_else(|| {
            anyhow!(
                "reallive: {} expects parameter {} but has {}",
                command.op,
                index + 1,
                command.params.len()
            )
        })
    }

    pub fn int_param(&mut self, command: &Command, index: usize) -> Result<i32> {
        let value = &self.param(command, index)?.value;
        self.eval_int(value)
    }

    pub fn int_param_or(&mut self, command: &Command, index: usize, default: i32) -> Result<i32> {
        match command.params.get(index) {
            Some(param) => self.eval_int(&param.value),
            None => Ok(default),
        }
    }

    pub fn str_param(&mut self, command: &Command, index: usize) -> Result<String> {
        let value = &self.param(command, index)?.value;
        self.eval_str(value)
    }

    pub fn str_param_or(
        &mut self,
        command: &Command,
        index: usize,
        default: &str,
    ) -> Result<String> {
        match command.params.get(index) {
            Some(param) => self.eval_str(&param.value),
            None => Ok(default.to_owned()),
        }
    }

    pub fn int_target_param(&mut self, command: &Command, index: usize) -> Result<IntTarget> {
        let value = &self.param(command, index)?.value;
        self.int_target(value)
    }

    pub fn str_target_param(&mut self, command: &Command, index: usize) -> Result<StrTarget> {
        let value = &self.param(command, index)?.value;
        self.str_target(value)
    }

    /// A tuple parameter, re-read as such.
    pub fn complex_param(&self, command: &Command, index: usize) -> Result<Vec<Expr>> {
        let param = self.param(command, index)?;
        Ok(param.complex(self.nls())?.pieces().to_vec())
    }

    /// Every parameter from `from` on, evaluated as integers.
    pub fn int_params_from(&mut self, command: &Command, from: usize) -> Result<Vec<i32>> {
        (from..command.params.len())
            .map(|index| self.int_param(command, index))
            .collect()
    }

    pub fn param_count(command: &Command) -> usize {
        command.params.len()
    }

    // ---- integer iteration over memory ranges ----------------------------

    /// `count` consecutive locations starting at `first` (same bank).
    pub fn int_range(first: IntTarget, count: usize) -> Vec<IntTarget> {
        match first {
            IntTarget::Store => vec![IntTarget::Store; count.min(1)],
            IntTarget::Mem(reference) => (0..count)
                .map(|offset| {
                    IntTarget::Mem(IntRef {
                        index: reference.index + offset as i32,
                        ..reference
                    })
                })
                .collect(),
        }
    }

    /// Locations from `first` to `last` inclusive.
    pub fn int_span(first: IntTarget, last: IntTarget) -> Result<Vec<IntTarget>> {
        match (first, last) {
            (IntTarget::Mem(a), IntTarget::Mem(b)) if a.bank == b.bank && a.width == b.width => {
                if b.index < a.index {
                    return Ok(Vec::new());
                }
                Ok(Self::int_range(first, (b.index - a.index + 1) as usize))
            }
            (IntTarget::Store, IntTarget::Store) => Ok(vec![IntTarget::Store]),
            _ => bail!("reallive: a memory range spans two banks"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavepointKind {
    Message,
    Selcom,
    SeenTop,
}
