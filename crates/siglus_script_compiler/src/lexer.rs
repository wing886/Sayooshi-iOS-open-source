use std::collections::HashSet;

use anyhow::{Result, bail};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Int(i32),
    Str(String),
    Text(String),
    Label(String),
    ZLabel(usize),
    NameOpen,
    NameClose,
    Symbol(String),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: i32,
}

pub struct Preprocessed {
    pub script: String,
    pub local_inc: String,
}

pub fn preprocess(source: &str, defined_names: &HashSet<String>) -> Result<Preprocessed> {
    let mut clean = String::with_capacity(source.len());
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    let mut line = 1;
    let mut quote: Option<char> = None;
    let mut line_comment = false;
    let mut block_comment = false;
    while i < chars.len() {
        let ch = chars[i];
        let next = chars.get(i + 1).copied();
        if ch == '\n' {
            if quote.is_some() {
                bail!("line {line}: newline inside quoted literal");
            }
            line_comment = false;
            clean.push(ch);
            line += 1;
            i += 1;
        } else if line_comment {
            i += 1;
        } else if block_comment {
            if ch == '*' && next == Some('/') {
                block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
        } else if let Some(q) = quote {
            clean.push(ch);
            if ch == '\\' {
                let escaped =
                    next.ok_or_else(|| anyhow::anyhow!("line {line}: dangling escape"))?;
                if !matches!(escaped, '\\' | '\'' | '"' | 'n') {
                    bail!("line {line}: invalid escape \\{escaped}");
                }
                clean.push(escaped);
                i += 2;
            } else {
                if ch == q {
                    quote = None;
                }
                i += 1;
            }
        } else if ch == '\'' || ch == '"' {
            quote = Some(ch);
            clean.push(ch);
            i += 1;
        } else if ch == ';' || (ch == '/' && next == Some('/')) {
            line_comment = true;
            i += if ch == ';' { 1 } else { 2 };
        } else if ch == '/' && next == Some('*') {
            block_comment = true;
            i += 2;
        } else {
            clean.push(if ch.is_ascii_uppercase() {
                ch.to_ascii_lowercase()
            } else {
                ch
            });
            i += 1;
        }
    }
    if quote.is_some() {
        bail!("line {line}: unclosed quote");
    }
    if block_comment {
        bail!("unclosed block comment");
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Quote {
        None,
        Single,
        SingleEscape,
        Double,
        DoubleEscape,
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum IfState {
        True,
        False,
        FalseMore,
    }

    let chars: Vec<char> = clean.chars().collect();
    let mut script = String::with_capacity(clean.len());
    let mut local_inc = String::new();
    let mut in_inc = false;
    let mut states: Vec<IfState> = Vec::new();
    let mut state = Quote::None;
    let mut p = 0usize;
    let mut line = 1usize;
    while p < chars.len() {
        let ch = chars[p];
        if ch == '\n' {
            if state != Quote::None {
                bail!("line {line}: newline inside quoted literal");
            }
            if in_inc {
                local_inc.push(ch);
            }
            script.push(ch);
            line += 1;
            p += 1;
            continue;
        }

        match state {
            Quote::Single => {
                if ch == '\\' {
                    state = Quote::SingleEscape;
                } else if ch == '\'' {
                    state = Quote::None;
                }
            }
            Quote::SingleEscape => state = Quote::Single,
            Quote::Double => {
                if ch == '\\' {
                    state = Quote::DoubleEscape;
                } else if ch == '"' {
                    state = Quote::None;
                }
            }
            Quote::DoubleEscape => state = Quote::Double,
            Quote::None => {
                if ch == '\'' {
                    state = Quote::Single;
                } else if ch == '"' {
                    state = Quote::Double;
                } else if consume_directive(&chars, &mut p, "#ifdef") {
                    let word = source_conditional_word(&chars, &mut p)
                        .ok_or_else(|| anyhow::anyhow!("line {line}: #ifdef needs a name"))?;
                    states.push(if defined_names.contains(&word) {
                        IfState::True
                    } else {
                        IfState::False
                    });
                    continue;
                } else if consume_directive(&chars, &mut p, "#elseifdef") {
                    let word = source_conditional_word(&chars, &mut p)
                        .ok_or_else(|| anyhow::anyhow!("line {line}: #elseifdef needs a name"))?;
                    let current = states
                        .last_mut()
                        .ok_or_else(|| anyhow::anyhow!("line {line}: #elseifdef without #ifdef"))?;
                    *current = match *current {
                        IfState::True | IfState::FalseMore => IfState::FalseMore,
                        IfState::False if defined_names.contains(&word) => IfState::True,
                        IfState::False => IfState::False,
                    };
                    continue;
                } else if consume_directive(&chars, &mut p, "#else") {
                    let current = states
                        .last_mut()
                        .ok_or_else(|| anyhow::anyhow!("line {line}: #else without #ifdef"))?;
                    *current = match *current {
                        IfState::True | IfState::FalseMore => IfState::FalseMore,
                        IfState::False => IfState::True,
                    };
                    continue;
                } else if consume_directive(&chars, &mut p, "#endif") {
                    states
                        .pop()
                        .ok_or_else(|| anyhow::anyhow!("line {line}: #endif without #ifdef"))?;
                    continue;
                } else if consume_directive(&chars, &mut p, "#inc_start") {
                    in_inc = true;
                    continue;
                } else if consume_directive(&chars, &mut p, "#inc_end") {
                    if !in_inc {
                        bail!("line {line}: #inc_end without #inc_start");
                    }
                    in_inc = false;
                    continue;
                }
            }
        }

        let active = states.last().is_none_or(|state| *state == IfState::True);
        if active {
            if in_inc {
                local_inc.push(ch);
            } else {
                script.push(ch);
            }
        }
        p += 1;
    }
    if in_inc {
        bail!("line {line}: unclosed #inc_start");
    }
    if !states.is_empty() {
        bail!("line {line}: unclosed #ifdef");
    }
    Ok(Preprocessed { script, local_inc })
}

fn consume_directive(chars: &[char], p: &mut usize, directive: &str) -> bool {
    if directive
        .chars()
        .enumerate()
        .all(|(offset, ch)| chars.get(*p + offset) == Some(&ch))
    {
        *p += directive.chars().count();
        true
    } else {
        false
    }
}

fn source_conditional_word(chars: &[char], p: &mut usize) -> Option<String> {
    while matches!(chars.get(*p), Some(' ' | '\t')) {
        *p += 1;
    }
    let start = *p;
    let first = *chars.get(*p)?;
    if !(first == '_' || first == '@' || first.is_alphabetic()) {
        return None;
    }
    *p += 1;
    while chars
        .get(*p)
        .is_some_and(|ch| *ch == '_' || *ch == '@' || ch.is_alphanumeric())
    {
        *p += 1;
    }
    Some(chars[start..*p].iter().collect())
}

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut line = 1i32;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '\n' {
            line += 1;
            i += 1;
            continue;
        }
        if ch == ' ' || ch == '\t' || ch == '\r' {
            i += 1;
            continue;
        }
        if ch == '【' {
            tokens.push(Token {
                kind: TokenKind::NameOpen,
                line,
            });
            i += 1;
            continue;
        }
        if ch == '】' {
            tokens.push(Token {
                kind: TokenKind::NameClose,
                line,
            });
            i += 1;
            continue;
        }
        if ch == '#' {
            i += 1;
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_lowercase() || chars[i].is_ascii_digit() || chars[i] == '_')
            {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            if let Some(digits) = name
                .strip_prefix('z')
                .filter(|d| !d.is_empty() && d.len() <= 3 && d.chars().all(|c| c.is_ascii_digit()))
            {
                tokens.push(Token {
                    kind: TokenKind::ZLabel(digits.parse()?),
                    line,
                });
            } else {
                if name.is_empty() {
                    bail!("line {line}: empty label");
                }
                tokens.push(Token {
                    kind: TokenKind::Label(name),
                    line,
                });
            }
            continue;
        }
        if ch == '"' {
            let (value, next) = quoted(&chars, i, '"', line)?;
            tokens.push(Token {
                kind: TokenKind::Str(value),
                line,
            });
            i = next;
            continue;
        }
        if ch == '\'' {
            let (value, next) = quoted(&chars, i, '\'', line)?;
            let mut iter = value.chars();
            let value = iter
                .next()
                .ok_or_else(|| anyhow::anyhow!("line {line}: empty character literal"))?;
            if iter.next().is_some() {
                bail!("line {line}: character literal must contain one character");
            }
            tokens.push(Token {
                kind: TokenKind::Int(value as i32),
                line,
            });
            i = next;
            continue;
        }
        if ch.is_ascii_digit() {
            let start = i;
            if ch == '0' && chars.get(i + 1) == Some(&'x') {
                i += 2;
                let digits = i;
                while i < chars.len() && chars[i].is_ascii_hexdigit() {
                    i += 1;
                }
                let raw: String = chars[digits..i].iter().collect();
                tokens.push(Token {
                    kind: TokenKind::Int(i32::from_str_radix(&raw, 16)?),
                    line,
                });
            } else if ch == '0' && chars.get(i + 1) == Some(&'b') {
                i += 2;
                let digits = i;
                while i < chars.len() && matches!(chars[i], '0' | '1') {
                    i += 1;
                }
                let raw: String = chars[digits..i].iter().collect();
                tokens.push(Token {
                    kind: TokenKind::Int(i32::from_str_radix(&raw, 2)?),
                    line,
                });
            } else {
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let raw: String = chars[start..i].iter().collect();
                tokens.push(Token {
                    kind: TokenKind::Int(raw.parse()?),
                    line,
                });
            }
            continue;
        }
        if ch == '_' || ch == '$' || ch == '@' || ch.is_ascii_lowercase() {
            let start = i;
            i += 1;
            while i < chars.len()
                && (chars[i] == '_'
                    || chars[i] == '$'
                    || chars[i] == '@'
                    || chars[i].is_ascii_lowercase()
                    || chars[i].is_ascii_digit())
            {
                i += 1;
            }
            tokens.push(Token {
                kind: TokenKind::Ident(chars[start..i].iter().collect()),
                line,
            });
            continue;
        }
        let operators = [
            ">>>=", ">>>", "<<=", ">>=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<",
            ">>", "==", "!=", ">=", "<=", "&&", "||",
        ];
        let rest: String = chars[i..chars.len().min(i + 4)].iter().collect();
        if let Some(op) = operators.iter().find(|op| rest.starts_with(**op)) {
            tokens.push(Token {
                kind: TokenKind::Symbol((*op).into()),
                line,
            });
            i += op.len();
            continue;
        }
        if "=+-*/%&|^><~.,:()[]{}".contains(ch) {
            tokens.push(Token {
                kind: TokenKind::Symbol(ch.to_string()),
                line,
            });
            i += 1;
            continue;
        }
        if !ch.is_ascii() {
            let start = i;
            i += 1;
            while i < chars.len()
                && !chars[i].is_ascii()
                && chars[i] != '【'
                && chars[i] != '】'
                && chars[i] != '\n'
            {
                i += 1;
            }
            tokens.push(Token {
                kind: TokenKind::Text(chars[start..i].iter().collect()),
                line,
            });
            continue;
        }
        bail!("line {line}: illegal character {ch:?}");
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        line,
    });
    Ok(tokens)
}

fn quoted(chars: &[char], mut i: usize, quote: char, line: i32) -> Result<(String, usize)> {
    i += 1;
    let mut out = String::new();
    while i < chars.len() && chars[i] != quote {
        if chars[i] == '\n' {
            bail!("line {line}: newline in quoted literal");
        }
        if chars[i] == '\\' {
            i += 1;
            let ch = *chars
                .get(i)
                .ok_or_else(|| anyhow::anyhow!("line {line}: dangling escape"))?;
            out.push(if ch == 'n' { '\n' } else { ch });
        } else {
            out.push(chars[i]);
        }
        i += 1;
    }
    if chars.get(i) != Some(&quote) {
        bail!("line {line}: unclosed quoted literal");
    }
    Ok((out, i + 1))
}
