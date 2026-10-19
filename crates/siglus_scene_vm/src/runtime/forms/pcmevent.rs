//! Global PCMEVENT form.
//!
//! This is a direct port of `C_elm_pcm_event`: an event owns a line scheduler
//! and may target either the independent global PCM player (`pcm_buf_no == -1`)
//! or one PCMCH channel.

use anyhow::Result;

use crate::runtime::globals::{
    PCM_EVENT_TYPE_LOOP, PCM_EVENT_TYPE_NONE, PCM_EVENT_TYPE_ONESHOT, PCM_EVENT_TYPE_RANDOM,
    PcmEventLine, PcmEventState,
};
use crate::runtime::{CommandContext, Value};

#[derive(Clone, Copy)]
enum PcmEventOp {
    StartOneShot,
    StartLoop,
    StartRandom,
    Stop,
    Check,
    Wait,
    WaitKey,
    Unknown,
}

fn resolve_pcm_event_op(op: i32) -> PcmEventOp {
    match op {
        crate::runtime::constants::PCMEVENT_START_ONESHOT => PcmEventOp::StartOneShot,
        crate::runtime::constants::PCMEVENT_START_LOOP => PcmEventOp::StartLoop,
        crate::runtime::constants::PCMEVENT_START_RANDOM => PcmEventOp::StartRandom,
        crate::runtime::constants::PCMEVENT_STOP => PcmEventOp::Stop,
        crate::runtime::constants::PCMEVENT_CHECK => PcmEventOp::Check,
        crate::runtime::constants::PCMEVENT_WAIT_KEY => PcmEventOp::WaitKey,
        crate::runtime::constants::PCMEVENT_WAIT => PcmEventOp::Wait,
        _ => PcmEventOp::Unknown,
    }
}

fn named_int(args: &[Value], id: i32) -> Option<i64> {
    args.iter().find_map(|v| match v {
        Value::NamedArg { id: nid, value } if *nid == id => value.as_i64(),
        _ => None,
    })
}

fn positional_pcm_buf_no(args: &[Value]) -> i32 {
    args.first()
        .and_then(|value| match value {
            Value::Int(value) => Some(*value as i32),
            _ => None,
        })
        .unwrap_or(-1)
}

fn collect_lines(args: &[Value], random: bool) -> Vec<PcmEventLine> {
    let mut out = Vec::new();
    for v in args {
        match v {
            Value::Str(s) => out.push(PcmEventLine {
                file_name: s.clone(),
                probability: if random { 1 } else { 0 },
                min_time: 0,
                max_time: 0,
            }),
            Value::List(items) if !items.is_empty() => {
                let file_name = items
                    .first()
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                if file_name.is_empty() {
                    continue;
                }
                let mut line = PcmEventLine {
                    file_name,
                    probability: if random { 1 } else { 0 },
                    min_time: 0,
                    max_time: 0,
                };
                if random {
                    if let Some(v) = items.get(1).and_then(Value::as_i64) {
                        line.probability = v as i32;
                    }
                    if let Some(v) = items.get(2).and_then(Value::as_i64) {
                        line.min_time = v as i32;
                        line.max_time = v as i32;
                    }
                    if let Some(v) = items.get(3).and_then(Value::as_i64) {
                        line.max_time = v as i32;
                    }
                } else {
                    if let Some(v) = items.get(1).and_then(Value::as_i64) {
                        line.min_time = v as i32;
                        line.max_time = v as i32;
                    }
                    if let Some(v) = items.get(2).and_then(Value::as_i64) {
                        line.max_time = v as i32;
                    }
                }
                out.push(line);
            }
            _ => {}
        }
    }
    out
}

