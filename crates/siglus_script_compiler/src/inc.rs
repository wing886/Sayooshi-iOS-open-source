use std::collections::HashSet;

use anyhow::{Context, Result, anyhow, bail};

use crate::definitions::DefinitionTable;

#[derive(Debug, Clone)]
pub struct IncProperty {
    pub name: String,
    pub form: i32,
    pub size: i32,
}

#[derive(Debug, Clone)]
pub struct IncCommand {
    pub name: String,
    pub form: i32,
    pub args: Vec<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplacementKind {
    Replace,
    Define,
    Macro,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroArg {
    pub name: String,
    pub default: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replacement {
    pub kind: ReplacementKind,
    pub name: String,
    pub after: String,
    pub args: Vec<MacroArg>,
}

#[derive(Debug, Clone)]
struct PendingDeclaration {
    text: String,
    line: usize,
}

#[derive(Debug, Clone, Default)]
pub struct IncDefinitions {
    pub properties: Vec<IncProperty>,
    pub commands: Vec<IncCommand>,
    pub replacements: Vec<Replacement>,
    names: HashSet<String>,
    pending_properties: Vec<PendingDeclaration>,
    pending_commands: Vec<PendingDeclaration>,
}

impl IncDefinitions {
    pub fn contains_name(&self, name: &str) -> bool {
        self.names.contains(name)
    }

    pub fn defined_names(&self) -> &HashSet<String> {
        &self.names
    }
}

/// Parse one INC file in isolation. The compiler uses the two step functions
/// directly so all project files receive IA's original two-pass ordering.
pub fn parse_inc(source: &str, defs: &DefinitionTable) -> Result<IncDefinitions> {
    let mut result = IncDefinitions::default();
    parse_inc_step_1(source, &mut result)?;
    parse_inc_step_2(&mut result, defs)?;
    Ok(result)
}

pub(crate) fn parse_inc_step_1(source: &str, result: &mut IncDefinitions) -> Result<()> {
    let mut text: Vec<char> = comment_cut(source)?.chars().collect();
    let mut p = 0usize;
    let mut line = 1usize;
    let mut expand_count = 0usize;

    while skip_separators(&text, &mut p, &mut line) {
        let declaration_start = p;
        let declaration_line = line;
        let kind = [
            "#replace",
            "#define_s",
            "#define",
            "#macro",
            "#property",
            "#command",
            "#expand",
        ]
        .into_iter()
        .find(|name| consume(&text, &mut p, name))
        .ok_or_else(|| anyhow!("inc line {line}: invalid declaration"))?;

        match kind {
            "#replace" | "#define" | "#define_s" => {
                skip_separators(&text, &mut p, &mut line);
                let name_start = p;
                if kind == "#define_s" {
                    while p < text.len() && !matches!(text[p], '\t' | '\n') {
                        p += 1;
                    }
                } else {
                    while p < text.len() && !matches!(text[p], ' ' | '\t' | '\n') {
                        p += 1;
                    }
                }
                let name = chars_string(&text[name_start..p]);
                if name.is_empty() {
                    bail!("inc line {declaration_line}: {kind} needs a name");
                }
                let after = parse_body(&text, &mut p, &mut line, &result.names)?;
                insert_name(result, &name, declaration_line)?;
                result.replacements.push(Replacement {
                    kind: if kind == "#replace" {
                        ReplacementKind::Replace
                    } else {
                        ReplacementKind::Define
                    },
                    name,
                    after,
                    args: Vec::new(),
                });
            }
            "#macro" => {
                skip_separators(&text, &mut p, &mut line);
                let start = p;
                while p < text.len() && !matches!(text[p], ' ' | '\t' | '\n' | '(') {
                    p += 1;
                }
                let name = chars_string(&text[start..p]);
                if name.is_empty() || !name.starts_with('@') {
                    bail!("inc line {declaration_line}: macro name must start with @");
                }
                let args = parse_macro_declaration_args(&text, &mut p, &mut line)?;
                let after = parse_body(&text, &mut p, &mut line, &result.names)?;
                insert_name(result, &name, declaration_line)?;
                result.replacements.push(Replacement {
                    kind: ReplacementKind::Macro,
                    name,
                    after,
                    args,
                });
            }
            "#property" | "#command" => {
                skip_separators(&text, &mut p, &mut line);
                let body_line = line;
                let start = p;
                while p < text.len() && text[p] != '#' {
                    if text[p] == '\n' {
                        line += 1;
                    }
                    p += 1;
                }
                let pending = PendingDeclaration {
                    text: chars_string(&text[start..p]),
                    line: body_line,
                };
                if kind == "#property" {
                    result.pending_properties.push(pending);
                } else {
                    result.pending_commands.push(pending);
                }
            }
            "#expand" => {
                let after = parse_body(&text, &mut p, &mut line, &result.names)?;
                let expanded = expand_text(&after, &result.replacements, &[])?;
                text.splice(declaration_start..p, expanded.chars());
                p = declaration_start;
                expand_count += 1;
                if expand_count > 10_000 {
                    bail!("inc line {declaration_line}: #expand recursion limit exceeded");
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

pub(crate) fn parse_inc_step_2(result: &mut IncDefinitions, defs: &DefinitionTable) -> Result<()> {
    let properties = std::mem::take(&mut result.pending_properties);
    for pending in properties {
        let expanded = expand_text(&pending.text, &result.replacements, &[])
            .with_context(|| format!("inc line {}", pending.line))?;
        let declared_name = raw_declaration_name(&expanded, false)?;
        let property = parse_property(&expanded, defs)
            .with_context(|| format!("inc line {}", pending.line))?;
        insert_name(result, &declared_name, pending.line)?;
        result.properties.push(property);
    }

    let commands = std::mem::take(&mut result.pending_commands);
    for pending in commands {
        let expanded = expand_text(&pending.text, &result.replacements, &[])
            .with_context(|| format!("inc line {}", pending.line))?;
        let declared_name = raw_declaration_name(&expanded, true)?;
        let command =
            parse_command(&expanded, defs).with_context(|| format!("inc line {}", pending.line))?;
        insert_name(result, &declared_name, pending.line)?;
        result.commands.push(command);
    }
    Ok(())
}

pub(crate) fn parse_local_inc(
    source: &str,
    base: &IncDefinitions,
    defs: &DefinitionTable,
) -> Result<IncDefinitions> {
    let mut combined = base.clone();
    let property_start = combined.properties.len();
    let command_start = combined.commands.len();
    let replacement_start = combined.replacements.len();
    parse_inc_step_1(source, &mut combined)?;
    parse_inc_step_2(&mut combined, defs)?;

    let names = combined.names.difference(&base.names).cloned().collect();
    Ok(IncDefinitions {
        properties: combined.properties.split_off(property_start),
        commands: combined.commands.split_off(command_start),
        replacements: combined.replacements.split_off(replacement_start),
        names,
        pending_properties: Vec::new(),
        pending_commands: Vec::new(),
    })
}

fn insert_name(result: &mut IncDefinitions, name: &str, line: usize) -> Result<()> {
    if !result.names.insert(name.to_owned()) {
        bail!("inc line {line}: duplicate declaration {name}");
    }
    Ok(())
}

fn raw_declaration_name(text: &str, command: bool) -> Result<String> {
    let name = text
        .trim_start_matches([' ', '\t', '\n'])
        .split(|ch| matches!(ch, ' ' | ':' | '\t' | '\n') || (command && ch == '('))
        .next()
        .unwrap_or_default();
    if name.is_empty() {
        bail!("declaration name is missing");
    }
    Ok(name.to_owned())
}

fn parse_property(text: &str, defs: &DefinitionTable) -> Result<IncProperty> {
    let chars: Vec<char> = text.chars().collect();
    let mut p = 0;
    skip_space(&chars, &mut p);
    let start = p;
    while p < chars.len() && !matches!(chars[p], ' ' | ':' | '\t' | '\n') {
        p += 1;
    }
    let raw_name = chars_string(&chars[start..p]);
    let name = normalize_symbol(&raw_name)?;
    skip_space(&chars, &mut p);
    let mut form = defs.form("int")?;
    let mut size = 0;
    if chars.get(p) == Some(&':') {
        p += 1;
        skip_space(&chars, &mut p);
        let start = p;
        while p < chars.len() && is_word_continue(chars[p]) {
            p += 1;
        }
        form = defs.form(&chars_string(&chars[start..p]))?;
        skip_space(&chars, &mut p);
        if chars.get(p) == Some(&'[') {
            p += 1;
            skip_space(&chars, &mut p);
            let start = p;
            while p < chars.len() && chars[p].is_ascii_digit() {
                p += 1;
            }
            if start == p {
                bail!("property array size is missing");
            }
            size = chars_string(&chars[start..p]).parse()?;
            skip_space(&chars, &mut p);
            if chars.get(p) != Some(&']') {
                bail!("unclosed property array size");
            }
            let int_list = defs.form("intlist")?;
            let str_list = defs.form("strlist")?;
            if form != int_list && form != str_list {
                bail!("only intlist and strlist properties can have an array size");
            }
        }
    }
    if form == defs.form("void")? {
        bail!("a property cannot have void form");
    }
    Ok(IncProperty { name, form, size })
}

fn parse_command(text: &str, defs: &DefinitionTable) -> Result<IncCommand> {
    let chars: Vec<char> = text.chars().collect();
    let mut p = 0;
    skip_space(&chars, &mut p);
    let start = p;
    while p < chars.len() && !matches!(chars[p], ' ' | '(' | ':' | '\t' | '\n') {
        p += 1;
    }
    let raw_name = chars_string(&chars[start..p]);
    let name = normalize_symbol(&raw_name)?;
    skip_space(&chars, &mut p);

    let mut args = Vec::new();
    let mut saw_default = false;
    if chars.get(p) == Some(&'(') {
        p += 1;
        if chars.get(p) == Some(&')') {
            bail!("empty command argument list; omit the parentheses");
        }
        loop {
            skip_space(&chars, &mut p);
            let start = p;
            while p < chars.len() && is_word_continue(chars[p]) {
                p += 1;
            }
            if start == p {
                bail!("command argument form is missing");
            }
            let arg_form = defs.form(&chars_string(&chars[start..p]))?;
            args.push(arg_form);
            skip_space(&chars, &mut p);
            let has_default = chars.get(p) == Some(&'(');
            if has_default {
                p += 1;
                if arg_form == defs.form("str")? {
                    if chars.get(p) != Some(&'"') {
                        bail!("string default must be double quoted");
                    }
                    p += 1;
                    while p < chars.len() && chars[p] != '"' {
                        if chars[p] == '\\' {
                            p += 1;
                        }
                        p += 1;
                    }
                    if chars.get(p) != Some(&'"') {
                        bail!("unclosed string default");
                    }
                    p += 1;
                } else {
                    let start = p;
                    if chars.get(p) == Some(&'-') {
                        p += 1;
                    }
                    while p < chars.len() && chars[p].is_ascii_digit() {
                        p += 1;
                    }
                    if start == p {
                        bail!("integer default is missing");
                    }
                }
                skip_space(&chars, &mut p);
                if chars.get(p) != Some(&')') {
                    bail!("unclosed command argument default");
                }
                p += 1;
            } else if saw_default {
                bail!("arguments after a default must also have defaults");
            }
            saw_default |= has_default;
            skip_space(&chars, &mut p);
            match chars.get(p) {
                Some(',') => p += 1,
                Some(')') => {
                    p += 1;
                    break;
                }
                _ => bail!("unclosed command argument list"),
            }
        }
    }
    skip_space(&chars, &mut p);
    let form = if chars.get(p) == Some(&':') {
        p += 1;
        skip_space(&chars, &mut p);
        let start = p;
        while p < chars.len() && is_word_continue(chars[p]) {
            p += 1;
        }
        defs.form(&chars_string(&chars[start..p]))?
    } else {
        defs.form("int")?
    };
    Ok(IncCommand { name, form, args })
}

fn normalize_symbol(raw: &str) -> Result<String> {
    let name = raw.trim_start_matches(['$', '@']).to_owned();
    if name.is_empty() || !name.chars().all(is_word_continue) {
        bail!("invalid user symbol {raw:?}");
    }
    Ok(name)
}

fn comment_cut(source: &str) -> Result<String> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        None,
        Single,
        SingleEscape,
        Double,
        DoubleEscape,
        LineComment,
        BlockComment,
    }
    let chars: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut state = State::None;
    let mut p = 0;
    let mut line = 1usize;
    let mut block_line = 1usize;
    while p < chars.len() {
        let ch = chars[p];
        let next = chars.get(p + 1).copied();
        if ch == '\n' {
            if matches!(
                state,
                State::Single | State::SingleEscape | State::Double | State::DoubleEscape
            ) {
                bail!("inc line {line}: newline inside quoted literal");
            }
            if state == State::LineComment {
                state = State::None;
            }
            out.push('\n');
            line += 1;
            p += 1;
            continue;
        }
        match state {
            State::LineComment => p += 1,
            State::BlockComment => {
                if ch == '*' && next == Some('/') {
                    state = State::None;
                    p += 2;
                } else {
                    p += 1;
                }
            }
            State::Single | State::Double => {
                out.push(ch);
                let quote = if state == State::Single { '\'' } else { '"' };
                if ch == quote {
                    state = State::None;
                } else if ch == '\\' {
                    state = if quote == '\'' {
                        State::SingleEscape
                    } else {
                        State::DoubleEscape
                    };
                }
                p += 1;
            }
            State::SingleEscape | State::DoubleEscape => {
                if !matches!(ch, '"' | '\\' | 'n') {
                    bail!("inc line {line}: invalid escape \\{ch}");
                }
                out.push(ch);
                state = if state == State::SingleEscape {
                    State::Single
                } else {
                    State::Double
                };
                p += 1;
            }
            State::None => {
                if ch == '\'' {
                    state = State::Single;
                    out.push(ch);
                    p += 1;
                } else if ch == '"' {
                    state = State::Double;
                    out.push(ch);
                    p += 1;
                } else if ch == ';' || (ch == '/' && next == Some('/')) {
                    state = State::LineComment;
                    p += if ch == ';' { 1 } else { 2 };
                } else if ch == '/' && next == Some('*') {
                    state = State::BlockComment;
                    block_line = line;
                    p += 2;
                } else {
                    out.push(ch.to_ascii_lowercase());
                    p += 1;
                }
            }
        }
    }
    match state {
        State::Single | State::SingleEscape | State::Double | State::DoubleEscape => {
            bail!("inc line {line}: unclosed quote")
        }
        State::BlockComment => bail!("inc line {block_line}: unclosed block comment"),
        _ => Ok(out),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum IfState {
    True,
    False,
    FalseMore,
}

fn parse_body(
    chars: &[char],
    p: &mut usize,
    line: &mut usize,
    names: &HashSet<String>,
) -> Result<String> {
    skip_separators(chars, p, line);
    let mut out = String::new();
    let mut states: Vec<IfState> = Vec::new();
    while *p < chars.len() {
        if starts_with(chars, *p, "##") {
            out.push('#');
            *p += 2;
        } else if consume(chars, p, "#ifdef") {
            let word = conditional_word(chars, p, line, "#ifdef")?;
            states.push(if names.contains(&word) {
                IfState::True
            } else {
                IfState::False
            });
        } else if consume(chars, p, "#elseifdef") {
            let word = conditional_word(chars, p, line, "#elseifdef")?;
            let state = states
                .last_mut()
                .ok_or_else(|| anyhow!("inc line {}: #elseifdef without #ifdef", *line))?;
            *state = match *state {
                IfState::True | IfState::FalseMore => IfState::FalseMore,
                IfState::False if names.contains(&word) => IfState::True,
                IfState::False => IfState::False,
            };
        } else if consume(chars, p, "#else") {
            let state = states
                .last_mut()
                .ok_or_else(|| anyhow!("inc line {}: #else without #ifdef", *line))?;
            *state = match *state {
                IfState::True | IfState::FalseMore => IfState::FalseMore,
                IfState::False => IfState::True,
            };
        } else if consume(chars, p, "#endif") {
            states
                .pop()
                .ok_or_else(|| anyhow!("inc line {}: #endif without #ifdef", *line))?;
        } else if chars[*p] == '\n' {
            out.push(' ');
            *line += 1;
            *p += 1;
        } else if !branch_active(&states) {
            *p += 1;
        } else if chars[*p] == '#' {
            break;
        } else {
            out.push(chars[*p]);
            *p += 1;
        }
    }
    while matches!(out.chars().last(), Some(' ' | '\t')) {
        out.pop();
    }
    Ok(out)
}

fn conditional_word(
    chars: &[char],
    p: &mut usize,
    line: &mut usize,
    directive: &str,
) -> Result<String> {
    if !skip_separators(chars, p, line) {
        bail!("inc line {}: {directive} needs a name", *line);
    }
    let start = *p;
    if !is_word_start(chars[*p]) {
        bail!("inc line {}: {directive} needs a name", *line);
    }
    *p += 1;
    while *p < chars.len() && is_word_continue(chars[*p]) {
        *p += 1;
    }
    Ok(chars_string(&chars[start..*p]))
}

fn parse_macro_declaration_args(
    chars: &[char],
    p: &mut usize,
    line: &mut usize,
) -> Result<Vec<MacroArg>> {
    if !skip_separators(chars, p, line) {
        bail!("inc line {}: macro argument parsing reached EOF", *line);
    }
    if chars.get(*p) != Some(&'(') {
        return Ok(Vec::new());
    }
    *p += 1;
    let mut args = Vec::new();
    loop {
        skip_separators(chars, p, line);
        let start = *p;
        while *p < chars.len()
            && !matches!(chars[*p], ' ' | '\t' | '\n' | ',' | '(' | ')' | '"' | '\'')
        {
            *p += 1;
        }
        let name = chars_string(&chars[start..*p]);
        if name.is_empty() {
            bail!("inc line {}: macro argument name is missing", *line);
        }
        skip_separators(chars, p, line);
        let default = if chars.get(*p) == Some(&'(') {
            *p += 1;
            let start = *p;
            while *p < chars.len() && chars[*p] != ')' {
                if matches!(chars[*p], '\t' | '\n') {
                    bail!("inc line {}: invalid macro default", *line);
                }
                *p += 1;
            }
            if *p == chars.len() {
                bail!("inc line {}: unclosed macro default", *line);
            }
            let value = chars_string(&chars[start..*p]);
            *p += 1;
            value
        } else {
            String::new()
        };
        args.push(MacroArg { name, default });
        skip_separators(chars, p, line);
        match chars.get(*p) {
            Some(',') => *p += 1,
            Some(')') => {
                *p += 1;
                return Ok(args);
            }
            _ => bail!("inc line {}: unclosed macro argument list", *line),
        }
    }
}

pub fn apply_replacements(script: &str, replacements: &[Replacement]) -> Result<String> {
    expand_text(script, replacements, &[])
}

fn expand_text(
    script: &str,
    replacements: &[Replacement],
    added: &[Replacement],
) -> Result<String> {
    let mut text: Vec<char> = script.chars().collect();
    let mut p = 0usize;
    let mut non_progress = 0usize;
    let mut shortest_remaining = text.len();
    while p < text.len() {
        if text[p] == '\n' {
            p += 1;
        } else {
            replace_one(&mut text, &mut p, replacements, added)?;
        }
        let remaining = text.len().saturating_sub(p);
        if remaining >= shortest_remaining {
            non_progress += 1;
            if non_progress > 10_000 {
                bail!("replacement recursion limit exceeded");
            }
        } else {
            shortest_remaining = remaining;
            non_progress = 0;
        }
    }
    Ok(chars_string(&text))
}

fn replace_one(
    text: &mut Vec<char>,
    p: &mut usize,
    defaults: &[Replacement],
    added: &[Replacement],
) -> Result<()> {
    let default_match = longest_match(text, *p, defaults);
    let added_match = longest_match(text, *p, added);
    let replacement = match (default_match, added_match) {
        (None, None) => {
            *p += 1;
            return Ok(());
        }
        (Some(a), Some(b)) => {
            if a.name > b.name {
                a
            } else {
                b
            }
        }
        (Some(a), None) => a,
        (None, Some(b)) => b,
    };
    let name_len = replacement.name.chars().count();
    let start = *p;
    match replacement.kind {
        ReplacementKind::Replace => {
            let after: Vec<char> = replacement.after.chars().collect();
            text.splice(start..start + name_len, after.iter().copied());
            *p = start + after.len();
        }
        ReplacementKind::Define => {
            text.splice(start..start + name_len, replacement.after.chars());
            *p = start;
        }
        ReplacementKind::Macro => {
            let mut end = start + name_len;
            let actual = parse_macro_actuals(text, &mut end)?;
            if replacement.args.is_empty() && !actual.is_empty() {
                bail!("macro {} does not take arguments", replacement.name);
            }
            if actual.len() > replacement.args.len() {
                bail!("macro {} received too many arguments", replacement.name);
            }
            let mut arg_replacements = Vec::with_capacity(replacement.args.len());
            for (index, arg) in replacement.args.iter().enumerate() {
                let value = actual
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| arg.default.clone());
                if value.is_empty() {
                    bail!(
                        "macro {} is missing argument {}",
                        replacement.name,
                        index + 1
                    );
                }
                arg_replacements.push(Replacement {
                    kind: ReplacementKind::Replace,
                    name: arg.name.clone(),
                    after: expand_text(&value, defaults, added)?,
                    args: Vec::new(),
                });
            }
            arg_replacements.sort_by_key(|a| std::cmp::Reverse(a.name.len()));
            let expanded = expand_text(&replacement.after, defaults, &arg_replacements)?;
            let expanded_chars: Vec<char> = expanded.chars().collect();
            text.splice(start..end, expanded_chars.iter().copied());
            *p = start + expanded_chars.len();
        }
    }
    Ok(())
}

fn parse_macro_actuals(text: &[char], p: &mut usize) -> Result<Vec<String>> {
    if text.get(*p) != Some(&'(') {
        return Ok(Vec::new());
    }
    *p += 1;
    let mut args = Vec::new();
    let mut start = *p;
    let mut depth = 0usize;
    while *p < text.len() {
        match text[*p] {
            '\'' | '"' => {
                let quote = text[*p];
                *p += 1;
                while *p < text.len() && text[*p] != quote {
                    if text[*p] == '\\' {
                        *p += 1;
                    }
                    *p += 1;
                }
                if *p == text.len() {
                    bail!("unclosed quote in macro invocation");
                }
                *p += 1;
            }
            '(' => {
                depth += 1;
                *p += 1;
            }
            ',' if depth == 0 => {
                if start == *p {
                    bail!("empty macro argument");
                }
                args.push(chars_string(&text[start..*p]));
                *p += 1;
                start = *p;
            }
            ')' if depth == 0 => {
                if start != *p {
                    args.push(chars_string(&text[start..*p]));
                } else if !args.is_empty() {
                    bail!("empty macro argument");
                }
                *p += 1;
                return Ok(args);
            }
            ')' => {
                depth -= 1;
                *p += 1;
            }
            _ => *p += 1,
        }
    }
    bail!("unclosed macro invocation")
}

fn longest_match<'a>(
    text: &[char],
    p: usize,
    replacements: &'a [Replacement],
) -> Option<&'a Replacement> {
    replacements
        .iter()
        .filter(|replacement| starts_with(text, p, &replacement.name))
        .max_by_key(|replacement| replacement.name.chars().count())
}

fn branch_active(states: &[IfState]) -> bool {
    states.last().is_none_or(|state| *state == IfState::True)
}

fn skip_separators(chars: &[char], p: &mut usize, line: &mut usize) -> bool {
    while *p < chars.len() && matches!(chars[*p], ' ' | '\t' | '\n') {
        if chars[*p] == '\n' {
            *line += 1;
        }
        *p += 1;
    }
    *p < chars.len()
}

fn skip_space(chars: &[char], p: &mut usize) {
    while *p < chars.len() && matches!(chars[*p], ' ' | '\t' | '\n' | '\r') {
        *p += 1;
    }
}

fn consume(chars: &[char], p: &mut usize, needle: &str) -> bool {
    if starts_with(chars, *p, needle) {
        *p += needle.chars().count();
        true
    } else {
        false
    }
}

fn starts_with(chars: &[char], p: usize, needle: &str) -> bool {
    needle
        .chars()
        .enumerate()
        .all(|(i, ch)| chars.get(p + i) == Some(&ch))
}

fn is_word_start(ch: char) -> bool {
    ch == '_' || ch == '@' || ch.is_alphabetic()
}

fn is_word_continue(ch: char) -> bool {
    ch == '_' || ch == '@' || ch.is_alphanumeric()
}

fn chars_string(chars: &[char]) -> String {
    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definitions(source: &str) -> IncDefinitions {
        parse_inc(source, &DefinitionTable::new().unwrap()).unwrap()
    }

    #[test]
    fn define_recurses_but_replace_does_not() {
        let inc = definitions("#define a b #replace b c");
        assert_eq!(apply_replacements("a b", &inc.replacements).unwrap(), "c c");

        let inc = definitions("#replace a b #replace b c");
        assert_eq!(apply_replacements("a b", &inc.replacements).unwrap(), "b c");
    }

    #[test]
    fn replacement_is_prefix_based_and_applies_inside_strings() {
        let inc = definitions("#replace foo X");
        assert_eq!(
            apply_replacements("foobar \"foo\"", &inc.replacements).unwrap(),
            "xbar \"x\""
        );
    }

    #[test]
    fn macro_expands_defaults_nested_calls_and_arguments() {
        let inc =
            definitions("#define one 1 #macro @pair(a,b(one)) a+(b) #macro @wrap(x) @pair(x)");
        assert_eq!(
            apply_replacements("@wrap((2+3))", &inc.replacements).unwrap(),
            "(2+3)+(1)"
        );
    }

    #[test]
    fn declaration_conditionals_follow_names_seen_so_far() {
        let inc = definitions("#define flag 1 #define value #ifdef flag yes#else no#endif");
        assert_eq!(
            apply_replacements("value", &inc.replacements).unwrap(),
            " yes"
        );
    }

    #[test]
    fn expand_inserts_declarations_back_into_ia_input() {
        let inc = definitions("#replace make #define made 7 #expand make");
        assert_eq!(apply_replacements("made", &inc.replacements).unwrap(), "7");
    }
}
