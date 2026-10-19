use anyhow::{Result, bail};

use crate::runtime::globals::{ObjectFrameActionState, PendingFrameActionFinish};
use crate::runtime::{CommandContext, Value, VmCallMeta};

use super::codes::{self, excall_op};
use super::{counter, frame_action, frame_action_ch, int_list, script, stage};

const EXCALL_LOCAL_NS_XOR: u32 = 0x4000;

fn excall_form_key(ctx: &CommandContext) -> u32 {
    if ctx.ids.form_global_excall != 0 {
        ctx.ids.form_global_excall
    } else {
        super::codes::FORM_GLOBAL_EXCALL
    }
}

fn stage_form_key(ctx: &CommandContext) -> u32 {
    if ctx.ids.form_global_stage != 0 {
        ctx.ids.form_global_stage
    } else {
        super::codes::FORM_GLOBAL_STAGE
    }
}

fn excall_stage_form_key(ctx: &CommandContext, selector: i32) -> u32 {
    let base = stage_form_key(ctx);
    if selector == 0 {
        base
    } else {
        base ^ EXCALL_LOCAL_NS_XOR
    }
}

fn synth_form_key(base: u32, selector: i32, op: i32) -> u32 {
    (base << 8) ^ (((selector as u32) & 0x0f) << 4) ^ (op as u32 & 0x0f)
}

type ParsedExcall = (
    usize,
    Vec<i32>,
    i32,
    i32,
    Vec<Value>,
    Option<i64>,
    Option<i64>,
);

fn parse_call(ctx: &CommandContext, args: &[Value]) -> Option<ParsedExcall> {
    let form_id = excall_form_key(ctx);
    let (chain_pos, chain) = super::prop_access::parse_element_chain_ctx(ctx, form_id, args)?;
    let (selector, op_pos) = if chain.len() >= 3
        && chain[1] == crate::runtime::forms::codes::ELM_ARRAY
        && (chain[2] == 0 || chain[2] == 1)
    {
        (chain[2], 3usize)
    } else {
        (1i32, 1usize)
    };
    let op = chain
        .get(op_pos)
        .copied()
        .or_else(|| args.first().and_then(|v| v.as_i64()).map(|v| v as i32))?;
    let params = super::prop_access::script_args(args, chain_pos.min(args.len()));
    let (meta_al_id, meta_ret_form) = crate::runtime::forms::prop_access::current_vm_meta(ctx);
    let al_id = meta_al_id;
    let ret_form = meta_ret_form;
    // Own the nested element chain and script parameters before returning.
    // parse_element_chain_ctx() may borrow the chain from ctx.vm_call; keeping
    // that borrow alive across dispatch would prevent child handlers from
    // mutably borrowing CommandContext.
    Some((
        chain_pos,
        chain.to_vec(),
        selector,
        op,
        params.to_vec(),
        al_id,
        ret_form,
    ))
}

fn local_flag_count(ctx: &CommandContext) -> usize {
    ctx.tables
        .gameexe
        .as_ref()
        .and_then(|cfg| {
            cfg.get_usize("#FLAG.CNT")
                .or_else(|| cfg.get_usize("FLAG.CNT"))
        })
        .unwrap_or(1000)
        .min(10000)
}

fn counter_count(ctx: &CommandContext) -> usize {
    ctx.tables
        .gameexe
        .as_ref()
        .and_then(|cfg| {
            cfg.get_usize("#COUNTER.CNT")
                .or_else(|| cfg.get_usize("COUNTER.CNT"))
        })
        .unwrap_or(16)
        .min(256)
}

fn frame_action_ch_count(ctx: &CommandContext) -> usize {
    ctx.tables
        .gameexe
        .as_ref()
        .and_then(|cfg| {
            cfg.get_usize("#FRAME_ACTION_CH.CNT")
                .or_else(|| cfg.get_usize("FRAME_ACTION_CH.CNT"))
        })
        .unwrap_or(4)
        .min(256)
}

