//! Screen and layer shaking.
//!
//! A shake is either a `#SHAKE.nnn` pattern (`x, y, wait` triples, as in
//! AVG32) or a procedural movement: an oscillation along one or both axes,
//! a bounce in one direction, or a zoom pulse. Procedural shakes run `rep`
//! full cycles and then `faderep` cycles of decreasing extent; with both 0
//! they run until stopped.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    DownUp,
    RightLeft,
    Up,
    Down,
    Left,
    Right,
    Zoom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShakeKind {
    Pattern(Vec<(i32, i32, i32)>),
    /// One motion with an amount (pixels, or percent for zoom) and the
    /// duration of one cycle in ms.
    Single {
        motion: Motion,
        amount: i32,
        speed: i32,
    },
    /// Horizontal and vertical oscillation together.
    TwoD {
        h_amount: i32,
        h_speed: i32,
        v_amount: i32,
        v_speed: i32,
    },
    /// `QUAKE_CIRCLE`: horizontal (cosine) and vertical (sine) together,
    /// so equal amounts and periods trace a circle.
    Circle {
        h_amount: i32,
        h_speed: i32,
        v_amount: i32,
        v_speed: i32,
    },
}

/// Which parts of the screen a shake moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layers {
    pub window: bool,
    pub text: bool,
    pub background: bool,
    pub objects: bool,
}

impl Layers {
    pub const ALL: Layers = Layers {
        window: true,
        text: true,
        background: true,
        objects: true,
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shake {
    pub kind: ShakeKind,
    pub start: u64,
    pub rep: i32,
    pub faderep: i32,
    pub layers: Layers,
    /// `ShakeStop(time)`: fading out since `start` over `duration` ms.
    pub stop: Option<(u64, u64)>,
}

/// The displacement a shake applies at a moment.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Offset {
    pub dx: f64,
    pub dy: f64,
    /// Extra zoom factor (0 = none, 0.05 = 105 %).
    pub zoom: f64,
}

impl Shake {
    pub fn new(kind: ShakeKind, start: u64, rep: i32, faderep: i32, layers: Layers) -> Self {
        Self {
            kind,
            start,
            rep,
            faderep,
            layers,
            stop: None,
        }
    }

    fn cycle_length(&self) -> f64 {
        match &self.kind {
            ShakeKind::Pattern(steps) => steps.iter().map(|s| s.2.max(1)).sum::<i32>() as f64,
            ShakeKind::Single { speed, .. } => f64::from((*speed).max(1)),
            ShakeKind::TwoD {
                h_speed, v_speed, ..
            }
            | ShakeKind::Circle {
                h_speed, v_speed, ..
            } => f64::from((*h_speed).max(*v_speed).max(1)),
        }
    }

    /// Extent multiplier at elapsed time `elapsed`, or `None` when done.
    fn envelope(&self, elapsed: f64, now: u64) -> Option<f64> {
        let cycle = self.cycle_length();
        let mut extent = if self.rep == 0 && self.faderep == 0 {
            1.0
        } else {
            let cycles = elapsed / cycle;
            let (rep, fade) = (f64::from(self.rep.max(0)), f64::from(self.faderep.max(0)));
            if cycles < rep {
                1.0
            } else if cycles < rep + fade {
                1.0 - (cycles - rep) / fade
            } else {
                return None;
            }
        };
        if let Some((stop_start, duration)) = self.stop {
            if duration == 0 || now >= stop_start + duration {
                return None;
            }
            extent *= 1.0 - (now - stop_start) as f64 / duration as f64;
        }
        Some(extent)
    }

