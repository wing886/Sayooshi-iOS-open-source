use crate::runtime::{CommandContext, ProcKind, Value};
use anyhow::Result;

pub fn dispatch(ctx: &mut CommandContext, allow_key_skip: bool, args: &[Value]) -> Result<bool> {
    let ms = args.first().and_then(|v| v.as_i64()).unwrap_or(0).max(0) as u64;
    if ms == 0 {
        // C++ tnm_time_wait_proc completes immediately and, for
        // TIMEWAIT_KEY, pushes the timeout result before returning.
        if allow_key_skip {
            ctx.push(Value::Int(0));
        }
        return Ok(true);
    }

    if allow_key_skip {
        ctx.wait.wait_ms_key(ms);
    } else {
        ctx.wait.wait_ms(ms);
    }
    ctx.request_wait_proc_boundary(ProcKind::TimeWait);

    Ok(true)
}
