use anyhow::Result;

use crate::runtime::{CommandContext, Value};

use super::codes::{self, intlist_op, intlistref_op};
use super::prop_access;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntListTarget {
    Root,
    Index { width: u32, index: i64 },
    Command { width: u32, op: i32 },
}

fn selector_width(op: i32) -> Option<u32> {
    if op == intlist_op::BIT || op == intlistref_op::BIT {
        Some(1)
    } else if op == intlist_op::BIT2 || op == intlistref_op::BIT2 {
        Some(2)
    } else if op == intlist_op::BIT4 || op == intlistref_op::BIT4 {
        Some(4)
    } else if op == intlist_op::BIT8 || op == intlistref_op::BIT8 {
        Some(8)
    } else if op == intlist_op::BIT16 || op == intlistref_op::BIT16 {
        Some(16)
    } else {
        None
    }
}

fn parse_target(ctx: &CommandContext, chain: &[i32]) -> Option<IntListTarget> {
    if chain.is_empty() {
        return None;
    }
    if chain.len() == 1 {
        return Some(IntListTarget::Root);
    }

    let mut pos = 1usize;
    let mut width = 32u32;
    if let Some(selected) = selector_width(chain[pos]) {
        width = selected;
        pos += 1;
    }
    if pos >= chain.len() {
        return Some(IntListTarget::Root);
    }

    let op = chain[pos];
    if op == ctx.ids.elm_array || op == codes::ELM_ARRAY {
        let index = i64::from(*chain.get(pos + 1)?);
        Some(IntListTarget::Index { width, index })
    } else {
        Some(IntListTarget::Command { width, op })
    }
}

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
    let local = [
        codes::ELM_GLOBAL_A as u32,
        codes::ELM_GLOBAL_B as u32,
        codes::ELM_GLOBAL_C as u32,
        codes::ELM_GLOBAL_D as u32,
        codes::ELM_GLOBAL_E as u32,
        codes::ELM_GLOBAL_F as u32,
        codes::ELM_GLOBAL_X as u32,
    ];
    if local.contains(&form_id) {
        return Some(configured_count(ctx, false));
    }

    let global = [codes::ELM_GLOBAL_G as u32, codes::ELM_GLOBAL_Z as u32];
    if global.contains(&form_id) {
        return Some(configured_count(ctx, true));
    }

    None
}

fn fixed_count_is_explicit(ctx: &CommandContext, form_id: u32) -> bool {
    let local = [
        codes::ELM_GLOBAL_A as u32,
        codes::ELM_GLOBAL_B as u32,
        codes::ELM_GLOBAL_C as u32,
        codes::ELM_GLOBAL_D as u32,
        codes::ELM_GLOBAL_E as u32,
        codes::ELM_GLOBAL_F as u32,
        codes::ELM_GLOBAL_X as u32,
    ];
    if local.contains(&form_id) {
        return configured_count_info(ctx, false).1;
    }
    let global = [codes::ELM_GLOBAL_G as u32, codes::ELM_GLOBAL_Z as u32];
    global.contains(&form_id) && configured_count_info(ctx, true).1
}

fn list_mut(ctx: &mut CommandContext, form_id: u32) -> &mut Vec<i64> {
    let fixed_len = fixed_default_len(ctx, form_id);
    let initial_len = fixed_len.unwrap_or(0);
    let list = ctx
        .globals
        .int_lists
        .entry(form_id)
        .or_insert_with(|| vec![0; initial_len]);
    // Old saves and the previous compatibility layer could leave these lists at
    // 32 words. The original engine always restores the configured fixed size.
    if let Some(fixed_len) = fixed_len
        && list.len() < fixed_len
    {
        list.resize(fixed_len, 0);
    }
    list
}

fn required_storage_words(width: u32, index: i64) -> Option<usize> {
    if index < 0 || !matches!(width, 1 | 2 | 4 | 8 | 16 | 32) {
        return None;
    }
    let bits = (index as u64)
        .checked_add(1)?
        .checked_mul(u64::from(width))?;
    let words = bits.checked_add(31)? / 32;
    usize::try_from(words).ok()
}

fn ensure_compatible_access_capacity(
    ctx: &mut CommandContext,
    form_id: u32,
    width: u32,
    index: i64,
) {
    if fixed_default_len(ctx, form_id).is_none() || fixed_count_is_explicit(ctx, form_id) {
        return;
    }
    let Some(required) = required_storage_words(width, index) else {
        return;
    };
    // FLAG.CNT/GLOBAL_FLAG.CNT are capped at 10000 storage words in the
    // original engine. When Gameexe was unavailable, grow only inside that
    // legal envelope so valid titles are not constrained by the fallback 1000.
    if required <= 10000 {
        let list = list_mut(ctx, form_id);
        if list.len() < required {
            log::warn!(
                "Gameexe flag count unavailable; extending INTLIST compatibility storage: form_id={} old_words={} new_words={}",
                form_id,
                list.len(),
                required
            );
            list.resize(required, 0);
        }
    }
}

