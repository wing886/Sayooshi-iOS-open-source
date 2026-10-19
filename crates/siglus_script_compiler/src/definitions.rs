use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Property,
    Command,
}

#[derive(Debug, Clone, Copy)]
pub struct FormDef {
    pub name: &'static str,
    pub code: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct ElementDef {
    pub kind: ElementKind,
    pub parent: &'static str,
    pub form: &'static str,
    pub name: &'static str,
    pub code: i32,
    pub arg_spec: &'static str,
}

// A checked-in Rust table keeps the compiler self-contained. The original
// headers are semantic reference material, not build inputs or FFI bindings.
include!("generated_definitions.rs");

#[derive(Debug, Clone)]
pub struct ArgDef {
    pub id: i32,
    pub name: Option<String>,
    pub form: i32,
    pub default_int: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Overload {
    pub id: i32,
    pub args: Vec<ArgDef>,
}

#[derive(Debug, Clone)]
pub struct ResolvedElement {
    pub kind: ElementKind,
    pub parent_form: i32,
    pub form: i32,
    pub code: i32,
    pub overloads: Vec<Overload>,
    pub named: Vec<ArgDef>,
}

#[derive(Debug, Clone)]
pub struct DefinitionTable {
    pub forms: HashMap<String, i32>,
    by_parent_name: HashMap<(i32, String), ResolvedElement>,
}

impl DefinitionTable {
    pub fn new() -> Result<Self> {
        let forms: HashMap<String, i32> = FORMS.iter().map(|f| (f.name.into(), f.code)).collect();
        if forms.len() != GENERATED_FORM_COUNT || FORMS.len() != GENERATED_FORM_COUNT {
            bail!("generated form table lost an entry");
        }
        let mut by_parent_name = HashMap::new();
        for def in ELEMENTS {
            let parent_form = *forms
                .get(def.parent)
                .ok_or_else(|| anyhow!("missing parent form {}", def.parent))?;
            let form = *forms
                .get(def.form)
                .ok_or_else(|| anyhow!("missing return form {}", def.form))?;
            let (overloads, named) = parse_arg_spec(def.arg_spec, &forms)?;
            let previous = by_parent_name.insert(
                (parent_form, def.name.into()),
                ResolvedElement {
                    kind: def.kind,
                    parent_form,
                    form,
                    code: def.code,
                    overloads,
                    named,
                },
            );
            if previous.is_some() {
                bail!(
                    "generated element table contains duplicate {}.{}",
                    def.parent,
                    def.name
                );
            }
        }
        if by_parent_name.len() != GENERATED_ELEMENT_COUNT
            || ELEMENTS.len() != GENERATED_ELEMENT_COUNT
        {
            bail!("generated element table lost an entry");
        }
        Ok(Self {
            forms,
            by_parent_name,
        })
    }

    pub fn form(&self, name: &str) -> Result<i32> {
        self.forms
            .get(&name.to_lowercase())
            .copied()
            .ok_or_else(|| anyhow!("unknown form {name}"))
    }

    pub fn element(&self, parent: i32, name: &str) -> Option<&ResolvedElement> {
        self.by_parent_name.get(&(parent, name.to_lowercase()))
    }
}

fn parse_arg_spec(
    spec: &str,
    forms: &HashMap<String, i32>,
) -> Result<(Vec<Overload>, Vec<ArgDef>)> {
    let mut overloads = Vec::new();
    let mut named = Vec::new();
    for segment in spec.split(';').filter(|s| !s.is_empty()) {
        let (id, list) = segment
            .split_once(':')
            .ok_or_else(|| anyhow!("bad argument spec {segment}"))?;
        let id: i32 = id.parse()?;
        let mut args = Vec::new();
        if id == -1 {
            for raw in list.split(',').filter(|s| !s.is_empty()) {
                let mut fields = raw.split('=');
                let arg_id = fields.next().unwrap().parse()?;
                let name = fields
                    .next()
                    .ok_or_else(|| anyhow!("bad named argument {raw}"))?;
                let form_name = fields
                    .next()
                    .ok_or_else(|| anyhow!("bad named argument {raw}"))?;
                named.push(ArgDef {
                    id: arg_id,
                    name: Some(name.into()),
                    form: lookup_form(form_name, forms)?,
                    default_int: None,
                });
            }
        } else {
            for raw in list.split(',').filter(|s| !s.is_empty()) {
                if raw == "__args" {
                    args.push(ArgDef {
                        id: 0,
                        name: None,
                        form: -2,
                        default_int: None,
                    });
                    continue;
                }
                if raw == "__argsref" {
                    args.push(ArgDef {
                        id: 0,
                        name: None,
                        form: -3,
                        default_int: None,
                    });
                    continue;
                }
                let (name, default_int) = if let Some(open) = raw.find('(') {
                    let close = raw
                        .rfind(')')
                        .ok_or_else(|| anyhow!("bad default argument {raw}"))?;
                    (&raw[..open], Some(raw[open + 1..close].parse()?))
                } else {
                    (raw, None)
                };
                args.push(ArgDef {
                    id: 0,
                    name: None,
                    form: lookup_form(name, forms)?,
                    default_int,
                });
            }
            overloads.push(Overload { id, args });
        }
    }
    if overloads.is_empty() && !spec.is_empty() {
        bail!("element has no positional overload in {spec}");
    }
    Ok((overloads, named))
}

fn lookup_form(name: &str, forms: &HashMap<String, i32>) -> Result<i32> {
    forms
        .get(name)
        .copied()
        .ok_or_else(|| anyhow!("unknown form {name}"))
}

#[cfg(test)]
mod generated_table_tests {
    use super::*;

    #[test]
    fn generated_definition_counts_are_complete() {
        assert_eq!(GENERATED_FORM_COUNT, 83);
        assert_eq!(GENERATED_ELEMENT_COUNT, 1378);
        let table = DefinitionTable::new().unwrap();
        assert_eq!(table.forms.len(), GENERATED_FORM_COUNT);
        assert_eq!(table.by_parent_name.len(), GENERATED_ELEMENT_COUNT);
    }
}
