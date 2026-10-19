//! RealLive expressions and command parameters.
//!
//! Bytecode grammar (all multi-byte integers little-endian):
//!
//! ```text
//! token      := '$' ( 0xff i32            integer constant
//!                   | 0xc8                the store register
//!                   | bank '[' expr ']' ) memory reference
//! term       := token | '\' 0x00 term | '\' 0x01 term (negate) | '(' boolean ')'
//! arith      := term ('\' op term)*       op 0x02-0x09 bind tighter than 0x00/0x01
//! condition  := arith ('\' 0x28-0x2d arith)*
//! boolean    := condition ('\' '<' condition)* ('\' '=' ...)*   && then ||
//! assignment := term '\' 0x14-0x1e expr
//! data       := ',' data | '\n' i16 data | string | complex | special | expr
//! complex    := '(' data* ')'
//! special    := 'a' tag ['a' tag2] ( data | '(' data* ')' )
//! ```
//!
//! Strings are raw bytes in the scenario encoding; `###PRINT(expr)` embeds
//! the string value of an expression.

use anyhow::{Result, anyhow, bail};

use crate::nls::Nls;

/// Memory bank bytes as they appear in bytecode.
pub mod bank {
    pub const INT_A: u8 = 0;
    pub const INT_B: u8 = 1;
    pub const INT_C: u8 = 2;
    pub const INT_D: u8 = 3;
    pub const INT_E: u8 = 4;
    pub const INT_F: u8 = 5;
    pub const INT_G: u8 = 6;
    /// `intL` (per call frame).
    pub const INT_L: u8 = 11;
    /// `intZ` (global).
    pub const INT_Z: u8 = 25;
    /// `strK` (per call frame).
    pub const STR_K: u8 = 0x0a;
    /// `strM` (global).
    pub const STR_M: u8 = 0x0c;
    /// `strS` (local).
    pub const STR_S: u8 = 0x12;

    pub fn is_string(bank: u8) -> bool {
        matches!(bank, STR_K | STR_M | STR_S)
    }
}

/// Binary operator codes.
pub mod op {
    pub const ADD: u8 = 0x00;
    pub const SUB: u8 = 0x01;
    pub const MUL: u8 = 0x02;
    pub const DIV: u8 = 0x03;
    pub const MOD: u8 = 0x04;
    pub const AND: u8 = 0x05;
    pub const OR: u8 = 0x06;
    pub const XOR: u8 = 0x07;
    pub const SHL: u8 = 0x08;
    pub const SHR: u8 = 0x09;
    /// `+=` ... `>>=` are 0x14-0x1d, `=` is 0x1e.
    pub const ASSIGN_BASE: u8 = 0x14;
    pub const ASSIGN: u8 = 0x1e;
    pub const EQ: u8 = 0x28;
    pub const NE: u8 = 0x29;
    pub const LE: u8 = 0x2a;
    pub const LT: u8 = 0x2b;
    pub const GE: u8 = 0x2c;
    pub const GT: u8 = 0x2d;
    pub const LAND: u8 = 0x3c;
    pub const LOR: u8 = 0x3d;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Int(i32),
    /// Raw string bytes (unquoted, `\"` unescaped).
    Str(Vec<u8>),
    /// `###PRINT(expr)`.
    Print(Box<Expr>),
    StoreRegister,
    Mem {
        bank: u8,
        index: Box<Expr>,
    },
    /// `-x` (op 0x01) or `+x` (op 0x00).
    Unary(u8, Box<Expr>),
    /// Arithmetic, comparison, logical and assignment operators.
    Binary(u8, Box<Expr>, Box<Expr>),
    /// `( data* )` — a tuple parameter.
    Complex(Vec<Expr>),
    /// `a<tag> ...` — a tagged parameter.
    Special {
        tag: i32,
        pieces: Vec<Expr>,
    },
}

impl Expr {
    pub fn is_string(&self) -> bool {
        match self {
            Expr::Str(_) | Expr::Print(_) => true,
            Expr::Mem { bank, .. } => bank::is_string(*bank),
            _ => false,
        }
    }

    pub fn is_memory_reference(&self) -> bool {
        matches!(self, Expr::Mem { .. } | Expr::StoreRegister)
    }

    /// The pieces of a complex or special parameter (a plain expression is
    /// a one-element tuple).
    pub fn pieces(&self) -> &[Expr] {
        match self {
            Expr::Complex(pieces) | Expr::Special { pieces, .. } => pieces,
            other => std::slice::from_ref(other),
        }
    }

