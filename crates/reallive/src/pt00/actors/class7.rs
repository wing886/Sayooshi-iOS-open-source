//! Actor 7 (vtable `0x1001C188`), and what it shares with actor 8: the
//! two run laps round the field and throw to each other.

use super::super::base::{distance, segment_foot};
use super::super::{BALL, Pt00, ftol};

impl Pt00 {
    /// `sub_1000CBC0` (slot 2 of classes 7 and 8).
    pub(crate) fn f_1000cbc0(&mut self, this: u32) {
        self.f_10004ab0(this);
        self.sb(this + 1252, 0);
        self.sb(this + 1253, 0);
    }

    /// Pattern set for the actor's cap (bytes 1252/1253 of the object).
    fn cap_set(&self, this: u32) -> i32 {
        (if self.b(this + 1252) != 0 { 0x20 } else { 0 })
            + if self.b(this + 1253) != 0 { 0x40 } else { 0 }
    }

    /// `sub_1000E3E0`: walks to the target.
    pub(crate) fn f_1000e3e0(&mut self, this: u32, step: f64, first: i32) {
        let set = self.cap_set(this);
        self.f_10005310(this, 0, set, first);
        self.f_10005100(this, step);
    }

    /// `sub_1000E430` / `sub_1000FC80`: follows the lap circle exactly.
    pub(crate) fn f_1000e430(&mut self, this: u32, first: bool) {
        let set = self.cap_set(this);
        let facing = if self.r(this, 12) == 4 {
            self.f_100056f0(this, 0)
        } else {
            self.f_10005800(this, 0)
        };
        self.sr(this, 52, facing);
        let a2 = i32::from(first);
        let (mirror, pattern) = match facing {
            d @ 0..=7 => (100, set + 4 * d),
            _ => (a2, a2),
        };
        if first {
            self.f_100050c0(this, 0, mirror, pattern, false);
        } else if mirror != self.r(this, 96) || pattern != self.r(this, 100) {
            self.f_100050c0(this, 0, mirror, pattern, true);
        }
        self.go_to_target(this);
    }

    /// `sub_1000E5D0`: relays the ball from where it is to the target.
    fn f_1000e5d0(&mut self, this: u32) {
        let ball = self.p(this + 16);
        let (x, y, z) = (self.i(ball + 24), self.i(ball + 28), self.i(ball + 32));
        self.throw_at_target(x, y, z, 700, 300);
    }

    /// `sub_1000E6B0` (to actor 8) / `sub_1000FDA0` (to actor 7): a long
    /// throw to the partner, who runs to catch it.
    pub(crate) fn throw_to_partner(&mut self, this: u32, partner: u32, state: i32) {
        let ball = self.p(this + 16);
        self.si(ball + 28, 400);
        let (bx, bz) = (self.i(ball + 24), self.i(ball + 32));
        let (px, pz) = (self.r(partner, 40), self.r(partner, 48));
        let d = distance(f64::from(bx), f64::from(bz), f64::from(px), f64::from(pz));
        let time = d * 0.01;
        let (x, y, z) = (bx, self.i(ball + 28), bz);
        // (The DLL passes an uninitialised y component; it is taken as 0.)
        self.f_10002bd0(
            x,
            y,
            z,
            ftol(f64::from(px - bx) / d * 1000.0),
            0,
            ftol(f64::from(pz - bz) / d * 1000.0),
            1000,
            ftol(time * 6.0),
        );
        self.vcall(partner, 20, state, 0);
        self.f_10005460(partner);
        self.sr(partner, 120, ftol(time));
        self.f_10004400(1, 50, false);
    }

    /// `sub_1000E550`: throws in at 100°.
    fn f_1000e550(&mut self, this: u32) {
        let ball = self.p(this + 16);
        let (x, y, z) = (self.i(ball + 24), self.i(ball + 28), self.i(ball + 32));
        let a = 1.745329251944444f64;
        self.f_10002bd0(
            x,
            y,
            z,
            ftol(a.cos() * 1000.0),
            0,
            ftol(a.sin() * 1000.0),
            700,
            700,
        );
        self.f_10004400(1, 50, false);
    }

