use anyhow::Result;

use crate::runtime::{CommandContext, Value};

use super::codes::{self, str_list_op, str_op};
use super::prop_access;
use crate::runtime::string_semantics as tstr;

fn configured_count_info(ctx: &CommandContext, global: bool) -> (usize, bool) {
    let keys = if global {
        ["#GLOBAL_FLAG.CNT", "GLOBAL_FLAG.CNT"]
    } else {
        ["#FLAG.CNT", "FLAG.CNT"]
    };
    let configured = ctx
        .tables
        .gameexe
        .as_ref()
        .and_then(|cfg| keys.into_iter().find_map(|key| cfg.get_usize(key)));
    (configured.unwrap_or(1000).min(10000), configured.is_some())
}

fn configured_count(ctx: &CommandContext, global: bool) -> usize {
    configured_count_info(ctx, global).0
}

pub(super) fn fixed_default_len(ctx: &CommandContext, form_id: u32) -> Option<usize> {
    if form_id == codes::ELM_GLOBAL_S as u32 {
        Some(configured_count(ctx, false))
    } else if form_id == codes::ELM_GLOBAL_M as u32 {
        Some(configured_count(ctx, true))
    } else if form_id == codes::ELM_GLOBAL_NAMAE_LOCAL as u32
        || form_id == codes::ELM_GLOBAL_NAMAE_GLOBAL as u32
    {
        Some(26 + 26 * 26)
    } else {
        None
    }
}

fn fixed_count_is_explicit(ctx: &CommandContext, form_id: u32) -> bool {
    if form_id == codes::ELM_GLOBAL_S as u32 {
        configured_count_info(ctx, false).1
    } else if form_id == codes::ELM_GLOBAL_M as u32 {
        configured_count_info(ctx, true).1
    } else {
        true
    }
}

