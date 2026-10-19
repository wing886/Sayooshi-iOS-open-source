use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, anyhow, bail};
use siglus_compiler_common::{put_i32 as push_i32, utf16le};

use crate::ast::*;
use crate::definitions::{ArgDef, DefinitionTable, ElementKind, Overload, ResolvedElement};
use crate::inc::{
    IncCommand, IncDefinitions, IncProperty, apply_replacements, parse_inc_step_1,
    parse_inc_step_2, parse_local_inc,
};
use crate::lexer::{TokenKind, lex, preprocess};
use crate::parser::Parser;

const FM_LIST: i32 = -1;
const FM_VOID: i32 = 0;
const FM_INT: i32 = 10;
const FM_INTLIST: i32 = 11;
const FM_INTREF: i32 = 13;
const FM_INTLISTREF: i32 = 14;
const FM_STR: i32 = 20;
const FM_STRLIST: i32 = 21;
const FM_STRREF: i32 = 23;
const FM_STRLISTREF: i32 = 24;
const FM_GLOBAL: i32 = 1000;
const FM_SCENE: i32 = 1010;
const FM_CALL: i32 = 1020;

const OWNER_USER_PROP: i32 = 127;
const OWNER_USER_CMD: i32 = 126;
const OWNER_CALL_PROP: i32 = 125;

const CD_NL: u8 = 0x01;
const CD_PUSH: u8 = 0x02;
const CD_POP: u8 = 0x03;
const CD_COPY: u8 = 0x04;
const CD_PROPERTY: u8 = 0x05;
const CD_COPY_ELM: u8 = 0x06;
const CD_DEC_PROP: u8 = 0x07;
const CD_ELM_POINT: u8 = 0x08;
const CD_ARG: u8 = 0x09;
const CD_GOTO: u8 = 0x10;
const CD_GOTO_TRUE: u8 = 0x11;
const CD_GOTO_FALSE: u8 = 0x12;
const CD_GOSUB: u8 = 0x13;
const CD_GOSUBSTR: u8 = 0x14;
const CD_RETURN: u8 = 0x15;
const CD_EOF: u8 = 0x16;
const CD_ASSIGN: u8 = 0x20;
const CD_OPERATE_1: u8 = 0x21;
const CD_OPERATE_2: u8 = 0x22;
const CD_COMMAND: u8 = 0x30;
const CD_TEXT: u8 = 0x31;
const CD_NAME: u8 = 0x32;
const CD_SEL_BLOCK_START: u8 = 0x33;
const CD_SEL_BLOCK_END: u8 = 0x34;

#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    /// Contents of project/global `.inc` files, in project order.
    pub global_inc: Vec<String>,
}

#[derive(Debug, Clone)]
struct PropertySymbol {
    name: String,
    form: i32,
    size: i32,
    id: i32,
    scene_local: bool,
}

#[derive(Debug, Clone)]
struct CommandSymbol {
    name: String,
    form: i32,
    args: Vec<i32>,
    id: i32,
    scene_local: bool,
}

#[derive(Debug, Clone)]
struct CallProperty {
    name: String,
    form: i32,
    index: i32,
}

#[derive(Debug, Clone)]
struct ExprInfo {
    form: i32,
    list_forms: Vec<i32>,
}

#[derive(Debug, Clone)]
struct ElementInfo {
    kind: ElementKind,
    parent: i32,
    form: i32,
    code: i32,
    overloads: Vec<Overload>,
    named: Vec<ArgDef>,
}

pub fn compile(source: &str, options: &CompileOptions) -> Result<Vec<u8>> {
    let defs = DefinitionTable::new()?;
    let mut global = IncDefinitions::default();
    for text in &options.global_inc {
        parse_inc_step_1(text, &mut global)?;
    }
    parse_inc_step_2(&mut global, &defs)?;
    let first = preprocess(source, global.defined_names())?;
    let local = parse_local_inc(&first.local_inc, &global, &defs)?;
    let mut replacements = global.replacements.clone();
    replacements.extend(local.replacements.clone());
    let replaced = apply_replacements(&first.script, &replacements)?;
    let tokens = lex(&replaced)?;
    let program = Parser::new(&tokens, &defs).parse()?;
    let mut writer = Compiler::new(&defs, global, local, &tokens, &program)?;
    writer.compile_program(&program)?;
    writer.finish()
}

struct Compiler<'a> {
    defs: &'a DefinitionTable,
    code: Vec<u8>,
    strings: Vec<String>,
    labels: Vec<i32>,
    label_ids: HashMap<String, usize>,
    defined_labels: HashSet<usize>,
    z_labels: Vec<i32>,
    cmd_labels: Vec<(i32, i32)>,
    properties: Vec<PropertySymbol>,
    inc_property_count: usize,
    commands: Vec<CommandSymbol>,
    inc_command_count: usize,
    scene_command_offsets: Vec<i32>,
    call_properties: Vec<CallProperty>,
    current_call: Vec<CallProperty>,
    loops: Vec<(usize, usize)>,
    names: Vec<i32>,
    name_values: HashSet<String>,
    read_flags: Vec<i32>,
    msg_block: ElementInfo,
    current_line: i32,
}

