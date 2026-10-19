//! MES interpreter: `sub_19640` and the operand/expression/condition
//! decoders of seg003.  Operands are decoded on the fly into far pointers
//! exactly like the executable, so every script-visible variable is the real
//! engine variable in the data segment.

use anyhow::Result;

use super::Engine;
use super::ds::*;
use super::mem::{FarPtr, ds_ptr, far, ptr_add};

pub const MES_CODE_OFFSET: u16 = 0x17;
const RECORD_SIZE: usize = 0x1F8;
const RECORD_COUNT: i32 = 0x1A4;
const LOCAL_TABLE_SIZE: usize = 0x44;

pub(crate) struct Program {
    pub bytes: Vec<u8>,
}

impl Engine {
    // ---- byte stream ---------------------------------------------------

    /// `sub_1ED9A`.
    pub fn fetch(&mut self) -> Result<u8> {
        let pc = self.w(MES_PC);
        if pc > self.w(MES_LEN) {
            return Err(self.fatal(0x0d));
        }
        let value = self
            .programs
            .last()
            .and_then(|p| p.bytes.get(usize::from(pc)))
            .copied()
            .unwrap_or(0);
        self.set_w(MES_PC, pc.wrapping_add(1));
        Ok(value)
    }

    pub fn fetch_u16(&mut self) -> Result<u16> {
        let lo = u16::from(self.fetch()?);
        let hi = u16::from(self.fetch()?);
        Ok(lo.wrapping_add(hi << 8))
    }

    // ---- operands --------------------------------------------------------

    /// `sub_14FFA`: decode one direct operand to a far pointer.
    pub fn operand(&mut self) -> Result<FarPtr> {
        let encoded = self.fetch()?;
        let class = encoded >> 5;
        let index = u16::from(encoded & 0x1f);
        let ptr = match class {
            0 => {
                let ptr = if index < 0x1f {
                    ds_ptr(SYSTEM_WORDS + index * 2)
                } else {
                    let value = self.fetch_u16()?;
                    self.set_w(IMM_SCRATCH, value);
                    ds_ptr(IMM_SCRATCH)
                };
                self.set_w(OPERAND_KIND, 0);
                self.set_w(OPERAND_WORD, 1);
                ptr
            }
            2 => {
                let table = self.mem.d(LOCAL_TABLE);
                self.set_w(OPERAND_KIND, 0);
                self.set_w(OPERAND_WORD, 1);
                ptr_add(table, i32::from(index) * 2)
            }
            1 => {
                self.set_w(OPERAND_KIND, 1);
                ds_ptr(FIXED_STRINGS + index * 0x3e)
            }
            6 => {
                self.set_w(OPERAND_KIND, 2);
                ds_ptr(LARGE_STRINGS.wrapping_add(index.wrapping_mul(0x7d2)))
            }
            5 => {
                match index {
                    0 => {
                        self.set_w(OPERAND_KIND, 0);
                        self.set_w(OPERAND_WORD, 0);
                    }
                    1 => {
                        self.set_w(OPERAND_KIND, 0);
                        self.set_w(OPERAND_WORD, 1);
                    }
                    2 => self.set_w(OPERAND_KIND, 1),
                    _ => {}
                }
                let slot = u16::from(self.fetch()?);
                self.mem.d(INDIRECT_TABLE.wrapping_add(slot * 4))
            }
            4 => {
                let result = self.expression()?;
                let value = self.mem.rw(result) as i16;
                self.set_w(OPERAND_KIND, 0);
                self.set_w(OPERAND_WORD, 0);
                if value > 0x100 {
                    return Err(self.fatal(0x17));
                }
                match index {
                    5 => ds_ptr(INDEXED_5.wrapping_add(value as u16)),
                    8 => ds_ptr(INDEXED_8.wrapping_add(value as u16)),
                    _ => 0,
                }
            }
            3 => {
                let mut si = 0u16;
                loop {
                    let byte = self.fetch()?;
                    match byte {
                        5 | 6 => {
                            let slot = u16::from(self.fetch()?).wrapping_sub(1);
                            let src = if byte == 5 {
                                ptr_add(self.mem.d(MES_RECORD), i32::from(slot) * 0x15)
                            } else {
                                ds_ptr(GLOBAL_SLOTS.wrapping_add(slot.wrapping_mul(0x15)))
                            };
                            self.mem.strcpy(ds_ptr(INLINE_BUF.wrapping_add(si)), src);
                            let len = self.mem.strlen(src) as u16;
                            si = si.wrapping_add(len).wrapping_sub(1);
                        }
                        _ => self.set_b(INLINE_BUF.wrapping_add(si), byte),
                    }
                    if (si as i16) >= 0x7d1 {
                        return Err(self.fatal(0x1b));
                    }
                    si = si.wrapping_add(1);
                    if byte == 0 {
                        break;
                    }
                }
                self.set_w(OPERAND_KIND, 1);
                ds_ptr(INLINE_BUF)
            }
            _ => 0,
        };
        Ok(ptr)
    }

