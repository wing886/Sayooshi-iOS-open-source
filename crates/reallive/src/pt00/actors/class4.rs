//! Actor 4 (vtable `0x1001C1D0`), also the pattern for actor 5.

use super::super::base::{arc, direction8, segment_foot};
use super::super::{BALL, Pt00, ftol};

impl Pt00 {
    /// `sub_1000AE10`: plays scene animation set `set` (used on actors 3
    /// and 5).
    pub(crate) fn f_1000ae10(&mut self, this: u32, set: i32) {
        self.sr(this, 16, 0);
        self.vcall(this, 20, 2, 0);
        self.sw(this, 311, set);
    }

    /// `sub_1000AE40` (slot 7).
    pub(crate) fn f_1000ae40(&mut self, this: u32) {
        match self.r(this, 24) {
            0 => {
                let state = if self.rand() % 3 > 1 { 3 } else { 2 };
                self.vcall(this, 20, state, 0);
            }
            1 => {
                if !self.f_1000af90(this) && !self.f_1000b1d0(this) {
                    self.chase_landing(this);
                    self.vcall(this, 20, 12, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_1000AF90`: catch a fly ball: low (4), high (5) or jumping (8).
    pub(crate) fn f_1000af90(&mut self, this: u32) -> bool {
        self.infield_fly_catch(this, (4, 5, 8))
    }

    /// Shared by `sub_1000AF90` and `sub_1000CCF0`.
    pub(crate) fn infield_fly_catch(&mut self, this: u32, states: (i32, i32, i32)) -> bool {
        let mut caught = false;
        let mut best = 99999999.0;
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let mut frame = 3;
        let mut reach = 60;
        let mut v4 = BALL + 44;
        while v4 < 0x1002_29C8 {
            let at = |pt: &Self, n: i32| pt.i((v4 as i32 + 4 * n) as u32);
            let d = segment_foot(
                f64::from(at(self, -11)),
                f64::from(at(self, -9)),
                f64::from(at(self, -2)),
                f64::from(at(self, 0)),
                f64::from(self.r(this, 40)),
                f64::from(self.r(this, 48)),
            )
            .distance;
            let height = f64::from(at(self, -1) - self.r(this, 44));
            if d < best {
                let reach_f = f64::from(reach);
                let (x, y, z) = (at(self, -2), at(self, -1), at(self, 0));
                let chosen = if d >= reach_f || height >= 300.0 {
                    if d < reach_f && height < 600.0 {
                        Some((states.1, [x, 0, z], false))
                    } else if f64::from(reach - 525) <= d || height >= 900.0 {
                        None
                    } else {
                        Some((states.2, [x, y, z], true))
                    }
                } else {
                    Some((states.0, [x, 0, z], false))
                };
                if let Some((state, target, jump)) = chosen {
                    best = d;
                    self.vcall(this, 20, state, 0);
                    if jump {
                        self.sr(this, 56, self.r(this, 40));
                        self.sr(this, 60, self.r(this, 44));
                        self.sr(this, 64, self.r(this, 48));
                    }
                    self.set_target(this, target);
                    self.sr(this, 120, frame);
                    self.f_10005460(this);
                    caught = true;
                }
            }
            v4 += 36;
            frame += 3;
            reach += 45;
        }
        caught
    }

    /// `sub_1000B1D0`: dive for a ground ball.
    pub(crate) fn f_1000b1d0(&mut self, this: u32) -> bool {
        self.dive_for_ball(this, 9)
    }

    /// Shared by `sub_1000B1D0` and `sub_1000CE90`: dives (state `state`)
    /// when the glove, offset by facing, can reach the ball's path.
    pub(crate) fn dive_for_ball(&mut self, this: u32, state: i32) -> bool {
        let mut found = false;
        let mut best = 99999999.0;
        let mut frame = 3;
        let mut reach = 60;
        let (mut ox, mut oy, mut oz) = (0, 0, 0);
        let mut point = BALL;
        loop {
            let p = |pt: &Self, n: u32| pt.i(point + 4 * n);
            let facing = direction8(
                f64::from(p(self, 9) - self.r(this, 40)),
                f64::from(p(self, 11) - self.r(this, 48)),
                self.r(this, 52),
            );
            self.sr(this, 52, facing);
            match facing {
                0 => (ox, oy, oz) = (0, 0, 320),
                1 => (ox, oy, oz) = (360, 0, 200),
                2 => (ox, oy, oz) = (420, 0, -80),
                3 => (ox, oy, oz) = (270, 0, -140),
                4 => (ox, oy, oz) = (80, 0, -200),
                5 => (ox, oy, oz) = (-440, 0, -80),
                6 => (ox, oy, oz) = (-520, 0, 0),
                7 => (ox, oy, oz) = (-320, 0, 160),
                _ => {}
            }
            let foot = segment_foot(
                f64::from(p(self, 0) - ox),
                f64::from(p(self, 2) - oz),
                f64::from(p(self, 9) - ox),
                f64::from(p(self, 11) - oz),
                f64::from(self.r(this, 40)),
                f64::from(self.r(this, 48)),
            );
            if f64::from(reach) > foot.distance && f64::from(p(self, 10) - self.r(this, 44)) < 300.0
            {
                let ball = self.p(this + 16);
                let toward = (foot.y - f64::from(self.r(this, 48))) * f64::from(self.i(ball + 56))
                    + (foot.x - f64::from(self.r(this, 40))) * f64::from(self.i(ball + 48));
                if self.i(ball + 60) > 100 && foot.distance < best && toward >= 0.0 {
                    best = foot.distance;
                    self.vcall(this, 20, state, 0);
                    self.sr(this, 56, self.r(this, 40));
                    self.sr(this, 60, self.r(this, 44));
                    self.sr(this, 64, self.r(this, 48));
                    let target = [p(self, 9) - ox, p(self, 10) - oy, p(self, 11) - oz];
                    self.set_target(this, target);
                    self.sr(this, 120, frame);
                    self.f_10005460(this);
                    found = true;
                }
            }
            reach += 45;
            frame += 3;
            point += 36;
            if point >= 0x1002_299C {
                break;
            }
        }
        found
    }

    /// `sub_1000B4A0` (slot 8).
    pub(crate) fn f_1000b4a0(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => self.f_1000b5a0(this),
            2 => self.f_1000b600(this),
            3 => self.f_1000b710(this),
            4 => self.f_1000b850(this),
            5 => self.f_1000b970(this),
            6 => self.f_1000b940(this),
            7 => self.f_1000baa0(this),
            8 => self.f_1000bd60(this),
            9 => self.f_1000bac0(this),
            10 => self.f_1000c120(this),
            11 => self.f_1000bfd0(this),
            12 => self.f_1000b790(this),
            13 => self.f_1000b7d0(this),
            _ => {}
        }
        self.glove_frame(this);
    }

    /// Which of the eight glove frames the current pattern shows
    /// (`record[34]`, 0 for none).
    pub(crate) fn glove_frame(&mut self, this: u32) {
        self.sr(this, 136, 0);
        let pattern = self.r(this, 80);
        for (n, top) in (51..83).step_by(4).enumerate() {
            if top - 2 <= pattern && pattern <= top {
                self.sr(this, 136, n as i32 + 1);
            }
        }
    }

    /// `sub_1000B5A0` (state 1): idle for a random while.
    fn f_1000b5a0(&mut self, this: u32) {
        if self.r(this, 112) == 0 {
            let facing = self.r(this, 52);
            self.f_10005090(this, facing, 100);
            let wait = self.rand() % 200;
            self.sr(this, 112, self.r(this, 112) + wait);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) >= 300 {
            self.vcall(this, 16, 0, 0);
        }
    }

    /// `sub_1000B600` (state 2): pacing, switching ends every 400 frames.
    pub(crate) fn f_1000b600(&mut self, this: u32) {
        let phase = self.r(this, 124) % 800;
        let (hx, hz) = (self.r(this, 28), self.r(this, 36));
        let near = !(200..600).contains(&phase);
        let target = if hx > 0 {
            if near {
                [hx - 1400, 0, hz + 700]
            } else {
                [hx + 1400, 0, hz - 700]
            }
        } else if hx == 0 {
            [if near { hx + 1400 } else { hx - 1400 }, 0, hz]
        } else if near {
            [hx + 1400, 0, hz + 700]
        } else {
            [hx - 1400, 0, hz - 700]
        };
        self.set_target(this, target);
        let first = i32::from(self.r(this, 112) == 0);
        self.f_10005310(this, 1, 0, first);
        self.f_10005100(this, 7.0);
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) >= 200 {
            self.sr(this, 20, 0);
        }
    }

    /// `sub_1000B710` (state 3).
    fn f_1000b710(&mut self, this: u32) {
        let table = 0x1002_2FB8;
        self.prefix_table(table, 0x1001_EC80, 0x1001_ECA4);
        let pattern = self.table_step(this, table, 0x1001_EC5C);
        self.f_10005090(this, pattern, 100);
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.table_total(table, 0x1001_ECA4) <= self.r(this, 112) {
            self.vcall(this, 16, 0, 0);
        }
    }

    /// `sub_1000B790` (state 12): running to the ball.
    pub(crate) fn f_1000b790(&mut self, this: u32) {
        let first = i32::from(self.r(this, 112) == 0);
        self.f_10005310(this, 0, 0, first);
        self.f_10005100(this, 15.0);
        self.sr(this, 112, self.r(this, 112) + 1);
        self.sr(this, 20, 0);
    }

    /// `sub_1000B7D0` (state 13): back home.
    pub(crate) fn f_1000b7d0(&mut self, this: u32) {
        let home = [self.r(this, 28), self.r(this, 32), self.r(this, 36)];
        self.set_target(this, home);
        let first = i32::from(self.r(this, 112) == 0);
        self.f_10005310(this, 0, 0, first);
        self.f_10005100(this, 15.0);
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.at_target(this) {
            self.vcall(this, 16, 0, 0);
        }
    }

    /// `sub_1000B850` (state 4): a low catch.
    fn f_1000b850(&mut self, this: u32) {
        self.camera_before_catch(this);
        let (t, at) = (self.r(this, 112), self.r(this, 120));
        if t < at {
            if self.at_target(this) {
                self.f_10005090(this, 45, 100);
            } else {
                let first = i32::from(t == 0);
                self.f_10005310(this, 0, 0, first);
                self.f_10005100(this, 15.0);
            }
        }
        if self.r(this, 112) == self.r(this, 120) {
            self.f_10005480(this);
            self.f_10014040(15, 255);
            self.f_10014080(40);
        }
        if self.r(this, 120) <= self.r(this, 112) {
            self.f_10005090(this, 44, 100);
        }
        self.advance_or(this, 24, 10);
    }

    pub(crate) fn camera_before_catch(&mut self, this: u32) {
        if self.r(this, 112) == self.r(this, 120) - 50 {
            let number = self.w(this, 312);
            self.f_10004400(number + 10, 50, false);
        }
    }

    /// Advances the frame counter, or enters `next` `after` frames past
    /// the catch.
    pub(crate) fn advance_or(&mut self, this: u32, after: i32, next: i32) {
        let t = self.r(this, 112);
        if t == self.r(this, 120) + after {
            self.vcall(this, 20, next, 0);
        } else {
            self.sr(this, 112, t + 1);
        }
    }

    /// `sub_1000B940` (state 6).
    fn f_1000b940(&mut self, this: u32) {
        self.sr(this, 80, 45);
        self.sr(this, 84, 100);
        self.sr(this, 88, 1);
        self.sr(this, 20, 0);
    }

    /// `sub_1000B970` (state 5): a high catch, which may be dropped.
    fn f_1000b970(&mut self, this: u32) {
        self.camera_before_catch(this);
        let (t, at) = (self.r(this, 112), self.r(this, 120));
        if t >= at {
            self.f_10005090(this, 46, 100);
        } else if self.at_target(this) {
            self.f_10005090(this, 47, 100);
        } else {
            let first = i32::from(t == 0);
            self.f_10005310(this, 0, 0, first);
            self.f_10005100(this, 15.0);
        }
        if self.r(this, 112) == self.r(this, 120) - 1 {
            let chance = 25 - self.g(this, 3728) / 2;
            if self.rand() % 100 < chance {
                self.vcall(this, 20, 11, 0);
                return;
            }
        }
        if self.r(this, 112) == self.r(this, 120) {
            self.f_10005480(this);
            self.f_10014040(15, 255);
            self.f_10014080(41);
        }
        self.advance_or(this, 24, 10);
    }

    /// `sub_1000BAA0` (state 7).
    fn f_1000baa0(&mut self, this: u32) {
        self.f_10005090(this, 47, 100);
        self.sr(this, 20, 0);
    }

    /// `sub_1000BAC0` (state 9): the dive.
    fn f_1000bac0(&mut self, this: u32) {
        self.prefix_table(0x1002_2FDC, 0x1001_ECBC, 0x1001_ECD0);
        self.dive(this, (0x1001_ECAC, 0x1001_ECB0, 0x1001_ECB4), 42);
    }

    /// Shared by `sub_1000BAC0` and `sub_1000D0D0`: slides from the start
    /// to the target, arriving at `record[120]`.
    pub(crate) fn dive(&mut self, this: u32, patterns: (u32, u32, u32), line: i32) {
        self.camera_before_catch(this);
        let dx = self.r(this, 68) - self.r(this, 56);
        let dz = self.r(this, 76) - self.r(this, 64);
        let facing = direction8(f64::from(dx), f64::from(dz), self.r(this, 52));
        self.sr(this, 52, facing);
        let (sx, sy, sz) = (self.r(this, 56), self.r(this, 60), self.r(this, 64));
        let (v3, v4, v5) = (
            self.r(this, 68) - sx,
            self.r(this, 72) - sy,
            self.r(this, 76) - sz,
        );
        let at = self.r(this, 120);
        let span = if at <= 1 { 1 } else { at };
        let t = self.r(this, 112);
        let slide = |pt: &mut Self| {
            pt.sr(this, 40, t * v3 / span + sx);
            pt.sr(this, 44, t * v4 / span + sy);
            pt.sr(this, 48, sz + t * v5 / span);
            pt.sr(this, 44, 0);
        };
        if t >= at - 10 {
            let pattern_of = |pt: &Self, base: u32| pt.i(base) + 4 * pt.r(this, 52);
            if t >= at {
                if t >= at + 15 {
                    if t >= at + 22 {
                        self.vcall(this, 20, 10, 0);
                        return;
                    }
                    let pattern = pattern_of(self, patterns.2);
                    self.f_10005090(this, pattern, 100);
                } else {
                    slide(self);
                    let pattern = pattern_of(self, patterns.1);
                    self.f_10005090(this, pattern, 100);
                }
            } else {
                slide(self);
                let pattern = pattern_of(self, patterns.0);
                self.f_10005090(this, pattern, 100);
            }
        } else {
            if t == 0 {
                self.f_10005310(this, 0, 0, 1);
            }
            slide(self);
        }
        if self.r(this, 112) == self.r(this, 120) - 10 {
            self.f_10014040(24, 255);
            self.f_10014080(line);
            self.f_100055b0(this);
        }
        if self.r(this, 112) == self.r(this, 120) {
            self.f_10005480(this);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
    }

    /// `sub_1000BD60` (state 8): the jumping catch.
    fn f_1000bd60(&mut self, this: u32) {
        self.prefix_table(0x1002_2FF0, 0x1001_ECE4, 0x1001_ECF4);
        self.jump_catch(
            this,
            [0x1001_ECD4, 0x1001_ECD8, 0x1001_ECDC, 0x1001_ECE0],
            43,
        );
    }

    /// Shared by `sub_1000BD60` and `sub_1000D3E0`-style jumps.
    pub(crate) fn jump_catch(&mut self, this: u32, patterns: [u32; 4], line: i32) {
        self.sr(this, 84, 100);
        self.camera_before_catch(this);
        let at = self.r(this, 120);
        let t = self.r(this, 112);
        let arrived = self.r(this, 40) == self.r(this, 68) && self.r(this, 48) == self.r(this, 76);
        if t >= at - 34 || arrived {
            let jump = |pt: &mut Self| {
                pt.go_to_target(this);
                let h = arc(pt.r(this, 112), at - 24, at, pt.r(this, 72) - 400);
                pt.sr(this, 44, h.max(0));
            };
            let pattern = if t >= at - 24 {
                if t >= at {
                    if t >= at + 23 {
                        self.go_to_target(this);
                        self.sr(this, 44, 0);
                        if t >= at + 35 {
                            self.vcall(this, 20, 10, 0);
                            return;
                        }
                        self.i(patterns[3])
                    } else {
                        jump(self);
                        self.i(patterns[2])
                    }
                } else {
                    jump(self);
                    self.i(patterns[1])
                }
            } else {
                self.go_to_target(this);
                self.sr(this, 44, 0);
                self.i(patterns[0])
            };
            self.sr(this, 80, pattern);
            self.sr(this, 88, 1);
        } else {
            if t == 0 {
                self.f_10005310(this, 0, 0, 1);
            }
            self.f_10005100(this, 15.0);
        }
        if self.r(this, 112) == at - 24 {
            self.f_10014040(23, 255);
        }
        if self.r(this, 112) == at {
            self.f_10005480(this);
            self.f_10014040(15, 255);
            self.f_10014080(line);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
    }

    /// `sub_1000BFD0` (state 11): the ball pops out of the glove.
    fn f_1000bfd0(&mut self, this: u32) {
        let table = 0x1002_3000;
        self.prefix_table(table, 0x1001_ED30, 0x1001_ED68);
        let pattern = self.table_seek(this, table, 0x1001_ECF8, 0x1001_ED68);
        self.f_10005090(this, pattern, 100);
        if self.r(this, 112) == 0 {
            self.f_drop_ball(this);
        }
        if self.r(this, 112) == 120 {
            self.f_10014080(44);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) == self.table_total(table, 0x1001_ED68) {
            self.f_10005590(this, 30);
        }
    }

    /// The error: the ball bounces back off the glove.
    pub(crate) fn f_drop_ball(&mut self, this: u32) {
        let ball = self.p(this + 16);
        self.si(ball + 120, 1);
        let v = |pt: &Self, n: u32| pt.i(ball + 4 * n);
        let (x, y, z) = (v(self, 6), v(self, 7), v(self, 8));
        let (dx, dy, dz) = (-v(self, 12), -v(self, 13), -v(self, 14));
        let speed = ftol(f64::from(v(self, 15)) * 0.1);
        self.f_10002bd0(x, y, z, dx, dy, dz, speed, 0);
        self.f_10014040(15, 255);
    }

    /// `sub_1000C120` (state 10): the throw, then home.
    fn f_1000c120(&mut self, this: u32) {
        let table = 0x1002_3038;
        self.prefix_table(table, 0x1001_ED7C, 0x1001_ED8C);
        let pattern = self.i(0x1001_ED6C + 4 * self.r(this, 108) as u32);
        self.f_10005090(this, pattern, 100);
        if self.r(this, 112) == 23 {
            let (x, y, z) = (self.r(this, 40), self.r(this, 44) + 400, self.r(this, 48));
            self.throw_at_target(x, y, z, 500, 300);
            self.f_10014040(6, 255);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        let index = self.r(this, 108);
        if self.i(table + 4 * index as u32) <= self.r(this, 112) {
            self.sr(this, 108, index + 1);
            if self.i(0x1001_ED8C) <= self.r(this, 108) {
                self.vcall(this, 16, 0, 0);
                self.vcall(this, 20, 13, 0);
            }
        }
    }
}
