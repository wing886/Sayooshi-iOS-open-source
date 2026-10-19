//! Actors 11-19: the cats (vtable `0x1001C0F8`).

use super::super::base::{distance, f_100021a0, segment_foot};
use super::super::{BALL, Pt00, ftol};

impl Pt00 {
    /// `sub_100122F0` (slot 2): reset, facing a random side.
    pub(crate) fn f_100122f0(&mut self, this: u32) {
        self.f_10004ab0(this);
        let facing = if self.rand() % 2 != 0 { 2 } else { 6 };
        self.sr(this, 52, facing);
    }

    /// `sub_10012320` (slot 7).
    pub(crate) fn f_10012320(&mut self, this: u32) {
        match self.r(this, 24) {
            0 => {
                let (x, z) = (f64::from(self.r(this, 40)), f64::from(self.r(this, 48)));
                if distance(
                    x,
                    z,
                    f64::from(self.r(this, 28)),
                    f64::from(self.r(this, 36)),
                ) > 3000.0
                {
                    self.vcall(this, 20, 3, 0);
                    return;
                }
                let to_toy = distance(
                    x,
                    z,
                    f64::from(self.g(this, 7408)),
                    f64::from(self.g(this, 7416)),
                );
                if self.g(this, 7400) != 0 && to_toy < 3000.0 {
                    let state = match self.rand() % 1000 {
                        0 => 4,
                        1 => 6,
                        2 => 5,
                        _ => 9,
                    };
                    self.vcall(this, 20, state, 0);
                } else if !self.at_home(this) {
                    self.vcall(this, 20, 3, 0);
                } else {
                    let state = match self.rand() % 3 {
                        0 => 4,
                        1 => 6,
                        _ => 5,
                    };
                    self.vcall(this, 20, state, 0);
                }
            }
            1 => self.cat_chase(this),
            _ => {}
        }
    }

    /// The ball is coming: pounce on it low (8), jump for it (7), or run to
    /// where it lands (2).
    fn cat_chase(&mut self, this: u32) {
        if self.i(BALL) != 0 || self.i(BALL + 4) != 0 || self.i(BALL + 8) != 0 {
            let mut frame = 10;
            let mut v11 = 0x1002_258Cu32;
            while v11 < 0x1002_276C {
                let at = |pt: &Self, n: i32| pt.i((v11 as i32 + 4 * n) as u32);
                let d = segment_foot(
                    f64::from(at(self, -10)),
                    f64::from(at(self, -8)),
                    f64::from(at(self, -1)),
                    f64::from(at(self, 1)),
                    f64::from(self.r(this, 40)),
                    f64::from(self.r(this, 48)),
                )
                .distance;
                if d < 120.0 {
                    let height = at(self, 0);
                    let flat = self.f_10004310(at(self, -1), at(self, 1), None) == 0;
                    let point = BALL + 12 * frame as u32;
                    if height < 300 && flat {
                        self.vcall(this, 20, 8, 0);
                        self.sr(this, 120, frame);
                        self.set_target(this, [self.i(point), 0, self.i(point + 8)]);
                        self.f_10005460(this);
                        return;
                    }
                    if (300..600).contains(&height) && flat {
                        self.vcall(this, 20, 7, 49 - frame);
                        self.sr(this, 56, self.r(this, 40));
                        self.sr(this, 60, self.r(this, 44));
                        self.sr(this, 64, self.r(this, 48));
                        self.set_target(
                            this,
                            [self.i(point), self.i(point + 4), self.i(point + 8)],
                        );
                        self.f_10005460(this);
                        return;
                    }
                }
                v11 += 36;
                frame += 3;
            }
        }
        self.vcall(this, 20, 2, 0);
        let ball = self.p(this + 16);
        let (x, z) = (self.i(ball + 92), self.i(ball + 100));
        self.set_target(this, [x, 0, z]);
    }

    /// The ball glances off the cat into the toy slot.
    fn deflect_ball(&mut self, this: u32) {
        let ball = self.p(this + 16);
        let v = |pt: &Self, n: u32| pt.i(ball + 4 * n);
        let speed = ftol(f64::from(v(self, 15)) * 0.1);
        let (x, y, z) = (v(self, 6), v(self, 7), v(self, 8));
        let (dx, dy, dz) = (-v(self, 12), -v(self, 13), -v(self, 14));
        self.f_10013f20(x, y, z, dx, dy, dz, speed);
    }

