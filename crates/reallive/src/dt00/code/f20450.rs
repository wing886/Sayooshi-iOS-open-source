//! Functions 0x10020450..=0x10021270, translated from the decompilation.

use super::super::Dt00;
use super::super::rt::*;

impl Dt00 {
    /// `sub_10020450` (1169 bytes).
    pub(crate) fn f_10020450(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1680);
        let r = self.f_10020450_body(fp, this, a2);
        self.leave(1680);
        r
    }

    fn f_10020450_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v11: bool = false;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
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
        let mut v31: u32 = 0;
        let mut v32: i32 = 0;
        let mut v33: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(((self.r32(this) as i32) as u32));
                    self.w32(fp.wrapping_add(8), 20_u32);
                    v3 = (self.r32(v2.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(v2.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(v2.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(self.r32(fp.wrapping_add(52)).wrapping_add(1388)) as i32)
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(384_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(380_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(388_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(this.wrapping_add(228)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(self.r32(fp.wrapping_add(36))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(36)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(20)) as i32).wrapping_add(
                            20_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)),
                        ) as u32),
                    );
                    let _ = self.memset(fp.wrapping_add(68), 0_i32 as u8, 1600_u32);
                    v4 = (v3.wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(4), (v3.wrapping_add(3600_i32) as u32));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v4) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (!v11) { 1 } else { 3 };
                }
                3 => {
                    v12 = 0_i32;
                    self.w32(fp.wrapping_add(16), 1_u32);
                    self.w32(fp.wrapping_add(4), 9999_u32);
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(68));
                    bb = 18;
                }
                4 => {
                    v5 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add((self.r32(v4.wrapping_add(124)) as i32));
                    v6 = self.r32(fp.wrapping_add(16));
                    v7 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v4.wrapping_add(128)) as i32));
                    v8 = (self.r32(fp.wrapping_add(48)) as i32).wrapping_add(v5);
                    bb = if (v7 < v7.wrapping_add((self.r32(fp.wrapping_add(64)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v4 = v4.wrapping_add(156);
                    v11 = ((self.r32(fp.wrapping_add(8)) as i32) == 1_i32);
                    self.w32(fp.wrapping_add(4), v4);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v9 = (20_i32.wrapping_mul(v7) as u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(64)) as i32) as u32),
                    );
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v10 = (v5 as u32);
                    bb = if (v5 < v8) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = 7;
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v9 = v9.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((((self.r32(v6) as i32) != 0) && (v9 < 400_u32)) && (v10 < 20_u32)) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v10 as i32) < v8) { 13 } else { 15 };
                }
                15 => {
                    v4 = self.r32(fp.wrapping_add(4));
                    bb = 12;
                }
                16 => {
                    self.w32(
                        fp.wrapping_add(68)
                            .wrapping_add(((v9.wrapping_add(v10) as i32) as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(68).wrapping_add(
                                ((v9.wrapping_add(v10) as i32) as u32).wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add((self.r32(v6) as i32)) as u32),
                    );
                    bb = 17;
                }
                17 => {
                    v6 = v6.wrapping_add(4);
                    v10 = v10.wrapping_add(1);
                    bb = 14;
                }
                18 => {
                    v13 = (crem_i32(v12, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs() as u32);
                    v14 = (cdiv_i32(v12, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs() as u32);
                    v15 = (v14.wrapping_add(v13) as i32);
                    v16 = (v13.wrapping_sub(v14) as i32).wrapping_abs();
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32)
                        < (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                19 => {
                    bb = if (v12 < 400_i32) { 18 } else { 20 };
                }
                20 => {
                    self.w32(
                        fp.wrapping_add(8),
                        (crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        (cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32) as u32),
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(36), 0_i32, 0_i32, 2_i32, 1_i32);
                    v17 = (self.r32(fp.wrapping_add(24)) as i32);
                    v18 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(12)) as i32);
                    v19 = (self.r32(self.r32(fp.wrapping_add(36))) as i32)
                        .wrapping_add(crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32));
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(36)).wrapping_add(16),
                    );
                    v20 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(4)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32));
                    v21 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(8)) as i32)
                        .wrapping_add(v19);
                    self.w32(
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_add(7400_i32) as u32),
                        0_u32,
                    );
                    self.w32(fp.wrapping_add(24), (v21 as u32));
                    v22 = v20.wrapping_add(v18);
                    bb = if (v20 < v20.wrapping_add(v18)) {
                        35
                    } else {
                        36
                    };
                }
                21 => {
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(self.r32(fp.wrapping_add(0))) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(4), ((v14.wrapping_add(v13) as i32) as u32));
                    bb = 23;
                }
                22 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32)
                        != (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                    {
                        26
                    } else {
                        27
                    };
                }
                23 => {
                    self.w32(fp.wrapping_add(8), (v16 as u32));
                    self.w32(fp.wrapping_add(12), (v12 as u32));
                    bb = 24;
                }
                24 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 19;
                }
                25 => {
                    bb = 22;
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        29
                    } else {
                        30
                    };
                }
                28 => {
                    bb = 27;
                }
                29 => {
                    self.w32(fp.wrapping_add(4), ((v14.wrapping_add(v13) as i32) as u32));
                    bb = 23;
                }
                30 => {
                    bb = if ((v15 == (self.r32(fp.wrapping_add(4)) as i32))
                        && (v16 < (self.r32(fp.wrapping_add(8)) as i32)))
                    {
                        32
                    } else {
                        33
                    };
                }
                31 => {
                    bb = 30;
                }
                32 => {
                    bb = 23;
                }
                33 => {
                    bb = 24;
                }
                34 => {
                    bb = 33;
                }
                35 => {
                    v23 = (20_i32.wrapping_mul(v20) as u32);
                    self.w32(fp.wrapping_add(16), (v22.wrapping_sub(v20) as u32));
                    bb = 37;
                }
                36 => {
                    v26 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32));
                    v27 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32));
                    v28 = (self.r32(fp.wrapping_add(44)) as i32);
                    v29 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs();
                    v30 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs();
                    bb = if (v30 == v29) { 51 } else { 53 };
                }
                37 => {
                    v24 = (v19 as u32);
                    bb = if (v19 < v21) { 40 } else { 41 };
                }
                38 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) != 0) {
                        37
                    } else {
                        39
                    };
                }
                39 => {
                    bb = 36;
                }
                40 => {
                    bb = 42;
                }
                41 => {
                    v23 = v23.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 38;
                }
                42 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0) {
                        45
                    } else {
                        46
                    };
                }
                43 => {
                    bb = if ((v24 as i32) < (self.r32(fp.wrapping_add(24)) as i32)) {
                        42
                    } else {
                        44
                    };
                }
                44 => {
                    bb = 41;
                }
                45 => {
                    bb = if ((v23 < 400_u32) && (v24 < 20_u32)) {
                        47
                    } else {
                        48
                    };
                }
                46 => {
                    v24 = v24.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    v21 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 43;
                }
                47 => {
                    {
                        let a0 = (v17
                            .wrapping_add(4_i32.wrapping_mul({
                                let t1 = (self.r32((v17.wrapping_add(7400_i32) as u32)) as i32);
                                self.w32(
                                    (v17.wrapping_add(7400_i32) as u32),
                                    (t1.wrapping_add(1) as u32),
                                );
                                t1
                            }))
                            .wrapping_add(6800_i32) as u32);
                        let a1 = ((v23.wrapping_add(v24).wrapping_add(400_u32) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    v25 = (self.r32(
                        (v17 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v23.wrapping_add(v24)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v25 > 4_i32) { 49 } else { 50 };
                }
                48 => {
                    bb = 46;
                }
                49 => {
                    let _ = self.f_1000a0a0(
                        this,
                        (self.r32(fp.wrapping_add(40)) as i32),
                        v25.wrapping_sub(5_i32),
                        67_i32,
                    );
                    bb = 50;
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    bb = if ((self.r32(fp.wrapping_add(44)) as i32) != 0) {
                        54
                    } else {
                        56
                    };
                }
                52 => {
                    v31 = self.r32(fp.wrapping_add(52));
                    self.w32(
                        self.r32(fp.wrapping_add(52)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        v31.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(28)) as i32) as u32),
                    );
                    v32 = (self.r32(fp.wrapping_add(60)) as i32);
                    self.w32(v31.wrapping_add(1540), 1_u32);
                    self.w32(v31.wrapping_add(1552), (v32 as u32));
                    self.w32(v31.wrapping_add(1464), 1_u32);
                    v33 = (self.r32(fp.wrapping_add(40)) as i32);
                    self.w32(
                        v31.wrapping_add(2324),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(v31.wrapping_add(1468), (v33 as u32));
                    self.w32(v31.wrapping_add(1556), (v28 as u32));
                    self.w32(v31.wrapping_add(1472), 12_u32);
                    self.w32(v31.wrapping_add(1476), 0_u32);
                    self.w32(
                        v31.wrapping_add(2328),
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        v31.wrapping_add(2332),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w8(a2, 1_u8);
                    return 0_i32;
                }
                53 => {
                    bb = if (v30 >= v29) { 80 } else { 82 };
                }
                54 => {
                    let t2 = (self.r32(fp.wrapping_add(44)) as i32);
                    bb = match t2 {
                        1_i32 => 58,
                        2_i32 => 59,
                        3_i32 => 60,
                        _ => 61,
                    };
                }
                55 => {
                    bb = 52;
                }
                56 => {
                    bb = if (v26 <= 0_i32) { 77 } else { 78 };
                }
                57 => {
                    bb = 55;
                }
                58 => {
                    bb = if (v27 >= 0_i32) { 62 } else { 63 };
                }
                59 => {
                    bb = if (v26 >= 0_i32) { 67 } else { 68 };
                }
                60 => {
                    bb = if (v27 <= 0_i32) { 71 } else { 72 };
                }
                61 => {
                    bb = 75;
                }
                62 => {
                    bb = 64;
                }
                63 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 57;
                }
                64 => {
                    bb = if (v28 == (self.r32(fp.wrapping_add(44)) as i32)) {
                        83
                    } else {
                        84
                    };
                }
                65 => {
                    bb = 63;
                }
                66 => {
                    bb = 59;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v28 = (if (v27 >= 0_i32) {
                        1_i32
                    } else {
                        (self.r32(fp.wrapping_add(44)) as i32).wrapping_add(1_i32)
                    });
                    bb = 57;
                }
                69 => {
                    bb = 68;
                }
                70 => {
                    bb = 60;
                }
                71 => {
                    bb = 64;
                }
                72 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 57;
                }
                73 => {
                    bb = 72;
                }
                74 => {
                    bb = 61;
                }
                75 => {
                    v28 = (1_i32).wrapping_neg();
                    bb = 84;
                }
                76 => {
                    bb = 57;
                }
                77 => {
                    bb = 64;
                }
                78 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 55;
                }
                79 => {
                    bb = 78;
                }
                80 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 81;
                }
                81 => {
                    bb = 64;
                }
                82 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 81;
                }
                83 => {
                    bb = 75;
                }
                84 => {
                    bb = 52;
                }
                85 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_100208F0` (44 bytes).
    pub(crate) fn f_100208f0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        v2 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(28_i32) as u32))
            as i32);
        let _ = self.f_10020450(this, a2);
        self.w32((v2.wrapping_add(1472_i32) as u32), 12_u32);
        self.w32((v2.wrapping_add(1476_i32) as u32), 3_u32);
        return 0_i32;
    }

    /// `sub_10020920` (44 bytes).
    pub(crate) fn f_10020920(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        v2 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(28_i32) as u32))
            as i32);
        let _ = self.f_10020450(this, a2);
        self.w32((v2.wrapping_add(1472_i32) as u32), 14_u32);
        self.w32((v2.wrapping_add(1476_i32) as u32), 1_u32);
        return 0_i32;
    }

    /// `sub_10020950` (44 bytes).
    pub(crate) fn f_10020950(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        v2 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(28_i32) as u32))
            as i32);
        let _ = self.f_10020450(this, a2);
        self.w32((v2.wrapping_add(1472_i32) as u32), 14_u32);
        self.w32((v2.wrapping_add(1476_i32) as u32), 3_u32);
        return 0_i32;
    }

    /// `sub_10020980` (1047 bytes).
    pub(crate) fn f_10020980(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1664);
        let r = self.f_10020980_body(fp, this, a2);
        self.leave(1664);
        r
    }

    fn f_10020980_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: u32 = 0;
        let mut v8: i32 = 0;
        let mut v9: i32 = 0;
        let mut v10: i32 = 0;
        let mut v11: u32 = 0;
        let mut v12: u32 = 0;
        let mut v13: bool = false;
        let mut v14: i32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: u32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
        let mut v20: i32 = 0;
        let mut v21: i32 = 0;
        let mut v22: i32 = 0;
        let mut v23: i32 = 0;
        let mut v24: i32 = 0;
        let mut v25: u32 = 0;
        let mut v26: i32 = 0;
        let mut v27: i32 = 0;
        let mut v28: i32 = 0;
        let mut v29: i32 = 0;
        let mut v30: u32 = 0;
        let mut v31: i32 = 0;
        let mut i: i32 = 0;
        let mut v33: i32 = 0;
        let mut v34: i32 = 0;
        let mut v35: i32 = 0;
        let mut v36: i32 = 0;
        let mut v37: i32 = 0;
        let mut v38: i32 = 0;
        let mut v40: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(((self.r32(this) as i32) as u32));
                    self.w32(fp.wrapping_add(20), 20_u32);
                    v3 = (self.r32(v2.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(v2.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(v2.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(self.r32(fp.wrapping_add(48)).wrapping_add(1388)) as i32)
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(24)) as i32)),
                            )
                            .wrapping_add(380_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(24)) as i32)),
                            )
                            .wrapping_add(384_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(24)) as i32)),
                            )
                            .wrapping_add(388_i32) as u32),
                        ) as i32) as u32),
                    );
                    v4 = ((self.r32(this.wrapping_add(224)) as i32) as u32);
                    v5 = (self.r32(v4) as i32);
                    v4 = v4.wrapping_add(16);
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(v4.wrapping_sub(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(v4.wrapping_sub(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(v4.wrapping_sub(4)) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(36), v4);
                    v6 = (v3.wrapping_add(3600_i32) as u32);
                    let _ = self.memset(fp.wrapping_add(64), 0_i32 as u8, 1600_u32);
                    self.w32(fp.wrapping_add(4), (v3.wrapping_add(3600_i32) as u32));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v6) as i32) == 1_i32)
                        && ((self.r32(v6.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (!v13) { 1 } else { 3 };
                }
                3 => {
                    v14 = 0_i32;
                    v15 = 0_i32;
                    self.w32(fp.wrapping_add(0), 9999_u32);
                    v16 = 0_i32;
                    v17 = fp.wrapping_add(64);
                    bb = 18;
                }
                4 => {
                    v7 = self.r32(fp.wrapping_add(36));
                    v8 = v5.wrapping_add((self.r32(v6.wrapping_add(124)) as i32));
                    v9 = (self.r32(fp.wrapping_add(28)) as i32)
                        .wrapping_add((self.r32(v6.wrapping_add(128)) as i32));
                    v10 = v8.wrapping_add((self.r32(fp.wrapping_add(52)) as i32));
                    bb = if (v9 < v9.wrapping_add((self.r32(fp.wrapping_add(16)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v6 = v6.wrapping_add(156);
                    v13 = ((self.r32(fp.wrapping_add(20)) as i32) == 1_i32);
                    self.w32(fp.wrapping_add(4), v6);
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(fp.wrapping_add(20)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v11 = (20_i32.wrapping_mul(v9) as u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(16)) as i32) as u32),
                    );
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
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = 7;
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v11 = v11.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((((self.r32(v7) as i32) != 0) && (v11 < 400_u32)) && (v12 < 20_u32)) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v12 as i32) < v10) { 13 } else { 15 };
                }
                15 => {
                    v6 = self.r32(fp.wrapping_add(4));
                    bb = 12;
                }
                16 => {
                    self.w32(
                        fp.wrapping_add(64)
                            .wrapping_add(((v11.wrapping_add(v12) as i32) as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(64).wrapping_add(
                                ((v11.wrapping_add(v12) as i32) as u32).wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add((self.r32(v7) as i32)) as u32),
                    );
                    bb = 17;
                }
                17 => {
                    v7 = v7.wrapping_add(4);
                    v12 = v12.wrapping_add(1);
                    bb = 14;
                }
                18 => {
                    v18 = 0_i32;
                    v19 = 20_i32;
                    bb = 21;
                }
                19 => {
                    bb = if (v16 < 20_i32) { 18 } else { 20 };
                }
                20 => {
                    self.w32(fp.wrapping_add(4), (v15 as u32));
                    bb = if (v15 >= 1_i32) { 32 } else { 34 };
                }
                21 => {
                    v20 = (self.r32({
                        let t1 = v17;
                        v17 = t1.wrapping_add(4);
                        t1
                    }) as i32);
                    v18 = v18.wrapping_add(v20);
                    v19 = v19.wrapping_sub(1);
                    bb = 22;
                }
                22 => {
                    bb = if (v19 != 0) { 21 } else { 23 };
                }
                23 => {
                    v21 = v16
                        .wrapping_sub((self.r32(fp.wrapping_add(8)) as i32))
                        .wrapping_abs();
                    bb = if (v14 < v18) { 24 } else { 25 };
                }
                24 => {
                    v14 = v18;
                    bb = 26;
                }
                25 => {
                    bb = if ((v14 == v18) && (v21 < (self.r32(fp.wrapping_add(0)) as i32))) {
                        29
                    } else {
                        30
                    };
                }
                26 => {
                    self.w32(fp.wrapping_add(0), (v21 as u32));
                    v15 = v16;
                    bb = 27;
                }
                27 => {
                    v16 = v16.wrapping_add(1);
                    bb = 19;
                }
                28 => {
                    bb = 25;
                }
                29 => {
                    bb = 26;
                }
                30 => {
                    bb = 27;
                }
                31 => {
                    bb = 30;
                }
                32 => {
                    bb = if (v15 > 18_i32) { 35 } else { 36 };
                }
                33 => {
                    v22 = 0_i32;
                    v23 = 0_i32;
                    v24 = 9999_i32;
                    self.w32(fp.wrapping_add(0), 0_u32);
                    v25 = fp.wrapping_add(64).wrapping_add(
                        (20_i32.wrapping_mul((self.r32(fp.wrapping_add(4)) as i32)) as u32)
                            .wrapping_mul(4),
                    );
                    bb = 37;
                }
                34 => {
                    self.w32(fp.wrapping_add(4), 1_u32);
                    bb = 33;
                }
                35 => {
                    self.w32(fp.wrapping_add(4), 18_u32);
                    bb = 36;
                }
                36 => {
                    bb = 33;
                }
                37 => {
                    bb = if (2_i32 != 0) { 38 } else { 39 };
                }
                38 => {
                    v26 = v23
                        .wrapping_sub((self.r32(fp.wrapping_add(12)) as i32))
                        .wrapping_abs();
                    bb = if (v22 < (self.r32(v25) as i32)) {
                        40
                    } else {
                        41
                    };
                }
                39 => {
                    v27 = (self.r32(fp.wrapping_add(4)) as i32);
                    self.w32(
                        ((self.r32(fp.wrapping_add(60)) as i32).wrapping_add(7400_i32) as u32),
                        0_u32,
                    );
                    v28 = (self.r32(fp.wrapping_add(4)) as i32).wrapping_sub(1_i32);
                    v29 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_sub(1_i32)
                        .wrapping_add(3_i32);
                    bb = if ((self.r32(fp.wrapping_add(4)) as i32).wrapping_sub(1_i32) < v29) {
                        50
                    } else {
                        51
                    };
                }
                40 => {
                    v22 = (self.r32(v25) as i32);
                    bb = 42;
                }
                41 => {
                    bb = if ((v22 == (self.r32(v25) as i32)) && (v26 < v24)) {
                        44
                    } else {
                        45
                    };
                }
                42 => {
                    v24 = v26;
                    self.w32(fp.wrapping_add(0), (v23 as u32));
                    bb = 45;
                }
                43 => {
                    bb = 41;
                }
                44 => {
                    bb = 42;
                }
                45 => {
                    v23 = v23.wrapping_add(1);
                    v25 = v25.wrapping_add(4);
                    bb = if (v23 < 20_i32) { 46 } else { 47 };
                }
                46 => {
                    bb = 37;
                }
                47 => {
                    bb = 39;
                }
                48 => {
                    bb = 47;
                }
                49 => {
                    bb = 37;
                }
                50 => {
                    v30 = (20_i32.wrapping_mul(v28) as u32);
                    v31 = v29.wrapping_sub(v28);
                    bb = 52;
                }
                51 => {
                    v34 = v27.wrapping_sub((self.r32(fp.wrapping_add(8)) as i32));
                    v35 = (self.r32(fp.wrapping_add(0)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(12)) as i32));
                    v36 = v34.wrapping_abs();
                    v37 = (self.r32(fp.wrapping_add(40)) as i32);
                    v38 = (self.r32(fp.wrapping_add(0)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(12)) as i32))
                        .wrapping_abs();
                    bb = if (v38 == v36) { 63 } else { 65 };
                }
                52 => {
                    i = 0_i32;
                    bb = 55;
                }
                53 => {
                    bb = if (v31 != 0) { 52 } else { 54 };
                }
                54 => {
                    v27 = (self.r32(fp.wrapping_add(4)) as i32);
                    bb = 51;
                }
                55 => {
                    bb = if (i < 20_i32) { 56 } else { 58 };
                }
                56 => {
                    bb = if ((v30 < 400_u32) && (i >= 0_i32)) {
                        59
                    } else {
                        60
                    };
                }
                57 => {
                    i = i.wrapping_add(1);
                    bb = 55;
                }
                58 => {
                    v30 = v30.wrapping_add(20_u32);
                    v31 = v31.wrapping_sub(1);
                    bb = 53;
                }
                59 => {
                    {
                        let a0 = ((self.r32(fp.wrapping_add(60)) as i32)
                            .wrapping_add(4_i32.wrapping_mul({
                                let t2 = (self.r32(
                                    ((self.r32(fp.wrapping_add(60)) as i32).wrapping_add(7400_i32)
                                        as u32),
                                ) as i32);
                                self.w32(
                                    ((self.r32(fp.wrapping_add(60)) as i32).wrapping_add(7400_i32)
                                        as u32),
                                    (t2.wrapping_add(1) as u32),
                                );
                                t2
                            }))
                            .wrapping_add(6800_i32) as u32);
                        let a1 =
                            ((v30.wrapping_add((i as u32)).wrapping_add(400_u32) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    v33 = (self.r32(
                        ((self.r32(fp.wrapping_add(60)) as i32) as u32)
                            .wrapping_add(4_u32.wrapping_mul(v30.wrapping_add((i as u32))))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v33 > 4_i32) { 61 } else { 62 };
                }
                60 => {
                    bb = 57;
                }
                61 => {
                    let _ = self.f_1000a0a0(
                        this,
                        (self.r32(fp.wrapping_add(24)) as i32),
                        v33.wrapping_sub(5_i32),
                        71_i32,
                    );
                    bb = 62;
                }
                62 => {
                    bb = 60;
                }
                63 => {
                    bb = if ((self.r32(fp.wrapping_add(40)) as i32) != 0) {
                        66
                    } else {
                        68
                    };
                }
                64 => {
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(48)).wrapping_add(1540), 1_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(1552),
                        ((self.r32(fp.wrapping_add(12)) as i32).wrapping_add(
                            20_i32.wrapping_mul((self.r32(fp.wrapping_add(8)) as i32)),
                        ) as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(48)).wrapping_add(1464), 1_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(2324),
                        ((self.r32(fp.wrapping_add(0)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(1468),
                        ((self.r32(fp.wrapping_add(24)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(1556),
                        (v37 as u32),
                    );
                    self.w32(self.r32(fp.wrapping_add(48)).wrapping_add(1472), 12_u32);
                    self.w32(self.r32(fp.wrapping_add(48)).wrapping_add(1476), 2_u32);
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(2328),
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        self.r32(fp.wrapping_add(48)).wrapping_add(2332),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_add(
                            20_i32.wrapping_mul((self.r32(fp.wrapping_add(4)) as i32)),
                        ) as u32),
                    );
                    self.w8(a2, 1_u8);
                    return 0_i32;
                }
                65 => {
                    bb = if (v38 >= v36) { 92 } else { 94 };
                }
                66 => {
                    let t3 = (self.r32(fp.wrapping_add(40)) as i32);
                    bb = match t3 {
                        1_i32 => 70,
                        2_i32 => 71,
                        3_i32 => 72,
                        _ => 73,
                    };
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    bb = if (v34 <= 0_i32) { 89 } else { 90 };
                }
                69 => {
                    bb = 67;
                }
                70 => {
                    bb = if (v35 >= 0_i32) { 74 } else { 75 };
                }
                71 => {
                    bb = if (v34 >= 0_i32) { 79 } else { 80 };
                }
                72 => {
                    bb = if (v35 <= 0_i32) { 83 } else { 84 };
                }
                73 => {
                    bb = 87;
                }
                74 => {
                    bb = 76;
                }
                75 => {
                    v37 = (if (v34 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 69;
                }
                76 => {
                    bb = if (v37 == (self.r32(fp.wrapping_add(40)) as i32)) {
                        95
                    } else {
                        96
                    };
                }
                77 => {
                    bb = 75;
                }
                78 => {
                    bb = 71;
                }
                79 => {
                    bb = 76;
                }
                80 => {
                    v37 = (if (v35 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 69;
                }
                81 => {
                    bb = 80;
                }
                82 => {
                    bb = 72;
                }
                83 => {
                    bb = 76;
                }
                84 => {
                    v40 = ((v34 >= 0_i32) as i32).wrapping_sub(1_i32);
                    v40 = (((((v40 as u32) & 0xFFFFFF00)
                        | (((((v40 & 254_i32) as u8) as u8) as u32) << 0))
                        as i32) as i32);
                    v37 = v40.wrapping_add(2_i32);
                    bb = 69;
                }
                85 => {
                    bb = 84;
                }
                86 => {
                    bb = 73;
                }
                87 => {
                    v37 = (1_i32).wrapping_neg();
                    bb = 96;
                }
                88 => {
                    bb = 69;
                }
                89 => {
                    bb = 76;
                }
                90 => {
                    v37 = (if (v35 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 67;
                }
                91 => {
                    bb = 90;
                }
                92 => {
                    v37 = (if (v35 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 93;
                }
                93 => {
                    bb = 76;
                }
                94 => {
                    v37 = (if (v34 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 93;
                }
                95 => {
                    bb = 87;
                }
                96 => {
                    bb = 64;
                }
                97 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10020DA0` (1169 bytes).
    pub(crate) fn f_10020da0(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1680);
        let r = self.f_10020da0_body(fp, this, a2);
        self.leave(1680);
        r
    }

    fn f_10020da0_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v11: bool = false;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
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
        let mut v31: u32 = 0;
        let mut v32: i32 = 0;
        let mut v33: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(((self.r32(this) as i32) as u32));
                    self.w32(fp.wrapping_add(8), 20_u32);
                    v3 = (self.r32(v2.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(v2.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(v2.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(self.r32(fp.wrapping_add(52)).wrapping_add(1388)) as i32)
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(384_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(380_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(388_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(this.wrapping_add(236)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(self.r32(fp.wrapping_add(36))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(36)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(20)) as i32).wrapping_add(
                            20_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)),
                        ) as u32),
                    );
                    let _ = self.memset(fp.wrapping_add(68), 0_i32 as u8, 1600_u32);
                    v4 = (v3.wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(4), (v3.wrapping_add(3600_i32) as u32));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v4) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (!v11) { 1 } else { 3 };
                }
                3 => {
                    v12 = 0_i32;
                    self.w32(fp.wrapping_add(16), 1_u32);
                    self.w32(fp.wrapping_add(4), 9999_u32);
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(68));
                    bb = 18;
                }
                4 => {
                    v5 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add((self.r32(v4.wrapping_add(124)) as i32));
                    v6 = self.r32(fp.wrapping_add(16));
                    v7 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v4.wrapping_add(128)) as i32));
                    v8 = (self.r32(fp.wrapping_add(48)) as i32).wrapping_add(v5);
                    bb = if (v7 < v7.wrapping_add((self.r32(fp.wrapping_add(64)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v4 = v4.wrapping_add(156);
                    v11 = ((self.r32(fp.wrapping_add(8)) as i32) == 1_i32);
                    self.w32(fp.wrapping_add(4), v4);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v9 = (20_i32.wrapping_mul(v7) as u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(64)) as i32) as u32),
                    );
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v10 = (v5 as u32);
                    bb = if (v5 < v8) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = 7;
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v9 = v9.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((((self.r32(v6) as i32) != 0) && (v9 < 400_u32)) && (v10 < 20_u32)) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v10 as i32) < v8) { 13 } else { 15 };
                }
                15 => {
                    v4 = self.r32(fp.wrapping_add(4));
                    bb = 12;
                }
                16 => {
                    self.w32(
                        fp.wrapping_add(68)
                            .wrapping_add(((v9.wrapping_add(v10) as i32) as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(68).wrapping_add(
                                ((v9.wrapping_add(v10) as i32) as u32).wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add((self.r32(v6) as i32)) as u32),
                    );
                    bb = 17;
                }
                17 => {
                    v6 = v6.wrapping_add(4);
                    v10 = v10.wrapping_add(1);
                    bb = 14;
                }
                18 => {
                    v13 = (crem_i32(v12, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs() as u32);
                    v14 = (cdiv_i32(v12, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs() as u32);
                    v15 = (v14.wrapping_add(v13) as i32);
                    v16 = (v13.wrapping_sub(v14) as i32).wrapping_abs();
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32)
                        < (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                19 => {
                    bb = if (v12 < 400_i32) { 18 } else { 20 };
                }
                20 => {
                    self.w32(
                        fp.wrapping_add(8),
                        (crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        (cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32) as u32),
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(36), 0_i32, 0_i32, 4_i32, 1_i32);
                    v17 = (self.r32(fp.wrapping_add(24)) as i32);
                    v18 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(12)) as i32);
                    v19 = (self.r32(self.r32(fp.wrapping_add(36))) as i32)
                        .wrapping_add(crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32));
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(36)).wrapping_add(16),
                    );
                    v20 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(4)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32));
                    v21 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(8)) as i32)
                        .wrapping_add(v19);
                    self.w32(
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_add(7400_i32) as u32),
                        0_u32,
                    );
                    self.w32(fp.wrapping_add(24), (v21 as u32));
                    v22 = v20.wrapping_add(v18);
                    bb = if (v20 < v20.wrapping_add(v18)) {
                        35
                    } else {
                        36
                    };
                }
                21 => {
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(self.r32(fp.wrapping_add(0))) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(4), ((v14.wrapping_add(v13) as i32) as u32));
                    bb = 23;
                }
                22 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32)
                        != (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                    {
                        26
                    } else {
                        27
                    };
                }
                23 => {
                    self.w32(fp.wrapping_add(8), (v16 as u32));
                    self.w32(fp.wrapping_add(12), (v12 as u32));
                    bb = 24;
                }
                24 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 19;
                }
                25 => {
                    bb = 22;
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        29
                    } else {
                        30
                    };
                }
                28 => {
                    bb = 27;
                }
                29 => {
                    self.w32(fp.wrapping_add(4), ((v14.wrapping_add(v13) as i32) as u32));
                    bb = 23;
                }
                30 => {
                    bb = if ((v15 == (self.r32(fp.wrapping_add(4)) as i32))
                        && (v16 < (self.r32(fp.wrapping_add(8)) as i32)))
                    {
                        32
                    } else {
                        33
                    };
                }
                31 => {
                    bb = 30;
                }
                32 => {
                    bb = 23;
                }
                33 => {
                    bb = 24;
                }
                34 => {
                    bb = 33;
                }
                35 => {
                    v23 = (20_i32.wrapping_mul(v20) as u32);
                    self.w32(fp.wrapping_add(16), (v22.wrapping_sub(v20) as u32));
                    bb = 37;
                }
                36 => {
                    v26 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32));
                    v27 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32));
                    v28 = (self.r32(fp.wrapping_add(44)) as i32);
                    v29 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs();
                    v30 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs();
                    bb = if (v30 == v29) { 51 } else { 53 };
                }
                37 => {
                    v24 = (v19 as u32);
                    bb = if (v19 < v21) { 40 } else { 41 };
                }
                38 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) != 0) {
                        37
                    } else {
                        39
                    };
                }
                39 => {
                    bb = 36;
                }
                40 => {
                    bb = 42;
                }
                41 => {
                    v23 = v23.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 38;
                }
                42 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0) {
                        45
                    } else {
                        46
                    };
                }
                43 => {
                    bb = if ((v24 as i32) < (self.r32(fp.wrapping_add(24)) as i32)) {
                        42
                    } else {
                        44
                    };
                }
                44 => {
                    bb = 41;
                }
                45 => {
                    bb = if ((v23 < 400_u32) && (v24 < 20_u32)) {
                        47
                    } else {
                        48
                    };
                }
                46 => {
                    v24 = v24.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    v21 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 43;
                }
                47 => {
                    {
                        let a0 = (v17
                            .wrapping_add(4_i32.wrapping_mul({
                                let t1 = (self.r32((v17.wrapping_add(7400_i32) as u32)) as i32);
                                self.w32(
                                    (v17.wrapping_add(7400_i32) as u32),
                                    (t1.wrapping_add(1) as u32),
                                );
                                t1
                            }))
                            .wrapping_add(6800_i32) as u32);
                        let a1 = ((v23.wrapping_add(v24).wrapping_add(400_u32) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    v25 = (self.r32(
                        (v17 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v23.wrapping_add(v24)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v25 > 4_i32) { 49 } else { 50 };
                }
                48 => {
                    bb = 46;
                }
                49 => {
                    let _ = self.f_1000a0a0(
                        this,
                        (self.r32(fp.wrapping_add(40)) as i32),
                        v25.wrapping_sub(5_i32),
                        72_i32,
                    );
                    bb = 50;
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    bb = if ((self.r32(fp.wrapping_add(44)) as i32) != 0) {
                        54
                    } else {
                        56
                    };
                }
                52 => {
                    v31 = self.r32(fp.wrapping_add(52));
                    self.w32(
                        self.r32(fp.wrapping_add(52)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        v31.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(28)) as i32) as u32),
                    );
                    v32 = (self.r32(fp.wrapping_add(60)) as i32);
                    self.w32(v31.wrapping_add(1540), 1_u32);
                    self.w32(v31.wrapping_add(1552), (v32 as u32));
                    self.w32(v31.wrapping_add(1464), 1_u32);
                    v33 = (self.r32(fp.wrapping_add(40)) as i32);
                    self.w32(
                        v31.wrapping_add(2324),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(v31.wrapping_add(1468), (v33 as u32));
                    self.w32(v31.wrapping_add(1556), (v28 as u32));
                    self.w32(v31.wrapping_add(1472), 12_u32);
                    self.w32(v31.wrapping_add(1476), 4_u32);
                    self.w32(
                        v31.wrapping_add(2328),
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        v31.wrapping_add(2332),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w8(a2, 1_u8);
                    return 0_i32;
                }
                53 => {
                    bb = if (v30 >= v29) { 80 } else { 82 };
                }
                54 => {
                    let t2 = (self.r32(fp.wrapping_add(44)) as i32);
                    bb = match t2 {
                        1_i32 => 58,
                        2_i32 => 59,
                        3_i32 => 60,
                        _ => 61,
                    };
                }
                55 => {
                    bb = 52;
                }
                56 => {
                    bb = if (v26 <= 0_i32) { 77 } else { 78 };
                }
                57 => {
                    bb = 55;
                }
                58 => {
                    bb = if (v27 >= 0_i32) { 62 } else { 63 };
                }
                59 => {
                    bb = if (v26 >= 0_i32) { 67 } else { 68 };
                }
                60 => {
                    bb = if (v27 <= 0_i32) { 71 } else { 72 };
                }
                61 => {
                    bb = 75;
                }
                62 => {
                    bb = 64;
                }
                63 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 57;
                }
                64 => {
                    bb = if (v28 == (self.r32(fp.wrapping_add(44)) as i32)) {
                        83
                    } else {
                        84
                    };
                }
                65 => {
                    bb = 63;
                }
                66 => {
                    bb = 59;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v28 = (if (v27 >= 0_i32) {
                        1_i32
                    } else {
                        (self.r32(fp.wrapping_add(44)) as i32).wrapping_add(1_i32)
                    });
                    bb = 57;
                }
                69 => {
                    bb = 68;
                }
                70 => {
                    bb = 60;
                }
                71 => {
                    bb = 64;
                }
                72 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 57;
                }
                73 => {
                    bb = 72;
                }
                74 => {
                    bb = 61;
                }
                75 => {
                    v28 = (1_i32).wrapping_neg();
                    bb = 84;
                }
                76 => {
                    bb = 57;
                }
                77 => {
                    bb = 64;
                }
                78 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 55;
                }
                79 => {
                    bb = 78;
                }
                80 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 81;
                }
                81 => {
                    bb = 64;
                }
                82 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 81;
                }
                83 => {
                    bb = 75;
                }
                84 => {
                    bb = 52;
                }
                85 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    /// `sub_10021240` (44 bytes).
    pub(crate) fn f_10021240(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: i32 = 0;
        v2 = (self
            .r32(((self.r32(((self.r32(this) as i32) as u32)) as i32).wrapping_add(28_i32) as u32))
            as i32);
        let _ = self.f_10020da0(this, a2);
        self.w32((v2.wrapping_add(1472_i32) as u32), 14_u32);
        self.w32((v2.wrapping_add(1476_i32) as u32), 2_u32);
        return 0_i32;
    }

    /// `sub_10021270` (1169 bytes).
    pub(crate) fn f_10021270(&mut self, mut this: u32, mut a2: u32) -> i32 {
        let fp = self.enter(1680);
        let r = self.f_10021270_body(fp, this, a2);
        self.leave(1680);
        r
    }

    fn f_10021270_body(&mut self, fp: u32, mut this: u32, mut a2: u32) -> i32 {
        let mut v2: u32 = 0;
        let mut v3: i32 = 0;
        let mut v4: u32 = 0;
        let mut v5: i32 = 0;
        let mut v6: u32 = 0;
        let mut v7: i32 = 0;
        let mut v8: i32 = 0;
        let mut v9: u32 = 0;
        let mut v10: u32 = 0;
        let mut v11: bool = false;
        let mut v12: i32 = 0;
        let mut v13: u32 = 0;
        let mut v14: u32 = 0;
        let mut v15: i32 = 0;
        let mut v16: i32 = 0;
        let mut v17: i32 = 0;
        let mut v18: i32 = 0;
        let mut v19: i32 = 0;
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
        let mut v31: u32 = 0;
        let mut v32: i32 = 0;
        let mut v33: i32 = 0;
        let mut bb: u32 = 0;
        loop {
            match bb {
                0 => {
                    v2 = self.r32(((self.r32(this) as i32) as u32));
                    self.w32(fp.wrapping_add(8), 20_u32);
                    v3 = (self.r32(v2.wrapping_add(12)) as i32);
                    self.w32(
                        fp.wrapping_add(24),
                        ((self.r32(v2.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(52),
                        ((self.r32(v2.wrapping_add(28)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(40),
                        ((self.r32(self.r32(fp.wrapping_add(52)).wrapping_add(1388)) as i32)
                            as u32),
                    );
                    self.w32(
                        fp.wrapping_add(28),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(384_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(20),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(380_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(44),
                        ((self.r32(
                            (v3.wrapping_add(
                                716_i32.wrapping_mul((self.r32(fp.wrapping_add(40)) as i32)),
                            )
                            .wrapping_add(388_i32) as u32),
                        ) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(36),
                        ((self.r32(this.wrapping_add(236)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(12),
                        ((self.r32(self.r32(fp.wrapping_add(36))) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(56),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(48),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(64),
                        ((self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(12)) as i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(16),
                        self.r32(fp.wrapping_add(36)).wrapping_add(16),
                    );
                    self.w32(
                        fp.wrapping_add(60),
                        ((self.r32(fp.wrapping_add(20)) as i32).wrapping_add(
                            20_i32.wrapping_mul((self.r32(fp.wrapping_add(28)) as i32)),
                        ) as u32),
                    );
                    let _ = self.memset(fp.wrapping_add(68), 0_i32 as u8, 1600_u32);
                    v4 = (v3.wrapping_add(3600_i32) as u32);
                    self.w32(fp.wrapping_add(4), (v3.wrapping_add(3600_i32) as u32));
                    bb = 1;
                }
                1 => {
                    bb = if (((self.r32(v4) as i32) == 1_i32)
                        && ((self.r32(v4.wrapping_add(116)) as i32) != 99_i32))
                    {
                        4
                    } else {
                        5
                    };
                }
                2 => {
                    bb = if (!v11) { 1 } else { 3 };
                }
                3 => {
                    v12 = 0_i32;
                    self.w32(fp.wrapping_add(16), 1_u32);
                    self.w32(fp.wrapping_add(4), 9999_u32);
                    self.w32(fp.wrapping_add(8), 9999_u32);
                    self.w32(fp.wrapping_add(12), 0_u32);
                    self.w32(fp.wrapping_add(0), fp.wrapping_add(68));
                    bb = 18;
                }
                4 => {
                    v5 = (self.r32(fp.wrapping_add(12)) as i32)
                        .wrapping_add((self.r32(v4.wrapping_add(124)) as i32));
                    v6 = self.r32(fp.wrapping_add(16));
                    v7 = (self.r32(fp.wrapping_add(56)) as i32)
                        .wrapping_add((self.r32(v4.wrapping_add(128)) as i32));
                    v8 = (self.r32(fp.wrapping_add(48)) as i32).wrapping_add(v5);
                    bb = if (v7 < v7.wrapping_add((self.r32(fp.wrapping_add(64)) as i32))) {
                        6
                    } else {
                        7
                    };
                }
                5 => {
                    v4 = v4.wrapping_add(156);
                    v11 = ((self.r32(fp.wrapping_add(8)) as i32) == 1_i32);
                    self.w32(fp.wrapping_add(4), v4);
                    self.w32(
                        fp.wrapping_add(8),
                        ((self.r32(fp.wrapping_add(8)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 2;
                }
                6 => {
                    v9 = (20_i32.wrapping_mul(v7) as u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(64)) as i32) as u32),
                    );
                    bb = 8;
                }
                7 => {
                    bb = 5;
                }
                8 => {
                    v10 = (v5 as u32);
                    bb = if (v5 < v8) { 11 } else { 12 };
                }
                9 => {
                    bb = if ((self.r32(fp.wrapping_add(0)) as i32) != 0) {
                        8
                    } else {
                        10
                    };
                }
                10 => {
                    bb = 7;
                }
                11 => {
                    bb = 13;
                }
                12 => {
                    v9 = v9.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(0),
                        ((self.r32(fp.wrapping_add(0)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 9;
                }
                13 => {
                    bb = if ((((self.r32(v6) as i32) != 0) && (v9 < 400_u32)) && (v10 < 20_u32)) {
                        16
                    } else {
                        17
                    };
                }
                14 => {
                    bb = if ((v10 as i32) < v8) { 13 } else { 15 };
                }
                15 => {
                    v4 = self.r32(fp.wrapping_add(4));
                    bb = 12;
                }
                16 => {
                    self.w32(
                        fp.wrapping_add(68)
                            .wrapping_add(((v9.wrapping_add(v10) as i32) as u32).wrapping_mul(4)),
                        ((self.r32(
                            fp.wrapping_add(68).wrapping_add(
                                ((v9.wrapping_add(v10) as i32) as u32).wrapping_mul(4),
                            ),
                        ) as i32)
                            .wrapping_add((self.r32(v6) as i32)) as u32),
                    );
                    bb = 17;
                }
                17 => {
                    v6 = v6.wrapping_add(4);
                    v10 = v10.wrapping_add(1);
                    bb = 14;
                }
                18 => {
                    v13 = (crem_i32(v12, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs() as u32);
                    v14 = (cdiv_i32(v12, 20_i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs() as u32);
                    v15 = (v14.wrapping_add(v13) as i32);
                    v16 = (v13.wrapping_sub(v14) as i32).wrapping_abs();
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32)
                        < (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                    {
                        21
                    } else {
                        22
                    };
                }
                19 => {
                    bb = if (v12 < 400_i32) { 18 } else { 20 };
                }
                20 => {
                    self.w32(
                        fp.wrapping_add(8),
                        (crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32) as u32),
                    );
                    self.w32(
                        fp.wrapping_add(4),
                        (cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32) as u32),
                    );
                    let _ = self.f_10006940(this, fp.wrapping_add(36), 0_i32, 0_i32, 8_i32, 1_i32);
                    v17 = (self.r32(fp.wrapping_add(24)) as i32);
                    v18 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(12)) as i32);
                    v19 = (self.r32(self.r32(fp.wrapping_add(36))) as i32)
                        .wrapping_add(crem_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32));
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(36)).wrapping_add(16),
                    );
                    v20 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(4)) as i32)
                        .wrapping_add(cdiv_i32((self.r32(fp.wrapping_add(12)) as i32), 20_i32));
                    v21 = (self.r32(self.r32(fp.wrapping_add(36)).wrapping_add(8)) as i32)
                        .wrapping_add(v19);
                    self.w32(
                        ((self.r32(fp.wrapping_add(24)) as i32).wrapping_add(7400_i32) as u32),
                        0_u32,
                    );
                    self.w32(fp.wrapping_add(24), (v21 as u32));
                    v22 = v20.wrapping_add(v18);
                    bb = if (v20 < v20.wrapping_add(v18)) {
                        35
                    } else {
                        36
                    };
                }
                21 => {
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(self.r32(fp.wrapping_add(0))) as i32) as u32),
                    );
                    self.w32(fp.wrapping_add(4), ((v14.wrapping_add(v13) as i32) as u32));
                    bb = 23;
                }
                22 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32)
                        != (self.r32(self.r32(fp.wrapping_add(0))) as i32))
                    {
                        26
                    } else {
                        27
                    };
                }
                23 => {
                    self.w32(fp.wrapping_add(8), (v16 as u32));
                    self.w32(fp.wrapping_add(12), (v12 as u32));
                    bb = 24;
                }
                24 => {
                    v12 = v12.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    bb = 19;
                }
                25 => {
                    bb = 22;
                }
                26 => {
                    bb = 24;
                }
                27 => {
                    bb = if (v15 < (self.r32(fp.wrapping_add(4)) as i32)) {
                        29
                    } else {
                        30
                    };
                }
                28 => {
                    bb = 27;
                }
                29 => {
                    self.w32(fp.wrapping_add(4), ((v14.wrapping_add(v13) as i32) as u32));
                    bb = 23;
                }
                30 => {
                    bb = if ((v15 == (self.r32(fp.wrapping_add(4)) as i32))
                        && (v16 < (self.r32(fp.wrapping_add(8)) as i32)))
                    {
                        32
                    } else {
                        33
                    };
                }
                31 => {
                    bb = 30;
                }
                32 => {
                    bb = 23;
                }
                33 => {
                    bb = 24;
                }
                34 => {
                    bb = 33;
                }
                35 => {
                    v23 = (20_i32.wrapping_mul(v20) as u32);
                    self.w32(fp.wrapping_add(16), (v22.wrapping_sub(v20) as u32));
                    bb = 37;
                }
                36 => {
                    v26 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32));
                    v27 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32));
                    v28 = (self.r32(fp.wrapping_add(44)) as i32);
                    v29 = (self.r32(fp.wrapping_add(4)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(28)) as i32))
                        .wrapping_abs();
                    v30 = (self.r32(fp.wrapping_add(8)) as i32)
                        .wrapping_sub((self.r32(fp.wrapping_add(20)) as i32))
                        .wrapping_abs();
                    bb = if (v30 == v29) { 51 } else { 53 };
                }
                37 => {
                    v24 = (v19 as u32);
                    bb = if (v19 < v21) { 40 } else { 41 };
                }
                38 => {
                    bb = if ((self.r32(fp.wrapping_add(16)) as i32) != 0) {
                        37
                    } else {
                        39
                    };
                }
                39 => {
                    bb = 36;
                }
                40 => {
                    bb = 42;
                }
                41 => {
                    v23 = v23.wrapping_add(20_u32);
                    self.w32(
                        fp.wrapping_add(16),
                        ((self.r32(fp.wrapping_add(16)) as i32).wrapping_sub(1) as u32),
                    );
                    bb = 38;
                }
                42 => {
                    bb = if ((self.r32(self.r32(fp.wrapping_add(0))) as i32) != 0) {
                        45
                    } else {
                        46
                    };
                }
                43 => {
                    bb = if ((v24 as i32) < (self.r32(fp.wrapping_add(24)) as i32)) {
                        42
                    } else {
                        44
                    };
                }
                44 => {
                    bb = 41;
                }
                45 => {
                    bb = if ((v23 < 400_u32) && (v24 < 20_u32)) {
                        47
                    } else {
                        48
                    };
                }
                46 => {
                    v24 = v24.wrapping_add(1);
                    self.w32(
                        fp.wrapping_add(0),
                        self.r32(fp.wrapping_add(0)).wrapping_add(4),
                    );
                    v21 = (self.r32(fp.wrapping_add(24)) as i32);
                    bb = 43;
                }
                47 => {
                    {
                        let a0 = (v17
                            .wrapping_add(4_i32.wrapping_mul({
                                let t1 = (self.r32((v17.wrapping_add(7400_i32) as u32)) as i32);
                                self.w32(
                                    (v17.wrapping_add(7400_i32) as u32),
                                    (t1.wrapping_add(1) as u32),
                                );
                                t1
                            }))
                            .wrapping_add(6800_i32) as u32);
                        let a1 = ((v23.wrapping_add(v24).wrapping_add(400_u32) as i32) as u32);
                        self.w32(a0, a1)
                    };
                    v25 = (self.r32(
                        (v17 as u32)
                            .wrapping_add(4_u32.wrapping_mul(v23.wrapping_add(v24)))
                            .wrapping_add(3200_u32),
                    ) as i32);
                    bb = if (v25 > 4_i32) { 49 } else { 50 };
                }
                48 => {
                    bb = 46;
                }
                49 => {
                    let _ = self.f_1000a0a0(
                        this,
                        (self.r32(fp.wrapping_add(40)) as i32),
                        v25.wrapping_sub(5_i32),
                        74_i32,
                    );
                    bb = 50;
                }
                50 => {
                    bb = 48;
                }
                51 => {
                    bb = if ((self.r32(fp.wrapping_add(44)) as i32) != 0) {
                        54
                    } else {
                        56
                    };
                }
                52 => {
                    v31 = self.r32(fp.wrapping_add(52));
                    self.w32(
                        self.r32(fp.wrapping_add(52)).wrapping_add(1544),
                        ((self.r32(fp.wrapping_add(20)) as i32) as u32),
                    );
                    self.w32(
                        v31.wrapping_add(1548),
                        ((self.r32(fp.wrapping_add(28)) as i32) as u32),
                    );
                    v32 = (self.r32(fp.wrapping_add(60)) as i32);
                    self.w32(v31.wrapping_add(1540), 1_u32);
                    self.w32(v31.wrapping_add(1552), (v32 as u32));
                    self.w32(v31.wrapping_add(1464), 1_u32);
                    v33 = (self.r32(fp.wrapping_add(40)) as i32);
                    self.w32(
                        v31.wrapping_add(2324),
                        ((self.r32(fp.wrapping_add(8)) as i32) as u32),
                    );
                    self.w32(v31.wrapping_add(1468), (v33 as u32));
                    self.w32(v31.wrapping_add(1556), (v28 as u32));
                    self.w32(v31.wrapping_add(1472), 14_u32);
                    self.w32(v31.wrapping_add(1476), 4_u32);
                    self.w32(
                        v31.wrapping_add(2328),
                        ((self.r32(fp.wrapping_add(4)) as i32) as u32),
                    );
                    self.w32(
                        v31.wrapping_add(2332),
                        ((self.r32(fp.wrapping_add(12)) as i32) as u32),
                    );
                    self.w8(a2, 1_u8);
                    return 0_i32;
                }
                53 => {
                    bb = if (v30 >= v29) { 80 } else { 82 };
                }
                54 => {
                    let t2 = (self.r32(fp.wrapping_add(44)) as i32);
                    bb = match t2 {
                        1_i32 => 58,
                        2_i32 => 59,
                        3_i32 => 60,
                        _ => 61,
                    };
                }
                55 => {
                    bb = 52;
                }
                56 => {
                    bb = if (v26 <= 0_i32) { 77 } else { 78 };
                }
                57 => {
                    bb = 55;
                }
                58 => {
                    bb = if (v27 >= 0_i32) { 62 } else { 63 };
                }
                59 => {
                    bb = if (v26 >= 0_i32) { 67 } else { 68 };
                }
                60 => {
                    bb = if (v27 <= 0_i32) { 71 } else { 72 };
                }
                61 => {
                    bb = 75;
                }
                62 => {
                    bb = 64;
                }
                63 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 57;
                }
                64 => {
                    bb = if (v28 == (self.r32(fp.wrapping_add(44)) as i32)) {
                        83
                    } else {
                        84
                    };
                }
                65 => {
                    bb = 63;
                }
                66 => {
                    bb = 59;
                }
                67 => {
                    bb = 64;
                }
                68 => {
                    v28 = (if (v27 >= 0_i32) {
                        1_i32
                    } else {
                        (self.r32(fp.wrapping_add(44)) as i32).wrapping_add(1_i32)
                    });
                    bb = 57;
                }
                69 => {
                    bb = 68;
                }
                70 => {
                    bb = 60;
                }
                71 => {
                    bb = 64;
                }
                72 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 57;
                }
                73 => {
                    bb = 72;
                }
                74 => {
                    bb = 61;
                }
                75 => {
                    v28 = (1_i32).wrapping_neg();
                    bb = 84;
                }
                76 => {
                    bb = 57;
                }
                77 => {
                    bb = 64;
                }
                78 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 55;
                }
                79 => {
                    bb = 78;
                }
                80 => {
                    v28 = (if (v27 >= 0_i32) { 1_i32 } else { 3_i32 });
                    bb = 81;
                }
                81 => {
                    bb = 64;
                }
                82 => {
                    v28 = (if (v26 >= 0_i32) { 2_i32 } else { 0_i32 });
                    bb = 81;
                }
                83 => {
                    bb = 75;
                }
                84 => {
                    bb = 52;
                }
                85 => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }
}
