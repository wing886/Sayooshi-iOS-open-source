//! UK2 operand, expression, and condition encoding.
//!
//! This module follows the original `UK2.EXE` routines `sub_14FFA`,
//! `sub_15222`, and `sub_1542C` rather than inferring value shapes from one
//! game's scripts.

use anyhow::{Result, bail};

use crate::mes::MesReader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// Class 0, indices 0..30: DS:3B62 + index * 2.
    SystemWord(u8),
    /// Class 0, index 31: inline little-endian immediate stored in scratch.
    Immediate(u16),
    /// Class 1: fixed 0x3e-byte string slot at DS:5312 + index * 0x3e.
    FixedString(u8),
    /// Class 2: word in the current table at base + index * 2.
    LocalWord(u8),
    /// Class 3: inline string assembled until a zero byte.
    InlineString(Vec<InlineStringPart>),
    /// Class 4: byte table selected by the low five bits and indexed by a
    /// nested expression.  The reference executable has concrete pointers for
    /// selectors 5 and 8.
    IndexedByte {
        selector: u8,
        index: Box<Expression>,
    },
    /// Class 5: one additional selector byte indexes the engine's far-pointer
    /// table.  The low five bits choose byte/word/string interpretation.
    Indirect { kind: IndirectKind, slot: u8 },
    /// Class 6: 0x7d2-byte string/buffer slot at DS:3B9C + index * 0x7d2.
    LargeString(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineStringPart {
    Literal(Vec<u8>),
    /// Inline marker byte 0x05 followed by a one-based slot number.
    Table5Slot(u8),
    /// Inline marker byte 0x06 followed by a one-based slot number.
    Table6Slot(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndirectKind {
    Byte,
    Word,
    String,
    Raw(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Set,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionTerm {
    pub op: ExpressionOp,
    pub operand: Operand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub terms: Vec<ExpressionTerm>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Less,
    Greater,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolJoin {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionClause {
    pub left: Expression,
    pub compare: CompareOp,
    pub right: Expression,
    pub join_after: Option<BoolJoin>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub clauses: Vec<ConditionClause>,
}

pub(crate) fn parse_operand(reader: &mut MesReader<'_>) -> Result<Operand> {
    let encoded = reader.byte()?;
    let class = encoded >> 5;
    let index = encoded & 0x1f;
    match class {
        0 => {
            if index == 0x1f {
                Ok(Operand::Immediate(reader.u16()?))
            } else {
                Ok(Operand::SystemWord(index))
            }
        }
        1 => Ok(Operand::FixedString(index)),
        2 => Ok(Operand::LocalWord(index)),
        3 => parse_inline_string(reader),
        4 => {
            let index_expression = parse_expression(reader)?.ok_or_else(|| {
                anyhow::anyhow!("uk2: class-4 operand contains a null nested expression")
            })?;
            Ok(Operand::IndexedByte {
                selector: index,
                index: Box::new(index_expression),
            })
        }
        5 => {
            let kind = match index {
                0 => IndirectKind::Byte,
                1 => IndirectKind::Word,
                2 => IndirectKind::String,
                other => IndirectKind::Raw(other),
            };
            Ok(Operand::Indirect {
                kind,
                slot: reader.byte()?,
            })
        }
        6 => Ok(Operand::LargeString(index)),
        _ => bail!(
            "uk2: invalid operand class 7 at {:#x}",
            reader.position() - 1
        ),
    }
}

fn parse_inline_string(reader: &mut MesReader<'_>) -> Result<Operand> {
    let mut parts = Vec::new();
    let mut literal = Vec::new();
    loop {
        let byte = reader.byte()?;
        match byte {
            0 => {
                push_literal(&mut parts, &mut literal);
                return Ok(Operand::InlineString(parts));
            }
            5 | 6 => {
                push_literal(&mut parts, &mut literal);
                let slot = reader.byte()?;
                parts.push(if byte == 5 {
                    InlineStringPart::Table5Slot(slot)
                } else {
                    InlineStringPart::Table6Slot(slot)
                });
            }
            other => literal.push(other),
        }
    }
}

fn push_literal(parts: &mut Vec<InlineStringPart>, literal: &mut Vec<u8>) {
    if !literal.is_empty() {
        parts.push(InlineStringPart::Literal(std::mem::take(literal)));
    }
}

pub(crate) fn parse_expression(reader: &mut MesReader<'_>) -> Result<Option<Expression>> {
    let mut terms = Vec::new();
    loop {
        let control = reader.byte()?;
        let last = control & 0x80 != 0;
        let op = control & 0x7f;
        if op == 0 {
            return Ok(None);
        }
        let op = match op {
            1 => ExpressionOp::Add,
            2 => ExpressionOp::Subtract,
            3 => ExpressionOp::Multiply,
            4 => ExpressionOp::Divide,
            5 => ExpressionOp::Modulo,
            7 => ExpressionOp::Set,
            6 => bail!("uk2: expression operation 6 is rejected by the original VM"),
            other => bail!("uk2: invalid expression operation {other}"),
        };
        terms.push(ExpressionTerm {
            op,
            operand: parse_operand(reader)?,
        });
        if last {
            return Ok(Some(Expression { terms }));
        }
    }
}

pub(crate) fn parse_required_expression(reader: &mut MesReader<'_>) -> Result<Expression> {
    parse_expression(reader)?
        .ok_or_else(|| anyhow::anyhow!("uk2: expected expression, found null expression marker"))
}

pub(crate) fn parse_expression_list(
    reader: &mut MesReader<'_>,
    max: usize,
) -> Result<Vec<Expression>> {
    let mut expressions = Vec::new();
    while expressions.len() < max {
        let Some(expression) = parse_expression(reader)? else {
            break;
        };
        expressions.push(expression);
    }
    Ok(expressions)
}

pub(crate) fn parse_condition(reader: &mut MesReader<'_>) -> Result<Condition> {
    let mut clauses = Vec::new();
    loop {
        let left = parse_required_expression(reader)?;
        let compare = match reader.byte()? {
            1 => CompareOp::Less,
            2 => CompareOp::Greater,
            3 => CompareOp::Equal,
            4 => CompareOp::NotEqual,
            other => bail!("uk2: invalid comparison operation {other}"),
        };
        let right = parse_required_expression(reader)?;
        let join_after = match reader.byte()? {
            0 => None,
            1 => Some(BoolJoin::And),
            2 => Some(BoolJoin::Or),
            other => bail!("uk2: invalid condition connector {other}"),
        };
        let done = join_after.is_none();
        clauses.push(ConditionClause {
            left,
            compare,
            right,
            join_after,
        });
        if done {
            break;
        }
    }
    Ok(Condition { clauses })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mes::{MES_MAGIC, MesProgram};

    fn reader(payload: &[u8]) -> MesReader<'static> {
        let mut bytes = MES_MAGIC.to_vec();
        bytes.extend_from_slice(payload);
        let leaked = Box::leak(Box::new(MesProgram::from_bytes(bytes).unwrap()));
        leaked.reader_at(MES_MAGIC.len()).unwrap()
    }

    #[test]
    fn parses_reference_immediate_expression() {
        let mut reader = reader(&[0x87, 0x1f, 0x34, 0x12]);
        let expression = parse_required_expression(&mut reader).unwrap();
        assert_eq!(
            expression.terms[0],
            ExpressionTerm {
                op: ExpressionOp::Set,
                operand: Operand::Immediate(0x1234),
            }
        );
    }

    #[test]
    fn parses_inline_string_slots() {
        let mut reader = reader(&[0x60, b'A', 5, 2, 6, 3, b'B', 0]);
        assert_eq!(
            parse_operand(&mut reader).unwrap(),
            Operand::InlineString(vec![
                InlineStringPart::Literal(vec![b'A']),
                InlineStringPart::Table5Slot(2),
                InlineStringPart::Table6Slot(3),
                InlineStringPart::Literal(vec![b'B']),
            ])
        );
    }
}
