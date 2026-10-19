//! Functions 0x1000F8C0..=0x100122B0, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_1000F8C0` (340 bytes).
    pub(crate) fn f_1000f8c0(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = this;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        v3 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 2_u32);
        self.w32(v2.wrapping_add(1460), 2_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 2_u32);
        self.w32(v2.wrapping_add(1476), 2_u32);
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
        if ((self.r32(v2.wrapping_add(1492)) as i32) == 1_i32) {
            if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
            {
                v5 = 0_i32;
                v6 = v3.wrapping_add(2060);
                'l1: loop {
                    if (!((self.r32(v6) as i32) != 0)) {
                        break 'l1;
                    }
                    v5 = v5.wrapping_add(1);
                    v6 = v6.wrapping_add(4);
                    if !(v5 < 2_i32) {
                        break 'l1;
                    }
                }
                v1 = this;
                self.w32(
                    v3.wrapping_add((v5.wrapping_add(515_i32) as u32).wrapping_mul(4)),
                    42_u32,
                );
                self.w32(
                    v3.wrapping_add((v5.wrapping_add(517_i32) as u32).wrapping_mul(4)),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
            let _ = self.f_100074e0(
                v1,
                (self.r32(v2.wrapping_add(1452)) as i32),
                (self.r32(v2.wrapping_add(1456)) as i32),
                (self.r32(v2.wrapping_add(1460)) as i32),
            );
        }
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_1000FA20` (132 bytes).
    pub(crate) fn f_1000fa20(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(12_i32) as u32));
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 2_u32);
        self.w32(v3.wrapping_add(1460), 3_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 2_u32);
        self.w32(v3.wrapping_add(1476), 3_u32);
        if (((self.r32(v2.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v2.wrapping_add(1804)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(2140), 62_u32);
            self.w32(v2.wrapping_add(2144), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000FAB0` (165 bytes).
    pub(crate) fn f_1000fab0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(12_i32) as u32));
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 2_u32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 2_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        if (((self.r32(v2.wrapping_add(1432)) as i32) != 1_i32)
            || ((self.r32(v2.wrapping_add(1804)) as i32) == 99_i32))
        {
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        } else {
            v5 = 0_i32;
            v6 = v2.wrapping_add(2076);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 3_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v2.wrapping_add((v5.wrapping_add(519_i32) as u32).wrapping_mul(4)),
                44_u32,
            );
            self.w32(
                v2.wrapping_add((v5.wrapping_add(522_i32) as u32).wrapping_mul(4)),
                1_u32,
            );
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000FB60` (199 bytes).
    pub(crate) fn f_1000fb60(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 2_u32);
        self.w32(v2.wrapping_add(1460), 5_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 2_u32);
        self.w32(v2.wrapping_add(1476), 5_u32);
        if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(2024);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 3_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(506_i32) as u32).wrapping_mul(4)),
                39_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(509_i32) as u32).wrapping_mul(4)),
                4_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(512_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000FC30` (252 bytes).
    pub(crate) fn f_1000fc30(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_1000fc30_body(fp, this);
        self.leave(16);
        r
    }

    fn f_1000fc30_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self.r32((v2.wrapping_add(12_i32) as u32));
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 6_u32);
        self.w32(v3.wrapping_add(1476), 6_u32);
        self.w32(v3.wrapping_add(1456), 2_u32);
        self.w32(v3.wrapping_add(1472), 2_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        let _ = self.f_10006d10(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            if (((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
            {
                v7 = 0_i32;
                v8 = v4.wrapping_add(1960);
                'l1: loop {
                    if (!((self.r32(v8) as i32) != 0)) {
                        break 'l1;
                    }
                    v7 = v7.wrapping_add(1);
                    v8 = v8.wrapping_add(4);
                    if !(v7 < 2_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v4.wrapping_add((v7.wrapping_add(490_i32) as u32).wrapping_mul(4)),
                    23_u32,
                );
                self.w32(
                    v4.wrapping_add((v7.wrapping_add(492_i32) as u32).wrapping_mul(4)),
                    4_u32,
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

    /// `sub_1000FD30` (186 bytes).
    pub(crate) fn f_1000fd30(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 2_u32);
        self.w32(v2.wrapping_add(1460), 7_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 2_u32);
        self.w32(v2.wrapping_add(1476), 7_u32);
        if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(1840);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(460_i32) as u32).wrapping_mul(4)),
                8_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(475_i32) as u32).wrapping_mul(4)),
                6_u32,
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

    /// `sub_1000FDF0` (132 bytes).
    pub(crate) fn f_1000fdf0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(12_i32) as u32));
        v3 = self.r32((v1.wrapping_add(28_i32) as u32));
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 2_u32);
        self.w32(v3.wrapping_add(1460), 8_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 2_u32);
        self.w32(v3.wrapping_add(1476), 8_u32);
        if (((self.r32(v2.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v2.wrapping_add(1804)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(2140), 63_u32);
            self.w32(v2.wrapping_add(2144), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000FE80` (246 bytes).
    pub(crate) fn f_1000fe80(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_1000fe80_body(fp, this);
        self.leave(16);
        r
    }

    fn f_1000fe80_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self.r32((v2.wrapping_add(12_i32) as u32));
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1456), 2_u32);
        self.w32(v3.wrapping_add(1472), 2_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 9_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 9_u32);
        let _ = self.f_10006b10(this, v5, fp.wrapping_add(0));
        if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
            self.w32(v3.wrapping_add(1480), 1_u32);
            if (((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
            {
                v6 = 0_i32;
                v7 = v4.wrapping_add(1840);
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
                    v4.wrapping_add((v6.wrapping_add(460_i32) as u32).wrapping_mul(4)),
                    9_u32,
                );
                self.w32(
                    v4.wrapping_add((v6.wrapping_add(475_i32) as u32).wrapping_mul(4)),
                    11_u32,
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

    /// `sub_1000FF80` (578 bytes).
    pub(crate) fn f_1000ff80(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
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
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1460), 0_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1476), 0_u32);
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
        if (((self.r32(v3) as i32) == 1_i32) && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                10_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            v8 = v3.wrapping_add(1124);
            'l2: loop {
                if (!((self.r32(v8) as i32) != 0)) {
                    break 'l2;
                }
                v7 = v7.wrapping_add(1);
                v8 = v8.wrapping_add(4);
                if !(v7 < 15_i32) {
                    break 'l2;
                }
            }
            self.w32(
                v3.wrapping_add((v7.wrapping_add(281_i32) as u32).wrapping_mul(4)),
                10_u32,
            );
            self.w32(
                v3.wrapping_add((v7.wrapping_add(296_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
        {
            v9 = 0_i32;
            v10 = v3.wrapping_add(1840);
            'l3: loop {
                if (!((self.r32(v10) as i32) != 0)) {
                    break 'l3;
                }
                v9 = v9.wrapping_add(1);
                v10 = v10.wrapping_add(4);
                if !(v9 < 15_i32) {
                    break 'l3;
                }
            }
            self.w32(
                v3.wrapping_add((v9.wrapping_add(460_i32) as u32).wrapping_mul(4)),
                10_u32,
            );
            self.w32(
                v3.wrapping_add((v9.wrapping_add(475_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v11 = 0_i32;
            v12 = v3.wrapping_add(2556);
            'l4: loop {
                if (!((self.r32(v12) as i32) != 0)) {
                    break 'l4;
                }
                v11 = v11.wrapping_add(1);
                v12 = v12.wrapping_add(4);
                if !(v11 < 15_i32) {
                    break 'l4;
                }
            }
            self.w32(
                v3.wrapping_add((v11.wrapping_add(639_i32) as u32).wrapping_mul(4)),
                10_u32,
            );
            self.w32(
                v3.wrapping_add((v11.wrapping_add(654_i32) as u32).wrapping_mul(4)),
                6_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
        {
            v13 = 0_i32;
            v14 = v3.wrapping_add(3272);
            'l5: loop {
                if (!((self.r32(v14) as i32) != 0)) {
                    break 'l5;
                }
                v13 = v13.wrapping_add(1);
                v14 = v14.wrapping_add(4);
                if !(v13 < 15_i32) {
                    break 'l5;
                }
            }
            self.w32(
                v3.wrapping_add((v13.wrapping_add(818_i32) as u32).wrapping_mul(4)),
                10_u32,
            );
            self.w32(
                v3.wrapping_add((v13.wrapping_add(833_i32) as u32).wrapping_mul(4)),
                5_u32,
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

    /// `sub_100101D0` (548 bytes).
    pub(crate) fn f_100101d0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1460), 1_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1476), 1_u32);
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
        if (((self.r32(v3) as i32) == 1_i32) && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(628);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 2_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(157_i32) as u32).wrapping_mul(4)),
                43_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(159_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            v8 = v3.wrapping_add(1344);
            'l2: loop {
                if (!((self.r32(v8) as i32) != 0)) {
                    break 'l2;
                }
                v7 = v7.wrapping_add(1);
                v8 = v8.wrapping_add(4);
                if !(v7 < 2_i32) {
                    break 'l2;
                }
            }
            self.w32(
                v3.wrapping_add((v7.wrapping_add(336_i32) as u32).wrapping_mul(4)),
                43_u32,
            );
            self.w32(
                v3.wrapping_add((v7.wrapping_add(338_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
        {
            v9 = 0_i32;
            v10 = v3.wrapping_add(2060);
            'l3: loop {
                if (!((self.r32(v10) as i32) != 0)) {
                    break 'l3;
                }
                v9 = v9.wrapping_add(1);
                v10 = v10.wrapping_add(4);
                if !(v9 < 2_i32) {
                    break 'l3;
                }
            }
            self.w32(
                v3.wrapping_add((v9.wrapping_add(515_i32) as u32).wrapping_mul(4)),
                43_u32,
            );
            self.w32(
                v3.wrapping_add((v9.wrapping_add(517_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
        {
            v11 = 0_i32;
            v12 = v3.wrapping_add(3492);
            'l4: loop {
                if (!((self.r32(v12) as i32) != 0)) {
                    break 'l4;
                }
                v11 = v11.wrapping_add(1);
                v12 = v12.wrapping_add(4);
                if !(v11 < 2_i32) {
                    break 'l4;
                }
            }
            self.w32(
                v3.wrapping_add((v11.wrapping_add(873_i32) as u32).wrapping_mul(4)),
                43_u32,
            );
            self.w32(
                v3.wrapping_add((v11.wrapping_add(875_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
        }
        if (((((self.r32(v2.wrapping_add(1480)) as i32) == 1_i32)
            || ((self.r32(v2.wrapping_add(1484)) as i32) == 1_i32))
            || ((self.r32(v2.wrapping_add(1488)) as i32) == 1_i32))
            || ((self.r32(v2.wrapping_add(1496)) as i32) == 1_i32))
        {
            let _ = self.f_100074e0(
                this,
                (self.r32(v2.wrapping_add(1452)) as i32),
                (self.r32(v2.wrapping_add(1456)) as i32),
                (self.r32(v2.wrapping_add(1460)) as i32),
            );
        }
        self.w32(v2.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10010400` (262 bytes).
    pub(crate) fn f_10010400(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v10 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 2_u32);
        self.w32(v3.wrapping_add(1476), 2_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1456), 3_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 3_u32);
        let _ = self.f_10016940(this);
        v9 = 0_i32;
        if ((self.r32(v3.wrapping_add(2072)) as i32) > 0_i32) {
            v5 = v3.wrapping_add(1572);
            'l1: loop {
                v6 = (self.r32(v5) as i32).wrapping_sub(5_i32);
                v7 = crem_i32(self.rand(), 5_i32).wrapping_add(1_i32);
                if (((self.r32(
                    (v10.wrapping_add(156_i32.wrapping_mul(v6))
                        .wrapping_add(3600_i32) as u32),
                ) as i32)
                    == 1_i32)
                    && ((self.r32(
                        (v10.wrapping_add(156_i32.wrapping_mul(v6))
                            .wrapping_add(3716_i32) as u32),
                    ) as i32)
                        != 99_i32))
                {
                    self.w32(
                        (v10.wrapping_add(156_i32.wrapping_mul(v6))
                            .wrapping_add(3740_i32) as u32),
                        52_u32,
                    );
                    self.w32(
                        (v10.wrapping_add(156_i32.wrapping_mul(v6))
                            .wrapping_add(3744_i32) as u32),
                        (v7 as u32),
                    );
                }
                v5 = v5.wrapping_add(4);
                v9 = v9.wrapping_add(1);
                if !(v9 < (self.r32(v3.wrapping_add(2072)) as i32)) {
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

    /// `sub_10010510` (188 bytes).
    pub(crate) fn f_10010510(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1460), 3_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1476), 3_u32);
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(2740);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 3_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(685_i32) as u32).wrapping_mul(4)),
                40_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(688_i32) as u32).wrapping_mul(4)),
                3_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(691_i32) as u32).wrapping_mul(4)),
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

    /// `sub_100105D0` (322 bytes).
    pub(crate) fn f_100105d0(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = self.r32((v2.wrapping_add(12_i32) as u32));
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1456), 3_u32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1472), 3_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        self.w32(v3.wrapping_add(1480), 0_u32);
        if (((self.r32(v4) as i32) == 1_i32) && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
        }
        self.w32(v3.wrapping_add(1484), 0_u32);
        if (((self.r32(v4.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v4.wrapping_add(1088)) as i32) != 99_i32))
        {
            self.w32(v3.wrapping_add(1484), 1_u32);
        }
        self.w32(v3.wrapping_add(1488), 0_u32);
        if (((self.r32(v4.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v4.wrapping_add(1804)) as i32) != 99_i32))
        {
            self.w32(v3.wrapping_add(1488), 1_u32);
        }
        self.w32(v3.wrapping_add(1492), 0_u32);
        if (((self.r32(v4.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v4.wrapping_add(2520)) as i32) != 99_i32))
        {
            self.w32(v3.wrapping_add(1492), 1_u32);
        }
        self.w32(v3.wrapping_add(1496), 0_u32);
        if (((self.r32(v4.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v4.wrapping_add(3236)) as i32) != 99_i32))
        {
            self.w32(v3.wrapping_add(1496), 1_u32);
        }
        if (((((self.r32(v3.wrapping_add(1480)) as i32) == 1_i32)
            || ((self.r32(v3.wrapping_add(1484)) as i32) == 1_i32))
            || ((self.r32(v3.wrapping_add(1488)) as i32) == 1_i32))
            || ((self.r32(v3.wrapping_add(1496)) as i32) == 1_i32))
        {
            let _ = self.f_10016ab0(this);
            let _ = self.f_100074e0(
                this,
                (self.r32(v3.wrapping_add(1452)) as i32),
                (self.r32(v3.wrapping_add(1456)) as i32),
                (self.r32(v3.wrapping_add(1460)) as i32),
            );
        }
        self.w32(v3.wrapping_add(1048), 1_u32);
        return 0_i32;
    }

    /// `sub_10010720` (194 bytes).
    pub(crate) fn f_10010720(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1460), 5_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1476), 5_u32);
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(2844);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 1_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(711_i32) as u32).wrapping_mul(4)),
                54_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(712_i32) as u32).wrapping_mul(4)),
                1_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(713_i32) as u32).wrapping_mul(4)),
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

    /// `sub_100107F0` (583 bytes).
    pub(crate) fn f_100107f0(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
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
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1460), 6_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1476), 6_u32);
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
        if (((self.r32(v3) as i32) == 1_i32) && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                11_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            v8 = v3.wrapping_add(1124);
            'l2: loop {
                if (!((self.r32(v8) as i32) != 0)) {
                    break 'l2;
                }
                v7 = v7.wrapping_add(1);
                v8 = v8.wrapping_add(4);
                if !(v7 < 15_i32) {
                    break 'l2;
                }
            }
            self.w32(
                v3.wrapping_add((v7.wrapping_add(281_i32) as u32).wrapping_mul(4)),
                11_u32,
            );
            self.w32(
                v3.wrapping_add((v7.wrapping_add(296_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
        {
            v9 = 0_i32;
            v10 = v3.wrapping_add(1840);
            'l3: loop {
                if (!((self.r32(v10) as i32) != 0)) {
                    break 'l3;
                }
                v9 = v9.wrapping_add(1);
                v10 = v10.wrapping_add(4);
                if !(v9 < 15_i32) {
                    break 'l3;
                }
            }
            self.w32(
                v3.wrapping_add((v9.wrapping_add(460_i32) as u32).wrapping_mul(4)),
                11_u32,
            );
            self.w32(
                v3.wrapping_add((v9.wrapping_add(475_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v11 = 0_i32;
            v12 = v3.wrapping_add(2556);
            'l4: loop {
                if (!((self.r32(v12) as i32) != 0)) {
                    break 'l4;
                }
                v11 = v11.wrapping_add(1);
                v12 = v12.wrapping_add(4);
                if !(v11 < 15_i32) {
                    break 'l4;
                }
            }
            self.w32(
                v3.wrapping_add((v11.wrapping_add(639_i32) as u32).wrapping_mul(4)),
                11_u32,
            );
            self.w32(
                v3.wrapping_add((v11.wrapping_add(654_i32) as u32).wrapping_mul(4)),
                6_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
        {
            v13 = 0_i32;
            v14 = v3.wrapping_add(3272);
            'l5: loop {
                if (!((self.r32(v14) as i32) != 0)) {
                    break 'l5;
                }
                v13 = v13.wrapping_add(1);
                v14 = v14.wrapping_add(4);
                if !(v13 < 15_i32) {
                    break 'l5;
                }
            }
            self.w32(
                v3.wrapping_add((v13.wrapping_add(818_i32) as u32).wrapping_mul(4)),
                11_u32,
            );
            self.w32(
                v3.wrapping_add((v13.wrapping_add(833_i32) as u32).wrapping_mul(4)),
                5_u32,
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

    /// `sub_10010A40` (199 bytes).
    pub(crate) fn f_10010a40(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1460), 7_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1476), 7_u32);
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(2716);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 2_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(679_i32) as u32).wrapping_mul(4)),
                33_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(681_i32) as u32).wrapping_mul(4)),
                6_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(683_i32) as u32).wrapping_mul(4)),
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

    /// `sub_10010B10` (583 bytes).
    pub(crate) fn f_10010b10(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
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
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1460), 8_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1476), 8_u32);
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
        if (((self.r32(v3) as i32) == 1_i32) && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(408);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                12_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            v8 = v3.wrapping_add(1124);
            'l2: loop {
                if (!((self.r32(v8) as i32) != 0)) {
                    break 'l2;
                }
                v7 = v7.wrapping_add(1);
                v8 = v8.wrapping_add(4);
                if !(v7 < 15_i32) {
                    break 'l2;
                }
            }
            self.w32(
                v3.wrapping_add((v7.wrapping_add(281_i32) as u32).wrapping_mul(4)),
                12_u32,
            );
            self.w32(
                v3.wrapping_add((v7.wrapping_add(296_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1804)) as i32) != 99_i32))
        {
            v9 = 0_i32;
            v10 = v3.wrapping_add(1840);
            'l3: loop {
                if (!((self.r32(v10) as i32) != 0)) {
                    break 'l3;
                }
                v9 = v9.wrapping_add(1);
                v10 = v10.wrapping_add(4);
                if !(v9 < 15_i32) {
                    break 'l3;
                }
            }
            self.w32(
                v3.wrapping_add((v9.wrapping_add(460_i32) as u32).wrapping_mul(4)),
                12_u32,
            );
            self.w32(
                v3.wrapping_add((v9.wrapping_add(475_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v11 = 0_i32;
            v12 = v3.wrapping_add(2556);
            'l4: loop {
                if (!((self.r32(v12) as i32) != 0)) {
                    break 'l4;
                }
                v11 = v11.wrapping_add(1);
                v12 = v12.wrapping_add(4);
                if !(v11 < 15_i32) {
                    break 'l4;
                }
            }
            self.w32(
                v3.wrapping_add((v11.wrapping_add(639_i32) as u32).wrapping_mul(4)),
                12_u32,
            );
            self.w32(
                v3.wrapping_add((v11.wrapping_add(654_i32) as u32).wrapping_mul(4)),
                6_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
        {
            v13 = 0_i32;
            v14 = v3.wrapping_add(3272);
            'l5: loop {
                if (!((self.r32(v14) as i32) != 0)) {
                    break 'l5;
                }
                v13 = v13.wrapping_add(1);
                v14 = v14.wrapping_add(4);
                if !(v13 < 15_i32) {
                    break 'l5;
                }
            }
            self.w32(
                v3.wrapping_add((v13.wrapping_add(818_i32) as u32).wrapping_mul(4)),
                12_u32,
            );
            self.w32(
                v3.wrapping_add((v13.wrapping_add(833_i32) as u32).wrapping_mul(4)),
                5_u32,
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

    /// `sub_10010D60` (186 bytes).
    pub(crate) fn f_10010d60(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 3_u32);
        self.w32(v2.wrapping_add(1460), 9_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 3_u32);
        self.w32(v2.wrapping_add(1476), 9_u32);
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(2556);
            'l1: loop {
                if (!((self.r32(v6) as i32) != 0)) {
                    break 'l1;
                }
                v5 = v5.wrapping_add(1);
                v6 = v6.wrapping_add(4);
                if !(v5 < 15_i32) {
                    break 'l1;
                }
            }
            self.w32(
                v3.wrapping_add((v5.wrapping_add(639_i32) as u32).wrapping_mul(4)),
                13_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(654_i32) as u32).wrapping_mul(4)),
                6_u32,
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

    /// `sub_10010E20` (203 bytes).
    pub(crate) fn f_10010e20(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 5_u32);
        self.w32(v2.wrapping_add(1472), 5_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1460), 0_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1476), 0_u32);
        v5 = (v3.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
        if (((self.r32(v5) as i32) == 1_i32) && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
        {
            v6 = 0_i32;
            v7 = v5.wrapping_add(668);
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
                v5.wrapping_add((v6.wrapping_add(167_i32) as u32).wrapping_mul(4)),
                49_u32,
            );
            self.w32(
                v5.wrapping_add((v6.wrapping_add(169_i32) as u32).wrapping_mul(4)),
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

    /// `sub_10010EF0` (267 bytes).
    pub(crate) fn f_10010ef0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10010ef0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10010ef0_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 5_u32);
        self.w32(v3.wrapping_add(1472), 5_u32);
        self.w32(fp.wrapping_add(2), this);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1476), 1_u32);
        let _ = self.f_10006c80(this, v4, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v6 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
            if (((self.r32(v6) as i32) == 1_i32)
                && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
            {
                v7 = 0_i32;
                v8 = v6.wrapping_add(408);
                'l1: loop {
                    if (!((self.r32(v8) as i32) != 0)) {
                        break 'l1;
                    }
                    v7 = v7.wrapping_add(1);
                    v8 = v8.wrapping_add(4);
                    if !(v7 < 15_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                    14_u32,
                );
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(117_i32) as u32).wrapping_mul(4)),
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

    /// `sub_10011000` (286 bytes).
    pub(crate) fn f_10011000(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011000_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011000_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1456), 5_u32);
        self.w32(v3.wrapping_add(1472), 5_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 2_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 2_u32);
        let _ = self.f_10006d10(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            && (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v6 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v6) as i32) == 1_i32)
                && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
            {
                v7 = 0_i32;
                v8 = v6.wrapping_add(528);
                'l1: loop {
                    if (!((self.r32(v8) as i32) != 0)) {
                        break 'l1;
                    }
                    v7 = v7.wrapping_add(1);
                    v8 = v8.wrapping_add(4);
                    if !(v7 < 2_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(132_i32) as u32).wrapping_mul(4)),
                    24_u32,
                );
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(134_i32) as u32).wrapping_mul(4)),
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

    /// `sub_10011120` (278 bytes).
    pub(crate) fn f_10011120(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011120_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011120_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1460), 3_u32);
        self.w32(v3.wrapping_add(1476), 3_u32);
        self.w32(v3.wrapping_add(1456), 5_u32);
        self.w32(v3.wrapping_add(1472), 5_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        let _ = self.f_10006d10(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(552);
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
                    v7.wrapping_add((v8.wrapping_add(138_i32) as u32).wrapping_mul(4)),
                    29_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(139_i32) as u32).wrapping_mul(4)),
                    2_u32,
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

    /// `sub_10011240` (222 bytes).
    pub(crate) fn f_10011240(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011240_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011240_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        self.w32(
            fp.wrapping_add(2),
            ((self.r32((v2.wrapping_add(12_i32) as u32)) as i32) as u32),
        );
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        self.w32(v3.wrapping_add(1456), 5_u32);
        self.w32(v3.wrapping_add(1472), 5_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        let _ = self.f_10006d10(this, v4, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            v6 = (self.r32(fp.wrapping_add(2)) as i32);
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v6.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                self.w32(v7.wrapping_add(708), 64_u32);
                self.w32(v7.wrapping_add(712), 1_u32);
            }
            let _ = self.f_10013c90(this);
            return 0_i32;
        } else {
            self.w32(v3.wrapping_add(1480), 0_u32);
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10011320` (276 bytes).
    pub(crate) fn f_10011320(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011320_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011320_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 6_u32);
        self.w32(v3.wrapping_add(1472), 6_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 0_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 0_u32);
        let _ = self.f_10006c80(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(408);
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
                    v7.wrapping_add((v8.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                    14_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(117_i32) as u32).wrapping_mul(4)),
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

    /// `sub_10011440` (275 bytes).
    pub(crate) fn f_10011440(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011440_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011440_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 6_u32);
        self.w32(v3.wrapping_add(1472), 6_u32);
        self.w32(fp.wrapping_add(2), this);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1476), 1_u32);
        let _ = self.f_10006d10(this, v4, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            && (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
            if (((self.r32(v5) as i32) == 1_i32)
                && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
            {
                v6 = 0_i32;
                v7 = v5.wrapping_add(528);
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
                    v5.wrapping_add((v6.wrapping_add(132_i32) as u32).wrapping_mul(4)),
                    24_u32,
                );
                self.w32(
                    v5.wrapping_add((v6.wrapping_add(134_i32) as u32).wrapping_mul(4)),
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

    /// `sub_10011560` (278 bytes).
    pub(crate) fn f_10011560(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011560_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011560_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 6_u32);
        self.w32(v3.wrapping_add(1472), 6_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        let _ = self.f_10006ec0(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
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
                    26_u32,
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

    /// `sub_10011680` (279 bytes).
    pub(crate) fn f_10011680(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011680_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011680_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 6_u32);
        self.w32(v3.wrapping_add(1472), 6_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 3_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 3_u32);
        let _ = self.f_10006d10(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(552);
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
                    v7.wrapping_add((v8.wrapping_add(138_i32) as u32).wrapping_mul(4)),
                    29_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(139_i32) as u32).wrapping_mul(4)),
                    3_u32,
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

    /// `sub_100117A0` (222 bytes).
    pub(crate) fn f_100117a0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100117a0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100117a0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        self.w32(
            fp.wrapping_add(2),
            ((self.r32((v2.wrapping_add(12_i32) as u32)) as i32) as u32),
        );
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        self.w32(v3.wrapping_add(1456), 6_u32);
        self.w32(v3.wrapping_add(1472), 6_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        let _ = self.f_10006ec0(this, v4, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            v6 = (self.r32(fp.wrapping_add(2)) as i32);
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v6.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                self.w32(v7.wrapping_add(708), 65_u32);
                self.w32(v7.wrapping_add(712), 1_u32);
            }
            let _ = self.f_10013c90(this);
            return 0_i32;
        } else {
            self.w32(v3.wrapping_add(1480), 0_u32);
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10011880` (276 bytes).
    pub(crate) fn f_10011880(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011880_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011880_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1460), 0_u32);
        self.w32(v3.wrapping_add(1476), 0_u32);
        self.w32(v3.wrapping_add(1456), 7_u32);
        self.w32(v3.wrapping_add(1472), 7_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        let _ = self.f_10006d10(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(408);
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
                    v7.wrapping_add((v8.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                    15_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(117_i32) as u32).wrapping_mul(4)),
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

    /// `sub_100119A0` (172 bytes).
    pub(crate) fn f_100119a0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 7_u32);
        self.w32(v3.wrapping_add(1472), 7_u32);
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

    /// `sub_10011A50` (278 bytes).
    pub(crate) fn f_10011a50(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011a50_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011a50_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 7_u32);
        self.w32(v3.wrapping_add(1472), 7_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        let _ = self.f_10006d10(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
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
                    27_u32,
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

    /// `sub_10011B70` (279 bytes).
    pub(crate) fn f_10011b70(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011b70_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011b70_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1460), 3_u32);
        self.w32(v3.wrapping_add(1476), 3_u32);
        self.w32(v3.wrapping_add(1456), 7_u32);
        self.w32(v3.wrapping_add(1472), 7_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        let _ = self.f_10006d10(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(552);
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
                    v7.wrapping_add((v8.wrapping_add(138_i32) as u32).wrapping_mul(4)),
                    29_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(139_i32) as u32).wrapping_mul(4)),
                    4_u32,
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

    /// `sub_10011C90` (222 bytes).
    pub(crate) fn f_10011c90(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011c90_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011c90_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        self.w32(
            fp.wrapping_add(2),
            ((self.r32((v2.wrapping_add(12_i32) as u32)) as i32) as u32),
        );
        v4 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        self.w32(v3.wrapping_add(1456), 7_u32);
        self.w32(v3.wrapping_add(1472), 7_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        let _ = self.f_10006d10(this, v4, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            v6 = (self.r32(fp.wrapping_add(2)) as i32);
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v6.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                self.w32(v7.wrapping_add(708), 66_u32);
                self.w32(v7.wrapping_add(712), 1_u32);
            }
            let _ = self.f_10013c90(this);
            return 0_i32;
        } else {
            self.w32(v3.wrapping_add(1480), 0_u32);
            self.w32(v3.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_10011D70` (203 bytes).
    pub(crate) fn f_10011d70(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 8_u32);
        self.w32(v2.wrapping_add(1472), 8_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1460), 0_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1476), 0_u32);
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
                16_u32,
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

    /// `sub_10011E40` (271 bytes).
    pub(crate) fn f_10011e40(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_10011e40_body(fp, this);
        self.leave(16);
        r
    }

    fn f_10011e40_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = self.r32((v2.wrapping_add(28_i32) as u32));
        v4 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(v3.wrapping_add(1456), 8_u32);
        self.w32(v3.wrapping_add(1472), 8_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 1_u32);
        let _ = self.f_10006b80(this, v5, fp.wrapping_add(0));
        if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v6 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v6) as i32) == 1_i32)
                && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
            {
                v7 = 0_i32;
                v8 = v6.wrapping_add(408);
                'l1: loop {
                    if (!((self.r32(v8) as i32) != 0)) {
                        break 'l1;
                    }
                    v7 = v7.wrapping_add(1);
                    v8 = v8.wrapping_add(4);
                    if !(v7 < 15_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                    17_u32,
                );
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(117_i32) as u32).wrapping_mul(4)),
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

    /// `sub_10011F50` (176 bytes).
    pub(crate) fn f_10011f50(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1460), 2_u32);
        self.w32(v3.wrapping_add(1476), 2_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1456), 8_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1472), 8_u32);
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
                45_u32,
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

    /// `sub_10012000` (192 bytes).
    pub(crate) fn f_10012000(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 8_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1472), 8_u32);
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
                30_u32,
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

    /// `sub_100120C0` (278 bytes).
    pub(crate) fn f_100120c0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100120c0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100120c0_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 8_u32);
        self.w32(v3.wrapping_add(1472), 8_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1476), 4_u32);
        let _ = self.f_10006e30(this, v5, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v7 = (v4.wrapping_add(716_i32.wrapping_mul(v5)) as u32);
            if (((self.r32(v7) as i32) == 1_i32)
                && ((self.r32(v7.wrapping_add(372)) as i32) != 99_i32))
            {
                v8 = 0_i32;
                v9 = v7.wrapping_add(408);
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
                    v7.wrapping_add((v8.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                    18_u32,
                );
                self.w32(
                    v7.wrapping_add((v8.wrapping_add(117_i32) as u32).wrapping_mul(4)),
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

    /// `sub_100121E0` (203 bytes).
    pub(crate) fn f_100121e0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1460), 0_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1476), 0_u32);
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
                19_u32,
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

    /// `sub_100122B0` (267 bytes).
    pub(crate) fn f_100122b0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_100122b0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_100122b0_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 9_u32);
        self.w32(v3.wrapping_add(1472), 9_u32);
        self.w32(fp.wrapping_add(2), this);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v4 as u32));
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1476), 1_u32);
        let _ = self.f_10006bf0(this, v4, fp.wrapping_add(0), fp.wrapping_add(1));
        if ((((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32)
            || (((self.r8(fp.wrapping_add(1)) as i8) as i32) == 1_i32))
        {
            self.w32(v3.wrapping_add(1480), 1_u32);
            v6 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
            if (((self.r32(v6) as i32) == 1_i32)
                && ((self.r32(v6.wrapping_add(372)) as i32) != 99_i32))
            {
                v7 = 0_i32;
                v8 = v6.wrapping_add(528);
                'l1: loop {
                    if (!((self.r32(v8) as i32) != 0)) {
                        break 'l1;
                    }
                    v7 = v7.wrapping_add(1);
                    v8 = v8.wrapping_add(4);
                    if !(v7 < 2_i32) {
                        break 'l1;
                    }
                }
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(132_i32) as u32).wrapping_mul(4)),
                    25_u32,
                );
                self.w32(
                    v6.wrapping_add((v7.wrapping_add(134_i32) as u32).wrapping_mul(4)),
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
}
