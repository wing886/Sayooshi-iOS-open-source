//! Decoded RealLive bytecode.
//!
//! A scenario's code is a flat sequence of elements:
//!
//! | first byte | element |
//! | --- | --- |
//! | `0x00`, `,` | separator |
//! | `\n` i16 | source line number |
//! | `@` / `!` i16 | kidoku marker, or an entrypoint when the kidoku table entry is ≥ 1 000 000 |
//! | `$` | expression statement (usually an assignment) |
//! | `#` | command: `# modtype module opcode:u16 argc:u16 overload` + parameters |
//! | anything else | text output |
//!
//! Jumps address byte offsets in the decompressed code; they are resolved
//! to element indices when a scenario is loaded.

use std::collections::HashMap;

use anyhow::{Context, Result, anyhow, bail};

use crate::expr::{Expr, Parser};
use crate::nls::Nls;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Opcode {
    pub modtype: u8,
    pub module: u8,
    pub opcode: u16,
    pub overload: u8,
}

impl Opcode {
    pub const fn new(modtype: u8, module: u8, opcode: u16, overload: u8) -> Self {
        Self {
            modtype,
            module,
            opcode,
            overload,
        }
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "op<{}:{:03}:{:05}, {}>",
            self.modtype, self.module, self.opcode, self.overload
        )
    }
}

/// One command parameter: the parsed value and the raw bytes it came from
/// (parameters declared as tuples are re-read from `raw`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub raw: Vec<u8>,
    pub value: Expr,
}