    /// Mirror for the facing (`sub_100021A0`).
    fn cat_mirror(&self, this: u32) -> i32 {
        f_100021a0(self.r(this, 52))
    }

    /// Plays a one-shot table; returns true at its end.
    fn cat_table(
        &mut self,
        this: u32,
        table: u32,
        source: u32,
        count_at: u32,
        patterns: u32,
    ) -> bool {
        self.prefix_table(table, source, count_at);
        let mirror = self.cat_mirror(this);
        let pattern = self.i(patterns + 4 * self.r(this, 108) as u32);
        self.f_10005090(this, pattern, mirror);
        self.sr(this, 112, self.r(this, 112) + 1);
        let index = self.r(this, 108);
        if self.i(table + 4 * index as u32) <= self.r(this, 112) {
            self.sr(this, 108, index + 1);
            return true;
        }
        false
    }

    /// Direction of the rebound for the knocked-over animations.
    fn tumble_direction(&self, this: u32) -> (i32, i32) {
        let dx = self.r(this, 68) - self.r(this, 56);
        let dz = self.r(this, 76) - self.r(this, 64);
        if dx > 0 && dx > dz {
            (4, 100)
        } else if dx < 0 && self.r(this, 56) - self.r(this, 68) > dz {
            (4, -100)
        } else {
            (0, 100)
        }
    }

    /// After being hit, the pitcher comes over (`frame_call`) and the cat
    /// leaves (`frame_end`); `frame_flag` sets the scene flag.
    fn after_hit(
        &mut self,
        this: u32,
        frame_flag: i32,
        line: i32,
        frame_call: i32,
        frame_end: i32,
    ) {
        self.sr(this, 112, self.r(this, 112) + 1);
        let t = self.r(this, 112);
        if t == frame_flag {
            self.sr(this, 128, 1);
            self.sr(this, 132, 1);
            self.f_10014080(line);
        }
        if t == frame_call {
            let pitcher = self.actor(0);
            self.vcall(pitcher, 20, 7, 0);
            self.f_10005460(pitcher);
        }
        if t == frame_end {
            self.vcall(this, 16, 0, 0);
        }
    }