    pub fn special_tag(&self) -> Option<i32> {
        match self {
            Expr::Special { tag, .. } => Some(*tag),
            _ => None,
        }
    }

    /// Folds constant arithmetic (as the compiler's output is often
    /// `$ 0xff n` for literals, this mostly matters for negative numbers).
    fn binary(operation: u8, left: Expr, right: Expr) -> Expr {
        if let (Expr::Int(a), Expr::Int(b)) = (&left, &right) {
            if !(op::ASSIGN_BASE..=op::ASSIGN).contains(&operation) {
                if let Some(value) = binary_op(operation, *a, *b) {
                    return Expr::Int(value);
                }
            }
        }
        Expr::Binary(operation, Box::new(left), Box::new(right))
    }
}

/// Applies a non-assigning binary operator (or the arithmetic part of an
/// assigning one). Division and modulo by zero leave the left operand,
/// shifts are masked, and arithmetic wraps like the original's 32-bit
/// integers.
pub fn binary_op(operation: u8, lhs: i32, rhs: i32) -> Option<i32> {
    let arithmetic = if (op::ASSIGN_BASE..op::ASSIGN).contains(&operation) {
        operation - op::ASSIGN_BASE
    } else {
        operation
    };
    Some(match arithmetic {
        op::ADD => lhs.wrapping_add(rhs),
        op::SUB => lhs.wrapping_sub(rhs),
        op::MUL => lhs.wrapping_mul(rhs),
        op::DIV => {
            if rhs == 0 {
                lhs
            } else {
                lhs.wrapping_div(rhs)
            }
        }
        op::MOD => {
            if rhs == 0 {
                lhs
            } else {
                lhs.wrapping_rem(rhs)
            }
        }
        op::AND => lhs & rhs,
        op::OR => lhs | rhs,
        op::XOR => lhs ^ rhs,
        op::SHL => lhs.wrapping_shl(rhs as u32),
        op::SHR => lhs.wrapping_shr(rhs as u32),
        op::EQ => i32::from(lhs == rhs),
        op::NE => i32::from(lhs != rhs),
        op::LE => i32::from(lhs <= rhs),
        op::LT => i32::from(lhs < rhs),
        op::GE => i32::from(lhs >= rhs),
        op::GT => i32::from(lhs > rhs),
        op::LAND => i32::from(lhs != 0 && rhs != 0),
        op::LOR => i32::from(lhs != 0 || rhs != 0),
        _ => return None,
    })
}