    fn acc(&self) -> i32 {
        (u32::from(self.w(ACC_LO)) | (u32::from(self.w(ACC_HI)) << 16)) as i32
    }

    fn set_acc(&mut self, value: i32) {
        self.set_w(ACC_LO, value as u16);
        self.set_w(ACC_HI, ((value as u32) >> 16) as u16);
    }

    /// `sub_15222`: evaluate one expression.  Returns a pointer to the
    /// numeric accumulator, a heap string, or null for the list terminator.
    pub fn expression(&mut self) -> Result<FarPtr> {
        let mut result: FarPtr = 0;
        let mut any = false;
        loop {
            let control = self.fetch()?;
            let last = control & 0x80 != 0;
            let op = control & 0x7f;
            if op == 0 {
                break;
            }
            any = true;
            let saved = self.acc();
            self.set_acc(0);
            let operand = self.operand()?;
            self.set_acc(saved);
            let value = if self.w(OPERAND_WORD) == 0 {
                u16::from(self.mem.rb(operand))
            } else {
                self.mem.rw(operand)
            };
            let is_string = self.w(OPERAND_KIND) != 0;
            match op {
                7 => {
                    if !is_string {
                        result = ds_ptr(ACC_LO);
                        self.set_acc(i32::from(value));
                    } else {
                        let len = self.mem.strlen(operand);
                        result = self.mem.realloc(result_heap(result), len + 1);
                        self.mem.strcpy(result, operand);
                    }
                }
                1 => {
                    if !is_string {
                        let acc = self.acc().wrapping_add(i32::from(value));
                        self.set_acc(acc);
                    } else {
                        let len = self.mem.strlen(result) + self.mem.strlen(operand);
                        let head = self.mem.cstr(result);
                        let tail = self.mem.cstr(operand);
                        result = self.mem.realloc(result_heap(result), len + 1);
                        let mut joined = head;
                        joined.extend_from_slice(&tail);
                        self.mem.strcpy_bytes(result, &joined);
                    }
                }
                2 => {
                    let acc = self.acc().wrapping_sub(i32::from(value));
                    self.set_acc(acc);
                }
                3 => {
                    let acc = self.acc().wrapping_mul(i32::from(value));
                    self.set_acc(acc);
                }
                4 | 5 => {
                    if value == 0 {
                        return Err(anyhow::anyhow!("uk2: divide error"));
                    }
                    let acc = self.acc();
                    let divisor = i32::from(value);
                    self.set_acc(if op == 4 {
                        acc.wrapping_div(divisor)
                    } else {
                        acc.wrapping_rem(divisor)
                    });
                }
                _ => return Err(self.fatal(5)),
            }
            if last {
                break;
            }
        }
        if self.w(OPERAND_KIND) == 0 {
            let mut acc = self.acc();
            if acc < 0 {
                acc = 0;
            }
            if self.w(OPERAND_WORD) != 0 {
                if acc > 0xffff {
                    acc = 0xffff;
                }
            } else if acc > 0xff {
                acc = 0xff;
            }
            self.set_acc(acc);
            if result == 0 && any {
                result = ds_ptr(ACC_LO);
            }
        }
        Ok(result)
    }

    /// Expression evaluated and read as a word (`mov ax, es:[bx]`).
    pub fn expr_word(&mut self) -> Result<u16> {
        let ptr = self.expression()?;
        Ok(self.mem.rw(ptr))
    }

    pub fn expr_byte(&mut self) -> Result<u8> {
        let ptr = self.expression()?;
        Ok(self.mem.rb(ptr))
    }

    /// Frees a string expression result (`sub_2302B`).
    pub fn free_expr(&mut self, ptr: FarPtr) {
        if ptr != ds_ptr(ACC_LO) {
            self.mem.free(ptr);
        }
    }

