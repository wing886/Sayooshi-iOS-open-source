//! Actor 5 (vtable `0x1001C1AC`).

use super::super::base::{distance, segment_foot};
use super::super::{BALL, Pt00, ftol};

impl Pt00 {
    /// `sub_1000C2B0` (slot 6).
    pub(crate) fn f_1000c2b0(&mut self, this: u32) {
        let mode = self.r(this, 12);
        if (mode == 9 || mode == 10 || self.f_10005500(this)) && !self.f_1000c380(this) {
            self.sr(this, 24, 0);
        }
    }

    /// `sub_1000C300` (slot 7).
    pub(crate) fn f_1000c300(&mut self, this: u32) {
        match self.r(this, 12) {
            9 => {
                self.vcall(this, 20, 5, 0);
            }
            10 => {
                self.vcall(this, 20, 6, 0);
            }
            _ if self.f_10005500(this) => {
                if self.at_home(this) {
                    if self.r(this, 16) != 2 {
                        self.vcall(this, 20, 1, 0);
                    }
                } else {
                    self.vcall(this, 20, 9, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_1000C380`: take a throw arriving at predicted frames 35-38.
    fn f_1000c380(&mut self, this: u32) -> bool {
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let d = segment_foot(
            f64::from(self.i(0x1002_26B4)),
            f64::from(self.i(0x1002_26BC)),
            f64::from(self.i(0x1002_26D8)),
            f64::from(self.i(0x1002_26E0)),
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        )
        .distance;
        if d >= 160.0 || f64::from(self.i(0x1002_26DC) - self.r(this, 44)) >= 600.0 {
            return false;
        }
        self.vcall(this, 20, 7, 0);
        self.f_10005460(this);
        let number = self.w(this, 312);
        self.f_10004400(number + 10, 50, false);
        self.sg(this, 3232, 5);
        true
    }

    /// `sub_1000CA80`: relays the ball back the way it came.
    fn f_1000ca80(&mut self, this: u32) {
        let ball = self.p(this + 16);
        let v = |pt: &Self, n: u32| pt.i(ball + 4 * n);
        let (x, y, z) = (v(self, 6), v(self, 7), v(self, 8));
        let (dx, dy, dz) = (-v(self, 12), -v(self, 13), -v(self, 14));
        self.f_10002bd0(x, y, z, dx, dy, dz, 100, 300);
        self.f_10004400(1, 50, false);
    }

    /// Walking with `sub_100051C0` at `step`.
    pub(crate) fn walk_field(&mut self, this: u32, set: i32, step: f64) {
        let first = i32::from(self.r(this, 112) == 0);
        self.f_100051c0(this, set, 0, first);
        self.f_10005100(this, step);
    }

    /// `sub_1000C490` (slot 8).
    pub(crate) fn f_1000c490(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => {
                self.f_10005090(this, 1, -100);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            2 => {
                const SETS: [(u32, u32, u32, u32); 6] = [
                    (0x1001_EDA0, 0x1001_ED90, 0x1001_ED98, 0x1002_306C),
                    (0x1001_EDB4, 0x1001_EDA4, 0x1001_EDAC, 0x1002_3074),
                    (0x1001_EE00, 0x1001_EDB8, 0x1001_EDDC, 0x1002_307C),
                    (0x1001_EE14, 0x1001_EE04, 0x1001_EE0C, 0x1002_30A0),
                    (0x1001_EE68, 0x1001_EE18, 0x1001_EE40, 0x1002_30A8),
                    (0x1001_EE9C, 0x1001_EE6C, 0x1001_EE84, 0x1002_30D0),
                ];
                self.scene_set(this, &SETS, false);
            }
            3 => {
                self.f_10005090(this, 9, -100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.g(this, 3232) == 1 && self.g(this, 4176) != 3 {
                    self.vcall(this, 20, 1, 0);
                }
                if self.g(this, 3232) == 3 && self.g(this, 4496) != 9 {
                    self.vcall(this, 20, 1, 0);
                }
            }
            4 => {
                self.f_10005090(this, 9, -100);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            5 => {
                if self.r(this, 112) == 0 {
                    self.f_100050c0(this, 2, 100, 0, false);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            // Jogging between two points beyond the base.
            6 => {
                if self.b(0x1002_3048) & 1 == 0 {
                    self.si(0x1002_3060, 11500);
                    self.si(0x1002_3064, 310);
                    self.si(0x1002_3068, 4000);
                    let flags = self.b(0x1002_3048) | 1;
                    self.sb(0x1002_3048, flags);
                }
                if self.b(0x1002_3048) & 2 == 0 {
                    self.si(0x1002_3050, 7500);
                    self.si(0x1002_3054, 310);
                    self.si(0x1002_3058, 2000);
                    let flags = self.b(0x1002_3048) | 2;
                    self.sb(0x1002_3048, flags);
                }
                let far = [
                    self.i(0x1002_3060),
                    self.i(0x1002_3064),
                    self.i(0x1002_3068),
                ];
                let near = [
                    self.i(0x1002_3050),
                    self.i(0x1002_3054),
                    self.i(0x1002_3058),
                ];
                let span = ftol(
                    distance(
                        f64::from(far[0]),
                        f64::from(far[2]),
                        f64::from(near[0]),
                        f64::from(near[2]),
                    ) * 0.1333333333333333,
                );
                let period = (2 * span).max(1);
                let target = if self.r(this, 124) % period >= period / 2 {
                    near
                } else {
                    far
                };
                self.set_target(this, target);
                self.walk_field(this, 0, 7.0);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            7 => {
                let table = 0x1002_30E8;
                self.prefix_table(table, 0x1001_EEC0, 0x1001_EEE0);
                if self.r(this, 112) == 0 {
                    let ball = self.p(this + 16);
                    let mirror = if self.i(ball + 48) <= 0 { 100 } else { -100 };
                    self.sr(this, 84, mirror);
                }
                let pattern = self.table_seek(this, table, 0x1001_EEA0, 0x1001_EEE0);
                let speed = self.r(this, 84);
                self.f_10005090(this, pattern, speed);
                if self.r(this, 112) == 38 {
                    self.f_10014040(17, 255);
                    self.f_10014080(50);
                    self.f_1000ca80(this);
                    if self.r(this, 12) == 7 {
                        let (a1, a3) = (self.actor(1), self.actor(3));
                        self.vcall(a1, 20, 9, 0);
                        self.vcall(a3, 20, 3, 0);
                    }
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_EEE0) {
                    self.f_10005590(this, 30);
                }
            }
            8 => {
                self.walk_field(this, 0, 15.0);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            9 => {
                let home = [self.r(this, 28), self.r(this, 32), self.r(this, 36)];
                self.set_target(this, home);
                self.walk_field(this, 0, 15.0);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.at_home(this) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            _ => {}
        }
    }
}
