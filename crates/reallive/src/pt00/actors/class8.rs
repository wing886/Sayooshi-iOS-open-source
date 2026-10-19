//! Actor 8 (vtable `0x1001C164`), actor 7's partner.

use super::super::base::{direction8, distance, segment_foot};
use super::super::{BALL, Pt00};

/// Animation rows by facing for state 9: approach (three frames), then
/// holding and settling.
const RECEIVE: [[i32; 5]; 4] = [
    [150, 149, 148, 149, 150],
    [32, 143, 142, 143, 32],
    [28, 145, 144, 145, 28],
    [24, 147, 146, 147, 24],
];

impl Pt00 {
    /// `sub_1000E7F0` (slot 6).
    pub(crate) fn f_1000e7f0(&mut self, this: u32) {
        match self.r(this, 12) {
            3 | 4 => {
                if !self.lap_catch(this, false) {
                    self.sr(this, 24, 0);
                }
            }
            0 => {
                if !self.f_1000e8c0(this) {
                    self.sr(this, 24, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_1000E840` (slot 7).
    pub(crate) fn f_1000e840(&mut self, this: u32) {
        let state = match self.r(this, 12) {
            4 => 3,
            3 => 4,
            _ if self.g(this, 360) == 1 => 2,
            _ if self.at_home(this) => 1,
            _ => 12,
        };
        self.vcall(this, 20, state, 0);
    }

    /// `sub_1000E8C0`: run under a fly ball (offset for the glove), or in
    /// the practice mode `F[90]`, wait for it.
    fn f_1000e8c0(&mut self, this: u32) -> bool {
        let mut found = false;
        let mut best = 99999999.0;
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        if self.g(this, 360) != 1 {
            let mut frame = 42;
            let mut reach = 300;
            let mut p = 0x1002_26E4;
            while p < 0x1002_299C {
                let v = |pt: &Self, n: u32| pt.i(p + 4 * n);
                let d = segment_foot(
                    f64::from(v(self, 0) - 300),
                    f64::from(v(self, 2) + 200),
                    f64::from(v(self, 9) - 300),
                    f64::from(v(self, 11) + 200),
                    f64::from(self.r(this, 40)),
                    f64::from(self.r(this, 48)),
                )
                .distance;
                if f64::from(reach) > d
                    && f64::from(v(self, 10) - self.r(this, 44)) < 300.0
                    && self.f_10004310(v(self, 9), v(self, 11), None) == 0
                    && d < best
                {
                    best = d;
                    self.vcall(this, 20, 6, 0);
                    self.set_target(this, [v(self, 9) - 300, 0, v(self, 11) + 200]);
                    self.sr(this, 120, frame);
                    self.f_10005460(this);
                    found = true;
                }
                p += 36;
                frame += 3;
                reach += 60;
            }
            return found;
        }
        let mut frame = 40;
        let mut p = 0x1002_26CC;
        while p < 0x1002_299C {
            let v = |pt: &Self, n: u32| pt.i(p + 4 * n);
            let height = f64::from(v(self, 10) - self.r(this, 44));
            let d = segment_foot(
                f64::from(v(self, 0)),
                f64::from(v(self, 2)),
                f64::from(v(self, 9)),
                f64::from(v(self, 11)),
                f64::from(self.r(this, 40)),
                f64::from(self.r(this, 48)),
            )
            .distance;
            if d < best && d < 240.0 && height < 400.0 {
                best = d;
                self.vcall(this, 20, 7, 0);
                let here = [self.r(this, 40), 0, self.r(this, 48)];
                self.set_target(this, here);
                self.sr(this, 120, frame);
                self.f_10005460(this);
                found = true;
            }
            p += 36;
            frame += 3;
        }
        found
    }

    /// `sub_1000EE90` (slot 8).
    pub(crate) fn f_1000ee90(&mut self, this: u32) {
        match self.r(this, 16) {
            state @ (1 | 2) => {
                let (table, source, count, patterns) = if state == 1 {
                    (0x1002_31EC, 0x1001_F184, 0x1001_F1B4, 0x1001_F154)
                } else {
                    (0x1002_321C, 0x1001_F1E8, 0x1001_F218, 0x1001_F1B8)
                };
                self.prefix_table(table, source, count);
                if self.r(this, 112) == 0 {
                    let start = self.rand() % 36;
                    self.sr(this, 112, start);
                }
                let pattern = self.table_step(this, table, patterns);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.table_total(table, count) <= self.r(this, 112) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            3 => {
                let p = self.f_10005690(this, 0);
                self.set_target(this, p);
                let first = self.r(this, 112) == 0;
                self.f_1000e430(this, first);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            4 => {
                let saved = [self.r(this, 40), self.r(this, 44), self.r(this, 48)];
                let p = self.f_100057a0(this, 0);
                self.set_target(this, p);
                let first = self.r(this, 112) == 0;
                self.f_1000e430(this, first);
                let partner = self.actor(7);
                if distance(
                    f64::from(self.r(this, 40)),
                    f64::from(self.r(this, 48)),
                    f64::from(self.r(partner, 40)),
                    f64::from(self.r(partner, 48)),
                ) < 720.0
                {
                    self.sr(this, 40, saved[0]);
                    self.sr(this, 44, saved[1]);
                    self.sr(this, 48, saved[2]);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            5 => self.lap_collision(this, 81, false),
            state @ (6 | 7 | 8) => {
                self.camera_before_catch(this);
                if state == 8 && self.r(this, 112) == 0 {
                    let p = [self.w(this, 314), self.w(this, 315), self.w(this, 316)];
                    self.set_target(this, p);
                }
                let (lead, table, source, count, patterns) = match state {
                    6 => (42, 0x1002_324C, 0x1001_F244, 0x1001_F26C, 0x1001_F21C),
                    7 => (40, 0x1002_3274, 0x1001_F298, 0x1001_F2C0, 0x1001_F270),
                    _ => (10, 0x1002_329C, 0x1001_F2E4, 0x1001_F304, 0x1001_F2C4),
                };
                if self.r(this, 112) >= self.r(this, 120) - lead {
                    self.prefix_table(table, source, count);
                    let offset = lead - self.r(this, 120);
                    let pattern = self.i(patterns + 4 * self.r(this, 108) as u32);
                    self.f_10005090(this, pattern, 100);
                    if offset + self.r(this, 112) == lead {
                        if state == 8 {
                            let partner = self.actor(7);
                            self.throw_to_partner(this, partner, 4);
                        } else {
                            let ball = self.p(this + 16);
                            let (x, y, z) =
                                (self.i(ball + 24), self.i(ball + 28), self.i(ball + 32));
                            self.throw_at_target(x, y, z, 700, 300);
                        }
                        self.f_10014040(13, 255);
                        self.f_10014080(80);
                    }
                    self.sr(this, 112, self.r(this, 112) + 1);
                    let index = self.r(this, 108);
                    if self.i(table + 4 * index as u32) <= offset + self.r(this, 112) {
                        self.sr(this, 108, index + 1);
                        if self.i(count) <= self.r(this, 108) {
                            self.vcall(this, 16, 0, 0);
                        }
                    }
                } else if state == 7 {
                    self.f_10005090(this, 162, 100);
                    self.sr(this, 112, self.r(this, 112) + 1);
                } else if self.at_target(this) {
                    self.f_10005090(this, 129, 100);
                    self.sr(this, 112, self.r(this, 112) + 1);
                } else {
                    let first = i32::from(self.r(this, 112) == 0);
                    self.f_1000e3e0(this, 20.0, first);
                    self.sr(this, 112, self.r(this, 112) + 1);
                }
            }
            // Taking a relay from actor 9.
            9 => {
                if self.r(this, 112) == 0 {
                    let from = self.actor(9);
                    let facing = direction8(
                        f64::from(self.r(from, 40) - self.r(this, 40)),
                        f64::from(self.r(from, 48) - self.r(this, 48)),
                        self.r(this, 52),
                    );
                    let row = match facing {
                        5 => 3,
                        6 => 2,
                        7 => 1,
                        _ => 0,
                    };
                    self.sr(this, 52, row);
                }
                let row = RECEIVE[self.r(this, 52).clamp(0, 3) as usize];
                let (t, at) = (self.r(this, 112), self.r(this, 120));
                let pattern = if t >= at {
                    if t < at + 8 { row[3] } else { row[4] }
                } else if t < 8 {
                    row[0]
                } else if t >= 16 {
                    row[2]
                } else {
                    row[1]
                };
                self.f_10005090(this, pattern, 100);
                if t == at {
                    self.hold_ball(this);
                    self.f_10014040(15, 255);
                    self.f_10014080(82);
                }
                if t >= at {
                    self.vcall(this, 20, 10, 0);
                } else {
                    self.sr(this, 112, t + 1);
                }
            }
            10 => {
                let table = 0x1002_32BC;
                self.prefix_table(table, 0x1001_F33C, 0x1001_F370);
                let mut index = 0;
                while self.r(this, 112) >= self.i(table + 4 * index as u32) {
                    index += 1;
                    if index > 1000 {
                        break;
                    }
                }
                let count = self.i(0x1001_F370);
                self.sr(this, 108, index.clamp(0, (count - 1).max(0)));
                let pattern = self.i(0x1001_F308 + 4 * self.r(this, 108) as u32);
                self.f_10005090(this, pattern, 100);
                let t = self.r(this, 112);
                if t == self.i(table) {
                    let (x, y, z) = (
                        self.r(this, 40) - 240,
                        self.r(this, 44) + 400,
                        self.r(this, 48) - 1,
                    );
                    self.f_10002bd0(x, y, z, 0, 1, 0, 0, 130);
                }
                if t == self.i(0x1002_32D0) {
                    self.hold_ball(this);
                    let ball = self.p(this + 16);
                    self.sr(this, 56, self.i(ball + 24));
                    self.sr(this, 60, self.i(ball + 28));
                    self.sr(this, 64, self.i(ball + 32));
                }
                if t == self.i(0x1002_32E0) {
                    let (x, y, z) = (self.r(this, 56), self.r(this, 60), self.r(this, 64));
                    self.throw_at_target(x, y, z, 700, 300);
                }
                self.sr(this, 112, t + 1);
                if self.r(this, 112) >= self.table_total(table, 0x1001_F370) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            11 => {
                let first = i32::from(self.r(this, 112) == 0);
                self.f_1000e3e0(this, 20.0, first);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            12 => {
                let home = [self.r(this, 28), self.r(this, 32), self.r(this, 36)];
                self.set_target(this, home);
                let first = i32::from(self.r(this, 112) == 0);
                self.f_1000e3e0(this, 20.0, first);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.at_target(this) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            _ => {}
        }
    }
}