impl<'a> Compiler<'a> {
    fn new(
        defs: &'a DefinitionTable,
        global: IncDefinitions,
        local: IncDefinitions,
        tokens: &[crate::lexer::Token],
        program: &Program,
    ) -> Result<Self> {
        let mut properties = Vec::new();
        for IncProperty { name, form, size } in global.properties {
            let id = properties.len() as i32;
            properties.push(PropertySymbol {
                name,
                form,
                size,
                id,
                scene_local: false,
            });
        }
        let inc_property_count = properties.len();
        for IncProperty { name, form, size } in local.properties {
            if properties.iter().any(|v| v.name == name) {
                bail!("duplicate property {name}");
            }
            let id = properties.len() as i32;
            properties.push(PropertySymbol {
                name,
                form,
                size,
                id,
                scene_local: true,
            });
        }
        let mut commands = Vec::new();
        for command in global.commands {
            push_command(&mut commands, command, false)?;
        }
        let inc_command_count = commands.len();
        for command in local.commands {
            push_command(&mut commands, command, true)?;
        }
        register_command_defs(&mut commands, program)?;
        let scene_command_offsets = vec![0; commands.len() - inc_command_count];

        let mut label_ids = HashMap::new();
        for token in tokens {
            let name = match &token.kind {
                TokenKind::Label(name) => Some(name.clone()),
                TokenKind::ZLabel(number) => Some(format!("z{number}")),
                _ => None,
            };
            if let Some(name) = name {
                let next = label_ids.len();
                label_ids.entry(name).or_insert(next);
            }
        }
        let labels = vec![0; label_ids.len()];
        let builtin = defs
            .element(FM_GLOBAL, "msg_block")
            .context("builtin MSG_BLOCK is absent")?;
        let msg_block = ElementInfo {
            kind: builtin.kind,
            parent: builtin.parent_form,
            form: builtin.form,
            code: builtin.code,
            overloads: builtin.overloads.clone(),
            named: builtin.named.clone(),
        };
        Ok(Self {
            defs,
            code: Vec::new(),
            strings: Vec::new(),
            labels,
            label_ids,
            defined_labels: HashSet::new(),
            z_labels: vec![0; 1000],
            cmd_labels: Vec::new(),
            properties,
            inc_property_count,
            commands,
            inc_command_count,
            scene_command_offsets,
            call_properties: Vec::new(),
            current_call: Vec::new(),
            loops: Vec::new(),
            names: Vec::new(),
            name_values: HashSet::new(),
            read_flags: Vec::new(),
            msg_block,
            current_line: 0,
        })
    }