    /// `sub_100126B0` (slot 8).
    pub(crate) fn f_100126b0(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => {
                if self.r(this, 112) == 0 {
                    let wait = self.rand() % 30;
                    self.sr(this, 112, self.r(this, 112) + wait);
                }
                let mirror = self.cat_mirror(this);
                self.f_10005090(this, 0, mirror);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) >= 50 {
                    self.vcall(this, 16, 0, 0);
                }
            }
            2 => {
                let first = i32::from(self.r(this, 112) == 0);
                self.f_100051c0(this, 0, 0, first);
                let saved = [self.r(this, 40), self.r(this, 44), self.r(this, 48)];
                self.f_10005100(this, 15.0);
                if self.f_10004310(self.r(this, 40), self.r(this, 48), None) > 0 {
                    self.sr(this, 40, saved[0]);
                    self.sr(this, 44, saved[1]);
                    self.sr(this, 48, saved[2]);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            3 => {
                let home = [self.r(this, 28), self.r(this, 32), self.r(this, 36)];
                self.set_target(this, home);
                self.walk_field(this, 0, 15.0);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.at_target(this) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            4 => {
                if self.cat_table(this, 0x1002_3350, 0x1001_F45C, 0x1001_F474, 0x1001_F444)
                    && self.i(0x1001_F474) <= self.r(this, 108)
                {
                    self.vcall(this, 16, 0, 0);
                }
            }
            5 => {
                if self.cat_table(this, 0x1002_33BC, 0x1001_F574, 0x1001_F5C4, 0x1001_F524)
                    && self.i(0x1001_F5C4) <= self.r(this, 108)
                {
                    self.vcall(this, 16, 0, 0);
                }
            }
            // Grooming, which usually loops back.
            6 => {
                let table = 0x1002_3368;
                if self.cat_table(this, table, 0x1001_F4CC, 0x1001_F520, 0x1001_F478) {
                    let count = self.i(0x1001_F520);
                    if count - 2 == self.r(this, 112) {
                        if self.rand() % 10 < 9 {
                            self.sr(this, 108, 2);
                            self.sr(this, 112, self.i(table + 4));
                        }
                    } else if count <= self.r(this, 108) {
                        self.vcall(this, 16, 0, 0);
                    }
                }
            }
            7 => self.cat_jump(this),
            // The pounce.
            8 => {
                let (t, at) = (self.r(this, 112), self.r(this, 120));
                if t < at {
                    if self.at_target(this) {
                        self.f_10005090(this, 12, 100);
                    } else {
                        let first = i32::from(t == 0);
                        self.f_100051c0(this, 0, 0, first);
                        self.f_10005100(this, 15.0);
                    }
                }
                let ball = self.p(this + 16);
                let (t, at) = (self.r(this, 112), self.r(this, 120));
                if t == at - 1 && self.i(ball + 60) > 400 {
                    self.knocked_over(this, 11);
                } else {
                    if t == at {
                        self.deflect_ball(this);
                        self.f_10004400(4, 50, false);
                        self.f_10005480(this);
                        self.f_10014040(25, 255);
                        self.f_10014080(110);
                    }
                    self.sr(this, 112, t + 1);
                    if t + 1 >= at + 10 {
                        self.f_100054e0(this);
                    }
                }
            }
            9 => self.cat_play(this),
            // Knocked flying by a fast ball.
            10 => {
                if self.r(this, 112) == 0 {
                    self.deflect_ball(this);
                    self.f_10005480(this);
                    self.f_10014040(27, 255);
                    self.f_10014080(112);
                }
                let (base, mirror) = self.tumble_direction(this);
                let lift = f64::from(self.r(this, 60).max(0));
                let flight = (lift * 4.999999925494195).sqrt();
                let t = f64::from(self.r(this, 112));
                let pattern = if t < flight {
                    self.f_10005100(this, 7.0);
                    let h = ftol(lift - lift / (flight * flight) * (t * t));
                    self.sr(this, 44, h.max(0));
                    if t >= 8.0 { base + 37 } else { base + 36 }
                } else {
                    self.sr(this, 44, 0);
                    if t < flight + 8.0 || t >= flight + 16.0 {
                        base + 38
                    } else {
                        base + 39
                    }
                };
                self.f_10005090(this, pattern, mirror);
                let end = ftol(flight);
                self.after_hit(this, end + 16, 64, end + 120, end + 240);
            }
            11 => {
                if self.r(this, 112) == 0 {
                    self.deflect_ball(this);
                    self.f_10005480(this);
                    self.f_10014040(27, 255);
                    self.f_10014080(112);
                }
                let t = f64::from(self.r(this, 112));
                if t >= 35.0 {
                    self.sr(this, 44, 0);
                } else {
                    let h = ftol(67.5 - (t - 15.0) * (t - 15.0) * 0.3);
                    self.sr(this, 44, h.max(0));
                    self.f_10005100(this, 7.0);
                }
                let (base, mirror) = self.tumble_direction(this);
                let n = self.r(this, 112);
                let pattern = if n < 8 {
                    base + 36
                } else if n < 30 {
                    base + 37
                } else if (38..46).contains(&n) {
                    base + 39
                } else {
                    base + 38
                };
                self.f_10005090(this, pattern, mirror);
                self.after_hit(this, 46, 63, 150, 270);
            }
            _ => {}
        }
    }

    /// Knocked over (`state`) by the ball: starts from here, flies along
    /// the ball's direction.
    fn knocked_over(&mut self, this: u32, state: i32) {
        self.vcall(this, 20, state, 0);
        self.sr(this, 56, self.r(this, 40));
        self.sr(this, 60, self.r(this, 44));
        self.sr(this, 64, self.r(this, 48));
        let ball = self.p(this + 16);
        let target = [
            self.r(this, 40) + self.i(ball + 48),
            self.r(this, 44) + self.i(ball + 52),
            self.r(this, 48) + self.i(ball + 56),
        ];
        self.set_target(this, target);
    }

    /// State 7: leaping for the ball, landing at frame 49.
    fn cat_jump(&mut self, this: u32) {
        let top = f64::from((self.r(this, 72) - 200).max(0));
        let half = (top + top).sqrt();
        let start = 49.0 - half;
        let end = half + 49.0;
        let t = f64::from(self.r(this, 112));
        if start > t || t >= end {
            self.sr(this, 44, 0);
            let pattern = self.i(0x1001_F5C8);
            self.sr(this, 80, pattern);
        } else {
            let k = (t - start) / (49.0 - start);
            self.sr(
                this,
                40,
                ftol(
                    f64::from(self.r(this, 68) - self.r(this, 56)) * k
                        + f64::from(self.r(this, 56)),
                ),
            );
            self.sr(
                this,
                48,
                ftol(
                    f64::from(self.r(this, 76) - self.r(this, 64)) * k
                        + f64::from(self.r(this, 64)),
                ),
            );
            let h = ftol(top - top / ((start - 49.0) * (start - 49.0)) * ((t - 49.0) * (t - 49.0)));
            self.sr(this, 44, h.max(0));
            let pattern = if t < (end - start) * 0.5 + start {
                self.i(0x1001_F5CC)
            } else if t >= (end - start) * 0.75 + start {
                self.i(0x1001_F5D4)
            } else {
                self.i(0x1001_F5D0)
            };
            self.sr(this, 80, pattern);
        }
        let pattern = self.r(this, 80);
        self.f_10005090(this, pattern, 100);
        if self.r(this, 112) as u32 == ftol(start) as u32 {
            self.f_10014040(23, 255);
        }
        let n = self.r(this, 112);
        let ball = self.p(this + 16);
        if n == 49 && self.i(ball + 60) > 400 {
            self.knocked_over(this, 10);
            return;
        }
        if n == 50 {
            self.deflect_ball(this);
            self.f_10005480(this);
            self.f_10014040(26, 255);
            self.f_10014080(111);
            self.f_10004400(4, 50, false);
        }
        if f64::from(self.r(this, 112)) < end + 16.0 {
            self.sr(this, 112, self.r(this, 112) + 1);
        } else {
            self.f_100054e0(this);
        }
    }

    /// State 9: chasing the toy and batting it along.
    fn cat_play(&mut self, this: u32) {
        if self.g(this, 7400) == 0 {
            self.vcall(this, 16, 0, 0);
            return;
        }
        self.sr(this, 68, self.g(this, 7408));
        self.sr(this, 72, 0);
        self.sr(this, 76, self.g(this, 7416));
        let first = i32::from(self.r(this, 112) == 0);
        self.f_100051c0(this, 0, 0, first);
        let saved = [self.r(this, 40), self.r(this, 44), self.r(this, 48)];
        self.f_10005100(this, 15.0);
        // Earlier cats already chasing it have priority.
        let number = self.w(this, 312);
        for n in 0..9 {
            if n + 11 >= number {
                continue;
            }
            let other = self.actor(n as u32 + 11);
            if self.r(other, 16) == 9
                && distance(
                    f64::from(self.r(this, 40)),
                    f64::from(self.r(this, 48)),
                    f64::from(self.r(other, 40)),
                    f64::from(self.r(other, 48)),
                ) < 240.0
            {
                self.sr(this, 40, saved[0]);
                self.sr(this, 44, saved[1]);
                self.sr(this, 48, saved[2]);
                break;
            }
        }
        let d = distance(
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
            f64::from(self.r(this, 68)),
            f64::from(self.r(this, 76)),
        );
        if self.f(1864) == 1 && self.f(1853) < 300 && d < 120.0 {
            self.sf(1855, self.f(1852));
            self.sf(1856, self.f(1853));
            self.sf(1857, self.f(1854));
            let speed = self.rand() % 50 + 100;
            self.sf(1861, speed);
            let lift = self.rand() % 200 + 50;
            self.sf(1862, lift);
            self.sf(1863, 0);
            self.sf(1864, 0);
            let (x, z) = (f64::from(self.f(1858)), f64::from(self.f(1860)));
            let (c, s) = (0.785398163375f64.cos(), 0.785398163375f64.sin());
            self.sf(1858, 2 * ftol(c * x * 0.5 - s * z));
            self.sf(1860, ftol(s * x * 0.5 + c * z));
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) % 200 == 0 {
            self.sr(this, 20, 0);
        }
    }
}
