//! Actor 0: the pitcher (vtable `0x1001C260`).

use super::super::base::{distance, normalize, segment_foot};
use super::super::{BALL, Pt00, ftol};

/// End of the ball prediction scanned by the catch checks.
const PREDICTION_END: u32 = 0x1002_299C;

impl Pt00 {
    /// `sub_10005950` (slot 7): when the play starts, try to catch.
    pub(crate) fn f_10005950(&mut self, this: u32) {
        match self.r(this, 24) {
            0 => {
                self.vcall(this, 20, 1, 0);
            }
            1 => {
                if !self.f_10005990(this) && !self.f_10005b00(this) {
                    self.vcall(this, 20, 1, 0);
                }
            }
            _ => {}
        }
    }

    /// Distance from the actor to the predicted segment at `point`.
    fn prediction_distance(&self, this: u32, point: u32) -> f64 {
        let v = |n: u32| f64::from(self.i(point + 4 * n));
        segment_foot(
            v(0),
            v(2),
            v(9),
            v(11),
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        )
        .distance
    }

    fn ball_predicted(&self) -> bool {
        self.i(BALL) != 0 || self.i(BALL + 4) != 0 || self.i(BALL + 8) != 0
    }

    /// `sub_10005990`: catch the ball on the fly (low or high).
    fn f_10005990(&mut self, this: u32) -> bool {
        let mut caught = false;
        let mut best = 99999999.0;
        if !self.ball_predicted() {
            return false;
        }
        let mut frame = 3;
        let mut point = BALL;
        while point < PREDICTION_END {
            let d = self.prediction_distance(this, point);
            let height = f64::from(self.i(point + 40) - self.r(this, 44));
            if d < best && d < 80.0 {
                let state = if height < 300.0 {
                    Some(5)
                } else if height < 600.0 {
                    Some(6)
                } else {
                    None
                };
                if let Some(state) = state {
                    best = d;
                    self.vcall(this, 20, state, 0);
                    self.sr(this, 120, frame);
                    self.f_10005460(this);
                    caught = true;
                }
            }
            point += 36;
            frame += 3;
        }
        caught
    }

    /// `sub_10005B00`: field a ground ball to the side.
    fn f_10005b00(&mut self, this: u32) -> bool {
        let mut moved = false;
        let mut best = 99999999.0;
        if !self.ball_predicted() {
            return false;
        }
        let mut frame = 14;
        let mut point = BALL + 132;
        while point < PREDICTION_END {
            let d = self.prediction_distance(this, point);
            if d < best && d < 320.0 && f64::from(self.i(point + 40) - self.r(this, 44)) < 600.0 {
                best = d;
                let state = if self.i(point + 36) <= self.r(this, 40) {
                    9
                } else {
                    8
                };
                self.f_10005570(this, state, frame - 14);
                moved = true;
                self.sr(this, 120, frame);
                self.sr(this, 20, 1);
            }
            point += 36;
            frame += 3;
        }
        moved
    }