/// A cursor over bytecode for the recursive-descent parser.
#[derive(Debug, Clone, Copy)]
pub struct Parser<'a> {
    pub data: &'a [u8],
    pub pos: usize,
    pub nls: Nls,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8], nls: Nls) -> Self {
        Self { data, pos: 0, nls }
    }

    pub fn at(&self, offset: usize) -> u8 {
        self.data.get(self.pos + offset).copied().unwrap_or(0)
    }

    pub fn peek(&self) -> u8 {
        self.at(0)
    }

    pub fn rest(&self) -> &'a [u8] {
        self.data.get(self.pos..).unwrap_or(&[])
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn expect(&mut self, byte: u8, context: &str) -> Result<()> {
        if self.peek() != byte {
            bail!(
                "reallive: expected {:?} in {context}, found 0x{:02x} at 0x{:x}",
                byte as char,
                self.peek(),
                self.pos
            );
        }
        self.pos += 1;
        Ok(())
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let bytes = self
            .data
            .get(self.pos..self.pos + 4)
            .ok_or_else(|| anyhow!("reallive: truncated integer at 0x{:x}", self.pos))?;
        self.pos += 4;
        Ok(i32::from_le_bytes(bytes.try_into().expect("four bytes")))
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        let bytes = self
            .data
            .get(self.pos..self.pos + 2)
            .ok_or_else(|| anyhow!("reallive: truncated integer at 0x{:x}", self.pos))?;
        self.pos += 2;
        Ok(i16::from_le_bytes(bytes.try_into().expect("two bytes")))
    }

    fn token(&mut self) -> Result<Expr> {
        match self.peek() {
            0xff => {
                self.pos += 1;
                Ok(Expr::Int(self.read_i32()?))
            }
            0xc8 => {
                self.pos += 1;
                Ok(Expr::StoreRegister)
            }
            bank if self.at(1) == b'[' => {
                self.pos += 2;
                let index = self.expression()?;
                self.expect(b']', "memory reference")?;
                Ok(Expr::Mem {
                    bank,
                    index: Box::new(index),
                })
            }
            other => bail!(
                "reallive: unknown expression token 0x{other:02x} at 0x{:x}",
                self.pos
            ),
        }
    }

    fn term(&mut self) -> Result<Expr> {
        match (self.peek(), self.at(1)) {
            (b'$', _) => {
                self.pos += 1;
                self.token()
            }
            (b'\\', 0x00) => {
                self.pos += 2;
                self.term()
            }
            (b'\\', 0x01) => {
                self.pos += 2;
                let operand = self.term()?;
                Ok(match operand {
                    Expr::Int(value) => Expr::Int(value.wrapping_neg()),
                    other => Expr::Unary(0x01, Box::new(other)),
                })
            }
            (b'(', _) => {
                self.pos += 1;
                let inner = self.boolean()?;
                self.expect(b')', "parenthesised expression")?;
                Ok(inner)
            }
            (0, _) if self.eof() => bail!("reallive: unexpected end of expression"),
            (other, _) => bail!(
                "reallive: unknown expression term 0x{other:02x} at 0x{:x}",
                self.pos
            ),
        }
    }

    fn arithmetic(&mut self) -> Result<Expr> {
        let first = self.term()?;
        let mut left = self.high_precedence(first)?;
        while self.peek() == b'\\' && matches!(self.at(1), 0x00 | 0x01) {
            let operation = self.at(1);
            self.pos += 2;
            let term = self.term()?;
            let right = self.high_precedence(term)?;
            left = Expr::binary(operation, left, right);
        }
        Ok(left)
    }

    fn high_precedence(&mut self, mut left: Expr) -> Result<Expr> {
        while self.peek() == b'\\' && (0x02..=0x09).contains(&self.at(1)) {
            let operation = self.at(1);
            self.pos += 2;
            let right = self.term()?;
            left = Expr::binary(operation, left, right);
        }
        Ok(left)
    }

    fn condition(&mut self) -> Result<Expr> {
        let mut left = self.arithmetic()?;
        while self.peek() == b'\\' && (op::EQ..=op::GT).contains(&self.at(1)) {
            let operation = self.at(1);
            self.pos += 2;
            let right = self.arithmetic()?;
            left = Expr::binary(operation, left, right);
        }
        Ok(left)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut left = self.condition()?;
        while self.peek() == b'\\' && self.at(1) == b'<' {
            self.pos += 2;
            let right = self.condition()?;
            left = Expr::binary(op::LAND, left, right);
        }
        Ok(left)
    }

    fn boolean(&mut self) -> Result<Expr> {
        let mut left = self.and()?;
        while self.peek() == b'\\' && self.at(1) == b'=' {
            self.pos += 2;
            let right = self.and()?;
            left = Expr::binary(op::LOR, left, right);
        }
        Ok(left)
    }

    /// A full (boolean) expression.
    pub fn expression(&mut self) -> Result<Expr> {
        self.boolean()
    }

    /// An assignment statement (`$` element).
    pub fn assignment(&mut self) -> Result<Expr> {
        let target = self.term()?;
        if self.peek() != b'\\' {
            bail!(
                "reallive: expected an assignment operator at 0x{:x}",
                self.pos
            );
        }
        let operation = self.at(1);
        self.pos += 2;
        if !(op::ASSIGN_BASE..=op::ASSIGN).contains(&operation) {
            bail!("reallive: undefined assignment operator 0x{operation:02x}");
        }
        let value = self.expression()?;
        Ok(Expr::Binary(operation, Box::new(target), Box::new(value)))
    }

    fn is_string_start(&self) -> bool {
        let c = self.peek();
        self.nls.is_lead(c)
            || c.is_ascii_uppercase()
            || c.is_ascii_digit()
            || matches!(c, b' ' | b'?' | b'_' | b'"')
            || self.rest().starts_with(b"###PRINT(")
    }

    fn is_unescaped_quote(&self, start: usize, at: usize) -> bool {
        self.data.get(at) == Some(&b'"') && (at == start || self.data[at - 1] != b'\\')
    }

    /// Length of the string token at the cursor.
    pub fn string_len(&self) -> Result<usize> {
        let start = self.pos;
        let mut end = start;
        let mut quoted = false;
        loop {
            if end >= self.data.len() {
                break;
            }
            let c = self.data[end];
            if quoted {
                if self.is_unescaped_quote(start, end) {
                    end += 1;
                    break;
                }
            } else {
                quoted = self.is_unescaped_quote(start, end);
                if self.data[end..].starts_with(b"###PRINT(") {
                    let mut inner = Parser {
                        data: self.data,
                        pos: end + 9,
                        nls: self.nls,
                    };
                    inner.expression()?;
                    end = inner.pos + 1;
                    continue;
                }
                // `a` + a control byte is the tag of a following special
                // parameter, not part of an unquoted string.
                let tag = c == b'a' && self.data.get(end + 1).is_some_and(|&next| next < 0x20);
                let allowed = (!tag && (self.nls.is_lead(c) || c.is_ascii_alphanumeric()))
                    || matches!(c, b' ' | b'?' | b'_' | b'"' | b'\\');
                if !allowed {
                    break;
                }
            }
            end += self.nls.char_len(c);
        }
        Ok(end.min(self.data.len()) - start)
    }

    fn string(&mut self) -> Result<Expr> {
        let length = self.string_len()?;
        let raw = &self.data[self.pos..self.pos + length];
        self.pos += length;
        if let Some(inner) = raw.strip_prefix(b"###PRINT(") {
            let mut parser = Parser::new(inner, self.nls);
            let expression = parser.expression()?;
            return Ok(Expr::Print(Box::new(expression)));
        }
        let body = if raw.first() == Some(&b'"') && raw.len() >= 2 && raw.last() == Some(&b'"') {
            &raw[1..raw.len() - 1]
        } else {
            raw
        };
        Ok(Expr::Str(unescape_quotes(body)))
    }

    /// One command parameter in "data" form.
    pub fn data(&mut self) -> Result<Expr> {
        loop {
            match self.peek() {
                b',' => self.pos += 1,
                b'\n' => self.pos += 3,
                _ => break,
            }
        }
        if self.is_string_start() {
            return self.string();
        }
        match self.peek() {
            b'a' => {
                self.pos += 1;
                let mut tag = i32::from(self.peek());
                self.pos += 1;
                if self.peek() == b'a' {
                    self.pos += 1;
                    tag |= i32::from(self.peek()) << 16;
                    self.pos += 1;
                }
                let pieces = if self.peek() == b'(' {
                    self.pos += 1;
                    self.pieces_until_close()?
                } else {
                    vec![self.data()?]
                };
                Ok(Expr::Special { tag, pieces })
            }
            b'(' => {
                // A parenthesised expression unless it cannot be one (a
                // tuple of several values); parameters declared as tuples
                // are re-read with [`Parser::complex`].
                let checkpoint = self.pos;
                match self.expression() {
                    Ok(expression) => Ok(expression),
                    Err(_) => {
                        self.pos = checkpoint + 1;
                        Ok(Expr::Complex(self.pieces_until_close()?))
                    }
                }
            }
            _ => self.expression(),
        }
    }

    /// A parameter declared as complex: a parenthesised list is always a
    /// tuple.
    pub fn complex(&mut self) -> Result<Expr> {
        while self.peek() == b',' {
            self.pos += 1;
        }
        if self.peek() == b'(' {
            self.pos += 1;
            return Ok(Expr::Complex(self.pieces_until_close()?));
        }
        self.data()
    }

    fn pieces_until_close(&mut self) -> Result<Vec<Expr>> {
        let mut pieces = Vec::new();
        loop {
            while matches!(self.peek(), b',' | b'\n') {
                self.pos += if self.peek() == b',' { 1 } else { 3 };
            }
            if self.peek() == b')' {
                self.pos += 1;
                return Ok(pieces);
            }
            if self.eof() {
                bail!("reallive: unterminated parameter list");
            }
            pieces.push(self.data()?);
        }
    }

    /// Byte length of one data parameter at the cursor (the extent the
    /// original tokenizer assigns, used to split a parameter list).
    pub fn data_len(&self) -> Result<usize> {
        let mut probe = *self;
        probe.skip_data()?;
        Ok(probe.pos - self.pos)
    }

    fn skip_data(&mut self) -> Result<()> {
        match self.peek() {
            b',' => {
                self.pos += 1;
                return self.skip_data();
            }
            b'\n' => {
                self.pos += 3;
                return self.skip_data();
            }
            _ => {}
        }
        if self.is_string_start() {
            self.pos += self.string_len()?;
            return Ok(());
        }
        if matches!(self.peek(), b'a' | b'(') {
            if self.peek() == b'a' {
                self.pos += 2;
                if self.peek() == b'a' {
                    self.pos += 2;
                }
                if self.peek() != b'(' {
                    return self.skip_data();
                }
            }
            self.pos += 1;
            loop {
                while matches!(self.peek(), b',' | b'\n') {
                    self.pos += if self.peek() == b',' { 1 } else { 3 };
                }
                if self.peek() == b')' {
                    break;
                }
                if self.eof() {
                    bail!("reallive: unterminated parameter list");
                }
                self.skip_data()?;
            }
            self.pos += 1;
            if self.peek() == b'\\' {
                self.skip_expression_tail()?;
            }
            return Ok(());
        }
        self.expression().map(|_| ())
    }

    /// After a parenthesised group that turned out to start an expression.
    fn skip_expression_tail(&mut self) -> Result<()> {
        while self.peek() == b'\\' {
            self.pos += 2;
            self.term()?;
        }
        Ok(())
    }
}

