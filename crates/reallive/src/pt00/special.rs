//! The scripted extras driven directly by the script: the cut-in with
//! actors 1, 3 and 5 (calls 900/901), the thrown cap (910/911), the toy
//! the cats play with (920/921) and a timed camera reset (930/931).

use super::base::distance;
use super::{BALL, Pt00, ftol};

/// `unk_100229D0`: `[1]` loop frame, `[2]` cut-in frame, `[3]` cut-in on.
const CUT_IN: u32 = 0x1002_29D0;
/// `unk_100229E8`: byte `[4]` shows the cap.
const CAP: u32 = 0x1002_29E8;
/// `unk_10022498`: `[1]` mode, `[2]` frame.
const TIMER: u32 = 0x1002_2498;

impl Pt00 {
    fn record_mode(&self, actor: u32) -> i32 {
        let object = self.actor(actor);
        self.r(object, 12)
    }

    fn record_state(&self, actor: u32) -> i32 {
        let object = self.actor(actor);
        self.r(object, 16)
    }

    fn set_state(&mut self, actor: u32, state: i32) {
        let object = self.actor(actor);
        self.vcall(object, 20, state, 0);
    }

    /// `sub_100135A0` (call 900).
    pub(crate) fn f_100135a0(&mut self) {
        self.sf(1800, 0);
        self.sf(1801, -6000);
        self.sf(1802, 0);
        self.sf(1803, 3100);
        if self.f(1043) == 7 && self.f(1123) == 7 && self.f(1203) == 7 {
            self.sf(1800, 1);
        }
        self.si(CUT_IN + 4, 0);
        self.si(CUT_IN + 12, 0);
    }

    /// `sub_10013600` (call 901).
    pub(crate) fn f_10013600(&mut self) {
        if self.f(1800) != 1 {
            return;
        }
        if self.f(210) == 2
            && self.f(252) == 1
            && distance(
                f64::from(self.i(BALL + 600)),
                f64::from(self.i(BALL + 608)),
                f64::from(self.f(1801)),
                f64::from(self.f(1803)),
            ) < 160.0
            && f64::from(self.i(BALL + 604) - self.f(1802)) < 600.0
        {
            self.sf(252, 0);
            self.si(CUT_IN + 12, 1);
            self.si(CUT_IN + 8, 0);
            self.f_10004400(11, 50, false);
        }
        if self.i(CUT_IN + 12) != 0 {
            let table = 0x1002_340C;
            self.prefix_table(table, 0x1001_F60C, 0x1001_F63C);
            let count = self.i(0x1001_F63C);
            let frame = self.i(CUT_IN + 8);
            let mut index = 0;
            while index < count - 1 {
                if frame < self.i(table + 4 * index as u32) {
                    break;
                }
                index += 1;
            }
            self.sf(1804, self.i(0x1001_F5DC + 4 * index as u32));
            if frame == 50 {
                self.set_state(1, 10);
                self.set_state(3, 4);
                self.set_state(5, 4);
                self.f_10014080(121);
            }
            if frame == 80 {
                self.f_10014080(122);
            }
            if frame == 140 {
                self.f_10014080(123);
            }
            if frame == 290 {
                self.sf(30, 7);
            }
            self.si(CUT_IN + 8, frame + 1);
            if frame + 1 == 320 {
                self.si(CUT_IN + 12, 0);
                self.sf(210, 0);
                self.sf(20, 3);
                self.sf(1804, 0);
                self.f_10004550(0, false);
                for actor in [1, 3, 5] {
                    let object = self.actor(actor);
                    self.vcall(object, 16, 0, 0);
                }
                self.sf(1805, 0);
                self.sf(1810, 0);
                self.sf(1815, 0);
            }
        }
        if self.record_state(1) == 7 && self.record_state(3) == 1 && self.record_state(5) == 1 {
            self.set_state(1, 8);
            self.set_state(3, 2);
            self.set_state(5, 2);
            self.si(CUT_IN + 4, 0);
        }
        if self.record_state(1) == 8 && self.record_state(3) == 2 && self.record_state(5) == 2 {
            let lines: [(u32, u32, u32, u32, u32, u32); 3] = [
                (
                    1,
                    0x1002_343C,
                    0x1001_F66C,
                    0x1001_F698,
                    0x1001_F640,
                    0x1001_F644,
                ),
                (
                    3,
                    0x1002_3468,
                    0x1001_F6C8,
                    0x1001_F6F4,
                    0x1001_F69C,
                    0x1001_F6A0,
                ),
                (
                    5,
                    0x1002_3494,
                    0x1001_F724,
                    0x1001_F750,
                    0x1001_F6F8,
                    0x1001_F6FC,
                ),
            ];
            for (actor, table, source, count_at, first, sets) in lines {
                self.prefix_table(table, source, count_at);
                let total = self.table_total(table, count_at);
                let frame = if total != 0 {
                    self.i(CUT_IN + 4).wrapping_rem(total)
                } else {
                    0
                };
                self.si(CUT_IN + 4, frame);
                let object = self.actor(actor);
                if frame == 0 {
                    let set = self.i(first);
                    self.cut_in_set(actor, object, set);
                }
                for i in 0..(self.i(count_at) - 1).max(0) as u32 {
                    if self.i(CUT_IN + 4) == self.i(table + 4 * i) {
                        let set = self.i(sets + 4 * i);
                        self.cut_in_set(actor, object, set);
                    }
                }
            }
            self.add(CUT_IN + 4, 1);
        }
    }

    fn cut_in_set(&mut self, actor: u32, object: u32, set: i32) {
        if actor == 1 {
            self.f_10008a50(object, set);
        } else {
            self.f_1000ae10(object, set);
        }
    }

