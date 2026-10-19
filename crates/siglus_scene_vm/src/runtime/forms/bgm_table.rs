use anyhow::{Result, bail};

use crate::runtime::{CommandContext, Value};

use super::codes::bgm_table_op;

fn arg_str(args: &[Value], idx: usize) -> Option<&str> {
    match args.get(idx) {
        Some(Value::Str(s)) => Some(s.as_str()),
        Some(Value::NamedArg { value, .. }) => value.as_str(),
        _ => None,
    }
}

fn arg_int(args: &[Value], idx: usize) -> Option<i64> {
    args.get(idx).and_then(|v| v.as_i64())
}

fn normalize_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn bgm_declared_count(ctx: &CommandContext) -> usize {
    ctx.tables
        .gameexe
        .as_ref()
        .map(|cfg| {
            cfg.get_usize("BGM.CNT")
                .unwrap_or_else(|| cfg.indexed_count("BGM"))
        })
        .unwrap_or(0)
}

fn bgm_count(ctx: &CommandContext) -> usize {
    bgm_declared_count(ctx)
        .max(ctx.globals.bgm_table_flags.len())
        .max(ctx.globals.bgm_table_listened.len())
}

fn ensure_bgm_flags_size(ctx: &mut CommandContext) {
    let count = bgm_declared_count(ctx);
    if ctx.globals.bgm_table_flags.len() < count {
        ctx.globals
            .bgm_table_flags
            .resize(count, ctx.globals.bgm_table_all_flag);
    }
}

fn bgm_regist_name(ctx: &CommandContext, index: usize) -> Option<String> {
    let cfg = ctx.tables.gameexe.as_ref()?;
    cfg.get_indexed_item_unquoted("BGM", index, 0)
        .or_else(|| cfg.get_indexed_field_unquoted("BGM", index, "REGIST_NAME"))
        .or_else(|| cfg.get_indexed_unquoted("BGM", index))
        .map(|s| s.to_string())
}

fn bgm_no_by_regist_name(ctx: &CommandContext, name: &str) -> Option<usize> {
    let needle = normalize_name(name);
    let count = bgm_declared_count(ctx);
    for i in 0..count {
        let Some(regist_name) = bgm_regist_name(ctx, i) else {
            continue;
        };
        if normalize_name(&regist_name) == needle {
            return Some(i);
        }
    }
    None
}

pub(crate) fn mark_listened_by_name(ctx: &mut CommandContext, name: &str, listened: bool) -> bool {
    ensure_bgm_flags_size(ctx);
    let key = normalize_name(name);
    let index = bgm_no_by_regist_name(ctx, name);
    if let Some(index) = index {
        if ctx.globals.bgm_table_flags.len() <= index {
            ctx.globals
                .bgm_table_flags
                .resize(index + 1, ctx.globals.bgm_table_all_flag);
        }
        ctx.globals.bgm_table_flags[index] = listened;
        ctx.globals.bgm_table_listened.insert(key, listened);
        true
    } else {
        false
    }
}

pub fn dispatch(ctx: &mut CommandContext, args: &[Value]) -> Result<bool> {
    // C++ tnm_command_proc_bgm_table() receives the operation from elm_begin[0]
    // and the script arguments separately through p_ai->al_begin[].  The VM now
    // carries that element chain in VmCallMeta; args contains only script args.
    let Some(op) = crate::runtime::forms::prop_access::current_op_from_ctx_or_args(ctx, args)
    else {
        bail!("BGMTABLE form expects an element opcode");
    };
    let args = crate::runtime::forms::prop_access::params_without_op(ctx, args);

    match op {
        bgm_table_op::GET_COUNT => {
            ctx.push(Value::Int(bgm_count(ctx) as i64));
            Ok(true)
        }
        bgm_table_op::GET_LISTEN_BY_NAME => {
            let Some(name) = arg_str(args, 0) else {
                ctx.push(Value::Int(-1));
                return Ok(true);
            };
            ensure_bgm_flags_size(ctx);
            let res = if let Some(index) = bgm_no_by_regist_name(ctx, name) {
                ctx.globals
                    .bgm_table_flags
                    .get(index)
                    .copied()
                    .unwrap_or(ctx.globals.bgm_table_all_flag) as i64
            } else {
                -1
            };
            ctx.push(Value::Int(res));
            Ok(true)
        }
        bgm_table_op::SET_LISTEN_CURRENT => {
            let Some(name) = arg_str(args, 0) else {
                return Ok(true);
            };
            let listened = arg_int(args, 1).unwrap_or(0) != 0;
            let _ = mark_listened_by_name(ctx, name, listened);
            Ok(true)
        }
        bgm_table_op::SET_ALL_FLAG => {
            let listened = arg_int(args, 0).unwrap_or(0) != 0;
            ctx.globals.bgm_table_all_flag = listened;
            ensure_bgm_flags_size(ctx);
            ctx.globals.bgm_table_flags.fill(listened);
            for v in ctx.globals.bgm_table_listened.values_mut() {
                *v = listened;
            }
            Ok(true)
        }
        _ => bail!("invalid BGMTABLE command opcode {op}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::VmCallMeta;
    use std::path::PathBuf;

    fn setup_call(ctx: &mut CommandContext, op: i32, ret_form: i64) {
        let form = if ctx.ids.form_global_bgm_table != 0 {
            ctx.ids.form_global_bgm_table
        } else {
            crate::runtime::forms::codes::FORM_GLOBAL_BGM_TABLE
        };
        ctx.vm_call = Some(VmCallMeta {
            element: vec![form as i32, op],
            al_id: 0,
            ret_form,
        });
    }

    fn test_context() -> CommandContext {
        let mut ctx = CommandContext::new(PathBuf::from("."));
        ctx.tables.gameexe = Some(crate::formats::gameexe::GameexeConfig::from_text(
            "#BGM.000=\"BGM079\",\"bgm079\"\n",
        ));
        ctx.globals.bgm_table_flags = vec![true];
        ctx
    }

    #[test]
    fn get_listen_by_name_takes_opcode_from_vm_call() {
        let mut ctx = test_context();
        setup_call(&mut ctx, bgm_table_op::GET_LISTEN_BY_NAME, 10);

        assert!(dispatch(&mut ctx, &[Value::Str("bgm079".into())]).unwrap());
        assert_eq!(ctx.stack.pop().and_then(|value| value.as_i64()), Some(1));
    }

    #[test]
    fn set_listen_by_name_uses_script_arguments_without_stack_result() {
        let mut ctx = test_context();
        ctx.globals.bgm_table_flags[0] = false;
        setup_call(&mut ctx, bgm_table_op::SET_LISTEN_CURRENT, 0);

        assert!(dispatch(&mut ctx, &[Value::Str("BGM079".into()), Value::Int(1)],).unwrap());
        assert!(ctx.globals.bgm_table_flags[0]);
        assert!(ctx.stack.is_empty());
    }
}