fn excall_local_f_key(ctx: &CommandContext) -> u32 {
    synth_form_key(excall_form_key(ctx), 1, excall_op::OP_7)
}

fn queue_frame_action_finish(
    ctx: &mut CommandContext,
    fa: &ObjectFrameActionState,
    frame_action_chain: Vec<i32>,
) {
    if fa.cmd_name.is_empty() {
        return;
    }
    ctx.globals
        .pending_frame_action_finishes
        .push(PendingFrameActionFinish {
            frame_action_chain,
            object_chain: None,
            snapshot: fa.clone(),
            reinit_after_finish: false,
            scn_name: fa.scn_name.clone(),
            cmd_name: fa.cmd_name.clone(),
            end_time: fa.end_time,
            args: fa.args.clone(),
        });
}

fn begin_free(ctx: &mut CommandContext) {
    let targets = tick_targets(ctx);

    // C_elm_excall::finish(): main frame action, channel list, then stage list.
    if let Some(fa) = ctx
        .globals
        .frame_actions
        .get(&targets.frame_action_id)
        .cloned()
    {
        queue_frame_action_finish(ctx, &fa, vec![targets.frame_action_id as i32]);
    }
    let ch_snapshot = ctx
        .globals
        .frame_action_lists
        .get(&targets.frame_action_ch_list_id)
        .cloned()
        .unwrap_or_default();
    for (idx, fa) in ch_snapshot.iter().enumerate() {
        queue_frame_action_finish(
            ctx,
            fa,
            vec![
                targets.frame_action_ch_list_id as i32,
                crate::runtime::forms::codes::ELM_ARRAY,
                idx as i32,
            ],
        );
    }
    stage::queue_stage_form_finishes(ctx, targets.stage_form_id);

    // Keep EXCALL ready until the VM drains the synchronous-equivalent finish
    // callbacks. C++ clears child storage only after finish() returns.
    ctx.excall_state.pending_free = true;
}

pub(crate) fn finalize_pending_free(ctx: &mut CommandContext) {
    if !ctx.excall_state.pending_free {
        return;
    }
    let targets = tick_targets(ctx);
    let f_key = excall_local_f_key(ctx);
    ctx.globals.int_lists.remove(&f_key);
    ctx.globals.counter_lists.remove(&targets.counter_list_id);
    ctx.globals
        .frame_action_lists
        .remove(&targets.frame_action_ch_list_id);
    stage::free_stage_form(ctx, targets.stage_form_id);
    ctx.excall_state.ready = false;
    ctx.excall_state.pending_free = false;
    // C_elm_excall::free() clears m_font_name but leaves the POD overrides.
    ctx.excall_state.font_name.clear();
}

pub(crate) fn tick_targets(ctx: &CommandContext) -> crate::runtime::globals::ExcallTickTargets {
    let form_key = excall_form_key(ctx);
    crate::runtime::globals::ExcallTickTargets {
        counter_list_id: synth_form_key(form_key, 1, excall_op::OP_6),
        frame_action_id: synth_form_key(form_key, 1, excall_op::OP_9),
        frame_action_ch_list_id: synth_form_key(form_key, 1, excall_op::OP_10),
        stage_form_id: excall_stage_form_key(ctx, 1),
    }
}

fn push_default(ctx: &mut CommandContext, ret_form: Option<i64>) {
    if ret_form == Some(codes::FM_STR as i64) || ret_form == Some(codes::FM_STRREF as i64) {
        ctx.push(Value::Str(String::new()));
    } else {
        ctx.push(Value::Int(0));
    }
}

fn child_requires_ready(op: i32) -> bool {
    matches!(
        op,
        excall_op::OP_0
            | excall_op::OP_1
            | excall_op::OP_2
            | excall_op::OP_3
            | excall_op::OP_6
            | excall_op::OP_7
            | excall_op::OP_9
            | excall_op::OP_10
            | excall_op::OP_13
    )
}

