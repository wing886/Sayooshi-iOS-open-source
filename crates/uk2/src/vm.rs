//! Deterministic core of the UK2 MES interpreter.
//!
//! Control flow, variables, strings, expression evaluation, conditions, and
//! MES-to-MES calls are implemented here from the reference interpreter.  The
//! DOS/PC-98 presentation and device commands are deliberately routed through
//! [`Uk2Host`]; they are not silently approximated by unrelated modern APIs.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

use crate::disasm::{InstructionArg, InstructionKind, ResourceArgument, decode_instruction};
use crate::mes::{MES_CODE_OFFSET, MesProgram};
use crate::opcode::TwoOpcode;
use crate::value::{
    BoolJoin, CompareOp, Condition, Expression, ExpressionOp, IndirectKind, InlineStringPart,
    Operand,
};

const FIXED_STRING_MAX_LEN: usize = 0x3e;
const LARGE_STRING_MAX_LEN: usize = 0x7d2;
const TEXT_SLOT_MAX_LEN: usize = 0x15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericWidth {
    Byte,
    Word,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeValue {
    Number { value: u16, width: NumericWidth },
    String(Vec<u8>),
}

impl RuntimeValue {
    pub fn number(&self) -> Result<u16> {
        match self {
            Self::Number { value, .. } => Ok(*value),
            Self::String(_) => bail!("uk2: expected numeric value, found string"),
        }
    }

    pub fn string(&self) -> Result<&[u8]> {
        match self {
            Self::String(bytes) => Ok(bytes),
            Self::Number { .. } => bail!("uk2: expected string value, found number"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Uk2Memory {
    system_words: [u16; 31],
    fixed_strings: [Vec<u8>; 32],
    local_words: [u16; 32],
    indexed_5: [u8; 257],
    indexed_8: [u8; 257],
    indirect_bytes: BTreeMap<u8, u8>,
    indirect_words: BTreeMap<u8, u16>,
    timer16_deadline: Option<Instant>,
    timer16_fast_forward: bool,
    mouse_event_deadline: Option<Instant>,
    mouse_event_ticks: u16,
    indirect_strings: BTreeMap<u8, Vec<u8>>,
    large_strings: [Vec<u8>; 32],
    table5_slots: BTreeMap<u8, Vec<u8>>,
    table6_slots: BTreeMap<u8, Vec<u8>>,
}

impl Default for Uk2Memory {
    fn default() -> Self {
        Self {
            system_words: [0; 31],
            fixed_strings: std::array::from_fn(|_| Vec::new()),
            local_words: [0; 32],
            indexed_5: [0; 257],
            indexed_8: [0; 257],
            indirect_bytes: BTreeMap::new(),
            indirect_words: BTreeMap::new(),
            timer16_deadline: None,
            timer16_fast_forward: false,
            mouse_event_deadline: None,
            mouse_event_ticks: 0,
            indirect_strings: BTreeMap::new(),
            large_strings: std::array::from_fn(|_| Vec::new()),
            table5_slots: BTreeMap::new(),
            table6_slots: BTreeMap::new(),
        }
    }
}

impl Uk2Memory {
    /// Advance the PC-98 IRQ countdown immediately on each read. This is used
    /// by offline probes that have no audio interrupt clock.
    pub fn set_timer16_fast_forward(&mut self, enabled: bool) {
        self.timer16_fast_forward = enabled;
    }

    /// The original mouse IRQ increments the left/right event counter and
    /// clears both after `word_29256` reaches zero. The game's default
    /// debounce period is ten timer ticks.
    pub fn register_mouse_press(&mut self, slot: u8) -> Result<()> {
        if slot != 1 && slot != 3 {
            bail!("uk2: mouse event slot must be 1 or 3, got {slot}");
        }
        let count = self.indirect_words.entry(slot).or_default();
        *count = count.wrapping_add(1);
        if self.mouse_event_ticks == 0 {
            self.mouse_event_ticks = 10;
            self.mouse_event_deadline = Some(
                Instant::now()
                    + Duration::from_nanos(16_666_667 * u64::from(self.mouse_event_ticks)),
            );
        }
        Ok(())
    }

    pub fn clear_mouse_events(&mut self) {
        self.indirect_words.insert(1, 0);
        self.indirect_words.insert(3, 0);
        self.mouse_event_ticks = 0;
        self.mouse_event_deadline = None;
    }

    fn advance_mouse_event_timer(&mut self) {
        if self.mouse_event_ticks == 0 {
            return;
        }
        if self.timer16_fast_forward {
            self.mouse_event_ticks -= 1;
        } else if self
            .mouse_event_deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            self.mouse_event_ticks = 0;
        }
        if self.mouse_event_ticks == 0 {
            self.clear_mouse_events();
        }
    }

    fn read_indirect_word(&mut self, slot: u8) -> u16 {
        if slot == 1 || slot == 3 {
            self.advance_mouse_event_timer();
        }
        let value = *self.indirect_words.get(&slot).unwrap_or(&0);
        if slot != 16 || value == 0 {
            return value;
        }
        if self.timer16_fast_forward {
            let value = value - 1;
            self.indirect_words.insert(slot, value);
            return value;
        }
        let Some(deadline) = self.timer16_deadline else {
            return value;
        };
        const TICK_NANOS: u128 = 16_666_667;
        let remaining = deadline.saturating_duration_since(Instant::now());
        let mut ticks = remaining.as_nanos().div_ceil(TICK_NANOS) as u16;
        if ticks == value && ticks != 0 {
            std::thread::sleep(Duration::from_millis(1));
            let remaining = deadline.saturating_duration_since(Instant::now());
            ticks = remaining.as_nanos().div_ceil(TICK_NANOS) as u16;
        }
        if ticks == 0 {
            self.timer16_deadline = None;
        }
        self.indirect_words.insert(slot, ticks);
        ticks
    }

    pub fn system_word(&self, index: usize) -> Option<u16> {
        self.system_words.get(index).copied()
    }

    pub fn set_system_word(&mut self, index: usize, value: u16) -> Result<()> {
        let slot = self
            .system_words
            .get_mut(index)
            .ok_or_else(|| anyhow::anyhow!("uk2: system word index {index} is out of range"))?;
        *slot = value;
        Ok(())
    }

    pub fn local_word(&self, index: usize) -> Option<u16> {
        self.local_words.get(index).copied()
    }

    pub fn set_local_word(&mut self, index: usize, value: u16) -> Result<()> {
        let slot = self
            .local_words
            .get_mut(index)
            .ok_or_else(|| anyhow::anyhow!("uk2: local word index {index} is out of range"))?;
        *slot = value;
        Ok(())
    }

    pub fn set_table5_slot(&mut self, index: u8, value: Vec<u8>) {
        self.table5_slots.insert(index, value);
    }

    pub fn set_table6_slot(&mut self, index: u8, value: Vec<u8>) {
        self.table6_slots.insert(index, value);
    }

    pub fn read_operand(&mut self, operand: &Operand) -> Result<RuntimeValue> {
        match operand {
            Operand::SystemWord(index) => Ok(RuntimeValue::Number {
                value: self.system_words[usize::from(*index)],
                width: NumericWidth::Word,
            }),
            Operand::Immediate(value) => Ok(RuntimeValue::Number {
                value: *value,
                width: NumericWidth::Word,
            }),
            Operand::FixedString(index) => Ok(RuntimeValue::String(
                self.fixed_strings[usize::from(*index)].clone(),
            )),
            Operand::LocalWord(index) => Ok(RuntimeValue::Number {
                value: self.local_words[usize::from(*index)],
                width: NumericWidth::Word,
            }),
            Operand::InlineString(parts) => {
                let mut output = Vec::new();
                for part in parts {
                    match part {
                        InlineStringPart::Literal(bytes) => output.extend_from_slice(bytes),
                        InlineStringPart::Table5Slot(slot) => {
                            let index = one_based_slot(*slot)?;
                            if let Some(bytes) = self.table5_slots.get(&index) {
                                output.extend_from_slice(bytes);
                            }
                        }
                        InlineStringPart::Table6Slot(slot) => {
                            let index = one_based_slot(*slot)?;
                            if let Some(bytes) = self.table6_slots.get(&index) {
                                output.extend_from_slice(bytes);
                            }
                        }
                    }
                    if output.len() > LARGE_STRING_MAX_LEN {
                        bail!(
                            "uk2: inline string length {} exceeds destination limit {LARGE_STRING_MAX_LEN}",
                            output.len()
                        );
                    }
                }
                Ok(RuntimeValue::String(output))
            }
            Operand::IndexedByte { selector, index } => {
                let index = usize::from(self.eval_expression(index)?.number()?);
                if index > 0x100 {
                    bail!("uk2: indexed-byte operand index {index:#x} exceeds 0x100");
                }
                let value = match selector {
                    5 => self.indexed_5[index],
                    8 => self.indexed_8[index],
                    other => bail!("uk2: class-4 selector {other} has no reference pointer"),
                };
                Ok(RuntimeValue::Number {
                    value: u16::from(value),
                    width: NumericWidth::Byte,
                })
            }
            Operand::Indirect { kind, slot } => match kind {
                IndirectKind::Byte => Ok(RuntimeValue::Number {
                    value: u16::from(*self.indirect_bytes.get(slot).unwrap_or(&0)),
                    width: NumericWidth::Byte,
                }),
                IndirectKind::Word => Ok(RuntimeValue::Number {
                    value: self.read_indirect_word(*slot),
                    width: NumericWidth::Word,
                }),
                IndirectKind::String => Ok(RuntimeValue::String(
                    self.indirect_strings.get(slot).cloned().unwrap_or_default(),
                )),
                IndirectKind::Raw(raw) => {
                    bail!("uk2: class-5 operand uses unknown kind {raw}")
                }
            },
            Operand::LargeString(index) => Ok(RuntimeValue::String(
                self.large_strings[usize::from(*index)].clone(),
            )),
        }
    }

    pub fn write_operand(&mut self, operand: &Operand, value: RuntimeValue) -> Result<()> {
        match operand {
            Operand::SystemWord(index) => {
                self.system_words[usize::from(*index)] =
                    numeric_for_width(value, NumericWidth::Word)?;
            }
            Operand::Immediate(_) | Operand::InlineString(_) => {
                bail!("uk2: operand {operand:?} is not writable")
            }
            Operand::FixedString(index) => {
                self.fixed_strings[usize::from(*index)] =
                    checked_string(value, FIXED_STRING_MAX_LEN)?;
            }
            Operand::LocalWord(index) => {
                self.local_words[usize::from(*index)] =
                    numeric_for_width(value, NumericWidth::Word)?;
            }
            Operand::IndexedByte { selector, index } => {
                let index = usize::from(self.eval_expression(index)?.number()?);
                if index > 0x100 {
                    bail!("uk2: indexed-byte destination {index:#x} exceeds 0x100");
                }
                let value = numeric_for_width(value, NumericWidth::Byte)? as u8;
                match selector {
                    5 => self.indexed_5[index] = value,
                    8 => self.indexed_8[index] = value,
                    other => bail!("uk2: class-4 selector {other} has no reference pointer"),
                }
            }
            Operand::Indirect { kind, slot } => match kind {
                IndirectKind::Byte => {
                    self.indirect_bytes
                        .insert(*slot, numeric_for_width(value, NumericWidth::Byte)? as u8);
                }
                IndirectKind::Word => {
                    let value = numeric_for_width(value, NumericWidth::Word)?;
                    self.indirect_words.insert(*slot, value);
                    if *slot == 16 {
                        self.timer16_deadline = Some(
                            Instant::now() + Duration::from_nanos(16_666_667 * u64::from(value)),
                        );
                    }
                }
                IndirectKind::String => {
                    self.indirect_strings
                        .insert(*slot, value.string()?.to_vec());
                }
                IndirectKind::Raw(raw) => {
                    bail!("uk2: class-5 destination uses unknown kind {raw}")
                }
            },
            Operand::LargeString(index) => {
                self.large_strings[usize::from(*index)] =
                    checked_string(value, LARGE_STRING_MAX_LEN)?;
            }
        }
        Ok(())
    }

    pub fn eval_expression(&mut self, expression: &Expression) -> Result<RuntimeValue> {
        let mut number = 0i64;
        let mut string = Vec::new();
        let mut last_kind = RuntimeValue::Number {
            value: 0,
            width: NumericWidth::Word,
        };

        for term in &expression.terms {
            let operand = self.read_operand(&term.operand)?;
            match (&term.op, &operand) {
                (ExpressionOp::Set, RuntimeValue::Number { value, .. }) => {
                    number = i64::from(*value);
                }
                (ExpressionOp::Add, RuntimeValue::Number { value, .. }) => {
                    number += i64::from(*value);
                }
                (ExpressionOp::Subtract, RuntimeValue::Number { value, .. }) => {
                    number -= i64::from(*value);
                }
                (ExpressionOp::Multiply, RuntimeValue::Number { value, .. }) => {
                    number *= i64::from(*value);
                }
                (ExpressionOp::Divide, RuntimeValue::Number { value, .. }) => {
                    if *value == 0 {
                        bail!("uk2: division by zero");
                    }
                    number /= i64::from(*value);
                }
                (ExpressionOp::Modulo, RuntimeValue::Number { value, .. }) => {
                    if *value == 0 {
                        bail!("uk2: modulo by zero");
                    }
                    number %= i64::from(*value);
                }
                (ExpressionOp::Set, RuntimeValue::String(bytes)) => {
                    string.clear();
                    string.extend_from_slice(bytes);
                }
                (ExpressionOp::Add, RuntimeValue::String(bytes)) => {
                    string.extend_from_slice(bytes);
                }
                (operation, RuntimeValue::String(_)) => {
                    bail!("uk2: {operation:?} is not defined for a string operand")
                }
            }
            last_kind = operand;
        }

        match last_kind {
            RuntimeValue::String(_) => Ok(RuntimeValue::String(string)),
            RuntimeValue::Number { width, .. } => {
                let max = match width {
                    NumericWidth::Byte => 0xff,
                    NumericWidth::Word => 0xffff,
                };
                let value = number.clamp(0, max) as u16;
                Ok(RuntimeValue::Number { value, width })
            }
        }
    }

    pub fn eval_condition(&mut self, condition: &Condition) -> Result<bool> {
        let mut result = false;
        let mut join_before = BoolJoin::Or;
        for clause in &condition.clauses {
            let left = self.eval_expression(&clause.left)?;
            let right = self.eval_expression(&clause.right)?;
            let clause_result = compare_values(&left, clause.compare, &right)?;
            result = match join_before {
                BoolJoin::And => result && clause_result,
                BoolJoin::Or => result || clause_result,
            };
            if let Some(join) = clause.join_after {
                join_before = join;
            }
        }
        Ok(result)
    }

    fn reset_external_locals(&mut self) -> ([u16; 32], BTreeMap<u8, Vec<u8>>) {
        let saved_words = self.local_words;
        let saved_slots = std::mem::take(&mut self.table5_slots);
        self.local_words = [0; 32];
        (saved_words, saved_slots)
    }

    fn restore_external_locals(&mut self, saved: ([u16; 32], BTreeMap<u8, Vec<u8>>)) {
        self.local_words = saved.0;
        self.table5_slots = saved.1;
    }
}

fn one_based_slot(slot: u8) -> Result<u8> {
    slot.checked_sub(1)
        .ok_or_else(|| anyhow::anyhow!("uk2: inline string slot 0 underflows one-based index"))
}

fn numeric_for_width(value: RuntimeValue, width: NumericWidth) -> Result<u16> {
    let value = value.number()?;
    Ok(match width {
        NumericWidth::Byte => value.min(0xff),
        NumericWidth::Word => value,
    })
}

fn checked_string(value: RuntimeValue, max_len: usize) -> Result<Vec<u8>> {
    let bytes = value.string()?.to_vec();
    if bytes.len() > max_len {
        bail!(
            "uk2: string length {} exceeds destination limit {max_len}",
            bytes.len()
        );
    }
    Ok(bytes)
}

fn compare_values(left: &RuntimeValue, op: CompareOp, right: &RuntimeValue) -> Result<bool> {
    match (left, right) {
        (RuntimeValue::Number { value: a, .. }, RuntimeValue::Number { value: b, .. }) => {
            Ok(match op {
                CompareOp::Less => a < b,
                CompareOp::Greater => a > b,
                CompareOp::Equal => a == b,
                CompareOp::NotEqual => a != b,
            })
        }
        (RuntimeValue::String(a), RuntimeValue::String(b)) => {
            let ordering = a.cmp(b);
            Ok(match op {
                CompareOp::Less => ordering.is_lt(),
                CompareOp::Greater => ordering.is_gt(),
                CompareOp::Equal => ordering.is_eq(),
                CompareOp::NotEqual => !ordering.is_eq(),
            })
        }
        _ => bail!("uk2: condition compares a string with a number"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedResource {
    pub resource: RuntimeValue,
    pub arguments: Vec<RuntimeValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedArg {
    Direct {
        operand: Operand,
        value: RuntimeValue,
    },
    Value(RuntimeValue),
    Values(Vec<RuntimeValue>),
    Offset(u16),
    Resources(Vec<ResolvedResource>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCommand {
    pub offset: usize,
    pub opcode: TwoOpcode,
    pub arguments: Vec<ResolvedArg>,
}

pub trait Uk2Host {
    /// Loads the MES named by UK2 bytecode (normally a virtual `.mes1` name).
    fn load_mes(&mut self, name: &[u8]) -> Result<MesProgram>;

    /// Executes a non-core UK2 engine service.  Return the interpreter status
    /// that the original handler would place in DI (normally zero).
    fn command(&mut self, command: &ServiceCommand, memory: &mut Uk2Memory) -> Result<u16>;

    /// Pump asynchronous device state before each instruction. The PC-98
    /// mouse/music IRQs update script-visible words even while MES bytecode
    /// is spinning in a condition loop with no service opcode.
    fn poll(&mut self, _memory: &mut Uk2Memory) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoreOutcome {
    Continue,
    Return(u16),
}

fn status_outcome(status: u16) -> CoreOutcome {
    if status == 0 {
        CoreOutcome::Continue
    } else {
        CoreOutcome::Return(status)
    }
}

#[derive(Debug, Clone)]
pub struct Uk2Vm {
    program: MesProgram,
    pc: usize,
    memory: Uk2Memory,
    condition_flag: bool,
    last_j3_name: Option<Vec<u8>>,
    instruction_limit: Option<usize>,
    instructions_executed: usize,
}

impl Uk2Vm {
    pub fn new(program: MesProgram) -> Self {
        Self {
            program,
            pc: MES_CODE_OFFSET,
            memory: Uk2Memory::default(),
            condition_flag: false,
            last_j3_name: None,
            instruction_limit: None,
            instructions_executed: 0,
        }
    }

    pub fn memory(&self) -> &Uk2Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Uk2Memory {
        &mut self.memory
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn last_j3_name(&self) -> Option<&[u8]> {
        self.last_j3_name.as_deref()
    }

    pub fn run(&mut self, host: &mut impl Uk2Host) -> Result<u16> {
        self.execute_block(host)
    }

    /// Bound VM execution for offline probes that cannot provide live input.
    pub fn set_instruction_limit(&mut self, limit: usize) {
        self.instruction_limit = Some(limit);
    }

    fn execute_block(&mut self, host: &mut impl Uk2Host) -> Result<u16> {
        loop {
            host.poll(&mut self.memory)?;
            if self
                .instruction_limit
                .is_some_and(|limit| self.instructions_executed >= limit)
            {
                bail!(
                    "uk2: instruction limit reached after {} instructions at pc {:#x}",
                    self.instructions_executed,
                    self.pc
                );
            }
            self.instructions_executed += 1;
            let instruction = decode_instruction(&self.program, self.pc)
                .with_context(|| format!("uk2: decode failed at pc {:#x}", self.pc))?;
            self.pc = instruction.end_offset;
            match instruction.kind {
                InstructionKind::End => return Ok(0),
                InstructionKind::Return { status, .. } => {
                    if !(0..=7).contains(&status) {
                        bail!("uk2: return status {status} is outside the interpreter range 0..7");
                    }
                    return Ok(status as u16);
                }
                InstructionKind::Assign { destination, value } => {
                    let value = self.memory.eval_expression(&value)?;
                    self.memory.write_operand(&destination, value)?;
                }
                InstructionKind::Decrement(destination) => {
                    self.adjust_numeric(&destination, -1)?;
                }
                InstructionKind::Increment(destination) => {
                    self.adjust_numeric(&destination, 1)?;
                }
                InstructionKind::BlockCall { target } => {
                    let saved_condition = self.condition_flag;
                    let status = self.execute_block(host)?;
                    self.condition_flag = saved_condition;
                    self.set_pc(target)?;
                    if status != 0 {
                        return Ok(status);
                    }
                }
                InstructionKind::Command { opcode, arguments } => {
                    match self.execute_core_command(host, instruction.offset, opcode, &arguments)? {
                        CoreOutcome::Continue => {}
                        CoreOutcome::Return(status) => return Ok(status),
                    }
                }
            }
        }
    }

    fn execute_core_command(
        &mut self,
        host: &mut impl Uk2Host,
        offset: usize,
        opcode: TwoOpcode,
        arguments: &[InstructionArg],
    ) -> Result<CoreOutcome> {
        match opcode {
            TwoOpcode::J0 => {
                self.set_pc(require_offset(arguments)?)?;
                Ok(CoreOutcome::Continue)
            }
            TwoOpcode::J1 => {
                let return_pc = self.pc;
                self.set_pc(require_offset(arguments)?)?;
                let status = self.execute_block(host)?;
                self.pc = return_pc;
                if status < 2 {
                    bail!("uk2: J1 callee returned {status}, expected status >= 2");
                }
                if status == 2 {
                    Ok(CoreOutcome::Continue)
                } else {
                    Ok(CoreOutcome::Return(status))
                }
            }
            TwoOpcode::J2 => {
                let name = self.direct_string(arguments, 0)?;
                let status = self.call_external(host, &name)?;
                if status < 3 {
                    bail!("uk2: J2 callee returned {status}, expected status >= 3");
                }
                if status == 3 {
                    Ok(CoreOutcome::Continue)
                } else {
                    Ok(CoreOutcome::Return(status))
                }
            }
            TwoOpcode::J3 => {
                self.last_j3_name = Some(self.direct_string(arguments, 0)?);
                Ok(CoreOutcome::Return(5))
            }
            TwoOpcode::L0 => Ok(status_outcome(self.execute_l0(host, arguments)?)),
            TwoOpcode::L1 => Ok(status_outcome(self.execute_l1(host, arguments)?)),
            TwoOpcode::L2 => Ok(status_outcome(self.execute_l2(host, arguments)?)),
            TwoOpcode::L3 => Ok(status_outcome(self.execute_l3(host, arguments)?)),
            TwoOpcode::L4 => {
                let condition = require_condition(arguments)?;
                let target = require_offset(arguments)?;
                if !self.memory.eval_condition(condition)? {
                    self.condition_flag = false;
                    self.set_pc(target)?;
                    return Ok(CoreOutcome::Continue);
                }
                self.condition_flag = true;
                let status = self.execute_block(host)?;
                Ok(CoreOutcome::Return(if status <= 1 { 0 } else { status }))
            }
            TwoOpcode::L5 => Ok(status_outcome(self.execute_l5(host, arguments)?)),
            TwoOpcode::T0 => {
                self.store_text_slots(arguments, true)?;
                Ok(CoreOutcome::Continue)
            }
            TwoOpcode::T1 => {
                self.store_text_slots(arguments, false)?;
                Ok(CoreOutcome::Continue)
            }
            TwoOpcode::M3 => Ok(CoreOutcome::Continue),
            _ => {
                let command = self.resolve_service(offset, opcode, arguments)?;
                Ok(status_outcome(host.command(&command, &mut self.memory)?))
            }
        }
    }

    fn execute_l0(&mut self, host: &mut impl Uk2Host, arguments: &[InstructionArg]) -> Result<u16> {
        let condition = require_condition(arguments)?;
        let target = require_offset(arguments)?;
        let body = self.pc;
        let mut active = true;
        let mut status = 0;
        while active && self.memory.eval_condition(condition)? {
            self.pc = body;
            status = self.execute_block(host)?;
            if status != 0 {
                active = false;
            }
            if status == 1 {
                status = 0;
            }
            if status == 6 {
                active = true;
                status = 0;
            }
        }
        self.set_pc(target)?;
        Ok(status)
    }

    fn execute_l1(&mut self, host: &mut impl Uk2Host, arguments: &[InstructionArg]) -> Result<u16> {
        let condition = require_condition(arguments)?;
        let target = require_offset(arguments)?;
        if self.memory.eval_condition(condition)? {
            self.condition_flag = true;
            self.execute_block(host)
        } else {
            self.condition_flag = false;
            self.set_pc(target)?;
            Ok(0)
        }
    }

    fn execute_l2(&mut self, host: &mut impl Uk2Host, arguments: &[InstructionArg]) -> Result<u16> {
        let target = require_offset(arguments)?;
        if self.condition_flag {
            self.set_pc(target)?;
            Ok(0)
        } else {
            let status = self.execute_block(host)?;
            self.condition_flag = false;
            Ok(status)
        }
    }

    fn execute_l3(&mut self, host: &mut impl Uk2Host, arguments: &[InstructionArg]) -> Result<u16> {
        let destination = require_direct(arguments, 0)?;
        let start = self
            .memory
            .eval_expression(require_expression(arguments, 1)?)?
            .number()?;
        let end = self
            .memory
            .eval_expression(require_expression(arguments, 2)?)?
            .number()?;
        let step = self
            .memory
            .eval_expression(require_expression(arguments, 3)?)?
            .number()?;
        let target = require_offset(arguments)?;
        let body = self.pc;
        let mut current = start;
        let mut status = 0;
        while current <= end {
            self.memory.write_operand(
                destination,
                RuntimeValue::Number {
                    value: current,
                    width: NumericWidth::Word,
                },
            )?;
            self.pc = body;
            status = self.execute_block(host)?;
            if status == 0 || status == 6 {
                current = current.wrapping_add(step);
                continue;
            }
            if status == 1 {
                status = 0;
            }
            break;
        }
        self.set_pc(target)?;
        Ok(status)
    }

    fn execute_l5(&mut self, host: &mut impl Uk2Host, arguments: &[InstructionArg]) -> Result<u16> {
        let condition = require_condition(arguments)?;
        let target = require_offset(arguments)?;
        if !self.condition_flag {
            return self.execute_l1(host, arguments);
        }
        // The original still evaluates the condition before skipping this
        // else-if arm, so preserve any operand reads/errors.
        let _ = self.memory.eval_condition(condition)?;
        self.set_pc(target)?;
        self.condition_flag = true;
        Ok(0)
    }

    fn call_external(&mut self, host: &mut impl Uk2Host, name: &[u8]) -> Result<u16> {
        let callee = host.load_mes(name)?;
        let caller = std::mem::replace(&mut self.program, callee);
        let caller_pc = self.pc;
        let saved_locals = self.memory.reset_external_locals();
        self.pc = MES_CODE_OFFSET;
        let result = self.execute_block(host);
        self.program = caller;
        self.pc = caller_pc;
        self.memory.restore_external_locals(saved_locals);
        result
    }

    fn store_text_slots(&mut self, arguments: &[InstructionArg], current: bool) -> Result<()> {
        let expressions = arguments
            .iter()
            .find_map(|argument| match argument {
                InstructionArg::ExpressionList(expressions) => Some(expressions),
                _ => None,
            })
            .ok_or_else(|| anyhow::anyhow!("uk2: T opcode has no expression list"))?;
        for (index, expression) in expressions.iter().enumerate() {
            let value = self.memory.eval_expression(expression)?;
            let bytes = checked_string(value, TEXT_SLOT_MAX_LEN)?;
            let index = u8::try_from(index)
                .map_err(|_| anyhow::anyhow!("uk2: text slot index exceeds u8"))?;
            if current {
                self.memory.set_table5_slot(index, bytes);
            } else {
                self.memory.set_table6_slot(index, bytes);
            }
        }
        Ok(())
    }

    fn resolve_service(
        &mut self,
        offset: usize,
        opcode: TwoOpcode,
        arguments: &[InstructionArg],
    ) -> Result<ServiceCommand> {
        let mut resolved = Vec::with_capacity(arguments.len());
        for argument in arguments {
            resolved.push(match argument {
                InstructionArg::Direct(operand) => ResolvedArg::Direct {
                    operand: operand.clone(),
                    value: self.memory.read_operand(operand)?,
                },
                InstructionArg::Expression(expression) => {
                    ResolvedArg::Value(self.memory.eval_expression(expression)?)
                }
                InstructionArg::ExpressionList(expressions) => ResolvedArg::Values(
                    expressions
                        .iter()
                        .map(|expression| self.memory.eval_expression(expression))
                        .collect::<Result<Vec<_>>>()?,
                ),
                InstructionArg::Offset(target) => ResolvedArg::Offset(*target),
                InstructionArg::ResourceList(resources) => {
                    ResolvedArg::Resources(self.resolve_resources(resources)?)
                }
                InstructionArg::Condition(_) => {
                    bail!("uk2: condition argument unexpectedly reached service opcode {opcode}")
                }
            });
        }
        Ok(ServiceCommand {
            offset,
            opcode,
            arguments: resolved,
        })
    }

    fn resolve_resources(
        &mut self,
        resources: &[ResourceArgument],
    ) -> Result<Vec<ResolvedResource>> {
        resources
            .iter()
            .map(|resource| {
                Ok(ResolvedResource {
                    resource: self.memory.eval_expression(&resource.resource)?,
                    arguments: resource
                        .arguments
                        .iter()
                        .map(|argument| self.memory.eval_expression(argument))
                        .collect::<Result<Vec<_>>>()?,
                })
            })
            .collect()
    }

    fn direct_string(&mut self, arguments: &[InstructionArg], index: usize) -> Result<Vec<u8>> {
        let operand = require_direct(arguments, index)?;
        let mut bytes = self.memory.read_operand(operand)?.string()?.to_vec();
        if bytes.len() > 13 {
            bytes.truncate(13);
        }
        Ok(bytes)
    }

    fn adjust_numeric(&mut self, operand: &Operand, delta: i32) -> Result<()> {
        let RuntimeValue::Number { value, width } = self.memory.read_operand(operand)? else {
            bail!("uk2: increment/decrement destination is not numeric")
        };
        let max = match width {
            NumericWidth::Byte => 0xff,
            NumericWidth::Word => 0xffff,
        };
        let adjusted = (i32::from(value) + delta).clamp(0, max);
        self.memory.write_operand(
            operand,
            RuntimeValue::Number {
                value: adjusted as u16,
                width,
            },
        )
    }

    fn set_pc(&mut self, target: u16) -> Result<()> {
        let target = usize::from(target);
        if target < MES_CODE_OFFSET || target >= self.program.len() {
            bail!(
                "uk2: jump target {target:#x} is outside MES {:#x}",
                self.program.len()
            );
        }
        self.pc = target;
        Ok(())
    }
}

fn require_offset(arguments: &[InstructionArg]) -> Result<u16> {
    arguments
        .iter()
        .rev()
        .find_map(|argument| match argument {
            InstructionArg::Offset(target) => Some(*target),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("uk2: command has no O16 target"))
}

fn require_condition(arguments: &[InstructionArg]) -> Result<&Condition> {
    arguments
        .iter()
        .find_map(|argument| match argument {
            InstructionArg::Condition(condition) => Some(condition),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("uk2: command has no condition"))
}

fn require_direct(arguments: &[InstructionArg], index: usize) -> Result<&Operand> {
    arguments
        .iter()
        .filter_map(|argument| match argument {
            InstructionArg::Direct(operand) => Some(operand),
            _ => None,
        })
        .nth(index)
        .ok_or_else(|| anyhow::anyhow!("uk2: command has no direct operand #{index}"))
}

fn require_expression(arguments: &[InstructionArg], index: usize) -> Result<&Expression> {
    arguments
        .get(index)
        .and_then(|argument| match argument {
            InstructionArg::Expression(expression) => Some(expression),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("uk2: command argument #{index} is not an expression"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mes::MES_MAGIC;

    struct TestHost;

    impl Uk2Host for TestHost {
        fn load_mes(&mut self, _name: &[u8]) -> Result<MesProgram> {
            bail!("not used")
        }

        fn command(&mut self, command: &ServiceCommand, _memory: &mut Uk2Memory) -> Result<u16> {
            bail!("unexpected service command {}", command.opcode)
        }
    }

    fn program(code: &[u8]) -> MesProgram {
        let mut bytes = MES_MAGIC.to_vec();
        bytes.extend_from_slice(code);
        MesProgram::from_bytes(bytes).unwrap()
    }

    #[test]
    fn assignment_and_saturating_increment_match_core_vm() {
        // system[1] = 0xfffe; increment twice; R3
        let mut vm = Uk2Vm::new(program(&[
            0x01, 0x01, 0x87, 0x1f, 0xfe, 0xff, 0x04, 0x01, 0x04, 0x01, b'R', b'3',
        ]));
        assert_eq!(vm.run(&mut TestHost).unwrap(), 3);
        assert_eq!(vm.memory().system_word(1), Some(0xffff));
    }

    #[test]
    fn condition_uses_reference_comparison_codes() {
        let condition = Condition {
            clauses: vec![crate::value::ConditionClause {
                left: Expression {
                    terms: vec![crate::value::ExpressionTerm {
                        op: ExpressionOp::Set,
                        operand: Operand::Immediate(1),
                    }],
                },
                compare: CompareOp::Less,
                right: Expression {
                    terms: vec![crate::value::ExpressionTerm {
                        op: ExpressionOp::Set,
                        operand: Operand::Immediate(2),
                    }],
                },
                join_after: None,
            }],
        };
        assert!(Uk2Memory::default().eval_condition(&condition).unwrap());
    }
}
