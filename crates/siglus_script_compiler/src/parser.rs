use anyhow::{Result, anyhow, bail};

use crate::ast::*;
use crate::definitions::DefinitionTable;
use crate::lexer::{Token, TokenKind};

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    defs: &'a DefinitionTable,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], defs: &'a DefinitionTable) -> Self {
        Self {
            tokens,
            pos: 0,
            defs,
        }
    }

    pub fn parse(mut self) -> Result<Program> {
        let mut statements = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Eof) {
            statements.push(self.statement()?);
        }
        Ok(Program { statements })
    }

    fn statement(&mut self) -> Result<Statement> {
        let line = self.peek().line;
        let kind = match self.peek().kind.clone() {
            TokenKind::Label(name) => {
                self.pos += 1;
                StatementKind::Label(name)
            }
            TokenKind::ZLabel(number) => {
                self.pos += 1;
                StatementKind::ZLabel(number)
            }
            TokenKind::Text(text) => {
                self.pos += 1;
                StatementKind::Text(text)
            }
            TokenKind::NameOpen => {
                self.pos += 1;
                let name = match self.next().kind.clone() {
                    TokenKind::Text(v) | TokenKind::Str(v) => v,
                    other => bail!("line {line}: expected name text, got {other:?}"),
                };
                self.expect_symbol_or_name_close("】")?;
                StatementKind::Name(name)
            }
            TokenKind::Ident(ref word) if word == "property" => {
                StatementKind::Property(self.property_decl()?)
            }
            TokenKind::Ident(ref word) if word == "command" => {
                StatementKind::CommandDef(self.command_decl()?)
            }
            TokenKind::Ident(ref word)
                if word == "goto" || word == "gosub" || word == "gosubstr" =>
            {
                let (kind, label, args) = self.goto_expr()?;
                StatementKind::Goto { kind, label, args }
            }
            TokenKind::Ident(ref word) if word == "return" => {
                self.pos += 1;
                let value = if self.consume_symbol("(") {
                    let v = self.expr(0)?;
                    self.expect_symbol(")")?;
                    Some(v)
                } else {
                    None
                };
                StatementKind::Return(value)
            }
            TokenKind::Ident(ref word) if word == "if" => self.if_statement()?,
            TokenKind::Ident(ref word) if word == "for" => self.for_statement()?,
            TokenKind::Ident(ref word) if word == "while" => self.while_statement()?,
            TokenKind::Ident(ref word) if word == "switch" => self.switch_statement()?,
            TokenKind::Ident(ref word) if word == "continue" => {
                self.pos += 1;
                StatementKind::Continue
            }
            TokenKind::Ident(ref word) if word == "break" => {
                self.pos += 1;
                StatementKind::Break
            }
            TokenKind::Ident(_) => {
                let element = self.element_expr()?;
                if let Some(op) = self.assignment_operator() {
                    let right = self.expr(0)?;
                    StatementKind::Assign {
                        left: element,
                        op,
                        right,
                    }
                } else {
                    StatementKind::Command(Expr::Element(element))
                }
            }
            ref other => bail!("line {line}: illegal statement beginning with {other:?}"),
        };
        Ok(Statement { line, kind })
    }

    fn property_decl(&mut self) -> Result<PropertyDecl> {
        self.expect_ident("property")?;
        let name = self.take_ident()?;
        let form = if self.consume_symbol(":") {
            let form = self.take_ident()?;
            self.defs.form(&form)?
        } else {
            10
        };
        let size = if self.consume_symbol("[") {
            let value = self.expr(0)?;
            self.expect_symbol("]")?;
            Some(value)
        } else {
            None
        };
        Ok(PropertyDecl { name, form, size })
    }

    fn command_decl(&mut self) -> Result<CommandDecl> {
        self.expect_ident("command")?;
        let name = self.take_ident()?;
        let mut args = Vec::new();
        if self.consume_symbol("(") && !self.consume_symbol(")") {
            loop {
                args.push(self.property_decl()?);
                if self.consume_symbol(")") {
                    break;
                }
                self.expect_symbol(",")?;
            }
        }
        let form = if self.consume_symbol(":") {
            let name = self.take_ident()?;
            self.defs.form(&name)?
        } else {
            10
        };
        let body = self.block()?;
        Ok(CommandDecl {
            name,
            args,
            form,
            body,
        })
    }

    fn if_statement(&mut self) -> Result<StatementKind> {
        let mut branches = Vec::new();
        self.expect_ident("if")?;
        loop {
            self.expect_symbol("(")?;
            let condition = self.expr(0)?;
            self.expect_symbol(")")?;
            branches.push((Some(condition), self.block()?));
            if self.consume_ident("elseif") {
                continue;
            }
            if self.consume_ident("else") {
                branches.push((None, self.block()?));
            }
            break;
        }
        Ok(StatementKind::If(branches))
    }

    fn for_statement(&mut self) -> Result<StatementKind> {
        self.expect_ident("for")?;
        self.expect_symbol("(")?;
        let mut init = Vec::new();
        while !self.check_symbol(",") {
            init.push(self.statement()?);
        }
        self.expect_symbol(",")?;
        let condition = self.expr(0)?;
        self.expect_symbol(",")?;
        let mut step = Vec::new();
        while !self.check_symbol(")") {
            step.push(self.statement()?);
        }
        self.expect_symbol(")")?;
        let body = self.block()?;
        Ok(StatementKind::For {
            init,
            condition,
            step,
            body,
        })
    }

    fn while_statement(&mut self) -> Result<StatementKind> {
        self.expect_ident("while")?;
        self.expect_symbol("(")?;
        let condition = self.expr(0)?;
        self.expect_symbol(")")?;
        Ok(StatementKind::While {
            condition,
            body: self.block()?,
        })
    }

    fn switch_statement(&mut self) -> Result<StatementKind> {
        self.expect_ident("switch")?;
        self.expect_symbol("(")?;
        let value = self.expr(0)?;
        self.expect_symbol(")")?;
        self.expect_symbol("{")?;
        let mut cases = Vec::new();
        let mut default = Vec::new();
        let mut saw_default = false;
        while !self.consume_symbol("}") {
            if self.consume_ident("case") {
                self.expect_symbol("(")?;
                let case = self.expr(0)?;
                self.expect_symbol(")")?;
                let mut body = Vec::new();
                while !self.check_ident("case")
                    && !self.check_ident("default")
                    && !self.check_symbol("}")
                {
                    body.push(self.statement()?);
                }
                cases.push((case, body));
            } else if self.consume_ident("default") {
                if saw_default {
                    bail!("line {}: duplicate switch default", self.peek().line);
                }
                saw_default = true;
                while !self.check_ident("case")
                    && !self.check_ident("default")
                    && !self.check_symbol("}")
                {
                    default.push(self.statement()?);
                }
            } else {
                bail!("line {}: expected case/default", self.peek().line);
            }
        }
        Ok(StatementKind::Switch {
            value,
            cases,
            default,
        })
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        self.expect_symbol("{")?;
        let mut out = Vec::new();
        while !self.consume_symbol("}") {
            if matches!(self.peek().kind, TokenKind::Eof) {
                bail!("line {}: unclosed block", self.peek().line);
            }
            out.push(self.statement()?);
        }
        Ok(out)
    }

    fn goto_expr(&mut self) -> Result<(GotoKind, LabelRef, Vec<Argument>)> {
        let kind = match self.take_ident()?.as_str() {
            "goto" => GotoKind::Goto,
            "gosub" => GotoKind::Gosub,
            "gosubstr" => GotoKind::GosubStr,
            _ => unreachable!(),
        };
        let args = if !matches!(kind, GotoKind::Goto) && self.check_symbol("(") {
            self.argument_list()?
        } else {
            Vec::new()
        };
        let label = match self.next().kind.clone() {
            TokenKind::Label(name) => LabelRef::Named(name),
            TokenKind::ZLabel(number) => LabelRef::Z(number),
            other => bail!(
                "line {}: expected label after goto, got {other:?}",
                self.peek().line
            ),
        };
        Ok((kind, label, args))
    }

    fn expr(&mut self, min_precedence: u8) -> Result<Expr> {
        let mut left = match self.peek().kind.clone() {
            TokenKind::Symbol(ref s) if matches!(s.as_str(), "+" | "-" | "~") => {
                let op = unary_code(s);
                self.pos += 1;
                Expr::Unary {
                    op,
                    expr: Box::new(self.expr(11)?),
                }
            }
            TokenKind::Int(value) => {
                self.pos += 1;
                Expr::Int(value)
            }
            TokenKind::Str(value) | TokenKind::Text(value) => {
                self.pos += 1;
                Expr::Str(value)
            }
            TokenKind::Symbol(ref s) if s == "(" => {
                self.pos += 1;
                let value = self.expr(0)?;
                self.expect_symbol(")")?;
                value
            }
            TokenKind::Symbol(ref s) if s == "{" => {
                self.pos += 1;
                let mut values = Vec::new();
                if !self.consume_symbol("}") {
                    loop {
                        values.push(self.expr(0)?);
                        if self.consume_symbol("}") {
                            break;
                        }
                        self.expect_symbol(",")?;
                    }
                }
                Expr::List(values)
            }
            TokenKind::Ident(ref word)
                if matches!(word.as_str(), "goto" | "gosub" | "gosubstr") =>
            {
                let (kind, label, args) = self.goto_expr()?;
                Expr::Goto { kind, label, args }
            }
            TokenKind::Ident(_) => Expr::Element(self.element_expr()?),
            ref other => bail!(
                "line {}: expected expression, got {other:?}",
                self.peek().line
            ),
        };
        while let TokenKind::Symbol(op) = self.peek().kind.clone() {
            let Some((precedence, code)) = binary_info(&op) else {
                break;
            };
            if precedence < min_precedence {
                break;
            }
            self.pos += 1;
            let right = self.expr(precedence + 1)?;
            left = Expr::Binary {
                op: code,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn element_expr(&mut self) -> Result<ElementExpr> {
        let mut parts = Vec::new();
        let name = self.take_ident()?;
        let args = if self.check_symbol("(") {
            Some(self.argument_list()?)
        } else {
            None
        };
        parts.push(ElementPart::Name { name, args });
        loop {
            if self.consume_symbol(".") {
                let name = self.take_ident()?;
                let args = if self.check_symbol("(") {
                    Some(self.argument_list()?)
                } else {
                    None
                };
                parts.push(ElementPart::Name { name, args });
            } else if self.consume_symbol("[") {
                let index = self.expr(0)?;
                self.expect_symbol("]")?;
                parts.push(ElementPart::Array(index));
            } else {
                break;
            }
        }
        Ok(ElementExpr { parts })
    }

    fn argument_list(&mut self) -> Result<Vec<Argument>> {
        self.expect_symbol("(")?;
        let mut args = Vec::new();
        if self.consume_symbol(")") {
            return Ok(args);
        }
        loop {
            let name = if let TokenKind::Ident(name) = self.peek().kind.clone() {
                if self
                    .tokens
                    .get(self.pos + 1)
                    .is_some_and(|t| t.kind == TokenKind::Symbol("=".into()))
                {
                    self.pos += 2;
                    Some(name)
                } else {
                    None
                }
            } else {
                None
            };
            args.push(Argument {
                name,
                value: self.expr(0)?,
            });
            if self.consume_symbol(")") {
                break;
            }
            self.expect_symbol(",")?;
        }
        Ok(args)
    }

    fn assignment_operator(&mut self) -> Option<u8> {
        let TokenKind::Symbol(symbol) = &self.peek().kind else {
            return None;
        };
        let code = match symbol.as_str() {
            "=" => 0,
            "+=" => 1,
            "-=" => 2,
            "*=" => 3,
            "/=" => 4,
            "%=" => 5,
            "&=" => 0x31,
            "|=" => 0x32,
            "^=" => 0x33,
            "<<=" => 0x34,
            ">>=" => 0x35,
            ">>>=" => 0x36,
            _ => return None,
        };
        self.pos += 1;
        Some(code)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn next(&mut self) -> &Token {
        let pos = self.pos;
        self.pos += 1;
        &self.tokens[pos]
    }
    fn check_symbol(&self, symbol: &str) -> bool {
        self.peek().kind == TokenKind::Symbol(symbol.into())
    }
    fn consume_symbol(&mut self, symbol: &str) -> bool {
        if self.check_symbol(symbol) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect_symbol(&mut self, symbol: &str) -> Result<()> {
        if self.consume_symbol(symbol) {
            Ok(())
        } else {
            bail!("line {}: expected {symbol}", self.peek().line)
        }
    }
    fn expect_symbol_or_name_close(&mut self, symbol: &str) -> Result<()> {
        if matches!(self.peek().kind, TokenKind::NameClose) {
            self.pos += 1;
            Ok(())
        } else {
            self.expect_symbol(symbol)
        }
    }
    fn check_ident(&self, word: &str) -> bool {
        matches!(&self.peek().kind, TokenKind::Ident(v) if v == word)
    }
    fn consume_ident(&mut self, word: &str) -> bool {
        if self.check_ident(word) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect_ident(&mut self, word: &str) -> Result<()> {
        if self.consume_ident(word) {
            Ok(())
        } else {
            bail!("line {}: expected {word}", self.peek().line)
        }
    }
    fn take_ident(&mut self) -> Result<String> {
        match self.next().kind.clone() {
            TokenKind::Ident(v) => Ok(v),
            other => Err(anyhow!(
                "line {}: expected identifier, got {other:?}",
                self.peek().line
            )),
        }
    }
}

fn unary_code(op: &str) -> u8 {
    match op {
        "+" => 1,
        "-" => 2,
        "~" => 0x30,
        _ => unreachable!(),
    }
}

fn binary_info(op: &str) -> Option<(u8, u8)> {
    Some(match op {
        "||" => (1, 0x21),
        "&&" => (2, 0x20),
        "|" => (3, 0x32),
        "^" => (4, 0x33),
        "&" => (5, 0x31),
        "==" => (6, 0x10),
        "!=" => (6, 0x11),
        ">" => (7, 0x12),
        ">=" => (7, 0x13),
        "<" => (7, 0x14),
        "<=" => (7, 0x15),
        "<<" => (8, 0x34),
        ">>" => (8, 0x35),
        ">>>" => (8, 0x36),
        "+" => (9, 1),
        "-" => (9, 2),
        "*" => (10, 3),
        "/" => (10, 4),
        "%" => (10, 5),
        _ => return None,
    })
}