pub fn dispatch(ctx: &mut CommandContext, args: &[Value]) -> Result<bool> {
    let form_global_pcm_event = ctx.ids.form_global_pcm_event;
    let elm_array = ctx.ids.elm_array;
    let Some((chain_pos, chain)) = crate::runtime::forms::prop_access::parse_element_chain_ctx(
        ctx,
        form_global_pcm_event,
        args,
    ) else {
        return Ok(false);
    };
    let chain = chain.to_vec();
    if chain.len() < 3 || chain[1] != elm_array {
        return Ok(false);
    }
    let idx = chain[2].max(0) as usize;

    {
        let list = ctx
            .globals
            .pcm_event_lists
            .entry(form_global_pcm_event)
            .or_default();
        if list.len() <= idx {
            list.resize(idx + 1, PcmEventState::default());
        }
    }

    if chain.len() == 3 {
        return Ok(true);
    }

    let op = resolve_pcm_event_op(chain[3]);
    let script_args = if chain_pos == args.len() {
        crate::runtime::forms::prop_access::script_args(args, chain_pos)
    } else {
        &args[..chain_pos]
    };
    match op {
        PcmEventOp::StartOneShot | PcmEventOp::StartLoop | PcmEventOp::StartRandom => {
            let event_type = match op {
                PcmEventOp::StartOneShot => PCM_EVENT_TYPE_ONESHOT,
                PcmEventOp::StartLoop => PCM_EVENT_TYPE_LOOP,
                PcmEventOp::StartRandom => PCM_EVENT_TYPE_RANDOM,
                _ => unreachable!(),
            };
            let random = event_type == PCM_EVENT_TYPE_RANDOM;
            let lines = collect_lines(script_args, random);
            let pcm_buf_no = positional_pcm_buf_no(script_args);
            let volume_type = named_int(script_args, 3)
                .filter(|value| (-1..32).contains(value))
                .unwrap_or(2) as i32;
            let chara_no = named_int(script_args, 6).unwrap_or(-1) as i32;
            let bgm_fade_target = named_int(script_args, 4).unwrap_or(0) != 0;
            let bgm_fade2_target = named_int(script_args, 5).unwrap_or(0) != 0;
            let time_type = named_int(script_args, 11).unwrap_or(0) != 0;
            let bgm_fade_source = named_int(script_args, 12).unwrap_or(0) != 0;

            if let Some(st) = ctx
                .globals
                .pcm_event_lists
                .get_mut(&form_global_pcm_event)
                .and_then(|v| v.get_mut(idx))
            {
                st.reinit();
                st.lines = lines;
                // cmd_sound.cpp initializes this to true and exposes no named
                // argument that changes it.
                st.start(
                    event_type,
                    pcm_buf_no,
                    volume_type,
                    chara_no,
                    bgm_fade_target,
                    bgm_fade2_target,
                    bgm_fade_source,
                    true,
                    time_type,
                );
            }
            Ok(true)
        }
        PcmEventOp::Stop => {
            let stop_pcm = script_args.first().and_then(Value::as_i64).unwrap_or(0) != 0;
            let (event_active, pcm_buf_no) = ctx
                .globals
                .pcm_event_lists
                .get(&form_global_pcm_event)
                .and_then(|v| v.get(idx))
                .map(|st| (st.is_active(), st.pcm_buf_no))
                .unwrap_or((false, -1));
            if stop_pcm {
                if pcm_buf_no == -1 {
                    // The C++ special case avoids stopping the global PCM
                    // player when this event itself is already inactive.
                    if event_active {
                        let _ = ctx.pcm.stop(None);
                    }
                } else if pcm_buf_no >= 0 {
                    let _ = ctx.pcm.stop_slot(pcm_buf_no as usize, None);
                }
            }
            if let Some(st) = ctx
                .globals
                .pcm_event_lists
                .get_mut(&form_global_pcm_event)
                .and_then(|v| v.get_mut(idx))
            {
                st.reinit();
            }
            crate::runtime::forms::syscom::update_audio_routing(ctx, 0, true);
            Ok(true)
        }
        PcmEventOp::Check => {
            let active = ctx
                .globals
                .pcm_event_lists
                .get(&form_global_pcm_event)
                .and_then(|v| v.get(idx))
                .map(PcmEventState::is_active)
                .unwrap_or(false);
            ctx.push(Value::Int(if active { 1 } else { 0 }));
            Ok(true)
        }
        PcmEventOp::Wait | PcmEventOp::WaitKey => {
            let oneshot = ctx
                .globals
                .pcm_event_lists
                .get(&form_global_pcm_event)
                .and_then(|v| v.get(idx))
                .is_some_and(|st| st.event_type == PCM_EVENT_TYPE_ONESHOT);
            if oneshot {
                let key = matches!(op, PcmEventOp::WaitKey);
                ctx.wait
                    .wait_pcm_event(form_global_pcm_event, idx, key, key);
            }
            Ok(true)
        }
        PcmEventOp::Unknown => Ok(false),
    }
}

