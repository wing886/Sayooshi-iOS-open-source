//! Functions 0x100123C0..=0x10015E00, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_100123C0` (278 bytes).
    pub(crate) fn f_100123c0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100123c0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100123c0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 2_u32);
        self.w32(v3.wrapping_add(1476), 2_u32);
        self.w32(v3.wrapping_add(1456), 9_u32);
        self.w32(v3.wrapping_add(1472), 9_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        let _ = self.f_10006f50(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(544);
                'l1: loop {
                    if (!((self.r32(v9) as i32) != 0)) {
                        break 'l1;
                    }
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    if !(v8 < 1_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(136_i32) as u32).wrapping_mul(4)),
                    28_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(137_i32) as u32).wrapping_mul(4)),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
            let _ = self.f_100074e0(
                this,
                (self.r32(v3.wrapping_add(1452)) as i32),
                (self.r32(v3.wrapping_add(1456)) as i32),
                (self.r32(v3.wrapping_add(1460)) as i32),
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            self.w32(v3.wrapping_add(1480), 0_u32);
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100124E0` (203 bytes).
    pub(crate) fn f_100124e0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1460), 3_u32);
        self.w32(v2.wrapping_add(1476), 3_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1456), 9_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1472), 9_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                20_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_100125B0` (203 bytes).
    pub(crate) fn f_100125b0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1456), 9_u32);
        self.w32(v2.wrapping_add(1472), 9_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1460), 4_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1476), 4_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                21_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012680` (276 bytes).
    pub(crate) fn f_10012680(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10012680_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10012680_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1456), 10_u32);
        self.w32(v3.wrapping_add(1472), 10_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 0_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 0_u32);
        let _ = self.f_10006bf0(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(528);
                'l1: loop {
                    if (!((self.r32(v9) as i32) != 0)) {
                        break 'l1;
                    }
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    if !(v8 < 2_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(132_i32) as u32).wrapping_mul(4)),
                    25_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(134_i32) as u32).wrapping_mul(4)),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
            let _ = self.f_100074e0(
                this,
                (self.r32(v3.wrapping_add(1452)) as i32),
                (self.r32(v3.wrapping_add(1456)) as i32),
                (self.r32(v3.wrapping_add(1460)) as i32),
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            self.w32(v3.wrapping_add(1480), 0_u32);
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100127A0` (266 bytes).
    pub(crate) fn f_100127a0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100127a0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100127a0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1456), 10_u32);
        self.w32(v3.wrapping_add(1472), 10_u32);
        self.w32(fp.wrapping_add(2), this);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1476), 1_u32);
        let _ = self.f_10006f50(this, v4, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v6 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
            if (((self.r32(v6) as i32) == 1_i32)
                && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
            {
                v7 = 0_i32;
                v8 = v6.wrapping_add(544);
                'l1: loop {
                    if (!((self.r32(v8) as i32) != 0)) {
                        break 'l1;
                    }
                    v7 = v7.wrapping_add(1);
                    v8 = v8.wrapping_add(4);
                    if !(v7 < 1_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(136_i32) as u32).wrapping_mul(4)),
                    28_u32,
                );
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(137_i32) as u32).wrapping_mul(4)),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
            let _ = self.f_100074e0(
                self.r32(fp.wrapping_add(2)),
                (self.r32(v3.wrapping_add(1452)) as i32),
                (self.r32(v3.wrapping_add(1456)) as i32),
                (self.r32(v3.wrapping_add(1460)) as i32),
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            self.w32(v3.wrapping_add(1480), 0_u32);
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100128B0` (203 bytes).
    pub(crate) fn f_100128b0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1460), 2_u32);
        self.w32(v2.wrapping_add(1476), 2_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1456), 10_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1472), 10_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                20_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012980` (216 bytes).
    pub(crate) fn f_10012980(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1460), 3_u32);
        self.w32(v2.wrapping_add(1476), 3_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1456), 10_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1472), 10_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(568);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 2_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(142_i32) as u32).wrapping_mul(4)),
                34_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(144_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(146_i32) as u32).wrapping_mul(4)),
                ((self.r32(v2.wrapping_add(180)) as i32) as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012A60` (203 bytes).
    pub(crate) fn f_10012a60(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1456), 10_u32);
        self.w32(v2.wrapping_add(1472), 10_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1460), 4_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1476), 4_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                21_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012B30` (276 bytes).
    pub(crate) fn f_10012b30(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10012b30_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10012b30_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1456), 11_u32);
        self.w32(v3.wrapping_add(1472), 11_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 0_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 0_u32);
        let _ = self.f_10006bf0(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(528);
                'l1: loop {
                    if (!((self.r32(v9) as i32) != 0)) {
                        break 'l1;
                    }
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    if !(v8 < 2_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(132_i32) as u32).wrapping_mul(4)),
                    25_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(134_i32) as u32).wrapping_mul(4)),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
            let _ = self.f_100074e0(
                this,
                (self.r32(v3.wrapping_add(1452)) as i32),
                (self.r32(v3.wrapping_add(1456)) as i32),
                (self.r32(v3.wrapping_add(1460)) as i32),
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            self.w32(v3.wrapping_add(1480), 0_u32);
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10012C50` (189 bytes).
    pub(crate) fn f_10012c50(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1456), 11_u32);
        self.w32(v2.wrapping_add(1472), 11_u32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1460), 1_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1476), 1_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                20_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012D10` (216 bytes).
    pub(crate) fn f_10012d10(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1456), 11_u32);
        self.w32(v2.wrapping_add(1472), 11_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1460), 2_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1476), 2_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(568);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 2_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(142_i32) as u32).wrapping_mul(4)),
                34_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(144_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(146_i32) as u32).wrapping_mul(4)),
                ((self.r32(v2.wrapping_add(180)) as i32) as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012DF0` (203 bytes).
    pub(crate) fn f_10012df0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1460), 3_u32);
        self.w32(v2.wrapping_add(1476), 3_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1456), 11_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1472), 11_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                21_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012EC0` (192 bytes).
    pub(crate) fn f_10012ec0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1456), 11_u32);
        self.w32(v2.wrapping_add(1472), 11_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1460), 4_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1476), 4_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(552);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 1_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(138_i32) as u32).wrapping_mul(4)),
                31_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(139_i32) as u32).wrapping_mul(4)),
                1_u32,
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10012F80` (184 bytes).
    pub(crate) fn f_10012f80(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10012f80_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10012f80_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        v5 = (self.r32(v4.wrapping_add(1388)) as i32);
        self.w32(v4.wrapping_add(1460), 0_u32);
        self.w32(v4.wrapping_add(1476), 0_u32);
        self.w32(v4.wrapping_add(1456), 12_u32);
        self.w32(v4.wrapping_add(1472), 12_u32);
        self.w32(v4.wrapping_add(1448), 1_u32);
        self.w32(v4.wrapping_add(1452), (v5 as u32));
        self.w32(v4.wrapping_add(1464), 1_u32);
        self.w32(v4.wrapping_add(1468), (v5 as u32));
        let _ = self.f_100073c0(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        v6 = ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) as i32);
        if (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32) {
            v6 = v6.wrapping_add(1);
        }
        v7 = (v3.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
        if (((self.r32(v7) as i32) == 1_i32) && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v7.wrapping_add(708), 67_u32);
            self.w32(v7.wrapping_add(712), (v6.wrapping_add(1_i32) as u32));
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_10013040` (261 bytes).
    pub(crate) fn f_10013040(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v10: i32 = 0;
        v1 = this;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1456), 12_u32);
        self.w32(v3.wrapping_add(1472), 12_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 1_u32);
        let _ = self.f_10016ca0(this);
        v10 = 0_i32;
        if ((self.r32(v3.wrapping_add(2072)) as i32) > 0_i32) {
            v6 = v3.wrapping_add(1572);
            'l1: loop {
                v7 = (self.r32(v6) as i32).wrapping_sub(5_i32);
                v8 = crem_i32(self.rand(), 3_i32).wrapping_add(1_i32);
                if (((self.r32(
                    (v4.wrapping_add(156_i32.wrapping_mul(v7))
                        .wrapping_add(3600_i32) as u32),
                ) as i32)
                    == 1_i32)
                    && ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul(v7))
                            .wrapping_add(3716_i32) as u32),
                    ) as i32)
                        != 99_i32))
                {
                    self.w32(
                        (v4.wrapping_add(156_i32.wrapping_mul(v7))
                            .wrapping_add(3740_i32) as u32),
                        53_u32,
                    );
                    self.w32(
                        (v4.wrapping_add(156_i32.wrapping_mul(v7))
                            .wrapping_add(3744_i32) as u32),
                        (v8 as u32),
                    );
                }
                v6 = v6.wrapping_add(4);
                v10 = v10.wrapping_add(1);
                if !(v10 < (self.r32(v3.wrapping_add(2072)) as i32)) {
                    break 'l1;
                }
            }
            v1 = this;
        }
        let _ = self.f_100074e0(
            v1,
            (self.r32(v3.wrapping_add(1452)) as i32),
            (self.r32(v3.wrapping_add(1456)) as i32),
            (self.r32(v3.wrapping_add(1460)) as i32),
        );
        self.w32(v3.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10013150` (149 bytes).
    pub(crate) fn f_10013150(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 12_u32);
        self.w32(v3.wrapping_add(1460), 2_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 12_u32);
        self.w32(v3.wrapping_add(1476), 2_u32);
        v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v5.wrapping_add(708), 71_u32);
            self.w32(v5.wrapping_add(712), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_100131F0` (189 bytes).
    pub(crate) fn f_100131f0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100131f0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100131f0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        v5 = (self.r32(v4.wrapping_add(1388)) as i32);
        self.w32(v4.wrapping_add(1460), 3_u32);
        self.w32(v4.wrapping_add(1476), 3_u32);
        self.w32(v4.wrapping_add(1456), 12_u32);
        self.w32(v4.wrapping_add(1472), 12_u32);
        self.w32(v4.wrapping_add(1448), 1_u32);
        self.w32(v4.wrapping_add(1452), (v5 as u32));
        self.w32(v4.wrapping_add(1464), 1_u32);
        self.w32(v4.wrapping_add(1468), (v5 as u32));
        let _ = self.f_100073c0(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        v6 = ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) as i32);
        if (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32) {
            v6 = v6.wrapping_add(1);
        }
        v7 = (v3.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
        if (((self.r32(v7) as i32) == 1_i32) && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v7.wrapping_add(708), 68_u32);
            self.w32(v7.wrapping_add(712), (v6.wrapping_add(3_i32) as u32));
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_100132B0` (149 bytes).
    pub(crate) fn f_100132b0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 12_u32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 12_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v5.wrapping_add(708), 72_u32);
            self.w32(v5.wrapping_add(712), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_10013350` (176 bytes).
    pub(crate) fn f_10013350(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1456), 13_u32);
        self.w32(v3.wrapping_add(1472), 13_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1460), 0_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1476), 0_u32);
        v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) != 1_i32) || ((self.r32(v5.wrapping_add(372)) as i32) == 99_i32))
        {
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            v6 = 0_i32;
            v7 = v5.wrapping_add(644);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 3_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(161_i32) as u32).wrapping_mul(4)),
                46_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(164_i32) as u32).wrapping_mul(4)),
                1_u32,
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10013400` (172 bytes).
    pub(crate) fn f_10013400(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1456), 13_u32);
        self.w32(v3.wrapping_add(1472), 13_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1476), 1_u32);
        v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) != 1_i32) || ((self.r32(v5.wrapping_add(372)) as i32) == 99_i32))
        {
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            v6 = 0_i32;
            v7 = v5.wrapping_add(644);
            'l1: loop {
                if (!((self.r32(v7) as i32) != 0)) {
                    break 'l1;
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < 3_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v5.wrapping_add((v6.wrapping_add(161_i32) as u32).wrapping_mul(4)),
                47_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(164_i32) as u32).wrapping_mul(4)),
                3_u32,
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100134B0` (275 bytes).
    pub(crate) fn f_100134b0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100134b0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100134b0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 2_u32);
        self.w32(v3.wrapping_add(1476), 2_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1456), 13_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1472), 13_u32);
        let _ = self.f_10007070(this, v5, fp.wrapping_add(0));
        if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v6 = (self.r32((v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32)) as i32);
            v7 = v4.wrapping_add(716_i32.wrapping_mul(v5));
            if ((v6 == 1_i32) && ((self.r32((v7.wrapping_add(372_i32) as u32)) as i32) != 99_i32)) {
                v8 = 0_i32;
                v9 = (v7.wrapping_add(408_i32) as u32);
                'l1: loop {
                    if (!((self.r32(v9) as i32) != 0)) {
                        break 'l1;
                    }
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    if !(v8 < 15_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    (v7.wrapping_add(4_i32.wrapping_mul(v8))
                        .wrapping_add(408_i32) as u32),
                    22_u32,
                );
                self.w32(
                    (v7.wrapping_add(4_i32.wrapping_mul(v8))
                        .wrapping_add(468_i32) as u32),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
            let _ = self.f_100074e0(
                this,
                (self.r32(v3.wrapping_add(1452)) as i32),
                (self.r32(v3.wrapping_add(1456)) as i32),
                (self.r32(v3.wrapping_add(1460)) as i32),
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            if (!((self.r8(fp.wrapping_add(0)) as i8) != 0)) {
                self.w32(v3.wrapping_add(1480), 0_u32);
            }
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100135D0` (213 bytes).
    pub(crate) fn f_100135d0(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 3_u32);
        self.w32(v3.wrapping_add(1476), 3_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1456), 13_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1472), 13_u32);
        let _ = self.f_10016ca0(this);
        v6 = 0_i32;
        if ((self.r32(v3.wrapping_add(2072)) as i32) > 0_i32) {
            v7 = v3.wrapping_add(1572);
            'l1: loop {
                v8 = (v4
                    .wrapping_add(156_i32.wrapping_mul((self.r32(v7) as i32)))
                    .wrapping_add(2820_i32) as u32);
                if (((self.r32(v8) as i32) == 1_i32)
                    && ((self.r32(
                        (v4.wrapping_add(156_i32.wrapping_mul((self.r32(v7) as i32)))
                            .wrapping_add(2936_i32) as u32),
                    ) as i32)
                        != 99_i32))
                {
                    self.w32(
                        (v4.wrapping_add(156_i32.wrapping_mul((self.r32(v7) as i32)))
                            .wrapping_add(2960_i32) as u32),
                        53_u32,
                    );
                    self.w32(v8.wrapping_add(144), 1_u32);
                }
                v6 = v6.wrapping_add(1);
                v7 = v7.wrapping_add(4);
                if !(v6 < (self.r32(v3.wrapping_add(2072)) as i32)) {
                    break 'l1;
                }
            }
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v3.wrapping_add(1452)) as i32),
            (self.r32(v3.wrapping_add(1456)) as i32),
            (self.r32(v3.wrapping_add(1460)) as i32),
        );
        self.w32(v3.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_100136B0` (439 bytes).
    pub(crate) fn f_100136b0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut i: i32 = 0;
        let mut v10: u32 = 0;
        let mut j: i32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 13_u32);
        self.w32(v2.wrapping_add(1460), 4_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1472), 13_u32);
        self.w32(v2.wrapping_add(1476), 4_u32);
        self.w32(v2.wrapping_add(1480), 0_u32);
        if (((self.r32(v3) as i32) == 1_i32) && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1480), 1_u32);
        }
        self.w32(v2.wrapping_add(1484), 0_u32);
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1484), 1_u32);
        }
        self.w32(v2.wrapping_add(1488), 0_u32);
        if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1488), 1_u32);
        }
        self.w32(v2.wrapping_add(1492), 0_u32);
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1492), 1_u32);
        }
        self.w32(v2.wrapping_add(1496), 0_u32);
        if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1496), 1_u32);
        }
        v5 = 0_i32;
        v6 = v3;
        'l1: loop {
            v7 = (self.r32(v6) as i32);
            if (v5 == v4) {
                if (v7 == 1_i32) {
                    v8 = v6.wrapping_add(560);
                    if ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32) {
                        i = 0_i32;
                        'l2: loop {
                            if !(i < 1_i32) {
                                break 'l2;
                            }
                            if (!((self.r32(v8) as i32) != 0)) {
                                break 'l2;
                            }
                            v8 = v8.wrapping_add(4);
                            i = i.wrapping_add(1);
                        }
                        self.w32(
                            v6.wrapping_add((i.wrapping_add(140_i32) as u32).wrapping_mul(4)),
                            32_u32,
                        );
                        self.w32(
                            v6.wrapping_add((i.wrapping_add(141_i32) as u32).wrapping_mul(4)),
                            4_u32,
                        );
                    }
                }
            } else {
                if (v7 == 1_i32) {
                    v10 = v6.wrapping_add(560);
                    if ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32) {
                        j = 0_i32;
                        'l3: loop {
                            if !(j < 1_i32) {
                                break 'l3;
                            }
                            if (!((self.r32(v10) as i32) != 0)) {
                                break 'l3;
                            }
                            v10 = v10.wrapping_add(4);
                            j = j.wrapping_add(1);
                        }
                        self.w32(
                            v6.wrapping_add((j.wrapping_add(140_i32) as u32).wrapping_mul(4)),
                            32_u32,
                        );
                        self.w32(
                            v6.wrapping_add((j.wrapping_add(141_i32) as u32).wrapping_mul(4)),
                            3_u32,
                        );
                    }
                }
            }
            v5 = v5.wrapping_add(1);
            v6 = v6.wrapping_add(716);
            if !(v5 < 5_i32) {
                break 'l1;
            }
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v2.wrapping_add(1452)) as i32),
            (self.r32(v2.wrapping_add(1456)) as i32),
            (self.r32(v2.wrapping_add(1460)) as i32),
        );
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10013870` (272 bytes).
    pub(crate) fn f_10013870(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10013870_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10013870_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1456), 14_u32);
        self.w32(v3.wrapping_add(1460), 0_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1472), 14_u32);
        self.w32(v3.wrapping_add(1476), 0_u32);
        let _ = self.f_10007070(this, v5, fp.wrapping_add(0));
        if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v6 = (self.r32((v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32)) as i32);
            v7 = v4.wrapping_add(716_i32.wrapping_mul(v5));
            if ((v6 == 1_i32) && ((self.r32((v7.wrapping_add(372_i32) as u32)) as i32) != 99_i32)) {
                v8 = 0_i32;
                v9 = (v7.wrapping_add(408_i32) as u32);
                'l1: loop {
                    if (!((self.r32(v9) as i32) != 0)) {
                        break 'l1;
                    }
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    if !(v8 < 15_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    (v7.wrapping_add(4_i32.wrapping_mul(v8))
                        .wrapping_add(408_i32) as u32),
                    22_u32,
                );
                self.w32(
                    (v7.wrapping_add(4_i32.wrapping_mul(v8))
                        .wrapping_add(468_i32) as u32),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
            let _ = self.f_100074e0(
                this,
                (self.r32(v3.wrapping_add(1452)) as i32),
                (self.r32(v3.wrapping_add(1456)) as i32),
                (self.r32(v3.wrapping_add(1460)) as i32),
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            if (!((self.r8(fp.wrapping_add(0)) as i8) != 0)) {
                self.w32(v3.wrapping_add(1480), 0_u32);
            }
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10013980` (184 bytes).
    pub(crate) fn f_10013980(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10013980_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10013980_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        v5 = (self.r32(v4.wrapping_add(1388)) as i32);
        self.w32(v4.wrapping_add(1456), 14_u32);
        self.w32(v4.wrapping_add(1472), 14_u32);
        self.w32(v4.wrapping_add(1448), 1_u32);
        self.w32(v4.wrapping_add(1452), (v5 as u32));
        self.w32(v4.wrapping_add(1460), 1_u32);
        self.w32(v4.wrapping_add(1464), 1_u32);
        self.w32(v4.wrapping_add(1468), (v5 as u32));
        self.w32(v4.wrapping_add(1476), 1_u32);
        let _ = self.f_100073c0(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        v6 = ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) as i32);
        if (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32) {
            v6 = v6.wrapping_add(1);
        }
        v7 = (v3.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
        if (((self.r32(v7) as i32) == 1_i32) && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v7.wrapping_add(708), 69_u32);
            self.w32(v7.wrapping_add(712), (v6.wrapping_add(5_i32) as u32));
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_10013A40` (149 bytes).
    pub(crate) fn f_10013a40(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 14_u32);
        self.w32(v3.wrapping_add(1460), 2_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 14_u32);
        self.w32(v3.wrapping_add(1476), 2_u32);
        v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v5.wrapping_add(708), 73_u32);
            self.w32(v5.wrapping_add(712), 2_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_10013AE0` (189 bytes).
    pub(crate) fn f_10013ae0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10013ae0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10013ae0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        v5 = (self.r32(v4.wrapping_add(1388)) as i32);
        self.w32(v4.wrapping_add(1460), 3_u32);
        self.w32(v4.wrapping_add(1476), 3_u32);
        self.w32(v4.wrapping_add(1456), 14_u32);
        self.w32(v4.wrapping_add(1472), 14_u32);
        self.w32(v4.wrapping_add(1448), 1_u32);
        self.w32(v4.wrapping_add(1452), (v5 as u32));
        self.w32(v4.wrapping_add(1464), 1_u32);
        self.w32(v4.wrapping_add(1468), (v5 as u32));
        let _ = self.f_100073c0(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        v6 = ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) as i32);
        if (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32) {
            v6 = v6.wrapping_add(1);
        }
        v7 = (v3.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
        if (((self.r32(v7) as i32) == 1_i32) && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v7.wrapping_add(708), 70_u32);
            self.w32(v7.wrapping_add(712), (v6.wrapping_add(10_i32) as u32));
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_10013BA0` (149 bytes).
    pub(crate) fn f_10013ba0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 14_u32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 14_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v5.wrapping_add(708), 74_u32);
            self.w32(v5.wrapping_add(712), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_10013C40` (70 bytes).
    pub(crate) fn f_10013c40(&mut self, mut this: u32) -> i32 {
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
            let _ = self.f_10013c90(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10013C90` (652 bytes).
    pub(crate) fn f_10013c90(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10013c90_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10013c90_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = (self.r32(self.r32(this)) as i32);
                    v3 = self.r32((v2.wrapping_add(28_i32) as u32));
                    v4 = (self.r32(v3.wrapping_add(1388)) as i32);
                    v5 = ((self.r32((v2.wrapping_add(12_i32) as u32)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v4)) as u32);
                    bb = if ((self.r32(v5) as i32) != 1_i32) {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    bb = 3;
                }
                2 => {
                    bb = if ((self.r32(v5.wrapping_add(372)) as i32) == 99_i32) {
                        5
                    } else {
                        6
                    };
                }
                3 => {
                    self.w32(
                        fp.wrapping_add(1),
                        ((self.r32(v5.wrapping_add(708)) as i32) as u32),
                    );
                    let t1 = (self.r32(fp.wrapping_add(1)) as i32);
                    bb = match t1 {
                        55 => 18,
                        56 => 19,
                        57 => 20,
                        58 => 21,
                        59 => 22,
                        60 => 23,
                        61 => 24,
                        62 => 25,
                        63 => 26,
                        64 => 27,
                        65 => 28,
                        66 => 29,
                        67 => 30,
                        68 => 31,
                        69 => 32,
                        70 => 33,
                        71 => 34,
                        72 => 35,
                        73 => 36,
                        74 => 37,
                        _ => 38,
                    };
                }
                4 => {
                    bb = 2;
                }
                5 => {
                    bb = 7;
                }
                6 => {
                    v6 = 0_i32;
                    v7 = v5.wrapping_add(592);
                    bb = 68;
                }
                7 => {
                    bb = if (((self.r32(v5) as i32) == 1_i32)
                        && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
                    {
                        8
                    } else {
                        9
                    };
                }
                8 => {
                    v8 = 0_i32;
                    v9 = v5.wrapping_add(668);
                    bb = 10;
                }
                9 => {
                    bb = 3;
                }
                10 => {
                    bb = if ((self.r32(v9) as i32) != 48_i32) {
                        11
                    } else {
                        12
                    };
                }
                11 => {
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    bb = if (v8 >= 2_i32) { 13 } else { 14 };
                }
                12 => {
                    let _ = self.f_10018250(this, fp.wrapping_add(0));
                    return 0_i32;
                }
                13 => {
                    bb = 3;
                }
                14 => {
                    bb = 10;
                }
                15 => {
                    bb = 14;
                }
                16 => {
                    bb = 9;
                }
                17 => {
                    bb = if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                        60
                    } else {
                        61
                    };
                }
                18 => {
                    let _ = self.f_10019340(this, fp.wrapping_add(0));
                    bb = 17;
                }
                19 => {
                    let _ = self.f_10019d80(this, fp.wrapping_add(0));
                    bb = 17;
                }
                20 => {
                    let _ = self.f_1001a7c0(this, fp.wrapping_add(0));
                    bb = 17;
                }
                21 => {
                    let _ = self.f_1001b200(this, fp.wrapping_add(0));
                    bb = 17;
                }
                22 => {
                    let _ = self.f_1001beb0(this, fp.wrapping_add(0));
                    bb = 17;
                }
                23 => {
                    let _ = self.f_1001c200(this, fp.wrapping_add(0));
                    bb = 17;
                }
                24 => {
                    let _ = self.f_1001cc40(this, fp.wrapping_add(0));
                    bb = 17;
                }
                25 => {
                    let _ = self.f_1001d680(this, fp.wrapping_add(0));
                    bb = 17;
                }
                26 => {
                    let _ = self.f_1001db20(this, fp.wrapping_add(0));
                    bb = 17;
                }
                27 => {
                    let _ = self.f_1001e560(this, fp.wrapping_add(0));
                    bb = 17;
                }
                28 => {
                    let _ = self.f_1001efb0(this, fp.wrapping_add(0));
                    bb = 17;
                }
                29 => {
                    let _ = self.f_1001fa00(this, fp.wrapping_add(0));
                    bb = 17;
                }
                30 => {
                    let _ = self.f_10020450(this, fp.wrapping_add(0));
                    bb = 17;
                }
                31 => {
                    let _ = self.f_100208f0(this, fp.wrapping_add(0));
                    bb = 17;
                }
                32 => {
                    let _ = self.f_10020920(this, fp.wrapping_add(0));
                    bb = 17;
                }
                33 => {
                    let _ = self.f_10020950(this, fp.wrapping_add(0));
                    bb = 17;
                }
                34 => {
                    let _ = self.f_10020980(this, fp.wrapping_add(0));
                    bb = 17;
                }
                35 => {
                    let _ = self.f_10020da0(this, fp.wrapping_add(0));
                    bb = 17;
                }
                36 => {
                    let _ = self.f_10021240(this, fp.wrapping_add(0));
                    bb = 17;
                }
                37 => {
                    let _ = self.f_10021270(this, fp.wrapping_add(0));
                    bb = 17;
                }
                38 => {
                    bb = 17;
                }
                39 => {
                    bb = 19;
                }
                40 => {
                    bb = 20;
                }
                41 => {
                    bb = 21;
                }
                42 => {
                    bb = 22;
                }
                43 => {
                    bb = 23;
                }
                44 => {
                    bb = 24;
                }
                45 => {
                    bb = 25;
                }
                46 => {
                    bb = 26;
                }
                47 => {
                    bb = 27;
                }
                48 => {
                    bb = 28;
                }
                49 => {
                    bb = 29;
                }
                50 => {
                    bb = 30;
                }
                51 => {
                    bb = 31;
                }
                52 => {
                    bb = 32;
                }
                53 => {
                    bb = 33;
                }
                54 => {
                    bb = 34;
                }
                55 => {
                    bb = 35;
                }
                56 => {
                    bb = 36;
                }
                57 => {
                    bb = 37;
                }
                58 => {
                    bb = 38;
                }
                59 => {
                    bb = 17;
                }
                60 => {
                    bb = if ({
                        let t2 = (self.r32(v5.wrapping_add(712)) as i32).wrapping_sub(1);
                        self.w32(v5.wrapping_add(712), (t2 as u32));
                        t2
                    } <= 0_i32)
                    {
                        62
                    } else {
                        63
                    };
                }
                61 => {
                    return 0_i32;
                }
                62 => {
                    v11 = (self.r32(fp.wrapping_add(1)) as i32);
                    self.w32(v5.wrapping_add(708), 0_u32);
                    bb = if (v11 != 61_i32) { 64 } else { 65 };
                }
                63 => {
                    let _ = self.f_10007530(this, v4, (self.r32(fp.wrapping_add(1)) as i32));
                    bb = 61;
                }
                64 => {
                    self.w32(v3.wrapping_add(2192), 0_u32);
                    v12 = (self.r32(v3.wrapping_add(2280)) as i32);
                    self.w32(v3.wrapping_add(2188), 1_u32);
                    self.w32(v3.wrapping_add(2196), (v4 as u32));
                    self.w32(
                        v3.wrapping_add((v12.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        (v11 as u32),
                    );
                    self.w32(
                        v3.wrapping_add(2280),
                        ((self.r32(v3.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 65;
                }
                65 => {
                    self.w32(v3.wrapping_add(1048), 1_u32);
                    let _ = self.f_10007530(this, v4, v11);
                    return 0_i32;
                }
                66 => {
                    bb = 63;
                }
                67 => {
                    bb = 6;
                }
                68 => {
                    bb = if ((self.r32(v7) as i32) != 35_i32) {
                        69
                    } else {
                        70
                    };
                }
                69 => {
                    v6 = v6.wrapping_add(1);
                    v7 = v7.wrapping_add(4);
                    bb = if (v6 >= 3_i32) { 71 } else { 72 };
                }
                70 => {
                    let _ = self.f_10016cb0(this, fp.wrapping_add(0));
                    return 0_i32;
                }
                71 => {
                    bb = 7;
                }
                72 => {
                    bb = 68;
                }
                73 => {
                    bb = 72;
                }
                74 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10013F70` (498 bytes).
    pub(crate) fn f_10013f70(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10013f70_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10013f70_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        v1 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) != 0) || {
            let _ = {
                let t1: i32 = 1_i32;
                self.w32((v1.wrapping_add(1404_i32) as u32), (t1 as u32));
                t1
            };
            let _ = self.f_10016cf0(this, fp.wrapping_add(0));
            (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
        }) {
            if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) != 1_i32) || {
                let _ = {
                    let t2: i32 = 2_i32;
                    self.w32((v1.wrapping_add(1404_i32) as u32), (t2 as u32));
                    t2
                };
                let _ = self.f_10016e70(this, fp.wrapping_add(0));
                (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
            }) {
                if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) != 2_i32) || {
                    let _ = {
                        let t3: i32 = 3_i32;
                        self.w32((v1.wrapping_add(1404_i32) as u32), (t3 as u32));
                        t3
                    };
                    let _ = self.f_10016ff0(this, fp.wrapping_add(0));
                    (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                }) {
                    if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) != 3_i32) || {
                        let _ = {
                            let t4: i32 = 4_i32;
                            self.w32((v1.wrapping_add(1404_i32) as u32), (t4 as u32));
                            t4
                        };
                        let _ = self.f_100172b0(this, fp.wrapping_add(0));
                        (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                    }) {
                        if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) != 4_i32) || {
                            let _ = {
                                let t5: i32 = 5_i32;
                                self.w32((v1.wrapping_add(1404_i32) as u32), (t5 as u32));
                                t5
                            };
                            let _ = self.f_10017790(this, fp.wrapping_add(0));
                            (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                        }) {
                            if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32) != 5_i32)
                                || {
                                    let _ = {
                                        let t6: i32 = 6_i32;
                                        self.w32((v1.wrapping_add(1404_i32) as u32), (t6 as u32));
                                        t6
                                    };
                                    let _ = self.f_10017ba0(this, fp.wrapping_add(0));
                                    (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                                })
                            {
                                if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32)
                                    != 6_i32)
                                    || {
                                        let _ = {
                                            let t7: i32 = 7_i32;
                                            self.w32(
                                                (v1.wrapping_add(1404_i32) as u32),
                                                (t7 as u32),
                                            );
                                            t7
                                        };
                                        let _ = self.f_10017ba0(this, fp.wrapping_add(0));
                                        (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                                    })
                                {
                                    if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32)
                                        != 7_i32)
                                        || {
                                            let _ = {
                                                let t8: i32 = 8_i32;
                                                self.w32(
                                                    (v1.wrapping_add(1404_i32) as u32),
                                                    (t8 as u32),
                                                );
                                                t8
                                            };
                                            let _ = self.f_10017ba0(this, fp.wrapping_add(0));
                                            (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                                        })
                                    {
                                        if (((self.r32((v1.wrapping_add(1404_i32) as u32)) as i32)
                                            != 8_i32)
                                            || {
                                                let _ = {
                                                    let t9: i32 = 9_i32;
                                                    self.w32(
                                                        (v1.wrapping_add(1404_i32) as u32),
                                                        (t9 as u32),
                                                    );
                                                    t9
                                                };
                                                let _ = self.f_10017fb0(this, fp.wrapping_add(0));
                                                (((self.r8(fp.wrapping_add(0)) as i8) as i32)
                                                    != 1_i32)
                                            })
                                        {
                                            if (((self.r32((v1.wrapping_add(1404_i32) as u32))
                                                as i32)
                                                != 9_i32)
                                                || {
                                                    let _ = {
                                                        let t10: i32 = 10_i32;
                                                        self.w32(
                                                            (v1.wrapping_add(1404_i32) as u32),
                                                            (t10 as u32),
                                                        );
                                                        t10
                                                    };
                                                    let _ =
                                                        self.f_10017fb0(this, fp.wrapping_add(0));
                                                    (((self.r8(fp.wrapping_add(0)) as i8) as i32)
                                                        != 1_i32)
                                                })
                                            {
                                                if (((self.r32((v1.wrapping_add(1404_i32) as u32))
                                                    as i32)
                                                    != 10_i32)
                                                    || {
                                                        let _ = {
                                                            let t11: i32 = 11_i32;
                                                            self.w32(
                                                                (v1.wrapping_add(1404_i32) as u32),
                                                                (t11 as u32),
                                                            );
                                                            t11
                                                        };
                                                        let _ = self
                                                            .f_10014170(this, fp.wrapping_add(0));
                                                        (((self.r8(fp.wrapping_add(0)) as i8)
                                                            as i32)
                                                            != 1_i32)
                                                    })
                                                {
                                                    if ((self
                                                        .r32((v1.wrapping_add(1404_i32) as u32))
                                                        as i32)
                                                        == 11_i32)
                                                    {
                                                        self.w32(
                                                            (v1.wrapping_add(1404_i32) as u32),
                                                            99_u32,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return 0_i32;
    }

    /// `sub_10014170` (1060 bytes).
    pub(crate) fn f_10014170(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: u32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        let mut v29: i32 = 0;
        let mut v30: u32 = 0;
        let mut v31: i32 = 0;
        let mut v32: i32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: u32 = 0;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: i32 = 0;
        let mut v43: i8 = 0;
        let mut v44: u32 = 0;
        let mut v45: i32 = 0;
        let mut v46: i32 = 0;
        let mut v47: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = 15_i32;
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        v5 = (self.r32(v4.wrapping_add(1388)) as i32);
        v43 = 0_i8;
        v6 = ((self.r32((v2.wrapping_add(12_i32) as u32)) as i32)
            .wrapping_add(716_i32.wrapping_mul(v5)) as u32);
        v44 = v6;
        v7 = v6.wrapping_add(408);
        'l1: loop {
            if ((self.r32(v7) as i32) != 0) {
                v8 = (self.r32(v7.wrapping_add(60)) as i32);
                if (v8 != (1_i32).wrapping_neg()) {
                    v9 = v8.wrapping_sub(1_i32);
                    self.w32(v7.wrapping_add(60), (v9 as u32));
                    if (v9 <= 0_i32) {
                        v10 = (self.r32(v4.wrapping_add(2280)) as i32);
                        self.w32(v4.wrapping_add(2188), 1_u32);
                        self.w32(v4.wrapping_add(2192), 0_u32);
                        self.w32(v4.wrapping_add(2196), (v5 as u32));
                        v43 = 1_i8;
                        self.w32(
                            v4.wrapping_add((v10.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            ((self.r32(v7) as i32) as u32),
                        );
                        v6 = v44;
                        self.w32(
                            v4.wrapping_add(2280),
                            ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                        self.w32(v7, 0_u32);
                    }
                }
            }
            v7 = v7.wrapping_add(4);
            v3 = v3.wrapping_sub(1);
            if !(v3 != 0) {
                break 'l1;
            }
        }
        v11 = v6.wrapping_add(528);
        v45 = 2_i32;
        'l2: loop {
            if ((self.r32(v11) as i32) != 0) {
                v12 = (self.r32(v11.wrapping_add(8)) as i32);
                if (v12 != (1_i32).wrapping_neg()) {
                    v13 = v12.wrapping_sub(1_i32);
                    self.w32(v11.wrapping_add(8), (v13 as u32));
                    if (v13 <= 0_i32) {
                        v14 = (self.r32(v4.wrapping_add(2280)) as i32);
                        self.w32(v4.wrapping_add(2188), 1_u32);
                        self.w32(v4.wrapping_add(2192), 0_u32);
                        self.w32(v4.wrapping_add(2196), (v5 as u32));
                        v43 = 1_i8;
                        self.w32(
                            v4.wrapping_add((v14.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            ((self.r32(v11) as i32) as u32),
                        );
                        self.w32(
                            v4.wrapping_add(2280),
                            ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                        self.w32(v11, 0_u32);
                    }
                }
            }
            v11 = v11.wrapping_add(4);
            v45 = v45.wrapping_sub(1);
            if !(v45 != 0) {
                break 'l2;
            }
        }
        if ((self.r32(v6.wrapping_add(544)) as i32) != 0) {
            v15 = (self.r32(v6.wrapping_add(548)) as i32);
            if (v15 != (1_i32).wrapping_neg()) {
                v16 = v15.wrapping_sub(1_i32);
                self.w32(v6.wrapping_add(548), (v16 as u32));
                if (v16 <= 0_i32) {
                    v17 = (self.r32(v4.wrapping_add(2280)) as i32);
                    self.w32(v4.wrapping_add(2188), 1_u32);
                    self.w32(v4.wrapping_add(2192), 0_u32);
                    self.w32(v4.wrapping_add(2196), (v5 as u32));
                    v43 = 1_i8;
                    self.w32(
                        v4.wrapping_add((v17.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        ((self.r32(v6.wrapping_add(544)) as i32) as u32),
                    );
                    self.w32(
                        v4.wrapping_add(2280),
                        ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(v6.wrapping_add(544), 0_u32);
                }
            }
        }
        if ((self.r32(v6.wrapping_add(560)) as i32) != 0) {
            v18 = (self.r32(v6.wrapping_add(564)) as i32);
            if (v18 != (1_i32).wrapping_neg()) {
                v19 = v18.wrapping_sub(1_i32);
                self.w32(v6.wrapping_add(564), (v19 as u32));
                if (v19 <= 0_i32) {
                    v20 = (self.r32(v4.wrapping_add(2280)) as i32);
                    self.w32(v4.wrapping_add(2188), 1_u32);
                    self.w32(v4.wrapping_add(2192), 0_u32);
                    self.w32(v4.wrapping_add(2196), (v5 as u32));
                    v43 = 1_i8;
                    self.w32(
                        v4.wrapping_add((v20.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        ((self.r32(v6.wrapping_add(560)) as i32) as u32),
                    );
                    self.w32(
                        v4.wrapping_add(2280),
                        ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(v6.wrapping_add(560), 0_u32);
                }
            }
        }
        v21 = v6.wrapping_add(568);
        v22 = 2_i32;
        'l3: loop {
            if ((self.r32(v21) as i32) != 0) {
                v23 = (self.r32(v21.wrapping_add(8)) as i32);
                if (v23 != (1_i32).wrapping_neg()) {
                    v24 = v23.wrapping_sub(1_i32);
                    self.w32(v21.wrapping_add(8), (v24 as u32));
                    if (v24 <= 0_i32) {
                        v25 = (self.r32(v4.wrapping_add(2280)) as i32);
                        self.w32(v4.wrapping_add(2188), 1_u32);
                        self.w32(v4.wrapping_add(2192), 0_u32);
                        self.w32(v4.wrapping_add(2196), (v5 as u32));
                        v43 = 1_i8;
                        self.w32(
                            v4.wrapping_add((v25.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            ((self.r32(v21) as i32) as u32),
                        );
                        v6 = v44;
                        self.w32(
                            v4.wrapping_add(2280),
                            ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                        self.w32(v21, 0_u32);
                    }
                }
            }
            v21 = v21.wrapping_add(4);
            v22 = v22.wrapping_sub(1);
            if !(v22 != 0) {
                break 'l3;
            }
        }
        v26 = v6.wrapping_add(592);
        v46 = 3_i32;
        'l4: loop {
            if ((self.r32(v26) as i32) != 0) {
                v27 = (self.r32(v26.wrapping_add(12)) as i32);
                if (v27 != (1_i32).wrapping_neg()) {
                    v28 = v27.wrapping_sub(1_i32);
                    self.w32(v26.wrapping_add(12), (v28 as u32));
                    if (v28 <= 0_i32) {
                        v29 = (self.r32(v4.wrapping_add(2280)) as i32);
                        self.w32(v4.wrapping_add(2188), 1_u32);
                        self.w32(v4.wrapping_add(2192), 0_u32);
                        self.w32(v4.wrapping_add(2196), (v5 as u32));
                        self.w32(
                            v4.wrapping_add((v29.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            ((self.r32(v26) as i32) as u32),
                        );
                        self.w32(
                            v4.wrapping_add(2280),
                            ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                        if ((self.r32(v26) as i32) == 35_i32) {
                            self.w32(v4.wrapping_add(1048), 1_u32);
                        }
                        self.w32(v26, 0_u32);
                        v43 = 1_i8;
                    }
                }
            }
            v26 = v26.wrapping_add(4);
            v46 = v46.wrapping_sub(1);
            if !(v46 != 0) {
                break 'l4;
            }
        }
        v30 = v6.wrapping_add(628);
        v31 = 2_i32;
        'l5: loop {
            if ((self.r32(v30) as i32) != 0) {
                v32 = (self.r32(v30.wrapping_add(8)) as i32);
                if (v32 != (1_i32).wrapping_neg()) {
                    v33 = v32.wrapping_sub(1_i32);
                    self.w32(v30.wrapping_add(8), (v33 as u32));
                    if (v33 <= 0_i32) {
                        v34 = (self.r32(v4.wrapping_add(2280)) as i32);
                        self.w32(v4.wrapping_add(2188), 1_u32);
                        self.w32(v4.wrapping_add(2192), 0_u32);
                        self.w32(v4.wrapping_add(2196), (v5 as u32));
                        v43 = 1_i8;
                        self.w32(
                            v4.wrapping_add((v34.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            ((self.r32(v30) as i32) as u32),
                        );
                        v6 = v44;
                        self.w32(
                            v4.wrapping_add(2280),
                            ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                        self.w32(v30, 0_u32);
                    }
                }
            }
            v30 = v30.wrapping_add(4);
            v31 = v31.wrapping_sub(1);
            if !(v31 != 0) {
                break 'l5;
            }
        }
        v35 = v6.wrapping_add(668);
        v47 = 2_i32;
        'l6: loop {
            if ((self.r32(v35) as i32) != 0) {
                v36 = (self.r32(v35.wrapping_add(8)) as i32);
                if (v36 != (1_i32).wrapping_neg()) {
                    v37 = v36.wrapping_sub(1_i32);
                    self.w32(v35.wrapping_add(8), (v37 as u32));
                    if (v37 <= 0_i32) {
                        v38 = (self.r32(v4.wrapping_add(2280)) as i32);
                        self.w32(v4.wrapping_add(2188), 1_u32);
                        self.w32(v4.wrapping_add(2192), 0_u32);
                        self.w32(v4.wrapping_add(2196), (v5 as u32));
                        self.w32(
                            v4.wrapping_add((v38.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                            ((self.r32(v35) as i32) as u32),
                        );
                        self.w32(
                            v4.wrapping_add(2280),
                            ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                        );
                        if ((self.r32(v35) as i32) == 48_i32) {
                            self.w32(v4.wrapping_add(1048), 1_u32);
                        }
                        self.w32(v35, 0_u32);
                        v43 = 1_i8;
                    }
                }
            }
            v35 = v35.wrapping_add(4);
            v47 = v47.wrapping_sub(1);
            if !(v47 != 0) {
                break 'l6;
            }
        }
        if ((self.r32(v6.wrapping_add(684)) as i32) != 0) {
            v39 = (self.r32(v6.wrapping_add(688)) as i32);
            if (v39 == (1_i32).wrapping_neg()) {
                self.w8(a2, (v43 as u8));
                return 0_i32;
            } else {
                v40 = v39.wrapping_sub(1_i32);
                self.w32(v6.wrapping_add(688), (v40 as u32));
                if (v40 > 0_i32) {
                    self.w8(a2, (v43 as u8));
                } else {
                    v41 = (self.r32(v4.wrapping_add(2280)) as i32);
                    self.w32(v4.wrapping_add(2188), 1_u32);
                    self.w32(v4.wrapping_add(2192), 0_u32);
                    self.w32(v4.wrapping_add(2196), (v5 as u32));
                    self.w32(
                        v4.wrapping_add((v41.wrapping_add(550_i32) as u32).wrapping_mul(4)),
                        ((self.r32(v6.wrapping_add(684)) as i32) as u32),
                    );
                    self.w32(
                        v4.wrapping_add(2280),
                        ((self.r32(v4.wrapping_add(2280)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(v6.wrapping_add(684), 0_u32);
                    self.w8(a2, 1_u8);
                }
                return 0_i32;
            }
        } else {
            self.w8(a2, (v43 as u8));
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_100145A0` (275 bytes).
    pub(crate) fn f_100145a0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100145a0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100145a0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        if (!((self.r32((v2.wrapping_add(1404_i32) as u32)) as i32) != 0)) {
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            let _ = self.f_100146c0(this);
        }
        if (((self.r32((v2.wrapping_add(1404_i32) as u32)) as i32) != 1_i32) || {
            let _ = {
                let t1: i32 = 2_i32;
                self.w32((v2.wrapping_add(1404_i32) as u32), (t1 as u32));
                t1
            };
            let _ = self.f_100148f0(this, 0_i32, fp.wrapping_add(0));
            (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
        }) {
            if (((self.r32((v2.wrapping_add(1404_i32) as u32)) as i32) != 2_i32) || {
                let _ = {
                    let t2: i32 = 3_i32;
                    self.w32((v2.wrapping_add(1404_i32) as u32), (t2 as u32));
                    t2
                };
                let _ = self.f_100148f0(this, 1_i32, fp.wrapping_add(0));
                (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
            }) {
                if (((self.r32((v2.wrapping_add(1404_i32) as u32)) as i32) != 3_i32) || {
                    let _ = {
                        let t3: i32 = 4_i32;
                        self.w32((v2.wrapping_add(1404_i32) as u32), (t3 as u32));
                        t3
                    };
                    let _ = self.f_100148f0(this, 2_i32, fp.wrapping_add(0));
                    (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                }) {
                    if (((self.r32((v2.wrapping_add(1404_i32) as u32)) as i32) != 4_i32) || {
                        let _ = {
                            let t4: i32 = 5_i32;
                            self.w32((v2.wrapping_add(1404_i32) as u32), (t4 as u32));
                            t4
                        };
                        let _ = self.f_100148f0(this, 3_i32, fp.wrapping_add(0));
                        (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                    }) {
                        if (((self.r32((v2.wrapping_add(1404_i32) as u32)) as i32) != 5_i32) || {
                            let _ = {
                                let t5: i32 = 6_i32;
                                self.w32((v2.wrapping_add(1404_i32) as u32), (t5 as u32));
                                t5
                            };
                            let _ = self.f_100148f0(this, 4_i32, fp.wrapping_add(0));
                            (((self.r8(fp.wrapping_add(0)) as i8) as i32) != 1_i32)
                        }) {
                            if ((self.r32((v2.wrapping_add(1404_i32) as u32)) as i32) == 6_i32) {
                                self.w32((v2.wrapping_add(1404_i32) as u32), 99_u32);
                            }
                        }
                    }
                }
            }
        }
        return 0_i32;
    }

    /// `sub_100146C0` (553 bytes).
    pub(crate) fn f_100146c0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: u32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i8 = 0;
        let mut v21: u32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        let mut v29: u32 = 0;
        let mut v30: u32 = 0;
        let mut v31: u32 = 0;
        let mut v32: i32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v31 = v3;
        v29 = v2;
        if ((self.r32(v2.wrapping_add(2356)) as i32) <= 0_i32) {
            return 0_i32;
        }
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(2360), (v4 as u32));
        v27 = 0_i32;
        v5 = v2.wrapping_add(2336);
        v32 = (v3
            .wrapping_add((39_i32.wrapping_mul(v4).wrapping_add(900_i32) as u32).wrapping_mul(4))
            as i32);
        'l1: while (1_i32 != 0) {
            v6 = (self.r32(v5) as i32);
            v7 = v3
                .wrapping_add((179_i32.wrapping_mul((self.r32(v5) as i32)) as u32).wrapping_mul(4));
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = v7.wrapping_add(668);
                v9 = 2_i32;
                'l2: loop {
                    if ((self.r32(v8) as i32) == 49_i32) {
                        self.w32(
                            v2.wrapping_add((v6.wrapping_add(591_i32) as u32).wrapping_mul(4)),
                            49_u32,
                        );
                    }
                    v8 = v8.wrapping_add(4);
                    v9 = v9.wrapping_sub(1);
                    if !(v9 != 0) {
                        break 'l2;
                    }
                }
                if ((self.r32(v2.wrapping_add((v6.wrapping_add(591_i32) as u32).wrapping_mul(4)))
                    as i32)
                    == 49_i32)
                {
                    v10 = (self.r32(v7.wrapping_add(380)) as i32);
                    v11 = (self.r32(v7.wrapping_add(384)) as i32);
                    v12 = (self.r32((v32.wrapping_add(124_i32) as u32)) as i32);
                    v13 = (self.r32((v32.wrapping_add(128_i32) as u32)) as i32);
                    if (((((v10 == v12) && (v11.wrapping_sub(1_i32) == v13))
                        || ((v10.wrapping_add(1_i32) == v12) && (v11 == v13)))
                        || ((v10 == v12) && (v11.wrapping_add(1_i32) == v13)))
                        || ((v10.wrapping_sub(1_i32) == v12) && (v11 == v13)))
                    {
                        v2 = v29;
                        self.w32(
                            v29.wrapping_add((v6.wrapping_add(596_i32) as u32).wrapping_mul(4)),
                            1_u32,
                        );
                        self.w32(
                            v29.wrapping_add((v6.wrapping_add(601_i32) as u32).wrapping_mul(4)),
                            1_u32,
                        );
                    } else {
                        v2 = v29;
                    }
                }
            }
            v5 = v5.wrapping_add(4);
            if ({
                let t1 = v27.wrapping_add(1);
                v27 = t1;
                t1
            } >= (self.r32(v2.wrapping_add(2356)) as i32))
            {
                break 'l1;
            }
            v3 = v31;
        }
        v14 = v31;
        v15 = 0_i32;
        v16 = v2.wrapping_add(2364);
        v28 = 0_i32;
        v30 = v2.wrapping_add(2364);
        'l3: loop {
            if ((self.r32(v14) as i32) == 1_i32) {
                v17 = v14.wrapping_add(668);
                if ((self.r32(v14.wrapping_add(372)) as i32) != 99_i32) {
                    v18 = 2_i32;
                    'l4: loop {
                        if ((self.r32(v17) as i32) == 48_i32) {
                            self.w32(v16, 48_u32);
                        }
                        v17 = v17.wrapping_add(4);
                        v18 = v18.wrapping_sub(1);
                        if !(v18 != 0) {
                            break 'l4;
                        }
                    }
                    if ((self.r32(v16) as i32) == 48_i32) {
                        v19 = (self.r32(v2.wrapping_add(2356)) as i32);
                        v20 = 0_i8;
                        if (v19 > 0_i32) {
                            v21 = v2.wrapping_add(2336);
                            'l5: loop {
                                if ((self.r32(v21) as i32) != v15) {
                                    v20 = 1_i8;
                                }
                                v21 = v21.wrapping_add(4);
                                v19 = v19.wrapping_sub(1);
                                if !(v19 != 0) {
                                    break 'l5;
                                }
                            }
                            if ((v20 as i32) == 1_i32) {
                                v22 = (self.r32(v14.wrapping_add(380)) as i32);
                                v23 = (self.r32(v14.wrapping_add(384)) as i32);
                                v24 = (self.r32((v32.wrapping_add(124_i32) as u32)) as i32);
                                v25 = (self.r32((v32.wrapping_add(128_i32) as u32)) as i32);
                                if (((((v22 == v24) && (v23.wrapping_sub(1_i32) == v25))
                                    || ((v22.wrapping_add(1_i32) == v24) && (v23 == v25)))
                                    || ((v22 == v24) && (v23.wrapping_add(1_i32) == v25)))
                                    || ((v22.wrapping_sub(1_i32) == v24) && (v23 == v25)))
                                {
                                    v16 = v30;
                                    self.w32(v30.wrapping_add(20), 1_u32);
                                } else {
                                    v16 = v30;
                                }
                                v15 = v28;
                                self.w32(v16.wrapping_add(40), 1_u32);
                            }
                        }
                    }
                }
            }
            v15 = v15.wrapping_add(1);
            v14 = v14.wrapping_add(716);
            v16 = v16.wrapping_add(4);
            v28 = v15;
            v30 = v16;
            if !(v15 < 5_i32) {
                break 'l3;
            }
        }
        return 0_i32;
    }

    /// `sub_100148F0` (115 bytes).
    pub(crate) fn f_100148f0(&mut self, mut this: u32, mut a2: i32, mut a3: u32) -> i32 {
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        v3 = (self.r32(self.r32(this)) as i32);
        v4 = (self.r32((v3.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32((v3.wrapping_add(28_i32) as u32)) as i32);
        self.w8(a3, 0_u8);
        if ((((self.r32(
            (v5.wrapping_add(4_i32.wrapping_mul(a2))
                .wrapping_add(2404_i32) as u32),
        ) as i32)
            == 1_i32)
            && ((self.r32(
                (v4.wrapping_add(
                    156_i32.wrapping_mul((self.r32((v5.wrapping_add(2360_i32) as u32)) as i32)),
                )
                .wrapping_add(3600_i32) as u32),
            ) as i32)
                == 1_i32))
            && ((self.r32(
                (v4.wrapping_add(
                    156_i32.wrapping_mul((self.r32((v5.wrapping_add(2360_i32) as u32)) as i32)),
                )
                .wrapping_add(3716_i32) as u32),
            ) as i32)
                != 99_i32))
        {
            v6 = (self.r32(
                (v5.wrapping_add(4_i32.wrapping_mul(a2))
                    .wrapping_add(2364_i32) as u32),
            ) as i32);
            if (v6 == 48_i32) {
                let _ = self.f_10018290(this, a2, a3);
                return 0_i32;
            }
            if (v6 == 49_i32) {
                let _ = self.f_10018320(this, a2, a3);
            }
        }
        return 0_i32;
    }

    /// `sub_10014970` (141 bytes).
    pub(crate) fn f_10014970(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        let _ = self.f_10006320(this);
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1400), 2_u32);
        v5 = 39_i32.wrapping_mul(v4);
        v6 = v2
            .wrapping_add(156_i32.wrapping_mul(v4))
            .wrapping_add(3600_i32);
        if ((self.r32(
            (v2.wrapping_add(4_i32.wrapping_mul(v5))
                .wrapping_add(3740_i32) as u32),
        ) as i32)
            != 0)
        {
            self.w32(v3.wrapping_add(1400), 9_u32);
        }
        if ((self.r32(v3.wrapping_add(1400)) as i32) == 2_i32) {
            if ((self.r32(v3.wrapping_add(164)) as i32) == 1_i32) {
                self.w32(v3.wrapping_add(1400), 4_u32);
            }
            if ((self.r32(v3.wrapping_add(1400)) as i32) == 2_i32) {
                's1: {
                    'b1_1: {
                        'b1_0: {
                            match (self.r32((v6.wrapping_add(4_i32) as u32)) as i32) {
                                0_i32 | 1_i32 | 2_i32 | 3_i32 | 4_i32 | 5_i32 | 6_i32 => {
                                    break 'b1_0;
                                }
                                _ => break 'b1_1,
                            }
                        }
                        self.w32(v3.wrapping_add(1400), 3_u32);
                        break 's1;
                    }
                    return 0_i32;
                }
            }
        }
        return 0_i32;
    }

    /// `sub_10014A20` (70 bytes).
    pub(crate) fn f_10014a20(&mut self, mut this: u32) -> i32 {
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
            let _ = self.f_10014a70(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10014A70` (561 bytes).
    pub(crate) fn f_10014a70(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(64);
        let r = self.f_10014a70_body(fp, this);
        self.leave(64);
        r
    }

    fn f_10014a70_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut i: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i32 = 0;
        let mut v19: u32 = 0;
        let mut v20: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v1 = (self.r32(this) as i32);
                    self.w32(fp.wrapping_add(12), this);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(((self.r32((v1 as u32)) as i32).wrapping_add(12_i32) as u32)),
                    );
                    v2 = self.r32(fp.wrapping_add(4)).wrapping_add(704);
                    i = 0_i32;
                    bb = 1;
                }
                1 => {
                    bb = if (i < 5_i32) { 2 } else { 4 };
                }
                2 => {
                    v4 = (self.r32(v2.wrapping_sub(704)) as i32);
                    self.w32(
                        fp.wrapping_add(36).wrapping_add((i as u32).wrapping_mul(4)),
                        0_u32,
                    );
                    self.w32(
                        fp.wrapping_add(16).wrapping_add((i as u32).wrapping_mul(4)),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    bb = if ((v4 == 1_i32) && ((self.r32(v2.wrapping_sub(332)) as i32) != 99_i32)) {
                        5
                    } else {
                        6
                    };
                }
                3 => {
                    i = i.wrapping_add(1);
                    bb = 1;
                }
                4 => {
                    v11 = 0_i32;
                    v12 = (1_i32).wrapping_neg();
                    self.w32(fp.wrapping_add(0), 5_u32);
                    v13 = self.r32(fp.wrapping_add(4)).wrapping_add(692);
                    bb = 18;
                }
                5 => {
                    v5 = v2.wrapping_sub(120);
                    v6 = 2_i32;
                    bb = 7;
                }
                6 => {
                    v2 = v2.wrapping_add(716);
                    bb = 3;
                }
                7 => {
                    v7 = (self.r32(v5.wrapping_sub(16)) as i32);
                    bb = if (v7 != 0) { 10 } else { 11 };
                }
                8 => {
                    bb = if (v6 != 0) { 7 } else { 9 };
                }
                9 => {
                    v9 = (self.r32(v2.wrapping_sub(8)) as i32);
                    bb = if (v9 != 0) { 14 } else { 15 };
                }
                10 => {
                    v8 = (self.r32(v5) as i32);
                    bb = if ((self.r32(fp.wrapping_add(16).wrapping_add((i as u32).wrapping_mul(4)))
                        as i32)
                        < (self.r32(v5) as i32))
                    {
                        12
                    } else {
                        13
                    };
                }
                11 => {
                    v5 = v5.wrapping_add(4);
                    v6 = v6.wrapping_sub(1);
                    bb = 8;
                }
                12 => {
                    self.w32(
                        fp.wrapping_add(36).wrapping_add((i as u32).wrapping_mul(4)),
                        (v7 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16).wrapping_add((i as u32).wrapping_mul(4)),
                        (v8 as u32),
                    );
                    bb = 13;
                }
                13 => {
                    bb = 11;
                }
                14 => {
                    v10 = (self.r32(v2) as i32);
                    bb = if ((self.r32(fp.wrapping_add(16).wrapping_add((i as u32).wrapping_mul(4)))
                        as i32)
                        < (self.r32(v2) as i32))
                    {
                        16
                    } else {
                        17
                    };
                }
                15 => {
                    bb = 6;
                }
                16 => {
                    self.w32(
                        fp.wrapping_add(36).wrapping_add((i as u32).wrapping_mul(4)),
                        (v9 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16).wrapping_add((i as u32).wrapping_mul(4)),
                        (v10 as u32),
                    );
                    bb = 17;
                }
                17 => {
                    bb = 15;
                }
                18 => {
                    bb = if (((self.r32(v13.wrapping_sub(692)) as i32) == 1_i32)
                        && ((self.r32(v13.wrapping_sub(320)) as i32) != 99_i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                19 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        18
                    } else {
                        20
                    };
                }
                20 => {
                    bb = if (v11 != 0) { 30 } else { 32 };
                }
                21 => {
                    v14 = v13.wrapping_sub(76);
                    v15 = 3_i32;
                    bb = 23;
                }
                22 => {
                    v13 = v13.wrapping_add(716);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 19;
                }
                23 => {
                    bb = if (((self.r32(v14.wrapping_sub(24)) as i32) != 0)
                        && (v12 < (self.r32(v14) as i32)))
                    {
                        26
                    } else {
                        27
                    };
                }
                24 => {
                    bb = if (v15 != 0) { 23 } else { 25 };
                }
                25 => {
                    bb = if (((self.r32(v13.wrapping_sub(8)) as i32) != 0)
                        && (v12 < (self.r32(v13) as i32)))
                    {
                        28
                    } else {
                        29
                    };
                }
                26 => {
                    v11 = (self.r32(v14.wrapping_sub(24)) as i32);
                    v12 = (self.r32(v14) as i32);
                    bb = 27;
                }
                27 => {
                    v14 = v14.wrapping_add(4);
                    v15 = v15.wrapping_sub(1);
                    bb = 24;
                }
                28 => {
                    v11 = (self.r32(v13.wrapping_sub(8)) as i32);
                    v12 = (self.r32(v13) as i32);
                    bb = 29;
                }
                29 => {
                    bb = 22;
                }
                30 => {
                    bb = if (v11 == 38_i32) { 33 } else { 34 };
                }
                31 => {
                    let _ = self.f_10014d00(
                        this,
                        (fp.wrapping_add(4) as i32),
                        (self.r8(fp.wrapping_add(0)) as i8),
                    );
                    return 0_i32;
                }
                32 => {
                    bb = 46;
                }
                33 => {
                    let _ = self.f_10015e00(self.r32(fp.wrapping_add(12)));
                    return 0_i32;
                }
                34 => {
                    self.w8(fp.wrapping_add(0), 1_u8);
                    let t1 = v11;
                    bb = match t1 {
                        35 => 37,
                        36 => 38,
                        37 => 39,
                        39 => 40,
                        40 => 41,
                        50 => 42,
                        _ => 43,
                    };
                }
                35 => {
                    bb = 34;
                }
                36 => {
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w8(fp.wrapping_add(8), 0_u8);
                    let t2 = v11;
                    bb = match t2 {
                        35 => 67,
                        36 => 68,
                        37 => 69,
                        39 => 70,
                        40 => 71,
                        50 => 72,
                        _ => 73,
                    };
                }
                37 => {
                    bb = if (((self.r32(fp.wrapping_add(36)) as i32) != 0)
                        && (v12 < (self.r32(fp.wrapping_add(16)) as i32)))
                    {
                        44
                    } else {
                        45
                    };
                }
                38 => {
                    bb = 39;
                }
                39 => {
                    bb = if (((self.r32(fp.wrapping_add(36).wrapping_add(4)) as i32) != 0)
                        && (v12 < (self.r32(fp.wrapping_add(16).wrapping_add(4)) as i32)))
                    {
                        49
                    } else {
                        50
                    };
                }
                40 => {
                    bb = if (((self.r32(fp.wrapping_add(36).wrapping_add(8)) as i32) != 0)
                        && (v12 < (self.r32(fp.wrapping_add(16).wrapping_add(8)) as i32)))
                    {
                        53
                    } else {
                        54
                    };
                }
                41 => {
                    bb = if (((self.r32(fp.wrapping_add(36).wrapping_add(12)) as i32) != 0)
                        && (v12 < (self.r32(fp.wrapping_add(16).wrapping_add(12)) as i32)))
                    {
                        57
                    } else {
                        58
                    };
                }
                42 => {
                    bb = if (((self.r32(fp.wrapping_add(36).wrapping_add(16)) as i32) != 0)
                        && (v12 < (self.r32(fp.wrapping_add(16).wrapping_add(16)) as i32)))
                    {
                        61
                    } else {
                        62
                    };
                }
                43 => {
                    bb = 36;
                }
                44 => {
                    bb = 46;
                }
                45 => {
                    bb = 36;
                }
                46 => {
                    v17 = self.r32(fp.wrapping_add(4));
                    self.w8(fp.wrapping_add(0), 0_u8);
                    v18 = 0_i32;
                    v19 = fp.wrapping_add(36);
                    bb = 80;
                }
                47 => {
                    bb = 45;
                }
                48 => {
                    bb = 38;
                }
                49 => {
                    bb = 46;
                }
                50 => {
                    bb = 36;
                }
                51 => {
                    bb = 50;
                }
                52 => {
                    bb = 40;
                }
                53 => {
                    bb = 46;
                }
                54 => {
                    bb = 36;
                }
                55 => {
                    bb = 54;
                }
                56 => {
                    bb = 41;
                }
                57 => {
                    bb = 46;
                }
                58 => {
                    bb = 36;
                }
                59 => {
                    bb = 58;
                }
                60 => {
                    bb = 42;
                }
                61 => {
                    bb = 46;
                }
                62 => {
                    bb = 36;
                }
                63 => {
                    bb = 62;
                }
                64 => {
                    bb = 43;
                }
                65 => {
                    bb = 36;
                }
                66 => {
                    bb = 31;
                }
                67 => {
                    self.w32(
                        fp.wrapping_add(4),
                        ((((self.r32(fp.wrapping_add(4)) as u32) & 0xFFFFFF00)
                            | (((1_u8 as u8) as u32) << 0)) as u32),
                    );
                    bb = 66;
                }
                68 => {
                    bb = 69;
                }
                69 => {
                    self.w32(
                        fp.wrapping_add(4),
                        ((((self.r32(fp.wrapping_add(4)) as u32) & 0xFFFF00FF)
                            | (((1_u8 as u8) as u32) << 8)) as u32),
                    );
                    bb = 66;
                }
                70 => {
                    self.w32(
                        fp.wrapping_add(4),
                        ((((self.r32(fp.wrapping_add(4)) as u32) & 0xFF00FFFF)
                            | (((1_u8 as u8) as u32) << 16)) as u32),
                    );
                    bb = 66;
                }
                71 => {
                    self.w32(
                        fp.wrapping_add(4),
                        ((((self.r32(fp.wrapping_add(4)) as u32) & 0xFFFFFF)
                            | (((1_u8 as u8) as u32) << 24)) as u32),
                    );
                    bb = 66;
                }
                72 => {
                    self.w8(fp.wrapping_add(8), 1_u8);
                    bb = 66;
                }
                73 => {
                    bb = 66;
                }
                74 => {
                    bb = 68;
                }
                75 => {
                    bb = 70;
                }
                76 => {
                    bb = 71;
                }
                77 => {
                    bb = 72;
                }
                78 => {
                    bb = 73;
                }
                79 => {
                    bb = 66;
                }
                80 => {
                    self.w8(fp.wrapping_add(4).wrapping_add((v18 as u32)), 0_u8);
                    bb = if (((self.r32(v17) as i32) == 1_i32)
                        && ((self.r32(v17.wrapping_add(372)) as i32) != 99_i32))
                    {
                        83
                    } else {
                        84
                    };
                }
                81 => {
                    bb = if (v18 < 5_i32) { 80 } else { 82 };
                }
                82 => {
                    bb = 31;
                }
                83 => {
                    v20 = (self.r32(v19) as i32);
                    self.w8(fp.wrapping_add(4).wrapping_add((v18 as u32)), 1_u8);
                    bb = if (v20 != 0) { 85 } else { 86 };
                }
                84 => {
                    v17 = v17.wrapping_add(716);
                    v18 = v18.wrapping_add(1);
                    v19 = v19.wrapping_add(4);
                    bb = 81;
                }
                85 => {
                    self.w8(fp.wrapping_add(4).wrapping_add((v18 as u32)), 0_u8);
                    bb = 86;
                }
                86 => {
                    bb = 84;
                }
                87 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10014D00` (4314 bytes).
    pub(crate) fn f_10014d00(&mut self, mut this: u32, mut a2: i32, mut a3: i8) -> i32 {
        let fp = self.enter(1968);
        let r = self.f_10014d00_body(fp, this, a2, a3);
        self.leave(1968);
        r
    }

    fn f_10014d00_body(&mut self, fp: u32, mut this: u32, mut a2: i32, mut a3: i8) -> i32 {
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: u32 = 0;
        let mut v15: u32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: u32 = 0;
        let mut v23: i32 = 0;
        let mut v24: u32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i8 = 0;
        let mut v29: i32 = 0;
        let mut v30: i32 = 0;
        let mut v31: u32 = 0;
        let mut v32: i32 = 0;
        let mut v33: u32 = 0;
        let mut v34: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: u32 = 0;
        let mut v39: u32 = 0;
        let mut v40: u32 = 0;
        let mut v41: i32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: u32 = 0;
        let mut v46: u32 = 0;
        let mut v47: i32 = 0;
        let mut v48: i32 = 0;
        let mut v49: u32 = 0;
        let mut v50: i32 = 0;
        let mut v51: u32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: bool = false;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: i32 = 0;
        let mut v58: u32 = 0;
        let mut v59: u32 = 0;
        let mut v60: i32 = 0;
        let mut v61: bool = false;
        let mut v62: u32 = 0;
        let mut v63: u32 = 0;
        let mut v64: u32 = 0;
        let mut v65: i32 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: u32 = 0;
        let mut v69: i32 = 0;
        let mut v70: i32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v73: i32 = 0;
        let mut v74: u32 = 0;
        let mut i: u32 = 0;
        let mut v76: i32 = 0;
        let mut v77: i32 = 0;
        let mut v78: i32 = 0;
        let mut v79: i32 = 0;
        let mut v80: u32 = 0;
        let mut v81: u32 = 0;
        let mut v82: i32 = 0;
        let mut v83: i8 = 0;
        let mut v84: i32 = 0;
        let mut v85: i32 = 0;
        let mut v86: i32 = 0;
        let mut v87: i32 = 0;
        let mut v88: i32 = 0;
        let mut v89: i32 = 0;
        let mut v91: u32 = 0;
        let mut v92: i32 = 0;
        let mut v93: i32 = 0;
        let mut v94: u32 = 0;
        let mut v95: i32 = 0;
        let mut v96: i32 = 0;
        let mut v97: i32 = 0;
        let mut v98: u32 = 0;
        let mut v99: i32 = 0;
        let mut v100: i32 = 0;
        let mut v101: u32 = 0;
        let mut v102: i32 = 0;
        let mut v103: u32 = 0;
        let mut v104: i32 = 0;
        let mut v105: i32 = 0;
        let mut v106: u32 = 0;
        let mut v107: i32 = 0;
        let mut v108: u32 = 0;
        let mut v109: i32 = 0;
        let mut v110: i32 = 0;
        let mut v111: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    self.w32(fp.wrapping_add(104), this);
                    v4 = ((self.r32(self.r32(this)) as i32) as u32);
                    v5 = (self.r32(v4.wrapping_add(20)) as i32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(v4.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(v4.wrapping_add(28)) as i32) as u32),
                    );
                    v6 = (self.r32(self.r32(fp.wrapping_add(88)).wrapping_add(1388)) as i32);
                    self.w32(fp.wrapping_add(120), (v5.wrapping_add(4840_i32) as u32));
                    self.w32(fp.wrapping_add(100), (v5 as u32));
                    self.w32(fp.wrapping_add(116), (v6 as u32));
                    v7 = (self.r32(self.r32(fp.wrapping_add(0)).wrapping_add(
                        (39_i32.wrapping_mul(v6).wrapping_add(931_i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(
                        fp.wrapping_add(96),
                        ((self.r32(fp.wrapping_add(0)).wrapping_add(
                            (39_i32.wrapping_mul(v6).wrapping_add(900_i32) as u32).wrapping_mul(4),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(40), (v7 as u32));
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(self.r32(fp.wrapping_add(0)).wrapping_add(
                            (39_i32.wrapping_mul(v6).wrapping_add(932_i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(112),
                        ((self.r32(self.r32(fp.wrapping_add(0)).wrapping_add(
                            (39_i32.wrapping_mul(v6).wrapping_add(933_i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        (v7.wrapping_add(
                            20_i32.wrapping_mul((self.r32(fp.wrapping_add(36)) as i32)),
                        ) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(128),
                        (fp.wrapping_add(148) as i32),
                        (self.r32(self.r32(fp.wrapping_add(88)).wrapping_add(1384)) as i32),
                        v6,
                        0_i32,
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(fp.wrapping_add(140)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(108), self.r32(fp.wrapping_add(136)));
                    self.w32(
                        fp.wrapping_add(84),
                        ((self.r32(fp.wrapping_add(132)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(124),
                        ((self.r32(fp.wrapping_add(144)) as i32) as u32),
                    );
                    let _ = self.f_10006940(
                        this,
                        fp.wrapping_add(92),
                        (self.r32(fp.wrapping_add(132)) as i32),
                        (self.r32(fp.wrapping_add(136)) as i32),
                        (self.r32(fp.wrapping_add(140)) as i32),
                        (self.r32(fp.wrapping_add(144)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(92))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        self.r32(fp.wrapping_add(92)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(32), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32) as u32),
                    );
                    v8 = self.r32(fp.wrapping_add(0)).wrapping_add(380);
                    self.w32(fp.wrapping_add(64), fp.wrapping_add(260));
                    v9 = 0_i32;
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(0)).wrapping_add(380),
                    );
                    self.w32(fp.wrapping_add(72), fp.wrapping_add(160));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v8.wrapping_sub(380)) as i32) == 1_i32)
                        && ((self.r32(v8.wrapping_sub(8)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (v9 < 5_i32) { 1 } else { 3 };
                }
                3 => {
                    v18 = (self
                        .r32(((self.r32(fp.wrapping_add(96)) as i32).wrapping_add(4_i32) as u32))
                        as i32);
                    bb = if ((v18 == 2_i32) && (!(a3 != 0))) {
                        33
                    } else {
                        34
                    };
                }
                4 => {
                    v10 =
                        (self.r32(v8) as i32).wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    v11 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_add((self.r32(v8.wrapping_add(4)) as i32));
                    self.w8(fp.wrapping_add(59), 0_u8);
                    self.w8(fp.wrapping_add(31), 0_u8);
                    v12 = v10.wrapping_add((self.r32(fp.wrapping_add(20)) as i32));
                    v13 = v11.wrapping_add((self.r32(fp.wrapping_add(48)) as i32));
                    self.w32(
                        fp.wrapping_add(68),
                        (v11.wrapping_add((self.r32(fp.wrapping_add(48)) as i32)) as u32),
                    );
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(44)));
                    self.w32(fp.wrapping_add(52), (v11 as u32));
                    bb = if (v11 >= v11.wrapping_add((self.r32(fp.wrapping_add(48)) as i32))) {
                        6
                    } else {
                        8
                    };
                }
                5 => {
                    v8 = v8.wrapping_add(716);
                    v9 = v9.wrapping_add(1);
                    self.w32(fp.wrapping_add(24), v8);
                    self.w32(fp.wrapping_add(12), (v9 as u32));
                    bb = 2;
                }
                6 => {
                    v9 = (self.r32(fp.wrapping_add(12)) as i32);
                    v8 = self.r32(fp.wrapping_add(24));
                    bb = 7;
                }
                7 => {
                    bb = if ((self.r8((v9.wrapping_add(a2) as u32)) as i32) == 1_i32) {
                        27
                    } else {
                        28
                    };
                }
                8 => {
                    v14 = (20_i32.wrapping_mul(v11) as u32);
                    bb = 9;
                }
                9 => {
                    v15 = (v10 as u32);
                    bb = if (v10 < v12) { 12 } else { 13 };
                }
                10 => {
                    bb = if ((self.r32(fp.wrapping_add(52)) as i32) < v13) {
                        9
                    } else {
                        11
                    };
                }
                11 => {
                    v8 = self.r32(fp.wrapping_add(24));
                    v9 = (self.r32(fp.wrapping_add(12)) as i32);
                    bb = 7;
                }
                12 => {
                    bb = 14;
                }
                13 => {
                    v14 = v14.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(fp.wrapping_add(52)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 10;
                }
                14 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0) {
                        17
                    } else {
                        18
                    };
                }
                15 => {
                    bb = if ((v15 as i32)
                        < v10.wrapping_add((self.r32(fp.wrapping_add(20)) as i32)))
                    {
                        14
                    } else {
                        16
                    };
                }
                16 => {
                    v13 = (self.r32(fp.wrapping_add(68)) as i32);
                    bb = 13;
                }
                17 => {
                    bb = if ((v14 < 400_u32) && (v15 < 20_u32)) {
                        19
                    } else {
                        20
                    };
                }
                18 => {
                    v15 = v15.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    v12 = v10.wrapping_add((self.r32(fp.wrapping_add(20)) as i32));
                    bb = 15;
                }
                19 => {
                    v16 = (4_u32.wrapping_mul(v14.wrapping_add(v15)) as i32);
                    bb = if ((self.r32((v16.wrapping_add(v5) as u32)) as i32)
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
                    v17 = (self.r32((v16.wrapping_add(v5).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v17 == (1_i32).wrapping_neg()) || (v17 == v6.wrapping_add(5_i32))) {
                        23
                    } else {
                        24
                    };
                }
                22 => {
                    bb = 20;
                }
                23 => {
                    self.w8(fp.wrapping_add(59), 1_u8);
                    bb = if ((v15 == ((self.r32(fp.wrapping_add(40)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(52)) as i32)
                            == (self.r32(fp.wrapping_add(36)) as i32)))
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
                    self.w8(fp.wrapping_add(31), 1_u8);
                    bb = 26;
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(59)) as i8) as i32) == 1_i32) {
                        29
                    } else {
                        30
                    };
                }
                28 => {
                    bb = 5;
                }
                29 => {
                    self.w32(self.r32(fp.wrapping_add(72)), (v9 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(72),
                        self.r32(fp.wrapping_add(72)).wrapping_add(4),
                    );
                    bb = 30;
                }
                30 => {
                    bb = if (((self.r8(fp.wrapping_add(31)) as i8) as i32) == 1_i32) {
                        31
                    } else {
                        32
                    };
                }
                31 => {
                    self.w32(self.r32(fp.wrapping_add(64)), (v9 as u32));
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        self.r32(fp.wrapping_add(64)).wrapping_add(4),
                    );
                    bb = 32;
                }
                32 => {
                    bb = 28;
                }
                33 => {
                    v19 = 0_i32;
                    v20 = 9999_i32;
                    v21 = 0_i32;
                    v22 = self.r32(fp.wrapping_add(0)).wrapping_add(236);
                    bb = 35;
                }
                34 => {
                    bb = if ((v18 != 6_i32) || (a3 != 0)) {
                        53
                    } else {
                        55
                    };
                }
                35 => {
                    bb = if ((((self.r32(v22.wrapping_sub(236)) as i32) == 1_i32)
                        && ((self.r32(v22.wrapping_add(136)) as i32) != 99_i32))
                        && ((self.r32(v22) as i32) <= v20))
                    {
                        38
                    } else {
                        39
                    };
                }
                36 => {
                    bb = if (v21 < 5_i32) { 35 } else { 37 };
                }
                37 => {
                    self.w32(fp.wrapping_add(68), (v19 as u32));
                    bb = if ((self.r8((v19.wrapping_add(a2) as u32)) as i32) == 1_i32) {
                        40
                    } else {
                        41
                    };
                }
                38 => {
                    v20 = (self.r32(v22) as i32);
                    v19 = v21;
                    bb = 39;
                }
                39 => {
                    v22 = v22.wrapping_add(716);
                    v21 = v21.wrapping_add(1);
                    bb = 36;
                }
                40 => {
                    v23 = 0_i32;
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        42
                    } else {
                        43
                    };
                }
                41 => {
                    bb = 34;
                }
                42 => {
                    v24 = fp.wrapping_add(160);
                    bb = 44;
                }
                43 => {
                    bb = if (v23 == (self.r32(fp.wrapping_add(4)) as i32)) {
                        50
                    } else {
                        52
                    };
                }
                44 => {
                    bb = if ((self.r32(v24) as i32) == v19) {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    bb = if (v23 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    bb = 46;
                }
                48 => {
                    v23 = v23.wrapping_add(1);
                    v24 = v24.wrapping_add(4);
                    bb = 45;
                }
                49 => {
                    bb = 48;
                }
                50 => {
                    self.w32(fp.wrapping_add(160), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    bb = 51;
                }
                51 => {
                    self.w32(fp.wrapping_add(260), 0_u32);
                    self.w32(fp.wrapping_add(32), 0_u32);
                    bb = 41;
                }
                52 => {
                    self.w32(fp.wrapping_add(160), (v19 as u32));
                    self.w32(fp.wrapping_add(4), 1_u32);
                    bb = 51;
                }
                53 => {
                    v28 = (self.r8(fp.wrapping_add(31)) as i8);
                    bb = 54;
                }
                54 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) <= 0_i32) {
                        67
                    } else {
                        68
                    };
                }
                55 => {
                    v25 = 0_i32;
                    v26 = self.r32(fp.wrapping_add(0)).wrapping_add(3600);
                    v27 = 20_i32;
                    bb = 56;
                }
                56 => {
                    bb = if (((self.r32(v26) as i32) == 1_i32)
                        && ((self.r32(v26.wrapping_add(116)) as i32) != 99_i32))
                    {
                        59
                    } else {
                        60
                    };
                }
                57 => {
                    bb = if (v27 != 0) { 56 } else { 58 };
                }
                58 => {
                    v28 = 0_i8;
                    bb = if (v25 >= 15_i32) { 61 } else { 62 };
                }
                59 => {
                    v25 = v25.wrapping_add(1);
                    bb = 60;
                }
                60 => {
                    v26 = v26.wrapping_add(156);
                    v27 = v27.wrapping_sub(1);
                    bb = 57;
                }
                61 => {
                    v28 = 1_i8;
                    self.w32(self.r32(fp.wrapping_add(88)).wrapping_add(1436), 1_u32);
                    bb = if ((self.r32(fp.wrapping_add(32)) as i32) <= 0_i32) {
                        63
                    } else {
                        64
                    };
                }
                62 => {
                    bb = 54;
                }
                63 => {
                    bb = 65;
                }
                64 => {
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(32)) as i32) as u32),
                    );
                    let _ = self.memcpy(
                        fp.wrapping_add(160),
                        fp.wrapping_add(260),
                        (4_i32.wrapping_mul((self.r32(fp.wrapping_add(32)) as i32)) as u32),
                    );
                    bb = 62;
                }
                65 => {
                    v91 = self.r32(fp.wrapping_add(0));
                    v92 = 0_i32;
                    v93 = 0_i32;
                    v94 = fp.wrapping_add(160);
                    bb = 69;
                }
                66 => {
                    bb = 64;
                }
                67 => {
                    bb = 65;
                }
                68 => {
                    v29 = 0_i32;
                    v30 = 9999_i32;
                    self.w32(fp.wrapping_add(24), 0_u32);
                    v31 = fp.wrapping_add(160);
                    bb = 111;
                }
                69 => {
                    bb = if ((((self.r32(v91) as i32) == 1_i32)
                        && ((self.r32(v91.wrapping_add(372)) as i32) != 99_i32))
                        && ((self.r8((v93.wrapping_add(a2) as u32)) as i32) == 1_i32))
                    {
                        72
                    } else {
                        73
                    };
                }
                70 => {
                    bb = if (v93 < 5_i32) { 69 } else { 71 };
                }
                71 => {
                    self.w32(fp.wrapping_add(4), (v92 as u32));
                    v95 = (self
                        .r32(((self.r32(fp.wrapping_add(96)) as i32).wrapping_add(4_i32) as u32))
                        as i32);
                    bb = if ((v95 == 2_i32) && (!(a3 != 0))) {
                        74
                    } else {
                        75
                    };
                }
                72 => {
                    self.w32(v94, (v93 as u32));
                    v92 = v92.wrapping_add(1);
                    v94 = v94.wrapping_add(4);
                    bb = 73;
                }
                73 => {
                    v91 = v91.wrapping_add(716);
                    v93 = v93.wrapping_add(1);
                    bb = 70;
                }
                74 => {
                    v96 = 0_i32;
                    bb = if (v92 <= 0_i32) { 76 } else { 78 };
                }
                75 => {
                    bb = if ((((v95 != 6_i32) || (a3 != 0)) || ((v28 as i32) != 1_i32))
                        && (v92 != 0))
                    {
                        87
                    } else {
                        89
                    };
                }
                76 => {
                    v97 = (self.r32(fp.wrapping_add(68)) as i32);
                    bb = 77;
                }
                77 => {
                    bb = if (v96 != v92) { 85 } else { 86 };
                }
                78 => {
                    v97 = (self.r32(fp.wrapping_add(68)) as i32);
                    v98 = fp.wrapping_add(160);
                    bb = 79;
                }
                79 => {
                    bb = if ((self.r32(v98) as i32) == (self.r32(fp.wrapping_add(68)) as i32)) {
                        82
                    } else {
                        83
                    };
                }
                80 => {
                    bb = if (v96 < v92) { 79 } else { 81 };
                }
                81 => {
                    bb = 77;
                }
                82 => {
                    bb = 81;
                }
                83 => {
                    v96 = v96.wrapping_add(1);
                    v98 = v98.wrapping_add(4);
                    bb = 80;
                }
                84 => {
                    bb = 83;
                }
                85 => {
                    v92 = 1_i32;
                    self.w32(fp.wrapping_add(160), (v97 as u32));
                    self.w32(fp.wrapping_add(4), 1_u32);
                    bb = 86;
                }
                86 => {
                    bb = 75;
                }
                87 => {
                    v99 = 0_i32;
                    v100 = 9999_i32;
                    self.w32(fp.wrapping_add(24), 0_u32);
                    bb = if (v92 > 0_i32) { 90 } else { 91 };
                }
                88 => {
                    bb = 68;
                }
                89 => {
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(88)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(36)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    return 0_i32;
                }
                90 => {
                    v101 = fp.wrapping_add(160);
                    bb = 92;
                }
                91 => {
                    v103 = self.r32(fp.wrapping_add(0)).wrapping_add(
                        (179_i32.wrapping_mul(
                            (self.r32(fp.wrapping_add(160).wrapping_add(
                                ((self.r32(fp.wrapping_add(24)) as i32) as u32).wrapping_mul(4),
                            )) as i32),
                        ) as u32)
                            .wrapping_mul(4),
                    );
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(v103.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(v103.wrapping_add(380)) as i32) as u32),
                    );
                    v104 = 0_i32;
                    v105 = (self.r32(
                        ((self.r32(fp.wrapping_add(100)) as i32).wrapping_add(6440_i32) as u32),
                    ) as i32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if (v105 > 0_i32) { 97 } else { 98 };
                }
                92 => {
                    v102 = (self.r32(
                        self.r32(fp.wrapping_add(0)).wrapping_add(
                            (179_i32
                                .wrapping_mul((self.r32(v101) as i32))
                                .wrapping_add(95_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(
                                self.r32(fp.wrapping_add(0)).wrapping_add(
                                    (179_i32
                                        .wrapping_mul((self.r32(v101) as i32))
                                        .wrapping_add(96_i32)
                                        as u32)
                                        .wrapping_mul(4),
                                ),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(36)) as i32))
                                .wrapping_abs(),
                        );
                    bb = if (v102 < v100) { 95 } else { 96 };
                }
                93 => {
                    bb = if (v99 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        92
                    } else {
                        94
                    };
                }
                94 => {
                    bb = 91;
                }
                95 => {
                    v100 = v102;
                    self.w32(fp.wrapping_add(24), (v99 as u32));
                    bb = 96;
                }
                96 => {
                    v99 = v99.wrapping_add(1);
                    v101 = v101.wrapping_add(4);
                    bb = 93;
                }
                97 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(360),
                        self.r32(fp.wrapping_add(120)),
                        (4_i32.wrapping_mul(v105) as u32),
                    );
                    v104 = v105;
                    self.w32(fp.wrapping_add(8), (v105 as u32));
                    bb = 98;
                }
                98 => {
                    self.w32(fp.wrapping_add(16), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v104 > 0_i32) { 99 } else { 100 };
                }
                99 => {
                    self.w32(fp.wrapping_add(60), fp.wrapping_add(360));
                    bb = 101;
                }
                100 => {
                    v111 = (self.r32(fp.wrapping_add(360).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1552),
                        (v111 as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(88)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1556),
                        ((1_i32).wrapping_neg() as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1544),
                        (crem_i32(v111, 20_i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1548),
                        (cdiv_i32(v111, 20_i32) as u32),
                    );
                    return 0_i32;
                }
                101 => {
                    v106 = ((self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(60))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v107 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(60))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v108 = ((self.r32(fp.wrapping_add(32)) as i32)
                        .wrapping_add(v107)
                        .wrapping_abs() as u32);
                    v109 = (v106.wrapping_sub(v108) as i32).wrapping_abs();
                    v110 = (((self.r32(fp.wrapping_add(36)) as i32)
                        .wrapping_add(v107)
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_sub(crem_i32(
                                    (self.r32(self.r32(fp.wrapping_add(60))) as i32),
                                    20_i32,
                                ))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v108.wrapping_add(v106)))
                        as i32);
                    bb = if (v110 >= (self.r32(fp.wrapping_add(16)) as i32)) {
                        104
                    } else {
                        106
                    };
                }
                102 => {
                    bb = if v54 { 101 } else { 103 };
                }
                103 => {
                    bb = 100;
                }
                104 => {
                    bb = if ((v110 == (self.r32(fp.wrapping_add(16)) as i32))
                        && (v109 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        107
                    } else {
                        108
                    };
                }
                105 => {
                    v54 = ({
                        let t1 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t1 as u32));
                        t1
                    } < (self.r32(fp.wrapping_add(8)) as i32));
                    self.w32(
                        fp.wrapping_add(60),
                        self.r32(fp.wrapping_add(60)).wrapping_add(4),
                    );
                    bb = 102;
                }
                106 => {
                    self.w32(fp.wrapping_add(16), (v110 as u32));
                    self.w32(fp.wrapping_add(4), (v109 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 105;
                }
                107 => {
                    self.w32(fp.wrapping_add(4), (v109 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 108;
                }
                108 => {
                    bb = 105;
                }
                109 => {
                    bb = 88;
                }
                110 => {
                    bb = 88;
                }
                111 => {
                    v32 = (self.r32(
                        self.r32(fp.wrapping_add(0)).wrapping_add(
                            (179_i32
                                .wrapping_mul((self.r32(v31) as i32))
                                .wrapping_add(95_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(
                                self.r32(fp.wrapping_add(0)).wrapping_add(
                                    (179_i32
                                        .wrapping_mul((self.r32(v31) as i32))
                                        .wrapping_add(96_i32)
                                        as u32)
                                        .wrapping_mul(4),
                                ),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(36)) as i32))
                                .wrapping_abs(),
                        );
                    bb = if (v32 < v30) { 114 } else { 115 };
                }
                112 => {
                    bb = if (v29 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        111
                    } else {
                        113
                    };
                }
                113 => {
                    v33 = self.r32(fp.wrapping_add(44));
                    v34 = 0_i32;
                    self.w32(fp.wrapping_add(8), 0_u32);
                    self.w32(
                        fp.wrapping_add(96),
                        ((self.r32(fp.wrapping_add(160).wrapping_add(
                            ((self.r32(fp.wrapping_add(24)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(
                            self.r32(fp.wrapping_add(0)).wrapping_add(
                                (179_i32
                                    .wrapping_mul((self.r32(fp.wrapping_add(96)) as i32))
                                    .wrapping_add(95_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    v35 = (self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(
                            self.r32(fp.wrapping_add(0)).wrapping_add(
                                (179_i32
                                    .wrapping_mul((self.r32(fp.wrapping_add(96)) as i32))
                                    .wrapping_add(96_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    v36 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(32)) as i32));
                    v37 = (self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(20)) as i32));
                    bb = if (v36 < v36.wrapping_add((self.r32(fp.wrapping_add(48)) as i32))) {
                        116
                    } else {
                        117
                    };
                }
                114 => {
                    v30 = v32;
                    self.w32(fp.wrapping_add(24), (v29 as u32));
                    bb = 115;
                }
                115 => {
                    v29 = v29.wrapping_add(1);
                    v31 = v31.wrapping_add(4);
                    bb = 112;
                }
                116 => {
                    v38 = (20_i32.wrapping_mul(v36) as u32);
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(48)) as i32) as u32),
                    );
                    bb = 118;
                }
                117 => {
                    bb = if ((((((self.r32(fp.wrapping_add(108)) < 4_u32)
                        || (self.r32(fp.wrapping_add(108)) == 5_u32))
                        || (self.r32(fp.wrapping_add(108)) == 6_u32))
                        || (self.r32(fp.wrapping_add(108)) == 7_u32))
                        || (self.r32(fp.wrapping_add(108)) == 8_u32))
                        || (self.r32(fp.wrapping_add(108)) == 9_u32))
                    {
                        134
                    } else {
                        136
                    };
                }
                118 => {
                    bb = if (v35 < v37) { 121 } else { 122 };
                }
                119 => {
                    bb = if ((self.r32(fp.wrapping_add(68)) as i32) != 0) {
                        118
                    } else {
                        120
                    };
                }
                120 => {
                    bb = 117;
                }
                121 => {
                    v39 = fp
                        .wrapping_add(360)
                        .wrapping_add((v34 as u32).wrapping_mul(4));
                    bb = 123;
                }
                122 => {
                    v38 = v38.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(68)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 119;
                }
                123 => {
                    bb = if ((self.r32(v33) as i32) != 0) {
                        126
                    } else {
                        127
                    };
                }
                124 => {
                    bb = if (v35 < v37) { 123 } else { 125 };
                }
                125 => {
                    v35 = (self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32));
                    v34 = (self.r32(fp.wrapping_add(8)) as i32);
                    bb = 122;
                }
                126 => {
                    bb = if ((v38 < 400_u32) && ((v35 as u32) < 20_u32)) {
                        128
                    } else {
                        129
                    };
                }
                127 => {
                    v37 = (self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(20)) as i32));
                    v33 = v33.wrapping_add(4);
                    v35 = v35.wrapping_add(1);
                    bb = 124;
                }
                128 => {
                    v40 = v38.wrapping_add((v35 as u32));
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(100)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v38.wrapping_add((v35 as u32)))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        130
                    } else {
                        131
                    };
                }
                129 => {
                    bb = 127;
                }
                130 => {
                    v41 = (self.r32(
                        ((self.r32(fp.wrapping_add(100)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v40))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v41 == (1_i32).wrapping_neg())
                        || (v41 == (self.r32(fp.wrapping_add(116)) as i32).wrapping_add(5_i32)))
                    {
                        132
                    } else {
                        133
                    };
                }
                131 => {
                    bb = 129;
                }
                132 => {
                    self.w32(
                        {
                            let t2 = v39;
                            v39 = t2.wrapping_add(4);
                            t2
                        },
                        v40,
                    );
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 133;
                }
                133 => {
                    bb = 131;
                }
                134 => {
                    self.w32(fp.wrapping_add(16), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v34 > 0_i32) { 137 } else { 138 };
                }
                135 => {
                    self.w32(fp.wrapping_add(24), (v48 as u32));
                    bb = 149;
                }
                136 => {
                    bb = if (self.r32(fp.wrapping_add(108)) != 4_u32) {
                        147
                    } else {
                        148
                    };
                }
                137 => {
                    self.w32(fp.wrapping_add(52), fp.wrapping_add(360));
                    bb = 139;
                }
                138 => {
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(fp.wrapping_add(360).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        (crem_i32((self.r32(fp.wrapping_add(52)) as i32), 20_i32) as u32),
                    );
                    v48 = cdiv_i32((self.r32(fp.wrapping_add(52)) as i32), 20_i32);
                    bb = 135;
                }
                139 => {
                    v49 = ((self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(52))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v50 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(52))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v51 = (v50
                        .wrapping_add((self.r32(fp.wrapping_add(32)) as i32))
                        .wrapping_abs() as u32);
                    v52 = (v49.wrapping_sub(v51) as i32).wrapping_abs();
                    v53 = ((v50
                        .wrapping_add((self.r32(fp.wrapping_add(36)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_sub(crem_i32(
                                    (self.r32(self.r32(fp.wrapping_add(52))) as i32),
                                    20_i32,
                                ))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v51.wrapping_add(v49)))
                        as i32);
                    bb = if (v53 >= (self.r32(fp.wrapping_add(16)) as i32)) {
                        142
                    } else {
                        144
                    };
                }
                140 => {
                    bb = if v54 { 139 } else { 141 };
                }
                141 => {
                    bb = 138;
                }
                142 => {
                    bb = if ((v53 == (self.r32(fp.wrapping_add(16)) as i32))
                        && (v52 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        145
                    } else {
                        146
                    };
                }
                143 => {
                    v54 = ({
                        let t3 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(8)) as i32));
                    self.w32(
                        fp.wrapping_add(52),
                        self.r32(fp.wrapping_add(52)).wrapping_add(4),
                    );
                    bb = 140;
                }
                144 => {
                    self.w32(fp.wrapping_add(16), (v53 as u32));
                    self.w32(fp.wrapping_add(4), (v52 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 143;
                }
                145 => {
                    self.w32(fp.wrapping_add(4), (v52 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 146;
                }
                146 => {
                    bb = 143;
                }
                147 => {
                    bb = 149;
                }
                148 => {
                    self.w32(fp.wrapping_add(16), 9999_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (v34 > 0_i32) { 151 } else { 152 };
                }
                149 => {
                    bb = if ((self.r32(fp.wrapping_add(84)) as i32) <= 0_i32) {
                        163
                    } else {
                        164
                    };
                }
                150 => {
                    bb = 148;
                }
                151 => {
                    v42 = fp.wrapping_add(360);
                    bb = 153;
                }
                152 => {
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(fp.wrapping_add(360).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        (crem_i32((self.r32(fp.wrapping_add(52)) as i32), 20_i32) as u32),
                    );
                    v48 = cdiv_i32((self.r32(fp.wrapping_add(52)) as i32), 20_i32);
                    bb = 135;
                }
                153 => {
                    v43 = cdiv_i32((self.r32(v42) as i32), (20_i32).wrapping_neg());
                    v44 = v43
                        .wrapping_add((self.r32(fp.wrapping_add(36)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_sub(crem_i32((self.r32(v42) as i32), 20_i32))
                                .wrapping_abs(),
                        );
                    v45 = ((self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_sub(crem_i32((self.r32(v42) as i32), 20_i32))
                        .wrapping_abs() as u32);
                    v46 = (v43
                        .wrapping_add((self.r32(fp.wrapping_add(32)) as i32))
                        .wrapping_abs() as u32);
                    v47 = (v45.wrapping_sub(v46) as i32).wrapping_abs();
                    bb = if (v46.wrapping_add(v45) != 2_u32) {
                        156
                    } else {
                        157
                    };
                }
                154 => {
                    bb = if ((self.r32(fp.wrapping_add(12)) as i32)
                        < (self.r32(fp.wrapping_add(8)) as i32))
                    {
                        153
                    } else {
                        155
                    };
                }
                155 => {
                    bb = 152;
                }
                156 => {
                    v44 = ((v44 as u32).wrapping_add(6_u32.wrapping_mul(v46.wrapping_add(v45)))
                        as i32);
                    bb = 157;
                }
                157 => {
                    bb = if (v44 >= (self.r32(fp.wrapping_add(16)) as i32)) {
                        158
                    } else {
                        160
                    };
                }
                158 => {
                    bb = if ((v44 == (self.r32(fp.wrapping_add(16)) as i32))
                        && (v47 < (self.r32(fp.wrapping_add(4)) as i32)))
                    {
                        161
                    } else {
                        162
                    };
                }
                159 => {
                    v42 = v42.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 154;
                }
                160 => {
                    self.w32(fp.wrapping_add(16), (v44 as u32));
                    self.w32(fp.wrapping_add(4), (v47 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 159;
                }
                161 => {
                    self.w32(fp.wrapping_add(4), (v47 as u32));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    bb = 162;
                }
                162 => {
                    bb = 159;
                }
                163 => {
                    v66 = (self.r32(fp.wrapping_add(40)) as i32);
                    v67 = (self.r32(fp.wrapping_add(24)) as i32);
                    v69 = (self.r32(fp.wrapping_add(80)) as i32);
                    v71 = (self.r32(fp.wrapping_add(100)) as i32);
                    v70 = (self.r32(fp.wrapping_add(60)) as i32);
                    bb = 165;
                }
                164 => {
                    self.w32(fp.wrapping_add(36), 9999_u32);
                    self.w32(
                        fp.wrapping_add(92),
                        self.r32(
                            self.r32(fp.wrapping_add(104)).wrapping_add(
                                ((self.r32(fp.wrapping_add(84)) as i32).wrapping_add(3_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ),
                    );
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(92)).wrapping_add(16),
                    );
                    v55 = (self.r32(fp.wrapping_add(24)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(fp.wrapping_add(40)) as i32)
                            .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32))
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(72)) as i32).wrapping_add(
                            (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(8)) as i32),
                        ) as u32),
                    );
                    v56 = v55.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32),
                    );
                    self.w32(fp.wrapping_add(68), (v56 as u32));
                    self.w32(fp.wrapping_add(16), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    bb = if (self.r32(fp.wrapping_add(108)) != 0) {
                        167
                    } else {
                        168
                    };
                }
                165 => {
                    self.w32(fp.wrapping_add(60), (v66.wrapping_add(v69) as u32));
                    v72 = self.r32(fp.wrapping_add(44));
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(20)) as i32)
                            .wrapping_add(v66)
                            .wrapping_add(v69) as u32),
                    );
                    v73 = v70.wrapping_add(v67);
                    bb = if (v73 < v73.wrapping_add((self.r32(fp.wrapping_add(48)) as i32))) {
                        231
                    } else {
                        232
                    };
                }
                166 => {
                    bb = 164;
                }
                167 => {
                    bb = if (self.r32(fp.wrapping_add(108)) != 2_u32) {
                        169
                    } else {
                        170
                    };
                }
                168 => {
                    bb = if (v55 >= v56) { 202 } else { 203 };
                }
                169 => {
                    bb = 171;
                }
                170 => {
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_add(
                            (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32),
                        ) as u32),
                    );
                    bb = if (v55 >= v56) { 173 } else { 174 };
                }
                171 => {
                    v66 = crem_i32((self.r32(fp.wrapping_add(4)) as i32), 20_i32);
                    v67 = cdiv_i32((self.r32(fp.wrapping_add(4)) as i32), 20_i32);
                    bb = if (self.r32(fp.wrapping_add(108)) != 0) {
                        225
                    } else {
                        227
                    };
                }
                172 => {
                    bb = 170;
                }
                173 => {
                    bb = 171;
                }
                174 => {
                    self.w32(fp.wrapping_add(20), (20_i32.wrapping_mul(v55) as u32));
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(v55) as u32),
                    );
                    bb = 176;
                }
                175 => {
                    bb = 174;
                }
                176 => {
                    bb = if (1_i32 != 0) { 177 } else { 178 };
                }
                177 => {
                    v62 = ((self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32))
                        as u32);
                    bb = if ((self.r32(fp.wrapping_add(72)) as i32)
                        < (self.r32(fp.wrapping_add(0)) as i32))
                    {
                        179
                    } else {
                        180
                    };
                }
                178 => {
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(64)) as i32)
                            .wrapping_sub((self.r32(fp.wrapping_add(72)) as i32))
                            as u32),
                    );
                    bb = 186;
                }
                179 => {
                    bb = 178;
                }
                180 => {
                    bb = 182;
                }
                181 => {
                    bb = 180;
                }
                182 => {
                    v54 = ({
                        let t4 = (self.r32(fp.wrapping_add(12)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(12), (t4 as u32));
                        t4
                    } < (self.r32(fp.wrapping_add(68)) as i32));
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(fp.wrapping_add(44)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (!v54) { 183 } else { 184 };
                }
                183 => {
                    bb = 171;
                }
                184 => {
                    bb = 176;
                }
                185 => {
                    bb = 184;
                }
                186 => {
                    bb = if (1_i32 != 0) { 187 } else { 188 };
                }
                187 => {
                    bb = if ((((!((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0))
                        || (self.r32(fp.wrapping_add(20)) >= 400_u32))
                        || (v62 >= 20_u32))
                        || (((self.r32(fp.wrapping_add(12)) as i32)
                            != (self.r32(fp.wrapping_add(32)) as i32))
                            && (v62 != ((self.r32(fp.wrapping_add(64)) as i32) as u32))))
                    {
                        189
                    } else {
                        190
                    };
                }
                188 => {
                    bb = 168;
                }
                189 => {
                    bb = 191;
                }
                190 => {
                    v63 = ((self.r32(fp.wrapping_add(48)) as i32).wrapping_abs() as u32);
                    v64 = ((self.r32(fp.wrapping_add(44)) as i32).wrapping_abs() as u32);
                    v65 = (v63.wrapping_sub(v64) as i32).wrapping_abs();
                    bb = if ((v64.wrapping_add(v63) as i32)
                        >= (self.r32(fp.wrapping_add(36)) as i32))
                    {
                        193
                    } else {
                        195
                    };
                }
                191 => {
                    v62 = v62.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if ((v62 as i32) >= (self.r32(fp.wrapping_add(0)) as i32)) {
                        199
                    } else {
                        200
                    };
                }
                192 => {
                    bb = 190;
                }
                193 => {
                    bb = if ((v64.wrapping_add(v63)
                        != ((self.r32(fp.wrapping_add(36)) as i32) as u32))
                        || (v65 >= (self.r32(fp.wrapping_add(16)) as i32)))
                    {
                        196
                    } else {
                        197
                    };
                }
                194 => {
                    self.w32(fp.wrapping_add(16), (v65 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(20)).wrapping_add(v62) as i32) as u32),
                    );
                    bb = 191;
                }
                195 => {
                    self.w32(fp.wrapping_add(36), ((v64.wrapping_add(v63) as i32) as u32));
                    bb = 194;
                }
                196 => {
                    bb = 191;
                }
                197 => {
                    bb = 194;
                }
                198 => {
                    bb = 197;
                }
                199 => {
                    bb = 182;
                }
                200 => {
                    bb = 186;
                }
                201 => {
                    bb = 200;
                }
                202 => {
                    bb = 171;
                }
                203 => {
                    v57 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32));
                    self.w32(fp.wrapping_add(20), (20_i32.wrapping_mul(v55) as u32));
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(fp.wrapping_add(32)) as i32).wrapping_sub(v55) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(84),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32) as u32),
                    );
                    bb = 205;
                }
                204 => {
                    bb = 203;
                }
                205 => {
                    bb = if (v57 >= (self.r32(fp.wrapping_add(0)) as i32)) {
                        208
                    } else {
                        209
                    };
                }
                206 => {
                    bb = if (!v61) { 205 } else { 207 };
                }
                207 => {
                    bb = 171;
                }
                208 => {
                    bb = 210;
                }
                209 => {
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(64)) as i32).wrapping_sub(v57) as u32),
                    );
                    bb = 212;
                }
                210 => {
                    v61 = ((self.r32(fp.wrapping_add(84)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(20),
                        self.r32(fp.wrapping_add(20)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(fp.wrapping_add(44)) as i32).wrapping_sub(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(84),
                        ((self.r32(fp.wrapping_add(84)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 206;
                }
                211 => {
                    bb = 209;
                }
                212 => {
                    bb = if ((((self.r32(self.r32(fp.wrapping_add(8))) as i32) != 0)
                        && (self.r32(fp.wrapping_add(20)) < 400_u32))
                        && ((v57 as u32) < 20_u32))
                    {
                        215
                    } else {
                        216
                    };
                }
                213 => {
                    bb = if (v57 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        212
                    } else {
                        214
                    };
                }
                214 => {
                    v57 = (self.r32(fp.wrapping_add(40)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(92))) as i32));
                    bb = 210;
                }
                215 => {
                    v58 = ((self.r32(fp.wrapping_add(48)) as i32).wrapping_abs() as u32);
                    v59 = ((self.r32(fp.wrapping_add(44)) as i32).wrapping_abs() as u32);
                    v60 = (v58.wrapping_sub(v59) as i32).wrapping_abs();
                    bb = if ((v59.wrapping_add(v58) as i32)
                        < (self.r32(fp.wrapping_add(36)) as i32))
                    {
                        217
                    } else {
                        218
                    };
                }
                216 => {
                    bb = 220;
                }
                217 => {
                    self.w32(fp.wrapping_add(36), ((v59.wrapping_add(v58) as i32) as u32));
                    bb = 219;
                }
                218 => {
                    bb = if ((v59.wrapping_add(v58)
                        == ((self.r32(fp.wrapping_add(36)) as i32) as u32))
                        && (v60 < (self.r32(fp.wrapping_add(16)) as i32)))
                    {
                        222
                    } else {
                        223
                    };
                }
                219 => {
                    self.w32(fp.wrapping_add(16), (v60 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(20)).wrapping_add((v57 as u32)) as i32) as u32),
                    );
                    bb = 220;
                }
                220 => {
                    v57 = v57.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 213;
                }
                221 => {
                    bb = 218;
                }
                222 => {
                    bb = 219;
                }
                223 => {
                    bb = 216;
                }
                224 => {
                    bb = 223;
                }
                225 => {
                    bb = if (self.r32(fp.wrapping_add(108)) == 2_u32) {
                        228
                    } else {
                        230
                    };
                }
                226 => {
                    v69 = (self.r32(v68) as i32);
                    v70 = (self.r32(v68.wrapping_add(4)) as i32);
                    v71 = (self.r32(fp.wrapping_add(100)) as i32);
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(v68.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(v68.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(44), v68.wrapping_add(16));
                    bb = 165;
                }
                227 => {
                    v68 = self.r32(
                        self.r32(fp.wrapping_add(104)).wrapping_add(
                            ((self.r32(fp.wrapping_add(76)) as i32).wrapping_add(3_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(92), v68);
                    bb = 226;
                }
                228 => {
                    v68 = self.r32(
                        self.r32(fp.wrapping_add(104)).wrapping_add(
                            ((self.r32(fp.wrapping_add(76)) as i32).wrapping_add(18_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(92), v68);
                    bb = 229;
                }
                229 => {
                    bb = 226;
                }
                230 => {
                    v68 = self.r32(fp.wrapping_add(92));
                    bb = 229;
                }
                231 => {
                    v74 = (20_i32.wrapping_mul(v73) as u32);
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(fp.wrapping_add(48)) as i32) as u32),
                    );
                    bb = 233;
                }
                232 => {
                    v77 = (self.r32(fp.wrapping_add(60)) as i32);
                    v78 = ((self.r32(fp.wrapping_add(80)) as i32) & 15_i32);
                    v79 = (self
                        .r32(fp.wrapping_add(60))
                        .wrapping_add(((self.r32(fp.wrapping_add(20)) as i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(84), (16_i32.wrapping_mul(v78) as u32));
                    self.w32(fp.wrapping_add(8), self.r32(fp.wrapping_add(44)));
                    self.w32(fp.wrapping_add(80), (v78 as u32));
                    self.w32((v71.wrapping_add(7400_i32) as u32), 0_u32);
                    bb = if (v73 < v73.wrapping_add((self.r32(fp.wrapping_add(48)) as i32))) {
                        246
                    } else {
                        247
                    };
                }
                233 => {
                    i = self.r32(fp.wrapping_add(60));
                    bb = 236;
                }
                234 => {
                    bb = if ((self.r32(fp.wrapping_add(76)) as i32) != 0) {
                        233
                    } else {
                        235
                    };
                }
                235 => {
                    bb = 232;
                }
                236 => {
                    bb = if ((i as i32) < (self.r32(fp.wrapping_add(0)) as i32)) {
                        237
                    } else {
                        239
                    };
                }
                237 => {
                    bb = if ((self.r32(v72) as i32) != 0) {
                        240
                    } else {
                        241
                    };
                }
                238 => {
                    i = i.wrapping_add(1);
                    bb = 236;
                }
                239 => {
                    v74 = v74.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(fp.wrapping_add(76)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 234;
                }
                240 => {
                    bb = if ((v74 < 400_u32) && (i < 20_u32)) {
                        242
                    } else {
                        243
                    };
                }
                241 => {
                    v72 = v72.wrapping_add(4);
                    bb = 238;
                }
                242 => {
                    v76 = (self.r32(
                        (v71 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v74.wrapping_add(i)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v76 != (1_i32).wrapping_neg())
                        && (v76 == (self.r32(fp.wrapping_add(96)) as i32)))
                    {
                        244
                    } else {
                        245
                    };
                }
                243 => {
                    bb = 241;
                }
                244 => {
                    self.w32(fp.wrapping_add(80), ((self.r32(v72) as i32) as u32));
                    bb = 245;
                }
                245 => {
                    bb = 243;
                }
                246 => {
                    v80 = (20_i32.wrapping_mul(v73) as u32);
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(fp.wrapping_add(48)) as i32) as u32),
                    );
                    bb = 248;
                }
                247 => {
                    bb = if (v78 != 15_i32) { 287 } else { 288 };
                }
                248 => {
                    bb = if (1_i32 != 0) { 249 } else { 250 };
                }
                249 => {
                    bb = if (v77 >= v79) { 251 } else { 252 };
                }
                250 => {
                    bb = 247;
                }
                251 => {
                    bb = 253;
                }
                252 => {
                    bb = 255;
                }
                253 => {
                    v80 = v80.wrapping_add(20_u32);
                    bb = if (!({
                        let t5 = (self.r32(fp.wrapping_add(76)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(76), (t5 as u32));
                        t5
                    } != 0))
                    {
                        284
                    } else {
                        285
                    };
                }
                254 => {
                    bb = 252;
                }
                255 => {
                    bb = if (((self.r32(fp.wrapping_add(84)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(8))) as i32))
                        == 0_i32)
                    {
                        258
                    } else {
                        259
                    };
                }
                256 => {
                    bb = if (v77
                        < (self
                            .r32(fp.wrapping_add(60))
                            .wrapping_add(((self.r32(fp.wrapping_add(20)) as i32) as u32))
                            as i32))
                    {
                        255
                    } else {
                        257
                    };
                }
                257 => {
                    v77 = (self.r32(fp.wrapping_add(60)) as i32);
                    bb = 253;
                }
                258 => {
                    bb = 260;
                }
                259 => {
                    bb = if (v80 >= 400_u32) { 262 } else { 263 };
                }
                260 => {
                    v79 = (self
                        .r32(fp.wrapping_add(60))
                        .wrapping_add(((self.r32(fp.wrapping_add(20)) as i32) as u32))
                        as i32);
                    v77 = v77.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    bb = 256;
                }
                261 => {
                    bb = 259;
                }
                262 => {
                    bb = 260;
                }
                263 => {
                    bb = if ((v77 as u32) >= 20_u32) { 265 } else { 266 };
                }
                264 => {
                    bb = 263;
                }
                265 => {
                    bb = 260;
                }
                266 => {
                    v81 = v80.wrapping_add((v77 as u32));
                    bb = if (v80.wrapping_add((v77 as u32))
                        == ((self.r32(fp.wrapping_add(52)) as i32) as u32))
                    {
                        268
                    } else {
                        269
                    };
                }
                267 => {
                    bb = 266;
                }
                268 => {
                    bb = 260;
                }
                269 => {
                    v82 = (self.r32(
                        (v71 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v81))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v83 = 0_i8;
                    bb = if ((self.r32(fp.wrapping_add(124)) as i32) != 0) {
                        271
                    } else {
                        273
                    };
                }
                270 => {
                    bb = 269;
                }
                271 => {
                    bb = if ((self.r32(fp.wrapping_add(124)) as i32) == 1_i32) {
                        274
                    } else {
                        275
                    };
                }
                272 => {
                    self.w32(
                        (v71.wrapping_add(7400_i32) as u32),
                        ((self.r32((v71.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if ((v83 as i32) == 1_i32) { 282 } else { 283 };
                }
                273 => {
                    self.w32(
                        (v71.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v71.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v81 as i32) as u32),
                    );
                    bb = if ((v82 != (1_i32).wrapping_neg())
                        && (v82 == (self.r32(fp.wrapping_add(96)) as i32)))
                    {
                        280
                    } else {
                        281
                    };
                }
                274 => {
                    self.w32(
                        (v71.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v71.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v81.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if ((v82 != (1_i32).wrapping_neg()) && (v82 < 5_i32)) {
                        276
                    } else {
                        277
                    };
                }
                275 => {
                    bb = 272;
                }
                276 => {
                    bb = 278;
                }
                277 => {
                    bb = 275;
                }
                278 => {
                    v83 = 1_i8;
                    bb = 281;
                }
                279 => {
                    bb = 277;
                }
                280 => {
                    self.w32(
                        (v71.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v71.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v81.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = 278;
                }
                281 => {
                    bb = 272;
                }
                282 => {
                    let _ = self.f_100096c0(
                        self.r32(fp.wrapping_add(104)),
                        (self.r32(fp.wrapping_add(116)) as i32),
                        v82,
                        fp.wrapping_add(128),
                        (fp.wrapping_add(148) as i32),
                    );
                    bb = 283;
                }
                283 => {
                    bb = 260;
                }
                284 => {
                    v78 = (self.r32(fp.wrapping_add(80)) as i32);
                    bb = 250;
                }
                285 => {
                    bb = 248;
                }
                286 => {
                    bb = 285;
                }
                287 => {
                    let t6 = v78;
                    bb = match t6 {
                        1_i32 => 290,
                        2_i32 => 291,
                        4_i32 => 292,
                        8_i32 => 293,
                        _ => 294,
                    };
                }
                288 => {
                    v84 = (self.r32(
                        (v71.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(52)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v84 == (1_i32).wrapping_neg()) {
                        302
                    } else {
                        303
                    };
                }
                289 => {
                    bb = 300;
                }
                290 => {
                    v84 = 0_i32;
                    bb = 289;
                }
                291 => {
                    v84 = 1_i32;
                    bb = 289;
                }
                292 => {
                    v84 = 2_i32;
                    bb = 289;
                }
                293 => {
                    v84 = 3_i32;
                    bb = 289;
                }
                294 => {
                    v84 = (self.r32(fp.wrapping_add(120)) as i32);
                    bb = 289;
                }
                295 => {
                    bb = 291;
                }
                296 => {
                    bb = 292;
                }
                297 => {
                    bb = 293;
                }
                298 => {
                    bb = 294;
                }
                299 => {
                    bb = 289;
                }
                300 => {
                    v89 = (self.r32(
                        (v71.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(52)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v89 == (1_i32).wrapping_neg()) {
                        329
                    } else {
                        330
                    };
                }
                301 => {
                    bb = 288;
                }
                302 => {
                    v84 = (self.r32(fp.wrapping_add(112)) as i32);
                    bb = 303;
                }
                303 => {
                    v85 = (self.r32(fp.wrapping_add(32)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(24)) as i32));
                    v86 = (self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32));
                    v87 = (self.r32(fp.wrapping_add(32)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(24)) as i32))
                        .wrapping_abs();
                    v88 = (self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32))
                        .wrapping_abs();
                    bb = if (v88 != v87) { 304 } else { 305 };
                }
                304 => {
                    bb = if (v88 < v87) { 306 } else { 307 };
                }
                305 => {
                    bb = if (v84 != 0) { 311 } else { 313 };
                }
                306 => {
                    v84 = (if (v85 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 300;
                }
                307 => {
                    bb = 309;
                }
                308 => {
                    bb = 307;
                }
                309 => {
                    v84 = (if (v86 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 300;
                }
                310 => {
                    bb = 305;
                }
                311 => {
                    bb = if (v84 == 1_i32) { 314 } else { 316 };
                }
                312 => {
                    bb = 300;
                }
                313 => {
                    bb = if (v85 > 0_i32) { 327 } else { 328 };
                }
                314 => {
                    bb = if (v86 < 0_i32) { 317 } else { 318 };
                }
                315 => {
                    bb = 312;
                }
                316 => {
                    bb = if (v84 == 2_i32) { 319 } else { 320 };
                }
                317 => {
                    v84 = (if (v85 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 318;
                }
                318 => {
                    bb = 315;
                }
                319 => {
                    bb = if (v85 >= 0_i32) { 321 } else { 322 };
                }
                320 => {
                    bb = if ((v84 == 3_i32) && (v86 > 0_i32)) {
                        325
                    } else {
                        326
                    };
                }
                321 => {
                    bb = 300;
                }
                322 => {
                    bb = 309;
                }
                323 => {
                    bb = 322;
                }
                324 => {
                    bb = 320;
                }
                325 => {
                    v84 = (if (v85 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 326;
                }
                326 => {
                    bb = 315;
                }
                327 => {
                    v84 = (if (v86 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 328;
                }
                328 => {
                    bb = 312;
                }
                329 => {
                    v89 = (self.r32(fp.wrapping_add(112)) as i32);
                    bb = 330;
                }
                330 => {
                    bb = if (v84 == v89) { 331 } else { 332 };
                }
                331 => {
                    v84 = (1_i32).wrapping_neg();
                    bb = 332;
                }
                332 => {
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(24)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(88)).wrapping_add(1540), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(88)).wrapping_add(1556),
                        (v84 as u32),
                    );
                    return 0_i32;
                }
                333 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10015E00` (805 bytes).
    pub(crate) fn f_10015e00(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(1856);
        let r = self.f_10015e00_body(fp, this);
        self.leave(1856);
        r
    }

    fn f_10015e00_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i64 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: u32 = 0;
        let mut v22: i32 = 0;
        let mut v23: u32 = 0;
        let mut v24: i32 = 0;
        let mut v25: u32 = 0;
        let mut v26: u32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        v1 = ((self.r32(self.r32(this)) as i32) as u32);
        v2 = 0_i32;
        v3 = fp.wrapping_add(152);
        v4 = ((self.r32(v1.wrapping_add(28)) as i32) as u32);
        v5 = ((self.r32(v1.wrapping_add(12)) as i32) as u32);
        self.w32(
            fp.wrapping_add(0),
            ((self.r32(v1.wrapping_add(20)) as i32) as u32),
        );
        v6 = (self.r32(v4.wrapping_add(1388)) as i32);
        self.w32(fp.wrapping_add(44), v5);
        self.w32(fp.wrapping_add(16), v4);
        v7 = (self.r32(
            v5.wrapping_add((39_i32.wrapping_mul(v6).wrapping_add(932_i32) as u32).wrapping_mul(4)),
        ) as i32);
        self.w32(fp.wrapping_add(40), (v7 as u32));
        self.w32(
            fp.wrapping_add(20),
            ((self.r32(v5.wrapping_add(
                (39_i32.wrapping_mul(v6).wrapping_add(931_i32) as u32).wrapping_mul(4),
            )) as i32) as u32),
        );
        v8 = 0_i32;
        v9 = v5;
        'l1: loop {
            if (((self.r32(v9) as i32) == 1_i32)
                && ((self.r32(v9.wrapping_add(372)) as i32) != 99_i32))
            {
                self.w32(v3, (v8 as u32));
                v2 = v2.wrapping_add(1);
                v3 = v3.wrapping_add(4);
            }
            v9 = v9.wrapping_add(716);
            v8 = v8.wrapping_add(1);
            if !(v8 < 5_i32) {
                break 'l1;
            }
        }
        v10 = 0_i32;
        self.w32(fp.wrapping_add(28), (v2 as u32));
        if (v2 > 0_i32) {
            'l2: loop {
                v11 = ((self.r32(
                    v5.wrapping_add(
                        (179_i32
                            .wrapping_mul(
                                (self.r32(
                                    fp.wrapping_add(152)
                                        .wrapping_add((v10 as u32).wrapping_mul(4)),
                                ) as i32),
                            )
                            .wrapping_add(96_i32) as u32)
                            .wrapping_mul(4),
                    ),
                ) as i32)
                    .wrapping_sub(v7) as i64);
                self.w32(
                    fp.wrapping_add(52)
                        .wrapping_add((v10 as u32).wrapping_mul(4)),
                    ((((self.r32(
                        v5.wrapping_add(
                            (179_i32
                                .wrapping_mul(
                                    (self.r32(
                                        fp.wrapping_add(152)
                                            .wrapping_add((v10 as u32).wrapping_mul(4)),
                                    ) as i32),
                                )
                                .wrapping_add(95_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs() as i64)
                        .wrapping_add((((((v11 as u64) >> 32) as u32) as i64) ^ v11))
                        .wrapping_sub(((((v11 as u64) >> 32) as u32) as i64))
                        as i32) as u32),
                );
                v10 = v10.wrapping_add(1);
                if !(v10 < v2) {
                    break 'l2;
                }
            }
            v4 = self.r32(fp.wrapping_add(16));
        }
        i = v2.wrapping_sub(1_i32);
        'l3: loop {
            if !(i > 0_i32) {
                break 'l3;
            }
            j = 0_i32;
            'l4: loop {
                if !(j < i) {
                    break 'l4;
                }
                v14 = (self.r32(
                    fp.wrapping_add(52)
                        .wrapping_add((j.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                ) as i32);
                v15 =
                    (self.r32(fp.wrapping_add(52).wrapping_add((j as u32).wrapping_mul(4))) as i32);
                if (v14 < v15) {
                    v16 = (self.r32(
                        fp.wrapping_add(152)
                            .wrapping_add((j as u32).wrapping_mul(4)),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(152)
                            .wrapping_add((j as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(152)
                                .wrapping_add((j.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52).wrapping_add((j as u32).wrapping_mul(4)),
                        (v14 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(152)
                            .wrapping_add((j.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        (v16 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52)
                            .wrapping_add((j.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        (v15 as u32),
                    );
                }
                j = j.wrapping_add(1);
            }
            v4 = self.r32(fp.wrapping_add(16));
            i = i.wrapping_sub(1);
        }
        if ((self.r32(fp.wrapping_add(52)) as i32) < 8_i32) {
            self.w32(fp.wrapping_add(36), 0_u32);
            v18 = (self.r32(((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(6440_i32) as u32))
                as i32);
            if (v18 > 0_i32) {
                self.w32(
                    fp.wrapping_add(36),
                    ((self
                        .r32(((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(6440_i32) as u32))
                        as i32) as u32),
                );
                let _ = self.memcpy(
                    fp.wrapping_add(252),
                    ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(4840_i32) as u32),
                    (4_i32.wrapping_mul(v18) as u32),
                );
            }
            v19 = 0_i32;
            self.w32(fp.wrapping_add(0), ((1_i32).wrapping_neg() as u32));
            self.w32(fp.wrapping_add(12), 0_u32);
            self.w32(fp.wrapping_add(4), 0_u32);
            if ((self.r32(fp.wrapping_add(36)) as i32) > 0_i32) {
                self.w32(fp.wrapping_add(24), fp.wrapping_add(252));
                'l5: while (1_i32 != 0) {
                    v20 = crem_i32((self.r32(self.r32(fp.wrapping_add(24))) as i32), 20_i32);
                    self.w32(
                        fp.wrapping_add(48),
                        (cdiv_i32((self.r32(self.r32(fp.wrapping_add(24))) as i32), 20_i32) as u32),
                    );
                    v21 = (v7
                        .wrapping_sub((self.r32(fp.wrapping_add(48)) as i32))
                        .wrapping_abs() as u32);
                    v22 = 0_i32;
                    self.w32(fp.wrapping_add(8), 0_u32);
                    v23 = ((self.r32(fp.wrapping_add(20)) as i32)
                        .wrapping_sub(v20)
                        .wrapping_abs() as u32)
                        .wrapping_add(v21);
                    v24 = 0_i32;
                    self.w32(fp.wrapping_add(32), 0_u32);
                    if ((self.r32(fp.wrapping_add(28)) as i32) > 0_i32) {
                        'l6: loop {
                            if ((!(v22 != 0))
                                || ((self.r32(
                                    fp.wrapping_add(52)
                                        .wrapping_add((v22 as u32).wrapping_mul(4)),
                                ) as i32)
                                    < 12_i32))
                            {
                                v25 = ((self.r32(
                                    self.r32(fp.wrapping_add(44)).wrapping_add(
                                        (179_i32
                                            .wrapping_mul(
                                                (self.r32(
                                                    fp.wrapping_add(152)
                                                        .wrapping_add((v22 as u32).wrapping_mul(4)),
                                                )
                                                    as i32),
                                            )
                                            .wrapping_add(95_i32)
                                            as u32)
                                            .wrapping_mul(4),
                                    ),
                                ) as i32)
                                    .wrapping_sub(v20)
                                    .wrapping_abs() as u32);
                                v26 = ((self.r32(
                                    self.r32(fp.wrapping_add(44)).wrapping_add(
                                        (179_i32
                                            .wrapping_mul(
                                                (self.r32(
                                                    fp.wrapping_add(152)
                                                        .wrapping_add((v22 as u32).wrapping_mul(4)),
                                                )
                                                    as i32),
                                            )
                                            .wrapping_add(96_i32)
                                            as u32)
                                            .wrapping_mul(4),
                                    ),
                                ) as i32)
                                    .wrapping_sub((self.r32(fp.wrapping_add(48)) as i32))
                                    .wrapping_abs() as u32);
                                self.w32(
                                    fp.wrapping_add(32),
                                    ((((self.r32(fp.wrapping_add(32)) as i32) as u32)
                                        .wrapping_add(v26.wrapping_add(v25))
                                        as i32) as u32),
                                );
                                self.w32(
                                    fp.wrapping_add(8),
                                    ((self.r32(fp.wrapping_add(8)) as i32)
                                        .wrapping_add((v25.wrapping_sub(v26) as i32).wrapping_abs())
                                        as u32),
                                );
                            }
                            v22 = v22.wrapping_add(1);
                            if !(v22 < (self.r32(fp.wrapping_add(28)) as i32)) {
                                break 'l6;
                            }
                        }
                        v24 = (self.r32(fp.wrapping_add(32)) as i32);
                        v19 = (self.r32(fp.wrapping_add(4)) as i32);
                    }
                    v27 = (v23.wrapping_add((6_i32.wrapping_mul(v24) as u32)) as i32);
                    if ((self.r32(fp.wrapping_add(0)) as i32) < v27) {
                        self.w32(fp.wrapping_add(0), (v27 as u32));
                        self.w32(fp.wrapping_add(12), (v19 as u32));
                    }
                    self.w32(
                        fp.wrapping_add(4),
                        ({
                            let t1 = v19.wrapping_add(1);
                            v19 = t1;
                            t1
                        } as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    if (v19 >= (self.r32(fp.wrapping_add(36)) as i32)) {
                        break 'l5;
                    }
                    v7 = (self.r32(fp.wrapping_add(40)) as i32);
                }
                v4 = self.r32(fp.wrapping_add(16));
            }
            self.w32(v4.wrapping_add(1540), 2_u32);
            v28 = (self.r32(
                fp.wrapping_add(252)
                    .wrapping_add(((self.r32(fp.wrapping_add(12)) as i32) as u32).wrapping_mul(4)),
            ) as i32);
            self.w32(
                self.r32(fp.wrapping_add(16)).wrapping_add(1552),
                (v28 as u32),
            );
            self.w32(
                self.r32(fp.wrapping_add(16)).wrapping_add(1556),
                ((1_i32).wrapping_neg() as u32),
            );
            self.w32(
                self.r32(fp.wrapping_add(16)).wrapping_add(1544),
                (crem_i32(v28, 20_i32) as u32),
            );
            self.w32(
                self.r32(fp.wrapping_add(16)).wrapping_add(1548),
                (cdiv_i32(v28, 20_i32) as u32),
            );
            return 0_i32;
        } else {
            self.w32(
                v4.wrapping_add(1544),
                ((self.r32(fp.wrapping_add(20)) as i32) as u32),
            );
            self.w32(v4.wrapping_add(1548), (v7 as u32));
            self.w32(v4.wrapping_add(1540), 2_u32);
            self.w32(
                v4.wrapping_add(1552),
                ((self.r32(fp.wrapping_add(20)) as i32).wrapping_add(20_i32.wrapping_mul(v7))
                    as u32),
            );
            self.w32(v4.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }
}