fn translated_call_element(form_id: u32, chain_tail: &[i32]) -> Vec<i32> {
    let mut chain = Vec::with_capacity(1 + chain_tail.len());
    chain.push(form_id as i32);
    chain.extend_from_slice(chain_tail);
    chain
}

/// Map EXCALL child selectors to the concrete stage index used by
/// C_elm_excall::m_stage_list.  The EXCALL element ids are *not* stage
/// indices: def_element_Siglus.h / cmd_call.cpp define FRONT=1, BACK=2,
/// NEXT=3, while TNM_STAGE_BACK/FRONT/NEXT are 0/1/2.
fn excall_stage_index(op: i32) -> Option<i32> {
    match op {
        excall_op::FRONT => Some(1),
        excall_op::BACK => Some(0),
        excall_op::NEXT => Some(2),
        _ => None,
    }
}

fn translated_stage_element(
    stage_form_id: u32,
    stage_idx: Option<i32>,
    chain_tail: &[i32],
) -> Vec<i32> {
    let mut chain = Vec::new();
    chain.push(stage_form_id as i32);
    if let Some(idx) = stage_idx {
        chain.push(crate::runtime::forms::codes::ELM_ARRAY);
        chain.push(idx);
    }
    chain.extend_from_slice(chain_tail);
    chain
}

fn with_forwarded_vm_call<F>(ctx: &mut CommandContext, element: Vec<i32>, f: F) -> Result<bool>
where
    F: FnOnce(&mut CommandContext) -> Result<bool>,
{
    // C++ passes elm_begin + 1/+2 directly to each EXCALL child handler. In
    // Rust, child form parsers read ctx.vm_call, so install the equivalent
    // advanced element chain for the duration of the nested dispatch.
    let saved = ctx.vm_call.clone();
    let (al_id, ret_form) = saved
        .as_ref()
        .map(|m| (m.al_id, m.ret_form))
        .unwrap_or((0, 0));
    ctx.vm_call = Some(VmCallMeta {
        element,
        al_id,
        ret_form,
    });
    let result = f(ctx);
    ctx.vm_call = saved;
    result
}

