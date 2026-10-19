//! Functions 0x10004900..=0x100071E0, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_10004900` (85 bytes).
    pub(crate) fn f_10004900(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: i32 = 0;
        self.w32(this.wrapping_add(12), 0x10028648_u32);
        v1 = 0x10028690_u32;
        v2 = 1800_i32;
        'l1: loop {
            self.w32(
                v1.wrapping_sub(40),
                ((self.r32(v1) as i32)
                    .wrapping_add((self.r32(v1.wrapping_add(4)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(4)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(8)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(12)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(16)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(20)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(24)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(28)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(32)) as i32))
                    .wrapping_add((self.r32(v1.wrapping_sub(36)) as i32)) as u32),
            );
            v1 = v1.wrapping_add(140);
            v2 = v2.wrapping_sub(1);
            if !(v2 != 0) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10004960` (100 bytes).
    pub(crate) fn f_10004960(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut result: i32 = 0;
        v1 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32));
        let _ = self.memset(v1.wrapping_add(1600), 0_i32 as u8, 1400_u32);
        let _ = self.memset(v1.wrapping_add(3000), 0_i32 as u8, 1400_u32);
        let _ = self.memset(v1.wrapping_add(4400), 255_i32 as u8, 1400_u32);
        result = 0_i32;
        let _ = self.memset(v1.wrapping_add(5800), 0_i32 as u8, 1400_u32);
        self.w32(v1.wrapping_add(7200), 0_u32);
        self.w32(v1.wrapping_add(7204), 0_u32);
        let _ = self.memset(v1.wrapping_add(7208), 0_i32 as u8, 140_u32);
        self.w32(v1.wrapping_add(7348), 0_u32);
        return result;
    }

    /// `sub_100049D0` (52 bytes).
    pub(crate) fn f_100049d0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut result: i32 = 0;
        result = 0_i32;
        let _ = self.memcpy(
            ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32)
                .wrapping_add(7208_i32) as u32),
            0x10028648_u32.wrapping_add((35_i32.wrapping_mul(a2) as u32).wrapping_mul(4)),
            140_u32,
        );
        return result;
    }

    /// `sub_10004A10` (144 bytes).
    pub(crate) fn f_10004a10(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = 0_i32;
        v4 = (self.r32((v2.wrapping_add(16_i32) as u32)) as i32);
        v5 = (v4.wrapping_add(1600_i32) as u32);
        'l1: loop {
            if (!((self.r32(v5) as i32) != 0)) {
                break 'l1;
            }
            v5 = v5.wrapping_add(4);
            v3 = v3.wrapping_add(1);
            if !(v3 < 350_i32) {
                break 'l1;
            }
        }
        if (v3 == 350_i32) {
            let _ = self.message_box(
                self.r32(self.r32((v2.wrapping_add(4_i32) as u32))),
                0x10065EB8_u32,
                0x10065ED8_u32,
                48_u32,
            );
            return 1_i32;
        } else {
            self.w32(
                (v4.wrapping_add(4_i32.wrapping_mul(v3))
                    .wrapping_add(1600_i32) as u32),
                (a2 as u32),
            );
            self.w32(
                (v4.wrapping_add(4_i32.wrapping_mul(v3))
                    .wrapping_add(3000_i32) as u32),
                0_u32,
            );
            self.w32(
                (v4.wrapping_add(4_i32.wrapping_mul(v3))
                    .wrapping_add(4400_i32) as u32),
                ((1_i32).wrapping_neg() as u32),
            );
            self.w32(
                (v4.wrapping_add(7204_i32) as u32),
                ((self.r32((v4.wrapping_add(7204_i32) as u32)) as i32).wrapping_add(
                    (self.r32(
                        0x10028664_u32
                            .wrapping_add((35_i32.wrapping_mul(a2) as u32).wrapping_mul(4)),
                    ) as i32),
                ) as u32),
            );
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10004AA0` (165 bytes).
    pub(crate) fn f_10004aa0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        v3 = (self.r32(self.r32(this)) as i32);
        v4 = (self.r32((v3.wrapping_add(16_i32) as u32)) as i32);
        if ((a2 as u32) > 349_u32) {
            let _ = self.message_box(
                self.r32(self.r32((v3.wrapping_add(4_i32) as u32))),
                0x10065EE8_u32,
                0x10065F04_u32,
                48_u32,
            );
            return 1_i32;
        } else {
            v5 = (v4
                .wrapping_add(4_i32.wrapping_mul(a2))
                .wrapping_add(1600_i32) as u32);
            v6 = (v4
                .wrapping_add(4_i32.wrapping_mul(a2))
                .wrapping_add(3000_i32) as u32);
            v7 = (v4
                .wrapping_add(4_i32.wrapping_mul(a2))
                .wrapping_add(4400_i32) as u32);
            if (a2 < 349_i32) {
                v8 = 349_i32.wrapping_sub(a2);
                'l1: loop {
                    v6 = v6.wrapping_add(4);
                    self.w32(v5, ((self.r32(v5.wrapping_add(4)) as i32) as u32));
                    self.w32(v6.wrapping_sub(4), ((self.r32(v6) as i32) as u32));
                    self.w32(v7, ((self.r32(v7.wrapping_add(4)) as i32) as u32));
                    v5 = v5.wrapping_add(4);
                    v7 = v7.wrapping_add(4);
                    v8 = v8.wrapping_sub(1);
                    if !(v8 != 0) {
                        break 'l1;
                    }
                }
            }
            self.w32(v5, 0_u32);
            self.w32(v6, 0_u32);
            self.w32(v7, ((1_i32).wrapping_neg() as u32));
            let _ = self.f_100055a0(this);
            let _ = self.f_100055f0(this);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10004B50` (2163 bytes).
    pub(crate) fn f_10004b50(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let fp = self.enter(1424);
        let r = self.f_10004b50_body(fp, this, a2, a3);
        self.leave(1424);
        r
    }

    fn f_10004b50_body(&mut self, fp: u32, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: u32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        let mut v29: i32 = 0;
        let mut v30: bool = false;
        let mut v31: i32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v39: u32 = 0;
        let mut v40: i32 = 0;
        let mut v41: i32 = 0;
        let mut v42: i32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: i32 = 0;
        let mut v48: i32 = 0;
        let mut v49: i32 = 0;
        let mut v50: i32 = 0;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: u32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: i32 = 0;
        let mut v58: i32 = 0;
        let mut v59: i32 = 0;
        let mut v60: u32 = 0;
        let mut v61: i32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: i32 = 0;
        let mut v65: i32 = 0;
        let mut v66: i32 = 0;
        let mut v67: u32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: i32 = 0;
        let mut v71: i32 = 0;
        let mut v72: i32 = 0;
        let mut v73: i32 = 0;
        let mut v74: u32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut v77: i32 = 0;
        let mut v78: i32 = 0;
        let mut v79: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32))
                        as i32);
                    v4 = 0_i32;
                    v5 = 0_i32;
                    v6 = v3.wrapping_add(1600_i32);
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(20));
                    self.w32(fp.wrapping_add(8), (v3.wrapping_add(1600_i32) as u32));
                    bb = 1;
                }
                1 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        2
                    } else {
                        3
                    };
                }
                2 => {
                    v7 = (self.r32(
                        0x10028648_u32.wrapping_add(
                            (35_i32.wrapping_mul((self.r32(self.r32(fp.wrapping_add(8))) as i32))
                                as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32);
                    bb = if (a2 != 0) { 4 } else { 5 };
                }
                3 => {
                    bb = if (a3 != 0) { 36 } else { 38 };
                }
                4 => {
                    let t1 = a2;
                    bb = match t1 {
                        1_i32 => 7,
                        4_i32 => 8,
                        6_i32 => 9,
                        13_i32 => 10,
                        _ => 11,
                    };
                }
                5 => {
                    bb = 14;
                }
                6 => {
                    v4 = v4.wrapping_add(1);
                    {
                        let a0 = {
                            let t2 = self.r32(fp.wrapping_add(0));
                            self.w32(fp.wrapping_add(0), t2.wrapping_add(4));
                            t2
                        };
                        let a1 = (v5 as u32);
                        self.w32(a0, a1)
                    };
                    bb = 5;
                }
                7 => {
                    bb = if ((v7 != 2_i32) && (v7 != 3_i32)) {
                        12
                    } else {
                        13
                    };
                }
                8 => {
                    bb = if (((((((v7 != 5_i32) && (v7 != 7_i32)) && (v7 != 8_i32))
                        && (v7 != 9_i32))
                        && (v7 != 10_i32))
                        && (v7 != 11_i32))
                        && (v7 != 12_i32))
                    {
                        17
                    } else {
                        18
                    };
                }
                9 => {
                    bb = if (((((v7 != 7_i32) && (v7 != 8_i32)) && (v7 != 9_i32))
                        && (v7 != 10_i32))
                        && (v7 != 11_i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                10 => {
                    bb = if ((v7 != 5_i32) && (v7 != 7_i32)) {
                        25
                    } else {
                        26
                    };
                }
                11 => {
                    bb = if ((a2 != 14_i32) && (a2 != v7)) {
                        29
                    } else {
                        30
                    };
                }
                12 => {
                    bb = 14;
                }
                13 => {
                    bb = 6;
                }
                14 => {
                    v5 = v5.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = if (v5 >= 350_i32) { 33 } else { 34 };
                }
                15 => {
                    bb = 13;
                }
                16 => {
                    bb = 8;
                }
                17 => {
                    bb = 14;
                }
                18 => {
                    bb = 6;
                }
                19 => {
                    bb = 18;
                }
                20 => {
                    bb = 9;
                }
                21 => {
                    bb = 14;
                }
                22 => {
                    bb = 6;
                }
                23 => {
                    bb = 22;
                }
                24 => {
                    bb = 10;
                }
                25 => {
                    bb = 14;
                }
                26 => {
                    bb = 6;
                }
                27 => {
                    bb = 26;
                }
                28 => {
                    bb = 11;
                }
                29 => {
                    bb = 14;
                }
                30 => {
                    bb = 6;
                }
                31 => {
                    bb = 30;
                }
                32 => {
                    bb = 6;
                }
                33 => {
                    bb = 3;
                }
                34 => {
                    bb = 1;
                }
                35 => {
                    bb = 34;
                }
                36 => {
                    let t3 = a3;
                    bb = match t3 {
                        1_i32 => 40,
                        2_i32 => 41,
                        3_i32 => 42,
                        4_i32 => 43,
                        5_i32 => 44,
                        6_i32 => 45,
                        7_i32 => 46,
                        8_i32 => 47,
                        9_i32 => 48,
                        _ => 39,
                    };
                }
                37 => {
                    let _ = self.f_100055f0(this);
                    return 0_i32;
                }
                38 => {
                    v8 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 139;
                }
                39 => {
                    bb = 37;
                }
                40 => {
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 49;
                }
                41 => {
                    v23 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 59;
                }
                42 => {
                    v31 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 69;
                }
                43 => {
                    v38 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 79;
                }
                44 => {
                    v45 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 89;
                }
                45 => {
                    v52 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 99;
                }
                46 => {
                    v59 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v4.wrapping_sub(1_i32) as u32));
                    bb = 109;
                }
                47 => {
                    v66 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v66 as u32));
                    bb = 119;
                }
                48 => {
                    v73 = v4.wrapping_sub(1_i32);
                    self.w32(fp.wrapping_add(0), (v73 as u32));
                    bb = 129;
                }
                49 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        50
                    } else {
                        52
                    };
                }
                50 => {
                    v16 = fp.wrapping_add(24);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    bb = 53;
                }
                51 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 49;
                }
                52 => {
                    bb = 39;
                }
                53 => {
                    v17 = 4_i32.wrapping_mul((self.r32(v16) as i32));
                    v18 = 4_i32.wrapping_mul((self.r32(v16.wrapping_sub(4)) as i32));
                    v19 = (self.r32((v17.wrapping_add(v6) as u32)) as i32);
                    v20 = (self.r32((v18.wrapping_add(v6) as u32)) as i32);
                    bb = if (v20 <= v19) { 56 } else { 57 };
                }
                54 => {
                    bb = if ((self.r32(fp.wrapping_add(8)) as i32) != 0) {
                        53
                    } else {
                        55
                    };
                }
                55 => {
                    bb = 51;
                }
                56 => {
                    self.w32((v18.wrapping_add(v6) as u32), (v19 as u32));
                    self.w32((v17.wrapping_add(v6) as u32), (v20 as u32));
                    v21 = (self.r32((v18.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v18.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v17.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v17.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v21 as u32),
                    );
                    v22 = (self.r32((v18.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v18.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v17.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v17.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v22 as u32),
                    );
                    bb = 57;
                }
                57 => {
                    v16 = v16.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 54;
                }
                58 => {
                    bb = 41;
                }
                59 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        60
                    } else {
                        62
                    };
                }
                60 => {
                    v24 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(4), (v23 as u32));
                    self.w32(fp.wrapping_add(12), fp.wrapping_add(24));
                    bb = 63;
                }
                61 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 59;
                }
                62 => {
                    bb = 39;
                }
                63 => {
                    v25 = 4_i32.wrapping_mul((self.r32(v24.wrapping_sub(4)) as i32));
                    v26 = 4_i32.wrapping_mul((self.r32(v24) as i32));
                    v27 = (self.r32((v26.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32((v25.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    bb = if ((self.r32(
                        0x10028660_u32
                            .wrapping_add((35_i32.wrapping_mul(v27) as u32).wrapping_mul(4)),
                    ) as i32)
                        <= (self.r32(
                            0x10028660_u32.wrapping_add(
                                (35_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32)) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    bb = if (!v30) { 63 } else { 65 };
                }
                65 => {
                    v23 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1_i32);
                    bb = 61;
                }
                66 => {
                    self.w32((v25.wrapping_add(v6) as u32), (v27 as u32));
                    self.w32(
                        (v26.wrapping_add(v6) as u32),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    v28 = (self.r32((v25.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v25.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v26.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v26.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v28 as u32),
                    );
                    v29 = (self.r32((v25.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v25.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v26.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v26.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v29 as u32),
                    );
                    bb = 67;
                }
                67 => {
                    v24 = self.r32(fp.wrapping_add(12)).wrapping_add(4);
                    v30 = ((self.r32(fp.wrapping_add(4)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(12),
                        self.r32(fp.wrapping_add(12)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 64;
                }
                68 => {
                    bb = 42;
                }
                69 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        70
                    } else {
                        72
                    };
                }
                70 => {
                    v32 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(12), (v31 as u32));
                    self.w32(fp.wrapping_add(4), fp.wrapping_add(24));
                    bb = 73;
                }
                71 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 69;
                }
                72 => {
                    bb = 39;
                }
                73 => {
                    v33 = 4_i32.wrapping_mul((self.r32(v32.wrapping_sub(4)) as i32));
                    v34 = 4_i32.wrapping_mul((self.r32(v32) as i32));
                    v35 = (self.r32((v34.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32((v33.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    bb = if ((self.r32(
                        0x10028660_u32.wrapping_add(
                            (35_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32)) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        <= (self.r32(
                            0x10028660_u32
                                .wrapping_add((35_i32.wrapping_mul(v35) as u32).wrapping_mul(4)),
                        ) as i32))
                    {
                        76
                    } else {
                        77
                    };
                }
                74 => {
                    bb = if (!v30) { 73 } else { 75 };
                }
                75 => {
                    v31 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1_i32);
                    bb = 71;
                }
                76 => {
                    self.w32((v33.wrapping_add(v6) as u32), (v35 as u32));
                    self.w32(
                        (v34.wrapping_add(v6) as u32),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    v36 = (self.r32((v33.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v33.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v34.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v34.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v36 as u32),
                    );
                    v37 = (self.r32((v33.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v33.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v34.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v34.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v37 as u32),
                    );
                    bb = 77;
                }
                77 => {
                    v32 = self.r32(fp.wrapping_add(4)).wrapping_add(4);
                    v30 = ((self.r32(fp.wrapping_add(12)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 74;
                }
                78 => {
                    bb = 43;
                }
                79 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        80
                    } else {
                        82
                    };
                }
                80 => {
                    v39 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(12), (v38 as u32));
                    self.w32(fp.wrapping_add(4), fp.wrapping_add(24));
                    bb = 83;
                }
                81 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 79;
                }
                82 => {
                    bb = 39;
                }
                83 => {
                    v40 = 4_i32.wrapping_mul((self.r32(v39.wrapping_sub(4)) as i32));
                    v41 = 4_i32.wrapping_mul((self.r32(v39) as i32));
                    v42 = (self.r32((v41.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32((v40.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    bb = if ((self.r32(
                        0x10028664_u32
                            .wrapping_add((35_i32.wrapping_mul(v42) as u32).wrapping_mul(4)),
                    ) as i32)
                        <= (self.r32(
                            0x10028664_u32.wrapping_add(
                                (35_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32)) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32))
                    {
                        86
                    } else {
                        87
                    };
                }
                84 => {
                    bb = if (!v30) { 83 } else { 85 };
                }
                85 => {
                    v38 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1_i32);
                    bb = 81;
                }
                86 => {
                    self.w32((v40.wrapping_add(v6) as u32), (v42 as u32));
                    self.w32(
                        (v41.wrapping_add(v6) as u32),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    v43 = (self.r32((v40.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v40.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v41.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v41.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v43 as u32),
                    );
                    v44 = (self.r32((v40.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v40.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v41.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v41.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v44 as u32),
                    );
                    bb = 87;
                }
                87 => {
                    v39 = self.r32(fp.wrapping_add(4)).wrapping_add(4);
                    v30 = ((self.r32(fp.wrapping_add(12)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 84;
                }
                88 => {
                    bb = 44;
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        90
                    } else {
                        92
                    };
                }
                90 => {
                    v46 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(12), (v45 as u32));
                    self.w32(fp.wrapping_add(4), fp.wrapping_add(24));
                    bb = 93;
                }
                91 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                92 => {
                    bb = 39;
                }
                93 => {
                    v47 = 4_i32.wrapping_mul((self.r32(v46.wrapping_sub(4)) as i32));
                    v48 = 4_i32.wrapping_mul((self.r32(v46) as i32));
                    v49 = (self.r32((v48.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32((v47.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    bb = if ((self.r32(
                        0x10028664_u32.wrapping_add(
                            (35_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32)) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        <= (self.r32(
                            0x10028664_u32
                                .wrapping_add((35_i32.wrapping_mul(v49) as u32).wrapping_mul(4)),
                        ) as i32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if (!v30) { 93 } else { 95 };
                }
                95 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1_i32);
                    bb = 91;
                }
                96 => {
                    self.w32((v47.wrapping_add(v6) as u32), (v49 as u32));
                    self.w32(
                        (v48.wrapping_add(v6) as u32),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    v50 = (self.r32((v47.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v47.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v48.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v48.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v50 as u32),
                    );
                    v51 = (self.r32((v47.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v47.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v48.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v48.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v51 as u32),
                    );
                    bb = 97;
                }
                97 => {
                    v46 = self.r32(fp.wrapping_add(4)).wrapping_add(4);
                    v30 = ((self.r32(fp.wrapping_add(12)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 94;
                }
                98 => {
                    bb = 45;
                }
                99 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        100
                    } else {
                        102
                    };
                }
                100 => {
                    v53 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(12), (v52 as u32));
                    self.w32(fp.wrapping_add(4), fp.wrapping_add(24));
                    bb = 103;
                }
                101 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 99;
                }
                102 => {
                    bb = 39;
                }
                103 => {
                    v54 = 4_i32.wrapping_mul((self.r32(v53.wrapping_sub(4)) as i32));
                    v55 = 4_i32.wrapping_mul((self.r32(v53) as i32));
                    v56 = (self.r32((v55.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32((v54.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    bb = if ((self.r32(
                        0x10028668_u32
                            .wrapping_add((35_i32.wrapping_mul(v56) as u32).wrapping_mul(4)),
                    ) as i32)
                        <= (self.r32(
                            0x10028668_u32.wrapping_add(
                                (35_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32)) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32))
                    {
                        106
                    } else {
                        107
                    };
                }
                104 => {
                    bb = if (!v30) { 103 } else { 105 };
                }
                105 => {
                    v52 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1_i32);
                    bb = 101;
                }
                106 => {
                    self.w32((v54.wrapping_add(v6) as u32), (v56 as u32));
                    self.w32(
                        (v55.wrapping_add(v6) as u32),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    v57 = (self.r32((v54.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v54.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v55.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v55.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v57 as u32),
                    );
                    v58 = (self.r32((v54.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v54.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v55.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v55.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v58 as u32),
                    );
                    bb = 107;
                }
                107 => {
                    v53 = self.r32(fp.wrapping_add(4)).wrapping_add(4);
                    v30 = ((self.r32(fp.wrapping_add(12)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 104;
                }
                108 => {
                    bb = 46;
                }
                109 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        110
                    } else {
                        112
                    };
                }
                110 => {
                    v60 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(12), (v59 as u32));
                    self.w32(fp.wrapping_add(4), fp.wrapping_add(24));
                    bb = 113;
                }
                111 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 109;
                }
                112 => {
                    bb = 39;
                }
                113 => {
                    v61 = 4_i32.wrapping_mul((self.r32(v60.wrapping_sub(4)) as i32));
                    v62 = 4_i32.wrapping_mul((self.r32(v60) as i32));
                    v63 = (self.r32((v62.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32((v61.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    bb = if ((self.r32(
                        0x10028668_u32.wrapping_add(
                            (35_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32)) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        <= (self.r32(
                            0x10028668_u32
                                .wrapping_add((35_i32.wrapping_mul(v63) as u32).wrapping_mul(4)),
                        ) as i32))
                    {
                        116
                    } else {
                        117
                    };
                }
                114 => {
                    bb = if (!v30) { 113 } else { 115 };
                }
                115 => {
                    v59 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1_i32);
                    bb = 111;
                }
                116 => {
                    self.w32((v61.wrapping_add(v6) as u32), (v63 as u32));
                    self.w32(
                        (v62.wrapping_add(v6) as u32),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    v64 = (self.r32((v61.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v61.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v62.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v62.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v64 as u32),
                    );
                    v65 = (self.r32((v61.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v61.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v62.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v62.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v65 as u32),
                    );
                    bb = 117;
                }
                117 => {
                    v60 = self.r32(fp.wrapping_add(4)).wrapping_add(4);
                    v30 = ((self.r32(fp.wrapping_add(12)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 114;
                }
                118 => {
                    bb = 47;
                }
                119 => {
                    bb = if (v66 > 0_i32) { 120 } else { 122 };
                }
                120 => {
                    v67 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(4), (v66 as u32));
                    bb = 123;
                }
                121 => {
                    self.w32(fp.wrapping_add(0), (v66 as u32));
                    bb = 119;
                }
                122 => {
                    bb = 39;
                }
                123 => {
                    v68 = 4_i32.wrapping_mul((self.r32(v67) as i32));
                    v69 = 4_i32.wrapping_mul((self.r32(v67.wrapping_sub(4)) as i32));
                    bb = if ((self.r32((v68.wrapping_add(v3).wrapping_add(3000_i32) as u32))
                        as i32)
                        <= (self.r32((v69.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32))
                    {
                        126
                    } else {
                        127
                    };
                }
                124 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) != 0) {
                        123
                    } else {
                        125
                    };
                }
                125 => {
                    v66 = v66.wrapping_sub(1);
                    bb = 121;
                }
                126 => {
                    v70 = (self.r32((v69.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        (v69.wrapping_add(v6) as u32),
                        ((self.r32((v68.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    self.w32((v68.wrapping_add(v6) as u32), (v70 as u32));
                    v71 = (self.r32((v69.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v69.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v68.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v68.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v71 as u32),
                    );
                    v72 = (self.r32((v69.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v69.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v68.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v68.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v72 as u32),
                    );
                    v66 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = 127;
                }
                127 => {
                    v67 = v67.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 124;
                }
                128 => {
                    bb = 48;
                }
                129 => {
                    bb = if (v73 > 0_i32) { 130 } else { 132 };
                }
                130 => {
                    v74 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    bb = 133;
                }
                131 => {
                    self.w32(fp.wrapping_add(0), (v73 as u32));
                    bb = 129;
                }
                132 => {
                    bb = 39;
                }
                133 => {
                    v75 = 4_i32.wrapping_mul((self.r32(v74.wrapping_sub(4)) as i32));
                    v76 = 4_i32.wrapping_mul((self.r32(v74) as i32));
                    bb = if ((self.r32((v75.wrapping_add(v3).wrapping_add(3000_i32) as u32))
                        as i32)
                        <= (self.r32((v76.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32))
                    {
                        136
                    } else {
                        137
                    };
                }
                134 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) != 0) {
                        133
                    } else {
                        135
                    };
                }
                135 => {
                    v73 = v73.wrapping_sub(1);
                    bb = 131;
                }
                136 => {
                    v77 = (self.r32((v75.wrapping_add(v6) as u32)) as i32);
                    self.w32(
                        (v75.wrapping_add(v6) as u32),
                        ((self.r32((v76.wrapping_add(v6) as u32)) as i32) as u32),
                    );
                    self.w32((v76.wrapping_add(v6) as u32), (v77 as u32));
                    v78 = (self.r32((v75.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v75.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v76.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v76.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v78 as u32),
                    );
                    v79 = (self.r32((v75.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v75.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v76.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v76.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v79 as u32),
                    );
                    v73 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = 137;
                }
                137 => {
                    v74 = v74.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 134;
                }
                138 => {
                    bb = 39;
                }
                139 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        140
                    } else {
                        142
                    };
                }
                140 => {
                    v9 = fp.wrapping_add(24);
                    self.w32(fp.wrapping_add(8), (v8 as u32));
                    bb = 143;
                }
                141 => {
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 139;
                }
                142 => {
                    bb = 37;
                }
                143 => {
                    v10 = 4_i32.wrapping_mul((self.r32(v9) as i32));
                    v11 = 4_i32.wrapping_mul((self.r32(v9.wrapping_sub(4)) as i32));
                    v12 = (self.r32((v10.wrapping_add(v6) as u32)) as i32);
                    v13 = (self.r32((v11.wrapping_add(v6) as u32)) as i32);
                    bb = if (v12 <= v13) { 146 } else { 147 };
                }
                144 => {
                    bb = if ((self.r32(fp.wrapping_add(8)) as i32) != 0) {
                        143
                    } else {
                        145
                    };
                }
                145 => {
                    v8 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1_i32);
                    bb = 141;
                }
                146 => {
                    self.w32((v11.wrapping_add(v6) as u32), (v12 as u32));
                    self.w32((v10.wrapping_add(v6) as u32), (v13 as u32));
                    v14 = (self.r32((v11.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32);
                    self.w32(
                        (v11.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        ((self.r32((v10.wrapping_add(v3).wrapping_add(3000_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v10.wrapping_add(v3).wrapping_add(3000_i32) as u32),
                        (v14 as u32),
                    );
                    v15 = (self.r32((v11.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32);
                    self.w32(
                        (v11.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        ((self.r32((v10.wrapping_add(v3).wrapping_add(4400_i32) as u32)) as i32)
                            as u32),
                    );
                    self.w32(
                        (v10.wrapping_add(v3).wrapping_add(4400_i32) as u32),
                        (v15 as u32),
                    );
                    bb = 147;
                }
                147 => {
                    v9 = v9.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 144;
                }
                148 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100053D0` (451 bytes).
    pub(crate) fn f_100053d0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let fp = self.enter(1424);
        let r = self.f_100053d0_body(fp, this, a2, a3, a4);
        self.leave(1424);
        r
    }

    fn f_100053d0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut i: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut result: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v4 = 0_i32;
                    v5 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32));
                    self.w32(fp.wrapping_add(4), fp.wrapping_add(16));
                    self.w32(fp.wrapping_add(8), v5);
                    self.w32(fp.wrapping_add(0), v5.wrapping_add(1600));
                    v6 = 0_i32;
                    bb = 1;
                }
                1 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0) {
                        2
                    } else {
                        3
                    };
                }
                2 => {
                    v7 = (self.r32(
                        0x10028648_u32.wrapping_add(
                            (35_i32.wrapping_mul((self.r32(self.r32(fp.wrapping_add(0))) as i32))
                                as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32);
                    bb = if (a2 != 0) { 4 } else { 5 };
                }
                3 => {
                    self.w32(fp.wrapping_add(4), (v4 as u32));
                    bb = if (a2 == 13_i32) { 37 } else { 38 };
                }
                4 => {
                    let t1 = a2;
                    bb = match t1 {
                        1_i32 => 7,
                        4_i32 => 8,
                        6_i32 => 9,
                        13_i32 => 10,
                        _ => 11,
                    };
                }
                5 => {
                    v6 = v6.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = if (v6 >= 350_i32) { 34 } else { 35 };
                }
                6 => {
                    bb = 5;
                }
                7 => {
                    bb = if ((v7 == 2_i32) || (v7 == 3_i32)) {
                        12
                    } else {
                        13
                    };
                }
                8 => {
                    bb = if (((((((v7 == 5_i32) || (v7 == 7_i32)) || (v7 == 8_i32))
                        || (v7 == 9_i32))
                        || (v7 == 10_i32))
                        || (v7 == 11_i32))
                        || (v7 == 12_i32))
                    {
                        17
                    } else {
                        18
                    };
                }
                9 => {
                    bb = if (((((v7 == 7_i32) || (v7 == 8_i32)) || (v7 == 9_i32))
                        || (v7 == 10_i32))
                        || (v7 == 11_i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                10 => {
                    bb = if ((v7 == 5_i32) || (v7 == 7_i32)) {
                        25
                    } else {
                        26
                    };
                }
                11 => {
                    bb = if ((a2 == 14_i32) || (a2 == v7)) {
                        29
                    } else {
                        30
                    };
                }
                12 => {
                    bb = 14;
                }
                13 => {
                    bb = 6;
                }
                14 => {
                    bb = if ((((self.r32(
                        0x10028658_u32.wrapping_add(
                            (35_i32.wrapping_mul((self.r32(self.r32(fp.wrapping_add(0))) as i32))
                                as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        & a3)
                        != 0_i32)
                        && (((self.r32(
                            0x1002865C_u32.wrapping_add(
                                (35_i32
                                    .wrapping_mul((self.r32(self.r32(fp.wrapping_add(0))) as i32))
                                    as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            & a4)
                            != 0_i32))
                    {
                        31
                    } else {
                        32
                    };
                }
                15 => {
                    bb = 13;
                }
                16 => {
                    bb = 8;
                }
                17 => {
                    bb = 14;
                }
                18 => {
                    bb = 6;
                }
                19 => {
                    bb = 18;
                }
                20 => {
                    bb = 9;
                }
                21 => {
                    bb = 14;
                }
                22 => {
                    bb = 6;
                }
                23 => {
                    bb = 22;
                }
                24 => {
                    bb = 10;
                }
                25 => {
                    bb = 14;
                }
                26 => {
                    bb = 6;
                }
                27 => {
                    bb = 26;
                }
                28 => {
                    bb = 11;
                }
                29 => {
                    bb = 14;
                }
                30 => {
                    bb = 6;
                }
                31 => {
                    v4 = v4.wrapping_add(1);
                    {
                        let a0 = {
                            let t2 = self.r32(fp.wrapping_add(4));
                            self.w32(fp.wrapping_add(4), t2.wrapping_add(4));
                            t2
                        };
                        let a1 = (v6 as u32);
                        self.w32(a0, a1)
                    };
                    bb = 32;
                }
                32 => {
                    bb = 30;
                }
                33 => {
                    bb = 6;
                }
                34 => {
                    bb = 3;
                }
                35 => {
                    bb = 1;
                }
                36 => {
                    bb = 35;
                }
                37 => {
                    i = v4.wrapping_sub(1_i32);
                    bb = 39;
                }
                38 => {
                    result = 0_i32;
                    let _ = self.memcpy(v5.wrapping_add(5800), fp.wrapping_add(16), 1400_u32);
                    self.w32(
                        v5.wrapping_add(7200),
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32),
                    );
                    return result;
                }
                39 => {
                    bb = if (i > 0_i32) { 40 } else { 42 };
                }
                40 => {
                    v9 = fp.wrapping_add(16).wrapping_add(4);
                    self.w32(fp.wrapping_add(0), (i as u32));
                    bb = 43;
                }
                41 => {
                    i = i.wrapping_sub(1);
                    bb = 39;
                }
                42 => {
                    bb = 38;
                }
                43 => {
                    v10 = (self.r32(v9.wrapping_sub(4)) as i32);
                    bb = if ((self.r32(
                        0x10028648_u32.wrapping_add(
                            (35_i32.wrapping_mul(
                                (self.r32(
                                    v5.wrapping_add(
                                        ((self.r32(v9) as i32).wrapping_add(400_i32) as u32)
                                            .wrapping_mul(4),
                                    ),
                                ) as i32),
                            ) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        < (self.r32(
                            0x10028648_u32.wrapping_add(
                                (35_i32.wrapping_mul(
                                    (self.r32(v5.wrapping_add(
                                        (v10.wrapping_add(400_i32) as u32).wrapping_mul(4),
                                    )) as i32),
                                ) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32))
                    {
                        46
                    } else {
                        47
                    };
                }
                44 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        43
                    } else {
                        45
                    };
                }
                45 => {
                    bb = 41;
                }
                46 => {
                    self.w32(v9.wrapping_sub(4), ((self.r32(v9) as i32) as u32));
                    self.w32(v9, (v10 as u32));
                    bb = 47;
                }
                47 => {
                    v5 = self.r32(fp.wrapping_add(8));
                    v9 = v9.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 44;
                }
                48 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100055A0` (71 bytes).
    pub(crate) fn f_100055a0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        v1 = 0_i32;
        v2 = 0_i32;
        v3 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        v4 = (v3.wrapping_add(1600_i32) as u32);
        'l1: loop {
            v5 = (self.r32(v4) as i32);
            if (!((self.r32(v4) as i32) != 0)) {
                break 'l1;
            }
            v4 = v4.wrapping_add(4);
            v1 = v1.wrapping_add(
                (self.r32(
                    0x10028664_u32.wrapping_add((35_i32.wrapping_mul(v5) as u32).wrapping_mul(4)),
                ) as i32),
            );
            v2 = v2.wrapping_add(1);
            if !(v2 < 350_i32) {
                break 'l1;
            }
        }
        self.w32((v3.wrapping_add(7204_i32) as u32), (v1 as u32));
        return 0_i32;
    }

    /// `sub_100055F0` (82 bytes).
    pub(crate) fn f_100055f0(&mut self, mut this: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        v2 = self.r32(self.r32(self.r32(this)).wrapping_add(16));
        v3 = v2.wrapping_add(7600);
        v4 = v2.wrapping_add(4400);
        let _ = self.memset(v3, 255_i32 as u8, 192_u32);
        i = 0_i32;
        'l1: loop {
            if !(i < 350_i32) {
                break 'l1;
            }
            if ((self.r32(v4) as i32) != (1_i32).wrapping_neg()) {
                self.w32(
                    v3.wrapping_add(((self.r32(v4) as i32) as u32).wrapping_mul(4)),
                    (i as u32),
                );
            }
            v4 = v4.wrapping_add(4);
            i = i.wrapping_add(1);
        }
        j = 0_i32;
        'l2: loop {
            if !(j < 5_i32) {
                break 'l2;
            }
            let _ = self.f_10003b60(self.r32(this.wrapping_add(8)), j);
            j = j.wrapping_add(1);
        }
        return 0_i32;
    }

    /// `sub_10005650` (64 bytes).
    pub(crate) fn f_10005650(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: bool = false;
        v1 = 0_i32;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32)) as i32);
        v3 = v2.wrapping_add(1600_i32);
        v4 = (v2.wrapping_add(3000_i32) as u32);
        v5 = v3.wrapping_sub((v4 as i32));
        'l1: loop {
            if (!((self.r32(v4.wrapping_add((v5 as u32))) as i32) != 0)) {
                break 'l1;
            }
            v6 = (self.r32(v4) as i32).wrapping_add(1_i32);
            v7 = ((self.r32(v4) as i32).wrapping_sub(9999_i32) < 0_i32);
            self.w32(v4, (v6 as u32));
            if (!((((v7 as i32) ^ (v6.overflowing_sub(10000_i32).1 as i32))
                | ((v6 == 10000_i32) as i32))
                != 0))
            {
                self.w32(v4, 10000_u32);
            }
            v4 = v4.wrapping_add(4);
            v1 = v1.wrapping_add(1);
            if !(v1 < 350_i32) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10005690` (328 bytes).
    pub(crate) fn f_10005690(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: u32 = 0;
        let mut v8: bool = false;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = (self.r32(self.r32(this)) as i32);
                    v4 = (self.r32((v3.wrapping_add(16_i32) as u32)) as i32);
                    v5 = ((self.r32((v3.wrapping_add(12_i32) as u32)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(a2)) as u32);
                    v6 = 0x10028648_u32
                        .wrapping_add((35_i32.wrapping_mul(a3) as u32).wrapping_mul(4));
                    bb = if ((self.r32(v6) as i32) == 2_i32) {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    bb = if ((self.r32(v5.wrapping_add(220)) as i32)
                        > (self.r32(v5.wrapping_add(268)) as i32))
                    {
                        3
                    } else {
                        4
                    };
                }
                2 => {
                    bb = if ((self.r32(v6) as i32) != 3_i32) {
                        8
                    } else {
                        9
                    };
                }
                3 => {
                    self.w32((v4.wrapping_add(7348_i32) as u32), 1_u32);
                    return 0_i32;
                }
                4 => {
                    bb = 6;
                }
                5 => {
                    bb = 4;
                }
                6 => {
                    self.w32((v4.wrapping_add(7348_i32) as u32), 0_u32);
                    return 0_i32;
                }
                7 => {
                    bb = 2;
                }
                8 => {
                    bb = 6;
                }
                9 => {
                    v8 = (0_i32 != 0);
                    bb = if ((self.r32(v6.wrapping_add(36)) as i32) > 0_i32) {
                        11
                    } else {
                        12
                    };
                }
                10 => {
                    bb = 9;
                }
                11 => {
                    v8 = ((self.r32(v5.wrapping_add(68)) as i32) < 500_i32);
                    bb = 12;
                }
                12 => {
                    bb = if (((self.r32(v6.wrapping_add(44)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(132)) as i32) < 500_i32))
                    {
                        13
                    } else {
                        14
                    };
                }
                13 => {
                    v8 = (1_i32 != 0);
                    bb = 14;
                }
                14 => {
                    bb = if (((self.r32(v6.wrapping_add(48)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(140)) as i32) < 500_i32))
                    {
                        15
                    } else {
                        16
                    };
                }
                15 => {
                    v8 = (1_i32 != 0);
                    bb = 16;
                }
                16 => {
                    bb = if (((self.r32(v6.wrapping_add(52)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(144)) as i32) < 500_i32))
                    {
                        17
                    } else {
                        18
                    };
                }
                17 => {
                    v8 = (1_i32 != 0);
                    bb = 18;
                }
                18 => {
                    bb = if (((self.r32(v6.wrapping_add(56)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(148)) as i32) < 500_i32))
                    {
                        19
                    } else {
                        20
                    };
                }
                19 => {
                    v8 = (1_i32 != 0);
                    bb = 20;
                }
                20 => {
                    bb = if (((self.r32(v6.wrapping_add(60)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(152)) as i32) < 500_i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                21 => {
                    v8 = (1_i32 != 0);
                    bb = 22;
                }
                22 => {
                    bb = if (((self.r32(v6.wrapping_add(64)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(156)) as i32) < 500_i32))
                    {
                        23
                    } else {
                        24
                    };
                }
                23 => {
                    v8 = (1_i32 != 0);
                    bb = 24;
                }
                24 => {
                    bb = if (((self.r32(v6.wrapping_add(68)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(160)) as i32) < 500_i32))
                    {
                        25
                    } else {
                        26
                    };
                }
                25 => {
                    v8 = (1_i32 != 0);
                    bb = 26;
                }
                26 => {
                    bb = if (((self.r32(v6.wrapping_add(72)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(164)) as i32) < 100_i32))
                    {
                        27
                    } else {
                        28
                    };
                }
                27 => {
                    v8 = (1_i32 != 0);
                    bb = 28;
                }
                28 => {
                    bb = if ((((self.r32(v6.wrapping_add(76)) as i32) > 0_i32)
                        && ((self.r32(v5.wrapping_add(168)) as i32) < 10_i32))
                        || v8)
                    {
                        29
                    } else {
                        30
                    };
                }
                29 => {
                    self.w32((v4.wrapping_add(7348_i32) as u32), 1_u32);
                    return 0_i32;
                }
                30 => {
                    bb = if (!v8) { 32 } else { 33 };
                }
                31 => {
                    bb = 30;
                }
                32 => {
                    bb = 6;
                }
                33 => {
                    return 0_i32;
                }
                34 => {
                    bb = 33;
                }
                35 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100057E0` (496 bytes).
    pub(crate) fn f_100057e0(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
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
        v4 = ((self.r32(self.r32(self.r32(this)).wrapping_add(12)) as i32)
            .wrapping_add(716_i32.wrapping_mul(a2)) as u32);
        v5 = (self
            .r32(0x10028648_u32.wrapping_add((35_i32.wrapping_mul(a3) as u32).wrapping_mul(4)))
            as i32);
        v6 = 0x10028648_u32.wrapping_add((35_i32.wrapping_mul(a3) as u32).wrapping_mul(4));
        if (v5 == 2_i32) {
            v7 = (self.r32(v6.wrapping_add(36)) as i32);
            v8 = (self.r32(v4.wrapping_add(220)) as i32);
            v9 = v7.wrapping_add((self.r32(v4.wrapping_add(268)) as i32));
            self.w32(v4.wrapping_add(268), (v9 as u32));
            if (v8 < v9) {
                self.w32(v4.wrapping_add(268), (v8 as u32));
            }
            return 0_i32;
        }
        if (v5 != 3_i32) {
            return 0_i32;
        }
        v11 = (self.r32(v6.wrapping_add(36)) as i32);
        if (v11 > 0_i32) {
            v12 = v11.wrapping_add((self.r32(v4.wrapping_add(68)) as i32));
            self.w32(v4.wrapping_add(68), (v12 as u32));
            if (v12 > 500_i32) {
                self.w32(v4.wrapping_add(68), 500_u32);
            }
        }
        v13 = (self.r32(v6.wrapping_add(44)) as i32);
        if (v13 > 0_i32) {
            v14 = v13.wrapping_add((self.r32(v4.wrapping_add(132)) as i32));
            self.w32(v4.wrapping_add(132), (v14 as u32));
            if (v14 > 500_i32) {
                self.w32(v4.wrapping_add(132), 500_u32);
            }
            self.w32(
                v4.wrapping_add(136),
                ((self.r32(v4.wrapping_add(132)) as i32) as u32),
            );
        }
        v15 = (self.r32(v6.wrapping_add(48)) as i32);
        if (v15 > 0_i32) {
            v16 = v15.wrapping_add((self.r32(v4.wrapping_add(140)) as i32));
            self.w32(v4.wrapping_add(140), (v16 as u32));
            if (v16 > 500_i32) {
                self.w32(v4.wrapping_add(140), 500_u32);
            }
        }
        v17 = (self.r32(v6.wrapping_add(52)) as i32);
        if (v17 > 0_i32) {
            v18 = v17.wrapping_add((self.r32(v4.wrapping_add(144)) as i32));
            self.w32(v4.wrapping_add(144), (v18 as u32));
            if (v18 > 500_i32) {
                self.w32(v4.wrapping_add(144), 500_u32);
            }
        }
        v19 = (self.r32(v6.wrapping_add(56)) as i32);
        if (v19 > 0_i32) {
            v20 = v19.wrapping_add((self.r32(v4.wrapping_add(148)) as i32));
            self.w32(v4.wrapping_add(148), (v20 as u32));
            if (v20 > 500_i32) {
                self.w32(v4.wrapping_add(148), 500_u32);
            }
        }
        v21 = (self.r32(v6.wrapping_add(60)) as i32);
        if (v21 > 0_i32) {
            v22 = v21.wrapping_add((self.r32(v4.wrapping_add(152)) as i32));
            self.w32(v4.wrapping_add(152), (v22 as u32));
            if (v22 > 500_i32) {
                self.w32(v4.wrapping_add(152), 500_u32);
            }
        }
        v23 = (self.r32(v6.wrapping_add(64)) as i32);
        if (v23 > 0_i32) {
            v24 = v23.wrapping_add((self.r32(v4.wrapping_add(156)) as i32));
            self.w32(v4.wrapping_add(156), (v24 as u32));
            if (v24 > 500_i32) {
                self.w32(v4.wrapping_add(156), 500_u32);
            }
        }
        v25 = (self.r32(v6.wrapping_add(68)) as i32);
        if (v25 > 0_i32) {
            v26 = v25.wrapping_add((self.r32(v4.wrapping_add(160)) as i32));
            self.w32(v4.wrapping_add(160), (v26 as u32));
            if (v26 > 500_i32) {
                self.w32(v4.wrapping_add(160), 500_u32);
            }
        }
        v27 = (self.r32(v6.wrapping_add(72)) as i32);
        if (v27 > 0_i32) {
            v28 = v27.wrapping_add((self.r32(v4.wrapping_add(164)) as i32));
            self.w32(v4.wrapping_add(164), (v28 as u32));
            if (v28 > 100_i32) {
                self.w32(v4.wrapping_add(164), 100_u32);
            }
        }
        v29 = (self.r32(v6.wrapping_add(76)) as i32);
        if (v29 > 0_i32) {
            v30 = v29.wrapping_add((self.r32(v4.wrapping_add(168)) as i32));
            self.w32(v4.wrapping_add(168), (v30 as u32));
            if (v30 > 10_i32) {
                self.w32(v4.wrapping_add(168), 10_u32);
            }
        }
        let _ = self.f_100027d0(self.r32(this.wrapping_add(4)), 0_i32, a2);
        let _ = self.f_10002970(self.r32(this.wrapping_add(4)), 0_i32, a2, 0_i32, 0_i32);
        return 0_i32;
    }

    /// `sub_100059D0` (222 bytes).
    pub(crate) fn f_100059d0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = 0x10028648_u32;
                    v4 = this.wrapping_add(16);
                    bb = if (a2 > 45_u32) { 1 } else { 3 };
                }
                1 => {
                    bb = 4;
                }
                2 => {
                    v6 = 0_i32;
                    self.w32(this.wrapping_add(14416), 0_u32);
                    self.w32(this.wrapping_add(14420), 0_u32);
                    bb = 28;
                }
                3 => {
                    let t1 =
                        ((self.r8(0x10005ADC_u32.wrapping_add(((a2 as i32) as u32))) as i8) as i32);
                    bb = match t1 {
                        0_i32 => 6,
                        1_i32 => 7,
                        2_i32 => 8,
                        3_i32 => 9,
                        4_i32 => 10,
                        5_i32 => 11,
                        6_i32 => 12,
                        7_i32 => 13,
                        8_i32 => 14,
                        9_i32 => 15,
                        10_i32 => 16,
                        _ => 5,
                    };
                }
                4 => {
                    let _ = self.message_box(
                        self.r32(self.r32(
                            ((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(4_i32)
                                as u32),
                        )),
                        0x10065F14_u32,
                        0x10065F2C_u32,
                        48_u32,
                    );
                    v5 = (a2 as i32);
                    bb = 2;
                }
                5 => {
                    bb = 2;
                }
                6 => {
                    v5 = 0_i32;
                    bb = 5;
                }
                7 => {
                    v5 = 1_i32;
                    bb = 5;
                }
                8 => {
                    v5 = 2_i32;
                    bb = 5;
                }
                9 => {
                    v5 = 3_i32;
                    bb = 5;
                }
                10 => {
                    v5 = 4_i32;
                    bb = 5;
                }
                11 => {
                    v5 = 5_i32;
                    bb = 5;
                }
                12 => {
                    v5 = 6_i32;
                    bb = 5;
                }
                13 => {
                    v5 = 7_i32;
                    bb = 5;
                }
                14 => {
                    v5 = 8_i32;
                    bb = 5;
                }
                15 => {
                    v5 = 9_i32;
                    bb = 5;
                }
                16 => {
                    bb = 4;
                }
                17 => {
                    bb = 7;
                }
                18 => {
                    bb = 8;
                }
                19 => {
                    bb = 9;
                }
                20 => {
                    bb = 10;
                }
                21 => {
                    bb = 11;
                }
                22 => {
                    bb = 12;
                }
                23 => {
                    bb = 13;
                }
                24 => {
                    bb = 14;
                }
                25 => {
                    bb = 15;
                }
                26 => {
                    bb = 16;
                }
                27 => {
                    bb = 5;
                }
                28 => {
                    bb = if ((self.r32(v3) as i32) != 0) { 31 } else { 32 };
                }
                29 => {
                    bb = if (v6 < 1800_i32) { 28 } else { 30 };
                }
                30 => {
                    return 0_i32;
                }
                31 => {
                    v7 = (self
                        .r32(v3.wrapping_add((v5.wrapping_add(25_i32) as u32).wrapping_mul(4)))
                        as i32);
                    bb = if (v7 > 0_i32) { 33 } else { 34 };
                }
                32 => {
                    v3 = v3.wrapping_add(140);
                    v6 = v6.wrapping_add(1);
                    bb = 29;
                }
                33 => {
                    self.w32(
                        this.wrapping_add(14416),
                        ((self.r32(this.wrapping_add(14416)) as i32).wrapping_add(v7) as u32),
                    );
                    self.w32(v4, (v6 as u32));
                    self.w32(
                        v4.wrapping_add(4),
                        ((self.r32(this.wrapping_add(14416)) as i32) as u32),
                    );
                    v4 = v4.wrapping_add(8);
                    self.w32(
                        this.wrapping_add(14420),
                        ((self.r32(this.wrapping_add(14420)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 34;
                }
                34 => {
                    bb = 32;
                }
                35 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10005B10` (66 bytes).
    pub(crate) fn f_10005b10(&mut self, mut this: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut i: i32 = 0;
        v2 = this.wrapping_add(16);
        v3 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        v4 = crem_i32(self.rand(), (self.r32(this.wrapping_add(14416)) as i32));
        v5 = (self.r32(this.wrapping_add(14420)) as i32);
        i = 0_i32;
        'l1: loop {
            if !(i < v5) {
                break 'l1;
            }
            if (v4 <= (self.r32(v2.wrapping_add(4)) as i32)) {
                break 'l1;
            }
            v2 = v2.wrapping_add(8);
            i = i.wrapping_add(1);
        }
        self.w32(
            (v3.wrapping_add(7352_i32) as u32),
            ((self.r32(v2) as i32) as u32),
        );
        return 0_i32;
    }

    /// `sub_10005B80` (16 bytes).
    pub(crate) fn f_10005b80(&mut self, mut this: u32, mut a2: i32) -> i32 {
        self.w32(this, (a2 as u32));
        let _ = self.f_10005b90(this);
        return 0_i32;
    }

    /// `sub_10005B90` (17 bytes).
    pub(crate) fn f_10005b90(&mut self, mut this: u32) -> i32 {
        self.w32(this.wrapping_add(4), 0x10065F38_u32);
        self.w32(this.wrapping_add(8), 0x10066668_u32);
        return 0_i32;
    }

    /// `sub_10005BB0` (487 bytes).
    pub(crate) fn f_10005bb0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self
            .r32(this.wrapping_add(4))
            .wrapping_add((9_i32.wrapping_mul(a2) as u32).wrapping_mul(4));
        v5 = self
            .r32(this.wrapping_add(8))
            .wrapping_add((400_i32.wrapping_mul(a2) as u32).wrapping_mul(4));
        v6 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        self.w32(v3.wrapping_add(40), ((self.r32(v4) as i32) as u32));
        self.w32(
            v3.wrapping_add(44),
            ((self.r32(v4.wrapping_add(4)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(48),
            ((self.r32(v4.wrapping_add(8)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(52),
            ((self.r32(v4.wrapping_add(12)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(56),
            ((self.r32(v4.wrapping_add(16)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(60),
            ((self.r32(v4.wrapping_add(20)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(64),
            ((self.r32(v4.wrapping_add(24)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(68),
            ((self.r32(v4.wrapping_add(28)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(72),
            ((self.r32(v4.wrapping_add(32)) as i32) as u32),
        );
        v7 = (v6.wrapping_add(3600_i32) as u32);
        let _ = self.memset(v3.wrapping_add(76), 255_i32 as u8, 80_u32);
        v8 = v5;
        self.w32(v3.wrapping_add(156), 0_u32);
        v9 = 20_i32;
        'l1: loop {
            if ((self.r32(v8) as i32) == 1_i32) {
                self.w32(
                    v3.wrapping_add(156),
                    ((self.r32(v3.wrapping_add(156)) as i32).wrapping_add(1) as u32),
                );
            }
            v8 = v8.wrapping_add(80);
            v9 = v9.wrapping_sub(1);
            if !(v9 != 0) {
                break 'l1;
            }
        }
        v10 = (self.r32(v3.wrapping_add(156)) as i32);
        self.w32(v3.wrapping_add(164), 0_u32);
        self.w32(v3.wrapping_add(160), (v10 as u32));
        self.w32(v3.wrapping_add(168), 0_u32);
        self.w32(v3.wrapping_add(172), 0_u32);
        self.w32(v3.wrapping_add(176), 0_u32);
        self.w32(v3.wrapping_add(180), 0_u32);
        self.w32(v3.wrapping_add(184), 0_u32);
        self.w32(v3.wrapping_add(188), 0_u32);
        self.w32(v3.wrapping_add(192), 15_u32);
        self.w32(v3.wrapping_add(324), 0_u32);
        self.w32(v3.wrapping_add(328), 0_u32);
        self.w32(v3.wrapping_add(332), 0_u32);
        v11 = 20_i32;
        'l2: loop {
            v12 = (self.r32(v5) as i32);
            v5 = v5.wrapping_add(80);
            self.w32(v7, (v12 as u32));
            self.w32(
                v7.wrapping_add(4),
                ((self.r32(v5.wrapping_sub(76)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(8),
                ((self.r32(v5.wrapping_sub(72)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(12),
                ((self.r32(v5.wrapping_sub(68)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(16),
                ((self.r32(v5.wrapping_sub(64)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(20),
                ((self.r32(v5.wrapping_sub(60)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(24),
                ((self.r32(v5.wrapping_sub(56)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(28),
                ((self.r32(v5.wrapping_sub(52)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(32),
                ((self.r32(v5.wrapping_sub(48)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(36),
                ((self.r32(v5.wrapping_sub(44)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(40),
                ((self.r32(v5.wrapping_sub(40)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(44),
                ((self.r32(v5.wrapping_sub(36)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(48),
                ((self.r32(v5.wrapping_sub(32)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(52),
                ((self.r32(v5.wrapping_sub(28)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(56),
                ((self.r32(v5.wrapping_sub(24)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(60),
                ((self.r32(v5.wrapping_sub(20)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(64),
                ((self.r32(v5.wrapping_sub(16)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(68),
                ((self.r32(v5.wrapping_sub(60)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(72),
                ((self.r32(v5.wrapping_sub(56)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(76),
                ((self.r32(v5.wrapping_sub(52)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(80),
                ((self.r32(v5.wrapping_sub(48)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(84),
                ((self.r32(v5.wrapping_sub(44)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(88),
                ((self.r32(v5.wrapping_sub(40)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(92),
                ((self.r32(v5.wrapping_sub(36)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(96),
                ((self.r32(v5.wrapping_sub(32)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(100),
                ((self.r32(v5.wrapping_sub(28)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(104),
                ((self.r32(v5.wrapping_sub(24)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(108),
                ((self.r32(v5.wrapping_sub(20)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(112),
                ((self.r32(v5.wrapping_sub(16)) as i32) as u32),
            );
            self.w32(v7.wrapping_add(116), 0_u32);
            self.w32(v7.wrapping_add(120), 0_u32);
            self.w32(
                v7.wrapping_add(124),
                ((self.r32(v5.wrapping_sub(12)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(128),
                ((self.r32(v5.wrapping_sub(8)) as i32) as u32),
            );
            self.w32(
                v7.wrapping_add(132),
                ((self.r32(v5.wrapping_sub(4)) as i32) as u32),
            );
            self.w32(v7.wrapping_add(136), 0_u32);
            self.w32(v7.wrapping_add(140), 0_u32);
            self.w32(v7.wrapping_add(144), 0_u32);
            self.w32(v7.wrapping_add(148), 0_u32);
            self.w32(v7.wrapping_add(152), 0_u32);
            v7 = v7.wrapping_add(156);
            v11 = v11.wrapping_sub(1);
            if !(v11 != 0) {
                break 'l2;
            }
        }
        return 0_i32;
    }

    /// `sub_10005DA0` (406 bytes).
    pub(crate) fn f_10005da0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut i: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v1 = (self.r32(self.r32(this)) as i32);
                    v2 = self.r32((v1.wrapping_add(20_i32) as u32));
                    v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
                    let _ = self.memset(v2, 255_i32 as u8, 1600_u32);
                    v4 = v2.wrapping_add(3200);
                    let _ = self.memset(v2.wrapping_add(1600), 255_i32 as u8, 1600_u32);
                    let _ = self.memset(v2.wrapping_add(3200), 255_i32 as u8, 1600_u32);
                    v5 = 0_i32;
                    v6 = (v3.wrapping_add(384_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v6.wrapping_sub(384)) as i32) == 1_i32)
                        && ((self.r32(v6.wrapping_sub(12)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (v5 < 5_i32) { 1 } else { 3 };
                }
                3 => {
                    v7 = (v3.wrapping_add(3600_i32) as u32);
                    i = 0_i32;
                    bb = 6;
                }
                4 => {
                    self.w32(
                        v4.wrapping_add(
                            (20_i32
                                .wrapping_mul((self.r32(v6) as i32))
                                .wrapping_add((self.r32(v6.wrapping_sub(4)) as i32))
                                as u32)
                                .wrapping_mul(4),
                        ),
                        (v5 as u32),
                    );
                    bb = 5;
                }
                5 => {
                    v6 = v6.wrapping_add(716);
                    v5 = v5.wrapping_add(1);
                    bb = 2;
                }
                6 => {
                    bb = if (i < 20_i32) { 7 } else { 9 };
                }
                7 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        10
                    } else {
                        11
                    };
                }
                8 => {
                    i = i.wrapping_add(1);
                    bb = 6;
                }
                9 => {
                    let _ = self.f_10005f40(
                        this,
                        ((self.r32(v2.wrapping_add(4804)) as i32) as u32),
                        ((self.r32(v2.wrapping_add(4808)) as i32) as u32),
                        (self.r32(v2.wrapping_add(4816)) as i32),
                        (self.r32(v2.wrapping_add(4820)) as i32),
                    );
                    v9 = (self.r32(v2.wrapping_add(4804)) as i32).wrapping_add(
                        20_i32.wrapping_mul((self.r32(v2.wrapping_add(4808)) as i32)),
                    );
                    v10 = 0_i32;
                    self.w32(v2.wrapping_add(6440), 0_u32);
                    self.w32(v2.wrapping_add(6700), 0_u32);
                    bb = 12;
                }
                10 => {
                    self.w32(
                        v4.wrapping_add(
                            (20_i32
                                .wrapping_mul((self.r32(v7.wrapping_add(128)) as i32))
                                .wrapping_add((self.r32(v7.wrapping_add(124)) as i32))
                                as u32)
                                .wrapping_mul(4),
                        ),
                        (i.wrapping_add(5_i32) as u32),
                    );
                    bb = 11;
                }
                11 => {
                    v7 = v7.wrapping_add(156);
                    bb = 8;
                }
                12 => {
                    bb = if ((self.r32(v4.wrapping_sub(3200)) as i32) == (1_i32).wrapping_neg()) {
                        15
                    } else {
                        16
                    };
                }
                13 => {
                    bb = if (v10 < 400_i32) { 12 } else { 14 };
                }
                14 => {
                    return 0_i32;
                }
                15 => {
                    bb = 17;
                }
                16 => {
                    v11 = (self.r32(v2.wrapping_add(4800)) as i32);
                    v12 = (self.r32(v4) as i32);
                    bb = if (v11 != 0) { 19 } else { 21 };
                }
                17 => {
                    v10 = v10.wrapping_add(1);
                    v4 = v4.wrapping_add(4);
                    bb = 13;
                }
                18 => {
                    bb = 16;
                }
                19 => {
                    bb = if (v11 != 1_i32) { 22 } else { 23 };
                }
                20 => {
                    self.w32(
                        v2.wrapping_add(6440),
                        ((self.r32(v2.wrapping_add(6440)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 17;
                }
                21 => {
                    bb = if (((v12 != (1_i32).wrapping_neg()) && (v12 < 5_i32)) && (v10 != v9)) {
                        28
                    } else {
                        29
                    };
                }
                22 => {
                    bb = 17;
                }
                23 => {
                    bb = if ((v12 > 4_i32) && (v10 != v9)) {
                        25
                    } else {
                        26
                    };
                }
                24 => {
                    bb = 23;
                }
                25 => {
                    {
                        let a0 = v2.wrapping_add(
                            ({
                                let t1 = (self.r32(v2.wrapping_add(6700)) as i32);
                                self.w32(v2.wrapping_add(6700), (t1.wrapping_add(1) as u32));
                                t1
                            }
                            .wrapping_add(1650_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = (v10 as u32);
                        self.w32(a0, a1)
                    };
                    bb = 17;
                }
                26 => {
                    self.w32(
                        v2.wrapping_add(
                            ((self.r32(v2.wrapping_add(6440)) as i32).wrapping_add(1210_i32)
                                as u32)
                                .wrapping_mul(4),
                        ),
                        (v10 as u32),
                    );
                    bb = 20;
                }
                27 => {
                    bb = 26;
                }
                28 => {
                    {
                        let a0 = v2.wrapping_add(
                            ({
                                let t2 = (self.r32(v2.wrapping_add(6700)) as i32);
                                self.w32(v2.wrapping_add(6700), (t2.wrapping_add(1) as u32));
                                t2
                            }
                            .wrapping_add(1650_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = (v10 as u32);
                        self.w32(a0, a1)
                    };
                    bb = 17;
                }
                29 => {
                    self.w32(
                        v2.wrapping_add(
                            ((self.r32(v2.wrapping_add(6440)) as i32).wrapping_add(1210_i32)
                                as u32)
                                .wrapping_mul(4),
                        ),
                        (v10 as u32),
                    );
                    bb = 20;
                }
                30 => {
                    bb = 29;
                }
                31 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10005F40` (311 bytes).
    pub(crate) fn f_10005f40(
        &mut self,
        mut this: u32,
        mut a2: u32,
        mut a3: u32,
        mut a4: i32,
        mut a5: i32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v5 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(20_i32) as u32))
                        as i32);
                    bb = if ((a2 < 20_u32) && (a3 < 20_u32)) {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    v6 = a2.wrapping_add(20_u32.wrapping_mul(a3));
                    bb = if (a4 == (1_i32).wrapping_neg()) { 3 } else { 4 };
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    bb = 5;
                }
                4 => {
                    v7 = (self.r32((v5.wrapping_add(4800_i32) as u32)) as i32);
                    v8 = (self.r32(
                        (v5 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v6))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (!(v7 != 0)) { 7 } else { 8 };
                }
                5 => {
                    v10 = a5.wrapping_sub(((a4 != (1_i32).wrapping_neg()) as i32));
                    bb = if (v10 >= 0_i32) { 15 } else { 16 };
                }
                6 => {
                    bb = 4;
                }
                7 => {
                    bb = if (v8 > 4_i32) { 9 } else { 10 };
                }
                8 => {
                    bb = if (((v7 != 1_i32) || (v8 == (1_i32).wrapping_neg())) || (v8 >= 5_i32)) {
                        13
                    } else {
                        14
                    };
                }
                9 => {
                    return 0_i32;
                }
                10 => {
                    bb = 5;
                }
                11 => {
                    bb = 10;
                }
                12 => {
                    bb = 8;
                }
                13 => {
                    bb = 5;
                }
                14 => {
                    bb = 2;
                }
                15 => {
                    v11 = (4_u32.wrapping_mul(v6) as i32);
                    v12 = (self.r32(4_u32.wrapping_mul(v6).wrapping_add((v5 as u32))) as i32);
                    bb = if ((v12 == (1_i32).wrapping_neg())
                        || (v12
                            > (self.r32((v5.wrapping_add(4820_i32) as u32)) as i32)
                                .wrapping_sub(v10)))
                    {
                        17
                    } else {
                        18
                    };
                }
                16 => {
                    bb = 14;
                }
                17 => {
                    self.w32(
                        (v11.wrapping_add(v5) as u32),
                        ((self.r32((v5.wrapping_add(4820_i32) as u32)) as i32).wrapping_sub(v10)
                            as u32),
                    );
                    v13 = a4;
                    self.w32(
                        (v11.wrapping_add(v5).wrapping_add(1600_i32) as u32),
                        (a4 as u32),
                    );
                    bb = if (a4 != 2_i32) { 19 } else { 20 };
                }
                18 => {
                    bb = 16;
                }
                19 => {
                    let _ = self.f_10005f40(this, a2, a3.wrapping_sub(1_u32), 0_i32, v10);
                    v13 = a4;
                    bb = 20;
                }
                20 => {
                    bb = if (v13 != 3_i32) { 21 } else { 22 };
                }
                21 => {
                    let _ = self.f_10005f40(this, a2.wrapping_add(1_u32), a3, 1_i32, v10);
                    v13 = a4;
                    bb = 22;
                }
                22 => {
                    bb = if (v13 != 0) { 23 } else { 24 };
                }
                23 => {
                    let _ = self.f_10005f40(this, a2, a3.wrapping_add(1_u32), 2_i32, v10);
                    v13 = a4;
                    bb = 24;
                }
                24 => {
                    bb = if (v13 != 1_i32) { 25 } else { 26 };
                }
                25 => {
                    let _ = self.f_10005f40(this, a2.wrapping_sub(1_u32), a3, 3_i32, v10);
                    bb = 26;
                }
                26 => {
                    bb = 18;
                }
                27 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10006080` (125 bytes).
    pub(crate) fn f_10006080(&mut self, mut this: u32, mut a2: i32, mut a3: i32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: u32 = 0;
        v3 = 0_i32;
        v4 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(20_i32) as u32)) as i32);
        v5 = a2.wrapping_add(20_i32.wrapping_mul(a3));
        v6 =
            (self.r32((v4.wrapping_add(4_i32.wrapping_mul(v5)) as u32)) as i32).wrapping_add(1_i32);
        self.w32((v4.wrapping_add(7800_i32) as u32), (v6 as u32));
        v7 = (v4
            .wrapping_add(4_i32.wrapping_mul(v6))
            .wrapping_add(7596_i32) as u32);
        if (v6 <= 0_i32) {
            return 0_i32;
        }
        v8 = (v4
            .wrapping_add(4_i32.wrapping_mul(v5))
            .wrapping_add(1600_i32) as u32);
        'l1: loop {
            self.w32(v7, (v5 as u32));
            's2: {
                'b2_4: {
                    'b2_3: {
                        'b2_2: {
                            'b2_1: {
                                'b2_0: {
                                    match (self.r32(v8) as i32) {
                                        0_i32 => break 'b2_0,
                                        1_i32 => break 'b2_1,
                                        2_i32 => break 'b2_2,
                                        3_i32 => break 'b2_3,
                                        _ => break 'b2_4,
                                    }
                                }
                                v5 = v5.wrapping_add(20_i32);
                                v8 = v8.wrapping_add(80);
                                break 's2;
                            }
                            v5 = v5.wrapping_sub(1);
                            v8 = v8.wrapping_sub(4);
                            break 's2;
                        }
                        v5 = v5.wrapping_sub(20_i32);
                        v8 = v8.wrapping_sub(80);
                        break 's2;
                    }
                    v5 = v5.wrapping_add(1);
                    v8 = v8.wrapping_add(4);
                    break 's2;
                }
                break 's2;
            }
            v7 = v7.wrapping_sub(4);
            v3 = v3.wrapping_add(1);
            if !(v3 < (self.r32((v4.wrapping_add(7800_i32) as u32)) as i32)) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10006110` (16 bytes).
    pub(crate) fn f_10006110(&mut self, mut this: u32, mut a2: i32) -> i32 {
        self.w32(this, (a2 as u32));
        let _ = self.f_10006130(this);
        return 0_i32;
    }

    /// `sub_10006120` (12 bytes).
    pub(crate) fn f_10006120(&mut self, mut this: u32, mut a2: i32) -> i32 {
        self.w32(this.wrapping_add(4), (a2 as u32));
        return 0_i32;
    }

    /// `sub_10006130` (486 bytes).
    pub(crate) fn f_10006130(&mut self, mut this: u32) -> i32 {
        self.w32(this.wrapping_add(12), 0x1007A528_u32);
        self.w32(this.wrapping_add(16), 0x1007A53C_u32);
        self.w32(this.wrapping_add(20), 0x1007A570_u32);
        self.w32(this.wrapping_add(24), 0x1007A5E4_u32);
        self.w32(this.wrapping_add(28), 0x1007A6B8_u32);
        self.w32(this.wrapping_add(32), 0x1007A80C_u32);
        self.w32(this.wrapping_add(36), 0x1007AA00_u32);
        self.w32(this.wrapping_add(40), 0x1007ACB4_u32);
        self.w32(this.wrapping_add(44), 0x1007B048_u32);
        self.w32(this.wrapping_add(48), 0x1007B4DC_u32);
        self.w32(this.wrapping_add(52), 0x1007B4F0_u32);
        self.w32(this.wrapping_add(56), 0x1007B524_u32);
        self.w32(this.wrapping_add(60), 0x1007B598_u32);
        self.w32(this.wrapping_add(64), 0x1007B66C_u32);
        self.w32(this.wrapping_add(68), 0x1007B7C0_u32);
        self.w32(this.wrapping_add(72), 0x1007B9B4_u32);
        self.w32(this.wrapping_add(76), 0x1007B9C8_u32);
        self.w32(this.wrapping_add(80), 0x1007B9FC_u32);
        self.w32(this.wrapping_add(84), 0x1007BA70_u32);
        self.w32(this.wrapping_add(88), 0x1007BB44_u32);
        self.w32(this.wrapping_add(92), 0x1007BC98_u32);
        self.w32(this.wrapping_add(96), 0x1007BE8C_u32);
        self.w32(this.wrapping_add(100), 0x1007C140_u32);
        self.w32(this.wrapping_add(104), 0x1007C4D4_u32);
        self.w32(this.wrapping_add(108), 0x1007C968_u32);
        self.w32(this.wrapping_add(112), 0x1007C97C_u32);
        self.w32(this.wrapping_add(116), 0x1007C9B0_u32);
        self.w32(this.wrapping_add(120), 0x1007CA24_u32);
        self.w32(this.wrapping_add(124), 0x1007CAF8_u32);
        self.w32(this.wrapping_add(128), 0x1007CC4C_u32);
        self.w32(this.wrapping_add(132), 0x1007CE40_u32);
        self.w32(this.wrapping_add(136), 0x1007D0F4_u32);
        self.w32(this.wrapping_add(140), 0x1007D488_u32);
        self.w32(this.wrapping_add(144), 0x1007D91C_u32);
        self.w32(this.wrapping_add(148), 0x1007D930_u32);
        self.w32(this.wrapping_add(152), 0x1007D964_u32);
        self.w32(this.wrapping_add(156), 0x1007D9D8_u32);
        self.w32(this.wrapping_add(160), 0x1007DAAC_u32);
        self.w32(this.wrapping_add(164), 0x1007DC00_u32);
        self.w32(this.wrapping_add(168), 0x1007DDF4_u32);
        self.w32(this.wrapping_add(172), 0x1007DE08_u32);
        self.w32(this.wrapping_add(176), 0x1007DE3C_u32);
        self.w32(this.wrapping_add(180), 0x1007DEB0_u32);
        self.w32(this.wrapping_add(184), 0x1007DEE4_u32);
        self.w32(this.wrapping_add(188), 0x1007DF58_u32);
        self.w32(this.wrapping_add(192), 0x1007DFCC_u32);
        self.w32(this.wrapping_add(196), 0x1007E120_u32);
        self.w32(this.wrapping_add(200), 0x1007E314_u32);
        self.w32(this.wrapping_add(204), 0x1007E5C8_u32);
        self.w32(this.wrapping_add(208), 0x1007E87C_u32);
        self.w32(this.wrapping_add(212), 0x1007EC10_u32);
        self.w32(this.wrapping_add(216), 0x1007F0A4_u32);
        self.w32(this.wrapping_add(220), 0x1007F0B8_u32);
        self.w32(this.wrapping_add(224), 0x1007F0CC_u32);
        self.w32(this.wrapping_add(228), 0x1007F100_u32);
        self.w32(this.wrapping_add(232), 0x1007F174_u32);
        self.w32(this.wrapping_add(236), 0x1007F248_u32);
        return 0_i32;
    }

    /// `sub_10006320` (249 bytes).
    pub(crate) fn f_10006320(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        self.w32(
            ((self.r32((v1.wrapping_add(20_i32) as u32)) as i32).wrapping_add(7400_i32) as u32),
            0_u32,
        );
        self.w32(
            v2.wrapping_add(1416),
            ((self.r32(v2.wrapping_add(1384)) as i32) as u32),
        );
        self.w32(
            v2.wrapping_add(1420),
            ((self.r32(v2.wrapping_add(1388)) as i32) as u32),
        );
        self.w32(v2.wrapping_add(1424), 0_u32);
        self.w32(v2.wrapping_add(1428), 0_u32);
        self.w32(v2.wrapping_add(1432), 0_u32);
        self.w32(v2.wrapping_add(1436), 0_u32);
        self.w32(v2.wrapping_add(1440), 0_u32);
        self.w32(v2.wrapping_add(1444), 0_u32);
        self.w32(v2.wrapping_add(1448), 0_u32);
        self.w32(v2.wrapping_add(1464), 0_u32);
        self.w32(v2.wrapping_add(1504), 0_u32);
        self.w32(v2.wrapping_add(1540), 0_u32);
        self.w32(v2.wrapping_add(1544), 0_u32);
        self.w32(v2.wrapping_add(1548), 0_u32);
        self.w32(v2.wrapping_add(1552), 0_u32);
        self.w32(v2.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
        self.w32(v2.wrapping_add(1560), 0_u32);
        self.w32(v2.wrapping_add(1564), 0_u32);
        self.w32(v2.wrapping_add(1568), 0_u32);
        self.w32(v2.wrapping_add(2072), 0_u32);
        self.w32(v2.wrapping_add(2176), 0_u32);
        self.w32(v2.wrapping_add(2180), 0_u32);
        self.w32(v2.wrapping_add(2188), 0_u32);
        self.w32(v2.wrapping_add(2280), 0_u32);
        self.w32(v2.wrapping_add(2336), 0_u32);
        self.w32(v2.wrapping_add(2340), 0_u32);
        self.w32(v2.wrapping_add(2344), 0_u32);
        self.w32(v2.wrapping_add(2348), 0_u32);
        self.w32(v2.wrapping_add(2352), 0_u32);
        self.w32(v2.wrapping_add(2356), 0_u32);
        self.w32(v2.wrapping_add(2360), 0_u32);
        v3 = v2.wrapping_add(2384);
        v4 = 5_i32;
        'l1: loop {
            self.w32(v3.wrapping_sub(20), 0_u32);
            self.w32(v3, 0_u32);
            self.w32(v3.wrapping_add(20), 0_u32);
            v3 = v3.wrapping_add(4);
            v4 = v4.wrapping_sub(1);
            if !(v4 != 0) {
                break 'l1;
            }
        }
        return 0_i32;
    }

    /// `sub_10006420` (181 bytes).
    pub(crate) fn f_10006420(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        self.w32(
            ((self.r32((v1.wrapping_add(20_i32) as u32)) as i32).wrapping_add(7400_i32) as u32),
            0_u32,
        );
        self.w32(
            v2.wrapping_add(1416),
            ((self.r32(v2.wrapping_add(1384)) as i32) as u32),
        );
        self.w32(
            v2.wrapping_add(1420),
            ((self.r32(v2.wrapping_add(1388)) as i32) as u32),
        );
        self.w32(v2.wrapping_add(1424), 0_u32);
        self.w32(v2.wrapping_add(1428), 0_u32);
        self.w32(v2.wrapping_add(1432), 0_u32);
        self.w32(v2.wrapping_add(1436), 0_u32);
        self.w32(v2.wrapping_add(1440), 0_u32);
        self.w32(v2.wrapping_add(1444), 0_u32);
        self.w32(v2.wrapping_add(1448), 0_u32);
        self.w32(v2.wrapping_add(1464), 0_u32);
        self.w32(v2.wrapping_add(1504), 0_u32);
        self.w32(v2.wrapping_add(1540), 0_u32);
        self.w32(v2.wrapping_add(1544), 0_u32);
        self.w32(v2.wrapping_add(1548), 0_u32);
        self.w32(v2.wrapping_add(1552), 0_u32);
        self.w32(v2.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
        self.w32(v2.wrapping_add(1560), 0_u32);
        self.w32(v2.wrapping_add(1564), 0_u32);
        self.w32(v2.wrapping_add(1568), 0_u32);
        self.w32(v2.wrapping_add(2072), 0_u32);
        self.w32(v2.wrapping_add(2176), 0_u32);
        self.w32(v2.wrapping_add(2180), 0_u32);
        self.w32(v2.wrapping_add(2188), 0_u32);
        self.w32(v2.wrapping_add(2280), 0_u32);
        return 0_i32;
    }

    /// `sub_100064E0` (968 bytes).
    pub(crate) fn f_100064e0(
        &mut self,
        mut this: u32,
        mut a2: u32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
    ) -> i32 {
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut result: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v6 = (self.r32(self.r32(this)) as i32);
                    v7 = (self.r32((v6.wrapping_add(16_i32) as u32)) as i32);
                    v14 = (self.r32((v6.wrapping_add(12_i32) as u32)) as i32);
                    bb = if (a4 != 0) { 1 } else { 3 };
                }
                1 => {
                    bb = if (a4 == 1_i32) { 4 } else { 5 };
                }
                2 => {
                    bb = 42;
                }
                3 => {
                    bb = if (a6 != 0) { 28 } else { 30 };
                }
                4 => {
                    bb = if (a6 != 0) { 6 } else { 8 };
                }
                5 => {
                    bb = 2;
                }
                6 => {
                    bb = if (a6 == 1_i32) { 9 } else { 10 };
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    self.w8((a3 as u32), 1_u8);
                    self.w32((a3.wrapping_add(4_i32) as u32), 0_u32);
                    self.w32((a3.wrapping_add(8_i32) as u32), 0_u32);
                    let t1 = (self.r32(
                        (v14.wrapping_add(156_i32.wrapping_mul(a5))
                            .wrapping_add(3604_i32) as u32),
                    ) as i32);
                    bb = match t1 {
                        0_i32 => 12,
                        1_i32 => 13,
                        2_i32 => 14,
                        3_i32 => 15,
                        4_i32 => 16,
                        5_i32 => 17,
                        6_i32 => 18,
                        _ => 19,
                    };
                }
                9 => {
                    self.w8((a3 as u32), 0_u8);
                    bb = 10;
                }
                10 => {
                    bb = 7;
                }
                11 => {
                    bb = 7;
                }
                12 => {
                    self.w32(a2, 22_u32);
                    bb = 11;
                }
                13 => {
                    self.w32(a2, 23_u32);
                    bb = 11;
                }
                14 => {
                    self.w32(a2, 24_u32);
                    bb = 11;
                }
                15 => {
                    self.w32(a2, 25_u32);
                    bb = 11;
                }
                16 => {
                    self.w32(a2, 26_u32);
                    bb = 11;
                }
                17 => {
                    self.w32(a2, 27_u32);
                    bb = 11;
                }
                18 => {
                    self.w32(a2, 29_u32);
                    bb = 11;
                }
                19 => {
                    bb = 11;
                }
                20 => {
                    bb = 13;
                }
                21 => {
                    bb = 14;
                }
                22 => {
                    bb = 15;
                }
                23 => {
                    bb = 16;
                }
                24 => {
                    bb = 17;
                }
                25 => {
                    bb = 18;
                }
                26 => {
                    bb = 19;
                }
                27 => {
                    bb = 11;
                }
                28 => {
                    bb = if (a6 == 1_i32) { 31 } else { 32 };
                }
                29 => {
                    bb = 2;
                }
                30 => {
                    bb = if (!((self.r32(
                        (v7.wrapping_add(4_i32.wrapping_mul(a5))
                            .wrapping_add(7792_i32) as u32),
                    ) as i32)
                        != 0))
                    {
                        40
                    } else {
                        41
                    };
                }
                31 => {
                    v13 = ((self.r32(
                        (v7.wrapping_add(4_i32.wrapping_mul(a5))
                            .wrapping_add(7792_i32) as u32),
                    ) as i32) as u32);
                    bb = if (v13 < 4_u32) { 33 } else { 35 };
                }
                32 => {
                    bb = 29;
                }
                33 => {
                    self.w8((a3 as u32), 0_u8);
                    bb = 34;
                }
                34 => {
                    bb = 32;
                }
                35 => {
                    bb = if (v13 == 4_u32) { 36 } else { 37 };
                }
                36 => {
                    self.w8((a3 as u32), 1_u8);
                    self.w32((a3.wrapping_add(4_i32) as u32), 1_u32);
                    bb = 38;
                }
                37 => {
                    bb = 34;
                }
                38 => {
                    self.w32((a3.wrapping_add(8_i32) as u32), 1_u32);
                    v12 = (self.r32(
                        (v7.wrapping_add(32_i32.wrapping_mul(a5))
                            .wrapping_add(7604_i32) as u32),
                    ) as i32);
                    bb = 49;
                }
                39 => {
                    bb = 37;
                }
                40 => {
                    self.w8((a3 as u32), 1_u8);
                    self.w32((a3.wrapping_add(4_i32) as u32), 0_u32);
                    self.w32((a3.wrapping_add(8_i32) as u32), 0_u32);
                    self.w32(a2, 1_u32);
                    bb = 42;
                }
                41 => {
                    v8 = (self.r32(
                        (v7.wrapping_add(4_i32.wrapping_mul(a5))
                            .wrapping_add(7792_i32) as u32),
                    ) as i32);
                    let t2 = v8;
                    bb = match t2 {
                        1_i32 => 45,
                        4_i32 => 46,
                        3_i32 => 47,
                        2_i32 => 48,
                        _ => 44,
                    };
                }
                42 => {
                    bb = if ((self.r8((a3 as u32)) as i32) != 1_i32) {
                        57
                    } else {
                        58
                    };
                }
                43 => {
                    bb = 41;
                }
                44 => {
                    bb = 29;
                }
                45 => {
                    bb = 46;
                }
                46 => {
                    self.w8((a3 as u32), 1_u8);
                    self.w32((a3.wrapping_add(4_i32) as u32), 0_u32);
                    self.w32((a3.wrapping_add(8_i32) as u32), 0_u32);
                    v12 = (self.r32(
                        (v7.wrapping_add(32_i32.wrapping_mul(a5))
                            .wrapping_add(7600_i32) as u32),
                    ) as i32);
                    bb = 49;
                }
                47 => {
                    self.w8((a3 as u32), 1_u8);
                    self.w32((a3.wrapping_add(4_i32) as u32), 0_u32);
                    v15 = v7.wrapping_add(32_i32.wrapping_mul(a5));
                    v9 = (self.r32((v15.wrapping_add(7600_i32) as u32)) as i32);
                    bb = if (v9 == (1_i32).wrapping_neg()) {
                        51
                    } else {
                        53
                    };
                }
                48 => {
                    self.w8((a3 as u32), 1_u8);
                    self.w32((a3.wrapping_add(4_i32) as u32), 0_u32);
                    bb = 38;
                }
                49 => {
                    v10 = 35_i32.wrapping_mul(
                        (self.r32(
                            (v7.wrapping_add(4_i32.wrapping_mul(v12))
                                .wrapping_add(1600_i32) as u32),
                        ) as i32),
                    );
                    bb = 54;
                }
                50 => {
                    bb = 47;
                }
                51 => {
                    v9 = (self.r32((v15.wrapping_add(7604_i32) as u32)) as i32);
                    self.w32((a3.wrapping_add(8_i32) as u32), 1_u32);
                    bb = 52;
                }
                52 => {
                    v10 = 35_i32.wrapping_mul(
                        (self.r32(
                            (v7.wrapping_add(4_i32.wrapping_mul(v9))
                                .wrapping_add(1600_i32) as u32),
                        ) as i32),
                    );
                    bb = 54;
                }
                53 => {
                    self.w32((a3.wrapping_add(8_i32) as u32), 0_u32);
                    bb = 52;
                }
                54 => {
                    self.w32(
                        a2,
                        ((self.r32(
                            ((self.r32(self.r32(this.wrapping_add(8)).wrapping_add(12)) as i32)
                                .wrapping_add(4_i32.wrapping_mul(v10))
                                .wrapping_add(12_i32) as u32),
                        ) as i32) as u32),
                    );
                    bb = 44;
                }
                55 => {
                    bb = 48;
                }
                56 => {
                    bb = 44;
                }
                57 => {
                    return 0_i32;
                }
                58 => {
                    bb = if (!(a4 != 0)) { 60 } else { 61 };
                }
                59 => {
                    bb = 58;
                }
                60 => {
                    let _ = self.f_100086f0(
                        this,
                        0_i32,
                        a5,
                        (self.r32((a3.wrapping_add(8_i32) as u32)) as i32),
                        a2,
                    );
                    bb = 61;
                }
                61 => {
                    let t3 = (self.r32(a2) as i32);
                    bb = match t3 {
                        1_i32 => 63,
                        2_i32 => 64,
                        3_i32 => 65,
                        4_i32 => 66,
                        6_i32 => 67,
                        8_i32 => 68,
                        9_i32 => 69,
                        22_i32 => 70,
                        23_i32 => 71,
                        24_i32 => 72,
                        25_i32 => 73,
                        26_i32 => 74,
                        29_i32 => 75,
                        30_i32 => 76,
                        5_i32 => 77,
                        7_i32 => 78,
                        13_i32 => 79,
                        10_i32 => 80,
                        11_i32 => 81,
                        12_i32 => 82,
                        14_i32 => 83,
                        15_i32 => 84,
                        16_i32 => 85,
                        17_i32 => 86,
                        18_i32 => 87,
                        19_i32 => 88,
                        20_i32 => 89,
                        21_i32 => 90,
                        27_i32 => 91,
                        28_i32 => 92,
                        _ => 93,
                    };
                }
                62 => {
                    return result;
                }
                63 => {
                    bb = 64;
                }
                64 => {
                    bb = 65;
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
                    bb = 69;
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
                    bb = 74;
                }
                74 => {
                    bb = 75;
                }
                75 => {
                    bb = 76;
                }
                76 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 3_u32);
                    self.w32(a2.wrapping_add(12), 1_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    return 0_i32;
                }
                77 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(12), 0_u32);
                    self.w32(a2.wrapping_add(8), 6_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                78 => {
                    bb = 79;
                }
                79 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 5_u32);
                    self.w32(a2.wrapping_add(12), 1_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                80 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 3_u32);
                    self.w32(a2.wrapping_add(12), 2_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                81 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 3_u32);
                    self.w32(a2.wrapping_add(12), 3_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                82 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 3_u32);
                    self.w32(a2.wrapping_add(12), 5_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                83 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 5_u32);
                    self.w32(a2.wrapping_add(12), 2_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                84 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(12), 0_u32);
                    self.w32(a2.wrapping_add(8), 7_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                85 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(12), 0_u32);
                    self.w32(a2.wrapping_add(8), 8_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                86 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(16), 0_u32);
                    self.w32(a2.wrapping_add(8), 3_u32);
                    self.w32(a2.wrapping_add(12), 8_u32);
                    result = 0_i32;
                    bb = 62;
                }
                87 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 0_u32);
                    self.w32(a2.wrapping_add(16), 0_u32);
                    self.w32(a2.wrapping_add(12), 8_u32);
                    result = 0_i32;
                    bb = 62;
                }
                88 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 0_u32);
                    self.w32(a2.wrapping_add(12), 3_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                89 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 3_u32);
                    self.w32(a2.wrapping_add(12), 8_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                90 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 0_u32);
                    self.w32(a2.wrapping_add(12), 4_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                91 => {
                    bb = 92;
                }
                92 => {
                    self.w32(a2.wrapping_add(4), 0_u32);
                    self.w32(a2.wrapping_add(8), 1_u32);
                    self.w32(a2.wrapping_add(12), 1_u32);
                    self.w32(a2.wrapping_add(16), 1_u32);
                    result = 0_i32;
                    bb = 62;
                }
                93 => {
                    return 0_i32;
                }
                94 => {
                    bb = 77;
                }
                95 => {
                    bb = 78;
                }
                96 => {
                    bb = 80;
                }
                97 => {
                    bb = 81;
                }
                98 => {
                    bb = 82;
                }
                99 => {
                    bb = 83;
                }
                100 => {
                    bb = 84;
                }
                101 => {
                    bb = 85;
                }
                102 => {
                    bb = 86;
                }
                103 => {
                    bb = 87;
                }
                104 => {
                    bb = 88;
                }
                105 => {
                    bb = 89;
                }
                106 => {
                    bb = 90;
                }
                107 => {
                    bb = 91;
                }
                108 => {
                    bb = 93;
                }
                109 => {
                    bb = 62;
                }
                110 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10006940` (412 bytes).
    pub(crate) fn f_10006940(
        &mut self,
        mut this: u32,
        mut a2: u32,
        mut a3: i32,
        mut a4: i32,
        mut a5: i32,
        mut a6: i32,
    ) -> i32 {
        let mut result: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    let t1 = a4;
                    bb = match t1 {
                        0_i32 => 2,
                        1_i32 => 3,
                        2_i32 => 4,
                        3_i32 => 5,
                        4_i32 => 6,
                        5_i32 => 7,
                        6_i32 => 8,
                        7_i32 => 9,
                        8_i32 => 10,
                        9_i32 => 11,
                        _ => 12,
                    };
                }
                1 => {
                    return result;
                }
                2 => {
                    self.w32(
                        a2,
                        ((self.r32(this.wrapping_add(
                            (a5.wrapping_add(3_i32).wrapping_add(a3) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    result = 0_i32;
                    bb = 1;
                }
                3 => {
                    self.w32(
                        a2,
                        ((self.r32(
                            this.wrapping_add((a5.wrapping_add(12_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    result = 0_i32;
                    bb = 1;
                }
                4 => {
                    result = a3;
                    self.w32(
                        a2,
                        ((self.r32(this.wrapping_add(
                            (a3.wrapping_add(3_i32).wrapping_add(a5) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    bb = if (a3 != 0) { 15 } else { 17 };
                }
                5 => {
                    result = 0_i32;
                    self.w32(
                        a2,
                        ((self.r32(
                            this.wrapping_add((a5.wrapping_add(27_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    bb = 1;
                }
                6 => {
                    result = 0_i32;
                    self.w32(
                        a2,
                        ((self.r32(
                            this.wrapping_add((a5.wrapping_add(36_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    bb = 1;
                }
                7 => {
                    result = 0_i32;
                    self.w32(
                        a2,
                        ((self.r32(
                            this.wrapping_add((a5.wrapping_add(42_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    bb = 1;
                }
                8 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(180)) as i32) as u32));
                    result = 0_i32;
                    bb = 1;
                }
                9 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(184)) as i32) as u32));
                    result = 0_i32;
                    bb = 1;
                }
                10 => {
                    result = 0_i32;
                    self.w32(a2, ((self.r32(this.wrapping_add(188)) as i32) as u32));
                    bb = 1;
                }
                11 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(44)) as i32) as u32));
                    bb = 20;
                }
                12 => {
                    bb = 20;
                }
                13 => {
                    bb = 3;
                }
                14 => {
                    bb = 4;
                }
                15 => {
                    bb = if (a5.wrapping_sub(a3) < 2_i32) {
                        18
                    } else {
                        19
                    };
                }
                16 => {
                    bb = 1;
                }
                17 => {
                    self.w32(
                        a2,
                        ((self.r32(
                            this.wrapping_add((a5.wrapping_add(18_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    bb = 16;
                }
                18 => {
                    bb = 20;
                }
                19 => {
                    let t2 = a5;
                    bb = match t2 {
                        3_i32 => 23,
                        4_i32 => 24,
                        5_i32 => 25,
                        _ => 26,
                    };
                }
                20 => {
                    result = 0_i32;
                    bb = 1;
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    bb = 16;
                }
                23 => {
                    bb = if (a3 != 1_i32) { 27 } else { 28 };
                }
                24 => {
                    bb = if (a3 == 1_i32) { 31 } else { 33 };
                }
                25 => {
                    let t3 = a3;
                    bb = match t3 {
                        1_i32 => 39,
                        2_i32 => 40,
                        3_i32 => 41,
                        _ => 42,
                    };
                }
                26 => {
                    bb = 20;
                }
                27 => {
                    bb = 20;
                }
                28 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(192)) as i32) as u32));
                    result = 0_i32;
                    bb = 22;
                }
                29 => {
                    bb = 28;
                }
                30 => {
                    bb = 24;
                }
                31 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(196)) as i32) as u32));
                    result = 0_i32;
                    bb = 32;
                }
                32 => {
                    bb = 22;
                }
                33 => {
                    bb = if (a3 != 2_i32) { 34 } else { 35 };
                }
                34 => {
                    bb = 20;
                }
                35 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(200)) as i32) as u32));
                    result = 0_i32;
                    bb = 32;
                }
                36 => {
                    bb = 35;
                }
                37 => {
                    bb = 25;
                }
                38 => {
                    bb = 22;
                }
                39 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(204)) as i32) as u32));
                    result = 0_i32;
                    bb = 38;
                }
                40 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(208)) as i32) as u32));
                    result = 0_i32;
                    bb = 38;
                }
                41 => {
                    self.w32(a2, ((self.r32(this.wrapping_add(212)) as i32) as u32));
                    result = 0_i32;
                    bb = 38;
                }
                42 => {
                    bb = 20;
                }
                43 => {
                    bb = 40;
                }
                44 => {
                    bb = 41;
                }
                45 => {
                    bb = 42;
                }
                46 => {
                    bb = 38;
                }
                47 => {
                    bb = 26;
                }
                48 => {
                    bb = 22;
                }
                49 => {
                    bb = 5;
                }
                50 => {
                    bb = 6;
                }
                51 => {
                    bb = 7;
                }
                52 => {
                    bb = 8;
                }
                53 => {
                    bb = 9;
                }
                54 => {
                    bb = 10;
                }
                55 => {
                    bb = 11;
                }
                56 => {
                    bb = 12;
                }
                57 => {
                    bb = 1;
                }
                58 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10006B10` (78 bytes).
    pub(crate) fn f_10006b10(&mut self, mut this: u32, mut a2: i32, mut a3: u32) -> i32 {
        let mut result: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    let t1 = (self.r32(
                        ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32))
                            as i32)
                            .wrapping_add(4_i32.wrapping_mul(a2))
                            .wrapping_add(7792_i32) as u32),
                    ) as i32);
                    bb = match t1 {
                        0_i32 => 2,
                        1_i32 => 3,
                        2_i32 => 4,
                        4_i32 => 5,
                        3_i32 => 6,
                        _ => 7,
                    };
                }
                1 => {
                    return result;
                }
                2 => {
                    result = 0_i32;
                    self.w8(a3, 1_u8);
                    bb = 1;
                }
                3 => {
                    self.w8(a3, 0_u8);
                    result = 0_i32;
                    bb = 1;
                }
                4 => {
                    bb = 5;
                }
                5 => {
                    self.w8(a3, 0_u8);
                    bb = 10;
                }
                6 => {
                    result = 0_i32;
                    self.w8(a3, 0_u8);
                    bb = 1;
                }
                7 => {
                    bb = 10;
                }
                8 => {
                    bb = 3;
                }
                9 => {
                    bb = 4;
                }
                10 => {
                    result = 0_i32;
                    bb = 1;
                }
                11 => {
                    bb = 6;
                }
                12 => {
                    bb = 7;
                }
                13 => {
                    bb = 1;
                }
                14 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10006B80` (78 bytes).
    pub(crate) fn f_10006b80(&mut self, mut this: u32, mut a2: i32, mut a3: u32) -> i32 {
        let mut result: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    let t1 = (self.r32(
                        ((self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(16_i32) as u32))
                            as i32)
                            .wrapping_add(4_i32.wrapping_mul(a2))
                            .wrapping_add(7792_i32) as u32),
                    ) as i32);
                    bb = match t1 {
                        0_i32 => 2,
                        3_i32 => 3,
                        1_i32 => 4,
                        2_i32 => 5,
                        4_i32 => 6,
                        _ => 7,
                    };
                }
                1 => {
                    return result;
                }
                2 => {
                    bb = 3;
                }
                3 => {
                    result = 0_i32;
                    self.w8(a3, 0_u8);
                    bb = 1;
                }
                4 => {
                    self.w8(a3, 0_u8);
                    result = 0_i32;
                    bb = 1;
                }
                5 => {
                    result = 0_i32;
                    self.w8(a3, 0_u8);
                    bb = 1;
                }
                6 => {
                    self.w8(a3, 1_u8);
                    bb = 11;
                }
                7 => {
                    bb = 11;
                }
                8 => {
                    bb = 4;
                }
                9 => {
                    bb = 5;
                }
                10 => {
                    bb = 6;
                }
                11 => {
                    result = 0_i32;
                    bb = 1;
                }
                12 => {
                    bb = 7;
                }
                13 => {
                    bb = 1;
                }
                14 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10006BF0` (135 bytes).
    pub(crate) fn f_10006bf0(
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
        v4 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        self.w8(a3, 0_u8);
        v5 = 32_i32.wrapping_mul(a2).wrapping_add(v4);
        v6 = (self.r32((v5.wrapping_add(7600_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 6_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7604_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 6_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10006C80` (135 bytes).
    pub(crate) fn f_10006c80(
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
        v4 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        self.w8(a3, 0_u8);
        v5 = 32_i32.wrapping_mul(a2).wrapping_add(v4);
        v6 = (self.r32((v5.wrapping_add(7600_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 8_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7604_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 8_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10006D10` (281 bytes).
    pub(crate) fn f_10006d10(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: bool = false;
        let mut v9: bool = false;
        let mut v10: bool = false;
        let mut v11: bool = false;
        v9 = (0_i32 != 0);
        v4 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        v5 = (self.r32(
            (32_i32
                .wrapping_mul(a2)
                .wrapping_add(v4)
                .wrapping_add(7600_i32) as u32),
        ) as i32);
        if (v5 != (1_i32).wrapping_neg()) {
            v9 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v5))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 7_i32);
        }
        v6 = (self.r32(
            (32_i32
                .wrapping_mul(a2)
                .wrapping_add(v4)
                .wrapping_add(7604_i32) as u32),
        ) as i32);
        v10 = (0_i32 != 0);
        if (v6 != (1_i32).wrapping_neg()) {
            v10 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 7_i32);
        }
        v11 = (0_i32 != 0);
        if (v5 != (1_i32).wrapping_neg()) {
            v11 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v5))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 8_i32);
        }
        v7 = (0_i32 != 0);
        if (v6 != (1_i32).wrapping_neg()) {
            v7 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 8_i32);
        }
        self.w8(a3, 0_u8);
        self.w8(a4, 0_u8);
        if (v9 || v11) {
            self.w8(a3, 1_u8);
        }
        if (v10 || v7) {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10006E30` (135 bytes).
    pub(crate) fn f_10006e30(
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
        v4 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        self.w8(a3, 0_u8);
        v5 = 32_i32.wrapping_mul(a2).wrapping_add(v4);
        v6 = (self.r32((v5.wrapping_add(7600_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 9_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7604_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 9_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10006EC0` (135 bytes).
    pub(crate) fn f_10006ec0(
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
        v4 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        self.w8(a3, 0_u8);
        v5 = 32_i32.wrapping_mul(a2).wrapping_add(v4);
        v6 = (self.r32((v5.wrapping_add(7600_i32) as u32)) as i32);
        if ((v6 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 11_i32))
        {
            self.w8(a3, 1_u8);
        }
        self.w8(a4, 0_u8);
        v7 = (self.r32((v5.wrapping_add(7604_i32) as u32)) as i32);
        if ((v7 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v7))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 11_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10006F50` (281 bytes).
    pub(crate) fn f_10006f50(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: u32,
        mut a4: u32,
    ) -> i32 {
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: bool = false;
        let mut v9: bool = false;
        let mut v10: bool = false;
        let mut v11: bool = false;
        v9 = (0_i32 != 0);
        v4 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        v5 = (self.r32(
            (32_i32
                .wrapping_mul(a2)
                .wrapping_add(v4)
                .wrapping_add(7600_i32) as u32),
        ) as i32);
        if (v5 != (1_i32).wrapping_neg()) {
            v9 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v5))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 12_i32);
        }
        v6 = (self.r32(
            (32_i32
                .wrapping_mul(a2)
                .wrapping_add(v4)
                .wrapping_add(7604_i32) as u32),
        ) as i32);
        v10 = (0_i32 != 0);
        if (v6 != (1_i32).wrapping_neg()) {
            v10 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 12_i32);
        }
        v11 = (0_i32 != 0);
        if (v5 != (1_i32).wrapping_neg()) {
            v11 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v5))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 13_i32);
        }
        v7 = (0_i32 != 0);
        if (v6 != (1_i32).wrapping_neg()) {
            v7 = ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v4.wrapping_add(4_i32.wrapping_mul(v6))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 13_i32);
        }
        self.w8(a3, 0_u8);
        self.w8(a4, 0_u8);
        if (v9 || v11) {
            self.w8(a3, 1_u8);
        }
        if (v10 || v7) {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10007070` (75 bytes).
    pub(crate) fn f_10007070(&mut self, mut this: u32, mut a2: i32, mut a3: u32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        v3 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(16_i32) as u32))
            as i32);
        self.w8(a3, 0_u8);
        v4 = (self.r32(
            (32_i32
                .wrapping_mul(a2)
                .wrapping_add(v3)
                .wrapping_add(7604_i32) as u32),
        ) as i32);
        if ((v4 != (1_i32).wrapping_neg())
            && ((self.r32(
                ((self.r32(((self.r32(this.wrapping_add(8)) as i32).wrapping_add(12_i32) as u32))
                    as i32)
                    .wrapping_add(
                        140_i32.wrapping_mul(
                            (self.r32(
                                (v3.wrapping_add(4_i32.wrapping_mul(v4))
                                    .wrapping_add(1600_i32)
                                    as u32),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(4_i32) as u32),
            ) as i32)
                == 14_i32))
        {
            self.w8(a3, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_100070C0` (95 bytes).
    pub(crate) fn f_100070c0(
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
                == 1714_i32))
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
                == 1714_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10007120` (95 bytes).
    pub(crate) fn f_10007120(
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
                == 1715_i32))
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
                == 1715_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_10007180` (95 bytes).
    pub(crate) fn f_10007180(
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
                == 1716_i32))
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
                == 1716_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }

    /// `sub_100071E0` (95 bytes).
    pub(crate) fn f_100071e0(
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
                == 1717_i32))
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
                == 1717_i32))
        {
            self.w8(a4, 1_u8);
        }
        return 0_i32;
    }
}
