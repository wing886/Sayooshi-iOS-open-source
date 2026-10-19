//! Functions 0x10016130..=0x1001FA00, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_10016130` (70 bytes).
    pub(crate) fn f_10016130(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        v3 = (self.r32((v2.wrapping_add(1404_i32) as u32)) as i32);
        if (v3 != 0) {
            if (v3 == 1_i32) {
                self.w32((v2.wrapping_add(1404_i32) as u32), 99_u32);
            }
            return 0_i32;
        } else {
            let _ = self.f_10016180(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10016180` (608 bytes).
    pub(crate) fn f_10016180(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(1648);
        let r = self.f_10016180_body(fp, this);
        self.leave(1648);
        r
    }

    fn f_10016180_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: u32 = 0;
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
        let mut v14: u32 = 0;
        let mut v15: u32 = 0;
        let mut v16: i32 = 0;
        let mut v17: bool = false;
        let mut v18: i32 = 0;
        let mut result: i32 = 0;
        v1 = ((self.r32(self.r32(this)) as i32) as u32);
        v2 = ((self.r32(v1.wrapping_add(28)) as i32) as u32);
        v3 = (self.r32(v1.wrapping_add(20)) as i32);
        v4 = (self.r32(v1.wrapping_add(12)) as i32);
        self.w32(fp.wrapping_add(40), v2);
        v5 = (self.r32(
            (v4.wrapping_add(156_i32.wrapping_mul((self.r32(v2.wrapping_add(1388)) as i32)))
                .wrapping_add(3724_i32) as u32),
        ) as i32);
        self.w32(fp.wrapping_add(44), (v5 as u32));
        v6 = (self.r32(
            (v4.wrapping_add(156_i32.wrapping_mul((self.r32(v2.wrapping_add(1388)) as i32)))
                .wrapping_add(3728_i32) as u32),
        ) as i32);
        self.w32(fp.wrapping_add(36), (v6 as u32));
        if (v5 > 9_i32) {
            v7 = 19_i32.wrapping_sub(v5);
        } else {
            v7 = v5;
        }
        if (v6 > 9_i32) {
            v8 = 19_i32.wrapping_sub(v6);
        } else {
            v8 = v5;
        }
        if (v7 > v8) {
            if (v6 > 9_i32) {
                self.w32(fp.wrapping_add(4), 2_u32);
                self.w32(fp.wrapping_add(8), 0_u32);
                self.w32(fp.wrapping_add(0), 19_u32);
            } else {
                self.w32(fp.wrapping_add(4), 0_u32);
                self.w32(fp.wrapping_add(8), 0_u32);
                self.w32(fp.wrapping_add(0), 0_u32);
            }
        } else {
            if (v5 > 9_i32) {
                self.w32(fp.wrapping_add(4), 1_u32);
                self.w32(fp.wrapping_add(8), 19_u32);
            } else {
                self.w32(fp.wrapping_add(4), 3_u32);
                self.w32(fp.wrapping_add(8), 0_u32);
            }
            self.w32(fp.wrapping_add(0), 0_u32);
        }
        v9 = (self.r32((v3.wrapping_add(6440_i32) as u32)) as i32);
        self.w32(fp.wrapping_add(16), 0_u32);
        if (v9 > 0_i32) {
            let _ = self.memcpy(
                fp.wrapping_add(48),
                (v3.wrapping_add(4840_i32) as u32),
                (4_i32.wrapping_mul(v9) as u32),
            );
            self.w32(fp.wrapping_add(16), (v9 as u32));
        }
        self.w32(fp.wrapping_add(32), 9999_u32);
        self.w32(fp.wrapping_add(20), 0_u32);
        self.w32(fp.wrapping_add(28), 0_u32);
        self.w32(fp.wrapping_add(12), 0_u32);
        if ((self.r32(fp.wrapping_add(16)) as i32) > 0_i32) {
            self.w32(fp.wrapping_add(24), fp.wrapping_add(48));
            'l1: loop {
                v10 = crem_i32((self.r32(self.r32(fp.wrapping_add(24))) as i32), 20_i32);
                v11 = cdiv_i32((self.r32(self.r32(fp.wrapping_add(24))) as i32), 20_i32);
                v12 = cdiv_i32(
                    (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                    (20_i32).wrapping_neg(),
                );
                v13 = v12
                    .wrapping_add((self.r32(fp.wrapping_add(36)) as i32))
                    .wrapping_abs()
                    .wrapping_add(
                        (self.r32(fp.wrapping_add(44)) as i32)
                            .wrapping_sub(v10)
                            .wrapping_abs(),
                    );
                v14 = ((self.r32(fp.wrapping_add(8)) as i32)
                    .wrapping_sub(v10)
                    .wrapping_abs() as u32);
                v15 = (v12
                    .wrapping_add((self.r32(fp.wrapping_add(0)) as i32))
                    .wrapping_abs() as u32);
                if (((self.r32(fp.wrapping_add(4)) as i32) == 3_i32)
                    || ((self.r32(fp.wrapping_add(4)) as i32) == 1_i32))
                {
                    v15 = 0_u32;
                } else {
                    v14 = 0_u32;
                }
                v16 = (v14.wrapping_sub(v15) as i32).wrapping_abs();
                if ((((v10 != 0) && (v10 != 19_i32)) && (v11 != 0)) && (v11 != 19_i32)) {
                    v13 = ((v13 as u32).wrapping_add(6_u32.wrapping_mul(v15.wrapping_add(v14)))
                        as i32);
                }
                if (v13 >= (self.r32(fp.wrapping_add(32)) as i32)) {
                    if ((v13 == (self.r32(fp.wrapping_add(32)) as i32))
                        && (v16 < (self.r32(fp.wrapping_add(20)) as i32)))
                    {
                        self.w32(fp.wrapping_add(20), (v16 as u32));
                        self.w32(
                            fp.wrapping_add(28),
                            ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                        );
                    }
                } else {
                    self.w32(fp.wrapping_add(20), (v16 as u32));
                    self.w32(fp.wrapping_add(32), (v13 as u32));
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                }
                v17 = ({
                    let t1 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                    self.w32(fp.wrapping_add(12), (t1 as u32));
                    t1
                } < (self.r32(fp.wrapping_add(16)) as i32));
                self.w32(
                    fp.wrapping_add(24),
                    self.r32(fp.wrapping_add(24)).wrapping_add(4),
                );
                if !v17 {
                    break 'l1;
                }
            }
            v2 = self.r32(fp.wrapping_add(40));
        }
        self.w32(v2.wrapping_add(1540), 2_u32);
        v18 = (self.r32(
            fp.wrapping_add(48)
                .wrapping_add(((self.r32(fp.wrapping_add(28)) as i32) as u32).wrapping_mul(4)),
        ) as i32);
        self.w32(v2.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
        self.w32(v2.wrapping_add(1552), (v18 as u32));
        self.w32(v2.wrapping_add(1544), (crem_i32(v18, 20_i32) as u32));
        result = 0_i32;
        self.w32(v2.wrapping_add(1548), (cdiv_i32(v18, 20_i32) as u32));
        return result;
    }

    /// `sub_100163E0` (27 bytes).
    pub(crate) fn f_100163e0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        v1 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        self.w32((v1.wrapping_add(1404_i32) as u32), 99_u32);
        return 0_i32;
    }

    /// `sub_10016400` (70 bytes).
    pub(crate) fn f_10016400(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        v3 = (self.r32((v2.wrapping_add(1404_i32) as u32)) as i32);
        if (v3 != 0) {
            if (v3 == 1_i32) {
                self.w32((v2.wrapping_add(1404_i32) as u32), 99_u32);
            }
            return 0_i32;
        } else {
            let _ = self.f_10016450(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10016450` (87 bytes).
    pub(crate) fn f_10016450(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10016450_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10016450_body(&mut self, fp: u32, mut this: u32) -> i32 {
        's1: {
            'b1_2: {
                'b1_1: {
                    'b1_0: {
                        match (self.r32(
                            ((self.r32(
                                ((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32),
                            ) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            ((self.r32(
                                                ((self.r32(self.r32(this)) as i32)
                                                    .wrapping_add(28_i32)
                                                    as u32),
                                            ) as i32)
                                                .wrapping_add(1388_i32)
                                                as u32),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3740_i32) as u32),
                        ) as i32)
                        {
                            51 => break 'b1_0,
                            52 => break 'b1_1,
                            53 => break 'b1_2,
                            _ => break 's1,
                        }
                    }
                    let _ = self.f_10018380(this, fp.wrapping_add(0));
                    break 's1;
                }
                let _ = self.f_100183c0(this, fp.wrapping_add(0));
                return 0_i32;
            }
            let _ = self.f_10018400(this, fp.wrapping_add(0));
            return 0_i32;
        }
        return 0_i32;
    }

    /// `sub_100164B0` (83 bytes).
    pub(crate) fn f_100164b0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100164b0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100164b0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        v1 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) != 0) || {
            let _ = {
                let t1: i32 = 1_i32;
                self.w32((v1.wrapping_add(1404_i32) as u32), (t1 as u32));
                t1
            };
            let _ = self.f_10016510(this, fp.wrapping_add(0));
            (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
        }) {
            if ((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) == 1_i32) {
                self.w32((v1.wrapping_add(1404_i32) as u32), 99_u32);
            }
        }
        return 0_i32;
    }

    /// `sub_10016510` (182 bytes).
    pub(crate) fn f_10016510(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut result: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        v5 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32)
            .wrapping_add(156_i32.wrapping_mul(v4))
            .wrapping_add(3600_i32);
        if ((self.r32((v5.wrapping_add(140_i32) as u32)) as i32) != 0) {
            v6 = (self.r32((v5.wrapping_add(144_i32) as u32)) as i32);
            if (v6 == (1_i32).wrapping_neg()) {
                self.w8(a2, 0_u8);
                return 0_i32;
            } else {
                v7 = v6.wrapping_sub(1_i32);
                self.w32((v5.wrapping_add(144_i32) as u32), (v7 as u32));
                if (v7 > 0_i32) {
                    result = 0_i32;
                    self.w8(a2, 0_u8);
                } else {
                    v8 = (self.r32(v3.wrapping_add(2280)) as i32);
                    self.w32(v3.wrapping_add(2188), 1_u32);
                    self.w32(v3.wrapping_add(2192), 0_u32);
                    self.w32(v3.wrapping_add(2196), (v4.wrapping_add(5_i32) as u32));
                    self.w32(
                        v3.wrapping_add((v8.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        ((self.r32((v5.wrapping_add(140_i32) as u32)) as i32) as u32),
                    );
                    self.w32(
                        v3.wrapping_add(2280),
                        ((self.r32(v3.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32((v5.wrapping_add(140_i32) as u32), 0_u32);
                    self.w8(a2, 1_u8);
                    return 0_i32;
                }
            }
        } else {
            result = 0_i32;
            self.w8(a2, 0_u8);
        }
        return result;
    }

    /// `sub_100165D0` (842 bytes).
    pub(crate) fn f_100165d0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(576);
        let r = self.f_100165d0_body(fp, this);
        self.leave(576);
        r
    }

    fn f_100165d0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        v2 = self.r32(self.r32(this));
        v3 = ((self.r32(v2.wrapping_add(12)) as i32) as u32);
        self.w32(
            fp.wrapping_add(2),
            ((self.r32(v2.wrapping_add(28)) as i32) as u32),
        );
        if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
        {
            return 0_i32;
        }
        let _ = self.f_10002380(self.r32(this.wrapping_add(4)), 0_i32, 4_i32);
        self.w32(v3.wrapping_add(2864), 1_u32);
        self.w32(
            v3.wrapping_add(2872),
            ((self.r32(v3.wrapping_add(8)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(2932),
            ((self.r32(v3.wrapping_add(68)) as i32) as u32),
        );
        let _ = self.memset(v3.wrapping_add(2936), 255_i32 as u8, 60_u32);
        self.w32(
            v3.wrapping_add(2996),
            (cdiv_i32(
                130_i32.wrapping_mul((self.r32(v3.wrapping_add(132)) as i32)),
                100_i32,
            ) as u32),
        );
        self.w32(
            v3.wrapping_add(3000),
            (cdiv_i32(
                130_i32.wrapping_mul((self.r32(v3.wrapping_add(136)) as i32)),
                100_i32,
            ) as u32),
        );
        self.w32(
            v3.wrapping_add(3004),
            (cdiv_i32(
                125_i32.wrapping_mul((self.r32(v3.wrapping_add(140)) as i32)),
                100_i32,
            ) as u32),
        );
        self.w32(
            v3.wrapping_add(3008),
            ((self.r32(v3.wrapping_add(144)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(3012),
            ((self.r32(v3.wrapping_add(148)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(3016),
            ((self.r32(v3.wrapping_add(152)) as i32) as u32),
        );
        self.w32(
            v3.wrapping_add(3020),
            ((self.r32(v3.wrapping_add(156)) as i32) as u32),
        );
        self.w32(v3.wrapping_add(3024), 0_u32);
        v4 = (self.r32(v3.wrapping_add(164)) as i32);
        self.w32(v3.wrapping_add(3032), 4_u32);
        self.w32(
            v3.wrapping_add(3028),
            (cdiv_i32(120_i32.wrapping_mul(v4), 100_i32) as u32),
        );
        let _ = self.f_10007360(this, 0_i32, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(
                v3.wrapping_add(2996),
                (cdiv_i32(
                    140_i32.wrapping_mul((self.r32(v3.wrapping_add(132)) as i32)),
                    100_i32,
                ) as u32),
            );
            self.w32(
                v3.wrapping_add(3000),
                (cdiv_i32(
                    140_i32.wrapping_mul((self.r32(v3.wrapping_add(136)) as i32)),
                    100_i32,
                ) as u32),
            );
        }
        let _ = self.f_100027d0(self.r32(this.wrapping_add(4)), 0_i32, 4_i32);
        let _ = self.f_10002970(self.r32(this.wrapping_add(4)), 0_i32, 4_i32, 2_i32, 2_i32);
        let _ = self.f_10002b10(self.r32(this.wrapping_add(4)), 0_i32, 4_i32);
        self.w32(v3.wrapping_add(3256), 500_u32);
        let _ = self.memset(fp.wrapping_add(174), 0_i32 as u8, 400_u32);
        v5 = v3.wrapping_add(384);
        v6 = 5_i32;
        'l1: loop {
            if (((self.r32(v5.wrapping_sub(384)) as i32) == 1_i32)
                && ((self.r32(v5.wrapping_sub(12)) as i32) != 99_i32))
            {
                self.w8(
                    fp.wrapping_add(174).wrapping_add(
                        (20_i32
                            .wrapping_mul((self.r32(v5) as i32))
                            .wrapping_add((self.r32(v5.wrapping_sub(4)) as i32))
                            as u32),
                    ),
                    1_u8,
                );
            }
            v5 = v5.wrapping_add(716);
            v6 = v6.wrapping_sub(1);
            if !(v6 != 0) {
                break 'l1;
            }
        }
        v7 = v3.wrapping_add(3600);
        v8 = 20_i32;
        'l2: loop {
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
            {
                self.w8(
                    fp.wrapping_add(174).wrapping_add(
                        (20_i32
                            .wrapping_mul((self.r32(v7.wrapping_add(128)) as i32))
                            .wrapping_add((self.r32(v7.wrapping_add(124)) as i32))
                            as u32),
                    ),
                    1_u8,
                );
            }
            v7 = v7.wrapping_add(156);
            v8 = v8.wrapping_sub(1);
            if !(v8 != 0) {
                break 'l2;
            }
        }
        v9 = 0_i32;
        v10 = 0_i32;
        v11 = fp.wrapping_add(14);
        v12 = 0x1007F39C_u32;
        'l3: loop {
            if (!(self.r8(fp
                .wrapping_add(174)
                .wrapping_add(((self.r32(v12) as i32) as u32)))
                != 0))
            {
                self.w32(v11, (v10 as u32));
                v9 = v9.wrapping_add(1);
                v11 = v11.wrapping_add(4);
            }
            v12 = v12.wrapping_add(4);
            v10 = v10.wrapping_add(1);
            if !((v12 as i32) < 0x1007F43C_i32) {
                break 'l3;
            }
        }
        v13 = ({
            let a0 = fp
                .wrapping_add(14)
                .wrapping_add((crem_i32(self.rand(), v9) as u32).wrapping_mul(4));
            self.r32(a0)
        } as i32);
        v14 = (self.r32(0x1007F39C_u32.wrapping_add((v13 as u32).wrapping_mul(4))) as i32);
        v15 = (self.r32(0x1007F43C_u32.wrapping_add((v13 as u32).wrapping_mul(4))) as i32);
        v16 = v14;
        v17 = v14;
        self.w32(fp.wrapping_add(10), (crem_i32(v14, 20_i32) as u32));
        v18 = 0_i32;
        self.w32(fp.wrapping_add(6), (cdiv_i32(v14, 20_i32) as u32));
        v19 = v14;
        'l4: loop {
            's5: {
                'b5_4: {
                    'b5_3: {
                        'b5_2: {
                            'b5_1: {
                                'b5_0: {
                                    match v15 {
                                        0_i32 => break 'b5_0,
                                        1_i32 => break 'b5_1,
                                        2_i32 => break 'b5_2,
                                        3_i32 => break 'b5_3,
                                        _ => break 'b5_4,
                                    }
                                }
                                v13 = v17;
                                break 's5;
                            }
                            v13 = v14.wrapping_add(v18);
                            break 's5;
                        }
                        v13 = v16;
                        break 's5;
                    }
                    v13 = v19;
                    break 's5;
                }
                break 's5;
            }
            if ((self.r8(fp.wrapping_add(174).wrapping_add((v13 as u32))) as i32) == 1_i32) {
                break 'l4;
            }
            v18 = v18.wrapping_add(1);
            v17 = v17.wrapping_sub(20_i32);
            v16 = v16.wrapping_add(20_i32);
            v19 = v19.wrapping_sub(1);
            if !(v18 < 6_i32) {
                break 'l4;
            }
        }
        v20 = self.r32(fp.wrapping_add(2));
        self.w32(
            self.r32(fp.wrapping_add(2)).wrapping_add(2284),
            ((self.r32(fp.wrapping_add(10)) as i32) as u32),
        );
        self.w32(
            v20.wrapping_add(2288),
            ((self.r32(fp.wrapping_add(6)) as i32) as u32),
        );
        self.w32(v20.wrapping_add(2296), (v15 as u32));
        self.w32(v20.wrapping_add(2300), (v18.wrapping_sub(1_i32) as u32));
        self.w32(v20.wrapping_add(2292), (v14 as u32));
        return 0_i32;
    }

    /// `sub_10016930` (8 bytes).
    pub(crate) fn f_10016930(&mut self, mut this: u32) -> i32 {
        let _ = self.f_100165d0(this);
        return 0_i32;
    }

    /// `sub_10016940` (360 bytes).
    pub(crate) fn f_10016940(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(128);
        let r = self.f_10016940_body(fp, this);
        self.leave(128);
        r
    }

    fn f_10016940_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut i: i32 = 0;
        let mut v17: u32 = 0;
        let mut j: i32 = 0;
        let mut v19: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v1 = (self.r32(self.r32(this)) as i32);
                    v2 = 0_i32;
                    v3 = fp.wrapping_add(40);
                    v4 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
                    v5 = self.r32((v1.wrapping_add(28_i32) as u32));
                    self.w32(fp.wrapping_add(16), v5);
                    v6 = (v4.wrapping_add(3600_i32) as u32);
                    v7 = 0_i32;
                    v8 = v6;
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v8) as i32) == 1_i32)
                        && ((self.r32(v8.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (v7 < 20_i32) { 1 } else { 3 };
                }
                3 => {
                    bb = if (!(v2 != 0)) { 8 } else { 9 };
                }
                4 => {
                    v9 = (self.r32(v8.wrapping_add(4)) as i32);
                    bb = if ((v9 != 5_i32) && (v9 != 6_i32)) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v8 = v8.wrapping_add(156);
                    v7 = v7.wrapping_add(1);
                    bb = 2;
                }
                6 => {
                    self.w32(v3, (v7 as u32));
                    v2 = v2.wrapping_add(1);
                    v3 = v3.wrapping_add(4);
                    bb = 7;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v10 = v6;
                    v11 = 0_i32;
                    bb = 10;
                }
                9 => {
                    bb = 15;
                }
                10 => {
                    bb = if (((self.r32(v10) as i32) != 1_i32)
                        || ((self.r32(v10.wrapping_add(116)) as i32) == 99_i32))
                    {
                        11
                    } else {
                        12
                    };
                }
                11 => {
                    v10 = v10.wrapping_add(156);
                    bb = if ({
                        let t1 = v11.wrapping_add(1);
                        v11 = t1;
                        t1
                    } >= 20_i32)
                    {
                        13
                    } else {
                        14
                    };
                }
                12 => {
                    self.w32(fp.wrapping_add(40), (v11 as u32));
                    v2 = 1_i32;
                    bb = 9;
                }
                13 => {
                    bb = 15;
                }
                14 => {
                    bb = 10;
                }
                15 => {
                    let _ = self.memset(fp.wrapping_add(20), 0_i32 as u8, 20_u32);
                    {
                        let a0 = fp.wrapping_add(20).wrapping_add(
                            (({
                                let a0 = fp.wrapping_add(40).wrapping_add(
                                    (crem_i32(self.rand(), v2) as u32).wrapping_mul(4),
                                );
                                self.r32(a0)
                            } as i32) as u32),
                        );
                        let a1 = 1_u8;
                        self.w8(a0, a1)
                    };
                    bb = if (v2 > 0_i32) { 17 } else { 18 };
                }
                16 => {
                    bb = 14;
                }
                17 => {
                    v12 = fp.wrapping_add(40);
                    v13 = v2;
                    bb = 19;
                }
                18 => {
                    v14 = cdiv_i32(v2, 2_i32);
                    bb = if (!(cdiv_i32(v2, 2_i32) != 0)) {
                        24
                    } else {
                        25
                    };
                }
                19 => {
                    bb = if (crem_i32(self.rand(), 100_i32) < 30_i32) {
                        22
                    } else {
                        23
                    };
                }
                20 => {
                    bb = if (v13 != 0) { 19 } else { 21 };
                }
                21 => {
                    v5 = self.r32(fp.wrapping_add(16));
                    bb = 18;
                }
                22 => {
                    self.w8(
                        fp.wrapping_add(20)
                            .wrapping_add(((self.r32(v12) as i32) as u32)),
                        1_u8,
                    );
                    bb = 23;
                }
                23 => {
                    v12 = v12.wrapping_add(4);
                    v13 = v13.wrapping_sub(1);
                    bb = 20;
                }
                24 => {
                    v14 = 1_i32;
                    bb = 25;
                }
                25 => {
                    v15 = 0_i32;
                    i = 0_i32;
                    bb = 26;
                }
                26 => {
                    bb = if (i < 20_i32) { 27 } else { 29 };
                }
                27 => {
                    bb = if ((self.r8(fp.wrapping_add(20).wrapping_add((i as u32))) as i32)
                        == 1_i32)
                    {
                        30
                    } else {
                        31
                    };
                }
                28 => {
                    i = i.wrapping_add(1);
                    bb = 26;
                }
                29 => {
                    bb = if (((self.r32(v5.wrapping_add(1472)) as i32) == 13_i32)
                        && ((self.r32(v5.wrapping_add(1476)) as i32) == 3_i32))
                    {
                        34
                    } else {
                        35
                    };
                }
                30 => {
                    v15 = v15.wrapping_add(1);
                    bb = 31;
                }
                31 => {
                    bb = if (v14 < v15) { 32 } else { 33 };
                }
                32 => {
                    self.w8(fp.wrapping_add(20).wrapping_add((i as u32)), 0_u8);
                    bb = 33;
                }
                33 => {
                    bb = 28;
                }
                34 => {
                    v17 = v6;
                    j = 0_i32;
                    bb = 36;
                }
                35 => {
                    v19 = 5_i32;
                    bb = 42;
                }
                36 => {
                    bb = if (j < 20_i32) { 37 } else { 39 };
                }
                37 => {
                    bb = if (((self.r32(v17) as i32) == 1_i32)
                        && ((self.r32(v17.wrapping_add(116)) as i32) != 99_i32))
                    {
                        40
                    } else {
                        41
                    };
                }
                38 => {
                    j = j.wrapping_add(1);
                    bb = 36;
                }
                39 => {
                    bb = 35;
                }
                40 => {
                    self.w8(fp.wrapping_add(20).wrapping_add((j as u32)), 1_u8);
                    bb = 41;
                }
                41 => {
                    v17 = v17.wrapping_add(156);
                    bb = 38;
                }
                42 => {
                    bb = if ((self.r8(fp
                        .wrapping_add(0)
                        .wrapping_add((v19 as u32))
                        .wrapping_add(15)) as i32)
                        == 1_i32)
                    {
                        45
                    } else {
                        46
                    };
                }
                43 => {
                    bb = if (v19.wrapping_sub(5_i32) < 20_i32) {
                        42
                    } else {
                        44
                    };
                }
                44 => {
                    return 0_i32;
                }
                45 => {
                    {
                        let a0 = v5.wrapping_add(
                            ({
                                let t2 = (self.r32(v5.wrapping_add(2072)) as i32);
                                self.w32(v5.wrapping_add(2072), (t2.wrapping_add(1) as u32));
                                t2
                            }
                            .wrapping_add(393_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = (v19 as u32);
                        self.w32(a0, a1)
                    };
                    bb = 46;
                }
                46 => {
                    v19 = v19.wrapping_add(1);
                    bb = 43;
                }
                47 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10016AB0` (494 bytes).
    pub(crate) fn f_10016ab0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(3264);
        let r = self.f_10016ab0_body(fp, this);
        self.leave(3264);
        r
    }

    fn f_10016ab0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: bool = false;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: u32 = 0;
        let mut v18: u32 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: u32 = 0;
        let mut v24: i32 = 0;
        let mut result: i32 = 0;
        v1 = self.r32(((self.r32(this) as i32) as u32));
        self.w32(fp.wrapping_add(8), 20_u32);
        v2 = (self.r32(v1.wrapping_add(12)) as i32);
        self.w32(
            fp.wrapping_add(28),
            ((self.r32(v1.wrapping_add(28)) as i32) as u32),
        );
        v3 = (self.r32(v1.wrapping_add(20)) as i32).wrapping_add(3200_i32);
        self.w32(fp.wrapping_add(16), (v3 as u32));
        v4 = (v2.wrapping_add(3600_i32) as u32);
        self.w32(
            fp.wrapping_add(20),
            ((self.r32(
                (v2.wrapping_add(
                    716_i32.wrapping_mul(
                        (self.r32(
                            ((self.r32(fp.wrapping_add(28)) as i32).wrapping_add(1388_i32) as u32),
                        ) as i32),
                    ),
                )
                .wrapping_add(380_i32) as u32),
            ) as i32)
                .wrapping_add(
                    20_i32.wrapping_mul(
                        (self.r32(
                            (v2.wrapping_add(716_i32.wrapping_mul(
                                (self.r32(
                                    ((self.r32(fp.wrapping_add(28)) as i32).wrapping_add(1388_i32)
                                        as u32),
                                ) as i32),
                            ))
                            .wrapping_add(384_i32) as u32),
                        ) as i32),
                    ),
                ) as u32),
        );
        v5 = ((self.r32(this.wrapping_add(236)) as i32) as u32);
        self.w32(fp.wrapping_add(0), (v2.wrapping_add(3600_i32) as u32));
        v6 = (self.r32(v5) as i32);
        v5 = v5.wrapping_add(16);
        self.w32(
            fp.wrapping_add(24),
            ((self.r32(v5.wrapping_sub(12)) as i32) as u32),
        );
        self.w32(
            fp.wrapping_add(12),
            ((self.r32(v5.wrapping_sub(8)) as i32) as u32),
        );
        self.w32(
            fp.wrapping_add(32),
            ((self.r32(v5.wrapping_sub(4)) as i32) as u32),
        );
        self.w32(fp.wrapping_add(36), v5);
        let _ = self.memset(fp.wrapping_add(64), 0_i32 as u8, 1600_u32);
        'l1: loop {
            if (((self.r32(v4) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(116)) as i32) != 99_i32))
            {
                v7 = self.r32(fp.wrapping_add(36));
                v8 = v6.wrapping_add((self.r32(v4.wrapping_add(124)) as i32));
                v9 = (self.r32(fp.wrapping_add(24)) as i32)
                    .wrapping_add((self.r32(v4.wrapping_add(128)) as i32));
                v10 = v8.wrapping_add((self.r32(fp.wrapping_add(12)) as i32));
                if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(32)) as i32))) {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(32)) as i32) as u32),
                    );
                    'l2: loop {
                        v12 = (v8 as u32);
                        if (v8 < v10) {
                            'l3: loop {
                                if ((((self.r32(v7) as i32) != 0) && (v11 < 400_u32))
                                    && (v12 < 20_u32))
                                {
                                    self.w32(
                                        fp.wrapping_add(64).wrapping_add(
                                            ((v11.wrapping_add(v12) as i32) as u32).wrapping_mul(4),
                                        ),
                                        ((self.r32(fp.wrapping_add(64).wrapping_add(
                                            ((v11.wrapping_add(v12) as i32) as u32).wrapping_mul(4),
                                        )) as i32)
                                            .wrapping_add((self.r32(v7) as i32))
                                            as u32),
                                    );
                                }
                                v7 = v7.wrapping_add(4);
                                v12 = v12.wrapping_add(1);
                                if !((v12 as i32) < v10) {
                                    break 'l3;
                                }
                            }
                            v4 = self.r32(fp.wrapping_add(0));
                        }
                        v11 = v11.wrapping_add(20_u32);
                        self.w32(
                            fp.wrapping_add(4),
                            ((self.r32(fp.wrapping_add(4)) as i32).wrapping_sub(1) as u32),
                        );
                        if !((self.r32(fp.wrapping_add(4)) as i32) != 0) {
                            break 'l2;
                        }
                    }
                }
                v3 = (self.r32(fp.wrapping_add(16)) as i32);
            }
            v4 = v4.wrapping_add(156);
            v13 = ((self.r32(fp.wrapping_add(8)) as i32) == 1_i32);
            self.w32(fp.wrapping_add(0), v4);
            self.w32(
                fp.wrapping_add(8),
                ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
            );
            if !(!v13) {
                break 'l1;
            }
        }
        v14 = 0_i32;
        v15 = 0_i32;
        v16 = fp.wrapping_add(1664);
        v17 = fp.wrapping_add(64);
        'l4: loop {
            v18 = ((self.r32(
                v17.wrapping_add((v3 as u32))
                    .wrapping_sub(((fp.wrapping_add(64) as i32) as u32)),
            ) as i32) as u32);
            if ((((v18 == ((1_i32).wrapping_neg() as u32)) || (v18 <= 2_u32)) || (v18 == 4_u32))
                && ((self.r32(v17) as i32) >= 3_i32))
            {
                self.w32(v16, (v15 as u32));
                v14 = v14.wrapping_add(1);
                v16 = v16.wrapping_add(4);
            }
            v15 = v15.wrapping_add(1);
            v17 = v17.wrapping_add(4);
            if !(v15 < 400_i32) {
                break 'l4;
            }
        }
        v19 = 0_i32;
        v20 = fp
            .wrapping_add(64)
            .wrapping_add((v14.wrapping_add(399_i32) as u32).wrapping_mul(4));
        'l5: loop {
            if (v19 == 3_i32) {
                self.w32(
                    fp.wrapping_add(44).wrapping_add(12),
                    ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                );
            } else {
                v21 = crem_i32(self.rand(), v14);
                v22 = (self.r32(v20) as i32);
                v14 = v14.wrapping_sub(1);
                v20 = v20.wrapping_sub(4);
                v23 = fp
                    .wrapping_add(1664)
                    .wrapping_add((v21 as u32).wrapping_mul(4));
                v24 = (self.r32(v23) as i32);
                self.w32(v23, (v22 as u32));
                self.w32(
                    fp.wrapping_add(44)
                        .wrapping_add((v19 as u32).wrapping_mul(4)),
                    (v24 as u32),
                );
            }
            v19 = v19.wrapping_add(1);
            if !(v19 < 5_i32) {
                break 'l5;
            }
        }
        result = 0_i32;
        let _ = self.memcpy(
            ((self.r32(fp.wrapping_add(28)) as i32).wrapping_add(2304_i32) as u32),
            fp.wrapping_add(44),
            20_u32,
        );
        return result;
    }

    /// `sub_10016CA0` (8 bytes).
    pub(crate) fn f_10016ca0(&mut self, mut this: u32) -> i32 {
        let _ = self.f_10016940(this);
        return 0_i32;
    }

    /// `sub_10016CB0` (51 bytes).
    pub(crate) fn f_10016cb0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        self.w8(a2, 1_u8);
        v3 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1504), 1_u32);
        self.w32(v2.wrapping_add(1508), (v3 as u32));
        self.w32(v2.wrapping_add(1512), 35_u32);
        return 0_i32;
    }

    /// `sub_10016CF0` (378 bytes).
    pub(crate) fn f_10016cf0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut i: u32 = 0;
        let mut v10: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(12_i32) as u32));
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        v5 = (self.r32(v4.wrapping_add(1388)) as i32);
        v6 = v3.wrapping_add((179_i32.wrapping_mul(v5) as u32).wrapping_mul(4));
        self.w8(a2, 0_u8);
        if (((self.r32(v6) as i32) == 1_i32) && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            i = v6.wrapping_add(592);
            'l1: loop {
                if !((self.r32(i) as i32) != 36_i32) {
                    break 'l1;
                }
                if ({
                    let t1 = v7.wrapping_add(1);
                    v7 = t1;
                    t1
                } >= 3_i32)
                {
                    return 0_i32;
                }
                i = i.wrapping_add(4);
            }
            self.w32(v4.wrapping_add(1516), 0_u32);
            if (((self.r32(v3) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1516), 1_u32);
            }
            self.w32(v4.wrapping_add(1520), 0_u32);
            if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1520), 1_u32);
            }
            self.w32(v4.wrapping_add(1524), 0_u32);
            if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1524), 1_u32);
            }
            self.w32(v4.wrapping_add(1528), 0_u32);
            if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1528), 1_u32);
            }
            self.w32(v4.wrapping_add(1532), 0_u32);
            if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1532), 1_u32);
            }
            if ((self.r32(v4.wrapping_add(1516)) as i32) == 1_i32) {
                v10 = (self.r32(v4.wrapping_add(2072)) as i32);
                self.w32(v4.wrapping_add(1504), 1_u32);
                self.w32(v4.wrapping_add(1508), (v5 as u32));
                self.w32(v4.wrapping_add(1512), 36_u32);
                self.w32(
                    v4.wrapping_add((v10.wrapping_add(393_i32) as u32).wrapping_mul(4)),
                    0_u32,
                );
                self.w32(
                    v4.wrapping_add(
                        ((self.r32(v4.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    1_u32,
                );
                self.w32(
                    v4.wrapping_add(
                        ((self.r32(v4.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    6_u32,
                );
                self.w32(
                    v4.wrapping_add(
                        ((self.r32(v4.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    (2_i32.wrapping_mul((self.r32(v6.wrapping_add(8)) as i32)) as u32),
                );
                {
                    let a0 = v4.wrapping_add(
                        ({
                            let t2 = (self.r32(v4.wrapping_add(2072)) as i32);
                            self.w32(v4.wrapping_add(2072), (t2.wrapping_add(1) as u32));
                            t2
                        }
                        .wrapping_add(493_i32) as u32)
                            .wrapping_mul(4),
                    );
                    let a1 = 0_u32;
                    self.w32(a0, a1)
                };
                self.w8(a2, 1_u8);
            }
        }
        return 0_i32;
    }

    /// `sub_10016E70` (378 bytes).
    pub(crate) fn f_10016e70(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut i: u32 = 0;
        let mut v10: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(12_i32) as u32));
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        v5 = (self.r32(v4.wrapping_add(1388)) as i32);
        v6 = v3.wrapping_add((179_i32.wrapping_mul(v5) as u32).wrapping_mul(4));
        self.w8(a2, 0_u8);
        if (((self.r32(v6) as i32) == 1_i32) && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            i = v6.wrapping_add(592);
            'l1: loop {
                if !((self.r32(i) as i32) != 37_i32) {
                    break 'l1;
                }
                if ({
                    let t1 = v7.wrapping_add(1);
                    v7 = t1;
                    t1
                } >= 3_i32)
                {
                    return 0_i32;
                }
                i = i.wrapping_add(4);
            }
            self.w32(v4.wrapping_add(1516), 0_u32);
            if (((self.r32(v3) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1516), 1_u32);
            }
            self.w32(v4.wrapping_add(1520), 0_u32);
            if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1520), 1_u32);
            }
            self.w32(v4.wrapping_add(1524), 0_u32);
            if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1524), 1_u32);
            }
            self.w32(v4.wrapping_add(1528), 0_u32);
            if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1528), 1_u32);
            }
            self.w32(v4.wrapping_add(1532), 0_u32);
            if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
            {
                self.w32(v4.wrapping_add(1532), 1_u32);
            }
            if ((self.r32(v4.wrapping_add(1516)) as i32) == 1_i32) {
                v10 = (self.r32(v4.wrapping_add(2072)) as i32);
                self.w32(v4.wrapping_add(1504), 1_u32);
                self.w32(v4.wrapping_add(1508), (v5 as u32));
                self.w32(v4.wrapping_add(1512), 37_u32);
                self.w32(
                    v4.wrapping_add((v10.wrapping_add(393_i32) as u32).wrapping_mul(4)),
                    0_u32,
                );
                self.w32(
                    v4.wrapping_add(
                        ((self.r32(v4.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    1_u32,
                );
                self.w32(
                    v4.wrapping_add(
                        ((self.r32(v4.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    6_u32,
                );
                self.w32(
                    v4.wrapping_add(
                        ((self.r32(v4.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    (2_i32.wrapping_mul((self.r32(v6.wrapping_add(8)) as i32)) as u32),
                );
                {
                    let a0 = v4.wrapping_add(
                        ({
                            let t2 = (self.r32(v4.wrapping_add(2072)) as i32);
                            self.w32(v4.wrapping_add(2072), (t2.wrapping_add(1) as u32));
                            t2
                        }
                        .wrapping_add(493_i32) as u32)
                            .wrapping_mul(4),
                    );
                    let a1 = 0_u32;
                    self.w32(a0, a1)
                };
                self.w8(a2, 1_u8);
            }
        }
        return 0_i32;
    }

    /// `sub_10016FF0` (691 bytes).
    pub(crate) fn f_10016ff0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut i: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self.r32((v2.wrapping_add(12_i32) as u32));
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w8(a2, 0_u8);
        v6 = v4.wrapping_add((179_i32.wrapping_mul(v5) as u32).wrapping_mul(4));
        if (((self.r32(v6) as i32) == 1_i32) && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            i = v6.wrapping_add(644);
            'l1: loop {
                if !((self.r32(i) as i32) != 44_i32) {
                    break 'l1;
                }
                if ({
                    let t1 = v7.wrapping_add(1);
                    v7 = t1;
                    t1
                } >= 3_i32)
                {
                    return 0_i32;
                }
                i = i.wrapping_add(4);
            }
            self.w32(v3.wrapping_add(1516), 0_u32);
            if ((((self.r32(v4) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                    (self.r32(v4.wrapping_add(220)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1516), 1_u32);
            }
            self.w32(v3.wrapping_add(1520), 0_u32);
            if ((((self.r32(v4.wrapping_add(716)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1088)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                    (self.r32(v4.wrapping_add(936)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1520), 1_u32);
            }
            self.w32(v3.wrapping_add(1524), 0_u32);
            if ((((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                    (self.r32(v4.wrapping_add(1652)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1524), 1_u32);
            }
            self.w32(v3.wrapping_add(1528), 0_u32);
            if ((((self.r32(v4.wrapping_add(2148)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(2520)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                    (self.r32(v4.wrapping_add(2368)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1528), 1_u32);
            }
            self.w32(v3.wrapping_add(1532), 0_u32);
            if ((((self.r32(v4.wrapping_add(2864)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(3236)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(3132)) as i32)),
                    (self.r32(v4.wrapping_add(3084)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1532), 1_u32);
            }
            if ((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32) {
                v10 = (self.r32(v3.wrapping_add(2072)) as i32);
                self.w32(v3.wrapping_add(1504), 1_u32);
                self.w32(v3.wrapping_add(1508), (v5 as u32));
                self.w32(v3.wrapping_add(1512), 44_u32);
                self.w32(
                    v3.wrapping_add((v10.wrapping_add(393_i32) as u32).wrapping_mul(4)),
                    (v5 as u32),
                );
                self.w32(
                    v3.wrapping_add(
                        ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    1_u32,
                );
                self.w32(
                    v3.wrapping_add(
                        ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    6_u32,
                );
                self.w32(
                    v3.wrapping_add(
                        ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    ((self.r32(v6.wrapping_add(220)) as i32) as u32),
                );
                self.w32(
                    v3.wrapping_add(
                        ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(493_i32) as u32)
                            .wrapping_mul(4),
                    ),
                    0_u32,
                );
                v16 = (self.r32(v3.wrapping_add(1512)) as i32);
                v15 = (self.r32(v3.wrapping_add(1508)) as i32);
                self.w32(
                    v3.wrapping_add(2072),
                    ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(1) as u32),
                );
                let _ = self.f_10007530(this, v15, v16);
                if (((self.r32(v6) as i32) == 1_i32)
                    && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
                {
                    v11 = 0_i32;
                    v12 = v6.wrapping_add(644);
                    'l2: loop {
                        if ((self.r32(v12) as i32) == 44_i32) {
                            break 'l2;
                        }
                        v11 = v11.wrapping_add(1);
                        v12 = v12.wrapping_add(4);
                        if !(v11 < 3_i32) {
                            break 'l2;
                        }
                    }
                    v13 = (self
                        .r32(v6.wrapping_add((v11.wrapping_add(164_i32) as u32).wrapping_mul(4)))
                        as i32)
                        .wrapping_sub(1_i32);
                    self.w32(
                        v6.wrapping_add((v11.wrapping_add(164_i32) as u32).wrapping_mul(4)),
                        (v13 as u32),
                    );
                    if (v13 <= 0_i32) {
                        self.w32(
                            v6.wrapping_add((v11.wrapping_add(161_i32) as u32).wrapping_mul(4)),
                            0_u32,
                        );
                        v14 = (self.r32(v3.wrapping_add(2280)) as i32);
                        self.w32(v3.wrapping_add(2188), 1_u32);
                        self.w32(v3.wrapping_add(2192), 0_u32);
                        self.w32(v3.wrapping_add(2196), (v5 as u32));
                        self.w32(
                            v3.wrapping_add((v14.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            44_u32,
                        );
                        self.w32(
                            v3.wrapping_add(2280),
                            ((self.r32(v3.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                    }
                }
                self.w8(a2, 1_u8);
            }
        }
        return 0_i32;
    }

    /// `sub_100172B0` (1235 bytes).
    pub(crate) fn f_100172b0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut i: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v15: u32 = 0;
        let mut v16: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self.r32((v2.wrapping_add(12_i32) as u32));
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w8(a2, 0_u8);
        v16 = v5;
        v6 = v4.wrapping_add((179_i32.wrapping_mul(v5) as u32).wrapping_mul(4));
        v15 = v6;
        if (((self.r32(v6) as i32) == 1_i32) && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            i = v6.wrapping_add(644);
            'l1: loop {
                if !((self.r32(i) as i32) != 45_i32) {
                    break 'l1;
                }
                if ({
                    let t1 = v7.wrapping_add(1);
                    v7 = t1;
                    t1
                } >= 3_i32)
                {
                    return 0_i32;
                }
                i = i.wrapping_add(4);
            }
            self.w32(v3.wrapping_add(1516), 0_u32);
            if ((((self.r32(v4) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                    (self.r32(v4.wrapping_add(220)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1516), 1_u32);
            }
            self.w32(v3.wrapping_add(1520), 0_u32);
            if ((((self.r32(v4.wrapping_add(716)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1088)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                    (self.r32(v4.wrapping_add(936)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1520), 1_u32);
            }
            self.w32(v3.wrapping_add(1524), 0_u32);
            if ((((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                    (self.r32(v4.wrapping_add(1652)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1524), 1_u32);
            }
            self.w32(v3.wrapping_add(1528), 0_u32);
            if ((((self.r32(v4.wrapping_add(2148)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(2520)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                    (self.r32(v4.wrapping_add(2368)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1528), 1_u32);
            }
            self.w32(v3.wrapping_add(1532), 0_u32);
            if ((((self.r32(v4.wrapping_add(2864)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(3236)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(3132)) as i32)),
                    (self.r32(v4.wrapping_add(3084)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1532), 1_u32);
            }
            if ((((((self.r32(v3.wrapping_add(1516)) as i32) == 1_i32)
                || ((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32))
            {
                self.w32(v3.wrapping_add(1504), 1_u32);
                self.w32(v3.wrapping_add(1508), (v5 as u32));
                self.w32(v3.wrapping_add(1512), 45_u32);
                self.w32(v3.wrapping_add(1516), 0_u32);
                if (((self.r32(v4) as i32) == 1_i32)
                    && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
                {
                    self.w32(v3.wrapping_add(1516), 1_u32);
                }
                self.w32(v3.wrapping_add(1520), 0_u32);
                if (((self.r32(v4.wrapping_add(716)) as i32) == 1_i32)
                    && ((self.r32(v4.wrapping_add(1088)) as i32) != 99_i32))
                {
                    self.w32(v3.wrapping_add(1520), 1_u32);
                }
                self.w32(v3.wrapping_add(1524), 0_u32);
                if (((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                    && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
                {
                    self.w32(v3.wrapping_add(1524), 1_u32);
                }
                self.w32(v3.wrapping_add(1528), 0_u32);
                if (((self.r32(v4.wrapping_add(2148)) as i32) == 1_i32)
                    && ((self.r32(v4.wrapping_add(2520)) as i32) != 99_i32))
                {
                    self.w32(v3.wrapping_add(1528), 1_u32);
                }
                self.w32(v3.wrapping_add(1532), 0_u32);
                if (((self.r32(v4.wrapping_add(2864)) as i32) == 1_i32)
                    && ((self.r32(v4.wrapping_add(3236)) as i32) != 99_i32))
                {
                    self.w32(v3.wrapping_add(1532), 1_u32);
                }
                if ((self.r32(v3.wrapping_add(1516)) as i32) == 1_i32) {
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(393_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        0_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        1_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        6_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        100_u32,
                    );
                    {
                        let a0 = v3.wrapping_add(
                            ({
                                let t2 = (self.r32(v3.wrapping_add(2072)) as i32);
                                self.w32(v3.wrapping_add(2072), (t2.wrapping_add(1) as u32));
                                t2
                            }
                            .wrapping_add(493_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                }
                if ((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32) {
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(393_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        1_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        1_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        6_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        100_u32,
                    );
                    {
                        let a0 = v3.wrapping_add(
                            ({
                                let t3 = (self.r32(v3.wrapping_add(2072)) as i32);
                                self.w32(v3.wrapping_add(2072), (t3.wrapping_add(1) as u32));
                                t3
                            }
                            .wrapping_add(493_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                }
                if ((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32) {
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(393_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        2_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        1_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        6_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        100_u32,
                    );
                    {
                        let a0 = v3.wrapping_add(
                            ({
                                let t4 = (self.r32(v3.wrapping_add(2072)) as i32);
                                self.w32(v3.wrapping_add(2072), (t4.wrapping_add(1) as u32));
                                t4
                            }
                            .wrapping_add(493_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                }
                if ((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32) {
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(393_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        3_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        1_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        6_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        100_u32,
                    );
                    {
                        let a0 = v3.wrapping_add(
                            ({
                                let t5 = (self.r32(v3.wrapping_add(2072)) as i32);
                                self.w32(v3.wrapping_add(2072), (t5.wrapping_add(1) as u32));
                                t5
                            }
                            .wrapping_add(493_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                }
                if ((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32) {
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(393_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        4_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(418_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        1_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(443_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        6_u32,
                    );
                    self.w32(
                        v3.wrapping_add(
                            ((self.r32(v3.wrapping_add(2072)) as i32).wrapping_add(468_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        100_u32,
                    );
                    {
                        let a0 = v3.wrapping_add(
                            ({
                                let t6 = (self.r32(v3.wrapping_add(2072)) as i32);
                                self.w32(v3.wrapping_add(2072), (t6.wrapping_add(1) as u32));
                                t6
                            }
                            .wrapping_add(493_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                }
                let _ = self.f_10007530(
                    this,
                    (self.r32(v3.wrapping_add(1508)) as i32),
                    (self.r32(v3.wrapping_add(1512)) as i32),
                );
                if (((self.r32(v15) as i32) == 1_i32)
                    && ((self.r32(v15.wrapping_add(372)) as i32) != 99_i32))
                {
                    v10 = 0_i32;
                    v11 = v15.wrapping_add(644);
                    'l2: loop {
                        if ((self.r32(v11) as i32) == 45_i32) {
                            break 'l2;
                        }
                        v10 = v10.wrapping_add(1);
                        v11 = v11.wrapping_add(4);
                        if !(v10 < 3_i32) {
                            break 'l2;
                        }
                    }
                    v12 = (self
                        .r32(v15.wrapping_add((v10.wrapping_add(164_i32) as u32).wrapping_mul(4)))
                        as i32)
                        .wrapping_sub(1_i32);
                    self.w32(
                        v15.wrapping_add((v10.wrapping_add(164_i32) as u32).wrapping_mul(4)),
                        (v12 as u32),
                    );
                    if (v12 <= 0_i32) {
                        self.w32(
                            v15.wrapping_add((v10.wrapping_add(161_i32) as u32).wrapping_mul(4)),
                            0_u32,
                        );
                        v13 = (self.r32(v3.wrapping_add(2280)) as i32);
                        self.w32(v3.wrapping_add(2188), 1_u32);
                        self.w32(v3.wrapping_add(2192), 0_u32);
                        self.w32(v3.wrapping_add(2196), (v16 as u32));
                        self.w32(
                            v3.wrapping_add((v13.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            45_u32,
                        );
                        self.w32(
                            v3.wrapping_add(2280),
                            ((self.r32(v3.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                    }
                }
                self.w8(a2, 1_u8);
            }
        }
        return 0_i32;
    }

    /// `sub_10017790` (1015 bytes).
    pub(crate) fn f_10017790(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut i: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v18: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self.r32((v2.wrapping_add(12_i32) as u32));
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w8(a2, 0_u8);
        v6 = v4.wrapping_add((179_i32.wrapping_mul(v5) as u32).wrapping_mul(4));
        v18 = v6;
        if (((self.r32(v6) as i32) == 1_i32) && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            i = v6.wrapping_add(644);
            'l1: loop {
                if !((self.r32(i) as i32) != 46_i32) {
                    break 'l1;
                }
                if ({
                    let t1 = v7.wrapping_add(1);
                    v7 = t1;
                    t1
                } >= 3_i32)
                {
                    return 0_i32;
                }
                i = i.wrapping_add(4);
            }
            self.w32(v3.wrapping_add(1516), 0_u32);
            if ((((self.r32(v4) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                    (self.r32(v4.wrapping_add(220)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1516), 1_u32);
            }
            self.w32(v3.wrapping_add(1520), 0_u32);
            if ((((self.r32(v4.wrapping_add(716)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1088)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                    (self.r32(v4.wrapping_add(936)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1520), 1_u32);
            }
            self.w32(v3.wrapping_add(1524), 0_u32);
            if ((((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                    (self.r32(v4.wrapping_add(1652)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1524), 1_u32);
            }
            self.w32(v3.wrapping_add(1528), 0_u32);
            if ((((self.r32(v4.wrapping_add(2148)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(2520)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                    (self.r32(v4.wrapping_add(2368)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1528), 1_u32);
            }
            self.w32(v3.wrapping_add(1532), 0_u32);
            if ((((self.r32(v4.wrapping_add(2864)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(3236)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(3132)) as i32)),
                    (self.r32(v4.wrapping_add(3084)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1532), 1_u32);
            }
            v10 = (self.r32(v3.wrapping_add(1516)) as i32);
            if (((((v10 == 1_i32) || ((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32))
            {
                self.w32(v3.wrapping_add(1504), 1_u32);
                self.w32(v3.wrapping_add(1508), (v5 as u32));
                self.w32(v3.wrapping_add(1512), 46_u32);
                v11 = 100_i32;
                if ((v10 == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                        (self.r32(v4.wrapping_add(220)) as i32),
                    ) < 100_i32))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                        (self.r32(v4.wrapping_add(220)) as i32),
                    );
                    v12 = 0_i32;
                } else {
                    v12 = (a2 as i32);
                }
                if (((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                        (self.r32(v4.wrapping_add(936)) as i32),
                    ) < v11))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                        (self.r32(v4.wrapping_add(936)) as i32),
                    );
                    v12 = 1_i32;
                }
                if (((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                        (self.r32(v4.wrapping_add(1652)) as i32),
                    ) < v11))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                        (self.r32(v4.wrapping_add(1652)) as i32),
                    );
                    v12 = 2_i32;
                }
                if (((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                        (self.r32(v4.wrapping_add(2368)) as i32),
                    ) < v11))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                        (self.r32(v4.wrapping_add(2368)) as i32),
                    );
                    v12 = 3_i32;
                }
                if (((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(3132)) as i32)),
                        (self.r32(v4.wrapping_add(3084)) as i32),
                    ) < v11))
                {
                    v12 = 4_i32;
                }
                self.w32(v3.wrapping_add(1516), 0_u32);
                self.w32(v3.wrapping_add(1520), 0_u32);
                self.w32(v3.wrapping_add(1524), 0_u32);
                self.w32(v3.wrapping_add(1528), 0_u32);
                self.w32(v3.wrapping_add(1532), 0_u32);
                's2: {
                    'b2_5: {
                        'b2_4: {
                            'b2_3: {
                                'b2_2: {
                                    'b2_1: {
                                        'b2_0: {
                                            match v12 {
                                                0_i32 => break 'b2_0,
                                                1_i32 => break 'b2_1,
                                                2_i32 => break 'b2_2,
                                                3_i32 => break 'b2_3,
                                                4_i32 => break 'b2_4,
                                                _ => break 'b2_5,
                                            }
                                        }
                                        self.w32(v3.wrapping_add(1516), 1_u32);
                                        break 's2;
                                    }
                                    self.w32(v3.wrapping_add(1520), 1_u32);
                                    break 's2;
                                }
                                self.w32(v3.wrapping_add(1524), 1_u32);
                                break 's2;
                            }
                            self.w32(v3.wrapping_add(1528), 1_u32);
                            break 's2;
                        }
                        self.w32(v3.wrapping_add(1532), 1_u32);
                        break 's2;
                    }
                    break 's2;
                }
                if ((self.r32(v3.wrapping_add(1516)) as i32) == 1_i32) {
                    let _ = self.f_1000a260(this, v5, 0_i32, 46_i32);
                } else {
                    if ((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32) {
                        let _ = self.f_1000a260(this, v5, 1_i32, 46_i32);
                    } else {
                        if ((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32) {
                            let _ = self.f_1000a260(this, v5, 2_i32, 46_i32);
                        } else {
                            if ((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32) {
                                let _ = self.f_1000a260(this, v5, 3_i32, 46_i32);
                            } else {
                                if ((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32) {
                                    let _ = self.f_1000a260(this, v5, 4_i32, 46_i32);
                                }
                            }
                        }
                    }
                }
                let _ = self.f_10007530(
                    this,
                    (self.r32(v3.wrapping_add(1508)) as i32),
                    (self.r32(v3.wrapping_add(1512)) as i32),
                );
                if (((self.r32(v18) as i32) == 1_i32)
                    && ((self.r32(v18.wrapping_add(372)) as i32) != 99_i32))
                {
                    v13 = 0_i32;
                    v14 = v18.wrapping_add(644);
                    'l3: loop {
                        if ((self.r32(v14) as i32) == 46_i32) {
                            break 'l3;
                        }
                        v13 = v13.wrapping_add(1);
                        v14 = v14.wrapping_add(4);
                        if !(v13 < 3_i32) {
                            break 'l3;
                        }
                    }
                    v15 = (self
                        .r32(v18.wrapping_add((v13.wrapping_add(164_i32) as u32).wrapping_mul(4)))
                        as i32)
                        .wrapping_sub(1_i32);
                    self.w32(
                        v18.wrapping_add((v13.wrapping_add(164_i32) as u32).wrapping_mul(4)),
                        (v15 as u32),
                    );
                    if (v15 <= 0_i32) {
                        self.w32(
                            v18.wrapping_add((v13.wrapping_add(161_i32) as u32).wrapping_mul(4)),
                            0_u32,
                        );
                        v16 = (self.r32(v3.wrapping_add(2280)) as i32);
                        self.w32(v3.wrapping_add(2188), 1_u32);
                        self.w32(v3.wrapping_add(2192), 0_u32);
                        self.w32(v3.wrapping_add(2196), (v5 as u32));
                        self.w32(
                            v3.wrapping_add((v16.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            46_u32,
                        );
                        self.w32(
                            v3.wrapping_add(2280),
                            ((self.r32(v3.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                    }
                }
                self.w8(a2, 1_u8);
            }
        }
        return 0_i32;
    }

    /// `sub_10017BA0` (1015 bytes).
    pub(crate) fn f_10017ba0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut i: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v18: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self.r32((v2.wrapping_add(12_i32) as u32));
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w8(a2, 0_u8);
        v6 = v4.wrapping_add((179_i32.wrapping_mul(v5) as u32).wrapping_mul(4));
        v18 = v6;
        if (((self.r32(v6) as i32) == 1_i32) && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            i = v6.wrapping_add(644);
            'l1: loop {
                if !((self.r32(i) as i32) != 47_i32) {
                    break 'l1;
                }
                if ({
                    let t1 = v7.wrapping_add(1);
                    v7 = t1;
                    t1
                } >= 3_i32)
                {
                    return 0_i32;
                }
                i = i.wrapping_add(4);
            }
            self.w32(v3.wrapping_add(1516), 0_u32);
            if ((((self.r32(v4) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                    (self.r32(v4.wrapping_add(220)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1516), 1_u32);
            }
            self.w32(v3.wrapping_add(1520), 0_u32);
            if ((((self.r32(v4.wrapping_add(716)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1088)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                    (self.r32(v4.wrapping_add(936)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1520), 1_u32);
            }
            self.w32(v3.wrapping_add(1524), 0_u32);
            if ((((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                    (self.r32(v4.wrapping_add(1652)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1524), 1_u32);
            }
            self.w32(v3.wrapping_add(1528), 0_u32);
            if ((((self.r32(v4.wrapping_add(2148)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(2520)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                    (self.r32(v4.wrapping_add(2368)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1528), 1_u32);
            }
            self.w32(v3.wrapping_add(1532), 0_u32);
            if ((((self.r32(v4.wrapping_add(2864)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(3236)) as i32) != 99_i32))
                && (cdiv_i32(
                    100_i32.wrapping_mul((self.r32(v4.wrapping_add(3132)) as i32)),
                    (self.r32(v4.wrapping_add(3084)) as i32),
                ) < 30_i32))
            {
                self.w32(v3.wrapping_add(1532), 1_u32);
            }
            v10 = (self.r32(v3.wrapping_add(1516)) as i32);
            if (((((v10 == 1_i32) || ((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32))
                || ((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32))
            {
                self.w32(v3.wrapping_add(1504), 1_u32);
                self.w32(v3.wrapping_add(1508), (v5 as u32));
                self.w32(v3.wrapping_add(1512), 47_u32);
                v11 = 100_i32;
                if ((v10 == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                        (self.r32(v4.wrapping_add(220)) as i32),
                    ) < 100_i32))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                        (self.r32(v4.wrapping_add(220)) as i32),
                    );
                    v12 = 0_i32;
                } else {
                    v12 = (a2 as i32);
                }
                if (((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                        (self.r32(v4.wrapping_add(936)) as i32),
                    ) < v11))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                        (self.r32(v4.wrapping_add(936)) as i32),
                    );
                    v12 = 1_i32;
                }
                if (((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                        (self.r32(v4.wrapping_add(1652)) as i32),
                    ) < v11))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                        (self.r32(v4.wrapping_add(1652)) as i32),
                    );
                    v12 = 2_i32;
                }
                if (((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                        (self.r32(v4.wrapping_add(2368)) as i32),
                    ) < v11))
                {
                    v11 = cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                        (self.r32(v4.wrapping_add(2368)) as i32),
                    );
                    v12 = 3_i32;
                }
                if (((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32)
                    && (cdiv_i32(
                        100_i32.wrapping_mul((self.r32(v4.wrapping_add(3132)) as i32)),
                        (self.r32(v4.wrapping_add(3084)) as i32),
                    ) < v11))
                {
                    v12 = 4_i32;
                }
                self.w32(v3.wrapping_add(1516), 0_u32);
                self.w32(v3.wrapping_add(1520), 0_u32);
                self.w32(v3.wrapping_add(1524), 0_u32);
                self.w32(v3.wrapping_add(1528), 0_u32);
                self.w32(v3.wrapping_add(1532), 0_u32);
                's2: {
                    'b2_5: {
                        'b2_4: {
                            'b2_3: {
                                'b2_2: {
                                    'b2_1: {
                                        'b2_0: {
                                            match v12 {
                                                0_i32 => break 'b2_0,
                                                1_i32 => break 'b2_1,
                                                2_i32 => break 'b2_2,
                                                3_i32 => break 'b2_3,
                                                4_i32 => break 'b2_4,
                                                _ => break 'b2_5,
                                            }
                                        }
                                        self.w32(v3.wrapping_add(1516), 1_u32);
                                        break 's2;
                                    }
                                    self.w32(v3.wrapping_add(1520), 1_u32);
                                    break 's2;
                                }
                                self.w32(v3.wrapping_add(1524), 1_u32);
                                break 's2;
                            }
                            self.w32(v3.wrapping_add(1528), 1_u32);
                            break 's2;
                        }
                        self.w32(v3.wrapping_add(1532), 1_u32);
                        break 's2;
                    }
                    break 's2;
                }
                if ((self.r32(v3.wrapping_add(1516)) as i32) == 1_i32) {
                    let _ = self.f_1000a260(this, v5, 0_i32, 47_i32);
                } else {
                    if ((self.r32(v3.wrapping_add(1520)) as i32) == 1_i32) {
                        let _ = self.f_1000a260(this, v5, 1_i32, 47_i32);
                    } else {
                        if ((self.r32(v3.wrapping_add(1524)) as i32) == 1_i32) {
                            let _ = self.f_1000a260(this, v5, 2_i32, 47_i32);
                        } else {
                            if ((self.r32(v3.wrapping_add(1528)) as i32) == 1_i32) {
                                let _ = self.f_1000a260(this, v5, 3_i32, 47_i32);
                            } else {
                                if ((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32) {
                                    let _ = self.f_1000a260(this, v5, 4_i32, 47_i32);
                                }
                            }
                        }
                    }
                }
                let _ = self.f_10007530(
                    this,
                    (self.r32(v3.wrapping_add(1508)) as i32),
                    (self.r32(v3.wrapping_add(1512)) as i32),
                );
                if (((self.r32(v18) as i32) == 1_i32)
                    && ((self.r32(v18.wrapping_add(372)) as i32) != 99_i32))
                {
                    v13 = 0_i32;
                    v14 = v18.wrapping_add(644);
                    'l3: loop {
                        if ((self.r32(v14) as i32) == 47_i32) {
                            break 'l3;
                        }
                        v13 = v13.wrapping_add(1);
                        v14 = v14.wrapping_add(4);
                        if !(v13 < 3_i32) {
                            break 'l3;
                        }
                    }
                    v15 = (self
                        .r32(v18.wrapping_add((v13.wrapping_add(164_i32) as u32).wrapping_mul(4)))
                        as i32)
                        .wrapping_sub(1_i32);
                    self.w32(
                        v18.wrapping_add((v13.wrapping_add(164_i32) as u32).wrapping_mul(4)),
                        (v15 as u32),
                    );
                    if (v15 <= 0_i32) {
                        self.w32(
                            v18.wrapping_add((v13.wrapping_add(161_i32) as u32).wrapping_mul(4)),
                            0_u32,
                        );
                        v16 = (self.r32(v3.wrapping_add(2280)) as i32);
                        self.w32(v3.wrapping_add(2188), 1_u32);
                        self.w32(v3.wrapping_add(2192), 0_u32);
                        self.w32(v3.wrapping_add(2196), (v5 as u32));
                        self.w32(
                            v3.wrapping_add((v16.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            47_u32,
                        );
                        self.w32(
                            v3.wrapping_add(2280),
                            ((self.r32(v3.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                    }
                }
                self.w8(a2, 1_u8);
            }
        }
        return 0_i32;
    }

    /// `sub_10017FB0` (643 bytes).
    pub(crate) fn f_10017fb0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10017fb0_body(fp, this, a2);
        self.leave(16);
        r
    }

    fn f_10017fb0_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v7: u32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(self.r32(this));
                    self.w32(fp.wrapping_add(2), this);
                    v3 = ((self.r32(v2.wrapping_add(28)) as i32) as u32);
                    v4 = ((self.r32(v2.wrapping_add(12)) as i32) as u32);
                    v5 = (self.r32(v3.wrapping_add(1388)) as i32);
                    self.w8(a2, 0_u8);
                    let _ = self.f_10007420(this, v5, fp.wrapping_add(1), fp.wrapping_add(0));
                    bb = if ((((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32)
                        || (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32))
                    {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    self.w32(v3.wrapping_add(1516), 0_u32);
                    bb = if ((((self.r32(v4) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
                        && (cdiv_i32(
                            100_i32.wrapping_mul((self.r32(v4.wrapping_add(268)) as i32)),
                            (self.r32(v4.wrapping_add(220)) as i32),
                        ) < 30_i32))
                    {
                        3
                    } else {
                        4
                    };
                }
                2 => {
                    return 0_i32;
                }
                3 => {
                    self.w32(v3.wrapping_add(1516), 1_u32);
                    bb = 4;
                }
                4 => {
                    self.w32(v3.wrapping_add(1520), 0_u32);
                    bb = if ((((self.r32(v4.wrapping_add(716)) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(1088)) as i32) != 99_i32))
                        && (cdiv_i32(
                            100_i32.wrapping_mul((self.r32(v4.wrapping_add(984)) as i32)),
                            (self.r32(v4.wrapping_add(936)) as i32),
                        ) < 30_i32))
                    {
                        5
                    } else {
                        6
                    };
                }
                5 => {
                    self.w32(v3.wrapping_add(1520), 1_u32);
                    bb = 6;
                }
                6 => {
                    self.w32(v3.wrapping_add(1524), 0_u32);
                    bb = if ((((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
                        && (cdiv_i32(
                            100_i32.wrapping_mul((self.r32(v4.wrapping_add(1700)) as i32)),
                            (self.r32(v4.wrapping_add(1652)) as i32),
                        ) < 30_i32))
                    {
                        7
                    } else {
                        8
                    };
                }
                7 => {
                    self.w32(v3.wrapping_add(1524), 1_u32);
                    bb = 8;
                }
                8 => {
                    self.w32(v3.wrapping_add(1528), 0_u32);
                    bb = if ((((self.r32(v4.wrapping_add(2148)) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(2520)) as i32) != 99_i32))
                        && (cdiv_i32(
                            100_i32.wrapping_mul((self.r32(v4.wrapping_add(2416)) as i32)),
                            (self.r32(v4.wrapping_add(2368)) as i32),
                        ) < 30_i32))
                    {
                        9
                    } else {
                        10
                    };
                }
                9 => {
                    self.w32(v3.wrapping_add(1528), 1_u32);
                    bb = 10;
                }
                10 => {
                    self.w32(v3.wrapping_add(1532), 0_u32);
                    bb = if ((((self.r32(v4.wrapping_add(2864)) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(3236)) as i32) != 99_i32))
                        && (cdiv_i32(
                            100_i32.wrapping_mul((self.r32(v4.wrapping_add(3132)) as i32)),
                            (self.r32(v4.wrapping_add(3084)) as i32),
                        ) < 30_i32))
                    {
                        11
                    } else {
                        12
                    };
                }
                11 => {
                    self.w32(v3.wrapping_add(1532), 1_u32);
                    bb = 12;
                }
                12 => {
                    let t1 = v5;
                    bb = match t1 {
                        0_i32 => 14,
                        1_i32 => 15,
                        2_i32 => 16,
                        3_i32 => 17,
                        _ => 13,
                    };
                }
                13 => {
                    bb = if ((v5 == 4_i32) && ((self.r32(v3.wrapping_add(1532)) as i32) == 1_i32)) {
                        35
                    } else {
                        36
                    };
                }
                14 => {
                    bb = if ((self.r32(v3.wrapping_add(1516)) as i32) != 1_i32) {
                        18
                    } else {
                        19
                    };
                }
                15 => {
                    bb = if ((self.r32(v3.wrapping_add(1520)) as i32) != 1_i32) {
                        23
                    } else {
                        24
                    };
                }
                16 => {
                    bb = if ((self.r32(v3.wrapping_add(1524)) as i32) != 1_i32) {
                        27
                    } else {
                        28
                    };
                }
                17 => {
                    bb = if ((self.r32(v3.wrapping_add(1528)) as i32) != 1_i32) {
                        31
                    } else {
                        32
                    };
                }
                18 => {
                    return 0_i32;
                }
                19 => {
                    bb = 21;
                }
                20 => {
                    bb = 19;
                }
                21 => {
                    self.w32(v3.wrapping_add(1516), 0_u32);
                    self.w32(v3.wrapping_add(1520), 0_u32);
                    self.w32(v3.wrapping_add(1524), 0_u32);
                    self.w32(v3.wrapping_add(1528), 0_u32);
                    self.w32(v3.wrapping_add(1532), 0_u32);
                    let t2 = v5;
                    bb = match t2 {
                        0_i32 => 38,
                        1_i32 => 39,
                        2_i32 => 40,
                        3_i32 => 41,
                        4_i32 => 42,
                        _ => 43,
                    };
                }
                22 => {
                    bb = 15;
                }
                23 => {
                    return 0_i32;
                }
                24 => {
                    bb = 21;
                }
                25 => {
                    bb = 24;
                }
                26 => {
                    bb = 16;
                }
                27 => {
                    return 0_i32;
                }
                28 => {
                    bb = 21;
                }
                29 => {
                    bb = 28;
                }
                30 => {
                    bb = 17;
                }
                31 => {
                    return 0_i32;
                }
                32 => {
                    bb = 21;
                }
                33 => {
                    bb = 32;
                }
                34 => {
                    bb = 13;
                }
                35 => {
                    bb = 21;
                }
                36 => {
                    bb = 2;
                }
                37 => {
                    self.w32(v3.wrapping_add(1504), 1_u32);
                    self.w32(v3.wrapping_add(1508), (v5 as u32));
                    self.w32(v3.wrapping_add(1512), 46_u32);
                    v7 = self.r32(fp.wrapping_add(2));
                    let _ = self.f_1000a260(self.r32(fp.wrapping_add(2)), v5, v5, 46_i32);
                    bb = if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                        50
                    } else {
                        52
                    };
                }
                38 => {
                    self.w32(v3.wrapping_add(1516), 1_u32);
                    bb = 37;
                }
                39 => {
                    self.w32(v3.wrapping_add(1520), 1_u32);
                    bb = 37;
                }
                40 => {
                    self.w32(v3.wrapping_add(1524), 1_u32);
                    bb = 37;
                }
                41 => {
                    self.w32(v3.wrapping_add(1528), 1_u32);
                    bb = 37;
                }
                42 => {
                    self.w32(v3.wrapping_add(1532), 1_u32);
                    bb = 37;
                }
                43 => {
                    bb = 37;
                }
                44 => {
                    bb = 39;
                }
                45 => {
                    bb = 40;
                }
                46 => {
                    bb = 41;
                }
                47 => {
                    bb = 42;
                }
                48 => {
                    bb = 43;
                }
                49 => {
                    bb = 37;
                }
                50 => {
                    let _ = self.f_10007480(v7, v5, 0_i8, 1_i8);
                    bb = 51;
                }
                51 => {
                    self.w8(a2, 1_u8);
                    bb = 36;
                }
                52 => {
                    bb = if (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32) {
                        53
                    } else {
                        54
                    };
                }
                53 => {
                    let _ = self.f_10007480(v7, v5, 1_i8, 0_i8);
                    bb = 54;
                }
                54 => {
                    bb = 51;
                }
                55 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10018250` (57 bytes).
    pub(crate) fn f_10018250(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        self.w8(a2, 1_u8);
        v3 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1504), 1_u32);
        self.w32(v2.wrapping_add(1508), (v3 as u32));
        self.w32(v2.wrapping_add(1512), 48_u32);
        self.w32(v2.wrapping_add(1516), 1_u32);
        return 0_i32;
    }

    /// `sub_10018290` (136 bytes).
    pub(crate) fn f_10018290(&mut self, mut this: u32, mut a2: i32, mut a3: u32) -> i32 {
        let fp = self.enter(32);
        let r = self.f_10018290_body(fp, this, a2, a3);
        self.leave(32);
        r
    }

    fn f_10018290_body(&mut self, fp: u32, mut this: u32, mut a2: i32, mut a3: u32) -> i32 {
        let mut v4: u32 = 0;
        v4 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        self.w8(a3, 1_u8);
        let _ = self.f_100064e0(
            this,
            fp.wrapping_add(12),
            (fp.wrapping_add(0) as i32),
            0_i32,
            a2,
            0_i32,
        );
        let _ = self.f_10009340(
            this,
            a2,
            (self.r32(v4.wrapping_add(2360)) as i32),
            fp.wrapping_add(12),
            (fp.wrapping_add(0) as i32),
        );
        self.w32(v4.wrapping_add(1416), 0_u32);
        self.w32(v4.wrapping_add(1420), (a2 as u32));
        self.w32(v4.wrapping_add(1504), 1_u32);
        self.w32(v4.wrapping_add(1508), (a2 as u32));
        self.w32(v4.wrapping_add(1512), 48_u32);
        self.w32(v4.wrapping_add(1516), 0_u32);
        return 0_i32;
    }

    /// `sub_10018320` (95 bytes).
    pub(crate) fn f_10018320(&mut self, mut this: u32, mut a2: i32, mut a3: u32) -> i32 {
        let mut v4: u32 = 0;
        v4 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        if (crem_i32(self.rand(), 100_i32) >= 30_i32) {
            self.w8(a3, 0_u8);
        } else {
            let _ = self.f_10018290(this, a2, a3);
            self.w32(v4.wrapping_add(1508), (a2 as u32));
            self.w32(v4.wrapping_add(1504), 1_u32);
            self.w32(v4.wrapping_add(1512), 49_u32);
        }
        return 0_i32;
    }

    /// `sub_10018380` (54 bytes).
    pub(crate) fn f_10018380(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        v3 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w8(a2, 1_u8);
        self.w32(v2.wrapping_add(1504), 1_u32);
        self.w32(v2.wrapping_add(1508), (v3.wrapping_add(5_i32) as u32));
        self.w32(v2.wrapping_add(1512), 51_u32);
        return 0_i32;
    }

    /// `sub_100183C0` (54 bytes).
    pub(crate) fn f_100183c0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        v3 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w8(a2, 1_u8);
        self.w32(v2.wrapping_add(1504), 1_u32);
        self.w32(v2.wrapping_add(1508), (v3.wrapping_add(5_i32) as u32));
        self.w32(v2.wrapping_add(1512), 52_u32);
        return 0_i32;
    }

    /// `sub_10018400` (61 bytes).
    pub(crate) fn f_10018400(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        v3 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w8(a2, 1_u8);
        self.w32(v2.wrapping_add(1504), 1_u32);
        self.w32(v2.wrapping_add(1508), (v3.wrapping_add(5_i32) as u32));
        self.w32(v2.wrapping_add(1512), 53_u32);
        let _ = self.f_10018440(this);
        return 0_i32;
    }

    /// `sub_10018440` (3804 bytes).
    pub(crate) fn f_10018440(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(1952);
        let r = self.f_10018440_body(fp, this);
        self.leave(1952);
        r
    }

    fn f_10018440_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        let mut v13: u32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: u32 = 0;
        let mut v20: i64 = 0;
        let mut v21: i32 = 0;
        let mut v22: u32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: u32 = 0;
        let mut v29: u32 = 0;
        let mut v30: u32 = 0;
        let mut v31: i32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: u32 = 0;
        let mut v36: u32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v39: u32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: i32 = 0;
        let mut v43: i32 = 0;
        let mut v44: bool = false;
        let mut v45: i32 = 0;
        let mut v46: i32 = 0;
        let mut v47: i32 = 0;
        let mut v48: u32 = 0;
        let mut v49: u32 = 0;
        let mut v50: i32 = 0;
        let mut v51: bool = false;
        let mut v52: u32 = 0;
        let mut v53: u32 = 0;
        let mut v54: u32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: i32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: i32 = 0;
        let mut v62: u32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut i: u32 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: u32 = 0;
        let mut v72: i32 = 0;
        let mut v73: i8 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut v77: i32 = 0;
        let mut v78: i32 = 0;
        let mut v79: i32 = 0;
        let mut v80: i32 = 0;
        let mut v81: u32 = 0;
        let mut v83: u32 = 0;
        let mut v84: i32 = 0;
        let mut v85: i32 = 0;
        let mut v86: u32 = 0;
        let mut v87: i32 = 0;
        let mut v88: i32 = 0;
        let mut v89: u32 = 0;
        let mut v90: i64 = 0;
        let mut v91: i32 = 0;
        let mut v92: i32 = 0;
        let mut v93: i32 = 0;
        let mut v94: i32 = 0;
        let mut v95: i32 = 0;
        let mut v96: u32 = 0;
        let mut v97: i32 = 0;
        let mut v98: u32 = 0;
        let mut v99: i32 = 0;
        let mut v100: i32 = 0;
        let mut v101: u32 = 0;
        let mut v102: i32 = 0;
        let mut v103: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(this);
                    self.w32(fp.wrapping_add(96), this);
                    v3 = ((self.r32(v2) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    v5 = (self.r32(v3.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(104),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(28), (v5 as u32));
                    self.w32(fp.wrapping_add(0), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(84),
                        ((self.r32(self.r32(fp.wrapping_add(104)).wrapping_add(1388)) as i32)
                            as u32),
                    );
                    v103 = (self.r32(self.r32(fp.wrapping_add(104)).wrapping_add(1384)) as i32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(
                            (v5.wrapping_add(
                                156_i32.wrapping_mul((self.r32(fp.wrapping_add(84)) as i32)),
                            )
                            .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v6 = (self.r32(
                        (v5.wrapping_add(
                            156_i32.wrapping_mul((self.r32(fp.wrapping_add(84)) as i32)),
                        )
                        .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(112),
                        ((self.r32(
                            (v5.wrapping_add(
                                156_i32.wrapping_mul((self.r32(fp.wrapping_add(84)) as i32)),
                            )
                            .wrapping_add(3732_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(88), (v4 as u32));
                    self.w32(fp.wrapping_add(12), (v6 as u32));
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(120),
                        (fp.wrapping_add(140) as i32),
                        v103,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        0_i32,
                    );
                    self.w32(fp.wrapping_add(108), self.r32(fp.wrapping_add(128)));
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(fp.wrapping_add(124)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(132)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(116),
                        ((self.r32(fp.wrapping_add(136)) as i32) as u32),
                    );
                    let _ = self.f_10006940(
                        this,
                        fp.wrapping_add(92),
                        (self.r32(fp.wrapping_add(124)) as i32),
                        (self.r32(fp.wrapping_add(128)) as i32),
                        (self.r32(fp.wrapping_add(132)) as i32),
                        (self.r32(fp.wrapping_add(136)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(self.r32(fp.wrapping_add(92))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        self.r32(fp.wrapping_add(92)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(48), fp.wrapping_add(252));
                    v7 = 0_i32;
                    v8 = (v5.wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(fp.wrapping_add(100), (v5.wrapping_add(3600_i32) as u32));
                    self.w32(fp.wrapping_add(4), (v5.wrapping_add(3600_i32) as u32));
                    self.w32(fp.wrapping_add(8), 0_u32);
                    self.w32(fp.wrapping_add(72), fp.wrapping_add(152));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v8) as i32) == 1_i32)
                        && ((self.r32(v8.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (v7 < 20_i32) { 1 } else { 3 };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) <= 0_i32) {
                        33
                    } else {
                        34
                    };
                }
                4 => {
                    v9 = (self.r32(fp.wrapping_add(76)) as i32)
                        .wrapping_add((self.r32(v8.wrapping_add(124)) as i32));
                    v10 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v8.wrapping_add(128)) as i32));
                    v11 = v9.wrapping_add((self.r32(fp.wrapping_add(36)) as i32));
                    self.w32(
                        fp.wrapping_add(64),
                        (v10.wrapping_add((self.r32(fp.wrapping_add(40)) as i32)) as u32),
                    );
                    v12 = self.r32(fp.wrapping_add(60));
                    self.w8(fp.wrapping_add(54), 0_u8);
                    self.w8(fp.wrapping_add(55), 0_u8);
                    self.w32(fp.wrapping_add(32), (v10 as u32));
                    bb = if (v10 >= v10.wrapping_add((self.r32(fp.wrapping_add(40)) as i32))) {
                        6
                    } else {
                        8
                    };
                }
                5 => {
                    v8 = v8.wrapping_add(156);
                    v7 = v7.wrapping_add(1);
                    self.w32(fp.wrapping_add(4), v8);
                    self.w32(fp.wrapping_add(8), (v7 as u32));
                    bb = 2;
                }
                6 => {
                    v7 = (self.r32(fp.wrapping_add(8)) as i32);
                    bb = 7;
                }
                7 => {
                    bb = if (v7 != (self.r32(fp.wrapping_add(84)) as i32)) {
                        27
                    } else {
                        28
                    };
                }
                8 => {
                    v13 = (20_i32.wrapping_mul(v10) as u32);
                    bb = 9;
                }
                9 => {
                    v14 = (v9 as u32);
                    bb = if (v9 < v11) { 12 } else { 13 };
                }
                10 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32)
                        < (self.r32(fp.wrapping_add(64)) as i32))
                    {
                        9
                    } else {
                        11
                    };
                }
                11 => {
                    v7 = (self.r32(fp.wrapping_add(8)) as i32);
                    v8 = self.r32(fp.wrapping_add(4));
                    bb = 7;
                }
                12 => {
                    bb = 14;
                }
                13 => {
                    v13 = v13.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 10;
                }
                14 => {
                    bb = if ((self.r32(v12) as i32) != 0) {
                        17
                    } else {
                        18
                    };
                }
                15 => {
                    bb = if ((v14 as i32) < v9.wrapping_add((self.r32(fp.wrapping_add(36)) as i32)))
                    {
                        14
                    } else {
                        16
                    };
                }
                16 => {
                    bb = 13;
                }
                17 => {
                    bb = if ((v13 < 400_u32) && (v14 < 20_u32)) {
                        19
                    } else {
                        20
                    };
                }
                18 => {
                    v12 = v12.wrapping_add(4);
                    v14 = v14.wrapping_add(1);
                    v11 = v9.wrapping_add((self.r32(fp.wrapping_add(36)) as i32));
                    bb = 15;
                }
                19 => {
                    v15 = (4_u32.wrapping_mul(v13.wrapping_add(v14)) as i32);
                    bb = if ((self.r32((v15.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        21
                    } else {
                        22
                    };
                }
                20 => {
                    bb = 18;
                }
                21 => {
                    v16 = (self.r32((v15.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v16 == (1_i32).wrapping_neg())
                        || (v16 == (self.r32(fp.wrapping_add(84)) as i32).wrapping_add(5_i32)))
                    {
                        23
                    } else {
                        24
                    };
                }
                22 => {
                    bb = 20;
                }
                23 => {
                    self.w8(fp.wrapping_add(54), 1_u8);
                    bb = if ((v14 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(32)) as i32)
                            == (self.r32(fp.wrapping_add(12)) as i32)))
                    {
                        25
                    } else {
                        26
                    };
                }
                24 => {
                    bb = 22;
                }
                25 => {
                    self.w8(fp.wrapping_add(55), 1_u8);
                    bb = 26;
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(54)) as i8) as i32) == 1_i32) {
                        29
                    } else {
                        30
                    };
                }
                28 => {
                    bb = 5;
                }
                29 => {
                    self.w32(self.r32(fp.wrapping_add(72)), (v7 as u32));
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(16)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(72),
                        self.r32(fp.wrapping_add(72)).wrapping_add(4),
                    );
                    bb = 30;
                }
                30 => {
                    bb = if (((self.r8(fp.wrapping_add(55)) as i8) as i32) == 1_i32) {
                        31
                    } else {
                        32
                    };
                }
                31 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(48));
                            self.w32(fp.wrapping_add(48), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = (v7 as u32);
                        self.w32(a0, a1)
                    };
                    bb = 32;
                }
                32 => {
                    bb = 28;
                }
                33 => {
                    v83 = self.r32(fp.wrapping_add(100));
                    v84 = 0_i32;
                    v85 = 0_i32;
                    v86 = fp.wrapping_add(152);
                    bb = 35;
                }
                34 => {
                    v17 = 0_i32;
                    v18 = 9999_i32;
                    self.w32(fp.wrapping_add(32), 0_u32);
                    v19 = fp.wrapping_add(152);
                    bb = 61;
                }
                35 => {
                    bb = if ((((self.r32(v83) as i32) == 1_i32)
                        && ((self.r32(v83.wrapping_add(116)) as i32) != 99_i32))
                        && (v85 != (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        38
                    } else {
                        39
                    };
                }
                36 => {
                    bb = if (v85 < 20_i32) { 35 } else { 37 };
                }
                37 => {
                    self.w32(fp.wrapping_add(16), (v84 as u32));
                    bb = if (v84 <= 0_i32) { 40 } else { 42 };
                }
                38 => {
                    self.w32(v86, (v85 as u32));
                    v84 = v84.wrapping_add(1);
                    v86 = v86.wrapping_add(4);
                    bb = 39;
                }
                39 => {
                    v83 = v83.wrapping_add(156);
                    v85 = v85.wrapping_add(1);
                    bb = 36;
                }
                40 => {
                    {
                        let a0 = fp.wrapping_add(48);
                        let a1 = (crem_i32(self.rand(), 20_i32) as u32);
                        self.w32(a0, a1)
                    };
                    v93 = crem_i32(self.rand(), 20_i32);
                    bb = 41;
                }
                41 => {
                    v94 = 0_i32;
                    v95 = (self
                        .r32(((self.r32(fp.wrapping_add(88)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    bb = if (v95 > 0_i32) { 48 } else { 49 };
                }
                42 => {
                    v87 = 0_i32;
                    v88 = 9999_i32;
                    self.w32(fp.wrapping_add(32), 0_u32);
                    v89 = fp.wrapping_add(152);
                    bb = 43;
                }
                43 => {
                    v90 = ((self.r32(
                        ((self.r32(fp.wrapping_add(28)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v89) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v91 = ((((((v90 as u64) >> 32) as u32) as i64) ^ v90)
                        .wrapping_sub(((((v90 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(28)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v89) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(12)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v91 < v88) { 46 } else { 47 };
                }
                44 => {
                    bb = if (v87 < (self.r32(fp.wrapping_add(16)) as i32)) {
                        43
                    } else {
                        45
                    };
                }
                45 => {
                    v92 = (self.r32(fp.wrapping_add(152).wrapping_add(
                        ((self.r32(fp.wrapping_add(32)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    v93 = (self.r32(
                        ((self.r32(fp.wrapping_add(28)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(v92))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(28)) as i32)
                                .wrapping_add(156_i32.wrapping_mul(v92))
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    bb = 41;
                }
                46 => {
                    v88 = v91;
                    self.w32(fp.wrapping_add(32), (v87 as u32));
                    bb = 47;
                }
                47 => {
                    v87 = v87.wrapping_add(1);
                    v89 = v89.wrapping_add(4);
                    bb = 44;
                }
                48 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(352),
                        self.r32(fp.wrapping_add(0)),
                        (4_i32.wrapping_mul(v95) as u32),
                    );
                    v94 = v95;
                    self.w32(fp.wrapping_add(4), (v95 as u32));
                    bb = 49;
                }
                49 => {
                    self.w32(fp.wrapping_add(20), 9999_u32);
                    self.w32(fp.wrapping_add(24), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if (v94 > 0_i32) { 50 } else { 51 };
                }
                50 => {
                    self.w32(fp.wrapping_add(56), fp.wrapping_add(352));
                    bb = 52;
                }
                51 => {
                    v101 = self.r32(fp.wrapping_add(104));
                    v102 = (self.r32(fp.wrapping_add(352).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(
                        self.r32(fp.wrapping_add(104)).wrapping_add(1552),
                        (v102 as u32),
                    );
                    self.w32(v101.wrapping_add(1540), 2_u32);
                    self.w32(v101.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
                    self.w32(v101.wrapping_add(1544), (crem_i32(v102, 20_i32) as u32));
                    self.w32(v101.wrapping_add(1548), (cdiv_i32(v102, 20_i32) as u32));
                    return 0_i32;
                }
                52 => {
                    v96 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(56))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v97 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(56))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v98 = (v93.wrapping_add(v97).wrapping_abs() as u32);
                    v99 = (v96.wrapping_sub(v98) as i32).wrapping_abs();
                    v100 = (((self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add(v97)
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(44)) as i32)
                                .wrapping_sub(crem_i32(
                                    (self.r32(self.r32(fp.wrapping_add(56))) as i32),
                                    20_i32,
                                ))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v98.wrapping_add(v96)))
                        as i32);
                    bb = if (v100 >= (self.r32(fp.wrapping_add(20)) as i32)) {
                        55
                    } else {
                        57
                    };
                }
                53 => {
                    bb = if v44 { 52 } else { 54 };
                }
                54 => {
                    bb = 51;
                }
                55 => {
                    bb = if ((v100 == (self.r32(fp.wrapping_add(20)) as i32))
                        && (v99 < (self.r32(fp.wrapping_add(24)) as i32)))
                    {
                        58
                    } else {
                        59
                    };
                }
                56 => {
                    v44 = ({
                        let t2 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t2 as u32));
                        t2
                    } < (self.r32(fp.wrapping_add(4)) as i32));
                    self.w32(
                        fp.wrapping_add(56),
                        self.r32(fp.wrapping_add(56)).wrapping_add(4),
                    );
                    bb = 53;
                }
                57 => {
                    self.w32(fp.wrapping_add(20), (v100 as u32));
                    self.w32(fp.wrapping_add(24), (v99 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(24), (v99 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 59;
                }
                59 => {
                    bb = 56;
                }
                60 => {
                    bb = 34;
                }
                61 => {
                    v20 = ((self.r32(
                        ((self.r32(fp.wrapping_add(28)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v19) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v21 = ((((((v20 as u64) >> 32) as u32) as i64) ^ v20)
                        .wrapping_sub(((((v20 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(28)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v19) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(12)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v21 < v18) { 64 } else { 65 };
                }
                62 => {
                    bb = if (v17 < (self.r32(fp.wrapping_add(16)) as i32)) {
                        61
                    } else {
                        63
                    };
                }
                63 => {
                    v22 = self.r32(fp.wrapping_add(60));
                    v23 = 0_i32;
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(
                        fp.wrapping_add(100),
                        ((self.r32(fp.wrapping_add(152).wrapping_add(
                            ((self.r32(fp.wrapping_add(32)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(28)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(100)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(76)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(28)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(100)) as i32)),
                                )
                                .wrapping_add(3728_i32) as u32),
                        ) as i32) as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(16)) as i32));
                    v26 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(76)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(36)) as i32));
                    v27 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(40)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(16)) as i32))
                        < v27)
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v18 = v21;
                    self.w32(fp.wrapping_add(32), (v17 as u32));
                    bb = 65;
                }
                65 => {
                    v17 = v17.wrapping_add(1);
                    v19 = v19.wrapping_add(4);
                    bb = 62;
                }
                66 => {
                    v28 = (20_i32.wrapping_mul(v25) as u32);
                    self.w32(fp.wrapping_add(28), (v27.wrapping_sub(v25) as u32));
                    bb = 68;
                }
                67 => {
                    bb = if ((((((self.r32(fp.wrapping_add(108)) < 4_u32)
                        || (self.r32(fp.wrapping_add(108)) == 5_u32))
                        || (self.r32(fp.wrapping_add(108)) == 6_u32))
                        || (self.r32(fp.wrapping_add(108)) == 7_u32))
                        || (self.r32(fp.wrapping_add(108)) == 8_u32))
                        || (self.r32(fp.wrapping_add(108)) == 9_u32))
                    {
                        84
                    } else {
                        86
                    };
                }
                68 => {
                    bb = if (v24 < v26) { 71 } else { 72 };
                }
                69 => {
                    bb = if ((self.r32(fp.wrapping_add(28)) as i32) != 0) {
                        68
                    } else {
                        70
                    };
                }
                70 => {
                    bb = 67;
                }
                71 => {
                    v29 = fp
                        .wrapping_add(352)
                        .wrapping_add((v23 as u32).wrapping_mul(4));
                    bb = 73;
                }
                72 => {
                    v28 = v28.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(fp.wrapping_add(28)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 69;
                }
                73 => {
                    bb = if ((self.r32(v22) as i32) != 0) {
                        76
                    } else {
                        77
                    };
                }
                74 => {
                    bb = if (v24 < v26) { 73 } else { 75 };
                }
                75 => {
                    v23 = (self.r32(fp.wrapping_add(4)) as i32);
                    v24 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(76)) as i32));
                    bb = 72;
                }
                76 => {
                    bb = if ((v28 < 400_u32) && ((v24 as u32) < 20_u32)) {
                        78
                    } else {
                        79
                    };
                }
                77 => {
                    v26 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(76)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(36)) as i32));
                    v22 = v22.wrapping_add(4);
                    v24 = v24.wrapping_add(1);
                    bb = 74;
                }
                78 => {
                    v30 = v28.wrapping_add((v24 as u32));
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(88)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28.wrapping_add((v24 as u32)))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    bb = 77;
                }
                80 => {
                    v31 = (self.r32(
                        ((self.r32(fp.wrapping_add(88)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v30))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v31 == (1_i32).wrapping_neg())
                        || (v31 == (self.r32(fp.wrapping_add(84)) as i32).wrapping_add(5_i32)))
                    {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    self.w32(
                        {
                            let t3 = v29;
                            v29 = t3.wrapping_add(4);
                            t3
                        },
                        v30,
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 83;
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(20), 9999_u32);
                    self.w32(fp.wrapping_add(24), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if (v23 > 0_i32) { 87 } else { 88 };
                }
                85 => {
                    self.w32(fp.wrapping_add(32), (v38 as u32));
                    bb = 99;
                }
                86 => {
                    bb = if (self.r32(fp.wrapping_add(108)) != 4_u32) {
                        97
                    } else {
                        98
                    };
                }
                87 => {
                    self.w32(fp.wrapping_add(28), fp.wrapping_add(352));
                    bb = 89;
                }
                88 => {
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(fp.wrapping_add(352).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (crem_i32((self.r32(fp.wrapping_add(28)) as i32), 20_i32) as u32),
                    );
                    v38 = cdiv_i32((self.r32(fp.wrapping_add(28)) as i32), 20_i32);
                    bb = 85;
                }
                89 => {
                    v39 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v40 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v41 = (v40
                        .wrapping_add((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs() as u32);
                    v42 = (v39.wrapping_sub(v41) as i32).wrapping_abs();
                    v43 = ((v40
                        .wrapping_add((self.r32(fp.wrapping_add(12)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(44)) as i32)
                                .wrapping_sub(crem_i32(
                                    (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                                    20_i32,
                                ))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v41.wrapping_add(v39)))
                        as i32);
                    bb = if (v43 >= (self.r32(fp.wrapping_add(20)) as i32)) {
                        92
                    } else {
                        94
                    };
                }
                90 => {
                    bb = if v44 { 89 } else { 91 };
                }
                91 => {
                    bb = 88;
                }
                92 => {
                    bb = if ((v43 == (self.r32(fp.wrapping_add(20)) as i32))
                        && (v42 < (self.r32(fp.wrapping_add(24)) as i32)))
                    {
                        95
                    } else {
                        96
                    };
                }
                93 => {
                    v44 = ({
                        let t4 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t4 as u32));
                        t4
                    } < (self.r32(fp.wrapping_add(4)) as i32));
                    self.w32(
                        fp.wrapping_add(28),
                        self.r32(fp.wrapping_add(28)).wrapping_add(4),
                    );
                    bb = 90;
                }
                94 => {
                    self.w32(fp.wrapping_add(20), (v43 as u32));
                    self.w32(fp.wrapping_add(24), (v42 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 93;
                }
                95 => {
                    self.w32(fp.wrapping_add(24), (v42 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 96;
                }
                96 => {
                    bb = 93;
                }
                97 => {
                    bb = 99;
                }
                98 => {
                    self.w32(fp.wrapping_add(20), 9999_u32);
                    self.w32(fp.wrapping_add(24), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if (v23 > 0_i32) { 101 } else { 102 };
                }
                99 => {
                    bb = if ((self.r32(fp.wrapping_add(80)) as i32) <= 0_i32) {
                        113
                    } else {
                        114
                    };
                }
                100 => {
                    bb = 98;
                }
                101 => {
                    v32 = fp.wrapping_add(352);
                    bb = 103;
                }
                102 => {
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(fp.wrapping_add(352).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (crem_i32((self.r32(fp.wrapping_add(28)) as i32), 20_i32) as u32),
                    );
                    v38 = cdiv_i32((self.r32(fp.wrapping_add(28)) as i32), 20_i32);
                    bb = 85;
                }
                103 => {
                    v33 = cdiv_i32((self.r32(v32) as i32), (20_i32).wrapping_neg());
                    v34 = v33
                        .wrapping_add((self.r32(fp.wrapping_add(12)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(44)) as i32)
                                .wrapping_sub(crem_i32((self.r32(v32) as i32), 20_i32))
                                .wrapping_abs(),
                        );
                    v35 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32((self.r32(v32) as i32), 20_i32))
                        .wrapping_abs() as u32);
                    v36 = (v33
                        .wrapping_add((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs() as u32);
                    v37 = (v35.wrapping_sub(v36) as i32).wrapping_abs();
                    bb = if (v36.wrapping_add(v35) != 2_u32) {
                        106
                    } else {
                        107
                    };
                }
                104 => {
                    bb = if ((self.r32(fp.wrapping_add(8)) as i32)
                        < (self.r32(fp.wrapping_add(4)) as i32))
                    {
                        103
                    } else {
                        105
                    };
                }
                105 => {
                    bb = 102;
                }
                106 => {
                    v34 = ((v34 as u32).wrapping_add(6_u32.wrapping_mul(v36.wrapping_add(v35)))
                        as i32);
                    bb = 107;
                }
                107 => {
                    bb = if (v34 >= (self.r32(fp.wrapping_add(20)) as i32)) {
                        108
                    } else {
                        110
                    };
                }
                108 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(20)) as i32))
                        && (v37 < (self.r32(fp.wrapping_add(24)) as i32)))
                    {
                        111
                    } else {
                        112
                    };
                }
                109 => {
                    v32 = v32.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 104;
                }
                110 => {
                    self.w32(fp.wrapping_add(20), (v34 as u32));
                    self.w32(fp.wrapping_add(24), (v37 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 109;
                }
                111 => {
                    self.w32(fp.wrapping_add(24), (v37 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 112;
                }
                112 => {
                    bb = 109;
                }
                113 => {
                    v56 = (self.r32(fp.wrapping_add(44)) as i32);
                    v57 = (self.r32(fp.wrapping_add(32)) as i32);
                    v61 = (self.r32(fp.wrapping_add(88)) as i32);
                    v59 = (self.r32(fp.wrapping_add(76)) as i32);
                    v60 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 115;
                }
                114 => {
                    self.w32(fp.wrapping_add(36), 9999_u32);
                    self.w32(
                        fp.wrapping_add(92),
                        self.r32(
                            self.r32(fp.wrapping_add(96)).wrapping_add(
                                ((self.r32(fp.wrapping_add(80)) as i32).wrapping_add(3_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(92)).wrapping_add(16),
                    );
                    v45 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(fp.wrapping_add(44)) as i32)
                            .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32))
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(72)) as i32).wrapping_add(
                            (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(8)) as i32),
                        ) as u32),
                    );
                    v46 = v45.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32),
                    );
                    self.w32(fp.wrapping_add(64), (v46 as u32));
                    self.w32(fp.wrapping_add(20), 0_u32);
                    self.w32(fp.wrapping_add(24), 0_u32);
                    bb = if (self.r32(fp.wrapping_add(108)) != 0) {
                        117
                    } else {
                        118
                    };
                }
                115 => {
                    self.w32(fp.wrapping_add(56), (v56.wrapping_add(v59) as u32));
                    v62 = self.r32(fp.wrapping_add(60));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(36)) as i32)
                            .wrapping_add(v56)
                            .wrapping_add(v59) as u32),
                    );
                    v63 = v60.wrapping_add(v57);
                    bb = if (v63 < v63.wrapping_add((self.r32(fp.wrapping_add(40)) as i32))) {
                        181
                    } else {
                        182
                    };
                }
                116 => {
                    bb = 114;
                }
                117 => {
                    bb = if (self.r32(fp.wrapping_add(108)) != 2_u32) {
                        119
                    } else {
                        120
                    };
                }
                118 => {
                    bb = if (v45 >= v46) { 152 } else { 153 };
                }
                119 => {
                    bb = 121;
                }
                120 => {
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_add(
                            (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32),
                        ) as u32),
                    );
                    bb = if (v45 >= v46) { 123 } else { 124 };
                }
                121 => {
                    v56 = crem_i32((self.r32(fp.wrapping_add(24)) as i32), 20_i32);
                    v57 = cdiv_i32((self.r32(fp.wrapping_add(24)) as i32), 20_i32);
                    bb = if (self.r32(fp.wrapping_add(108)) != 0) {
                        175
                    } else {
                        177
                    };
                }
                122 => {
                    bb = 120;
                }
                123 => {
                    bb = 121;
                }
                124 => {
                    self.w32(fp.wrapping_add(12), (20_i32.wrapping_mul(v45) as u32));
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(v45) as u32),
                    );
                    bb = 126;
                }
                125 => {
                    bb = 124;
                }
                126 => {
                    bb = if (1_i32 != 0) { 127 } else { 128 };
                }
                127 => {
                    v52 = ((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32))
                        as u32);
                    bb = if ((self.r32(fp.wrapping_add(72)) as i32)
                        < (self.r32(fp.wrapping_add(0)) as i32))
                    {
                        129
                    } else {
                        130
                    };
                }
                128 => {
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(48)) as i32)
                            .wrapping_sub((self.r32(fp.wrapping_add(72)) as i32))
                            as u32),
                    );
                    bb = 136;
                }
                129 => {
                    bb = 128;
                }
                130 => {
                    bb = 132;
                }
                131 => {
                    bb = 130;
                }
                132 => {
                    v44 = ({
                        let t5 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(64)) as i32));
                    self.w32(
                        fp.wrapping_add(12),
                        self.r32(fp.wrapping_add(12)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(60)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (!v44) { 133 } else { 134 };
                }
                133 => {
                    bb = 121;
                }
                134 => {
                    bb = 126;
                }
                135 => {
                    bb = 134;
                }
                136 => {
                    bb = if (1_i32 != 0) { 137 } else { 138 };
                }
                137 => {
                    bb = if ((((!((self.r32(self.r32(fp.wrapping_add(4))) as i32) != 0))
                        || (self.r32(fp.wrapping_add(12)) >= 400_u32))
                        || (v52 >= 20_u32))
                        || (((self.r32(fp.wrapping_add(8)) as i32)
                            != (self.r32(fp.wrapping_add(16)) as i32))
                            && (v52 != ((self.r32(fp.wrapping_add(48)) as i32) as u32))))
                    {
                        139
                    } else {
                        140
                    };
                }
                138 => {
                    bb = 118;
                }
                139 => {
                    bb = 141;
                }
                140 => {
                    v53 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_abs() as u32);
                    v54 = ((self.r32(fp.wrapping_add(60)) as i32).wrapping_abs() as u32);
                    v55 = (v53.wrapping_sub(v54) as i32).wrapping_abs();
                    bb = if ((v54.wrapping_add(v53) as i32)
                        >= (self.r32(fp.wrapping_add(36)) as i32))
                    {
                        143
                    } else {
                        145
                    };
                }
                141 => {
                    v52 = v52.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if ((v52 as i32) >= (self.r32(fp.wrapping_add(0)) as i32)) {
                        149
                    } else {
                        150
                    };
                }
                142 => {
                    bb = 140;
                }
                143 => {
                    bb = if ((v54.wrapping_add(v53)
                        != ((self.r32(fp.wrapping_add(36)) as i32) as u32))
                        || (v55 >= (self.r32(fp.wrapping_add(20)) as i32)))
                    {
                        146
                    } else {
                        147
                    };
                }
                144 => {
                    self.w32(fp.wrapping_add(20), (v55 as u32));
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(12)).wrapping_add(v52) as i32) as u32),
                    );
                    bb = 141;
                }
                145 => {
                    self.w32(fp.wrapping_add(36), ((v54.wrapping_add(v53) as i32) as u32));
                    bb = 144;
                }
                146 => {
                    bb = 141;
                }
                147 => {
                    bb = 144;
                }
                148 => {
                    bb = 147;
                }
                149 => {
                    bb = 132;
                }
                150 => {
                    bb = 136;
                }
                151 => {
                    bb = 150;
                }
                152 => {
                    bb = 121;
                }
                153 => {
                    v47 = (self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32));
                    self.w32(fp.wrapping_add(12), (20_i32.wrapping_mul(v45) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(v45) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32) as u32),
                    );
                    bb = 155;
                }
                154 => {
                    bb = 153;
                }
                155 => {
                    bb = if (v47 >= (self.r32(fp.wrapping_add(0)) as i32)) {
                        158
                    } else {
                        159
                    };
                }
                156 => {
                    bb = if (!v51) { 155 } else { 157 };
                }
                157 => {
                    bb = 121;
                }
                158 => {
                    bb = 160;
                }
                159 => {
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_sub(v47) as u32),
                    );
                    bb = 162;
                }
                160 => {
                    v51 = ((self.r32(fp.wrapping_add(80)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(12),
                        self.r32(fp.wrapping_add(12)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_sub(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(fp.wrapping_add(80)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 156;
                }
                161 => {
                    bb = 159;
                }
                162 => {
                    bb = if ((((self.r32(self.r32(fp.wrapping_add(4))) as i32) != 0)
                        && (self.r32(fp.wrapping_add(12)) < 400_u32))
                        && ((v47 as u32) < 20_u32))
                    {
                        165
                    } else {
                        166
                    };
                }
                163 => {
                    bb = if (v47 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        162
                    } else {
                        164
                    };
                }
                164 => {
                    v47 = (self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32));
                    bb = 160;
                }
                165 => {
                    v48 = ((self.r32(fp.wrapping_add(64)) as i32).wrapping_abs() as u32);
                    v49 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_abs() as u32);
                    v50 = (v48.wrapping_sub(v49) as i32).wrapping_abs();
                    bb = if ((v49.wrapping_add(v48) as i32)
                        < (self.r32(fp.wrapping_add(36)) as i32))
                    {
                        167
                    } else {
                        168
                    };
                }
                166 => {
                    bb = 170;
                }
                167 => {
                    self.w32(fp.wrapping_add(36), ((v49.wrapping_add(v48) as i32) as u32));
                    bb = 169;
                }
                168 => {
                    bb = if ((v49.wrapping_add(v48)
                        == ((self.r32(fp.wrapping_add(36)) as i32) as u32))
                        && (v50 < (self.r32(fp.wrapping_add(20)) as i32)))
                    {
                        172
                    } else {
                        173
                    };
                }
                169 => {
                    self.w32(fp.wrapping_add(20), (v50 as u32));
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(12)).wrapping_add((v47 as u32)) as i32) as u32),
                    );
                    bb = 170;
                }
                170 => {
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(64)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 163;
                }
                171 => {
                    bb = 168;
                }
                172 => {
                    bb = 169;
                }
                173 => {
                    bb = 166;
                }
                174 => {
                    bb = 173;
                }
                175 => {
                    bb = if (self.r32(fp.wrapping_add(108)) == 2_u32) {
                        178
                    } else {
                        180
                    };
                }
                176 => {
                    v59 = (self.r32(v58) as i32);
                    v60 = (self.r32(v58.wrapping_add(4)) as i32);
                    v61 = (self.r32(fp.wrapping_add(88)) as i32);
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(v58.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v58.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(60), v58.wrapping_add(16));
                    bb = 115;
                }
                177 => {
                    v58 = self.r32(
                        self.r32(fp.wrapping_add(96)).wrapping_add(
                            ((self.r32(fp.wrapping_add(68)) as i32).wrapping_add(3_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(92), v58);
                    bb = 176;
                }
                178 => {
                    v58 = self.r32(
                        self.r32(fp.wrapping_add(96)).wrapping_add(
                            ((self.r32(fp.wrapping_add(68)) as i32).wrapping_add(18_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(92), v58);
                    bb = 179;
                }
                179 => {
                    bb = 176;
                }
                180 => {
                    v58 = self.r32(fp.wrapping_add(92));
                    bb = 179;
                }
                181 => {
                    v64 = (20_i32.wrapping_mul(v63) as u32);
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32),
                    );
                    bb = 183;
                }
                182 => {
                    v67 = (self.r32(fp.wrapping_add(56)) as i32);
                    v68 = ((self.r32(fp.wrapping_add(76)) as i32) & 15_i32);
                    v69 = (self
                        .r32(fp.wrapping_add(56))
                        .wrapping_add(((self.r32(fp.wrapping_add(36)) as i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(80), (16_i32.wrapping_mul(v68) as u32));
                    self.w32(fp.wrapping_add(4), self.r32(fp.wrapping_add(60)));
                    self.w32(fp.wrapping_add(76), (v68 as u32));
                    self.w32((v61.wrapping_add(7400_i32) as u32), 0_u32);
                    bb = if (v63 < v63.wrapping_add((self.r32(fp.wrapping_add(40)) as i32))) {
                        196
                    } else {
                        197
                    };
                }
                183 => {
                    i = self.r32(fp.wrapping_add(56));
                    bb = 186;
                }
                184 => {
                    bb = if ((self.r32(fp.wrapping_add(68)) as i32) != 0) {
                        183
                    } else {
                        185
                    };
                }
                185 => {
                    bb = 182;
                }
                186 => {
                    bb = if ((i as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        187
                    } else {
                        189
                    };
                }
                187 => {
                    bb = if ((self.r32(v62) as i32) != 0) {
                        190
                    } else {
                        191
                    };
                }
                188 => {
                    i = i.wrapping_add(1);
                    bb = 186;
                }
                189 => {
                    v64 = v64.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(68)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 184;
                }
                190 => {
                    bb = if ((v64 < 400_u32) && (i < 20_u32)) {
                        192
                    } else {
                        193
                    };
                }
                191 => {
                    v62 = v62.wrapping_add(4);
                    bb = 188;
                }
                192 => {
                    v66 = (self.r32(
                        (v61 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v64.wrapping_add(i)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v66 != (1_i32).wrapping_neg())
                        && ((v66.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(100))))
                    {
                        194
                    } else {
                        195
                    };
                }
                193 => {
                    bb = 191;
                }
                194 => {
                    self.w32(fp.wrapping_add(76), ((self.r32(v62) as i32) as u32));
                    bb = 195;
                }
                195 => {
                    bb = 193;
                }
                196 => {
                    v70 = (20_i32.wrapping_mul(v63) as u32);
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32),
                    );
                    bb = 198;
                }
                197 => {
                    bb = if (v68 != 15_i32) { 239 } else { 240 };
                }
                198 => {
                    bb = if (1_i32 != 0) { 199 } else { 200 };
                }
                199 => {
                    bb = if (v67 >= v69) { 201 } else { 202 };
                }
                200 => {
                    bb = 197;
                }
                201 => {
                    bb = 203;
                }
                202 => {
                    bb = 205;
                }
                203 => {
                    v70 = v70.wrapping_add(20_u32);
                    bb = if (!({
                        let t6 = (self.r32(fp.wrapping_add(68)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(68), (t6 as u32));
                        t6
                    } != 0))
                    {
                        236
                    } else {
                        237
                    };
                }
                204 => {
                    bb = 202;
                }
                205 => {
                    bb = if (((self.r32(fp.wrapping_add(80)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(4))) as i32))
                        == 0_i32)
                    {
                        208
                    } else {
                        209
                    };
                }
                206 => {
                    bb = if (v67
                        < (self
                            .r32(fp.wrapping_add(56))
                            .wrapping_add(((self.r32(fp.wrapping_add(36)) as i32) as u32))
                            as i32))
                    {
                        205
                    } else {
                        207
                    };
                }
                207 => {
                    v67 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 203;
                }
                208 => {
                    bb = 210;
                }
                209 => {
                    bb = if (v70 >= 400_u32) { 212 } else { 213 };
                }
                210 => {
                    v69 = (self
                        .r32(fp.wrapping_add(56))
                        .wrapping_add(((self.r32(fp.wrapping_add(36)) as i32) as u32))
                        as i32);
                    v67 = v67.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    bb = 206;
                }
                211 => {
                    bb = 209;
                }
                212 => {
                    bb = 210;
                }
                213 => {
                    bb = if ((v67 as u32) >= 20_u32) { 215 } else { 216 };
                }
                214 => {
                    bb = 213;
                }
                215 => {
                    bb = 210;
                }
                216 => {
                    v71 = v70.wrapping_add((v67 as u32));
                    bb = if (v70.wrapping_add((v67 as u32))
                        == ((self.r32(fp.wrapping_add(28)) as i32) as u32))
                    {
                        218
                    } else {
                        219
                    };
                }
                217 => {
                    bb = 216;
                }
                218 => {
                    bb = 210;
                }
                219 => {
                    v72 = (self.r32(
                        (v61 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v71))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v73 = 0_i8;
                    bb = if ((self.r32(fp.wrapping_add(116)) as i32) != 0) {
                        221
                    } else {
                        223
                    };
                }
                220 => {
                    bb = 219;
                }
                221 => {
                    bb = if ((self.r32(fp.wrapping_add(116)) as i32) == 1_i32) {
                        224
                    } else {
                        225
                    };
                }
                222 => {
                    self.w32(
                        (v61.wrapping_add(7400_i32) as u32),
                        ((self.r32((v61.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if ((v73 as i32) == 1_i32) { 232 } else { 233 };
                }
                223 => {
                    self.w32(
                        (v61.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v61.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v71 as i32) as u32),
                    );
                    bb = if ((v72 != (1_i32).wrapping_neg())
                        && ((v72.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(100))))
                    {
                        230
                    } else {
                        231
                    };
                }
                224 => {
                    self.w32(
                        (v61.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v61.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v71.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v72 > 4_i32) { 226 } else { 227 };
                }
                225 => {
                    bb = 222;
                }
                226 => {
                    bb = 228;
                }
                227 => {
                    bb = 225;
                }
                228 => {
                    v73 = 1_i8;
                    bb = 231;
                }
                229 => {
                    bb = 227;
                }
                230 => {
                    self.w32(
                        (v61.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v61.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v71.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = 228;
                }
                231 => {
                    bb = 222;
                }
                232 => {
                    v74 = v72.wrapping_sub(5_i32);
                    bb = if (v74 != (self.r32(fp.wrapping_add(84)) as i32)) {
                        234
                    } else {
                        235
                    };
                }
                233 => {
                    bb = 210;
                }
                234 => {
                    let _ = self.f_10009a40(
                        self.r32(fp.wrapping_add(96)),
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v74,
                        fp.wrapping_add(120),
                        (fp.wrapping_add(140) as i32),
                    );
                    bb = 235;
                }
                235 => {
                    bb = 233;
                }
                236 => {
                    v68 = (self.r32(fp.wrapping_add(76)) as i32);
                    bb = 200;
                }
                237 => {
                    bb = 198;
                }
                238 => {
                    bb = 237;
                }
                239 => {
                    let t7 = v68;
                    bb = match t7 {
                        1_i32 => 242,
                        2_i32 => 243,
                        4_i32 => 244,
                        8_i32 => 245,
                        _ => 246,
                    };
                }
                240 => {
                    v75 = (self.r32(
                        (v61.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v75 == (1_i32).wrapping_neg()) {
                        254
                    } else {
                        255
                    };
                }
                241 => {
                    bb = 252;
                }
                242 => {
                    v75 = 0_i32;
                    bb = 241;
                }
                243 => {
                    v75 = 1_i32;
                    bb = 241;
                }
                244 => {
                    v75 = 2_i32;
                    bb = 241;
                }
                245 => {
                    v75 = 3_i32;
                    bb = 241;
                }
                246 => {
                    v75 = (self.r32(fp.wrapping_add(116)) as i32);
                    bb = 241;
                }
                247 => {
                    bb = 243;
                }
                248 => {
                    bb = 244;
                }
                249 => {
                    bb = 245;
                }
                250 => {
                    bb = 246;
                }
                251 => {
                    bb = 241;
                }
                252 => {
                    v80 = (self.r32(
                        (v61.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v80 == (1_i32).wrapping_neg()) {
                        281
                    } else {
                        282
                    };
                }
                253 => {
                    bb = 240;
                }
                254 => {
                    v75 = (self.r32(fp.wrapping_add(112)) as i32);
                    bb = 255;
                }
                255 => {
                    v76 = (self.r32(fp.wrapping_add(16)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(32)) as i32));
                    v77 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v78 = (self.r32(fp.wrapping_add(16)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(32)) as i32))
                        .wrapping_abs();
                    v79 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    bb = if (v79 != v78) { 256 } else { 257 };
                }
                256 => {
                    bb = if (v79 < v78) { 258 } else { 259 };
                }
                257 => {
                    bb = if (v75 != 0) { 263 } else { 265 };
                }
                258 => {
                    v75 = (if (v76 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 252;
                }
                259 => {
                    bb = 261;
                }
                260 => {
                    bb = 259;
                }
                261 => {
                    v75 = (if (v77 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 252;
                }
                262 => {
                    bb = 257;
                }
                263 => {
                    bb = if (v75 == 1_i32) { 266 } else { 268 };
                }
                264 => {
                    bb = 252;
                }
                265 => {
                    bb = if (v76 > 0_i32) { 279 } else { 280 };
                }
                266 => {
                    bb = if (v77 < 0_i32) { 269 } else { 270 };
                }
                267 => {
                    bb = 264;
                }
                268 => {
                    bb = if (v75 == 2_i32) { 271 } else { 272 };
                }
                269 => {
                    v75 = (if (v76 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 270;
                }
                270 => {
                    bb = 267;
                }
                271 => {
                    bb = if (v76 >= 0_i32) { 273 } else { 274 };
                }
                272 => {
                    bb = if ((v75 == 3_i32) && (v77 > 0_i32)) {
                        277
                    } else {
                        278
                    };
                }
                273 => {
                    bb = 252;
                }
                274 => {
                    bb = 261;
                }
                275 => {
                    bb = 274;
                }
                276 => {
                    bb = 272;
                }
                277 => {
                    v75 = (if (v76 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 278;
                }
                278 => {
                    bb = 267;
                }
                279 => {
                    v75 = (if (v77 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 280;
                }
                280 => {
                    bb = 264;
                }
                281 => {
                    v80 = (self.r32(fp.wrapping_add(112)) as i32);
                    bb = 282;
                }
                282 => {
                    bb = if (v75 == v80) { 283 } else { 284 };
                }
                283 => {
                    v75 = (1_i32).wrapping_neg();
                    bb = 284;
                }
                284 => {
                    v81 = self.r32(fp.wrapping_add(104));
                    self.w32(
                        self.r32(fp.wrapping_add(104)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        v81.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(32)) as i32) as u32),
                    );
                    self.w32(v81.wrapping_add(1540), 2_u32);
                    self.w32(
                        v81.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(v81.wrapping_add(1556), (v75 as u32));
                    return 0_i32;
                }
                285 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10019340` (2583 bytes).
    pub(crate) fn f_10019340(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_10019340_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_10019340_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(92), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(52), (v4 as u32));
                    self.w32(fp.wrapping_add(84), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(216),
                        (fp.wrapping_add(104) as i32),
                        (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(100), 0_i32, 0_i32, 3_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(100))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(100)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(32), fp.wrapping_add(236));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(116));
                    v7 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(39), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w8(fp.wrapping_add(31), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(39)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(39), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(32));
                            self.w32(fp.wrapping_add(32), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(116);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(116);
                    bb = 151;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(76));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(116).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(32), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(336)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(32), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(336));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    v37 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(336).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    v39 = crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    v40 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(24), (v44 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v45 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        86
                    } else {
                        87
                    };
                }
                70 => {
                    v42 = (v39 as u32);
                    bb = if (v39 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v43 != (1_i32).wrapping_neg()) {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    bb = if (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)) {
                        84
                    } else {
                        85
                    };
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v37) as i32) as u32));
                    bb = 85;
                }
                85 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 83;
                }
                86 => {
                    v46 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 88;
                }
                87 => {
                    bb = if (v44 != 15_i32) { 104 } else { 105 };
                }
                88 => {
                    v47 = (v39 as u32);
                    bb = if (v39 < v45) { 91 } else { 92 };
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v44 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 87;
                }
                91 => {
                    bb = 93;
                }
                92 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                93 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                95 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 92;
                }
                96 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 94;
                }
                98 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        100
                    } else {
                        101
                    };
                }
                99 => {
                    bb = 97;
                }
                100 => {
                    v50 = (v49 >= 5_i32);
                    bb = 101;
                }
                101 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 102 } else { 103 };
                }
                102 => {
                    let _ = self.f_10009c80(
                        this,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v49.wrapping_sub(5_i32),
                        55_i32,
                    );
                    bb = 103;
                }
                103 => {
                    bb = 99;
                }
                104 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 107,
                        2_i32 => 108,
                        4_i32 => 109,
                        8_i32 => 110,
                        _ => 111,
                    };
                }
                105 => {
                    v51 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        119
                    } else {
                        120
                    };
                }
                106 => {
                    bb = 117;
                }
                107 => {
                    v51 = 0_i32;
                    bb = 106;
                }
                108 => {
                    v51 = 1_i32;
                    bb = 106;
                }
                109 => {
                    v51 = 2_i32;
                    bb = 106;
                }
                110 => {
                    v51 = 3_i32;
                    bb = 106;
                }
                111 => {
                    v51 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 106;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 110;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    v56 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        141
                    } else {
                        142
                    };
                }
                118 => {
                    bb = 105;
                }
                119 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 120;
                }
                120 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 121 } else { 123 };
                }
                121 => {
                    bb = if (!(v51 != 0)) { 124 } else { 125 };
                }
                122 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 117;
                }
                123 => {
                    bb = if (v55 < v54) { 147 } else { 148 };
                }
                124 => {
                    bb = if (v52 > 0_i32) { 126 } else { 127 };
                }
                125 => {
                    bb = if (v51 == 1_i32) { 129 } else { 130 };
                }
                126 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 127;
                }
                127 => {
                    bb = 117;
                }
                128 => {
                    bb = 125;
                }
                129 => {
                    bb = if (v53 < 0_i32) { 131 } else { 132 };
                }
                130 => {
                    bb = if (v51 != 2_i32) { 134 } else { 135 };
                }
                131 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 132;
                }
                132 => {
                    bb = 117;
                }
                133 => {
                    bb = 130;
                }
                134 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        136
                    } else {
                        137
                    };
                }
                135 => {
                    bb = if (v52 >= 0_i32) { 139 } else { 140 };
                }
                136 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 137;
                }
                137 => {
                    bb = 117;
                }
                138 => {
                    bb = 135;
                }
                139 => {
                    bb = 117;
                }
                140 => {
                    bb = 122;
                }
                141 => {
                    v56 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 142;
                }
                142 => {
                    bb = if (v51 == v56) { 143 } else { 144 };
                }
                143 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 144;
                }
                144 => {
                    v57 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 145;
                }
                145 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1472), 0_u32);
                    self.w32(v57.wrapping_add(1476), 1_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                146 => {
                    bb = 140;
                }
                147 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 117;
                }
                148 => {
                    bb = 122;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 31;
                }
                151 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v60 < 20_i32) { 151 } else { 153 };
                }
                153 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 155;
                }
                155 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 152;
                }
                156 => {
                    v64 = fp.wrapping_add(116);
                    bb = 158;
                }
                157 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(116).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(116).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 163 } else { 164 };
                }
                158 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 161 } else { 162 };
                }
                159 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                160 => {
                    bb = 157;
                }
                161 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 162;
                }
                162 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 159;
                }
                163 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(336),
                        self.r32(fp.wrapping_add(92)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 164;
                }
                164 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 165 } else { 166 };
                }
                165 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(336));
                    bb = 167;
                }
                166 => {
                    v75 = (self.r32(fp.wrapping_add(336).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(72));
                    bb = 145;
                }
                167 => {
                    v70 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        170
                    } else {
                        172
                    };
                }
                168 => {
                    bb = if v35 { 167 } else { 169 };
                }
                169 => {
                    bb = 166;
                }
                170 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                171 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 168;
                }
                172 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 171;
                }
                173 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 174;
                }
                174 => {
                    bb = 171;
                }
                175 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10019D80` (2587 bytes).
    pub(crate) fn f_10019d80(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_10019d80_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_10019d80_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(92), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(52), (v4 as u32));
                    self.w32(fp.wrapping_add(84), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(216),
                        (fp.wrapping_add(104) as i32),
                        (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(100), 0_i32, 0_i32, 4_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(100))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(100)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(32), fp.wrapping_add(236));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(116));
                    v7 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(39), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w8(fp.wrapping_add(31), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(39)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(39), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(32));
                            self.w32(fp.wrapping_add(32), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(116);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(116);
                    bb = 151;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(76));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(116).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(32), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(336)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(32), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(336));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    v37 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(336).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    v39 = crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    v40 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(24), (v44 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v45 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        86
                    } else {
                        87
                    };
                }
                70 => {
                    v42 = (v39 as u32);
                    bb = if (v39 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v43 != (1_i32).wrapping_neg()) {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    bb = if (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)) {
                        84
                    } else {
                        85
                    };
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v37) as i32) as u32));
                    bb = 85;
                }
                85 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 83;
                }
                86 => {
                    v46 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 88;
                }
                87 => {
                    bb = if (v44 != 15_i32) { 104 } else { 105 };
                }
                88 => {
                    v47 = (v39 as u32);
                    bb = if (v39 < v45) { 91 } else { 92 };
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v44 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 87;
                }
                91 => {
                    bb = 93;
                }
                92 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                93 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                95 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 92;
                }
                96 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 94;
                }
                98 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        100
                    } else {
                        101
                    };
                }
                99 => {
                    bb = 97;
                }
                100 => {
                    v50 = (v49 >= 5_i32);
                    bb = 101;
                }
                101 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 102 } else { 103 };
                }
                102 => {
                    let _ = self.f_10009c80(
                        this,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v49.wrapping_sub(5_i32),
                        56_i32,
                    );
                    bb = 103;
                }
                103 => {
                    bb = 99;
                }
                104 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 107,
                        2_i32 => 108,
                        4_i32 => 109,
                        8_i32 => 110,
                        _ => 111,
                    };
                }
                105 => {
                    v51 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        119
                    } else {
                        120
                    };
                }
                106 => {
                    bb = 117;
                }
                107 => {
                    v51 = 0_i32;
                    bb = 106;
                }
                108 => {
                    v51 = 1_i32;
                    bb = 106;
                }
                109 => {
                    v51 = 2_i32;
                    bb = 106;
                }
                110 => {
                    v51 = 3_i32;
                    bb = 106;
                }
                111 => {
                    v51 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 106;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 110;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    v56 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        141
                    } else {
                        142
                    };
                }
                118 => {
                    bb = 105;
                }
                119 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 120;
                }
                120 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 121 } else { 123 };
                }
                121 => {
                    bb = if (!(v51 != 0)) { 124 } else { 125 };
                }
                122 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 117;
                }
                123 => {
                    bb = if (v55 < v54) { 147 } else { 148 };
                }
                124 => {
                    bb = if (v52 > 0_i32) { 126 } else { 127 };
                }
                125 => {
                    bb = if (v51 == 1_i32) { 129 } else { 130 };
                }
                126 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 127;
                }
                127 => {
                    bb = 117;
                }
                128 => {
                    bb = 125;
                }
                129 => {
                    bb = if (v53 < 0_i32) { 131 } else { 132 };
                }
                130 => {
                    bb = if (v51 != 2_i32) { 134 } else { 135 };
                }
                131 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 132;
                }
                132 => {
                    bb = 117;
                }
                133 => {
                    bb = 130;
                }
                134 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        136
                    } else {
                        137
                    };
                }
                135 => {
                    bb = if (v52 >= 0_i32) { 139 } else { 140 };
                }
                136 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 137;
                }
                137 => {
                    bb = 117;
                }
                138 => {
                    bb = 135;
                }
                139 => {
                    bb = 117;
                }
                140 => {
                    bb = 122;
                }
                141 => {
                    v56 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 142;
                }
                142 => {
                    bb = if (v51 == v56) { 143 } else { 144 };
                }
                143 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 144;
                }
                144 => {
                    v57 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 145;
                }
                145 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1472), 0_u32);
                    self.w32(v57.wrapping_add(1476), 3_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                146 => {
                    bb = 140;
                }
                147 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 117;
                }
                148 => {
                    bb = 122;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 31;
                }
                151 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v60 < 20_i32) { 151 } else { 153 };
                }
                153 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 155;
                }
                155 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 152;
                }
                156 => {
                    v64 = fp.wrapping_add(116);
                    bb = 158;
                }
                157 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(116).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(116).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 163 } else { 164 };
                }
                158 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 161 } else { 162 };
                }
                159 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                160 => {
                    bb = 157;
                }
                161 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 162;
                }
                162 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 159;
                }
                163 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(336),
                        self.r32(fp.wrapping_add(92)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 164;
                }
                164 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 165 } else { 166 };
                }
                165 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(336));
                    bb = 167;
                }
                166 => {
                    v75 = (self.r32(fp.wrapping_add(336).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(72));
                    bb = 145;
                }
                167 => {
                    v70 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        170
                    } else {
                        172
                    };
                }
                168 => {
                    bb = if v35 { 167 } else { 169 };
                }
                169 => {
                    bb = 166;
                }
                170 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                171 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 168;
                }
                172 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 171;
                }
                173 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 174;
                }
                174 => {
                    bb = 171;
                }
                175 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001A7C0` (2587 bytes).
    pub(crate) fn f_1001a7c0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_1001a7c0_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_1001a7c0_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(92), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(52), (v4 as u32));
                    self.w32(fp.wrapping_add(84), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(216),
                        (fp.wrapping_add(104) as i32),
                        (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(100), 0_i32, 0_i32, 8_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(100))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(100)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(32), fp.wrapping_add(236));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(116));
                    v7 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(39), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w8(fp.wrapping_add(31), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(39)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(39), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(32));
                            self.w32(fp.wrapping_add(32), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(116);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(116);
                    bb = 151;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(76));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(116).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(32), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(336)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(32), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(336));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    v37 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(336).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    v39 = crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    v40 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(24), (v44 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v45 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        86
                    } else {
                        87
                    };
                }
                70 => {
                    v42 = (v39 as u32);
                    bb = if (v39 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v43 != (1_i32).wrapping_neg()) {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    bb = if (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)) {
                        84
                    } else {
                        85
                    };
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v37) as i32) as u32));
                    bb = 85;
                }
                85 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 83;
                }
                86 => {
                    v46 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 88;
                }
                87 => {
                    bb = if (v44 != 15_i32) { 104 } else { 105 };
                }
                88 => {
                    v47 = (v39 as u32);
                    bb = if (v39 < v45) { 91 } else { 92 };
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v44 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 87;
                }
                91 => {
                    bb = 93;
                }
                92 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                93 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                95 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 92;
                }
                96 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 94;
                }
                98 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        100
                    } else {
                        101
                    };
                }
                99 => {
                    bb = 97;
                }
                100 => {
                    v50 = (v49 >= 5_i32);
                    bb = 101;
                }
                101 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 102 } else { 103 };
                }
                102 => {
                    let _ = self.f_10009c80(
                        this,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v49.wrapping_sub(5_i32),
                        57_i32,
                    );
                    bb = 103;
                }
                103 => {
                    bb = 99;
                }
                104 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 107,
                        2_i32 => 108,
                        4_i32 => 109,
                        8_i32 => 110,
                        _ => 111,
                    };
                }
                105 => {
                    v51 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        119
                    } else {
                        120
                    };
                }
                106 => {
                    bb = 117;
                }
                107 => {
                    v51 = 0_i32;
                    bb = 106;
                }
                108 => {
                    v51 = 1_i32;
                    bb = 106;
                }
                109 => {
                    v51 = 2_i32;
                    bb = 106;
                }
                110 => {
                    v51 = 3_i32;
                    bb = 106;
                }
                111 => {
                    v51 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 106;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 110;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    v56 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        141
                    } else {
                        142
                    };
                }
                118 => {
                    bb = 105;
                }
                119 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 120;
                }
                120 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 121 } else { 123 };
                }
                121 => {
                    bb = if (!(v51 != 0)) { 124 } else { 125 };
                }
                122 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 117;
                }
                123 => {
                    bb = if (v55 < v54) { 147 } else { 148 };
                }
                124 => {
                    bb = if (v52 > 0_i32) { 126 } else { 127 };
                }
                125 => {
                    bb = if (v51 == 1_i32) { 129 } else { 130 };
                }
                126 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 127;
                }
                127 => {
                    bb = 117;
                }
                128 => {
                    bb = 125;
                }
                129 => {
                    bb = if (v53 < 0_i32) { 131 } else { 132 };
                }
                130 => {
                    bb = if (v51 != 2_i32) { 134 } else { 135 };
                }
                131 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 132;
                }
                132 => {
                    bb = 117;
                }
                133 => {
                    bb = 130;
                }
                134 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        136
                    } else {
                        137
                    };
                }
                135 => {
                    bb = if (v52 >= 0_i32) { 139 } else { 140 };
                }
                136 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 137;
                }
                137 => {
                    bb = 117;
                }
                138 => {
                    bb = 135;
                }
                139 => {
                    bb = 117;
                }
                140 => {
                    bb = 122;
                }
                141 => {
                    v56 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 142;
                }
                142 => {
                    bb = if (v51 == v56) { 143 } else { 144 };
                }
                143 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 144;
                }
                144 => {
                    v57 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 145;
                }
                145 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1472), 0_u32);
                    self.w32(v57.wrapping_add(1476), 9_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                146 => {
                    bb = 140;
                }
                147 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 117;
                }
                148 => {
                    bb = 122;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 31;
                }
                151 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v60 < 20_i32) { 151 } else { 153 };
                }
                153 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 155;
                }
                155 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 152;
                }
                156 => {
                    v64 = fp.wrapping_add(116);
                    bb = 158;
                }
                157 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(116).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(116).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 163 } else { 164 };
                }
                158 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 161 } else { 162 };
                }
                159 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                160 => {
                    bb = 157;
                }
                161 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 162;
                }
                162 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 159;
                }
                163 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(336),
                        self.r32(fp.wrapping_add(92)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 164;
                }
                164 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 165 } else { 166 };
                }
                165 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(336));
                    bb = 167;
                }
                166 => {
                    v75 = (self.r32(fp.wrapping_add(336).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(72));
                    bb = 145;
                }
                167 => {
                    v70 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        170
                    } else {
                        172
                    };
                }
                168 => {
                    bb = if v35 { 167 } else { 169 };
                }
                169 => {
                    bb = 166;
                }
                170 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                171 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 168;
                }
                172 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 171;
                }
                173 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 174;
                }
                174 => {
                    bb = 171;
                }
                175 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001B200` (2681 bytes).
    pub(crate) fn f_1001b200(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1952);
        let r = self.f_1001b200_body(fp, this, a2);
        self.leave(1952);
        r
    }

    fn f_1001b200_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: u32 = 0;
        let mut v16: u32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: bool = false;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: u32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: i32 = 0;
        let mut v32: i32 = 0;
        let mut v33: i32 = 0;
        let mut v34: u32 = 0;
        let mut v35: u32 = 0;
        let mut v36: u32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v39: u32 = 0;
        let mut v40: u32 = 0;
        let mut v41: i32 = 0;
        let mut v42: i32 = 0;
        let mut v43: i32 = 0;
        let mut v44: u32 = 0;
        let mut v45: u32 = 0;
        let mut v46: i32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: u32 = 0;
        let mut v50: i32 = 0;
        let mut v51: i8 = 0;
        let mut v52: i32 = 0;
        let mut v53: u32 = 0;
        let mut v54: u32 = 0;
        let mut v55: u32 = 0;
        let mut v56: u32 = 0;
        let mut v57: i32 = 0;
        let mut v58: bool = false;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: i32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: i32 = 0;
        let mut v65: u32 = 0;
        let mut v66: u32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: u32 = 0;
        let mut v70: i32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut v77: u32 = 0;
        let mut v78: i32 = 0;
        let mut v79: u32 = 0;
        let mut v80: i32 = 0;
        let mut v81: i32 = 0;
        let mut v82: u32 = 0;
        let mut v83: i32 = 0;
        let mut v84: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = self.r32(this);
                    self.w32(fp.wrapping_add(104), this);
                    v4 = ((self.r32(v3) as i32) as u32);
                    v5 = (self.r32(v4.wrapping_add(12)) as i32);
                    v6 = (self.r32(v4.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(v4.wrapping_add(28)) as i32) as u32),
                    );
                    v7 = (self.r32(self.r32(fp.wrapping_add(80)).wrapping_add(1388)) as i32);
                    v8 = (self.r32(self.r32(fp.wrapping_add(80)).wrapping_add(1384)) as i32);
                    self.w32(fp.wrapping_add(108), (v6.wrapping_add(4840_i32) as u32));
                    self.w32(fp.wrapping_add(12), (v5 as u32));
                    self.w32(fp.wrapping_add(56), (v6 as u32));
                    self.w32(fp.wrapping_add(92), (v7 as u32));
                    v9 = (v5.wrapping_add(716_i32.wrapping_mul(v7)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v9.wrapping_add(380)) as i32) as u32),
                    );
                    v10 = (self.r32(v9.wrapping_add(384)) as i32);
                    v11 = (self.r32(v9.wrapping_add(388)) as i32);
                    self.w32(fp.wrapping_add(64), (v10 as u32));
                    self.w32(fp.wrapping_add(100), (v11 as u32));
                    let _ = self.f_100064e0(
                        self.r32(fp.wrapping_add(104)),
                        fp.wrapping_add(232),
                        (fp.wrapping_add(120) as i32),
                        v8,
                        v7,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(116), 0_i32, 3_i32, 1_i32, 1_i32);
                    v12 = (self.r32(self.r32(fp.wrapping_add(116)).wrapping_add(4)) as i32);
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(self.r32(fp.wrapping_add(116))) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(28), (v12 as u32));
                    v13 = (self.r32(self.r32(fp.wrapping_add(116)).wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(self.r32(fp.wrapping_add(116)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(72),
                        self.r32(fp.wrapping_add(116)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(40), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    self.w32(fp.wrapping_add(84), (v13 as u32));
                    self.w32(fp.wrapping_add(112), (v5.wrapping_add(3600_i32) as u32));
                    self.w32(fp.wrapping_add(0), (v5.wrapping_add(3600_i32) as u32));
                    self.w32(fp.wrapping_add(36), fp.wrapping_add(252));
                    self.w32(fp.wrapping_add(68), fp.wrapping_add(132));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(self.r32(fp.wrapping_add(0))) as i32) == 1_i32)
                        && ((self.r32(self.r32(fp.wrapping_add(0)).wrapping_add(116)) as i32)
                            != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if v21 { 1 } else { 3 };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(40)) as i32) > 0_i32) {
                        33
                    } else {
                        34
                    };
                }
                4 => {
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(self.r32(fp.wrapping_add(0)).wrapping_add(124)) as i32) as u32),
                    );
                    v14 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                        as u32);
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(self.r32(fp.wrapping_add(0)).wrapping_add(128)) as i32) as u32),
                    );
                    v15 = self.r32(fp.wrapping_add(72));
                    self.w8(fp.wrapping_add(35), 0_u8);
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(48)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(76)) as i32))
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(96),
                        ((self.r32(fp.wrapping_add(52)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(28)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(84)) as i32))
                            as u32),
                    );
                    self.w8(fp.wrapping_add(27), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(52)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(28)) as i32))
                            as u32),
                    );
                    bb = if ((self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(28)) as i32))
                        < (self.r32(fp.wrapping_add(96)) as i32))
                    {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v21 = ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1_i32) < 20_i32);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(156),
                    );
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v16 = (20_i32.wrapping_mul(
                        (self.r32(fp.wrapping_add(52)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(28)) as i32)),
                    ) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    bb = if (1_i32 != 0) { 9 } else { 10 };
                }
                9 => {
                    v17 = v14;
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                        < (self.r32(fp.wrapping_add(4)) as i32))
                    {
                        11
                    } else {
                        12
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(35)) as i8) as i32) == 1_i32) {
                        29
                    } else {
                        30
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v16 = v16.wrapping_add(20_u32);
                    bb = if ({
                        let t1 = (self.r32(fp.wrapping_add(20)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(20), (t1 as u32));
                        t1
                    } >= (self.r32(fp.wrapping_add(96)) as i32))
                    {
                        26
                    } else {
                        27
                    };
                }
                13 => {
                    bb = if ((self.r32(v15) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v17 as i32) < (self.r32(fp.wrapping_add(4)) as i32)) {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v16 < 400_u32) && (v17 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v15 = v15.wrapping_add(4);
                    v17 = v17.wrapping_add(1);
                    bb = 14;
                }
                18 => {
                    v18 = (4_u32.wrapping_mul(v16.wrapping_add(v17)) as i32);
                    bb = if ((self.r32((v18.wrapping_add(v6) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v19 = (self.r32((v18.wrapping_add(v6).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v19 == (1_i32).wrapping_neg()) || (v19 == v7)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(35), 1_u8);
                    bb = if ((v17 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(20)) as i32)
                            == (self.r32(fp.wrapping_add(64)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(27), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    bb = 10;
                }
                27 => {
                    v14 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                        as u32);
                    bb = 8;
                }
                28 => {
                    bb = 27;
                }
                29 => {
                    v20 = self.r32(fp.wrapping_add(68));
                    self.w32(
                        self.r32(fp.wrapping_add(68)),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(fp.wrapping_add(68), v20.wrapping_add(4));
                    bb = 30;
                }
                30 => {
                    bb = if (((self.r8(fp.wrapping_add(27)) as i8) as i32) == 1_i32) {
                        31
                    } else {
                        32
                    };
                }
                31 => {
                    self.w32(
                        self.r32(fp.wrapping_add(36)),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        self.r32(fp.wrapping_add(36)).wrapping_add(4),
                    );
                    bb = 32;
                }
                32 => {
                    bb = 7;
                }
                33 => {
                    v22 = 0_i32;
                    v23 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v24 = fp.wrapping_add(132);
                    bb = 35;
                }
                34 => {
                    v66 = self.r32(fp.wrapping_add(112));
                    v67 = 0_i32;
                    v68 = 0_i32;
                    v69 = fp.wrapping_add(132);
                    bb = 154;
                }
                35 => {
                    v25 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(v24) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(12)) as i32)
                                .wrapping_add(156_i32.wrapping_mul((self.r32(v24) as i32)))
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32((v25.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v26 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(52)) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(64)) as i32))
                                .wrapping_abs(),
                        );
                    bb = if (v26 < v23) { 38 } else { 39 };
                }
                36 => {
                    bb = if (v22 < (self.r32(fp.wrapping_add(40)) as i32)) {
                        35
                    } else {
                        37
                    };
                }
                37 => {
                    v27 = (self.r32(fp.wrapping_add(12)) as i32);
                    v28 = self.r32(fp.wrapping_add(72));
                    v29 = 0_i32;
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(fp.wrapping_add(132).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(12), 0_u32);
                    v30 = v27
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(88)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            (v27.wrapping_add(
                                156_i32.wrapping_mul((self.r32(fp.wrapping_add(88)) as i32)),
                            )
                            .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v31 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(60)) as i32));
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32((v30.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v32 = (self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(52)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(48)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(48)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(76)) as i32))
                            as u32),
                    );
                    v33 = (self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(52)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(84)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(52)) as i32))
                        < v33)
                    {
                        40
                    } else {
                        41
                    };
                }
                38 => {
                    v23 = v26;
                    self.w32(fp.wrapping_add(0), (v22 as u32));
                    bb = 39;
                }
                39 => {
                    v22 = v22.wrapping_add(1);
                    v24 = v24.wrapping_add(4);
                    bb = 36;
                }
                40 => {
                    v34 = (20_i32.wrapping_mul(v32) as u32);
                    self.w32(fp.wrapping_add(36), (v33.wrapping_sub(v32) as u32));
                    bb = 42;
                }
                41 => {
                    self.w32(fp.wrapping_add(4), 9999_u32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if (v29 > 0_i32) { 61 } else { 62 };
                }
                42 => {
                    bb = if (1_i32 != 0) { 43 } else { 44 };
                }
                43 => {
                    bb = if (v31 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        45
                    } else {
                        46
                    };
                }
                44 => {
                    self.w32(fp.wrapping_add(12), (v29 as u32));
                    bb = 41;
                }
                45 => {
                    v35 = fp
                        .wrapping_add(352)
                        .wrapping_add((v29 as u32).wrapping_mul(4));
                    bb = 47;
                }
                46 => {
                    v34 = v34.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(36)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(36), (t2 as u32));
                        t2
                    } != 0))
                    {
                        58
                    } else {
                        59
                    };
                }
                47 => {
                    bb = if ((self.r32(v28) as i32) != 0) {
                        50
                    } else {
                        51
                    };
                }
                48 => {
                    bb = if (v31 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        47
                    } else {
                        49
                    };
                }
                49 => {
                    bb = 46;
                }
                50 => {
                    bb = if ((v34 < 400_u32) && ((v31 as u32) < 20_u32)) {
                        52
                    } else {
                        53
                    };
                }
                51 => {
                    v28 = v28.wrapping_add(4);
                    v31 = v31.wrapping_add(1);
                    bb = 48;
                }
                52 => {
                    v36 = v34.wrapping_add((v31 as u32));
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(56)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v34.wrapping_add((v31 as u32)))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        54
                    } else {
                        55
                    };
                }
                53 => {
                    bb = 51;
                }
                54 => {
                    v37 = (self.r32(
                        ((self.r32(fp.wrapping_add(56)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v36))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v37 == (1_i32).wrapping_neg())
                        || (v37 == (self.r32(fp.wrapping_add(92)) as i32)))
                    {
                        56
                    } else {
                        57
                    };
                }
                55 => {
                    bb = 53;
                }
                56 => {
                    self.w32(v35, v36);
                    v29 = v29.wrapping_add(1);
                    v35 = v35.wrapping_add(4);
                    bb = 57;
                }
                57 => {
                    bb = 55;
                }
                58 => {
                    bb = 44;
                }
                59 => {
                    v31 = (self.r32(fp.wrapping_add(16)) as i32);
                    bb = 42;
                }
                60 => {
                    bb = 59;
                }
                61 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(352));
                    bb = 63;
                }
                62 => {
                    v43 = (self.r32(fp.wrapping_add(56)) as i32);
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(352).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    let _ = self.f_1001bca0(
                        this,
                        (fp.wrapping_add(132) as i32),
                        (self.r32(fp.wrapping_add(40)) as i32),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                        (self.r32(fp.wrapping_add(20)) as i32),
                        fp.wrapping_add(88),
                        fp.wrapping_add(48),
                        fp.wrapping_add(52),
                    );
                    v44 = (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                        as u32);
                    v45 = self.r32(fp.wrapping_add(72));
                    v46 = cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(28)) as i32));
                    self.w32(fp.wrapping_add(68), v44);
                    v47 = v44.wrapping_add(((self.r32(fp.wrapping_add(76)) as i32) as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((v44 as i32).wrapping_add((self.r32(fp.wrapping_add(76)) as i32)) as u32),
                    );
                    bb = if (v46 < v46.wrapping_add((self.r32(fp.wrapping_add(84)) as i32))) {
                        71
                    } else {
                        72
                    };
                }
                63 => {
                    v38 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v39 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v40 = (v38
                        .wrapping_add((self.r32(fp.wrapping_add(52)) as i32))
                        .wrapping_abs() as u32);
                    v41 = (v39.wrapping_sub(v40) as i32).wrapping_abs();
                    v42 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(64)) as i32)
                                .wrapping_add(v38)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v40.wrapping_add(v39)))
                        as i32);
                    bb = if (v42 >= (self.r32(fp.wrapping_add(4)) as i32)) {
                        66
                    } else {
                        68
                    };
                }
                64 => {
                    bb = if v21 { 63 } else { 65 };
                }
                65 => {
                    bb = 62;
                }
                66 => {
                    bb = if ((v42 == (self.r32(fp.wrapping_add(4)) as i32))
                        && (v41 < (self.r32(fp.wrapping_add(16)) as i32)))
                    {
                        69
                    } else {
                        70
                    };
                }
                67 => {
                    v21 = ({
                        let t3 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(12)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 64;
                }
                68 => {
                    self.w32(fp.wrapping_add(4), (v42 as u32));
                    self.w32(fp.wrapping_add(16), (v41 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 67;
                }
                69 => {
                    self.w32(fp.wrapping_add(16), (v41 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 70;
                }
                70 => {
                    bb = 67;
                }
                71 => {
                    v48 = (20_i32.wrapping_mul(v46) as u32);
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    bb = 73;
                }
                72 => {
                    v51 = ((self.r32(fp.wrapping_add(60)) as i32) as i8);
                    self.w32((v43.wrapping_add(7400_i32) as u32), 0_u32);
                    v52 = ((v51 as i32) & 15_i32);
                    self.w32(fp.wrapping_add(60), (v52 as u32));
                    self.w32(fp.wrapping_add(96), (16_i32.wrapping_mul(v52) as u32));
                    self.w32(fp.wrapping_add(28), self.r32(fp.wrapping_add(72)));
                    v53 = v44.wrapping_add(((self.r32(fp.wrapping_add(76)) as i32) as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((v44 as i32).wrapping_add((self.r32(fp.wrapping_add(76)) as i32)) as u32),
                    );
                    bb = if (v46 < v46.wrapping_add((self.r32(fp.wrapping_add(84)) as i32))) {
                        89
                    } else {
                        90
                    };
                }
                73 => {
                    v49 = v44;
                    bb = if ((v44 as i32) < (v47 as i32)) {
                        76
                    } else {
                        77
                    };
                }
                74 => {
                    bb = if ((self.r32(fp.wrapping_add(36)) as i32) != 0) {
                        73
                    } else {
                        75
                    };
                }
                75 => {
                    bb = 72;
                }
                76 => {
                    bb = 78;
                }
                77 => {
                    v48 = v48.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 74;
                }
                78 => {
                    bb = if ((self.r32(v45) as i32) != 0) {
                        81
                    } else {
                        82
                    };
                }
                79 => {
                    bb = if ((v49 as i32) < (self.r32(fp.wrapping_add(4)) as i32)) {
                        78
                    } else {
                        80
                    };
                }
                80 => {
                    v44 = self.r32(fp.wrapping_add(68));
                    bb = 77;
                }
                81 => {
                    bb = if ((v48 < 400_u32) && (v49 < 20_u32)) {
                        83
                    } else {
                        84
                    };
                }
                82 => {
                    v47 = ((self.r32(fp.wrapping_add(4)) as i32) as u32);
                    v45 = v45.wrapping_add(4);
                    v49 = v49.wrapping_add(1);
                    bb = 79;
                }
                83 => {
                    v50 = (self.r32(
                        (v43 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48.wrapping_add(v49)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v50 != (1_i32).wrapping_neg()) {
                        85
                    } else {
                        86
                    };
                }
                84 => {
                    bb = 82;
                }
                85 => {
                    bb = if (v50.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(88)) as i32)) {
                        87
                    } else {
                        88
                    };
                }
                86 => {
                    bb = 84;
                }
                87 => {
                    self.w32(fp.wrapping_add(60), ((self.r32(v45) as i32) as u32));
                    bb = 88;
                }
                88 => {
                    v43 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 86;
                }
                89 => {
                    v54 = (20_i32.wrapping_mul(v46) as u32);
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    bb = 91;
                }
                90 => {
                    bb = if (v52 != 15_i32) { 107 } else { 108 };
                }
                91 => {
                    v55 = v44;
                    bb = if ((v44 as i32) < (v53 as i32)) {
                        94
                    } else {
                        95
                    };
                }
                92 => {
                    bb = if ((self.r32(fp.wrapping_add(36)) as i32) != 0) {
                        91
                    } else {
                        93
                    };
                }
                93 => {
                    v52 = (self.r32(fp.wrapping_add(60)) as i32);
                    bb = 90;
                }
                94 => {
                    bb = 96;
                }
                95 => {
                    v54 = v54.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 92;
                }
                96 => {
                    bb = if (((((self.r32(fp.wrapping_add(96)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(28))) as i32))
                        != 0_i32)
                        && (v54 < 400_u32))
                        && (v55 < 20_u32))
                    {
                        99
                    } else {
                        100
                    };
                }
                97 => {
                    bb = if ((v55 as i32) < (self.r32(fp.wrapping_add(4)) as i32)) {
                        96
                    } else {
                        98
                    };
                }
                98 => {
                    v44 = self.r32(fp.wrapping_add(68));
                    bb = 95;
                }
                99 => {
                    v56 = v54.wrapping_add(v55);
                    bb = if (v54.wrapping_add(v55)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        101
                    } else {
                        102
                    };
                }
                100 => {
                    v53 = ((self.r32(fp.wrapping_add(4)) as i32) as u32);
                    v55 = v55.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(28),
                        self.r32(fp.wrapping_add(28)).wrapping_add(4),
                    );
                    bb = 97;
                }
                101 => {
                    v57 = (self.r32(
                        (v43 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v56))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v58 = (0_i32 != 0);
                    self.w32(
                        (v43.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v43.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v56.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v57 != (1_i32).wrapping_neg()) {
                        103
                    } else {
                        104
                    };
                }
                102 => {
                    bb = 100;
                }
                103 => {
                    v58 = (v57 >= 5_i32);
                    bb = 104;
                }
                104 => {
                    self.w32(
                        (v43.wrapping_add(7400_i32) as u32),
                        ((self.r32((v43.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v58 { 105 } else { 106 };
                }
                105 => {
                    let _ = self.f_10009c80(
                        self.r32(fp.wrapping_add(104)),
                        (self.r32(fp.wrapping_add(92)) as i32),
                        v57.wrapping_sub(5_i32),
                        58_i32,
                    );
                    bb = 106;
                }
                106 => {
                    bb = 102;
                }
                107 => {
                    let t4 = v52;
                    bb = match t4 {
                        1_i32 => 110,
                        2_i32 => 111,
                        4_i32 => 112,
                        8_i32 => 113,
                        _ => 114,
                    };
                }
                108 => {
                    v59 = (self.r32(
                        (v43.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v59 == (1_i32).wrapping_neg()) {
                        122
                    } else {
                        123
                    };
                }
                109 => {
                    bb = 120;
                }
                110 => {
                    v59 = 0_i32;
                    bb = 109;
                }
                111 => {
                    v59 = 1_i32;
                    bb = 109;
                }
                112 => {
                    v59 = 2_i32;
                    bb = 109;
                }
                113 => {
                    v59 = 3_i32;
                    bb = 109;
                }
                114 => {
                    v59 = (self.r32(fp.wrapping_add(108)) as i32);
                    bb = 109;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 112;
                }
                117 => {
                    bb = 113;
                }
                118 => {
                    bb = 114;
                }
                119 => {
                    bb = 109;
                }
                120 => {
                    v64 = (self.r32(
                        (v43.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v64 == (1_i32).wrapping_neg()) {
                        144
                    } else {
                        145
                    };
                }
                121 => {
                    bb = 108;
                }
                122 => {
                    v59 = (self.r32(fp.wrapping_add(100)) as i32);
                    bb = 123;
                }
                123 => {
                    v60 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(12)) as i32));
                    v61 = (self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v62 = (self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v63 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(12)) as i32))
                        .wrapping_abs();
                    bb = if (v63 == v62) { 124 } else { 126 };
                }
                124 => {
                    bb = if (!(v59 != 0)) { 127 } else { 128 };
                }
                125 => {
                    v59 = (if (v60 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 120;
                }
                126 => {
                    bb = if (v63 < v62) { 150 } else { 151 };
                }
                127 => {
                    bb = if (v61 > 0_i32) { 129 } else { 130 };
                }
                128 => {
                    bb = if (v59 == 1_i32) { 132 } else { 133 };
                }
                129 => {
                    v59 = (if (v60 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 130;
                }
                130 => {
                    bb = 120;
                }
                131 => {
                    bb = 128;
                }
                132 => {
                    bb = if (v60 < 0_i32) { 134 } else { 135 };
                }
                133 => {
                    bb = if (v59 != 2_i32) { 137 } else { 138 };
                }
                134 => {
                    v59 = (if (v61 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 135;
                }
                135 => {
                    bb = 120;
                }
                136 => {
                    bb = 133;
                }
                137 => {
                    bb = if ((v59 == 3_i32) && (v60 > 0_i32)) {
                        139
                    } else {
                        140
                    };
                }
                138 => {
                    bb = if (v61 >= 0_i32) { 142 } else { 143 };
                }
                139 => {
                    v59 = (if (v61 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 140;
                }
                140 => {
                    bb = 120;
                }
                141 => {
                    bb = 138;
                }
                142 => {
                    bb = 120;
                }
                143 => {
                    bb = 125;
                }
                144 => {
                    v64 = (self.r32(fp.wrapping_add(100)) as i32);
                    bb = 145;
                }
                145 => {
                    bb = if (v59 == v64) { 146 } else { 147 };
                }
                146 => {
                    v59 = (1_i32).wrapping_neg();
                    bb = 147;
                }
                147 => {
                    v65 = self.r32(fp.wrapping_add(80));
                    self.w32(
                        self.r32(fp.wrapping_add(80)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(v65.wrapping_add(1540), 2_u32);
                    self.w32(
                        v65.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        v65.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(v65.wrapping_add(1556), (v59 as u32));
                    bb = 148;
                }
                148 => {
                    self.w32(
                        v65.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(92)) as i32) as u32),
                    );
                    v84 = (self.r32(v65.wrapping_add(2072)) as i32);
                    self.w32(v65.wrapping_add(1464), 1_u32);
                    self.w32(v65.wrapping_add(1472), 1_u32);
                    self.w32(v65.wrapping_add(1476), 1_u32);
                    self.w32(v65.wrapping_add(1500), (((v84 > 0_i32) as i32) as u32));
                    self.w8(a2, (v84 > 0_i32) as u8);
                    return 0_i32;
                }
                149 => {
                    bb = 143;
                }
                150 => {
                    v59 = (if (v61 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 120;
                }
                151 => {
                    bb = 125;
                }
                152 => {
                    bb = 151;
                }
                153 => {
                    bb = 34;
                }
                154 => {
                    bb = if (((self.r32(v66) as i32) == 1_i32)
                        && ((self.r32(v66.wrapping_add(116)) as i32) != 99_i32))
                    {
                        157
                    } else {
                        158
                    };
                }
                155 => {
                    bb = if (v68 < 20_i32) { 154 } else { 156 };
                }
                156 => {
                    v70 = 0_i32;
                    self.w32(fp.wrapping_add(40), (v67 as u32));
                    v71 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if (v67 > 0_i32) { 159 } else { 160 };
                }
                157 => {
                    self.w32(v69, (v68 as u32));
                    v67 = v67.wrapping_add(1);
                    v69 = v69.wrapping_add(4);
                    bb = 158;
                }
                158 => {
                    v66 = v66.wrapping_add(156);
                    v68 = v68.wrapping_add(1);
                    bb = 155;
                }
                159 => {
                    v72 = fp.wrapping_add(132);
                    bb = 161;
                }
                160 => {
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(fp.wrapping_add(132).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(12)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(88)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(12)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(88)) as i32)),
                                )
                                .wrapping_add(3728_i32) as u32),
                        ) as i32) as u32),
                    );
                    v75 = 0_i32;
                    v76 = (self
                        .r32(((self.r32(fp.wrapping_add(56)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v76 > 0_i32) { 166 } else { 167 };
                }
                161 => {
                    v73 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(v72) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(12)) as i32)
                                .wrapping_add(156_i32.wrapping_mul((self.r32(v72) as i32)))
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32((v73.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v74 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(52)) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(64)) as i32))
                                .wrapping_abs(),
                        );
                    bb = if (v74 < v71) { 164 } else { 165 };
                }
                162 => {
                    bb = if (v70 < (self.r32(fp.wrapping_add(40)) as i32)) {
                        161
                    } else {
                        163
                    };
                }
                163 => {
                    bb = 160;
                }
                164 => {
                    v71 = v74;
                    self.w32(fp.wrapping_add(0), (v70 as u32));
                    bb = 165;
                }
                165 => {
                    v70 = v70.wrapping_add(1);
                    v72 = v72.wrapping_add(4);
                    bb = 162;
                }
                166 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(352),
                        self.r32(fp.wrapping_add(108)),
                        (4_i32.wrapping_mul(v76) as u32),
                    );
                    v75 = v76;
                    self.w32(fp.wrapping_add(12), (v76 as u32));
                    bb = 167;
                }
                167 => {
                    self.w32(fp.wrapping_add(4), 9999_u32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if (v75 > 0_i32) { 168 } else { 169 };
                }
                168 => {
                    self.w32(fp.wrapping_add(28), fp.wrapping_add(352));
                    bb = 170;
                }
                169 => {
                    v82 = self.r32(fp.wrapping_add(80));
                    v83 = (self.r32(fp.wrapping_add(352).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(80)).wrapping_add(1540), 2_u32);
                    self.w32(v82.wrapping_add(1552), (v83 as u32));
                    self.w32(v82.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
                    self.w32(v82.wrapping_add(1544), (crem_i32(v83, 20_i32) as u32));
                    self.w32(v82.wrapping_add(1548), (cdiv_i32(v83, 20_i32) as u32));
                    v65 = v82;
                    bb = 148;
                }
                170 => {
                    v77 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v78 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v79 = (v78
                        .wrapping_add((self.r32(fp.wrapping_add(52)) as i32))
                        .wrapping_abs() as u32);
                    v80 = (v77.wrapping_sub(v79) as i32).wrapping_abs();
                    v81 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v78.wrapping_add((self.r32(fp.wrapping_add(64)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v79.wrapping_add(v77)))
                        as i32);
                    bb = if (v81 >= (self.r32(fp.wrapping_add(4)) as i32)) {
                        173
                    } else {
                        175
                    };
                }
                171 => {
                    bb = if v21 { 170 } else { 172 };
                }
                172 => {
                    bb = 169;
                }
                173 => {
                    bb = if ((v81 == (self.r32(fp.wrapping_add(4)) as i32))
                        && (v80 < (self.r32(fp.wrapping_add(16)) as i32)))
                    {
                        176
                    } else {
                        177
                    };
                }
                174 => {
                    v21 = ({
                        let t5 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(12)) as i32));
                    self.w32(
                        fp.wrapping_add(28),
                        self.r32(fp.wrapping_add(28)).wrapping_add(4),
                    );
                    bb = 171;
                }
                175 => {
                    self.w32(fp.wrapping_add(4), (v81 as u32));
                    self.w32(fp.wrapping_add(16), (v80 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 174;
                }
                176 => {
                    self.w32(fp.wrapping_add(16), (v80 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 177;
                }
                177 => {
                    bb = 174;
                }
                178 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001BCA0` (477 bytes).
    pub(crate) fn f_1001bca0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: u32,
        mut a6: i32,
        mut a7: u32,
        mut a8: u32,
        mut a9: u32,
    ) -> i32 {
        let fp = self.enter(64);
        let r = self.f_1001bca0_body(fp, this, a2, a3, a4, a5, a6, a7, a8, a9);
        self.leave(64);
        r
    }

    fn f_1001bca0_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: u32,
        mut a5: u32,
        mut a6: i32,
        mut a7: u32,
        mut a8: u32,
        mut a9: u32,
    ) -> i32 {
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: i32 = 0;
        let mut v18: u32 = 0;
        let mut i: i32 = 0;
        let mut v20: i8 = 0;
        let mut v21: i32 = 0;
        let mut v22: bool = false;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: u32 = 0;
        self.w32(fp.wrapping_add(52), a5);
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v9 = self.r32(fp.wrapping_add(52));
                    v10 = (self.r32(self.r32(this)) as i32);
                    v11 = a4;
                    v12 =
                        (self.r32((v10.wrapping_add(20_i32) as u32)) as i32).wrapping_add(3200_i32);
                    v13 = 0_i32;
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(
                            ((self.r32((v10.wrapping_add(28_i32) as u32)) as i32)
                                .wrapping_add(1388_i32) as u32),
                        ) as i32) as u32),
                    );
                    v14 = a4;
                    bb = 1;
                }
                1 => {
                    self.w8(fp.wrapping_add(52).wrapping_add((v13 as u32)), 0_u8);
                    self.w32(
                        fp.wrapping_add(20)
                            .wrapping_add((v13.wrapping_add(4_i32) as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    let t1 = v13;
                    bb = match t1 {
                        0_i32 => 5,
                        1_i32 => 6,
                        2_i32 => 7,
                        3_i32 => 8,
                        _ => 9,
                    };
                }
                2 => {
                    bb = if (v13 < 4_i32) { 1 } else { 3 };
                }
                3 => {
                    v18 = a7;
                    i = 0_i32;
                    bb = 42;
                }
                4 => {
                    bb = if ((v11 < 20_u32) && (v14 < 20_u32)) {
                        16
                    } else {
                        17
                    };
                }
                5 => {
                    v11 = a4;
                    v14 = v9.wrapping_sub(1_u32);
                    bb = 4;
                }
                6 => {
                    v11 = a4.wrapping_add(1_u32);
                    bb = 11;
                }
                7 => {
                    v11 = a4;
                    v14 = v9.wrapping_add(1_u32);
                    bb = 4;
                }
                8 => {
                    v11 = a4.wrapping_sub(1_u32);
                    bb = 11;
                }
                9 => {
                    bb = 4;
                }
                10 => {
                    bb = 6;
                }
                11 => {
                    v14 = v9;
                    bb = 4;
                }
                12 => {
                    bb = 7;
                }
                13 => {
                    bb = 8;
                }
                14 => {
                    bb = 9;
                }
                15 => {
                    bb = 4;
                }
                16 => {
                    v15 = (self.r32((v12 as u32).wrapping_add(
                        4_u32.wrapping_mul(v11.wrapping_add(20_u32.wrapping_mul(v14))),
                    )) as i32);
                    bb = if (v15 > 4_i32) { 18 } else { 19 };
                }
                17 => {
                    self.w32(
                        fp.wrapping_add(20)
                            .wrapping_add((v13 as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    v16 = 20_u32.wrapping_mul(v14);
                    bb = 20;
                }
                18 => {
                    self.w8(fp.wrapping_add(52).wrapping_add((v13 as u32)), 1_u8);
                    self.w32(
                        fp.wrapping_add(20)
                            .wrapping_add((v13.wrapping_add(4_i32) as u32).wrapping_mul(4)),
                        (v15.wrapping_sub(5_i32) as u32),
                    );
                    bb = 19;
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    bb = if ((v11 < 20_u32) && (v16 < 400_u32)) {
                        21
                    } else {
                        22
                    };
                }
                21 => {
                    v17 = (self
                        .r32((v12 as u32).wrapping_add(4_u32.wrapping_mul(v16.wrapping_add(v11))))
                        as i32);
                    bb = if (v17 != (1_i32).wrapping_neg()) {
                        23
                    } else {
                        24
                    };
                }
                22 => {
                    v13 = v13.wrapping_add(1);
                    bb = 2;
                }
                23 => {
                    bb = if (v17 >= 5_i32) { 25 } else { 27 };
                }
                24 => {
                    let t2 = v13;
                    bb = match t2 {
                        0_i32 => 32,
                        1_i32 => 33,
                        2_i32 => 34,
                        3_i32 => 35,
                        _ => 36,
                    };
                }
                25 => {
                    self.w32(
                        fp.wrapping_add(20)
                            .wrapping_add((v13 as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(20)
                                .wrapping_add((v13 as u32).wrapping_mul(4)),
                        ) as i32)
                            .wrapping_add(1) as u32),
                    );
                    bb = 26;
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    bb = if (v17 != (self.r32(fp.wrapping_add(0)) as i32)) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    bb = 22;
                }
                29 => {
                    bb = 26;
                }
                30 => {
                    bb = 29;
                }
                31 => {
                    bb = 20;
                }
                32 => {
                    v14 = v14.wrapping_sub(1);
                    v16 = v16.wrapping_sub(20_u32);
                    bb = 31;
                }
                33 => {
                    v11 = v11.wrapping_add(1);
                    bb = 31;
                }
                34 => {
                    v14 = v14.wrapping_add(1);
                    v16 = v16.wrapping_add(20_u32);
                    bb = 31;
                }
                35 => {
                    v11 = v11.wrapping_sub(1);
                    bb = 31;
                }
                36 => {
                    bb = 20;
                }
                37 => {
                    bb = 33;
                }
                38 => {
                    bb = 34;
                }
                39 => {
                    bb = 35;
                }
                40 => {
                    bb = 36;
                }
                41 => {
                    bb = 31;
                }
                42 => {
                    bb = if (i < 4_i32) { 43 } else { 45 };
                }
                43 => {
                    v20 = (self.r8(fp.wrapping_add(52).wrapping_add((i as u32))) as i8);
                    self.w32(
                        fp.wrapping_add(4).wrapping_add((i as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    bb = if ((v20 as i32) == 1_i32) { 46 } else { 47 };
                }
                44 => {
                    i = i.wrapping_add(1);
                    bb = 42;
                }
                45 => {
                    v23 = (self.r32(fp.wrapping_add(4)) as i32);
                    v24 = 0_i32;
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32)
                        < (self.r32(fp.wrapping_add(8)) as i32))
                    {
                        50
                    } else {
                        51
                    };
                }
                46 => {
                    v21 = 10_i32.wrapping_mul(
                        (self.r32(fp.wrapping_add(20).wrapping_add((i as u32).wrapping_mul(4)))
                            as i32),
                    );
                    v22 = ((self.r32(
                        fp.wrapping_add(20)
                            .wrapping_add((i.wrapping_add(4_i32) as u32).wrapping_mul(4)),
                    ) as i32)
                        == (self.r32(v18) as i32));
                    self.w32(
                        fp.wrapping_add(4).wrapping_add((i as u32).wrapping_mul(4)),
                        (v21 as u32),
                    );
                    bb = if v22 { 48 } else { 49 };
                }
                47 => {
                    bb = 44;
                }
                48 => {
                    self.w32(
                        fp.wrapping_add(4).wrapping_add((i as u32).wrapping_mul(4)),
                        (v21.wrapping_add(1_i32) as u32),
                    );
                    bb = 49;
                }
                49 => {
                    bb = 47;
                }
                50 => {
                    v24 = 1_i32;
                    v23 = (self.r32(fp.wrapping_add(8)) as i32);
                    bb = 51;
                }
                51 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(12)) as i32)) {
                        52
                    } else {
                        53
                    };
                }
                52 => {
                    v24 = 2_i32;
                    v23 = (self.r32(fp.wrapping_add(12)) as i32);
                    bb = 53;
                }
                53 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(16)) as i32)) {
                        54
                    } else {
                        55
                    };
                }
                54 => {
                    v24 = 3_i32;
                    bb = 55;
                }
                55 => {
                    self.w32(
                        v18,
                        ((self.r32(
                            fp.wrapping_add(20)
                                .wrapping_add((v24.wrapping_add(4_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    let t3 = v24;
                    bb = match t3 {
                        0_i32 => 57,
                        1_i32 => 58,
                        2_i32 => 59,
                        3_i32 => 60,
                        _ => 61,
                    };
                }
                56 => {
                    return 0;
                }
                57 => {
                    v25 = a9;
                    self.w32(a8, a4);
                    self.w32(v25, ((v9.wrapping_sub(1_u32) as i32) as u32));
                    return 0_i32;
                }
                58 => {
                    v27 = a4.wrapping_add(1_u32);
                    bb = 63;
                }
                59 => {
                    v28 = a9;
                    self.w32(a8, a4);
                    self.w32(v28, ((v9.wrapping_add(1_u32) as i32) as u32));
                    return 0_i32;
                }
                60 => {
                    v27 = a4.wrapping_sub(1_u32);
                    bb = 63;
                }
                61 => {
                    return 0_i32;
                }
                62 => {
                    bb = 58;
                }
                63 => {
                    v29 = a9;
                    self.w32(a8, v27);
                    self.w32(v29, v9);
                    return 0_i32;
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
                    bb = 56;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001BEB0` (846 bytes).
    pub(crate) fn f_1001beb0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(64);
        let r = self.f_1001beb0_body(fp, this, a2);
        self.leave(64);
        r
    }

    fn f_1001beb0_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: u32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: u32 = 0;
        let mut v24: u32 = 0;
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
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    self.w32(fp.wrapping_add(40), this);
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = ((self.r32(v3.wrapping_add(28)) as i32) as u32);
                    v5 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(v4.wrapping_add(1388)) as i32) as u32),
                    );
                    v6 = ((self.r32(v3.wrapping_add(12)) as i32)
                        .wrapping_add(716_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)))
                        as u32);
                    v7 = (self.r32(v6.wrapping_add(384)) as i32);
                    v8 = (self.r32(v6.wrapping_add(380)) as i32);
                    self.w32(fp.wrapping_add(20), (v8 as u32));
                    self.w32(fp.wrapping_add(24), (v7 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    v9 = v8.wrapping_add(20_i32.wrapping_mul(v7));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v8 >= 10_i32) { 1 } else { 3 };
                }
                1 => {
                    v11 = ((v7 >= 10_i32) as i32).wrapping_add(1_i32);
                    bb = 2;
                }
                2 => {
                    v12 = 0_i32;
                    v13 = 0x1007F4DC_u32.wrapping_add((96_i32.wrapping_mul(v11) as u32));
                    bb = 4;
                }
                3 => {
                    v10 = ((v7 >= 10_i32) as i32).wrapping_sub(1_i32);
                    v10 = (((((v10 as u32) & 0xFFFFFF00)
                        | (((((v10 & 253_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v11 = v10.wrapping_add(3_i32);
                    bb = 2;
                }
                4 => {
                    v14 = (self.r32(v13) as i32);
                    bb = if ((self.r32(
                        (v5.wrapping_add(4_i32.wrapping_mul((self.r32(v13) as i32)))
                            .wrapping_add(3200_i32) as u32),
                    ) as i32)
                        == (1_i32).wrapping_neg())
                    {
                        7
                    } else {
                        8
                    };
                }
                5 => {
                    bb = if (v12 < 24_i32) { 4 } else { 6 };
                }
                6 => {
                    bb = if (v12 == 24_i32) { 10 } else { 11 };
                }
                7 => {
                    bb = 6;
                }
                8 => {
                    v12 = v12.wrapping_add(1);
                    v13 = v13.wrapping_add(4);
                    bb = 5;
                }
                9 => {
                    bb = 8;
                }
                10 => {
                    v14 = v9;
                    bb = 11;
                }
                11 => {
                    self.w32(fp.wrapping_add(52), (v14 as u32));
                    self.w32(fp.wrapping_add(8), (crem_i32(v14, 20_i32) as u32));
                    self.w32(fp.wrapping_add(12), (cdiv_i32(v14, 20_i32) as u32));
                    let _ = self.f_10006940(this, fp.wrapping_add(32), 0_i32, 0_i32, 6_i32, 1_i32);
                    v15 = (self.r32(self.r32(fp.wrapping_add(32))) as i32);
                    v16 = (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(4)) as i32);
                    v17 = (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(8)) as i32);
                    v18 = (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(12)) as i32);
                    v19 = self.r32(fp.wrapping_add(32)).wrapping_add(16);
                    self.w32((v5.wrapping_add(7400_i32) as u32), 0_u32);
                    self.w32(fp.wrapping_add(4), v19);
                    v20 = v16.wrapping_add((self.r32(fp.wrapping_add(12)) as i32));
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(v15) as u32),
                    );
                    v21 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_add(v15)
                        .wrapping_add(v17);
                    v22 = v20.wrapping_add(v18);
                    self.w32(fp.wrapping_add(44), (v21 as u32));
                    bb = if (v20 < v20.wrapping_add(v18)) {
                        12
                    } else {
                        13
                    };
                }
                12 => {
                    v23 = (20_i32.wrapping_mul(v20) as u32);
                    self.w32(fp.wrapping_add(16), (v22.wrapping_sub(v20) as u32));
                    bb = 14;
                }
                13 => {
                    v26 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(24)) as i32));
                    v27 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32));
                    v28 = (self.r32(fp.wrapping_add(0)) as i32);
                    v29 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(24)) as i32))
                        .wrapping_abs();
                    v30 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs();
                    bb = if (v30 != v29) { 28 } else { 29 };
                }
                14 => {
                    v24 = ((self.r32(fp.wrapping_add(36)) as i32) as u32);
                    bb = if ((self.r32(fp.wrapping_add(36)) as i32) < v21) {
                        17
                    } else {
                        18
                    };
                }
                15 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) != 0) {
                        14
                    } else {
                        16
                    };
                }
                16 => {
                    bb = 13;
                }
                17 => {
                    bb = 19;
                }
                18 => {
                    v23 = v23.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 15;
                }
                19 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(4))) as i32) != 0) {
                        22
                    } else {
                        23
                    };
                }
                20 => {
                    bb = if ((v24 as i32) < (self.r32(fp.wrapping_add(44)) as i32)) {
                        19
                    } else {
                        21
                    };
                }
                21 => {
                    bb = 18;
                }
                22 => {
                    bb = if ((v23 < 400_u32) && (v24 < 20_u32)) {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    v24 = v24.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(4),
                    );
                    v21 = (self.r32(fp.wrapping_add(44)) as i32);
                    bb = 20;
                }
                24 => {
                    {
                        let a0 = (v5
                            .wrapping_add(4_i32.wrapping_mul({
                                let t1 = (self.r32((v5.wrapping_add(7400_i32) as u32)) as i32);
                                self.w32(
                                    (v5.wrapping_add(7400_i32) as u32),
                                    (t1.wrapping_add(1) as u32),
                                );
                                t1
                            }))
                            .wrapping_add(6800_i32) as u32);
                        let a1 = ((v24.wrapping_add(v23).wrapping_add(400_u32) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    v25 = (self.r32(
                        (v5 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v24.wrapping_add(v23)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v25 > 4_i32) { 26 } else { 27 };
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    let _ = self.f_10009c80(
                        self.r32(fp.wrapping_add(40)),
                        (self.r32(fp.wrapping_add(28)) as i32),
                        v25.wrapping_sub(5_i32),
                        59_i32,
                    );
                    bb = 27;
                }
                27 => {
                    bb = 25;
                }
                28 => {
                    bb = if (v30 >= v29) { 30 } else { 32 };
                }
                29 => {
                    v32 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        35
                    } else {
                        37
                    };
                }
                30 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 31;
                }
                31 => {
                    v32 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = 33;
                }
                32 => {
                    v31 = ((v26 >= 0_i32) as i32).wrapping_sub(1_i32);
                    v31 = (((((v31 as u32) & 0xFFFFFF00)
                        | (((((v31 & 254_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v28 = v31.wrapping_add(2_i32);
                    bb = 31;
                }
                33 => {
                    bb = if (v28 != v32) { 53 } else { 54 };
                }
                34 => {
                    bb = 29;
                }
                35 => {
                    let t2 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = match t2 {
                        1_i32 => 39,
                        2_i32 => 40,
                        3_i32 => 41,
                        _ => 42,
                    };
                }
                36 => {
                    self.w32(
                        v4.wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        v4.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(48)) as i32) as u32),
                    );
                    v33 = (self.r32(v4.wrapping_add(2072)) as i32);
                    self.w32(
                        v4.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(24)) as i32) as u32),
                    );
                    self.w32(v4.wrapping_add(1556), (v28 as u32));
                    self.w32(v4.wrapping_add(1540), 1_u32);
                    self.w32(v4.wrapping_add(1464), 1_u32);
                    self.w32(
                        v4.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(v4.wrapping_add(1472), 1_u32);
                    self.w32(v4.wrapping_add(1476), 4_u32);
                    self.w32(v4.wrapping_add(1500), (((v33 > 0_i32) as i32) as u32));
                    v35 = (self.r32(fp.wrapping_add(52)) as i32);
                    self.w32(
                        v4.wrapping_add(2324),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        v4.wrapping_add(2328),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(v4.wrapping_add(2332), (v35 as u32));
                    self.w8(a2, 1_u8);
                    return 0_i32;
                }
                37 => {
                    bb = if (v26 <= 0_i32) { 60 } else { 61 };
                }
                38 => {
                    bb = 36;
                }
                39 => {
                    bb = if (v27 >= 0_i32) { 43 } else { 44 };
                }
                40 => {
                    bb = if (v26 >= 0_i32) { 47 } else { 48 };
                }
                41 => {
                    bb = if (v27 <= 0_i32) { 51 } else { 52 };
                }
                42 => {
                    bb = 56;
                }
                43 => {
                    bb = 33;
                }
                44 => {
                    v34 = ((v26 >= 0_i32) as i32).wrapping_sub(1_i32);
                    v34 = (((((v34 as u32) & 0xFFFFFF00)
                        | (((((v34 & 254_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v28 = v34.wrapping_add(2_i32);
                    bb = 38;
                }
                45 => {
                    bb = 44;
                }
                46 => {
                    bb = 40;
                }
                47 => {
                    bb = 33;
                }
                48 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 38;
                }
                49 => {
                    bb = 48;
                }
                50 => {
                    bb = 41;
                }
                51 => {
                    bb = 33;
                }
                52 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 38;
                }
                53 => {
                    bb = 38;
                }
                54 => {
                    bb = 56;
                }
                55 => {
                    bb = 54;
                }
                56 => {
                    v28 = (1_i32).wrapping_neg();
                    bb = 38;
                }
                57 => {
                    bb = 52;
                }
                58 => {
                    bb = 42;
                }
                59 => {
                    bb = 38;
                }
                60 => {
                    bb = 33;
                }
                61 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 36;
                }
                62 => {
                    bb = 61;
                }
                63 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001C200` (2589 bytes).
    pub(crate) fn f_1001c200(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_1001c200_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_1001c200_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(92), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(52), (v4 as u32));
                    self.w32(fp.wrapping_add(84), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(216),
                        (fp.wrapping_add(104) as i32),
                        (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(100), 0_i32, 0_i32, 3_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(100))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(100)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(32), fp.wrapping_add(236));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(116));
                    v7 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(39), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w8(fp.wrapping_add(31), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(39)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(39), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(32));
                            self.w32(fp.wrapping_add(32), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(116);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(116);
                    bb = 151;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(76));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(116).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(32), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(336)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(32), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(336));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    v37 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(336).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    v39 = crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    v40 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(24), (v44 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v45 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        86
                    } else {
                        87
                    };
                }
                70 => {
                    v42 = (v39 as u32);
                    bb = if (v39 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v43 != (1_i32).wrapping_neg()) {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    bb = if (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)) {
                        84
                    } else {
                        85
                    };
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v37) as i32) as u32));
                    bb = 85;
                }
                85 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 83;
                }
                86 => {
                    v46 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 88;
                }
                87 => {
                    bb = if (v44 != 15_i32) { 104 } else { 105 };
                }
                88 => {
                    v47 = (v39 as u32);
                    bb = if (v39 < v45) { 91 } else { 92 };
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v44 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 87;
                }
                91 => {
                    bb = 93;
                }
                92 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                93 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                95 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 92;
                }
                96 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 94;
                }
                98 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        100
                    } else {
                        101
                    };
                }
                99 => {
                    bb = 97;
                }
                100 => {
                    v50 = (v49 >= 5_i32);
                    bb = 101;
                }
                101 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 102 } else { 103 };
                }
                102 => {
                    let _ = self.f_10009c80(
                        this,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v49.wrapping_sub(5_i32),
                        60_i32,
                    );
                    bb = 103;
                }
                103 => {
                    bb = 99;
                }
                104 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 107,
                        2_i32 => 108,
                        4_i32 => 109,
                        8_i32 => 110,
                        _ => 111,
                    };
                }
                105 => {
                    v51 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        119
                    } else {
                        120
                    };
                }
                106 => {
                    bb = 117;
                }
                107 => {
                    v51 = 0_i32;
                    bb = 106;
                }
                108 => {
                    v51 = 1_i32;
                    bb = 106;
                }
                109 => {
                    v51 = 2_i32;
                    bb = 106;
                }
                110 => {
                    v51 = 3_i32;
                    bb = 106;
                }
                111 => {
                    v51 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 106;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 110;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    v56 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        141
                    } else {
                        142
                    };
                }
                118 => {
                    bb = 105;
                }
                119 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 120;
                }
                120 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 121 } else { 123 };
                }
                121 => {
                    bb = if (!(v51 != 0)) { 124 } else { 125 };
                }
                122 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 117;
                }
                123 => {
                    bb = if (v55 < v54) { 147 } else { 148 };
                }
                124 => {
                    bb = if (v52 > 0_i32) { 126 } else { 127 };
                }
                125 => {
                    bb = if (v51 == 1_i32) { 129 } else { 130 };
                }
                126 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 127;
                }
                127 => {
                    bb = 117;
                }
                128 => {
                    bb = 125;
                }
                129 => {
                    bb = if (v53 < 0_i32) { 131 } else { 132 };
                }
                130 => {
                    bb = if (v51 != 2_i32) { 134 } else { 135 };
                }
                131 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 132;
                }
                132 => {
                    bb = 117;
                }
                133 => {
                    bb = 130;
                }
                134 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        136
                    } else {
                        137
                    };
                }
                135 => {
                    bb = if (v52 >= 0_i32) { 139 } else { 140 };
                }
                136 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 137;
                }
                137 => {
                    bb = 117;
                }
                138 => {
                    bb = 135;
                }
                139 => {
                    bb = 117;
                }
                140 => {
                    bb = 122;
                }
                141 => {
                    v56 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 142;
                }
                142 => {
                    bb = if (v51 == v56) { 143 } else { 144 };
                }
                143 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 144;
                }
                144 => {
                    v57 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 145;
                }
                145 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1472), 1_u32);
                    self.w32(v57.wrapping_add(1476), 8_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                146 => {
                    bb = 140;
                }
                147 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 117;
                }
                148 => {
                    bb = 122;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 31;
                }
                151 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v60 < 20_i32) { 151 } else { 153 };
                }
                153 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 155;
                }
                155 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 152;
                }
                156 => {
                    v64 = fp.wrapping_add(116);
                    bb = 158;
                }
                157 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(116).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(116).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 163 } else { 164 };
                }
                158 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 161 } else { 162 };
                }
                159 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                160 => {
                    bb = 157;
                }
                161 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 162;
                }
                162 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 159;
                }
                163 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(336),
                        self.r32(fp.wrapping_add(92)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 164;
                }
                164 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 165 } else { 166 };
                }
                165 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(336));
                    bb = 167;
                }
                166 => {
                    v75 = (self.r32(fp.wrapping_add(336).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(72));
                    bb = 145;
                }
                167 => {
                    v70 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        170
                    } else {
                        172
                    };
                }
                168 => {
                    bb = if v35 { 167 } else { 169 };
                }
                169 => {
                    bb = 166;
                }
                170 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                171 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 168;
                }
                172 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 171;
                }
                173 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 174;
                }
                174 => {
                    bb = 171;
                }
                175 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001CC40` (2582 bytes).
    pub(crate) fn f_1001cc40(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1904);
        let r = self.f_1001cc40_body(fp, this, a2);
        self.leave(1904);
        r
    }

    fn f_1001cc40_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: bool = false;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: i64 = 0;
        let mut v18: i32 = 0;
        let mut v19: u32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: u32 = 0;
        let mut v24: u32 = 0;
        let mut v25: u32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: u32 = 0;
        let mut v29: u32 = 0;
        let mut v30: i32 = 0;
        let mut v31: i32 = 0;
        let mut v32: u32 = 0;
        let mut v33: u32 = 0;
        let mut v34: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: u32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: u32 = 0;
        let mut v44: bool = false;
        let mut v45: i32 = 0;
        let mut v46: i32 = 0;
        let mut v47: i32 = 0;
        let mut v48: i32 = 0;
        let mut v49: i32 = 0;
        let mut v50: i32 = 0;
        let mut v51: i32 = 0;
        let mut v52: u32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: u32 = 0;
        let mut v56: i32 = 0;
        let mut v57: i32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i64 = 0;
        let mut v60: i32 = 0;
        let mut v61: i32 = 0;
        let mut v62: i32 = 0;
        let mut v63: u32 = 0;
        let mut v64: i32 = 0;
        let mut v65: u32 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: i32 = 0;
        let mut v71: bool = false;
        let mut v72: bool = false;
        let mut v73: u32 = 0;
        let mut v74: u32 = 0;
        let mut v75: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(((self.r32(this) as i32) as u32));
                    v3 = (self.r32(v2.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(v2.wrapping_add(28)) as i32) as u32),
                    );
                    v4 = (self.r32(self.r32(fp.wrapping_add(28)).wrapping_add(1388)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v2.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(40), (v3 as u32));
                    self.w32(fp.wrapping_add(84), (v4 as u32));
                    v5 = ((self.r32(fp.wrapping_add(72)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v4)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v5.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(v5.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(v5.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(88), 0_i32, 5_i32, 2_i32, 1_i32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(88))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(self.r32(fp.wrapping_add(88)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(88)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(88)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(self.r32(fp.wrapping_add(88)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(72)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(48), fp.wrapping_add(192));
                    self.w32(fp.wrapping_add(16), fp.wrapping_add(92));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(self.r32(fp.wrapping_add(4))) as i32) == 1_i32)
                        && ((self.r32(self.r32(fp.wrapping_add(4)).wrapping_add(116)) as i32)
                            != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if v13 { 1 } else { 3 };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v6 = (self.r32(fp.wrapping_add(56)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(4)).wrapping_add(124)) as i32),
                    );
                    v7 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(4)).wrapping_add(128)) as i32),
                    );
                    v8 = self.r32(fp.wrapping_add(76));
                    self.w8(fp.wrapping_add(27), 0_u8);
                    self.w32(
                        fp.wrapping_add(60),
                        (v7.wrapping_add((self.r32(fp.wrapping_add(52)) as i32)) as u32),
                    );
                    self.w8(fp.wrapping_add(26), 0_u8);
                    self.w32(fp.wrapping_add(20), (v7 as u32));
                    bb = if (v7 < v7.wrapping_add((self.r32(fp.wrapping_add(52)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v13 = ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1_i32) < 20_i32);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(156),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v9 = (20_i32.wrapping_mul(v7) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v10 = ((self.r32(fp.wrapping_add(56)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(4)).wrapping_add(124)) as i32),
                    ) as u32);
                    bb = if (v6 < v6.wrapping_add((self.r32(fp.wrapping_add(68)) as i32))) {
                        11
                    } else {
                        12
                    };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(20)) as i32)
                        < (self.r32(fp.wrapping_add(60)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(27)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v9 = v9.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(20)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(v8) as i32) != 0) { 16 } else { 17 };
                }
                14 => {
                    bb = if ((v10 as i32) < v6.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v9 < 400_u32) && (v10 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v8 = v8.wrapping_add(4);
                    v10 = v10.wrapping_add(1);
                    bb = 14;
                }
                18 => {
                    v11 = (4_u32.wrapping_mul(v9.wrapping_add(v10)) as i32);
                    bb = if ((self.r32((v11.wrapping_add(v3) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v12 = (self.r32((v11.wrapping_add(v3).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v12 == (1_i32).wrapping_neg()) || (v12 == v4)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(27), 1_u8);
                    bb = if ((v10 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(20)) as i32)
                            == (self.r32(fp.wrapping_add(36)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(26), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(16)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(26)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    self.w32(
                        self.r32(fp.wrapping_add(48)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        self.r32(fp.wrapping_add(48)).wrapping_add(4),
                    );
                    bb = 29;
                }
                29 => {
                    bb = 7;
                }
                30 => {
                    v14 = 0_i32;
                    v15 = 9999_i32;
                    self.w32(fp.wrapping_add(4), 0_u32);
                    v16 = fp.wrapping_add(92);
                    bb = 32;
                }
                31 => {
                    v52 = ((self.r32(fp.wrapping_add(72)) as i32).wrapping_add(3600_i32) as u32);
                    v53 = 0_i32;
                    v54 = 0_i32;
                    v55 = fp.wrapping_add(92);
                    bb = 147;
                }
                32 => {
                    v17 = ((self.r32(
                        ((self.r32(fp.wrapping_add(72)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v16) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v18 = ((((((v17 as u64) >> 32) as u32) as i64) ^ v17)
                        .wrapping_sub(((((v17 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(72)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v16) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(36)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v18 < v15) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v14 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v19 = self.r32(fp.wrapping_add(76));
                    v20 = 0_i32;
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(92).wrapping_add(
                            ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(20), 0_u32);
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(72)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v21 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(72)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3728_i32) as u32),
                        ) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(60)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    bb = if (v22 < v22.wrapping_add((self.r32(fp.wrapping_add(52)) as i32))) {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v15 = v18;
                    self.w32(fp.wrapping_add(4), (v14 as u32));
                    bb = 36;
                }
                36 => {
                    v14 = v14.wrapping_add(1);
                    v16 = v16.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v23 = (20_i32.wrapping_mul(v22) as u32);
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32),
                    );
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(0), 9999_u32);
                    self.w32(fp.wrapping_add(32), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v20 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v21 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(20), (v20 as u32));
                    bb = 38;
                }
                42 => {
                    v24 = fp
                        .wrapping_add(292)
                        .wrapping_add((v20 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v23 = v23.wrapping_add(20_u32);
                    bb = if (!({
                        let t1 = (self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(16), (t1 as u32));
                        t1
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v19) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v21 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v23 < 400_u32) && ((v21 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v19 = v19.wrapping_add(4);
                    v21 = v21.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v25 = (v21 as u32).wrapping_add(v23);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v21 as u32).wrapping_add(v23))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v26 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v25))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v26 == (1_i32).wrapping_neg())
                        || (v26 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v24, v25);
                    v20 = v20.wrapping_add(1);
                    v24 = v24.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v21 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(16), fp.wrapping_add(292));
                    bb = 60;
                }
                59 => {
                    v32 = self.r32(fp.wrapping_add(28));
                    v33 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(292).wrapping_add(
                            ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        (cdiv_i32((self.r32(fp.wrapping_add(16)) as i32), 20_i32) as u32),
                    );
                    v34 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(16)) as i32), 20_i32));
                    v35 = crem_i32((self.r32(fp.wrapping_add(16)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(44),
                        (crem_i32((self.r32(fp.wrapping_add(16)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        (v35.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(20), (v35 as u32));
                    bb = if (v34 < v34.wrapping_add((self.r32(fp.wrapping_add(52)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v27 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(16))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v28 = ((self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(16))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v29 = (v27
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v30 = (v28.wrapping_sub(v29) as i32).wrapping_abs();
                    v31 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(16))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(36)) as i32)
                                .wrapping_add(v27)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v29.wrapping_add(v28)))
                        as i32);
                    bb = if (v31 >= (self.r32(fp.wrapping_add(0)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v13 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v31 == (self.r32(fp.wrapping_add(0)) as i32))
                        && (v30 < (self.r32(fp.wrapping_add(32)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v13 = ({
                        let t2 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t2 as u32));
                        t2
                    } < (self.r32(fp.wrapping_add(20)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(0), (v31 as u32));
                    self.w32(fp.wrapping_add(32), (v30 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(32), (v30 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v36 = (20_i32.wrapping_mul(v34) as u32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    v39 = ((self.r32(fp.wrapping_add(56)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v39) as u32));
                    self.w32(
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(7400_i32) as u32),
                        0_u32,
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v40 = v35.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v35.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v34 < v34.wrapping_add((self.r32(fp.wrapping_add(52)) as i32))) {
                        82
                    } else {
                        83
                    };
                }
                70 => {
                    v37 = (v35 as u32);
                    bb = if (v35 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        73
                    } else {
                        74
                    };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(8)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v36 = v36.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((((self.r32(v33) as i32) != 0) && (v36 < 400_u32)) && (v37 < 20_u32)) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v37 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v35 = (self.r32(fp.wrapping_add(20)) as i32);
                    bb = 74;
                }
                78 => {
                    v38 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v36.wrapping_add(v37)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v38 != (1_i32).wrapping_neg())
                        && (v38.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)))
                    {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v33 = v33.wrapping_add(4);
                    v37 = v37.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    self.w32(fp.wrapping_add(56), ((self.r32(v33) as i32) as u32));
                    bb = 81;
                }
                81 => {
                    v32 = self.r32(fp.wrapping_add(28));
                    bb = 79;
                }
                82 => {
                    v41 = (20_i32.wrapping_mul(v34) as u32);
                    bb = 84;
                }
                83 => {
                    bb = if (v39 != 15_i32) { 100 } else { 101 };
                }
                84 => {
                    v42 = (v35 as u32);
                    bb = if (v35 < v40) { 87 } else { 88 };
                }
                85 => {
                    bb = if ((self.r32(fp.wrapping_add(52)) as i32) != 0) {
                        84
                    } else {
                        86
                    };
                }
                86 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 83;
                }
                87 => {
                    bb = 89;
                }
                88 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(fp.wrapping_add(52)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 85;
                }
                89 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v41 < 400_u32))
                        && (v42 < 20_u32))
                    {
                        92
                    } else {
                        93
                    };
                }
                90 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        89
                    } else {
                        91
                    };
                }
                91 => {
                    v35 = (self.r32(fp.wrapping_add(20)) as i32);
                    bb = 88;
                }
                92 => {
                    v43 = v41.wrapping_add(v42);
                    bb = if (v41.wrapping_add(v42)
                        != ((self.r32(fp.wrapping_add(16)) as i32) as u32))
                    {
                        94
                    } else {
                        95
                    };
                }
                93 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v42 = v42.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 90;
                }
                94 => {
                    v44 = (0_i32 != 0);
                    v45 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v43))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    self.w32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(4_i32.wrapping_mul(
                                (self.r32(
                                    ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(7400_i32)
                                        as u32),
                                ) as i32),
                            ))
                            .wrapping_add(6800_i32) as u32),
                        ((v43.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v45 != (1_i32).wrapping_neg()) {
                        96
                    } else {
                        97
                    };
                }
                95 => {
                    bb = 93;
                }
                96 => {
                    v44 = (v45 >= 5_i32);
                    bb = 97;
                }
                97 => {
                    self.w32(
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(7400_i32) as u32),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(7400_i32) as u32),
                        ) as i32)
                            .wrapping_add(1) as u32),
                    );
                    v32 = self.r32(fp.wrapping_add(28));
                    bb = if v44 { 98 } else { 99 };
                }
                98 => {
                    {
                        let a0 = self.r32(fp.wrapping_add(28)).wrapping_add(
                            ({
                                let t3 = (self.r32(self.r32(fp.wrapping_add(28)).wrapping_add(2072))
                                    as i32);
                                self.w32(
                                    self.r32(fp.wrapping_add(28)).wrapping_add(2072),
                                    (t3.wrapping_add(1) as u32),
                                );
                                t3
                            }
                            .wrapping_add(393_i32) as u32)
                                .wrapping_mul(4),
                        );
                        let a1 = (v45 as u32);
                        self.w32(a0, a1)
                    };
                    bb = 99;
                }
                99 => {
                    bb = 95;
                }
                100 => {
                    let t4 = v39;
                    bb = match t4 {
                        1_i32 => 103,
                        2_i32 => 104,
                        4_i32 => 105,
                        8_i32 => 106,
                        _ => 107,
                    };
                }
                101 => {
                    v46 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(
                                4_i32.wrapping_mul((self.r32(fp.wrapping_add(16)) as i32)),
                            )
                            .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v46 == (1_i32).wrapping_neg()) {
                        115
                    } else {
                        116
                    };
                }
                102 => {
                    bb = 113;
                }
                103 => {
                    v46 = 0_i32;
                    bb = 102;
                }
                104 => {
                    v46 = 1_i32;
                    bb = 102;
                }
                105 => {
                    v46 = 2_i32;
                    bb = 102;
                }
                106 => {
                    v46 = 3_i32;
                    bb = 102;
                }
                107 => {
                    v46 = (self.r32(fp.wrapping_add(64)) as i32);
                    bb = 102;
                }
                108 => {
                    bb = 104;
                }
                109 => {
                    bb = 105;
                }
                110 => {
                    bb = 106;
                }
                111 => {
                    bb = 107;
                }
                112 => {
                    bb = 102;
                }
                113 => {
                    v51 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(
                                4_i32.wrapping_mul((self.r32(fp.wrapping_add(16)) as i32)),
                            )
                            .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        137
                    } else {
                        138
                    };
                }
                114 => {
                    bb = 101;
                }
                115 => {
                    v46 = (self.r32(fp.wrapping_add(80)) as i32);
                    bb = 116;
                }
                116 => {
                    v47 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(36)) as i32));
                    v48 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v49 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(36)) as i32))
                        .wrapping_abs();
                    v50 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    bb = if (v50 == v49) { 117 } else { 119 };
                }
                117 => {
                    bb = if (!(v46 != 0)) { 120 } else { 121 };
                }
                118 => {
                    v46 = (if (v48 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 113;
                }
                119 => {
                    bb = if (v50 < v49) { 143 } else { 144 };
                }
                120 => {
                    bb = if (v47 > 0_i32) { 122 } else { 123 };
                }
                121 => {
                    bb = if (v46 == 1_i32) { 125 } else { 126 };
                }
                122 => {
                    v46 = (if (v48 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 123;
                }
                123 => {
                    bb = 113;
                }
                124 => {
                    bb = 121;
                }
                125 => {
                    bb = if (v48 < 0_i32) { 127 } else { 128 };
                }
                126 => {
                    bb = if (v46 != 2_i32) { 130 } else { 131 };
                }
                127 => {
                    v46 = (if (v47 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 128;
                }
                128 => {
                    bb = 113;
                }
                129 => {
                    bb = 126;
                }
                130 => {
                    bb = if ((v46 == 3_i32) && (v48 > 0_i32)) {
                        132
                    } else {
                        133
                    };
                }
                131 => {
                    bb = if (v47 >= 0_i32) { 135 } else { 136 };
                }
                132 => {
                    v46 = (if (v47 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 133;
                }
                133 => {
                    bb = 113;
                }
                134 => {
                    bb = 131;
                }
                135 => {
                    bb = 113;
                }
                136 => {
                    bb = 118;
                }
                137 => {
                    v51 = (self.r32(fp.wrapping_add(80)) as i32);
                    bb = 138;
                }
                138 => {
                    bb = if (v46 == v51) { 139 } else { 140 };
                }
                139 => {
                    v46 = (1_i32).wrapping_neg();
                    bb = 140;
                }
                140 => {
                    self.w32(v32.wrapping_add(1540), 2_u32);
                    self.w32(
                        v32.wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        v32.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(36)) as i32) as u32),
                    );
                    self.w32(
                        v32.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(v32.wrapping_add(1556), (v46 as u32));
                    bb = 141;
                }
                141 => {
                    v69 = (self.r32(v32.wrapping_add(2072)) as i32);
                    v70 = 0_i32;
                    v71 = (v69 == 0_i32);
                    v72 = (v69 < 0_i32);
                    self.w32(v32.wrapping_add(1464), 1_u32);
                    self.w32(
                        v32.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    self.w32(v32.wrapping_add(1472), 2_u32);
                    self.w32(v32.wrapping_add(1476), 1_u32);
                    bb = if (v69 > 0_i32) { 171 } else { 172 };
                }
                142 => {
                    bb = 136;
                }
                143 => {
                    v46 = (if (v47 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 113;
                }
                144 => {
                    bb = 118;
                }
                145 => {
                    bb = 144;
                }
                146 => {
                    bb = 31;
                }
                147 => {
                    bb = if (((self.r32(v52) as i32) == 1_i32)
                        && ((self.r32(v52.wrapping_add(116)) as i32) != 99_i32))
                    {
                        150
                    } else {
                        151
                    };
                }
                148 => {
                    bb = if (v54 < 20_i32) { 147 } else { 149 };
                }
                149 => {
                    self.w32(fp.wrapping_add(0), (v53 as u32));
                    v56 = 0_i32;
                    v57 = 9999_i32;
                    self.w32(fp.wrapping_add(4), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        152
                    } else {
                        153
                    };
                }
                150 => {
                    self.w32(v55, (v54 as u32));
                    v53 = v53.wrapping_add(1);
                    v55 = v55.wrapping_add(4);
                    bb = 151;
                }
                151 => {
                    v52 = v52.wrapping_add(156);
                    v54 = v54.wrapping_add(1);
                    bb = 148;
                }
                152 => {
                    v58 = fp.wrapping_add(92);
                    bb = 154;
                }
                153 => {
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(72)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(92).wrapping_add(
                                                ((self.r32(fp.wrapping_add(4)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v61 = 0_i32;
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(72)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(92).wrapping_add(
                                                ((self.r32(fp.wrapping_add(4)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3728_i32) as u32),
                        ) as i32) as u32),
                    );
                    v62 = (self.r32((v3.wrapping_add(6440_i32) as u32)) as i32);
                    self.w32(fp.wrapping_add(20), 0_u32);
                    bb = if (v62 > 0_i32) { 159 } else { 160 };
                }
                154 => {
                    v59 = ((self.r32(
                        ((self.r32(fp.wrapping_add(72)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v58) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v60 = ((((((v59 as u64) >> 32) as u32) as i64) ^ v59)
                        .wrapping_sub(((((v59 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(72)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v58) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(36)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v60 < v57) { 157 } else { 158 };
                }
                155 => {
                    bb = if (v56 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        154
                    } else {
                        156
                    };
                }
                156 => {
                    bb = 153;
                }
                157 => {
                    v57 = v60;
                    self.w32(fp.wrapping_add(4), (v56 as u32));
                    bb = 158;
                }
                158 => {
                    v56 = v56.wrapping_add(1);
                    v58 = v58.wrapping_add(4);
                    bb = 155;
                }
                159 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(292),
                        (v3.wrapping_add(4840_i32) as u32),
                        (4_i32.wrapping_mul(v62) as u32),
                    );
                    v61 = v62;
                    self.w32(fp.wrapping_add(20), (v62 as u32));
                    bb = 160;
                }
                160 => {
                    self.w32(fp.wrapping_add(0), 9999_u32);
                    self.w32(fp.wrapping_add(32), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v61 > 0_i32) { 161 } else { 162 };
                }
                161 => {
                    self.w32(fp.wrapping_add(8), fp.wrapping_add(292));
                    bb = 163;
                }
                162 => {
                    v68 = (self.r32(fp.wrapping_add(292).wrapping_add(
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    v32 = self.r32(fp.wrapping_add(28));
                    self.w32(self.r32(fp.wrapping_add(28)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(28)).wrapping_add(1552),
                        (v68 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(28)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(28)).wrapping_add(1544),
                        (crem_i32(v68, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(28)).wrapping_add(1548),
                        (cdiv_i32(v68, 20_i32) as u32),
                    );
                    bb = 141;
                }
                163 => {
                    v63 = ((self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(8))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v64 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(8))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v65 = (v64
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v66 = (v63.wrapping_sub(v65) as i32).wrapping_abs();
                    v67 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(8))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v64.wrapping_add((self.r32(fp.wrapping_add(36)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v65.wrapping_add(v63)))
                        as i32);
                    bb = if (v67 >= (self.r32(fp.wrapping_add(0)) as i32)) {
                        166
                    } else {
                        168
                    };
                }
                164 => {
                    bb = if v13 { 163 } else { 165 };
                }
                165 => {
                    bb = 162;
                }
                166 => {
                    bb = if ((v67 == (self.r32(fp.wrapping_add(0)) as i32))
                        && (v66 < (self.r32(fp.wrapping_add(32)) as i32)))
                    {
                        169
                    } else {
                        170
                    };
                }
                167 => {
                    v13 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(20)) as i32));
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 164;
                }
                168 => {
                    self.w32(fp.wrapping_add(0), (v67 as u32));
                    self.w32(fp.wrapping_add(32), (v66 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 167;
                }
                169 => {
                    self.w32(fp.wrapping_add(32), (v66 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 170;
                }
                170 => {
                    bb = 167;
                }
                171 => {
                    v73 = v32.wrapping_add(1572);
                    bb = 173;
                }
                172 => {
                    self.w8(a2, ((!v72) && (!v71)) as u8);
                    return 0_i32;
                }
                173 => {
                    v74 = ((self.r32(fp.wrapping_add(72)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(v73) as i32)))
                        .wrapping_add(2820_i32) as u32);
                    bb = if (((self.r32(v74) as i32) == 1_i32)
                        && ((self.r32(
                            ((self.r32(fp.wrapping_add(72)) as i32)
                                .wrapping_add(156_i32.wrapping_mul((self.r32(v73) as i32)))
                                .wrapping_add(2936_i32) as u32),
                        ) as i32)
                            != 99_i32))
                    {
                        176
                    } else {
                        177
                    };
                }
                174 => {
                    bb = if (v70 < v75) { 173 } else { 175 };
                }
                175 => {
                    v71 = (v75 == 0_i32);
                    v72 = (v75 < 0_i32);
                    bb = 172;
                }
                176 => {
                    self.w32(
                        ((self.r32(fp.wrapping_add(72)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v73) as i32)))
                            .wrapping_add(2960_i32) as u32),
                        51_u32,
                    );
                    self.w32(v74.wrapping_add(144), 5_u32);
                    bb = 177;
                }
                177 => {
                    v75 = (self.r32(v32.wrapping_add(2072)) as i32);
                    v70 = v70.wrapping_add(1);
                    v73 = v73.wrapping_add(4);
                    bb = 174;
                }
                178 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001D680` (1096 bytes).
    pub(crate) fn f_1001d680(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(112);
        let r = self.f_1001d680_body(fp, this, a2);
        self.leave(112);
        r
    }

    fn f_1001d680_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v11: u32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: u32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = ((self.r32(self.r32(this)) as i32) as u32);
                    v3 = a2;
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(v2.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v2.wrapping_add(28)) as i32) as u32),
                    );
                    v4 = (self.r32(v2.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(self.r32(fp.wrapping_add(40)).wrapping_add(1388)) as i32)
                            as u32),
                    );
                    v5 = ((self.r32(
                        (v4.wrapping_add(
                            716_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)),
                        )
                        .wrapping_add(380_i32) as u32),
                    ) as i32) as u32);
                    v6 = 0_i32;
                    self.w32(fp.wrapping_add(12), v5);
                    v7 = v4
                        .wrapping_add(716_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)));
                    self.w32(fp.wrapping_add(4), 0_u32);
                    v8 = ((self.r32((v7.wrapping_add(384_i32) as u32)) as i32) as u32);
                    self.w32(fp.wrapping_add(16), v8);
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32((v7.wrapping_add(388_i32) as u32)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        v5.wrapping_add(20_u32.wrapping_mul(v8)),
                    );
                    v9 = a2;
                    bb = 1;
                }
                1 => {
                    bb = if (1_i32 != 0) { 2 } else { 3 };
                }
                2 => {
                    self.w32(
                        fp.wrapping_add(72)
                            .wrapping_add((v6 as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    self.w32(
                        fp.wrapping_add(72)
                            .wrapping_add((v6.wrapping_add(4_i32) as u32).wrapping_mul(4)),
                        ((self.r32(fp.wrapping_add(24)) as i32) as u32),
                    );
                    v10 = v8;
                    bb = 4;
                }
                3 => {
                    j = 0_i32;
                    bb = 66;
                }
                4 => {
                    bb = if ((v5 < 20_u32) && (v10 < 20_u32)) {
                        5
                    } else {
                        6
                    };
                }
                5 => {
                    v11 = v5.wrapping_add(20_u32.wrapping_mul(v10));
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v11))
                            .wrapping_add(3200_u32),
                    ) as i32)
                        == (1_i32).wrapping_neg())
                    {
                        7
                    } else {
                        8
                    };
                }
                6 => {
                    self.w32(
                        fp.wrapping_add(4),
                        ({
                            let t5 = v6.wrapping_add(1);
                            v6 = t5;
                            t5
                        } as u32),
                    );
                    bb = if (v6 >= 4_i32) { 63 } else { 64 };
                }
                7 => {
                    self.w32(
                        fp.wrapping_add(72)
                            .wrapping_add((v6.wrapping_add(4_i32) as u32).wrapping_mul(4)),
                        ((v11 as i32) as u32),
                    );
                    bb = 8;
                }
                8 => {
                    i = 0_i32;
                    bb = 9;
                }
                9 => {
                    bb = if (i < 3_i32) { 10 } else { 12 };
                }
                10 => {
                    let t1 = v6;
                    bb = match t1 {
                        0_i32 => 14,
                        2_i32 => 15,
                        1_i32 => 16,
                        3_i32 => 17,
                        _ => 18,
                    };
                }
                11 => {
                    i = i.wrapping_add(1);
                    bb = 9;
                }
                12 => {
                    let t4 = v6;
                    bb = match t4 {
                        0_i32 => 53,
                        1_i32 => 54,
                        2_i32 => 55,
                        3_i32 => 56,
                        _ => 57,
                    };
                }
                13 => {
                    bb = if ((v3 < 20_u32) && (v9 < 20_u32)) {
                        48
                    } else {
                        49
                    };
                }
                14 => {
                    bb = 15;
                }
                15 => {
                    let t2 = i;
                    bb = match t2 {
                        0_i32 => 20,
                        1_i32 => 21,
                        2_i32 => 22,
                        _ => 19,
                    };
                }
                16 => {
                    let t3 = i;
                    bb = match t3 {
                        0_i32 => 30,
                        1_i32 => 31,
                        2_i32 => 32,
                        _ => 29,
                    };
                }
                17 => {
                    bb = if (i != 0) { 38 } else { 40 };
                }
                18 => {
                    bb = 13;
                }
                19 => {
                    bb = 13;
                }
                20 => {
                    bb = 23;
                }
                21 => {
                    v3 = v5.wrapping_sub(1_u32);
                    bb = 25;
                }
                22 => {
                    v3 = v5.wrapping_add(1_u32);
                    bb = 25;
                }
                23 => {
                    v3 = v5;
                    bb = 25;
                }
                24 => {
                    bb = 21;
                }
                25 => {
                    v9 = v10;
                    bb = 39;
                }
                26 => {
                    bb = 22;
                }
                27 => {
                    bb = 19;
                }
                28 => {
                    bb = 16;
                }
                29 => {
                    bb = 13;
                }
                30 => {
                    bb = 23;
                }
                31 => {
                    bb = 34;
                }
                32 => {
                    v3 = v5;
                    v9 = v10.wrapping_add(1_u32);
                    bb = 29;
                }
                33 => {
                    bb = 31;
                }
                34 => {
                    v3 = v5;
                    v9 = v10.wrapping_sub(1_u32);
                    bb = 42;
                }
                35 => {
                    bb = 32;
                }
                36 => {
                    bb = 29;
                }
                37 => {
                    bb = 17;
                }
                38 => {
                    bb = if (i == 1_i32) { 41 } else { 43 };
                }
                39 => {
                    bb = 13;
                }
                40 => {
                    bb = 23;
                }
                41 => {
                    bb = 34;
                }
                42 => {
                    bb = 39;
                }
                43 => {
                    bb = if (i == 2_i32) { 44 } else { 45 };
                }
                44 => {
                    v3 = v5;
                    v9 = v10.wrapping_add(1_u32);
                    bb = 45;
                }
                45 => {
                    bb = 42;
                }
                46 => {
                    bb = 18;
                }
                47 => {
                    bb = 13;
                }
                48 => {
                    v6 = (self.r32(fp.wrapping_add(4)) as i32);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32)
                            .wrapping_add(
                                4_u32.wrapping_mul(v3.wrapping_add(20_u32.wrapping_mul(v9))),
                            )
                            .wrapping_add(3200_u32),
                    ) as i32)
                        > 4_i32)
                    {
                        50
                    } else {
                        51
                    };
                }
                49 => {
                    bb = 11;
                }
                50 => {
                    self.w32(
                        fp.wrapping_add(72).wrapping_add(
                            ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                        ),
                        ((self.r32(fp.wrapping_add(72).wrapping_add(
                            ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                        )) as i32)
                            .wrapping_add(1) as u32),
                    );
                    bb = 51;
                }
                51 => {
                    bb = 49;
                }
                52 => {
                    bb = 4;
                }
                53 => {
                    v10 = v10.wrapping_sub(1);
                    bb = 52;
                }
                54 => {
                    v5 = v5.wrapping_add(1);
                    bb = 52;
                }
                55 => {
                    v10 = v10.wrapping_add(1);
                    bb = 52;
                }
                56 => {
                    v5 = v5.wrapping_sub(1);
                    bb = 52;
                }
                57 => {
                    bb = 4;
                }
                58 => {
                    bb = 54;
                }
                59 => {
                    bb = 55;
                }
                60 => {
                    bb = 56;
                }
                61 => {
                    bb = 57;
                }
                62 => {
                    bb = 52;
                }
                63 => {
                    bb = 3;
                }
                64 => {
                    v5 = self.r32(fp.wrapping_add(12));
                    v8 = self.r32(fp.wrapping_add(16));
                    bb = 1;
                }
                65 => {
                    bb = 64;
                }
                66 => {
                    bb = if (j < 4_i32) { 67 } else { 69 };
                }
                67 => {
                    v14 = 10_i32.wrapping_mul(
                        (self.r32(fp.wrapping_add(72).wrapping_add((j as u32).wrapping_mul(4)))
                            as i32),
                    );
                    self.w32(
                        fp.wrapping_add(56).wrapping_add((j as u32).wrapping_mul(4)),
                        (v14 as u32),
                    );
                    bb = if (j == (self.r32(fp.wrapping_add(20)) as i32)) {
                        70
                    } else {
                        71
                    };
                }
                68 => {
                    j = j.wrapping_add(1);
                    bb = 66;
                }
                69 => {
                    v15 = (self.r32(fp.wrapping_add(56)) as i32);
                    v16 = 0_i32;
                    bb = if ((self.r32(fp.wrapping_add(56)) as i32)
                        < (self.r32(fp.wrapping_add(60)) as i32))
                    {
                        72
                    } else {
                        73
                    };
                }
                70 => {
                    self.w32(
                        fp.wrapping_add(56).wrapping_add((j as u32).wrapping_mul(4)),
                        (v14.wrapping_add(1_i32) as u32),
                    );
                    bb = 71;
                }
                71 => {
                    bb = 68;
                }
                72 => {
                    v16 = 1_i32;
                    v15 = (self.r32(fp.wrapping_add(60)) as i32);
                    bb = 73;
                }
                73 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(64)) as i32)) {
                        74
                    } else {
                        75
                    };
                }
                74 => {
                    v16 = 2_i32;
                    v15 = (self.r32(fp.wrapping_add(64)) as i32);
                    bb = 75;
                }
                75 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(68)) as i32)) {
                        76
                    } else {
                        77
                    };
                }
                76 => {
                    v16 = 3_i32;
                    bb = 77;
                }
                77 => {
                    let t6 = v16;
                    bb = match t6 {
                        0_i32 => 79,
                        1_i32 => 80,
                        2_i32 => 81,
                        3_i32 => 82,
                        _ => 83,
                    };
                }
                78 => {
                    v17 = self.r32(fp.wrapping_add(12));
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(
                            fp.wrapping_add(72).wrapping_add(
                                ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(4_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    v18 = self.r32(fp.wrapping_add(16));
                    v19 = (self.r32(fp.wrapping_add(8)) as i32);
                    self.w32(
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(7400_i32) as u32),
                        0_u32,
                    );
                    bb = 89;
                }
                79 => {
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = 78;
                }
                80 => {
                    self.w32(fp.wrapping_add(0), 1_u32);
                    bb = 78;
                }
                81 => {
                    self.w32(fp.wrapping_add(0), 2_u32);
                    bb = 78;
                }
                82 => {
                    self.w32(fp.wrapping_add(0), 3_u32);
                    bb = 78;
                }
                83 => {
                    bb = 78;
                }
                84 => {
                    bb = 80;
                }
                85 => {
                    bb = 81;
                }
                86 => {
                    bb = 82;
                }
                87 => {
                    bb = 83;
                }
                88 => {
                    bb = 78;
                }
                89 => {
                    bb = if ((v17 < 20_u32) && (v18 < 20_u32)) {
                        90
                    } else {
                        91
                    };
                }
                90 => {
                    v20 = 0_i32;
                    self.w32(fp.wrapping_add(4), 0_u32);
                    bb = 92;
                }
                91 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32)
                        == (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        145
                    } else {
                        146
                    };
                }
                92 => {
                    let t7 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = match t7 {
                        0_i32 => 96,
                        2_i32 => 97,
                        1_i32 => 98,
                        3_i32 => 99,
                        _ => 100,
                    };
                }
                93 => {
                    bb = if (v20 < 3_i32) { 92 } else { 94 };
                }
                94 => {
                    let t12 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = match t12 {
                        0_i32 => 135,
                        1_i32 => 136,
                        2_i32 => 137,
                        3_i32 => 138,
                        _ => 139,
                    };
                }
                95 => {
                    bb = if ((v3 < 20_u32) && (v9 < 20_u32)) {
                        130
                    } else {
                        131
                    };
                }
                96 => {
                    bb = 97;
                }
                97 => {
                    let t8 = v20;
                    bb = match t8 {
                        0_i32 => 102,
                        1_i32 => 103,
                        2_i32 => 104,
                        _ => 101,
                    };
                }
                98 => {
                    let t9 = v20;
                    bb = match t9 {
                        0_i32 => 112,
                        1_i32 => 113,
                        2_i32 => 114,
                        _ => 111,
                    };
                }
                99 => {
                    bb = if (v20 != 0) { 120 } else { 122 };
                }
                100 => {
                    bb = 95;
                }
                101 => {
                    bb = 95;
                }
                102 => {
                    bb = 105;
                }
                103 => {
                    v3 = v17.wrapping_sub(1_u32);
                    bb = 107;
                }
                104 => {
                    v3 = v17.wrapping_add(1_u32);
                    bb = 107;
                }
                105 => {
                    v3 = v17;
                    bb = 107;
                }
                106 => {
                    bb = 103;
                }
                107 => {
                    v9 = v18;
                    bb = 121;
                }
                108 => {
                    bb = 104;
                }
                109 => {
                    bb = 101;
                }
                110 => {
                    bb = 98;
                }
                111 => {
                    bb = 95;
                }
                112 => {
                    bb = 105;
                }
                113 => {
                    bb = 116;
                }
                114 => {
                    v3 = v17;
                    v9 = v18.wrapping_add(1_u32);
                    bb = 111;
                }
                115 => {
                    bb = 113;
                }
                116 => {
                    v3 = v17;
                    v9 = v18.wrapping_sub(1_u32);
                    bb = 124;
                }
                117 => {
                    bb = 114;
                }
                118 => {
                    bb = 111;
                }
                119 => {
                    bb = 99;
                }
                120 => {
                    bb = if (v20 == 1_i32) { 123 } else { 125 };
                }
                121 => {
                    bb = 95;
                }
                122 => {
                    bb = 105;
                }
                123 => {
                    bb = 116;
                }
                124 => {
                    bb = 121;
                }
                125 => {
                    bb = if (v20 == 2_i32) { 126 } else { 127 };
                }
                126 => {
                    v3 = v17;
                    v9 = v18.wrapping_add(1_u32);
                    bb = 127;
                }
                127 => {
                    bb = 124;
                }
                128 => {
                    bb = 100;
                }
                129 => {
                    bb = 95;
                }
                130 => {
                    self.w32(
                        fp.wrapping_add(32),
                        v3.wrapping_add(20_u32.wrapping_mul(v9)),
                    );
                    {
                        let a0 = (v19
                            .wrapping_add(4_i32.wrapping_mul({
                                let t10 = (self.r32((v19.wrapping_add(7400_i32) as u32)) as i32);
                                self.w32(
                                    (v19.wrapping_add(7400_i32) as u32),
                                    (t10.wrapping_add(1) as u32),
                                );
                                t10
                            }))
                            .wrapping_add(6800_i32) as u32);
                        let a1 =
                            ((self.r32(fp.wrapping_add(32)).wrapping_add(400_u32) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    v21 = (self.r32(
                        (v19 as u32)
                            .wrapping_add(4_u32.wrapping_mul(self.r32(fp.wrapping_add(32))))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v21 > 4_i32) { 132 } else { 133 };
                }
                131 => {
                    self.w32(
                        fp.wrapping_add(4),
                        ({
                            let t11 = v20.wrapping_add(1);
                            v20 = t11;
                            t11
                        } as u32),
                    );
                    bb = 93;
                }
                132 => {
                    let _ = self.f_10009c80(
                        this,
                        (self.r32(fp.wrapping_add(28)) as i32),
                        v21.wrapping_sub(5_i32),
                        62_i32,
                    );
                    v19 = (self.r32(fp.wrapping_add(8)) as i32);
                    bb = 133;
                }
                133 => {
                    v20 = (self.r32(fp.wrapping_add(4)) as i32);
                    bb = 131;
                }
                134 => {
                    bb = 89;
                }
                135 => {
                    v18 = v18.wrapping_sub(1);
                    bb = 134;
                }
                136 => {
                    v17 = v17.wrapping_add(1);
                    bb = 134;
                }
                137 => {
                    v18 = v18.wrapping_add(1);
                    bb = 134;
                }
                138 => {
                    v17 = v17.wrapping_sub(1);
                    bb = 134;
                }
                139 => {
                    bb = 89;
                }
                140 => {
                    bb = 136;
                }
                141 => {
                    bb = 137;
                }
                142 => {
                    bb = 138;
                }
                143 => {
                    bb = 139;
                }
                144 => {
                    bb = 134;
                }
                145 => {
                    self.w32(fp.wrapping_add(0), ((1_i32).wrapping_neg() as u32));
                    bb = 146;
                }
                146 => {
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(24)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(1556),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(28)) as i32) as u32),
                    );
                    v22 = (self.r32(self.r32(fp.wrapping_add(40)).wrapping_add(2072)) as i32);
                    self.w32(self.r32(fp.wrapping_add(40)).wrapping_add(1540), 1_u32);
                    self.w32(self.r32(fp.wrapping_add(40)).wrapping_add(1464), 1_u32);
                    self.w32(self.r32(fp.wrapping_add(40)).wrapping_add(1472), 2_u32);
                    self.w32(self.r32(fp.wrapping_add(40)).wrapping_add(1476), 3_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(1500),
                        (((v22 > 0_i32) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(2324),
                        (crem_i32((self.r32(fp.wrapping_add(52)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(2328),
                        (cdiv_i32((self.r32(fp.wrapping_add(52)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(40)).wrapping_add(2332),
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32),
                    );
                    self.w8(a2, 1_u8);
                    return 0_i32;
                }
                147 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001DB20` (2591 bytes).
    pub(crate) fn f_1001db20(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_1001db20_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_1001db20_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(88), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(40), (v4 as u32));
                    self.w32(fp.wrapping_add(80), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(84),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(212),
                        (fp.wrapping_add(100) as i32),
                        (self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(96), 0_i32, 0_i32, 6_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(self.r32(fp.wrapping_add(96))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(self.r32(fp.wrapping_add(96)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(self.r32(fp.wrapping_add(96)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(72),
                        self.r32(fp.wrapping_add(96)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(self.r32(fp.wrapping_add(96)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(232));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(112));
                    v7 = ((self.r32(fp.wrapping_add(36)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(35), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(64)) as i32));
                    self.w8(fp.wrapping_add(34), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(76)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(72)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(76)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(35)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(64)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(64)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(35), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(56)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(34), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(34)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(24));
                            self.w32(fp.wrapping_add(24), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(112);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(36)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(112);
                    bb = 149;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(36)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(36)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(56)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(72));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(112).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(60)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(36)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(60)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(52)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(36)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(52)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(64)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(76)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(24), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(332)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(24)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(24), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(80)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(52)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(332));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(40)) as i32);
                    v37 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(332).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_add(crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v39 = (self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    self.w32(fp.wrapping_add(52), (v38 as u32));
                    v40 = v38.wrapping_add((self.r32(fp.wrapping_add(64)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v38.wrapping_add((self.r32(fp.wrapping_add(64)) as i32)) as u32),
                    );
                    bb = if (v39 < v39.wrapping_add((self.r32(fp.wrapping_add(76)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(56)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v39) as u32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(76)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(28)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(28), (v44 as u32));
                    self.w32(fp.wrapping_add(60), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(72)));
                    v45 = v38.wrapping_add((self.r32(fp.wrapping_add(64)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v38.wrapping_add((self.r32(fp.wrapping_add(64)) as i32)) as u32),
                    );
                    bb = if (v39 < v39.wrapping_add((self.r32(fp.wrapping_add(76)) as i32))) {
                        84
                    } else {
                        85
                    };
                }
                70 => {
                    v42 = (v38 as u32);
                    bb = if (v38 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v36 = (self.r32(fp.wrapping_add(40)) as i32);
                    v38 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v43 != (1_i32).wrapping_neg())
                        && (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    self.w32(fp.wrapping_add(28), ((self.r32(v37) as i32) as u32));
                    bb = 83;
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    v46 = (20_i32.wrapping_mul(v39) as u32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(76)) as i32) as u32),
                    );
                    bb = 86;
                }
                85 => {
                    bb = if (v44 != 15_i32) { 102 } else { 103 };
                }
                86 => {
                    v47 = (v38 as u32);
                    bb = if (v38 < v45) { 89 } else { 90 };
                }
                87 => {
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32) != 0) {
                        86
                    } else {
                        88
                    };
                }
                88 => {
                    v44 = (self.r32(fp.wrapping_add(28)) as i32);
                    bb = 85;
                }
                89 => {
                    bb = 91;
                }
                90 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 87;
                }
                91 => {
                    bb = if (((((self.r32(fp.wrapping_add(60)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        94
                    } else {
                        95
                    };
                }
                92 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        91
                    } else {
                        93
                    };
                }
                93 => {
                    v38 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 90;
                }
                94 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                95 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 92;
                }
                96 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    bb = 95;
                }
                98 => {
                    v50 = (v49 >= 5_i32);
                    bb = 99;
                }
                99 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 100 } else { 101 };
                }
                100 => {
                    let _ = self.f_10009c80(
                        this,
                        (self.r32(fp.wrapping_add(80)) as i32),
                        v49.wrapping_sub(5_i32),
                        63_i32,
                    );
                    bb = 101;
                }
                101 => {
                    bb = 97;
                }
                102 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 105,
                        2_i32 => 106,
                        4_i32 => 107,
                        8_i32 => 108,
                        _ => 109,
                    };
                }
                103 => {
                    v51 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(
                                4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                            )
                            .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        117
                    } else {
                        118
                    };
                }
                104 => {
                    bb = 115;
                }
                105 => {
                    v51 = 0_i32;
                    bb = 104;
                }
                106 => {
                    v51 = 1_i32;
                    bb = 104;
                }
                107 => {
                    v51 = 2_i32;
                    bb = 104;
                }
                108 => {
                    v51 = 3_i32;
                    bb = 104;
                }
                109 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 104;
                }
                110 => {
                    bb = 106;
                }
                111 => {
                    bb = 107;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 104;
                }
                115 => {
                    v56 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(
                                4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                            )
                            .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        139
                    } else {
                        140
                    };
                }
                116 => {
                    bb = 103;
                }
                117 => {
                    v51 = (self.r32(fp.wrapping_add(84)) as i32);
                    bb = 118;
                }
                118 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 119 } else { 121 };
                }
                119 => {
                    bb = if (!(v51 != 0)) { 122 } else { 123 };
                }
                120 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 115;
                }
                121 => {
                    bb = if (v55 < v54) { 145 } else { 146 };
                }
                122 => {
                    bb = if (v52 > 0_i32) { 124 } else { 125 };
                }
                123 => {
                    bb = if (v51 == 1_i32) { 127 } else { 128 };
                }
                124 => {
                    v51 = (((((v51 as u32) & 0xFFFFFF00)
                        | (((((v53 >= 0_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v51 = (v51.wrapping_sub(1_i32) & 2_i32).wrapping_add(1_i32);
                    bb = 125;
                }
                125 => {
                    bb = 115;
                }
                126 => {
                    bb = 123;
                }
                127 => {
                    bb = if (v53 < 0_i32) { 129 } else { 130 };
                }
                128 => {
                    bb = if (v51 != 2_i32) { 132 } else { 133 };
                }
                129 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 130;
                }
                130 => {
                    bb = 115;
                }
                131 => {
                    bb = 128;
                }
                132 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        134
                    } else {
                        135
                    };
                }
                133 => {
                    bb = if (v52 >= 0_i32) { 137 } else { 138 };
                }
                134 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 135;
                }
                135 => {
                    bb = 115;
                }
                136 => {
                    bb = 133;
                }
                137 => {
                    bb = 115;
                }
                138 => {
                    bb = 120;
                }
                139 => {
                    v56 = (self.r32(fp.wrapping_add(84)) as i32);
                    bb = 140;
                }
                140 => {
                    bb = if (v51 == v56) { 141 } else { 142 };
                }
                141 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 142;
                }
                142 => {
                    v57 = self.r32(fp.wrapping_add(68));
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(68)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 143;
                }
                143 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1472), 2_u32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1476), 8_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                144 => {
                    bb = 138;
                }
                145 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 115;
                }
                146 => {
                    bb = 120;
                }
                147 => {
                    bb = 146;
                }
                148 => {
                    bb = 31;
                }
                149 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        152
                    } else {
                        153
                    };
                }
                150 => {
                    bb = if (v60 < 20_i32) { 149 } else { 151 };
                }
                151 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 153;
                }
                153 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 150;
                }
                154 => {
                    v64 = fp.wrapping_add(112);
                    bb = 156;
                }
                155 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(36)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(112).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(36)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(112).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 161 } else { 162 };
                }
                156 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(36)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(36)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(56)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 159 } else { 160 };
                }
                157 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        156
                    } else {
                        158
                    };
                }
                158 => {
                    bb = 155;
                }
                159 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 160;
                }
                160 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 157;
                }
                161 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(332),
                        self.r32(fp.wrapping_add(88)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 162;
                }
                162 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 163 } else { 164 };
                }
                163 => {
                    self.w32(fp.wrapping_add(28), fp.wrapping_add(332));
                    bb = 165;
                }
                164 => {
                    v75 = (self.r32(fp.wrapping_add(332).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(68)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(68)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(68));
                    bb = 143;
                }
                165 => {
                    v70 = ((self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(28))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        168
                    } else {
                        170
                    };
                }
                166 => {
                    bb = if v35 { 165 } else { 167 };
                }
                167 => {
                    bb = 164;
                }
                168 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        171
                    } else {
                        172
                    };
                }
                169 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(28),
                        self.r32(fp.wrapping_add(28)).wrapping_add(4),
                    );
                    bb = 166;
                }
                170 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 169;
                }
                171 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 172;
                }
                172 => {
                    bb = 169;
                }
                173 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001E560` (2593 bytes).
    pub(crate) fn f_1001e560(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_1001e560_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_1001e560_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(92), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(52), (v4 as u32));
                    self.w32(fp.wrapping_add(84), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(216),
                        (fp.wrapping_add(104) as i32),
                        (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(100), 0_i32, 0_i32, 3_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(100))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(100)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(32), fp.wrapping_add(236));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(116));
                    v7 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(39), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w8(fp.wrapping_add(31), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(39)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(39), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(32));
                            self.w32(fp.wrapping_add(32), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(116);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(116);
                    bb = 151;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(76));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(116).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(32), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(336)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(32), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(336));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    v37 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(336).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    v39 = crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    v40 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(24), (v44 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v45 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        86
                    } else {
                        87
                    };
                }
                70 => {
                    v42 = (v39 as u32);
                    bb = if (v39 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v43 != (1_i32).wrapping_neg()) {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    bb = if (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)) {
                        84
                    } else {
                        85
                    };
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v37) as i32) as u32));
                    bb = 85;
                }
                85 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 83;
                }
                86 => {
                    v46 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 88;
                }
                87 => {
                    bb = if (v44 != 15_i32) { 104 } else { 105 };
                }
                88 => {
                    v47 = (v39 as u32);
                    bb = if (v39 < v45) { 91 } else { 92 };
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v44 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 87;
                }
                91 => {
                    bb = 93;
                }
                92 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                93 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                95 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 92;
                }
                96 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 94;
                }
                98 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        100
                    } else {
                        101
                    };
                }
                99 => {
                    bb = 97;
                }
                100 => {
                    v50 = (v49 >= 5_i32);
                    bb = 101;
                }
                101 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 102 } else { 103 };
                }
                102 => {
                    let _ = self.f_10009e90(
                        this,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v49.wrapping_sub(5_i32),
                        64_i32,
                    );
                    bb = 103;
                }
                103 => {
                    bb = 99;
                }
                104 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 107,
                        2_i32 => 108,
                        4_i32 => 109,
                        8_i32 => 110,
                        _ => 111,
                    };
                }
                105 => {
                    v51 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        119
                    } else {
                        120
                    };
                }
                106 => {
                    bb = 117;
                }
                107 => {
                    v51 = 0_i32;
                    bb = 106;
                }
                108 => {
                    v51 = 1_i32;
                    bb = 106;
                }
                109 => {
                    v51 = 2_i32;
                    bb = 106;
                }
                110 => {
                    v51 = 3_i32;
                    bb = 106;
                }
                111 => {
                    v51 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 106;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 110;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    v56 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        141
                    } else {
                        142
                    };
                }
                118 => {
                    bb = 105;
                }
                119 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 120;
                }
                120 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 121 } else { 123 };
                }
                121 => {
                    bb = if (!(v51 != 0)) { 124 } else { 125 };
                }
                122 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 117;
                }
                123 => {
                    bb = if (v55 < v54) { 147 } else { 148 };
                }
                124 => {
                    bb = if (v52 > 0_i32) { 126 } else { 127 };
                }
                125 => {
                    bb = if (v51 == 1_i32) { 129 } else { 130 };
                }
                126 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 127;
                }
                127 => {
                    bb = 117;
                }
                128 => {
                    bb = 125;
                }
                129 => {
                    bb = if (v53 < 0_i32) { 131 } else { 132 };
                }
                130 => {
                    bb = if (v51 != 2_i32) { 134 } else { 135 };
                }
                131 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 132;
                }
                132 => {
                    bb = 117;
                }
                133 => {
                    bb = 130;
                }
                134 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        136
                    } else {
                        137
                    };
                }
                135 => {
                    bb = if (v52 >= 0_i32) { 139 } else { 140 };
                }
                136 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 137;
                }
                137 => {
                    bb = 117;
                }
                138 => {
                    bb = 135;
                }
                139 => {
                    bb = 117;
                }
                140 => {
                    bb = 122;
                }
                141 => {
                    v56 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 142;
                }
                142 => {
                    bb = if (v51 == v56) { 143 } else { 144 };
                }
                143 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 144;
                }
                144 => {
                    v57 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 145;
                }
                145 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1472), 5_u32);
                    self.w32(v57.wrapping_add(1476), 4_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                146 => {
                    bb = 140;
                }
                147 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 117;
                }
                148 => {
                    bb = 122;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 31;
                }
                151 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v60 < 20_i32) { 151 } else { 153 };
                }
                153 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 155;
                }
                155 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 152;
                }
                156 => {
                    v64 = fp.wrapping_add(116);
                    bb = 158;
                }
                157 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(116).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(116).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 163 } else { 164 };
                }
                158 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 161 } else { 162 };
                }
                159 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                160 => {
                    bb = 157;
                }
                161 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 162;
                }
                162 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 159;
                }
                163 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(336),
                        self.r32(fp.wrapping_add(92)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 164;
                }
                164 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 165 } else { 166 };
                }
                165 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(336));
                    bb = 167;
                }
                166 => {
                    v75 = (self.r32(fp.wrapping_add(336).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(72));
                    bb = 145;
                }
                167 => {
                    v70 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        170
                    } else {
                        172
                    };
                }
                168 => {
                    bb = if v35 { 167 } else { 169 };
                }
                169 => {
                    bb = 166;
                }
                170 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                171 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 168;
                }
                172 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 171;
                }
                173 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 174;
                }
                174 => {
                    bb = 171;
                }
                175 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001EFB0` (2593 bytes).
    pub(crate) fn f_1001efb0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_1001efb0_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_1001efb0_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(92), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(52), (v4 as u32));
                    self.w32(fp.wrapping_add(84), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(216),
                        (fp.wrapping_add(104) as i32),
                        (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(100), 0_i32, 0_i32, 5_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(100))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(100)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(32), fp.wrapping_add(236));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(116));
                    v7 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(39), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w8(fp.wrapping_add(31), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(39)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(39), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(32));
                            self.w32(fp.wrapping_add(32), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(116);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(116);
                    bb = 151;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(76));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(116).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(32), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(336)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(32), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(336));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    v37 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(336).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    v39 = crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    v40 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(24), (v44 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v45 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        86
                    } else {
                        87
                    };
                }
                70 => {
                    v42 = (v39 as u32);
                    bb = if (v39 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v43 != (1_i32).wrapping_neg()) {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    bb = if (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)) {
                        84
                    } else {
                        85
                    };
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v37) as i32) as u32));
                    bb = 85;
                }
                85 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 83;
                }
                86 => {
                    v46 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 88;
                }
                87 => {
                    bb = if (v44 != 15_i32) { 104 } else { 105 };
                }
                88 => {
                    v47 = (v39 as u32);
                    bb = if (v39 < v45) { 91 } else { 92 };
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v44 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 87;
                }
                91 => {
                    bb = 93;
                }
                92 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                93 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                95 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 92;
                }
                96 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 94;
                }
                98 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        100
                    } else {
                        101
                    };
                }
                99 => {
                    bb = 97;
                }
                100 => {
                    v50 = (v49 >= 5_i32);
                    bb = 101;
                }
                101 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 102 } else { 103 };
                }
                102 => {
                    let _ = self.f_10009e90(
                        this,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v49.wrapping_sub(5_i32),
                        65_i32,
                    );
                    bb = 103;
                }
                103 => {
                    bb = 99;
                }
                104 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 107,
                        2_i32 => 108,
                        4_i32 => 109,
                        8_i32 => 110,
                        _ => 111,
                    };
                }
                105 => {
                    v51 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        119
                    } else {
                        120
                    };
                }
                106 => {
                    bb = 117;
                }
                107 => {
                    v51 = 0_i32;
                    bb = 106;
                }
                108 => {
                    v51 = 1_i32;
                    bb = 106;
                }
                109 => {
                    v51 = 2_i32;
                    bb = 106;
                }
                110 => {
                    v51 = 3_i32;
                    bb = 106;
                }
                111 => {
                    v51 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 106;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 110;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    v56 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        141
                    } else {
                        142
                    };
                }
                118 => {
                    bb = 105;
                }
                119 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 120;
                }
                120 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 121 } else { 123 };
                }
                121 => {
                    bb = if (!(v51 != 0)) { 124 } else { 125 };
                }
                122 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 117;
                }
                123 => {
                    bb = if (v55 < v54) { 147 } else { 148 };
                }
                124 => {
                    bb = if (v52 > 0_i32) { 126 } else { 127 };
                }
                125 => {
                    bb = if (v51 == 1_i32) { 129 } else { 130 };
                }
                126 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 127;
                }
                127 => {
                    bb = 117;
                }
                128 => {
                    bb = 125;
                }
                129 => {
                    bb = if (v53 < 0_i32) { 131 } else { 132 };
                }
                130 => {
                    bb = if (v51 != 2_i32) { 134 } else { 135 };
                }
                131 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 132;
                }
                132 => {
                    bb = 117;
                }
                133 => {
                    bb = 130;
                }
                134 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        136
                    } else {
                        137
                    };
                }
                135 => {
                    bb = if (v52 >= 0_i32) { 139 } else { 140 };
                }
                136 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 137;
                }
                137 => {
                    bb = 117;
                }
                138 => {
                    bb = 135;
                }
                139 => {
                    bb = 117;
                }
                140 => {
                    bb = 122;
                }
                141 => {
                    v56 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 142;
                }
                142 => {
                    bb = if (v51 == v56) { 143 } else { 144 };
                }
                143 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 144;
                }
                144 => {
                    v57 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 145;
                }
                145 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1472), 6_u32);
                    self.w32(v57.wrapping_add(1476), 4_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                146 => {
                    bb = 140;
                }
                147 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 117;
                }
                148 => {
                    bb = 122;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 31;
                }
                151 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v60 < 20_i32) { 151 } else { 153 };
                }
                153 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 155;
                }
                155 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 152;
                }
                156 => {
                    v64 = fp.wrapping_add(116);
                    bb = 158;
                }
                157 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(116).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(116).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 163 } else { 164 };
                }
                158 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 161 } else { 162 };
                }
                159 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                160 => {
                    bb = 157;
                }
                161 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 162;
                }
                162 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 159;
                }
                163 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(336),
                        self.r32(fp.wrapping_add(92)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 164;
                }
                164 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 165 } else { 166 };
                }
                165 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(336));
                    bb = 167;
                }
                166 => {
                    v75 = (self.r32(fp.wrapping_add(336).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(72));
                    bb = 145;
                }
                167 => {
                    v70 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        170
                    } else {
                        172
                    };
                }
                168 => {
                    bb = if v35 { 167 } else { 169 };
                }
                169 => {
                    bb = 166;
                }
                170 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                171 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 168;
                }
                172 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 171;
                }
                173 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 174;
                }
                174 => {
                    bb = 171;
                }
                175 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1001FA00` (2593 bytes).
    pub(crate) fn f_1001fa00(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1936);
        let r = self.f_1001fa00_body(fp, this, a2);
        self.leave(1936);
        r
    }

    fn f_1001fa00_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i64 = 0;
        let mut v19: i32 = 0;
        let mut v20: u32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: u32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: u32 = 0;
        let mut v49: i32 = 0;
        let mut v50: bool = false;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: u32 = 0;
        let mut v62: i32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i64 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v3 = ((self.r32(self.r32(this)) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(v3.wrapping_add(28)) as i32) as u32),
                    );
                    v5 = (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(92), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v3.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(52), (v4 as u32));
                    self.w32(fp.wrapping_add(84), (v5 as u32));
                    v6 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(v6.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v6.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v6.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(216),
                        (fp.wrapping_add(104) as i32),
                        (self.r32(self.r32(fp.wrapping_add(72)).wrapping_add(1384)) as i32),
                        v5,
                        0_i32,
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(100), 0_i32, 7_i32, 0_i32, 1_i32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(100))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        self.r32(fp.wrapping_add(100)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(100)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(32), fp.wrapping_add(236));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(116));
                    v7 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v7) as i32) == 1_i32)
                        && ((self.r32(v7.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32) < 20_i32) {
                        1
                    } else {
                        3
                    };
                }
                3 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                4 => {
                    v8 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(v7.wrapping_add(128)) as i32));
                    self.w8(fp.wrapping_add(39), 0_u8);
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w8(fp.wrapping_add(31), 0_u8);
                    self.w32(
                        fp.wrapping_add(20),
                        (v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    self.w32(fp.wrapping_add(48), (v9 as u32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v7 = v7.wrapping_add(156);
                    self.w32(fp.wrapping_add(16), v7);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v12 = (v8 as u32);
                    bb = if (v8 < v10) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = if (((self.r8(fp.wrapping_add(39)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = 12;
                }
                16 => {
                    bb = if ((v11 < 400_u32) && (v12 < 20_u32)) {
                        18
                    } else {
                        19
                    };
                }
                17 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    bb = 14;
                }
                18 => {
                    v13 = (4_u32.wrapping_mul(v11.wrapping_add(v12)) as i32);
                    bb = if ((self.r32((v13.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                19 => {
                    bb = 17;
                }
                20 => {
                    v14 = (self.r32((v13.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v14 == (1_i32).wrapping_neg()) || (v14 == v5)) {
                        22
                    } else {
                        23
                    };
                }
                21 => {
                    bb = 19;
                }
                22 => {
                    self.w8(fp.wrapping_add(39), 1_u8);
                    bb = if ((v12 == ((self.r32(fp.wrapping_add(44)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(48)) as i32)
                            == (self.r32(fp.wrapping_add(60)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 25;
                }
                25 => {
                    bb = 23;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(32));
                            self.w32(fp.wrapping_add(32), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(12)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 29;
                }
                29 => {
                    v7 = self.r32(fp.wrapping_add(16));
                    bb = 7;
                }
                30 => {
                    v15 = 0_i32;
                    v16 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v17 = fp.wrapping_add(116);
                    bb = 32;
                }
                31 => {
                    v58 = ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(3600_i32) as u32);
                    v59 = 0_i32;
                    v60 = 0_i32;
                    v61 = fp.wrapping_add(116);
                    bb = 151;
                }
                32 => {
                    v18 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v19 = ((((((v18 as u64) >> 32) as u32) as i64) ^ v18)
                        .wrapping_sub(((((v18 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v17) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v19 < v16) { 35 } else { 36 };
                }
                33 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        32
                    } else {
                        34
                    };
                }
                34 => {
                    v20 = self.r32(fp.wrapping_add(76));
                    v21 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(116).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add(156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)))
                        .wrapping_add(3600_i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(64)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32((v22.wrapping_add(128_i32) as u32)) as i32) as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(56)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    v25 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        < v25)
                    {
                        37
                    } else {
                        38
                    };
                }
                35 => {
                    v16 = v19;
                    self.w32(fp.wrapping_add(0), (v15 as u32));
                    bb = 36;
                }
                36 => {
                    v15 = v15.wrapping_add(1);
                    v17 = v17.wrapping_add(4);
                    bb = 33;
                }
                37 => {
                    v26 = (20_i32.wrapping_mul(v24) as u32);
                    self.w32(fp.wrapping_add(32), (v25.wrapping_sub(v24) as u32));
                    bb = 39;
                }
                38 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v21 > 0_i32) { 58 } else { 59 };
                }
                39 => {
                    bb = if (1_i32 != 0) { 40 } else { 41 };
                }
                40 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(16), (v21 as u32));
                    bb = 38;
                }
                42 => {
                    v27 = fp
                        .wrapping_add(336)
                        .wrapping_add((v21 as u32).wrapping_mul(4));
                    bb = 44;
                }
                43 => {
                    v26 = v26.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(32), (t2 as u32));
                        t2
                    } != 0))
                    {
                        55
                    } else {
                        56
                    };
                }
                44 => {
                    bb = if ((self.r32(v20) as i32) != 0) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = if ((v26 < 400_u32) && ((v23 as u32) < 20_u32)) {
                        49
                    } else {
                        50
                    };
                }
                48 => {
                    v20 = v20.wrapping_add(4);
                    v23 = v23.wrapping_add(1);
                    bb = 45;
                }
                49 => {
                    v28 = (v23 as u32).wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul((v23 as u32).wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        51
                    } else {
                        52
                    };
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                52 => {
                    bb = 50;
                }
                53 => {
                    self.w32(v27, v28);
                    v21 = v21.wrapping_add(1);
                    v27 = v27.wrapping_add(4);
                    bb = 54;
                }
                54 => {
                    bb = 52;
                }
                55 => {
                    bb = 41;
                }
                56 => {
                    v23 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    bb = 39;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), fp.wrapping_add(336));
                    bb = 60;
                }
                59 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    v37 = self.r32(fp.wrapping_add(76));
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(336).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    v38 = (self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32));
                    v39 = crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32)
                        .wrapping_add((self.r32(fp.wrapping_add(56)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        (crem_i32((self.r32(fp.wrapping_add(20)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), (v39 as u32));
                    v40 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        68
                    } else {
                        69
                    };
                }
                60 => {
                    v30 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v31 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v32 = (v30
                        .wrapping_add((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v33 = (v31.wrapping_sub(v32) as i32).wrapping_abs();
                    v34 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(20))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(v30)
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v32.wrapping_add(v31)))
                        as i32);
                    bb = if (v34 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        63
                    } else {
                        65
                    };
                }
                61 => {
                    bb = if v35 { 60 } else { 62 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    bb = if ((v34 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v33 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        66
                    } else {
                        67
                    };
                }
                64 => {
                    v35 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(4),
                    );
                    bb = 61;
                }
                65 => {
                    self.w32(fp.wrapping_add(8), (v34 as u32));
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 64;
                }
                66 => {
                    self.w32(fp.wrapping_add(4), (v33 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 67;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v41 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 70;
                }
                69 => {
                    self.w32((v36.wrapping_add(7400_i32) as u32), 0_u32);
                    v44 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    self.w32(fp.wrapping_add(24), (v44 as u32));
                    self.w32(fp.wrapping_add(64), (16_i32.wrapping_mul(v44) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(76)));
                    v45 = v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(0),
                        (v39.wrapping_add((self.r32(fp.wrapping_add(68)) as i32)) as u32),
                    );
                    bb = if (v38 < v38.wrapping_add((self.r32(fp.wrapping_add(80)) as i32))) {
                        86
                    } else {
                        87
                    };
                }
                70 => {
                    v42 = (v39 as u32);
                    bb = if (v39 < v40) { 73 } else { 74 };
                }
                71 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        70
                    } else {
                        72
                    };
                }
                72 => {
                    bb = 69;
                }
                73 => {
                    bb = 75;
                }
                74 => {
                    v41 = v41.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 71;
                }
                75 => {
                    bb = if ((self.r32(v37) as i32) != 0) {
                        78
                    } else {
                        79
                    };
                }
                76 => {
                    bb = if ((v42 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        75
                    } else {
                        77
                    };
                }
                77 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 74;
                }
                78 => {
                    bb = if ((v41 < 400_u32) && (v42 < 20_u32)) {
                        80
                    } else {
                        81
                    };
                }
                79 => {
                    v40 = (self.r32(fp.wrapping_add(0)) as i32);
                    v37 = v37.wrapping_add(4);
                    v42 = v42.wrapping_add(1);
                    bb = 76;
                }
                80 => {
                    v43 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v41.wrapping_add(v42)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v43 != (1_i32).wrapping_neg()) {
                        82
                    } else {
                        83
                    };
                }
                81 => {
                    bb = 79;
                }
                82 => {
                    bb = if (v43.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(64)) as i32)) {
                        84
                    } else {
                        85
                    };
                }
                83 => {
                    bb = 81;
                }
                84 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v37) as i32) as u32));
                    bb = 85;
                }
                85 => {
                    v36 = (self.r32(fp.wrapping_add(52)) as i32);
                    bb = 83;
                }
                86 => {
                    v46 = (20_i32.wrapping_mul(v38) as u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    bb = 88;
                }
                87 => {
                    bb = if (v44 != 15_i32) { 104 } else { 105 };
                }
                88 => {
                    v47 = (v39 as u32);
                    bb = if (v39 < v45) { 91 } else { 92 };
                }
                89 => {
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) != 0) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v44 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 87;
                }
                91 => {
                    bb = 93;
                }
                92 => {
                    v46 = v46.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                93 => {
                    bb = if (((((self.r32(fp.wrapping_add(64)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        != 0_i32)
                        && (v46 < 400_u32))
                        && (v47 < 20_u32))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    bb = if ((v47 as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                95 => {
                    v39 = (self.r32(fp.wrapping_add(56)) as i32);
                    bb = 92;
                }
                96 => {
                    v48 = v46.wrapping_add(v47);
                    bb = if (v46.wrapping_add(v47)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                    {
                        98
                    } else {
                        99
                    };
                }
                97 => {
                    v45 = (self.r32(fp.wrapping_add(0)) as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 94;
                }
                98 => {
                    v49 = (self.r32(
                        (v36 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v48))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v50 = (0_i32 != 0);
                    self.w32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v36.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v48.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v49 != (1_i32).wrapping_neg()) {
                        100
                    } else {
                        101
                    };
                }
                99 => {
                    bb = 97;
                }
                100 => {
                    v50 = (v49 >= 5_i32);
                    bb = 101;
                }
                101 => {
                    self.w32(
                        (v36.wrapping_add(7400_i32) as u32),
                        ((self.r32((v36.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if v50 { 102 } else { 103 };
                }
                102 => {
                    let _ = self.f_10009e90(
                        this,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v49.wrapping_sub(5_i32),
                        66_i32,
                    );
                    bb = 103;
                }
                103 => {
                    bb = 99;
                }
                104 => {
                    let t4 = v44;
                    bb = match t4 {
                        1_i32 => 107,
                        2_i32 => 108,
                        4_i32 => 109,
                        8_i32 => 110,
                        _ => 111,
                    };
                }
                105 => {
                    v51 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v51 == (1_i32).wrapping_neg()) {
                        119
                    } else {
                        120
                    };
                }
                106 => {
                    bb = 117;
                }
                107 => {
                    v51 = 0_i32;
                    bb = 106;
                }
                108 => {
                    v51 = 1_i32;
                    bb = 106;
                }
                109 => {
                    v51 = 2_i32;
                    bb = 106;
                }
                110 => {
                    v51 = 3_i32;
                    bb = 106;
                }
                111 => {
                    v51 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 106;
                }
                112 => {
                    bb = 108;
                }
                113 => {
                    bb = 109;
                }
                114 => {
                    bb = 110;
                }
                115 => {
                    bb = 111;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    v56 = (self.r32(
                        (v36.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(20)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v56 == (1_i32).wrapping_neg()) {
                        141
                    } else {
                        142
                    };
                }
                118 => {
                    bb = 105;
                }
                119 => {
                    v51 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 120;
                }
                120 => {
                    v52 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32));
                    v53 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32));
                    v54 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs();
                    v55 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs();
                    bb = if (v55 == v54) { 121 } else { 123 };
                }
                121 => {
                    bb = if (!(v51 != 0)) { 124 } else { 125 };
                }
                122 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 117;
                }
                123 => {
                    bb = if (v55 < v54) { 147 } else { 148 };
                }
                124 => {
                    bb = if (v52 > 0_i32) { 126 } else { 127 };
                }
                125 => {
                    bb = if (v51 == 1_i32) { 129 } else { 130 };
                }
                126 => {
                    v51 = (if (v53 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 127;
                }
                127 => {
                    bb = 117;
                }
                128 => {
                    bb = 125;
                }
                129 => {
                    bb = if (v53 < 0_i32) { 131 } else { 132 };
                }
                130 => {
                    bb = if (v51 != 2_i32) { 134 } else { 135 };
                }
                131 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 132;
                }
                132 => {
                    bb = 117;
                }
                133 => {
                    bb = 130;
                }
                134 => {
                    bb = if ((v51 == 3_i32) && (v53 > 0_i32)) {
                        136
                    } else {
                        137
                    };
                }
                135 => {
                    bb = if (v52 >= 0_i32) { 139 } else { 140 };
                }
                136 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 137;
                }
                137 => {
                    bb = 117;
                }
                138 => {
                    bb = 135;
                }
                139 => {
                    bb = 117;
                }
                140 => {
                    bb = 122;
                }
                141 => {
                    v56 = (self.r32(fp.wrapping_add(88)) as i32);
                    bb = 142;
                }
                142 => {
                    bb = if (v51 == v56) { 143 } else { 144 };
                }
                143 => {
                    v51 = (1_i32).wrapping_neg();
                    bb = 144;
                }
                144 => {
                    v57 = self.r32(fp.wrapping_add(72));
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(44)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        (v51 as u32),
                    );
                    bb = 145;
                }
                145 => {
                    self.w32(
                        v57.wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(84)) as i32) as u32),
                    );
                    v76 = (self.r32(v57.wrapping_add(2072)) as i32);
                    self.w32(v57.wrapping_add(1464), 1_u32);
                    self.w32(v57.wrapping_add(1472), 7_u32);
                    self.w32(v57.wrapping_add(1476), 4_u32);
                    self.w32(v57.wrapping_add(1500), (((v76 > 0_i32) as i32) as u32));
                    self.w8(a2, (v76 > 0_i32) as u8);
                    return 0_i32;
                }
                146 => {
                    bb = 140;
                }
                147 => {
                    v51 = (if (v52 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 117;
                }
                148 => {
                    bb = 122;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 31;
                }
                151 => {
                    bb = if (((self.r32(v58) as i32) == 1_i32)
                        && ((self.r32(v58.wrapping_add(116)) as i32) != 99_i32))
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v60 < 20_i32) { 151 } else { 153 };
                }
                153 => {
                    self.w32(fp.wrapping_add(4), (v59 as u32));
                    v62 = 0_i32;
                    v63 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    self.w32(v61, (v60 as u32));
                    v59 = v59.wrapping_add(1);
                    v61 = v61.wrapping_add(4);
                    bb = 155;
                }
                155 => {
                    v58 = v58.wrapping_add(156);
                    v60 = v60.wrapping_add(1);
                    bb = 152;
                }
                156 => {
                    v64 = fp.wrapping_add(116);
                    bb = 158;
                }
                157 => {
                    v67 = (self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul(
                                (self.r32(fp.wrapping_add(116).wrapping_add(
                                    ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                                )) as i32),
                            ))
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(116).wrapping_add(
                                                ((self.r32(fp.wrapping_add(0)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v68 = 0_i32;
                    v69 = (self
                        .r32(((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(16), 0_u32);
                    bb = if (v69 > 0_i32) { 163 } else { 164 };
                }
                158 => {
                    v65 = ((self.r32(
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        as i64);
                    v66 = ((((((v65 as u64) >> 32) as u32) as i64) ^ v65)
                        .wrapping_sub(((((v65 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(40)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v64) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v66 < v63) { 161 } else { 162 };
                }
                159 => {
                    bb = if (v62 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                160 => {
                    bb = 157;
                }
                161 => {
                    v63 = v66;
                    self.w32(fp.wrapping_add(0), (v62 as u32));
                    bb = 162;
                }
                162 => {
                    v62 = v62.wrapping_add(1);
                    v64 = v64.wrapping_add(4);
                    bb = 159;
                }
                163 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(336),
                        self.r32(fp.wrapping_add(92)),
                        (4_i32.wrapping_mul(v69) as u32),
                    );
                    v68 = v69;
                    self.w32(fp.wrapping_add(16), (v69 as u32));
                    bb = 164;
                }
                164 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v68 > 0_i32) { 165 } else { 166 };
                }
                165 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(336));
                    bb = 167;
                }
                166 => {
                    v75 = (self.r32(fp.wrapping_add(336).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(72)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1552),
                        (v75 as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1544),
                        (crem_i32(v75, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(72)).wrapping_add(1548),
                        (cdiv_i32(v75, 20_i32) as u32),
                    );
                    v57 = self.r32(fp.wrapping_add(72));
                    bb = 145;
                }
                167 => {
                    v70 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v71 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v72 = (v71.wrapping_add(v67).wrapping_abs() as u32);
                    v73 = (v70.wrapping_sub(v72) as i32).wrapping_abs();
                    v74 = (((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v71.wrapping_add((self.r32(fp.wrapping_add(60)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v72.wrapping_add(v70)))
                        as i32);
                    bb = if (v74 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        170
                    } else {
                        172
                    };
                }
                168 => {
                    bb = if v35 { 167 } else { 169 };
                }
                169 => {
                    bb = 166;
                }
                170 => {
                    bb = if ((v74 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v73 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                171 => {
                    v35 = ({
                        let t5 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(16)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 168;
                }
                172 => {
                    self.w32(fp.wrapping_add(8), (v74 as u32));
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 171;
                }
                173 => {
                    self.w32(fp.wrapping_add(4), (v73 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 174;
                }
                174 => {
                    bb = 171;
                }
                175 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }
}