pub(super) fn ensure_fixed_direct_index(ctx: &mut CommandContext, form_id: u32, index: usize) {
    if let Ok(index) = i64::try_from(index) {
        ensure_compatible_access_capacity(ctx, form_id, 32, index);
    }
    let _ = list_mut(ctx, form_id);
}

fn is_fixed(ctx: &CommandContext, form_id: u32) -> bool {
    fixed_default_len(ctx, form_id).is_some()
}

fn i32_value(value: i64) -> i64 {
    i64::from(value as i32)
}

fn bit_location(list_len: usize, width: u32, index: i64) -> Option<(usize, u32)> {
    if index < 0 || !matches!(width, 1 | 2 | 4 | 8 | 16 | 32) {
        return None;
    }
    let bit_index = (index as u64).checked_mul(u64::from(width))?;
    let word = bit_index / 32;
    if word >= list_len as u64 {
        return None;
    }
    Some((word as usize, (bit_index % 32) as u32))
}

fn log_out_of_range(form_id: u32, width: u32, index: i64, words: usize) {
    log::error!(
        "INTLIST index out of range: form_id={} width={} index={} storage_words={}",
        form_id,
        width,
        index,
        words
    );
}

pub(super) fn bit_get(form_id: u32, list: &[i64], width: u32, index: i64) -> i64 {
    let Some((word, shift)) = bit_location(list.len(), width, index) else {
        log_out_of_range(form_id, width, index, list.len());
        return 0;
    };

    if width == 32 {
        return i64::from(list[word] as i32);
    }
    let mask = (1u32 << width) - 1;
    i64::from(((list[word] as i32 as u32) >> shift) & mask)
}

pub(super) fn bit_set(form_id: u32, list: &mut [i64], width: u32, index: i64, value: i64) {
    let Some((word, shift)) = bit_location(list.len(), width, index) else {
        log_out_of_range(form_id, width, index, list.len());
        return;
    };

    if width == 32 {
        list[word] = i32_value(value);
        return;
    }

    let value_mask = (1u32 << width) - 1;
    let field_mask = value_mask << shift;
    let raw = list[word] as i32 as u32;
    let next = (raw & !field_mask) | (((value as u32) & value_mask) << shift);
    list[word] = i64::from(next as i32);
}

pub(super) fn logical_size(storage_words: usize, width: u32) -> i64 {
    let per_word = 32usize / width as usize;
    storage_words.saturating_mul(per_word) as i64
}

fn resize_list(ctx: &mut CommandContext, form_id: u32, requested: i64) {
    if is_fixed(ctx, form_id) {
        log::error!(
            "INTLIST.RESIZE rejected for fixed list: form_id={} requested={}",
            form_id,
            requested
        );
        return;
    }
    let Ok(new_len) = usize::try_from(requested.max(0)) else {
        log::error!(
            "INTLIST.RESIZE size is not representable: form_id={} requested={}",
            form_id,
            requested
        );
        return;
    };
    list_mut(ctx, form_id).resize(new_len, 0);
}

fn reinit_list(ctx: &mut CommandContext, form_id: u32) {
    if let Some(default_len) = fixed_default_len(ctx, form_id) {
        let list = list_mut(ctx, form_id);
        list.resize(default_len, 0);
        list.fill(0);
    } else {
        // The original extendable INTLIST instances used here (for example EXCALL.F)
        // have a default size of zero.
        list_mut(ctx, form_id).clear();
    }
}

fn params(args: &[Value], chain_pos: usize) -> &[Value] {
    prop_access::script_args(args, chain_pos.min(args.len()))
}

