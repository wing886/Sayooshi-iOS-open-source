//! Functions 0x10001C50..=0x100048F0, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_10001C50` (9 bytes).
    pub(crate) fn f_10001c50(&mut self, mut this: u32) -> i32 {
        self.w32(this, 0_u32);
        return 0_i32;
    }

    /// `sub_10001C60` (3 bytes).
    pub(crate) fn f_10001c60(&mut self) -> i32 {
        return 0_i32;
    }

    /// `sub_10001C70` (11 bytes).
    pub(crate) fn f_10001c70(&mut self, mut this: u32, mut a2: i32) -> i32 {
        self.w32(this, (a2 as u32));
        return 0_i32;
    }

    /// `sub_10001C80` (128 bytes).
    pub(crate) fn f_10001c80(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut result: i32 = 0;
        v1 = 5_i32;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        v3 = v2.wrapping_add(360);
        'l1: loop {
            v4 = v3;
            v5 = v3.wrapping_add(60);
            let _ = self.memset(v4, 0_i32 as u8, 60_u32);
            self.w32(v5, 1_u32);
            v3 = v5.wrapping_add(4);
            v1 = v1.wrapping_sub(1);
            if !(v1 != 0) {
                break 'l1;
            }
        }
        let _ = self.memset(v2.wrapping_add(680), 0_i32 as u8, 320_u32);
        self.w32(v2.wrapping_add(1000), 0_u32);
        self.w32(v2.wrapping_add(1004), 0_u32);
        self.w32(v2.wrapping_add(1008), 0_u32);
        self.w32(v2.wrapping_add(1012), 0_u32);
        self.w32(v2.wrapping_add(1016), 0_u32);
        result = 0_i32;
        self.w32(v2.wrapping_add(1020), 0_u32);
        self.w32(v2.wrapping_add(1024), 0_u32);
        self.w32(v2.wrapping_add(1028), 0_u32);
        self.w32(v2.wrapping_add(1032), 0_u32);
        self.w32(v2.wrapping_add(1036), 0_u32);
        self.w32(v2.wrapping_add(1040), 0_u32);
        self.w32(v2.wrapping_add(1044), 0_u32);
        self.w32(v2.wrapping_add(1048), 0_u32);
        return result;
    }

    /// `sub_10001D00` (119 bytes).
    pub(crate) fn f_10001d00(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut result: i32 = 0;
        v1 = 5_i32;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        v3 = v2.wrapping_add(360);
        'l1: loop {
            v4 = v3;
            v3 = v3.wrapping_add(64);
            v1 = v1.wrapping_sub(1);
            let _ = self.memset(v4, 0_i32 as u8, 60_u32);
            if !(v1 != 0) {
                break 'l1;
            }
        }
        let _ = self.memset(v2.wrapping_add(680), 0_i32 as u8, 320_u32);
        self.w32(v2.wrapping_add(1000), 0_u32);
        self.w32(v2.wrapping_add(1004), 0_u32);
        self.w32(v2.wrapping_add(1008), 0_u32);
        self.w32(v2.wrapping_add(1012), 0_u32);
        self.w32(v2.wrapping_add(1016), 0_u32);
        result = 0_i32;
        self.w32(v2.wrapping_add(1020), 0_u32);
        self.w32(v2.wrapping_add(1024), 0_u32);
        self.w32(v2.wrapping_add(1028), 0_u32);
        self.w32(v2.wrapping_add(1032), 0_u32);
        self.w32(v2.wrapping_add(1036), 0_u32);
        self.w32(v2.wrapping_add(1040), 0_u32);
        self.w32(v2.wrapping_add(1044), 0_u32);
        self.w32(v2.wrapping_add(1048), 0_u32);
        return result;
    }

    /// `sub_10001D80` (53 bytes).
    pub(crate) fn f_10001d80(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v1 = 5_i32;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32)
            .wrapping_add(1000_i32);
        'l1: loop {
            v3 = (((self.r32((v2.wrapping_add(20_i32) as u32)) as i32) != 0_i32) as i32)
                .wrapping_neg();
            v2 = v2.wrapping_add(4_i32);
            v1 = v1.wrapping_sub(1);
            self.w32(
                (v2.wrapping_sub(4_i32) as u32),
                ((((v3 as u32) & 0xFFFFFFF1_u32).wrapping_add(15_u32) as i32) as u32),
            );
            if !(v1 != 0) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10001DC0` (70 bytes).
    pub(crate) fn f_10001dc0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        if ((self.r32((v2.wrapping_add(1048_i32) as u32)) as i32) == 1_i32) {
            v3 = (self.r32(
                (v2.wrapping_add(4_i32.wrapping_mul(a2))
                    .wrapping_add(1000_i32) as u32),
            ) as i32);
            if (v3
                < (self.r32(
                    (v2.wrapping_add(4_i32.wrapping_mul(a2))
                        .wrapping_add(1020_i32) as u32),
                ) as i32)
                    .wrapping_sub(1_i32))
            {
                self.w32(
                    (v2.wrapping_add(4_i32.wrapping_mul(a2))
                        .wrapping_add(1000_i32) as u32),
                    (v3.wrapping_add(1_i32) as u32),
                );
                return 0_i32;
            }
            self.w32(
                (v2.wrapping_add(4_i32.wrapping_mul(a2))
                    .wrapping_add(1000_i32) as u32),
                15_u32,
            );
        }
        return 0_i32;
    }

    /// `sub_10001E10` (77 bytes).
    pub(crate) fn f_10001e10(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        self.w32(
            v2.wrapping_add(1040),
            ((self.r32(
                v2.wrapping_add(
                    (16_i32.wrapping_mul(a2).wrapping_add(90_i32).wrapping_add(
                        (self
                            .r32(v2.wrapping_add((a2.wrapping_add(250_i32) as u32).wrapping_mul(4)))
                            as i32),
                    ) as u32)
                        .wrapping_mul(4),
                ),
            ) as i32) as u32),
        );
        v3 = (self.r32(
            v2.wrapping_add(
                (16_i32.wrapping_mul(a2).wrapping_add(170_i32).wrapping_add(
                    (self.r32(v2.wrapping_add((a2.wrapping_add(250_i32) as u32).wrapping_mul(4)))
                        as i32),
                ) as u32)
                    .wrapping_mul(4),
            ),
        ) as i32);
        self.w32(v2.wrapping_add(1048), 0_u32);
        self.w32(v2.wrapping_add(1044), (v3 as u32));
        return 0_i32;
    }

    /// `sub_10001E80` (19 bytes).
    pub(crate) fn f_10001e80(&mut self, mut this: u32) -> i32 {
        let _ = self.f_10001c60();
        let _ = self.f_10001ea0(this);
        return 0_i32;
    }

    /// `sub_10001EA0` (16 bytes).
    pub(crate) fn f_10001ea0(&mut self, mut this: u32) -> i32 {
        self.w32(this, 0_u32);
        self.w32(this.wrapping_add(4), 0_u32);
        return 0_i32;
    }

    /// `sub_10001EB0` (69 bytes).
    pub(crate) fn f_10001eb0(&mut self, mut this: u32) -> i32 {
        let mut result: i32 = 0;
        let mut v2: u32 = 0;
        result = 0_i32;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        let _ = self.memset(v2.wrapping_add(1080), 0_i32 as u8, 100_u32);
        let _ = self.memset(v2.wrapping_add(1180), 0_i32 as u8, 100_u32);
        let _ = self.memset(v2.wrapping_add(1280), 0_i32 as u8, 100_u32);
        self.w32(v2.wrapping_add(1380), 0_u32);
        self.w32(v2.wrapping_add(1384), 0_u32);
        self.w32(v2.wrapping_add(1388), 0_u32);
        return result;
    }

    /// `sub_10001F00` (338 bytes).
    pub(crate) fn f_10001f00(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10001f00_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10001f00_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = 0_i32;
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        self.w32(
            fp.wrapping_add(0),
            ((self.r32((v1.wrapping_add(12_i32) as u32)) as i32) as u32),
        );
        v4 = ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(3600_i32) as u32);
        self.w32(v3.wrapping_add(1380), 0_u32);
        v5 = v3.wrapping_add(1200);
        'l1: loop {
            if (((self.r32(v4) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(116)) as i32) != 99_i32))
            {
                self.w32(
                    v5.wrapping_sub(100),
                    ((self.r32(v4.wrapping_add(136)) as i32) as u32),
                );
                v6 = (self.r32(v4.wrapping_add(108)) as i32)
                    .wrapping_add(10_i32)
                    .wrapping_add((self.r32(v4.wrapping_add(136)) as i32));
                self.w32(v4.wrapping_add(136), (v6 as u32));
                self.w32(v5, (v6 as u32));
                v7 = (self.r32(v4.wrapping_add(136)) as i32);
                if (v7 > 1000_i32) {
                    self.w32(v4.wrapping_add(136), (v7.wrapping_sub(1000_i32) as u32));
                    {
                        let a0 = v3.wrapping_add(
                            ({
                                let t1 = (self.r32(v3.wrapping_add(1380)) as i32);
                                self.w32(v3.wrapping_add(1380), (t1.wrapping_add(1) as u32));
                                t1
                            }
                            .wrapping_add(320_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = (v2.wrapping_add(5_i32) as u32);
                        self.w32(a0, a1)
                    };
                }
            }
            v4 = v4.wrapping_add(156);
            v2 = v2.wrapping_add(1);
            v5 = v5.wrapping_add(4);
            if !(v2 < 20_i32) {
                break 'l1;
            }
        }
        v8 = 0_i32;
        v9 = v3.wrapping_add(1180);
        v10 = ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(392_i32) as u32);
        'l2: loop {
            if (((self.r32(v10.wrapping_sub(392)) as i32) == 1_i32)
                && ((self.r32(v10.wrapping_sub(20)) as i32) != 99_i32))
            {
                self.w32(v9.wrapping_sub(100), ((self.r32(v10) as i32) as u32));
                self.w32(
                    v10,
                    ((self.r32(v10) as i32)
                        .wrapping_add((self.r32(v10.wrapping_sub(28)) as i32).wrapping_add(10_i32))
                        as u32),
                );
                let _ = self.f_100084b0(
                    self.r32(this.wrapping_add(4)),
                    0_i32,
                    v8,
                    fp.wrapping_add(8),
                );
                v11 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add((self.r32(v10) as i32));
                self.w32(v10, (v11 as u32));
                self.w32(v9, (v11 as u32));
                if ((self.r32(v10) as i32) > 1000_i32) {
                    self.w32(v10, ((self.r32(v10) as i32).wrapping_sub(1000_i32) as u32));
                    {
                        let a0 = v3.wrapping_add(
                            ({
                                let t2 = (self.r32(v3.wrapping_add(1380)) as i32);
                                self.w32(v3.wrapping_add(1380), (t2.wrapping_add(1) as u32));
                                t2
                            }
                            .wrapping_add(320_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = (v8 as u32);
                        self.w32(a0, a1)
                    };
                }
            }
            v10 = v10.wrapping_add(716);
            v8 = v8.wrapping_add(1);
            v9 = v9.wrapping_add(4);
            if !(v8 < 5_i32) {
                break 'l2;
            }
        }
        return 0_i32;
    }

    /// `sub_10002060` (177 bytes).
    pub(crate) fn f_10002060(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = (self.r32((v1.wrapping_add(28_i32) as u32)) as i32);
        v4 = (self.r32((v3.wrapping_add(1380_i32) as u32)) as i32).wrapping_sub(1_i32);
        v12 = (v3.wrapping_add(1280_i32) as u32);
        if (v4 <= 0_i32) {
            return 0_i32;
        }
        'l1: loop {
            v5 = v12;
            v11 = v4;
            'l2: loop {
                v6 = (self.r32(v5) as i32);
                v7 = (self.r32(v5.wrapping_add(4)) as i32);
                if ((self.r32(v5) as i32) >= 5_i32) {
                    v8 = (self.r32(
                        (v2.wrapping_add(156_i32.wrapping_mul((self.r32(v5) as i32)))
                            .wrapping_add(2956_i32) as u32),
                    ) as i32);
                } else {
                    v8 = (self.r32(
                        (v2.wrapping_add(716_i32.wrapping_mul((self.r32(v5) as i32)))
                            .wrapping_add(392_i32) as u32),
                    ) as i32);
                }
                if (v7 >= 5_i32) {
                    v9 = (self.r32(
                        (v2.wrapping_add(156_i32.wrapping_mul(v7))
                            .wrapping_add(2956_i32) as u32),
                    ) as i32);
                } else {
                    v9 = (self.r32(
                        (v2.wrapping_add(716_i32.wrapping_mul(v7))
                            .wrapping_add(392_i32) as u32),
                    ) as i32);
                }
                if (v9 < v8) {
                    self.w32(v5, (v7 as u32));
                    self.w32(v5.wrapping_add(4), (v6 as u32));
                }
                v5 = v5.wrapping_add(4);
                v11 = v11.wrapping_sub(1);
                if !(v11 != 0) {
                    break 'l2;
                }
            }
            v4 = v4.wrapping_sub(1);
            if !(v4 > 0_i32) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10002120` (151 bytes).
    pub(crate) fn f_10002120(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: bool = false;
        let mut i: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v1 = (self.r32(self.r32(this)) as i32);
                    v2 = 0_i32;
                    v3 = (self.r32((v1.wrapping_add(28_i32) as u32)) as i32);
                    v4 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
                    v5 = (self.r32((v3.wrapping_add(1380_i32) as u32)) as i32).wrapping_sub(1_i32);
                    v6 = ((self.r32((v3.wrapping_add(1380_i32) as u32)) as i32) == 1_i32);
                    self.w32((v3.wrapping_add(1380_i32) as u32), (v5 as u32));
                    bb = if ((v5 >= 0_i32) && (!v6)) { 1 } else { 2 };
                }
                1 => {
                    i = (v3.wrapping_add(1280_i32) as u32);
                    bb = 3;
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    bb = 4;
                }
                4 => {
                    bb = if ((self.r32(i) as i32) >= 5_i32) {
                        7
                    } else {
                        9
                    };
                }
                5 => {
                    i = i.wrapping_add(4);
                    bb = 3;
                }
                6 => {
                    bb = 2;
                }
                7 => {
                    bb = if ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul((self.r32(i) as i32)))
                            .wrapping_add(2936_i32) as u32),
                    ) as i32)
                        == 99_i32)
                    {
                        10
                    } else {
                        11
                    };
                }
                8 => {
                    bb = if ({
                        let t1 = v2.wrapping_add(1);
                        v2 = t1;
                        t1
                    } >= v5)
                    {
                        21
                    } else {
                        22
                    };
                }
                9 => {
                    bb = if ((self.r32(
                        (v4.wrapping_add(716_i32.wrapping_mul((self.r32(i) as i32)))
                            .wrapping_add(372_i32) as u32),
                    ) as i32)
                        == 99_i32)
                    {
                        18
                    } else {
                        19
                    };
                }
                10 => {
                    bb = 12;
                }
                11 => {
                    bb = 8;
                }
                12 => {
                    v8 = v2;
                    bb = if (v2 < v5.wrapping_sub(1_i32)) {
                        13
                    } else {
                        14
                    };
                }
                13 => {
                    v9 = i;
                    bb = 15;
                }
                14 => {
                    v10 = (self.r32((v3.wrapping_add(1380_i32) as u32)) as i32).wrapping_sub(1_i32);
                    self.w32((v3.wrapping_add(1380_i32) as u32), (v10 as u32));
                    v5 = v10;
                    bb = 11;
                }
                15 => {
                    v8 = v8.wrapping_add(1);
                    self.w32(v9, ((self.r32(v9.wrapping_add(4)) as i32) as u32));
                    v9 = v9.wrapping_add(4);
                    bb = 16;
                }
                16 => {
                    bb = if (v8
                        < (self.r32((v3.wrapping_add(1380_i32) as u32)) as i32).wrapping_sub(1_i32))
                    {
                        15
                    } else {
                        17
                    };
                }
                17 => {
                    bb = 14;
                }
                18 => {
                    bb = 12;
                }
                19 => {
                    bb = 8;
                }
                20 => {
                    bb = 19;
                }
                21 => {
                    return 0_i32;
                }
                22 => {
                    bb = 5;
                }
                23 => {
                    bb = 22;
                }
                24 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100021C0` (66 bytes).
    pub(crate) fn f_100021c0(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: i32 = 0;
        v1 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        v2 = (self.r32(v1.wrapping_add(
            ((self.r32(v1.wrapping_add(1380)) as i32).wrapping_add(319_i32) as u32).wrapping_mul(4),
        )) as i32);
        if (v2 >= 5_i32) {
            v2 = v2.wrapping_sub(5_i32);
            self.w32(v1.wrapping_add(1384), 1_u32);
        } else {
            self.w32(v1.wrapping_add(1384), 0_u32);
        }
        self.w32(v1.wrapping_add(1388), (v2 as u32));
        return 0_i32;
    }

    /// `sub_10002210` (283 bytes).
    pub(crate) fn f_10002210(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(208);
        let r = self.f_10002210_body(fp, this);
        self.leave(208);
        r
    }

    fn f_10002210_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        v1 = 0_i32;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32);
        v3 = 0_i32;
        self.w32(fp.wrapping_add(0), (v2 as u32));
        v4 = (v2.wrapping_add(364_i32) as u32);
        'l1: loop {
            if (((self.r32(v4.wrapping_sub(364)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(8)) as i32) != 99_i32))
            {
                v5 = (self.r32(v4) as i32);
                self.w32(
                    fp.wrapping_add(4).wrapping_add((v1 as u32).wrapping_mul(4)),
                    (v3 as u32),
                );
                self.w32(
                    fp.wrapping_add(4).wrapping_add(
                        ({
                            let t1 = v1;
                            v1 = t1.wrapping_add(1);
                            t1
                        }
                        .wrapping_add(25_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    (v5 as u32),
                );
            }
            v4 = v4.wrapping_add(716);
            v3 = v3.wrapping_add(1);
            if !(v3 < 5_i32) {
                break 'l1;
            }
        }
        v6 = (v2.wrapping_add(3600_i32) as u32);
        i = 0_i32;
        'l2: loop {
            if !(i < 20_i32) {
                break 'l2;
            }
            if (((self.r32(v6) as i32) == 1_i32)
                && ((self.r32(v6.wrapping_add(116)) as i32) != 99_i32))
            {
                self.w32(
                    fp.wrapping_add(4).wrapping_add((v1 as u32).wrapping_mul(4)),
                    (i.wrapping_add(5_i32) as u32),
                );
                self.w32(
                    fp.wrapping_add(4).wrapping_add(
                        ({
                            let t2 = v1;
                            v1 = t2.wrapping_add(1);
                            t2
                        }
                        .wrapping_add(25_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    ((self.r32(v6.wrapping_add(108)) as i32) as u32),
                );
            }
            v6 = v6.wrapping_add(156);
            i = i.wrapping_add(1);
        }
        j = v1.wrapping_sub(1_i32);
        'l3: loop {
            if !(j > 0_i32) {
                break 'l3;
            }
            k = 0_i32;
            'l4: loop {
                if !(k < j) {
                    break 'l4;
                }
                v10 = (self.r32(
                    fp.wrapping_add(4)
                        .wrapping_add((k.wrapping_add(25_i32) as u32).wrapping_mul(4)),
                ) as i32);
                v11 = (self.r32(
                    fp.wrapping_add(4)
                        .wrapping_add((k.wrapping_add(26_i32) as u32).wrapping_mul(4)),
                ) as i32);
                if (v10 < v11) {
                    v12 = (self.r32(fp.wrapping_add(4).wrapping_add((k as u32).wrapping_mul(4)))
                        as i32);
                    self.w32(
                        fp.wrapping_add(4).wrapping_add((k as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(4)
                                .wrapping_add((k.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    v2 = (self.r32(fp.wrapping_add(0)) as i32);
                    self.w32(
                        fp.wrapping_add(4)
                            .wrapping_add((k.wrapping_add(25_i32) as u32).wrapping_mul(4)),
                        (v11 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4)
                            .wrapping_add((k.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        (v12 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4)
                            .wrapping_add((k.wrapping_add(26_i32) as u32).wrapping_mul(4)),
                        (v10 as u32),
                    );
                }
                k = k.wrapping_add(1);
            }
            j = j.wrapping_sub(1);
        }
        if (v1 <= 0_i32) {
            v13 = (self.r32(fp.wrapping_add(0)) as i32);
        } else {
            v13 = cdiv_i32(850_i32, v1);
        }
        if (v1 > 0_i32) {
            v14 = fp.wrapping_add(4);
            v15 = 950_i32;
            'l5: loop {
                if ((self.r32(v14) as i32) >= 5_i32) {
                    self.w32(
                        (v2.wrapping_add(3600_i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v14) as i32)))
                            .wrapping_sub(644_i32) as u32),
                        (v15 as u32),
                    );
                } else {
                    self.w32(
                        (v2.wrapping_add(716_i32.wrapping_mul((self.r32(v14) as i32)))
                            .wrapping_add(392_i32) as u32),
                        (v15 as u32),
                    );
                }
                v14 = v14.wrapping_add(4);
                v15 = v15.wrapping_sub(v13);
                v1 = v1.wrapping_sub(1);
                if !(v1 != 0) {
                    break 'l5;
                }
            }
        }
        return 0_i32;
    }

    /// `sub_10002330` (19 bytes).
    pub(crate) fn f_10002330(&mut self, mut this: u32) -> i32 {
        let _ = self.f_10001c60();
        let _ = self.f_10001c50(this);
        return 0_i32;
    }

    /// `sub_10002350` (16 bytes).
    pub(crate) fn f_10002350(&mut self, mut this: u32, mut a2: i32) -> i32 {
        self.w32(this, (a2 as u32));
        let _ = self.f_10002360(this);
        return 0_i32;
    }

    /// `sub_10002360` (24 bytes).
    pub(crate) fn f_10002360(&mut self, mut this: u32) -> i32 {
        self.w32(this.wrapping_add(4), 0x10028050_u32);
        self.w32(this.wrapping_add(8), 0x10028370_u32);
        self.w32(this.wrapping_add(12), 0x100284D8_u32);
        return 0_i32;
    }

    /// `sub_10002380` (1058 bytes).
    pub(crate) fn f_10002380(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: i32 = 0;
        let mut v18: u32 = 0;
        let mut v19: i32 = 0;
        let mut v21: u32 = 0;
        let mut v22: i32 = 0;
        v3 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(12_i32) as u32))
            as i32);
        v22 = v3;
        if (a2 != 0) {
            if (a2 == 1_i32) {
                v21 = (v3
                    .wrapping_add(156_i32.wrapping_mul(a3))
                    .wrapping_add(3600_i32) as u32);
                self.w32(v21, 0_u32);
                self.w32(v21.wrapping_add(4), ((1_i32).wrapping_neg() as u32));
                self.w32(v21.wrapping_add(8), 1_u32);
                self.w32(v21.wrapping_add(12), 0_u32);
                self.w32(v21.wrapping_add(16), 0_u32);
                self.w32(v21.wrapping_add(20), 0_u32);
                self.w32(v21.wrapping_add(24), 0_u32);
                self.w32(v21.wrapping_add(28), 0_u32);
                self.w32(v21.wrapping_add(32), 0_u32);
                self.w32(v21.wrapping_add(36), 0_u32);
                self.w32(v21.wrapping_add(40), 0_u32);
                self.w32(v21.wrapping_add(44), 0_u32);
                self.w32(v21.wrapping_add(48), 0_u32);
                self.w32(v21.wrapping_add(52), 0_u32);
                self.w32(v21.wrapping_add(56), 0_u32);
                self.w32(v21.wrapping_add(60), 0_u32);
                self.w32(v21.wrapping_add(64), 0_u32);
                self.w32(v21.wrapping_add(68), 0_u32);
                self.w32(v21.wrapping_add(72), 0_u32);
                self.w32(v21.wrapping_add(76), 0_u32);
                self.w32(v21.wrapping_add(80), 0_u32);
                self.w32(v21.wrapping_add(84), 0_u32);
                self.w32(v21.wrapping_add(88), 0_u32);
                self.w32(v21.wrapping_add(92), 0_u32);
                self.w32(v21.wrapping_add(96), 0_u32);
                self.w32(v21.wrapping_add(100), 0_u32);
                self.w32(v21.wrapping_add(104), 0_u32);
                self.w32(v21.wrapping_add(108), 0_u32);
                self.w32(v21.wrapping_add(112), 0_u32);
                self.w32(v21.wrapping_add(116), 0_u32);
                self.w32(v21.wrapping_add(120), 0_u32);
                self.w32(v21.wrapping_add(124), 0_u32);
                self.w32(v21.wrapping_add(128), 0_u32);
                self.w32(v21.wrapping_add(132), ((1_i32).wrapping_neg() as u32));
                self.w32(v21.wrapping_add(136), 0_u32);
                self.w32(v21.wrapping_add(140), 0_u32);
                self.w32(v21.wrapping_add(144), 0_u32);
                self.w32(v21.wrapping_add(148), 0_u32);
                self.w32(v21.wrapping_add(152), 0_u32);
            }
            self.w32((v3.wrapping_add(6800_i32) as u32), 0_u32);
            return 0_i32;
        } else {
            v4 = (v3.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
            self.w32(v4, 0_u32);
            self.w32(v4.wrapping_add(4), 0_u32);
            self.w32(v4.wrapping_add(8), 1_u32);
            self.w32(v4.wrapping_add(12), ((1_i32).wrapping_neg() as u32));
            let _ = self.memset(v4.wrapping_add(16), 0_i32 as u8, 40_u32);
            self.w32(v4.wrapping_add(56), 1_u32);
            's1: {
                'b1_3: {
                    'b1_2: {
                        'b1_1: {
                            'b1_0: {
                                match a3 {
                                    0_i32 | 1_i32 | 2_i32 => break 'b1_0,
                                    3_i32 => break 'b1_1,
                                    4_i32 => break 'b1_2,
                                    _ => break 'b1_3,
                                }
                            }
                            self.w32(v4.wrapping_add(12), 0_u32);
                            self.w32(v4.wrapping_add(16), 1_u32);
                            break 's1;
                        }
                        self.w32(v4.wrapping_add(12), 7_u32);
                        self.w32(v4.wrapping_add(44), 1_u32);
                        break 's1;
                    }
                    self.w32(v4.wrapping_add(12), ((1_i32).wrapping_neg() as u32));
                    break 's1;
                }
                break 's1;
            }
            self.w32(
                v4.wrapping_add(60),
                ((self.r32(
                    ((self.r32(this.wrapping_add(4)) as i32)
                        .wrapping_add(8_i32.wrapping_mul((self.r32(v4.wrapping_add(8)) as i32)))
                        .wrapping_add(4_i32) as u32),
                ) as i32) as u32),
            );
            self.w32(v4.wrapping_add(64), 0_u32);
            self.w32(v4.wrapping_add(68), 0_u32);
            let _ = self.memset(v4.wrapping_add(72), 255_i32 as u8, 60_u32);
            self.w32(v4.wrapping_add(132), 0_u32);
            self.w32(v4.wrapping_add(136), 0_u32);
            self.w32(v4.wrapping_add(140), 0_u32);
            self.w32(v4.wrapping_add(144), 0_u32);
            self.w32(v4.wrapping_add(148), 0_u32);
            self.w32(v4.wrapping_add(152), 0_u32);
            self.w32(v4.wrapping_add(156), 0_u32);
            self.w32(v4.wrapping_add(160), 0_u32);
            self.w32(v4.wrapping_add(164), 0_u32);
            self.w32(v4.wrapping_add(168), 0_u32);
            self.w32(v4.wrapping_add(172), 0_u32);
            self.w32(v4.wrapping_add(176), 0_u32);
            self.w32(v4.wrapping_add(180), 0_u32);
            self.w32(v4.wrapping_add(184), 0_u32);
            self.w32(v4.wrapping_add(188), 0_u32);
            self.w32(v4.wrapping_add(192), 0_u32);
            self.w32(v4.wrapping_add(196), 0_u32);
            self.w32(v4.wrapping_add(200), 0_u32);
            self.w32(v4.wrapping_add(204), 0_u32);
            self.w32(v4.wrapping_add(208), 0_u32);
            self.w32(v4.wrapping_add(212), 0_u32);
            self.w32(v4.wrapping_add(216), 0_u32);
            self.w32(v4.wrapping_add(220), 0_u32);
            self.w32(v4.wrapping_add(224), 0_u32);
            self.w32(v4.wrapping_add(228), 0_u32);
            self.w32(v4.wrapping_add(232), 0_u32);
            self.w32(v4.wrapping_add(236), 0_u32);
            self.w32(v4.wrapping_add(240), 0_u32);
            self.w32(v4.wrapping_add(244), 0_u32);
            self.w32(v4.wrapping_add(248), 0_u32);
            self.w32(v4.wrapping_add(252), 0_u32);
            self.w32(v4.wrapping_add(256), 0_u32);
            self.w32(v4.wrapping_add(260), 0_u32);
            self.w32(v4.wrapping_add(264), 0_u32);
            self.w32(v4.wrapping_add(268), 0_u32);
            let _ = self.memset(v4.wrapping_add(272), 0_i32 as u8, 60_u32);
            self.w32(v4.wrapping_add(332), 0_u32);
            self.w32(v4.wrapping_add(336), 0_u32);
            self.w32(v4.wrapping_add(340), 0_u32);
            self.w32(v4.wrapping_add(344), 0_u32);
            self.w32(v4.wrapping_add(348), 0_u32);
            self.w32(v4.wrapping_add(352), 0_u32);
            self.w32(v4.wrapping_add(356), 0_u32);
            self.w32(v4.wrapping_add(360), 0_u32);
            self.w32(v4.wrapping_add(364), 0_u32);
            self.w32(v4.wrapping_add(368), 0_u32);
            self.w32(v4.wrapping_add(372), 0_u32);
            self.w32(v4.wrapping_add(376), 0_u32);
            's2: {
                'b2_5: {
                    'b2_4: {
                        'b2_3: {
                            'b2_2: {
                                'b2_1: {
                                    'b2_0: {
                                        match a3 {
                                            0_i32 => break 'b2_0,
                                            1_i32 => break 'b2_1,
                                            2_i32 => break 'b2_2,
                                            3_i32 => break 'b2_3,
                                            4_i32 => break 'b2_4,
                                            _ => break 'b2_5,
                                        }
                                    }
                                    self.w32(v4.wrapping_add(380), 4_u32);
                                    self.w32(v4.wrapping_add(384), 6_u32);
                                    break 's2;
                                }
                                self.w32(v4.wrapping_add(380), 4_u32);
                                self.w32(v4.wrapping_add(384), 8_u32);
                                break 's2;
                            }
                            self.w32(v4.wrapping_add(380), 4_u32);
                            self.w32(v4.wrapping_add(384), 11_u32);
                            break 's2;
                        }
                        self.w32(v4.wrapping_add(380), 4_u32);
                        self.w32(v4.wrapping_add(384), 13_u32);
                        break 's2;
                    }
                    self.w32(v4.wrapping_add(380), 4_u32);
                    self.w32(v4.wrapping_add(384), 10_u32);
                    break 's2;
                }
                break 's2;
            }
            self.w32(
                v4.wrapping_add(396),
                ((self.r32(v4.wrapping_add(380)) as i32) as u32),
            );
            v5 = (self.r32(v4.wrapping_add(384)) as i32);
            self.w32(v4.wrapping_add(388), 1_u32);
            self.w32(v4.wrapping_add(400), (v5 as u32));
            self.w32(v4.wrapping_add(404), 1_u32);
            self.w32(v4.wrapping_add(392), 0_u32);
            v6 = v4.wrapping_add(468);
            v7 = 15_i32;
            'l3: loop {
                self.w32(v6.wrapping_sub(60), 0_u32);
                self.w32(
                    {
                        let t1 = v6;
                        v6 = t1.wrapping_add(4);
                        t1
                    },
                    0_u32,
                );
                v7 = v7.wrapping_sub(1);
                if !(v7 != 0) {
                    break 'l3;
                }
            }
            v8 = v4.wrapping_add(536);
            v9 = 2_i32;
            'l4: loop {
                self.w32(v8.wrapping_sub(8), 0_u32);
                self.w32(
                    {
                        let t2 = v8;
                        v8 = t2.wrapping_add(4);
                        t2
                    },
                    0_u32,
                );
                v9 = v9.wrapping_sub(1);
                if !(v9 != 0) {
                    break 'l4;
                }
            }
            self.w32(v4.wrapping_add(544), 0_u32);
            self.w32(v4.wrapping_add(548), 0_u32);
            self.w32(v4.wrapping_add(552), 0_u32);
            self.w32(v4.wrapping_add(556), 0_u32);
            self.w32(v4.wrapping_add(560), 0_u32);
            self.w32(v4.wrapping_add(564), 0_u32);
            v10 = v4.wrapping_add(576);
            v11 = 2_i32;
            'l5: loop {
                self.w32(v10.wrapping_sub(8), 0_u32);
                self.w32(v10, 0_u32);
                self.w32(v10.wrapping_add(8), 0_u32);
                v10 = v10.wrapping_add(4);
                v11 = v11.wrapping_sub(1);
                if !(v11 != 0) {
                    break 'l5;
                }
            }
            v12 = v4.wrapping_add(604);
            v13 = 3_i32;
            'l6: loop {
                self.w32(v12.wrapping_sub(12), 0_u32);
                self.w32(v12, 0_u32);
                self.w32(v12.wrapping_add(12), 0_u32);
                v12 = v12.wrapping_add(4);
                v13 = v13.wrapping_sub(1);
                if !(v13 != 0) {
                    break 'l6;
                }
            }
            v14 = v4.wrapping_add(636);
            v15 = 2_i32;
            'l7: loop {
                self.w32(v14.wrapping_sub(8), 0_u32);
                self.w32(
                    {
                        let t3 = v14;
                        v14 = t3.wrapping_add(4);
                        t3
                    },
                    0_u32,
                );
                v15 = v15.wrapping_sub(1);
                if !(v15 != 0) {
                    break 'l7;
                }
            }
            v16 = v4.wrapping_add(656);
            v17 = 3_i32;
            'l8: loop {
                self.w32(v16.wrapping_sub(12), 0_u32);
                self.w32(
                    {
                        let t4 = v16;
                        v16 = t4.wrapping_add(4);
                        t4
                    },
                    0_u32,
                );
                v17 = v17.wrapping_sub(1);
                if !(v17 != 0) {
                    break 'l8;
                }
            }
            v18 = v4.wrapping_add(676);
            v19 = 2_i32;
            'l9: loop {
                self.w32(v18.wrapping_sub(8), 0_u32);
                self.w32(
                    {
                        let t5 = v18;
                        v18 = t5.wrapping_add(4);
                        t5
                    },
                    0_u32,
                );
                v19 = v19.wrapping_sub(1);
                if !(v19 != 0) {
                    break 'l9;
                }
            }
            self.w32(v4.wrapping_add(684), 0_u32);
            self.w32(v4.wrapping_add(688), 0_u32);
            self.w32(v4.wrapping_add(696), 0_u32);
            self.w32(v4.wrapping_add(700), 0_u32);
            self.w32(v4.wrapping_add(704), 0_u32);
            self.w32(v4.wrapping_add(708), 0_u32);
            self.w32(v4.wrapping_add(712), 0_u32);
            self.w32((v22.wrapping_add(6800_i32) as u32), 0_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100027D0` (407 bytes).
    pub(crate) fn f_100027d0(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: i32 = 0;
        v3 = (self.r32(self.r32(this)) as i32);
        v4 = (self.r32((v3.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32((v3.wrapping_add(16_i32) as u32)) as i32);
        if (!(a2 != 0)) {
            v6 = (v4.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
            v7 = (self.r32(v6.wrapping_add(132)) as i32);
            self.w32(
                v6.wrapping_add(220),
                ((self.r32(v6.wrapping_add(68)) as i32)
                    .wrapping_add((self.r32(v6.wrapping_add(172)) as i32)) as u32),
            );
            v8 = (self.r32(v6.wrapping_add(180)) as i32);
            self.w32(v6.wrapping_add(224), 0_u32);
            v9 = v7.wrapping_add(v8);
            v10 = (self.r32(v6.wrapping_add(136)) as i32);
            self.w32(v6.wrapping_add(228), (v9 as u32));
            v11 = v10.wrapping_add((self.r32(v6.wrapping_add(184)) as i32));
            v12 = (self.r32(v6.wrapping_add(140)) as i32);
            self.w32(v6.wrapping_add(232), (v11 as u32));
            v13 = v12.wrapping_add((self.r32(v6.wrapping_add(188)) as i32));
            v14 = (self.r32(v6.wrapping_add(144)) as i32);
            self.w32(v6.wrapping_add(236), (v13 as u32));
            v15 = v14.wrapping_add((self.r32(v6.wrapping_add(192)) as i32));
            v16 = (self.r32(v6.wrapping_add(148)) as i32);
            self.w32(v6.wrapping_add(240), (v15 as u32));
            v17 = v16.wrapping_add((self.r32(v6.wrapping_add(196)) as i32));
            v18 = (self.r32(v6.wrapping_add(152)) as i32);
            self.w32(v6.wrapping_add(244), (v17 as u32));
            v19 = v18.wrapping_add((self.r32(v6.wrapping_add(200)) as i32));
            v20 = (self.r32(v6.wrapping_add(156)) as i32);
            self.w32(v6.wrapping_add(248), (v19 as u32));
            v21 = v20.wrapping_add((self.r32(v6.wrapping_add(204)) as i32));
            v22 = (self.r32(v6.wrapping_add(160)) as i32);
            self.w32(v6.wrapping_add(252), (v21 as u32));
            v23 = v22.wrapping_add((self.r32(v6.wrapping_add(208)) as i32));
            v24 = (self.r32(v6.wrapping_add(164)) as i32);
            self.w32(v6.wrapping_add(256), (v23 as u32));
            v25 = v24.wrapping_add((self.r32(v6.wrapping_add(212)) as i32));
            v26 = (self.r32(v6.wrapping_add(168)) as i32)
                .wrapping_add((self.r32(v6.wrapping_add(216)) as i32));
            self.w32(v6.wrapping_add(260), (v25 as u32));
            self.w32(v6.wrapping_add(264), (v26 as u32));
            v27 = (((0x66666667_i64.wrapping_mul(
                ((self.r32(
                    (v5.wrapping_add(4_i32.wrapping_mul(a3))
                        .wrapping_add(7816_i32) as u32),
                ) as i32) as i64),
            ) as u64)
                .wrapping_shr(32_i32 as u32) as i32)
                .wrapping_shr(2_i32 as u32) as u32);
            self.w32(
                v6.wrapping_add(260),
                (((v25 as u32).wrapping_sub((v27).wrapping_shr(31_i32 as u32).wrapping_add(v27))
                    as i32) as u32),
            );
        }
        self.w32((v4.wrapping_add(6800_i32) as u32), 0_u32);
        v28 = (v4.wrapping_add(256_i32) as u32);
        v29 = 5_i32;
        'l1: loop {
            v30 = (self.r32(v28) as i32)
                .wrapping_add((self.r32((v4.wrapping_add(6800_i32) as u32)) as i32));
            v28 = v28.wrapping_add(716);
            v29 = v29.wrapping_sub(1);
            self.w32((v4.wrapping_add(6800_i32) as u32), (v30 as u32));
            if !(v29 != 0) {
                break 'l1;
            }
        }
        v31 = cdiv_i32(200_i32.wrapping_mul(v30), 100_i32);
        self.w32((v4.wrapping_add(6800_i32) as u32), (v31 as u32));
        if (v31 > 999_i32) {
            self.w32((v4.wrapping_add(6800_i32) as u32), 999_u32);
        }
        return 0_i32;
    }

    /// `sub_10002970` (415 bytes).
    pub(crate) fn f_10002970(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v5 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32))
                        as i32);
                    bb = if (!(a2 != 0)) { 1 } else { 2 };
                }
                1 => {
                    v6 = (v5.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
                    bb = if (a4 != 0) { 3 } else { 4 };
                }
                2 => {
                    bb = if (a2 == 1_i32) { 38 } else { 39 };
                }
                3 => {
                    bb = if (a4 == 1_i32) { 5 } else { 7 };
                }
                4 => {
                    v7 = (self.r32(v6.wrapping_add(220)) as i32);
                    bb = if (v7 < (self.r32(v6.wrapping_add(268)) as i32)) {
                        12
                    } else {
                        13
                    };
                }
                5 => {
                    bb = if (!((self.r32(v6.wrapping_add(268)) as i32) != 0)) {
                        8
                    } else {
                        9
                    };
                }
                6 => {
                    bb = 4;
                }
                7 => {
                    bb = if (a4 == 2_i32) { 10 } else { 11 };
                }
                8 => {
                    self.w32(v6.wrapping_add(268), 1_u32);
                    bb = 9;
                }
                9 => {
                    bb = 6;
                }
                10 => {
                    self.w32(
                        v6.wrapping_add(268),
                        ((self.r32(v6.wrapping_add(220)) as i32) as u32),
                    );
                    bb = 11;
                }
                11 => {
                    bb = 6;
                }
                12 => {
                    self.w32(v6.wrapping_add(268), (v7 as u32));
                    bb = 13;
                }
                13 => {
                    v8 = v6.wrapping_add(272);
                    v9 = 15_i32;
                    bb = 14;
                }
                14 => {
                    bb = if (1_i32 != 0) { 15 } else { 16 };
                }
                15 => {
                    bb = if ((self.r32(v8.wrapping_sub(200)) as i32) == (1_i32).wrapping_neg()) {
                        17
                    } else {
                        19
                    };
                }
                16 => {
                    bb = 2;
                }
                17 => {
                    self.w32(v8, 0_u32);
                    bb = 18;
                }
                18 => {
                    v8 = v8.wrapping_add(4);
                    bb = if (!({
                        let t1 = v9.wrapping_sub(1);
                        v9 = t1;
                        t1
                    } != 0))
                    {
                        35
                    } else {
                        36
                    };
                }
                19 => {
                    bb = if (a5 != 0) { 20 } else { 22 };
                }
                20 => {
                    bb = if (a5 == 1_i32) { 23 } else { 25 };
                }
                21 => {
                    bb = 18;
                }
                22 => {
                    bb = if (!((self.r32(v8) as i32) != 0)) {
                        32
                    } else {
                        33
                    };
                }
                23 => {
                    bb = if ((self.r32(v8) as i32) != 3_i32) {
                        26
                    } else {
                        27
                    };
                }
                24 => {
                    bb = 21;
                }
                25 => {
                    bb = if (a5 == 2_i32) { 30 } else { 31 };
                }
                26 => {
                    bb = 28;
                }
                27 => {
                    bb = 24;
                }
                28 => {
                    self.w32(v8, 1_u32);
                    bb = 31;
                }
                29 => {
                    bb = 27;
                }
                30 => {
                    bb = 28;
                }
                31 => {
                    bb = 24;
                }
                32 => {
                    bb = 28;
                }
                33 => {
                    bb = 21;
                }
                34 => {
                    bb = 33;
                }
                35 => {
                    v10 = (self.r32(v6.wrapping_add(232)) as i32);
                    self.w32(
                        v6.wrapping_add(332),
                        ((self.r32(v6.wrapping_add(228)) as i32) as u32),
                    );
                    v11 = (self.r32(v6.wrapping_add(236)) as i32);
                    self.w32(v6.wrapping_add(336), (v10 as u32));
                    v12 = (self.r32(v6.wrapping_add(240)) as i32);
                    self.w32(v6.wrapping_add(340), (v11 as u32));
                    v13 = (self.r32(v6.wrapping_add(244)) as i32);
                    self.w32(v6.wrapping_add(344), (v12 as u32));
                    v14 = (self.r32(v6.wrapping_add(248)) as i32);
                    self.w32(v6.wrapping_add(348), (v13 as u32));
                    v15 = (self.r32(v6.wrapping_add(252)) as i32);
                    self.w32(v6.wrapping_add(352), (v14 as u32));
                    v16 = (self.r32(v6.wrapping_add(256)) as i32);
                    self.w32(v6.wrapping_add(356), (v15 as u32));
                    v17 = (self.r32(v6.wrapping_add(260)) as i32);
                    self.w32(v6.wrapping_add(360), (v16 as u32));
                    v18 = (self.r32(v6.wrapping_add(264)) as i32);
                    self.w32(v6.wrapping_add(364), (v17 as u32));
                    self.w32(v6.wrapping_add(368), (v18 as u32));
                    return 0_i32;
                }
                36 => {
                    bb = 14;
                }
                37 => {
                    bb = 36;
                }
                38 => {
                    v20 = (v5
                        .wrapping_add(156_i32.wrapping_mul(a3))
                        .wrapping_add(3600_i32) as u32);
                    v21 = (self.r32(
                        (v5.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3624_i32) as u32),
                    ) as i32);
                    self.w32(
                        v20.wrapping_add(68),
                        ((self.r32(
                            (v5.wrapping_add(156_i32.wrapping_mul(a3))
                                .wrapping_add(3620_i32) as u32),
                        ) as i32) as u32),
                    );
                    v22 = (self.r32(
                        (v5.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3628_i32) as u32),
                    ) as i32);
                    self.w32(v20.wrapping_add(72), (v21 as u32));
                    v23 = (self.r32(v20.wrapping_add(32)) as i32);
                    self.w32(v20.wrapping_add(76), (v22 as u32));
                    v24 = (self.r32(v20.wrapping_add(36)) as i32);
                    self.w32(v20.wrapping_add(80), (v23 as u32));
                    v25 = (self.r32(v20.wrapping_add(40)) as i32);
                    self.w32(v20.wrapping_add(84), (v24 as u32));
                    v26 = (self.r32(v20.wrapping_add(44)) as i32);
                    self.w32(v20.wrapping_add(88), (v25 as u32));
                    v27 = (self.r32(v20.wrapping_add(48)) as i32);
                    self.w32(v20.wrapping_add(92), (v26 as u32));
                    v28 = (self.r32(v20.wrapping_add(52)) as i32);
                    self.w32(v20.wrapping_add(96), (v27 as u32));
                    v29 = (self.r32(v20.wrapping_add(56)) as i32);
                    self.w32(v20.wrapping_add(100), (v28 as u32));
                    v30 = (self.r32(v20.wrapping_add(60)) as i32);
                    self.w32(v20.wrapping_add(104), (v29 as u32));
                    v31 = (self.r32(v20.wrapping_add(64)) as i32);
                    self.w32(v20.wrapping_add(108), (v30 as u32));
                    self.w32(v20.wrapping_add(112), (v31 as u32));
                    bb = 39;
                }
                39 => {
                    return 0_i32;
                }
                40 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10002B10` (423 bytes).
    pub(crate) fn f_10002b10(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: i32 = 0;
        let mut v18: u32 = 0;
        let mut v19: i32 = 0;
        let mut v21: u32 = 0;
        v3 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32);
        if (a2 != 0) {
            if (a2 == 1_i32) {
                v21 = (v3
                    .wrapping_add(156_i32.wrapping_mul(a3))
                    .wrapping_add(3600_i32) as u32);
                self.w32(v21.wrapping_add(116), 0_u32);
                self.w32(v21.wrapping_add(120), 0_u32);
                self.w32(v21.wrapping_add(124), 0_u32);
                self.w32(v21.wrapping_add(128), 0_u32);
                self.w32(v21.wrapping_add(132), ((1_i32).wrapping_neg() as u32));
                self.w32(v21.wrapping_add(136), 0_u32);
                self.w32(v21.wrapping_add(140), 0_u32);
                self.w32(v21.wrapping_add(144), 0_u32);
                self.w32(v21.wrapping_add(148), 0_u32);
                self.w32(v21.wrapping_add(152), 0_u32);
            }
            return 0_i32;
        } else {
            v4 = (v3.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
            v5 = 15_i32;
            v6 = (self.r32(v4.wrapping_add(396)) as i32);
            self.w32(v4.wrapping_add(372), 0_u32);
            self.w32(v4.wrapping_add(380), (v6 as u32));
            self.w32(
                v4.wrapping_add(384),
                ((self.r32(v4.wrapping_add(400)) as i32) as u32),
            );
            self.w32(
                v4.wrapping_add(388),
                ((self.r32(v4.wrapping_add(404)) as i32) as u32),
            );
            self.w32(v4.wrapping_add(376), 0_u32);
            self.w32(v4.wrapping_add(392), 0_u32);
            v7 = v4.wrapping_add(468);
            'l1: loop {
                self.w32(v7.wrapping_sub(60), 0_u32);
                self.w32(
                    {
                        let t1 = v7;
                        v7 = t1.wrapping_add(4);
                        t1
                    },
                    0_u32,
                );
                v5 = v5.wrapping_sub(1);
                if !(v5 != 0) {
                    break 'l1;
                }
            }
            v8 = v4.wrapping_add(536);
            v9 = 2_i32;
            'l2: loop {
                self.w32(v8.wrapping_sub(8), 0_u32);
                self.w32(
                    {
                        let t2 = v8;
                        v8 = t2.wrapping_add(4);
                        t2
                    },
                    0_u32,
                );
                v9 = v9.wrapping_sub(1);
                if !(v9 != 0) {
                    break 'l2;
                }
            }
            self.w32(v4.wrapping_add(544), 0_u32);
            self.w32(v4.wrapping_add(548), 0_u32);
            self.w32(v4.wrapping_add(552), 0_u32);
            self.w32(v4.wrapping_add(556), 0_u32);
            self.w32(v4.wrapping_add(560), 0_u32);
            self.w32(v4.wrapping_add(564), 0_u32);
            v10 = v4.wrapping_add(576);
            v11 = 2_i32;
            'l3: loop {
                self.w32(v10.wrapping_sub(8), 0_u32);
                self.w32(v10, 0_u32);
                self.w32(v10.wrapping_add(8), 0_u32);
                v10 = v10.wrapping_add(4);
                v11 = v11.wrapping_sub(1);
                if !(v11 != 0) {
                    break 'l3;
                }
            }
            v12 = v4.wrapping_add(604);
            v13 = 3_i32;
            'l4: loop {
                self.w32(v12.wrapping_sub(12), 0_u32);
                self.w32(v12, 0_u32);
                self.w32(v12.wrapping_add(12), 0_u32);
                v12 = v12.wrapping_add(4);
                v13 = v13.wrapping_sub(1);
                if !(v13 != 0) {
                    break 'l4;
                }
            }
            v14 = v4.wrapping_add(636);
            v15 = 2_i32;
            'l5: loop {
                self.w32(v14.wrapping_sub(8), 0_u32);
                self.w32(
                    {
                        let t3 = v14;
                        v14 = t3.wrapping_add(4);
                        t3
                    },
                    0_u32,
                );
                v15 = v15.wrapping_sub(1);
                if !(v15 != 0) {
                    break 'l5;
                }
            }
            v16 = v4.wrapping_add(656);
            v17 = 3_i32;
            'l6: loop {
                self.w32(v16.wrapping_sub(12), 0_u32);
                self.w32(
                    {
                        let t4 = v16;
                        v16 = t4.wrapping_add(4);
                        t4
                    },
                    0_u32,
                );
                v17 = v17.wrapping_sub(1);
                if !(v17 != 0) {
                    break 'l6;
                }
            }
            v18 = v4.wrapping_add(676);
            v19 = 2_i32;
            'l7: loop {
                self.w32(v18.wrapping_sub(8), 0_u32);
                self.w32(
                    {
                        let t5 = v18;
                        v18 = t5.wrapping_add(4);
                        t5
                    },
                    0_u32,
                );
                v19 = v19.wrapping_sub(1);
                if !(v19 != 0) {
                    break 'l7;
                }
            }
            self.w32(v4.wrapping_add(684), 0_u32);
            self.w32(v4.wrapping_add(688), 0_u32);
            self.w32(v4.wrapping_add(696), 0_u32);
            self.w32(v4.wrapping_add(700), 0_u32);
            self.w32(v4.wrapping_add(704), 0_u32);
            self.w32(v4.wrapping_add(708), 0_u32);
            self.w32(v4.wrapping_add(712), 0_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10002CC0` (87 bytes).
    pub(crate) fn f_10002cc0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        v1 = 4_i32;
        v2 = ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32)
            .wrapping_add(6000_i32) as u32);
        'l1: loop {
            self.w32(v2, 0_u32);
            self.w32(v2.wrapping_add(4), 0_u32);
            self.w32(v2.wrapping_add(8), 0_u32);
            self.w32(v2.wrapping_add(12), 0_u32);
            self.w32(v2.wrapping_add(16), 0_u32);
            self.w32(v2.wrapping_add(20), 0_u32);
            self.w32(v2.wrapping_add(24), 0_u32);
            self.w32(v2.wrapping_add(28), 0_u32);
            self.w32(v2.wrapping_add(32), 0_u32);
            self.w32(v2.wrapping_add(36), 0_u32);
            self.w32(v2.wrapping_add(40), 0_u32);
            self.w32(v2.wrapping_add(44), 0_u32);
            self.w32(v2.wrapping_add(48), 0_u32);
            self.w32(v2.wrapping_add(52), 0_u32);
            self.w32(v2.wrapping_add(56), 0_u32);
            self.w32(v2.wrapping_add(60), 0_u32);
            self.w32(v2.wrapping_add(64), 0_u32);
            self.w32(v2.wrapping_add(68), 0_u32);
            self.w32(v2.wrapping_add(72), 0_u32);
            self.w32(v2.wrapping_add(76), 0_u32);
            v2 = v2.wrapping_add(80);
            v1 = v1.wrapping_sub(1);
            if !(v1 != 0) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10002D20` (879 bytes).
    pub(crate) fn f_10002d20(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: i32 = 0;
        let mut v32: i32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: i32 = 0;
        let mut v43: i8 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = (self.r32(self.r32(this)) as i32);
                    v4 = self.r32(this.wrapping_add(4));
                    v5 = (self.r32((v3.wrapping_add(28_i32) as u32)) as i32);
                    v6 = ((self.r32((v3.wrapping_add(12_i32) as u32)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(a2)) as u32);
                    v7 = self.r32(this.wrapping_add(8));
                    v8 = (v5.wrapping_add(
                        16_i32.wrapping_mul(5_i32.wrapping_mul(a2).wrapping_add(375_i32)),
                    ) as u32);
                    bb = if ((self.r32(v6.wrapping_add(60)) as i32).wrapping_sub(a3) > 0_i32) {
                        1
                    } else {
                        3
                    };
                }
                1 => {
                    self.w32((v5.wrapping_add(6320_i32) as u32), 0_u32);
                    self.w32(
                        v6.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(60)) as i32).wrapping_sub(a3) as u32),
                    );
                    bb = 2;
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    self.w32(
                        (v5.wrapping_add(6320_i32) as u32),
                        (a3.wrapping_sub((self.r32(v6.wrapping_add(60)) as i32)) as u32),
                    );
                    v9 = (self.r32(v6.wrapping_add(8)) as i32);
                    v43 = 0_i8;
                    bb = if (v9 >= 99_i32) { 4 } else { 6 };
                }
                4 => {
                    v43 = 1_i8;
                    bb = 5;
                }
                5 => {
                    v10 = (self.r32(v6.wrapping_add(56)) as i32);
                    bb = if (v10 < 99_i32) { 7 } else { 8 };
                }
                6 => {
                    self.w32(v6.wrapping_add(8), (v9.wrapping_add(1_i32) as u32));
                    self.w32(v8, 1_u32);
                    bb = 5;
                }
                7 => {
                    self.w32(v6.wrapping_add(56), (v10.wrapping_add(1_i32) as u32));
                    bb = 8;
                }
                8 => {
                    v11 = (self.r32(v6.wrapping_add(8)) as i32);
                    self.w32(
                        v6.wrapping_add(60),
                        ((self.r32(v4.wrapping_add(
                            (2_i32.wrapping_mul(v11).wrapping_add(1_i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v12 = (self
                        .r32(v4.wrapping_add((2_i32.wrapping_mul(v11) as u32).wrapping_mul(4)))
                        as i32);
                    bb = if (v12 != (1_i32).wrapping_neg()) {
                        9
                    } else {
                        10
                    };
                }
                9 => {
                    self.w32(v8, 1_u32);
                    self.w32(
                        v8.wrapping_add(8),
                        (((self.r32(v8.wrapping_add(8)) as i32) | (1_i32).wrapping_shl(v12 as u32))
                            as u32),
                    );
                    bb = 10;
                }
                10 => {
                    let t1 = (self.r32(v6.wrapping_add(12)) as i32);
                    bb = match t1 {
                        0_i32 => 12,
                        4_i32 => 13,
                        7_i32 => 14,
                        1_i32 => 15,
                        5_i32 => 16,
                        8_i32 => 17,
                        2_i32 => 18,
                        3_i32 => 19,
                        6_i32 => 20,
                        9_i32 => 21,
                        _ => 22,
                    };
                }
                11 => {
                    bb = 40;
                }
                12 => {
                    bb = 13;
                }
                13 => {
                    bb = 14;
                }
                14 => {
                    let t2 = (self.r32(v6.wrapping_add(56)) as i32);
                    bb = match t2 {
                        2_i32 => 24,
                        5_i32 => 25,
                        8_i32 => 26,
                        11_i32 => 27,
                        14_i32 => 28,
                        _ => 29,
                    };
                }
                15 => {
                    bb = 16;
                }
                16 => {
                    bb = 17;
                }
                17 => {
                    let t3 = (self.r32(v6.wrapping_add(56)) as i32);
                    bb = match t3 {
                        2_i32 => 43,
                        5_i32 => 44,
                        7_i32 => 45,
                        10_i32 => 46,
                        12_i32 => 47,
                        _ => 48,
                    };
                }
                18 => {
                    bb = 19;
                }
                19 => {
                    bb = 20;
                }
                20 => {
                    let t4 = (self.r32(v6.wrapping_add(56)) as i32);
                    bb = match t4 {
                        2_i32 => 56,
                        4_i32 => 57,
                        6_i32 => 58,
                        8_i32 => 59,
                        10_i32 => 60,
                        _ => 61,
                    };
                }
                21 => {
                    let t5 = (self.r32(v6.wrapping_add(56)) as i32);
                    bb = match t5 {
                        2_i32 => 69,
                        3_i32 => 70,
                        5_i32 => 71,
                        7_i32 => 72,
                        10_i32 => 73,
                        _ => 74,
                    };
                }
                22 => {
                    bb = 11;
                }
                23 => {
                    bb = 15;
                }
                24 => {
                    bb = 30;
                }
                25 => {
                    bb = 32;
                }
                26 => {
                    bb = 34;
                }
                27 => {
                    bb = 36;
                }
                28 => {
                    bb = 38;
                }
                29 => {
                    bb = 40;
                }
                30 => {
                    v13 = 10_i32;
                    bb = 75;
                }
                31 => {
                    bb = 25;
                }
                32 => {
                    v13 = 11_i32;
                    bb = 75;
                }
                33 => {
                    bb = 26;
                }
                34 => {
                    v13 = 12_i32;
                    bb = 75;
                }
                35 => {
                    bb = 27;
                }
                36 => {
                    v13 = 13_i32;
                    bb = 75;
                }
                37 => {
                    bb = 28;
                }
                38 => {
                    v13 = 14_i32;
                    bb = 75;
                }
                39 => {
                    bb = 29;
                }
                40 => {
                    bb = if ((v43 as i32) != 1_i32) { 86 } else { 87 };
                }
                41 => {
                    bb = 23;
                }
                42 => {
                    bb = 18;
                }
                43 => {
                    bb = 30;
                }
                44 => {
                    bb = 32;
                }
                45 => {
                    bb = 34;
                }
                46 => {
                    bb = 36;
                }
                47 => {
                    bb = 38;
                }
                48 => {
                    bb = 40;
                }
                49 => {
                    bb = 44;
                }
                50 => {
                    bb = 45;
                }
                51 => {
                    bb = 46;
                }
                52 => {
                    bb = 47;
                }
                53 => {
                    bb = 48;
                }
                54 => {
                    bb = 42;
                }
                55 => {
                    bb = 21;
                }
                56 => {
                    bb = 30;
                }
                57 => {
                    bb = 32;
                }
                58 => {
                    bb = 34;
                }
                59 => {
                    bb = 36;
                }
                60 => {
                    bb = 38;
                }
                61 => {
                    bb = 40;
                }
                62 => {
                    bb = 57;
                }
                63 => {
                    bb = 58;
                }
                64 => {
                    bb = 59;
                }
                65 => {
                    bb = 60;
                }
                66 => {
                    bb = 61;
                }
                67 => {
                    bb = 55;
                }
                68 => {
                    bb = 11;
                }
                69 => {
                    bb = 30;
                }
                70 => {
                    bb = 32;
                }
                71 => {
                    bb = 34;
                }
                72 => {
                    bb = 36;
                }
                73 => {
                    bb = 38;
                }
                74 => {
                    bb = 40;
                }
                75 => {
                    bb = if ((self
                        .r32(v6.wrapping_add((v13.wrapping_add(18_i32) as u32).wrapping_mul(4)))
                        as i32)
                        == (1_i32).wrapping_neg())
                    {
                        80
                    } else {
                        81
                    };
                }
                76 => {
                    bb = 70;
                }
                77 => {
                    bb = 71;
                }
                78 => {
                    bb = 72;
                }
                79 => {
                    bb = 73;
                }
                80 => {
                    self.w32(v8, 1_u32);
                    self.w32(
                        v8.wrapping_add(8),
                        (((self.r32(v8.wrapping_add(8)) as i32) | (1_i32).wrapping_shl(v13 as u32))
                            as u32),
                    );
                    bb = 81;
                }
                81 => {
                    bb = 68;
                }
                82 => {
                    bb = 74;
                }
                83 => {
                    bb = 68;
                }
                84 => {
                    bb = 22;
                }
                85 => {
                    bb = 11;
                }
                86 => {
                    v14 = (self.r32(v6.wrapping_add(8)) as i32);
                    bb = if (v14 > 10_i32) { 88 } else { 90 };
                }
                87 => {
                    let t6 = (self.r32(v6.wrapping_add(8)) as i32);
                    bb = match t6 {
                        5_i32 => 95,
                        10_i32 => 96,
                        15_i32 => 97,
                        20_i32 => 98,
                        25_i32 => 99,
                        30_i32 => 100,
                        35_i32 => 101,
                        40_i32 => 102,
                        _ => 103,
                    };
                }
                88 => {
                    bb = if (v14 > 40_i32) { 91 } else { 93 };
                }
                89 => {
                    bb = 87;
                }
                90 => {
                    v15 = (self.r32(v8.wrapping_add(16)) as i32).wrapping_add(2_i32);
                    self.w32(
                        v8.wrapping_add(12),
                        ((self.r32(v8.wrapping_add(12)) as i32).wrapping_add(6_i32) as u32),
                    );
                    v16 = (self.r32(v8.wrapping_add(20)) as i32);
                    self.w32(v8.wrapping_add(16), (v15 as u32));
                    v17 = (self.r32(v8.wrapping_add(24)) as i32).wrapping_add(2_i32);
                    self.w32(v8.wrapping_add(20), (v16.wrapping_add(2_i32) as u32));
                    v18 = (self.r32(v8.wrapping_add(28)) as i32);
                    self.w32(v8.wrapping_add(24), (v17 as u32));
                    v19 = (self.r32(v8.wrapping_add(32)) as i32);
                    self.w32(v8.wrapping_add(28), (v18.wrapping_add(2_i32) as u32));
                    self.w32(v8.wrapping_add(32), (v19.wrapping_add(2_i32) as u32));
                    bb = 89;
                }
                91 => {
                    v27 = (self.r32(v8.wrapping_add(20)) as i32);
                    v28 = (self.r32(v8.wrapping_add(16)) as i32).wrapping_add(1_i32);
                    self.w32(
                        v8.wrapping_add(12),
                        ((self.r32(v8.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    v29 = (self.r32(v8.wrapping_add(24)) as i32);
                    self.w32(v8.wrapping_add(16), (v28 as u32));
                    v30 = (self.r32(v8.wrapping_add(28)) as i32);
                    self.w32(v8.wrapping_add(20), (v27.wrapping_add(1_i32) as u32));
                    v31 = (self.r32(v8.wrapping_add(32)) as i32).wrapping_add(1_i32);
                    self.w32(v8.wrapping_add(24), (v29.wrapping_add(1_i32) as u32));
                    self.w32(v8.wrapping_add(28), (v30.wrapping_add(1_i32) as u32));
                    self.w32(v8.wrapping_add(32), (v31 as u32));
                    bb = 92;
                }
                92 => {
                    bb = 89;
                }
                93 => {
                    v20 = (self.r32(v8.wrapping_add(16)) as i32);
                    self.w32(
                        v8.wrapping_add(12),
                        ((self.r32(v8.wrapping_add(12)) as i32).wrapping_add(
                            (self.r32(
                                v7.wrapping_add(
                                    (9_i32.wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                        as u32)
                                        .wrapping_mul(4),
                                ),
                            ) as i32),
                        ) as u32),
                    );
                    v21 = (self.r32(v8.wrapping_add(20)) as i32);
                    self.w32(
                        v8.wrapping_add(16),
                        ((self.r32(
                            v7.wrapping_add(
                                (9_i32
                                    .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                    .wrapping_add(1_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add(v20) as u32),
                    );
                    v22 = (self.r32(v8.wrapping_add(24)) as i32);
                    self.w32(
                        v8.wrapping_add(20),
                        ((self.r32(
                            v7.wrapping_add(
                                (9_i32
                                    .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                    .wrapping_add(2_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add(v21) as u32),
                    );
                    v23 = (self.r32(v8.wrapping_add(28)) as i32);
                    self.w32(
                        v8.wrapping_add(24),
                        ((self.r32(
                            v7.wrapping_add(
                                (9_i32
                                    .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                    .wrapping_add(3_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add(v22) as u32),
                    );
                    v24 = (self.r32(v8.wrapping_add(32)) as i32);
                    self.w32(
                        v8.wrapping_add(28),
                        ((self.r32(
                            v7.wrapping_add(
                                (9_i32
                                    .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                    .wrapping_add(4_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add(v23) as u32),
                    );
                    v25 = (self.r32(v8.wrapping_add(36)) as i32);
                    self.w32(
                        v8.wrapping_add(32),
                        ((self.r32(
                            v7.wrapping_add(
                                (9_i32
                                    .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                    .wrapping_add(5_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add(v24) as u32),
                    );
                    v26 = (self.r32(v8.wrapping_add(40)) as i32);
                    self.w32(
                        v8.wrapping_add(36),
                        ((self.r32(
                            v7.wrapping_add(
                                (9_i32
                                    .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                    .wrapping_add(6_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add(v25) as u32),
                    );
                    self.w32(
                        v8.wrapping_add(40),
                        ((self.r32(
                            v7.wrapping_add(
                                (9_i32
                                    .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                    .wrapping_add(7_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add(v26) as u32),
                    );
                    bb = 92;
                }
                94 => {
                    v32 = (self.r32(v6.wrapping_add(8)) as i32);
                    bb = if (v32 > 10_i32) { 106 } else { 108 };
                }
                95 => {
                    bb = 96;
                }
                96 => {
                    bb = 97;
                }
                97 => {
                    bb = 98;
                }
                98 => {
                    bb = 99;
                }
                99 => {
                    bb = 100;
                }
                100 => {
                    bb = 101;
                }
                101 => {
                    bb = 102;
                }
                102 => {
                    self.w32(
                        v8.wrapping_add(36),
                        ((self.r32(v8.wrapping_add(36)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 94;
                }
                103 => {
                    bb = 94;
                }
                104 => {
                    bb = 103;
                }
                105 => {
                    bb = 94;
                }
                106 => {
                    bb = if (v32 <= 40_i32) { 109 } else { 110 };
                }
                107 => {
                    v33 = (self.r32(v8.wrapping_add(12)) as i32);
                    bb = if (v33 >= 0_i32) { 111 } else { 113 };
                }
                108 => {
                    self.w32(
                        v8.wrapping_add(76),
                        ((self.r32(v8.wrapping_add(76)) as i32).wrapping_add(3_i32) as u32),
                    );
                    bb = 107;
                }
                109 => {
                    self.w32(
                        v8.wrapping_add(76),
                        ((self.r32(v8.wrapping_add(76)) as i32).wrapping_add(
                            (self.r32(
                                v7.wrapping_add(
                                    (9_i32
                                        .wrapping_mul((self.r32(v6.wrapping_add(12)) as i32))
                                        .wrapping_add(8_i32)
                                        as u32)
                                        .wrapping_mul(4),
                                ),
                            ) as i32),
                        ) as u32),
                    );
                    bb = 110;
                }
                110 => {
                    bb = 107;
                }
                111 => {
                    bb = if (v33 > 99_i32) { 114 } else { 115 };
                }
                112 => {
                    v34 = (self.r32(v8.wrapping_add(16)) as i32);
                    bb = if (v34 >= 0_i32) { 116 } else { 118 };
                }
                113 => {
                    self.w32(v8.wrapping_add(12), 0_u32);
                    bb = 112;
                }
                114 => {
                    self.w32(v8.wrapping_add(12), 99_u32);
                    bb = 115;
                }
                115 => {
                    bb = 112;
                }
                116 => {
                    bb = if (v34 > 99_i32) { 119 } else { 120 };
                }
                117 => {
                    v35 = (self.r32(v8.wrapping_add(20)) as i32);
                    bb = if (v35 >= 0_i32) { 121 } else { 123 };
                }
                118 => {
                    self.w32(v8.wrapping_add(16), 0_u32);
                    bb = 117;
                }
                119 => {
                    self.w32(v8.wrapping_add(16), 99_u32);
                    bb = 120;
                }
                120 => {
                    bb = 117;
                }
                121 => {
                    bb = if (v35 > 99_i32) { 124 } else { 125 };
                }
                122 => {
                    v36 = (self.r32(v8.wrapping_add(24)) as i32);
                    bb = if (v36 >= 0_i32) { 126 } else { 128 };
                }
                123 => {
                    self.w32(v8.wrapping_add(20), 0_u32);
                    bb = 122;
                }
                124 => {
                    self.w32(v8.wrapping_add(20), 99_u32);
                    bb = 125;
                }
                125 => {
                    bb = 122;
                }
                126 => {
                    bb = if (v36 > 99_i32) { 129 } else { 130 };
                }
                127 => {
                    v37 = (self.r32(v8.wrapping_add(28)) as i32);
                    bb = if (v37 >= 0_i32) { 131 } else { 133 };
                }
                128 => {
                    self.w32(v8.wrapping_add(24), 0_u32);
                    bb = 127;
                }
                129 => {
                    self.w32(v8.wrapping_add(24), 99_u32);
                    bb = 130;
                }
                130 => {
                    bb = 127;
                }
                131 => {
                    bb = if (v37 > 99_i32) { 134 } else { 135 };
                }
                132 => {
                    v38 = (self.r32(v8.wrapping_add(32)) as i32);
                    bb = if (v38 >= 0_i32) { 136 } else { 138 };
                }
                133 => {
                    self.w32(v8.wrapping_add(28), 0_u32);
                    bb = 132;
                }
                134 => {
                    self.w32(v8.wrapping_add(28), 99_u32);
                    bb = 135;
                }
                135 => {
                    bb = 132;
                }
                136 => {
                    bb = if (v38 > 99_i32) { 139 } else { 140 };
                }
                137 => {
                    v39 = (self.r32(v8.wrapping_add(36)) as i32);
                    bb = if (v39 >= 0_i32) { 141 } else { 143 };
                }
                138 => {
                    self.w32(v8.wrapping_add(32), 0_u32);
                    bb = 137;
                }
                139 => {
                    self.w32(v8.wrapping_add(32), 99_u32);
                    bb = 140;
                }
                140 => {
                    bb = 137;
                }
                141 => {
                    bb = if (v39 > 99_i32) { 144 } else { 145 };
                }
                142 => {
                    v40 = (self.r32(v8.wrapping_add(40)) as i32);
                    bb = if (v40 >= 0_i32) { 146 } else { 148 };
                }
                143 => {
                    self.w32(v8.wrapping_add(36), 0_u32);
                    bb = 142;
                }
                144 => {
                    self.w32(v8.wrapping_add(36), 99_u32);
                    bb = 145;
                }
                145 => {
                    bb = 142;
                }
                146 => {
                    bb = if (v40 > 99_i32) { 149 } else { 150 };
                }
                147 => {
                    v41 = (self.r32(v8.wrapping_add(76)) as i32);
                    bb = if (v41 < 0_i32) { 151 } else { 152 };
                }
                148 => {
                    self.w32(v8.wrapping_add(40), 0_u32);
                    bb = 147;
                }
                149 => {
                    self.w32(v8.wrapping_add(40), 99_u32);
                    bb = 150;
                }
                150 => {
                    bb = 147;
                }
                151 => {
                    self.w32(v8.wrapping_add(76), 0_u32);
                    return 0_i32;
                }
                152 => {
                    bb = if (v41 > 99_i32) { 154 } else { 155 };
                }
                153 => {
                    bb = 152;
                }
                154 => {
                    self.w32(v8.wrapping_add(76), 99_u32);
                    return 0_i32;
                }
                155 => {
                    bb = 2;
                }
                156 => {
                    bb = 155;
                }
                157 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10003180` (435 bytes).
    pub(crate) fn f_10003180(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        v3 = (self.r32(self.r32(this)) as i32);
        v4 = (self.r32((v3.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32((v3.wrapping_add(28_i32) as u32)) as i32);
        v6 = self.r32(this.wrapping_add(12));
        v7 = a3;
        v8 = (v5.wrapping_add(16_i32.wrapping_mul(5_i32.wrapping_mul(a2).wrapping_add(375_i32)))
            as u32);
        v9 = (v4.wrapping_add(716_i32.wrapping_mul(a2)) as u32);
        if (a3 != 0) {
            if (a3 == 1_i32) {
                self.w32(v8.wrapping_add(4), 0_u32);
                v12 = 0_i32;
                v13 = v9.wrapping_add(16);
                'l1: loop {
                    if ((((((((((self.r32(v6) as i32)
                        <= (self.r32(v9.wrapping_add(68)) as i32)
                            .wrapping_add((self.r32(v8.wrapping_add(12)) as i32))
                            .wrapping_add((self.r32(v8.wrapping_add(44)) as i32)))
                        && ((self.r32(v6.wrapping_add(4)) as i32)
                            <= (self.r32(v8.wrapping_add(16)) as i32)
                                .wrapping_add((self.r32(v8.wrapping_add(48)) as i32))
                                .wrapping_add((self.r32(v9.wrapping_add(132)) as i32))))
                        && ((self.r32(v6.wrapping_add(8)) as i32)
                            <= (self.r32(v8.wrapping_add(20)) as i32)
                                .wrapping_add((self.r32(v8.wrapping_add(52)) as i32))
                                .wrapping_add((self.r32(v9.wrapping_add(140)) as i32))))
                        && ((self.r32(v6.wrapping_add(12)) as i32)
                            <= (self.r32(v8.wrapping_add(24)) as i32)
                                .wrapping_add((self.r32(v8.wrapping_add(56)) as i32))
                                .wrapping_add((self.r32(v9.wrapping_add(144)) as i32))))
                        && ((self.r32(v6.wrapping_add(16)) as i32)
                            <= (self.r32(v8.wrapping_add(28)) as i32)
                                .wrapping_add((self.r32(v8.wrapping_add(60)) as i32))
                                .wrapping_add((self.r32(v9.wrapping_add(152)) as i32))))
                        && ((self.r32(v6.wrapping_add(20)) as i32)
                            <= (self.r32(v8.wrapping_add(32)) as i32)
                                .wrapping_add((self.r32(v8.wrapping_add(64)) as i32))
                                .wrapping_add((self.r32(v9.wrapping_add(160)) as i32))))
                        && ((self.r32(v6.wrapping_add(24)) as i32)
                            <= (self.r32(v8.wrapping_add(36)) as i32)
                                .wrapping_add((self.r32(v8.wrapping_add(68)) as i32))
                                .wrapping_add((self.r32(v9.wrapping_add(164)) as i32))))
                        && ((self.r32(v6.wrapping_add(28)) as i32)
                            <= (self.r32(v8.wrapping_add(40)) as i32)
                                .wrapping_add((self.r32(v8.wrapping_add(72)) as i32))
                                .wrapping_add((self.r32(v9.wrapping_add(168)) as i32))))
                        && (!((self.r32(v13) as i32) != 0)))
                    {
                        self.w32(v13, 1_u32);
                        self.w32(v8, 1_u32);
                        self.w32(
                            v8.wrapping_add(4),
                            (((self.r32(v8.wrapping_add(4)) as i32)
                                | (1_i32).wrapping_shl(v12 as u32))
                                as u32),
                        );
                    }
                    v6 = v6.wrapping_add(32);
                    v12 = v12.wrapping_add(1);
                    v13 = v13.wrapping_add(4);
                    if !(v12 < 10_i32) {
                        break 'l1;
                    }
                }
            }
            return 0_i32;
        } else {
            self.w32(v8.wrapping_add(4), 0_u32);
            v10 = v9.wrapping_add(16);
            'l2: loop {
                if ((((((((((self.r32(v6) as i32)
                    <= (self.r32(v9.wrapping_add(68)) as i32)
                        .wrapping_add((self.r32(v8.wrapping_add(12)) as i32)))
                    && ((self.r32(v6.wrapping_add(4)) as i32)
                        <= (self.r32(v8.wrapping_add(16)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(132)) as i32))))
                    && ((self.r32(v6.wrapping_add(8)) as i32)
                        <= (self.r32(v8.wrapping_add(20)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(140)) as i32))))
                    && ((self.r32(v6.wrapping_add(12)) as i32)
                        <= (self.r32(v8.wrapping_add(24)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(144)) as i32))))
                    && ((self.r32(v6.wrapping_add(16)) as i32)
                        <= (self.r32(v8.wrapping_add(28)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(152)) as i32))))
                    && ((self.r32(v6.wrapping_add(20)) as i32)
                        <= (self.r32(v8.wrapping_add(32)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(160)) as i32))))
                    && ((self.r32(v6.wrapping_add(24)) as i32)
                        <= (self.r32(v8.wrapping_add(36)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(164)) as i32))))
                    && ((self.r32(v6.wrapping_add(28)) as i32)
                        <= (self.r32(v8.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(168)) as i32))))
                    && (!((self.r32(v10) as i32) != 0)))
                {
                    self.w32(v10, 1_u32);
                    self.w32(v8, 1_u32);
                    self.w32(
                        v8.wrapping_add(4),
                        (((self.r32(v8.wrapping_add(4)) as i32) | (1_i32).wrapping_shl(v7 as u32))
                            as u32),
                    );
                }
                v6 = v6.wrapping_add(32);
                v7 = v7.wrapping_add(1);
                v10 = v10.wrapping_add(4);
                if !(v7 < 10_i32) {
                    break 'l2;
                }
            }
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10003340` (519 bytes).
    pub(crate) fn f_10003340(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        v4 = (self.r32(self.r32(this)) as i32);
        v5 = (self.r32((v4.wrapping_add(12_i32) as u32)) as i32);
        v6 = (self.r32((v4.wrapping_add(28_i32) as u32)) as i32);
        v7 = a3;
        v8 = (v5.wrapping_add(716_i32.wrapping_mul(a2)) as u32);
        v9 = (v6.wrapping_add(16_i32.wrapping_mul(5_i32.wrapping_mul(a2).wrapping_add(375_i32)))
            as u32);
        if (a3 != 0) {
            if (a3 == 1_i32) {
                self.w32(
                    v8.wrapping_add(68),
                    ((self.r32(v8.wrapping_add(68)) as i32).wrapping_add(
                        (self.r32(v9.wrapping_add(44)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(12)) as i32)),
                    ) as u32),
                );
                v15 = (self.r32(v9.wrapping_add(16)) as i32)
                    .wrapping_add((self.r32(v9.wrapping_add(48)) as i32))
                    .wrapping_add((self.r32(v8.wrapping_add(132)) as i32));
                self.w32(v8.wrapping_add(132), (v15 as u32));
                v16 = (self.r32(v8.wrapping_add(140)) as i32);
                self.w32(v8.wrapping_add(136), (v15 as u32));
                self.w32(
                    v8.wrapping_add(140),
                    ((self.r32(v9.wrapping_add(20)) as i32)
                        .wrapping_add((self.r32(v9.wrapping_add(52)) as i32))
                        .wrapping_add(v16) as u32),
                );
                self.w32(
                    v8.wrapping_add(144),
                    ((self.r32(v8.wrapping_add(144)) as i32).wrapping_add(
                        (self.r32(v9.wrapping_add(24)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(56)) as i32)),
                    ) as u32),
                );
                self.w32(
                    v8.wrapping_add(152),
                    ((self.r32(v8.wrapping_add(152)) as i32).wrapping_add(
                        (self.r32(v9.wrapping_add(28)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(60)) as i32)),
                    ) as u32),
                );
                self.w32(
                    v8.wrapping_add(160),
                    ((self.r32(v8.wrapping_add(160)) as i32).wrapping_add(
                        (self.r32(v9.wrapping_add(32)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(64)) as i32)),
                    ) as u32),
                );
                self.w32(
                    v8.wrapping_add(164),
                    ((self.r32(v8.wrapping_add(164)) as i32).wrapping_add(
                        (self.r32(v9.wrapping_add(36)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(68)) as i32)),
                    ) as u32),
                );
                self.w32(
                    v8.wrapping_add(168),
                    ((self.r32(v8.wrapping_add(168)) as i32).wrapping_add(
                        (self.r32(v9.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(72)) as i32)),
                    ) as u32),
                );
                self.w32(
                    v8.wrapping_add(268),
                    ((self.r32(v8.wrapping_add(268)) as i32).wrapping_add(
                        (self.r32(v9.wrapping_add(44)) as i32)
                            .wrapping_add((self.r32(v9.wrapping_add(12)) as i32)),
                    ) as u32),
                );
                let _ = self.f_100027d0(this, 0_i32, a2);
                let _ = self.f_10002970(this, 0_i32, a2, 0_i32, 0_i32);
            }
            return 0_i32;
        } else {
            v10 = a2;
            v11 = v8.wrapping_add(72);
            'l1: loop {
                if (((1_i32).wrapping_shl(v7 as u32) & (self.r32(v9.wrapping_add(8)) as i32))
                    != 0_i32)
                {
                    's2: {
                        'b2_4: {
                            'b2_3: {
                                'b2_2: {
                                    'b2_1: {
                                        'b2_0: {
                                            match a2 {
                                                0_i32 => break 'b2_0,
                                                1_i32 => break 'b2_1,
                                                2_i32 => break 'b2_2,
                                                3_i32 => break 'b2_3,
                                                _ => break 'b2_4,
                                            }
                                        }
                                        v10 = 0_i32;
                                        break 's2;
                                    }
                                    v10 = 10_i32;
                                    break 's2;
                                }
                                v10 = 20_i32;
                                break 's2;
                            }
                            v10 = 30_i32;
                            break 's2;
                        }
                        break 's2;
                    }
                    self.w32(v11, (v10.wrapping_add(v7) as u32));
                }
                v7 = v7.wrapping_add(1);
                v11 = v11.wrapping_add(4);
                if !(v7 < 10_i32) {
                    break 'l1;
                }
            }
            v12 = 10_i32;
            v13 = v8.wrapping_add(112);
            'l3: loop {
                if (((1_i32).wrapping_shl(v12 as u32) & (self.r32(v9.wrapping_add(8)) as i32))
                    != 0_i32)
                {
                    's4: {
                        'b4_10: {
                            'b4_9: {
                                'b4_8: {
                                    'b4_7: {
                                        'b4_6: {
                                            'b4_5: {
                                                'b4_4: {
                                                    'b4_3: {
                                                        'b4_2: {
                                                            'b4_1: {
                                                                'b4_0: {
                                                                    match (self
                                                                        .r32(v8.wrapping_add(12))
                                                                        as i32)
                                                                    {
                                                                        0_i32 => break 'b4_0,
                                                                        1_i32 => break 'b4_1,
                                                                        2_i32 => break 'b4_2,
                                                                        3_i32 => break 'b4_3,
                                                                        4_i32 => break 'b4_4,
                                                                        5_i32 => break 'b4_5,
                                                                        6_i32 => break 'b4_6,
                                                                        7_i32 => break 'b4_7,
                                                                        8_i32 => break 'b4_8,
                                                                        9_i32 => break 'b4_9,
                                                                        _ => break 'b4_10,
                                                                    }
                                                                }
                                                                v10 = 50_i32;
                                                                break 's4;
                                                            }
                                                            v10 = 55_i32;
                                                            break 's4;
                                                        }
                                                        v10 = 60_i32;
                                                        break 's4;
                                                    }
                                                    v10 = 65_i32;
                                                    break 's4;
                                                }
                                                v10 = 70_i32;
                                                break 's4;
                                            }
                                            v10 = 75_i32;
                                            break 's4;
                                        }
                                        v10 = 80_i32;
                                        break 's4;
                                    }
                                    v10 = 85_i32;
                                    break 's4;
                                }
                                v10 = 90_i32;
                                break 's4;
                            }
                            v10 = 95_i32;
                            break 's4;
                        }
                        break 's4;
                    }
                    self.w32(v13, (v10.wrapping_add(v12).wrapping_sub(10_i32) as u32));
                }
                v12 = v12.wrapping_add(1);
                v13 = v13.wrapping_add(4);
                if !(v12 < 15_i32) {
                    break 'l3;
                }
            }
            let _ = self.f_10002970(this, 0_i32, a2, 0_i32, 0_i32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100035A0` (79 bytes).
    pub(crate) fn f_100035a0(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut result: i32 = 0;
        v1 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32));
        let _ = self.memset(v1.wrapping_add(7600), 255_i32 as u8, 192_u32);
        result = 0_i32;
        let _ = self.memset(v1.wrapping_add(7792), 0_i32 as u8, 24_u32);
        let _ = self.memset(v1.wrapping_add(7816), 0_i32 as u8, 24_u32);
        let _ = self.memset(v1.wrapping_add(7840), 0_i32 as u8, 48_u32);
        self.w32(v1.wrapping_add(7888), 0_u32);
        self.w32(v1.wrapping_add(7892), 0_u32);
        return result;
    }

    /// `sub_100035F0` (240 bytes).
    pub(crate) fn f_100035f0(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        v3 = (self.r32(self.r32(this)) as i32);
        v4 = (self.r32((v3.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32((v3.wrapping_add(16_i32) as u32)) as i32);
        's1: {
            'b1_5: {
                'b1_4: {
                    'b1_3: {
                        'b1_2: {
                            'b1_1: {
                                'b1_0: {
                                    match a2 {
                                        0_i32 => break 'b1_0,
                                        1_i32 => break 'b1_1,
                                        2_i32 => break 'b1_2,
                                        3_i32 => break 'b1_3,
                                        4_i32 => break 'b1_4,
                                        _ => break 'b1_5,
                                    }
                                }
                                v6 = 1_i32;
                                break 's1;
                            }
                            v6 = 2_i32;
                            break 's1;
                        }
                        v6 = 4_i32;
                        break 's1;
                    }
                    v6 = 8_i32;
                    break 's1;
                }
                v6 = 16_i32;
                break 's1;
            }
            v6 = a2;
            break 's1;
        }
        's2: {
            'b2_10: {
                'b2_9: {
                    'b2_8: {
                        'b2_7: {
                            'b2_6: {
                                'b2_5: {
                                    'b2_4: {
                                        'b2_3: {
                                            'b2_2: {
                                                'b2_1: {
                                                    'b2_0: {
                                                        match (self.r32(
                                                            (v4.wrapping_add(
                                                                716_i32.wrapping_mul(a2),
                                                            )
                                                            .wrapping_add(12_i32)
                                                                as u32),
                                                        )
                                                            as i32)
                                                        {
                                                            0_i32 => break 'b2_0,
                                                            1_i32 => break 'b2_1,
                                                            2_i32 => break 'b2_2,
                                                            3_i32 => break 'b2_3,
                                                            4_i32 => break 'b2_4,
                                                            5_i32 => break 'b2_5,
                                                            6_i32 => break 'b2_6,
                                                            7_i32 => break 'b2_7,
                                                            8_i32 => break 'b2_8,
                                                            9_i32 => break 'b2_9,
                                                            _ => break 'b2_10,
                                                        }
                                                    }
                                                    v7 = 1_i32;
                                                    break 's2;
                                                }
                                                v7 = 2_i32;
                                                break 's2;
                                            }
                                            v7 = 4_i32;
                                            break 's2;
                                        }
                                        v7 = 8_i32;
                                        break 's2;
                                    }
                                    v7 = 16_i32;
                                    break 's2;
                                }
                                v7 = 32_i32;
                                break 's2;
                            }
                            v7 = 64_i32;
                            break 's2;
                        }
                        v7 = 128_i32;
                        break 's2;
                    }
                    v7 = 256_i32;
                    break 's2;
                }
                v7 = 512_i32;
                break 's2;
            }
            v7 = a2;
            break 's2;
        }
        v8 = (self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32)
            .wrapping_add(140_i32.wrapping_mul(a3));
        if (((v6 & (self.r32((v8.wrapping_add(16_i32) as u32)) as i32)) != 0_i32)
            && ((v7 & (self.r32((v8.wrapping_add(20_i32) as u32)) as i32)) != 0_i32))
        {
            self.w32((v5.wrapping_add(7888_i32) as u32), 1_u32);
            return 0_i32;
        } else {
            self.w32((v5.wrapping_add(7888_i32) as u32), 0_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10003720` (839 bytes).
    pub(crate) fn f_10003720(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut result: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = (self.r32(self.r32(this)) as i32);
                    v4 = (self.r32((v3.wrapping_add(16_i32) as u32)) as i32);
                    bb = if (a3 == (1_i32).wrapping_neg()) { 1 } else { 3 };
                }
                1 => {
                    let _ = self.message_box(
                        self.r32(self.r32((v3.wrapping_add(4_i32) as u32))),
                        0x10028618_u32,
                        0x10028634_u32,
                        48_u32,
                    );
                    return 1_i32;
                }
                2 => {
                    return result;
                }
                3 => {
                    let t1 = (self.r32(
                        ((self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32)
                            .wrapping_add(
                                140_i32.wrapping_mul(
                                    (self.r32(
                                        (v4.wrapping_add(4_i32.wrapping_mul(a3))
                                            .wrapping_add(1600_i32)
                                            as u32),
                                    ) as i32),
                                ),
                            ) as u32),
                    ) as i32);
                    bb = match t1 {
                        5_i32 => 6,
                        7_i32 => 7,
                        8_i32 => 8,
                        9_i32 => 9,
                        10_i32 => 10,
                        11_i32 => 11,
                        12_i32 => 12,
                        _ => 13,
                    };
                }
                4 => {
                    bb = 2;
                }
                5 => {
                    bb = 2;
                }
                6 => {
                    v6 = (self.r32(
                        (v4.wrapping_add(4_i32.wrapping_mul(a3))
                            .wrapping_add(4400_i32) as u32),
                    ) as i32);
                    bb = if (v6 == 8_i32.wrapping_mul(a2)) {
                        14
                    } else {
                        15
                    };
                }
                7 => {
                    v7 = 1_i32;
                    bb = if ((self.r32(
                        (v4.wrapping_add(4_i32.wrapping_mul(a3))
                            .wrapping_add(4400_i32) as u32),
                    ) as i32)
                        == 8_i32.wrapping_mul(a2).wrapping_add(1_i32))
                    {
                        29
                    } else {
                        31
                    };
                }
                8 => {
                    self.w32(
                        (v4.wrapping_add(7888_i32) as u32),
                        ((((self.r32(
                            (v4.wrapping_add(4_i32.wrapping_mul(a3))
                                .wrapping_add(4400_i32) as u32),
                        ) as i32)
                            != 8_i32.wrapping_mul(a2).wrapping_add(2_i32))
                            as i32) as u32),
                    );
                    self.w32((v4.wrapping_add(7892_i32) as u32), 2_u32);
                    result = 0_i32;
                    bb = 5;
                }
                9 => {
                    self.w32(
                        (v4.wrapping_add(7888_i32) as u32),
                        ((((self.r32(
                            (v4.wrapping_add(4_i32.wrapping_mul(a3))
                                .wrapping_add(4400_i32) as u32),
                        ) as i32)
                            != 8_i32.wrapping_mul(a2).wrapping_add(3_i32))
                            as i32) as u32),
                    );
                    self.w32((v4.wrapping_add(7892_i32) as u32), 3_u32);
                    result = 0_i32;
                    bb = 5;
                }
                10 => {
                    self.w32(
                        (v4.wrapping_add(7888_i32) as u32),
                        ((((self.r32(
                            (v4.wrapping_add(4_i32.wrapping_mul(a3))
                                .wrapping_add(4400_i32) as u32),
                        ) as i32)
                            != 8_i32.wrapping_mul(a2).wrapping_add(4_i32))
                            as i32) as u32),
                    );
                    self.w32((v4.wrapping_add(7892_i32) as u32), 4_u32);
                    result = 0_i32;
                    bb = 5;
                }
                11 => {
                    self.w32(
                        (v4.wrapping_add(7888_i32) as u32),
                        ((((self.r32(
                            (v4.wrapping_add(4_i32.wrapping_mul(a3))
                                .wrapping_add(4400_i32) as u32),
                        ) as i32)
                            != 8_i32.wrapping_mul(a2).wrapping_add(5_i32))
                            as i32) as u32),
                    );
                    self.w32((v4.wrapping_add(7892_i32) as u32), 5_u32);
                    result = 0_i32;
                    bb = 5;
                }
                12 => {
                    v8 = (self.r32(
                        (v4.wrapping_add(4_i32.wrapping_mul(a3))
                            .wrapping_add(4400_i32) as u32),
                    ) as i32);
                    bb = if (v8 == 8_i32.wrapping_mul(a2).wrapping_add(6_i32)) {
                        37
                    } else {
                        39
                    };
                }
                13 => {
                    bb = 26;
                }
                14 => {
                    v7 = 0_i32;
                    bb = 16;
                }
                15 => {
                    bb = if (v6 == 8_i32.wrapping_mul(a2).wrapping_add(1_i32)) {
                        18
                    } else {
                        20
                    };
                }
                16 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), (v7 as u32));
                    self.w32((v4.wrapping_add(7892_i32) as u32), (v7 as u32));
                    result = 0_i32;
                    bb = 30;
                }
                17 => {
                    bb = 15;
                }
                18 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 0_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 1_u32);
                    result = 0_i32;
                    bb = 19;
                }
                19 => {
                    bb = 5;
                }
                20 => {
                    bb = if (((self.r32(
                        (32_i32
                            .wrapping_mul(a2)
                            .wrapping_add(v4)
                            .wrapping_add(7600_i32) as u32),
                    ) as i32)
                        == (1_i32).wrapping_neg())
                        || ((self.r32(
                            (32_i32
                                .wrapping_mul(a2)
                                .wrapping_add(v4)
                                .wrapping_add(7604_i32) as u32),
                        ) as i32)
                            != (1_i32).wrapping_neg()))
                    {
                        21
                    } else {
                        23
                    };
                }
                21 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 1_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 0_u32);
                    result = 0_i32;
                    bb = 22;
                }
                22 => {
                    bb = 19;
                }
                23 => {
                    bb = if (((self.r32(
                        ((self.r32((v3.wrapping_add(12_i32) as u32)) as i32)
                            .wrapping_add(716_i32.wrapping_mul(a2))
                            .wrapping_add(12_i32) as u32),
                    ) as i32) as u32)
                        > 9_u32)
                    {
                        24
                    } else {
                        25
                    };
                }
                24 => {
                    bb = 26;
                }
                25 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 1_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 1_u32);
                    result = 0_i32;
                    bb = 22;
                }
                26 => {
                    result = 0_i32;
                    bb = 5;
                }
                27 => {
                    bb = 25;
                }
                28 => {
                    bb = 7;
                }
                29 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 0_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 1_u32);
                    result = 0_i32;
                    bb = 30;
                }
                30 => {
                    bb = 5;
                }
                31 => {
                    bb = 16;
                }
                32 => {
                    bb = 8;
                }
                33 => {
                    bb = 9;
                }
                34 => {
                    bb = 10;
                }
                35 => {
                    bb = 11;
                }
                36 => {
                    bb = 12;
                }
                37 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 0_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 6_u32);
                    result = 0_i32;
                    bb = 38;
                }
                38 => {
                    bb = 5;
                }
                39 => {
                    bb = if (v8 == 8_i32.wrapping_mul(a2).wrapping_add(7_i32)) {
                        40
                    } else {
                        42
                    };
                }
                40 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 0_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 7_u32);
                    result = 0_i32;
                    bb = 41;
                }
                41 => {
                    bb = 38;
                }
                42 => {
                    bb = if (((self.r32(
                        (32_i32
                            .wrapping_mul(a2)
                            .wrapping_add(v4)
                            .wrapping_add(7624_i32) as u32),
                    ) as i32)
                        == (1_i32).wrapping_neg())
                        || ((self.r32(
                            (32_i32
                                .wrapping_mul(a2)
                                .wrapping_add(v4)
                                .wrapping_add(7628_i32) as u32),
                        ) as i32)
                            != (1_i32).wrapping_neg()))
                    {
                        43
                    } else {
                        44
                    };
                }
                43 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 1_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 6_u32);
                    bb = 26;
                }
                44 => {
                    self.w32((v4.wrapping_add(7888_i32) as u32), 1_u32);
                    self.w32((v4.wrapping_add(7892_i32) as u32), 7_u32);
                    result = 0_i32;
                    bb = 41;
                }
                45 => {
                    bb = 44;
                }
                46 => {
                    bb = 13;
                }
                47 => {
                    bb = 5;
                }
                48 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10003AA0` (188 bytes).
    pub(crate) fn f_10003aa0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        v4 = a3.wrapping_add(8_i32.wrapping_mul(a2));
        v5 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        v6 = (self.r32(
            (v5.wrapping_add(4_i32.wrapping_mul(v4))
                .wrapping_add(7600_i32) as u32),
        ) as i32);
        v7 = (v5
            .wrapping_add(4_i32.wrapping_mul(v4))
            .wrapping_add(7600_i32) as u32);
        if (v6 != (1_i32).wrapping_neg()) {
            self.w32(
                (v5.wrapping_add(4_i32.wrapping_mul(v6))
                    .wrapping_add(4400_i32) as u32),
                ((1_i32).wrapping_neg() as u32),
            );
            self.w32(v7, ((1_i32).wrapping_neg() as u32));
        }
        if (a4 != (1_i32).wrapping_neg()) {
            v8 = (self.r32(
                (v5.wrapping_add(4_i32.wrapping_mul(a4))
                    .wrapping_add(4400_i32) as u32),
            ) as i32);
            if (v8 != (1_i32).wrapping_neg()) {
                self.w32(
                    (v5.wrapping_add(4_i32.wrapping_mul(v8))
                        .wrapping_add(7600_i32) as u32),
                    ((1_i32).wrapping_neg() as u32),
                );
                self.w32(
                    (v5.wrapping_add(4_i32.wrapping_mul(a4))
                        .wrapping_add(4400_i32) as u32),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
        }
        let _ = self.f_10003c50(this, a2);
        let _ = self.f_10003cf0(this, a2, a3, a4);
        let _ = self.f_10003ca0(this, a2);
        let _ = self.memset((v5.wrapping_add(4400_i32) as u32), 255_i32 as u8, 1400_u32);
        v9 = (v5.wrapping_add(7600_i32) as u32);
        i = 0_i32;
        'l1: loop {
            if !(i < 40_i32) {
                break 'l1;
            }
            if ((self.r32(v9) as i32) != (1_i32).wrapping_neg()) {
                self.w32(
                    (v5.wrapping_add(4400_i32)
                        .wrapping_add(4_i32.wrapping_mul((self.r32(v9) as i32)))
                        as u32),
                    (i as u32),
                );
            }
            v9 = v9.wrapping_add(4);
            i = i.wrapping_add(1);
        }
        j = 0_i32;
        'l2: loop {
            if !(j < 5_i32) {
                break 'l2;
            }
            let _ = self.f_10003b60(this, j);
            j = j.wrapping_add(1);
        }
        return 0_i32;
    }

    /// `sub_10003B60` (235 bytes).
    pub(crate) fn f_10003b60(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: u32 = 0;
        v3 = self.r32(self.r32(this));
        v4 = (self.r32(v3.wrapping_add(12)) as i32);
        v5 = ((self.r32(v3.wrapping_add(16)) as i32) as u32);
        let _ = self.f_10003c50(this, a2);
        let _ = self.f_10003f60(this);
        let _ = self.f_10003ca0(this, a2);
        v6 = (v4.wrapping_add(716_i32.wrapping_mul(a2)) as u32);
        self.w32(
            v6.wrapping_add(172),
            ((self.r32(v5.wrapping_add(7840)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(176),
            ((self.r32(v5.wrapping_add(7844)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(180),
            ((self.r32(v5.wrapping_add(7848)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(184),
            ((self.r32(v5.wrapping_add(7852)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(188),
            ((self.r32(v5.wrapping_add(7856)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(192),
            ((self.r32(v5.wrapping_add(7860)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(196),
            ((self.r32(v5.wrapping_add(7864)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(200),
            ((self.r32(v5.wrapping_add(7868)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(204),
            ((self.r32(v5.wrapping_add(7872)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(208),
            ((self.r32(v5.wrapping_add(7876)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(212),
            ((self.r32(v5.wrapping_add(7880)) as i32) as u32),
        );
        self.w32(
            v6.wrapping_add(216),
            ((self.r32(v5.wrapping_add(7884)) as i32) as u32),
        );
        let _ = self.f_100027d0(self.r32(this.wrapping_add(4)), 0_i32, a2);
        let _ = self.f_10002970(self.r32(this.wrapping_add(4)), 0_i32, a2, 0_i32, 0_i32);
        return 0_i32;
    }

    /// `sub_10003C50` (79 bytes).
    pub(crate) fn f_10003c50(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v2: u32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32));
        let _ = self.memcpy(
            v2.wrapping_add(7760),
            v2.wrapping_add((8_i32.wrapping_mul(a2).wrapping_add(1900_i32) as u32).wrapping_mul(4)),
            32_u32,
        );
        self.w32(
            v2.wrapping_add(7812),
            ((self.r32(v2.wrapping_add((a2.wrapping_add(1948_i32) as u32).wrapping_mul(4))) as i32)
                as u32),
        );
        self.w32(
            v2.wrapping_add(7836),
            ((self.r32(v2.wrapping_add((a2.wrapping_add(1954_i32) as u32).wrapping_mul(4))) as i32)
                as u32),
        );
        return 0_i32;
    }

    /// `sub_10003CA0` (79 bytes).
    pub(crate) fn f_10003ca0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32));
        let _ = self.memcpy(
            v2.wrapping_add((8_i32.wrapping_mul(a2).wrapping_add(1900_i32) as u32).wrapping_mul(4)),
            v2.wrapping_add(7760),
            32_u32,
        );
        v3 = a2;
        self.w32(
            v2.wrapping_add((v3.wrapping_add(1948_i32) as u32).wrapping_mul(4)),
            ((self.r32(v2.wrapping_add(7812)) as i32) as u32),
        );
        self.w32(
            v2.wrapping_add((v3.wrapping_add(1954_i32) as u32).wrapping_mul(4)),
            ((self.r32(v2.wrapping_add(7836)) as i32) as u32),
        );
        return 0_i32;
    }

    /// `sub_10003CF0` (603 bytes).
    pub(crate) fn f_10003cf0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v17: bool = false;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        v4 = (self.r32(self.r32(this)) as i32);
        v5 = (self.r32((v4.wrapping_add(16_i32) as u32)) as i32);
        's1: {
            'b1_2: {
                'b1_1: {
                    'b1_0: {
                        match (self.r32(
                            ((self.r32((v4.wrapping_add(12_i32) as u32)) as i32)
                                .wrapping_add(716_i32.wrapping_mul(a2))
                                .wrapping_add(12_i32) as u32),
                        ) as i32)
                        {
                            0_i32 | 1_i32 | 2_i32 | 3_i32 | 4_i32 | 5_i32 | 6_i32 => break 'b1_0,
                            7_i32 | 8_i32 | 9_i32 => break 'b1_1,
                            _ => break 'b1_2,
                        }
                    }
                    a2 = (((((a2 as u32) & 0xFFFFFF00) | (((1_u8 as u8) as u32) << 0)) as i32)
                        as i32);
                    break 's1;
                }
                a2 = (((((a2 as u32) & 0xFFFFFF00) | (((0_u8 as u8) as u32) << 0)) as i32) as i32);
                break 's1;
            }
            break 's1;
        }
        v7 = 0_i32;
        v8 = 0_i32;
        v9 = (self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32);
        v18 = 0_i32;
        if (a4 != (1_i32).wrapping_neg()) {
            v10 = (self.r32(
                (v5.wrapping_add(4_i32.wrapping_mul(a4))
                    .wrapping_add(1600_i32) as u32),
            ) as i32);
            v8 = (self.r32((v9.wrapping_add(140_i32.wrapping_mul(v10)) as u32)) as i32);
            v18 = (self.r32(
                (v9.wrapping_add(140_i32.wrapping_mul(v10))
                    .wrapping_add(8_i32) as u32),
            ) as i32);
        }
        v11 = (self.r32((v5.wrapping_add(7760_i32) as u32)) as i32);
        v19 = 0_i32;
        v20 = 0_i32;
        if (v11 != (1_i32).wrapping_neg()) {
            v12 = (self.r32(
                (v5.wrapping_add(4_i32.wrapping_mul(v11))
                    .wrapping_add(1600_i32) as u32),
            ) as i32);
            v19 = (self.r32((v9.wrapping_add(140_i32.wrapping_mul(v12)) as u32)) as i32);
            v20 = (self.r32(
                (v9.wrapping_add(140_i32.wrapping_mul(v12))
                    .wrapping_add(8_i32) as u32),
            ) as i32);
        }
        v13 = (self.r32((v5.wrapping_add(7764_i32) as u32)) as i32);
        v14 = 0_i32;
        if (v13 != (1_i32).wrapping_neg()) {
            v15 = (self.r32(
                (v5.wrapping_add(4_i32.wrapping_mul(v13))
                    .wrapping_add(1600_i32) as u32),
            ) as i32);
            v7 = (self.r32((v9.wrapping_add(140_i32.wrapping_mul(v15)) as u32)) as i32);
            v14 = (self.r32(
                (v9.wrapping_add(140_i32.wrapping_mul(v15))
                    .wrapping_add(8_i32) as u32),
            ) as i32);
        }
        if (!(a3 != 0)) {
            's2: {
                'b2_3: {
                    'b2_2: {
                        'b2_1: {
                            'b2_0: {
                                match v8 {
                                    0_i32 => break 'b2_0,
                                    5_i32 => break 'b2_1,
                                    7_i32 => break 'b2_2,
                                    _ => break 'b2_3,
                                }
                            }
                            self.w32(
                                (v5.wrapping_add(7760_i32) as u32),
                                ((1_i32).wrapping_neg() as u32),
                            );
                            return 0_i32;
                        }
                        self.w32((v5.wrapping_add(7760_i32) as u32), (a4 as u32));
                        if (v18 == 1_i32) {
                            if (v14 == 3_i32) {
                                self.w32(
                                    (v5.wrapping_add(7764_i32) as u32),
                                    ((1_i32).wrapping_neg() as u32),
                                );
                            }
                            if (!((a2 as u8) != 0)) {
                                v17 = (v7 == 5_i32);
                                break 's2;
                            }
                            return 0_i32;
                        }
                        v17 = (v18 == 3_i32);
                        break 's2;
                    }
                    self.w32((v5.wrapping_add(7760_i32) as u32), (a4 as u32));
                    v17 = (v14 == 3_i32);
                    break 's2;
                }
                return 0_i32;
            }
            if v17 {
                self.w32(
                    (v5.wrapping_add(7764_i32) as u32),
                    ((1_i32).wrapping_neg() as u32),
                );
                return 0_i32;
            }
            return 0_i32;
        }
        if (a3 == 1_i32) {
            if (!(v8 != 0)) {
                self.w32(
                    (v5.wrapping_add(7764_i32) as u32),
                    ((1_i32).wrapping_neg() as u32),
                );
                return 0_i32;
            }
            if (v8 != 5_i32) {
                if (v8 == 7_i32) {
                    self.w32((v5.wrapping_add(7764_i32) as u32), (a4 as u32));
                    if (v20 == 3_i32) {
                        self.w32(
                            (v5.wrapping_add(7760_i32) as u32),
                            ((1_i32).wrapping_neg() as u32),
                        );
                        return 0_i32;
                    }
                }
                return 0_i32;
            }
            self.w32((v5.wrapping_add(7764_i32) as u32), (a4 as u32));
            if (v18 == 1_i32) {
                if (v20 == 3_i32) {
                    self.w32(
                        (v5.wrapping_add(7760_i32) as u32),
                        ((1_i32).wrapping_neg() as u32),
                    );
                }
                if ((a2 as u8) != 0) {
                    return 0_i32;
                }
                if (v19 == 5_i32) {
                    self.w32(
                        (v5.wrapping_add(7760_i32) as u32),
                        ((1_i32).wrapping_neg() as u32),
                    );
                }
            } else {
                if (v18 != 3_i32) {
                    return 0_i32;
                }
                self.w32(
                    (v5.wrapping_add(7760_i32) as u32),
                    ((1_i32).wrapping_neg() as u32),
                );
                if ((a2 as u8) != 0) {
                    return 0_i32;
                }
            }
            self.w32(
                (v5.wrapping_add(7760_i32) as u32),
                ((self.r32((v5.wrapping_add(7764_i32) as u32)) as i32) as u32),
            );
            self.w32(
                (v5.wrapping_add(7764_i32) as u32),
                ((1_i32).wrapping_neg() as u32),
            );
            return 0_i32;
        }
        if (!(v8 != 0)) {
            self.w32(
                (v5.wrapping_add(4_i32.wrapping_mul(a3))
                    .wrapping_add(7760_i32) as u32),
                ((1_i32).wrapping_neg() as u32),
            );
            return 0_i32;
        }
        self.w32(
            (v5.wrapping_add(4_i32.wrapping_mul(a3))
                .wrapping_add(7760_i32) as u32),
            (a4 as u32),
        );
        return 0_i32;
    }

    /// `sub_10003F60` (2380 bytes).
    pub(crate) fn f_10003f60(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: u32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: u32 = 0;
        let mut v28: i32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: i32 = 0;
        let mut v32: i32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: u32 = 0;
        let mut v41: i32 = 0;
        let mut v42: i32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: i32 = 0;
        let mut v47: i32 = 0;
        let mut v48: i32 = 0;
        let mut v49: i32 = 0;
        let mut v50: i32 = 0;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: u32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: i32 = 0;
        let mut v58: i32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: i32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: i32 = 0;
        let mut v65: i32 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: u32 = 0;
        let mut v69: i32 = 0;
        let mut v70: i32 = 0;
        let mut v71: i32 = 0;
        let mut v72: i32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut v77: i32 = 0;
        let mut v78: i32 = 0;
        let mut v79: i32 = 0;
        let mut v80: i32 = 0;
        let mut v81: i32 = 0;
        let mut v82: u32 = 0;
        let mut v83: i32 = 0;
        let mut v84: i32 = 0;
        let mut v85: i32 = 0;
        let mut v86: i32 = 0;
        let mut v87: i32 = 0;
        let mut v88: i32 = 0;
        let mut v89: i32 = 0;
        let mut v90: i32 = 0;
        let mut v91: i32 = 0;
        let mut v92: i32 = 0;
        let mut v93: i32 = 0;
        let mut v94: i32 = 0;
        let mut v95: i32 = 0;
        let mut v96: u32 = 0;
        let mut v97: i32 = 0;
        let mut v98: i32 = 0;
        let mut v99: i32 = 0;
        let mut v100: i32 = 0;
        let mut v101: i32 = 0;
        let mut v102: i32 = 0;
        let mut v103: i32 = 0;
        let mut v104: i32 = 0;
        let mut v105: i32 = 0;
        let mut v106: i32 = 0;
        let mut v107: i32 = 0;
        let mut v108: i32 = 0;
        let mut v109: i32 = 0;
        let mut v110: u32 = 0;
        let mut v111: i32 = 0;
        let mut v112: i32 = 0;
        let mut v113: i32 = 0;
        let mut v114: i32 = 0;
        let mut v115: i32 = 0;
        let mut v116: i32 = 0;
        let mut v117: i32 = 0;
        let mut v118: i32 = 0;
        let mut v119: i32 = 0;
        v1 = self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32));
        v2 = (self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
            as i32);
        self.w32(v1.wrapping_add(7812), 0_u32);
        v3 = (self.r32(v1.wrapping_add(7760)) as i32);
        if (v3 != (1_i32).wrapping_neg()) {
            v4 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v3.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v5 = (self.r32((v2.wrapping_add(4_i32.wrapping_mul(v4)) as u32)) as i32);
            v6 = v2.wrapping_add(4_i32.wrapping_mul(v4));
            if (v5 == 5_i32) {
                v7 = (self.r32((v6.wrapping_add(8_i32) as u32)) as i32);
                if (v7 == 1_i32) {
                    self.w32(v1.wrapping_add(7812), 1_u32);
                } else {
                    if (v7 == 3_i32) {
                        self.w32(v1.wrapping_add(7812), 3_u32);
                    }
                }
            }
        }
        v8 = (self.r32(v1.wrapping_add(7764)) as i32);
        if (v8 != (1_i32).wrapping_neg()) {
            v9 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v8.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v10 = (self.r32((v2.wrapping_add(4_i32.wrapping_mul(v9)) as u32)) as i32);
            v11 = v2.wrapping_add(4_i32.wrapping_mul(v9));
            if (v10 == 5_i32) {
                v12 = (self.r32((v11.wrapping_add(8_i32) as u32)) as i32);
                if (v12 == 1_i32) {
                    self.w32(
                        v1.wrapping_add(7812),
                        ((if ((self.r32(v1.wrapping_add(7812)) as i32) != 1_i32) {
                            2_i32
                        } else {
                            4_i32
                        }) as u32),
                    );
                } else {
                    if (v12 == 3_i32) {
                        self.w32(v1.wrapping_add(7812), 3_u32);
                    }
                }
            }
        }
        self.w32(v1.wrapping_add(7836), 0_u32);
        v13 = (self.r32(v1.wrapping_add(7760)) as i32);
        if (v13 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(
                    (v2.wrapping_add(140_i32.wrapping_mul(
                        (self.r32(
                            v1.wrapping_add((v13.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                        ) as i32),
                    ))
                    .wrapping_add(28_i32) as u32),
                ) as i32) as u32),
            );
        }
        v14 = (self.r32(v1.wrapping_add(7764)) as i32);
        if (v14 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(v1.wrapping_add(7836)) as i32).wrapping_add(
                    (self.r32(
                        (v2.wrapping_add(140_i32.wrapping_mul(
                            (self.r32(
                                v1.wrapping_add((v14.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                            ) as i32),
                        ))
                        .wrapping_add(28_i32) as u32),
                    ) as i32),
                ) as u32),
            );
        }
        v15 = (self.r32(v1.wrapping_add(7768)) as i32);
        if (v15 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(v1.wrapping_add(7836)) as i32).wrapping_add(
                    (self.r32(
                        (v2.wrapping_add(140_i32.wrapping_mul(
                            (self.r32(
                                v1.wrapping_add((v15.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                            ) as i32),
                        ))
                        .wrapping_add(28_i32) as u32),
                    ) as i32),
                ) as u32),
            );
        }
        v16 = (self.r32(v1.wrapping_add(7772)) as i32);
        if (v16 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(v1.wrapping_add(7836)) as i32).wrapping_add(
                    (self.r32(
                        (v2.wrapping_add(140_i32.wrapping_mul(
                            (self.r32(
                                v1.wrapping_add((v16.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                            ) as i32),
                        ))
                        .wrapping_add(28_i32) as u32),
                    ) as i32),
                ) as u32),
            );
        }
        v17 = (self.r32(v1.wrapping_add(7776)) as i32);
        if (v17 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(v1.wrapping_add(7836)) as i32).wrapping_add(
                    (self.r32(
                        (v2.wrapping_add(140_i32.wrapping_mul(
                            (self.r32(
                                v1.wrapping_add((v17.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                            ) as i32),
                        ))
                        .wrapping_add(28_i32) as u32),
                    ) as i32),
                ) as u32),
            );
        }
        v18 = (self.r32(v1.wrapping_add(7780)) as i32);
        if (v18 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(v1.wrapping_add(7836)) as i32).wrapping_add(
                    (self.r32(
                        (v2.wrapping_add(140_i32.wrapping_mul(
                            (self.r32(
                                v1.wrapping_add((v18.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                            ) as i32),
                        ))
                        .wrapping_add(28_i32) as u32),
                    ) as i32),
                ) as u32),
            );
        }
        v19 = (self.r32(v1.wrapping_add(7784)) as i32);
        if (v19 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(v1.wrapping_add(7836)) as i32).wrapping_add(
                    (self.r32(
                        (v2.wrapping_add(140_i32.wrapping_mul(
                            (self.r32(
                                v1.wrapping_add((v19.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                            ) as i32),
                        ))
                        .wrapping_add(28_i32) as u32),
                    ) as i32),
                ) as u32),
            );
        }
        v20 = (self.r32(v1.wrapping_add(7788)) as i32);
        if (v20 != (1_i32).wrapping_neg()) {
            self.w32(
                v1.wrapping_add(7836),
                ((self.r32(v1.wrapping_add(7836)) as i32).wrapping_add(
                    (self.r32(
                        (v2.wrapping_add(140_i32.wrapping_mul(
                            (self.r32(
                                v1.wrapping_add((v20.wrapping_add(400_i32) as u32).wrapping_mul(4)),
                            ) as i32),
                        ))
                        .wrapping_add(28_i32) as u32),
                    ) as i32),
                ) as u32),
            );
        }
        self.w32(v1.wrapping_add(7840), 0_u32);
        self.w32(v1.wrapping_add(7844), 0_u32);
        self.w32(v1.wrapping_add(7848), 0_u32);
        self.w32(v1.wrapping_add(7852), 0_u32);
        self.w32(v1.wrapping_add(7856), 0_u32);
        self.w32(v1.wrapping_add(7860), 0_u32);
        self.w32(v1.wrapping_add(7864), 0_u32);
        self.w32(v1.wrapping_add(7868), 0_u32);
        self.w32(v1.wrapping_add(7872), 0_u32);
        self.w32(v1.wrapping_add(7876), 0_u32);
        self.w32(v1.wrapping_add(7880), 0_u32);
        self.w32(v1.wrapping_add(7884), 0_u32);
        v21 = (self.r32(v1.wrapping_add(7760)) as i32);
        if (v21 != (1_i32).wrapping_neg()) {
            v22 = (v2.wrapping_add(140_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v21.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            )) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v22.wrapping_add(36)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v22.wrapping_add(40)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7848),
                ((self.r32(v22.wrapping_add(44)) as i32) as u32),
            );
            self.w32(v1.wrapping_add(7852), 0_u32);
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v22.wrapping_add(48)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v22.wrapping_add(52)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v22.wrapping_add(56)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v22.wrapping_add(60)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v22.wrapping_add(64)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v22.wrapping_add(68)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v22.wrapping_add(72)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v22.wrapping_add(76)) as i32) as u32),
            );
        }
        v23 = (self.r32(v1.wrapping_add(7764)) as i32);
        if (v23 != (1_i32).wrapping_neg()) {
            v24 = (self.r32(v1.wrapping_add(7844)) as i32);
            v25 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v23.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v26 = (self.r32(
                (v2.wrapping_add(4_i32.wrapping_mul(v25))
                    .wrapping_add(36_i32) as u32),
            ) as i32);
            v27 = (v2.wrapping_add(4_i32.wrapping_mul(v25)) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v1.wrapping_add(7840)) as i32).wrapping_add(v26) as u32),
            );
            v28 = (self.r32(v1.wrapping_add(7856)) as i32);
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v27.wrapping_add(40)) as i32).wrapping_add(v24) as u32),
            );
            v29 = (self.r32(v1.wrapping_add(7860)) as i32);
            self.w32(
                v1.wrapping_add(7852),
                ((self.r32(v27.wrapping_add(44)) as i32) as u32),
            );
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v27.wrapping_add(48)) as i32).wrapping_add(v28) as u32),
            );
            v30 = (self.r32(v1.wrapping_add(7864)) as i32);
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v27.wrapping_add(52)) as i32).wrapping_add(v29) as u32),
            );
            v31 = (self.r32(v1.wrapping_add(7868)) as i32);
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v27.wrapping_add(56)) as i32).wrapping_add(v30) as u32),
            );
            v32 = (self.r32(v1.wrapping_add(7872)) as i32);
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v27.wrapping_add(60)) as i32).wrapping_add(v31) as u32),
            );
            v33 = (self.r32(v1.wrapping_add(7876)) as i32);
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v27.wrapping_add(64)) as i32).wrapping_add(v32) as u32),
            );
            v34 = (self.r32(v1.wrapping_add(7880)) as i32);
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v27.wrapping_add(68)) as i32).wrapping_add(v33) as u32),
            );
            v35 = (self.r32(v1.wrapping_add(7884)) as i32);
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v27.wrapping_add(72)) as i32).wrapping_add(v34) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v27.wrapping_add(76)) as i32).wrapping_add(v35) as u32),
            );
        }
        v36 = (self.r32(v1.wrapping_add(7768)) as i32);
        if (v36 != (1_i32).wrapping_neg()) {
            v37 = (self.r32(v1.wrapping_add(7844)) as i32);
            v38 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v36.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v39 = (self.r32(
                (v2.wrapping_add(4_i32.wrapping_mul(v38))
                    .wrapping_add(36_i32) as u32),
            ) as i32);
            v40 = (v2.wrapping_add(4_i32.wrapping_mul(v38)) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v1.wrapping_add(7840)) as i32).wrapping_add(v39) as u32),
            );
            v41 = (self.r32(v1.wrapping_add(7848)) as i32);
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v40.wrapping_add(40)) as i32).wrapping_add(v37) as u32),
            );
            v42 = (self.r32(v1.wrapping_add(7856)) as i32);
            self.w32(
                v1.wrapping_add(7848),
                ((self.r32(v40.wrapping_add(44)) as i32).wrapping_add(v41) as u32),
            );
            v43 = (self.r32(v1.wrapping_add(7860)) as i32);
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v40.wrapping_add(48)) as i32).wrapping_add(v42) as u32),
            );
            v44 = (self.r32(v1.wrapping_add(7864)) as i32);
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v40.wrapping_add(52)) as i32).wrapping_add(v43) as u32),
            );
            v45 = (self.r32(v1.wrapping_add(7868)) as i32);
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v40.wrapping_add(56)) as i32).wrapping_add(v44) as u32),
            );
            v46 = (self.r32(v1.wrapping_add(7872)) as i32);
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v40.wrapping_add(60)) as i32).wrapping_add(v45) as u32),
            );
            v47 = (self.r32(v1.wrapping_add(7876)) as i32);
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v40.wrapping_add(64)) as i32).wrapping_add(v46) as u32),
            );
            v48 = (self.r32(v1.wrapping_add(7880)) as i32);
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v40.wrapping_add(68)) as i32).wrapping_add(v47) as u32),
            );
            v49 = (self.r32(v1.wrapping_add(7884)) as i32);
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v40.wrapping_add(72)) as i32).wrapping_add(v48) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v40.wrapping_add(76)) as i32).wrapping_add(v49) as u32),
            );
        }
        v50 = (self.r32(v1.wrapping_add(7772)) as i32);
        if (v50 != (1_i32).wrapping_neg()) {
            v51 = (self.r32(v1.wrapping_add(7844)) as i32);
            v52 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v50.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v53 = (self.r32(
                (v2.wrapping_add(4_i32.wrapping_mul(v52))
                    .wrapping_add(36_i32) as u32),
            ) as i32);
            v54 = (v2.wrapping_add(4_i32.wrapping_mul(v52)) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v1.wrapping_add(7840)) as i32).wrapping_add(v53) as u32),
            );
            v55 = (self.r32(v1.wrapping_add(7848)) as i32);
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v54.wrapping_add(40)) as i32).wrapping_add(v51) as u32),
            );
            v56 = (self.r32(v1.wrapping_add(7856)) as i32);
            self.w32(
                v1.wrapping_add(7848),
                ((self.r32(v54.wrapping_add(44)) as i32).wrapping_add(v55) as u32),
            );
            v57 = (self.r32(v1.wrapping_add(7860)) as i32);
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v54.wrapping_add(48)) as i32).wrapping_add(v56) as u32),
            );
            v58 = (self.r32(v1.wrapping_add(7864)) as i32);
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v54.wrapping_add(52)) as i32).wrapping_add(v57) as u32),
            );
            v59 = (self.r32(v1.wrapping_add(7868)) as i32);
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v54.wrapping_add(56)) as i32).wrapping_add(v58) as u32),
            );
            v60 = (self.r32(v1.wrapping_add(7872)) as i32);
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v54.wrapping_add(60)) as i32).wrapping_add(v59) as u32),
            );
            v61 = (self.r32(v1.wrapping_add(7876)) as i32);
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v54.wrapping_add(64)) as i32).wrapping_add(v60) as u32),
            );
            v62 = (self.r32(v1.wrapping_add(7880)) as i32);
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v54.wrapping_add(68)) as i32).wrapping_add(v61) as u32),
            );
            v63 = (self.r32(v1.wrapping_add(7884)) as i32);
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v54.wrapping_add(72)) as i32).wrapping_add(v62) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v54.wrapping_add(76)) as i32).wrapping_add(v63) as u32),
            );
        }
        v64 = (self.r32(v1.wrapping_add(7776)) as i32);
        if (v64 != (1_i32).wrapping_neg()) {
            v65 = (self.r32(v1.wrapping_add(7844)) as i32);
            v66 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v64.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v67 = (self.r32(
                (v2.wrapping_add(4_i32.wrapping_mul(v66))
                    .wrapping_add(36_i32) as u32),
            ) as i32);
            v68 = (v2.wrapping_add(4_i32.wrapping_mul(v66)) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v1.wrapping_add(7840)) as i32).wrapping_add(v67) as u32),
            );
            v69 = (self.r32(v1.wrapping_add(7848)) as i32);
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v68.wrapping_add(40)) as i32).wrapping_add(v65) as u32),
            );
            v70 = (self.r32(v1.wrapping_add(7856)) as i32);
            self.w32(
                v1.wrapping_add(7848),
                ((self.r32(v68.wrapping_add(44)) as i32).wrapping_add(v69) as u32),
            );
            v71 = (self.r32(v1.wrapping_add(7860)) as i32);
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v68.wrapping_add(48)) as i32).wrapping_add(v70) as u32),
            );
            v72 = (self.r32(v1.wrapping_add(7864)) as i32);
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v68.wrapping_add(52)) as i32).wrapping_add(v71) as u32),
            );
            v73 = (self.r32(v1.wrapping_add(7868)) as i32);
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v68.wrapping_add(56)) as i32).wrapping_add(v72) as u32),
            );
            v74 = (self.r32(v1.wrapping_add(7872)) as i32);
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v68.wrapping_add(60)) as i32).wrapping_add(v73) as u32),
            );
            v75 = (self.r32(v1.wrapping_add(7876)) as i32);
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v68.wrapping_add(64)) as i32).wrapping_add(v74) as u32),
            );
            v76 = (self.r32(v1.wrapping_add(7880)) as i32);
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v68.wrapping_add(68)) as i32).wrapping_add(v75) as u32),
            );
            v77 = (self.r32(v1.wrapping_add(7884)) as i32);
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v68.wrapping_add(72)) as i32).wrapping_add(v76) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v68.wrapping_add(76)) as i32).wrapping_add(v77) as u32),
            );
        }
        v78 = (self.r32(v1.wrapping_add(7780)) as i32);
        if (v78 != (1_i32).wrapping_neg()) {
            v79 = (self.r32(v1.wrapping_add(7844)) as i32);
            v80 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v78.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v81 = (self.r32(
                (v2.wrapping_add(4_i32.wrapping_mul(v80))
                    .wrapping_add(36_i32) as u32),
            ) as i32);
            v82 = (v2.wrapping_add(4_i32.wrapping_mul(v80)) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v1.wrapping_add(7840)) as i32).wrapping_add(v81) as u32),
            );
            v83 = (self.r32(v1.wrapping_add(7848)) as i32);
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v82.wrapping_add(40)) as i32).wrapping_add(v79) as u32),
            );
            v84 = (self.r32(v1.wrapping_add(7856)) as i32);
            self.w32(
                v1.wrapping_add(7848),
                ((self.r32(v82.wrapping_add(44)) as i32).wrapping_add(v83) as u32),
            );
            v85 = (self.r32(v1.wrapping_add(7860)) as i32);
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v82.wrapping_add(48)) as i32).wrapping_add(v84) as u32),
            );
            v86 = (self.r32(v1.wrapping_add(7864)) as i32);
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v82.wrapping_add(52)) as i32).wrapping_add(v85) as u32),
            );
            v87 = (self.r32(v1.wrapping_add(7868)) as i32);
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v82.wrapping_add(56)) as i32).wrapping_add(v86) as u32),
            );
            v88 = (self.r32(v1.wrapping_add(7872)) as i32);
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v82.wrapping_add(60)) as i32).wrapping_add(v87) as u32),
            );
            v89 = (self.r32(v1.wrapping_add(7876)) as i32);
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v82.wrapping_add(64)) as i32).wrapping_add(v88) as u32),
            );
            v90 = (self.r32(v1.wrapping_add(7880)) as i32);
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v82.wrapping_add(68)) as i32).wrapping_add(v89) as u32),
            );
            v91 = (self.r32(v1.wrapping_add(7884)) as i32);
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v82.wrapping_add(72)) as i32).wrapping_add(v90) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v82.wrapping_add(76)) as i32).wrapping_add(v91) as u32),
            );
        }
        v92 = (self.r32(v1.wrapping_add(7784)) as i32);
        if (v92 != (1_i32).wrapping_neg()) {
            v93 = (self.r32(v1.wrapping_add(7844)) as i32);
            v94 = 35_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v92.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            );
            v95 = (self.r32(
                (v2.wrapping_add(4_i32.wrapping_mul(v94))
                    .wrapping_add(36_i32) as u32),
            ) as i32);
            v96 = (v2.wrapping_add(4_i32.wrapping_mul(v94)) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v1.wrapping_add(7840)) as i32).wrapping_add(v95) as u32),
            );
            v97 = (self.r32(v1.wrapping_add(7848)) as i32);
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v96.wrapping_add(40)) as i32).wrapping_add(v93) as u32),
            );
            v98 = (self.r32(v1.wrapping_add(7852)) as i32);
            self.w32(
                v1.wrapping_add(7848),
                ((self.r32(v96.wrapping_add(44)) as i32).wrapping_add(v97) as u32),
            );
            v99 = (self.r32(v1.wrapping_add(7856)) as i32);
            self.w32(
                v1.wrapping_add(7852),
                ((self.r32(v96.wrapping_add(44)) as i32).wrapping_add(v98) as u32),
            );
            v100 = (self.r32(v1.wrapping_add(7860)) as i32);
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v96.wrapping_add(48)) as i32).wrapping_add(v99) as u32),
            );
            v101 = (self.r32(v1.wrapping_add(7864)) as i32);
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v96.wrapping_add(52)) as i32).wrapping_add(v100) as u32),
            );
            v102 = (self.r32(v1.wrapping_add(7868)) as i32);
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v96.wrapping_add(56)) as i32).wrapping_add(v101) as u32),
            );
            v103 = (self.r32(v1.wrapping_add(7872)) as i32);
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v96.wrapping_add(60)) as i32).wrapping_add(v102) as u32),
            );
            v104 = (self.r32(v1.wrapping_add(7876)) as i32);
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v96.wrapping_add(64)) as i32).wrapping_add(v103) as u32),
            );
            v105 = (self.r32(v1.wrapping_add(7880)) as i32);
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v96.wrapping_add(68)) as i32).wrapping_add(v104) as u32),
            );
            v106 = (self.r32(v1.wrapping_add(7884)) as i32);
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v96.wrapping_add(72)) as i32).wrapping_add(v105) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v96.wrapping_add(76)) as i32).wrapping_add(v106) as u32),
            );
        }
        v107 = (self.r32(v1.wrapping_add(7788)) as i32);
        if (v107 != (1_i32).wrapping_neg()) {
            v108 = (self.r32(v1.wrapping_add(7848)) as i32);
            v109 = (self.r32(v1.wrapping_add(7844)) as i32);
            v110 = (v2.wrapping_add(140_i32.wrapping_mul(
                (self.r32(v1.wrapping_add((v107.wrapping_add(400_i32) as u32).wrapping_mul(4)))
                    as i32),
            )) as u32);
            self.w32(
                v1.wrapping_add(7840),
                ((self.r32(v1.wrapping_add(7840)) as i32)
                    .wrapping_add((self.r32(v110.wrapping_add(36)) as i32))
                    as u32),
            );
            v111 = (self.r32(v1.wrapping_add(7852)) as i32);
            self.w32(
                v1.wrapping_add(7844),
                ((self.r32(v110.wrapping_add(40)) as i32).wrapping_add(v109) as u32),
            );
            v112 = (self.r32(v1.wrapping_add(7856)) as i32);
            self.w32(
                v1.wrapping_add(7848),
                ((self.r32(v110.wrapping_add(44)) as i32).wrapping_add(v108) as u32),
            );
            v113 = (self.r32(v1.wrapping_add(7860)) as i32);
            self.w32(
                v1.wrapping_add(7852),
                ((self.r32(v110.wrapping_add(44)) as i32).wrapping_add(v111) as u32),
            );
            v114 = (self.r32(v1.wrapping_add(7864)) as i32);
            self.w32(
                v1.wrapping_add(7856),
                ((self.r32(v110.wrapping_add(48)) as i32).wrapping_add(v112) as u32),
            );
            v115 = (self.r32(v1.wrapping_add(7868)) as i32);
            self.w32(
                v1.wrapping_add(7860),
                ((self.r32(v110.wrapping_add(52)) as i32).wrapping_add(v113) as u32),
            );
            v116 = (self.r32(v1.wrapping_add(7872)) as i32);
            self.w32(
                v1.wrapping_add(7864),
                ((self.r32(v110.wrapping_add(56)) as i32).wrapping_add(v114) as u32),
            );
            v117 = (self.r32(v1.wrapping_add(7876)) as i32);
            self.w32(
                v1.wrapping_add(7868),
                ((self.r32(v110.wrapping_add(60)) as i32).wrapping_add(v115) as u32),
            );
            v118 = (self.r32(v1.wrapping_add(7880)) as i32);
            self.w32(
                v1.wrapping_add(7872),
                ((self.r32(v110.wrapping_add(64)) as i32).wrapping_add(v116) as u32),
            );
            self.w32(
                v1.wrapping_add(7876),
                ((self.r32(v110.wrapping_add(68)) as i32).wrapping_add(v117) as u32),
            );
            v119 = (self.r32(v1.wrapping_add(7884)) as i32);
            self.w32(
                v1.wrapping_add(7880),
                ((self.r32(v110.wrapping_add(72)) as i32).wrapping_add(v118) as u32),
            );
            self.w32(
                v1.wrapping_add(7884),
                ((self.r32(v110.wrapping_add(76)) as i32).wrapping_add(v119) as u32),
            );
        }
        return 0_i32;
    }

    /// `sub_100048B0` (19 bytes).
    pub(crate) fn f_100048b0(&mut self, mut this: u32) -> i32 {
        let _ = self.f_10001c60();
        let _ = self.unknown_libname_21(this);
        return 0_i32;
    }

    /// `unknown_libname_21` (11 bytes).
    pub(crate) fn unknown_libname_21(&mut self, mut this: u32) -> i32 {
        let mut result: i32 = 0;
        result = 0_i32;
        self.w32(this, 0_u32);
        self.w32(this.wrapping_add(4), 0_u32);
        self.w32(this.wrapping_add(8), 0_u32);
        return result;
    }

    /// `sub_100048E0` (16 bytes).
    pub(crate) fn f_100048e0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        self.w32(this, (a2 as u32));
        let _ = self.f_10004900(this);
        return 0_i32;
    }

    /// `sub_100048F0` (12 bytes).
    pub(crate) fn f_100048f0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        self.w32(this.wrapping_add(8), (a2 as u32));
        return 0_i32;
    }
}
