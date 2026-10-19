//! Screen shaking (module 1:13) and layer shaking (module 1:12).

use anyhow::Result;

use crate::bytecode::Command;
use crate::machine::{LongOp, Machine, Next};
use crate::shake::{Layers, Motion, Shake, ShakeKind};

/// Waits for a shake to finish (the `Ex` variants and `shake()`).
#[derive(Debug)]
struct ShakeWait {
    layer: bool,
}

impl LongOp for ShakeWait {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let now = machine.sys.now();
        let gfx = &mut machine.sys.gfx;
        // Expires finished shakes.
        gfx.shake_offsets(now);
        let running = if self.layer {
            gfx.layer_shake.is_some()
        } else {
            gfx.screen_shake.is_some()
        };
        if machine.sys.should_fast_forward() {
            if self.layer {
                machine.sys.gfx.layer_shake = None;
            } else {
                machine.sys.gfx.screen_shake = None;
            }
            return Ok(true);
        }
        Ok(!running)
    }

    fn name(&self) -> &'static str {
        "shake"
    }
}

fn pattern(machine: &Machine, spec: i32) -> ShakeKind {
    let values = {
        let own = machine.gameexe.ints(&format!("SHAKE.{spec:03}"));
        if own.is_empty() {
            machine.gameexe.ints("SHAKE.000")
        } else {
            own
        }
    };
    ShakeKind::Pattern(values.chunks_exact(3).map(|c| (c[0], c[1], c[2])).collect())
}

/// `grp shake(spec)`: plays `#SHAKE.spec` once and waits for it.
pub fn shake_from_gameexe(machine: &mut Machine, spec: i32) -> Result<()> {
    let kind = pattern(machine, spec);
    let now = machine.sys.now();
    machine.sys.gfx.screen_shake = Some(Shake::new(kind, now, 1, 0, Layers::ALL));
    machine.push_long_op(Box::new(ShakeWait { layer: false }));
    Ok(())
}

fn motion_of(opcode: u16) -> Option<Motion> {
    Some(match opcode {
        100 => Motion::DownUp,
        101 => Motion::RightLeft,
        200 => Motion::Up,
        201 => Motion::Down,
        202 => Motion::Left,
        203 => Motion::Right,
        400 => Motion::Zoom,
        _ => return None,
    })
}

/// Both modules share numbering: 1xxx starts a shake, 3xxx starts it and
/// waits; xx100-xx203 are motions, xx102 is 2D, xx300 uses `#SHAKE`.
pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let layered = command.op.module == 12;
    let opcode = command.op.opcode;
    let now = machine.sys.now();
    if opcode == 0 {
        // ShakeStop([time]) / ShakeLayersStop([time][, flag])
        let time = machine.int_param_or(command, 0, 0)?.max(0) as u64;
        let gfx = &mut machine.sys.gfx;
        let slot = if layered {
            &mut gfx.layer_shake
        } else {
            &mut gfx.screen_shake
        };
        match slot {
            Some(shake) if time > 0 => shake.stop = Some((now, time)),
            _ => *slot = None,
        }
        return Ok(Next::Advance);
    }
    let wait = opcode >= 3000;
    let sub = opcode % 1000;
    let mut index = 0;
    let mut next = |machine: &mut Machine| -> Result<i32> {
        let value = machine.int_param(command, index)?;
        index += 1;
        Ok(value)
    };
    let kind = match sub {
        // QUAKE_CIRCLE(power1, time1, power2, time2, count, endCount,
        // window, text, background, objects[, buf[, waipCopy]])
        103 if layered => ShakeKind::Circle {
            h_amount: next(machine)?,
            h_speed: next(machine)?,
            v_amount: next(machine)?,
            v_speed: next(machine)?,
        },
        102 => ShakeKind::TwoD {
            h_amount: next(machine)?,
            h_speed: next(machine)?,
            v_amount: next(machine)?,
            v_speed: next(machine)?,
        },
        300 => {
            let spec = next(machine)?;
            pattern(machine, spec)
        }
        other => match motion_of(other) {
            Some(motion) => ShakeKind::Single {
                motion,
                amount: next(machine)?,
                speed: next(machine)?,
            },
            None => return machine.unimplemented(command),
        },
    };
    let rep = next(machine)?;
    let faderep = next(machine)?;
    let layers = if layered {
        Layers {
            window: next(machine)? != 0,
            text: next(machine)? != 0,
            background: next(machine)? != 0,
            objects: next(machine)? != 0,
        }
    } else {
        Layers::ALL
    };
    let shake = Shake::new(kind, now, rep, faderep, layers);
    if layered {
        machine.sys.gfx.layer_shake = Some(shake);
    } else {
        machine.sys.gfx.screen_shake = Some(shake);
    }
    if wait && (rep != 0 || faderep != 0) {
        machine.push_long_op(Box::new(ShakeWait { layer: layered }));
    }
    Ok(Next::Advance)
}
