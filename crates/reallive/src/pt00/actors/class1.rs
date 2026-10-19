//! Actor 1: the catcher (vtable `0x1001C23C`).

use super::super::base::{arc, direction8, distance, segment_foot};
use super::super::{BALL, Pt00, ftol};

impl Pt00 {
    /// `sub_10007070` (slot 6).
    pub(crate) fn f_10007070(&mut self, this: u32) {
        match self.r(this, 12) {
            2 | 4 => {
                if !self.f_10007300(this) {
                    self.sr(this, 24, 0);
                }
            }
            5 | 6 | 8 => {
                if !self.f_10007430(this) {
                    self.sr(this, 24, 0);
                }
            }
            _ if self.f_10005500(this) => {
                // (sub_100074E0 always fails.)
                if !self.f_10007430(this) {
                    self.sr(this, 24, 0);
                }
            }
            0 => self.f_10004ef0(this),
            _ => {}
        }
    }

    /// `sub_10007100` (slot 7).
    pub(crate) fn f_10007100(&mut self, this: u32) {
        match self.r(this, 12) {
            2 => {
                self.vcall(this, 20, 1, 0);
            }
            4 => {
                self.vcall(this, 20, 4, 0);
            }
            5 => {
                self.vcall(this, 20, 5, 0);
            }
            6 => {
                if !self.f_100074f0(this) {
                    self.vcall(this, 20, 6, 0);
                }
            }
            8 => {
                let state = if self.b(0x1002_29EC) != 0 { 12 } else { 11 };
                self.vcall(this, 20, state, 0);
            }
            _ if self.f_10005500(this) => {
                if !self.f_100074f0(this) && self.r(this, 16) != 8 {
                    self.vcall(this, 20, 7, 0);
                }
            }
            0 => {
                if self.r(this, 24) == 1 {
                    if !self.f_10007530(this) {
                        self.chase_landing(this);
                        self.vcall(this, 20, 17, 0);
                    }
                } else {
                    self.vcall(this, 20, 13, 0);
                }
            }
            _ => {}
        }
    }

    /// Targets the nearest point of the ball's predicted path, or where
    /// it lands if that is nearer home.
    pub(crate) fn chase_landing(&mut self, this: u32) {
        let foot = segment_foot(
            f64::from(self.i(BALL)),
            f64::from(self.i(BALL + 8)),
            f64::from(self.i(0x1002_29B4)),
            f64::from(self.i(0x1002_29BC)),
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        );
        let ball = self.p(this + 16);
        if f64::from(self.i(ball + 100)) <= foot.y {
            self.sr(this, 68, ftol(foot.x));
            self.sr(this, 72, 0);
            self.sr(this, 76, ftol(foot.y));
        } else {
            self.sr(this, 68, self.i(ball + 92));
            self.sr(this, 72, 0);
            self.sr(this, 76, self.i(ball + 100));
        }
    }

    /// `sub_10007300`: take a pitch that comes close.
    fn f_10007300(&mut self, this: u32) -> bool {
        let ball = self.p(this + 16);
        if distance(
            f64::from(self.i(ball + 24)),
            f64::from(self.i(ball + 32)),
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        ) >= 600.0
        {
            return false;
        }
        if self.i(ball + 32) >= self.r(this, 48)
            || f64::from(self.i(ball + 28) - self.r(this, 44)) >= 1200.0
        {
            return false;
        }
        self.vcall(this, 20, 2, 0);
        self.sr(this, 56, self.r(this, 40));
        self.sr(this, 60, self.r(this, 44));
        self.sr(this, 64, self.r(this, 48));
        let (v7, v8) = if self.r(this, 12) == 2 {
            (self.r(this, 28), self.r(this, 36))
        } else {
            (0, 5000)
        };
        self.sr(this, 68, v8 + self.r(this, 40) - self.r(this, 48));
        self.sr(this, 72, 0);
        self.sr(this, 76, self.r(this, 40) + self.r(this, 48) - v7);
        self.f_10005460(this);
        let number = self.w(this, 312);
        self.f_10004400(number + 10, 50, false);
        true
    }

    /// `sub_10007430`: catch a pitch passing within reach.
    fn f_10007430(&mut self, this: u32) -> bool {
        self.pitch_catch(this, 3, 1)
    }

