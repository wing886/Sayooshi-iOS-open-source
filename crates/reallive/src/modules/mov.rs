//! Movie playback (1:26).

use anyhow::{Context, Result};

use crate::bytecode::Command;
use crate::machine::{LongOp, Machine, Next};
use crate::movie::Movie;
use crate::resource::Kind;
use crate::surface::Rect;

/// `movWait` and the blocking variants.
#[derive(Debug)]
struct MovieWait {
    cancellable: bool,
}

impl LongOp for MovieWait {
    fn step(&mut self, machine: &mut Machine) -> Result<bool> {
        let sys = &mut machine.sys;
        if sys.movie.is_none() {
            return Ok(true);
        }
        if self.cancellable && sys.input.take_click().is_some() {
            sys.stop_movie();
            return Ok(true);
        }
        Ok(false)
    }

    fn name(&self) -> &'static str {
        "movie"
    }
}

pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let opcode = command.op.opcode;
    match opcode {
        // movPlay, movPlayEx, movLoop, movPlayExC
        0..=2 | 20 => {
            let name = machine.str_param(command, 0)?;
            let values = machine.int_params_from(command, 1)?;
            let dest = (values.len() >= 4 && values[..4].iter().any(|&v| v != 0))
                .then(|| Rect::from_corners(values[0], values[1], values[2], values[3]));
            let path = machine
                .sys
                .resources
                .find(Kind::Movie, &name)
                .with_context(|| format!("reallive: movie {name:?} not found"))?;
            let now = machine.sys.now();
            machine.sys.stop_movie();
            machine.sys.movie = Some(Movie::open(path, dest, opcode == 2, now));
            if opcode == 1 || opcode == 20 {
                machine.push_long_op(Box::new(MovieWait {
                    cancellable: opcode == 20,
                }));
            }
        }
        3 => machine.push_long_op(Box::new(MovieWait { cancellable: false })),
        // MOVWAITKEY: wait for the movie; a click ends it.
        21 => machine.push_long_op(Box::new(MovieWait { cancellable: true })),
        // SET_WMP_VOL / SET_WMP_STOP_MOD: settings of the Windows Media
        // player the original used for some formats; remembered only.
        1000 | 1001 => {
            let value = machine.int_param(command, 0)?;
            machine
                .sys
                .remembered
                .insert(26000 + command.op.opcode, value);
        }
        4 => machine.store = i32::from(machine.sys.movie.is_some()),
        5 => machine.sys.stop_movie(),
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}