    /// `sub_19598(n, buf)`: up to `n` null-terminated list values.
    pub fn expr_list(&mut self, max: usize) -> Result<Vec<u16>> {
        let mut values = Vec::new();
        while values.len() < max {
            let ptr = self.expression()?;
            if ptr == 0 {
                break;
            }
            values.push(self.mem.rw(ptr));
        }
        Ok(values)
    }

    /// Fixed-size variant: the handler's stack array keeps zeroes for
    /// missing entries.
    pub fn expr_array<const N: usize>(&mut self) -> Result<[u16; N]> {
        let values = self.expr_list(N)?;
        let mut out = [0u16; N];
        out[..values.len()].copy_from_slice(&values);
        Ok(out)
    }

    /// `sub_195D1`: operand (mode 0) or expression (mode 1) formatted with
    /// `%-13.13s` into a 14-byte name.
    pub fn read_name(&mut self, expression: bool) -> Result<Vec<u8>> {
        let ptr = if expression {
            self.expression()?
        } else {
            self.operand()?
        };
        let mut name = self.mem.cstr(ptr);
        name.truncate(13);
        name.resize(13, b' ');
        if expression {
            self.free_expr(ptr);
        }
        Ok(name)
    }

    // ---- conditions ------------------------------------------------------

    /// `sub_1542C`.
    pub fn condition(&mut self) -> Result<bool> {
        self.set_w(COND_FLAG, 0);
        let mut join = 2u8;
        loop {
            let left = self.expression()?;
            let mut left_value = 0u16;
            if self.w(OPERAND_KIND) == 0 {
                left_value = if self.w(OPERAND_WORD) == 0 {
                    u16::from(self.mem.rb(left))
                } else {
                    self.mem.rw(left)
                };
            }
            let compare = self.fetch()?;
            let right = self.expression()?;
            let mut right_value = 0u16;
            let string = self.w(OPERAND_KIND) != 0;
            if !string {
                right_value = if self.w(OPERAND_WORD) == 0 {
                    u16::from(self.mem.rb(right))
                } else {
                    self.mem.rw(right)
                };
            }
            let outcome = if string {
                let a = self.mem.cstr(left);
                let b = self.mem.cstr(right);
                let ordering = strcmp(&a, &b);
                match compare {
                    1 => ordering < 0,
                    2 => ordering > 0,
                    3 => ordering == 0,
                    4 => ordering != 0,
                    _ => false,
                }
            } else {
                match compare {
                    1 => left_value < right_value,
                    2 => left_value > right_value,
                    3 => left_value == right_value,
                    4 => left_value != right_value,
                    _ => false,
                }
            };
            let flag = self.w(COND_FLAG);
            match join {
                1 => self.set_w(COND_FLAG, flag & u16::from(outcome)),
                2 => self.set_w(COND_FLAG, flag | u16::from(outcome)),
                _ => {}
            }
            if string {
                self.free_expr(left);
                self.free_expr(right);
            }
            join = self.fetch()?;
            if join == 0 {
                break;
            }
        }
        Ok(self.w(COND_FLAG) != 0)
    }

    // ---- single-byte commands and loops ---------------------------------

    /// `sub_156C0`: assignment.
    fn assign(&mut self) -> Result<()> {
        let dst = self.operand()?;
        match self.w(OPERAND_KIND) {
            0 => {
                if self.w(OPERAND_WORD) != 0 {
                    let value = self.expr_word()?;
                    self.mem.ww(dst, value);
                } else {
                    let value = self.expr_word()?.min(0xff);
                    self.mem.wb(dst, value as u8);
                }
            }
            kind @ (1 | 2) => {
                let src = self.expression()?;
                let limit = if kind == 1 { 0x3d } else { 0x7d1 };
                if self.mem.strlen(src) >= limit {
                    return Err(self.fatal(0x0f));
                }
                self.mem.strcpy(dst, src);
                self.free_expr(src);
            }
            _ => {}
        }
        Ok(())
    }

    fn jump_target(&mut self) -> Result<u16> {
        self.fetch_u16()
    }

    /// `sub_1579F`: L0 loop.
    fn op_l0(&mut self) -> Result<u16> {
        let mut status = 0u16;
        let mut active = true;
        let start = self.w(MES_PC);
        loop {
            let ok = self.condition()?;
            if !ok || !active {
                break;
            }
            let pc = self.w(MES_PC).wrapping_add(2);
            self.set_w(MES_PC, pc);
            status = self.run_block()?;
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
            self.set_w(MES_PC, start);
        }
        let target = self.jump_target()?;
        self.set_w(MES_PC, target);
        Ok(status)
    }