#[derive(Debug, Clone)]
struct PendingEventPlay {
    pcm_buf_no: i32,
    file_name: String,
    volume_type: i32,
    chara_no: i32,
    bgm_fade_target_flag: bool,
    bgm_fade2_target_flag: bool,
    bgm_fade_source_flag: bool,
    time_type: bool,
    min_time: i32,
    max_time: i32,
}

fn next_random(state: &mut u32) -> u32 {
    let mut value = if *state == 0 { 0x1234_5678 } else { *state };
    value ^= value << 13;
    value ^= value >> 17;
    value ^= value << 5;
    *state = value;
    value
}

fn random_exclusive(state: &mut u32, min_value: i64, max_value: i64) -> i64 {
    if max_value <= min_value {
        return min_value;
    }
    let span = (max_value - min_value) as u64;
    min_value + (u64::from(next_random(state)) % span) as i64
}

fn select_random_line(st: &mut PcmEventState, rng_state: &mut u32) -> Option<usize> {
    let line_count = st.lines.len();
    if line_count == 0 {
        return None;
    }
    st.last_line_no = st.cur_line_no;
    let total: i64 = st
        .lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let probability = i64::from(line.probability.max(0));
            if index as i32 == st.last_line_no {
                probability
            } else {
                probability.saturating_mul(2)
            }
        })
        .sum();
    if total <= 0 {
        st.cur_line_no = 0;
        return Some(0);
    }
    let pos = random_exclusive(rng_state, 0, total);
    let mut current = 0i64;
    for (index, line) in st.lines.iter().enumerate() {
        let probability = i64::from(line.probability.max(0));
        current += if index as i32 == st.last_line_no {
            probability
        } else {
            probability.saturating_mul(2)
        };
        if pos < current {
            st.cur_line_no = index as i32;
            return Some(index);
        }
    }
    st.cur_line_no = (line_count - 1) as i32;
    Some(line_count - 1)
}

fn prepare_next_play(st: &mut PcmEventState, rng_state: &mut u32) -> Option<PendingEventPlay> {
    if !st.is_active() || st.cur_time - st.next_time < 0 {
        return None;
    }
    let index = match st.event_type {
        PCM_EVENT_TYPE_ONESHOT => {
            st.cur_line_no += 1;
            if st.cur_line_no < 0 || st.cur_line_no as usize >= st.lines.len() {
                st.reinit();
                return None;
            }
            st.cur_line_no as usize
        }
        PCM_EVENT_TYPE_LOOP => {
            if st.lines.is_empty() {
                st.reinit();
                return None;
            }
            st.cur_line_no = (st.cur_line_no + 1).rem_euclid(st.lines.len() as i32);
            st.cur_line_no as usize
        }
        PCM_EVENT_TYPE_RANDOM => match select_random_line(st, rng_state) {
            Some(index) => index,
            None => {
                st.reinit();
                return None;
            }
        },
        _ => return None,
    };
    st.cur_time = 0;
    let line = st.lines[index].clone();
    Some(PendingEventPlay {
        pcm_buf_no: st.pcm_buf_no,
        file_name: line.file_name,
        volume_type: st.volume_type,
        chara_no: st.chara_no,
        bgm_fade_target_flag: st.bgm_fade_target_flag,
        bgm_fade2_target_flag: st.bgm_fade2_target_flag,
        bgm_fade_source_flag: st.bgm_fade_source_flag,
        time_type: st.time_type,
        min_time: line.min_time,
        max_time: line.max_time,
    })
}