    /// `sub_1000CBE0` (slot 6).
    pub(crate) fn f_1000cbe0(&mut self, this: u32) {
        let partner = self.actor(8);
        let lapping = self.r(partner, 0) == 1 && matches!(self.r(partner, 12), 3 | 4);
        match self.r(this, 12) {
            3 | 4 => {
                let ok = if lapping {
                    self.lap_catch(this, true)
                } else {
                    self.f_1000ce90(this)
                };
                if !ok {
                    self.sr(this, 24, 0);
                }
            }
            11 => {
                if !self.f_1000d3e0(this) {
                    self.sr(this, 24, 0);
                }
            }
            0 => {
                if !self.f_1000ccf0(this) {
                    self.sr(this, 24, 0);
                }
            }
            _ => {}
        }
    }

    /// `sub_1000CC80` (slot 7).
    pub(crate) fn f_1000cc80(&mut self, this: u32) {
        match self.r(this, 12) {
            4 => {
                self.vcall(this, 20, 2, 0);
            }
            3 => {
                self.vcall(this, 20, 3, 0);
            }
            11 => {
                self.vcall(this, 20, 1, 0);
            }
            0 => {
                let state = if self.at_home(this) { 1 } else { 13 };
                self.vcall(this, 20, state, 0);
            }
            _ => {}
        }
    }

    /// `sub_1000CCF0`: run under a fly ball.
    fn f_1000ccf0(&mut self, this: u32) -> bool {
        let mut found = false;
        let mut best = 99999999.0;
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let mut frame = 40;
        let mut reach = 20;
        let mut p = 0x1002_26CC;
        while p < 0x1002_299C {
            let v = |pt: &Self, n: u32| pt.i(p + 4 * n);
            let d = segment_foot(
                f64::from(v(self, 0)),
                f64::from(v(self, 2)),
                f64::from(v(self, 9)),
                f64::from(v(self, 11)),
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
                let state = if self.g(this, 304) == 3 { 11 } else { 5 };
                self.vcall(this, 20, state, 0);
                self.set_target(this, [v(self, 9), 0, v(self, 11)]);
                self.sr(this, 120, frame);
                self.f_10005460(this);
                found = true;
            }
            p += 36;
            frame += 3;
            reach += 60;
        }
        found
    }

