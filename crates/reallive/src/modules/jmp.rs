//! Flow control: `goto`/`gosub` families (modules 0:1, 0:5 and 0:6),
//! `jump`/`farcall`, returns and the `*_with` argument-passing calls.

use anyhow::{Result, bail};

use crate::bytecode::{Command, CommandKind};
use crate::expr::Expr;
use crate::machine::{Machine, Next};

/// Is this opcode of a flow module a gosub (as opposed to a goto)?
fn is_gosub(command: &Command) -> bool {
    matches!(command.op.opcode, 5..=9 | 16)
}

fn transfer(machine: &mut Machine, command: &Command, target: usize) -> Next {
    if is_gosub(command) {
        machine.gosub(target);
    } else {
        machine.goto(target);
    }
    Next::Jumped
}

/// Commands with pointer operands.
pub fn flow(machine: &mut Machine, command: &Command) -> Result<Next> {
    match &command.kind {
        CommandKind::Goto { target } => Ok(transfer(machine, command, *target)),
        CommandKind::GotoIf { condition, target } => {
            let value = match condition {
                Some(condition) => machine.eval_int(condition)?,
                None => 1,
            };
            // `_if` opcodes are 1 and 6 in module 1, `_unless` 2 and 7;
            // module 6 numbers its `goto_if` 0.
            let unless = matches!(command.op.opcode, 2 | 7);
            if (value != 0) != unless {
                Ok(transfer(machine, command, *target))
            } else {
                Ok(Next::Advance)
            }
        }
        CommandKind::GotoOn { value, targets } => {
            let value = machine.eval_int(value)?;
            match usize::try_from(value)
                .ok()
                .and_then(|index| targets.get(index))
            {
                Some(&target) => Ok(transfer(machine, command, target)),
                None => Ok(Next::Advance),
            }
        }
        CommandKind::GotoCase {
            value,
            cases,
            targets,
        } => {
            let value = machine.eval_int(value)?;
            for (case, &target) in cases.iter().zip(targets) {
                let matched = match case {
                    None => true,
                    Some(case) => machine.eval_int(case)? == value,
                };
                if matched {
                    return Ok(transfer(machine, command, target));
                }
            }
            bail!("reallive: goto_case without a matching or default case");
        }
        CommandKind::GosubWith { target } => {
            let (integers, strings) = with_arguments(machine, &command.params)?;
            machine.gosub(*target);
            machine.write_with_arguments(&integers, &strings);
            Ok(Next::Jumped)
        }
        CommandKind::Select(_) => crate::modules::sel::select(machine, command),
        CommandKind::Plain => dispatch(machine, command),
    }
}

/// Reads `*_with` arguments: `a\x00 int` or `a\x01 str` specials (plain
/// values are accepted too).
fn with_arguments(
    machine: &mut Machine,
    params: &[crate::bytecode::Param],
) -> Result<(Vec<i32>, Vec<String>)> {
    let mut integers = Vec::new();
    let mut strings = Vec::new();
    for param in params {
        match &param.value {
            Expr::Special { tag: 1, pieces } => {
                strings.push(match pieces.first() {
                    Some(piece) => machine.eval_str(piece)?,
                    None => String::new(),
                });
            }
            Expr::Special { pieces, .. } => {
                integers.push(match pieces.first() {
                    Some(piece) => machine.eval_int(piece)?,
                    None => 0,
                });
            }
            other if other.is_string() => strings.push(machine.eval_str(other)?),
            other => integers.push(machine.eval_int(other)?),
        }
    }
    Ok((integers, strings))
}

/// Plain-encoded commands of the flow modules.
pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let op = command.op;
    match (op.opcode, op.overload) {
        // ret / ret_with
        (10, _) => {
            machine.return_from_gosub()?;
            Ok(Next::Jumped)
        }
        (17, _) => {
            if op.module != 6 && op.overload == 0 {
                if let Some(value) = command.params.first() {
                    machine.store = machine.eval_int(&value.value)?;
                }
            }
            machine.return_from_gosub()?;
            Ok(Next::Jumped)
        }
        // jump
        (11, _) => {
            let scene = machine.int_param(command, 0)?;
            let entrypoint = machine.int_param_or(command, 1, 0)?;
            machine.jump(scene, entrypoint)?;
            Ok(Next::Jumped)
        }
        // farcall
        (12, _) => {
            let scene = machine.int_param(command, 0)?;
            let entrypoint = machine.int_param_or(command, 1, 0)?;
            machine.farcall(scene, entrypoint)?;
            Ok(Next::Jumped)
        }
        // rtl / rtl_with
        (13, _) => {
            machine.return_from_farcall()?;
            Ok(Next::Jumped)
        }
        (19, _) => {
            if op.module != 6 && op.overload == 0 {
                if let Some(value) = command.params.first() {
                    machine.store = machine.eval_int(&value.value)?;
                }
            }
            machine.return_from_farcall()?;
            Ok(Next::Jumped)
        }
        // farcall_with
        (18, _) => {
            let scene = machine.int_param(command, 0)?;
            let entrypoint = machine.int_param(command, 1)?;
            let (integers, strings) = with_arguments(machine, &command.params[2..])?;
            machine.farcall(scene, entrypoint)?;
            machine.write_with_arguments(&integers, &strings);
            Ok(Next::Jumped)
        }
        // RETURN_L_FLAG_SET(index, value)
        (100, _) => {
            let index = machine.int_param(command, 0)?;
            let value = machine.int_param(command, 1)?;
            machine.push_int_value_up(index, value)?;
            Ok(Next::Advance)
        }
        // RETURN_L_FLAG_SETS((index, value)...), RETURN_K_FLAG_SETS((index,
        // string)... or (index, value)...)
        (102 | 103, _) => {
            for at in 0..command.params.len() {
                let pieces = machine.complex_param(command, at)?;
                let [index, value, ..] = &pieces[..] else {
                    continue;
                };
                let index = machine.eval_int(index)?;
                if op.opcode == 103 && value.is_string() {
                    let value = machine.eval_str(value)?;
                    machine.push_string_value_up(index, value)?;
                } else if op.opcode == 103 {
                    let value = machine.eval_int(value)?;
                    machine.push_string_value_up(index, value.to_string())?;
                } else {
                    let value = machine.eval_int(value)?;
                    machine.push_int_value_up(index, value)?;
                }
            }
            Ok(Next::Advance)
        }
        // pushStringValueUp
        (101, _) => {
            let index = machine.int_param(command, 0)?;
            let value = machine.str_param(command, 1)?;
            machine.push_string_value_up(index, value)?;
            Ok(Next::Advance)
        }
        _ => machine.unimplemented(command),
    }
}