    /// `sub_15836`: L1.
    fn op_l1(&mut self) -> Result<u16> {
        if self.condition()? {
            let pc = self.w(MES_PC).wrapping_add(2);
            self.set_w(MES_PC, pc);
            let status = self.run_block()?;
            self.set_w(COND_FLAG, 1);
            Ok(status)
        } else {
            self.set_w(COND_FLAG, 0);
            let target = self.jump_target()?;
            self.set_w(MES_PC, target);
            Ok(0)
        }
    }

    /// `sub_15872`: L2.
    fn op_l2(&mut self) -> Result<u16> {
        if self.w(COND_FLAG) != 0 {
            let target = self.jump_target()?;
            self.set_w(MES_PC, target);
            Ok(0)
        } else {
            let pc = self.w(MES_PC).wrapping_add(2);
            self.set_w(MES_PC, pc);
            let status = self.run_block()?;
            self.set_w(COND_FLAG, 0);
            Ok(status)
        }
    }

    /// `sub_158A7`: L3 counted loop.
    fn op_l3(&mut self) -> Result<u16> {
        let dst = self.operand()?;
        let start = self.expr_word()?;
        let end = self.expr_word()?;
        let step = self.expr_word()?;
        let exit = self.fetch_u16()?;
        let mut status = 0u16;
        let mut value = start;
        while value <= end {
            self.mem.ww(dst, value);
            let body = self.w(MES_PC);
            status = self.run_block()?;
            if status == 0 || status == 6 {
                self.set_w(MES_PC, body);
                value = value.wrapping_add(step);
                status = 0;
                continue;
            }
            if status == 1 {
                status = 0;
            }
            break;
        }
        self.set_w(MES_PC, exit);
        Ok(status)
    }

    /// `sub_15783`: L4.
    fn op_l4(&mut self) -> Result<u16> {
        let status = self.op_l1()?;
        if self.w(COND_FLAG) != 0 {
            Ok(if status == 0 { 1 } else { status })
        } else {
            Ok(0)
        }
    }

    /// `sub_15802`: L5.
    fn op_l5(&mut self) -> Result<u16> {
        if self.w(COND_FLAG) == 0 {
            return self.op_l1();
        }
        self.condition()?;
        let target = self.jump_target()?;
        self.set_w(MES_PC, target);
        self.set_w(COND_FLAG, 1);
        Ok(0)
    }

    // ---- external MES --------------------------------------------------------

    /// `sub_15595`: run another MES with a fresh record and local table.
    pub fn call_mes(&mut self, name: &[u8]) -> Result<u16> {
        if name.first().copied().unwrap_or(0) == 0 {
            return Ok(0);
        }
        let saved_record = self.mem.d(MES_RECORD);
        let record = self.mem.alloc(RECORD_SIZE);
        self.mem.set_d(MES_RECORD, record);
        let saved_pc = self.w(MES_PC);
        let saved_len = self.w(MES_LEN);
        let bytes = self.read_resource(name)?;
        if bytes.is_empty() || bytes.len() > 0xea60 {
            return Err(self.fatal(2));
        }
        let len = bytes.len() as u16;
        if u32::from(len) + u32::from(self.w(MES_STACK)) > 0x88b8 {
            return Err(self.fatal(0x11));
        }
        self.set_w(MES_LEN, len);
        self.programs.push(Program { bytes });
        self.set_w(MES_PC, MES_CODE_OFFSET);
        let saved_locals = self.mem.d(LOCAL_TABLE);
        let locals = self.mem.alloc(LOCAL_TABLE_SIZE);
        self.mem.set_d(LOCAL_TABLE, locals);
        let saved_name = self.mem.cstr(ds_ptr(CUR_MES_NAME));
        if self.trace_mes {
            eprintln!(
                "[tick {}] mes {}",
                self.w(TICKS),
                String::from_utf8_lossy(&trim_name(name))
            );
        }
        self.mem
            .strcpy_bytes(ds_ptr(CUR_MES_NAME), &trim_name(name));
        let stack = self.w(MES_STACK).wrapping_add(len);
        self.set_w(MES_STACK, stack);
        let result = self.run_block();
        let stack = self.w(MES_STACK).wrapping_sub(len);
        self.set_w(MES_STACK, stack);
        self.mem.strcpy_bytes(ds_ptr(CUR_MES_NAME), &saved_name);
        self.mem.set_d(LOCAL_TABLE, saved_locals);
        self.mem.free(locals);
        self.programs.pop();
        self.set_w(MES_PC, saved_pc);
        self.set_w(MES_LEN, saved_len);
        self.mem.set_d(MES_RECORD, saved_record);
        self.mem.free(record);
        let status = result?;
        if !(3..=5).contains(&status) && status != 0 {
            return Err(self.fatal(7));
        }
        Ok(status)
    }

