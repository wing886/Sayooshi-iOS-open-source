//! Structural UK2 bytecode decoder and control-flow traversal.

use std::collections::{BTreeMap, VecDeque};

use anyhow::{Context, Result, bail};

use crate::mes::{MES_CODE_OFFSET, MesProgram};
use crate::opcode::{SingleOpcode, TwoOpcode};
use crate::value::{
    Condition, Expression, Operand, parse_condition, parse_expression, parse_expression_list,
    parse_operand, parse_required_expression,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceArgument {
    pub resource: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionArg {
    Direct(Operand),
    Expression(Expression),
    ExpressionList(Vec<Expression>),
    Condition(Condition),
    Offset(u16),
    ResourceList(Vec<ResourceArgument>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionKind {
    End,
    Assign {
        destination: Operand,
        value: Expression,
    },
    BlockCall {
        target: u16,
    },
    Decrement(Operand),
    Increment(Operand),
    Return {
        raw: u8,
        status: i16,
    },
    Command {
        opcode: TwoOpcode,
        arguments: Vec<InstructionArg>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub offset: usize,
    pub end_offset: usize,
    pub kind: InstructionKind,
}

impl Instruction {
    pub fn len(&self) -> usize {
        self.end_offset - self.offset
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn decode_instruction(program: &MesProgram, offset: usize) -> Result<Instruction> {
    if offset < MES_CODE_OFFSET {
        bail!("uk2: instruction offset {offset:#x} lies inside the MES header");
    }
    let mut reader = program.reader_at(offset)?;
    let first = reader.byte()?;
    let kind = if let Some(single) = SingleOpcode::from_byte(first) {
        match single {
            SingleOpcode::End => InstructionKind::End,
            SingleOpcode::Assign => InstructionKind::Assign {
                destination: parse_operand(&mut reader)?,
                value: parse_required_expression(&mut reader)?,
            },
            SingleOpcode::BlockCall => InstructionKind::BlockCall {
                target: reader.u16()?,
            },
            SingleOpcode::Decrement => InstructionKind::Decrement(parse_operand(&mut reader)?),
            SingleOpcode::Increment => InstructionKind::Increment(parse_operand(&mut reader)?),
            SingleOpcode::Return => {
                let raw = reader.byte()?;
                InstructionKind::Return {
                    raw,
                    status: i16::from(raw) - i16::from(b'0'),
                }
            }
        }
    } else {
        let second = reader.byte()?;
        let opcode = TwoOpcode::from_pair(first, second).ok_or_else(|| {
            anyhow::anyhow!("uk2: unknown opcode bytes {first:#04x} {second:#04x} at {offset:#x}")
        })?;
        let arguments = parse_signature(opcode, &mut reader)
            .with_context(|| format!("uk2: failed to decode {opcode} at {offset:#x}"))?;
        InstructionKind::Command { opcode, arguments }
    };
    Ok(Instruction {
        offset,
        end_offset: reader.position(),
        kind,
    })
}

fn parse_signature(
    opcode: TwoOpcode,
    reader: &mut crate::mes::MesReader<'_>,
) -> Result<Vec<InstructionArg>> {
    let mut arguments = Vec::new();
    for token in opcode.signature().split_whitespace() {
        match token {
            "D" => arguments.push(InstructionArg::Direct(parse_operand(reader)?)),
            "E" => arguments.push(InstructionArg::Expression(parse_required_expression(
                reader,
            )?)),
            "C" => arguments.push(InstructionArg::Condition(parse_condition(reader)?)),
            "O16" => arguments.push(InstructionArg::Offset(reader.u16()?)),
            "X*" => {
                let mut expressions = Vec::new();
                while let Some(expression) = parse_expression(reader)? {
                    expressions.push(expression);
                }
                arguments.push(InstructionArg::ExpressionList(expressions));
            }
            "RESOURCE_LIST" => {
                let mut resources = Vec::new();
                loop {
                    let Some(resource) = parse_expression(reader)? else {
                        break;
                    };
                    resources.push(ResourceArgument {
                        resource,
                        arguments: parse_expression_list(reader, 2)?,
                    });
                }
                arguments.push(InstructionArg::ResourceList(resources));
            }
            token if token.starts_with("X<=") => {
                let max = token[3..]
                    .parse::<usize>()
                    .with_context(|| format!("uk2: invalid opcode signature token {token}"))?;
                arguments.push(InstructionArg::ExpressionList(parse_expression_list(
                    reader, max,
                )?));
            }
            other => bail!("uk2: unsupported opcode signature token {other}"),
        }
    }
    Ok(arguments)
}

/// Decodes instructions reachable from the MES entry point using the control
/// flow encoded by UK2's own recursive interpreter.  Data that follows an
/// unreachable block terminator is not misidentified as bytecode.
pub fn disassemble_reachable(program: &MesProgram) -> Result<BTreeMap<usize, Instruction>> {
    let mut pending = VecDeque::from([MES_CODE_OFFSET]);
    let mut instructions = BTreeMap::new();

    while let Some(offset) = pending.pop_front() {
        if instructions.contains_key(&offset) {
            continue;
        }
        if offset < MES_CODE_OFFSET || offset >= program.len() {
            bail!(
                "uk2: control-flow edge points outside MES: {offset:#x} / {:#x}",
                program.len()
            );
        }
        let instruction = decode_instruction(program, offset)?;
        let fallthrough = instruction.end_offset;
        let mut push_fallthrough = true;

        match &instruction.kind {
            InstructionKind::End | InstructionKind::Return { .. } => {
                push_fallthrough = false;
            }
            InstructionKind::BlockCall { target } => {
                push_target(program, &mut pending, usize::from(*target))?;
            }
            InstructionKind::Command { opcode, arguments } => match opcode {
                TwoOpcode::J0 => {
                    push_target(program, &mut pending, argument_offset(arguments)?)?;
                    push_fallthrough = false;
                }
                TwoOpcode::J1 => {
                    push_target(program, &mut pending, argument_offset(arguments)?)?;
                }
                TwoOpcode::J3 => {
                    // The reference handler returns interpreter status 5.
                    push_fallthrough = false;
                }
                TwoOpcode::L0
                | TwoOpcode::L1
                | TwoOpcode::L2
                | TwoOpcode::L3
                | TwoOpcode::L4
                | TwoOpcode::L5 => {
                    push_target(program, &mut pending, argument_offset(arguments)?)?;
                }
                _ => {}
            },
            _ => {}
        }
        if push_fallthrough && fallthrough < program.len() {
            pending.push_back(fallthrough);
        }
        instructions.insert(offset, instruction);
    }
    Ok(instructions)
}

fn argument_offset(arguments: &[InstructionArg]) -> Result<usize> {
    arguments
        .iter()
        .rev()
        .find_map(|argument| match argument {
            InstructionArg::Offset(offset) => Some(usize::from(*offset)),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("uk2: control-flow instruction has no O16 argument"))
}

fn push_target(program: &MesProgram, pending: &mut VecDeque<usize>, target: usize) -> Result<()> {
    if target < MES_CODE_OFFSET || target >= program.len() {
        bail!(
            "uk2: branch target {target:#x} is outside MES {:#x}",
            program.len()
        );
    }
    pending.push_back(target);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mes::MES_MAGIC;
    use crate::value::{ExpressionOp, ExpressionTerm};

    fn program(code: &[u8]) -> MesProgram {
        let mut bytes = MES_MAGIC.to_vec();
        bytes.extend_from_slice(code);
        MesProgram::from_bytes(bytes).unwrap()
    }

    #[test]
    fn decodes_j2_filename_operand() {
        let mes = program(&[b'J', b'2', 0x60, b'a', b'.', b'm', b'e', b's', b'1', 0, 0]);
        let instruction = decode_instruction(&mes, MES_CODE_OFFSET).unwrap();
        assert_eq!(instruction.end_offset, MES_CODE_OFFSET + 10);
        assert!(matches!(
            instruction.kind,
            InstructionKind::Command {
                opcode: TwoOpcode::J2,
                ..
            }
        ));
    }

    #[test]
    fn decodes_w5_filename_and_mode() {
        let mes = program(&[
            b'W', b'5', 0x60, b'x', b'.', b'p', b'd', b't', b'1', 0, 0x87, 0x1f, 3, 0, 0,
        ]);
        let instruction = decode_instruction(&mes, MES_CODE_OFFSET).unwrap();
        let InstructionKind::Command { opcode, arguments } = instruction.kind else {
            panic!("not a command")
        };
        assert_eq!(opcode, TwoOpcode::W5);
        assert_eq!(arguments.len(), 2);
        let InstructionArg::Expression(expression) = &arguments[1] else {
            panic!("mode is not an expression")
        };
        assert_eq!(
            expression.terms[0],
            ExpressionTerm {
                op: ExpressionOp::Set,
                operand: Operand::Immediate(3),
            }
        );
    }

    #[test]
    fn w8_consumes_helper_parsed_payload() {
        let mes = program(&[
            b'W', b'8', 0x40, 0x87, 0x1f, 11, 0, 0x87, 0x1f, 5, 0, 0x87, 0x1f, 1, 0, 0x80, 0,
        ]);
        let instruction = decode_instruction(&mes, MES_CODE_OFFSET).unwrap();
        assert_eq!(instruction.end_offset, mes.len() - 1);
    }
}