    /// `sub_10013A20` (call 910).
    pub(crate) fn f_10013a20(&mut self) {
        if self.f(1043) == 8 && self.f(1123) == 8 {
            self.sf(1830, 1);
            self.sf(1835, self.f(1130));
            self.sf(1836, self.f(1131));
            self.sf(1837, self.f(1132));
        } else {
            self.sf(1830, 0);
            self.sf(1835, 0);
            self.sf(1836, 0);
            self.sf(1837, 0);
        }
        self.sf(1831, 0);
        self.sf(1832, self.f(1835));
        self.sf(1833, self.f(1836));
        self.sf(1834, self.f(1837));
        self.sf(1839, 0);
        self.sf(1840, 0);
        self.sf(1841, 0);
        self.sb(CAP + 4, 0);
    }

    /// `sub_10013AF0` (call 911).
    pub(crate) fn f_10013af0(&mut self) {
        if self.f(1830) != 1 {
            return;
        }
        let shown = if self.record_mode(1) != 8 || matches!(self.record_state(1), 11 | 12) {
            !(self.record_mode(3) == 8 && !matches!(self.record_state(3), 5 | 6))
        } else {
            false
        };
        self.sb(CAP + 4, u8::from(shown));
        if self.f(1831) == 1 {
            let t = self.f(1838);
            self.sf(
                1835,
                self.f(1832).wrapping_add(t.wrapping_mul(self.f(1839))),
            );
            let y = self
                .f(1833)
                .wrapping_add(t.wrapping_mul(self.f(1840)))
                .wrapping_sub(ftol(f64::from(t) * f64::from(t) * 0.6));
            self.sf(1836, y);
            self.sf(
                1837,
                self.f(1834).wrapping_add(t.wrapping_mul(self.f(1841))),
            );
            self.sf(1838, t.wrapping_add(1));
        }
        if self.f(1836) < 0 {
            self.sf(1831, 0);
        }
    }

    /// `sub_10013BF0` (call 920): the toy appears only when a cat (class
    /// 18 in the records' `+8`) is on the field.
    pub(crate) fn f_10013bf0(&mut self) {
        for n in 1850..=1865 {
            self.sf(n, 0);
        }
        self.sf(1858, 2000);
        let present = (11..crate::pt00::ACTOR_COUNT).any(|n| {
            let object = self.actor(n);
            self.r(object, 8) == 18
        });
        if present {
            self.sf(1850, 1);
            self.sf(1851, 1);
            for n in [1852, 1853, 1854, 1855, 1856, 1857] {
                self.sf(n, -1000);
            }
        }
    }

    /// `sub_10013D10` (call 921): moves the toy.
    pub(crate) fn f_10013d10(&mut self) {
        if self.f(1850) != 1 {
            return;
        }
        let t = self.f(1863).wrapping_add(1);
        self.sf(1863, t);
        let run = self.f(1861);
        let x = self
            .f(1855)
            .wrapping_add(self.f(1858).wrapping_mul(run).wrapping_mul(t) / 10 / 1000);
        self.sf(1852, x);
        let y = f64::from(self.f(1856).wrapping_add(t.wrapping_mul(self.f(1862)) / 10))
            - f64::from(t.wrapping_mul(t)) * 0.6;
        self.sf(1853, ftol(y));
        let z = self
            .f(1857)
            .wrapping_add(self.f(1860).wrapping_mul(run).wrapping_mul(t) / 10 / 1000);
        self.sf(1854, z);
        if self.f_10004310(x, z, None) > 0 {
            self.f_10013fd0();
            return;
        }
        if self.f(1853) < 0 {
            let run = (run - 4 - self.f(1862) / 20).max(0);
            self.sf(1861, run);
            let lift = ftol((f64::from(self.f(1862)) * 0.1 - f64::from(t) * 1.2) * -0.7 * 10.0);
            self.sf(1862, lift);
            self.sf(1853, 0);
            self.sf(1855, self.f(1852));
            self.sf(1856, self.f(1853));
            self.sf(1857, self.f(1854));
            self.sf(1863, 0);
            self.sf(1864, 1);
        }
    }

    /// `sub_10013F20`: sets the toy moving.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn f_10013f20(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        dx: i32,
        dy: i32,
        dz: i32,
        speed: i32,
    ) {
        self.sf(1850, 1);
        self.sf(1851, 1);
        self.sf(1852, x);
        self.sf(1853, y);
        self.sf(1854, z);
        self.sf(1855, x);
        self.sf(1856, y);
        self.sf(1857, z);
        self.sf(1861, speed);
        self.sf(1858, dx);
        self.sf(1859, dy);
        self.sf(1860, dz);
        self.sf(1862, 0);
        self.sf(1863, 0);
        self.sf(1864, 0);
    }

    /// `sub_10013FD0`: the toy is gone.
    pub(crate) fn f_10013fd0(&mut self) {
        self.sf(1850, 0);
        self.sf(1851, 0);
    }

    /// `sub_10013FF0` (call 930).
    pub(crate) fn f_10013ff0(&mut self) {
        self.si(TIMER + 4, -1);
        self.si(TIMER + 8, 0);
    }

    /// `sub_10014000` (call 931).
    pub(crate) fn f_10014000(&mut self) {
        let frame = self.i(TIMER + 8);
        if self.i(TIMER + 4) == 0 {
            if frame == 0 {
                self.sf(30, 7);
            } else if frame == 30 {
                self.f_10004550(0, false);
            }
        }
        self.si(TIMER + 8, frame + 1);
    }
}