impl Param {
    pub fn complex(&self, nls: Nls) -> Result<Expr> {
        Parser::new(&self.raw, nls).complex()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectCondition {
    pub condition: Option<Expr>,
    /// `'0'` colour, `'1'` title, `'2'` hide, `'3'` blank, `'4'` cursor.
    pub effect: u8,
    pub argument: Option<Expr>,
}

pub mod select_effect {
    pub const COLOUR: u8 = b'0';
    pub const TITLE: u8 = b'1';
    pub const HIDE: u8 = b'2';
    pub const BLANK: u8 = b'3';
    pub const CURSOR: u8 = b'4';
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub conditions: Vec<SelectCondition>,
    pub text: Expr,
    pub line: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select {
    pub window: Option<Expr>,
    pub options: Vec<SelectOption>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandKind {
    Plain,
    Goto {
        target: usize,
    },
    GotoIf {
        condition: Option<Expr>,
        target: usize,
    },
    GotoOn {
        value: Expr,
        targets: Vec<usize>,
    },
    GotoCase {
        value: Expr,
        cases: Vec<Option<Expr>>,
        targets: Vec<usize>,
    },
    GosubWith {
        target: usize,
    },
    Select(Select),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub op: Opcode,
    pub argc: u16,
    pub params: Vec<Param>,
    pub kind: CommandKind,
}

impl Command {
    /// Jump targets (element indices after [`Script::resolve`]).
    pub fn targets(&self) -> Vec<usize> {
        match &self.kind {
            CommandKind::Goto { target }
            | CommandKind::GotoIf { target, .. }
            | CommandKind::GosubWith { target } => vec![*target],
            CommandKind::GotoOn { targets, .. } | CommandKind::GotoCase { targets, .. } => {
                targets.clone()
            }
            _ => Vec::new(),
        }
    }

    fn map_targets(&mut self, map: impl Fn(usize) -> Result<usize>) -> Result<()> {
        match &mut self.kind {
            CommandKind::Goto { target }
            | CommandKind::GotoIf { target, .. }
            | CommandKind::GosubWith { target } => *target = map(*target)?,
            CommandKind::GotoOn { targets, .. } | CommandKind::GotoCase { targets, .. } => {
                for target in targets {
                    *target = map(*target)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    Comma,
    Line(i32),
    Kidoku(i32),
    Entrypoint(i32),
    /// Raw text-output bytes (quotes and escapes still in place).
    Textout(Vec<u8>),
    Expression(Expr),
    Command(Command),
}

impl Element {
    /// Text of a text-output element with quoting removed.
    pub fn text(raw: &[u8], nls: Nls) -> Vec<u8> {
        let mut out = Vec::with_capacity(raw.len());
        let mut quoted = false;
        let mut i = 0;
        while i < raw.len() {
            let c = raw[i];
            if c == b'"' {
                quoted = !quoted;
                i += 1;
            } else if !quoted && c == b',' {
                // Unquoted commas separate text segments.
                i += 1;
            } else if quoted && c == b'\\' {
                if raw.get(i + 1) == Some(&b'"') {
                    out.push(b'"');
                    i += 2;
                } else {
                    out.push(b'\\');
                    i += 1;
                }
            } else {
                let length = nls.char_len(c).min(raw.len() - i);
                out.extend_from_slice(&raw[i..i + length]);
                i += length;
            }
        }
        out
    }
}

/// A scenario's decoded code.
#[derive(Debug, Clone, Default)]
pub struct Script {
    pub elements: Vec<Element>,
    /// Byte offset of every element in the decompressed code.
    pub offsets: Vec<u32>,
    /// Entrypoint number → element index.
    pub entrypoints: HashMap<i32, usize>,
}

impl Script {
    /// Decodes `code` using the scenario's kidoku table.
    pub fn parse(code: &[u8], kidoku_table: &[i32], nls: Nls) -> Result<Self> {
        let mut script = Script::default();
        let mut by_offset: HashMap<usize, usize> = HashMap::new();
        let mut reader = Reader {
            data: code,
            pos: 0,
            nls,
            kidoku_table,
            entrypoint_marker: b'@',
        };
        while reader.pos < code.len() {
            let start = reader.pos;
            let element = reader
                .element()
                .with_context(|| format!("while decoding bytecode at 0x{start:x}"))?;
            if let Element::Entrypoint(entrypoint) = element {
                script.entrypoints.insert(entrypoint, script.elements.len());
            }
            by_offset.insert(start, script.elements.len());
            script.offsets.push(start as u32);
            script.elements.push(element);
            if reader.pos == start {
                reader.pos += 1;
            }
        }
        let count = script.elements.len();
        for element in &mut script.elements {
            if let Element::Command(command) = element {
                command.map_targets(|offset| {
                    // A jump to the very end of the code halts.
                    if offset == code.len() {
                        return Ok(count);
                    }
                    by_offset.get(&offset).copied().ok_or_else(|| {
                        anyhow!("reallive: jump to 0x{offset:x} is not an element boundary")
                    })
                })?;
            }
        }
        Ok(script)
    }

    pub fn entrypoint(&self, number: i32) -> Option<usize> {
        self.entrypoints.get(&number).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flow {
    Goto,
    GotoIf,
    GotoOn,
    GotoCase,
    GosubWith,
    Select,
}

/// Commands with a non-standard encoding (modtype 0). Modules 5 and 6 are
/// the flow-control modules of later engine versions; their numbering is
/// shifted relative to module 1.
fn flow_kind(module: u8, opcode: u16) -> Option<Flow> {
    Some(match (module, opcode) {
        (1, 0 | 5) | (5, 1 | 5) | (6, 1 | 5) => Flow::Goto,
        (1, 1 | 2 | 6 | 7) | (5, 2 | 6 | 7) | (6, 0 | 2 | 6 | 7) => Flow::GotoIf,
        (1 | 5 | 6, 3 | 8) => Flow::GotoOn,
        (1 | 5 | 6, 4 | 9) => Flow::GotoCase,
        (1 | 6, 16) => Flow::GosubWith,
        (2, 0..=3 | 16) => Flow::Select,
        _ => return None,
    })
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
    nls: Nls,
    kidoku_table: &'a [i32],
    entrypoint_marker: u8,
}

impl Reader<'_> {
    fn at(&self, offset: usize) -> u8 {
        self.data.get(self.pos + offset).copied().unwrap_or(0)
    }

    fn parser(&self) -> Parser<'_> {
        Parser {
            data: self.data,
            pos: self.pos,
            nls: self.nls,
        }
    }

    fn read_i16(&mut self) -> Result<i32> {
        let mut parser = self.parser();
        let value = parser.read_i16()?;
        self.pos = parser.pos;
        Ok(i32::from(value))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let mut parser = self.parser();
        let value = parser.read_i32()?;
        self.pos = parser.pos;
        Ok(value)
    }

    fn element(&mut self) -> Result<Element> {
        match self.at(0) {
            0 | b',' => {
                self.pos += 1;
                Ok(Element::Comma)
            }
            b'\n' => {
                self.pos += 1;
                Ok(Element::Line(self.read_i16()?))
            }
            marker @ (b'@' | b'!') => {
                if marker == b'!' {
                    self.entrypoint_marker = b'!';
                }
                self.pos += 1;
                let index = self.read_i16()?;
                let value = self.kidoku_table.get(index as usize).copied().unwrap_or(0);
                Ok(if value >= 1_000_000 {
                    Element::Entrypoint(value - 1_000_000)
                } else {
                    Element::Kidoku(index)
                })
            }
            b'$' => {
                let mut parser = self.parser();
                let expression = parser.assignment()?;
                self.pos = parser.pos;
                Ok(Element::Expression(expression))
            }
            b'#' => self.command(),
            _ => Ok(Element::Textout(self.textout())),
        }
    }

    fn textout(&mut self) -> Vec<u8> {
        let start = self.pos;
        let mut end = start;
        let mut quoted = false;
        while end < self.data.len() {
            let mut c = self.data[end];
            if quoted {
                quoted = c != b'"';
                if c == b'\\' && self.data.get(end + 1) == Some(&b'"') {
                    end += 1;
                }
            } else {
                if c == b',' {
                    end += 1;
                    c = self.data.get(end).copied().unwrap_or(0);
                }
                quoted = c == b'"';
                if matches!(c, 0 | b'#' | b'$' | b'\n' | b'@') || c == self.entrypoint_marker {
                    break;
                }
            }
            end += self.nls.char_len(self.data.get(end).copied().unwrap_or(0));
        }
        let end = end.min(self.data.len());
        self.pos = end;
        self.data[start..end].to_vec()
    }

    fn command(&mut self) -> Result<Element> {
        let header = self
            .data
            .get(self.pos..self.pos + 8)
            .ok_or_else(|| anyhow!("reallive: truncated command"))?;
        let op = Opcode {
            modtype: header[1],
            module: header[2],
            opcode: u16::from_le_bytes([header[3], header[4]]),
            overload: header[7],
        };
        let argc = u16::from_le_bytes([header[5], header[6]]);
        self.pos += 8;
        let flow = if op.modtype == 0 {
            flow_kind(op.module, op.opcode)
        } else {
            None
        };
        let command = if flow == Some(Flow::Goto) {
            let target = self.read_i32()? as usize;
            Command {
                op,
                argc,
                params: Vec::new(),
                kind: CommandKind::Goto { target },
            }
        } else if flow == Some(Flow::GotoIf) {
            let condition = if self.at(0) == b'(' {
                self.pos += 1;
                let mut parser = self.parser();
                let condition = parser.expression()?;
                self.pos = parser.pos;
                if self.at(0) != b')' {
                    bail!("reallive: expected ')' after a goto condition");
                }
                self.pos += 1;
                Some(condition)
            } else {
                None
            };
            let target = self.read_i32()? as usize;
            Command {
                op,
                argc,
                params: Vec::new(),
                kind: CommandKind::GotoIf { condition, target },
            }
        } else if flow == Some(Flow::GotoOn) {
            let value = self.expression()?;
            self.expect(b'{')?;
            let mut targets = Vec::with_capacity(usize::from(argc));
            for _ in 0..argc {
                targets.push(self.read_i32()? as usize);
            }
            self.expect(b'}')?;
            Command {
                op,
                argc,
                params: Vec::new(),
                kind: CommandKind::GotoOn { value, targets },
            }
        } else if flow == Some(Flow::GotoCase) {
            let value = self.expression()?;
            self.expect(b'{')?;
            let mut cases = Vec::with_capacity(usize::from(argc));
            let mut targets = Vec::with_capacity(usize::from(argc));
            for _ in 0..argc {
                self.expect(b'(')?;
                if self.at(0) == b')' {
                    self.pos += 1;
                    cases.push(None);
                } else {
                    let case = self.expression()?;
                    self.expect(b')')?;
                    cases.push(Some(case));
                }
                targets.push(self.read_i32()? as usize);
            }
            self.expect(b'}')?;
            Command {
                op,
                argc,
                params: Vec::new(),
                kind: CommandKind::GotoCase {
                    value,
                    cases,
                    targets,
                },
            }
        } else if flow == Some(Flow::GosubWith) {
            let params = self.params()?;
            let target = self.read_i32()? as usize;
            Command {
                op,
                argc,
                params,
                kind: CommandKind::GosubWith { target },
            }
        } else if flow == Some(Flow::Select) {
            let select = self.select(argc)?;
            Command {
                op,
                argc,
                params: Vec::new(),
                kind: CommandKind::Select(select),
            }
        } else {
            let params = self.params()?;
            Command {
                op,
                argc,
                params,
                kind: CommandKind::Plain,
            }
        };
        Ok(Element::Command(command))
    }

    fn expect(&mut self, byte: u8) -> Result<()> {
        if self.at(0) != byte {
            bail!(
                "reallive: expected {:?} at 0x{:x}, found 0x{:02x}",
                byte as char,
                self.pos,
                self.at(0)
            );
        }
        self.pos += 1;
        Ok(())
    }

    fn expression(&mut self) -> Result<Expr> {
        let mut parser = self.parser();
        let expression = parser.expression()?;
        self.pos = parser.pos;
        Ok(expression)
    }

    fn params(&mut self) -> Result<Vec<Param>> {
        let mut params = Vec::new();
        if self.at(0) != b'(' {
            return Ok(params);
        }
        self.pos += 1;
        loop {
            // Separators: commas and line markers (which may also precede
            // the closing parenthesis).
            match self.at(0) {
                b',' => {
                    self.pos += 1;
                    continue;
                }
                b'\n' => {
                    self.pos += 3;
                    continue;
                }
                b')' => break,
                _ => {}
            }
            if self.pos >= self.data.len() {
                bail!("reallive: unterminated parameter list");
            }
            let parser = self.parser();
            let length = parser.data_len()?;
            let raw = self.data[self.pos..self.pos + length].to_vec();
            let value = Parser::new(&raw, self.nls).data()?;
            params.push(Param { raw, value });
            self.pos += length;
        }
        self.pos += 1;
        Ok(params)
    }

    fn select(&mut self, argc: u16) -> Result<Select> {
        let window = if self.at(0) == b'(' {
            self.pos += 1;
            let window = self.expression()?;
            self.expect(b')')?;
            Some(window)
        } else {
            None
        };
        self.expect(b'{')?;
        if self.at(0) == b'\n' {
            self.pos += 3;
        }
        let mut options = Vec::with_capacity(usize::from(argc));
        for _ in 0..argc {
            while self.at(0) == b',' {
                self.pos += 1;
            }
            let mut conditions = Vec::new();
            if self.at(0) == b'(' {
                self.pos += 1;
                while self.at(0) != b')' {
                    if self.pos >= self.data.len() {
                        bail!("reallive: unterminated select condition");
                    }
                    let condition = if self.at(0) == b'(' {
                        Some(self.expression()?)
                    } else {
                        None
                    };
                    let effect = self.at(0);
                    self.pos += 1;
                    let takes_argument = effect != b'2' && effect != b'3';
                    let argument =
                        if takes_argument && self.at(0) != b')' && !self.at(0).is_ascii_digit() {
                            Some(self.expression()?)
                        } else {
                            None
                        };
                    conditions.push(SelectCondition {
                        condition,
                        effect,
                        argument,
                    });
                }
                self.pos += 1;
            }
            let parser = self.parser();
            let length = parser.string_len()?;
            let raw = &self.data[self.pos..self.pos + length];
            let text = if raw.is_empty() {
                Expr::Str(Vec::new())
            } else {
                Parser::new(raw, self.nls).data()?
            };
            self.pos += length;
            if self.at(0) != b'\n' {
                bail!("reallive: expected a line marker after a select option");
            }
            self.pos += 1;
            let line = self.read_i16()?;
            options.push(SelectOption {
                conditions,
                text,
                line,
            });
        }
        while self.at(0) == b'\n' {
            self.pos += 3;
        }
        self.expect(b'}')?;
        Ok(Select { window, options })
    }
}

/// Builders for hand-assembled bytecode (tests and tools).
pub mod asm {
    use super::Opcode;

    pub fn command(op: Opcode, argc: u16, params: &[Vec<u8>]) -> Vec<u8> {
        let mut out = vec![b'#', op.modtype, op.module];
        out.extend_from_slice(&op.opcode.to_le_bytes());
        out.extend_from_slice(&argc.to_le_bytes());
        out.push(op.overload);
        if !params.is_empty() {
            out.push(b'(');
            for param in params {
                out.extend_from_slice(param);
            }
            out.push(b')');
        }
        out
    }

    pub fn line(number: i16) -> Vec<u8> {
        let mut out = vec![b'\n'];
        out.extend_from_slice(&number.to_le_bytes());
        out
    }

    pub fn marker(index: i16) -> Vec<u8> {
        let mut out = vec![b'@'];
        out.extend_from_slice(&index.to_le_bytes());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::encode::*;
    use crate::expr::op;

    #[test]
    fn decodes_elements_and_resolves_jumps() {
        let mut code = asm::marker(0); // entrypoint 0
        code.extend(asm::line(1));
        let assignment = binary(&int_a(0), op::ASSIGN, &int(5));
        code.push(b'$');
        code.extend_from_slice(&assignment[1..]);
        let goto_at = code.len();
        code.extend(asm::command(Opcode::new(0, 1, 0, 0), 0, &[]));
        code.extend_from_slice(&0i32.to_le_bytes()); // patched below
        let text_at = code.len();
        code.extend(b"\x82\xa0,\"a\\\"b\"");
        code.push(0);
        let target = text_at as i32;
        code[goto_at + 8..goto_at + 12].copy_from_slice(&target.to_le_bytes());
        let script = Script::parse(&code, &[1_000_000], Nls::Sjis).unwrap();
        assert_eq!(script.entrypoint(0), Some(0));
        let Element::Command(goto) = &script.elements[3] else {
            panic!("{:?}", script.elements[3]);
        };
        assert_eq!(goto.targets(), vec![4]);
        let Element::Textout(raw) = &script.elements[4] else {
            panic!();
        };
        assert_eq!(Element::text(raw, Nls::Sjis), b"\x82\xa0a\"b");
    }

    #[test]
    fn decodes_plain_command_parameters() {
        let mut params = vec![quoted(b"BG01")];
        params.push(int(3));
        let code = asm::command(Opcode::new(1, 33, 73, 0), 2, &params);
        let script = Script::parse(&code, &[], Nls::Sjis).unwrap();
        let Element::Command(command) = &script.elements[0] else {
            panic!();
        };
        assert_eq!(command.params.len(), 2);
        assert_eq!(command.params[0].value, Expr::Str(b"BG01".to_vec()));
        assert_eq!(command.params[1].value, Expr::Int(3));
    }

    /// Seen in Little Busters!: an unquoted string directly followed by a
    /// special parameter, and a line marker before the closing parenthesis.
    #[test]
    fn decodes_specials_after_bare_strings_and_trailing_line_markers() {
        let mut code = vec![b'#', 1, 4, 0x2b, 0x03, 2, 0, 0, b'('];
        code.extend(int(1));
        code.extend(b"a\x01BG001N1a\x00");
        code.extend(int(2));
        code.extend(b"\n\x94\x00)");
        let script = Script::parse(&code, &[], Nls::Sjis).unwrap();
        let Element::Command(command) = &script.elements[0] else {
            panic!("{:?}", script.elements);
        };
        assert_eq!(command.params.len(), 3);
        assert_eq!(
            command.params[1].value,
            Expr::Special {
                tag: 1,
                pieces: vec![Expr::Str(b"BG001N1".to_vec())]
            }
        );
        assert_eq!(
            command.params[2].value,
            Expr::Special {
                tag: 0,
                pieces: vec![Expr::Int(2)]
            }
        );
    }
}