    /// `sub_1000CE90`: catch while running laps (no partner running).
    fn f_1000ce90(&mut self, this: u32) -> bool {
        let mut found = false;
        let mut best = 99999999.0;
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let mut frame = 24;
        let mut reach = -500;
        let mut v4 = 0x1002_2638u32;
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
            let (x, z) = (at(self, -2), at(self, 0));
            for (state, ok) in [
                (
                    6,
                    f64::from(reach + 520) > d && (300.0..600.0).contains(&height),
                ),
                (
                    7,
                    f64::from(reach) > d && height < 300.0 && v4 >= 0x1002_2770,
                ),
            ] {
                if ok && self.f_10004310(x, z, None) == 0 && d < best {
                    best = d;
                    self.vcall(this, 20, state, 0);
                    self.set_target(this, [x, 0, z]);
                    self.sr(this, 120, frame);
                    self.f_10005460(this);
                    found = true;
                }
            }
            v4 += 36;
            frame += 3;
            reach += 60;
        }
        found
    }

    /// `sub_1000D0D0` (actor 7) / `sub_1000EBB0` (actor 8): while both run
    /// laps, meet the ball where the lap crosses its path.
    pub(crate) fn lap_catch(&mut self, this: u32, low_and_high: bool) -> bool {
        let mut found = false;
        let mut best = 99999999.0;
        if self.i(BALL) == 0 && self.i(BALL + 4) == 0 && self.i(BALL + 8) == 0 {
            return false;
        }
        let mut v3: i32 = -4;
        let mut v16 = 0x1002_2558u32;
        let mut v20: i32 = -4;
        let mut v17: i32 = -150;
        loop {
            let mut lead = 6;
            if v3 > 6 {
                let mut frame = v3 + 4;
                let mut reach = v17;
                loop {
                    let clockwise = self.r(this, 12) == 4;
                    let point = |pt: &Self, a: i32| {
                        if clockwise {
                            pt.f_10005690(this, a)
                        } else {
                            pt.f_100057a0(this, a)
                        }
                    };
                    let here = point(self, lead);
                    let before = point(self, lead - 10);
                    let bx = self.i(v16);
                    let bz = self.i(v16 + 8);
                    let dot = f64::from(
                        (here[0] - before[0]) * (bx - here[0])
                            + (here[2] - before[2]) * (bz - here[2]),
                    );
                    let mode = self.r(this, 12);
                    if (mode == 4 && dot > 0.0) || (mode == 3 && dot < 0.0) {
                        let d = segment_foot(
                            f64::from(self.i(v16 - 72)),
                            f64::from(self.i(v16 - 64)),
                            f64::from(bx),
                            f64::from(bz),
                            f64::from(here[0]),
                            f64::from(here[2]),
                        )
                        .distance;
                        let height = f64::from(self.i(v16 + 4) - here[1]);
                        if f64::from(reach) > d && height < 600.0 && d < best {
                            best = d;
                            let state = if !low_and_high || height >= 300.0 {
                                8
                            } else {
                                9
                            };
                            self.f_10005570(this, state, lead);
                            self.sr(this, 56, self.r(this, 40));
                            self.sr(this, 60, self.r(this, 44));
                            self.sr(this, 64, self.r(this, 48));
                            self.sw(this, 314, bx);
                            self.sw(this, 315, 0);
                            self.sw(this, 316, bz);
                            self.sr(this, 120, frame);
                            self.f_10005460(this);
                            found = true;
                        }
                    }
                    lead += 6;
                    reach -= 90;
                    frame -= 6;
                    if lead >= v20 {
                        break;
                    }
                }
                v3 = v20;
            }
            v3 += 6;
            v17 += 90;
            v20 = v3;
            v16 += 72;
            if v3 + 10 >= 100 {
                break;
            }
        }
        found
    }

    /// `sub_1000D3E0`: catch the ball arriving at the plate.
    fn f_1000d3e0(&mut self, this: u32) -> bool {
        let x = f64::from(self.g(this, 1024));
        // (The DLL compares against (x, x).)
        if distance(
            x,
            x,
            f64::from(self.r(this, 40)),
            f64::from(self.r(this, 48)),
        ) >= 240.0
            || f64::from(self.g(this, 1028) - self.r(this, 44)) >= 600.0
        {
            return false;
        }
        self.vcall(this, 20, 10, 0);
        self.f_10005460(this);
        true
    }

    /// Plays a relay table offset so that frame `key` falls on the catch
    /// frame (`record[120]`); runs `at_key` there. Returns the new frame.
    fn keyed_table(
        &mut self,
        this: u32,
        table: (u32, u32, u32, u32),
        key: i32,
        at_key: impl FnOnce(&mut Self),
    ) {
        let (table, source, count_at, patterns) = table;
        self.prefix_table(table, source, count_at);
        let offset = key - self.r(this, 120);
        let pattern = self.i(patterns + 4 * self.r(this, 108) as u32);
        self.f_10005090(this, pattern, 100);
        if offset + self.r(this, 112) == key {
            at_key(self);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        let index = self.r(this, 108);
        if self.i(table + 4 * index as u32) <= offset + self.r(this, 112) {
            self.sr(this, 108, index + 1);
            if self.i(count_at) <= self.r(this, 108) {
                self.vcall(this, 16, 0, 0);
            }
        }
    }

    /// Waiting at the target with a fidget that grows with time.
    fn fidget(&mut self, this: u32) {
        let pattern = match self.r(this, 116) {
            n if n < 4 => 149,
            n if n < 8 => 153,
            _ => 154,
        };
        self.f_10005090(this, pattern, 100);
        self.sr(this, 116, self.r(this, 116) + 1);
        self.sr(this, 112, self.r(this, 112) + 1);
    }

    fn run_to_catch(&mut self, this: u32) {
        let first = i32::from(self.r(this, 112) == 0);
        self.f_1000e3e0(this, 20.0, first);
        self.sr(this, 112, self.r(this, 112) + 1);
    }

    /// The collision while both run laps (actor 7 state 4, actor 8 state
    /// 5): the ball hits the runner; the caps fly; afterwards the two
    /// swap directions.
    pub(crate) fn lap_collision(&mut self, this: u32, line: i32, swap_self_first: bool) {
        if self.r(this, 112) == 0 {
            if self.r(this, 12) == 4 {
                let p = self.f_10005690(this, 0);
                self.set_target(this, p);
            } else {
                let partner = self.actor(if swap_self_first { 8 } else { 7 });
                let p = [
                    self.r(partner, 40),
                    self.r(partner, 44),
                    self.r(partner, 48),
                ];
                self.set_target(this, p);
            }
            self.f_1000e3e0(this, 0.0, 1);
        }
        if self.r(this, 112) == self.r(this, 120) {
            let (sx, sz) = match self.r(this, 52) {
                0 => (0, 1),
                1 => (1, 1),
                2 => (1, 0),
                3 => (1, -1),
                4 => (0, -1),
                5 => (-1, -1),
                6 => (-1, 0),
                7 => (-1, 1),
                _ => (0, 0),
            };
            let ball = self.p(this + 16);
            if sx * self.i(ball + 48) + sz * self.i(ball + 56) <= 0 {
                self.sb(this + 1252, 1);
                self.f_10014040(18, 255);
            } else {
                self.sb(this + 1253, 1);
                self.f_10014040(19, 255);
            }
            self.f_10014080(line);
            self.f_1000e3e0(this, 0.0, 1);
            self.si(ball, 0);
            self.si(ball + 60, 0);
            let number = self.w(this, 312);
            self.f_10004400(number + 10, 50, false);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
        if self.r(this, 112) == self.r(this, 120) + 90 {
            self.f_10005590(this, 30);
        }
        if self.r(this, 112) == self.r(this, 120) + 119 {
            let (a7, a8) = (self.actor(7), self.actor(8));
            let (d7, d8) = if self.r(this, 12) == 4 {
                (3, 4)
            } else {
                (4, 3)
            };
            let (d7, d8) = if swap_self_first { (d7, d8) } else { (d8, d7) };
            let lap = |pt: &Self, a: u32, dir: i32| {
                if dir == 4 {
                    pt.f_10005690(a, 0)
                } else {
                    pt.f_100057a0(a, 0)
                }
            };
            self.sr(a7, 12, d7);
            let p = lap(self, a7, d7);
            self.sr(a7, 40, p[0]);
            self.sr(a7, 44, p[1]);
            self.sr(a7, 48, p[2]);
            if !swap_self_first {
                self.vcall(a7, 16, 0, 0);
            }
            self.sr(a8, 12, d8);
            let p = lap(self, a8, d8);
            self.sr(a8, 40, p[0]);
            self.sr(a8, 44, p[1]);
            self.sr(a8, 48, p[2]);
            if swap_self_first {
                self.vcall(a8, 16, 0, 0);
            }
        }
    }

    /// `sub_1000D480` (slot 8).
    pub(crate) fn f_1000d480(&mut self, this: u32) {
        match self.r(this, 16) {
            1 => {
                let table = 0x1002_3108;
                self.prefix_table(table, 0x1001_EF94, 0x1001_EFBC);
                let pattern = self.table_seek(this, table, 0x1001_EF6C, 0x1001_EFBC);
                self.f_10005090(this, pattern, 100);
                self.sr(this, 112, self.r(this, 112) + 1);
                if self.r(this, 112) == self.table_total(table, 0x1001_EFBC) {
                    self.vcall(this, 16, 0, 0);
                }
            }
            // Laps, clockwise (2) or anticlockwise (3).
            2 => {
                let p = self.f_10005690(this, 0);
                self.set_target(this, p);
                let first = self.r(this, 112) == 0;
                self.f_1000e430(this, first);
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            3 => {
                let saved = [self.r(this, 40), self.r(this, 44), self.r(this, 48)];
                let p = self.f_100057a0(this, 0);
                self.set_target(this, p);
                let first = self.r(this, 112) == 0;
                self.f_1000e430(this, first);
                let partner = self.actor(8);
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
            4 => self.lap_collision(this, 72, true),
            5 => {
                self.camera_before_catch(this);
                let (t, at) = (self.r(this, 112), self.r(this, 120));
                if t >= at - 40 {
                    let table = 0x1002_3130;
                    self.prefix_table(table, 0x1001_EFE8, 0x1001_F010);
                    let offset = 40 - at;
                    self.sr(this, 108, 0);
                    let count = self.i(0x1001_F010);
                    let mut index = 0;
                    while index < count - 1
                        && offset + self.r(this, 112) >= self.i(table + 4 * index as u32)
                    {
                        index += 1;
                        self.sr(this, 108, index);
                    }
                    let pattern = self.i(0x1001_EFC0 + 4 * self.r(this, 108) as u32);
                    self.f_10005090(this, pattern, 100);
                    if offset + self.r(this, 112) == 40 {
                        self.f_1000e5d0(this);
                        self.f_10014040(13, 255);
                        self.f_10014080(70);
                    }
                    self.sr(this, 112, self.r(this, 112) + 1);
                    if self.r(this, 112) == self.table_total(table, 0x1001_F010) {
                        self.vcall(this, 16, 0, 0);
                    }
                } else if self.at_target(this) {
                    self.f_10005090(this, 129, 100);
                    self.sr(this, 112, t + 1);
                } else {
                    self.run_to_catch(this);
                }
            }
            6 => {
                self.camera_before_catch(this);
                if self.r(this, 112) >= self.r(this, 120) - 24 {
                    self.keyed_table(
                        this,
                        (0x1002_3180, 0x1001_F078, 0x1001_F088, 0x1001_F068),
                        24,
                        |pt| {
                            pt.f_1000e5d0(this);
                            pt.f_10014040(33, 255);
                            pt.f_10014080(73);
                        },
                    );
                } else if self.at_target(this) {
                    self.f_10005090(this, 149, 100);
                    self.sr(this, 112, self.r(this, 112) + 1);
                } else {
                    self.run_to_catch(this);
                }
            }
            7 => {
                self.camera_before_catch(this);
                if self.r(this, 112) >= self.r(this, 120) - 6 {
                    self.keyed_table(
                        this,
                        (0x1002_3190, 0x1001_F0A4, 0x1001_F0BC, 0x1001_F08C),
                        6,
                        |pt| {
                            pt.f_1000e5d0(this);
                            pt.f_10014040(33, 255);
                            pt.f_10014080(73);
                        },
                    );
                } else if self.at_target(this) {
                    self.fidget(this);
                } else {
                    self.run_to_catch(this);
                }
            }
            state @ (8 | 9) => {
                self.camera_before_catch(this);
                if self.r(this, 112) == 0 {
                    let p = [self.w(this, 314), self.w(this, 315), self.w(this, 316)];
                    self.set_target(this, p);
                }
                let (lead, table) = if state == 8 {
                    (10, (0x1002_31A8, 0x1001_F0D0, 0x1001_F0E0, 0x1001_F0C0))
                } else {
                    (6, (0x1002_31B8, 0x1001_F0FC, 0x1001_F114, 0x1001_F0E4))
                };
                if self.r(this, 112) >= self.r(this, 120) - lead {
                    let partner = self.actor(8);
                    self.keyed_table(this, table, lead, |pt| {
                        pt.throw_to_partner(this, partner, 5);
                        pt.f_10014040(33, 255);
                        pt.f_10014080(73);
                    });
                } else if self.at_target(this) {
                    if state == 8 {
                        self.f_10005090(this, 149, 100);
                        self.sr(this, 112, self.r(this, 112) + 1);
                    } else {
                        self.fidget(this);
                    }
                } else {
                    self.run_to_catch(this);
                }
            }
            10 => self.f_1000e270(this),
            11 => {
                self.camera_before_catch(this);
                let (t, at) = (self.r(this, 112), self.r(this, 120));
                if t >= at - 40 {
                    let table = 0x1002_3158;
                    self.prefix_table(table, 0x1001_F03C, 0x1001_F064);
                    let offset = 40 - at;
                    self.sr(this, 108, 0);
                    let count = self.i(0x1001_F064);
                    let mut index = 0;
                    while index < count - 1
                        && offset + self.r(this, 112) >= self.i(table + 4 * index as u32)
                    {
                        index += 1;
                        self.sr(this, 108, index);
                    }
                    let pattern = self.i(0x1001_F014 + 4 * self.r(this, 108) as u32) + 10;
                    self.f_10005090(this, pattern, 100);
                    if offset + self.r(this, 112) == self.i(0x1002_316C) {
                        self.f_1000e550(this);
                        self.f_10014040(13, 255);
                        self.f_10014080(70);
                    }
                    let mark = self.i(0x1002_3178);
                    if offset + self.r(this, 112) == mark + 60 {
                        self.f_10014080(68);
                    }
                    if self.r(this, 112) == mark + 180 {
                        self.sg(this, 160, 123);
                        self.sg(this, 168, 0);
                    }
                } else {
                    self.f_10005090(this, 139, 100);
                }
                self.sr(this, 112, self.r(this, 112) + 1);
            }
            12 => {
                let first = i32::from(self.r(this, 112) == 0);
                self.f_1000e3e0(this, 20.0, first);
                self.sr(this, 112, self.r(this, 112) + 1);
                self.sr(this, 20, 0);
            }
            13 => {
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

    /// `sub_1000E270` (state 10): the ball bounces off; a cut-in follows.
    fn f_1000e270(&mut self, this: u32) {
        let table = 0x1002_31D0;
        self.prefix_table(table, 0x1001_F134, 0x1001_F150);
        let pattern = self.table_seek(this, table, 0x1001_F118, 0x1001_F150);
        self.f_10005090(this, pattern, 100);
        if self.r(this, 112) == 0 {
            self.f_10014040(16, 255);
            self.f_10014080(71);
            let ball = self.p(this + 16);
            let v = |pt: &Self, n: u32| pt.i(ball + 4 * n);
            let (x, y, z) = (v(self, 6), v(self, 7), v(self, 8));
            let (dx, dy, dz) = (-v(self, 12), -v(self, 13), -v(self, 14));
            let speed = ftol(f64::from(v(self, 15)) * 0.1);
            self.f_10002bd0(x, y, z, dx, dy, dz, speed, 0);
            let number = self.w(this, 312);
            self.f_10004400(number + 10, 50, false);
        }
        if self.r(this, 112) == 80 {
            self.f_10014080(67);
        }
        if self.r(this, 112) == 120 {
            self.sg(this, 160, 123);
            self.sg(this, 168, 0);
        }
        self.sr(this, 112, self.r(this, 112) + 1);
    }
}