fn list_mut(ctx: &mut CommandContext, form_id: u32) -> &mut Vec<String> {
    let fixed_len = fixed_default_len(ctx, form_id);
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

fn ensure_compatible_index(ctx: &mut CommandContext, form_id: u32, index: usize) {
    let _ = list_mut(ctx, form_id);
    if fixed_default_len(ctx, form_id).is_some()
        && !fixed_count_is_explicit(ctx, form_id)
        && index < 10000
    {
        let list = list_mut(ctx, form_id);
        if list.len() <= index {
            log::warn!(
                "Gameexe string flag count unavailable; extending STRLIST compatibility storage: form_id={} old_size={} new_size={}",
                form_id,
                list.len(),
                index + 1
            );
            list.resize_with(index + 1, String::new);
        }
    }
}

pub(super) fn ensure_fixed_direct_index(ctx: &mut CommandContext, form_id: u32, index: usize) {
    ensure_compatible_index(ctx, form_id, index);
}

fn default_for_ret_form(ret_form: i64) -> Value {
    if prop_access::ret_form_is_string(ret_form) {
        Value::Str(String::new())
    } else {
        Value::Int(0)
    }
}

fn execute_str_op(current: &str, op: i32, params: &[Value], al_id: i64) -> Value {
    match op {
        str_op::UPPER => Value::Str(tstr::ascii_upper(current)),
        str_op::LOWER => Value::Str(tstr::ascii_lower(current)),
        str_op::CNT => Value::Int(tstr::utf16_len(current) as i64),
        str_op::LEN => Value::Int(tstr::display_width(current) as i64),
        str_op::LEFT => {
            let len = params.first().and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
            Value::Str(tstr::utf16_left(current, len))
        }
        str_op::LEFT_LEN => {
            let len = params.first().and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
            Value::Str(tstr::left_by_display_width(current, len))
        }
        str_op::RIGHT => {
            let len = params.first().and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
            Value::Str(tstr::utf16_right(current, len))
        }
        str_op::RIGHT_LEN => {
            let len = params.first().and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
            Value::Str(tstr::right_by_display_width(current, len))
        }
        str_op::MID => {
            let start = params.first().and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
            if al_id == 0 || params.len() <= 1 {
                Value::Str(tstr::utf16_slice(current, start, None))
            } else {
                let len = params.get(1).and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
                Value::Str(tstr::utf16_slice(current, start, Some(len)))
            }
        }
        str_op::MID_LEN => {
            let start = params.first().and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
            let len = if al_id == 0 || params.len() <= 1 {
                None
            } else {
                Some(params.get(1).and_then(Value::as_i64).unwrap_or(0).max(0) as usize)
            };
            Value::Str(tstr::mid_by_display_width(current, start, len))
        }
        str_op::SEARCH => {
            let needle = params.first().and_then(Value::as_str).unwrap_or("");
            Value::Int(
                tstr::search_ascii_case_insensitive(current, needle)
                    .map(|v| v as i64)
                    .unwrap_or(-1),
            )
        }
        str_op::SEARCH_LAST => {
            let needle = params.first().and_then(Value::as_str).unwrap_or("");
            Value::Int(
                tstr::rsearch_ascii_case_insensitive(current, needle)
                    .map(|v| v as i64)
                    .unwrap_or(-1),
            )
        }
        str_op::GET_CODE => {
            let pos = params.first().and_then(Value::as_i64).unwrap_or(0);
            Value::Int(
                usize::try_from(pos)
                    .ok()
                    .and_then(|pos| tstr::utf16_code_unit(current, pos))
                    .map(i64::from)
                    .unwrap_or(-1),
            )
        }
        str_op::TONUM => Value::Int(current.parse::<i64>().unwrap_or(0)),
        _ => {
            log::error!("unsupported STR command in STRLIST element: op={op}");
            Value::Str(current.to_string())
        }
    }
}

fn log_out_of_range(form_id: u32, index: i64, len: usize) {
    log::error!(
        "STRLIST index out of range: form_id={} index={} size={}",
        form_id,
        index,
        len
    );
}

pub fn dispatch(ctx: &mut CommandContext, form_id: u32, args: &[Value]) -> Result<bool> {
    let Some((chain_pos, chain_ref)) = prop_access::parse_element_chain_ctx(ctx, form_id, args)
        .or_else(|| prop_access::parse_element_chain(form_id, args))
    else {
        ctx.push(Value::Str(String::new()));
        return Ok(true);
    };
    let chain = chain_ref.to_vec();
    let params = prop_access::script_args(args, chain_pos.min(args.len()));
    let (al_id, ret_form) = prop_access::current_vm_meta(ctx);
    let al_id = al_id.unwrap_or(0);
    let ret_form = ret_form.unwrap_or(prop_access::FM_STR);

    if chain.len() >= 3 && (chain[1] == ctx.ids.elm_array || chain[1] == codes::ELM_ARRAY) {
        let index = i64::from(chain[2]);
        let Some(index_usize) = usize::try_from(index).ok() else {
            let len = list_mut(ctx, form_id).len();
            log_out_of_range(form_id, index, len);
            ctx.push(default_for_ret_form(ret_form));
            return Ok(true);
        };

        ensure_compatible_index(ctx, form_id, index_usize);
        let in_range = index_usize < list_mut(ctx, form_id).len();
        if !in_range {
            let len = list_mut(ctx, form_id).len();
            log_out_of_range(form_id, index, len);
            ctx.push(default_for_ret_form(ret_form));
            return Ok(true);
        }

        if chain.len() == 3 {
            if al_id == 1 {
                let rhs = args
                    .first()
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                list_mut(ctx, form_id)[index_usize] = rhs;
                ctx.push(Value::Int(0));
            } else {
                let value = list_mut(ctx, form_id)[index_usize].clone();
                ctx.push(Value::Str(value));
            }
        } else {
            let op = chain[3];
            let current = list_mut(ctx, form_id)[index_usize].clone();
            ctx.push(execute_str_op(&current, op, params, al_id));
        }
        return Ok(true);
    }

    if chain.len() >= 2 {
        match chain[1] {
            str_list_op::INIT => {
                if let Some(default_len) = fixed_default_len(ctx, form_id) {
                    let list = list_mut(ctx, form_id);
                    list.resize_with(default_len, String::new);
                    list.fill(String::new());
                } else {
                    list_mut(ctx, form_id).clear();
                }
                ctx.push(Value::Int(0));
                return Ok(true);
            }
            str_list_op::RESIZE => {
                if fixed_default_len(ctx, form_id).is_some() {
                    let requested = params.first().and_then(Value::as_i64).unwrap_or(0);
                    log::error!(
                        "STRLIST.RESIZE rejected for fixed list: form_id={} requested={}",
                        form_id,
                        requested
                    );
                } else {
                    let requested = params.first().and_then(Value::as_i64).unwrap_or(0).max(0);
                    if let Ok(new_len) = usize::try_from(requested) {
                        list_mut(ctx, form_id).resize_with(new_len, String::new);
                    }
                }
                ctx.push(Value::Int(0));
                return Ok(true);
            }
            str_list_op::GET_SIZE => {
                let len = list_mut(ctx, form_id).len() as i64;
                ctx.push(Value::Int(len));
                return Ok(true);
            }
            str_list_op::SETS => {
                // Present in the element table, but not implemented by the original
                // `tnm_command_proc_str_list` in this engine version.
                log::error!(
                    "STRLIST.SETS is unsupported by the original engine path: form_id={form_id}"
                );
                ctx.push(Value::Int(0));
                return Ok(true);
            }
            op => {
                log::error!("unsupported STRLIST command: form_id={} op={}", form_id, op);
            }
        }
    }

    ctx.push(default_for_ret_form(ret_form));
    Ok(true)
}