    /// `sub_100058B0` (slot 8): the pitcher's states.
    pub(crate) fn f_100058b0(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => self.f_10005090(this, 0, 100),
            2 => self.f_10005c50(this),
            // Waiting on the mound, swaying.
            3 => {
                self.f_10005090(this, 0, 100);
                let x = self.r(this, 40);
                let x = if self.r(this, 52) == 2 { x + 3 } else { x - 3 };
                self.sr(this, 40, x);
                let x = self.r(this, 40);
                if x > 50 {
                    self.sr(this, 40, 100 - x);
                    self.sr(this, 52, 6);
                } else if x < -50 {
                    self.sr(this, 40, -100 - x);
                    self.sr(this, 52, 2);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) >= self.r(this, 120) {
                    self.vcall(this, 20, 11, 0);
                }
            }
            4 => {
                self.prefix_table(0x1002_2A2C, 0x1001_E0EC, 0x1001_E128);
                let pattern = self.table_step(this, 0x1002_2A2C, 0x1001_E0B0);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.table_total(0x1002_2A2C, 0x1001_E128) <= self.r(this, 112) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            // Catching low (5) or high (6).
            state @ (5 | 6) => {
                let (held, reaching, line) = if state == 5 { (17, 18, 1) } else { (19, 20, 2) };
                let pattern = if self.r(this, 112) >= self.r(this, 120) {
                    held
                } else {
                    reaching
                };
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == self.r(this, 120) {
                    self.f_10005480(this);
                    self.f_10014040(15, 255);
                    self.f_10014080(line);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.r(this, 120) + 30 {
                    if self.g(this, 304) == 7 {
                        self.f_10005590(this, 30);
                    } else {
                        self.vcall(this, 20, 10, 0);
                    }
                }
            }
            7 => {
                let table = 0x1002_2A68;
                self.prefix_table(table, 0x1001_E148, 0x1001_E164);
                if self.r(this, 112) == 0 {
                    let number = self.w(this, 312);
                    self.f_10004400(number + 10, 50, false);
                }
                let pattern = self.table_seek(this, table, 0x1001_E12C, 0x1001_E164);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == self.i(table + 8) {
                    self.sr(this, 128, 5);
                    self.sr(this, 132, 1);
                }
                if self.r(this, 112) == self.i(table + 4) {
                    self.f_10014080(4);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_E164) {
                    self.f_10005590(this, 30);
                }
            }
            // Diving to either side.
            state @ (8 | 9) => {
                self.sr(this, 24, 2);
                let (table, source, count, patterns) = if state == 8 {
                    (0x1002_2A84, 0x1001_E17C, 0x1001_E190, 0x1001_E168)
                } else {
                    (0x1002_2A98, 0x1001_E1A8, 0x1001_E1BC, 0x1001_E194)
                };
                self.prefix_table(table, source, count);
                let pattern = self.table_seek(this, table, patterns, count);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == 0 {
                    self.f_10014080(3);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, count) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            // Throwing the ball on.
            10 => {
                let table = 0x1002_2AAC;
                self.prefix_table(table, 0x1001_E1D4, 0x1001_E1E8);
                let pattern = self.table_seek(this, table, 0x1001_E1C0, 0x1001_E1E8);
                self.f_10005090(this, pattern, 100);
                if self.r(this, 112) == self.i(table + 4) {
                    self.f_10006f00(this);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_E1E8) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            11 => {
                let table = 0x1002_2A04;
                self.prefix_table(table, 0x1001_E054, 0x1001_E058);
                let pattern = self.table_step(this, table, 0x1001_E050);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.table_total(table, 0x1001_E058) <= self.r(this, 112) {
                    let next = if self.g(this, 364) == 1 || self.g(this, 320) == 1 {
                        12
                    } else {
                        let kind = self.g(this, 2000);
                        if !(7..=8).contains(&kind) { 13 } else { 14 }
                    };
                    self.vcall(this, 20, next, 0);
                }
            }
            12 => {
                let table = 0x1002_2A08;
                self.prefix_table(table, 0x1001_E060, 0x1001_E064);
                let index = self.r(this, 108);
                if self.i(table + 4 * index as u32) <= self.r(this, 112) {
                    self.sr(this, 108, index + 1);
                }
                if self.r(this, 112) == 0 {
                    self.f_10014040(31, 255);
                    let mode = if self.g(this, 304) == 4 { 8 } else { 3 };
                    self.sr(this, 128, mode);
                    self.sr(this, 132, 1);
                }
                let pattern = self.i(0x1001_E05C + 4 * self.r(this, 108) as u32);
                self.f_10005090(this, pattern, 100);
                if self.g(this, 304) == 4 && self.r(this, 112) == 50 {
                    self.f_100140a0(127);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.g(this, 304) == 4 && self.r(this, 112) == 51 {
                    self.vcall(this, 20, 13, 0);
                }
                if self.table_total(table, 0x1001_E064) <= self.r(this, 112) {
                    self.vcall(this, 20, 13, 0);
                }
            }
            13 => self.f_100061f0(this),
            14 => {
                let table = 0x1002_2A0C;
                self.prefix_table(table, 0x1001_E06C, 0x1001_E070);
                let pattern = self.table_step(this, table, 0x1001_E068);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.table_total(table, 0x1001_E070) <= self.r(this, 112) {
                    self.vcall(this, 20, 13, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_10005C50` (state 2): chooses the next pitch (`F[500]`).
    fn f_10005c50(&mut self, this: u32) {
        self.f_10005090(this, 0, 100);
        self.sr(this, 40, self.r(this, 40).clamp(-50, 50));
        self.sr(this, 44, 0);
        self.sr(this, 48, self.r(this, 36));
        let wait = self.rand() % 20 + 10;
        self.sr(this, 120, wait);
        let facing = if self.rand() % 2 != 0 { 6 } else { 2 };
        self.sr(this, 52, facing);
        // Straight balls three times over, plus every pitch unlocked.
        let mut pitches = vec![0, 0, 0];
        pitches.extend((0..9).filter(|&n| self.g(this, 4 * n as u32 + 2080) == 1));
        let pick = pitches[(self.rand() as usize) % pitches.len()];
        self.sg(this, 2000, pick);
        let mode = self.g(this, 304);
        if (1..=3).contains(&mode) {
            self.sr(this, 120, 10);
            self.sr(this, 52, 2);
            self.sg(this, 2000, 0);
        }
        if mode == 4 || mode == 5 {
            self.sg(this, 2000, 7);
        }
        if self.g(this, 304) == 6 {
            self.sg(this, 2000, 1);
            self.f_10014080(130);
        }
        if self.g(this, 364) == 1 {
            self.sg(this, 2000, self.g(this, 368));
        }
        match self.g(this, 2000) {
            2..=6 => self.sg(this, 3224, self.g(this, 3224) + 1),
            7 | 8 => self.sg(this, 3224, self.g(this, 3224) + 2),
            _ => {}
        }
        self.vcall(this, 20, 3, 0);
    }

    /// `sub_100061F0` (state 13): the delivery; the ball leaves the hand
    /// at frame 2 of the animation.
    fn f_100061f0(&mut self, this: u32) {
        let table = 0x1002_2A10;
        self.prefix_table(table, 0x1001_E090, 0x1001_E0AC);
        let pattern = self.table_step(this, table, 0x1001_E074);
        self.f_10005090(this, pattern, 100);
        let release = self.i(table + 8);
        if self.r(this, 112) == release {
            let aim = self.f_10006390(this);
            let speed = self.f_10006470(this);
            let lift = f_10006500(f64::from(speed));
            self.f_10006530(this, f64::from(aim), -2000.0, speed, lift);
            let se = match self.g(this, 2000) {
                7 => 8,
                8 => 9,
                _ => 6,
            };
            self.f_10014040(se, 255);
            if self.g(this, 304) == 6 {
                self.f_10014040(7, 255);
            }
        }
        if self.r(this, 112) == release + 15 {
            let mode = self.g(this, 304);
            if mode == 5 || mode == 6 {
                self.sg(this, 2400, 3);
                self.sg(this, 2520, 0);
            }
        }
        let total = self.table_total(table, 0x1001_E0AC);
        if self.r(this, 112) == total - 1 && self.g(this, 364) == 1 {
            self.f_10014080(66);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if total <= self.r(this, 112) {
            self.vcall(this, 16, 0, 0);
        }
    }

    /// `sub_10006390`: where across the plate the pitch goes (`F[715]`:
    /// 1 strike zone, 2 outside).
    fn f_10006390(&mut self, this: u32) -> i32 {
        let mut spread = self.rand() % 150;
        if self.g(this, 312) == 1 {
            spread = self.rand() % 100 + 50;
        }
        let kind = self.g(this, 2000);
        if kind == 7 || kind == 8 {
            spread = 0;
        }
        let v5 = spread * (130 - self.g(this, 3616));
        let v6 = if v5 / 100 >= 0 {
            (v5 / 100).min(125)
        } else {
            0
        };
        let side = if self.rand() % 2 != 0 { -1 } else { 1 };
        let mode = self.g(this, 304);
        let mut offset = v6 * side;
        if mode == 4 || mode == 5 {
            offset = 0;
        }
        let zone = if mode == 6 {
            offset = 150;
            2
        } else if offset <= -50 || offset >= 50 {
            2
        } else {
            1
        };
        self.sg(this, 2860, zone);
        offset - self.r(this, 40)
    }

    /// `sub_10006470`: the pitch speed.
    fn f_10006470(&mut self, this: u32) -> i32 {
        let mut speed = ftol((f64::from(self.rand()) * 0.000003051850947599719 + 0.9) * 400.0);
        match self.g(this, 2000) {
            7 => speed = 3 * speed / 2,
            8 => speed *= 2,
            1 | 6 => speed = 2 * speed / 3,
            _ => {}
        }
        match self.g(this, 304) {
            4 | 5 => speed = 1200,
            6 => speed = 260,
            _ => {}
        }
        speed
    }

    /// `sub_10006530`: releases the pitch (record `F[220]`) towards
    /// (dx, dz) at `speed` with `lift`.
    fn f_10006530(&mut self, this: u32, dx: f64, dz: f64, speed: i32, lift: i32) {
        let (mut a2, mut a3) = (dx, dz);
        normalize(&mut a2, &mut a3);
        self.sg(this, 840, 1);
        self.sg(this, 880, 1);
        self.sg(this, 884, 255);
        self.sg(this, 888, 1);
        self.sg(this, 892, self.r(this, 40));
        self.sg(this, 896, self.r(this, 44) + 500);
        self.sg(this, 900, self.r(this, 48));
        self.sg(this, 904, self.g(this, 892));
        self.sg(this, 908, self.g(this, 896));
        self.sg(this, 912, self.g(this, 900));
        self.sg(this, 928, ftol(a2 * 1000.0));
        self.sg(this, 932, 0);
        self.sg(this, 936, ftol(a3 * 1000.0));
        self.sg(this, 940, speed);
        self.sg(this, 916, self.g(this, 904));
        self.sg(this, 920, self.g(this, 908));
        self.sg(this, 924, self.g(this, 912));
        for (i, from) in [904, 908, 912, 904, 908, 912].into_iter().enumerate() {
            self.sg(this, 944 + 4 * i as u32, 100 * self.g(this, from));
        }
        self.sg(this, 968, self.g(this, 896));
        self.sg(this, 972, lift);
        self.sg(this, 976, 0);
        self.sg(this, 980, 0);
        self.sg(this, 2004, self.g(this, 940) / 5);
        let mode = self.g(this, 304);
        // Practice modes 1 and 3: the ball goes straight to the catcher.
        if mode == 1 || mode == 3 {
            self.throw_to_catcher(this);
        }
        if mode == 2 {
            let length = 4032400.0f64.sqrt();
            let (x, z) = if length == 0.0 {
                (0, 0)
            } else {
                (ftol(-180000.0 / length), ftol(-2000000.0 / length))
            };
            self.sg(this, 928, x);
            self.sg(this, 932, 0);
            self.sg(this, 936, z);
            self.sg(this, 940, 600);
            self.sg(this, 972, 0);
        }
    }

    /// Part of `sub_10006530`: a throw from the pitch position to actor 7.
    fn throw_to_catcher(&mut self, this: u32) {
        let catcher = self.actor(7);
        let (x, y, z) = (self.g(this, 892), self.g(this, 896), self.g(this, 900));
        let dx = self.r(catcher, 40) - self.r(this, 40);
        let dy = self.r(catcher, 44) - self.r(this, 44);
        let dz = self.r(catcher, 48) - self.r(this, 48);
        let length = f64::from(dx * dx + dz * dz + dy * dy).sqrt();
        let (ux, uy, uz) = if length == 0.0 {
            (0, 0, 0)
        } else {
            (
                ftol(f64::from(dx) * 1000.0 / length),
                ftol(f64::from(dy) * 1000.0 / length),
                ftol(f64::from(dz) * 1000.0 / length),
            )
        };
        self.f_10002bd0(x, y, z, ux, uy, uz, 600, 250);
        self.sg(this, 840, 2);
        self.sg(this, 1008, 1);
    }

    /// `sub_10006F00`: throws the ball on to the target; the lift grows
    /// with the distance.
    fn f_10006f00(&mut self, this: u32) {
        let (x, y, z) = (self.r(this, 40), self.r(this, 44) + 200, self.r(this, 48));
        let (tx, tz) = (self.i(0x1002_29F8), self.i(0x1002_2A00));
        let lift = ftol(
            (distance(f64::from(x), f64::from(z), f64::from(tx), f64::from(tz)) - 240.0)
                * 0.03333333333333333
                * 6.0,
        );
        self.throw_at_target(x, y, z, 300, lift);
    }
}

/// `sub_10006500`: the lift for a pitch speed.
fn f_10006500(speed: f64) -> i32 {
    let t = 2000.0 / (speed * 0.1);
    ftol(t * 0.3 - 250.0 / t)
}