    fn compile_program(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            self.statement(statement)?;
        }
        self.code.push(CD_EOF);
        for (name, &id) in &self.label_ids {
            if !self.defined_labels.contains(&id) {
                bail!("label #{name} is referenced but never defined");
            }
        }
        if let Some(&z0) = self.label_ids.get("z0")
            && !self.defined_labels.contains(&z0)
        {
            bail!("#z00 is referenced but never defined");
        }
        Ok(())
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        self.current_line = statement.line;
        self.code.push(CD_NL);
        push_i32(&mut self.code, statement.line);
        let selection = statement_contains_selection(&statement.kind);
        if selection {
            self.code.push(CD_SEL_BLOCK_START);
        }
        match &statement.kind {
            StatementKind::Label(name) => self.define_label_name(name)?,
            StatementKind::ZLabel(number) => {
                if *number >= 1000 {
                    bail!("line {}: z-label must be below 1000", statement.line);
                }
                self.define_label_name(&format!("z{number}"))?;
                self.z_labels[*number] = self.code.len() as i32;
            }
            StatementKind::Property(property) => bail!(
                "line {}: property {} declared outside a command",
                statement.line,
                property.name
            ),
            StatementKind::CommandDef(command) => self.command_definition(command)?,
            StatementKind::Goto { kind, label, args } => {
                let info = self.goto_expr(*kind, label, args)?;
                if !matches!(kind, GotoKind::Goto) {
                    self.code.push(CD_POP);
                    push_i32(&mut self.code, info.form);
                }
            }
            StatementKind::Return(value) => {
                let form = if let Some(value) = value {
                    Some(deref(self.expression(value, true)?.form))
                } else {
                    None
                };
                self.code.push(CD_RETURN);
                push_i32(&mut self.code, form.is_some() as i32);
                if let Some(form) = form {
                    push_i32(&mut self.code, form);
                }
            }
            StatementKind::If(branches) => self.if_statement(branches)?,
            StatementKind::For {
                init,
                condition,
                step,
                body,
            } => self.for_statement(init, condition, step, body)?,
            StatementKind::While { condition, body } => self.while_statement(condition, body)?,
            StatementKind::Continue => {
                let label = self.loops.last().context("continue outside a loop")?.0;
                self.jump(CD_GOTO, label);
            }
            StatementKind::Break => {
                let label = self.loops.last().context("break outside a loop")?.1;
                self.jump(CD_GOTO, label);
            }
            StatementKind::Switch {
                value,
                cases,
                default,
            } => self.switch_statement(value, cases, default)?,
            StatementKind::Assign { left, op, right } => self.assignment(left, *op, right)?,
            StatementKind::Command(expr) => {
                let form = self.expression(expr, true)?.form;
                self.code.push(CD_POP);
                push_i32(&mut self.code, form);
            }
            StatementKind::Text(text) => {
                self.push_msg_block()?;
                let id = self.intern(text);
                self.push_literal(FM_STR, id);
                self.code.push(CD_TEXT);
                push_i32(&mut self.code, self.read_flags.len() as i32);
                self.read_flags.push(statement.line);
            }
            StatementKind::Name(name) => {
                self.push_msg_block()?;
                let id = self.intern(name);
                self.push_literal(FM_STR, id);
                self.code.push(CD_NAME);
                if self.name_values.insert(name.clone()) {
                    self.names.push(id);
                }
            }
        }
        if selection {
            self.code.push(CD_SEL_BLOCK_END);
        }
        Ok(())
    }

    fn command_definition(&mut self, command: &CommandDecl) -> Result<()> {
        let symbol = self
            .commands
            .iter()
            .find(|v| v.name == clean_symbol(&command.name))
            .cloned()
            .ok_or_else(|| anyhow!("command {} was not registered", command.name))?;
        let actual: Vec<i32> = command.args.iter().map(|p| p.form).collect();
        if symbol.form != command.form || symbol.args != actual {
            bail!(
                "definition of {} does not match its inc declaration",
                symbol.name
            );
        }
        let end = self.new_label();
        self.jump(CD_GOTO, end);
        let offset = self.code.len() as i32;
        self.cmd_labels.push((symbol.id, offset));
        if symbol.scene_local {
            self.scene_command_offsets[symbol.id as usize - self.inc_command_count] = offset;
        }

        self.current_call.clear();
        for (index, property) in command.args.iter().enumerate() {
            let total_id = self.call_properties.len() as i32;
            let call = CallProperty {
                name: clean_symbol(&property.name),
                form: property.form,
                index: index as i32,
            };
            if matches!(property.form, FM_INTLIST | FM_STRLIST) {
                if let Some(size) = &property.size {
                    let size = self.expression(size, true)?;
                    require_int(size.form, "property size")?;
                } else {
                    self.push_literal(FM_INT, 0);
                }
            }
            self.code.push(CD_DEC_PROP);
            push_i32(&mut self.code, property.form);
            push_i32(&mut self.code, total_id);
            self.call_properties.push(call.clone());
            self.current_call.push(call);
        }
        self.code.push(CD_ARG);
        for statement in &command.body {
            self.statement(statement)?;
        }
        self.code.push(CD_RETURN);
        push_i32(&mut self.code, 0);
        self.set_label(end);
        self.current_call.clear();
        Ok(())
    }

    fn if_statement(&mut self, branches: &[(Option<Expr>, Vec<Statement>)]) -> Result<()> {
        let end = self.new_label();
        for (condition, body) in branches {
            if let Some(condition) = condition {
                if expr_contains_selection(condition) {
                    bail!("selection command cannot be used in an if condition");
                }
                let next = self.new_label();
                let info = self.expression(condition, true)?;
                require_int(info.form, "if condition")?;
                self.jump(CD_GOTO_FALSE, next);
                for statement in body {
                    self.statement(statement)?;
                }
                self.jump(CD_GOTO, end);
                self.set_label(next);
            } else {
                for statement in body {
                    self.statement(statement)?;
                }
            }
        }
        self.set_label(end);
        Ok(())
    }

    fn for_statement(
        &mut self,
        init: &[Statement],
        condition: &Expr,
        step: &[Statement],
        body: &[Statement],
    ) -> Result<()> {
        if expr_contains_selection(condition) {
            bail!("selection command cannot be used in a for condition");
        }
        let check = self.new_label();
        let loop_label = self.new_label();
        let out = self.new_label();
        self.loops.push((loop_label, out));
        for statement in init {
            self.statement(statement)?;
        }
        self.jump(CD_GOTO, check);
        self.set_label(loop_label);
        for statement in step {
            self.statement(statement)?;
        }
        self.set_label(check);
        let info = self.expression(condition, true)?;
        require_int(info.form, "for condition")?;
        self.jump(CD_GOTO_FALSE, out);
        for statement in body {
            self.statement(statement)?;
        }
        self.jump(CD_GOTO, loop_label);
        self.set_label(out);
        self.loops.pop();
        Ok(())
    }

    fn while_statement(&mut self, condition: &Expr, body: &[Statement]) -> Result<()> {
        if expr_contains_selection(condition) {
            bail!("selection command cannot be used in a while condition");
        }
        let loop_label = self.new_label();
        let out = self.new_label();
        self.loops.push((loop_label, out));
        self.set_label(loop_label);
        let info = self.expression(condition, true)?;
        require_int(info.form, "while condition")?;
        self.jump(CD_GOTO_FALSE, out);
        for statement in body {
            self.statement(statement)?;
        }
        self.jump(CD_GOTO, loop_label);
        self.set_label(out);
        self.loops.pop();
        Ok(())
    }

    fn switch_statement(
        &mut self,
        value: &Expr,
        cases: &[(Expr, Vec<Statement>)],
        default: &[Statement],
    ) -> Result<()> {
        if expr_contains_selection(value)
            || cases
                .iter()
                .any(|(case_value, _)| expr_contains_selection(case_value))
        {
            bail!("selection command cannot be used in switch/case expressions");
        }
        let out = self.new_label();
        let case_labels: Vec<_> = cases.iter().map(|_| self.new_label()).collect();
        let default_label = if default.is_empty() {
            None
        } else {
            Some(self.new_label())
        };
        let value_form = deref(self.expression(value, true)?.form);
        for ((case, _), &label) in cases.iter().zip(&case_labels) {
            self.code.push(CD_COPY);
            push_i32(&mut self.code, value_form);
            let case_form = deref(self.expression(case, true)?.form);
            if case_form != value_form {
                bail!("switch and case forms differ");
            }
            self.code.push(CD_OPERATE_2);
            push_i32(&mut self.code, value_form);
            push_i32(&mut self.code, case_form);
            self.code.push(0x10);
            self.jump(CD_GOTO_TRUE, label);
        }
        self.code.push(CD_POP);
        push_i32(&mut self.code, value_form);
        self.jump(CD_GOTO, default_label.unwrap_or(out));
        for ((_, body), &label) in cases.iter().zip(&case_labels) {
            self.set_label(label);
            self.code.push(CD_POP);
            push_i32(&mut self.code, value_form);
            for statement in body {
                self.statement(statement)?;
            }
            self.jump(CD_GOTO, out);
        }
        if let Some(label) = default_label {
            self.set_label(label);
            for statement in default {
                self.statement(statement)?;
            }
            self.jump(CD_GOTO, out);
        }
        self.set_label(out);
        Ok(())
    }

    fn assignment(&mut self, left: &ElementExpr, op: u8, right: &Expr) -> Result<()> {
        let left_info = self.element_expression(left, false)?;
        if !is_reference(left_info.form) {
            bail!("assignment target is not a reference");
        }
        if op != 0 {
            self.code.push(CD_COPY_ELM);
            self.code.push(CD_PROPERTY);
        }
        let right_info = self.expression(right, true)?;
        let result_form = if op == 0 {
            deref(right_info.form)
        } else {
            let form = operation_result(left_info.form, right_info.form, op)?;
            self.code.push(CD_OPERATE_2);
            push_i32(&mut self.code, deref(left_info.form));
            push_i32(&mut self.code, deref(right_info.form));
            self.code.push(op);
            form
        };
        if deref(left_info.form) != deref(result_form) {
            bail!("assignment form mismatch");
        }
        self.code.push(CD_ASSIGN);
        push_i32(&mut self.code, left_info.form);
        push_i32(&mut self.code, deref(result_form));
        push_i32(&mut self.code, 1);
        Ok(())
    }

    fn expression(&mut self, expr: &Expr, need_value: bool) -> Result<ExprInfo> {
        match expr {
            Expr::Int(value) => {
                if !need_value {
                    bail!("integer is not a reference");
                }
                self.push_literal(FM_INT, *value);
                Ok(simple(FM_INT))
            }
            Expr::Str(value) => {
                if !need_value {
                    bail!("string is not a reference");
                }
                let id = self.intern(value);
                self.push_literal(FM_STR, id);
                Ok(simple(FM_STR))
            }
            Expr::List(values) => {
                if !need_value {
                    bail!("list is not a reference");
                }
                let mut forms = Vec::new();
                for value in values {
                    forms.push(self.expression(value, true)?.form);
                }
                Ok(ExprInfo {
                    form: FM_LIST,
                    list_forms: forms,
                })
            }
            Expr::Element(element) => self.element_expression(element, need_value),
            Expr::Goto { kind, label, args } => {
                if !need_value {
                    bail!("gosub result is not a reference");
                }
                self.goto_expr(*kind, label, args)
            }
            Expr::Unary { op, expr } => {
                if !need_value {
                    bail!("operator result is not a reference");
                }
                let form = self.expression(expr, true)?.form;
                require_int(form, "unary operator")?;
                self.code.push(CD_OPERATE_1);
                push_i32(&mut self.code, FM_INT);
                self.code.push(*op);
                Ok(simple(FM_INT))
            }
            Expr::Binary { op, left, right } => {
                if !need_value {
                    bail!("operator result is not a reference");
                }
                let left = self.expression(left, true)?;
                let right = self.expression(right, true)?;
                let form = operation_result(left.form, right.form, *op)?;
                self.code.push(CD_OPERATE_2);
                push_i32(&mut self.code, deref(left.form));
                // BS.cpp serializes exp_1's form in both operand slots. This
                // looks accidental for `str * int`, but it is the original
                // bytecode writer's observable behavior.
                push_i32(&mut self.code, deref(left.form));
                self.code.push(*op);
                Ok(simple(form))
            }
        }
    }

    fn goto_expr(
        &mut self,
        kind: GotoKind,
        label: &LabelRef,
        args: &[Argument],
    ) -> Result<ExprInfo> {
        if args.iter().any(|arg| expr_contains_selection(&arg.value)) {
            bail!("selection command cannot be used in gosub arguments");
        }
        let label = self.label_ref(label)?;
        if matches!(kind, GotoKind::Goto) {
            if !args.is_empty() {
                bail!("goto cannot have arguments");
            }
            self.jump(CD_GOTO, label);
            return Ok(simple(FM_VOID));
        }
        let mut forms = Vec::new();
        for arg in args {
            if arg.name.is_some() {
                bail!("gosub does not accept named arguments");
            }
            forms.push(deref(self.expression(&arg.value, true)?.form));
        }
        self.code.push(if matches!(kind, GotoKind::Gosub) {
            CD_GOSUB
        } else {
            CD_GOSUBSTR
        });
        push_i32(&mut self.code, label as i32);
        push_i32(&mut self.code, forms.len() as i32);
        for form in forms {
            push_i32(&mut self.code, form);
        }
        Ok(simple(if matches!(kind, GotoKind::Gosub) {
            FM_INT
        } else {
            FM_STR
        }))
    }

    fn element_expression(&mut self, element: &ElementExpr, need_value: bool) -> Result<ExprInfo> {
        let code_checkpoint = self.code.len();
        self.code.push(CD_ELM_POINT);
        let root_name = match element.parts.first() {
            Some(ElementPart::Name { name, .. }) => clean_symbol(name),
            _ => bail!("element chain cannot begin with an index"),
        };
        let mut parent;
        let mut current;
        if let Some(call) = self
            .current_call
            .iter()
            .find(|v| v.name == root_name)
            .cloned()
        {
            parent = FM_CALL;
            self.push_literal(
                FM_INT,
                self.defs
                    .element(FM_GLOBAL, "cur_call")
                    .context("CUR_CALL builtin missing")?
                    .code,
            );
            current = ElementInfo {
                kind: ElementKind::Property,
                parent,
                form: call.form,
                code: element_code(OWNER_CALL_PROP, call.index),
                overloads: vec![],
                named: vec![],
            };
        } else if let Some(command) = self.commands.iter().find(|v| v.name == root_name).cloned() {
            parent = if command.scene_local {
                FM_SCENE
            } else {
                FM_GLOBAL
            };
            current = ElementInfo {
                kind: ElementKind::Command,
                parent,
                form: command.form,
                code: element_code(OWNER_USER_CMD, command.id),
                overloads: vec![Overload {
                    id: 0,
                    args: command
                        .args
                        .iter()
                        .map(|&form| ArgDef {
                            id: 0,
                            name: None,
                            form,
                            default_int: None,
                        })
                        .collect(),
                }],
                named: vec![],
            };
        } else if let Some(property) = self
            .properties
            .iter()
            .find(|v| v.name == root_name)
            .cloned()
        {
            parent = if property.scene_local {
                FM_SCENE
            } else {
                FM_GLOBAL
            };
            current = ElementInfo {
                kind: ElementKind::Property,
                parent,
                form: property.form,
                code: element_code(OWNER_USER_PROP, property.id),
                overloads: vec![],
                named: vec![],
            };
        } else {
            let (p, builtin) = [FM_GLOBAL, FM_SCENE, FM_CALL]
                .into_iter()
                .find_map(|p| self.defs.element(p, &root_name).map(|v| (p, v)))
                .ok_or_else(|| anyhow!("unknown element {root_name}"))?;
            parent = p;
            current = from_builtin(builtin);
        }

        let mut last_kind = current.kind;
        let mut final_form = FM_VOID;
        let mut final_parent = current.parent;
        let mut final_name = root_name.clone();
        for (index, part) in element.parts.iter().enumerate() {
            if index > 0 {
                current = match part {
                    ElementPart::Name { name, .. } => from_builtin(
                        self.defs
                            .element(parent, name)
                            .ok_or_else(|| anyhow!("unknown element {} for form {parent}", name))?,
                    ),
                    ElementPart::Array(_) => from_builtin(
                        self.defs
                            .element(parent, "array")
                            .ok_or_else(|| anyhow!("form {parent} is not indexable"))?,
                    ),
                };
            }
            match part {
                ElementPart::Name { name, args } => {
                    final_name = clean_symbol(name);
                    self.push_literal(FM_INT, current.code);
                    if current.kind == ElementKind::Command {
                        let args = args.as_deref().unwrap_or(&[]);
                        let matched = self.match_arguments(&current, args)?;
                        self.emit_command(&current, args, matched)?;
                    } else if args.as_ref().is_some_and(|v| !v.is_empty()) {
                        bail!("property does not accept arguments");
                    }
                }
                ElementPart::Array(index) => {
                    if expr_contains_selection(index) {
                        bail!("selection command cannot be used as an array index");
                    }
                    self.push_literal(FM_INT, -1);
                    let info = self.expression(index, true)?;
                    require_int(info.form, "array index")?;
                }
            }
            final_form = current.form;
            last_kind = current.kind;
            final_parent = current.parent;
            parent = final_form;
            if (current.code >> 24) & 0xff == OWNER_CALL_PROP && is_reference(final_form) {
                self.code.push(CD_PROPERTY);
            }
        }
        if last_kind == ElementKind::Command && needs_message_block(final_parent, &final_name) {
            let element_code = self.code.split_off(code_checkpoint);
            self.push_msg_block()?;
            self.code.extend_from_slice(&element_code);
        }
        if last_kind == ElementKind::Command && needs_read_flag(final_parent, &final_name) {
            push_i32(&mut self.code, self.read_flags.len() as i32);
            self.read_flags.push(self.current_line);
        }
        if last_kind == ElementKind::Property {
            final_form = reference_form(final_form);
        }
        if need_value && is_reference(final_form) {
            self.code.push(CD_PROPERTY);
        }
        Ok(simple(final_form))
    }

    fn match_arguments(&mut self, element: &ElementInfo, args: &[Argument]) -> Result<Overload> {
        if args.iter().any(|arg| expr_contains_selection(&arg.value)) {
            bail!("selection command cannot be nested inside command arguments");
        }
        let positional_count = args.iter().take_while(|a| a.name.is_none()).count();
        if args[..positional_count].iter().any(|a| a.name.is_some())
            || args[positional_count..].iter().any(|a| a.name.is_none())
        {
            bail!("positional arguments must precede named arguments");
        }
        for overload in &element.overloads {
            let mut ok = true;
            let mut spec_index = 0;
            for arg in &args[..positional_count] {
                let Some(spec) = overload.args.get(spec_index) else {
                    ok = false;
                    break;
                };
                if spec.form == -2 || spec.form == -3 {
                    continue;
                }
                let form = infer_form(&arg.value, self)?;
                if !form_matches(spec.form, form) {
                    ok = false;
                    break;
                }
                spec_index += 1;
            }
            if !ok {
                continue;
            }
            for spec in overload.args.iter().skip(spec_index) {
                if !matches!(spec.form, -2 | -3) && spec.default_int.is_none() {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
            for arg in &args[positional_count..] {
                let Some(name) = &arg.name else {
                    unreachable!()
                };
                let Some(spec) = element
                    .named
                    .iter()
                    .find(|v| v.name.as_deref() == Some(name.as_str()))
                else {
                    ok = false;
                    break;
                };
                if !form_matches(spec.form, infer_form(&arg.value, self)?) {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Ok(overload.clone());
            }
        }
        bail!("no argument overload matches")
    }

    fn emit_command(
        &mut self,
        element: &ElementInfo,
        args: &[Argument],
        overload: Overload,
    ) -> Result<()> {
        let positional_count = args.iter().take_while(|a| a.name.is_none()).count();
        let mut emitted_forms: Vec<ExprInfo> = Vec::new();
        let mut spec_index = 0;
        for arg in &args[..positional_count] {
            let spec = overload.args.get(spec_index).context("argument overflow")?;
            let need_value = matches!(spec.form, -2 | FM_LIST) || !is_reference(spec.form);
            let info = self.expression(&arg.value, need_value)?;
            let form = if spec.form == -2 {
                deref(info.form)
            } else if spec.form == -3 {
                info.form
            } else {
                spec.form
            };
            emitted_forms.push(ExprInfo {
                form,
                list_forms: info.list_forms,
            });
            if !matches!(spec.form, -2 | -3) {
                spec_index += 1;
            }
        }
        for spec in overload.args.iter().skip(spec_index) {
            if matches!(spec.form, -2 | -3) {
                break;
            }
            let value = spec.default_int.context("missing required argument")?;
            self.push_literal(spec.form, value);
            emitted_forms.push(simple(spec.form));
        }
        let mut named_ids = Vec::new();
        for arg in &args[positional_count..] {
            let spec = element
                .named
                .iter()
                .find(|v| v.name.as_deref() == arg.name.as_deref())
                .context("unknown named argument")?;
            let need_value = matches!(spec.form, -2 | FM_LIST) || !is_reference(spec.form);
            let info = self.expression(&arg.value, need_value)?;
            emitted_forms.push(ExprInfo {
                form: spec.form,
                list_forms: info.list_forms,
            });
            named_ids.push(spec.id);
        }
        self.code.push(CD_COMMAND);
        push_i32(&mut self.code, overload.id);
        push_i32(&mut self.code, emitted_forms.len() as i32);
        for info in emitted_forms.iter().rev() {
            push_i32(&mut self.code, info.form);
            if info.form == FM_LIST {
                push_i32(&mut self.code, info.list_forms.len() as i32);
                for form in info.list_forms.iter().rev() {
                    push_i32(&mut self.code, deref(*form));
                }
            }
        }
        push_i32(&mut self.code, named_ids.len() as i32);
        for id in named_ids.iter().rev() {
            push_i32(&mut self.code, *id);
        }
        push_i32(&mut self.code, element.form);
        Ok(())
    }

    fn push_msg_block(&mut self) -> Result<()> {
        self.code.push(CD_ELM_POINT);
        self.push_literal(FM_INT, self.msg_block.code);
        self.code.push(CD_COMMAND);
        push_i32(&mut self.code, 0);
        push_i32(&mut self.code, 0);
        push_i32(&mut self.code, 0);
        push_i32(&mut self.code, FM_VOID);
        Ok(())
    }

    fn intern(&mut self, value: &str) -> i32 {
        let id = self.strings.len() as i32;
        self.strings.push(value.to_owned());
        id
    }
    fn push_literal(&mut self, form: i32, value: i32) {
        self.code.push(CD_PUSH);
        push_i32(&mut self.code, form);
        push_i32(&mut self.code, value);
    }
    fn new_label(&mut self) -> usize {
        let id = self.labels.len();
        self.labels.push(0);
        id
    }
    fn set_label(&mut self, id: usize) {
        self.labels[id] = self.code.len() as i32;
        self.defined_labels.insert(id);
    }
    fn jump(&mut self, opcode: u8, label: usize) {
        self.code.push(opcode);
        push_i32(&mut self.code, label as i32);
    }
    fn define_label_name(&mut self, name: &str) -> Result<()> {
        let id = *self
            .label_ids
            .get(name)
            .ok_or_else(|| anyhow!("internal label registration error for {name}"))?;
        if self.defined_labels.contains(&id) {
            bail!("duplicate label #{name}");
        }
        self.set_label(id);
        Ok(())
    }
    fn label_ref(&self, label: &LabelRef) -> Result<usize> {
        let name = match label {
            LabelRef::Named(name) => name.clone(),
            LabelRef::Z(number) => format!("z{number}"),
        };
        self.label_ids
            .get(&name)
            .copied()
            .ok_or_else(|| anyhow!("unknown label #{name}"))
    }

    fn finish(mut self) -> Result<Vec<u8>> {
        self.strings.push(String::new()); // The original lexer always appends its dummy string.
        let mut output = vec![0; 132];
        let mut header = [0i32; 33];
        header[0] = 132;
        header[3] = output.len() as i32;
        header[4] = self.strings.len() as i32;
        let mut unit_offset = 0;
        for value in &self.strings {
            push_i32(&mut output, unit_offset);
            push_i32(&mut output, value.encode_utf16().count() as i32);
            unit_offset += value.encode_utf16().count() as i32;
        }
        header[5] = output.len() as i32;
        header[6] = self.strings.len() as i32;
        for (id, value) in self.strings.iter().enumerate() {
            for unit in value.encode_utf16() {
                output
                    .extend_from_slice(&(unit ^ (28807u16.wrapping_mul(id as u16))).to_le_bytes());
            }
        }
        header[1] = output.len() as i32;
        header[2] = self.code.len() as i32;
        output.extend_from_slice(&self.code);
        append_i32_list(&mut output, &mut header, 7, 8, &self.labels);
        append_i32_list(&mut output, &mut header, 9, 10, &self.z_labels);
        header[11] = output.len() as i32;
        header[12] = self.cmd_labels.len() as i32;
        for (id, ofs) in &self.cmd_labels {
            push_i32(&mut output, *id);
            push_i32(&mut output, *ofs);
        }
        let scene_props: Vec<_> = self
            .properties
            .iter()
            .skip(self.inc_property_count)
            .collect();
        header[13] = output.len() as i32;
        header[14] = scene_props.len() as i32;
        for prop in &scene_props {
            push_i32(&mut output, prop.form);
            push_i32(&mut output, prop.size);
        }
        let prop_names: Vec<_> = scene_props.iter().map(|p| p.name.as_str()).collect();
        append_names(&mut output, &mut header, 15, 16, 17, 18, &prop_names);
        header[19] = output.len() as i32;
        header[20] = self.scene_command_offsets.len() as i32;
        for value in &self.scene_command_offsets {
            push_i32(&mut output, *value);
        }
        let command_names: Vec<_> = self
            .commands
            .iter()
            .skip(self.inc_command_count)
            .map(|c| c.name.as_str())
            .collect();
        append_names(&mut output, &mut header, 21, 22, 23, 24, &command_names);
        let call_names: Vec<_> = self
            .call_properties
            .iter()
            .map(|p| p.name.as_str())
            .collect();
        append_names(&mut output, &mut header, 25, 26, 27, 28, &call_names);
        append_i32_list(&mut output, &mut header, 29, 30, &self.names);
        append_i32_list(&mut output, &mut header, 31, 32, &self.read_flags);
        for (index, value) in header.into_iter().enumerate() {
            output[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        Ok(output)
    }
}

fn push_command(
    commands: &mut Vec<CommandSymbol>,
    command: IncCommand,
    scene_local: bool,
) -> Result<()> {
    if commands.iter().any(|v| v.name == command.name) {
        bail!("duplicate command {}", command.name);
    }
    let id = commands.len() as i32;
    commands.push(CommandSymbol {
        name: command.name,
        form: command.form,
        args: command.args,
        id,
        scene_local,
    });
    Ok(())
}

fn register_command_defs(commands: &mut Vec<CommandSymbol>, program: &Program) -> Result<()> {
    fn walk(
        commands: &mut Vec<CommandSymbol>,
        statements: &[Statement],
        defined: &mut HashSet<String>,
    ) -> Result<()> {
        for statement in statements {
            if let StatementKind::CommandDef(def) = &statement.kind {
                let name = clean_symbol(&def.name);
                if !defined.insert(name.clone()) {
                    bail!("command {name} is defined more than once");
                }
                let args = def.args.iter().map(|p| p.form).collect::<Vec<_>>();
                if let Some(existing) = commands.iter().find(|v| v.name == name) {
                    if existing.form != def.form || existing.args != args {
                        bail!("definition of {name} conflicts with inc declaration");
                    }
                } else {
                    let id = commands.len() as i32;
                    commands.push(CommandSymbol {
                        name,
                        form: def.form,
                        args,
                        id,
                        scene_local: true,
                    });
                }
            }
        }
        Ok(())
    }
    walk(commands, &program.statements, &mut HashSet::new())
}

fn infer_form(expr: &Expr, compiler: &mut Compiler<'_>) -> Result<i32> {
    let checkpoint = compiler.code.len();
    let strings = compiler.strings.len();
    let result = compiler.expression(expr, true).map(|v| v.form);
    compiler.code.truncate(checkpoint);
    compiler.strings.truncate(strings);
    result
}

fn from_builtin(value: &ResolvedElement) -> ElementInfo {
    ElementInfo {
        kind: value.kind,
        parent: value.parent_form,
        form: value.form,
        code: value.code,
        overloads: value.overloads.clone(),
        named: value.named.clone(),
    }
}
fn simple(form: i32) -> ExprInfo {
    ExprInfo {
        form,
        list_forms: Vec::new(),
    }
}
fn clean_symbol(name: &str) -> String {
    name.trim_start_matches(['$', '@']).to_ascii_lowercase()
}
fn element_code(owner: i32, index: i32) -> i32 {
    (owner << 24) | (index & 0xffff)
}
fn deref(form: i32) -> i32 {
    match form {
        FM_INTREF => FM_INT,
        FM_STRREF => FM_STR,
        FM_INTLISTREF => FM_INTLIST,
        FM_STRLISTREF => FM_STRLIST,
        _ => form,
    }
}
fn reference_form(form: i32) -> i32 {
    match form {
        FM_INT => FM_INTREF,
        FM_STR => FM_STRREF,
        FM_INTLIST => FM_INTLISTREF,
        FM_STRLIST => FM_STRLISTREF,
        _ => form,
    }
}
fn is_reference(form: i32) -> bool {
    !matches!(form, FM_VOID | FM_INT | FM_STR | FM_INTLIST | FM_STRLIST)
}
fn require_int(form: i32, context: &str) -> Result<()> {
    if matches!(form, FM_INT | FM_INTREF) {
        Ok(())
    } else {
        bail!("{context} must be int")
    }
}
fn form_matches(expected: i32, actual: i32) -> bool {
    expected == actual
        || expected == deref(actual)
        || matches!(expected, -2)
        || (expected == -3 && is_reference(actual))
}
fn operation_result(left: i32, right: i32, op: u8) -> Result<i32> {
    let (left, right) = (deref(left), deref(right));
    if left == FM_INT && right == FM_INT {
        return Ok(FM_INT);
    }
    if left == FM_STR && right == FM_STR && matches!(op, 0x01) {
        return Ok(FM_STR);
    }
    if left == FM_STR && right == FM_STR && matches!(op, 0x10..=0x15) {
        return Ok(FM_INT);
    }
    if left == FM_STR && right == FM_INT && op == 0x03 {
        return Ok(FM_STR);
    }
    bail!("operator {op:#x} does not accept forms {left} and {right}")
}

fn statement_contains_selection(statement: &StatementKind) -> bool {
    match statement {
        StatementKind::Assign { right, .. } | StatementKind::Command(right) => {
            expr_contains_selection(right)
        }
        StatementKind::Return(Some(value)) => expr_contains_selection(value),
        _ => false,
    }
}

fn expr_contains_selection(expr: &Expr) -> bool {
    match expr {
        Expr::Element(element) => element.parts.iter().any(|part| match part {
            ElementPart::Name { name, args } => {
                matches!(
                    clean_symbol(name).as_str(),
                    "sel" | "sel_cancel" | "selmsg" | "selmsg_cancel" | "sel_image"
                ) || args
                    .as_ref()
                    .is_some_and(|args| args.iter().any(|arg| expr_contains_selection(&arg.value)))
            }
            ElementPart::Array(index) => expr_contains_selection(index),
        }),
        Expr::List(values) => values.iter().any(expr_contains_selection),
        Expr::Goto { args, .. } => args.iter().any(|arg| expr_contains_selection(&arg.value)),
        Expr::Unary { expr, .. } => expr_contains_selection(expr),
        Expr::Binary { left, right, .. } => {
            expr_contains_selection(left) || expr_contains_selection(right)
        }
        Expr::Int(_) | Expr::Str(_) => false,
    }
}

fn needs_message_block(parent: i32, name: &str) -> bool {
    matches!(parent, FM_GLOBAL | 1320)
        && matches!(
            name,
            "koe" | "set_face" | "set_namae" | "print" | "ruby" | "nl" | "nli"
        )
}

fn needs_read_flag(parent: i32, name: &str) -> bool {
    matches!(parent, FM_GLOBAL | 1320)
        && matches!(
            name,
            "print"
                | "koe"
                | "koe_play_wait"
                | "koe_play_wait_key"
                | "sel"
                | "sel_cancel"
                | "selmsg"
                | "selmsg_cancel"
                | "selbtn"
                | "selbtn_cancel"
                | "selbtn_start"
                | "sel_image"
        )
}

fn append_i32_list(
    output: &mut Vec<u8>,
    header: &mut [i32; 33],
    ofs: usize,
    cnt: usize,
    values: &[i32],
) {
    header[ofs] = output.len() as i32;
    header[cnt] = values.len() as i32;
    for value in values {
        push_i32(output, *value);
    }
}

fn append_names(
    output: &mut Vec<u8>,
    header: &mut [i32; 33],
    index_ofs: usize,
    index_cnt: usize,
    list_ofs: usize,
    list_cnt: usize,
    names: &[&str],
) {
    header[index_ofs] = output.len() as i32;
    header[index_cnt] = names.len() as i32;
    let mut offset = 0;
    for name in names {
        let units = name.encode_utf16().count() as i32;
        push_i32(output, offset);
        push_i32(output, units);
        offset += units;
    }
    header[list_ofs] = output.len() as i32;
    header[list_cnt] = names.len() as i32;
    for name in names {
        output.extend_from_slice(&utf16le(name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_minimal_scene() {
        let data = compile("#z00\n【真人】\nテスト\nr\n", &CompileOptions::default()).unwrap();
        assert_eq!(i32::from_le_bytes(data[0..4].try_into().unwrap()), 132);
        assert_eq!(i32::from_le_bytes(data[40..44].try_into().unwrap()), 1000);
    }

    #[test]
    fn compiles_user_command_and_assignment() {
        let data = compile("#inc_start\n#property $flag : int\n#inc_end\n#z00\n$flag = 1\ncommand $twice(property $x:int):int { return($x * 2) }\n", &CompileOptions::default()).unwrap();
        assert!(data.len() > 132);
    }
}