fn play_pending(ctx: &mut CommandContext, pending: &PendingEventPlay) -> u64 {
    if pending.pcm_buf_no == -1 {
        let result = {
            let (pcm, audio) = (&mut ctx.pcm, &mut ctx.audio);
            pcm.play_file_name(audio, &pending.file_name)
        };
        if let Err(error) = result {
            log::error!(
                "PCMEVENT failed to play global PCM {:?}: {error:#}",
                pending.file_name
            );
            0
        } else {
            ctx.pcm.global_duration_ms()
        }
    } else if pending.pcm_buf_no >= 0 {
        match crate::runtime::forms::pcmch::play_event_line(
            ctx,
            pending.pcm_buf_no as usize,
            &pending.file_name,
            pending.volume_type,
            pending.chara_no,
            pending.bgm_fade_target_flag,
            pending.bgm_fade2_target_flag,
            pending.bgm_fade_source_flag,
        ) {
            Ok(duration) => duration,
            Err(error) => {
                log::error!(
                    "PCMEVENT failed to play PCMCH[{}] {:?}: {error:#}",
                    pending.pcm_buf_no,
                    pending.file_name
                );
                0
            }
        }
    } else {
        0
    }
}

/// Advance all PCMEVENT schedulers once, matching `C_elm_pcm_event::frame`.
pub(crate) fn tick_all(ctx: &mut CommandContext, game_delta_ms: i32, real_delta_ms: i32) {
    let form_ids: Vec<u32> = ctx.globals.pcm_event_lists.keys().copied().collect();
    for form_id in form_ids {
        let event_count = ctx
            .globals
            .pcm_event_lists
            .get(&form_id)
            .map(Vec::len)
            .unwrap_or(0);
        for index in 0..event_count {
            if let Some(st) = ctx
                .globals
                .pcm_event_lists
                .get_mut(&form_id)
                .and_then(|events| events.get_mut(index))
                && st.is_active()
            {
                let delta = if st.real_flag {
                    real_delta_ms
                } else {
                    game_delta_ms
                };
                st.cur_time = st.cur_time.saturating_add(delta.max(0) as i64);
            }

            // Valid scripts provide a positive interval or include the sound
            // length. Keep a guard so malformed zero-interval LOOP data cannot
            // lock the native frame forever.
            for _ in 0..1024 {
                let pending = {
                    let (events, rng_state) =
                        (&mut ctx.globals.pcm_event_lists, &mut ctx.globals.rng_state);
                    events
                        .get_mut(&form_id)
                        .and_then(|events| events.get_mut(index))
                        .and_then(|st| prepare_next_play(st, rng_state))
                };
                let Some(pending) = pending else {
                    break;
                };
                let duration_ms = play_pending(ctx, &pending) as i64;
                let min_time = i64::from(pending.min_time.min(pending.max_time));
                let max_time = i64::from(pending.min_time.max(pending.max_time));
                let mut next_time =
                    random_exclusive(&mut ctx.globals.rng_state, min_time, max_time);
                if min_time == max_time {
                    next_time = max_time;
                }
                if pending.time_type {
                    next_time = next_time.saturating_add(duration_ms);
                }
                if let Some(st) = ctx
                    .globals
                    .pcm_event_lists
                    .get_mut(&form_id)
                    .and_then(|events| events.get_mut(index))
                {
                    st.next_time = next_time;
                }
            }
        }
    }
}