fn unescape_quotes(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && bytes.get(i + 1) == Some(&b'"') {
            out.push(b'"');
            i += 2;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

/// Encoders used by tests and tools that assemble bytecode.
pub mod encode {
    use super::bank;

    pub fn int(value: i32) -> Vec<u8> {
        let mut out = vec![b'$', 0xff];
        out.extend_from_slice(&value.to_le_bytes());
        out
    }

    pub fn mem(bank: u8, index: i32) -> Vec<u8> {
        let mut out = vec![b'$', bank, b'['];
        out.extend(int(index));
        out.push(b']');
        out
    }

    pub fn int_a(index: i32) -> Vec<u8> {
        mem(bank::INT_A, index)
    }

    pub fn binary(left: &[u8], operation: u8, right: &[u8]) -> Vec<u8> {
        let mut out = left.to_vec();
        out.push(b'\\');
        out.push(operation);
        out.extend_from_slice(right);
        out
    }

    pub fn quoted(text: &[u8]) -> Vec<u8> {
        let mut out = vec![b'"'];
        out.extend_from_slice(text);
        out.push(b'"');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::encode::*;
    use super::*;

    fn parse(bytes: &[u8]) -> Expr {
        Parser::new(bytes, Nls::Sjis).expression().unwrap()
    }

    #[test]
    fn precedence_and_folding() {
        // 1 + 2 * 3
        let bytes = binary(&binary(&int(1), op::ADD, &int(2)), op::MUL, &int(3));
        assert_eq!(parse(&bytes), Expr::Int(7));
        // intA[0] == 3 && 1
        let bytes = binary(&binary(&int_a(0), op::EQ, &int(3)), b'<', &int(1));
        match parse(&bytes) {
            Expr::Binary(op::LAND, left, right) => {
                assert!(matches!(*left, Expr::Binary(op::EQ, _, _)));
                assert_eq!(*right, Expr::Int(1));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn negation_and_nested_references() {
        let mut bytes = vec![b'\\', 0x01];
        bytes.extend(int(5));
        assert_eq!(parse(&bytes), Expr::Int(-5));
        // intB[intA[1]]
        let mut nested = vec![b'$', bank::INT_B, b'['];
        nested.extend(int_a(1));
        nested.push(b']');
        match parse(&nested) {
            Expr::Mem { bank: 1, index } => assert!(matches!(*index, Expr::Mem { bank: 0, .. })),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn data_strings_tuples_and_specials() {
        let mut bytes = b"\"a\\\"b\",".to_vec();
        bytes.extend(b"(");
        bytes.extend(int(1));
        bytes.extend(int(2));
        bytes.extend(b")");
        bytes.extend(b"a\x01");
        bytes.extend(int(9));
        let mut parser = Parser::new(&bytes, Nls::Sjis);
        assert_eq!(parser.data().unwrap(), Expr::Str(b"a\"b".to_vec()));
        assert_eq!(
            parser.data().unwrap(),
            Expr::Complex(vec![Expr::Int(1), Expr::Int(2)])
        );
        assert_eq!(
            parser.data().unwrap(),
            Expr::Special {
                tag: 1,
                pieces: vec![Expr::Int(9)]
            }
        );
        assert!(parser.eof());
    }

    #[test]
    fn print_strings_and_multibyte_text() {
        let mut bytes = b"###PRINT(".to_vec();
        bytes.extend(mem(bank::STR_S, 3));
        bytes.push(b')');
        let mut parser = Parser::new(&bytes, Nls::Sjis);
        assert!(matches!(parser.data().unwrap(), Expr::Print(_)));
        // GBK text whose trail byte is '@' must stay one string.
        let text = [0x81, 0x40, 0x81, 0x40];
        let mut parser = Parser::new(&text, Nls::Gbk);
        assert_eq!(parser.data().unwrap(), Expr::Str(text.to_vec()));
    }

    #[test]
    fn assignments() {
        let bytes = binary(&int_a(2), op::ASSIGN, &int(4));
        let mut parser = Parser::new(&bytes, Nls::Sjis);
        assert!(matches!(
            parser.assignment().unwrap(),
            Expr::Binary(op::ASSIGN, _, _)
        ));
    }
}