    pub fn offset(&self, now: u64) -> Option<Offset> {
        let elapsed = now.saturating_sub(self.start) as f64;
        if let ShakeKind::Pattern(steps) = &self.kind {
            // Patterns play once per repetition.
            let total = self.cycle_length();
            let repeats = f64::from(self.rep.max(1) + self.faderep.max(0));
            if elapsed >= total * repeats {
                return None;
            }
            let extent = self.envelope(elapsed, now).unwrap_or(0.0).max(0.0);
            let mut t = elapsed % total;
            for &(x, y, wait) in steps {
                let wait = f64::from(wait.max(1));
                if t < wait {
                    return Some(Offset {
                        dx: f64::from(x) * extent,
                        dy: f64::from(y) * extent,
                        zoom: 0.0,
                    });
                }
                t -= wait;
            }
            return Some(Offset::default());
        }
        let extent = self.envelope(elapsed, now)?;
        let wave = |speed: i32| {
            let phase = elapsed / f64::from(speed.max(1));
            (phase * std::f64::consts::TAU).sin()
        };
        let bounce = |speed: i32| {
            let phase = (elapsed / f64::from(speed.max(1))).fract();
            (phase * std::f64::consts::PI).sin()
        };
        Some(match &self.kind {
            ShakeKind::Single {
                motion,
                amount,
                speed,
            } => {
                let a = f64::from(*amount) * extent;
                match motion {
                    Motion::DownUp => Offset {
                        dy: a * wave(*speed),
                        ..Offset::default()
                    },
                    Motion::RightLeft => Offset {
                        dx: a * wave(*speed),
                        ..Offset::default()
                    },
                    Motion::Up => Offset {
                        dy: -a * bounce(*speed),
                        ..Offset::default()
                    },
                    Motion::Down => Offset {
                        dy: a * bounce(*speed),
                        ..Offset::default()
                    },
                    Motion::Left => Offset {
                        dx: -a * bounce(*speed),
                        ..Offset::default()
                    },
                    Motion::Right => Offset {
                        dx: a * bounce(*speed),
                        ..Offset::default()
                    },
                    Motion::Zoom => Offset {
                        zoom: a / 100.0 * bounce(*speed),
                        ..Offset::default()
                    },
                }
            }
            ShakeKind::TwoD {
                h_amount,
                h_speed,
                v_amount,
                v_speed,
            } => Offset {
                dx: f64::from(*h_amount) * extent * wave(*h_speed),
                dy: f64::from(*v_amount) * extent * wave(*v_speed),
                zoom: 0.0,
            },
            ShakeKind::Circle {
                h_amount,
                h_speed,
                v_amount,
                v_speed,
            } => {
                let phase = |speed: i32| elapsed / f64::from(speed.max(1)) * std::f64::consts::TAU;
                Offset {
                    dx: f64::from(*h_amount) * extent * phase(*h_speed).cos(),
                    dy: f64::from(*v_amount) * extent * phase(*v_speed).sin(),
                    zoom: 0.0,
                }
            }
            ShakeKind::Pattern(_) => unreachable!("handled above"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns_step_and_end() {
        let shake = Shake::new(
            ShakeKind::Pattern(vec![(4, 0, 10), (-4, 0, 10), (0, 0, 10)]),
            0,
            1,
            0,
            Layers::ALL,
        );
        assert_eq!(shake.offset(5).unwrap().dx, 4.0);
        assert_eq!(shake.offset(15).unwrap().dx, -4.0);
        assert!(shake.offset(30).is_none());
    }

    #[test]
    fn fading_repetitions_shrink() {
        let shake = Shake::new(
            ShakeKind::Single {
                motion: Motion::Down,
                amount: 10,
                speed: 100,
            },
            0,
            1,
            2,
            Layers::ALL,
        );
        let full = shake.offset(50).unwrap().dy;
        let fading = shake.offset(250).unwrap().dy;
        assert!(full > fading && fading > 0.0);
        assert!(shake.offset(300).is_none());
        let endless = Shake::new(
            ShakeKind::Single {
                motion: Motion::RightLeft,
                amount: 5,
                speed: 100,
            },
            0,
            0,
            0,
            Layers::ALL,
        );
        assert!(endless.offset(1_000_000).is_some());
    }
}
