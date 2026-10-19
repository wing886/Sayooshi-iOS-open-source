//! Functions 0x1000A0A0..=0x1000F840, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_1000A0A0` (407 bytes).
    pub(crate) fn f_1000a0a0(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let fp = self.enter(80);
        let r = self.f_1000a0a0_body(fp, this, a2, a3, a4);
        self.leave(80);
        r
    }

    fn f_1000a0a0_body(
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
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v5 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32))
                        as i32);
                    let _ = self.f_100079b0(this, 0_i32, a2, 0_i32, fp.wrapping_add(32));
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
                        67 => 2,
                        71 => 3,
                        72 => 4,
                        74 => 5,
                        _ => 6,
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
                        5_u32,
                    );
                    self.w32(
                        (v5.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1872_i32) as u32),
                        ((self.r32(fp.wrapping_add(16).wrapping_add(8)) as i32) as u32),
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
                    v6 = cdiv_i32(
                        30_i32.wrapping_mul((self.r32(fp.wrapping_add(44)) as i32)),
                        100_i32,
                    );
                    bb = 7;
                }
                3 => {
                    v7 = 5_i32.wrapping_mul((self.r32(fp.wrapping_add(44)) as i32));
                    bb = 9;
                }
                4 => {
                    v6 = cdiv_i32(
                        50_i32.wrapping_mul((self.r32(fp.wrapping_add(44)) as i32)),
                        100_i32,
                    );
                    bb = 7;
                }
                5 => {
                    v7 = 7_i32.wrapping_mul((self.r32(fp.wrapping_add(44)) as i32));
                    bb = 9;
                }
                6 => {
                    bb = 1;
                }
                7 => {
                    self.w32(fp.wrapping_add(44), (v6 as u32));
                    bb = 1;
                }
                8 => {
                    bb = 3;
                }
                9 => {
                    v6 = cdiv_i32(10_i32.wrapping_mul(v7), 100_i32);
                    bb = 7;
                }
                10 => {
                    bb = 4;
                }
                11 => {
                    bb = 5;
                }
                12 => {
                    bb = 6;
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

    /// `sub_1000A260` (351 bytes).
    pub(crate) fn f_1000a260(
        &mut self,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let fp = self.enter(80);
        let r = self.f_1000a260_body(fp, this, a2, a3, a4);
        self.leave(80);
        r
    }

    fn f_1000a260_body(
        &mut self,
        fp: u32,
        mut this: u32,
        mut a2: i32,
        mut a3: i32,
        mut a4: i32,
    ) -> i32 {
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v5 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32))
                        as i32);
                    let _ = self.f_100079b0(this, 0_i32, a2, 0_i32, fp.wrapping_add(32));
                    let _ = self.f_100079b0(this, 0_i32, a2, 0_i32, fp.wrapping_add(56));
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
                    bb = if (a4 == 46_i32) { 1 } else { 2 };
                }
                1 => {
                    v6 = cdiv_i32(
                        200_i32.wrapping_mul((self.r32(fp.wrapping_add(44)) as i32)),
                        100_i32,
                    );
                    bb = 3;
                }
                2 => {
                    bb = if (a4 == 47_i32) { 5 } else { 6 };
                }
                3 => {
                    self.w32(fp.wrapping_add(44), (v6 as u32));
                    bb = 6;
                }
                4 => {
                    bb = 2;
                }
                5 => {
                    v6 = cdiv_i32(
                        200_i32.wrapping_mul((self.r32(fp.wrapping_add(44)) as i32)),
                        100_i32,
                    );
                    bb = 3;
                }
                6 => {
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
                        (a3 as u32),
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
                        6_u32,
                    );
                    self.w32(
                        (v5.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(1872_i32) as u32),
                        ((self.r32(fp.wrapping_add(16).wrapping_add(12)) as i32) as u32),
                    );
                    {
                        let a0 = (v5
                            .wrapping_add(4_i32.wrapping_mul({
                                let t1 = (self.r32((v5.wrapping_add(2072_i32) as u32)) as i32);
                                self.w32(
                                    (v5.wrapping_add(2072_i32) as u32),
                                    (t1.wrapping_add(1) as u32),
                                );
                                t1
                            }))
                            .wrapping_add(1972_i32) as u32);
                        let a1 = 0_u32;
                        self.w32(a0, a1)
                    };
                    return 0_i32;
                }
                7 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1000A3C0` (135 bytes).
    pub(crate) fn f_1000a3c0(&mut self, mut this: u32, mut a2: i32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_1000a3c0_body(fp, this, a2);
        self.leave(16);
        r
    }

    fn f_1000a3c0_body(&mut self, fp: u32, mut this: u32, mut a2: i32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        self.w32(fp.wrapping_add(0), (a2 as u32));
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = (self.r32(fp.wrapping_add(0)) as i32);
        v4 = self.r32((v2.wrapping_add(28_i32) as u32));
        's1: {
            'b1_2: {
                'b1_1: {
                    'b1_0: {
                        match (self.r32(
                            ((self.r32((v2.wrapping_add(12_i32) as u32)) as i32)
                                .wrapping_add(
                                    716_i32.wrapping_mul((self.r32(fp.wrapping_add(0)) as i32)),
                                )
                                .wrapping_add(12_i32) as u32),
                        ) as i32)
                        {
                            -1_i32 | 0_i32 | 1_i32 | 2_i32 | 3_i32 | 5_i32 | 6_i32 | 7_i32
                            | 8_i32 | 9_i32 => break 'b1_0,
                            4_i32 => break 'b1_1,
                            _ => break 'b1_2,
                        }
                    }
                    self.w32(fp.wrapping_add(0), 20_u32);
                    break 's1;
                }
                self.w32(fp.wrapping_add(0), 40_u32);
                break 's1;
            }
            break 's1;
        }
        let _ = self.f_10008590(this, 0_i32, v3, fp.wrapping_add(0));
        v5 = ((crem_i32(self.rand(), 100_i32) < (self.r32(fp.wrapping_add(0)) as i32)) as i32);
        if ((self.r32(v4.wrapping_add(324)) as i32) >= 10_i32) {
            v5 = 0_i32;
        }
        self.w32(v4.wrapping_add(2180), 1_u32);
        self.w32(v4.wrapping_add(2184), (v5 as u32));
        return 0_i32;
    }

    /// `sub_1000A480` (332 bytes).
    pub(crate) fn f_1000a480(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut result: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v1 = (self.r32(self.r32(this)) as i32);
                    v2 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32);
                    v3 = self.r32((v1.wrapping_add(28_i32) as u32));
                    let _ = self.f_10006320(this);
                    v4 = (self.r32(v3.wrapping_add(1388)) as i32);
                    self.w32(v3.wrapping_add(1400), 2_u32);
                    v5 = (v2.wrapping_add(716_i32.wrapping_mul(v4)) as u32);
                    bb = if (((self.r32(v5) as i32) == 1_i32)
                        && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
                    {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    v6 = 0_i32;
                    v7 = v5.wrapping_add(592);
                    bb = 3;
                }
                2 => {
                    bb = 8;
                }
                3 => {
                    bb = if ((self.r32(v7) as i32) != 35_i32) {
                        4
                    } else {
                        5
                    };
                }
                4 => {
                    v6 = v6.wrapping_add(1);
                    v7 = v7.wrapping_add(4);
                    bb = if (v6 >= 3_i32) { 6 } else { 7 };
                }
                5 => {
                    self.w32(v3.wrapping_add(1400), 9_u32);
                    bb = 2;
                }
                6 => {
                    bb = 8;
                }
                7 => {
                    bb = 3;
                }
                8 => {
                    bb = if ((self.r32(v3.wrapping_add(1400)) as i32) == 2_i32) {
                        10
                    } else {
                        11
                    };
                }
                9 => {
                    bb = 7;
                }
                10 => {
                    bb = if (((self.r32(v5) as i32) == 1_i32)
                        && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
                    {
                        12
                    } else {
                        13
                    };
                }
                11 => {
                    bb = 28;
                }
                12 => {
                    v8 = 0_i32;
                    v9 = v5.wrapping_add(668);
                    bb = 14;
                }
                13 => {
                    bb = 19;
                }
                14 => {
                    bb = if ((self.r32(v9) as i32) != 48_i32) {
                        15
                    } else {
                        16
                    };
                }
                15 => {
                    v8 = v8.wrapping_add(1);
                    v9 = v9.wrapping_add(4);
                    bb = if (v8 >= 2_i32) { 17 } else { 18 };
                }
                16 => {
                    self.w32(v3.wrapping_add(1400), 9_u32);
                    bb = 13;
                }
                17 => {
                    bb = 19;
                }
                18 => {
                    bb = 14;
                }
                19 => {
                    bb = if ((((self.r32(v3.wrapping_add(1400)) as i32) == 2_i32)
                        && ((self.r32(v5) as i32) == 1_i32))
                        && ((self.r32(v5.wrapping_add(372)) as i32) != 99_i32))
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
                    v10 = 0_i32;
                    v11 = v5.wrapping_add(708);
                    bb = 23;
                }
                22 => {
                    bb = 11;
                }
                23 => {
                    bb = if (!((self.r32(v11) as i32) != 0)) {
                        24
                    } else {
                        25
                    };
                }
                24 => {
                    v10 = v10.wrapping_add(1);
                    v11 = v11.wrapping_add(4);
                    bb = if (v10 >= 1_i32) { 26 } else { 27 };
                }
                25 => {
                    self.w32(v3.wrapping_add(1400), 9_u32);
                    bb = 22;
                }
                26 => {
                    bb = 28;
                }
                27 => {
                    bb = 23;
                }
                28 => {
                    bb = if ((self.r32(v3.wrapping_add(1400)) as i32) != 2_i32) {
                        30
                    } else {
                        31
                    };
                }
                29 => {
                    bb = 27;
                }
                30 => {
                    return 0_i32;
                }
                31 => {
                    let t1 = (self.r32(v3.wrapping_add(1040)) as i32);
                    bb = match t1 {
                        1_i32 => 34,
                        2_i32 => 35,
                        3_i32 => 36,
                        4_i32 => 37,
                        5_i32 => 38,
                        6_i32 => 39,
                        7_i32 => 40,
                        _ => 41,
                    };
                }
                32 => {
                    bb = 31;
                }
                33 => {
                    return result;
                }
                34 => {
                    self.w32(v3.wrapping_add(1400), 3_u32);
                    result = 0_i32;
                    bb = 33;
                }
                35 => {
                    self.w32(v3.wrapping_add(1400), 4_u32);
                    result = 0_i32;
                    bb = 33;
                }
                36 => {
                    self.w32(v3.wrapping_add(1400), 5_u32);
                    result = 0_i32;
                    bb = 33;
                }
                37 => {
                    self.w32(v3.wrapping_add(1400), 6_u32);
                    result = 0_i32;
                    bb = 33;
                }
                38 => {
                    self.w32(v3.wrapping_add(1400), 7_u32);
                    result = 0_i32;
                    bb = 33;
                }
                39 => {
                    bb = 40;
                }
                40 => {
                    self.w32(v3.wrapping_add(1400), 8_u32);
                    return 0_i32;
                }
                41 => {
                    return 0_i32;
                }
                42 => {
                    bb = 35;
                }
                43 => {
                    bb = 36;
                }
                44 => {
                    bb = 37;
                }
                45 => {
                    bb = 38;
                }
                46 => {
                    bb = 39;
                }
                47 => {
                    bb = 41;
                }
                48 => {
                    bb = 33;
                }
                49 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1000A5F0` (111 bytes).
    pub(crate) fn f_1000a5f0(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        v3 = (self.r32((v2.wrapping_add(1404_i32) as u32)) as i32);
        if (v3 != 0) {
            if (v3 == 1_i32) {
                let _ = self.f_1000c630(this, 0_i8, 0_i8);
                self.w32((v2.wrapping_add(1404_i32) as u32), 2_u32);
                return 0_i32;
            } else {
                if (v3 == 2_i32) {
                    self.w32((v2.wrapping_add(1048_i32) as u32), 1_u32);
                    self.w32((v2.wrapping_add(1404_i32) as u32), 99_u32);
                }
                return 0_i32;
            }
        } else {
            let _ = self.f_1000a660(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000A660` (130 bytes).
    pub(crate) fn f_1000a660(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        v1 = 0_i32;
        v2 = (self.r32(self.r32(this)) as i32);
        v3 = (self.r32((v2.wrapping_add(12_i32) as u32)) as i32);
        v4 = (v3.wrapping_add(716_i32.wrapping_mul(
            (self.r32(
                ((self.r32((v2.wrapping_add(28_i32) as u32)) as i32).wrapping_add(1388_i32) as u32),
            ) as i32),
        )) as u32);
        if (((self.r32(v4) as i32) == 1_i32) && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
        {
            v5 = v4.wrapping_add(628);
            v6 = 2_i32;
            'l1: loop {
                if ((self.r32(v5) as i32) != 0) {
                    v1 = (self.r32(v5) as i32);
                }
                v5 = v5.wrapping_add(4);
                v6 = v6.wrapping_sub(1);
                if !(v6 != 0) {
                    break 'l1;
                }
            }
        }
        if (((self.r32((v3.wrapping_add(2148_i32) as u32)) as i32) == 1_i32)
            && ((self.r32((v3.wrapping_add(2520_i32) as u32)) as i32) != 99_i32))
        {
            if (v1 != 0) {
                let _ = self.f_1000b5e0(this);
            } else {
                let _ = self.f_1000a6f0(this);
            }
            return 0_i32;
        } else {
            let _ = self.f_1000a6f0(this);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000A6F0` (3789 bytes).
    pub(crate) fn f_1000a6f0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(1952);
        let r = self.f_1000a6f0_body(fp, this);
        self.leave(1952);
        r
    }

    fn f_1000a6f0_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        let mut v25: u32 = 0;
        let mut v26: u32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: u32 = 0;
        let mut v31: i32 = 0;
        let mut v32: i32 = 0;
        let mut v33: u32 = 0;
        let mut v34: u32 = 0;
        let mut v35: i32 = 0;
        let mut v36: i32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: u32 = 0;
        let mut v40: i32 = 0;
        let mut v41: i32 = 0;
        let mut v42: bool = false;
        let mut v43: i32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: u32 = 0;
        let mut v48: i32 = 0;
        let mut v49: bool = false;
        let mut v50: u32 = 0;
        let mut v51: u32 = 0;
        let mut v52: u32 = 0;
        let mut v53: i32 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: u32 = 0;
        let mut v57: i32 = 0;
        let mut v58: i32 = 0;
        let mut v59: i32 = 0;
        let mut v60: u32 = 0;
        let mut v61: i32 = 0;
        let mut v62: u32 = 0;
        let mut i: u32 = 0;
        let mut v64: i32 = 0;
        let mut v65: i32 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: u32 = 0;
        let mut v69: u32 = 0;
        let mut v70: i32 = 0;
        let mut v71: i8 = 0;
        let mut v72: i32 = 0;
        let mut v73: i32 = 0;
        let mut v74: i32 = 0;
        let mut v75: i32 = 0;
        let mut v76: i32 = 0;
        let mut v77: i32 = 0;
        let mut v78: u32 = 0;
        let mut v80: u32 = 0;
        let mut v81: i32 = 0;
        let mut v82: i32 = 0;
        let mut v83: u32 = 0;
        let mut v84: i32 = 0;
        let mut v85: i32 = 0;
        let mut v86: u32 = 0;
        let mut v87: i64 = 0;
        let mut v88: i32 = 0;
        let mut v89: i32 = 0;
        let mut v90: i32 = 0;
        let mut v91: u32 = 0;
        let mut v92: i32 = 0;
        let mut v93: u32 = 0;
        let mut v94: i32 = 0;
        let mut v95: i32 = 0;
        let mut v96: u32 = 0;
        let mut v97: i32 = 0;
        let mut v98: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(this);
                    self.w32(fp.wrapping_add(96), this);
                    v3 = ((self.r32(v2) as i32) as u32);
                    v4 = (self.r32(v3.wrapping_add(20)) as i32);
                    v5 = ((self.r32(v3.wrapping_add(28)) as i32) as u32);
                    v6 = (self.r32(v3.wrapping_add(12)) as i32);
                    self.w32(fp.wrapping_add(88), (v4 as u32));
                    self.w32(fp.wrapping_add(24), (v6 as u32));
                    self.w32(fp.wrapping_add(12), (v4.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(84),
                        ((self.r32(v5.wrapping_add(1388)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(104), v5);
                    v98 = (self.r32(v5.wrapping_add(1384)) as i32);
                    v7 = (v6
                        .wrapping_add(716_i32.wrapping_mul((self.r32(fp.wrapping_add(84)) as i32)))
                        as u32);
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(v7.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(v7.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(108),
                        ((self.r32(v7.wrapping_add(388)) as i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(116),
                        (fp.wrapping_add(136) as i32),
                        v98,
                        (self.r32(fp.wrapping_add(84)) as i32),
                        0_i32,
                    );
                    bb = if (((self.r32(fp.wrapping_add(116)) as i32) == 17_i32)
                        || ((self.r32(fp.wrapping_add(116)) as i32) == 18_i32))
                    {
                        1
                    } else {
                        2
                    };
                }
                1 => {
                    let _ = self.f_1000c630(this, 1_i8, 1_i8);
                    bb = if ((self.r32(v5.wrapping_add(2072)) as i32) > 0_i32) {
                        3
                    } else {
                        4
                    };
                }
                2 => {
                    self.w32(fp.wrapping_add(72), self.r32(fp.wrapping_add(124)));
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(120)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(128)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(112),
                        ((self.r32(fp.wrapping_add(132)) as i32) as u32),
                    );
                    let _ = self.f_10006940(
                        this,
                        fp.wrapping_add(92),
                        (self.r32(fp.wrapping_add(120)) as i32),
                        (self.r32(fp.wrapping_add(124)) as i32),
                        (self.r32(fp.wrapping_add(128)) as i32),
                        (self.r32(fp.wrapping_add(132)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(self.r32(fp.wrapping_add(92))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(76),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(56),
                        self.r32(fp.wrapping_add(92)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    v8 = (v6.wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(100), (v6.wrapping_add(3600_i32) as u32));
                    self.w32(fp.wrapping_add(44), (v6.wrapping_add(3600_i32) as u32));
                    self.w32(fp.wrapping_add(4), fp.wrapping_add(248));
                    self.w32(fp.wrapping_add(48), fp.wrapping_add(148));
                    bb = 6;
                }
                3 => {
                    return 0_i32;
                }
                4 => {
                    let _ = self.f_10006420(this);
                    bb = 2;
                }
                5 => {
                    bb = 4;
                }
                6 => {
                    bb = if (((self.r32(v8) as i32) == 1_i32)
                        && ((self.r32(v8.wrapping_add(116)) as i32) != 99_i32))
                    {
                        9
                    } else {
                        10
                    };
                }
                7 => {
                    bb = if ((self.r32(fp.wrapping_add(8)) as i32) < 20_i32) {
                        6
                    } else {
                        8
                    };
                }
                8 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) <= 0_i32) {
                        35
                    } else {
                        36
                    };
                }
                9 => {
                    v9 = (self.r32(fp.wrapping_add(80)) as i32)
                        .wrapping_add((self.r32(v8.wrapping_add(124)) as i32));
                    v10 = (self.r32(fp.wrapping_add(76)) as i32)
                        .wrapping_add((self.r32(v8.wrapping_add(128)) as i32));
                    v11 = v9.wrapping_add((self.r32(fp.wrapping_add(32)) as i32));
                    self.w32(
                        fp.wrapping_add(60),
                        (v10.wrapping_add((self.r32(fp.wrapping_add(36)) as i32)) as u32),
                    );
                    v12 = self.r32(fp.wrapping_add(56));
                    self.w8(fp.wrapping_add(54), 0_u8);
                    self.w8(fp.wrapping_add(55), 0_u8);
                    self.w32(fp.wrapping_add(28), (v10 as u32));
                    bb = if (v10 < v10.wrapping_add((self.r32(fp.wrapping_add(36)) as i32))) {
                        11
                    } else {
                        12
                    };
                }
                10 => {
                    v8 = v8.wrapping_add(156);
                    self.w32(fp.wrapping_add(44), v8);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 7;
                }
                11 => {
                    v13 = (20_i32.wrapping_mul(v10) as u32);
                    bb = 13;
                }
                12 => {
                    bb = 10;
                }
                13 => {
                    v14 = (v9 as u32);
                    bb = if (v9 < v11) { 16 } else { 17 };
                }
                14 => {
                    bb = if ((self.r32(fp.wrapping_add(28)) as i32)
                        < (self.r32(fp.wrapping_add(60)) as i32))
                    {
                        13
                    } else {
                        15
                    };
                }
                15 => {
                    bb = if (((self.r8(fp.wrapping_add(54)) as i8) as i32) == 1_i32) {
                        31
                    } else {
                        32
                    };
                }
                16 => {
                    bb = 18;
                }
                17 => {
                    v13 = v13.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(fp.wrapping_add(28)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 14;
                }
                18 => {
                    bb = if ((self.r32(v12) as i32) != 0) {
                        21
                    } else {
                        22
                    };
                }
                19 => {
                    bb = if ((v14 as i32) < v9.wrapping_add((self.r32(fp.wrapping_add(32)) as i32)))
                    {
                        18
                    } else {
                        20
                    };
                }
                20 => {
                    bb = 17;
                }
                21 => {
                    bb = if ((v13 < 400_u32) && (v14 < 20_u32)) {
                        23
                    } else {
                        24
                    };
                }
                22 => {
                    v12 = v12.wrapping_add(4);
                    v14 = v14.wrapping_add(1);
                    v11 = v9.wrapping_add((self.r32(fp.wrapping_add(32)) as i32));
                    bb = 19;
                }
                23 => {
                    v15 = (4_u32.wrapping_mul(v13.wrapping_add(v14)) as i32);
                    bb = if ((self.r32((v15.wrapping_add(v4) as u32)) as i32)
                        != (1_i32).wrapping_neg())
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
                    v16 = (self.r32((v15.wrapping_add(v4).wrapping_add(3200_i32) as u32)) as i32);
                    bb = if ((v16 == (1_i32).wrapping_neg())
                        || (v16 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        27
                    } else {
                        28
                    };
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    self.w8(fp.wrapping_add(54), 1_u8);
                    bb = if ((v14 == ((self.r32(fp.wrapping_add(40)) as i32) as u32))
                        && ((self.r32(fp.wrapping_add(28)) as i32)
                            == (self.r32(fp.wrapping_add(16)) as i32)))
                    {
                        29
                    } else {
                        30
                    };
                }
                28 => {
                    bb = 26;
                }
                29 => {
                    self.w8(fp.wrapping_add(55), 1_u8);
                    bb = 30;
                }
                30 => {
                    bb = 28;
                }
                31 => {
                    self.w32(
                        self.r32(fp.wrapping_add(48)),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        self.r32(fp.wrapping_add(48)).wrapping_add(4),
                    );
                    bb = 32;
                }
                32 => {
                    bb = if (((self.r8(fp.wrapping_add(55)) as i8) as i32) == 1_i32) {
                        33
                    } else {
                        34
                    };
                }
                33 => {
                    {
                        let a0 = {
                            let t1 = self.r32(fp.wrapping_add(4));
                            self.w32(fp.wrapping_add(4), t1.wrapping_add(4));
                            t1
                        };
                        let a1 = ((self.r32(fp.wrapping_add(8)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 34;
                }
                34 => {
                    v8 = self.r32(fp.wrapping_add(44));
                    bb = 12;
                }
                35 => {
                    v80 = self.r32(fp.wrapping_add(100));
                    v81 = 0_i32;
                    v82 = 0_i32;
                    v83 = fp.wrapping_add(148);
                    bb = 37;
                }
                36 => {
                    v17 = 0_i32;
                    v18 = 9999_i32;
                    self.w32(fp.wrapping_add(28), 0_u32);
                    v19 = fp.wrapping_add(148);
                    bb = 62;
                }
                37 => {
                    bb = if (((self.r32(v80) as i32) == 1_i32)
                        && ((self.r32(v80.wrapping_add(116)) as i32) != 99_i32))
                    {
                        40
                    } else {
                        41
                    };
                }
                38 => {
                    bb = if (v82 < 20_i32) { 37 } else { 39 };
                }
                39 => {
                    self.w32(fp.wrapping_add(0), (v81 as u32));
                    v84 = 0_i32;
                    v85 = 9999_i32;
                    self.w32(fp.wrapping_add(28), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        42
                    } else {
                        43
                    };
                }
                40 => {
                    self.w32(v83, (v82 as u32));
                    v81 = v81.wrapping_add(1);
                    v83 = v83.wrapping_add(4);
                    bb = 41;
                }
                41 => {
                    v80 = v80.wrapping_add(156);
                    v82 = v82.wrapping_add(1);
                    bb = 38;
                }
                42 => {
                    v86 = fp.wrapping_add(148);
                    bb = 44;
                }
                43 => {
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(24)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(148).wrapping_add(
                                                ((self.r32(fp.wrapping_add(28)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    v89 = 0_i32;
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(24)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul(
                                        (self.r32(
                                            fp.wrapping_add(148).wrapping_add(
                                                ((self.r32(fp.wrapping_add(28)) as i32) as u32)
                                                    .wrapping_mul(4),
                                            ),
                                        ) as i32),
                                    ),
                                )
                                .wrapping_add(3728_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v90 = (self
                        .r32(((self.r32(fp.wrapping_add(88)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    bb = if (v90 > 0_i32) { 49 } else { 50 };
                }
                44 => {
                    v87 = ((self.r32(
                        ((self.r32(fp.wrapping_add(24)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v86) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32))
                        as i64);
                    v88 = ((((((v87 as u64) >> 32) as u32) as i64) ^ v87)
                        .wrapping_sub(((((v87 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(24)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v86) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v88 < v85) { 47 } else { 48 };
                }
                45 => {
                    bb = if (v84 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                46 => {
                    bb = 43;
                }
                47 => {
                    v85 = v88;
                    self.w32(fp.wrapping_add(28), (v84 as u32));
                    bb = 48;
                }
                48 => {
                    v84 = v84.wrapping_add(1);
                    v86 = v86.wrapping_add(4);
                    bb = 45;
                }
                49 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(348),
                        self.r32(fp.wrapping_add(12)),
                        (4_i32.wrapping_mul(v90) as u32),
                    );
                    v89 = v90;
                    self.w32(fp.wrapping_add(0), (v90 as u32));
                    bb = 50;
                }
                50 => {
                    self.w32(fp.wrapping_add(20), 9999_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if (v89 > 0_i32) { 51 } else { 52 };
                }
                51 => {
                    self.w32(fp.wrapping_add(84), fp.wrapping_add(348));
                    bb = 53;
                }
                52 => {
                    v96 = self.r32(fp.wrapping_add(104));
                    v97 = (self.r32(fp.wrapping_add(348).wrapping_add(
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                    )) as i32);
                    self.w32(self.r32(fp.wrapping_add(104)).wrapping_add(1540), 2_u32);
                    self.w32(v96.wrapping_add(1552), (v97 as u32));
                    self.w32(v96.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
                    self.w32(v96.wrapping_add(1544), (crem_i32(v97, 20_i32) as u32));
                    self.w32(v96.wrapping_add(1548), (cdiv_i32(v97, 20_i32) as u32));
                    return 0_i32;
                }
                53 => {
                    v91 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(84))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v92 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(84))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v93 = ((self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_add(v92)
                        .wrapping_abs() as u32);
                    v94 = (v91.wrapping_sub(v93) as i32).wrapping_abs();
                    v95 = (((self.r32(fp.wrapping_add(16)) as i32)
                        .wrapping_add(v92)
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_sub(crem_i32(
                                    (self.r32(self.r32(fp.wrapping_add(84))) as i32),
                                    20_i32,
                                ))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v93.wrapping_add(v91)))
                        as i32);
                    bb = if (v95 >= (self.r32(fp.wrapping_add(20)) as i32)) {
                        56
                    } else {
                        58
                    };
                }
                54 => {
                    bb = if v42 { 53 } else { 55 };
                }
                55 => {
                    bb = 52;
                }
                56 => {
                    bb = if ((v95 == (self.r32(fp.wrapping_add(20)) as i32))
                        && (v94 < (self.r32(fp.wrapping_add(12)) as i32)))
                    {
                        59
                    } else {
                        60
                    };
                }
                57 => {
                    v42 = ({
                        let t2 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t2 as u32));
                        t2
                    } < (self.r32(fp.wrapping_add(0)) as i32));
                    self.w32(
                        fp.wrapping_add(84),
                        self.r32(fp.wrapping_add(84)).wrapping_add(4),
                    );
                    bb = 54;
                }
                58 => {
                    self.w32(fp.wrapping_add(20), (v95 as u32));
                    self.w32(fp.wrapping_add(12), (v94 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 57;
                }
                59 => {
                    self.w32(fp.wrapping_add(12), (v94 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 60;
                }
                60 => {
                    bb = 57;
                }
                61 => {
                    bb = 36;
                }
                62 => {
                    v20 = ((self.r32(
                        ((self.r32(fp.wrapping_add(24)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v19) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32))
                        as i64);
                    v21 = ((((((v20 as u64) >> 32) as u32) as i64) ^ v20)
                        .wrapping_sub(((((v20 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(24)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v19) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v21 < v18) { 65 } else { 66 };
                }
                63 => {
                    bb = if (v17 < (self.r32(fp.wrapping_add(0)) as i32)) {
                        62
                    } else {
                        64
                    };
                }
                64 => {
                    v22 = self.r32(fp.wrapping_add(56));
                    self.w32(
                        fp.wrapping_add(100),
                        ((self.r32(fp.wrapping_add(148).wrapping_add(
                            ((self.r32(fp.wrapping_add(28)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(24)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(100)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(24)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(100)) as i32)),
                                )
                                .wrapping_add(3728_i32) as u32),
                        ) as i32) as u32),
                    );
                    v23 = (self.r32(fp.wrapping_add(76)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(44)) as i32));
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(48)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(80)) as i32))
                            .wrapping_add((self.r32(fp.wrapping_add(32)) as i32))
                            as u32),
                    );
                    v24 = (self.r32(fp.wrapping_add(76)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_add((self.r32(fp.wrapping_add(36)) as i32));
                    bb = if ((self.r32(fp.wrapping_add(76)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(44)) as i32))
                        < v24)
                    {
                        67
                    } else {
                        68
                    };
                }
                65 => {
                    v18 = v21;
                    self.w32(fp.wrapping_add(28), (v17 as u32));
                    bb = 66;
                }
                66 => {
                    v17 = v17.wrapping_add(1);
                    v19 = v19.wrapping_add(4);
                    bb = 63;
                }
                67 => {
                    v25 = (20_i32.wrapping_mul(v23) as u32);
                    self.w32(fp.wrapping_add(24), (v24.wrapping_sub(v23) as u32));
                    bb = 69;
                }
                68 => {
                    bb = if ((((((self.r32(fp.wrapping_add(72)) < 4_u32)
                        || (self.r32(fp.wrapping_add(72)) == 5_u32))
                        || (self.r32(fp.wrapping_add(72)) == 6_u32))
                        || (self.r32(fp.wrapping_add(72)) == 7_u32))
                        || (self.r32(fp.wrapping_add(72)) == 8_u32))
                        || (self.r32(fp.wrapping_add(72)) == 9_u32))
                    {
                        85
                    } else {
                        87
                    };
                }
                69 => {
                    v26 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32))
                        as u32);
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(80)) as i32))
                        < (self.r32(fp.wrapping_add(4)) as i32))
                    {
                        72
                    } else {
                        73
                    };
                }
                70 => {
                    bb = if ((self.r32(fp.wrapping_add(24)) as i32) != 0) {
                        69
                    } else {
                        71
                    };
                }
                71 => {
                    bb = 68;
                }
                72 => {
                    v27 = fp.wrapping_add(348).wrapping_add(
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                    );
                    bb = 74;
                }
                73 => {
                    v25 = v25.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 70;
                }
                74 => {
                    bb = if ((self.r32(v22) as i32) != 0) {
                        77
                    } else {
                        78
                    };
                }
                75 => {
                    bb = if ((v26 as i32) < (self.r32(fp.wrapping_add(4)) as i32)) {
                        74
                    } else {
                        76
                    };
                }
                76 => {
                    bb = 73;
                }
                77 => {
                    bb = if ((v25 < 400_u32) && (v26 < 20_u32)) {
                        79
                    } else {
                        80
                    };
                }
                78 => {
                    v22 = v22.wrapping_add(4);
                    v26 = v26.wrapping_add(1);
                    bb = 75;
                }
                79 => {
                    v28 = v25.wrapping_add(v26);
                    bb = if ((self.r32(
                        ((self.r32(fp.wrapping_add(88)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v25.wrapping_add(v26))),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        81
                    } else {
                        82
                    };
                }
                80 => {
                    bb = 78;
                }
                81 => {
                    v29 = (self.r32(
                        ((self.r32(fp.wrapping_add(88)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v28))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v29 == (1_i32).wrapping_neg())
                        || (v29 == (self.r32(fp.wrapping_add(84)) as i32)))
                    {
                        83
                    } else {
                        84
                    };
                }
                82 => {
                    bb = 80;
                }
                83 => {
                    self.w32(
                        {
                            let t3 = v27;
                            v27 = t3.wrapping_add(4);
                            t3
                        },
                        v28,
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 84;
                }
                84 => {
                    bb = 82;
                }
                85 => {
                    self.w32(fp.wrapping_add(20), 9999_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        88
                    } else {
                        89
                    };
                }
                86 => {
                    self.w32(fp.wrapping_add(28), (v36 as u32));
                    bb = 100;
                }
                87 => {
                    bb = if (self.r32(fp.wrapping_add(72)) != 4_u32) {
                        98
                    } else {
                        99
                    };
                }
                88 => {
                    self.w32(fp.wrapping_add(24), fp.wrapping_add(348));
                    bb = 90;
                }
                89 => {
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(348).wrapping_add(
                            ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        (crem_i32((self.r32(fp.wrapping_add(24)) as i32), 20_i32) as u32),
                    );
                    v36 = cdiv_i32((self.r32(fp.wrapping_add(24)) as i32), 20_i32);
                    bb = 86;
                }
                90 => {
                    v37 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v38 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v39 = (v38
                        .wrapping_add((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs() as u32);
                    v40 = (v37.wrapping_sub(v39) as i32).wrapping_abs();
                    v41 = ((v38
                        .wrapping_add((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_sub(crem_i32(
                                    (self.r32(self.r32(fp.wrapping_add(24))) as i32),
                                    20_i32,
                                ))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v39.wrapping_add(v37)))
                        as i32);
                    bb = if (v41 >= (self.r32(fp.wrapping_add(20)) as i32)) {
                        93
                    } else {
                        95
                    };
                }
                91 => {
                    bb = if v42 { 90 } else { 92 };
                }
                92 => {
                    bb = 89;
                }
                93 => {
                    bb = if ((v41 == (self.r32(fp.wrapping_add(20)) as i32))
                        && (v40 < (self.r32(fp.wrapping_add(12)) as i32)))
                    {
                        96
                    } else {
                        97
                    };
                }
                94 => {
                    v42 = ({
                        let t4 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t4 as u32));
                        t4
                    } < (self.r32(fp.wrapping_add(0)) as i32));
                    self.w32(
                        fp.wrapping_add(24),
                        self.r32(fp.wrapping_add(24)).wrapping_add(4),
                    );
                    bb = 91;
                }
                95 => {
                    self.w32(fp.wrapping_add(20), (v41 as u32));
                    self.w32(fp.wrapping_add(12), (v40 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 94;
                }
                96 => {
                    self.w32(fp.wrapping_add(12), (v40 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 97;
                }
                97 => {
                    bb = 94;
                }
                98 => {
                    bb = 100;
                }
                99 => {
                    self.w32(fp.wrapping_add(20), 9999_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) > 0_i32) {
                        102
                    } else {
                        103
                    };
                }
                100 => {
                    bb = if ((self.r32(fp.wrapping_add(68)) as i32) <= 0_i32) {
                        114
                    } else {
                        115
                    };
                }
                101 => {
                    bb = 99;
                }
                102 => {
                    v30 = fp.wrapping_add(348);
                    bb = 104;
                }
                103 => {
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(348).wrapping_add(
                            ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        (crem_i32((self.r32(fp.wrapping_add(24)) as i32), 20_i32) as u32),
                    );
                    v36 = cdiv_i32((self.r32(fp.wrapping_add(24)) as i32), 20_i32);
                    bb = 86;
                }
                104 => {
                    v31 = cdiv_i32((self.r32(v30) as i32), (20_i32).wrapping_neg());
                    v32 = v31
                        .wrapping_add((self.r32(fp.wrapping_add(16)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(40)) as i32)
                                .wrapping_sub(crem_i32((self.r32(v30) as i32), 20_i32))
                                .wrapping_abs(),
                        );
                    v33 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub(crem_i32((self.r32(v30) as i32), 20_i32))
                        .wrapping_abs() as u32);
                    v34 = (v31
                        .wrapping_add((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs() as u32);
                    v35 = (v33.wrapping_sub(v34) as i32).wrapping_abs();
                    bb = if (v34.wrapping_add(v33) != 2_u32) {
                        107
                    } else {
                        108
                    };
                }
                105 => {
                    bb = if ((self.r32(fp.wrapping_add(8)) as i32)
                        < (self.r32(fp.wrapping_add(0)) as i32))
                    {
                        104
                    } else {
                        106
                    };
                }
                106 => {
                    bb = 103;
                }
                107 => {
                    v32 = ((v32 as u32).wrapping_add(6_u32.wrapping_mul(v34.wrapping_add(v33)))
                        as i32);
                    bb = 108;
                }
                108 => {
                    bb = if (v32 >= (self.r32(fp.wrapping_add(20)) as i32)) {
                        109
                    } else {
                        111
                    };
                }
                109 => {
                    bb = if ((v32 == (self.r32(fp.wrapping_add(20)) as i32))
                        && (v35 < (self.r32(fp.wrapping_add(12)) as i32)))
                    {
                        112
                    } else {
                        113
                    };
                }
                110 => {
                    v30 = v30.wrapping_add(4);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 105;
                }
                111 => {
                    self.w32(fp.wrapping_add(20), (v32 as u32));
                    self.w32(fp.wrapping_add(12), (v35 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 110;
                }
                112 => {
                    self.w32(fp.wrapping_add(12), (v35 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    bb = 113;
                }
                113 => {
                    bb = 110;
                }
                114 => {
                    v54 = (self.r32(fp.wrapping_add(40)) as i32);
                    v55 = (self.r32(fp.wrapping_add(28)) as i32);
                    v59 = (self.r32(fp.wrapping_add(88)) as i32);
                    v57 = (self.r32(fp.wrapping_add(80)) as i32);
                    v58 = (self.r32(fp.wrapping_add(76)) as i32);
                    bb = 116;
                }
                115 => {
                    self.w32(fp.wrapping_add(32), 9999_u32);
                    self.w32(
                        fp.wrapping_add(92),
                        self.r32(
                            self.r32(fp.wrapping_add(96)).wrapping_add(
                                ((self.r32(fp.wrapping_add(68)) as i32).wrapping_add(3_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ),
                    );
                    v43 = (self.r32(fp.wrapping_add(28)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(92)).wrapping_add(16),
                    );
                    v44 = v43.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(92))) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(40)) as i32))
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(56)) as i32).wrapping_add(
                            (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(8)) as i32),
                        ) as u32),
                    );
                    self.w32(fp.wrapping_add(60), (v44 as u32));
                    self.w32(fp.wrapping_add(20), 0_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    bb = if (self.r32(fp.wrapping_add(72)) != 0) {
                        118
                    } else {
                        119
                    };
                }
                116 => {
                    self.w32(fp.wrapping_add(76), (v54.wrapping_add(v57) as u32));
                    v60 = self.r32(fp.wrapping_add(56));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(32)) as i32)
                            .wrapping_add(v54)
                            .wrapping_add(v57) as u32),
                    );
                    v61 = v58.wrapping_add(v55);
                    bb = if (v61 < v61.wrapping_add((self.r32(fp.wrapping_add(36)) as i32))) {
                        182
                    } else {
                        183
                    };
                }
                117 => {
                    bb = 115;
                }
                118 => {
                    bb = if (self.r32(fp.wrapping_add(72)) != 2_u32) {
                        120
                    } else {
                        121
                    };
                }
                119 => {
                    bb = if (v43 >= v44) { 153 } else { 154 };
                }
                120 => {
                    bb = 122;
                }
                121 => {
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(28)) as i32).wrapping_add(
                            (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(4)) as i32),
                        ) as u32),
                    );
                    bb = if (v43 >= v44) { 124 } else { 125 };
                }
                122 => {
                    v54 = crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32);
                    v55 = cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32);
                    bb = if (self.r32(fp.wrapping_add(72)) != 0) {
                        176
                    } else {
                        178
                    };
                }
                123 => {
                    bb = 121;
                }
                124 => {
                    bb = 122;
                }
                125 => {
                    self.w32(fp.wrapping_add(16), (20_i32.wrapping_mul(v43) as u32));
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(fp.wrapping_add(44)) as i32).wrapping_sub(v43) as u32),
                    );
                    bb = 127;
                }
                126 => {
                    bb = 125;
                }
                127 => {
                    bb = if (1_i32 != 0) { 128 } else { 129 };
                }
                128 => {
                    v50 = ((self.r32(self.r32(fp.wrapping_add(92))) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(40)) as i32))
                        as u32);
                    bb = if ((self.r32(fp.wrapping_add(56)) as i32)
                        < (self.r32(fp.wrapping_add(4)) as i32))
                    {
                        130
                    } else {
                        131
                    };
                }
                129 => {
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(48)) as i32)
                            .wrapping_sub((self.r32(fp.wrapping_add(56)) as i32))
                            as u32),
                    );
                    bb = 137;
                }
                130 => {
                    bb = 129;
                }
                131 => {
                    bb = 133;
                }
                132 => {
                    bb = 131;
                }
                133 => {
                    v42 = ({
                        let t5 = (self.r32(fp.wrapping_add(8)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(8), (t5 as u32));
                        t5
                    } < (self.r32(fp.wrapping_add(60)) as i32));
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(fp.wrapping_add(80)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (!v42) { 134 } else { 135 };
                }
                134 => {
                    bb = 122;
                }
                135 => {
                    bb = 127;
                }
                136 => {
                    bb = 135;
                }
                137 => {
                    bb = if (1_i32 != 0) { 138 } else { 139 };
                }
                138 => {
                    bb = if ((((!((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0))
                        || (self.r32(fp.wrapping_add(16)) >= 400_u32))
                        || (v50 >= 20_u32))
                        || (((self.r32(fp.wrapping_add(8)) as i32)
                            != (self.r32(fp.wrapping_add(44)) as i32))
                            && (v50 != ((self.r32(fp.wrapping_add(48)) as i32) as u32))))
                    {
                        140
                    } else {
                        141
                    };
                }
                139 => {
                    bb = 119;
                }
                140 => {
                    bb = 142;
                }
                141 => {
                    v51 = ((self.r32(fp.wrapping_add(36)) as i32).wrapping_abs() as u32);
                    v52 = ((self.r32(fp.wrapping_add(80)) as i32).wrapping_abs() as u32);
                    v53 = (v51.wrapping_sub(v52) as i32).wrapping_abs();
                    bb = if ((v52.wrapping_add(v51) as i32)
                        >= (self.r32(fp.wrapping_add(32)) as i32))
                    {
                        144
                    } else {
                        146
                    };
                }
                142 => {
                    v50 = v50.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if ((v50 as i32) >= (self.r32(fp.wrapping_add(4)) as i32)) {
                        150
                    } else {
                        151
                    };
                }
                143 => {
                    bb = 141;
                }
                144 => {
                    bb = if ((v52.wrapping_add(v51)
                        != ((self.r32(fp.wrapping_add(32)) as i32) as u32))
                        || (v53 >= (self.r32(fp.wrapping_add(20)) as i32)))
                    {
                        147
                    } else {
                        148
                    };
                }
                145 => {
                    self.w32(fp.wrapping_add(20), (v53 as u32));
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(16)).wrapping_add(v50) as i32) as u32),
                    );
                    bb = 142;
                }
                146 => {
                    self.w32(fp.wrapping_add(32), ((v52.wrapping_add(v51) as i32) as u32));
                    bb = 145;
                }
                147 => {
                    bb = 142;
                }
                148 => {
                    bb = 145;
                }
                149 => {
                    bb = 148;
                }
                150 => {
                    bb = 133;
                }
                151 => {
                    bb = 137;
                }
                152 => {
                    bb = 151;
                }
                153 => {
                    bb = 122;
                }
                154 => {
                    v45 = (self.r32(self.r32(fp.wrapping_add(92))) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(40)) as i32));
                    self.w32(fp.wrapping_add(16), (20_i32.wrapping_mul(v43) as u32));
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(44)) as i32).wrapping_sub(v43) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(12)) as i32) as u32),
                    );
                    bb = 156;
                }
                155 => {
                    bb = 154;
                }
                156 => {
                    bb = if (v45 >= (self.r32(fp.wrapping_add(4)) as i32)) {
                        159
                    } else {
                        160
                    };
                }
                157 => {
                    bb = if (!v49) { 156 } else { 158 };
                }
                158 => {
                    bb = 122;
                }
                159 => {
                    bb = 161;
                }
                160 => {
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_sub(v45) as u32),
                    );
                    bb = 163;
                }
                161 => {
                    v49 = ((self.r32(fp.wrapping_add(68)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_sub(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(fp.wrapping_add(68)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 157;
                }
                162 => {
                    bb = 160;
                }
                163 => {
                    bb = if ((((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0)
                        && (self.r32(fp.wrapping_add(16)) < 400_u32))
                        && ((v45 as u32) < 20_u32))
                    {
                        166
                    } else {
                        167
                    };
                }
                164 => {
                    bb = if (v45 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        163
                    } else {
                        165
                    };
                }
                165 => {
                    v45 = (self.r32(self.r32(fp.wrapping_add(92))) as i32)
                        .wrapping_add((self.r32(fp.wrapping_add(40)) as i32));
                    bb = 161;
                }
                166 => {
                    v46 = ((self.r32(fp.wrapping_add(60)) as i32).wrapping_abs() as u32);
                    v47 = ((self.r32(fp.wrapping_add(36)) as i32).wrapping_abs() as u32);
                    v48 = (v46.wrapping_sub(v47) as i32).wrapping_abs();
                    bb = if ((v47.wrapping_add(v46) as i32)
                        < (self.r32(fp.wrapping_add(32)) as i32))
                    {
                        168
                    } else {
                        169
                    };
                }
                167 => {
                    bb = 171;
                }
                168 => {
                    self.w32(fp.wrapping_add(32), ((v47.wrapping_add(v46) as i32) as u32));
                    bb = 170;
                }
                169 => {
                    bb = if ((v47.wrapping_add(v46)
                        == ((self.r32(fp.wrapping_add(32)) as i32) as u32))
                        && (v48 < (self.r32(fp.wrapping_add(20)) as i32)))
                    {
                        173
                    } else {
                        174
                    };
                }
                170 => {
                    self.w32(fp.wrapping_add(20), (v48 as u32));
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(16)).wrapping_add((v45 as u32)) as i32) as u32),
                    );
                    bb = 171;
                }
                171 => {
                    v45 = v45.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(60)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 164;
                }
                172 => {
                    bb = 169;
                }
                173 => {
                    bb = 170;
                }
                174 => {
                    bb = 167;
                }
                175 => {
                    bb = 174;
                }
                176 => {
                    bb = if (self.r32(fp.wrapping_add(72)) == 2_u32) {
                        179
                    } else {
                        181
                    };
                }
                177 => {
                    v57 = (self.r32(v56) as i32);
                    v58 = (self.r32(v56.wrapping_add(4)) as i32);
                    v59 = (self.r32(fp.wrapping_add(88)) as i32);
                    self.w32(
                        fp.wrapping_add(32),
                        ((self.r32(v56.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(v56.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(56), v56.wrapping_add(16));
                    bb = 116;
                }
                178 => {
                    v56 = self.r32(
                        self.r32(fp.wrapping_add(96)).wrapping_add(
                            ((self.r32(fp.wrapping_add(64)) as i32).wrapping_add(3_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(92), v56);
                    bb = 177;
                }
                179 => {
                    v56 = self.r32(
                        self.r32(fp.wrapping_add(96)).wrapping_add(
                            ((self.r32(fp.wrapping_add(64)) as i32).wrapping_add(18_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(92), v56);
                    bb = 180;
                }
                180 => {
                    bb = 177;
                }
                181 => {
                    v56 = self.r32(fp.wrapping_add(92));
                    bb = 180;
                }
                182 => {
                    v62 = (20_i32.wrapping_mul(v61) as u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(36)) as i32) as u32),
                    );
                    bb = 184;
                }
                183 => {
                    v65 = (self.r32(fp.wrapping_add(76)) as i32);
                    v66 = ((self.r32(fp.wrapping_add(72)) & 15_u32) as i32);
                    v67 = (self
                        .r32(fp.wrapping_add(76))
                        .wrapping_add(((self.r32(fp.wrapping_add(32)) as i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(68), (16_i32.wrapping_mul(v66) as u32));
                    self.w32(fp.wrapping_add(0), self.r32(fp.wrapping_add(56)));
                    self.w32(fp.wrapping_add(72), (v66 as u32));
                    self.w32((v59.wrapping_add(7400_i32) as u32), 0_u32);
                    bb = if (v61 < v61.wrapping_add((self.r32(fp.wrapping_add(36)) as i32))) {
                        197
                    } else {
                        198
                    };
                }
                184 => {
                    i = self.r32(fp.wrapping_add(76));
                    bb = 187;
                }
                185 => {
                    bb = if ((self.r32(fp.wrapping_add(64)) as i32) != 0) {
                        184
                    } else {
                        186
                    };
                }
                186 => {
                    bb = 183;
                }
                187 => {
                    bb = if ((i as i32) < (self.r32(fp.wrapping_add(4)) as i32)) {
                        188
                    } else {
                        190
                    };
                }
                188 => {
                    bb = if (self.r32(v60) != 0) { 191 } else { 192 };
                }
                189 => {
                    i = i.wrapping_add(1);
                    bb = 187;
                }
                190 => {
                    v62 = v62.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(64)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 185;
                }
                191 => {
                    bb = if ((v62 < 400_u32) && (i < 20_u32)) {
                        193
                    } else {
                        194
                    };
                }
                192 => {
                    v60 = v60.wrapping_add(4);
                    bb = 189;
                }
                193 => {
                    v64 = (self.r32(
                        (v59 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v62.wrapping_add(i)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v64 != (1_i32).wrapping_neg())
                        && ((v64.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(100))))
                    {
                        195
                    } else {
                        196
                    };
                }
                194 => {
                    bb = 192;
                }
                195 => {
                    self.w32(fp.wrapping_add(72), self.r32(v60));
                    bb = 196;
                }
                196 => {
                    bb = 194;
                }
                197 => {
                    v68 = (20_i32.wrapping_mul(v61) as u32);
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(fp.wrapping_add(36)) as i32) as u32),
                    );
                    bb = 199;
                }
                198 => {
                    bb = if (v66 != 15_i32) { 238 } else { 239 };
                }
                199 => {
                    bb = if (1_i32 != 0) { 200 } else { 201 };
                }
                200 => {
                    bb = if (v65 >= v67) { 202 } else { 203 };
                }
                201 => {
                    bb = 198;
                }
                202 => {
                    bb = 204;
                }
                203 => {
                    bb = 206;
                }
                204 => {
                    v68 = v68.wrapping_add(20_u32);
                    bb = if (!({
                        let t6 = (self.r32(fp.wrapping_add(64)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(64), (t6 as u32));
                        t6
                    } != 0))
                    {
                        235
                    } else {
                        236
                    };
                }
                205 => {
                    bb = 203;
                }
                206 => {
                    bb = if (((self.r32(fp.wrapping_add(68)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                        == 0_i32)
                    {
                        209
                    } else {
                        210
                    };
                }
                207 => {
                    bb = if (v65
                        < (self
                            .r32(fp.wrapping_add(76))
                            .wrapping_add(((self.r32(fp.wrapping_add(32)) as i32) as u32))
                            as i32))
                    {
                        206
                    } else {
                        208
                    };
                }
                208 => {
                    v65 = (self.r32(fp.wrapping_add(76)) as i32);
                    bb = 204;
                }
                209 => {
                    bb = 211;
                }
                210 => {
                    bb = if (v68 >= 400_u32) { 213 } else { 214 };
                }
                211 => {
                    v67 = (self
                        .r32(fp.wrapping_add(76))
                        .wrapping_add(((self.r32(fp.wrapping_add(32)) as i32) as u32))
                        as i32);
                    v65 = v65.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 207;
                }
                212 => {
                    bb = 210;
                }
                213 => {
                    bb = 211;
                }
                214 => {
                    bb = if ((v65 as u32) >= 20_u32) { 216 } else { 217 };
                }
                215 => {
                    bb = 214;
                }
                216 => {
                    bb = 211;
                }
                217 => {
                    v69 = v68.wrapping_add((v65 as u32));
                    bb = if (v68.wrapping_add((v65 as u32))
                        == ((self.r32(fp.wrapping_add(24)) as i32) as u32))
                    {
                        219
                    } else {
                        220
                    };
                }
                218 => {
                    bb = 217;
                }
                219 => {
                    bb = 211;
                }
                220 => {
                    v70 = (self.r32(
                        (v59 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v69))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v71 = 0_i8;
                    bb = if ((self.r32(fp.wrapping_add(112)) as i32) != 0) {
                        222
                    } else {
                        224
                    };
                }
                221 => {
                    bb = 220;
                }
                222 => {
                    bb = if ((self.r32(fp.wrapping_add(112)) as i32) == 1_i32) {
                        225
                    } else {
                        226
                    };
                }
                223 => {
                    self.w32(
                        (v59.wrapping_add(7400_i32) as u32),
                        ((self.r32((v59.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if ((v71 as i32) == 1_i32) { 233 } else { 234 };
                }
                224 => {
                    self.w32(
                        (v59.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v59.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v69 as i32) as u32),
                    );
                    bb = if ((v70 != (1_i32).wrapping_neg())
                        && ((v70.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(100))))
                    {
                        231
                    } else {
                        232
                    };
                }
                225 => {
                    self.w32(
                        (v59.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v59.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v69.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v70 > 4_i32) { 227 } else { 228 };
                }
                226 => {
                    bb = 223;
                }
                227 => {
                    bb = 229;
                }
                228 => {
                    bb = 226;
                }
                229 => {
                    v71 = 1_i8;
                    bb = 232;
                }
                230 => {
                    bb = 228;
                }
                231 => {
                    self.w32(
                        (v59.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v59.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v69.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = 229;
                }
                232 => {
                    bb = 223;
                }
                233 => {
                    let _ = self.f_10008fc0(
                        self.r32(fp.wrapping_add(96)),
                        (self.r32(fp.wrapping_add(84)) as i32),
                        v70.wrapping_sub(5_i32),
                        fp.wrapping_add(116),
                        (fp.wrapping_add(136) as i32),
                    );
                    bb = 234;
                }
                234 => {
                    bb = 211;
                }
                235 => {
                    v66 = (self.r32(fp.wrapping_add(72)) as i32);
                    bb = 201;
                }
                236 => {
                    bb = 199;
                }
                237 => {
                    bb = 236;
                }
                238 => {
                    let t7 = v66;
                    bb = match t7 {
                        1_i32 => 241,
                        2_i32 => 242,
                        4_i32 => 243,
                        8_i32 => 244,
                        _ => 245,
                    };
                }
                239 => {
                    v72 = (self.r32(
                        (v59.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(24)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v72 == (1_i32).wrapping_neg()) {
                        253
                    } else {
                        254
                    };
                }
                240 => {
                    bb = 251;
                }
                241 => {
                    v72 = 0_i32;
                    bb = 240;
                }
                242 => {
                    v72 = 1_i32;
                    bb = 240;
                }
                243 => {
                    v72 = 2_i32;
                    bb = 240;
                }
                244 => {
                    v72 = 3_i32;
                    bb = 240;
                }
                245 => {
                    v72 = (self.r32(fp.wrapping_add(112)) as i32);
                    bb = 240;
                }
                246 => {
                    bb = 242;
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
                    bb = 240;
                }
                251 => {
                    v77 = (self.r32(
                        (v59.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(24)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v77 == (1_i32).wrapping_neg()) {
                        280
                    } else {
                        281
                    };
                }
                252 => {
                    bb = 239;
                }
                253 => {
                    v72 = (self.r32(fp.wrapping_add(108)) as i32);
                    bb = 254;
                }
                254 => {
                    v73 = (self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32));
                    v74 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32));
                    v75 = (self.r32(fp.wrapping_add(44)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs();
                    v76 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(40)) as i32))
                        .wrapping_abs();
                    bb = if (v76 != v75) { 255 } else { 256 };
                }
                255 => {
                    bb = if (v76 < v75) { 257 } else { 258 };
                }
                256 => {
                    bb = if (v72 != 0) { 262 } else { 264 };
                }
                257 => {
                    v72 = (if (v73 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 251;
                }
                258 => {
                    bb = 260;
                }
                259 => {
                    bb = 258;
                }
                260 => {
                    v72 = (if (v74 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 251;
                }
                261 => {
                    bb = 256;
                }
                262 => {
                    bb = if (v72 == 1_i32) { 265 } else { 267 };
                }
                263 => {
                    bb = 251;
                }
                264 => {
                    bb = if (v73 > 0_i32) { 278 } else { 279 };
                }
                265 => {
                    bb = if (v74 < 0_i32) { 268 } else { 269 };
                }
                266 => {
                    bb = 263;
                }
                267 => {
                    bb = if (v72 == 2_i32) { 270 } else { 271 };
                }
                268 => {
                    v72 = (if (v73 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 269;
                }
                269 => {
                    bb = 266;
                }
                270 => {
                    bb = if (v73 >= 0_i32) { 272 } else { 273 };
                }
                271 => {
                    bb = if ((v72 == 3_i32) && (v74 > 0_i32)) {
                        276
                    } else {
                        277
                    };
                }
                272 => {
                    bb = 251;
                }
                273 => {
                    bb = 260;
                }
                274 => {
                    bb = 273;
                }
                275 => {
                    bb = 271;
                }
                276 => {
                    v72 = (if (v73 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 277;
                }
                277 => {
                    bb = 266;
                }
                278 => {
                    v72 = (if (v74 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 279;
                }
                279 => {
                    bb = 263;
                }
                280 => {
                    v77 = (self.r32(fp.wrapping_add(108)) as i32);
                    bb = 281;
                }
                281 => {
                    bb = if (v72 == v77) { 282 } else { 283 };
                }
                282 => {
                    v72 = (1_i32).wrapping_neg();
                    bb = 283;
                }
                283 => {
                    v78 = self.r32(fp.wrapping_add(104));
                    self.w32(
                        self.r32(fp.wrapping_add(104)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(40)) as i32) as u32),
                    );
                    self.w32(
                        v78.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(v78.wrapping_add(1540), 2_u32);
                    self.w32(
                        v78.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(24)) as i32) as u32),
                    );
                    self.w32(v78.wrapping_add(1556), (v72 as u32));
                    return 0_i32;
                }
                284 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1000B5E0` (3000 bytes).
    pub(crate) fn f_1000b5e0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(1952);
        let r = self.f_1000b5e0_body(fp, this);
        self.leave(1952);
        r
    }

    fn f_1000b5e0_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: u32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: i32 = 0;
        let mut v13: i32 = 0;
        let mut v14: bool = false;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i32 = 0;
        let mut v19: u32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: u32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: u32 = 0;
        let mut v27: i32 = 0;
        let mut v28: u32 = 0;
        let mut v29: u32 = 0;
        let mut v30: i32 = 0;
        let mut v31: i32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i64 = 0;
        let mut v34: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v39: i32 = 0;
        let mut v40: u32 = 0;
        let mut v41: u32 = 0;
        let mut v42: i32 = 0;
        let mut v43: bool = false;
        let mut v44: u32 = 0;
        let mut v45: u32 = 0;
        let mut v46: u32 = 0;
        let mut v47: i32 = 0;
        let mut v48: i32 = 0;
        let mut v49: i32 = 0;
        let mut v50: u32 = 0;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: u32 = 0;
        let mut v55: i32 = 0;
        let mut v56: u32 = 0;
        let mut i: u32 = 0;
        let mut v58: i32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: i32 = 0;
        let mut v62: u32 = 0;
        let mut v63: u32 = 0;
        let mut v64: i32 = 0;
        let mut v65: i8 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: i32 = 0;
        let mut v70: i32 = 0;
        let mut v71: i32 = 0;
        let mut v72: u32 = 0;
        let mut v74: u32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v1 = self.r32(this);
                    self.w32(fp.wrapping_add(76), this);
                    v2 = ((self.r32(v1) as i32) as u32);
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    v3 = (self.r32(v2.wrapping_add(20)) as i32);
                    v4 = ((self.r32(v2.wrapping_add(12)) as i32) as u32);
                    v5 = ((self.r32(v2.wrapping_add(28)) as i32) as u32);
                    self.w32(fp.wrapping_add(84), (v3 as u32));
                    self.w32(fp.wrapping_add(92), v5);
                    self.w32(fp.wrapping_add(80), (v3.wrapping_add(4840_i32) as u32));
                    self.w32(
                        fp.wrapping_add(104),
                        ((self.r32(v5.wrapping_add(1388)) as i32) as u32),
                    );
                    v6 = (self.r32(v4.wrapping_add(2528)) as i32);
                    v7 = (self.r32(v4.wrapping_add(2532)) as i32);
                    self.w32(fp.wrapping_add(24), ((v4 as i32) as u32));
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(
                            v4.wrapping_add(
                                (179_i32
                                    .wrapping_mul((self.r32(fp.wrapping_add(104)) as i32))
                                    .wrapping_add(95_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(
                            v4.wrapping_add(
                                (179_i32
                                    .wrapping_mul((self.r32(fp.wrapping_add(104)) as i32))
                                    .wrapping_add(96_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(108),
                        ((self.r32(
                            v4.wrapping_add(
                                (179_i32
                                    .wrapping_mul((self.r32(fp.wrapping_add(104)) as i32))
                                    .wrapping_add(97_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(32), v4.wrapping_add(3600));
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(self.r32(fp.wrapping_add(32))) as i32) == 1_i32)
                        && ((self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(116)) as i32)
                            != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if v14 { 1 } else { 3 };
                }
                3 => {
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(
                            v4.wrapping_add(
                                (39_i32
                                    .wrapping_mul((self.r32(fp.wrapping_add(4)) as i32))
                                    .wrapping_add(931_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    v15 = 0_i32;
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(
                            v4.wrapping_add(
                                (39_i32
                                    .wrapping_mul((self.r32(fp.wrapping_add(4)) as i32))
                                    .wrapping_add(932_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32) as u32),
                    );
                    v16 = (self
                        .r32(((self.r32(fp.wrapping_add(84)) as i32).wrapping_add(6440_i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(20), 0_u32);
                    bb = if (v16 > 0_i32) { 14 } else { 15 };
                }
                4 => {
                    bb = if (v6
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            v7.wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                                .wrapping_abs(),
                        )
                        > 3_i32)
                    {
                        6
                    } else {
                        8
                    };
                }
                5 => {
                    v14 = ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1_i32) < 20_i32);
                    self.w32(
                        fp.wrapping_add(32),
                        self.r32(fp.wrapping_add(32)).wrapping_add(156),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v8 = ((self.r32(fp.wrapping_add(16)) as i32)
                        .wrapping_sub(
                            (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(128)) as i32),
                        )
                        .wrapping_abs()
                        .wrapping_add(
                            (self.r32(fp.wrapping_add(28)) as i32)
                                .wrapping_sub(
                                    (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(124))
                                        as i32),
                                )
                                .wrapping_abs(),
                        ) as u32);
                    v10 = v7.wrapping_sub(
                        (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(128)) as i32),
                    );
                    v9 = (v6
                        .wrapping_sub(
                            (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(124)) as i32),
                        )
                        .wrapping_abs() as u32);
                    bb = 7;
                }
                7 => {
                    v11 = (v10.wrapping_abs() as u32);
                    v12 = (v9.wrapping_sub(v11) as i32).wrapping_abs();
                    v13 = (v8.wrapping_add(6_u32.wrapping_mul(v11.wrapping_add(v9))) as i32);
                    bb = if (v13 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        9
                    } else {
                        11
                    };
                }
                8 => {
                    v8 = (v7
                        .wrapping_sub(
                            (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(128)) as i32),
                        )
                        .wrapping_abs()
                        .wrapping_add(
                            v6.wrapping_sub(
                                (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(124)) as i32),
                            )
                            .wrapping_abs(),
                        ) as u32);
                    v9 = ((self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_sub(
                            (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(124)) as i32),
                        )
                        .wrapping_abs() as u32);
                    v10 = (self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(
                        (self.r32(self.r32(fp.wrapping_add(32)).wrapping_add(128)) as i32),
                    );
                    bb = 7;
                }
                9 => {
                    bb = if ((v13 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v12 < (self.r32(fp.wrapping_add(12)) as i32)))
                    {
                        12
                    } else {
                        13
                    };
                }
                10 => {
                    bb = 5;
                }
                11 => {
                    self.w32(fp.wrapping_add(12), (v12 as u32));
                    self.w32(fp.wrapping_add(8), (v13 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    bb = 10;
                }
                12 => {
                    self.w32(fp.wrapping_add(12), (v12 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    bb = 13;
                }
                13 => {
                    bb = 10;
                }
                14 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(344),
                        self.r32(fp.wrapping_add(80)),
                        (4_i32.wrapping_mul(v16) as u32),
                    );
                    v15 = v16;
                    self.w32(fp.wrapping_add(20), (v16 as u32));
                    bb = 15;
                }
                15 => {
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    bb = if (v15 > 0_i32) { 16 } else { 17 };
                }
                16 => {
                    self.w32(fp.wrapping_add(56), fp.wrapping_add(344));
                    bb = 18;
                }
                17 => {
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(fp.wrapping_add(344).wrapping_add(
                            ((self.r32(fp.wrapping_add(4)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        (crem_i32((self.r32(fp.wrapping_add(56)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(32),
                        (cdiv_i32((self.r32(fp.wrapping_add(56)) as i32), 20_i32) as u32),
                    );
                    let _ = self.f_100064e0(
                        self.r32(fp.wrapping_add(76)),
                        fp.wrapping_add(112),
                        (fp.wrapping_add(132) as i32),
                        (self.r32(self.r32(fp.wrapping_add(92)).wrapping_add(1384)) as i32),
                        (self.r32(fp.wrapping_add(104)) as i32),
                        0_i32,
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(fp.wrapping_add(116)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(100),
                        ((self.r32(fp.wrapping_add(120)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(124)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(96),
                        ((self.r32(fp.wrapping_add(128)) as i32) as u32),
                    );
                    let _ = self.f_10006940(
                        self.r32(fp.wrapping_add(76)),
                        fp.wrapping_add(68),
                        (self.r32(fp.wrapping_add(116)) as i32),
                        (self.r32(fp.wrapping_add(120)) as i32),
                        (self.r32(fp.wrapping_add(124)) as i32),
                        (self.r32(fp.wrapping_add(128)) as i32),
                    );
                    self.w32(fp.wrapping_add(44), 0_u32);
                    self.w32(fp.wrapping_add(0), 0_u32);
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(self.r32(fp.wrapping_add(68))) as i32) as u32),
                    );
                    v22 = (self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(8)) as i32);
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(8), (v22 as u32));
                    self.w32(
                        fp.wrapping_add(12),
                        self.r32(fp.wrapping_add(68)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(12)) as i32) as u32),
                    );
                    v23 = ((self.r32(fp.wrapping_add(24)) as i32).wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(52), fp.wrapping_add(244));
                    self.w32(fp.wrapping_add(48), fp.wrapping_add(144));
                    bb = 26;
                }
                18 => {
                    v17 = ((self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(56))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v18 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(56))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v19 = ((self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_add(v18)
                        .wrapping_abs() as u32);
                    v20 = (v17.wrapping_sub(v19) as i32).wrapping_abs();
                    v21 = (((self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(56))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v18.wrapping_add((self.r32(fp.wrapping_add(16)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v19.wrapping_add(v17)))
                        as i32);
                    bb = if (v21 >= (self.r32(fp.wrapping_add(8)) as i32)) {
                        21
                    } else {
                        23
                    };
                }
                19 => {
                    bb = if v14 { 18 } else { 20 };
                }
                20 => {
                    bb = 17;
                }
                21 => {
                    bb = if ((v21 == (self.r32(fp.wrapping_add(8)) as i32))
                        && (v20 < (self.r32(fp.wrapping_add(12)) as i32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                22 => {
                    v14 = ({
                        let t1 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(0), (t1 as u32));
                        t1
                    } < (self.r32(fp.wrapping_add(20)) as i32));
                    self.w32(
                        fp.wrapping_add(56),
                        self.r32(fp.wrapping_add(56)).wrapping_add(4),
                    );
                    bb = 19;
                }
                23 => {
                    self.w32(fp.wrapping_add(8), (v21 as u32));
                    self.w32(fp.wrapping_add(12), (v20 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    bb = 22;
                }
                24 => {
                    self.w32(fp.wrapping_add(12), (v20 as u32));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    bb = 25;
                }
                25 => {
                    bb = 22;
                }
                26 => {
                    bb = if (((self.r32(v23) as i32) == 1_i32)
                        && ((self.r32(v23.wrapping_add(116)) as i32) != 99_i32))
                    {
                        29
                    } else {
                        30
                    };
                }
                27 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) < 20_i32) {
                        26
                    } else {
                        28
                    };
                }
                28 => {
                    bb = if ((self.r32(fp.wrapping_add(44)) as i32) <= 0_i32) {
                        51
                    } else {
                        52
                    };
                }
                29 => {
                    v24 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_add((self.r32(v23.wrapping_add(124)) as i32));
                    v25 = (self.r32(fp.wrapping_add(72)) as i32)
                        .wrapping_add((self.r32(v23.wrapping_add(128)) as i32));
                    v26 = self.r32(fp.wrapping_add(12));
                    v27 = v24.wrapping_add((self.r32(fp.wrapping_add(8)) as i32));
                    self.w8(fp.wrapping_add(42), 0_u8);
                    self.w8(fp.wrapping_add(43), 0_u8);
                    v28 = (v25 as u32);
                    bb = if (v25 < v25.wrapping_add((self.r32(fp.wrapping_add(20)) as i32))) {
                        31
                    } else {
                        32
                    };
                }
                30 => {
                    v23 = v23.wrapping_add(156);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 27;
                }
                31 => {
                    bb = 33;
                }
                32 => {
                    bb = 30;
                }
                33 => {
                    v29 = ((self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_add((self.r32(v23.wrapping_add(124)) as i32))
                        as u32);
                    bb = if (v24 < v27) { 36 } else { 37 };
                }
                34 => {
                    bb = if ((v28 as i32)
                        < v25.wrapping_add((self.r32(fp.wrapping_add(20)) as i32)))
                    {
                        33
                    } else {
                        35
                    };
                }
                35 => {
                    bb = if (((self.r8(fp.wrapping_add(42)) as i8) as i32) == 1_i32) {
                        47
                    } else {
                        48
                    };
                }
                36 => {
                    bb = 38;
                }
                37 => {
                    v28 = v28.wrapping_add(1);
                    bb = 34;
                }
                38 => {
                    bb = if ((self.r32(v26) as i32) != 0) {
                        41
                    } else {
                        42
                    };
                }
                39 => {
                    bb = if ((v29 as i32) < v27) { 38 } else { 40 };
                }
                40 => {
                    bb = 37;
                }
                41 => {
                    bb = if ((((v28 < 20_u32) && (v29 < 20_u32))
                        && (v29 == ((self.r32(fp.wrapping_add(64)) as i32) as u32)))
                        && (v28 == ((self.r32(fp.wrapping_add(32)) as i32) as u32)))
                    {
                        43
                    } else {
                        44
                    };
                }
                42 => {
                    v26 = v26.wrapping_add(4);
                    v29 = v29.wrapping_add(1);
                    bb = 39;
                }
                43 => {
                    self.w8(fp.wrapping_add(42), 1_u8);
                    bb = if ((v29 == ((self.r32(fp.wrapping_add(28)) as i32) as u32))
                        && (v28 == ((self.r32(fp.wrapping_add(16)) as i32) as u32)))
                    {
                        45
                    } else {
                        46
                    };
                }
                44 => {
                    bb = 42;
                }
                45 => {
                    self.w8(fp.wrapping_add(43), 1_u8);
                    bb = 46;
                }
                46 => {
                    bb = 44;
                }
                47 => {
                    self.w32(
                        self.r32(fp.wrapping_add(48)),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(fp.wrapping_add(44)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        self.r32(fp.wrapping_add(48)).wrapping_add(4),
                    );
                    bb = 48;
                }
                48 => {
                    bb = if (((self.r8(fp.wrapping_add(43)) as i8) as i32) == 1_i32) {
                        49
                    } else {
                        50
                    };
                }
                49 => {
                    {
                        let a0 = {
                            let t2 = self.r32(fp.wrapping_add(52));
                            self.w32(fp.wrapping_add(52), t2.wrapping_add(4));
                            t2
                        };
                        let a1 = ((self.r32(fp.wrapping_add(0)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 50;
                }
                50 => {
                    bb = 32;
                }
                51 => {
                    v74 = self.r32(fp.wrapping_add(92));
                    self.w32(
                        self.r32(fp.wrapping_add(92)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(64)) as i32) as u32),
                    );
                    self.w32(v74.wrapping_add(1540), 2_u32);
                    self.w32(
                        v74.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(32)) as i32) as u32),
                    );
                    self.w32(
                        v74.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(56)) as i32) as u32),
                    );
                    self.w32(v74.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
                    return 0_i32;
                }
                52 => {
                    v30 = 0_i32;
                    v31 = 9999_i32;
                    self.w32(fp.wrapping_add(48), 0_u32);
                    v32 = fp.wrapping_add(144);
                    bb = 54;
                }
                53 => {
                    bb = 52;
                }
                54 => {
                    v33 = ((self.r32(
                        ((self.r32(fp.wrapping_add(24)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v32) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        as i64);
                    v34 = ((((((v33 as u64) >> 32) as u32) as i64) ^ v33)
                        .wrapping_sub(((((v33 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(24)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v32) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(16)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v34 < v31) { 57 } else { 58 };
                }
                55 => {
                    bb = if (v30 < (self.r32(fp.wrapping_add(44)) as i32)) {
                        54
                    } else {
                        56
                    };
                }
                56 => {
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(fp.wrapping_add(144).wrapping_add(
                            ((self.r32(fp.wrapping_add(48)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v35 = (self.r32(
                        ((self.r32(fp.wrapping_add(24)) as i32)
                            .wrapping_add(
                                156_i32.wrapping_mul((self.r32(fp.wrapping_add(80)) as i32)),
                            )
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(24)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(80)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(48), (v35 as u32));
                    bb = if ((self.r32(fp.wrapping_add(88)) as i32) <= 0_i32) {
                        59
                    } else {
                        60
                    };
                }
                57 => {
                    v31 = v34;
                    self.w32(fp.wrapping_add(48), (v30 as u32));
                    bb = 58;
                }
                58 => {
                    v30 = v30.wrapping_add(1);
                    v32 = v32.wrapping_add(4);
                    bb = 55;
                }
                59 => {
                    v48 = crem_i32((self.r32(fp.wrapping_add(56)) as i32), 20_i32);
                    v49 = cdiv_i32((self.r32(fp.wrapping_add(56)) as i32), 20_i32);
                    v53 = (self.r32(fp.wrapping_add(84)) as i32);
                    v52 = (self.r32(fp.wrapping_add(72)) as i32);
                    v51 = (self.r32(fp.wrapping_add(4)) as i32);
                    bb = 61;
                }
                60 => {
                    self.w32(fp.wrapping_add(28), 9999_u32);
                    self.w32(fp.wrapping_add(8), 0_u32);
                    self.w32(
                        fp.wrapping_add(68),
                        self.r32(
                            self.r32(fp.wrapping_add(76)).wrapping_add(
                                ((self.r32(fp.wrapping_add(88)) as i32).wrapping_add(3_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ),
                    );
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(68)).wrapping_add(16),
                    );
                    v36 = (self.r32(fp.wrapping_add(32)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(4)) as i32),
                    );
                    v37 = (self.r32(fp.wrapping_add(64)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(68))) as i32));
                    v38 = v37.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(8)) as i32),
                    );
                    v39 = v36.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(68)).wrapping_add(12)) as i32),
                    );
                    self.w32(fp.wrapping_add(72), (v37 as u32));
                    self.w32(fp.wrapping_add(4), (v38 as u32));
                    self.w32(fp.wrapping_add(36), (v39 as u32));
                    bb = if ((self.r32(fp.wrapping_add(100)) as i32) != 0) {
                        63
                    } else {
                        64
                    };
                }
                61 => {
                    self.w32(fp.wrapping_add(36), (v48.wrapping_add(v51) as u32));
                    v54 = self.r32(fp.wrapping_add(12));
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(8)) as i32)
                            .wrapping_add(v48)
                            .wrapping_add(v51) as u32),
                    );
                    v55 = v52.wrapping_add(v49);
                    bb = if (v55 < v55.wrapping_add((self.r32(fp.wrapping_add(20)) as i32))) {
                        127
                    } else {
                        128
                    };
                }
                62 => {
                    bb = 60;
                }
                63 => {
                    bb = if ((self.r32(fp.wrapping_add(100)) as i32) != 2_i32) {
                        65
                    } else {
                        66
                    };
                }
                64 => {
                    bb = if (v36 >= v39) { 98 } else { 99 };
                }
                65 => {
                    bb = 67;
                }
                66 => {
                    self.w32(fp.wrapping_add(0), (v36 as u32));
                    bb = if (v36 >= v39) { 69 } else { 70 };
                }
                67 => {
                    v48 = crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32);
                    v49 = cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32);
                    bb = if ((self.r32(fp.wrapping_add(100)) as i32) != 0) {
                        121
                    } else {
                        123
                    };
                }
                68 => {
                    bb = 66;
                }
                69 => {
                    bb = 67;
                }
                70 => {
                    self.w32(fp.wrapping_add(44), (20_i32.wrapping_mul(v36) as u32));
                    self.w32(fp.wrapping_add(24), (v35.wrapping_sub(v36) as u32));
                    bb = 72;
                }
                71 => {
                    bb = 70;
                }
                72 => {
                    bb = if (1_i32 != 0) { 73 } else { 74 };
                }
                73 => {
                    v44 = ((self.r32(fp.wrapping_add(72)) as i32) as u32);
                    bb = if ((self.r32(fp.wrapping_add(72)) as i32) < v38) {
                        75
                    } else {
                        76
                    };
                }
                74 => {
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(52)) as i32)
                            .wrapping_sub((self.r32(fp.wrapping_add(72)) as i32))
                            as u32),
                    );
                    bb = 82;
                }
                75 => {
                    bb = 74;
                }
                76 => {
                    bb = 78;
                }
                77 => {
                    bb = 76;
                }
                78 => {
                    v14 = ({
                        let t3 = (self.r32(fp.wrapping_add(0)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(0), (t3 as u32));
                        t3
                    } < (self.r32(fp.wrapping_add(36)) as i32));
                    self.w32(
                        fp.wrapping_add(44),
                        self.r32(fp.wrapping_add(44)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (!v14) { 79 } else { 80 };
                }
                79 => {
                    bb = 67;
                }
                80 => {
                    bb = 72;
                }
                81 => {
                    bb = 80;
                }
                82 => {
                    bb = if (1_i32 != 0) { 83 } else { 84 };
                }
                83 => {
                    bb = if ((((!((self.r32(self.r32(fp.wrapping_add(16))) as i32) != 0))
                        || (self.r32(fp.wrapping_add(44)) >= 400_u32))
                        || (v44 >= 20_u32))
                        || (((self.r32(fp.wrapping_add(0)) as i32) != v35)
                            && (v44 != ((self.r32(fp.wrapping_add(52)) as i32) as u32))))
                    {
                        85
                    } else {
                        86
                    };
                }
                84 => {
                    bb = 64;
                }
                85 => {
                    bb = 87;
                }
                86 => {
                    v45 = ((self.r32(fp.wrapping_add(20)) as i32).wrapping_abs() as u32);
                    v46 = ((self.r32(fp.wrapping_add(24)) as i32).wrapping_abs() as u32);
                    v47 = (v45.wrapping_sub(v46) as i32).wrapping_abs();
                    bb = if ((v46.wrapping_add(v45) as i32)
                        >= (self.r32(fp.wrapping_add(28)) as i32))
                    {
                        89
                    } else {
                        91
                    };
                }
                87 => {
                    v44 = v44.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(20)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if ((v44 as i32) >= v38) { 95 } else { 96 };
                }
                88 => {
                    bb = 86;
                }
                89 => {
                    bb = if ((v46.wrapping_add(v45)
                        != ((self.r32(fp.wrapping_add(28)) as i32) as u32))
                        || (v47 >= (self.r32(fp.wrapping_add(8)) as i32)))
                    {
                        92
                    } else {
                        93
                    };
                }
                90 => {
                    self.w32(fp.wrapping_add(8), (v47 as u32));
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(44)).wrapping_add(v44) as i32) as u32),
                    );
                    bb = 87;
                }
                91 => {
                    self.w32(fp.wrapping_add(28), ((v46.wrapping_add(v45) as i32) as u32));
                    bb = 90;
                }
                92 => {
                    bb = 87;
                }
                93 => {
                    bb = 90;
                }
                94 => {
                    bb = 93;
                }
                95 => {
                    bb = 78;
                }
                96 => {
                    bb = 82;
                }
                97 => {
                    bb = 96;
                }
                98 => {
                    bb = 67;
                }
                99 => {
                    self.w32(fp.wrapping_add(0), (20_i32.wrapping_mul(v36) as u32));
                    self.w32(fp.wrapping_add(36), (v35.wrapping_sub(v36) as u32));
                    self.w32(fp.wrapping_add(24), (v39.wrapping_sub(v36) as u32));
                    bb = 101;
                }
                100 => {
                    bb = 99;
                }
                101 => {
                    bb = if (v37 >= v38) { 104 } else { 105 };
                }
                102 => {
                    bb = if (!v43) { 101 } else { 103 };
                }
                103 => {
                    bb = 67;
                }
                104 => {
                    bb = 106;
                }
                105 => {
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(52)) as i32).wrapping_sub(v37) as u32),
                    );
                    bb = 108;
                }
                106 => {
                    v43 = ((self.r32(fp.wrapping_add(24)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_sub(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 102;
                }
                107 => {
                    bb = 105;
                }
                108 => {
                    bb = if ((((self.r32(self.r32(fp.wrapping_add(16))) as i32) != 0)
                        && (self.r32(fp.wrapping_add(0)) < 400_u32))
                        && ((v37 as u32) < 20_u32))
                    {
                        111
                    } else {
                        112
                    };
                }
                109 => {
                    bb = if (v37 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        108
                    } else {
                        110
                    };
                }
                110 => {
                    v37 = (self.r32(fp.wrapping_add(72)) as i32);
                    v38 = (self.r32(fp.wrapping_add(4)) as i32);
                    bb = 106;
                }
                111 => {
                    v40 = ((self.r32(fp.wrapping_add(20)) as i32).wrapping_abs() as u32);
                    v41 = ((self.r32(fp.wrapping_add(36)) as i32).wrapping_abs() as u32);
                    v42 = (v40.wrapping_sub(v41) as i32).wrapping_abs();
                    bb = if ((v41.wrapping_add(v40) as i32)
                        < (self.r32(fp.wrapping_add(28)) as i32))
                    {
                        113
                    } else {
                        114
                    };
                }
                112 => {
                    bb = 116;
                }
                113 => {
                    self.w32(fp.wrapping_add(28), ((v41.wrapping_add(v40) as i32) as u32));
                    bb = 115;
                }
                114 => {
                    bb = if ((v41.wrapping_add(v40)
                        == ((self.r32(fp.wrapping_add(28)) as i32) as u32))
                        && (v42 < (self.r32(fp.wrapping_add(8)) as i32)))
                    {
                        118
                    } else {
                        119
                    };
                }
                115 => {
                    self.w32(fp.wrapping_add(8), (v42 as u32));
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(0)).wrapping_add((v37 as u32)) as i32) as u32),
                    );
                    bb = 116;
                }
                116 => {
                    v37 = v37.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(20)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 109;
                }
                117 => {
                    bb = 114;
                }
                118 => {
                    bb = 115;
                }
                119 => {
                    bb = 112;
                }
                120 => {
                    bb = 119;
                }
                121 => {
                    bb = if ((self.r32(fp.wrapping_add(100)) as i32) == 2_i32) {
                        124
                    } else {
                        126
                    };
                }
                122 => {
                    v51 = (self.r32(v50) as i32);
                    v52 = (self.r32(v50.wrapping_add(4)) as i32);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(v50.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(v50.wrapping_add(12)) as i32) as u32),
                    );
                    v53 = (self.r32(fp.wrapping_add(84)) as i32);
                    self.w32(fp.wrapping_add(12), v50.wrapping_add(16));
                    bb = 61;
                }
                123 => {
                    v50 = self.r32(
                        self.r32(fp.wrapping_add(76)).wrapping_add(
                            ((self.r32(fp.wrapping_add(60)) as i32).wrapping_add(3_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(68), v50);
                    bb = 122;
                }
                124 => {
                    v50 = self.r32(
                        self.r32(fp.wrapping_add(76)).wrapping_add(
                            ((self.r32(fp.wrapping_add(60)) as i32).wrapping_add(18_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(68), v50);
                    bb = 125;
                }
                125 => {
                    bb = 122;
                }
                126 => {
                    v50 = self.r32(fp.wrapping_add(68));
                    bb = 125;
                }
                127 => {
                    v56 = (20_i32.wrapping_mul(v55) as u32);
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    bb = 129;
                }
                128 => {
                    v59 = (self.r32(fp.wrapping_add(36)) as i32);
                    v60 = ((self.r32(fp.wrapping_add(24)) as i32) & 15_i32);
                    v61 = (self
                        .r32(fp.wrapping_add(36))
                        .wrapping_add(((self.r32(fp.wrapping_add(8)) as i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(88), (16_i32.wrapping_mul(v60) as u32));
                    self.w32(fp.wrapping_add(16), self.r32(fp.wrapping_add(12)));
                    self.w32(fp.wrapping_add(24), (v60 as u32));
                    self.w32((v53.wrapping_add(7400_i32) as u32), 0_u32);
                    bb = if (v55 < v55.wrapping_add((self.r32(fp.wrapping_add(20)) as i32))) {
                        142
                    } else {
                        143
                    };
                }
                129 => {
                    i = self.r32(fp.wrapping_add(36));
                    bb = 132;
                }
                130 => {
                    bb = if ((self.r32(fp.wrapping_add(60)) as i32) != 0) {
                        129
                    } else {
                        131
                    };
                }
                131 => {
                    bb = 128;
                }
                132 => {
                    bb = if ((i as i32) < (self.r32(fp.wrapping_add(4)) as i32)) {
                        133
                    } else {
                        135
                    };
                }
                133 => {
                    bb = if ((self.r32(v54) as i32) != 0) {
                        136
                    } else {
                        137
                    };
                }
                134 => {
                    i = i.wrapping_add(1);
                    bb = 132;
                }
                135 => {
                    v56 = v56.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(60)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 130;
                }
                136 => {
                    bb = if ((v56 < 400_u32) && (i < 20_u32)) {
                        138
                    } else {
                        139
                    };
                }
                137 => {
                    v54 = v54.wrapping_add(4);
                    bb = 134;
                }
                138 => {
                    v58 = (self.r32(
                        (v53 as u32)
                            .wrapping_add(4_u32.wrapping_mul(i.wrapping_add(v56)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v58 != (1_i32).wrapping_neg())
                        && ((v58.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(80))))
                    {
                        140
                    } else {
                        141
                    };
                }
                139 => {
                    bb = 137;
                }
                140 => {
                    self.w32(fp.wrapping_add(24), ((self.r32(v54) as i32) as u32));
                    bb = 141;
                }
                141 => {
                    bb = 139;
                }
                142 => {
                    v62 = (20_i32.wrapping_mul(v55) as u32);
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    bb = 144;
                }
                143 => {
                    bb = if (v60 != 15_i32) { 183 } else { 184 };
                }
                144 => {
                    bb = if (1_i32 != 0) { 145 } else { 146 };
                }
                145 => {
                    bb = if (v59 >= v61) { 147 } else { 148 };
                }
                146 => {
                    bb = 143;
                }
                147 => {
                    bb = 149;
                }
                148 => {
                    bb = 151;
                }
                149 => {
                    v62 = v62.wrapping_add(20_u32);
                    bb = if (!({
                        let t4 = (self.r32(fp.wrapping_add(60)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(60), (t4 as u32));
                        t4
                    } != 0))
                    {
                        180
                    } else {
                        181
                    };
                }
                150 => {
                    bb = 148;
                }
                151 => {
                    bb = if (((self.r32(fp.wrapping_add(88)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(16))) as i32))
                        == 0_i32)
                    {
                        154
                    } else {
                        155
                    };
                }
                152 => {
                    bb = if (v59
                        < (self
                            .r32(fp.wrapping_add(36))
                            .wrapping_add(((self.r32(fp.wrapping_add(8)) as i32) as u32))
                            as i32))
                    {
                        151
                    } else {
                        153
                    };
                }
                153 => {
                    v59 = (self.r32(fp.wrapping_add(36)) as i32);
                    bb = 149;
                }
                154 => {
                    bb = 156;
                }
                155 => {
                    bb = if (v62 >= 400_u32) { 158 } else { 159 };
                }
                156 => {
                    v61 = (self
                        .r32(fp.wrapping_add(36))
                        .wrapping_add(((self.r32(fp.wrapping_add(8)) as i32) as u32))
                        as i32);
                    v59 = v59.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    bb = 152;
                }
                157 => {
                    bb = 155;
                }
                158 => {
                    bb = 156;
                }
                159 => {
                    bb = if ((v59 as u32) >= 20_u32) { 161 } else { 162 };
                }
                160 => {
                    bb = 159;
                }
                161 => {
                    bb = 156;
                }
                162 => {
                    v63 = v62.wrapping_add((v59 as u32));
                    bb = if (v62.wrapping_add((v59 as u32))
                        == ((self.r32(fp.wrapping_add(56)) as i32) as u32))
                    {
                        164
                    } else {
                        165
                    };
                }
                163 => {
                    bb = 162;
                }
                164 => {
                    bb = 156;
                }
                165 => {
                    v64 = (self.r32(
                        (v53 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v63))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v65 = 0_i8;
                    bb = if ((self.r32(fp.wrapping_add(96)) as i32) != 0) {
                        167
                    } else {
                        169
                    };
                }
                166 => {
                    bb = 165;
                }
                167 => {
                    bb = if ((self.r32(fp.wrapping_add(96)) as i32) == 1_i32) {
                        170
                    } else {
                        171
                    };
                }
                168 => {
                    self.w32(
                        (v53.wrapping_add(7400_i32) as u32),
                        ((self.r32((v53.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if ((v65 as i32) == 1_i32) { 178 } else { 179 };
                }
                169 => {
                    self.w32(
                        (v53.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v53.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v63 as i32) as u32),
                    );
                    bb = if ((v64 != (1_i32).wrapping_neg())
                        && ((v64.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(80))))
                    {
                        176
                    } else {
                        177
                    };
                }
                170 => {
                    self.w32(
                        (v53.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v53.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v63.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v64 > 4_i32) { 172 } else { 173 };
                }
                171 => {
                    bb = 168;
                }
                172 => {
                    bb = 174;
                }
                173 => {
                    bb = 171;
                }
                174 => {
                    v65 = 1_i8;
                    bb = 177;
                }
                175 => {
                    bb = 173;
                }
                176 => {
                    self.w32(
                        (v53.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v53.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v63.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = 174;
                }
                177 => {
                    bb = 168;
                }
                178 => {
                    let _ = self.f_10008fc0(
                        self.r32(fp.wrapping_add(76)),
                        (self.r32(fp.wrapping_add(104)) as i32),
                        v64.wrapping_sub(5_i32),
                        fp.wrapping_add(112),
                        (fp.wrapping_add(132) as i32),
                    );
                    bb = 179;
                }
                179 => {
                    bb = 156;
                }
                180 => {
                    v60 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 146;
                }
                181 => {
                    bb = 144;
                }
                182 => {
                    bb = 181;
                }
                183 => {
                    let t5 = v60;
                    bb = match t5 {
                        1_i32 => 186,
                        2_i32 => 187,
                        4_i32 => 188,
                        8_i32 => 189,
                        _ => 190,
                    };
                }
                184 => {
                    v66 = (self.r32(
                        (v53.wrapping_add(
                            4_i32.wrapping_mul((self.r32(fp.wrapping_add(56)) as i32)),
                        )
                        .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v66 == (1_i32).wrapping_neg()) {
                        198
                    } else {
                        199
                    };
                }
                185 => {
                    bb = 196;
                }
                186 => {
                    v66 = 0_i32;
                    bb = 185;
                }
                187 => {
                    v66 = 1_i32;
                    bb = 185;
                }
                188 => {
                    v66 = 2_i32;
                    bb = 185;
                }
                189 => {
                    v66 = 3_i32;
                    bb = 185;
                }
                190 => {
                    v66 = (self.r32(fp.wrapping_add(96)) as i32);
                    bb = 185;
                }
                191 => {
                    bb = 187;
                }
                192 => {
                    bb = 188;
                }
                193 => {
                    bb = 189;
                }
                194 => {
                    bb = 190;
                }
                195 => {
                    bb = 185;
                }
                196 => {
                    v71 = (self.r32(
                        ((self.r32(fp.wrapping_add(84)) as i32)
                            .wrapping_add(
                                4_i32.wrapping_mul((self.r32(fp.wrapping_add(56)) as i32)),
                            )
                            .wrapping_add(1600_i32) as u32),
                    ) as i32);
                    bb = if (v71 == (1_i32).wrapping_neg()) {
                        225
                    } else {
                        226
                    };
                }
                197 => {
                    bb = 184;
                }
                198 => {
                    v66 = (self.r32(fp.wrapping_add(108)) as i32);
                    bb = 199;
                }
                199 => {
                    v67 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(32)) as i32));
                    v68 = (self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(64)) as i32));
                    v69 = (self.r32(fp.wrapping_add(48)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(32)) as i32))
                        .wrapping_abs();
                    v70 = (self.r32(fp.wrapping_add(52)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(64)) as i32))
                        .wrapping_abs();
                    bb = if (v70 != v69) { 200 } else { 201 };
                }
                200 => {
                    bb = if (v70 < v69) { 202 } else { 203 };
                }
                201 => {
                    bb = if (v66 != 0) { 207 } else { 209 };
                }
                202 => {
                    v66 = (if (v67 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 196;
                }
                203 => {
                    bb = 205;
                }
                204 => {
                    bb = 203;
                }
                205 => {
                    v66 = (if (v68 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 196;
                }
                206 => {
                    bb = 201;
                }
                207 => {
                    bb = if (v66 == 1_i32) { 210 } else { 212 };
                }
                208 => {
                    bb = 196;
                }
                209 => {
                    bb = if (v67 > 0_i32) { 223 } else { 224 };
                }
                210 => {
                    bb = if (v68 < 0_i32) { 213 } else { 214 };
                }
                211 => {
                    bb = 208;
                }
                212 => {
                    bb = if (v66 == 2_i32) { 215 } else { 216 };
                }
                213 => {
                    v66 = (if (v67 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 214;
                }
                214 => {
                    bb = 211;
                }
                215 => {
                    bb = if (v67 >= 0_i32) { 217 } else { 218 };
                }
                216 => {
                    bb = if ((v66 == 3_i32) && (v68 > 0_i32)) {
                        221
                    } else {
                        222
                    };
                }
                217 => {
                    bb = 196;
                }
                218 => {
                    bb = 205;
                }
                219 => {
                    bb = 218;
                }
                220 => {
                    bb = 216;
                }
                221 => {
                    v66 = (if (v67 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 222;
                }
                222 => {
                    bb = 211;
                }
                223 => {
                    v66 = (if (v68 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 224;
                }
                224 => {
                    bb = 208;
                }
                225 => {
                    v71 = (self.r32(fp.wrapping_add(108)) as i32);
                    bb = 226;
                }
                226 => {
                    bb = if (v66 == v71) { 227 } else { 228 };
                }
                227 => {
                    v66 = (1_i32).wrapping_neg();
                    bb = 228;
                }
                228 => {
                    v72 = self.r32(fp.wrapping_add(92));
                    self.w32(
                        self.r32(fp.wrapping_add(92)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(64)) as i32) as u32),
                    );
                    self.w32(
                        v72.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(32)) as i32) as u32),
                    );
                    self.w32(v72.wrapping_add(1540), 2_u32);
                    self.w32(
                        v72.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(56)) as i32) as u32),
                    );
                    self.w32(v72.wrapping_add(1556), (v66 as u32));
                    return 0_i32;
                }
                229 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1000C1C0` (76 bytes).
    pub(crate) fn f_1000c1c0(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        v3 = (self.r32((v2.wrapping_add(1404_i32) as u32)) as i32);
        if (v3 != 0) {
            if (v3 == 1_i32) {
                self.w32((v2.wrapping_add(1048_i32) as u32), 1_u32);
                self.w32((v2.wrapping_add(1404_i32) as u32), 99_u32);
            }
            return 0_i32;
        } else {
            let _ = self.f_1000c210(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000C210` (791 bytes).
    pub(crate) fn f_1000c210(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(1856);
        let r = self.f_1000c210_body(fp, this);
        self.leave(1856);
        r
    }

    fn f_1000c210_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut i: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: i64 = 0;
        let mut v12: i32 = 0;
        let mut k: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
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
        v1 = ((self.r32(self.r32(this)) as i32) as u32);
        v2 = fp.wrapping_add(152);
        v3 = ((self.r32(v1.wrapping_add(28)) as i32) as u32);
        v4 = (self.r32(v1.wrapping_add(12)) as i32);
        self.w32(
            fp.wrapping_add(16),
            ((self.r32(v1.wrapping_add(20)) as i32) as u32),
        );
        v5 = (self.r32(v3.wrapping_add(1388)) as i32);
        self.w32(fp.wrapping_add(48), (v4 as u32));
        self.w32(fp.wrapping_add(40), v3);
        v6 = (self.r32(
            (v4.wrapping_add(716_i32.wrapping_mul(v5))
                .wrapping_add(384_i32) as u32),
        ) as i32);
        self.w32(fp.wrapping_add(36), (v6 as u32));
        self.w32(
            fp.wrapping_add(24),
            ((self.r32(
                (v4.wrapping_add(716_i32.wrapping_mul(v5))
                    .wrapping_add(380_i32) as u32),
            ) as i32) as u32),
        );
        v7 = 0_i32;
        v8 = (v4.wrapping_add(3600_i32) as u32);
        i = 0_i32;
        'l1: loop {
            if !(i < 20_i32) {
                break 'l1;
            }
            if (((self.r32(v8) as i32) == 1_i32)
                && ((self.r32(v8.wrapping_add(116)) as i32) != 99_i32))
            {
                self.w32(v2, (i as u32));
                v7 = v7.wrapping_add(1);
                v2 = v2.wrapping_add(4);
            }
            v8 = v8.wrapping_add(156);
            i = i.wrapping_add(1);
        }
        v10 = 0_i32;
        self.w32(fp.wrapping_add(4), (v7 as u32));
        if (v7 > 0_i32) {
            'l2: loop {
                v11 = ((self.r32(
                    (v4.wrapping_add(
                        156_i32.wrapping_mul(
                            (self.r32(
                                fp.wrapping_add(152)
                                    .wrapping_add((v10 as u32).wrapping_mul(4)),
                            ) as i32),
                        ),
                    )
                    .wrapping_add(3728_i32) as u32),
                ) as i32)
                    .wrapping_sub(v6) as i64);
                self.w32(
                    fp.wrapping_add(52)
                        .wrapping_add((v10 as u32).wrapping_mul(4)),
                    ((((self.r32(
                        (v4.wrapping_add(
                            156_i32.wrapping_mul(
                                (self.r32(
                                    fp.wrapping_add(152)
                                        .wrapping_add((v10 as u32).wrapping_mul(4)),
                                ) as i32),
                            ),
                        )
                        .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(24)) as i32))
                        .wrapping_abs() as i64)
                        .wrapping_add((((((v11 as u64) >> 32) as u32) as i64) ^ v11))
                        .wrapping_sub(((((v11 as u64) >> 32) as u32) as i64))
                        as i32) as u32),
                );
                v10 = v10.wrapping_add(1);
                if !(v10 < (self.r32(fp.wrapping_add(4)) as i32)) {
                    break 'l2;
                }
            }
            v7 = (self.r32(fp.wrapping_add(4)) as i32);
        }
        v12 = v7.wrapping_sub(1_i32);
        self.w32(fp.wrapping_add(0), (v7.wrapping_sub(1_i32) as u32));
        'l3: loop {
            if !(v12 > 0_i32) {
                break 'l3;
            }
            k = 0_i32;
            'l4: loop {
                if !(k < v12) {
                    break 'l4;
                }
                v14 = (self.r32(
                    fp.wrapping_add(52)
                        .wrapping_add((k.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                ) as i32);
                v15 =
                    (self.r32(fp.wrapping_add(52).wrapping_add((k as u32).wrapping_mul(4))) as i32);
                if (v14 < v15) {
                    v16 = (self.r32(
                        fp.wrapping_add(152)
                            .wrapping_add((k as u32).wrapping_mul(4)),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(152)
                            .wrapping_add((k as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(152)
                                .wrapping_add((k.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        ) as i32) as u32),
                    );
                    v12 = (self.r32(fp.wrapping_add(0)) as i32);
                    self.w32(
                        fp.wrapping_add(52).wrapping_add((k as u32).wrapping_mul(4)),
                        (v14 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(152)
                            .wrapping_add((k.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        (v16 as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52)
                            .wrapping_add((k.wrapping_add(1_i32) as u32).wrapping_mul(4)),
                        (v15 as u32),
                    );
                }
                k = k.wrapping_add(1);
            }
            v12 = v12.wrapping_sub(1);
            self.w32(fp.wrapping_add(0), (v12 as u32));
        }
        if ((self.r32(fp.wrapping_add(52)) as i32) < 8_i32) {
            self.w32(fp.wrapping_add(32), 0_u32);
            v18 = (self.r32(((self.r32(fp.wrapping_add(16)) as i32).wrapping_add(6440_i32) as u32))
                as i32);
            if (v18 > 0_i32) {
                self.w32(
                    fp.wrapping_add(32),
                    ((self
                        .r32(((self.r32(fp.wrapping_add(16)) as i32).wrapping_add(6440_i32) as u32))
                        as i32) as u32),
                );
                let _ = self.memcpy(
                    fp.wrapping_add(252),
                    ((self.r32(fp.wrapping_add(16)) as i32).wrapping_add(4840_i32) as u32),
                    (4_i32.wrapping_mul(v18) as u32),
                );
            }
            v19 = 0_i32;
            self.w32(fp.wrapping_add(12), ((1_i32).wrapping_neg() as u32));
            self.w32(fp.wrapping_add(20), 0_u32);
            self.w32(fp.wrapping_add(0), 0_u32);
            if ((self.r32(fp.wrapping_add(32)) as i32) > 0_i32) {
                self.w32(fp.wrapping_add(8), fp.wrapping_add(252));
                'l5: while (1_i32 != 0) {
                    v20 = crem_i32((self.r32(self.r32(fp.wrapping_add(8))) as i32), 20_i32);
                    self.w32(
                        fp.wrapping_add(44),
                        (cdiv_i32((self.r32(self.r32(fp.wrapping_add(8))) as i32), 20_i32) as u32),
                    );
                    v21 = (v6
                        .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                        .wrapping_abs() as u32);
                    v22 = 0_i32;
                    self.w32(fp.wrapping_add(16), 0_u32);
                    v23 = ((self.r32(fp.wrapping_add(24)) as i32)
                        .wrapping_sub(v20)
                        .wrapping_abs() as u32)
                        .wrapping_add(v21);
                    v24 = 0_i32;
                    self.w32(fp.wrapping_add(28), 0_u32);
                    if ((self.r32(fp.wrapping_add(4)) as i32) > 0_i32) {
                        'l6: loop {
                            if ((!(v22 != 0))
                                || ((self.r32(
                                    fp.wrapping_add(52)
                                        .wrapping_add((v22 as u32).wrapping_mul(4)),
                                ) as i32)
                                    < 12_i32))
                            {
                                v25 = ((self.r32(
                                    ((self.r32(fp.wrapping_add(48)) as i32)
                                        .wrapping_add(
                                            156_i32.wrapping_mul(
                                                (self.r32(
                                                    fp.wrapping_add(152)
                                                        .wrapping_add((v22 as u32).wrapping_mul(4)),
                                                )
                                                    as i32),
                                            ),
                                        )
                                        .wrapping_add(3724_i32)
                                        as u32),
                                ) as i32)
                                    .wrapping_sub(v20)
                                    .wrapping_abs() as u32);
                                v26 = ((self.r32(
                                    ((self.r32(fp.wrapping_add(48)) as i32)
                                        .wrapping_add(
                                            156_i32.wrapping_mul(
                                                (self.r32(
                                                    fp.wrapping_add(152)
                                                        .wrapping_add((v22 as u32).wrapping_mul(4)),
                                                )
                                                    as i32),
                                            ),
                                        )
                                        .wrapping_add(3728_i32)
                                        as u32),
                                ) as i32)
                                    .wrapping_sub((self.r32(fp.wrapping_add(44)) as i32))
                                    .wrapping_abs() as u32);
                                self.w32(
                                    fp.wrapping_add(28),
                                    ((((self.r32(fp.wrapping_add(28)) as i32) as u32)
                                        .wrapping_add(v26.wrapping_add(v25))
                                        as i32) as u32),
                                );
                                self.w32(
                                    fp.wrapping_add(16),
                                    ((self.r32(fp.wrapping_add(16)) as i32)
                                        .wrapping_add((v25.wrapping_sub(v26) as i32).wrapping_abs())
                                        as u32),
                                );
                            }
                            v22 = v22.wrapping_add(1);
                            if !(v22 < (self.r32(fp.wrapping_add(4)) as i32)) {
                                break 'l6;
                            }
                        }
                        v24 = (self.r32(fp.wrapping_add(28)) as i32);
                        v19 = (self.r32(fp.wrapping_add(0)) as i32);
                    }
                    v27 = (v23.wrapping_add((6_i32.wrapping_mul(v24) as u32)) as i32);
                    if ((self.r32(fp.wrapping_add(12)) as i32) < v27) {
                        self.w32(fp.wrapping_add(12), (v27 as u32));
                        self.w32(fp.wrapping_add(20), (v19 as u32));
                    }
                    self.w32(
                        fp.wrapping_add(0),
                        ({
                            let t1 = v19.wrapping_add(1);
                            v19 = t1;
                            t1
                        } as u32),
                    );
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(4),
                    );
                    if (v19 >= (self.r32(fp.wrapping_add(32)) as i32)) {
                        break 'l5;
                    }
                    v6 = (self.r32(fp.wrapping_add(36)) as i32);
                }
                v3 = self.r32(fp.wrapping_add(40));
            }
            self.w32(v3.wrapping_add(1540), 2_u32);
            v17 = (self.r32(
                fp.wrapping_add(252)
                    .wrapping_add(((self.r32(fp.wrapping_add(20)) as i32) as u32).wrapping_mul(4)),
            ) as i32);
            self.w32(v3.wrapping_add(1544), (crem_i32(v17, 20_i32) as u32));
            self.w32(v3.wrapping_add(1548), (cdiv_i32(v17, 20_i32) as u32));
        } else {
            v17 = (self.r32(fp.wrapping_add(24)) as i32).wrapping_add(20_i32.wrapping_mul(v6));
            self.w32(v3.wrapping_add(1540), 2_u32);
            self.w32(
                v3.wrapping_add(1544),
                ((self.r32(fp.wrapping_add(24)) as i32) as u32),
            );
            self.w32(v3.wrapping_add(1548), (v6 as u32));
        }
        self.w32(v3.wrapping_add(1552), (v17 as u32));
        self.w32(v3.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
        return 0_i32;
    }

    /// `sub_1000C530` (76 bytes).
    pub(crate) fn f_1000c530(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_10006420(this);
        v3 = (self.r32((v2.wrapping_add(1404_i32) as u32)) as i32);
        if (v3 != 0) {
            if (v3 == 1_i32) {
                self.w32((v2.wrapping_add(1048_i32) as u32), 1_u32);
                self.w32((v2.wrapping_add(1404_i32) as u32), 99_u32);
            }
            return 0_i32;
        } else {
            let _ = self.f_1000c580(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000C580` (37 bytes).
    pub(crate) fn f_1000c580(&mut self, mut this: u32) -> i32 {
        let mut v2: i32 = 0;
        v2 = (self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32)) as i32);
        let _ = self.f_1000c210(this);
        let _ = self.f_1000a3c0(this, (self.r32((v2.wrapping_add(1388_i32) as u32)) as i32));
        return 0_i32;
    }

    /// `sub_1000C5B0` (125 bytes).
    pub(crate) fn f_1000c5b0(&mut self, mut this: u32) -> i32 {
        let mut v1: u32 = 0;
        let mut v2: i32 = 0;
        v1 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        let _ = self.f_10006420(this);
        v2 = (self.r32(v1.wrapping_add(1404)) as i32);
        if (v2 != 0) {
            if (v2 == 1_i32) {
                let _ = self.f_1000c630(this, 0_i8, 0_i8);
                self.w32(v1.wrapping_add(1404), 2_u32);
                return 0_i32;
            } else {
                if (v2 == 2_i32) {
                    self.w32(v1.wrapping_add(1048), 1_u32);
                    self.w32(v1.wrapping_add(1404), 99_u32);
                }
                return 0_i32;
            }
        } else {
            self.w32(v1.wrapping_add(1436), 1_u32);
            let _ = self.f_1000c630(this, 1_i8, 1_i8);
            self.w32(v1.wrapping_add(1404), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000C630` (2349 bytes).
    pub(crate) fn f_1000c630(&mut self, mut this: u32, mut a2: i8, mut a3: i8) -> i32 {
        let fp = self.enter(256);
        let r = self.f_1000c630_body(fp, this, a2, a3);
        self.leave(256);
        r
    }

    fn f_1000c630_body(&mut self, fp: u32, mut this: u32, mut a2: i8, mut a3: i8) -> i32 {
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        let mut v9: i32 = 0;
        let mut v10: u32 = 0;
        let mut v11: i32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: u32 = 0;
        let mut v16: u32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: u32 = 0;
        let mut v20: i64 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: u32 = 0;
        let mut v28: u32 = 0;
        let mut v29: i32 = 0;
        let mut v30: bool = false;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: u32 = 0;
        let mut v34: i32 = 0;
        let mut v35: bool = false;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: u32 = 0;
        let mut v39: i32 = 0;
        let mut v40: i32 = 0;
        let mut v41: i32 = 0;
        let mut v42: u32 = 0;
        let mut v43: i32 = 0;
        let mut v44: u32 = 0;
        let mut i: u32 = 0;
        let mut v46: i32 = 0;
        let mut v47: i32 = 0;
        let mut v48: i32 = 0;
        let mut v49: i32 = 0;
        let mut v50: u32 = 0;
        let mut v51: u32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i8 = 0;
        let mut v54: i32 = 0;
        let mut v55: i32 = 0;
        let mut v56: i32 = 0;
        let mut v57: i32 = 0;
        let mut v58: i32 = 0;
        let mut v59: i32 = 0;
        let mut v60: i32 = 0;
        let mut v61: i32 = 0;
        let mut v62: u32 = 0;
        let mut v63: i32 = 0;
        let mut v64: u32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    self.w32(fp.wrapping_add(76), this);
                    v4 = ((self.r32(self.r32(this)) as i32) as u32);
                    v5 = (self.r32(v4.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(v4.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(84),
                        ((self.r32(v4.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(60), (v5 as u32));
                    v6 = (self.r32(self.r32(fp.wrapping_add(84)).wrapping_add(1388)) as i32);
                    v7 = (self.r32(self.r32(fp.wrapping_add(84)).wrapping_add(1384)) as i32);
                    self.w32(fp.wrapping_add(92), (v6 as u32));
                    v8 = (v5.wrapping_add(716_i32.wrapping_mul(v6)) as u32);
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(v8.wrapping_add(380)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(v8.wrapping_add(384)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(100),
                        ((self.r32(v8.wrapping_add(388)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(80),
                        ((self.r32(fp.wrapping_add(56)) as i32).wrapping_add(
                            20_i32.wrapping_mul((self.r32(fp.wrapping_add(52)) as i32)),
                        ) as u32),
                    );
                    bb = if ((a2 as i32) == 1_i32) { 1 } else { 3 };
                }
                1 => {
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(112),
                        (fp.wrapping_add(132) as i32),
                        v7,
                        v6,
                        0_i32,
                    );
                    bb = 2;
                }
                2 => {
                    bb = if (!(self.r8(fp.wrapping_add(132)) != 0)) {
                        6
                    } else {
                        7
                    };
                }
                3 => {
                    bb = if (!(a2 != 0)) { 4 } else { 5 };
                }
                4 => {
                    let _ = self.f_100064e0(
                        this,
                        fp.wrapping_add(112),
                        (fp.wrapping_add(132) as i32),
                        v7,
                        v6,
                        1_i32,
                    );
                    bb = 5;
                }
                5 => {
                    bb = 2;
                }
                6 => {
                    return 0_i32;
                }
                7 => {
                    self.w32(
                        fp.wrapping_add(72),
                        ((self.r32(fp.wrapping_add(124)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(88),
                        ((self.r32(fp.wrapping_add(120)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(116)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(108),
                        ((self.r32(fp.wrapping_add(128)) as i32) as u32),
                    );
                    let _ = self.f_10006940(
                        this,
                        fp.wrapping_add(64),
                        (self.r32(fp.wrapping_add(116)) as i32),
                        (self.r32(fp.wrapping_add(120)) as i32),
                        (self.r32(fp.wrapping_add(124)) as i32),
                        (self.r32(fp.wrapping_add(128)) as i32),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(self.r32(fp.wrapping_add(64))) as i32) as u32),
                    );
                    v9 = (self.r32(self.r32(fp.wrapping_add(64)).wrapping_add(8)) as i32);
                    self.w32(
                        fp.wrapping_add(68),
                        ((self.r32(self.r32(fp.wrapping_add(64)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(28), (v9 as u32));
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(64)).wrapping_add(16),
                    );
                    self.w32(fp.wrapping_add(40), 0_u32);
                    self.w32(fp.wrapping_add(4), 0_u32);
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(self.r32(fp.wrapping_add(64)).wrapping_add(12)) as i32) as u32),
                    );
                    v10 = (v5.wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(36), ((fp.wrapping_add(244) as i32) as u32));
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(144));
                    bb = 9;
                }
                8 => {
                    bb = 7;
                }
                9 => {
                    bb = if (((self.r32(v10) as i32) == 1_i32)
                        && ((self.r32(v10.wrapping_add(116)) as i32) != 99_i32))
                    {
                        12
                    } else {
                        13
                    };
                }
                10 => {
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32) < 20_i32) {
                        9
                    } else {
                        11
                    };
                }
                11 => {
                    bb = if ((self.r32(fp.wrapping_add(40)) as i32) <= 0_i32) {
                        30
                    } else {
                        31
                    };
                }
                12 => {
                    v11 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add((self.r32(v10.wrapping_add(124)) as i32));
                    v12 = self.r32(fp.wrapping_add(8));
                    v13 = (self.r32(fp.wrapping_add(68)) as i32)
                        .wrapping_add((self.r32(v10.wrapping_add(128)) as i32));
                    v14 = v11.wrapping_add((self.r32(fp.wrapping_add(28)) as i32));
                    self.w8(fp.wrapping_add(35), 0_u8);
                    self.w32(
                        fp.wrapping_add(44),
                        (v13.wrapping_add((self.r32(fp.wrapping_add(20)) as i32)) as u32),
                    );
                    self.w8(fp.wrapping_add(27), 0_u8);
                    v15 = (v13 as u32);
                    bb = if (v13 < v13.wrapping_add((self.r32(fp.wrapping_add(20)) as i32))) {
                        14
                    } else {
                        15
                    };
                }
                13 => {
                    v10 = v10.wrapping_add(156);
                    self.w32(
                        fp.wrapping_add(4),
                        ((self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 10;
                }
                14 => {
                    bb = 16;
                }
                15 => {
                    bb = 13;
                }
                16 => {
                    v16 = ((self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add((self.r32(v10.wrapping_add(124)) as i32))
                        as u32);
                    bb = if (v11 < v14) { 19 } else { 20 };
                }
                17 => {
                    bb = if ((v15 as i32) < (self.r32(fp.wrapping_add(44)) as i32)) {
                        16
                    } else {
                        18
                    };
                }
                18 => {
                    bb = if (((self.r8(fp.wrapping_add(35)) as i8) as i32) == 1_i32) {
                        26
                    } else {
                        27
                    };
                }
                19 => {
                    bb = 21;
                }
                20 => {
                    v15 = v15.wrapping_add(1);
                    bb = 17;
                }
                21 => {
                    bb = if ((((((self.r32(v12) as i32) != 0) && (v15 < 20_u32))
                        && (v16 < 20_u32))
                        && (v16 == ((self.r32(fp.wrapping_add(56)) as i32) as u32)))
                        && (v15 == ((self.r32(fp.wrapping_add(52)) as i32) as u32)))
                    {
                        24
                    } else {
                        25
                    };
                }
                22 => {
                    bb = if ((v16 as i32) < v14) { 21 } else { 23 };
                }
                23 => {
                    bb = 20;
                }
                24 => {
                    self.w8(fp.wrapping_add(35), 1_u8);
                    self.w8(fp.wrapping_add(27), 1_u8);
                    bb = 25;
                }
                25 => {
                    v12 = v12.wrapping_add(4);
                    v16 = v16.wrapping_add(1);
                    bb = 22;
                }
                26 => {
                    self.w32(
                        self.r32(fp.wrapping_add(0)),
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(40)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 27;
                }
                27 => {
                    bb = if (((self.r8(fp.wrapping_add(27)) as i8) as i32) == 1_i32) {
                        28
                    } else {
                        29
                    };
                }
                28 => {
                    self.w32(
                        ((self.r32(fp.wrapping_add(36)) as i32) as u32),
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_add(4_i32) as u32),
                    );
                    bb = 29;
                }
                29 => {
                    bb = 15;
                }
                30 => {
                    v64 = self.r32(fp.wrapping_add(84));
                    self.w32(
                        self.r32(fp.wrapping_add(84)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(56)) as i32) as u32),
                    );
                    self.w32(
                        v64.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(80)) as i32) as u32),
                    );
                    self.w32(v64.wrapping_add(1540), 1_u32);
                    self.w32(
                        v64.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32),
                    );
                    self.w32(v64.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
                    v62 = v64;
                    bb = 32;
                }
                31 => {
                    v17 = 0_i32;
                    v18 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v19 = fp.wrapping_add(144);
                    bb = 34;
                }
                32 => {
                    bb = if ((a3 as i32) == 1_i32) { 208 } else { 209 };
                }
                33 => {
                    bb = 31;
                }
                34 => {
                    v20 = ((self.r32(
                        ((self.r32(fp.wrapping_add(60)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v19) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(56)) as i32))
                        as i64);
                    v21 = ((((((v20 as u64) >> 32) as u32) as i64) ^ v20)
                        .wrapping_sub(((((v20 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(60)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v19) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(52)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v21 < v18) { 37 } else { 38 };
                }
                35 => {
                    bb = if (v17 < (self.r32(fp.wrapping_add(40)) as i32)) {
                        34
                    } else {
                        36
                    };
                }
                36 => {
                    self.w32(
                        fp.wrapping_add(96),
                        ((self.r32(fp.wrapping_add(144).wrapping_add(
                            ((self.r32(fp.wrapping_add(0)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v22 = (self.r32(
                        ((self.r32(fp.wrapping_add(60)) as i32)
                            .wrapping_add(
                                156_i32.wrapping_mul((self.r32(fp.wrapping_add(96)) as i32)),
                            )
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(fp.wrapping_add(104), (v22 as u32));
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(60)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(96)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) <= 0_i32) {
                        39
                    } else {
                        40
                    };
                }
                37 => {
                    v18 = v21;
                    self.w32(fp.wrapping_add(0), (v17 as u32));
                    bb = 38;
                }
                38 => {
                    v17 = v17.wrapping_add(1);
                    v19 = v19.wrapping_add(4);
                    bb = 35;
                }
                39 => {
                    v36 = (self.r32(fp.wrapping_add(56)) as i32);
                    v37 = (self.r32(fp.wrapping_add(52)) as i32);
                    v40 = (self.r32(fp.wrapping_add(68)) as i32);
                    v39 = (self.r32(fp.wrapping_add(12)) as i32);
                    bb = 41;
                }
                40 => {
                    self.w32(fp.wrapping_add(20), 9999_u32);
                    self.w32(fp.wrapping_add(28), 0_u32);
                    self.w32(
                        fp.wrapping_add(64),
                        self.r32(
                            self.r32(fp.wrapping_add(76)).wrapping_add(
                                ((self.r32(fp.wrapping_add(16)) as i32).wrapping_add(3_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ),
                    );
                    self.w32(fp.wrapping_add(40), 0_u32);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(64)).wrapping_add(16),
                    );
                    v23 = (self.r32(fp.wrapping_add(52)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(64)).wrapping_add(4)) as i32),
                    );
                    v24 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(64))) as i32));
                    v25 = v24.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(64)).wrapping_add(8)) as i32),
                    );
                    v26 = v23.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(64)).wrapping_add(12)) as i32),
                    );
                    self.w32(fp.wrapping_add(68), (v24 as u32));
                    self.w32(fp.wrapping_add(12), (v25 as u32));
                    self.w32(fp.wrapping_add(44), (v26 as u32));
                    bb = if ((self.r32(fp.wrapping_add(88)) as i32) != 0) {
                        43
                    } else {
                        44
                    };
                }
                41 => {
                    self.w32(fp.wrapping_add(0), (v36.wrapping_add(v39) as u32));
                    v41 = (self.r32(fp.wrapping_add(48)) as i32);
                    v42 = self.r32(fp.wrapping_add(8));
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(fp.wrapping_add(28)) as i32)
                            .wrapping_add(v36)
                            .wrapping_add(v39) as u32),
                    );
                    v43 = v40.wrapping_add(v37);
                    bb = if (v43 < v43.wrapping_add((self.r32(fp.wrapping_add(20)) as i32))) {
                        107
                    } else {
                        108
                    };
                }
                42 => {
                    bb = 40;
                }
                43 => {
                    bb = if ((self.r32(fp.wrapping_add(88)) as i32) != 2_i32) {
                        45
                    } else {
                        46
                    };
                }
                44 => {
                    bb = if (v23 >= v26) { 78 } else { 79 };
                }
                45 => {
                    bb = 47;
                }
                46 => {
                    self.w32(fp.wrapping_add(4), (v23 as u32));
                    bb = if (v23 >= v26) { 49 } else { 50 };
                }
                47 => {
                    v36 = crem_i32((self.r32(fp.wrapping_add(40)) as i32), 20_i32);
                    v37 = cdiv_i32((self.r32(fp.wrapping_add(40)) as i32), 20_i32);
                    bb = if ((self.r32(fp.wrapping_add(88)) as i32) != 0) {
                        101
                    } else {
                        103
                    };
                }
                48 => {
                    bb = 46;
                }
                49 => {
                    bb = 47;
                }
                50 => {
                    self.w32(fp.wrapping_add(8), (20_i32.wrapping_mul(v23) as u32));
                    self.w32(fp.wrapping_add(0), (v22.wrapping_sub(v23) as u32));
                    bb = 52;
                }
                51 => {
                    bb = 50;
                }
                52 => {
                    bb = if (1_i32 != 0) { 53 } else { 54 };
                }
                53 => {
                    v31 = ((self.r32(fp.wrapping_add(68)) as i32) as u32);
                    bb = if ((self.r32(fp.wrapping_add(68)) as i32) < v25) {
                        55
                    } else {
                        56
                    };
                }
                54 => {
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(60)) as i32)
                            .wrapping_sub((self.r32(fp.wrapping_add(68)) as i32))
                            as u32),
                    );
                    bb = 62;
                }
                55 => {
                    bb = 54;
                }
                56 => {
                    bb = 58;
                }
                57 => {
                    bb = 56;
                }
                58 => {
                    v35 = ({
                        let t1 = (self.r32(fp.wrapping_add(4)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(4), (t1 as u32));
                        t1
                    } < (self.r32(fp.wrapping_add(44)) as i32));
                    self.w32(
                        fp.wrapping_add(8),
                        self.r32(fp.wrapping_add(8)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (!v35) { 59 } else { 60 };
                }
                59 => {
                    bb = 47;
                }
                60 => {
                    bb = 52;
                }
                61 => {
                    bb = 60;
                }
                62 => {
                    bb = if (1_i32 != 0) { 63 } else { 64 };
                }
                63 => {
                    bb = if ((((!((self.r32(self.r32(fp.wrapping_add(16))) as i32) != 0))
                        || (self.r32(fp.wrapping_add(8)) >= 400_u32))
                        || (v31 >= 20_u32))
                        || (((self.r32(fp.wrapping_add(4)) as i32)
                            != (self.r32(fp.wrapping_add(104)) as i32))
                            && (v31 != ((self.r32(fp.wrapping_add(60)) as i32) as u32))))
                    {
                        65
                    } else {
                        66
                    };
                }
                64 => {
                    bb = 44;
                }
                65 => {
                    bb = 67;
                }
                66 => {
                    v32 = ((self.r32(fp.wrapping_add(36)) as i32).wrapping_abs() as u32);
                    v33 = ((self.r32(fp.wrapping_add(0)) as i32).wrapping_abs() as u32);
                    v34 = (v32.wrapping_sub(v33) as i32).wrapping_abs();
                    bb = if ((v33.wrapping_add(v32) as i32)
                        >= (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        69
                    } else {
                        71
                    };
                }
                67 => {
                    v31 = v31.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(fp.wrapping_add(36)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if ((v31 as i32) >= v25) { 75 } else { 76 };
                }
                68 => {
                    bb = 66;
                }
                69 => {
                    bb = if ((v33.wrapping_add(v32)
                        != ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                        || (v34 >= (self.r32(fp.wrapping_add(28)) as i32)))
                    {
                        72
                    } else {
                        73
                    };
                }
                70 => {
                    self.w32(fp.wrapping_add(28), (v34 as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(8)).wrapping_add(v31) as i32) as u32),
                    );
                    bb = 67;
                }
                71 => {
                    self.w32(fp.wrapping_add(20), ((v33.wrapping_add(v32) as i32) as u32));
                    bb = 70;
                }
                72 => {
                    bb = 67;
                }
                73 => {
                    bb = 70;
                }
                74 => {
                    bb = 73;
                }
                75 => {
                    bb = 58;
                }
                76 => {
                    bb = 62;
                }
                77 => {
                    bb = 76;
                }
                78 => {
                    bb = 47;
                }
                79 => {
                    self.w32(fp.wrapping_add(4), (20_i32.wrapping_mul(v23) as u32));
                    self.w32(fp.wrapping_add(8), (v22.wrapping_sub(v23) as u32));
                    self.w32(fp.wrapping_add(0), (v26.wrapping_sub(v23) as u32));
                    bb = 81;
                }
                80 => {
                    bb = 79;
                }
                81 => {
                    bb = if (v24 >= v25) { 84 } else { 85 };
                }
                82 => {
                    bb = if (!v30) { 81 } else { 83 };
                }
                83 => {
                    bb = 47;
                }
                84 => {
                    bb = 86;
                }
                85 => {
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(fp.wrapping_add(60)) as i32).wrapping_sub(v24) as u32),
                    );
                    bb = 88;
                }
                86 => {
                    v30 = ((self.r32(fp.wrapping_add(0)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(4),
                        self.r32(fp.wrapping_add(4)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 82;
                }
                87 => {
                    bb = 85;
                }
                88 => {
                    bb = if ((((self.r32(self.r32(fp.wrapping_add(16))) as i32) != 0)
                        && (self.r32(fp.wrapping_add(4)) < 400_u32))
                        && ((v24 as u32) < 20_u32))
                    {
                        91
                    } else {
                        92
                    };
                }
                89 => {
                    bb = if (v24 < (self.r32(fp.wrapping_add(12)) as i32)) {
                        88
                    } else {
                        90
                    };
                }
                90 => {
                    v24 = (self.r32(fp.wrapping_add(68)) as i32);
                    v25 = (self.r32(fp.wrapping_add(12)) as i32);
                    bb = 86;
                }
                91 => {
                    v27 = ((self.r32(fp.wrapping_add(44)) as i32).wrapping_abs() as u32);
                    v28 = ((self.r32(fp.wrapping_add(8)) as i32).wrapping_abs() as u32);
                    v29 = (v27.wrapping_sub(v28) as i32).wrapping_abs();
                    bb = if ((v28.wrapping_add(v27) as i32)
                        < (self.r32(fp.wrapping_add(20)) as i32))
                    {
                        93
                    } else {
                        94
                    };
                }
                92 => {
                    bb = 96;
                }
                93 => {
                    self.w32(fp.wrapping_add(20), ((v28.wrapping_add(v27) as i32) as u32));
                    bb = 95;
                }
                94 => {
                    bb = if ((v28.wrapping_add(v27)
                        == ((self.r32(fp.wrapping_add(20)) as i32) as u32))
                        && (v29 < (self.r32(fp.wrapping_add(28)) as i32)))
                    {
                        98
                    } else {
                        99
                    };
                }
                95 => {
                    self.w32(fp.wrapping_add(28), (v29 as u32));
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(fp.wrapping_add(4)).wrapping_add((v24 as u32)) as i32) as u32),
                    );
                    bb = 96;
                }
                96 => {
                    v24 = v24.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(fp.wrapping_add(44)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 89;
                }
                97 => {
                    bb = 94;
                }
                98 => {
                    bb = 95;
                }
                99 => {
                    bb = 92;
                }
                100 => {
                    bb = 99;
                }
                101 => {
                    bb = if ((self.r32(fp.wrapping_add(88)) as i32) == 2_i32) {
                        104
                    } else {
                        106
                    };
                }
                102 => {
                    v39 = (self.r32(v38) as i32);
                    v40 = (self.r32(v38.wrapping_add(4)) as i32);
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(v38.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(v38.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(8), v38.wrapping_add(16));
                    bb = 41;
                }
                103 => {
                    v38 = self.r32(
                        self.r32(fp.wrapping_add(76)).wrapping_add(
                            ((self.r32(fp.wrapping_add(72)) as i32).wrapping_add(3_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(64), v38);
                    bb = 102;
                }
                104 => {
                    v38 = self.r32(
                        self.r32(fp.wrapping_add(76)).wrapping_add(
                            ((self.r32(fp.wrapping_add(72)) as i32).wrapping_add(18_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(64), v38);
                    bb = 105;
                }
                105 => {
                    bb = 102;
                }
                106 => {
                    v38 = self.r32(fp.wrapping_add(64));
                    bb = 105;
                }
                107 => {
                    v44 = (20_i32.wrapping_mul(v43) as u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    bb = 109;
                }
                108 => {
                    v47 = (self.r32(fp.wrapping_add(0)) as i32);
                    v48 = ((self.r32(fp.wrapping_add(36)) as i32) & 15_i32);
                    v49 = (self
                        .r32(fp.wrapping_add(0))
                        .wrapping_add(((self.r32(fp.wrapping_add(28)) as i32) as u32))
                        as i32);
                    self.w32(fp.wrapping_add(72), (16_i32.wrapping_mul(v48) as u32));
                    self.w32(fp.wrapping_add(16), self.r32(fp.wrapping_add(8)));
                    self.w32(fp.wrapping_add(36), (v48 as u32));
                    self.w32((v41.wrapping_add(7400_i32) as u32), 0_u32);
                    bb = if (v43 < v43.wrapping_add((self.r32(fp.wrapping_add(20)) as i32))) {
                        122
                    } else {
                        123
                    };
                }
                109 => {
                    i = self.r32(fp.wrapping_add(0));
                    bb = 112;
                }
                110 => {
                    bb = if ((self.r32(fp.wrapping_add(48)) as i32) != 0) {
                        109
                    } else {
                        111
                    };
                }
                111 => {
                    bb = 108;
                }
                112 => {
                    bb = if ((i as i32) < (self.r32(fp.wrapping_add(12)) as i32)) {
                        113
                    } else {
                        115
                    };
                }
                113 => {
                    bb = if ((self.r32(v42) as i32) != 0) {
                        116
                    } else {
                        117
                    };
                }
                114 => {
                    i = i.wrapping_add(1);
                    bb = 112;
                }
                115 => {
                    v44 = v44.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(48)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 110;
                }
                116 => {
                    bb = if ((v44 < 400_u32) && (i < 20_u32)) {
                        118
                    } else {
                        119
                    };
                }
                117 => {
                    v42 = v42.wrapping_add(4);
                    bb = 114;
                }
                118 => {
                    v46 = (self.r32(
                        (v41 as u32)
                            .wrapping_add(4_u32.wrapping_mul(i.wrapping_add(v44)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if ((v46 != (1_i32).wrapping_neg())
                        && (v46.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(96)) as i32)))
                    {
                        120
                    } else {
                        121
                    };
                }
                119 => {
                    bb = 117;
                }
                120 => {
                    self.w32(fp.wrapping_add(36), ((self.r32(v42) as i32) as u32));
                    bb = 121;
                }
                121 => {
                    bb = 119;
                }
                122 => {
                    v50 = (20_i32.wrapping_mul(v43) as u32);
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    bb = 124;
                }
                123 => {
                    bb = if (v48 == 15_i32) { 163 } else { 165 };
                }
                124 => {
                    bb = if (1_i32 != 0) { 125 } else { 126 };
                }
                125 => {
                    bb = if (v47 >= v49) { 127 } else { 128 };
                }
                126 => {
                    bb = 123;
                }
                127 => {
                    bb = 129;
                }
                128 => {
                    bb = 131;
                }
                129 => {
                    v50 = v50.wrapping_add(20_u32);
                    bb = if (!({
                        let t2 = (self.r32(fp.wrapping_add(48)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(48), (t2 as u32));
                        t2
                    } != 0))
                    {
                        160
                    } else {
                        161
                    };
                }
                130 => {
                    bb = 128;
                }
                131 => {
                    bb = if (((self.r32(fp.wrapping_add(72)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(16))) as i32))
                        == 0_i32)
                    {
                        134
                    } else {
                        135
                    };
                }
                132 => {
                    bb = if (v47
                        < (self
                            .r32(fp.wrapping_add(0))
                            .wrapping_add(((self.r32(fp.wrapping_add(28)) as i32) as u32))
                            as i32))
                    {
                        131
                    } else {
                        133
                    };
                }
                133 => {
                    v47 = (self.r32(fp.wrapping_add(0)) as i32);
                    bb = 129;
                }
                134 => {
                    bb = 136;
                }
                135 => {
                    bb = if (v50 >= 400_u32) { 138 } else { 139 };
                }
                136 => {
                    v49 = (self
                        .r32(fp.wrapping_add(0))
                        .wrapping_add(((self.r32(fp.wrapping_add(28)) as i32) as u32))
                        as i32);
                    v47 = v47.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(16)).wrapping_add(4),
                    );
                    bb = 132;
                }
                137 => {
                    bb = 135;
                }
                138 => {
                    bb = 136;
                }
                139 => {
                    bb = if ((v47 as u32) >= 20_u32) { 141 } else { 142 };
                }
                140 => {
                    bb = 139;
                }
                141 => {
                    bb = 136;
                }
                142 => {
                    v51 = v50.wrapping_add((v47 as u32));
                    bb = if (v50.wrapping_add((v47 as u32))
                        == ((self.r32(fp.wrapping_add(80)) as i32) as u32))
                    {
                        144
                    } else {
                        145
                    };
                }
                143 => {
                    bb = 142;
                }
                144 => {
                    bb = 136;
                }
                145 => {
                    v52 = (self.r32(
                        (v41 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v51))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    v53 = 0_i8;
                    bb = if ((self.r32(fp.wrapping_add(108)) as i32) != 0) {
                        147
                    } else {
                        149
                    };
                }
                146 => {
                    bb = 145;
                }
                147 => {
                    bb = if ((self.r32(fp.wrapping_add(108)) as i32) == 1_i32) {
                        150
                    } else {
                        151
                    };
                }
                148 => {
                    self.w32(
                        (v41.wrapping_add(7400_i32) as u32),
                        ((self.r32((v41.wrapping_add(7400_i32) as u32)) as i32).wrapping_add(1)
                            as u32),
                    );
                    bb = if ((v53 as i32) == 1_i32) { 158 } else { 159 };
                }
                149 => {
                    self.w32(
                        (v41.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v41.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v51 as i32) as u32),
                    );
                    bb = if ((v52 != (1_i32).wrapping_neg())
                        && (v52.wrapping_sub(5_i32) == (self.r32(fp.wrapping_add(96)) as i32)))
                    {
                        156
                    } else {
                        157
                    };
                }
                150 => {
                    self.w32(
                        (v41.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v41.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v51.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v52 > 4_i32) { 152 } else { 153 };
                }
                151 => {
                    bb = 148;
                }
                152 => {
                    bb = 154;
                }
                153 => {
                    bb = 151;
                }
                154 => {
                    v53 = 1_i8;
                    bb = 157;
                }
                155 => {
                    bb = 153;
                }
                156 => {
                    self.w32(
                        (v41.wrapping_add(
                            4_i32.wrapping_mul(
                                (self.r32((v41.wrapping_add(7400_i32) as u32)) as i32),
                            ),
                        )
                        .wrapping_add(6800_i32) as u32),
                        ((v51.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = 154;
                }
                157 => {
                    bb = 148;
                }
                158 => {
                    let _ = self.f_10008fc0(
                        self.r32(fp.wrapping_add(76)),
                        (self.r32(fp.wrapping_add(92)) as i32),
                        v52.wrapping_sub(5_i32),
                        fp.wrapping_add(112),
                        (fp.wrapping_add(132) as i32),
                    );
                    bb = 159;
                }
                159 => {
                    bb = 136;
                }
                160 => {
                    v48 = (self.r32(fp.wrapping_add(36)) as i32);
                    bb = 126;
                }
                161 => {
                    bb = 124;
                }
                162 => {
                    bb = 161;
                }
                163 => {
                    v55 = (self.r32(fp.wrapping_add(104)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(52)) as i32));
                    v56 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(56)) as i32));
                    v57 = (self.r32(fp.wrapping_add(104)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(52)) as i32))
                        .wrapping_abs();
                    v58 = (self.r32(fp.wrapping_add(60)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(56)) as i32))
                        .wrapping_abs();
                    bb = if (v58 != v57) { 166 } else { 167 };
                }
                164 => {
                    v62 = self.r32(fp.wrapping_add(84));
                    self.w32(
                        self.r32(fp.wrapping_add(84)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(56)) as i32) as u32),
                    );
                    self.w32(
                        v62.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(52)) as i32) as u32),
                    );
                    v63 = (self.r32(fp.wrapping_add(80)) as i32);
                    self.w32(v62.wrapping_add(1540), 1_u32);
                    self.w32(v62.wrapping_add(1552), (v63 as u32));
                    self.w32(v62.wrapping_add(1556), (v54 as u32));
                    bb = 32;
                }
                165 => {
                    let t3 = v48;
                    bb = match t3 {
                        1_i32 => 196,
                        2_i32 => 197,
                        4_i32 => 198,
                        8_i32 => 199,
                        _ => 200,
                    };
                }
                166 => {
                    bb = if (v58 >= v57) { 168 } else { 170 };
                }
                167 => {
                    bb = if ((self.r32(fp.wrapping_add(100)) as i32) != 0) {
                        173
                    } else {
                        175
                    };
                }
                168 => {
                    v54 = (if (v56 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 169;
                }
                169 => {
                    bb = 171;
                }
                170 => {
                    v59 = ((v55 >= 0_i32) as i32).wrapping_sub(1_i32);
                    v59 = (((((v59 as u32) & 0xFFFFFF00)
                        | (((((v59 & 254_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v54 = v59.wrapping_add(2_i32);
                    bb = 169;
                }
                171 => {
                    bb = if (v54 == (self.r32(fp.wrapping_add(100)) as i32)) {
                        206
                    } else {
                        207
                    };
                }
                172 => {
                    bb = 167;
                }
                173 => {
                    bb = if ((self.r32(fp.wrapping_add(100)) as i32) == 1_i32) {
                        176
                    } else {
                        178
                    };
                }
                174 => {
                    bb = 164;
                }
                175 => {
                    bb = if (v55 <= 0_i32) { 192 } else { 193 };
                }
                176 => {
                    bb = if (v56 >= 0_i32) { 179 } else { 180 };
                }
                177 => {
                    bb = 174;
                }
                178 => {
                    bb = if ((self.r32(fp.wrapping_add(100)) as i32) == 2_i32) {
                        183
                    } else {
                        185
                    };
                }
                179 => {
                    bb = 181;
                }
                180 => {
                    v60 = ((v55 >= 0_i32) as i32).wrapping_sub(1_i32);
                    v60 = (((((v60 as u32) & 0xFFFFFF00)
                        | (((((v60 & 254_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v54 = v60.wrapping_add(2_i32);
                    bb = 177;
                }
                181 => {
                    v54 = (1_i32).wrapping_neg();
                    bb = 207;
                }
                182 => {
                    bb = 180;
                }
                183 => {
                    bb = if (v55 >= 0_i32) { 186 } else { 187 };
                }
                184 => {
                    bb = 177;
                }
                185 => {
                    bb = if (((self.r32(fp.wrapping_add(100)) as i32) != 3_i32) || (v56 <= 0_i32)) {
                        189
                    } else {
                        190
                    };
                }
                186 => {
                    bb = 181;
                }
                187 => {
                    v54 = (if (v56 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 184;
                }
                188 => {
                    bb = 187;
                }
                189 => {
                    bb = 181;
                }
                190 => {
                    v61 = ((v55 >= 0_i32) as i32).wrapping_sub(1_i32);
                    v61 = (((((v61 as u32) & 0xFFFFFF00)
                        | (((((v61 & 254_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v54 = v61.wrapping_add(2_i32);
                    bb = 184;
                }
                191 => {
                    bb = 190;
                }
                192 => {
                    bb = 181;
                }
                193 => {
                    v54 = (if (v56 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 174;
                }
                194 => {
                    bb = 193;
                }
                195 => {
                    bb = 171;
                }
                196 => {
                    v54 = 0_i32;
                    bb = 195;
                }
                197 => {
                    v54 = 1_i32;
                    bb = 195;
                }
                198 => {
                    v54 = 2_i32;
                    bb = 195;
                }
                199 => {
                    v54 = 3_i32;
                    bb = 195;
                }
                200 => {
                    v54 = (self.r32(fp.wrapping_add(92)) as i32);
                    bb = 195;
                }
                201 => {
                    bb = 197;
                }
                202 => {
                    bb = 198;
                }
                203 => {
                    bb = 199;
                }
                204 => {
                    bb = 200;
                }
                205 => {
                    bb = 195;
                }
                206 => {
                    bb = 181;
                }
                207 => {
                    bb = 164;
                }
                208 => {
                    self.w32(v62.wrapping_add(1540), 2_u32);
                    return 0_i32;
                }
                209 => {
                    bb = if (!(a3 != 0)) { 211 } else { 212 };
                }
                210 => {
                    bb = 209;
                }
                211 => {
                    self.w32(v62.wrapping_add(1540), 1_u32);
                    bb = 212;
                }
                212 => {
                    return 0_i32;
                }
                213 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1000CF80` (137 bytes).
    pub(crate) fn f_1000cf80(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_1000cf80_body(fp, this);
        self.leave(16);
        r
    }

    fn f_1000cf80_body(&mut self, fp: u32, mut this: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        v2 = self.r32(((self.r32(self.r32(this)) as i32).wrapping_add(28_i32) as u32));
        let _ = self.f_10006420(this);
        v3 = (self.r32(v2.wrapping_add(1404)) as i32);
        if (v3 != 0) {
            if (v3 == 1_i32) {
                let _ = self.f_1000c630(this, 0_i8, 0_i8);
                self.w32(v2.wrapping_add(1404), 2_u32);
                return 0_i32;
            } else {
                if (v3 == 2_i32) {
                    self.w32(v2.wrapping_add(1404), 99_u32);
                }
                return 0_i32;
            }
        } else {
            self.w32(v2.wrapping_add(1440), 1_u32);
            let _ = self.f_1000d010(this, fp.wrapping_add(0));
            if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                self.w32(v2.wrapping_add(1444), 1_u32);
                self.w32(v2.wrapping_add(1048), 1_u32);
            }
            self.w32(v2.wrapping_add(1404), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000D010` (3741 bytes).
    pub(crate) fn f_1000d010(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1952);
        let r = self.f_1000d010_body(fp, this, a2);
        self.leave(1952);
        r
    }

    fn f_1000d010_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: u32 = 0;
        let mut v6: i32 = 0;
        let mut v7: u32 = 0;
        let mut i: u32 = 0;
        let mut v9: bool = false;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: i32 = 0;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: u32 = 0;
        let mut v20: i32 = 0;
        let mut v21: u32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: bool = false;
        let mut v25: i32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        let mut v29: i32 = 0;
        let mut v30: u32 = 0;
        let mut v31: u32 = 0;
        let mut v32: u32 = 0;
        let mut v33: i32 = 0;
        let mut j: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: u32 = 0;
        let mut v37: u32 = 0;
        let mut v38: i32 = 0;
        let mut v39: u32 = 0;
        let mut v40: i32 = 0;
        let mut v41: i32 = 0;
        let mut v42: u32 = 0;
        let mut v43: u32 = 0;
        let mut v44: i32 = 0;
        let mut v45: i32 = 0;
        let mut v46: u32 = 0;
        let mut v47: i64 = 0;
        let mut v48: i32 = 0;
        let mut v49: i32 = 0;
        let mut v50: i32 = 0;
        let mut v51: i32 = 0;
        let mut v52: i32 = 0;
        let mut v53: i32 = 0;
        let mut v54: u32 = 0;
        let mut v55: u32 = 0;
        let mut v56: i32 = 0;
        let mut v57: u32 = 0;
        let mut v58: u32 = 0;
        let mut v59: u32 = 0;
        let mut v60: i32 = 0;
        let mut v61: i32 = 0;
        let mut v62: i32 = 0;
        let mut v63: u32 = 0;
        let mut v64: i32 = 0;
        let mut v65: i32 = 0;
        let mut v66: i32 = 0;
        let mut v67: i32 = 0;
        let mut v68: i32 = 0;
        let mut v69: u32 = 0;
        let mut v70: u32 = 0;
        let mut v71: i32 = 0;
        let mut v72: i32 = 0;
        let mut v73: i32 = 0;
        let mut v74: u32 = 0;
        let mut v75: u32 = 0;
        let mut v76: i8 = 0;
        let mut v77: i32 = 0;
        let mut v78: i32 = 0;
        let mut v79: i32 = 0;
        let mut v80: i32 = 0;
        let mut v81: i32 = 0;
        let mut v82: i32 = 0;
        let mut v83: i32 = 0;
        let mut v84: u32 = 0;
        let mut v86: u32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = ((self.r32(self.r32(this)) as i32) as u32);
                    self.w32(fp.wrapping_add(85), this);
                    v3 = (self.r32(v2.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(69),
                        ((self.r32(v2.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(105),
                        ((self.r32(v2.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(89),
                        self.r32(fp.wrapping_add(69)).wrapping_add(4840),
                    );
                    self.w32(
                        fp.wrapping_add(81),
                        ((self.r32(self.r32(fp.wrapping_add(105)).wrapping_add(1384)) as i32)
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(109),
                        ((self.r32(self.r32(fp.wrapping_add(105)).wrapping_add(1388)) as i32)
                            as u32),
                    );
                    self.w32(fp.wrapping_add(41), (v3 as u32));
                    self.w32(fp.wrapping_add(49), 20_u32);
                    self.w32(
                        fp.wrapping_add(5),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(109)) as i32)),
                            )
                            .wrapping_add(380_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(33),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(109)) as i32)),
                            )
                            .wrapping_add(384_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(61),
                        (v3.wrapping_add(
                            716_i32.wrapping_mul((self.r32(fp.wrapping_add(109)) as i32)),
                        ) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(101),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(61)) as i32).wrapping_add(388_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(77), self.r32(this.wrapping_add(236)));
                    self.w32(
                        fp.wrapping_add(65),
                        ((self.r32(self.r32(fp.wrapping_add(77))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(21),
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(1),
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(25),
                        self.r32(fp.wrapping_add(77)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(9),
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(12)) as i32) as u32),
                    );
                    let _ = self.memset(fp.wrapping_add(345), 0_i32 as u8, 1600_u32);
                    self.w32(fp.wrapping_add(29), (v3.wrapping_add(3600_i32) as u32));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(self.r32(fp.wrapping_add(29))) as i32) == 1_i32)
                        && ((self.r32(self.r32(fp.wrapping_add(29)).wrapping_add(116)) as i32)
                            != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (!v9) { 1 } else { 3 };
                }
                3 => {
                    self.w32(fp.wrapping_add(9), 9999_u32);
                    self.w32(fp.wrapping_add(1), 9999_u32);
                    self.w32(fp.wrapping_add(37), fp.wrapping_add(345));
                    v10 = 0_i32;
                    self.w32(fp.wrapping_add(25), 1_u32);
                    self.w32(fp.wrapping_add(13), 0_u32);
                    self.w32(
                        fp.wrapping_add(93),
                        (((self
                            .r32(fp.wrapping_add(69))
                            .wrapping_add(3200)
                            .wrapping_sub(fp.wrapping_add(345))) as i32)
                            as u32),
                    );
                    bb = 17;
                }
                4 => {
                    v4 = self.r32(fp.wrapping_add(25));
                    v5 = ((self.r32(fp.wrapping_add(65)) as i32).wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(29)).wrapping_add(124)) as i32),
                    ) as u32);
                    self.w32(fp.wrapping_add(73), v5);
                    self.w32(
                        fp.wrapping_add(97),
                        ((self.r32(fp.wrapping_add(21)) as i32).wrapping_add(
                            (self.r32(self.r32(fp.wrapping_add(29)).wrapping_add(128)) as i32),
                        ) as u32),
                    );
                    v6 = (v5.wrapping_add(((self.r32(fp.wrapping_add(1)) as i32) as u32)) as i32);
                    bb = if ((self.r32(fp.wrapping_add(97)) as i32)
                        < (self.r32(fp.wrapping_add(97)) as i32)
                            .wrapping_add((self.r32(fp.wrapping_add(9)) as i32)))
                    {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v9 = ((self.r32(fp.wrapping_add(49)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(29),
                        self.r32(fp.wrapping_add(29)).wrapping_add(156),
                    );
                    self.w32(
                        fp.wrapping_add(49),
                        ((self.r32(fp.wrapping_add(49)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v7 = (20_i32.wrapping_mul((self.r32(fp.wrapping_add(97)) as i32)) as u32);
                    self.w32(
                        fp.wrapping_add(57),
                        ((self.r32(fp.wrapping_add(9)) as i32) as u32),
                    );
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    i = v5;
                    bb = 11;
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(57)) as i32) != 0) {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = 7;
                }
                11 => {
                    bb = if ((i as i32) < v6) { 12 } else { 14 };
                }
                12 => {
                    bb = if ((((self.r32(v4) as i32) != 0) && (v7 < 400_u32)) && (i < 20_u32)) {
                        15
                    } else {
                        16
                    };
                }
                13 => {
                    i = i.wrapping_add(1);
                    bb = 11;
                }
                14 => {
                    v7 = v7.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(57),
                        ((self.r32(fp.wrapping_add(57)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 9;
                }
                15 => {
                    self.w32(
                        fp.wrapping_add(345)
                            .wrapping_add(((v7.wrapping_add(i) as i32) as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(345)
                                .wrapping_add(((v7.wrapping_add(i) as i32) as u32).wrapping_mul(4)),
                        ) as i32)
                            .wrapping_add((self.r32(v4) as i32)) as u32),
                    );
                    bb = 16;
                }
                16 => {
                    v4 = v4.wrapping_add(4);
                    bb = 13;
                }
                17 => {
                    bb = if ((self.r32(
                        self.r32(fp.wrapping_add(37))
                            .wrapping_add(((self.r32(fp.wrapping_add(93)) as i32) as u32)),
                    ) as i32)
                        != (1_i32).wrapping_neg())
                    {
                        20
                    } else {
                        21
                    };
                }
                18 => {
                    bb = if (v10 < 400_i32) { 17 } else { 19 };
                }
                19 => {
                    v15 = crem_i32((self.r32(fp.wrapping_add(13)) as i32), 20_i32);
                    v16 = 0_i32;
                    v17 = cdiv_i32((self.r32(fp.wrapping_add(13)) as i32), 20_i32);
                    v18 = (self.r32(self.r32(fp.wrapping_add(69)).wrapping_add(6440)) as i32);
                    self.w32(
                        fp.wrapping_add(29),
                        (crem_i32((self.r32(fp.wrapping_add(13)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(37),
                        (cdiv_i32((self.r32(fp.wrapping_add(13)) as i32), 20_i32) as u32),
                    );
                    self.w32(fp.wrapping_add(21), 0_u32);
                    bb = if (v18 > 0_i32) { 37 } else { 38 };
                }
                20 => {
                    bb = 22;
                }
                21 => {
                    v11 = (crem_i32(v10, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(5)) as i32))
                        .wrapping_abs() as u32);
                    v12 = (cdiv_i32(v10, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(33)) as i32))
                        .wrapping_abs() as u32);
                    v13 = (v12.wrapping_add(v11) as i32);
                    v14 = (v11.wrapping_sub(v12) as i32).wrapping_abs();
                    bb = if ((self.r32(fp.wrapping_add(25)) as i32)
                        < (self.r32(self.r32(fp.wrapping_add(37))) as i32))
                    {
                        24
                    } else {
                        25
                    };
                }
                22 => {
                    v10 = v10.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(37),
                        self.r32(fp.wrapping_add(37)).wrapping_add(4),
                    );
                    bb = 18;
                }
                23 => {
                    bb = 21;
                }
                24 => {
                    self.w32(
                        fp.wrapping_add(25),
                        ((self.r32(self.r32(fp.wrapping_add(37))) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(9), ((v12.wrapping_add(v11) as i32) as u32));
                    bb = 26;
                }
                25 => {
                    bb = if ((self.r32(fp.wrapping_add(25)) as i32)
                        != (self.r32(self.r32(fp.wrapping_add(37))) as i32))
                    {
                        28
                    } else {
                        29
                    };
                }
                26 => {
                    self.w32(fp.wrapping_add(1), (v14 as u32));
                    self.w32(fp.wrapping_add(13), (v10 as u32));
                    bb = 22;
                }
                27 => {
                    bb = 25;
                }
                28 => {
                    bb = 22;
                }
                29 => {
                    bb = if (v13 < (self.r32(fp.wrapping_add(9)) as i32)) {
                        31
                    } else {
                        32
                    };
                }
                30 => {
                    bb = 29;
                }
                31 => {
                    self.w32(fp.wrapping_add(9), ((v12.wrapping_add(v11) as i32) as u32));
                    bb = 26;
                }
                32 => {
                    bb = if ((v13 == (self.r32(fp.wrapping_add(9)) as i32))
                        && (v14 < (self.r32(fp.wrapping_add(1)) as i32)))
                    {
                        34
                    } else {
                        35
                    };
                }
                33 => {
                    bb = 32;
                }
                34 => {
                    bb = 26;
                }
                35 => {
                    bb = 22;
                }
                36 => {
                    bb = 35;
                }
                37 => {
                    let _ = self.memcpy(
                        fp.wrapping_add(345),
                        self.r32(fp.wrapping_add(89)),
                        (4_i32.wrapping_mul(v18) as u32),
                    );
                    v16 = v18;
                    self.w32(fp.wrapping_add(21), (v18 as u32));
                    bb = 38;
                }
                38 => {
                    self.w32(fp.wrapping_add(9), 9999_u32);
                    self.w32(fp.wrapping_add(1), 0_u32);
                    self.w32(fp.wrapping_add(13), 0_u32);
                    self.w32(fp.wrapping_add(17), 0_u32);
                    bb = if (v16 > 0_i32) { 39 } else { 40 };
                }
                39 => {
                    self.w32(fp.wrapping_add(57), fp.wrapping_add(345));
                    bb = 41;
                }
                40 => {
                    self.w8(fp.wrapping_add(0), (0_i32 != 0) as u8);
                    self.w32(
                        fp.wrapping_add(57),
                        ((self.r32(fp.wrapping_add(345).wrapping_add(
                            ((self.r32(fp.wrapping_add(13)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(13),
                        (crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(45),
                        (cdiv_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32) as u32),
                    );
                    bb = if (v15 == crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32)) {
                        49
                    } else {
                        50
                    };
                }
                41 => {
                    v19 = ((self.r32(fp.wrapping_add(29)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(57))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs() as u32);
                    v20 = cdiv_i32(
                        (self.r32(self.r32(fp.wrapping_add(57))) as i32),
                        (20_i32).wrapping_neg(),
                    );
                    v21 = ((self.r32(fp.wrapping_add(37)) as i32)
                        .wrapping_add(v20)
                        .wrapping_abs() as u32);
                    v22 = (v19.wrapping_sub(v21) as i32).wrapping_abs();
                    v23 = (((self.r32(fp.wrapping_add(5)) as i32)
                        .wrapping_sub(crem_i32(
                            (self.r32(self.r32(fp.wrapping_add(57))) as i32),
                            20_i32,
                        ))
                        .wrapping_abs()
                        .wrapping_add(
                            v20.wrapping_add((self.r32(fp.wrapping_add(33)) as i32))
                                .wrapping_abs(),
                        ) as u32)
                        .wrapping_add(6_u32.wrapping_mul(v21.wrapping_add(v19)))
                        as i32);
                    bb = if (v23 >= (self.r32(fp.wrapping_add(9)) as i32)) {
                        44
                    } else {
                        46
                    };
                }
                42 => {
                    bb = if v24 { 41 } else { 43 };
                }
                43 => {
                    v17 = (self.r32(fp.wrapping_add(37)) as i32);
                    v15 = (self.r32(fp.wrapping_add(29)) as i32);
                    bb = 40;
                }
                44 => {
                    bb = if ((v23 == (self.r32(fp.wrapping_add(9)) as i32))
                        && (v22 < (self.r32(fp.wrapping_add(1)) as i32)))
                    {
                        47
                    } else {
                        48
                    };
                }
                45 => {
                    v24 = ({
                        let t1 = (self.r32(fp.wrapping_add(17)) as i32).wrapping_add(1);
                        self.w32(fp.wrapping_add(17), (t1 as u32));
                        t1
                    } < (self.r32(fp.wrapping_add(21)) as i32));
                    self.w32(
                        fp.wrapping_add(57),
                        self.r32(fp.wrapping_add(57)).wrapping_add(4),
                    );
                    bb = 42;
                }
                46 => {
                    self.w32(fp.wrapping_add(9), (v23 as u32));
                    self.w32(fp.wrapping_add(1), (v22 as u32));
                    self.w32(
                        fp.wrapping_add(13),
                        ((self.r32(fp.wrapping_add(17)) as i32) as u32),
                    );
                    bb = 45;
                }
                47 => {
                    self.w32(fp.wrapping_add(1), (v22 as u32));
                    self.w32(
                        fp.wrapping_add(13),
                        ((self.r32(fp.wrapping_add(17)) as i32) as u32),
                    );
                    bb = 48;
                }
                48 => {
                    bb = 45;
                }
                49 => {
                    self.w8(
                        fp.wrapping_add(0),
                        (v17 == cdiv_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32)) as u8,
                    );
                    bb = 50;
                }
                50 => {
                    v25 = (self
                        .r32(((self.r32(fp.wrapping_add(61)) as i32).wrapping_add(368_i32) as u32))
                        as i32);
                    bb = if (v15
                        .wrapping_sub((self.r32(fp.wrapping_add(5)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            v17.wrapping_sub((self.r32(fp.wrapping_add(33)) as i32))
                                .wrapping_abs(),
                        )
                        <= v25)
                    {
                        51
                    } else {
                        52
                    };
                }
                51 => {
                    self.w8(fp.wrapping_add(0), (1_i32 != 0) as u8);
                    bb = 52;
                }
                52 => {
                    bb = if (((self.r32(fp.wrapping_add(5)) as i32)
                        == crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32))
                        && (((self.r32(fp.wrapping_add(33)) as i32) as u32)
                            == self.r32(fp.wrapping_add(45))))
                    {
                        53
                    } else {
                        54
                    };
                }
                53 => {
                    self.w8(fp.wrapping_add(0), (1_i32 != 0) as u8);
                    bb = 54;
                }
                54 => {
                    v26 = ((v17 as u32).wrapping_sub(self.r32(fp.wrapping_add(45))) as i32);
                    v27 = v15.wrapping_sub((self.r32(fp.wrapping_add(13)) as i32));
                    bb = if (v15
                        .wrapping_sub((self.r32(fp.wrapping_add(13)) as i32))
                        .wrapping_abs()
                        .wrapping_add(
                            ((v17 as u32).wrapping_sub(self.r32(fp.wrapping_add(45))) as i32)
                                .wrapping_abs(),
                        )
                        > v25)
                    {
                        55
                    } else {
                        57
                    };
                }
                55 => {
                    v32 = ((self.r32(fp.wrapping_add(97)) as i32) as u32);
                    v31 = self.r32(fp.wrapping_add(73));
                    bb = 56;
                }
                56 => {
                    v33 = cdiv_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32);
                    j = 0_i32;
                    bb = 83;
                }
                57 => {
                    bb = if (v27 >= 0_i32) { 58 } else { 60 };
                }
                58 => {
                    bb = if (v27 <= 0_i32) { 61 } else { 63 };
                }
                59 => {
                    bb = if (((v30 < 20_u32) && (self.r32(fp.wrapping_add(45)) < 20_u32))
                        && ((self.r32(
                            self.r32(fp.wrapping_add(69)).wrapping_add(
                                ((20_u32
                                    .wrapping_mul(self.r32(fp.wrapping_add(45)))
                                    .wrapping_add(800_u32)
                                    .wrapping_add(v30) as i32)
                                    as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            > 4_i32))
                    {
                        73
                    } else {
                        74
                    };
                }
                60 => {
                    bb = if (v26 >= 0_i32) { 70 } else { 72 };
                }
                61 => {
                    v29 = crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32);
                    bb = if (v26 >= 0_i32) { 64 } else { 66 };
                }
                62 => {
                    bb = 59;
                }
                63 => {
                    v29 = crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32);
                    bb = if (v26 >= 0_i32) { 67 } else { 69 };
                }
                64 => {
                    v28 = ((v26 > 0_i32) as i32);
                    bb = 65;
                }
                65 => {
                    v30 = (crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32) as u32);
                    bb = 62;
                }
                66 => {
                    v28 = (1_i32).wrapping_neg();
                    bb = 65;
                }
                67 => {
                    v28 = ((v26 > 0_i32) as i32);
                    bb = 68;
                }
                68 => {
                    v30 = ((self.r32(fp.wrapping_add(13)) as i32).wrapping_add(1_i32) as u32);
                    bb = 62;
                }
                69 => {
                    v28 = (1_i32).wrapping_neg();
                    bb = 68;
                }
                70 => {
                    v28 = ((v26 > 0_i32) as i32);
                    bb = 71;
                }
                71 => {
                    v29 = crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32);
                    v30 = ((self.r32(fp.wrapping_add(13)) as i32).wrapping_sub(1_i32) as u32);
                    bb = 59;
                }
                72 => {
                    v28 = (1_i32).wrapping_neg();
                    bb = 71;
                }
                73 => {
                    self.w8(fp.wrapping_add(0), (1_i32 != 0) as u8);
                    bb = 74;
                }
                74 => {
                    bb = if (v28 == (1_i32).wrapping_neg()) {
                        75
                    } else {
                        77
                    };
                }
                75 => {
                    v31 = (v29 as u32);
                    v32 = self.r32(fp.wrapping_add(45)).wrapping_sub(1_u32);
                    bb = 76;
                }
                76 => {
                    bb = if (((v31 < 20_u32) && (v32 < 20_u32))
                        && ((self.r32(
                            self.r32(fp.wrapping_add(69)).wrapping_add(
                                ((20_u32
                                    .wrapping_mul(v32)
                                    .wrapping_add(800_u32)
                                    .wrapping_add(v31) as i32)
                                    as u32)
                                    .wrapping_mul(4),
                            ),
                        ) as i32)
                            > 4_i32))
                    {
                        81
                    } else {
                        82
                    };
                }
                77 => {
                    v31 = (v29 as u32);
                    bb = if (v28 != 0) { 78 } else { 80 };
                }
                78 => {
                    v32 = self.r32(fp.wrapping_add(45)).wrapping_add(1_u32);
                    bb = 79;
                }
                79 => {
                    bb = 76;
                }
                80 => {
                    v32 = (cdiv_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32) as u32);
                    bb = 79;
                }
                81 => {
                    self.w8(fp.wrapping_add(0), (1_i32 != 0) as u8);
                    bb = 82;
                }
                82 => {
                    bb = 56;
                }
                83 => {
                    bb = if (j < 4_i32) { 84 } else { 86 };
                }
                84 => {
                    let t2 = j;
                    bb = match t2 {
                        0_i32 => 88,
                        1_i32 => 89,
                        2_i32 => 90,
                        3_i32 => 91,
                        _ => 92,
                    };
                }
                85 => {
                    j = j.wrapping_add(1);
                    bb = 83;
                }
                86 => {
                    v35 = (self.r32(fp.wrapping_add(109)) as i32);
                    v36 = self.r32(fp.wrapping_add(85));
                    self.w8(a2, (self.r8(fp.wrapping_add(0)) != 0) as u8);
                    let _ = self.f_100064e0(
                        v36,
                        fp.wrapping_add(113),
                        (fp.wrapping_add(133) as i32),
                        (self.r32(fp.wrapping_add(81)) as i32),
                        v35,
                        0_i32,
                    );
                    self.w32(
                        fp.wrapping_add(81),
                        ((self.r32(fp.wrapping_add(117)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(97),
                        ((self.r32(fp.wrapping_add(121)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(61),
                        ((self.r32(fp.wrapping_add(125)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(93),
                        ((self.r32(fp.wrapping_add(129)) as i32) as u32),
                    );
                    let _ = self.f_10006940(
                        v36,
                        fp.wrapping_add(77),
                        (self.r32(fp.wrapping_add(117)) as i32),
                        (self.r32(fp.wrapping_add(121)) as i32),
                        (self.r32(fp.wrapping_add(125)) as i32),
                        (self.r32(fp.wrapping_add(129)) as i32),
                    );
                    self.w32(fp.wrapping_add(37), 0_u32);
                    self.w32(fp.wrapping_add(17), 0_u32);
                    self.w32(
                        fp.wrapping_add(65),
                        ((self.r32(self.r32(fp.wrapping_add(77))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(21),
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(1),
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(25),
                        self.r32(fp.wrapping_add(77)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(9),
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(12)) as i32) as u32),
                    );
                    v37 = ((self.r32(fp.wrapping_add(41)) as i32).wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(29), fp.wrapping_add(245));
                    self.w32(fp.wrapping_add(73), fp.wrapping_add(145));
                    bb = 103;
                }
                87 => {
                    bb = if ((v31 < 20_u32) && (v32 < 20_u32)) {
                        99
                    } else {
                        100
                    };
                }
                88 => {
                    v31 = (crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32) as u32);
                    v32 = (v33.wrapping_sub(1_i32) as u32);
                    bb = 87;
                }
                89 => {
                    v31 = ((self.r32(fp.wrapping_add(13)) as i32).wrapping_add(1_i32) as u32);
                    bb = 94;
                }
                90 => {
                    v31 = (crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32) as u32);
                    v32 = (v33.wrapping_add(1_i32) as u32);
                    bb = 87;
                }
                91 => {
                    v31 = ((self.r32(fp.wrapping_add(13)) as i32).wrapping_sub(1_i32) as u32);
                    bb = 94;
                }
                92 => {
                    bb = 87;
                }
                93 => {
                    bb = 89;
                }
                94 => {
                    v32 = (v33 as u32);
                    bb = 87;
                }
                95 => {
                    bb = 90;
                }
                96 => {
                    bb = 91;
                }
                97 => {
                    bb = 92;
                }
                98 => {
                    bb = 87;
                }
                99 => {
                    bb = if ((self.r32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((20_u32
                                .wrapping_mul(v32)
                                .wrapping_add(800_u32)
                                .wrapping_add(v31) as i32) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32)
                        > 4_i32)
                    {
                        101
                    } else {
                        102
                    };
                }
                100 => {
                    bb = 85;
                }
                101 => {
                    self.w8(fp.wrapping_add(0), (1_i32 != 0) as u8);
                    bb = 102;
                }
                102 => {
                    v33 = cdiv_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32);
                    bb = 100;
                }
                103 => {
                    bb = if (((self.r32(v37) as i32) == 1_i32)
                        && ((self.r32(v37.wrapping_add(116)) as i32) != 99_i32))
                    {
                        106
                    } else {
                        107
                    };
                }
                104 => {
                    bb = if ((self.r32(fp.wrapping_add(17)) as i32) < 20_i32) {
                        103
                    } else {
                        105
                    };
                }
                105 => {
                    bb = if ((self.r32(fp.wrapping_add(37)) as i32) <= 0_i32) {
                        128
                    } else {
                        129
                    };
                }
                106 => {
                    v38 = (self.r32(fp.wrapping_add(65)) as i32)
                        .wrapping_add((self.r32(v37.wrapping_add(124)) as i32));
                    v39 = self.r32(fp.wrapping_add(25));
                    v40 = (self.r32(fp.wrapping_add(21)) as i32)
                        .wrapping_add((self.r32(v37.wrapping_add(128)) as i32));
                    v41 = v38.wrapping_add((self.r32(fp.wrapping_add(1)) as i32));
                    self.w8(fp.wrapping_add(0), 0_u8);
                    self.w8(fp.wrapping_add(56), 0_u8);
                    v42 = (v40 as u32);
                    bb = if (v40 < v40.wrapping_add((self.r32(fp.wrapping_add(9)) as i32))) {
                        108
                    } else {
                        109
                    };
                }
                107 => {
                    v37 = v37.wrapping_add(156);
                    self.w32(
                        fp.wrapping_add(17),
                        ((self.r32(fp.wrapping_add(17)) as i32).wrapping_add(1) as u32),
                    );
                    bb = 104;
                }
                108 => {
                    bb = 110;
                }
                109 => {
                    bb = 107;
                }
                110 => {
                    v43 = ((self.r32(fp.wrapping_add(65)) as i32)
                        .wrapping_add((self.r32(v37.wrapping_add(124)) as i32))
                        as u32);
                    bb = if (v38 < v41) { 113 } else { 114 };
                }
                111 => {
                    bb = if ((v42 as i32) < v40.wrapping_add((self.r32(fp.wrapping_add(9)) as i32)))
                    {
                        110
                    } else {
                        112
                    };
                }
                112 => {
                    bb = if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
                        124
                    } else {
                        125
                    };
                }
                113 => {
                    bb = 115;
                }
                114 => {
                    v42 = v42.wrapping_add(1);
                    bb = 111;
                }
                115 => {
                    bb = if ((self.r32(v39) as i32) != 0) {
                        118
                    } else {
                        119
                    };
                }
                116 => {
                    bb = if ((v43 as i32) < v41) { 115 } else { 117 };
                }
                117 => {
                    bb = 114;
                }
                118 => {
                    bb = if ((((v42 < 20_u32) && (v43 < 20_u32))
                        && (v43 == ((self.r32(fp.wrapping_add(13)) as i32) as u32)))
                        && (v42 == self.r32(fp.wrapping_add(45))))
                    {
                        120
                    } else {
                        121
                    };
                }
                119 => {
                    v39 = v39.wrapping_add(4);
                    v43 = v43.wrapping_add(1);
                    bb = 116;
                }
                120 => {
                    self.w8(fp.wrapping_add(0), 1_u8);
                    bb = if ((v43 == ((self.r32(fp.wrapping_add(5)) as i32) as u32))
                        && (v42 == ((self.r32(fp.wrapping_add(33)) as i32) as u32)))
                    {
                        122
                    } else {
                        123
                    };
                }
                121 => {
                    bb = 119;
                }
                122 => {
                    self.w8(fp.wrapping_add(56), 1_u8);
                    bb = 123;
                }
                123 => {
                    bb = 121;
                }
                124 => {
                    self.w32(
                        self.r32(fp.wrapping_add(73)),
                        ((self.r32(fp.wrapping_add(17)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(37),
                        ((self.r32(fp.wrapping_add(37)) as i32).wrapping_add(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(73),
                        self.r32(fp.wrapping_add(73)).wrapping_add(4),
                    );
                    bb = 125;
                }
                125 => {
                    bb = if (((self.r8(fp.wrapping_add(56)) as i8) as i32) == 1_i32) {
                        126
                    } else {
                        127
                    };
                }
                126 => {
                    {
                        let a0 = {
                            let t3 = self.r32(fp.wrapping_add(29));
                            self.w32(fp.wrapping_add(29), t3.wrapping_add(4));
                            t3
                        };
                        let a1 = ((self.r32(fp.wrapping_add(17)) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    bb = 127;
                }
                127 => {
                    bb = 109;
                }
                128 => {
                    v86 = self.r32(fp.wrapping_add(105));
                    self.w32(
                        self.r32(fp.wrapping_add(105)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(13)) as i32) as u32),
                    );
                    self.w32(v86.wrapping_add(1540), 2_u32);
                    self.w32(
                        v86.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(45)) as i32) as u32),
                    );
                    self.w32(
                        v86.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(57)) as i32) as u32),
                    );
                    self.w32(v86.wrapping_add(1556), ((1_i32).wrapping_neg() as u32));
                    return 0_i32;
                }
                129 => {
                    v44 = 0_i32;
                    v45 = 9999_i32;
                    self.w32(fp.wrapping_add(49), 0_u32);
                    v46 = fp.wrapping_add(145);
                    bb = 131;
                }
                130 => {
                    bb = 129;
                }
                131 => {
                    v47 = ((self.r32(
                        ((self.r32(fp.wrapping_add(41)) as i32)
                            .wrapping_add(156_i32.wrapping_mul((self.r32(v46) as i32)))
                            .wrapping_add(3724_i32) as u32),
                    ) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(5)) as i32))
                        as i64);
                    v48 = ((((((v47 as u64) >> 32) as u32) as i64) ^ v47)
                        .wrapping_sub(((((v47 as u64) >> 32) as u32) as i64))
                        .wrapping_add(
                            ((self.r32(
                                ((self.r32(fp.wrapping_add(41)) as i32)
                                    .wrapping_add(156_i32.wrapping_mul((self.r32(v46) as i32)))
                                    .wrapping_add(3728_i32)
                                    as u32),
                            ) as i32)
                                .wrapping_sub((self.r32(fp.wrapping_add(33)) as i32))
                                .wrapping_abs() as i64),
                        ) as i32);
                    bb = if (v48 < v45) { 134 } else { 135 };
                }
                132 => {
                    bb = if (v44 < (self.r32(fp.wrapping_add(37)) as i32)) {
                        131
                    } else {
                        133
                    };
                }
                133 => {
                    self.w32(
                        fp.wrapping_add(89),
                        ((self.r32(fp.wrapping_add(145).wrapping_add(
                            ((self.r32(fp.wrapping_add(49)) as i32) as u32).wrapping_mul(4),
                        )) as i32) as u32),
                    );
                    v49 = (self.r32(
                        ((self.r32(fp.wrapping_add(41)) as i32)
                            .wrapping_add(
                                156_i32.wrapping_mul((self.r32(fp.wrapping_add(89)) as i32)),
                            )
                            .wrapping_add(3728_i32) as u32),
                    ) as i32);
                    self.w32(
                        fp.wrapping_add(29),
                        ((self.r32(
                            ((self.r32(fp.wrapping_add(41)) as i32)
                                .wrapping_add(
                                    156_i32.wrapping_mul((self.r32(fp.wrapping_add(89)) as i32)),
                                )
                                .wrapping_add(3724_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(37), (v49 as u32));
                    bb = if ((self.r32(fp.wrapping_add(81)) as i32) <= 0_i32) {
                        136
                    } else {
                        137
                    };
                }
                134 => {
                    v45 = v48;
                    self.w32(fp.wrapping_add(49), (v44 as u32));
                    bb = 135;
                }
                135 => {
                    v44 = v44.wrapping_add(1);
                    v46 = v46.wrapping_add(4);
                    bb = 132;
                }
                136 => {
                    v61 = crem_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32);
                    v62 = cdiv_i32((self.r32(fp.wrapping_add(57)) as i32), 20_i32);
                    v64 = (self.r32(fp.wrapping_add(65)) as i32);
                    v65 = (self.r32(fp.wrapping_add(21)) as i32);
                    bb = 138;
                }
                137 => {
                    self.w32(fp.wrapping_add(25), 9999_u32);
                    self.w32(fp.wrapping_add(9), 0_u32);
                    self.w32(
                        fp.wrapping_add(77),
                        self.r32(
                            self.r32(fp.wrapping_add(85)).wrapping_add(
                                ((self.r32(fp.wrapping_add(81)) as i32).wrapping_add(3_i32) as u32)
                                    .wrapping_mul(4),
                            ),
                        ),
                    );
                    self.w32(fp.wrapping_add(1), 0_u32);
                    self.w32(
                        fp.wrapping_add(5),
                        self.r32(fp.wrapping_add(77)).wrapping_add(16),
                    );
                    v50 = (self.r32(fp.wrapping_add(45)).wrapping_add(
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(4)) as i32) as u32),
                    ) as i32);
                    v51 = (self.r32(fp.wrapping_add(13)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(77))) as i32));
                    v52 = v51.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(8)) as i32),
                    );
                    v53 = v50.wrapping_add(
                        (self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(12)) as i32),
                    );
                    self.w32(fp.wrapping_add(73), (v51 as u32));
                    self.w32(fp.wrapping_add(33), (v52 as u32));
                    self.w32(fp.wrapping_add(49), (v53 as u32));
                    bb = if ((self.r32(fp.wrapping_add(97)) as i32) != 0) {
                        140
                    } else {
                        141
                    };
                }
                138 => {
                    self.w32(fp.wrapping_add(5), self.r32(fp.wrapping_add(25)));
                    v66 = v64.wrapping_add(v61);
                    v67 = v65.wrapping_add(v62);
                    v68 = v66.wrapping_add((self.r32(fp.wrapping_add(1)) as i32));
                    self.w32(fp.wrapping_add(41), (v66 as u32));
                    self.w32(
                        fp.wrapping_add(33),
                        (v66.wrapping_add((self.r32(fp.wrapping_add(1)) as i32)) as u32),
                    );
                    bb = if (v67 < v67.wrapping_add((self.r32(fp.wrapping_add(9)) as i32))) {
                        210
                    } else {
                        211
                    };
                }
                139 => {
                    bb = 137;
                }
                140 => {
                    bb = if ((self.r32(fp.wrapping_add(97)) as i32) != 2_i32) {
                        142
                    } else {
                        143
                    };
                }
                141 => {
                    bb = if (v50 >= v53) { 175 } else { 176 };
                }
                142 => {
                    bb = 144;
                }
                143 => {
                    self.w32(
                        fp.wrapping_add(17),
                        self.r32(fp.wrapping_add(45)).wrapping_add(
                            ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(4)) as i32)
                                as u32),
                        ),
                    );
                    bb = if (v50 >= v53) { 146 } else { 147 };
                }
                144 => {
                    v61 = crem_i32((self.r32(fp.wrapping_add(1)) as i32), 20_i32);
                    v62 = cdiv_i32((self.r32(fp.wrapping_add(1)) as i32), 20_i32);
                    bb = if ((self.r32(fp.wrapping_add(97)) as i32) != 0) {
                        204
                    } else {
                        206
                    };
                }
                145 => {
                    bb = 143;
                }
                146 => {
                    bb = 144;
                }
                147 => {
                    self.w32(fp.wrapping_add(65), (20_i32.wrapping_mul(v50) as u32));
                    self.w32(fp.wrapping_add(21), (v49.wrapping_sub(v50) as u32));
                    bb = 149;
                }
                148 => {
                    bb = 147;
                }
                149 => {
                    bb = if (1_i32 != 0) { 150 } else { 151 };
                }
                150 => {
                    v57 = ((self.r32(fp.wrapping_add(13)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(77))) as i32))
                        as u32);
                    bb = if ((self.r32(fp.wrapping_add(73)) as i32) < v52) {
                        152
                    } else {
                        153
                    };
                }
                151 => {
                    self.w32(
                        fp.wrapping_add(41),
                        ((self.r32(fp.wrapping_add(29)) as i32)
                            .wrapping_sub((self.r32(fp.wrapping_add(73)) as i32))
                            as u32),
                    );
                    bb = 159;
                }
                152 => {
                    bb = 151;
                }
                153 => {
                    bb = 155;
                }
                154 => {
                    bb = 153;
                }
                155 => {
                    v24 = (({
                        let t4 = self.r32(fp.wrapping_add(17)).wrapping_add(1);
                        self.w32(fp.wrapping_add(17), t4);
                        t4
                    } as i32)
                        < (self.r32(fp.wrapping_add(49)) as i32));
                    self.w32(
                        fp.wrapping_add(65),
                        self.r32(fp.wrapping_add(65)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(21),
                        ((self.r32(fp.wrapping_add(21)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (!v24) { 156 } else { 157 };
                }
                156 => {
                    bb = 144;
                }
                157 => {
                    bb = 149;
                }
                158 => {
                    bb = 157;
                }
                159 => {
                    bb = if (1_i32 != 0) { 160 } else { 161 };
                }
                160 => {
                    bb = if ((((!((self.r32(self.r32(fp.wrapping_add(5))) as i32) != 0))
                        || (self.r32(fp.wrapping_add(65)) >= 400_u32))
                        || (v57 >= 20_u32))
                        || ((self.r32(fp.wrapping_add(17)) != (v49 as u32))
                            && (v57 != ((self.r32(fp.wrapping_add(29)) as i32) as u32))))
                    {
                        162
                    } else {
                        163
                    };
                }
                161 => {
                    bb = 141;
                }
                162 => {
                    bb = 164;
                }
                163 => {
                    v58 = ((self.r32(fp.wrapping_add(41)) as i32).wrapping_abs() as u32);
                    v59 = ((self.r32(fp.wrapping_add(21)) as i32).wrapping_abs() as u32);
                    v60 = (v58.wrapping_sub(v59) as i32).wrapping_abs();
                    bb = if ((v59.wrapping_add(v58) as i32)
                        >= (self.r32(fp.wrapping_add(25)) as i32))
                    {
                        166
                    } else {
                        168
                    };
                }
                164 => {
                    v57 = v57.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(5),
                        self.r32(fp.wrapping_add(5)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(41),
                        ((self.r32(fp.wrapping_add(41)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if ((v57 as i32) >= v52) { 172 } else { 173 };
                }
                165 => {
                    bb = 163;
                }
                166 => {
                    bb = if ((v59.wrapping_add(v58)
                        != ((self.r32(fp.wrapping_add(25)) as i32) as u32))
                        || (v60 >= (self.r32(fp.wrapping_add(9)) as i32)))
                    {
                        169
                    } else {
                        170
                    };
                }
                167 => {
                    self.w32(fp.wrapping_add(9), (v60 as u32));
                    self.w32(
                        fp.wrapping_add(1),
                        ((self.r32(fp.wrapping_add(65)).wrapping_add(v57) as i32) as u32),
                    );
                    bb = 164;
                }
                168 => {
                    self.w32(fp.wrapping_add(25), ((v59.wrapping_add(v58) as i32) as u32));
                    bb = 167;
                }
                169 => {
                    bb = 164;
                }
                170 => {
                    bb = 167;
                }
                171 => {
                    bb = 170;
                }
                172 => {
                    bb = 155;
                }
                173 => {
                    bb = 159;
                }
                174 => {
                    bb = 173;
                }
                175 => {
                    bb = 144;
                }
                176 => {
                    self.w32(fp.wrapping_add(17), (20_i32.wrapping_mul(v50) as u32));
                    self.w32(fp.wrapping_add(21), (v49.wrapping_sub(v50) as u32));
                    self.w32(
                        fp.wrapping_add(49),
                        ((self.r32(self.r32(fp.wrapping_add(77)).wrapping_add(12)) as i32) as u32),
                    );
                    bb = 178;
                }
                177 => {
                    bb = 176;
                }
                178 => {
                    bb = if (2_i32 != 0) { 179 } else { 180 };
                }
                179 => {
                    bb = if (v51 >= v52) { 181 } else { 182 };
                }
                180 => {
                    bb = 144;
                }
                181 => {
                    bb = 183;
                }
                182 => {
                    self.w32(
                        fp.wrapping_add(41),
                        ((self.r32(fp.wrapping_add(29)) as i32).wrapping_sub(v51) as u32),
                    );
                    bb = 185;
                }
                183 => {
                    v9 = ((self.r32(fp.wrapping_add(49)) as i32) == 1_i32);
                    self.w32(
                        fp.wrapping_add(17),
                        self.r32(fp.wrapping_add(17)).wrapping_add(20_u32),
                    );
                    self.w32(
                        fp.wrapping_add(21),
                        ((self.r32(fp.wrapping_add(21)) as i32).wrapping_sub(1) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(49),
                        ((self.r32(fp.wrapping_add(49)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (!v9) { 200 } else { 201 };
                }
                184 => {
                    bb = 182;
                }
                185 => {
                    bb = if (2_i32 != 0) { 186 } else { 187 };
                }
                186 => {
                    bb = if ((((self.r32(self.r32(fp.wrapping_add(5))) as i32) != 0)
                        && (self.r32(fp.wrapping_add(17)) < 400_u32))
                        && ((v51 as u32) < 20_u32))
                    {
                        188
                    } else {
                        189
                    };
                }
                187 => {
                    v51 = (self.r32(fp.wrapping_add(13)) as i32)
                        .wrapping_add((self.r32(self.r32(fp.wrapping_add(77))) as i32));
                    v52 = (self.r32(fp.wrapping_add(33)) as i32);
                    bb = 183;
                }
                188 => {
                    v54 = ((self.r32(fp.wrapping_add(41)) as i32).wrapping_abs() as u32);
                    v55 = ((self.r32(fp.wrapping_add(21)) as i32).wrapping_abs() as u32);
                    v56 = (v54.wrapping_sub(v55) as i32).wrapping_abs();
                    bb = if ((v55.wrapping_add(v54) as i32)
                        < (self.r32(fp.wrapping_add(25)) as i32))
                    {
                        190
                    } else {
                        191
                    };
                }
                189 => {
                    v51 = v51.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(5),
                        self.r32(fp.wrapping_add(5)).wrapping_add(4),
                    );
                    self.w32(
                        fp.wrapping_add(41),
                        ((self.r32(fp.wrapping_add(41)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = if (v51 < (self.r32(fp.wrapping_add(33)) as i32)) {
                        196
                    } else {
                        197
                    };
                }
                190 => {
                    self.w32(fp.wrapping_add(25), ((v55.wrapping_add(v54) as i32) as u32));
                    bb = 192;
                }
                191 => {
                    bb = if ((v55.wrapping_add(v54)
                        == ((self.r32(fp.wrapping_add(25)) as i32) as u32))
                        && (v56 < (self.r32(fp.wrapping_add(9)) as i32)))
                    {
                        194
                    } else {
                        195
                    };
                }
                192 => {
                    self.w32(fp.wrapping_add(9), (v56 as u32));
                    self.w32(
                        fp.wrapping_add(1),
                        ((self.r32(fp.wrapping_add(17)).wrapping_add((v51 as u32)) as i32) as u32),
                    );
                    bb = 195;
                }
                193 => {
                    bb = 191;
                }
                194 => {
                    bb = 192;
                }
                195 => {
                    bb = 189;
                }
                196 => {
                    bb = 185;
                }
                197 => {
                    bb = 187;
                }
                198 => {
                    bb = 197;
                }
                199 => {
                    bb = 185;
                }
                200 => {
                    bb = 178;
                }
                201 => {
                    bb = 180;
                }
                202 => {
                    bb = 201;
                }
                203 => {
                    bb = 178;
                }
                204 => {
                    bb = if ((self.r32(fp.wrapping_add(97)) as i32) == 2_i32) {
                        207
                    } else {
                        209
                    };
                }
                205 => {
                    v64 = (self.r32(v63) as i32);
                    v65 = (self.r32(v63.wrapping_add(4)) as i32);
                    self.w32(
                        fp.wrapping_add(1),
                        ((self.r32(v63.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(9),
                        ((self.r32(v63.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(25), v63.wrapping_add(16));
                    bb = 138;
                }
                206 => {
                    v63 = self.r32(
                        self.r32(fp.wrapping_add(85)).wrapping_add(
                            ((self.r32(fp.wrapping_add(61)) as i32).wrapping_add(3_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(77), v63);
                    bb = 205;
                }
                207 => {
                    v63 = self.r32(
                        self.r32(fp.wrapping_add(85)).wrapping_add(
                            ((self.r32(fp.wrapping_add(61)) as i32).wrapping_add(18_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    );
                    self.w32(fp.wrapping_add(77), v63);
                    bb = 208;
                }
                208 => {
                    bb = 205;
                }
                209 => {
                    v63 = self.r32(fp.wrapping_add(77));
                    bb = 208;
                }
                210 => {
                    v69 = (20_i32.wrapping_mul(v67) as u32);
                    self.w32(
                        fp.wrapping_add(61),
                        ((self.r32(fp.wrapping_add(9)) as i32) as u32),
                    );
                    bb = 212;
                }
                211 => {
                    v72 = ((self.r32(fp.wrapping_add(21)) as i32) & 15_i32);
                    v73 = v66.wrapping_add((self.r32(fp.wrapping_add(1)) as i32));
                    self.w32(fp.wrapping_add(81), (16_i32.wrapping_mul(v72) as u32));
                    self.w32(fp.wrapping_add(5), self.r32(fp.wrapping_add(25)));
                    self.w32(self.r32(fp.wrapping_add(69)).wrapping_add(7400), 0_u32);
                    self.w32(
                        fp.wrapping_add(33),
                        (v66.wrapping_add((self.r32(fp.wrapping_add(1)) as i32)) as u32),
                    );
                    bb = if (v67 >= v67.wrapping_add((self.r32(fp.wrapping_add(9)) as i32))) {
                        226
                    } else {
                        227
                    };
                }
                212 => {
                    v70 = (v66 as u32);
                    bb = if (v66 < v68) { 215 } else { 216 };
                }
                213 => {
                    bb = if ((self.r32(fp.wrapping_add(61)) as i32) != 0) {
                        212
                    } else {
                        214
                    };
                }
                214 => {
                    bb = 211;
                }
                215 => {
                    bb = 217;
                }
                216 => {
                    v66 = (self.r32(fp.wrapping_add(41)) as i32);
                    v69 = v69.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(61),
                        ((self.r32(fp.wrapping_add(61)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 213;
                }
                217 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(5))) as i32) != 0) {
                        220
                    } else {
                        221
                    };
                }
                218 => {
                    bb = if ((v70 as i32) < (self.r32(fp.wrapping_add(33)) as i32)) {
                        217
                    } else {
                        219
                    };
                }
                219 => {
                    bb = 216;
                }
                220 => {
                    bb = if ((v69 < 400_u32) && (v70 < 20_u32)) {
                        222
                    } else {
                        223
                    };
                }
                221 => {
                    v70 = v70.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(5),
                        self.r32(fp.wrapping_add(5)).wrapping_add(4),
                    );
                    v68 = (self.r32(fp.wrapping_add(33)) as i32);
                    bb = 218;
                }
                222 => {
                    v71 = (self.r32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((v70.wrapping_add(800_u32).wrapping_add(v69) as i32) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32);
                    bb = if ((v71 != (1_i32).wrapping_neg())
                        && ((v71.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(89))))
                    {
                        224
                    } else {
                        225
                    };
                }
                223 => {
                    bb = 221;
                }
                224 => {
                    self.w32(
                        fp.wrapping_add(21),
                        ((self.r32(self.r32(fp.wrapping_add(5))) as i32) as u32),
                    );
                    bb = 225;
                }
                225 => {
                    bb = 223;
                }
                226 => {
                    bb = 228;
                }
                227 => {
                    v74 = (20_i32.wrapping_mul(v67) as u32);
                    self.w32(
                        fp.wrapping_add(61),
                        ((self.r32(fp.wrapping_add(9)) as i32) as u32),
                    );
                    bb = 230;
                }
                228 => {
                    bb = if (v72 != 15_i32) { 265 } else { 266 };
                }
                229 => {
                    bb = 227;
                }
                230 => {
                    bb = if (2_i32 != 0) { 231 } else { 232 };
                }
                231 => {
                    bb = if (v66 >= v73) { 233 } else { 234 };
                }
                232 => {
                    bb = 228;
                }
                233 => {
                    bb = 235;
                }
                234 => {
                    bb = 237;
                }
                235 => {
                    v74 = v74.wrapping_add(20_u32);
                    bb = if ({
                        let t5 = (self.r32(fp.wrapping_add(61)) as i32).wrapping_sub(1);
                        self.w32(fp.wrapping_add(61), (t5 as u32));
                        t5
                    } != 0)
                    {
                        261
                    } else {
                        262
                    };
                }
                236 => {
                    bb = 234;
                }
                237 => {
                    bb = if (2_i32 != 0) { 238 } else { 239 };
                }
                238 => {
                    bb = if (((((self.r32(fp.wrapping_add(81)) as i32)
                        & (self.r32(self.r32(fp.wrapping_add(5))) as i32))
                        != 0_i32)
                        && (v74 < 400_u32))
                        && ((v66 as u32) < 20_u32))
                    {
                        240
                    } else {
                        241
                    };
                }
                239 => {
                    v66 = (self.r32(fp.wrapping_add(41)) as i32);
                    bb = 235;
                }
                240 => {
                    v75 = (v66 as u32).wrapping_add(v74);
                    bb = if ((v66 as u32).wrapping_add(v74)
                        != ((self.r32(fp.wrapping_add(57)) as i32) as u32))
                    {
                        242
                    } else {
                        243
                    };
                }
                241 => {
                    v73 = (self.r32(fp.wrapping_add(33)) as i32);
                    v66 = v66.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(5),
                        self.r32(fp.wrapping_add(5)).wrapping_add(4),
                    );
                    bb = if (v66 < (self.r32(fp.wrapping_add(33)) as i32)) {
                        257
                    } else {
                        258
                    };
                }
                242 => {
                    v76 = 0_i8;
                    v77 =
                        (self.r32(self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((v75.wrapping_add(800_u32) as i32) as u32).wrapping_mul(4),
                        )) as i32);
                    bb = if ((self.r32(fp.wrapping_add(93)) as i32) != 0) {
                        244
                    } else {
                        246
                    };
                }
                243 => {
                    bb = 241;
                }
                244 => {
                    bb = if ((self.r32(fp.wrapping_add(93)) as i32) == 1_i32) {
                        247
                    } else {
                        248
                    };
                }
                245 => {
                    self.w32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(7400),
                        ((self.r32(self.r32(fp.wrapping_add(69)).wrapping_add(7400)) as i32)
                            .wrapping_add(1) as u32),
                    );
                    bb = if ((v76 as i32) == 1_i32) { 255 } else { 256 };
                }
                246 => {
                    self.w32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((self.r32(self.r32(fp.wrapping_add(69)).wrapping_add(7400)) as i32)
                                .wrapping_add(1700_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        ((v75 as i32) as u32),
                    );
                    bb = if ((v77 != (1_i32).wrapping_neg())
                        && ((v77.wrapping_sub(5_i32) as u32) == self.r32(fp.wrapping_add(89))))
                    {
                        252
                    } else {
                        253
                    };
                }
                247 => {
                    self.w32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((self.r32(self.r32(fp.wrapping_add(69)).wrapping_add(7400)) as i32)
                                .wrapping_add(1700_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        ((v75.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = if (v77 > 4_i32) { 249 } else { 250 };
                }
                248 => {
                    bb = 245;
                }
                249 => {
                    bb = 251;
                }
                250 => {
                    bb = 248;
                }
                251 => {
                    v76 = 1_i8;
                    bb = 250;
                }
                252 => {
                    self.w32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((self.r32(self.r32(fp.wrapping_add(69)).wrapping_add(7400)) as i32)
                                .wrapping_add(1700_i32) as u32)
                                .wrapping_mul(4),
                        ),
                        ((v75.wrapping_add(400_u32) as i32) as u32),
                    );
                    bb = 251;
                }
                253 => {
                    bb = 245;
                }
                254 => {
                    bb = 253;
                }
                255 => {
                    let _ = self.f_10008fc0(
                        self.r32(fp.wrapping_add(85)),
                        (self.r32(fp.wrapping_add(109)) as i32),
                        v77.wrapping_sub(5_i32),
                        fp.wrapping_add(113),
                        (fp.wrapping_add(133) as i32),
                    );
                    bb = 256;
                }
                256 => {
                    bb = 243;
                }
                257 => {
                    bb = 237;
                }
                258 => {
                    bb = 239;
                }
                259 => {
                    bb = 258;
                }
                260 => {
                    bb = 237;
                }
                261 => {
                    bb = 230;
                }
                262 => {
                    bb = 232;
                }
                263 => {
                    bb = 262;
                }
                264 => {
                    bb = 230;
                }
                265 => {
                    let t6 = v72;
                    bb = match t6 {
                        1_i32 => 268,
                        2_i32 => 269,
                        4_i32 => 270,
                        8_i32 => 271,
                        _ => 272,
                    };
                }
                266 => {
                    v78 = (self.r32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((self.r32(fp.wrapping_add(57)) as i32).wrapping_add(400_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32);
                    bb = if (v78 == (1_i32).wrapping_neg()) {
                        280
                    } else {
                        281
                    };
                }
                267 => {
                    bb = 278;
                }
                268 => {
                    v78 = 0_i32;
                    bb = 267;
                }
                269 => {
                    v78 = 1_i32;
                    bb = 267;
                }
                270 => {
                    v78 = 2_i32;
                    bb = 267;
                }
                271 => {
                    v78 = 3_i32;
                    bb = 267;
                }
                272 => {
                    v78 = (self.r32(fp.wrapping_add(81)) as i32);
                    bb = 267;
                }
                273 => {
                    bb = 269;
                }
                274 => {
                    bb = 270;
                }
                275 => {
                    bb = 271;
                }
                276 => {
                    bb = 272;
                }
                277 => {
                    bb = 267;
                }
                278 => {
                    v83 = (self.r32(
                        self.r32(fp.wrapping_add(69)).wrapping_add(
                            ((self.r32(fp.wrapping_add(57)) as i32).wrapping_add(400_i32) as u32)
                                .wrapping_mul(4),
                        ),
                    ) as i32);
                    bb = if (v83 == (1_i32).wrapping_neg()) {
                        307
                    } else {
                        308
                    };
                }
                279 => {
                    bb = 266;
                }
                280 => {
                    v78 = (self.r32(fp.wrapping_add(101)) as i32);
                    bb = 281;
                }
                281 => {
                    v79 = (((self.r32(fp.wrapping_add(37)) as i32) as u32)
                        .wrapping_sub(self.r32(fp.wrapping_add(45)))
                        as i32);
                    v80 = (self.r32(fp.wrapping_add(29)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(13)) as i32));
                    v81 = (((self.r32(fp.wrapping_add(37)) as i32) as u32)
                        .wrapping_sub(self.r32(fp.wrapping_add(45)))
                        as i32)
                        .wrapping_abs();
                    v82 = (self.r32(fp.wrapping_add(29)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(13)) as i32))
                        .wrapping_abs();
                    bb = if (v82 != v81) { 282 } else { 283 };
                }
                282 => {
                    bb = if (v82 < v81) { 284 } else { 285 };
                }
                283 => {
                    bb = if (v78 != 0) { 289 } else { 291 };
                }
                284 => {
                    v78 = (if (v79 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 278;
                }
                285 => {
                    bb = 287;
                }
                286 => {
                    bb = 285;
                }
                287 => {
                    v78 = (if (v80 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 278;
                }
                288 => {
                    bb = 283;
                }
                289 => {
                    bb = if (v78 == 1_i32) { 292 } else { 294 };
                }
                290 => {
                    bb = 278;
                }
                291 => {
                    bb = if (v79 > 0_i32) { 305 } else { 306 };
                }
                292 => {
                    bb = if (v80 < 0_i32) { 295 } else { 296 };
                }
                293 => {
                    bb = 290;
                }
                294 => {
                    bb = if (v78 == 2_i32) { 297 } else { 298 };
                }
                295 => {
                    v78 = (if (v79 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 296;
                }
                296 => {
                    bb = 293;
                }
                297 => {
                    bb = if (v79 >= 0_i32) { 299 } else { 300 };
                }
                298 => {
                    bb = if ((v78 == 3_i32) && (v80 > 0_i32)) {
                        303
                    } else {
                        304
                    };
                }
                299 => {
                    bb = 278;
                }
                300 => {
                    bb = 287;
                }
                301 => {
                    bb = 300;
                }
                302 => {
                    bb = 298;
                }
                303 => {
                    v78 = (if (v79 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 304;
                }
                304 => {
                    bb = 293;
                }
                305 => {
                    v78 = (if (v80 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 306;
                }
                306 => {
                    bb = 290;
                }
                307 => {
                    v83 = (self.r32(fp.wrapping_add(101)) as i32);
                    bb = 308;
                }
                308 => {
                    bb = if (v78 == v83) { 309 } else { 310 };
                }
                309 => {
                    v78 = (1_i32).wrapping_neg();
                    bb = 310;
                }
                310 => {
                    v84 = self.r32(fp.wrapping_add(105));
                    self.w32(
                        self.r32(fp.wrapping_add(105)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(13)) as i32) as u32),
                    );
                    self.w32(
                        v84.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(45)) as i32) as u32),
                    );
                    self.w32(v84.wrapping_add(1540), 2_u32);
                    self.w32(
                        v84.wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(57)) as i32) as u32),
                    );
                    self.w32(v84.wrapping_add(1556), (v78 as u32));
                    return 0_i32;
                }
                311 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1000DEE0` (70 bytes).
    pub(crate) fn f_1000dee0(&mut self, mut this: u32) -> i32 {
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
            let _ = self.f_1000df30(this);
            self.w32((v2.wrapping_add(1404_i32) as u32), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000DF30` (1233 bytes).
    pub(crate) fn f_1000df30(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut result: i32 = 0;
        let mut v9: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v1 = (self.r32(self.r32(this)) as i32);
                    v2 = (self.r32((v1.wrapping_add(28_i32) as u32)) as i32);
                    v3 = (self.r32((v2.wrapping_add(1388_i32) as u32)) as i32);
                    v4 = (self.r32((v2.wrapping_add(1044_i32) as u32)) as i32);
                    v5 = (self.r32((v1.wrapping_add(12_i32) as u32)) as i32)
                        .wrapping_add(716_i32.wrapping_mul(v3));
                    bb = if (v4 >= 10_i32) { 1 } else { 3 };
                }
                1 => {
                    let t1 = (self.r32((v5.wrapping_add(12_i32) as u32)) as i32);
                    bb = match t1 {
                        0_i32 => 5,
                        1_i32 => 6,
                        2_i32 => 7,
                        3_i32 => 8,
                        4_i32 => 9,
                        5_i32 => 10,
                        6_i32 => 11,
                        7_i32 => 12,
                        8_i32 => 13,
                        9_i32 => 14,
                        _ => 15,
                    };
                }
                2 => {
                    let t3 = v6;
                    bb = match t3 {
                        0_i32 => 39,
                        1_i32 => 40,
                        2_i32 => 41,
                        3_i32 => 42,
                        5_i32 => 43,
                        6_i32 => 44,
                        7_i32 => 45,
                        8_i32 => 46,
                        9_i32 => 47,
                        10_i32 => 48,
                        11_i32 => 49,
                        12_i32 => 50,
                        13_i32 => 51,
                        14_i32 => 52,
                        _ => 53,
                    };
                }
                3 => {
                    let t2 = v3;
                    bb = match t2 {
                        0_i32 => 28,
                        1_i32 => 29,
                        2_i32 => 30,
                        3_i32 => 31,
                        _ => 32,
                    };
                }
                4 => {
                    v7 = v4.wrapping_sub(10_i32);
                    bb = 2;
                }
                5 => {
                    v6 = 5_i32;
                    bb = 4;
                }
                6 => {
                    v6 = 6_i32;
                    bb = 4;
                }
                7 => {
                    v6 = 7_i32;
                    bb = 4;
                }
                8 => {
                    v6 = 8_i32;
                    bb = 4;
                }
                9 => {
                    v6 = 9_i32;
                    bb = 4;
                }
                10 => {
                    v6 = 10_i32;
                    bb = 4;
                }
                11 => {
                    v6 = 11_i32;
                    bb = 4;
                }
                12 => {
                    v6 = 12_i32;
                    bb = 4;
                }
                13 => {
                    v6 = 13_i32;
                    bb = 4;
                }
                14 => {
                    v6 = 14_i32;
                    bb = 4;
                }
                15 => {
                    v6 = v9;
                    bb = 4;
                }
                16 => {
                    bb = 6;
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
                    bb = 4;
                }
                27 => {
                    bb = 2;
                }
                28 => {
                    v6 = 0_i32;
                    v7 = v4;
                    bb = 27;
                }
                29 => {
                    v6 = 1_i32;
                    v7 = v4;
                    bb = 27;
                }
                30 => {
                    v6 = 2_i32;
                    v7 = v4;
                    bb = 27;
                }
                31 => {
                    v6 = 3_i32;
                    v7 = v4;
                    bb = 27;
                }
                32 => {
                    v6 = v9;
                    v7 = v4;
                    bb = 27;
                }
                33 => {
                    bb = 29;
                }
                34 => {
                    bb = 30;
                }
                35 => {
                    bb = 31;
                }
                36 => {
                    bb = 32;
                }
                37 => {
                    bb = 27;
                }
                38 => {
                    return result;
                }
                39 => {
                    let t4 = v7;
                    bb = match t4 {
                        0_i32 => 55,
                        1_i32 => 56,
                        2_i32 => 57,
                        3_i32 => 58,
                        4_i32 => 59,
                        5_i32 => 60,
                        6_i32 => 61,
                        7_i32 => 62,
                        8_i32 => 63,
                        9_i32 => 64,
                        _ => 65,
                    };
                }
                40 => {
                    let t5 = v7;
                    bb = match t5 {
                        0_i32 => 80,
                        1_i32 => 81,
                        2_i32 => 82,
                        3_i32 => 83,
                        4_i32 => 84,
                        5_i32 => 85,
                        6_i32 => 86,
                        7_i32 => 87,
                        8_i32 => 88,
                        9_i32 => 89,
                        _ => 90,
                    };
                }
                41 => {
                    let t6 = v7;
                    bb = match t6 {
                        0_i32 => 104,
                        1_i32 => 105,
                        2_i32 => 106,
                        3_i32 => 107,
                        4_i32 => 108,
                        5_i32 => 109,
                        6_i32 => 110,
                        7_i32 => 111,
                        8_i32 => 112,
                        9_i32 => 113,
                        _ => 114,
                    };
                }
                42 => {
                    let t7 = v7;
                    bb = match t7 {
                        0_i32 => 128,
                        1_i32 => 129,
                        2_i32 => 130,
                        3_i32 => 131,
                        4_i32 => 132,
                        5_i32 => 133,
                        6_i32 => 134,
                        7_i32 => 135,
                        8_i32 => 136,
                        9_i32 => 137,
                        _ => 138,
                    };
                }
                43 => {
                    let t8 = v7;
                    bb = match t8 {
                        0_i32 => 152,
                        1_i32 => 153,
                        2_i32 => 154,
                        3_i32 => 155,
                        4_i32 => 156,
                        _ => 157,
                    };
                }
                44 => {
                    let t9 = v7;
                    bb = match t9 {
                        0_i32 => 166,
                        1_i32 => 167,
                        2_i32 => 168,
                        3_i32 => 169,
                        4_i32 => 170,
                        _ => 171,
                    };
                }
                45 => {
                    let t10 = v7;
                    bb = match t10 {
                        0_i32 => 180,
                        1_i32 => 181,
                        2_i32 => 182,
                        3_i32 => 183,
                        4_i32 => 184,
                        _ => 185,
                    };
                }
                46 => {
                    let t11 = v7;
                    bb = match t11 {
                        0_i32 => 194,
                        1_i32 => 195,
                        2_i32 => 196,
                        3_i32 => 197,
                        4_i32 => 198,
                        _ => 199,
                    };
                }
                47 => {
                    let t12 = v7;
                    bb = match t12 {
                        0_i32 => 208,
                        1_i32 => 209,
                        2_i32 => 210,
                        3_i32 => 211,
                        4_i32 => 212,
                        _ => 213,
                    };
                }
                48 => {
                    let t13 = v7;
                    bb = match t13 {
                        0_i32 => 222,
                        1_i32 => 223,
                        2_i32 => 224,
                        3_i32 => 225,
                        4_i32 => 226,
                        _ => 227,
                    };
                }
                49 => {
                    let t14 = v7;
                    bb = match t14 {
                        0_i32 => 236,
                        1_i32 => 237,
                        2_i32 => 238,
                        3_i32 => 239,
                        4_i32 => 240,
                        _ => 241,
                    };
                }
                50 => {
                    let t15 = v7;
                    bb = match t15 {
                        0_i32 => 250,
                        1_i32 => 251,
                        2_i32 => 252,
                        3_i32 => 253,
                        4_i32 => 254,
                        _ => 255,
                    };
                }
                51 => {
                    let t16 = v7;
                    bb = match t16 {
                        0_i32 => 264,
                        1_i32 => 265,
                        2_i32 => 266,
                        3_i32 => 267,
                        4_i32 => 268,
                        _ => 269,
                    };
                }
                52 => {
                    let t17 = v7;
                    bb = match t17 {
                        0_i32 => 278,
                        1_i32 => 279,
                        2_i32 => 280,
                        3_i32 => 281,
                        4_i32 => 282,
                        _ => 283,
                    };
                }
                53 => {
                    bb = 76;
                }
                54 => {
                    bb = 38;
                }
                55 => {
                    let _ = self.f_1000e5e0(this);
                    result = 0_i32;
                    bb = 54;
                }
                56 => {
                    let _ = self.f_1000e690(this);
                    result = 0_i32;
                    bb = 54;
                }
                57 => {
                    let _ = self.f_1000e710(this);
                    result = 0_i32;
                    bb = 54;
                }
                58 => {
                    let _ = self.f_1000e900(this);
                    result = 0_i32;
                    bb = 54;
                }
                59 => {
                    let _ = self.f_1000e990(this);
                    result = 0_i32;
                    bb = 54;
                }
                60 => {
                    let _ = self.f_1000ead0(this);
                    result = 0_i32;
                    bb = 54;
                }
                61 => {
                    let _ = self.f_1000eb90(this);
                    result = 0_i32;
                    bb = 54;
                }
                62 => {
                    let _ = self.f_1000ed40(this);
                    result = 0_i32;
                    bb = 54;
                }
                63 => {
                    let _ = self.f_1000eea0(this);
                    result = 0_i32;
                    bb = 54;
                }
                64 => {
                    let _ = self.f_1000efa0(this);
                    result = 0_i32;
                    bb = 54;
                }
                65 => {
                    bb = 76;
                }
                66 => {
                    bb = 56;
                }
                67 => {
                    bb = 57;
                }
                68 => {
                    bb = 58;
                }
                69 => {
                    bb = 59;
                }
                70 => {
                    bb = 60;
                }
                71 => {
                    bb = 61;
                }
                72 => {
                    bb = 62;
                }
                73 => {
                    bb = 63;
                }
                74 => {
                    bb = 64;
                }
                75 => {
                    bb = 65;
                }
                76 => {
                    result = 0_i32;
                    bb = 38;
                }
                77 => {
                    bb = 54;
                }
                78 => {
                    bb = 40;
                }
                79 => {
                    bb = 38;
                }
                80 => {
                    let _ = self.f_1000f030(this);
                    result = 0_i32;
                    bb = 79;
                }
                81 => {
                    let _ = self.f_1000f0f0(this);
                    result = 0_i32;
                    bb = 79;
                }
                82 => {
                    let _ = self.f_1000f170(this);
                    result = 0_i32;
                    bb = 79;
                }
                83 => {
                    let _ = self.f_1000f240(this);
                    result = 0_i32;
                    bb = 79;
                }
                84 => {
                    let _ = self.f_1000f3a0(this);
                    result = 0_i32;
                    bb = 79;
                }
                85 => {
                    let _ = self.f_1000f420(this);
                    result = 0_i32;
                    bb = 79;
                }
                86 => {
                    let _ = self.f_1000f4e0(this);
                    result = 0_i32;
                    bb = 79;
                }
                87 => {
                    let _ = self.f_1000f5a0(this);
                    result = 0_i32;
                    bb = 79;
                }
                88 => {
                    let _ = self.f_1000f650(this);
                    result = 0_i32;
                    bb = 79;
                }
                89 => {
                    let _ = self.f_1000f6d0(this);
                    result = 0_i32;
                    bb = 79;
                }
                90 => {
                    bb = 76;
                }
                91 => {
                    bb = 81;
                }
                92 => {
                    bb = 82;
                }
                93 => {
                    bb = 83;
                }
                94 => {
                    bb = 84;
                }
                95 => {
                    bb = 85;
                }
                96 => {
                    bb = 86;
                }
                97 => {
                    bb = 87;
                }
                98 => {
                    bb = 88;
                }
                99 => {
                    bb = 89;
                }
                100 => {
                    bb = 90;
                }
                101 => {
                    bb = 79;
                }
                102 => {
                    bb = 41;
                }
                103 => {
                    bb = 38;
                }
                104 => {
                    let _ = self.f_1000f780(this);
                    result = 0_i32;
                    bb = 103;
                }
                105 => {
                    let _ = self.f_1000f840(this);
                    result = 0_i32;
                    bb = 103;
                }
                106 => {
                    let _ = self.f_1000f8c0(this);
                    result = 0_i32;
                    bb = 103;
                }
                107 => {
                    let _ = self.f_1000fa20(this);
                    result = 0_i32;
                    bb = 103;
                }
                108 => {
                    let _ = self.f_1000fab0(this);
                    result = 0_i32;
                    bb = 103;
                }
                109 => {
                    let _ = self.f_1000fb60(this);
                    result = 0_i32;
                    bb = 103;
                }
                110 => {
                    let _ = self.f_1000fc30(this);
                    result = 0_i32;
                    bb = 103;
                }
                111 => {
                    let _ = self.f_1000fd30(this);
                    result = 0_i32;
                    bb = 103;
                }
                112 => {
                    let _ = self.f_1000fdf0(this);
                    result = 0_i32;
                    bb = 103;
                }
                113 => {
                    let _ = self.f_1000fe80(this);
                    result = 0_i32;
                    bb = 103;
                }
                114 => {
                    bb = 76;
                }
                115 => {
                    bb = 105;
                }
                116 => {
                    bb = 106;
                }
                117 => {
                    bb = 107;
                }
                118 => {
                    bb = 108;
                }
                119 => {
                    bb = 109;
                }
                120 => {
                    bb = 110;
                }
                121 => {
                    bb = 111;
                }
                122 => {
                    bb = 112;
                }
                123 => {
                    bb = 113;
                }
                124 => {
                    bb = 114;
                }
                125 => {
                    bb = 103;
                }
                126 => {
                    bb = 42;
                }
                127 => {
                    bb = 38;
                }
                128 => {
                    let _ = self.f_1000ff80(this);
                    result = 0_i32;
                    bb = 127;
                }
                129 => {
                    let _ = self.f_100101d0(this);
                    result = 0_i32;
                    bb = 127;
                }
                130 => {
                    let _ = self.f_10010400(this);
                    result = 0_i32;
                    bb = 127;
                }
                131 => {
                    let _ = self.f_10010510(this);
                    result = 0_i32;
                    bb = 127;
                }
                132 => {
                    let _ = self.f_100105d0(this);
                    result = 0_i32;
                    bb = 127;
                }
                133 => {
                    let _ = self.f_10010720(this);
                    result = 0_i32;
                    bb = 127;
                }
                134 => {
                    let _ = self.f_100107f0(this);
                    result = 0_i32;
                    bb = 127;
                }
                135 => {
                    let _ = self.f_10010a40(this);
                    result = 0_i32;
                    bb = 127;
                }
                136 => {
                    let _ = self.f_10010b10(this);
                    result = 0_i32;
                    bb = 127;
                }
                137 => {
                    let _ = self.f_10010d60(this);
                    result = 0_i32;
                    bb = 127;
                }
                138 => {
                    bb = 76;
                }
                139 => {
                    bb = 129;
                }
                140 => {
                    bb = 130;
                }
                141 => {
                    bb = 131;
                }
                142 => {
                    bb = 132;
                }
                143 => {
                    bb = 133;
                }
                144 => {
                    bb = 134;
                }
                145 => {
                    bb = 135;
                }
                146 => {
                    bb = 136;
                }
                147 => {
                    bb = 137;
                }
                148 => {
                    bb = 138;
                }
                149 => {
                    bb = 127;
                }
                150 => {
                    bb = 43;
                }
                151 => {
                    bb = 38;
                }
                152 => {
                    let _ = self.f_10010e20(this);
                    result = 0_i32;
                    bb = 151;
                }
                153 => {
                    let _ = self.f_10010ef0(this);
                    result = 0_i32;
                    bb = 151;
                }
                154 => {
                    let _ = self.f_10011000(this);
                    result = 0_i32;
                    bb = 151;
                }
                155 => {
                    let _ = self.f_10011120(this);
                    result = 0_i32;
                    bb = 151;
                }
                156 => {
                    let _ = self.f_10011240(this);
                    result = 0_i32;
                    bb = 151;
                }
                157 => {
                    bb = 76;
                }
                158 => {
                    bb = 153;
                }
                159 => {
                    bb = 154;
                }
                160 => {
                    bb = 155;
                }
                161 => {
                    bb = 156;
                }
                162 => {
                    bb = 157;
                }
                163 => {
                    bb = 151;
                }
                164 => {
                    bb = 44;
                }
                165 => {
                    bb = 38;
                }
                166 => {
                    let _ = self.f_10011320(this);
                    result = 0_i32;
                    bb = 165;
                }
                167 => {
                    let _ = self.f_10011440(this);
                    result = 0_i32;
                    bb = 165;
                }
                168 => {
                    let _ = self.f_10011560(this);
                    result = 0_i32;
                    bb = 165;
                }
                169 => {
                    let _ = self.f_10011680(this);
                    result = 0_i32;
                    bb = 165;
                }
                170 => {
                    let _ = self.f_100117a0(this);
                    result = 0_i32;
                    bb = 165;
                }
                171 => {
                    bb = 76;
                }
                172 => {
                    bb = 167;
                }
                173 => {
                    bb = 168;
                }
                174 => {
                    bb = 169;
                }
                175 => {
                    bb = 170;
                }
                176 => {
                    bb = 171;
                }
                177 => {
                    bb = 165;
                }
                178 => {
                    bb = 45;
                }
                179 => {
                    bb = 38;
                }
                180 => {
                    let _ = self.f_10011880(this);
                    result = 0_i32;
                    bb = 179;
                }
                181 => {
                    let _ = self.f_100119a0(this);
                    result = 0_i32;
                    bb = 179;
                }
                182 => {
                    let _ = self.f_10011a50(this);
                    result = 0_i32;
                    bb = 179;
                }
                183 => {
                    let _ = self.f_10011b70(this);
                    result = 0_i32;
                    bb = 179;
                }
                184 => {
                    let _ = self.f_10011c90(this);
                    result = 0_i32;
                    bb = 179;
                }
                185 => {
                    bb = 76;
                }
                186 => {
                    bb = 181;
                }
                187 => {
                    bb = 182;
                }
                188 => {
                    bb = 183;
                }
                189 => {
                    bb = 184;
                }
                190 => {
                    bb = 185;
                }
                191 => {
                    bb = 179;
                }
                192 => {
                    bb = 46;
                }
                193 => {
                    bb = 38;
                }
                194 => {
                    let _ = self.f_10011d70(this);
                    result = 0_i32;
                    bb = 193;
                }
                195 => {
                    let _ = self.f_10011e40(this);
                    result = 0_i32;
                    bb = 193;
                }
                196 => {
                    let _ = self.f_10011f50(this);
                    result = 0_i32;
                    bb = 193;
                }
                197 => {
                    let _ = self.f_10012000(this);
                    result = 0_i32;
                    bb = 193;
                }
                198 => {
                    let _ = self.f_100120c0(this);
                    result = 0_i32;
                    bb = 193;
                }
                199 => {
                    bb = 76;
                }
                200 => {
                    bb = 195;
                }
                201 => {
                    bb = 196;
                }
                202 => {
                    bb = 197;
                }
                203 => {
                    bb = 198;
                }
                204 => {
                    bb = 199;
                }
                205 => {
                    bb = 193;
                }
                206 => {
                    bb = 47;
                }
                207 => {
                    bb = 38;
                }
                208 => {
                    let _ = self.f_100121e0(this);
                    result = 0_i32;
                    bb = 207;
                }
                209 => {
                    let _ = self.f_100122b0(this);
                    result = 0_i32;
                    bb = 207;
                }
                210 => {
                    let _ = self.f_100123c0(this);
                    result = 0_i32;
                    bb = 207;
                }
                211 => {
                    let _ = self.f_100124e0(this);
                    result = 0_i32;
                    bb = 207;
                }
                212 => {
                    let _ = self.f_100125b0(this);
                    result = 0_i32;
                    bb = 207;
                }
                213 => {
                    bb = 76;
                }
                214 => {
                    bb = 209;
                }
                215 => {
                    bb = 210;
                }
                216 => {
                    bb = 211;
                }
                217 => {
                    bb = 212;
                }
                218 => {
                    bb = 213;
                }
                219 => {
                    bb = 207;
                }
                220 => {
                    bb = 48;
                }
                221 => {
                    bb = 38;
                }
                222 => {
                    let _ = self.f_10012680(this);
                    result = 0_i32;
                    bb = 221;
                }
                223 => {
                    let _ = self.f_100127a0(this);
                    result = 0_i32;
                    bb = 221;
                }
                224 => {
                    let _ = self.f_100128b0(this);
                    result = 0_i32;
                    bb = 221;
                }
                225 => {
                    let _ = self.f_10012980(this);
                    result = 0_i32;
                    bb = 221;
                }
                226 => {
                    let _ = self.f_10012a60(this);
                    result = 0_i32;
                    bb = 221;
                }
                227 => {
                    bb = 76;
                }
                228 => {
                    bb = 223;
                }
                229 => {
                    bb = 224;
                }
                230 => {
                    bb = 225;
                }
                231 => {
                    bb = 226;
                }
                232 => {
                    bb = 227;
                }
                233 => {
                    bb = 221;
                }
                234 => {
                    bb = 49;
                }
                235 => {
                    bb = 38;
                }
                236 => {
                    let _ = self.f_10012b30(this);
                    result = 0_i32;
                    bb = 235;
                }
                237 => {
                    let _ = self.f_10012c50(this);
                    result = 0_i32;
                    bb = 235;
                }
                238 => {
                    let _ = self.f_10012d10(this);
                    result = 0_i32;
                    bb = 235;
                }
                239 => {
                    let _ = self.f_10012df0(this);
                    result = 0_i32;
                    bb = 235;
                }
                240 => {
                    let _ = self.f_10012ec0(this);
                    result = 0_i32;
                    bb = 235;
                }
                241 => {
                    bb = 76;
                }
                242 => {
                    bb = 237;
                }
                243 => {
                    bb = 238;
                }
                244 => {
                    bb = 239;
                }
                245 => {
                    bb = 240;
                }
                246 => {
                    bb = 241;
                }
                247 => {
                    bb = 235;
                }
                248 => {
                    bb = 50;
                }
                249 => {
                    bb = 38;
                }
                250 => {
                    let _ = self.f_10012f80(this);
                    result = 0_i32;
                    bb = 249;
                }
                251 => {
                    let _ = self.f_10013040(this);
                    result = 0_i32;
                    bb = 249;
                }
                252 => {
                    let _ = self.f_10013150(this);
                    result = 0_i32;
                    bb = 249;
                }
                253 => {
                    let _ = self.f_100131f0(this);
                    result = 0_i32;
                    bb = 249;
                }
                254 => {
                    let _ = self.f_100132b0(this);
                    result = 0_i32;
                    bb = 249;
                }
                255 => {
                    bb = 76;
                }
                256 => {
                    bb = 251;
                }
                257 => {
                    bb = 252;
                }
                258 => {
                    bb = 253;
                }
                259 => {
                    bb = 254;
                }
                260 => {
                    bb = 255;
                }
                261 => {
                    bb = 249;
                }
                262 => {
                    bb = 51;
                }
                263 => {
                    bb = 38;
                }
                264 => {
                    let _ = self.f_10013350(this);
                    result = 0_i32;
                    bb = 263;
                }
                265 => {
                    let _ = self.f_10013400(this);
                    result = 0_i32;
                    bb = 263;
                }
                266 => {
                    let _ = self.f_100134b0(this);
                    result = 0_i32;
                    bb = 263;
                }
                267 => {
                    let _ = self.f_100135d0(this);
                    result = 0_i32;
                    bb = 263;
                }
                268 => {
                    let _ = self.f_100136b0(this);
                    result = 0_i32;
                    bb = 263;
                }
                269 => {
                    bb = 76;
                }
                270 => {
                    bb = 265;
                }
                271 => {
                    bb = 266;
                }
                272 => {
                    bb = 267;
                }
                273 => {
                    bb = 268;
                }
                274 => {
                    bb = 269;
                }
                275 => {
                    bb = 263;
                }
                276 => {
                    bb = 52;
                }
                277 => {
                    bb = 38;
                }
                278 => {
                    let _ = self.f_10013870(this);
                    result = 0_i32;
                    bb = 277;
                }
                279 => {
                    let _ = self.f_10013980(this);
                    result = 0_i32;
                    bb = 277;
                }
                280 => {
                    let _ = self.f_10013a40(this);
                    result = 0_i32;
                    bb = 277;
                }
                281 => {
                    let _ = self.f_10013ae0(this);
                    result = 0_i32;
                    bb = 277;
                }
                282 => {
                    let _ = self.f_10013ba0(this);
                    bb = 76;
                }
                283 => {
                    bb = 76;
                }
                284 => {
                    bb = 279;
                }
                285 => {
                    bb = 280;
                }
                286 => {
                    bb = 281;
                }
                287 => {
                    bb = 282;
                }
                288 => {
                    bb = 283;
                }
                289 => {
                    bb = 277;
                }
                290 => {
                    bb = 53;
                }
                291 => {
                    bb = 38;
                }
                292 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_1000E5E0` (168 bytes).
    pub(crate) fn f_1000e5e0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 0_u32);
        self.w32(v2.wrapping_add(1460), 0_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 0_u32);
        self.w32(v2.wrapping_add(1476), 0_u32);
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
                1_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(117_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000E690` (120 bytes).
    pub(crate) fn f_1000e690(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 0_u32);
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 0_u32);
        self.w32(v3.wrapping_add(1476), 1_u32);
        if (((self.r32(v2) as i32) == 1_i32) && ((self.r32(v2.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(708), 55_u32);
            self.w32(v2.wrapping_add(712), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000E710` (488 bytes).
    pub(crate) fn f_1000e710(&mut self, mut this: u32) -> i32 {
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
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 0_u32);
        self.w32(v2.wrapping_add(1460), 2_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 0_u32);
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
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(1124);
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
                v3.wrapping_add((v5.wrapping_add(281_i32) as u32).wrapping_mul(4)),
                2_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(296_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2148)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(2520)) as i32) != 99_i32))
        {
            v7 = 0_i32;
            v8 = v3.wrapping_add(2556);
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
                v3.wrapping_add((v7.wrapping_add(639_i32) as u32).wrapping_mul(4)),
                2_u32,
            );
            self.w32(
                v3.wrapping_add((v7.wrapping_add(654_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if (((self.r32(v3.wrapping_add(2864)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(3236)) as i32) != 99_i32))
        {
            v9 = 0_i32;
            v10 = v3.wrapping_add(3272);
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
                v3.wrapping_add((v9.wrapping_add(818_i32) as u32).wrapping_mul(4)),
                2_u32,
            );
            self.w32(
                v3.wrapping_add((v9.wrapping_add(833_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
        }
        if ((((self.r32(v2.wrapping_add(1484)) as i32) == 1_i32)
            || ((self.r32(v2.wrapping_add(1492)) as i32) == 1_i32))
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

    /// `sub_1000E900` (134 bytes).
    pub(crate) fn f_1000e900(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 0_u32);
        self.w32(v3.wrapping_add(1460), 3_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 0_u32);
        self.w32(v3.wrapping_add(1476), 3_u32);
        if (((self.r32(v2) as i32) == 1_i32) && ((self.r32(v2.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(708), 56_u32);
            self.w32(v2.wrapping_add(712), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000E990` (310 bytes).
    pub(crate) fn f_1000e990(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 0_u32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1472), 0_u32);
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
        if (!((self.r32(v3.wrapping_add(1496)) as i32) != 0)) {
            self.w32(v4.wrapping_add(3548), 0_u32);
            self.w32(v4.wrapping_add(3552), 0_u32);
            self.w32(v4.wrapping_add(3556), 0_u32);
            let _ = self.f_100165d0(this);
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

    /// `sub_1000EAD0` (191 bytes).
    pub(crate) fn f_1000ead0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 0_u32);
        self.w32(v3.wrapping_add(1460), 5_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 0_u32);
        self.w32(v3.wrapping_add(1476), 5_u32);
        if (((self.r32(v2) as i32) == 1_i32) && ((self.r32(v2.wrapping_add(372)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v2.wrapping_add(592);
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
                v2.wrapping_add((v5.wrapping_add(148_i32) as u32).wrapping_mul(4)),
                35_u32,
            );
            self.w32(
                v2.wrapping_add((v5.wrapping_add(151_i32) as u32).wrapping_mul(4)),
                2_u32,
            );
            self.w32(
                v2.wrapping_add((v5.wrapping_add(154_i32) as u32).wrapping_mul(4)),
                ((self.r32(v3.wrapping_add(180)) as i32) as u32),
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v3.wrapping_add(1452)) as i32),
            (self.r32(v3.wrapping_add(1456)) as i32),
            (self.r32(v3.wrapping_add(1460)) as i32),
        );
        return 0_i32;
    }

    /// `sub_1000EB90` (423 bytes).
    pub(crate) fn f_1000eb90(&mut self, mut this: u32) -> i32 {
        let mut v1: i32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: u32 = 0;
        v1 = (self.r32(self.r32(this)) as i32);
        v2 = self.r32((v1.wrapping_add(28_i32) as u32));
        v3 = self.r32((v1.wrapping_add(12_i32) as u32));
        v4 = (self.r32(v2.wrapping_add(1388)) as i32);
        self.w32(v2.wrapping_add(1448), 1_u32);
        self.w32(v2.wrapping_add(1452), (v4 as u32));
        self.w32(v2.wrapping_add(1456), 0_u32);
        self.w32(v2.wrapping_add(1460), 6_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 0_u32);
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
        if ((self.r32(v2.wrapping_add(1484)) as i32) == 1_i32) {
            if (((self.r32(v3) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(372)) as i32) != 99_i32))
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
                    3_u32,
                );
                self.w32(
                    v3.wrapping_add((v5.wrapping_add(117_i32) as u32).wrapping_mul(4)),
                    6_u32,
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
                    3_u32,
                );
                self.w32(
                    v3.wrapping_add((v7.wrapping_add(296_i32) as u32).wrapping_mul(4)),
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
        } else {
            self.w32(v2.wrapping_add(1048), 1_u32);
            return 0_i32;
        }
        #[allow(unreachable_code)]
        0
    }

    /// `sub_1000ED40` (352 bytes).
    pub(crate) fn f_1000ed40(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 0_u32);
        self.w32(v2.wrapping_add(1460), 7_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 0_u32);
        self.w32(v2.wrapping_add(1476), 7_u32);
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
        if (!((self.r32(v2.wrapping_add(1496)) as i32) != 0)) {
            let _ = self.f_10016930(this);
            v5 = 0_i32;
            v6 = v3.wrapping_add(3548);
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
                v3.wrapping_add((v5.wrapping_add(887_i32) as u32).wrapping_mul(4)),
                50_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(888_i32) as u32).wrapping_mul(4)),
                ((1_i32).wrapping_neg() as u32),
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(889_i32) as u32).wrapping_mul(4)),
                ((self.r32(v2.wrapping_add(180)) as i32) as u32),
            );
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

    /// `sub_1000EEA0` (256 bytes).
    pub(crate) fn f_1000eea0(&mut self, mut this: u32) -> i32 {
        let fp = self.enter(16);
        let r = self.f_1000eea0_body(fp, this);
        self.leave(16);
        r
    }

    fn f_1000eea0_body(&mut self, fp: u32, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1460), 8_u32);
        self.w32(v3.wrapping_add(1476), 8_u32);
        self.w32(v3.wrapping_add(1448), 1_u32);
        self.w32(v3.wrapping_add(1452), (v5 as u32));
        self.w32(v3.wrapping_add(1456), 0_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v5 as u32));
        self.w32(v3.wrapping_add(1472), 0_u32);
        let _ = self.f_10006b10(this, v5, fp.wrapping_add(0));
        if (((self.r8(fp.wrapping_add(0)) as i8) as i32) == 1_i32) {
            self.w32(v3.wrapping_add(1480), 1_u32);
            if (((self.r32(v4) as i32) == 1_i32)
                && ((self.r32(v4.wrapping_add(372)) as i32) != 99_i32))
            {
                v6 = 0_i32;
                v7 = v4.wrapping_add(408);
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
                    v4.wrapping_add((v6.wrapping_add(102_i32) as u32).wrapping_mul(4)),
                    4_u32,
                );
                self.w32(
                    v4.wrapping_add((v6.wrapping_add(117_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000EFA0` (134 bytes).
    pub(crate) fn f_1000efa0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 0_u32);
        self.w32(v3.wrapping_add(1460), 9_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 0_u32);
        self.w32(v3.wrapping_add(1476), 9_u32);
        if (((self.r32(v2) as i32) == 1_i32) && ((self.r32(v2.wrapping_add(372)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(708), 57_u32);
            self.w32(v2.wrapping_add(712), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000F030` (181 bytes).
    pub(crate) fn f_1000f030(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 1_u32);
        self.w32(v2.wrapping_add(1460), 0_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 1_u32);
        self.w32(v2.wrapping_add(1476), 0_u32);
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(1124);
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
                v3.wrapping_add((v5.wrapping_add(281_i32) as u32).wrapping_mul(4)),
                5_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(296_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000F0F0` (116 bytes).
    pub(crate) fn f_1000f0f0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 1_u32);
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 1_u32);
        self.w32(v3.wrapping_add(1476), 1_u32);
        if (((self.r32(v2.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v2.wrapping_add(1088)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1424), 58_u32);
            self.w32(v2.wrapping_add(1428), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000F170` (194 bytes).
    pub(crate) fn f_1000f170(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 1_u32);
        self.w32(v2.wrapping_add(1460), 2_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 1_u32);
        self.w32(v2.wrapping_add(1476), 2_u32);
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(1308);
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
                v3.wrapping_add((v5.wrapping_add(327_i32) as u32).wrapping_mul(4)),
                36_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(330_i32) as u32).wrapping_mul(4)),
                2_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(333_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000F240` (337 bytes).
    pub(crate) fn f_1000f240(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 1_u32);
        self.w32(v2.wrapping_add(1460), 3_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 1_u32);
        self.w32(v2.wrapping_add(1476), 3_u32);
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
            if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
                && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
            {
                v5 = 0_i32;
                v6 = v3.wrapping_add(1344);
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
                    v3.wrapping_add((v5.wrapping_add(336_i32) as u32).wrapping_mul(4)),
                    41_u32,
                );
                self.w32(
                    v3.wrapping_add((v5.wrapping_add(338_i32) as u32).wrapping_mul(4)),
                    ((1_i32).wrapping_neg() as u32),
                );
            }
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

    /// `sub_1000F3A0` (124 bytes).
    pub(crate) fn f_1000f3a0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 1_u32);
        self.w32(v3.wrapping_add(1460), 4_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 1_u32);
        self.w32(v3.wrapping_add(1476), 4_u32);
        if (((self.r32(v2.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v2.wrapping_add(1088)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1424), 59_u32);
            self.w32(v2.wrapping_add(1428), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000F420` (191 bytes).
    pub(crate) fn f_1000f420(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 1_u32);
        self.w32(v2.wrapping_add(1460), 5_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 1_u32);
        self.w32(v2.wrapping_add(1476), 5_u32);
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(1308);
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
                v3.wrapping_add((v5.wrapping_add(327_i32) as u32).wrapping_mul(4)),
                37_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(330_i32) as u32).wrapping_mul(4)),
                3_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(333_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000F4E0` (191 bytes).
    pub(crate) fn f_1000f4e0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 1_u32);
        self.w32(v2.wrapping_add(1460), 6_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 1_u32);
        self.w32(v2.wrapping_add(1476), 6_u32);
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(1308);
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
                v3.wrapping_add((v5.wrapping_add(327_i32) as u32).wrapping_mul(4)),
                38_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(330_i32) as u32).wrapping_mul(4)),
                3_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(333_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000F5A0` (173 bytes).
    pub(crate) fn f_1000f5a0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 1_u32);
        self.w32(v3.wrapping_add(1460), 7_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 1_u32);
        self.w32(v3.wrapping_add(1476), 7_u32);
        if (((self.r32(v2.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v2.wrapping_add(1088)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v2.wrapping_add(1384);
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
                v2.wrapping_add((v5.wrapping_add(346_i32) as u32).wrapping_mul(4)),
                48_u32,
            );
            self.w32(
                v2.wrapping_add((v5.wrapping_add(348_i32) as u32).wrapping_mul(4)),
                3_u32,
            );
        }
        let _ = self.f_100074e0(
            this,
            (self.r32(v3.wrapping_add(1452)) as i32),
            (self.r32(v3.wrapping_add(1456)) as i32),
            (self.r32(v3.wrapping_add(1460)) as i32),
        );
        return 0_i32;
    }

    /// `sub_1000F650` (124 bytes).
    pub(crate) fn f_1000f650(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1456), 1_u32);
        self.w32(v3.wrapping_add(1460), 8_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 1_u32);
        self.w32(v3.wrapping_add(1476), 8_u32);
        if (((self.r32(v2.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v2.wrapping_add(1088)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(1424), 60_u32);
            self.w32(v2.wrapping_add(1428), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }

    /// `sub_1000F6D0` (175 bytes).
    pub(crate) fn f_1000f6d0(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1456), 1_u32);
        self.w32(v2.wrapping_add(1460), 9_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 1_u32);
        self.w32(v2.wrapping_add(1476), 9_u32);
        if (((self.r32(v3.wrapping_add(716)) as i32) == 1_i32)
            && ((self.r32(v3.wrapping_add(1088)) as i32) != 99_i32))
        {
            v5 = 0_i32;
            v6 = v3.wrapping_add(1124);
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
                v3.wrapping_add((v5.wrapping_add(281_i32) as u32).wrapping_mul(4)),
                6_u32,
            );
            self.w32(
                v3.wrapping_add((v5.wrapping_add(296_i32) as u32).wrapping_mul(4)),
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

    /// `sub_1000F780` (186 bytes).
    pub(crate) fn f_1000f780(&mut self, mut this: u32) -> i32 {
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
        self.w32(v2.wrapping_add(1460), 0_u32);
        self.w32(v2.wrapping_add(1464), 1_u32);
        self.w32(v2.wrapping_add(1468), (v4 as u32));
        self.w32(v2.wrapping_add(1472), 2_u32);
        self.w32(v2.wrapping_add(1476), 0_u32);
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
                7_u32,
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

    /// `sub_1000F840` (124 bytes).
    pub(crate) fn f_1000f840(&mut self, mut this: u32) -> i32 {
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
        self.w32(v3.wrapping_add(1460), 1_u32);
        self.w32(v3.wrapping_add(1464), 1_u32);
        self.w32(v3.wrapping_add(1468), (v4 as u32));
        self.w32(v3.wrapping_add(1472), 2_u32);
        self.w32(v3.wrapping_add(1476), 1_u32);
        if (((self.r32(v2.wrapping_add(1432)) as i32) == 1_i32)
            && ((self.r32(v2.wrapping_add(1804)) as i32) != 99_i32))
        {
            self.w32(v2.wrapping_add(2140), 61_u32);
            self.w32(v2.wrapping_add(2144), 1_u32);
        }
        let _ = self.f_10013c90(this);
        return 0_i32;
    }
}