    // ---- dispatcher ------------------------------------------------------------

    /// `sub_19640`: execute until a block returns a status.
    pub fn run_block(&mut self) -> Result<u16> {
        loop {
            self.instruction_tick()?;
            if self.w(ABORT) != 0 {
                return Ok(4);
            }
            if self.w(SKIP_SERVICE) == 0 && !self.interpreter_service()? {
                continue;
            }
            let op = self.fetch()?;
            let mut status: u16 = 0;
            match op {
                0 => return Ok(0),
                1 => self.assign()?,
                2 => {
                    let target = self.fetch_u16()?;
                    let saved = self.w(COND_FLAG);
                    status = self.run_block()?;
                    self.set_w(COND_FLAG, saved);
                    self.set_w(MES_PC, target);
                }
                3 | 4 => {
                    let dst = self.operand()?;
                    let up = op == 4;
                    if self.w(OPERAND_WORD) != 0 {
                        let v = self.mem.rw(dst);
                        if up && v < 0xffff {
                            self.mem.ww(dst, v + 1);
                        } else if !up && v > 0 {
                            self.mem.ww(dst, v - 1);
                        }
                    } else {
                        let v = self.mem.rb(dst);
                        if up && v < 0xff {
                            self.mem.wb(dst, v + 1);
                        } else if !up && v > 0 {
                            self.mem.wb(dst, v - 1);
                        }
                    }
                }
                b'R' => {
                    status = u16::from(self.fetch()?).wrapping_sub(0x30);
                    if status == 0 {
                        continue;
                    }
                    if status > 7 {
                        return Err(self.fatal(0x1c));
                    }
                    return Ok(status);
                }
                _ => {
                    if self.w(OPCODES_ENABLED) == 0 {
                        return Ok(0);
                    }
                    let second = self.fetch()?;
                    if self.trace {
                        let name = self.mem.cstr(ds_ptr(CUR_MES_NAME));
                        eprintln!(
                            "[{:>6}] {} {:04x} {}{}",
                            self.w(TICKS),
                            String::from_utf8_lossy(&name),
                            self.w(MES_PC).wrapping_sub(2),
                            op as char,
                            second as char
                        );
                    }
                    status = self.dispatch(op, second)?;
                    if status == STATUS_RETURN_ZERO {
                        return Ok(0);
                    }
                }
            }
            if status != 0 {
                if status > 7 {
                    return Err(self.fatal(0x1c));
                }
                return Ok(status);
            }
        }
    }

    /// Pre-instruction service of `sub_19640`.  Returns `false` when the
    /// interpreter loops back without executing an instruction.
    fn interpreter_service(&mut self) -> Result<bool> {
        let wait = u16::from(self.b(WAIT_OBJECT));
        if wait != 0 && self.shift_state() & 0x10 != 0 {
            if self.obj_raise(wait)? != 0 {
                return Ok(false);
            }
            self.set_b(WAIT_FLAG, 0);
            self.set_b(WAIT_OBJECT, 0);
            return Ok(false);
        }
        if self.w(RIGHT_HELD) != 0 {
            let mut handled = false;
            let wait = u16::from(self.b(WAIT_OBJECT));
            if wait != 0 {
                if self.w(WAIT_MODE) == 0 {
                    if wait != self.w(TOP_OBJECT) {
                        self.obj_raise(wait)?;
                    } else {
                        let cursor = self.cursor_show(0);
                        self.text_scroll_request(5)?;
                        self.cursor_show(cursor);
                    }
                } else if self.obj_raise(wait)? == 0 {
                    self.set_b(WAIT_FLAG, 0);
                    self.set_b(WAIT_OBJECT, 0);
                    self.set_w(WAIT_MODE, 2);
                }
                handled = true;
            }
            if handled {
                while self.w(RIGHT_HELD) != 0 {
                    self.idle()?;
                }
                self.set_w(RIGHT_PRESSES, 0);
                return Ok(false);
            }
        }
        if self.b(WAIT_FLAG) != 0 && self.w(TEXT_DONE) != 0 {
            self.set_b(WAIT_FLAG, 0);
            self.set_b(WAIT_OBJECT, 0);
        }
        self.mouse_service()?;
        if self.b(WAIT_OBJECT) != 0 {
            self.idle()?;
            return Ok(false);
        }
        if self.w(W_28FC4) != 0 {
            // A-family effect sequencer requires EMS; never active here.
        }
        Ok(true)
    }

