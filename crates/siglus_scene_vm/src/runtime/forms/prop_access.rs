use crate::runtime::globals::HashMap;
use crate::runtime::{CommandContext, Value};

pub const FM_VOID: i64 = 0;
pub const FM_INT: i64 = 10;
pub const FM_INTLIST: i64 = 11;
pub const FM_STR: i64 = 20;
pub const FM_STRLIST: i64 = 21;
pub const FM_LABEL: i64 = 30;

pub fn ret_form_is_string(ret_form: i64) -> bool {
    ret_form == FM_STR
}

pub fn ret_form_is_string_opt(ret_form: Option<i64>) -> bool {
    matches!(ret_form, Some(rf) if ret_form_is_string(rf))
}

pub fn parse_element_chain(form_id: u32, args: &[Value]) -> Option<(usize, &[i32])> {
    for (i, v) in args.iter().enumerate() {
        if let Value::Element(ch) = v
            && ch.first().copied() == Some(form_id as i32)
        {
            return Some((i, ch.as_slice()));
        }
    }
    None
}

pub fn parse_element_chain_ctx<'a>(
    ctx: &'a CommandContext,
    form_id: u32,
    args: &'a [Value],
) -> Option<(usize, &'a [i32])> {
    let vm_call = ctx.vm_call.as_ref()?;
    if vm_call.element.first().copied() == Some(form_id as i32) {
        return Some((args.len(), vm_call.element.as_slice()));
    }
    None
}

pub fn parse_current_element_chain<'a>(
    ctx: &'a CommandContext,
    args: &'a [Value],
) -> Option<(usize, &'a [i32])> {
    let vm_call = ctx.vm_call.as_ref()?;
    Some((args.len(), vm_call.element.as_slice()))
}

pub fn script_args(args: &[Value], chain_pos: usize) -> &[Value] {
    if chain_pos == args.len() {
        args
    } else if chain_pos > 1 {
        &args[1..chain_pos]
    } else {
        &[]
    }
}

pub fn current_op_from_chain(chain: &[i32]) -> Option<i32> {
    chain.get(1).copied()
}

pub fn current_op_from_ctx_or_args(ctx: &CommandContext, _args: &[Value]) -> Option<i32> {
    let vm_call = ctx.vm_call.as_ref()?;
    current_op_from_chain(&vm_call.element)
}

pub fn params_without_op<'a>(_ctx: &CommandContext, args: &'a [Value]) -> &'a [Value] {
    args
}

pub fn current_vm_meta(ctx: &CommandContext) -> (Option<i64>, Option<i64>) {
    ctx.vm_call
        .as_ref()
        .map(|m| (Some(m.al_id), Some(m.ret_form)))
        .unwrap_or((None, None))
}

pub fn infer_assign_and_ret_ctx(
    ctx: &CommandContext,
    _chain_pos: usize,
    args: &[Value],
) -> (Option<i64>, Option<i64>, Option<Value>) {
    let (meta_al, meta_ret) = current_vm_meta(ctx);
    let rhs = if meta_al == Some(1) {
        args.first().cloned()
    } else {
        None
    };
    (meta_al, meta_ret, rhs)
}

fn int_map(ctx: &mut CommandContext, form_id: u32) -> &mut HashMap<i32, i64> {
    ctx.globals.int_props.entry(form_id).or_default()
}

fn str_map(ctx: &mut CommandContext, form_id: u32) -> &mut HashMap<i32, String> {
    ctx.globals.str_props.entry(form_id).or_default()
}

fn int_list(ctx: &mut CommandContext, form_id: u32) -> &mut Vec<i64> {
    let fixed_len = fixed_int_list_len(ctx, form_id);
    let initial_len = fixed_len.unwrap_or(0);
    let list = ctx
        .globals
        .int_lists
        .entry(form_id)
        .or_insert_with(|| vec![0; initial_len]);
    if let Some(fixed_len) = fixed_len
        && list.len() < fixed_len
    {
        list.resize(fixed_len, 0);
    }
    list
}