    /// Catch a pitch passing within reach: enter `state` when it arrives
    /// and set `F[808]` to `flag` (`sub_10007430`, `sub_10009AB0`).
    pub(crate) fn pitch_catch(&mut self, this: u32, state: i32, flag: i32) -> bool {
        let mut frame = 0;
        let mut point = BALL;
        loop {
            let near = distance(
                f64::from(self.i(point)),
                f64::from(self.i(point + 8)),
                f64::from(self.r(this, 40)),
                f64::from(self.r(this, 48)),
            ) < 160.0
                && f64::from(self.i(point + 4) - self.r(this, 44)) < 600.0;
            if near {
                break;
            }
            point += 36;
            frame += 3;
            if point >= BALL + 1200 {
                return false;
            }
        }
        self.f_10005570(this, state, frame);
        self.f_10005460(this);
        self.sg(this, 3232, flag);
        true
    }

    /// `sub_100074F0`: walk home if away from it.
    fn f_100074f0(&mut self, this: u32) -> bool {
        if self.at_home(this) {
            return false;
        }
        self.vcall(this, 20, 18, 0);
        true
    }

    pub(crate) fn at_home(&self, this: u32) -> bool {
        self.r(this, 40) == self.r(this, 28)
            && self.r(this, 44) == self.r(this, 32)
            && self.r(this, 48) == self.r(this, 36)
    }

    pub(crate) fn at_target(&self, this: u32) -> bool {
        self.r(this, 40) == self.r(this, 68)
            && self.r(this, 44) == self.r(this, 72)
            && self.r(this, 48) == self.r(this, 76)
    }

    /// `sub_10007530`: catch a fly ball on its predicted path.
    fn f_10007530(&mut self, this: u32) -> bool {
        self.fly_catch(this, 14, 15)
    }

