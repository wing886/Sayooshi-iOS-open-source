//! Functions 0x10007240..=0x10009E90, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_10007240` (95 bytes).
    pub(crate) fn f_10007240(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        self.w8(a3, 0_u8);
        v5 = v4.wrapping_add(32_i32.wrapping_mul(a2));
        v6 = (self.r32((v5.wrapping_add(7624_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1718_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7628_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1718_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_100072A0` (95 bytes).
    pub(crate) fn f_100072a0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        self.w8(a3, 0_u8);
        v5 = v4.wrapping_add(32_i32.wrapping_mul(a2));
        v6 = (self.r32((v5.wrapping_add(7624_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1719_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7628_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1719_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10007300` (95 bytes).
    pub(crate) fn f_10007300(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        self.w8(a3, 0_u8);
        v5 = v4.wrapping_add(32_i32.wrapping_mul(a2));
        v6 = (self.r32((v5.wrapping_add(7624_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1720_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7628_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1720_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10007360` (95 bytes).
    pub(crate) fn f_10007360(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        self.w8(a3, 0_u8);
        v5 = v4.wrapping_add(32_i32.wrapping_mul(a2));
        v6 = (self.r32((v5.wrapping_add(7624_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1721_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7628_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1721_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_100073C0` (95 bytes).
    pub(crate) fn f_100073c0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        self.w8(a3, 0_u8);
        v5 = v4.wrapping_add(32_i32.wrapping_mul(a2));
        v6 = (self.r32((v5.wrapping_add(7624_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1722_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7628_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1722_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10007420` (95 bytes).
    pub(crate) fn f_10007420(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        self.w8(a3, 0_u8);
        v5 = v4.wrapping_add(32_i32.wrapping_mul(a2));
        v6 = (self.r32((v5.wrapping_add(7624_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1723_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7628_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                    .wrapping_add(1600_i32) as u32),
            ) as i32)
                == 1723_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10007480` (90 bytes).
    pub(crate) fn f_10007480(&mut self, mut this: u32, mut a2: i32, mut a3: i8, mut a4: i8) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        v5 = (self.r32(self.r32(self.r32(this)).wrapping_add(16)) as i32);
        if (((a3 as i32) == 1_i32)
            && ((self.r32(
                (32_i32
                    .wrapping_mul(a2)
                    .wrapping_add(v5)
                    .wrapping_add(7624_i32) as u32),
            ) as i32)
                != (1_i32).wrapping_neg()))
        {
            let _ = self.f_10004aa0(
                self.r32(this.wrapping_add(8)),
                (self.r32(
                    (32_i32
                        .wrapping_mul(a2)
                        .wrapping_add(v5)
                        .wrapping_add(7624_i32) as u32),
                ) as i32),
            );
        }
        if ((a4 as i32) == 1_i32) {
            v6 = (self.r32(
                (32_i32
                    .wrapping_mul(a2)
                    .wrapping_add(v5)
                    .wrapping_add(7628_i32) as u32),
            ) as i32);
            if (v6 != (1_i32).wrapping_neg()) {
                let _ = self.f_10004aa0(self.r32(this.wrapping_add(8)), v6);
            }
        }
        return 0_i32;
    }

    /// `sub_100074E0` (66 bytes).
    pub(crate) fn f_100074e0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v4: i32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32)
            .wrapping_add(716_i32.wrapping_mul(a2));
        if (a3 > 3_i32) {
            self.w32(
                (v4.wrapping_add(4_i32.wrapping_mul(a4))
                    .wrapping_add(312_i32) as u32),
                3_u32,
            );
        } else {
            self.w32(
                (v4.wrapping_add(4_i32.wrapping_mul(a4))
                    .wrapping_add(272_i32) as u32),
                3_u32,
            );
        }
        return 0_i32;
    }

    /// `sub_10007530` (318 bytes).
    pub(crate) fn f_10007530(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: u32 = 0;
        let mut result: i32 = 0;
        let mut v5: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32))
                        as i32)
                        .wrapping_add(716_i32.wrapping_mul(a2)) as u32);
                    let t1 = a3;
                    bb = match t1 {
                        1_i32 => 2,
                        5_i32 => 3,
                        7_i32 => 4,
                        10_i32 => 5,
                        2_i32 => 6,
                        36_i32 => 7,
                        42_i32 => 8,
                        52_i32 => 9,
                        3_i32 => 10,
                        11_i32 => 11,
                        23_i32 => 12,
                        38_i32 => 13,
                        4_i32 => 14,
                        12_i32 => 15,
                        60_i32 => 16,
                        63_i32 => 17,
                        6_i32 => 18,
                        9_i32 => 19,
                        13_i32 => 20,
                        57_i32 => 21,
                        8_i32 => 22,
                        33_i32 => 23,
                        48_i32 => 24,
                        50_i32 => 25,
                        14_i32 => 26,
                        17_i32 => 27,
                        25_i32 => 28,
                        47_i32 => 29,
                        53_i32 => 30,
                        69_i32 => 31,
                        15_i32 => 32,
                        16_i32 => 33,
                        19_i32 => 34,
                        46_i32 => 35,
                        49_i32 => 36,
                        67_i32 => 37,
                        18_i32 => 38,
                        21_i32 => 39,
                        31_i32 => 40,
                        32_i32 => 41,
                        64_i32 => 42,
                        65_i32 => 43,
                        66_i32 => 44,
                        72_i32 => 45,
                        74_i32 => 46,
                        20_i32 => 47,
                        29_i32 => 48,
                        30_i32 => 49,
                        34_i32 => 50,
                        68_i32 => 51,
                        70_i32 => 52,
                        22_i32 => 53,
                        24_i32 => 54,
                        26_i32 => 55,
                        27_i32 => 56,
                        28_i32 => 57,
                        45_i32 => 58,
                        71_i32 => 59,
                        73_i32 => 60,
                        35_i32 => 61,
                        37_i32 => 62,
                        39_i32 => 63,
                        54_i32 => 64,
                        40_i32 => 65,
                        41_i32 => 66,
                        56_i32 => 67,
                        62_i32 => 68,
                        43_i32 => 69,
                        51_i32 => 70,
                        55_i32 => 71,
                        58_i32 => 72,
                        61_i32 => 73,
                        44_i32 => 74,
                        59_i32 => 75,
                        _ => 76,
                    };
                }
                1 => {
                    return result;
                }
                2 => {
                    bb = 3;
                }
                3 => {
                    bb = 4;
                }
                4 => {
                    bb = 5;
                }
                5 => {
                    result = 0_i32;
                    self.w32(v3.wrapping_add(272), 3_u32);
                    bb = 1;
                }
                6 => {
                    bb = 7;
                }
                7 => {
                    bb = 8;
                }
                8 => {
                    bb = 9;
                }
                9 => {
                    self.w32(v3.wrapping_add(280), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                10 => {
                    bb = 11;
                }
                11 => {
                    bb = 12;
                }
                12 => {
                    bb = 13;
                }
                13 => {
                    self.w32(v3.wrapping_add(296), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                14 => {
                    bb = 15;
                }
                15 => {
                    bb = 16;
                }
                16 => {
                    bb = 17;
                }
                17 => {
                    self.w32(v3.wrapping_add(304), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                18 => {
                    bb = 19;
                }
                19 => {
                    bb = 20;
                }
                20 => {
                    bb = 21;
                }
                21 => {
                    self.w32(v3.wrapping_add(308), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                22 => {
                    bb = 23;
                }
                23 => {
                    bb = 24;
                }
                24 => {
                    bb = 25;
                }
                25 => {
                    self.w32(v3.wrapping_add(300), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                26 => {
                    bb = 27;
                }
                27 => {
                    bb = 28;
                }
                28 => {
                    bb = 29;
                }
                29 => {
                    bb = 30;
                }
                30 => {
                    bb = 31;
                }
                31 => {
                    v5 = 1_i32;
                    bb = 83;
                }
                32 => {
                    bb = 33;
                }
                33 => {
                    bb = 34;
                }
                34 => {
                    bb = 35;
                }
                35 => {
                    bb = 36;
                }
                36 => {
                    bb = 37;
                }
                37 => {
                    result = 0_i32;
                    self.w32(v3.wrapping_add(312), 3_u32);
                    bb = 1;
                }
                38 => {
                    bb = 39;
                }
                39 => {
                    bb = 40;
                }
                40 => {
                    bb = 41;
                }
                41 => {
                    bb = 42;
                }
                42 => {
                    bb = 43;
                }
                43 => {
                    bb = 44;
                }
                44 => {
                    bb = 45;
                }
                45 => {
                    bb = 46;
                }
                46 => {
                    self.w32(v3.wrapping_add(328), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                47 => {
                    bb = 48;
                }
                48 => {
                    bb = 49;
                }
                49 => {
                    bb = 50;
                }
                50 => {
                    bb = 51;
                }
                51 => {
                    bb = 52;
                }
                52 => {
                    self.w32(v3.wrapping_add(324), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                53 => {
                    bb = 54;
                }
                54 => {
                    bb = 55;
                }
                55 => {
                    bb = 56;
                }
                56 => {
                    bb = 57;
                }
                57 => {
                    bb = 58;
                }
                58 => {
                    bb = 59;
                }
                59 => {
                    bb = 60;
                }
                60 => {
                    self.w32(v3.wrapping_add(320), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                61 => {
                    bb = 62;
                }
                62 => {
                    bb = 63;
                }
                63 => {
                    bb = 64;
                }
                64 => {
                    self.w32(v3.wrapping_add(292), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                65 => {
                    bb = 66;
                }
                66 => {
                    bb = 67;
                }
                67 => {
                    bb = 68;
                }
                68 => {
                    self.w32(v3.wrapping_add(284), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                69 => {
                    bb = 70;
                }
                70 => {
                    bb = 71;
                }
                71 => {
                    bb = 72;
                }
                72 => {
                    bb = 73;
                }
                73 => {
                    self.w32(v3.wrapping_add(276), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                74 => {
                    bb = 75;
                }
                75 => {
                    self.w32(v3.wrapping_add(288), 3_u32);
                    result = 0_i32;
                    bb = 1;
                }
                76 => {
                    v5 = a2;
                    bb = if (a2 > 3_i32) { 93 } else { 95 };
                }
                77 => {
                    bb = 6;
                }
                78 => {
                    bb = 10;
                }
                79 => {
                    bb = 14;
                }
                80 => {
                    bb = 18;
                }
                81 => {
                    bb = 22;
                }
                82 => {
                    bb = 26;
                }
                83 => {
                    self.w32(
                        v3.wrapping_add((v5.wrapping_add(78_i32) as u32).wrapping_mul(4)),
                        3_u32,
                    );
                    result = 0_i32;
                    bb = 94;
                }
                84 => {
                    bb = 32;
                }
                85 => {
                    bb = 38;
                }
                86 => {
                    bb = 47;
                }
                87 => {
                    bb = 53;
                }
                88 => {
                    bb = 61;
                }
                89 => {
                    bb = 65;
                }
                90 => {
                    bb = 69;
                }
                91 => {
                    bb = 74;
                }
                92 => {
                    bb = 76;
                }
                93 => {
                    bb = 83;
                }
                94 => {
                    bb = 1;
                }
                95 => {
                    self.w32(
                        v3.wrapping_add((a2.wrapping_add(68_i32) as u32).wrapping_mul(4)),
                        3_u32,
                    );
                    result = 0_i32;
                    bb = 94;
                }
                96 => {
                    bb = 1;
                }
                97 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100077A0` (497 bytes).
    pub(crate) fn f_100077a0(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let fp = self.enter(64);
        let r = self.f_100077a0_body(fp, this, a2, a3);
        self.leave(64);
        r
    }

    fn f_100077a0_body(&mut self, fp: u32, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
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
        let mut v19: u32 = 0;
        let mut i: u32 = 0;
        let mut v21: u32 = 0;
        v4 = (self.r32(self.r32(this)) as i32);
        v5 = (self.r32((v4.wrapping_add(12_i32) as u32)) as i32);
        self.w32(
            fp.wrapping_add(4),
            ((self.r32((v4.wrapping_add(20_i32) as u32)) as i32) as u32),
        );
        if (a2 != 0) {
            if (a2 == 1_i32) {
                v6 = 1_i32;
                v7 = a3;
                v8 = (self.r32(
                    (v5.wrapping_add(156_i32.wrapping_mul(a3))
                        .wrapping_add(3732_i32) as u32),
                ) as i32);
                v10 = (self.r32(
                    (v5.wrapping_add(156_i32.wrapping_mul(a3))
                        .wrapping_add(3728_i32) as u32),
                ) as i32);
                v11 = (self.r32(
                    (v5.wrapping_add(156_i32.wrapping_mul(a3))
                        .wrapping_add(3724_i32) as u32),
                ) as i32);
                a2 = v11.wrapping_add(20_i32.wrapping_mul(v10));
            } else {
                v6 = a3;
                v7 = a3;
                v11 = a3;
                v10 = a3;
                v8 = a3;
            }
        } else {
            v6 = 0_i32;
            v7 = a3;
            v8 = (self.r32(
                (v5.wrapping_add(716_i32.wrapping_mul(a3))
                    .wrapping_add(388_i32) as u32),
            ) as i32);
            v9 = v5.wrapping_add(716_i32.wrapping_mul(a3));
            v10 = (self.r32((v9.wrapping_add(384_i32) as u32)) as i32);
            v11 = (self.r32((v9.wrapping_add(380_i32) as u32)) as i32);
            a2 = v11.wrapping_add(20_i32.wrapping_mul(v10));
        }
        let _ = self.f_100064e0(
            this,
            fp.wrapping_add(36),
            (fp.wrapping_add(24) as i32),
            v6,
            v7,
            0_i32,
        );
        self.w32(
            fp.wrapping_add(20),
            ((self.r32(fp.wrapping_add(52)) as i32) as u32),
        );
        let _ = self.f_10006940(
            this,
            fp.wrapping_add(8),
            (self.r32(fp.wrapping_add(36).wrapping_add(4)) as i32),
            (self.r32(fp.wrapping_add(36).wrapping_add(8)) as i32),
            (self.r32(fp.wrapping_add(36).wrapping_add(12)) as i32),
            (self.r32(fp.wrapping_add(52)) as i32),
        );
        v12 = (self.r32(self.r32(fp.wrapping_add(8))) as i32);
        v13 = (self.r32(self.r32(fp.wrapping_add(8)).wrapping_add(4)) as i32);
        self.w32(
            fp.wrapping_add(12),
            ((self.r32(self.r32(fp.wrapping_add(8)).wrapping_add(8)) as i32) as u32),
        );
        v14 = (self.r32(self.r32(fp.wrapping_add(8)).wrapping_add(12)) as i32);
        's1: {
            'b1_4: {
                'b1_3: {
                    'b1_2: {
                        'b1_1: {
                            'b1_0: {
                                match v8 {
                                    0_i32 => break 'b1_0,
                                    1_i32 => break 'b1_1,
                                    2_i32 => break 'b1_2,
                                    3_i32 => break 'b1_3,
                                    _ => break 'b1_4,
                                }
                            }
                            a3 = 16_i32;
                            break 's1;
                        }
                        a3 = 32_i32;
                        break 's1;
                    }
                    a3 = 64_i32;
                    break 's1;
                }
                a3 = 128_i32;
                break 's1;
            }
            break 's1;
        }
        v15 = (self.r32(fp.wrapping_add(4)) as i32);
        self.w32(
            fp.wrapping_add(0),
            self.r32(fp.wrapping_add(8)).wrapping_add(16),
        );
        v16 = v13.wrapping_add(v10);
        self.w32(fp.wrapping_add(16), (v12.wrapping_add(v11) as u32));
        v17 = (self.r32(fp.wrapping_add(12)) as i32)
            .wrapping_add(v12)
            .wrapping_add(v11);
        self.w32(
            ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(7400_i32) as u32),
            0_u32,
        );
        v18 = v13.wrapping_add(v10).wrapping_add(v14);
        if (v13.wrapping_add(v10) < v18) {
            v19 = (20_i32.wrapping_mul(v16) as u32);
            self.w32(fp.wrapping_add(4), (v18.wrapping_sub(v16) as u32));
            'l2: loop {
                i = self.r32(fp.wrapping_add(16));
                'l3: loop {
                    if !((i as i32) < v17) {
                        break 'l3;
                    }
                    if ((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0) {
                        if ((v19 < 400_u32) && (i < 20_u32)) {
                            v21 = i.wrapping_add(v19);
                            if (i.wrapping_add(v19) != (a2 as u32)) {
                                self.w32(
                                    (v15.wrapping_add(4_i32.wrapping_mul(
                                        (self.r32((v15.wrapping_add(7400_i32) as u32)) as i32),
                                    ))
                                    .wrapping_add(6800_i32)
                                        as u32),
                                    ((v21 as i32) as u32),
                                );
                                if (((self.r32(fp.wrapping_add(20)) as i32) == 1_i32)
                                    && ((a3 & (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                                        != 0_i32))
                                {
                                    self.w32(
                                        (v15.wrapping_add(4_i32.wrapping_mul(
                                            (self.r32((v15.wrapping_add(7400_i32) as u32)) as i32),
                                        ))
                                        .wrapping_add(6800_i32)
                                            as u32),
                                        ((v21.wrapping_add(400_u32) as i32) as u32),
                                    );
                                }
                                self.w32(
                                    (v15.wrapping_add(7400_i32) as u32),
                                    ((self.r32((v15.wrapping_add(7400_i32) as u32)) as i32)
                                        .wrapping_add(1)
                                        as u32),
                                );
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                }
                v19 = v19.wrapping_add(20_u32);
                self.w32(
                    fp.wrapping_add(4),
                    ((self.r32(fp.wrapping_add(4)) as i32).wrapping_sub(1) as u32),
                );
                if !((self.r32(fp.wrapping_add(4)) as i32) != 0) {
                    break 'l2;
                }
            }
        }
        return 0_i32;
    }

    /// `sub_100079B0` (201 bytes).
    pub(crate) fn f_100079b0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: u32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
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
                    self.w32(a5, 0_u32);
                    bb = if (a4 != 0) { 3 } else { 5 };
                }
                2 => {
                    bb = if (a2 == 1_i32) { 11 } else { 12 };
                }
                3 => {
                    bb = if (a4 != 1_i32) { 6 } else { 7 };
                }
                4 => {
                    self.w32(a5.wrapping_add(4), (v7 as u32));
                    bb = 8;
                }
                5 => {
                    v7 = (self.r32(v6.wrapping_add(332)) as i32);
                    bb = 4;
                }
                6 => {
                    bb = 8;
                }
                7 => {
                    v7 = (self.r32(v6.wrapping_add(336)) as i32);
                    bb = 4;
                }
                8 => {
                    self.w32(
                        a5.wrapping_add(8),
                        ((self.r32(v6.wrapping_add(340)) as i32) as u32),
                    );
                    self.w32(
                        a5.wrapping_add(12),
                        ((self.r32(v6.wrapping_add(344)) as i32) as u32),
                    );
                    self.w32(
                        a5.wrapping_add(16),
                        ((self.r32(v6.wrapping_add(348)) as i32) as u32),
                    );
                    self.w32(
                        a5.wrapping_add(20),
                        ((self.r32(v6.wrapping_add(352)) as i32) as u32),
                    );
                    return 0_i32;
                }
                9 => {
                    bb = 7;
                }
                10 => {
                    bb = 2;
                }
                11 => {
                    v9 = (v5
                        .wrapping_add(156_i32.wrapping_mul(a3))
                        .wrapping_add(3600_i32) as u32);
                    self.w32(a5, 1_u32);
                    bb = if (a4 != 0) { 13 } else { 15 };
                }
                12 => {
                    return 0_i32;
                }
                13 => {
                    bb = if (a4 != 1_i32) { 16 } else { 17 };
                }
                14 => {
                    self.w32(a5.wrapping_add(4), (v10 as u32));
                    bb = 18;
                }
                15 => {
                    v10 = (self.r32(
                        (v5.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3676_i32) as u32),
                    ) as i32);
                    bb = 14;
                }
                16 => {
                    bb = 18;
                }
                17 => {
                    v10 = (self.r32(
                        (v5.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3680_i32) as u32),
                    ) as i32);
                    bb = 14;
                }
                18 => {
                    self.w32(
                        a5.wrapping_add(8),
                        ((self.r32(v9.wrapping_add(84)) as i32) as u32),
                    );
                    self.w32(
                        a5.wrapping_add(12),
                        ((self.r32(v9.wrapping_add(88)) as i32) as u32),
                    );
                    self.w32(
                        a5.wrapping_add(16),
                        ((self.r32(v9.wrapping_add(92)) as i32) as u32),
                    );
                    self.w32(
                        a5.wrapping_add(20),
                        ((self.r32(v9.wrapping_add(96)) as i32) as u32),
                    );
                    return 0_i32;
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    bb = 12;
                }
                21 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10007A80` (162 bytes).
    pub(crate) fn f_10007a80(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32);
        if (a2 != 0) {
            if (a2 == 1_i32) {
                self.w32(a4, 1_u32);
                self.w32(
                    a4.wrapping_add(4),
                    ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3676_i32) as u32),
                    ) as i32) as u32),
                );
                self.w32(
                    a4.wrapping_add(8),
                    ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3684_i32) as u32),
                    ) as i32) as u32),
                );
                self.w32(
                    a4.wrapping_add(12),
                    ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3688_i32) as u32),
                    ) as i32) as u32),
                );
                self.w32(
                    a4.wrapping_add(16),
                    ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3692_i32) as u32),
                    ) as i32) as u32),
                );
                self.w32(
                    a4.wrapping_add(20),
                    ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3696_i32) as u32),
                    ) as i32) as u32),
                );
            }
            return 0_i32;
        } else {
            v5 = (v4.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
            self.w32(a4, 0_u32);
            self.w32(
                a4.wrapping_add(4),
                ((self.r32(v5.wrapping_add(132)) as i32) as u32),
            );
            self.w32(
                a4.wrapping_add(8),
                ((self.r32(v5.wrapping_add(340)) as i32) as u32),
            );
            self.w32(
                a4.wrapping_add(12),
                ((self.r32(v5.wrapping_add(344)) as i32) as u32),
            );
            self.w32(
                a4.wrapping_add(16),
                ((self.r32(v5.wrapping_add(348)) as i32) as u32),
            );
            self.w32(
                a4.wrapping_add(20),
                ((self.r32(v5.wrapping_add(352)) as i32) as u32),
            );
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10007B30` (358 bytes).
    pub(crate) fn f_10007b30(&mut self, mut a1: i32, mut a2: i32, mut a3: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: bool = false;
        v3 = (a1 as u32);
        v4 = (self.r32((a1.wrapping_add(20_i32) as u32)) as i32);
        if (v4 < (self.r32((a2.wrapping_add(20_i32) as u32)) as i32)) {
            v4 = (self.r32((a2.wrapping_add(20_i32) as u32)) as i32);
        }
        if (v4 >= 30_i32) {
            if (v4 >= 60_i32) {
                if (v4 >= 100_i32) {
                    v5 = (if (v4 >= 150_i32) { 50_i32 } else { 40_i32 });
                } else {
                    v5 = 30_i32;
                }
            } else {
                v5 = 20_i32;
            }
        } else {
            v5 = 10_i32;
        }
        if ((self.r32((a1 as u32)) as i32) != 0) {
            if ((self.r32((a1 as u32)) as i32) == 1_i32) {
                a1 = cdiv_i32(40_i32.wrapping_mul(v5), 100_i32);
            }
        } else {
            a1 = v5;
        }
        v6 = cdiv_i32(
            10_i32.wrapping_mul((self.r32(v3.wrapping_add(4)) as i32)),
            100_i32,
        );
        self.w32(a3, (v6 as u32));
        if (v6 < 2_i32) {
            self.w32(a3, 2_u32);
        }
        v7 = cdiv_i32(
            10_i32.wrapping_mul((self.r32(v3.wrapping_add(12)) as i32)),
            100_i32,
        );
        self.w32(a3.wrapping_add(4), (v7 as u32));
        if (v7 < 2_i32) {
            self.w32(a3.wrapping_add(4), 2_u32);
        }
        v8 = (((0x51EB851F_i64.wrapping_mul((v5 as i64)).wrapping_mul(
            (cdiv_i32(
                100_i32.wrapping_mul(
                    (self.r32(v3.wrapping_add(20)) as i32)
                        .wrapping_sub((self.r32((a2.wrapping_add(20_i32) as u32)) as i32)),
                ),
                (self.r32(v3.wrapping_add(20)) as i32),
            ) as i64),
        ) as u64)
            .wrapping_shr(32_i32 as u32) as i32)
            .wrapping_shr(5_i32 as u32) as u32);
        self.w32(
            a3.wrapping_add(8),
            (cdiv_i32(
                v5.wrapping_mul(cdiv_i32(
                    100_i32.wrapping_mul(
                        (self.r32(v3.wrapping_add(20)) as i32)
                            .wrapping_sub((self.r32((a2.wrapping_add(20_i32) as u32)) as i32)),
                    ),
                    (self.r32(v3.wrapping_add(20)) as i32),
                )),
                100_i32,
            )
            .wrapping_add(90_i32) as u32),
        );
        if ((v8
            .wrapping_add((v8).wrapping_shr(31_i32 as u32))
            .wrapping_add(90_u32) as i32)
            < 0_i32)
        {
            self.w32(a3.wrapping_add(8), 0_u32);
        }
        if ((self.r32(a3.wrapping_add(8)) as i32) > 100_i32) {
            self.w32(a3.wrapping_add(8), 100_u32);
        }
        v9 = cdiv_i32(
            a1.wrapping_mul(cdiv_i32(
                100_i32.wrapping_mul(
                    (self.r32(v3.wrapping_add(20)) as i32)
                        .wrapping_sub((self.r32((a2.wrapping_add(20_i32) as u32)) as i32)),
                ),
                (self.r32(v3.wrapping_add(20)) as i32),
            )),
            100_i32,
        )
        .wrapping_add(5_i32);
        v10 = (cdiv_i32(
            a1.wrapping_mul(cdiv_i32(
                100_i32.wrapping_mul(
                    (self.r32(v3.wrapping_add(20)) as i32)
                        .wrapping_sub((self.r32((a2.wrapping_add(20_i32) as u32)) as i32)),
                ),
                (self.r32(v3.wrapping_add(20)) as i32),
            )),
            100_i32,
        ) < 0_i32);
        self.w32(a3.wrapping_add(12), (v9 as u32));
        if ((v10 as i32) != (v9.overflowing_sub(5_i32).1 as i32)) {
            self.w32(a3.wrapping_add(12), 5_u32);
        }
        if ((self.r32(a3.wrapping_add(12)) as i32) > 100_i32) {
            self.w32(a3.wrapping_add(12), 100_u32);
        }
        return 0_i32;
    }

    /// `sub_10007CA0` (171 bytes).
    pub(crate) fn f_10007ca0(&mut self, mut a1: i32, mut a2: i32, mut a3: u32, mut a4: i32) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut result: i32 = 0;
        if (crem_i32(self.rand(), 100_i32) >= (self.r32(a3.wrapping_add(8)) as i32)) {
            self.w8((a4 as u32), 0_u8);
            self.w8((a4.wrapping_add(1_i32) as u32), 0_u8);
            self.w32((a4.wrapping_add(4_i32) as u32), 0_u32);
        } else {
            self.w8((a4 as u32), 1_u8);
            v4 = crem_i32(self.rand(), (self.r32(a3) as i32));
            if (v4 <= 0_i32) {
                v4 = 1_i32;
            }
            v5 = v4
                .wrapping_add((self.r32((a1.wrapping_add(4_i32) as u32)) as i32))
                .wrapping_sub((self.r32((a2.wrapping_add(8_i32) as u32)) as i32));
            self.w32((a4.wrapping_add(4_i32) as u32), (v5 as u32));
            if (v5 <= 0_i32) {
                self.w32((a4.wrapping_add(4_i32) as u32), (v4 as u32));
            }
            if (crem_i32(self.rand(), 100_i32) >= (self.r32(a3.wrapping_add(12)) as i32)) {
                self.w8((a4.wrapping_add(1_i32) as u32), 0_u8);
            } else {
                v6 = (self.r32((a4.wrapping_add(4_i32) as u32)) as i32);
                self.w8((a4.wrapping_add(1_i32) as u32), 1_u8);
                self.w32(
                    (a4.wrapping_add(4_i32) as u32),
                    (2_i32.wrapping_mul(v6) as u32),
                );
            }
        }
        v7 = crem_i32(self.rand(), (self.r32(a3.wrapping_add(4)) as i32));
        self.w32(
            (a4.wrapping_add(8_i32) as u32),
            (v7.wrapping_add((self.r32((a1.wrapping_add(12_i32) as u32)) as i32))
                .wrapping_sub((self.r32((a2.wrapping_add(16_i32) as u32)) as i32))
                as u32),
        );
        result = 0_i32;
        self.w32(
            (a4.wrapping_add(12_i32) as u32),
            (v7.wrapping_add((self.r32((a1.wrapping_add(12_i32) as u32)) as i32)) as u32),
        );
        return result;
    }

    /// `sub_10007D50` (87 bytes).
    pub(crate) fn f_10007d50(&mut self, mut a1: i32, mut a2: i32, mut a3: u32, mut a4: i32) -> i32 {
        let mut v4: i32 = 0;
        self.w8((a4 as u32), 1_u8);
        self.w8((a4.wrapping_add(1_i32) as u32), 0_u8);
        {
            let a0 = (a4.wrapping_add(4_i32) as u32);
            let a1 = (crem_i32(self.rand(), (self.r32(a3) as i32))
                .wrapping_add((self.r32((a1.wrapping_add(4_i32) as u32)) as i32))
                .wrapping_sub((self.r32((a2.wrapping_add(8_i32) as u32)) as i32))
                as u32);
            self.w32(a0, a1)
        };
        v4 = crem_i32(self.rand(), (self.r32(a3.wrapping_add(4)) as i32));
        self.w32(
            (a4.wrapping_add(8_i32) as u32),
            ((self.r32((a1.wrapping_add(12_i32) as u32)) as i32)
                .wrapping_add(v4)
                .wrapping_sub((self.r32((a2.wrapping_add(16_i32) as u32)) as i32))
                as u32),
        );
        self.w32(
            (a4.wrapping_add(12_i32) as u32),
            ((self.r32((a1.wrapping_add(12_i32) as u32)) as i32).wrapping_add(v4) as u32),
        );
        return 0_i32;
    }

    /// `sub_10007DB0` (30 bytes).
    pub(crate) fn f_10007db0(&mut self, mut a1: i32, mut a2: i32) -> i32 {
        if ((self.r32((a1.wrapping_add(4_i32) as u32)) as i32) == 1_i32) {
            self.w32(
                (a2.wrapping_add(4_i32) as u32),
                (cdiv_i32((self.r32((a2.wrapping_add(4_i32) as u32)) as i32), 2_i32) as u32),
            );
        }
        return 0_i32;
    }

    /// `sub_10007DD0` (20 bytes).
    pub(crate) fn f_10007dd0(&mut self, mut a1: i32) -> i32 {
        self.w32(
            (a1.wrapping_add(4_i32) as u32),
            (cdiv_i32((self.r32((a1.wrapping_add(4_i32) as u32)) as i32), 2_i32) as u32),
        );
        return 0_i32;
    }

    /// `sub_10007DF0` (95 bytes).
    pub(crate) fn f_10007df0(&mut self, mut a1: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v1 = (self.r32(a1.wrapping_add(4)) as i32);
        if (v1 >= 0_i32) {
            if (v1 > 999_i32) {
                self.w32(a1.wrapping_add(4), 999_u32);
            }
        } else {
            self.w32(a1.wrapping_add(4), 0_u32);
        }
        v2 = (self.r32(a1.wrapping_add(8)) as i32);
        if (v2 >= 0_i32) {
            if (v2 > 999_i32) {
                self.w32(a1.wrapping_add(8), 999_u32);
            }
        } else {
            self.w32(a1.wrapping_add(8), 0_u32);
        }
        v3 = (self.r32(a1.wrapping_add(12)) as i32);
        if (v3 >= 0_i32) {
            if (v3 > 999_i32) {
                self.w32(a1.wrapping_add(12), 999_u32);
            }
            return 0_i32;
        } else {
            self.w32(a1.wrapping_add(12), 0_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10007E50` (453 bytes).
    pub(crate) fn f_10007e50(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
    ) -> i32 {
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v6 = ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32))
                        as i32)
                        .wrapping_add(716_i32.wrapping_mul(a3)) as u32);
                    bb = if (a2 != 0) { 1 } else { 3 };
                }
                1 => {
                    bb = if (a2 == 1_i32) { 4 } else { 5 };
                }
                2 => {
                    return 0;
                }
                3 => {
                    v7 = v6.wrapping_add(408);
                    v14 = 15_i32;
                    bb = 24;
                }
                4 => {
                    v10 = v6.wrapping_add(408);
                    v11 = 15_i32;
                    bb = 6;
                }
                5 => {
                    return 0_i32;
                }
                6 => {
                    bb = if ((self.r32(v10) as i32) != 0) { 9 } else { 10 };
                }
                7 => {
                    bb = if (v11 != 0) { 6 } else { 8 };
                }
                8 => {
                    bb = 5;
                }
                9 => {
                    let t1 = (self.r32(v10) as i32);
                    bb = match t1 {
                        6_i32 => 12,
                        13_i32 => 13,
                        11_i32 => 14,
                        16_i32 => 15,
                        _ => 16,
                    };
                }
                10 => {
                    v10 = v10.wrapping_add(4);
                    v11 = v11.wrapping_sub(1);
                    bb = 7;
                }
                11 => {
                    bb = 10;
                }
                12 => {
                    bb = 13;
                }
                13 => {
                    v12 = 40_i32.wrapping_mul((self.r32(v6.wrapping_add(140)) as i32));
                    bb = 17;
                }
                14 => {
                    v13 = 5_i32.wrapping_mul((self.r32(v6.wrapping_add(140)) as i32));
                    bb = 19;
                }
                15 => {
                    v13 = (5_i32)
                        .wrapping_neg()
                        .wrapping_mul((self.r32(v6.wrapping_add(140)) as i32));
                    bb = 19;
                }
                16 => {
                    bb = 11;
                }
                17 => {
                    self.w32(
                        (a5.wrapping_add(8_i32) as u32),
                        ((self.r32((a5.wrapping_add(8_i32) as u32)) as i32)
                            .wrapping_add(cdiv_i32(v12, 100_i32)) as u32),
                    );
                    bb = 11;
                }
                18 => {
                    bb = 14;
                }
                19 => {
                    v12 = 4_i32.wrapping_mul(v13);
                    bb = 17;
                }
                20 => {
                    bb = 15;
                }
                21 => {
                    bb = 16;
                }
                22 => {
                    bb = 11;
                }
                23 => {
                    bb = 2;
                }
                24 => {
                    bb = if ((self.r32(v7) as i32) != 0) { 27 } else { 28 };
                }
                25 => {
                    bb = if (v14 != 0) { 24 } else { 26 };
                }
                26 => {
                    return 0_i32;
                }
                27 => {
                    let t2 = (self.r32(v7) as i32);
                    bb = match t2 {
                        1_i32 => 30,
                        2_i32 => 31,
                        3_i32 => 32,
                        10_i32 => 33,
                        16_i32 => 34,
                        5_i32 => 35,
                        6_i32 => 36,
                        13_i32 => 37,
                        7_i32 => 38,
                        21_i32 => 39,
                        8_i32 => 40,
                        _ => 41,
                    };
                }
                28 => {
                    v7 = v7.wrapping_add(4);
                    v14 = v14.wrapping_sub(1);
                    bb = 25;
                }
                29 => {
                    bb = 28;
                }
                30 => {
                    bb = 31;
                }
                31 => {
                    bb = 32;
                }
                32 => {
                    bb = 33;
                }
                33 => {
                    bb = 34;
                }
                34 => {
                    self.w32(
                        (a4.wrapping_add(4_i32) as u32),
                        ((self.r32((a4.wrapping_add(4_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                            20_i32.wrapping_mul((self.r32(v6.wrapping_add(132)) as i32)),
                            100_i32,
                        )) as u32),
                    );
                    bb = 29;
                }
                35 => {
                    self.w32(
                        (a4.wrapping_add(4_i32) as u32),
                        ((self.r32((a4.wrapping_add(4_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                            30_i32.wrapping_mul((self.r32(v6.wrapping_add(132)) as i32)),
                            100_i32,
                        )) as u32),
                    );
                    bb = 29;
                }
                36 => {
                    bb = 37;
                }
                37 => {
                    self.w32(
                        (a4.wrapping_add(4_i32) as u32),
                        ((self.r32((a4.wrapping_add(4_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                            40_i32.wrapping_mul((self.r32(v6.wrapping_add(132)) as i32)),
                            100_i32,
                        )) as u32),
                    );
                    self.w32(
                        (a4.wrapping_add(12_i32) as u32),
                        ((self.r32((a4.wrapping_add(12_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                            40_i32.wrapping_mul((self.r32(v6.wrapping_add(144)) as i32)),
                            100_i32,
                        )) as u32),
                    );
                    bb = 29;
                }
                38 => {
                    bb = 39;
                }
                39 => {
                    v8 = (self.r32((a6.wrapping_add(12_i32) as u32)) as i32).wrapping_add(50_i32);
                    self.w32(
                        (a6.wrapping_add(8_i32) as u32),
                        ((self.r32((a6.wrapping_add(8_i32) as u32)) as i32).wrapping_add(100_i32)
                            as u32),
                    );
                    bb = 45;
                }
                40 => {
                    v8 = (self.r32((a6.wrapping_add(12_i32) as u32)) as i32).wrapping_add(100_i32);
                    self.w32(
                        (a6.wrapping_add(8_i32) as u32),
                        ((self.r32((a6.wrapping_add(8_i32) as u32)) as i32).wrapping_add(100_i32)
                            as u32),
                    );
                    bb = 45;
                }
                41 => {
                    bb = 29;
                }
                42 => {
                    bb = 35;
                }
                43 => {
                    bb = 36;
                }
                44 => {
                    bb = 38;
                }
                45 => {
                    self.w32((a6.wrapping_add(12_i32) as u32), (v8 as u32));
                    bb = 29;
                }
                46 => {
                    bb = 40;
                }
                47 => {
                    bb = 41;
                }
                48 => {
                    bb = 29;
                }
                49 => {
                    bb = 2;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10008070` (844 bytes).
    pub(crate) fn f_10008070(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
        mut a7: i32,
    ) -> i32 {
        let fp = self.enter(32);
        let r = self.f_10008070_body(fp, this, a2, a3, a4, a5, a6, a7);
        self.leave(32);
        r
    }

    fn f_10008070_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
        mut a7: i32,
    ) -> i32 {
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
        let mut v19: bool = false;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: u32 = 0;
        let mut v24: i32 = 0;
        self.w32(fp.wrapping_add(16), (a2 as u32));
        self.w32(fp.wrapping_add(20), (a4 as u32));
        self.w32(fp.wrapping_add(24), (a5 as u32));
        self.w32(fp.wrapping_add(28), (a6 as u32));
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v7 = a3;
                    v8 = (self.r32(self.r32(this)) as i32);
                    v9 = (self.r32((v8.wrapping_add(16_i32) as u32)) as i32);
                    v10 = (self.r32((v8.wrapping_add(12_i32) as u32)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(a3));
                    self.w32(fp.wrapping_add(4), (v10 as u32));
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) != 0) {
                        1
                    } else {
                        3
                    };
                }
                1 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) == 1_i32) {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    return 0;
                }
                3 => {
                    v11 = (self.r32(fp.wrapping_add(24)) as i32);
                    v12 = a3;
                    self.w32(fp.wrapping_add(12), 15_u32);
                    self.w32(fp.wrapping_add(0), (v10.wrapping_add(408_i32) as u32));
                    v13 = (self.r32(fp.wrapping_add(20)) as i32);
                    bb = 17;
                }
                4 => {
                    v21 = (self.r32(fp.wrapping_add(28)) as i32);
                    v22 = a3;
                    v23 = (v10.wrapping_add(408_i32) as u32);
                    self.w32(fp.wrapping_add(16), 15_u32);
                    bb = 6;
                }
                5 => {
                    return 0_i32;
                }
                6 => {
                    bb = if (1_i32 != 0) { 7 } else { 8 };
                }
                7 => {
                    bb = if ((self.r32(v23) as i32) == 22_i32) {
                        9
                    } else {
                        10
                    };
                }
                8 => {
                    bb = 5;
                }
                9 => {
                    v24 = (self.r32(
                        (32_i32
                            .wrapping_mul(v7)
                            .wrapping_add(v9)
                            .wrapping_add(7604_i32) as u32),
                    ) as i32);
                    bb = if (v24 != (1_i32).wrapping_neg()) {
                        11
                    } else {
                        12
                    };
                }
                10 => {
                    v23 = v23.wrapping_add(4);
                    bb = if (!({
                        let t1 = (self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(16), (t1 as u32));
                        t1
                    } != 0))
                    {
                        13
                    } else {
                        14
                    };
                }
                11 => {
                    v22 = (self.r32(
                        ((self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32)
                            .wrapping_add(
                                140_i32.wrapping_mul(
                                    (self.r32(
                                        (v9.wrapping_add(4_i32.wrapping_mul(v24))
                                            .wrapping_add(1600_i32)
                                            as u32),
                                    ) as i32),
                                ),
                            )
                            .wrapping_add(48_i32) as u32),
                    ) as i32);
                    bb = 12;
                }
                12 => {
                    self.w32(
                        (v21.wrapping_add(8_i32) as u32),
                        ((self.r32((v21.wrapping_add(8_i32) as u32)) as i32)
                            .wrapping_add(cdiv_i32(150_i32.wrapping_mul(v22), 100_i32))
                            as u32),
                    );
                    bb = 10;
                }
                13 => {
                    bb = 8;
                }
                14 => {
                    v7 = a3;
                    bb = 6;
                }
                15 => {
                    bb = 14;
                }
                16 => {
                    bb = 2;
                }
                17 => {
                    bb = if (1_i32 != 0) { 18 } else { 19 };
                }
                18 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0) {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 2;
                }
                20 => {
                    let t2 = (self.r32(self.r32(fp.wrapping_add(0))) as i32);
                    bb = match t2 {
                        4_i32 => 23,
                        9_i32 => 24,
                        14_i32 => 25,
                        15_i32 => 26,
                        17_i32 => 27,
                        18_i32 => 28,
                        _ => 29,
                    };
                }
                21 => {
                    v19 = ((self.r32(fp.wrapping_add(12)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if v19 { 78 } else { 79 };
                }
                22 => {
                    bb = 21;
                }
                23 => {
                    v14 = 200_i32.wrapping_mul(
                        (self.r32(
                            ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(132_i32) as u32),
                        ) as i32),
                    );
                    bb = 30;
                }
                24 => {
                    v14 = 300_i32.wrapping_mul(
                        (self.r32(
                            ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(132_i32) as u32),
                        ) as i32),
                    );
                    bb = 30;
                }
                25 => {
                    let _ = self.f_10006c80(this, v7, fp.wrapping_add(16), fp.wrapping_add(24));
                    bb = if (v13 != 0) { 33 } else { 35 };
                }
                26 => {
                    let _ = self.f_10006d10(this, v7, fp.wrapping_add(16), fp.wrapping_add(24));
                    bb = if (v13 != 0) { 46 } else { 48 };
                }
                27 => {
                    bb = if (v13 != 0) { 59 } else { 61 };
                }
                28 => {
                    let _ = self.f_10006e30(this, v7, fp.wrapping_add(20), fp.wrapping_add(28));
                    bb = if (v13 != 0) { 65 } else { 67 };
                }
                29 => {
                    bb = 22;
                }
                30 => {
                    self.w32(
                        (v11.wrapping_add(4_i32) as u32),
                        ((self.r32((v11.wrapping_add(4_i32) as u32)) as i32)
                            .wrapping_add(cdiv_i32(v14, 100_i32)) as u32),
                    );
                    bb = 22;
                }
                31 => {
                    bb = 24;
                }
                32 => {
                    bb = 25;
                }
                33 => {
                    bb = if ((v13 != 1_i32)
                        || ((((self.r32(fp.wrapping_add(24)) as i32) as u8) as i32) != 1_i32))
                    {
                        36
                    } else {
                        37
                    };
                }
                34 => {
                    bb = if (v15 != (1_i32).wrapping_neg()) {
                        42
                    } else {
                        43
                    };
                }
                35 => {
                    bb = if ((((self.r32(fp.wrapping_add(16)) as i32) as u8) as i32) != 1_i32) {
                        39
                    } else {
                        40
                    };
                }
                36 => {
                    bb = 22;
                }
                37 => {
                    v15 = (self.r32(
                        (32_i32
                            .wrapping_mul(a3)
                            .wrapping_add(v9)
                            .wrapping_add(7604_i32) as u32),
                    ) as i32);
                    bb = 34;
                }
                38 => {
                    bb = 37;
                }
                39 => {
                    bb = 22;
                }
                40 => {
                    v15 = (self.r32(
                        (32_i32
                            .wrapping_mul(a3)
                            .wrapping_add(v9)
                            .wrapping_add(7600_i32) as u32),
                    ) as i32);
                    bb = 34;
                }
                41 => {
                    bb = 40;
                }
                42 => {
                    v12 = (self.r32(
                        ((self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32)
                            .wrapping_add(
                                140_i32.wrapping_mul(
                                    (self.r32(
                                        (v9.wrapping_add(4_i32.wrapping_mul(v15))
                                            .wrapping_add(1600_i32)
                                            as u32),
                                    ) as i32),
                                ),
                            )
                            .wrapping_add(44_i32) as u32),
                    ) as i32);
                    bb = 43;
                }
                43 => {
                    v16 = 25_i32.wrapping_mul(v12);
                    bb = 44;
                }
                44 => {
                    v14 = 2_i32.wrapping_mul(v16);
                    bb = 30;
                }
                45 => {
                    bb = 26;
                }
                46 => {
                    bb = if ((v13 != 1_i32)
                        || ((((self.r32(fp.wrapping_add(24)) as i32) as u8) as i32) != 1_i32))
                    {
                        49
                    } else {
                        50
                    };
                }
                47 => {
                    bb = 55;
                }
                48 => {
                    bb = if ((((self.r32(fp.wrapping_add(16)) as i32) as u8) as i32) != 1_i32) {
                        52
                    } else {
                        53
                    };
                }
                49 => {
                    bb = 22;
                }
                50 => {
                    v17 = (self.r32(
                        (32_i32
                            .wrapping_mul(a3)
                            .wrapping_add(v9)
                            .wrapping_add(7604_i32) as u32),
                    ) as i32);
                    bb = 47;
                }
                51 => {
                    bb = 50;
                }
                52 => {
                    bb = 22;
                }
                53 => {
                    v17 = (self.r32(
                        (32_i32
                            .wrapping_mul(a3)
                            .wrapping_add(v9)
                            .wrapping_add(7600_i32) as u32),
                    ) as i32);
                    bb = 47;
                }
                54 => {
                    bb = 53;
                }
                55 => {
                    bb = if (v17 != (1_i32).wrapping_neg()) {
                        56
                    } else {
                        57
                    };
                }
                56 => {
                    v12 = (self.r32(
                        ((self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32)
                            .wrapping_add(
                                140_i32.wrapping_mul(
                                    (self.r32(
                                        (v9.wrapping_add(4_i32.wrapping_mul(v17))
                                            .wrapping_add(1600_i32)
                                            as u32),
                                    ) as i32),
                                ),
                            )
                            .wrapping_add(44_i32) as u32),
                    ) as i32);
                    bb = 57;
                }
                57 => {
                    v14 = 75_i32.wrapping_mul(v12);
                    bb = 30;
                }
                58 => {
                    bb = 27;
                }
                59 => {
                    bb = if (v13 == 1_i32) { 62 } else { 63 };
                }
                60 => {
                    v17 = (self.r32(fp.wrapping_add(8)) as i32);
                    bb = 55;
                }
                61 => {
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(
                            (32_i32
                                .wrapping_mul(v7)
                                .wrapping_add(v9)
                                .wrapping_add(7600_i32) as u32),
                        ) as i32) as u32),
                    );
                    bb = 60;
                }
                62 => {
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(
                            (32_i32
                                .wrapping_mul(v7)
                                .wrapping_add(v9)
                                .wrapping_add(7604_i32) as u32),
                        ) as i32) as u32),
                    );
                    bb = 63;
                }
                63 => {
                    bb = 60;
                }
                64 => {
                    bb = 28;
                }
                65 => {
                    bb = if ((v13 != 1_i32)
                        || ((((self.r32(fp.wrapping_add(28)) as i32) as u8) as i32) != 1_i32))
                    {
                        68
                    } else {
                        69
                    };
                }
                66 => {
                    bb = if (v18 != (1_i32).wrapping_neg()) {
                        74
                    } else {
                        75
                    };
                }
                67 => {
                    bb = if ((((self.r32(fp.wrapping_add(20)) as i32) as u8) as i32) != 1_i32) {
                        71
                    } else {
                        72
                    };
                }
                68 => {
                    bb = 22;
                }
                69 => {
                    v18 = (self.r32(
                        (32_i32
                            .wrapping_mul(a3)
                            .wrapping_add(v9)
                            .wrapping_add(7604_i32) as u32),
                    ) as i32);
                    bb = 66;
                }
                70 => {
                    bb = 69;
                }
                71 => {
                    bb = 22;
                }
                72 => {
                    v18 = (self.r32(
                        (32_i32
                            .wrapping_mul(a3)
                            .wrapping_add(v9)
                            .wrapping_add(7600_i32) as u32),
                    ) as i32);
                    bb = 66;
                }
                73 => {
                    bb = 72;
                }
                74 => {
                    v12 = (self.r32(
                        ((self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32)
                            .wrapping_add(
                                140_i32.wrapping_mul(
                                    (self.r32(
                                        (v9.wrapping_add(4_i32.wrapping_mul(v18))
                                            .wrapping_add(1600_i32)
                                            as u32),
                                    ) as i32),
                                ),
                            )
                            .wrapping_add(44_i32) as u32),
                    ) as i32);
                    bb = 75;
                }
                75 => {
                    v16 = 45_i32.wrapping_mul(v12);
                    bb = 44;
                }
                76 => {
                    bb = 29;
                }
                77 => {
                    bb = 22;
                }
                78 => {
                    return 0_i32;
                }
                79 => {
                    v7 = a3;
                    bb = 17;
                }
                80 => {
                    bb = 79;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100083F0` (179 bytes).
    pub(crate) fn f_100083f0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: u32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v11: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v6 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32))
                        as i32);
                    self.w8(a6, 0_u8);
                    v7 = v6
                        .wrapping_add(156_i32.wrapping_mul(a4))
                        .wrapping_add(3600_i32);
                    bb = if (!(a2 != 0)) { 1 } else { 2 };
                }
                1 => {
                    v8 = (v6
                        .wrapping_add(716_i32.wrapping_mul(a3))
                        .wrapping_add(408_i32) as u32);
                    v11 = 15_i32;
                    bb = 3;
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    bb = if (1_i32 != 0) { 4 } else { 5 };
                }
                4 => {
                    v9 = (self.r32(v8) as i32);
                    bb = if ((self.r32(v8) as i32) != 0) { 6 } else { 7 };
                }
                5 => {
                    bb = 2;
                }
                6 => {
                    bb = if (v9 != 4_i32) { 8 } else { 9 };
                }
                7 => {
                    bb = 12;
                }
                8 => {
                    bb = if ((v9 != 9_i32) || (crem_i32(self.rand(), 100_i32) >= 30_i32)) {
                        10
                    } else {
                        11
                    };
                }
                9 => {
                    bb = if (crem_i32(self.rand(), 100_i32) < 15_i32) {
                        16
                    } else {
                        17
                    };
                }
                10 => {
                    bb = 12;
                }
                11 => {
                    bb = 14;
                }
                12 => {
                    v8 = v8.wrapping_add(4);
                    bb = if (!({
                        let t1 = v11.wrapping_sub(1);
                        v11 = t1;
                        t1
                    } != 0))
                    {
                        19
                    } else {
                        20
                    };
                }
                13 => {
                    bb = 11;
                }
                14 => {
                    self.w8(a6, 1_u8);
                    self.w8((a5 as u32), 1_u8);
                    self.w32(
                        (a5.wrapping_add(4_i32) as u32),
                        ((self.r32((v7.wrapping_add(20_i32) as u32)) as i32) as u32),
                    );
                    bb = 12;
                }
                15 => {
                    bb = 9;
                }
                16 => {
                    bb = 14;
                }
                17 => {
                    bb = 7;
                }
                18 => {
                    bb = 17;
                }
                19 => {
                    return 0_i32;
                }
                20 => {
                    bb = 3;
                }
                21 => {
                    bb = 20;
                }
                22 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100084B0` (185 bytes).
    pub(crate) fn f_100084b0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32))
                        as i32)
                        .wrapping_add(716_i32.wrapping_mul(a3));
                    self.w32(a4, 0_u32);
                    bb = if (a2 != 0) { 1 } else { 2 };
                }
                1 => {
                    return 0_i32;
                }
                2 => {
                    v5 = (v4.wrapping_add(408_i32) as u32);
                    v6 = 15_i32;
                    bb = 4;
                }
                3 => {
                    bb = 2;
                }
                4 => {
                    bb = if ((self.r32(v5) as i32) != 0) { 7 } else { 8 };
                }
                5 => {
                    bb = if (v6 != 0) { 4 } else { 6 };
                }
                6 => {
                    return 0_i32;
                }
                7 => {
                    let t1 = (self.r32(v5) as i32);
                    bb = match t1 {
                        6_i32 => 10,
                        12_i32 => 11,
                        20_i32 => 12,
                        13_i32 => 13,
                        _ => 14,
                    };
                }
                8 => {
                    v5 = v5.wrapping_add(4);
                    v6 = v6.wrapping_sub(1);
                    bb = 5;
                }
                9 => {
                    bb = 8;
                }
                10 => {
                    v7 = 200_i32.wrapping_mul((self.r32((v4.wrapping_add(260_i32) as u32)) as i32));
                    bb = 15;
                }
                11 => {
                    bb = 12;
                }
                12 => {
                    v7 = 100_i32.wrapping_mul((self.r32((v4.wrapping_add(260_i32) as u32)) as i32));
                    bb = 15;
                }
                13 => {
                    v7 = 150_i32.wrapping_mul((self.r32((v4.wrapping_add(260_i32) as u32)) as i32));
                    bb = 15;
                }
                14 => {
                    bb = 9;
                }
                15 => {
                    self.w32(
                        a4,
                        ((self.r32(a4) as i32).wrapping_add(cdiv_i32(v7, 100_i32)) as u32),
                    );
                    bb = 9;
                }
                16 => {
                    bb = 11;
                }
                17 => {
                    bb = 13;
                }
                18 => {
                    bb = 14;
                }
                19 => {
                    bb = 9;
                }
                20 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10008590` (83 bytes).
    pub(crate) fn f_10008590(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        if (a2 != 0) {
            return 0_i32;
        }
        v4 = ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32)
            .wrapping_add(716_i32.wrapping_mul(a3))
            .wrapping_add(408_i32) as u32);
        v5 = 15_i32;
        'l1: loop {
            if ((self.r32(v4) as i32) == 19_i32) {
                self.w32(a4, 75_u32);
            }
            v4 = v4.wrapping_add(4);
            v5 = v5.wrapping_sub(1);
            if !(v5 != 0) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_100085F0` (255 bytes).
    pub(crate) fn f_100085f0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: u32,
    ) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100085f0_body(fp, this, a2, a3, a4, a5);
        self.leave(16);
        r
    }

    fn f_100085f0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: u32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        self.w32(fp.wrapping_add(4), (a2 as u32));
        self.w32(fp.wrapping_add(8), (a3 as u32));
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v6 = (self.r32(fp.wrapping_add(8)) as i32);
                    v7 = (self.r32(
                        ((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(12_i32)
                            as u32),
                    ) as i32);
                    v8 = 180_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32));
                    self.w32(a5, 1_u32);
                    v9 = v7.wrapping_add(4_i32.wrapping_mul(v8.wrapping_sub(v6)));
                    bb = if (!((self.r32(fp.wrapping_add(4)) as i32) != 0)) {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    v10 = a4;
                    v11 = (v9.wrapping_add(528_i32) as u32);
                    self.w32(fp.wrapping_add(0), 2_u32);
                    bb = 3;
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    bb = if (1_i32 != 0) { 4 } else { 5 };
                }
                4 => {
                    bb = if ((self.r32(v11) as i32) != 0) { 6 } else { 7 };
                }
                5 => {
                    bb = 2;
                }
                6 => {
                    let t1 = (self.r32(v11) as i32);
                    bb = match t1 {
                        23_i32 => 9,
                        24_i32 => 10,
                        25_i32 => 11,
                        _ => 8,
                    };
                }
                7 => {
                    v11 = v11.wrapping_add(4);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(0), (t2 as u32));
                        t2
                    } != 0))
                    {
                        41
                    } else {
                        42
                    };
                }
                8 => {
                    bb = 7;
                }
                9 => {
                    let _ = self.f_10006d10(this, v6, fp.wrapping_add(8), fp.wrapping_add(4));
                    bb = if (v10 != 0) { 12 } else { 14 };
                }
                10 => {
                    let _ = self.f_10006d10(this, v6, fp.wrapping_add(8), fp.wrapping_add(4));
                    bb = if (v10 != 0) { 23 } else { 24 };
                }
                11 => {
                    let _ = self.f_10006bf0(this, v6, fp.wrapping_add(8), fp.wrapping_add(4));
                    bb = if (!(v10 != 0)) { 31 } else { 32 };
                }
                12 => {
                    bb = if ((v10 != 1_i32)
                        || ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) != 1_i32))
                    {
                        15
                    } else {
                        16
                    };
                }
                13 => {
                    bb = 21;
                }
                14 => {
                    bb = if ((((self.r32(fp.wrapping_add(8)) as i32) as u8) as i32) != 1_i32) {
                        18
                    } else {
                        19
                    };
                }
                15 => {
                    bb = 8;
                }
                16 => {
                    bb = 13;
                }
                17 => {
                    bb = 16;
                }
                18 => {
                    bb = 8;
                }
                19 => {
                    bb = 13;
                }
                20 => {
                    bb = 19;
                }
                21 => {
                    self.w32(a5, ((self.r32(a5) as i32).wrapping_add(1) as u32));
                    bb = 8;
                }
                22 => {
                    bb = 10;
                }
                23 => {
                    bb = 25;
                }
                24 => {
                    bb = if ((((self.r32(fp.wrapping_add(8)) as i32) as u8) as i32) != 1_i32) {
                        27
                    } else {
                        28
                    };
                }
                25 => {
                    bb = if ((v10 != 1_i32)
                        || ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) != 1_i32))
                    {
                        37
                    } else {
                        38
                    };
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    bb = 8;
                }
                28 => {
                    bb = 21;
                }
                29 => {
                    bb = 28;
                }
                30 => {
                    bb = 11;
                }
                31 => {
                    bb = if ((((self.r32(fp.wrapping_add(8)) as i32) as u8) as i32) != 1_i32) {
                        33
                    } else {
                        34
                    };
                }
                32 => {
                    bb = 25;
                }
                33 => {
                    bb = 8;
                }
                34 => {
                    bb = 21;
                }
                35 => {
                    bb = 34;
                }
                36 => {
                    bb = 32;
                }
                37 => {
                    bb = 8;
                }
                38 => {
                    bb = 21;
                }
                39 => {
                    bb = 38;
                }
                40 => {
                    bb = 8;
                }
                41 => {
                    return 0_i32;
                }
                42 => {
                    bb = 3;
                }
                43 => {
                    bb = 42;
                }
                44 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100086F0` (269 bytes).
    pub(crate) fn f_100086f0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: u32,
    ) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100086f0_body(fp, this, a2, a3, a4, a5);
        self.leave(16);
        r
    }

    fn f_100086f0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: u32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        self.w32(fp.wrapping_add(0), (a2 as u32));
        self.w32(fp.wrapping_add(4), (a3 as u32));
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    return 0_i32;
                }
                2 => {
                    v5 = (self.r32(
                        ((self.r32(
                            ((self.r32(((self.r32(this) as i32) as u32)) as i32)
                                .wrapping_add(12_i32) as u32),
                        ) as i32)
                            .wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(4)) as i32)),
                            )
                            .wrapping_add(544_i32) as u32),
                    ) as i32);
                    bb = if (!(v5 != 0)) { 4 } else { 5 };
                }
                3 => {
                    bb = 2;
                }
                4 => {
                    return 0_i32;
                }
                5 => {
                    v6 = v5.wrapping_sub(26_i32);
                    bb = if (v6 != 0) { 7 } else { 9 };
                }
                6 => {
                    bb = 5;
                }
                7 => {
                    v7 = v6.wrapping_sub(1_i32);
                    bb = if (v7 != 0) { 10 } else { 11 };
                }
                8 => {
                    return 0_i32;
                }
                9 => {
                    let _ = self.f_10006ec0(
                        this,
                        (self.r32(fp.wrapping_add(4)) as i32),
                        fp.wrapping_add(4),
                        fp.wrapping_add(0),
                    );
                    bb = if (a4 != 0) { 35 } else { 37 };
                }
                10 => {
                    bb = if (v7 == 1_i32) { 12 } else { 13 };
                }
                11 => {
                    let _ = self.f_10006d10(
                        this,
                        (self.r32(fp.wrapping_add(4)) as i32),
                        fp.wrapping_add(4),
                        fp.wrapping_add(0),
                    );
                    bb = if (!(a4 != 0)) { 25 } else { 26 };
                }
                12 => {
                    let _ = self.f_10006f50(
                        this,
                        (self.r32(fp.wrapping_add(4)) as i32),
                        fp.wrapping_add(4),
                        fp.wrapping_add(0),
                    );
                    bb = if (!(a4 != 0)) { 14 } else { 15 };
                }
                13 => {
                    return 0_i32;
                }
                14 => {
                    bb = if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) != 1_i32) {
                        16
                    } else {
                        17
                    };
                }
                15 => {
                    bb = if ((a4 == 1_i32)
                        && ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                16 => {
                    return 0_i32;
                }
                17 => {
                    bb = 19;
                }
                18 => {
                    bb = 17;
                }
                19 => {
                    self.w32(a5, 21_u32);
                    return 0_i32;
                }
                20 => {
                    bb = 15;
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    bb = 13;
                }
                23 => {
                    bb = 22;
                }
                24 => {
                    bb = 11;
                }
                25 => {
                    bb = if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) != 1_i32) {
                        27
                    } else {
                        28
                    };
                }
                26 => {
                    bb = if ((a4 == 1_i32)
                        && ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32))
                    {
                        32
                    } else {
                        33
                    };
                }
                27 => {
                    return 0_i32;
                }
                28 => {
                    bb = 30;
                }
                29 => {
                    bb = 28;
                }
                30 => {
                    self.w32(a5, 20_u32);
                    return 0_i32;
                }
                31 => {
                    bb = 26;
                }
                32 => {
                    bb = 30;
                }
                33 => {
                    bb = 8;
                }
                34 => {
                    bb = 33;
                }
                35 => {
                    bb = if ((a4 != 1_i32)
                        || ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) != 1_i32))
                    {
                        38
                    } else {
                        39
                    };
                }
                36 => {
                    self.w32(a5, 19_u32);
                    bb = 8;
                }
                37 => {
                    bb = if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) != 1_i32) {
                        41
                    } else {
                        42
                    };
                }
                38 => {
                    return 0_i32;
                }
                39 => {
                    bb = 36;
                }
                40 => {
                    bb = 39;
                }
                41 => {
                    return 0_i32;
                }
                42 => {
                    bb = 36;
                }
                43 => {
                    bb = 42;
                }
                44 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10008800` (202 bytes).
    pub(crate) fn f_10008800(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        v4 = (self.r32(self.r32(this)) as i32);
        v5 = (self.r32((v4.wrapping_add(12_i32) as u32)) as i32);
        v6 = self.r32((v4.wrapping_add(28_i32) as u32));
        v7 = (v5.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
        if ((a2 == 1_i32) && ((self.r32(v7.wrapping_add(552)) as i32) == 29_i32)) {
            self.w32((a4.wrapping_add(8_i32) as u32), 0_u32);
            self.w32((a4.wrapping_add(12_i32) as u32), 0_u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(552);
                'l1: loop {
                    if ((self.r32(v9) as i32) == 29_i32) {
                        break 'l1;
                    }
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    if !(v8 < 1_i32) {
                        break 'l1;
                    }
                }
                v10 = (self.r32(v7.wrapping_add((v8.wrapping_add(139_i32) as u32).wrapping_mul(4)))
                    as i32)
                    .wrapping_sub(1_i32);
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(139_i32) as u32).wrapping_mul(4)),
                    (v10 as u32),
                );
                if (v10 <= 0_i32) {
                    self.w32(
                        v7.wrapping_add((v8.wrapping_add(138_i32) as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    v11 = (self.r32(v6.wrapping_add(2280)) as i32);
                    self.w32(v6.wrapping_add(2188), 1_u32);
                    self.w32(v6.wrapping_add(2192), 0_u32);
                    self.w32(v6.wrapping_add(2196), (a3 as u32));
                    self.w32(
                        v6.wrapping_add((v11.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        29_u32,
                    );
                    self.w32(
                        v6.wrapping_add(2280),
                        ((self.r32(v6.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                }
            }
        }
        return 0_i32;
    }

    /// `sub_100088D0` (292 bytes).
    pub(crate) fn f_100088d0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i8 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v14: i32 = 0;
        let mut v15: u32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: u32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v6 = (self.r32(self.r32(this)) as i32);
                    v7 = (self.r32((v6.wrapping_add(12_i32) as u32)) as i32);
                    v8 = self.r32((v6.wrapping_add(28_i32) as u32));
                    v9 = (v7.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
                    bb = if (!(a2 != 0)) { 1 } else { 2 };
                }
                1 => {
                    v10 = 0_i8;
                    bb = if ((self.r32(v8.wrapping_add(2188)) as i32) == 1_i32) {
                        3
                    } else {
                        4
                    };
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    v11 = (self.r32(v8.wrapping_add(2280)) as i32);
                    v12 = 0_i32;
                    bb = if (v11 > 0_i32) { 5 } else { 6 };
                }
                4 => {
                    bb = 12;
                }
                5 => {
                    v13 = v8.wrapping_add(2200);
                    bb = 7;
                }
                6 => {
                    bb = 4;
                }
                7 => {
                    bb = if ((self.r32(v13) as i32) != 30_i32) {
                        8
                    } else {
                        9
                    };
                }
                8 => {
                    v12 = v12.wrapping_add(1);
                    v13 = v13.wrapping_add(4);
                    bb = if (v12 >= v11) { 10 } else { 11 };
                }
                9 => {
                    bb = 14;
                }
                10 => {
                    bb = 12;
                }
                11 => {
                    bb = 7;
                }
                12 => {
                    bb = if ((self.r32(v9.wrapping_add(552)) as i32) == 30_i32) {
                        16
                    } else {
                        17
                    };
                }
                13 => {
                    bb = 11;
                }
                14 => {
                    v18 = (((0x200000001C_i64
                        .wrapping_mul(((self.r32(v9.wrapping_add(132)) as i32) as i64))
                        as u64)
                        .wrapping_shr(32_i32 as u32) as i32)
                        .wrapping_shr(5_i32 as u32) as u32);
                    self.w32(
                        (a4.wrapping_add(4_i32) as u32),
                        ((((self.r32((a4.wrapping_add(4_i32) as u32)) as i32) as u32)
                            .wrapping_add((v18).wrapping_shr(31_i32 as u32).wrapping_add(v18))
                            as i32) as u32),
                    );
                    self.w32(
                        (a6.wrapping_add(8_i32) as u32),
                        ((self.r32((a6.wrapping_add(8_i32) as u32)) as i32).wrapping_add(100_i32)
                            as u32),
                    );
                    bb = 29;
                }
                15 => {
                    bb = 6;
                }
                16 => {
                    bb = if (((self.r32(v9) as i32) == 1_i32)
                        && ((self.r32(v9.wrapping_add(372)) as i32) != 99_i32))
                    {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    bb = if ((v10 as i32) == 1_i32) { 28 } else { 29 };
                }
                18 => {
                    v14 = 0_i32;
                    v15 = v9.wrapping_add(552);
                    bb = 20;
                }
                19 => {
                    v10 = 1_i8;
                    bb = 17;
                }
                20 => {
                    bb = if ((self.r32(v15) as i32) == 30_i32) {
                        23
                    } else {
                        24
                    };
                }
                21 => {
                    bb = if (v14 < 1_i32) { 20 } else { 22 };
                }
                22 => {
                    v16 = (self
                        .r32(v9.wrapping_add((v14.wrapping_add(139_i32) as u32).wrapping_mul(4)))
                        as i32)
                        .wrapping_sub(1_i32);
                    self.w32(
                        v9.wrapping_add((v14.wrapping_add(139_i32) as u32).wrapping_mul(4)),
                        (v16 as u32),
                    );
                    bb = if (v16 <= 0_i32) { 26 } else { 27 };
                }
                23 => {
                    bb = 22;
                }
                24 => {
                    v14 = v14.wrapping_add(1);
                    v15 = v15.wrapping_add(4);
                    bb = 21;
                }
                25 => {
                    bb = 24;
                }
                26 => {
                    self.w32(
                        v9.wrapping_add((v14.wrapping_add(138_i32) as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    v17 = (self.r32(v8.wrapping_add(2280)) as i32);
                    self.w32(v8.wrapping_add(2188), 1_u32);
                    self.w32(v8.wrapping_add(2192), 0_u32);
                    self.w32(v8.wrapping_add(2196), (a3 as u32));
                    self.w32(
                        v8.wrapping_add((v17.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        30_u32,
                    );
                    self.w32(
                        v8.wrapping_add(2280),
                        ((self.r32(v8.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 27;
                }
                27 => {
                    bb = 19;
                }
                28 => {
                    bb = 14;
                }
                29 => {
                    bb = 2;
                }
                30 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10008A00` (282 bytes).
    pub(crate) fn f_10008a00(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: u32,
    ) -> i32 {
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i8 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v20: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v7 = (self.r32(self.r32(this)) as i32);
                    v8 = (self.r32((v7.wrapping_add(12_i32) as u32)) as i32);
                    v9 = self.r32((v7.wrapping_add(28_i32) as u32));
                    v10 = (v8.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
                    v20 = v8
                        .wrapping_add(156_i32.wrapping_mul(a4))
                        .wrapping_add(3600_i32);
                    bb = if (!(a2 != 0)) { 1 } else { 2 };
                }
                1 => {
                    v11 = 0_i8;
                    bb = if ((self.r32(v9.wrapping_add(2188)) as i32) == 1_i32) {
                        3
                    } else {
                        4
                    };
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    v12 = (self.r32(v9.wrapping_add(2280)) as i32);
                    v13 = 0_i32;
                    bb = if (v12 > 0_i32) { 5 } else { 6 };
                }
                4 => {
                    bb = 12;
                }
                5 => {
                    v14 = v9.wrapping_add(2200);
                    bb = 7;
                }
                6 => {
                    bb = 4;
                }
                7 => {
                    bb = if ((self.r32(v14) as i32) != 31_i32) {
                        8
                    } else {
                        9
                    };
                }
                8 => {
                    v13 = v13.wrapping_add(1);
                    v14 = v14.wrapping_add(4);
                    bb = if (v13 >= v12) { 10 } else { 11 };
                }
                9 => {
                    bb = 14;
                }
                10 => {
                    bb = 12;
                }
                11 => {
                    bb = 7;
                }
                12 => {
                    bb = if ((self.r32(v10.wrapping_add(552)) as i32) == 31_i32) {
                        16
                    } else {
                        17
                    };
                }
                13 => {
                    bb = 11;
                }
                14 => {
                    self.w8(a6, 1_u8);
                    self.w8((a5 as u32), 1_u8);
                    self.w32(
                        (a5.wrapping_add(4_i32) as u32),
                        ((self.r32((v20.wrapping_add(20_i32) as u32)) as i32) as u32),
                    );
                    bb = 29;
                }
                15 => {
                    bb = 6;
                }
                16 => {
                    bb = if (((self.r32(v10) as i32) == 1_i32)
                        && ((self.r32(v10.wrapping_add(372)) as i32) != 99_i32))
                    {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    bb = if ((v11 as i32) == 1_i32) { 28 } else { 29 };
                }
                18 => {
                    v15 = 0_i32;
                    v16 = v10.wrapping_add(552);
                    bb = 20;
                }
                19 => {
                    v11 = 1_i8;
                    bb = 17;
                }
                20 => {
                    bb = if ((self.r32(v16) as i32) == 31_i32) {
                        23
                    } else {
                        24
                    };
                }
                21 => {
                    bb = if (v15 < 1_i32) { 20 } else { 22 };
                }
                22 => {
                    v17 = (self
                        .r32(v10.wrapping_add((v15.wrapping_add(139_i32) as u32).wrapping_mul(4)))
                        as i32)
                        .wrapping_sub(1_i32);
                    self.w32(
                        v10.wrapping_add((v15.wrapping_add(139_i32) as u32).wrapping_mul(4)),
                        (v17 as u32),
                    );
                    bb = if (v17 <= 0_i32) { 26 } else { 27 };
                }
                23 => {
                    bb = 22;
                }
                24 => {
                    v15 = v15.wrapping_add(1);
                    v16 = v16.wrapping_add(4);
                    bb = 21;
                }
                25 => {
                    bb = 24;
                }
                26 => {
                    self.w32(
                        v10.wrapping_add((v15.wrapping_add(138_i32) as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    v18 = (self.r32(v9.wrapping_add(2280)) as i32);
                    self.w32(v9.wrapping_add(2188), 1_u32);
                    self.w32(v9.wrapping_add(2192), 0_u32);
                    self.w32(v9.wrapping_add(2196), (a3 as u32));
                    self.w32(
                        v9.wrapping_add((v18.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        31_u32,
                    );
                    self.w32(
                        v9.wrapping_add(2280),
                        ((self.r32(v9.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 27;
                }
                27 => {
                    bb = 19;
                }
                28 => {
                    bb = 14;
                }
                29 => {
                    bb = 2;
                }
                30 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10008B20` (70 bytes).
    pub(crate) fn f_10008b20(
        &mut self,
        mut this: u32,
        mut a1: i32,
        mut a2: i32,
        mut a3: i32,
    ) -> i32 {
        if ((a1 == 1_i32)
            && ((self.r32(
                ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32)
                    .wrapping_add(716_i32.wrapping_mul(a2))
                    .wrapping_add(560_i32) as u32),
            ) as i32)
                == 32_i32))
        {
            self.w32((a3.wrapping_add(8_i32) as u32), 0_u32);
            self.w32((a3.wrapping_add(12_i32) as u32), 0_u32);
        }
        return 0_i32;
    }

    /// `sub_10008B70` (88 bytes).
    pub(crate) fn f_10008b70(
        &mut self,
        mut this: u32,
        mut a1: i32,
        mut a2: i32,
        mut a3: u32,
    ) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        if (a1 != 1_i32) {
            return 0_i32;
        }
        v3 = ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32)
            .wrapping_add(716_i32.wrapping_mul(a2))
            .wrapping_add(592_i32) as u32);
        v4 = 3_i32;
        'l1: loop {
            if ((self.r32(v3) as i32) == 35_i32) {
                self.w32(a3, 1_u32);
            }
            v3 = v3.wrapping_add(4);
            v4 = v4.wrapping_sub(1);
            if !(v4 != 0) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10008BD0` (67 bytes).
    pub(crate) fn f_10008bd0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
    ) -> i32 {
        let mut v5: i32 = 0;
        if (!(a2 != 0)) {
            v5 = (self.r32(
                ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32)
                    .wrapping_add(156_i32.wrapping_mul(a4))
                    .wrapping_add(3740_i32) as u32),
            ) as i32);
            if ((v5 > 50_i32) && (v5 <= 53_i32)) {
                self.w32(
                    (a5.wrapping_add(8_i32) as u32),
                    ((self.r32((a5.wrapping_add(8_i32) as u32)) as i32).wrapping_add(100_i32)
                        as u32),
                );
            }
        }
        return 0_i32;
    }

    /// `sub_10008C20` (200 bytes).
    pub(crate) fn f_10008c20(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        v4 = (self.r32(self.r32(this)) as i32);
        v5 = (self.r32((v4.wrapping_add(12_i32) as u32)) as i32);
        v6 = self.r32((v4.wrapping_add(28_i32) as u32));
        v7 = (v5.wrapping_add(716_i32.wrapping_mul(a3)) as u32);
        if (((((!(a2 != 0)) && ((self.r32(v7.wrapping_add(696)) as i32) == 54_i32))
            && ((self.r8(a4) as i32) == 1_i32))
            && ((self.r32(v7) as i32) == 1_i32))
            && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
        {
            v8 = 0_i32;
            v9 = v7.wrapping_add(696);
            'l1: loop {
                if ((self.r32(v9) as i32) == 54_i32) {
                    break 'l1;
                }
                v8 = v8.wrapping_add(1);
                v9 = v9.wrapping_add(4);
                if !(v8 < 1_i32) {
                    break 'l1;
                }
            }
            v10 = (self.r32(v7.wrapping_add((v8.wrapping_add(175_i32) as u32).wrapping_mul(4)))
                as i32)
                .wrapping_sub(1_i32);
            self.w32(
                v7.wrapping_add((v8.wrapping_add(175_i32) as u32).wrapping_mul(4)),
                (v10 as u32),
            );
            if (v10 <= 0_i32) {
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(174_i32) as u32).wrapping_mul(4)),
                    0_u32,
                );
                v11 = (self.r32(v6.wrapping_add(2280)) as i32);
                self.w32(v6.wrapping_add(2188), 1_u32);
                self.w32(v6.wrapping_add(2192), 0_u32);
                self.w32(v6.wrapping_add(2196), (a3 as u32));
                self.w32(
                    v6.wrapping_add((v11.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                    54_u32,
                );
                self.w32(
                    v6.wrapping_add(2280),
                    ((self.r32(v6.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                );
            }
        }
        return 0_i32;
    }

    /// `sub_10008CF0` (196 bytes).
    pub(crate) fn f_10008cf0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
    ) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10008cf0_body(fp, this, a2, a3, a4);
        self.leave(16);
        r
    }

    fn f_10008cf0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        self.w32(fp.wrapping_add(0), (a2 as u32));
        self.w32(fp.wrapping_add(4), (a3 as u32));
        if ((self.r32(fp.wrapping_add(0)) as i32) == 1_i32) {
            v5 = (self.r32(fp.wrapping_add(4)) as i32);
            let _ = self.f_100072a0(
                this,
                (self.r32(fp.wrapping_add(4)) as i32),
                fp.wrapping_add(0),
                fp.wrapping_add(4),
            );
            v6 = a4;
            if (((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32)
                && (crem_i32(self.rand(), 100_i32) < 20_i32))
            {
                self.w32(v6, 1_u32);
            }
            if (((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) == 1_i32)
                && (crem_i32(self.rand(), 100_i32) < 20_i32))
            {
                self.w32(v6, 1_u32);
            }
            let _ = self.f_10007300(this, v5, fp.wrapping_add(0), fp.wrapping_add(4));
            if (((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32)
                && (crem_i32(self.rand(), 100_i32) < 25_i32))
            {
                self.w32(v6, 1_u32);
            }
            if (((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) == 1_i32)
                && (crem_i32(self.rand(), 100_i32) < 25_i32))
            {
                self.w32(v6, 1_u32);
            }
        }
        return 0_i32;
    }

    /// `sub_10008DC0` (505 bytes).
    pub(crate) fn f_10008dc0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
    ) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10008dc0_body(fp, this, a2, a3, a4, a5, a6);
        self.leave(16);
        r
    }

    fn f_10008dc0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
    ) -> i32 {
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        self.w32(fp.wrapping_add(0), (a2 as u32));
        self.w32(fp.wrapping_add(4), (a3 as u32));
        v7 = (self.r32(fp.wrapping_add(4)) as i32);
        v8 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32)) as i32)
            .wrapping_add(716_i32.wrapping_mul((self.r32(fp.wrapping_add(4)) as i32)));
        if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
            if ((self.r32(fp.wrapping_add(0)) as i32) == 1_i32) {
                let _ = self.f_10007240(
                    this,
                    (self.r32(fp.wrapping_add(4)) as i32),
                    fp.wrapping_add(4),
                    fp.wrapping_add(0),
                );
                v16 = a6;
                if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) == 1_i32) {
                    v17 = (self.r32((a6.wrapping_add(8_i32) as u32)) as i32);
                    self.w32(
                        (a6.wrapping_add(8_i32) as u32),
                        (v17.wrapping_sub(15_i32) as u32),
                    );
                    if (v17.wrapping_sub(15_i32) < 0_i32) {
                        self.w32((v16.wrapping_add(8_i32) as u32), 0_u32);
                    }
                }
                if ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32) {
                    v18 = (self.r32((v16.wrapping_add(8_i32) as u32)) as i32);
                    self.w32(
                        (v16.wrapping_add(8_i32) as u32),
                        (v18.wrapping_sub(15_i32) as u32),
                    );
                    if (v18.wrapping_sub(15_i32) < 0_i32) {
                        self.w32((v16.wrapping_add(8_i32) as u32), 0_u32);
                    }
                }
            }
        } else {
            let _ = self.f_100070c0(
                this,
                (self.r32(fp.wrapping_add(4)) as i32),
                fp.wrapping_add(4),
                fp.wrapping_add(0),
            );
            v9 = a4;
            if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) == 1_i32) {
                self.w32(
                    (a4.wrapping_add(4_i32) as u32),
                    ((self.r32((a4.wrapping_add(4_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                        10_i32.wrapping_mul((self.r32((v8.wrapping_add(132_i32) as u32)) as i32)),
                        100_i32,
                    )) as u32),
                );
            }
            if ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32) {
                self.w32(
                    (v9.wrapping_add(4_i32) as u32),
                    ((self.r32((v9.wrapping_add(4_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                        10_i32.wrapping_mul((self.r32((v8.wrapping_add(132_i32) as u32)) as i32)),
                        100_i32,
                    )) as u32),
                );
            }
            let _ = self.f_10007120(this, v7, fp.wrapping_add(4), fp.wrapping_add(0));
            if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) == 1_i32) {
                self.w32(
                    (v9.wrapping_add(12_i32) as u32),
                    ((self.r32((v9.wrapping_add(12_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                        10_i32.wrapping_mul((self.r32((v8.wrapping_add(144_i32) as u32)) as i32)),
                        100_i32,
                    )) as u32),
                );
            }
            if ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32) {
                self.w32(
                    (v9.wrapping_add(12_i32) as u32),
                    ((self.r32((v9.wrapping_add(12_i32) as u32)) as i32).wrapping_add(cdiv_i32(
                        10_i32.wrapping_mul((self.r32((v8.wrapping_add(144_i32) as u32)) as i32)),
                        100_i32,
                    )) as u32),
                );
            }
            let _ = self.f_10007180(this, v7, fp.wrapping_add(4), fp.wrapping_add(0));
            v10 = a6;
            if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) == 1_i32) {
                v11 = (self.r32((a6.wrapping_add(12_i32) as u32)) as i32).wrapping_add(10_i32);
                self.w32((a6.wrapping_add(12_i32) as u32), (v11 as u32));
                if (v11 > 100_i32) {
                    self.w32((v10.wrapping_add(12_i32) as u32), 100_u32);
                }
            }
            if ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32) {
                v12 = (self.r32((v10.wrapping_add(12_i32) as u32)) as i32).wrapping_add(10_i32);
                self.w32((v10.wrapping_add(12_i32) as u32), (v12 as u32));
                if (v12 > 100_i32) {
                    self.w32((v10.wrapping_add(12_i32) as u32), 100_u32);
                }
            }
            let _ = self.f_100071e0(this, v7, fp.wrapping_add(4), fp.wrapping_add(0));
            if ((((self.r32(fp.wrapping_add(4)) as i32) as u8) as i32) == 1_i32) {
                v13 = (self.r32((v10.wrapping_add(12_i32) as u32)) as i32).wrapping_add(15_i32);
                self.w32((v10.wrapping_add(12_i32) as u32), (v13 as u32));
                if (v13 > 100_i32) {
                    self.w32((v10.wrapping_add(12_i32) as u32), 100_u32);
                }
            }
            if ((((self.r32(fp.wrapping_add(0)) as i32) as u8) as i32) == 1_i32) {
                v14 = (self.r32((v10.wrapping_add(12_i32) as u32)) as i32).wrapping_add(15_i32);
                self.w32((v10.wrapping_add(12_i32) as u32), (v14 as u32));
                if (v14 > 100_i32) {
                    self.w32((v10.wrapping_add(12_i32) as u32), 100_u32);
                    return 0_i32;
                }
            }
        }
        return 0_i32;
    }

    /// `sub_10008FC0` (859 bytes).
    pub(crate) fn f_10008fc0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let fp = self.enter(128);
        let r = self.f_10008fc0_body(fp, this, a2, a3, a4, a5);
        self.leave(128);
        r
    }

    fn f_10008fc0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        v6 = (self.r32(self.r32(this)) as i32);
        v7 = (self.r32((v6.wrapping_add(12_i32) as u32)) as i32);
        v8 = self.r32((v6.wrapping_add(28_i32) as u32));
        self.w32(fp.wrapping_add(13), 1_u32);
        self.w32(fp.wrapping_add(33), (a3.wrapping_add(5_i32) as u32));
        v9 = (self.r32(
            (v7.wrapping_add(156_i32.wrapping_mul(a3))
                .wrapping_add(3604_i32) as u32),
        ) as i32);
        self.w32(fp.wrapping_add(17), 0_u32);
        self.w32(fp.wrapping_add(21), 0_u32);
        self.w32(fp.wrapping_add(25), 0_u32);
        's1: {
            'b1_1: {
                'b1_0: {
                    match v9 {
                        0_i32 | 1_i32 | 2_i32 | 3_i32 | 4_i32 | 5_i32 | 6_i32 => break 'b1_0,
                        _ => break 'b1_1,
                    }
                }
                self.w32(fp.wrapping_add(1), 0_u32);
                break 's1;
            }
            break 's1;
        }
        let _ = self.f_10008b70(this, 0_i32, a2, fp.wrapping_add(1));
        let _ = self.f_10008cf0(this, 0_i32, a2, fp.wrapping_add(1));
        let _ = self.f_100085f0(
            this,
            0_i32,
            a2,
            (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
            fp.wrapping_add(13),
        );
        self.w32(fp.wrapping_add(29), 0_u32);
        'l2: loop {
            if !((self.r32(fp.wrapping_add(29)) as i32) < (self.r32(fp.wrapping_add(13)) as i32)) {
                break 'l2;
            }
            let _ = self.f_100079b0(
                this,
                0_i32,
                a2,
                (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
                fp.wrapping_add(93),
            );
            let _ = self.f_100079b0(this, 1_i32, a3, 0_i32, fp.wrapping_add(69));
            let _ = self.f_10007b30(
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                fp.wrapping_add(53),
            );
            let _ = self.f_10007e50(
                this,
                0_i32,
                a2,
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008dc0(
                this,
                0_i32,
                a2,
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008070(
                this,
                0_i32,
                a2,
                (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008800(this, 0_i32, a2, (fp.wrapping_add(53) as i32));
            let _ = self.f_100088d0(
                this,
                0_i32,
                a2,
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008b20(this, 0_i32, a2, (fp.wrapping_add(53) as i32));
            let _ = self.f_10008bd0(this, 0_i32, a2, a3, (fp.wrapping_add(53) as i32));
            let _ = self.f_10007ca0(
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                fp.wrapping_add(53),
                (fp.wrapping_add(37) as i32),
            );
            let _ = self.f_10007db0(a5, (fp.wrapping_add(37) as i32));
            if ((self.r32(fp.wrapping_add(1)) as i32) == 1_i32) {
                let _ = self.f_10007dd0((fp.wrapping_add(37) as i32));
            }
            let _ = self.f_100083f0(
                this,
                0_i32,
                a2,
                a3,
                (fp.wrapping_add(37) as i32),
                fp.wrapping_add(0),
            );
            let _ = self.f_10008a00(
                this,
                0_i32,
                a2,
                a3,
                (fp.wrapping_add(37) as i32),
                fp.wrapping_add(0),
            );
            let _ = self.f_10008c20(this, 0_i32, a2, fp.wrapping_add(37));
            let _ = self.f_10007df0(fp.wrapping_add(37));
            if ((self.r32(fp.wrapping_add(29)) as i32) != 0) {
                if ((self.r32(fp.wrapping_add(29)) as i32) == 1_i32) {
                    self.w32(fp.wrapping_add(5), 1000_u32);
                } else {
                    if ((self.r32(fp.wrapping_add(29)) as i32) == 2_i32) {
                        self.w32(fp.wrapping_add(5), 0xF4240_u32);
                    }
                }
            } else {
                self.w32(fp.wrapping_add(5), 1_u32);
            }
            if ((((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 0) as u8) as i32) == 1_i32) {
                if ((((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 8) as u8) as i32)
                    == 1_i32)
                {
                    self.w32(fp.wrapping_add(9), 2_u32);
                } else {
                    if (!(((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 8) as u8) != 0)) {
                        self.w32(fp.wrapping_add(9), 1_u32);
                    }
                }
                if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                    self.w32(fp.wrapping_add(9), 3_u32);
                }
            } else {
                if (!(((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 0) as u8) != 0)) {
                    self.w32(fp.wrapping_add(9), 4_u32);
                }
            }
            self.w32(
                fp.wrapping_add(17),
                ((self.r32(fp.wrapping_add(17)) as i32).wrapping_add(
                    (self.r32(fp.wrapping_add(5)) as i32)
                        .wrapping_mul((self.r32(fp.wrapping_add(9)) as i32)),
                ) as u32),
            );
            self.w32(
                fp.wrapping_add(21),
                ((self.r32(fp.wrapping_add(21)) as i32).wrapping_add(
                    (self.r32(fp.wrapping_add(5)) as i32)
                        .wrapping_mul((self.r32(fp.wrapping_add(37).wrapping_add(4)) as i32)),
                ) as u32),
            );
            self.w32(
                fp.wrapping_add(25),
                ((self.r32(fp.wrapping_add(25)) as i32).wrapping_add(
                    (self.r32(fp.wrapping_add(1)) as i32)
                        .wrapping_mul((self.r32(fp.wrapping_add(5)) as i32)),
                ) as u32),
            );
            self.w32(
                fp.wrapping_add(29),
                ((self.r32(fp.wrapping_add(29)) as i32).wrapping_add(1) as u32),
            );
        }
        v10 = (self.r32(v8.wrapping_add(2072)) as i32);
        self.w32(v8.wrapping_add(1560), 1_u32);
        v11 = (self.r32(fp.wrapping_add(33)) as i32);
        self.w32(v8.wrapping_add(1564), ((self.r32(a4) as i32) as u32));
        self.w32(v8.wrapping_add(1568), 0_u32);
        self.w32(
            v8.wrapping_add((v10.wrapping_add(393_i32) as u32).wrapping_mul(4)),
            (v11 as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(13)) as i32) as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(17)) as i32) as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(21)) as i32) as u32),
        );
        {
            let a0 = v8.wrapping_add(
                ({
                    let t1 = (self.r32(v8.wrapping_add(2072)) as i32);
                    self.w32(v8.wrapping_add(2072), (t1.wrapping_add(1) as u32));
                    t1
                }
                .wrapping_add(493_i32) as u32)
                    .wrapping_mul(4),
            );
            let a1 = ((self.r32(fp.wrapping_add(25)) as i32) as u32);
            self.w32(a0, a1)
        };
        return 0_i32;
    }

    /// `sub_10009340` (859 bytes).
    pub(crate) fn f_10009340(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let fp = self.enter(128);
        let r = self.f_10009340_body(fp, this, a2, a3, a4, a5);
        self.leave(128);
        r
    }

    fn f_10009340_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        v6 = (self.r32(self.r32(this)) as i32);
        v7 = (self.r32((v6.wrapping_add(12_i32) as u32)) as i32);
        v8 = self.r32((v6.wrapping_add(28_i32) as u32));
        self.w32(fp.wrapping_add(13), 1_u32);
        self.w32(fp.wrapping_add(33), (a3.wrapping_add(5_i32) as u32));
        v9 = (self.r32(
            (v7.wrapping_add(156_i32.wrapping_mul(a3))
                .wrapping_add(3604_i32) as u32),
        ) as i32);
        self.w32(fp.wrapping_add(17), 0_u32);
        self.w32(fp.wrapping_add(21), 0_u32);
        self.w32(fp.wrapping_add(25), 0_u32);
        's1: {
            'b1_1: {
                'b1_0: {
                    match v9 {
                        0_i32 | 1_i32 | 2_i32 | 3_i32 | 4_i32 | 5_i32 | 6_i32 => break 'b1_0,
                        _ => break 'b1_1,
                    }
                }
                self.w32(fp.wrapping_add(1), 0_u32);
                break 's1;
            }
            break 's1;
        }
        let _ = self.f_10008b70(this, 0_i32, a2, fp.wrapping_add(1));
        let _ = self.f_10008cf0(this, 0_i32, a2, fp.wrapping_add(1));
        let _ = self.f_100085f0(
            this,
            0_i32,
            a2,
            (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
            fp.wrapping_add(13),
        );
        self.w32(fp.wrapping_add(29), 0_u32);
        'l2: loop {
            if !((self.r32(fp.wrapping_add(29)) as i32) < (self.r32(fp.wrapping_add(13)) as i32)) {
                break 'l2;
            }
            let _ = self.f_100079b0(
                this,
                0_i32,
                a2,
                (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
                fp.wrapping_add(93),
            );
            let _ = self.f_100079b0(this, 1_i32, a3, 0_i32, fp.wrapping_add(69));
            let _ = self.f_10007b30(
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                fp.wrapping_add(53),
            );
            let _ = self.f_10007e50(
                this,
                0_i32,
                a2,
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008dc0(
                this,
                0_i32,
                a2,
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008070(
                this,
                0_i32,
                a2,
                (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008800(this, 0_i32, a2, (fp.wrapping_add(53) as i32));
            let _ = self.f_100088d0(
                this,
                0_i32,
                a2,
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                (fp.wrapping_add(53) as i32),
            );
            let _ = self.f_10008b20(this, 0_i32, a2, (fp.wrapping_add(53) as i32));
            let _ = self.f_10008bd0(this, 0_i32, a2, a3, (fp.wrapping_add(53) as i32));
            let _ = self.f_10007ca0(
                (fp.wrapping_add(93) as i32),
                (fp.wrapping_add(69) as i32),
                fp.wrapping_add(53),
                (fp.wrapping_add(37) as i32),
            );
            let _ = self.f_10007db0(a5, (fp.wrapping_add(37) as i32));
            if ((self.r32(fp.wrapping_add(1)) as i32) == 1_i32) {
                let _ = self.f_10007dd0((fp.wrapping_add(37) as i32));
            }
            let _ = self.f_100083f0(
                this,
                0_i32,
                a2,
                a3,
                (fp.wrapping_add(37) as i32),
                fp.wrapping_add(0),
            );
            let _ = self.f_10008a00(
                this,
                0_i32,
                a2,
                a3,
                (fp.wrapping_add(37) as i32),
                fp.wrapping_add(0),
            );
            let _ = self.f_10008c20(this, 0_i32, a2, fp.wrapping_add(37));
            let _ = self.f_10007df0(fp.wrapping_add(37));
            if ((self.r32(fp.wrapping_add(29)) as i32) != 0) {
                if ((self.r32(fp.wrapping_add(29)) as i32) == 1_i32) {
                    self.w32(fp.wrapping_add(5), 1000_u32);
                } else {
                    if ((self.r32(fp.wrapping_add(29)) as i32) == 2_i32) {
                        self.w32(fp.wrapping_add(5), 0xF4240_u32);
                    }
                }
            } else {
                self.w32(fp.wrapping_add(5), 1_u32);
            }
            if ((((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 0) as u8) as i32) == 1_i32) {
                if ((((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 8) as u8) as i32)
                    == 1_i32)
                {
                    self.w32(fp.wrapping_add(9), 2_u32);
                } else {
                    if (!(((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 8) as u8) != 0)) {
                        self.w32(fp.wrapping_add(9), 1_u32);
                    }
                }
                if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                    self.w32(fp.wrapping_add(9), 3_u32);
                }
            } else {
                if (!(((((self.r32(fp.wrapping_add(37)) as i32) as u32) >> 0) as u8) != 0)) {
                    self.w32(fp.wrapping_add(9), 4_u32);
                }
            }
            self.w32(
                fp.wrapping_add(17),
                ((self.r32(fp.wrapping_add(17)) as i32).wrapping_add(
                    (self.r32(fp.wrapping_add(5)) as i32)
                        .wrapping_mul((self.r32(fp.wrapping_add(9)) as i32)),
                ) as u32),
            );
            self.w32(
                fp.wrapping_add(21),
                ((self.r32(fp.wrapping_add(21)) as i32).wrapping_add(
                    (self.r32(fp.wrapping_add(5)) as i32)
                        .wrapping_mul((self.r32(fp.wrapping_add(37).wrapping_add(4)) as i32)),
                ) as u32),
            );
            self.w32(
                fp.wrapping_add(25),
                ((self.r32(fp.wrapping_add(25)) as i32).wrapping_add(
                    (self.r32(fp.wrapping_add(1)) as i32)
                        .wrapping_mul((self.r32(fp.wrapping_add(5)) as i32)),
                ) as u32),
            );
            self.w32(
                fp.wrapping_add(29),
                ((self.r32(fp.wrapping_add(29)) as i32).wrapping_add(1) as u32),
            );
        }
        v10 = (self.r32(v8.wrapping_add(2072)) as i32);
        self.w32(v8.wrapping_add(1560), 1_u32);
        v11 = (self.r32(fp.wrapping_add(33)) as i32);
        self.w32(v8.wrapping_add(1564), ((self.r32(a4) as i32) as u32));
        self.w32(v8.wrapping_add(1568), 0_u32);
        self.w32(
            v8.wrapping_add((v10.wrapping_add(393_i32) as u32).wrapping_mul(4)),
            (v11 as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(13)) as i32) as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(17)) as i32) as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(21)) as i32) as u32),
        );
        {
            let a0 = v8.wrapping_add(
                ({
                    let t1 = (self.r32(v8.wrapping_add(2072)) as i32);
                    self.w32(v8.wrapping_add(2072), (t1.wrapping_add(1) as u32));
                    t1
                }
                .wrapping_add(493_i32) as u32)
                    .wrapping_mul(4),
            );
            let a1 = ((self.r32(fp.wrapping_add(25)) as i32) as u32);
            self.w32(a0, a1)
        };
        return 0_i32;
    }

    /// `sub_100096C0` (892 bytes).
    pub(crate) fn f_100096c0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let fp = self.enter(112);
        let r = self.f_100096c0_body(fp, this, a2, a3, a4, a5);
        self.leave(112);
        r
    }

    fn f_100096c0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        self.w32(fp.wrapping_add(108), (a3 as u32));
        v6 = (self.r32(fp.wrapping_add(108)) as i32);
        v7 = (self.r32(self.r32(this)) as i32);
        self.w32(fp.wrapping_add(12), 0_u32);
        self.w32(fp.wrapping_add(16), 0_u32);
        v8 = self.r32((v7.wrapping_add(28_i32) as u32));
        self.w32(fp.wrapping_add(8), 1_u32);
        self.w32(fp.wrapping_add(20), 0_u32);
        self.w32(
            fp.wrapping_add(4),
            ((((self.r32(
                v8.wrapping_add(
                    (16_i32
                        .wrapping_mul((self.r32(fp.wrapping_add(108)) as i32))
                        .wrapping_add(90_i32)
                        .wrapping_add(
                            (self.r32(
                                v8.wrapping_add(
                                    ((self.r32(fp.wrapping_add(108)) as i32).wrapping_add(250_i32)
                                        as u32)
                                        .wrapping_mul(4),
                                ),
                            ) as i32),
                        ) as u32)
                        .wrapping_mul(4),
                ),
            ) as i32)
                == 2_i32) as i32) as u32),
        );
        let _ = self.f_10008b70(
            this,
            1_i32,
            (self.r32(fp.wrapping_add(108)) as i32),
            fp.wrapping_add(4),
        );
        let _ = self.f_10008cf0(this, 1_i32, v6, fp.wrapping_add(4));
        let _ = self.f_100085f0(
            this,
            1_i32,
            v6,
            (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
            fp.wrapping_add(8),
        );
        self.w32(fp.wrapping_add(24), 0_u32);
        if ((self.r32(fp.wrapping_add(8)) as i32) > 0_i32) {
            v9 = a2;
            'l1: loop {
                let _ = self.f_100079b0(
                    this,
                    1_i32,
                    v9,
                    (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
                    fp.wrapping_add(84),
                );
                let _ = self.f_100079b0(this, 0_i32, v6, 0_i32, fp.wrapping_add(60));
                let _ = self.f_10007b30(
                    (fp.wrapping_add(84) as i32),
                    (fp.wrapping_add(60) as i32),
                    fp.wrapping_add(44),
                );
                let _ = self.f_10007e50(
                    this,
                    1_i32,
                    v6,
                    (fp.wrapping_add(84) as i32),
                    (fp.wrapping_add(60) as i32),
                    (fp.wrapping_add(44) as i32),
                );
                let _ = self.f_10008dc0(
                    this,
                    1_i32,
                    v6,
                    (fp.wrapping_add(84) as i32),
                    (fp.wrapping_add(60) as i32),
                    (fp.wrapping_add(44) as i32),
                );
                let _ = self.f_10008070(
                    this,
                    1_i32,
                    v6,
                    (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
                    (fp.wrapping_add(84) as i32),
                    (fp.wrapping_add(60) as i32),
                    (fp.wrapping_add(44) as i32),
                );
                let _ = self.f_10008800(this, 1_i32, v6, (fp.wrapping_add(44) as i32));
                let _ = self.f_100088d0(
                    this,
                    1_i32,
                    v6,
                    (fp.wrapping_add(84) as i32),
                    (fp.wrapping_add(60) as i32),
                    (fp.wrapping_add(44) as i32),
                );
                let _ = self.f_10008b20(this, 1_i32, v6, (fp.wrapping_add(44) as i32));
                let _ = self.f_10008bd0(this, 1_i32, v6, v9, (fp.wrapping_add(44) as i32));
                let _ = self.f_10007ca0(
                    (fp.wrapping_add(84) as i32),
                    (fp.wrapping_add(60) as i32),
                    fp.wrapping_add(44),
                    (fp.wrapping_add(28) as i32),
                );
                let _ = self.f_10007db0(a5, (fp.wrapping_add(28) as i32));
                if ((self.r32(fp.wrapping_add(4)) as i32) != 0) {
                    let _ = self.f_10007dd0((fp.wrapping_add(28) as i32));
                }
                let _ = self.f_100083f0(
                    this,
                    1_i32,
                    v6,
                    v9,
                    (fp.wrapping_add(28) as i32),
                    fp.wrapping_add(108),
                );
                let _ = self.f_10008a00(
                    this,
                    1_i32,
                    v6,
                    v9,
                    (fp.wrapping_add(28) as i32),
                    fp.wrapping_add(108),
                );
                let _ = self.f_10008c20(this, 1_i32, v6, fp.wrapping_add(28));
                let _ = self.f_10007df0(fp.wrapping_add(28));
                if ((self.r32(fp.wrapping_add(24)) as i32) != 0) {
                    if ((self.r32(fp.wrapping_add(24)) as i32) == 1_i32) {
                        a2 = 1000_i32;
                    } else {
                        if ((self.r32(fp.wrapping_add(24)) as i32) == 2_i32) {
                            a2 = 0xF4240_i32;
                        }
                    }
                } else {
                    a2 = 1_i32;
                }
                if ((((((self.r32(fp.wrapping_add(28)) as i32) as u32) >> 0) as u8) as i32)
                    == 1_i32)
                {
                    if ((((((self.r32(fp.wrapping_add(28)) as i32) as u32) >> 8) as u8) as i32)
                        == 1_i32)
                    {
                        self.w32(fp.wrapping_add(0), 2_u32);
                    } else {
                        if (!(((((self.r32(fp.wrapping_add(28)) as i32) as u32) >> 8) as u8) != 0))
                        {
                            self.w32(fp.wrapping_add(0), 1_u32);
                        }
                    }
                    if ((((self.r32(fp.wrapping_add(108)) as i32) as u8) as i32) == 1_i32) {
                        self.w32(fp.wrapping_add(0), 3_u32);
                    }
                } else {
                    if (!(((((self.r32(fp.wrapping_add(28)) as i32) as u32) >> 0) as u8) != 0)) {
                        self.w32(fp.wrapping_add(0), 4_u32);
                    }
                }
                self.w32(
                    fp.wrapping_add(12),
                    ((self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add(a2.wrapping_mul((self.r32(fp.wrapping_add(0)) as i32)))
                        as u32),
                );
                self.w32(
                    fp.wrapping_add(16),
                    ((self.r32(fp.wrapping_add(16)) as i32).wrapping_add(
                        a2.wrapping_mul((self.r32(fp.wrapping_add(28).wrapping_add(4)) as i32)),
                    ) as u32),
                );
                self.w32(
                    fp.wrapping_add(20),
                    ((self.r32(fp.wrapping_add(20)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(4)) as i32).wrapping_mul(a2))
                        as u32),
                );
                self.w32(
                    fp.wrapping_add(24),
                    ((self.r32(fp.wrapping_add(24)) as i32).wrapping_add(1) as u32),
                );
                if !((self.r32(fp.wrapping_add(24)) as i32) < (self.r32(fp.wrapping_add(8)) as i32))
                {
                    break 'l1;
                }
            }
        }
        v10 = a4;
        v11 = (self.r32(v8.wrapping_add(2072)) as i32);
        self.w32(v8.wrapping_add(1560), 1_u32);
        v12 = (self.r32(v10) as i32);
        self.w32(v8.wrapping_add(1568), 0_u32);
        self.w32(v8.wrapping_add(1564), (v12 as u32));
        self.w32(
            v8.wrapping_add((v11.wrapping_add(393_i32) as u32).wrapping_mul(4)),
            (v6 as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(8)) as i32) as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(12)) as i32) as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(16)) as i32) as u32),
        );
        self.w32(
            v8.wrapping_add(
                ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(493_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(20)) as i32) as u32),
        );
        v13 = (self.r32(v8.wrapping_add(2356)) as i32);
        self.w32(
            v8.wrapping_add(2072),
            ((self.r32(v8.wrapping_add(2072)) as i32).wrapping_add(1) as u32),
        );
        self.w32(
            v8.wrapping_add((v13.wrapping_add(584_i32) as u32).wrapping_mul(4)),
            (v6 as u32),
        );
        self.w32(
            v8.wrapping_add(2356),
            ((self.r32(v8.wrapping_add(2356)) as i32).wrapping_add(1) as u32),
        );
        return 0_i32;
    }

    /// `sub_10009A40` (537 bytes).
    pub(crate) fn f_10009a40(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let fp = self.enter(112);
        let r = self.f_10009a40_body(fp, this, a2, a3, a4, a5);
        self.leave(112);
        r
    }

    fn f_10009a40_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: i32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        v6 = (self.r32(self.r32(this)) as i32);
        v7 = self.r32((v6.wrapping_add(28_i32) as u32));
        self.w32(fp.wrapping_add(4), 0_u32);
        self.w32(fp.wrapping_add(8), 0_u32);
        self.w32(fp.wrapping_add(12), 0_u32);
        's1: {
            'b1_1: {
                'b1_0: {
                    match (self.r32(
                        ((self.r32((v6.wrapping_add(12_i32) as u32)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(a3))
                            .wrapping_add(3604_i32) as u32),
                    ) as i32)
                    {
                        0_i32 | 1_i32 | 2_i32 | 3_i32 | 4_i32 | 5_i32 | 6_i32 => break 'b1_0,
                        _ => break 'b1_1,
                    }
                }
                v8 = 0_i32;
                break 's1;
            }
            v8 = a3;
            break 's1;
        }
        v9 = a3;
        self.w32(fp.wrapping_add(16), 0_u32);
        'l2: loop {
            if !((self.r32(fp.wrapping_add(16)) as i32) < 1_i32) {
                break 'l2;
            }
            let _ = self.f_100079b0(
                this,
                1_i32,
                a2,
                (self.r32((a5.wrapping_add(8_i32) as u32)) as i32),
                fp.wrapping_add(80),
            );
            let _ = self.f_100079b0(this, 1_i32, a3, 0_i32, fp.wrapping_add(56));
            let _ = self.f_10007b30(
                (fp.wrapping_add(80) as i32),
                (fp.wrapping_add(56) as i32),
                fp.wrapping_add(40),
            );
            let _ = self.f_10007ca0(
                (fp.wrapping_add(80) as i32),
                (fp.wrapping_add(56) as i32),
                fp.wrapping_add(40),
                (fp.wrapping_add(24) as i32),
            );
            let _ = self.f_10007db0(a5, (fp.wrapping_add(24) as i32));
            if (v8 == 1_i32) {
                let _ = self.f_10007dd0((fp.wrapping_add(24) as i32));
            }
            let _ = self.f_10007df0(fp.wrapping_add(24));
            if (!((self.r32(fp.wrapping_add(16)) as i32) != 0)) {
                v9 = 1_i32;
            }
            if ((((((self.r32(fp.wrapping_add(24)) as i32) as u32) >> 0) as u8) as i32) == 1_i32) {
                if ((((((self.r32(fp.wrapping_add(24)) as i32) as u32) >> 8) as u8) as i32)
                    == 1_i32)
                {
                    self.w32(fp.wrapping_add(0), 2_u32);
                } else {
                    if (!(((((self.r32(fp.wrapping_add(24)) as i32) as u32) >> 8) as u8) != 0)) {
                        self.w32(fp.wrapping_add(0), 1_u32);
                    }
                }
            } else {
                if (!(((((self.r32(fp.wrapping_add(24)) as i32) as u32) >> 0) as u8) != 0)) {
                    self.w32(fp.wrapping_add(0), 4_u32);
                }
            }
            self.w32(
                fp.wrapping_add(4),
                ((self.r32(fp.wrapping_add(4)) as i32)
                    .wrapping_add(v9.wrapping_mul((self.r32(fp.wrapping_add(0)) as i32)))
                    as u32),
            );
            self.w32(
                fp.wrapping_add(8),
                ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(
                    v9.wrapping_mul((self.r32(fp.wrapping_add(24).wrapping_add(4)) as i32)),
                ) as u32),
            );
            self.w32(
                fp.wrapping_add(12),
                ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(v8.wrapping_mul(v9)) as u32),
            );
            self.w32(
                fp.wrapping_add(16),
                ((self.r32(fp.wrapping_add(16)) as i32).wrapping_add(1) as u32),
            );
        }
        self.w32(v7.wrapping_add(1560), 1_u32);
        v10 = (self.r32(v7.wrapping_add(2072)) as i32);
        self.w32(v7.wrapping_add(1564), ((self.r32(a4) as i32) as u32));
        self.w32(v7.wrapping_add(1568), 1_u32);
        self.w32(
            v7.wrapping_add((v10.wrapping_add(393_i32) as u32).wrapping_mul(4)),
            (a3.wrapping_add(5_i32) as u32),
        );
        self.w32(
            v7.wrapping_add(
                ((self.r32(v7.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                    .wrapping_mul(4),
            ),
            1_u32,
        );
        self.w32(
            v7.wrapping_add(
                ((self.r32(v7.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(4)) as i32) as u32),
        );
        self.w32(
            v7.wrapping_add(
                ((self.r32(v7.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                    .wrapping_mul(4),
            ),
            ((self.r32(fp.wrapping_add(8)) as i32) as u32),
        );
        {
            let a0 = v7.wrapping_add(
                ({
                    let t1 = (self.r32(v7.wrapping_add(2072)) as i32);
                    self.w32(v7.wrapping_add(2072), (t1.wrapping_add(1) as u32));
                    t1
                }
                .wrapping_add(493_i32) as u32)
                    .wrapping_mul(4),
            );
            let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
            self.w32(a0, a1)
        };
        return 0_i32;
    }

    /// `sub_10009C80` (483 bytes).
    pub(crate) fn f_10009c80(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let fp = self.enter(80);
        let r = self.f_10009c80_body(fp, this, a2, a3, a4);
        self.leave(80);
        r
    }

    fn f_10009c80_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v5 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32))
                        as i32);
                    let _ = self.f_10007a80(this, 0_i32, a2, fp.wrapping_add(32));
                    let _ = self.f_100079b0(this, 1_i32, a3, 0_i32, fp.wrapping_add(56));
                    let _ = self.f_10007b30(
                        (fp.wrapping_add(32) as i32),
                        (fp.wrapping_add(56) as i32),
                        fp.wrapping_add(0),
                    );
                    let _ = self.f_10007e50(
                        this,
                        0_i32,
                        a2,
                        (fp.wrapping_add(32) as i32),
                        (fp.wrapping_add(56) as i32),
                        (fp.wrapping_add(0) as i32),
                    );
                    let _ = self.f_10008dc0(
                        this,
                        0_i32,
                        a2,
                        (fp.wrapping_add(32) as i32),
                        (fp.wrapping_add(56) as i32),
                        (fp.wrapping_add(0) as i32),
                    );
                    let t1 = a4;
                    bb = match t1 {
                        55 => 2,
                        56 => 3,
                        57 => 4,
                        58 => 5,
                        59 => 6,
                        60 => 7,
                        62 => 8,
                        63 => 9,
                        _ => 10,
                    };
                }
                1 => {
                    let _ = self.f_10007d50(
                        (fp.wrapping_add(32) as i32),
                        (fp.wrapping_add(56) as i32),
                        fp.wrapping_add(0),
                        (fp.wrapping_add(16) as i32),
                    );
                    let _ = self.f_10007df0(fp.wrapping_add(16));
                    self.w32(
                        (v5.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1572_i32) as u32),
                        (a3.wrapping_add(5_i32) as u32),
                    );
                    self.w32(
                        (v5.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1672_i32) as u32),
                        1_u32,
                    );
                    self.w32(
                        (v5.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1772_i32) as u32),
                        1_u32,
                    );
                    self.w32(
                        (v5.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1872_i32) as u32),
                        ((self.r32(fp.wrapping_add(16).wrapping_add(4)) as i32) as u32),
                    );
                    {
                        let a0 = (v5
                            .wrapping_add(4_i32.wrapping_mul({
                                let t2 = (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32);
                                self.w32(
                                    (v5.wrapping_add(2072_i32) as u32),
                                    (t2.wrapping_add(1) as u32),
                                );
                                t2
                            }))
                            .wrapping_add(1972_i32) as u32);
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                    return 0_i32;
                }
                2 => {
                    v6 = 35_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 11;
                }
                3 => {
                    v7 = 150_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 13;
                }
                4 => {
                    v8 = 160_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 15;
                }
                5 => {
                    v7 = 140_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 13;
                }
                6 => {
                    v9 = cdiv_i32(
                        140_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32)),
                        100_i32,
                    );
                    bb = 18;
                }
                7 => {
                    v7 = 130_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 13;
                }
                8 => {
                    v6 = 45_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 11;
                }
                9 => {
                    v7 = 200_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 13;
                }
                10 => {
                    bb = 1;
                }
                11 => {
                    v8 = 4_i32.wrapping_mul(v6);
                    bb = 15;
                }
                12 => {
                    bb = 3;
                }
                13 => {
                    v9 = cdiv_i32(v7, 100_i32);
                    bb = 18;
                }
                14 => {
                    bb = 4;
                }
                15 => {
                    v9 = cdiv_i32(v8, 100_i32);
                    bb = 18;
                }
                16 => {
                    bb = 5;
                }
                17 => {
                    bb = 6;
                }
                18 => {
                    self.w32(fp.wrapping_add(36), (v9 as u32));
                    bb = 1;
                }
                19 => {
                    bb = 7;
                }
                20 => {
                    bb = 8;
                }
                21 => {
                    bb = 9;
                }
                22 => {
                    bb = 10;
                }
                23 => {
                    bb = 1;
                }
                24 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10009E90` (525 bytes).
    pub(crate) fn f_10009e90(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let fp = self.enter(96);
        let r = self.f_10009e90_body(fp, this, a2, a3, a4);
        self.leave(96);
        r
    }

    fn f_10009e90_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        self.w32(fp.wrapping_add(81), (a2 as u32));
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v5 = (self.r32(fp.wrapping_add(81)) as i32);
                    v6 = (self.r32(
                        ((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(28_i32)
                            as u32),
                    ) as i32);
                    bb = if (a4 == 64_i32) { 1 } else { 2 };
                }
                1 => {
                    let _ = self.f_10006d10(
                        this,
                        (self.r32(fp.wrapping_add(81)) as i32),
                        fp.wrapping_add(81),
                        fp.wrapping_add(0),
                    );
                    bb = if ((((self.r32(fp.wrapping_add(81)) as i32) as u8) as i32) == 1_i32) {
                        3
                    } else {
                        4
                    };
                }
                2 => {
                    bb = if (a4 == 65_i32) { 12 } else { 13 };
                }
                3 => {
                    v7 = 0_i32;
                    bb = 5;
                }
                4 => {
                    v7 = 1_i32;
                    bb = if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                        7
                    } else {
                        8
                    };
                }
                5 => {
                    let _ = self.f_100079b0(this, 0_i32, v5, v7, fp.wrapping_add(33));
                    let _ = self.f_100079b0(this, 1_i32, a3, 0_i32, fp.wrapping_add(57));
                    let _ = self.f_10007b30(
                        (fp.wrapping_add(33) as i32),
                        (fp.wrapping_add(57) as i32),
                        fp.wrapping_add(1),
                    );
                    let _ = self.f_10007e50(
                        this,
                        0_i32,
                        v5,
                        (fp.wrapping_add(33) as i32),
                        (fp.wrapping_add(57) as i32),
                        (fp.wrapping_add(1) as i32),
                    );
                    let _ = self.f_10008dc0(
                        this,
                        0_i32,
                        v5,
                        (fp.wrapping_add(33) as i32),
                        (fp.wrapping_add(57) as i32),
                        (fp.wrapping_add(1) as i32),
                    );
                    let _ = self.f_10008070(
                        this,
                        0_i32,
                        v5,
                        v7,
                        (fp.wrapping_add(33) as i32),
                        (fp.wrapping_add(57) as i32),
                        (fp.wrapping_add(1) as i32),
                    );
                    let t1 = a4;
                    bb = match t1 {
                        64 => 29,
                        65 => 30,
                        66 => 31,
                        _ => 28,
                    };
                }
                6 => {
                    bb = 4;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    bb = 10;
                }
                9 => {
                    bb = 8;
                }
                10 => {
                    v7 = a4;
                    bb = 5;
                }
                11 => {
                    bb = 2;
                }
                12 => {
                    let _ = self.f_10006ec0(
                        this,
                        (self.r32(fp.wrapping_add(81)) as i32),
                        fp.wrapping_add(81),
                        fp.wrapping_add(0),
                    );
                    bb = if ((((self.r32(fp.wrapping_add(81)) as i32) as u8) as i32) == 1_i32) {
                        14
                    } else {
                        15
                    };
                }
                13 => {
                    bb = if (a4 != 66_i32) { 22 } else { 23 };
                }
                14 => {
                    v7 = 0_i32;
                    bb = 5;
                }
                15 => {
                    bb = 17;
                }
                16 => {
                    bb = 15;
                }
                17 => {
                    bb = if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                        18
                    } else {
                        19
                    };
                }
                18 => {
                    v7 = 1_i32;
                    bb = 5;
                }
                19 => {
                    bb = 10;
                }
                20 => {
                    bb = 19;
                }
                21 => {
                    bb = 13;
                }
                22 => {
                    bb = 10;
                }
                23 => {
                    let _ = self.f_10006d10(
                        this,
                        (self.r32(fp.wrapping_add(81)) as i32),
                        fp.wrapping_add(81),
                        fp.wrapping_add(0),
                    );
                    bb = if ((((self.r32(fp.wrapping_add(81)) as i32) as u8) as i32) != 1_i32) {
                        25
                    } else {
                        26
                    };
                }
                24 => {
                    bb = 23;
                }
                25 => {
                    bb = 17;
                }
                26 => {
                    v7 = 0_i32;
                    bb = 5;
                }
                27 => {
                    bb = 26;
                }
                28 => {
                    let _ = self.f_10007d50(
                        (fp.wrapping_add(33) as i32),
                        (fp.wrapping_add(57) as i32),
                        fp.wrapping_add(1),
                        (fp.wrapping_add(17) as i32),
                    );
                    let _ = self.f_10007df0(fp.wrapping_add(17));
                    self.w32(
                        (v6.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v6.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1572_i32) as u32),
                        (a3.wrapping_add(5_i32) as u32),
                    );
                    self.w32(
                        (v6.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v6.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1672_i32) as u32),
                        1_u32,
                    );
                    self.w32(
                        (v6.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v6.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1772_i32) as u32),
                        1_u32,
                    );
                    self.w32(
                        (v6.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v6.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1872_i32) as u32),
                        ((self.r32(fp.wrapping_add(17).wrapping_add(4)) as i32) as u32),
                    );
                    {
                        let a0 = (v6
                            .wrapping_add(4_i32.wrapping_mul({
                                let t2 = (self.r32((v6.wrapping_add(2072_i32) as u32)) as i32);
                                self.w32(
                                    (v6.wrapping_add(2072_i32) as u32),
                                    (t2.wrapping_add(1) as u32),
                                );
                                t2
                            }))
                            .wrapping_add(1972_i32) as u32);
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                    return 0_i32;
                }
                29 => {
                    bb = 32;
                }
                30 => {
                    v8 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(fp.wrapping_add(37)) as i32)),
                        100_i32,
                    );
                    bb = 34;
                }
                31 => {
                    bb = 32;
                }
                32 => {
                    v8 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(fp.wrapping_add(37)) as i32)),
                        100_i32,
                    );
                    bb = 34;
                }
                33 => {
                    bb = 30;
                }
                34 => {
                    self.w32(fp.wrapping_add(37), (v8 as u32));
                    bb = 28;
                }
                35 => {
                    bb = 31;
                }
                36 => {
                    bb = 28;
                }
                37 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }
}
