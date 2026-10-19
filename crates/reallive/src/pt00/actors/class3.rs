//! Actor 3 (vtable `0x1001C1F4`).

use super::super::Pt00;
use super::super::base::{arc, direction8, distance};

impl Pt00 {
    /// `sub_100095B0` (slot 6).
    pub(crate) fn f_100095b0(&mut self, this: u32) {
        match self.r(this, 12) {
            3 => {
                if !self.f_100099d0(this) {
                    self.sr(this, 24, 0);
                }
            }
            8 => {
                if !self.pitch_catch(this, 9, 3) {
                    self.sr(this, 24, 0);
                }
            }
            _ if self.f_10005500(this) => {
                if !self.pitch_catch(this, 9, 3) {
                    self.sr(this, 24, 0);
                }
            }
            0 => self.f_10004ef0(this),
            _ => {}
        }
    }

    /// `sub_10009630` (slot 7).
    pub(crate) fn f_10009630(&mut self, this: u32) {
        if self.f_10005500(this) {
            if self.at_home(this) {
                if self.r(this, 16) != 2 {
                    self.vcall(this, 20, 1, 0);
                }
            } else {
                self.vcall(this, 20, 15, 0);
                self.sr(this, 68, self.r(this, 28));
                self.sr(this, 72, self.r(this, 32));
                self.sr(this, 76, self.r(this, 36));
            }
            return;
        }
        match self.r(this, 12) {
            8 => {
                let state = if self.b(0x1002_29EC) != 0 { 6 } else { 5 };
                self.vcall(this, 20, state, 0);
            }
            3 => {
                self.vcall(this, 20, 7, 0);
            }
            0 => {
                if self.r(this, 24) == 1 {
                    if !self.fly_catch(this, 11, 12) {
                        self.chase_landing(this);
                        self.vcall(this, 20, 14, 0);
                    }
                } else {
                    self.vcall(this, 20, 10, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_100099D0`: take a pitch that comes close.
    fn f_100099d0(&mut self, this: u32) -> bool {
        let ball = self.p(this + 16);
        if distance(
            f64::from(self.i(ball + 24)),
            f64::from(self.i(ball + 32)),
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        ) >= 600.0
            || self.i(ball + 32) >= self.r(this, 48)
        {
            return false;
        }
        self.vcall(this, 20, 8, 0);
        self.sr(this, 56, self.r(this, 40));
        self.sr(this, 60, self.r(this, 44));
        self.sr(this, 64, self.r(this, 48));
        self.sr(
            this,
            68,
            self.r(this, 36) + self.r(this, 40) - self.r(this, 48),
        );
        self.sr(this, 72, 0);
        self.sr(
            this,
            76,
            self.r(this, 40) + self.r(this, 48) - self.r(this, 28),
        );
        self.f_10005460(this);
        let number = self.w(this, 312);
        self.f_10004400(number + 10, 50, false);
        true
    }

    /// Catches at `record[120]` (low `low_pattern`/high `high_pattern`).
    fn catch_at_frame(&mut self, this: u32, high: bool, patterns: (i32, i32), line: i32) {
        if self.r(this, 112) == self.r(this, 120) - 50 {
            let number = self.w(this, 312);
            self.f_10004400(number + 10, 50, false);
        }
        let (t, at) = (self.r(this, 112), self.r(this, 120));
        if !high {
            if t < at {
                if self.at_target(this) {
                    self.f_10005090(this, patterns.1, 100);
                } else {
                    let first = i32::from(t == 0);
                    self.f_10005310(this, 0, 108, first);
                    self.f_10005100(this, 15.0);
                }
            }
        } else if t >= at {
            self.f_10005090(this, patterns.0, 100);
        } else if self.at_target(this) {
            self.f_10005090(this, patterns.1, 100);
        } else {
            let first = i32::from(t == 0);
            self.f_10005310(this, 0, 108, first);
            self.f_10005100(this, 15.0);
        }
        if self.r(this, 112) == self.r(this, 120) {
            self.f_10005480(this);
            self.f_10014040(15, 255);
            self.f_10014080(line);
        }
        if !high && self.r(this, 120) <= self.r(this, 112) {
            self.f_10005090(this, patterns.0, 100);
        }
        let t = self.r(this, 112);
        if t == self.r(this, 120) + 48 {
            self.vcall(this, 20, 13, 0);
        } else {
            self.sr(this, 112, t + 1);
        }
    }

    /// `sub_10009B60` (slot 8).
    pub(crate) fn f_10009b60(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => {
                self.f_10005090(this, 48, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            2 => {
                const SETS: [(u32, u32, u32, u32); 6] = [
                    (0x1001_E8C4, 0x1001_E8B4, 0x1001_E8BC, 0x1002_2E00),
                    (0x1001_E8D8, 0x1001_E8C8, 0x1001_E8D0, 0x1002_2E08),
                    (0x1001_E924, 0x1001_E8DC, 0x1001_E900, 0x1002_2E10),
                    (0x1001_E938, 0x1001_E928, 0x1001_E930, 0x1002_2E34),
                    (0x1001_E98C, 0x1001_E93C, 0x1001_E964, 0x1002_2E3C),
                    (0x1001_E9C0, 0x1001_E990, 0x1001_E9A8, 0x1002_2E64),
                ];
                self.scene_set(this, &SETS, false);
            }
            3 => {
                let table = 0x1002_2E7C;
                self.prefix_table(table, 0x1001_E9CC, 0x1001_E9D4);
                let pattern = self.table_seek(this, table, 0x1001_E9C4, 0x1001_E9D4);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == 0 {
                    self.sr(this, 128, 6);
                    self.sr(this, 132, 1);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.g(this, 3232) == 1 && self.g(this, 4176) != 3 {
                    self.vcall(this, 20, 1, 0);
                }
                if self.g(this, 3232) == 5 && self.g(this, 4816) != 7 {
                    self.vcall(this, 20, 1, 0);
                }
            }
            // Hopping sideways with the thrown cap and glove.
            4 => {
                let table = 0x1002_2F20;
                self.prefix_table(table, 0x1001_EB2C, 0x1001_EB40);
                let pattern = self.table_seek(this, table, 0x1001_EB18, 0x1001_EB40);
                self.f_10005090(this, pattern, 100);
                let t = self.r(this, 112);
                let slide = self.i(0x1002_2F2C);
                self.sr(this, 40, self.r(this, 28) + 3 * t.min(slide));
                let hop = self.i(0x1002_2F24);
                let (a2, a3, a4) = if t >= hop {
                    (hop, (hop + slide) / 2, 30)
                } else {
                    (0, hop / 2, 50)
                };
                self.sr(this, 44, arc(t, a2, a3, a4).max(0));
                self.f_1000a4e0(this);
                self.f_1000a600(this);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            5 => {
                self.f_10005090(this, 33, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            6 => {
                self.sr(this, 20, 0);
                self.si(0x1001_E9F8, 232);
                let table = 0x1002_2E84;
                self.prefix_table(table, 0x1001_E9F8, 0x1001_EA18);
                let total = self.table_total(table, 0x1001_EA18);
                if total == 0 {
                    return;
                }
                self.sr(this, 112, self.r(this, 112) % total);
                // Half a cycle out of phase with actor 1.
                let phase = (self.r(this, 112) + total / 2) % total;
                self.sr(this, 108, 0);
                let count = self.i(0x1001_EA18);
                let mut index = 0;
                while index < count - 1 && phase >= self.i(table + 4 * index as u32) {
                    index += 1;
                    self.sr(this, 108, index);
                }
                let pattern = self.i(0x1001_E9D8 + 4 * self.r(this, 108) as u32);
                self.f_10005090(this, pattern, 100);
                if phase == self.i(table) {
                    for off in [7324, 7356, 7360, 7364] {
                        self.sg(this, off, 0);
                    }
                }
                if phase == self.i(0x1002_2E94) {
                    self.sg(this, 7324, 1);
                    self.sg(this, 7328, self.r(this, 40));
                    self.sg(this, 7332, 300);
                    self.sg(this, 7336, self.r(this, 48));
                    self.sg(this, 7340, self.g(this, 7328));
                    self.sg(this, 7344, self.g(this, 7332));
                    self.sg(this, 7348, self.g(this, 7336));
                    self.sg(this, 7352, 0);
                    self.sg(this, 7356, -33);
                    self.sg(this, 7360, 72);
                    self.sg(this, 7364, 0);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            7 => self.f_1000a730(this),
            8 => {
                let table = 0x1002_2F94;
                self.prefix_table(table, 0x1001_EC20, 0x1001_EC34);
                self.table_seek(this, table, 0x1001_EC0C, 0x1001_EC34);
                let dx = self.r(this, 68) - self.r(this, 56);
                let dz = self.r(this, 76) - self.r(this, 64);
                let facing = direction8(f64::from(dx), f64::from(dz), self.r(this, 52));
                self.sr(this, 52, facing);
                if (0..=7).contains(&facing) {
                    let base = self.i(0x1001_EC0C + 4 * self.r(this, 108) as u32);
                    self.f_10005090(this, base + 4 * facing, 100);
                }
                let step = (10 * (60 - self.r(this, 112)) / 60).max(0);
                self.f_10005100(this, f64::from(step));
                if self.r(this, 112) == 0 {
                    self.f_10014080(31);
                }
                if self.r(this, 112) == self.i(table + 4) {
                    self.f_10014040(20, 255);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_EC34) {
                    self.f_10005590(this, 30);
                }
            }
            9 => {
                let table = 0x1002_2EA4;
                self.prefix_table(table, 0x1001_EA98, 0x1001_EB14);
                if self.r(this, 112) == 0 {
                    let ball = self.p(this + 16);
                    let v = |n: u32| self.i(ball + 4 * n);
                    let (x, y, z) = (v(6), v(7), v(8));
                    let (dx, dy, dz) = (v(12), v(13), v(14));
                    let speed = super::super::ftol(f64::from(v(15)) * 0.3);
                    let lift = super::super::ftol(f64::from(v(20)) * 0.3);
                    self.f_10002bd0(x, y, z, dx, dy, dz, speed, lift);
                    self.f_10014040(16, 255);
                    self.f_10014080(30);
                    let number = self.w(this, 312);
                    self.f_10004400(number + 10, 50, false);
                    if self.r(this, 12) == 7 {
                        let (a1, a5) = (self.actor(1), self.actor(5));
                        self.vcall(a1, 20, 9, 0);
                        self.vcall(a5, 20, 3, 0);
                    }
                }
                let pattern = self.table_seek(this, table, 0x1001_EA1C, 0x1001_EB14);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == self.i(table + 8) {
                    self.sr(this, 128, 1);
                    self.sr(this, 132, 1);
                }
                if self.r(this, 112) == self.i(0x1002_2F1C) {
                    self.f_100140a0(30);
                    self.sr(this, 128, 4);
                    self.sr(this, 132, 1);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_EB14) {
                    self.f_10005590(this, 30);
                }
            }
            10 => self.patrol(this, 108),
            11 => self.catch_at_frame(this, false, (141, 142), 32),
            12 => self.catch_at_frame(this, true, (143, 144), 33),
            13 => {
                let table = 0x1002_2FA8;
                self.prefix_table(table, 0x1001_EC48, 0x1001_EC58);
                let pattern = self.table_seek(this, table, 0x1001_EC38, 0x1001_EC58);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == 36 {
                    self.f_1000ac70(this);
                    self.f_10014040(6, 255);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_EC58) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            14 => {
                self.walk_plate(this);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            15 => {
                self.walk_plate(this);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.at_home(this) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            _ => {}
        }
    }

    /// Walking towards the target with pattern set 0 (in the field) or
    /// 108 (at the plate).
    fn walk_plate(&mut self, this: u32) {
        let first = i32::from(self.r(this, 112) == 0);
        let pattern = if self.r(this, 12) != 0 { 0 } else { 108 };
        self.f_10005310(this, 0, pattern, first);
        self.f_10005100(this, 15.0);
    }

    /// `sub_1000A4E0`: the thrown cap (`F[1810..=1814]`).
    fn f_1000a4e0(&mut self, this: u32) {
        let table = 0x1002_2F34;
        self.prefix_table(table, 0x1001_EB64, 0x1001_EB84);
        let pattern = self.table_seek(this, table, 0x1001_EB44, 0x1001_EB84);
        self.sg(this, 7256, pattern);
        if self.r(this, 112) == 0 {
            self.sg(this, 7240, 1);
        }
        let t = self.r(this, 112);
        let span = self.i(0x1002_2F38);
        let x = self.r(this, 28) - 5 * (t + 10);
        self.sg(this, 7244, x.max(self.r(this, 28) - (5 * span + 50)));
        self.sg(this, 7252, self.r(this, 36));
        self.sg(this, 7248, arc(t, -span, 0, 50).max(0));
    }

    /// `sub_1000A600`: the thrown glove (`F[1815..=1819]`).
    fn f_1000a600(&mut self, this: u32) {
        let table = 0x1002_2F54;
        self.prefix_table(table, 0x1001_EBC8, 0x1001_EC08);
        let pattern = self.table_seek(this, table, 0x1001_EB88, 0x1001_EC08);
        self.sg(this, 7276, pattern);
        let t = self.r(this, 112);
        if t == self.i(table) {
            self.sg(this, 7260, 1);
        }
        let span = self.i(0x1002_2F88);
        let x = self.r(this, 40) - 2 * t;
        let limit = self.r(this, 40) - 2 * span;
        self.sg(this, 7264, if limit <= x { x } else { limit });
        self.sg(this, 7272, self.r(this, 48));
        let height = arc(t, self.i(table), span / 2, 400) + 400;
        self.sg(this, 7268, height.max(100));
    }

    /// `sub_1000A730` (state 7): follow actor 10's trail, 50 frames behind.
    fn f_1000a730(&mut self, this: u32) {
        let leader = self.actor(10);
        let p = self.f_10005510(leader, 50);
        self.set_target(this, p);
        let first = i32::from(self.r(this, 112) == 0);
        self.f_10005310(this, 0, 0, first);
        self.f_10005100(this, 15.0);
        self.sr(this, 112, self.r(this, 112) + 1);
    }
}