fn str_list(ctx: &mut CommandContext, form_id: u32) -> &mut Vec<String> {
    let fixed_len = fixed_str_list_len(ctx, form_id);
    let initial_len = fixed_len.unwrap_or(0);
    let list = ctx
        .globals
        .str_lists
        .entry(form_id)
        .or_insert_with(|| vec![String::new(); initial_len]);
    if let Some(fixed_len) = fixed_len
        && list.len() < fixed_len
    {
        list.resize_with(fixed_len, String::new);
    }
    list
}

fn fixed_int_list_len(ctx: &CommandContext, form_id: u32) -> Option<usize> {
    super::int_list::fixed_default_len(ctx, form_id)
}

fn fixed_str_list_len(ctx: &CommandContext, form_id: u32) -> Option<usize> {
    super::str_list::fixed_default_len(ctx, form_id)
}

fn ensure_int_index(ctx: &mut CommandContext, form_id: u32, index: usize) -> bool {
    super::int_list::ensure_fixed_direct_index(ctx, form_id, index);
    if let Some(fixed_len) = fixed_int_list_len(ctx, form_id) {
        let list = int_list(ctx, form_id);
        if index < list.len() {
            return true;
        }
        log::error!(
            "INTLIST reference index out of range: form_id={} index={} size={} configured_size={}",
            form_id,
            index,
            list.len(),
            fixed_len
        );
        return false;
    }

    let list = int_list(ctx, form_id);
    if index < list.len() {
        return true;
    }
    log::error!(
        "INTLIST reference index out of range: form_id={} index={} size={}",
        form_id,
        index,
        list.len()
    );
    false
}

fn ensure_str_index(ctx: &mut CommandContext, form_id: u32, index: usize) -> bool {
    super::str_list::ensure_fixed_direct_index(ctx, form_id, index);
    if let Some(fixed_len) = fixed_str_list_len(ctx, form_id) {
        let list = str_list(ctx, form_id);
        if index < list.len() {
            return true;
        }
        log::error!(
            "STRLIST reference index out of range: form_id={} index={} size={} configured_size={}",
            form_id,
            index,
            list.len(),
            fixed_len
        );
        return false;
    }

    let list = str_list(ctx, form_id);
    if index < list.len() {
        return true;
    }
    log::error!(
        "STRLIST reference index out of range: form_id={} index={} size={}",
        form_id,
        index,
        list.len()
    );
    false
}

fn prefers_string(ret_form: Option<i64>, rhs: Option<&Value>) -> bool {
    ret_form_is_string_opt(ret_form) || matches!(rhs, Some(Value::Str(_)))
}