    /// Two-character opcode handlers (`jpt_19849`).
    fn dispatch(&mut self, a: u8, b: u8) -> Result<u16> {
        let status = match (a, b) {
            // ---- J / L --------------------------------------------------
            (b'J', b'0') => {
                let target = self.fetch_u16()?;
                self.set_w(MES_PC, target);
                0
            }
            (b'J', b'1') => {
                let saved = self.w(MES_PC);
                let target = self.fetch_u16()?;
                self.set_w(MES_PC, target);
                let status = self.run_block()?;
                self.set_w(MES_PC, saved.wrapping_add(2));
                if status < 2 {
                    return Err(self.fatal(8));
                }
                if status == 2 { 0 } else { status }
            }
            (b'J', b'2') => {
                let name = self.read_name(false)?;
                let status = self.call_mes(&name)?;
                if status < 3 {
                    return Err(self.fatal(9));
                }
                if status == 3 { 0 } else { status }
            }
            (b'J', b'3') => {
                let name = self.read_name(false)?;
                self.mem.write_bytes(ds_ptr(NEXT_MES_NAME), &name);
                self.set_b(NEXT_MES_NAME + 13, 0);
                5
            }
            (b'L', b'0') => self.op_l0()?,
            (b'L', b'1') => self.op_l1()?,
            (b'L', b'2') => self.op_l2()?,
            (b'L', b'3') => self.op_l3()?,
            (b'L', b'4') => {
                let status = self.op_l4()?;
                if status == 1 {
                    return Ok(STATUS_RETURN_ZERO);
                }
                status
            }
            (b'L', b'5') => self.op_l5()?,
            // ---- T ----------------------------------------------------
            (b'T', b'0') => {
                let record = self.mem.d(MES_RECORD);
                self.mem.ww(ptr_add(record, RECORD_COUNT), 0);
                loop {
                    let ptr = self.expression()?;
                    if ptr == 0 {
                        break;
                    }
                    let count = self.mem.rw(ptr_add(record, RECORD_COUNT));
                    self.mem
                        .strcpy(ptr_add(record, i32::from(count) * 0x15), ptr);
                    self.mem
                        .ww(ptr_add(record, RECORD_COUNT), count.wrapping_add(1));
                    self.free_expr(ptr);
                }
                0
            }
            (b'T', b'1') => {
                let mut index = 0u16;
                loop {
                    let ptr = self.expression()?;
                    if ptr == 0 {
                        break;
                    }
                    self.mem.strcpy(
                        ds_ptr(GLOBAL_SLOTS.wrapping_add(index.wrapping_mul(0x15))),
                        ptr,
                    );
                    index += 1;
                    self.free_expr(ptr);
                }
                0
            }
            _ => self.dispatch_service(a, b)?,
        };
        Ok(status)
    }
}

/// Sentinel: handler asked the block to return status 0 (L4 taken arm).
pub(crate) const STATUS_RETURN_ZERO: u16 = 0xfff0;

fn result_heap(ptr: FarPtr) -> FarPtr {
    if super::mem::seg_of(ptr) == super::mem::DS_SEG {
        0
    } else {
        ptr
    }
}

/// Borland `strcmp` sign.
pub fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    for i in 0.. {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        if x != y {
            return i32::from(x) - i32::from(y);
        }
        if x == 0 {
            return 0;
        }
    }
    0
}

/// Strips the `%-13.13s` padding of an engine resource name.
pub fn trim_name(name: &[u8]) -> Vec<u8> {
    let end = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    let mut out = name[..end].to_vec();
    while out.last() == Some(&b' ') {
        out.pop();
    }
    out
}

#[allow(dead_code)]
fn _far(seg: u16, off: u16) -> FarPtr {
    far(seg, off)
}
