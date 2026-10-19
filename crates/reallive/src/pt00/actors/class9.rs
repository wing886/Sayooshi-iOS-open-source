//! Actor 9 (vtable `0x1001C140`).

use super::super::base::{direction8, distance, normalize, segment_foot, side};
use super::super::{BALL, Pt00, ftol};

/// Glove offsets by facing for actor 9's dives.
const GLOVE: [(i32, i32); 8] = [
    (0, 320),
    (320, 160),
    (520, 0),
    (440, -80),
    (80, -200),
    (-270, -140),
    (-420, -80),
    (-360, 200),
];

impl Pt00 {
    /// `sub_1000FFC0` (slot 7).
    pub(crate) fn f_1000ffc0(&mut self, this: u32) {
        match self.r(this, 24) {
            0 => {
                if self.at_home(this) {
                    let state = if self.rand() % 2 == 0 { 1 } else { 2 };
                    self.vcall(this, 20, state, 0);
                } else {
                    self.vcall(this, 20, 4, 0);
                }
            }
            1 => {
                if !self.f_10010160(this) && !self.f_10010580(this) && !self.f_10010880(this) {
                    self.chase_landing(this);
                    self.vcall(this, 20, 3, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_10010160`: catch at the ball, or reaching to either side.
    fn f_10010160(&mut self, this: u32) -> bool {
        let ball = self.p(this + 16);
        let side_of = side(
            f64::from(self.i(ball + 12)),
            f64::from(self.i(ball + 20)),
            f64::from(self.i(ball + 48)),
            f64::from(self.i(ball + 56)),
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        );
        let mut found = false;
        let mut best = 99999999.0;
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let mut frame = 3;
        let mut reach = 80;
        let mut p = BALL;
        while p < 0x1002_299C {
            let v = |pt: &Self, n: u32| pt.i(p + 4 * n);
            let (px, py) = (f64::from(self.r(this, 40)), f64::from(self.r(this, 48)));
            let foot = |pt: &Self, dx: i32| {
                segment_foot(
                    f64::from(v(pt, 0) + dx),
                    f64::from(v(pt, 2)),
                    f64::from(v(pt, 9) + dx),
                    f64::from(v(pt, 11)),
                    px,
                    py,
                )
                .distance
            };
            let reach_f = f64::from(reach);
            let d = foot(self, 0);
            let height = f64::from(v(self, 10) - self.r(this, 44));
            let mut take = |pt: &mut Self, state: i32, dx: i32, d: f64, best: &mut f64| {
                *best = d;
                pt.vcall(this, 20, state, 0);
                pt.set_target(this, [v(pt, 9) + dx, 0, v(pt, 11)]);
                pt.sr(this, 120, frame);
                pt.f_10005460(this);
                found = true;
            };
            if d >= reach_f || height >= 300.0 {
                if d < reach_f && height < 600.0 && d < best {
                    take(self, 6, 0, d, &mut best);
                }
            } else if d < best {
                take(self, 5, 0, d, &mut best);
            }
            let low = f64::from(v(self, 10) - self.r(this, 44)) < 150.0;
            let d = foot(self, -230);
            if side_of == -1 && d < reach_f && low && d < best {
                take(self, 7, -230, d, &mut best);
            }
            let d = foot(self, 230);
            if side_of == 1 && d < reach_f && low && d < best {
                take(self, 8, 230, d, &mut best);
            }
            p += 36;
            frame += 3;
            reach += 60;
        }
        found
    }

    /// `sub_10010580`: dive for a ground ball.
    fn f_10010580(&mut self, this: u32) -> bool {
        let mut found = false;
        let mut best = 99999999.0;
        let mut frame = 3;
        let mut reach = 120;
        let (mut ox, mut oz) = (0, 0);
        let mut p = BALL;
        loop {
            let v = |pt: &Self, n: u32| pt.i(p + 4 * n);
            let facing = direction8(
                f64::from(v(self, 9) - self.r(this, 40)),
                f64::from(v(self, 11) - self.r(this, 48)),
                self.r(this, 52),
            );
            self.sr(this, 52, facing);
            if let Some(&(x, z)) = GLOVE.get(facing as usize) {
                (ox, oz) = (x, z);
            }
            let foot = segment_foot(
                f64::from(v(self, 0) - ox),
                f64::from(v(self, 2) - oz),
                f64::from(v(self, 9) - ox),
                f64::from(v(self, 11) - oz),
                f64::from(self.r(this, 40)),
                f64::from(self.r(this, 48)),
            );
            let ball = self.p(this + 16);
            if f64::from(reach) > foot.distance
                && f64::from(v(self, 10) - self.r(this, 44)) < 100.0
                && self.i(ball + 60) > 100
                && self.f_10004310(v(self, 9), v(self, 11), None) == 0
                && foot.distance < best
                && (foot.y - f64::from(self.r(this, 48))) * f64::from(self.i(ball + 56))
                    + (foot.x - f64::from(self.r(this, 40))) * f64::from(self.i(ball + 48))
                    >= 0.0
            {
                best = foot.distance;
                self.vcall(this, 20, 9, 0);
                self.sr(this, 56, self.r(this, 40));
                self.sr(this, 60, self.r(this, 44));
                self.sr(this, 64, self.r(this, 48));
                self.set_target(this, [v(self, 9) - ox, v(self, 10), v(self, 11) - oz]);
                self.sr(this, 120, frame);
                self.f_10005460(this);
                found = true;
            }
            frame += 3;
            reach += 90;
            p += 36;
            if p >= 0x1002_299C {
                break;
            }
        }
        found
    }

    /// `sub_10010880`: a long flying dive towards the ball.
    fn f_10010880(&mut self, this: u32) -> bool {
        let mut found = false;
        let mut best = 99999999.0;
        let mut frame = 3;
        let mut length = 120;
        let (mut ox, mut oz) = (0, 0);
        let mut v2 = BALL + 44;
        loop {
            let at = |pt: &Self, n: i32| pt.i((v2 as i32 + 4 * n) as u32);
            let facing = direction8(
                f64::from(at(self, -2) - self.r(this, 40)),
                f64::from(at(self, 0) - self.r(this, 48)),
                self.r(this, 52),
            );
            self.sr(this, 52, facing);
            if let Some(&(x, z)) = GLOVE.get(facing as usize) {
                (ox, oz) = (x, z);
            }
            let foot = segment_foot(
                f64::from(at(self, -11) - ox),
                f64::from(at(self, -9) - oz),
                f64::from(at(self, -2) - ox),
                f64::from(at(self, 0) - oz),
                f64::from(self.r(this, 40)),
                f64::from(self.r(this, 48)),
            );
            let ball = self.p(this + 16);
            if f64::from(length + 270) > foot.distance
                && f64::from(at(self, -1) - self.r(this, 44)) < 100.0
                && self.i(ball + 60) > 100
                && self.f_10004310(at(self, -2), at(self, 0), None) == 0
                && foot.distance < best
                && (foot.y - f64::from(self.r(this, 48))) * f64::from(self.i(ball + 56))
                    + (foot.x - f64::from(self.r(this, 40))) * f64::from(self.i(ball + 48))
                    >= 0.0
            {
                best = foot.distance;
                self.vcall(this, 20, 10, 0);
                self.sr(this, 56, self.r(this, 40));
                self.sr(this, 60, self.r(this, 44));
                self.sr(this, 64, self.r(this, 48));
                self.sr(this, 120, frame - 3);
                let (mut dx, mut dz) = (foot.dx, foot.dy);
                normalize(&mut dx, &mut dz);
                let reach = f64::from(length);
                let target = [
                    ftol(reach * dx) + self.r(this, 40),
                    self.r(this, 44),
                    ftol(reach * dz) + self.r(this, 48),
                ];
                self.set_target(this, target);
                found = true;
            }
            v2 += 36;
            frame += 3;
            length += 90;
            if length >= 3030 {
                break;
            }
        }
        found
    }

    /// `sub_1000FEE0` (slot 8).
    pub(crate) fn f_1000fee0(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => {
                if self.r(this, 112) == 0 {
                    let wait = self.rand() % 200;
                    self.sr(this, 112, self.r(this, 112) + wait);
                }
                self.f_10005090(this, 0, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) >= 300 {
                    self.vcall(this, 16, 0, 0);
                }
            }
            2 => {
                let table = 0x1002_32F0;
                self.prefix_table(table, 0x1001_F398, 0x1001_F3BC);
                let pattern = self.table_seek(this, table, 0x1001_F374, 0x1001_F3BC);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_F3BC) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            3 => {
                self.walk_quick(this);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            4 => {
                let home = [self.r(this, 28), self.r(this, 32), self.r(this, 36)];
                self.set_target(this, home);
                self.walk_quick(this);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.at_target(this) {
                    self.vcall(this, 20, 2, 0);
                }
            }
            5 => self.catch_frames(this, [68, 68, 69]),
            6 => self.catch_frames(this, [70, 70, 71]),
            7 => self.catch_frames(this, [74, 75, 73]),
            8 => self.catch_frames(this, [82, 83, 81]),
            9 => self.f_10011210(this),
            10 => self.f_10011700(this),
            11 => self.f_100110c0(this),
            _ => {}
        }
        self.sr(this, 136, 0);
        let pattern = self.r(this, 80);
        for (n, top) in (87..119).step_by(4).enumerate() {
            if top - 2 <= pattern && pattern <= top {
                self.sr(this, 136, n as i32 + 1);
            }
        }
    }

    fn walk_quick(&mut self, this: u32) {
        let first = i32::from(self.r(this, 112) == 0);
        self.f_10005310(this, 0, 0, first);
        self.f_10005100(this, 20.0);
    }

    /// States 5-8: run to the target and catch at `record[120]`.
    /// `patterns` are (caught, settled 24 frames later, waiting).
    fn catch_frames(&mut self, this: u32, patterns: [i32; 3]) {
        let (t, at) = (self.r(this, 112), self.r(this, 120));
        if t >= at {
            let pattern = if t >= at + 24 {
                patterns[1]
            } else {
                patterns[0]
            };
            self.f_10005090(this, pattern, 100);
        } else if self.at_target(this) {
            self.f_10005090(this, patterns[2], 100);
        } else {
            self.walk_quick(this);
        }
        if self.r(this, 112) == self.r(this, 120) {
            self.hold_ball(this);
            self.f_10014040(15, 255);
            self.f_10014080(90);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) == self.r(this, 120) + 10 {
            self.vcall(this, 20, 11, 0);
        }
    }

    /// The flight of a dive: returns (landing start, catch frame, end of
    /// flight) and moves the actor. `patterns` are (run-in, rising,
    /// sliding, after) pattern bases (plus four per facing).
    fn dive_flight(&mut self, this: u32, patterns: [u32; 4]) -> (i32, i32, i32) {
        let (sx, sy, sz) = (self.r(this, 56), self.r(this, 60), self.r(this, 64));
        let (tx, ty, tz) = (self.r(this, 68), self.r(this, 72), self.r(this, 76));
        let (dx, dy, dz) = (tx - sx, ty - sy, tz - sz);
        let capped = ty.min(300);
        let at = self.r(this, 120).max(1);
        let atf = f64::from(at);
        let takeoff = ftol(atf - (f64::from(ty - capped) * 1.666666666666667).sqrt());
        let top = f64::from(ty);
        let land = ftol((top * 1.666666666666667).sqrt() + atf);
        let t = self.r(this, 112);
        let facing = self.r(this, 52);
        let pattern = |pt: &Self, base: u32| pt.i(base) + 4 * facing;
        let height = |t: i32| ftol(top - f64::from(t - at) * f64::from(t - at) * 0.6).max(0);
        if t >= takeoff {
            if t < at {
                self.sr(this, 40, t * dx / at + sx);
                self.sr(this, 44, sy + t * dy / at);
                self.sr(this, 48, sz + t * dz / at);
                self.sr(this, 44, height(t));
                let p = pattern(self, patterns[0]);
                self.f_10005090(this, p, 100);
            } else if t < land {
                self.sr(this, 40, 5 * dx * (t - at) / (2 * at) + tx);
                self.sr(this, 44, 5 * (t - at) * dy / (2 * at) + ty);
                self.sr(this, 48, tz + 5 * (t - at) * dz / (2 * at));
                self.sr(this, 44, height(t));
                let p = pattern(self, patterns[1]);
                self.f_10005090(this, p, 100);
            } else if t < land + 15 {
                self.sr(this, 40, tx + 5 * dx * (land - at) / (2 * at));
                self.sr(this, 44, 5 * (land - at) * dy / (2 * at) + ty);
                self.sr(this, 48, tz + 5 * (land - at) * dz / (2 * at));
                let k = 225 - (t - land - 15) * (t - land - 15);
                let (ddx, ddy, ddz) = (
                    k * (self.r(this, 68) - self.r(this, 56)) / at / 15,
                    k * (self.r(this, 72) - self.r(this, 60)) / at / 15,
                    k * (self.r(this, 76) - self.r(this, 64)) / at / 15,
                );
                self.sr(this, 40, self.r(this, 40) + ddx / 2);
                self.sr(this, 44, ddy / 2 + self.r(this, 44));
                self.sr(this, 48, ddz / 2 + self.r(this, 48));
                self.sr(this, 44, 0);
                let p = pattern(self, patterns[2]);
                self.f_10005090(this, p, 100);
            } else if t < land + 19 {
                let p = pattern(self, patterns[3]);
                self.f_10005090(this, p, 100);
            }
        } else {
            if t == 0 {
                self.f_10005310(this, 0, 0, 1);
            }
            self.sr(this, 40, sx + t * dx / at);
            self.sr(this, 44, 0);
            self.sr(this, 48, sz + t * dz / at);
        }
        (takeoff, at, land)
    }

    /// `sub_10011210` (state 9): the diving catch.
    fn f_10011210(&mut self, this: u32) {
        let (x0, z0) = (self.r(this, 40), self.r(this, 48));
        if self.r(this, 112) == self.r(this, 120) - 50 {
            let number = self.w(this, 312);
            self.f_10004400(number + 10, 50, false);
        }
        self.prefix_table(0x1002_3328, 0x1001_F400, 0x1001_F414);
        let (takeoff, at, land) =
            self.dive_flight(this, [0x1001_F3F0, 0x1001_F3F4, 0x1001_F3F8, 0x1001_F3EC]);
        let t = self.r(this, 112);
        if t == takeoff {
            self.f_10014080(91);
        }
        if t == at {
            self.hold_ball(this);
            self.f_10014040(15, 255);
            self.f_10014080(91);
        }
        if t == land {
            self.f_10014040(24, 255);
            self.f_100055b0(this);
        }
        if self.f_10004310(self.r(this, 40), self.r(this, 48), None) > 0 {
            self.sr(this, 40, x0);
            self.sr(this, 48, z0);
        }
        self.sr(this, 112, t + 1);
        if t + 1 == land + 19 {
            self.vcall(this, 20, 11, 0);
        }
    }

    /// `sub_10011700` (state 10): the long dive, which ends the play.
    fn f_10011700(&mut self, this: u32) {
        self.sr(this, 24, 2);
        self.prefix_table(0x1002_333C, 0x1001_F42C, 0x1001_F440);
        let (takeoff, _, land) =
            self.dive_flight(this, [0x1001_F41C, 0x1001_F420, 0x1001_F424, 0x1001_F418]);
        let t = self.r(this, 112);
        if t == land {
            self.f_10014040(24, 255);
            self.f_100055b0(this);
        }
        let ball = self.p(this + 16);
        if t < takeoff && self.i(ball + 120) == 1 {
            self.vcall(this, 16, 0, 0);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) == land + 19 {
            self.vcall(this, 16, 0, 0);
        }
    }

    /// Whether actor 8 can take a relay.
    fn relay_blocked(&self, this: u32) -> bool {
        let partner = self.actor(8);
        self.r(partner, 0) != 1 || self.r(partner, 12) != 0 || self.g(this, 360) != 0
    }

    /// `sub_100110C0` (state 11): the throw.
    fn f_100110c0(&mut self, this: u32) {
        let table = 0x1002_3314;
        self.prefix_table(table, 0x1001_F3D4, 0x1001_F3E8);
        let (tx, tz) = if self.relay_blocked(this) {
            (self.i(0x1002_29F8), self.i(0x1002_2A00))
        } else {
            let partner = self.actor(8);
            (self.r(partner, 40), self.r(partner, 48))
        };
        let facing = direction8(
            f64::from(tx - self.r(this, 40)),
            f64::from(tz - self.r(this, 48)),
            self.r(this, 52),
        );
        self.sr(this, 52, facing);
        self.table_seek(this, table, 0x1001_F3C0, 0x1001_F3E8);
        let pattern = self.i(0x1001_F3C0 + 4 * self.r(this, 108) as u32) + 4 * self.r(this, 52);
        self.f_10005090(this, pattern, 100);
        if self.r(this, 112) == 24 {
            self.f_10011ba0(this);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) == self.table_total(table, 0x1001_F3E8) {
            self.vcall(this, 16, 0, 0);
        }
    }

    /// `sub_10011BA0`: throws to the target, or relays to actor 8.
    fn f_10011ba0(&mut self, this: u32) {
        let (x, y, z) = (self.r(this, 40), self.r(this, 44) + 200, self.r(this, 48));
        if self.relay_blocked(this) {
            self.throw_at_target(x, y, z, 500, 300);
            return;
        }
        let partner = self.actor(8);
        let (px, pz) = (self.r(partner, 40), self.r(partner, 48));
        let time =
            (distance(f64::from(x), f64::from(z), f64::from(px), f64::from(pz)) - 240.0) * 0.00625;
        let (dx, dz) = (px - x, pz - z);
        let length = f64::from(dx * dx + dz * dz).sqrt();
        let (ux, uy, uz) = if length == 0.0 {
            (0, 0, 0)
        } else {
            (
                ftol(f64::from(dx) * 1000.0 / length),
                ftol(0.0 / length),
                ftol(f64::from(dz) * 1000.0 / length),
            )
        };
        self.f_10002bd0(x, y, z, ux, uy, uz, 1600, ftol(time * 6.0));
        self.f_10004400(1, 50, false);
        self.vcall(partner, 20, 9, 0);
        self.f_10005460(partner);
        self.sr(partner, 120, ftol(time));
    }
}