    /// Catch a fly ball on its predicted path, low (`low`) or high
    /// (`high`) (`sub_10007530`, `sub_10009810`).
    pub(crate) fn fly_catch(&mut self, this: u32, low: i32, high: i32) -> bool {
        let mut caught = false;
        let mut best = 99999999.0;
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let mut frame = 3;
        let mut reach = 60;
        let mut v4 = BALL + 44;
        while v4 < 0x1002_29C8 {
            let at = |n: i32| self.i((v4 as i32 + 4 * n) as u32);
            let d = segment_foot(
                f64::from(at(-11)),
                f64::from(at(-9)),
                f64::from(at(-2)),
                f64::from(at(0)),
                f64::from(self.r(this, 40)),
                f64::from(self.r(this, 48)),
            )
            .distance;
            let height = f64::from(at(-1) - self.r(this, 44));
            if d < best {
                let reach_f = f64::from(reach);
                let state = if d >= reach_f || height >= 300.0 {
                    if d >= reach_f || height >= 600.0 {
                        None
                    } else {
                        Some(high)
                    }
                } else {
                    Some(low)
                };
                if let Some(state) = state {
                    best = d;
                    let (x, z) = (at(-2), at(0));
                    self.vcall(this, 20, state, 0);
                    self.sr(this, 68, x);
                    self.sr(this, 72, 0);
                    self.sr(this, 76, z);
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

    /// Walks towards the target (`sub_100051C0` in the field,
    /// `sub_10005310` at the plate).
    pub(crate) fn walk_to_target(&mut self, this: u32) {
        let first = i32::from(self.r(this, 112) == 0);
        if self.r(this, 12) != 0 {
            self.f_100051c0(this, 0, 0, first);
        } else {
            self.f_10005310(this, 0, 101, first);
        }
        self.f_10005100(this, 15.0);
    }

    /// `sub_100076F0` (slot 8): the catcher's states.
    pub(crate) fn f_100076f0(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => self.f_100079f0(this),
            // Standing up after the catch.
            2 => {
                let table = 0x1002_2AC0;
                self.prefix_table(table, 0x1001_E200, 0x1001_E214);
                self.table_seek(this, table, 0x1001_E1EC, 0x1001_E214);
                let dx = self.r(this, 68) - self.r(this, 56);
                let dz = self.r(this, 76) - self.r(this, 64);
                let facing = direction8(f64::from(dx), f64::from(dz), self.r(this, 52));
                self.sr(this, 52, facing);
                let base = self.i(0x1001_E1EC + 4 * self.r(this, 108) as u32);
                let look = match facing {
                    0 => Some((base, -100)),
                    1 => Some((base + 4, -100)),
                    2 => Some((base + 8, -100)),
                    3 => Some((base + 12, -100)),
                    4 => Some((base + 16, 100)),
                    5 => Some((base + 12, 100)),
                    6 => Some((base + 8, 100)),
                    7 => Some((base + 4, 100)),
                    _ => None,
                };
                if let Some((pattern, mirror)) = look {
                    self.f_10005090(this, pattern, mirror);
                }
                let step = (10 * (60 - self.r(this, 112)) / 60).max(0);
                self.f_10005100(this, f64::from(step));
                if self.r(this, 112) == 0 {
                    self.f_10014080(11);
                }
                if self.r(this, 112) == self.i(table + 4) {
                    self.f_10014040(20, 255);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_E214) {
                    self.f_10005590(this, 30);
                }
            }
            // Throwing back to the pitcher.
            3 => {
                let table = 0x1002_2AD4;
                self.prefix_table(table, 0x1001_E280, 0x1001_E2E8);
                if self.r(this, 112) == 0 {
                    let ball = self.p(this + 16);
                    let v = |n: u32| self.i(ball + 4 * n);
                    let (x, y, z) = (v(6), v(7), v(8));
                    let (dx, dy, dz) = (v(12), v(13), v(14));
                    let speed = ftol(f64::from(v(15)) * 0.3);
                    let lift = ftol(f64::from(v(20)) * 0.3);
                    self.f_10002bd0(x, y, z, dx, dy, dz, speed, lift);
                    self.f_10014040(16, 255);
                    self.f_10014080(10);
                    let number = self.w(this, 312);
                    self.f_10004400(number + 10, 50, false);
                    if self.r(this, 12) == 7 {
                        let (a3, a5) = (self.actor(3), self.actor(5));
                        self.vcall(a3, 20, 3, 0);
                        self.vcall(a5, 20, 3, 0);
                    }
                }
                let pattern = self.table_seek(this, table, 0x1001_E218, 0x1001_E2E8);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == self.i(0x1002_2AE0) {
                    self.sr(this, 128, 1);
                    self.sr(this, 132, 1);
                }
                let end = self.i(0x1002_2B34);
                if self.r(this, 112) == end {
                    self.f_100140a0(10);
                    self.sr(this, 128, 4);
                    self.sr(this, 132, 1);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == end {
                    self.f_10005590(this, 30);
                }
            }
            // Running round the field.
            4 => {
                let a = f64::from(self.r(this, 124)) * 0.003333333333333334 + 3.1415926535;
                self.sr(this, 68, ftol(a.cos() * 4500.0));
                self.sr(this, 72, 0);
                self.sr(this, 76, ftol(a.sin() * 4500.0 + 5000.0));
                let first = i32::from(self.r(this, 112) == 0);
                self.f_100051c0(this, 0, 0, first);
                self.f_10005100(this, 15.0);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            // Looping animations.
            state @ (5 | 6) => {
                let (table, source, count, patterns) = if state == 5 {
                    (0x1002_2B3C, 0x1001_E300, 0x1001_E314, 0x1001_E2EC)
                } else {
                    (0x1002_2B50, 0x1001_E330, 0x1001_E348, 0x1001_E318)
                };
                self.prefix_table(table, source, count);
                let pattern = self.table_seek(this, table, patterns, count);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, count) {
                    self.sr(this, 112, 0);
                }
            }
            7 => {
                self.f_10005090(this, 41, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            8 => {
                const SETS: [(u32, u32, u32, u32); 6] = [
                    (0x1001_E35C, 0x1001_E34C, 0x1001_E354, 0x1002_2B68),
                    (0x1001_E370, 0x1001_E360, 0x1001_E368, 0x1002_2B70),
                    (0x1001_E3BC, 0x1001_E374, 0x1001_E398, 0x1002_2B78),
                    (0x1001_E3D0, 0x1001_E3C0, 0x1001_E3C8, 0x1002_2B9C),
                    (0x1001_E424, 0x1001_E3D4, 0x1001_E3FC, 0x1002_2BA4),
                    (0x1001_E458, 0x1001_E428, 0x1001_E440, 0x1002_2BCC),
                ];
                self.scene_set(this, &SETS, true);
            }
            9 => {
                let table = 0x1002_2BE4;
                self.prefix_table(table, 0x1001_E464, 0x1001_E46C);
                let (mirror, mode) = if self.g(this, 3232) == 3 {
                    (-100, 7)
                } else {
                    (100, 6)
                };
                let pattern = self.table_seek(this, table, 0x1001_E45C, 0x1001_E46C);
                self.f_10005090(this, pattern, mirror);
                if self.r(this, 112) == 0 {
                    self.sr(this, 128, mode);
                    self.sr(this, 132, 1);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.g(this, 3232) == 3 && self.g(this, 4496) != 9 {
                    self.vcall(this, 20, 7, 0);
                }
                if self.g(this, 3232) == 5 && self.g(this, 4816) != 7 {
                    self.vcall(this, 20, 7, 0);
                }
            }
            10 => {
                let table = 0x1002_2BEC;
                self.prefix_table(table, 0x1001_E498, 0x1001_E4C0);
                let pattern = self.table_seek(this, table, 0x1001_E470, 0x1001_E4C0);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == 0 {
                    self.f_10014040(30, 255);
                    self.f_10014080(120);
                }
                if self.r(this, 112) < self.i(0x1002_2C04) {
                    self.sr(this, 68, self.r(this, 40));
                    self.sr(this, 72, self.r(this, 44));
                    self.sr(this, 76, self.r(this, 48) + 1000);
                    self.f_10005100(this, 4.0);
                }
                self.f_100083b0(this);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            11 => {
                self.f_10005090(this, 54, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            12 => {
                self.sr(this, 20, 0);
                self.si(0x1001_E538, 232);
                let table = 0x1002_2C3C;
                self.prefix_table(table, 0x1001_E538, 0x1001_E558);
                let total = self.table_total(table, 0x1001_E558);
                if total != 0 {
                    self.sr(this, 112, self.r(this, 112) % total);
                }
                let pattern = self.table_seek(this, table, 0x1001_E518, 0x1001_E558);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == self.i(table) {
                    for off in [7324, 7356, 7360, 7364] {
                        self.sg(this, off, 0);
                    }
                }
                if self.r(this, 112) == self.i(0x1002_2C4C) {
                    self.sg(this, 7324, 1);
                    self.sg(this, 7328, self.r(this, 40));
                    self.sg(this, 7332, 300);
                    self.sg(this, 7336, self.r(this, 48));
                    self.sg(this, 7340, self.g(this, 7328));
                    self.sg(this, 7344, self.g(this, 7332));
                    self.sg(this, 7348, self.g(this, 7336));
                    self.sg(this, 7352, 0);
                    self.sg(this, 7356, 34);
                    self.sg(this, 7360, 72);
                    self.sg(this, 7364, 0);
                    if self.g(this, 1400) == 3 {
                        self.f_10004400(3, 50, true);
                    }
                }
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            13 => self.patrol(this, 101),
            // Catching the ball low (14) or high (15).
            state @ (14 | 15) => {
                if self.r(this, 112) == self.r(this, 120) - 50 {
                    let number = self.w(this, 312);
                    self.f_10004400(number + 10, 50, false);
                }
                let (t, catch_at) = (self.r(this, 112), self.r(this, 120));
                if state == 14 {
                    if t < catch_at {
                        if self.at_target(this) {
                            self.f_10005090(this, 135, 100);
                        } else {
                            let first = i32::from(t == 0);
                            self.f_10005310(this, 0, 101, first);
                            self.f_10005100(this, 15.0);
                        }
                    }
                } else if t >= catch_at {
                    self.f_10005090(this, 136, 100);
                } else if self.at_target(this) {
                    self.f_10005090(this, 137, 100);
                } else {
                    let first = i32::from(t == 0);
                    self.f_10005310(this, 0, 101, first);
                    self.f_10005100(this, 15.0);
                }
                if self.r(this, 112) == self.r(this, 120) {
                    let ball = self.p(this + 16);
                    self.si(ball, 0);
                    self.si(ball + 60, 0);
                    self.f_10014040(15, 255);
                    self.f_10014080(if state == 14 { 12 } else { 13 });
                }
                if state == 14 && self.r(this, 120) <= self.r(this, 112) {
                    self.f_10005090(this, 134, 100);
                }
                let t = self.r(this, 112);
                if t == self.r(this, 120) + 48 {
                    self.vcall(this, 20, 16, 0);
                } else {
                    self.sr(this, 112, t + 1);
                }
            }
            16 => {
                let table = 0x1002_2C5C;
                self.prefix_table(table, 0x1001_E56C, 0x1001_E57C);
                let pattern = self.table_seek(this, table, 0x1001_E55C, 0x1001_E57C);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == 36 {
                    self.f_1000ac70(this);
                    self.f_10014040(6, 255);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_E57C) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            17 => {
                self.walk_to_target(this);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            18 => {
                self.sr(this, 68, self.r(this, 28));
                self.sr(this, 72, self.r(this, 32));
                self.sr(this, 76, self.r(this, 36));
                self.walk_to_target(this);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.at_home(this) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            _ => {}
        }
    }

    /// Scene animations chosen by `this[311]` (sets 0 and 1 share a table;
    /// with `mirror`, set 0 faces left and set 1 right).
    pub(crate) fn scene_set(&mut self, this: u32, sets: &[(u32, u32, u32, u32); 6], mirror: bool) {
        let set = self.w(this, 311);
        if set == 99 || !(0..=6).contains(&set) {
            return;
        }
        let (count_at, patterns, source, table) = sets[(set.max(1) - 1) as usize];
        self.prefix_table(table, source, count_at);
        if mirror {
            match set {
                0 => self.sr(this, 84, -100),
                1 => self.sr(this, 84, 100),
                _ => {}
            }
        }
        let pattern = self.table_seek(this, table, patterns, count_at);
        let speed = self.r(this, 84);
        self.f_10005090(this, pattern, speed);
        self.sr(this, 112, self.r(this, 112) + 1);
    }

    /// Pacing between two points either side of home (`pattern` is the
    /// walking animation).
    pub(crate) fn patrol(&mut self, this: u32, pattern: i32) {
        let (hx, hy, hz) = (self.r(this, 28), self.r(this, 32), self.r(this, 36));
        let (a, b) = match hx {
            x if x > 0 => ([x - 1400, hy, hz + 700], [x + 1400, hy, hz - 700]),
            x if x < 0 => ([x - 1400, hy, hz - 700], [x + 1400, hy, hz + 700]),
            x => ([x - 1400, hy, hz], [x + 1400, hy, hz]),
        };
        let target = [self.r(this, 68), self.r(this, 72), self.r(this, 76)];
        if target != a && target != b {
            self.set_target(this, a);
        }
        if self.at_target(this) {
            let target = [self.r(this, 68), self.r(this, 72), self.r(this, 76)];
            self.set_target(this, if target == a { b } else { a });
        }
        let first = i32::from(self.r(this, 112) == 0);
        self.f_10005310(this, 0, pattern, first);
        self.f_10005100(this, 15.0);
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) >= 200 {
            self.sr(this, 20, 0);
        }
    }

    pub(crate) fn set_target(&mut self, this: u32, p: [i32; 3]) {
        self.sr(this, 68, p[0]);
        self.sr(this, 72, p[1]);
        self.sr(this, 76, p[2]);
    }

    /// `sub_100079F0` (state 1): strolling around home.
    fn f_100079f0(&mut self, this: u32) {
        let a = f64::from(self.r(this, 124) % 360) * 0.01745329251944444;
        self.sr(
            this,
            68,
            ftol(a.cos() * 1000.0 + f64::from(self.r(this, 28))),
        );
        self.sr(this, 72, 0);
        self.sr(
            this,
            76,
            ftol(a.sin() * 1000.0 + f64::from(self.r(this, 36))),
        );
        let first = i32::from(self.r(this, 112) == 0);
        self.f_100051c0(this, 0, 20, first);
        self.f_10005100(this, 15.0);
        self.sr(this, 112, self.r(this, 112) + 1);
    }

    /// `sub_100083B0`: the thrown cap (`F[1805..=1809]`).
    fn f_100083b0(&mut self, this: u32) {
        let table = 0x1002_2C14;
        self.prefix_table(table, 0x1001_E4EC, 0x1001_E514);
        self.sr(this, 108, 0);
        let count = self.i(0x1001_E514);
        let mut index = 0;
        while index < count - 1 && self.r(this, 112) >= self.i(table + 4 * index as u32) {
            index += 1;
            self.sr(this, 108, index);
        }
        self.sg(
            this,
            7236,
            self.i(0x1001_E4C4 + 4 * self.r(this, 108) as u32),
        );
        if self.r(this, 112) == 0 {
            self.sg(this, 7220, 1);
        }
        self.sg(this, 7224, self.r(this, 28));
        self.sg(this, 7232, self.r(this, 36) - 100);
        let span = self.i(0x1002_2C20);
        let height = arc(self.r(this, 112), span / -2, span / 2, 100);
        self.sg(this, 7228, height.max(0));
    }

    /// `sub_1000AC70`: throws to the target.
    pub(crate) fn f_1000ac70(&mut self, this: u32) {
        let (x, y, z) = (self.r(this, 40), self.r(this, 44) + 200, self.r(this, 48));
        self.throw_at_target(x, y, z, 400, 400);
    }
}