/// Implements the original `tnm_command_proc_int_list` behavior.
pub fn dispatch(ctx: &mut CommandContext, form_id: u32, args: &[Value]) -> Result<bool> {
    let Some((chain_pos, chain_ref)) = prop_access::parse_element_chain_ctx(ctx, form_id, args)
        .or_else(|| prop_access::parse_element_chain(form_id, args))
    else {
        ctx.push(Value::Int(0));
        return Ok(true);
    };
    let chain = chain_ref.to_vec();
    let target = parse_target(ctx, &chain);
    let script_params = params(args, chain_pos);
    let (al_id, ret_form) = prop_access::current_vm_meta(ctx);
    let al_id = al_id.unwrap_or(0);

    match target {
        Some(IntListTarget::Root) => {
            // Returning an element reference is represented by the VM's element chain,
            // not by a scalar stack value. Keep the historical neutral result here.
            ctx.push(Value::Int(0));
        }
        Some(IntListTarget::Index { width, index }) => {
            ensure_compatible_access_capacity(ctx, form_id, width, index);
            if al_id == 1 {
                let value = args.first().and_then(Value::as_i64).unwrap_or(0);
                let list = list_mut(ctx, form_id);
                bit_set(form_id, list.as_mut_slice(), width, index, value);
                ctx.push(Value::Int(0));
            } else {
                let value = {
                    let list = list_mut(ctx, form_id);
                    bit_get(form_id, list.as_slice(), width, index)
                };
                ctx.push(Value::Int(value));
            }
        }
        Some(IntListTarget::Command { width, op }) => match op {
            intlist_op::INIT => {
                reinit_list(ctx, form_id);
                ctx.push(Value::Int(0));
            }
            intlist_op::RESIZE => {
                let requested = script_params.first().and_then(Value::as_i64).unwrap_or(0);
                resize_list(ctx, form_id, requested);
                ctx.push(Value::Int(0));
            }
            intlist_op::GET_SIZE => {
                let size = {
                    let list = list_mut(ctx, form_id);
                    logical_size(list.len(), width)
                };
                ctx.push(Value::Int(size));
            }
            intlist_op::CLEAR => {
                let start = script_params.first().and_then(Value::as_i64).unwrap_or(0);
                let end = script_params
                    .get(1)
                    .and_then(Value::as_i64)
                    .unwrap_or(start);
                let value = if al_id == 0 {
                    0
                } else {
                    script_params.get(2).and_then(Value::as_i64).unwrap_or(0)
                };
                if start <= end {
                    ensure_compatible_access_capacity(ctx, form_id, width, end);
                    let list = list_mut(ctx, form_id);
                    for index in start..=end {
                        bit_set(form_id, list.as_mut_slice(), width, index, value);
                    }
                }
                ctx.push(Value::Int(0));
            }
            intlist_op::SETS => {
                let start = script_params.first().and_then(Value::as_i64).unwrap_or(0);
                if let Some(last_offset) = script_params.len().checked_sub(2)
                    && let Some(last_index) = start.checked_add(last_offset as i64)
                {
                    ensure_compatible_access_capacity(ctx, form_id, width, last_index);
                }
                let list = list_mut(ctx, form_id);
                for (offset, value) in script_params.iter().skip(1).enumerate() {
                    let Some(index) = start.checked_add(offset as i64) else {
                        log::error!(
                            "INTLIST.SETS index overflow: form_id={} start={}",
                            form_id,
                            start
                        );
                        break;
                    };
                    bit_set(
                        form_id,
                        list.as_mut_slice(),
                        width,
                        index,
                        value.as_i64().unwrap_or(0),
                    );
                }
                ctx.push(Value::Int(0));
            }
            _ => {
                log::error!(
                    "unsupported INTLIST command: form_id={} op={} width={} ret_form={:?}",
                    form_id,
                    op,
                    width,
                    ret_form
                );
                ctx.push(Value::Int(0));
            }
        },
        None => {
            log::error!(
                "malformed INTLIST element chain: form_id={} chain={:?}",
                form_id,
                chain
            );
            ctx.push(Value::Int(0));
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_access_uses_signed_i32_storage() {
        let mut words = vec![0_i64];
        bit_set(25, &mut words, 8, 3, 0xAB);
        assert_eq!(bit_get(25, &words, 8, 3), 0xAB);
        assert_eq!(words[0], i64::from(0xAB00_0000u32 as i32));

        bit_set(25, &mut words, 32, 0, 0xFFFF_FFFFu32 as i64);
        assert_eq!(words[0], -1);
        assert_eq!(bit_get(25, &words, 32, 0), -1);
    }

    #[test]
    fn packed_access_does_not_grow_or_alias() {
        let mut words = vec![0x1234_i64];
        let before = words.clone();
        bit_set(25, &mut words, 8, 4, 0xCD);
        bit_set(25, &mut words, 16, -1, 0xFFFF);
        assert_eq!(words, before);
        assert_eq!(bit_get(25, &words, 8, 4), 0);
        assert_eq!(bit_get(25, &words, 16, -1), 0);
    }

    #[test]
    fn logical_sizes_match_cpp_views() {
        assert_eq!(logical_size(7, 1), 224);
        assert_eq!(logical_size(7, 2), 112);
        assert_eq!(logical_size(7, 4), 56);
        assert_eq!(logical_size(7, 8), 28);
        assert_eq!(logical_size(7, 16), 14);
        assert_eq!(logical_size(7, 32), 7);
    }
}