pub fn dispatch(ctx: &mut CommandContext, args: &[Value]) -> Result<bool> {
    let Some((_chain_pos, chain, selector, op, params, _al_id, ret_form)) = parse_call(ctx, args)
    else {
        if args.is_empty() {
            bail!("EXCALL form expects at least one argument (op id)");
        }
        return Ok(false);
    };

    let op_pos =
        if chain.len() >= 3 && chain[1] == codes::ELM_ARRAY && (chain[2] == 0 || chain[2] == 1) {
            3usize
        } else {
            1usize
        };
    let tail = chain.get(op_pos + 1..).unwrap_or(&[]);
    let form_key = excall_form_key(ctx);

    match op {
        excall_op::OP_4 => {
            if selector != 1 {
                log::error!("EXCALL.ALLOC requires EXCALL[1], selector={}", selector);
                push_default(ctx, ret_form);
                return Ok(true);
            }
            let targets = tick_targets(ctx);
            let f_key = excall_local_f_key(ctx);
            let f_count = local_flag_count(ctx);
            let counter_count = counter_count(ctx);
            let frame_action_ch_count = frame_action_ch_count(ctx);
            ctx.globals
                .int_lists
                .entry(f_key)
                .or_default()
                .resize(f_count, 0);
            ctx.globals
                .counter_lists
                .entry(targets.counter_list_id)
                .or_default()
                .resize_with(counter_count, Default::default);
            ctx.globals
                .frame_actions
                .entry(targets.frame_action_id)
                .or_default();
            ctx.globals
                .frame_action_lists
                .entry(targets.frame_action_ch_list_id)
                .or_default()
                .resize_with(frame_action_ch_count, ObjectFrameActionState::default);
            stage::ensure_stage_form_allocated(ctx, targets.stage_form_id);
            ctx.excall_state.ready = true;
            push_default(ctx, ret_form);
            return Ok(true);
        }
        excall_op::OP_5 => {
            if selector != 1 {
                log::error!("EXCALL.FREE requires EXCALL[1], selector={}", selector);
                push_default(ctx, ret_form);
                return Ok(true);
            }
            begin_free(ctx);
            push_default(ctx, ret_form);
            return Ok(true);
        }
        excall_op::OP_8 => {
            ctx.push(Value::Int(if selector == 1 && ctx.excall_state.ready {
                1
            } else {
                0
            }));
            return Ok(true);
        }
        excall_op::OP_12 => {
            ctx.push(Value::Int(if ctx.excall_state.ex_call_flag {
                1
            } else {
                0
            }));
            return Ok(true);
        }
        _ => {}
    }

    if selector == 1 && child_requires_ready(op) && !ctx.excall_state.ready {
        log::error!(
            "EXCALL child accessed before ALLOC: op={} chain={:?}",
            op,
            chain
        );
        push_default(ctx, ret_form);
        return Ok(true);
    }

    match op {
        excall_op::OP_0 => {
            let element =
                translated_stage_element(excall_stage_form_key(ctx, selector), None, tail);
            with_forwarded_vm_call(ctx, element, |ctx| stage::dispatch(ctx, &params))
        }
        excall_op::FRONT | excall_op::BACK | excall_op::NEXT => {
            let stage_idx = excall_stage_index(op)
                .expect("FRONT/BACK/NEXT must map to a concrete EXCALL stage");
            let element = translated_stage_element(
                excall_stage_form_key(ctx, selector),
                Some(stage_idx),
                tail,
            );
            with_forwarded_vm_call(ctx, element, |ctx| stage::dispatch(ctx, &params))
        }
        excall_op::OP_6 => {
            let key = synth_form_key(form_key, selector, op);
            let element = translated_call_element(key, tail);
            with_forwarded_vm_call(ctx, element, |ctx| counter::dispatch(ctx, key, &params))
        }
        excall_op::OP_7 => {
            let key = if selector == 0 {
                codes::ELM_GLOBAL_F as u32
            } else {
                excall_local_f_key(ctx)
            };
            let element = translated_call_element(key, tail);
            with_forwarded_vm_call(ctx, element, |ctx| int_list::dispatch(ctx, key, &params))
        }
        excall_op::OP_9 => {
            let key = synth_form_key(form_key, selector, op);
            let element = translated_call_element(key, tail);
            with_forwarded_vm_call(ctx, element, |ctx| {
                frame_action::dispatch(ctx, key, &params)
            })
        }
        excall_op::OP_10 => {
            let key = synth_form_key(form_key, selector, op);
            let element = translated_call_element(key, tail);
            with_forwarded_vm_call(ctx, element, |ctx| {
                frame_action_ch::dispatch(ctx, key, &params)
            })
        }
        excall_op::OP_13 => {
            let Some(script_op) = tail.first().copied() else {
                push_default(ctx, ret_form);
                return Ok(true);
            };
            if script::dispatch_excall(ctx, script_op, &params)? {
                Ok(true)
            } else {
                log::error!(
                    "unsupported EXCALL.SCRIPT operation: selector={} op={}",
                    selector,
                    script_op
                );
                Ok(false)
            }
        }
        _ => {
            log::error!(
                "unsupported EXCALL operation: selector={} op={}",
                selector,
                op
            );
            push_default(ctx, ret_form);
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excall_stage_selectors_match_original_stage_indices() {
        // Original cmd_call.cpp:
        //   ELM_EXCALL_FRONT -> TNM_STAGE_FRONT (1)
        //   ELM_EXCALL_BACK  -> TNM_STAGE_BACK  (0)
        //   ELM_EXCALL_NEXT  -> TNM_STAGE_NEXT  (2)
        assert_eq!(excall_stage_index(excall_op::FRONT), Some(1));
        assert_eq!(excall_stage_index(excall_op::BACK), Some(0));
        assert_eq!(excall_stage_index(excall_op::NEXT), Some(2));
        assert_eq!(excall_stage_index(excall_op::STAGE), None);
    }
}