pub fn chain_key(parts: &[i32]) -> i32 {
    let mut h: u32 = 0x811C_9DC5;
    for &p in parts {
        h ^= p as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    h as i32
}

pub fn push_stored_or_default(
    ctx: &mut CommandContext,
    form_id: u32,
    op: i32,
    ret_form: Option<i64>,
) {
    if ret_form_is_string_opt(ret_form) {
        let s = ctx
            .globals
            .str_props
            .get(&form_id)
            .and_then(|m| m.get(&op))
            .cloned()
            .unwrap_or_default();
        ctx.push(Value::Str(s));
        return;
    }

    if let Some(s) = ctx
        .globals
        .str_props
        .get(&form_id)
        .and_then(|m| m.get(&op))
        .cloned()
    {
        ctx.push(Value::Str(s));
        return;
    }

    let v = ctx
        .globals
        .int_props
        .get(&form_id)
        .and_then(|m| m.get(&op).copied())
        .unwrap_or(0);
    ctx.push(Value::Int(v));
}

pub fn store_or_push_prop(
    ctx: &mut CommandContext,
    form_id: u32,
    prop_key: i32,
    chain_pos: usize,
    args: &[Value],
) {
    let (al_id, ret_form, rhs) = infer_assign_and_ret_ctx(ctx, chain_pos, args);
    if al_id == Some(1) {
        if let Some(v) = rhs {
            match v {
                Value::Str(s) => {
                    str_map(ctx, form_id).insert(prop_key, s);
                }
                Value::Int(n) => {
                    int_map(ctx, form_id).insert(prop_key, n);
                }
                _ => {}
            }
        }
        ctx.push(Value::Int(0));
        return;
    }

    push_stored_or_default(ctx, form_id, prop_key, ret_form);
}

pub fn store_or_push_direct_prop(
    ctx: &mut CommandContext,
    form_id: u32,
    prop_key: i32,
    args: &[Value],
    value_idx: usize,
) {
    if let Some(v) = args.get(value_idx).cloned() {
        match v {
            Value::Str(s) => {
                str_map(ctx, form_id).insert(prop_key, s);
            }
            Value::Int(n) => {
                int_map(ctx, form_id).insert(prop_key, n);
            }
            _ => {}
        }
        ctx.push(Value::Int(0));
        return;
    }

    push_stored_or_default(ctx, form_id, prop_key, None);
}

pub fn store_or_push_indexed(
    ctx: &mut CommandContext,
    form_id: u32,
    index: usize,
    chain_pos: usize,
    args: &[Value],
) {
    let (al_id, ret_form, rhs) = infer_assign_and_ret_ctx(ctx, chain_pos, args);
    if al_id == Some(1) {
        match rhs {
            Some(Value::Str(s)) => {
                if ensure_str_index(ctx, form_id, index) {
                    let list = str_list(ctx, form_id);
                    if let Some(dst) = list.get_mut(index) {
                        *dst = s;
                    }
                }
            }
            Some(Value::Int(n)) if ensure_int_index(ctx, form_id, index) => {
                let list = int_list(ctx, form_id);
                if let Some(dst) = list.get_mut(index) {
                    *dst = n;
                }
            }
            _ => {}
        }
        ctx.push(Value::Int(0));
        return;
    }

    if prefers_string(ret_form, rhs.as_ref()) {
        if !ensure_str_index(ctx, form_id, index) {
            ctx.push(Value::Str(String::new()));
            return;
        }
        let value = {
            let list = str_list(ctx, form_id);
            list.get(index).cloned().unwrap_or_default()
        };
        ctx.push(Value::Str(value));
    } else {
        if !ensure_int_index(ctx, form_id, index) {
            ctx.push(Value::Int(0));
            return;
        }
        let value = {
            let list = int_list(ctx, form_id);
            list.get(index).copied().unwrap_or(0)
        };
        ctx.push(Value::Int(value));
    }
}

pub fn store_or_push_indexed_direct(
    ctx: &mut CommandContext,
    form_id: u32,
    index: usize,
    args: &[Value],
    value_idx: usize,
) {
    if let Some(v) = args.get(value_idx).cloned() {
        match v {
            Value::Str(s) => {
                if ensure_str_index(ctx, form_id, index) {
                    let list = str_list(ctx, form_id);
                    if let Some(dst) = list.get_mut(index) {
                        *dst = s;
                    }
                }
            }
            Value::Int(n) if ensure_int_index(ctx, form_id, index) => {
                let list = int_list(ctx, form_id);
                if let Some(dst) = list.get_mut(index) {
                    *dst = n;
                }
            }
            _ => {}
        }
        ctx.push(Value::Int(0));
        return;
    }

    if fixed_str_list_len(ctx, form_id).is_some() {
        if !ensure_str_index(ctx, form_id, index) {
            ctx.push(Value::Str(String::new()));
            return;
        }
        let value = ctx
            .globals
            .str_lists
            .get(&form_id)
            .and_then(|v| v.get(index))
            .cloned()
            .unwrap_or_default();
        ctx.push(Value::Str(value));
        return;
    }

    if !ensure_int_index(ctx, form_id, index) {
        ctx.push(Value::Int(0));
        return;
    }
    let value = ctx
        .globals
        .int_lists
        .get(&form_id)
        .and_then(|v| v.get(index).copied())
        .unwrap_or(0);
    ctx.push(Value::Int(value));
}

pub fn dispatch_stateful_form(ctx: &mut CommandContext, form_id: u32, args: &[Value]) {
    if let Some((chain_pos, chain)) = parse_element_chain_ctx(ctx, form_id, args) {
        if chain.len() >= 3 && chain[1] == ctx.ids.elm_array {
            let Ok(index) = usize::try_from(chain[2]) else {
                log::error!(
                    "list index is negative: form_id={} index={}",
                    form_id,
                    chain[2]
                );
                let (_, ret_form) = current_vm_meta(ctx);
                if ret_form_is_string_opt(ret_form) {
                    ctx.push(Value::Str(String::new()));
                } else {
                    ctx.push(Value::Int(0));
                }
                return;
            };
            if chain.len() == 3 {
                store_or_push_indexed(ctx, form_id, index, chain_pos, args);
            } else {
                let key = chain_key(&chain[1..]);
                store_or_push_prop(ctx, form_id, key, chain_pos, args);
            }
            return;
        }

        if chain.len() >= 2 {
            let key = if chain.len() == 2 {
                chain[1]
            } else {
                chain_key(&chain[1..])
            };
            store_or_push_prop(ctx, form_id, key, chain_pos, args);
            return;
        }
    }

    if let Some(op) = args.first().and_then(|v| v.as_i64()) {
        if op == ctx.ids.elm_array as i64 {
            let raw_index = args.get(1).and_then(|v| v.as_i64()).unwrap_or(0);
            let Ok(index) = usize::try_from(raw_index) else {
                log::error!(
                    "list index is negative: form_id={} index={}",
                    form_id,
                    raw_index
                );
                ctx.push(Value::Int(0));
                return;
            };
            store_or_push_indexed_direct(ctx, form_id, index, args, 2);
        } else {
            store_or_push_direct_prop(ctx, form_id, op as i32, args, 1);
        }
        return;
    }

    ctx.push(Value::Int(0));
}

pub fn dispatch_generic_form(ctx: &mut CommandContext, form_id: u32, args: &[Value]) {
    dispatch_stateful_form(ctx, form_id, args);
}

pub fn assign_to_chain(ctx: &mut CommandContext, chain: &[i32], value: Value) {
    if chain.is_empty() {
        return;
    }
    let form_id = chain[0].max(0) as u32;
    if chain.len() >= 3 && chain[1] == ctx.ids.elm_array {
        let raw_index = chain[2];
        let Ok(index) = usize::try_from(raw_index) else {
            log::error!(
                "list reference index is negative: form_id={} index={}",
                form_id,
                raw_index
            );
            return;
        };
        match value {
            Value::Str(s) => {
                if !ensure_str_index(ctx, form_id, index) {
                    return;
                }
                let list = str_list(ctx, form_id);
                if let Some(dst) = list.get_mut(index) {
                    *dst = s;
                }
            }
            Value::Int(n) => {
                if !ensure_int_index(ctx, form_id, index) {
                    return;
                }
                let list = int_list(ctx, form_id);
                if let Some(dst) = list.get_mut(index) {
                    *dst = n;
                }
            }
            _ => {}
        }
        return;
    }

    if chain.len() >= 2 {
        let key = if chain.len() == 2 {
            chain[1]
        } else {
            chain_key(&chain[1..])
        };
        match value {
            Value::Str(s) => {
                str_map(ctx, form_id).insert(key, s);
            }
            Value::Int(n) => {
                int_map(ctx, form_id).insert(key, n);
            }
            _ => {}
        }
    }
}
