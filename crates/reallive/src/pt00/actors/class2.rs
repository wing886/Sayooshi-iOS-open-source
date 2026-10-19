//! Actor 2 (vtable `0x1001C218`).

use super::super::base::{distance, segment_foot, segment_intersection};
use super::super::{BALL, Pt00, ftol};

impl Pt00 {
    /// `sub_10008A50`: plays scene animation set `set` (used on actor 1).
    pub(crate) fn f_10008a50(&mut self, this: u32, set: i32) {
        self.sr(this, 16, 0);
        self.vcall(this, 20, 8, 0);
        self.sw(this, 311, set);
    }

    /// `sub_10008AD0` (slot 6).
    pub(crate) fn f_10008ad0(&mut self, this: u32) {
        match self.r(this, 12) {
            0 => {
                if !self.f_10008b40(this) {
                    self.sr(this, 24, 0);
                }
            }
            11 => {
                if !self.f_10008d10(this) {
                    self.sr(this, 24, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_10008B10` (slot 7).
    pub(crate) fn f_10008b10(&mut self, this: u32) {
        match self.r(this, 12) {
            0 => {
                self.vcall(this, 20, 1, 0);
            }
            11 => {
                self.vcall(this, 20, 5, 0);
            }
            _ => {}
        }
    }

    /// `sub_10008B40`: cut the ball off where its path crosses the
    /// actor's diagonal.
    fn f_10008b40(&mut self, this: u32) -> bool {
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let mut frame = 49;
        let mut point = 0x1002_2738;
        let state = loop {
            let (hx, hz) = (self.r(this, 28), self.r(this, 36));
            let v = |n: u32| f64::from(self.i(point + 4 * n));
            let crossing = segment_intersection([
                v(0),
                v(2),
                v(9),
                v(11),
                f64::from(hx + 1400),
                f64::from(hz + 700),
                f64::from(hx - 1400),
                f64::from(hz - 700),
            ]);
            if let Some(hit) = crossing {
                if self.i(point + 40) < 600 {
                    let d = distance(
                        f64::from(self.r(this, 40)),
                        f64::from(self.r(this, 48)),
                        hit.x,
                        hit.y,
                    );
                    self.sr(this, 68, ftol(hit.x));
                    self.sr(this, 72, 0);
                    self.sr(this, 76, ftol(hit.y));
                    if d <= 160.0 {
                        break 2;
                    }
                    if d <= 640.0 {
                        break 4;
                    }
                    let partner = self.actor(8);
                    if self.r(partner, 0) == 1 {
                        break 3;
                    }
                }
            }
            point += 36;
            frame += 3;
            if point >= 0x1002_299C {
                return false;
            }
        };
        self.f_10005570(this, state, frame - 49);
        self.f_10005460(this);
        true
    }

    /// `sub_10008D10`: take a throw arriving at predicted frame 48.
    fn f_10008d10(&mut self, this: u32) -> bool {
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let height = f64::from(self.i(0x1002_2760) - self.r(this, 44));
        let d = segment_foot(
            f64::from(self.i(0x1002_2750)),
            f64::from(self.i(0x1002_2758)),
            f64::from(self.i(0x1002_275C)),
            f64::from(self.i(0x1002_2764)),
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        )
        .distance;
        if d >= 99999999.0 || d >= 160.0 || height >= 600.0 {
            return false;
        }
        self.vcall(this, 20, 6, 0);
        let number = self.w(this, 312);
        self.f_10004400(number + 10, 50, false);
        self.f_10005460(this);
        true
    }

    /// Takes the ball out of play (a catch).
    pub(crate) fn hold_ball(&mut self, this: u32) {
        let ball = self.p(this + 16);
        self.si(ball, 0);
        self.si(ball + 60, 0);
    }

    pub(crate) fn go_to_target(&mut self, this: u32) {
        self.sr(this, 40, self.r(this, 68));
        self.sr(this, 44, self.r(this, 72));
        self.sr(this, 48, self.r(this, 76));
    }

    pub(crate) fn go_home(&mut self, this: u32) {
        self.sr(this, 40, self.r(this, 28));
        self.sr(this, 44, self.r(this, 32));
        self.sr(this, 48, self.r(this, 36));
    }

    /// `sub_100093F0`: throws on from the actor's spot, 400 high.
    fn f_100093f0(&mut self, this: u32) {
        let (x, z) = (self.r(this, 40), self.r(this, 48));
        self.throw_at_target(x, 400, z, 500, 300);
    }

    /// `sub_100094D0`: relays the ball from where it is.
    fn f_100094d0(&mut self, this: u32) {
        let ball = self.p(this + 16);
        let (x, y, z) = (self.i(ball + 24), self.i(ball + 28), self.i(ball + 32));
        self.throw_at_target(x, y, z, 700, 300);
    }

    /// `sub_10008A80` (slot 8).
    pub(crate) fn f_10008a80(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => {
                if self.r(this, 112) == 0 {
                    self.f_100050c0(this, 0, 100, 0, false);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            2 => {
                let table = 0x1002_2C6C;
                self.prefix_table(table, 0x1001_E5DC, 0x1001_E638);
                let pattern = self.table_seek(this, table, 0x1001_E580, 0x1001_E638);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == 48 {
                    self.hold_ball(this);
                    self.f_10014040(15, 255);
                    self.f_10014080(20);
                }
                if self.r(this, 112) == self.i(0x1002_2C8C) {
                    self.f_100093f0(this);
                    self.f_10014040(6, 255);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_E638) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            // Diving catches: frames 0 (sound), 5 (land), 10 (catch),
            // 15 (throw) and 25 (back home) of the table.
            state @ (3 | 4) => {
                let (table, source, count, patterns, se, line) = if state == 3 {
                    (0x1002_2D3C, 0x1001_E79C, 0x1001_E810, 0x1001_E728, 22, 21)
                } else {
                    (0x1002_2CC8, 0x1001_E6B0, 0x1001_E724, 0x1001_E63C, 21, 20)
                };
                self.prefix_table(table, source, count);
                let pattern = self.table_seek(this, table, patterns, count);
                self.f_10005090(this, pattern, 100);
                let key = |pt: &Self, n: u32| pt.i(table + 4 * n);
                if self.r(this, 112) == key(self, 0) {
                    self.f_10014040(se, 255);
                }
                if self.r(this, 112) == key(self, 5) {
                    self.go_to_target(this);
                    let number = self.w(this, 312);
                    self.f_10004400(number + 10, 50, false);
                }
                if self.r(this, 112) == key(self, 10) {
                    self.hold_ball(this);
                    self.f_10014040(15, 255);
                    self.f_10014080(line);
                }
                if self.r(this, 112) == key(self, 15) {
                    self.f_100093f0(this);
                    self.f_10014040(6, 255);
                }
                if self.r(this, 112) == key(self, 25) {
                    self.go_home(this);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, count) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            5 => {
                let table = 0x1002_2DB0;
                self.prefix_n(table, 0x1001_E83C, 10);
                let pattern = self.seek_n(this, table, 0x1001_E814, 10);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.i(0x1002_2DD4) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            6 => {
                let table = 0x1002_2DD8;
                self.prefix_n(table, 0x1001_E88C, 10);
                let pattern = self.seek_n(this, table, 0x1001_E864, 10);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == self.i(0x1002_2DE8) {
                    self.f_100094d0(this);
                    self.f_10014040(12, 255);
                    self.f_10014080(22);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.i(0x1002_2DFC) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            _ => {}
        }
    }
}
