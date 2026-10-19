//! Music (1:20), PCM channels (1:21), sound effects (1:22) and voices
//! (1:23).

use anyhow::Result;

use crate::bytecode::Command;
use crate::longop::{Wait, WaitEvent};
use crate::machine::{Machine, Next};
use crate::sound::WAV_CHANNELS;

fn ms(value: i32) -> u64 {
    value.max(0) as u64
}

fn wait(machine: &mut Machine, event: WaitEvent, cancellable: bool) {
    let wait = Wait::event(event);
    let wait = if cancellable {
        wait.cancellable()
    } else {
        wait
    };
    machine.push_long_op(Box::new(wait));
}

/// Errors from missing or undecodable files are reported, not fatal.
fn report(machine: &mut Machine, result: Result<()>) {
    if let Err(error) = result {
        machine.report(format!(
            "SEEN{:04} line {}: {error:#}",
            machine.scene_number(),
            machine.line
        ));
    }
}

pub fn bgm(machine: &mut Machine, command: &Command) -> Result<Next> {
    let now = machine.sys.now();
    match command.op.opcode {
        // bgmLoop, bgmPlayEx, bgmPlay
        0..=2 => {
            let name = machine.str_param(command, 0)?;
            let fade_in = ms(machine.int_param_or(command, 1, 0)?);
            let fade_out = ms(machine.int_param_or(command, 2, 0)?);
            let looped = command.op.opcode != 2;
            let sys = &mut machine.sys;
            let result = sys.sound.bgm_play(
                &sys.resources,
                &sys.settings,
                now,
                &name,
                looped,
                fade_in,
                fade_out,
            );
            report(machine, result);
            if command.op.opcode == 1 {
                wait(machine, WaitEvent::Bgm, false);
            }
        }
        3 => wait(machine, WaitEvent::Bgm, false),
        4 => machine.store = i32::from(machine.sys.sound.bgm_status(now) == 1),
        5 | 6 => machine.sys.sound.bgm_stop(),
        7 | 107 => machine.store = machine.sys.sound.bgm_status(now),
        8 => {
            let sys = &mut machine.sys;
            let result = sys.sound.bgm_rewind(&sys.resources, &sys.settings, now);
            report(machine, result);
        }
        9 => machine.sys.sound.bgm_pause(now),
        10 => machine.sys.sound.bgm_resume(now),
        11 => machine.store = machine.sys.sound.bgm_volume(now),
        12 => {
            let volume = machine.int_param(command, 0)?;
            let fade = ms(machine.int_param_or(command, 1, 0)?);
            machine.sys.sound.set_bgm_volume(now, volume, fade);
        }
        13 => {
            let fade = ms(machine.int_param_or(command, 0, 0)?);
            machine.sys.sound.set_bgm_volume(now, 255, fade);
        }
        14 => {
            let fade = ms(machine.int_param_or(command, 0, 0)?);
            machine.sys.sound.set_bgm_volume(now, 0, fade);
        }
        105 => {
            let fade = ms(machine.int_param_or(command, 0, 1000)?);
            machine.sys.sound.bgm_fade_out(now, fade);
        }
        106 => {
            let fade = ms(machine.int_param_or(command, 0, 1000)?);
            machine.sys.sound.bgm_fade_out(now, fade);
            wait(machine, WaitEvent::Bgm, false);
        }
        // bgmTimer: milliseconds since the track started.
        200 => machine.store = 0,
        // MCHANGELOOP / MCHANGEONESHOT: the playing track loops or not.
        15 | 16 => machine
            .sys
            .sound
            .set_bgm_looped(command.op.opcode == 15, now),
        // MCHANGE(name): another track, looping as the current one does.
        300 => {
            let name = machine.str_param(command, 0)?;
            let sys = &mut machine.sys;
            let looped = sys.sound.bgm_status(now) == 0 || sys.sound.bgm_looped();
            let result =
                sys.sound
                    .bgm_play(&sys.resources, &sys.settings, now, &name, looped, 0, 0);
            report(machine, result);
        }
        // DEBUG_MPLAY(_WAIT, _ONESHOT)_MILLISECOND / _SECOND / _SAMPLE(name,
        // position): music from a point in the track.
        1000..=1002 | 2000..=2002 | 3000..=3002 => {
            use crate::sound::StartAt;
            let name = machine.str_param(command, 0)?;
            let at = machine.int_param_or(command, 1, 0)?.max(0);
            let from = match command.op.opcode / 1000 {
                1 => StartAt::Ms(at as u64),
                2 => StartAt::Ms(at as u64 * 1000),
                _ => StartAt::Frame(at as usize),
            };
            let variant = command.op.opcode % 1000;
            let sys = &mut machine.sys;
            let result = sys.sound.bgm_play_from(
                &sys.resources,
                &sys.settings,
                now,
                &name,
                variant == 0,
                0,
                0,
                Some(from),
            );
            report(machine, result);
            if variant == 1 {
                wait(machine, WaitEvent::Bgm, false);
            }
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

fn channel(machine: &mut Machine, command: &Command, index: usize) -> Result<usize> {
    Ok(machine
        .int_param(command, index)?
        .clamp(0, WAV_CHANNELS as i32 - 1) as usize)
}

pub fn pcm(machine: &mut Machine, command: &Command) -> Result<Next> {
    let now = machine.sys.now();
    match command.op.opcode {
        // wavPlay, wavPlayEx, wavLoop
        0..=2 => {
            let name = machine.str_param(command, 0)?;
            let channel = match Machine::param_count(command) {
                0 | 1 => None,
                _ => Some(channel(machine, command, 1)?),
            };
            let fade_in = ms(machine.int_param_or(command, 2, 0)?);
            let looped = command.op.opcode == 2;
            let sys = &mut machine.sys;
            let played = sys.sound.wav_play(
                &sys.resources,
                &sys.settings,
                now,
                &name,
                channel,
                looped,
                fade_in,
            );
            match played {
                Ok(channel) if command.op.opcode == 1 => {
                    wait(machine, WaitEvent::Wav(channel), false)
                }
                Ok(_) => {}
                Err(error) => report(machine, Err(error)),
            }
        }
        3 => {
            let channel = channel(machine, command, 0)?;
            wait(machine, WaitEvent::Wav(channel), false);
        }
        4 | 7 => {
            let channel = channel(machine, command, 0)?;
            machine.store = i32::from(machine.sys.sound.wav_playing(channel, now));
        }
        5 if Machine::param_count(command) == 0 => machine.sys.sound.wav_stop_all(0, now),
        5 | 9 | 10 => {
            let channel = channel(machine, command, 0)?;
            machine.sys.sound.wav_stop(channel, 0, now);
        }
        11 => {
            let channel = channel(machine, command, 0)?;
            machine.store = machine.sys.sound.wav_volume(channel, now);
        }
        12 => {
            let channel = channel(machine, command, 0)?;
            let volume = machine.int_param(command, 1)?;
            let fade = ms(machine.int_param_or(command, 2, 0)?);
            machine.sys.sound.set_wav_volume(channel, now, volume, fade);
        }
        13 | 14 => {
            let channel = channel(machine, command, 0)?;
            let fade = ms(machine.int_param_or(command, 1, 0)?);
            let volume = if command.op.opcode == 13 { 255 } else { 0 };
            machine.sys.sound.set_wav_volume(channel, now, volume, fade);
        }
        20 => machine.sys.sound.wav_stop_all(0, now),
        // PCMEVENT_LOOP / _RANDOM / _ONESHOT(event, (name, [weight,] min, max)...)
        40..=42 => {
            use crate::pcm_event::{Entry, Mode, PcmEvent};
            let number = machine.int_param(command, 0)?;
            let mode = match command.op.opcode {
                40 => Mode::Loop,
                41 => Mode::Random,
                _ => Mode::OneShot,
            };
            let mut entries = Vec::new();
            for index in 1..command.params.len() {
                let pieces = machine.complex_param(command, index)?;
                let Some(name) = pieces.first() else { continue };
                let name = machine.eval_str(name)?;
                let ints = pieces[1..]
                    .iter()
                    .map(|piece| machine.eval_int(piece))
                    .collect::<Result<Vec<_>>>()?;
                let (weight, waits) = if mode == Mode::Random && ints.len() >= 3 {
                    (ints[0], &ints[1..])
                } else {
                    (1, &ints[..])
                };
                entries.push(Entry {
                    name,
                    weight,
                    wait_min: waits.first().copied().unwrap_or(0),
                    wait_max: waits.get(1).copied().unwrap_or(0),
                });
            }
            if let Some(mut old) = machine.sys.pcm_events.remove(&number) {
                old.stop(&mut machine.sys, true);
            }
            machine
                .sys
                .pcm_events
                .insert(number, PcmEvent::new(mode, entries));
        }
        // PCMEVENT_STOP(event, stop sound) / PCMEVENT_STOPALL(stop sound)
        50 | 51 => {
            let (numbers, stop_sound) = if command.op.opcode == 50 {
                let number = machine.int_param(command, 0)?;
                (vec![number], machine.int_param_or(command, 1, 0)? != 0)
            } else {
                let all = machine.sys.pcm_events.keys().copied().collect();
                (all, machine.int_param_or(command, 0, 0)? != 0)
            };
            for number in numbers {
                if let Some(mut event) = machine.sys.pcm_events.remove(&number) {
                    event.stop(&mut machine.sys, stop_sound);
                }
            }
        }
        // PCMEVENT_CHECK(event)
        52 => {
            let number = machine.int_param(command, 0)?;
            machine.store = i32::from(machine.sys.pcm_events.contains_key(&number));
        }
        // PCMEVENT_WAIT(event)
        53 => {
            let number = machine.int_param(command, 0)?;
            machine.push_long_op(Box::new(crate::longop::Wait::event(WaitEvent::PcmEvent(
                number,
            ))));
        }
        // PCMBUF_LOAD / PCMBUF_FREE / PCMBUF_FREEALL: preloading; files are
        // read when played.
        1000..=1002 => {}
        // wavRewind(channel)
        8 => {
            let channel = channel(machine, command, 0)?;
            let sys = &mut machine.sys;
            let result = sys
                .sound
                .wav_rewind(&sys.resources, &sys.settings, now, channel);
            report(machine, result);
        }
        // PCMCHANGELOOP / PCMCHANGEONESHOT(channel)
        15 | 16 => {
            let channel = channel(machine, command, 0)?;
            machine
                .sys
                .sound
                .set_wav_looped(channel, command.op.opcode == 15, now);
        }
        // PCMSTOPALLMAIN / PCMSTOPALLEXTRA: the first / second half of the
        // channels.
        21 | 22 => {
            let half = WAV_CHANNELS / 2;
            let range = if command.op.opcode == 21 {
                0..half
            } else {
                half..WAV_CHANNELS
            };
            for channel in range {
                machine.sys.sound.wav_stop(channel, 0, now);
            }
        }
        // PCMVOLSETALL(volume) / PCMVOLMAXALL(time) / PCMVOLMINALL(time)
        32..=34 => {
            let (volume, fade) = match command.op.opcode {
                32 => (machine.int_param(command, 0)?, 0),
                33 => (255, ms(machine.int_param_or(command, 0, 0)?)),
                _ => (0, ms(machine.int_param_or(command, 0, 0)?)),
            };
            for channel in 0..WAV_CHANNELS {
                machine.sys.sound.set_wav_volume(channel, now, volume, fade);
            }
        }
        // PCMFADECHECK(channel)
        107 => {
            let channel = channel(machine, command, 0)?;
            machine.store = i32::from(machine.sys.sound.wav_fading(channel, now));
        }
        // PCMPLAYWAITKEY(name, channel) / PCMWAITKEY(channel): a click ends
        // the wait.
        201 => {
            let name = machine.str_param(command, 0)?;
            let channel = channel(machine, command, 1)?;
            let sys = &mut machine.sys;
            let played = sys.sound.wav_play(
                &sys.resources,
                &sys.settings,
                now,
                &name,
                Some(channel),
                false,
                0,
            );
            match played {
                Ok(channel) => wait(machine, WaitEvent::Wav(channel), true),
                Err(error) => report(machine, Err(error)),
            }
        }
        203 => {
            let channel = channel(machine, command, 0)?;
            wait(machine, WaitEvent::Wav(channel), true);
        }
        105 | 106 => {
            let channel = channel(machine, command, 0)?;
            let fade = ms(machine.int_param_or(command, 1, 1000)?);
            machine.sys.sound.wav_stop(channel, fade, now);
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

pub fn se(machine: &mut Machine, command: &Command) -> Result<Next> {
    match command.op.opcode {
        0 => {
            let number = machine.int_param(command, 0)?;
            machine.sys.play_se(number);
        }
        // SEVOLGET / SEVOLSET(volume) / SEVOLMAX(time) / SEVOLMIN(time)
        11 => machine.store = machine.sys.sound.se_volume(machine.sys.now()),
        12..=14 => {
            let now = machine.sys.now();
            let (volume, fade) = match command.op.opcode {
                12 => (machine.int_param(command, 0)?, 0),
                13 => (255, ms(machine.int_param_or(command, 0, 0)?)),
                _ => (0, ms(machine.int_param_or(command, 0, 0)?)),
            };
            machine.sys.sound.set_se_volume(now, volume, fade);
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

/// Starts a voice and records it for the backlog.
pub(crate) fn koe_play(machine: &mut Machine, id: i32, character: Option<i32>) {
    machine.sys.last_koe = (id, character.unwrap_or(0));
    let active = machine.sys.text.active;
    machine.sys.text.current_page.voices.push(id);
    let state = &mut machine.sys.text.states[active];
    let (x, y) = (state.x, state.y);
    state.koe_markers.push((id, x, y));
    let sys = &mut machine.sys;
    if sys.should_fast_forward() {
        return;
    }
    if let Some(character) = character {
        if !sys.sound.character_enabled(&sys.settings, character) {
            return;
        }
    }
    let now = sys.now();
    let result = sys.sound.koe_play(&sys.resources, &sys.settings, now, id);
    report(machine, result);
}

pub fn koe(machine: &mut Machine, command: &Command) -> Result<Next> {
    let now = machine.sys.now();
    match command.op.opcode {
        // koePlay, koePlayEx, koePlayExC and the "Do" variants, which play
        // whatever the character switches say.
        0 | 1 | 7..=10 => {
            let id = machine.int_param(command, 0)?;
            let character = match Machine::param_count(command) {
                0 | 1 => None,
                _ => Some(machine.int_param(command, 1)?),
            };
            let honour_switches = matches!(command.op.opcode, 0 | 1 | 7);
            koe_play(machine, id, character.filter(|_| honour_switches));
            match command.op.opcode {
                1 | 9 => wait(machine, WaitEvent::Koe, false),
                7 | 10 => wait(machine, WaitEvent::Koe, true),
                _ => {}
            }
        }
        3 => wait(machine, WaitEvent::Koe, false),
        4 => machine.store = i32::from(machine.sys.sound.koe_playing(now)),
        // KOE_REPLAY_CLEAR: drop the voice-replay markers.
        101 => {
            for state in &mut machine.sys.text.states {
                state.koe_markers.clear();
            }
        }
        5 => machine.sys.sound.koe_stop(),
        6 => wait(machine, WaitEvent::Koe, true),
        11 => machine.store = machine.sys.sound.koe_volume(now),
        12 => {
            let volume = machine.int_param(command, 0)?;
            let fade = ms(machine.int_param_or(command, 1, 0)?);
            machine.sys.sound.set_koe_volume(now, volume, fade);
        }
        13 | 14 => {
            let fade = ms(machine.int_param_or(command, 0, 0)?);
            let volume = if command.op.opcode == 13 { 255 } else { 0 };
            machine.sys.sound.set_koe_volume(now, volume, fade);
        }
        // KOE_DONT_MEMORY / KOEPLAYWAIT_ / KOEPLAYWAITKEY_ and the EX forms
        // (which ignore the character switches): a voice that is not
        // remembered for replay or the backlog.
        15..=20 => {
            let id = machine.int_param(command, 0)?;
            let sys = &mut machine.sys;
            if !sys.should_fast_forward() {
                let result = sys.sound.koe_play(&sys.resources, &sys.settings, now, id);
                report(machine, result);
            }
            match (command.op.opcode - 15) % 3 {
                1 => wait(machine, WaitEvent::Koe, false),
                2 => wait(machine, WaitEvent::Koe, true),
                _ => {}
            }
        }
        // SET_KOEFILEMODE(mode): which voice archives to read; any is read.
        100 => {
            let mode = machine.int_param(command, 0)?;
            machine.sys.remembered.insert(23100, mode);
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}
