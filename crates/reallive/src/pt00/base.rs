//! Shared code: geometry helpers, the projection functions (calls 10-12),
//! the ball (calls 30/31), the bat (call 50), the field and camera
//! (calls 60/61) and the actor base class.

use super::{BALL, FBASE, FIELD, Pt00, ftol};

const HALF_PI: f64 = 1.57079632675;
const PI: f64 = 3.1415926535;
const THREE_HALF_PI: f64 = 4.71238898025;

/// `sub_100019A0`: normalises (x, y); zero stays zero.
pub(crate) fn normalize(x: &mut f64, y: &mut f64) {
    let length = (*y * *y + *x * *x).sqrt();
    if length == 0.0 {
        *x = 0.0;
        *y = 0.0;
    } else {
        *x /= length;
        *y /= length;
    }
}

/// `sub_100019F0`.
pub(crate) fn length3(x: f64, y: f64, z: f64) -> f64 {
    (x * x + y * y + z * z).sqrt()
}

/// `sub_10001A10`: distance between (x1, y1) and (x2, y2).
pub(crate) fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1)).sqrt()
}

/// Closest-point result: the point, the vector from the query point to
/// it, and its length.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Foot {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub distance: f64,
}

/// `sub_10001A40`: perpendicular foot of (px, py) on the line through
/// (x1, y1)-(x2, y2).
pub(crate) fn line_foot(x1: f64, y1: f64, x2: f64, y2: f64, px: f64, py: f64) -> Foot {
    let c = (-HALF_PI).cos();
    let s = (-HALF_PI).sin();
    let mut nx = (x2 - x1) * c - (y2 - y1) * s;
    let mut ny = s * (x2 - x1) + (y2 - y1) * c;
    normalize(&mut nx, &mut ny);
    let d = (px - x1) * nx + (py - y1) * ny;
    let x = px - d * nx;
    let y = py - d * ny;
    Foot {
        x,
        y,
        dx: x - px,
        dy: y - py,
        distance: d.abs(),
    }
}

/// `sub_10001AF0`: closest point of the segment (x1, y1)-(x2, y2).
pub(crate) fn segment_foot(x1: f64, y1: f64, x2: f64, y2: f64, px: f64, py: f64) -> Foot {
    let end = |x: f64, y: f64| {
        let (dx, dy) = (x - px, y - py);
        Foot {
            x,
            y,
            dx,
            dy,
            distance: length3(dx, dy, 0.0),
        }
    };
    // (The degenerate-segment result is overwritten by the general case,
    // exactly as in the DLL.)
    let foot = line_foot(x1, y1, x2, y2, px, py);
    let fx = foot.x;
    let first = if x1 < fx {
        x2 <= x1
    } else {
        x1 < x2 || x1 <= fx
    };
    if first {
        return end(x1, y1);
    }
    let second = if fx > x2 {
        if x1 >= x2 {
            return foot;
        }
        true
    } else {
        x2 <= x1 || x2 <= fx
    };
    if second { end(x2, y2) } else { foot }
}

/// `sub_10001CD0`: intersection of the lines through (a1, a2)-(a3, a4)
/// and (a5, a6)-(a7, a8), with the unit vector and distance from
/// (a5, a6) to it.
pub(crate) fn line_intersection(a: [f64; 8]) -> Option<Foot> {
    let [a1, a2, a3, a4, a5, a6, a7, a8] = a;
    let m1 = if a1 != a3 { (a4 - a2) / (a3 - a1) } else { 0.0 };
    let m2 = if a5 == a7 { 0.0 } else { (a8 - a6) / (a7 - a5) };
    let (x, y) = if a1 != a3 {
        let x = if a5 == a7 {
            a5
        } else {
            if m1 == m2 {
                return None;
            }
            (m1 * a1 - a2 - m2 * a5 + a6) / (m1 - m2)
        };
        (x, (x - a1) * m1 + a2)
    } else if a5 != a7 {
        (a1, (a1 - a5) * m2 + a6)
    } else {
        return None;
    };
    let (dx0, dy0) = (x - a5, y - a6);
    let (mut dx, mut dy) = (dx0, dy0);
    normalize(&mut dx, &mut dy);
    Some(Foot {
        x,
        y,
        dx,
        dy,
        distance: (dy0 * dy0 + dx0 * dx0).sqrt(),
    })
}

/// `sub_10001E40`: segment intersection.
pub(crate) fn segment_intersection(a: [f64; 8]) -> Option<Foot> {
    let [a1, a2, a3, a4, a5, a6, a7, a8] = a;
    let foot = line_intersection(a)?;
    let (x, y) = (foot.x, foot.y);
    let outside = |lo: f64, hi: f64, v: f64| (lo > v || v > hi) && (hi > v || v > lo);
    if outside(a1, a3, x) || outside(a2, a4, y) || outside(a5, a7, x) || outside(a6, a8, y) {
        return None;
    }
    Some(foot)
}

/// `sub_10001FA0`: one of eight directions for a vector (`-1` for none).
/// Angles above 3600 leave `current` unchanged.
pub(crate) fn direction8(x: f64, y: f64, current: i32) -> i32 {
    let angle = if x == 0.0 {
        if y == 0.0 {
            return -1;
        }
        if y < 0.0 { PI } else { 0.0 }
    } else {
        let a = (y / x).atan();
        if x >= 0.0 {
            HALF_PI - a
        } else {
            THREE_HALF_PI - a
        }
    };
    let tenths = angle * 572.9577951471996;
    match tenths {
        t if t <= 225.0 => 0,
        t if t <= 675.0 => 1,
        t if t <= 1125.0 => 2,
        t if t <= 1575.0 => 3,
        t if t <= 2025.0 => 4,
        t if t <= 2475.0 => 5,
        t if t <= 2925.0 => 6,
        t if t <= 3375.0 => 7,
        t if t <= 3600.0 => 0,
        _ => current,
    }
}

/// `sub_100021A0`: 100 beyond 4, else -100.
pub(crate) fn f_100021a0(a1: i32) -> i32 {
    if a1 > 4 { 100 } else { -100 }
}

/// `sub_100021C0`: which side of the line through (x, z) with direction
/// (dx, dz) the point (px, pz) lies on: 1, -1, or 0 on it.
pub(crate) fn side(x: f64, z: f64, dx: f64, dz: f64, px: f64, pz: f64) -> i32 {
    if dx == 0.0 {
        if px == 0.0 {
            return 0;
        }
        return if px > 0.0 { 1 } else { -1 };
    }
    let line = (px - x) * (dz / dx) + z;
    if pz == line {
        return 0;
    }
    if dx < 0.0 {
        if pz > line { 1 } else { -1 }
    } else if pz < line {
        1
    } else {
        -1
    }
}

/// `sub_100022B0`: a parabolic arc: `a4` at `a1 == a3`, 0 at `a2`.
pub(crate) fn arc(a1: i32, a2: i32, a3: i32, a4: i32) -> i32 {
    let span = f64::from(a2 - a3);
    if span == 0.0 {
        return 0;
    }
    let t = f64::from(a1 - a3);
    ftol((1.0 - t * t / (span * span)) * f64::from(a4))
}

type Mat3 = [f64; 9];

const IDENTITY: Mat3 = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

/// `sub_100023C0`.
fn mat_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut m = [0.0; 9];
    for r in 0..3 {
        for c in 0..3 {
            m[3 * r + c] = a[3 * r] * b[c] + a[3 * r + 1] * b[3 + c] + a[3 * r + 2] * b[6 + c];
        }
    }
    m
}

/// `sub_100024D0` (x), `sub_10002560` (y), `sub_10002600` (z).
fn rotate_x(m: &Mat3, a: f64) -> Mat3 {
    let (c, s) = (a.cos(), a.sin());
    mat_mul(m, &[1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c])
}

fn rotate_y(m: &Mat3, a: f64) -> Mat3 {
    let (c, s) = (a.cos(), a.sin());
    mat_mul(m, &[c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c])
}

fn rotate_z(m: &Mat3, a: f64) -> Mat3 {
    let (c, s) = (a.cos(), a.sin());
    mat_mul(m, &[c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0])
}

/// `sub_10002690`.
fn transform(m: &Mat3, v: [f64; 3]) -> [f64; 3] {
    [
        m[2] * v[2] + m[1] * v[1] + m[0] * v[0],
        m[3] * v[0] + m[5] * v[2] + m[4] * v[1],
        m[6] * v[0] + m[8] * v[2] + m[7] * v[1],
    ]
}

impl Pt00 {
    // ---- projection (calls 10-12) -----------------------------------------------

    /// `sub_10002710`: projects (x, 0, z) to the screen (`F[1900..=1902]`)
    /// and gives the screen heights of y = `y0` and y = `y1`
    /// (`F[1903]`, `F[1904]`).
    pub(crate) fn f_10002710(&mut self, x: i32, y0: i32, z: i32, y1: i32) {
        let m = rotate_z(&IDENTITY, 0.0);
        let m = rotate_y(&m, 0.0);
        let m = rotate_x(&m, -0.4363323129861111);
        let project = |y: i32| {
            let v = [f64::from(x), f64::from(y) - 2000.0, f64::from(z) - 1000.0];
            transform(&m, v)
        };
        let v = project(0);
        let sy = ftol(v[1] * 400.0 / v[2]);
        let depth = ftol(v[2]);
        self.sf(1900, 620 - ftol(v[0] * -850.0 / v[2]));
        self.sf(1901, 176 - sy);
        self.sf(1902, depth);
        let v = project(y0);
        let base = self.f(1901);
        self.sf(1903, 176 - ftol(v[1] * 400.0 / v[2]) - base);
        let v = project(y1);
        self.sf(1904, 176 - ftol(v[1] * 400.0 / v[2]) - base);
    }

    /// `sub_10002990`: the overhead map projection.
    pub(crate) fn f_10002990(&mut self, a2: i32, a3: i32, a4: i32, a5: i32) {
        self.sf(1902, 0);
        self.sf(1900, a2 / 10 + 1300);
        self.sf(1901, 1100 - a4 / 10);
        self.sf(1903, a3 / -10);
        self.sf(1904, a5 / -10);
    }

    // ---- the ball (calls 30/31) ---------------------------------------------------

    /// `sub_10002A30`: clears both ball records and the prediction.
    pub(crate) fn f_10002a30(&mut self) {
        self.sf(210, 0);
        for i in 220..=246 {
            self.sf(i, 0);
        }
        self.sf(221, 255);
        for i in 250..=280 {
            self.sf(i, 0);
        }
        self.sf(251, 255);
        self.fill(BALL, 0x4B0, 0);
        self.sb(BALL + 1204, 0);
        self.si(BALL + 1208, 0);
        self.si(BALL + 1212, 0);
    }

    /// `sub_10002BD0`: throws the ball (record `F[250]`) from (x, y, z)
    /// in direction (dx, dy, dz)/1000 at `speed`, rising at `lift`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn f_10002bd0(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        dx: i32,
        dy: i32,
        dz: i32,
        speed: i32,
        lift: i32,
    ) {
        let r = 250;
        self.sf(r, 1);
        self.sf(r + 1, 255);
        self.sf(r + 2, 0);
        for base in [r + 3, r + 6, r + 9] {
            self.sf(base, x);
            self.sf(base + 1, y);
            self.sf(base + 2, z);
        }
        self.sf(r + 12, dx);
        self.sf(r + 13, dy);
        self.sf(r + 14, dz);
        self.sf(r + 15, speed);
        let (px, py, pz) = (self.f(r + 6), self.f(r + 7), self.f(r + 8));
        self.sf(r + 21, 0);
        self.sf(r + 22, 0);
        self.sf(r + 16, 2000 * (5 * px));
        self.sf(r + 17, 10000 * py);
        self.sf(r + 20, lift);
        self.sf(r + 18, 10000 * pz);
        self.sf(r + 19, self.f(r + 4));
        for i in 23..=27 {
            self.sf(r + i, 0);
        }
        self.fill(BALL, 0x4B0, 0);
    }

    /// `sub_10002CD0`: advances the ball by one frame.
    pub(crate) fn f_10002cd0(&mut self) {
        self.f_10002e20();
        let state = self.f(210);
        if state == 1 {
            if self.f(228) < 1500 {
                self.sf(210, 0);
                self.sf(20, 3);
            }
        } else if state == 2 && self.b(BALL + 1204) == 0 {
            // A fielder holding the ball?
            let held = (0..20).any(|n| self.f(1006 + 40 * n) == 2);
            if self.i(0x1002_29DC) == 0 && !held {
                let limit = if self.f(76) != 7 { 12000 } else { 2000 };
                let away =
                    distance(0.0, 2000.0, f64::from(self.f(256)), f64::from(self.f(258))) as i64;
                if self.f(265) == 0 || away > i64::from(limit) || self.f(258) < 1000 {
                    self.f_10003b10();
                }
            }
        }
        if self.b(BALL + 1204) == 0 {
            return;
        }
        let count = self.i(BALL + 1212) + 1;
        self.si(BALL + 1212, count);
        if count == self.i(BALL + 1208) - 30 {
            self.sf(30, 7);
        }
        if self.i(BALL + 1212) >= self.i(BALL + 1208) {
            self.f_10003ae0();
            self.sb(BALL + 1204, 0);
            self.si(BALL + 1208, 0);
            self.si(BALL + 1212, 0);
        }
    }

    /// `sub_10002E20`.
    fn f_10002e20(&mut self) {
        match self.f(210) {
            1 => self.f_10002e50(),
            2 => self.f_10003180(),
            _ => {}
        }
    }

    /// `sub_10002E50`: the pitch in flight (record `F[220]`), mirrored
    /// into `F[250]`.
    fn f_10002e50(&mut self) {
        let bounced = self.f_10002ec0(FBASE + 880);
        if bounced {
            let v = self.f(220 + 23);
            if v > 10 && self.f(220) == 1 {
                self.f_10014040(1, 10 * v - 10);
            }
        }
        self.f_10003140(FBASE + 880, FBASE + 1000);
    }

    /// `sub_10002EC0`: moves a pitched ball; true when it bounced.
    fn f_10002ec0(&mut self, a2: u32) -> bool {
        let at = |i: u32| a2 + 4 * i;
        let mut bounced = false;
        let v4 = self.i(at(15));
        let v5 = self.i(at(19));
        self.si(at(9), self.i(at(6)));
        self.si(at(10), self.i(at(7)));
        self.si(at(11), self.i(at(8)));
        let v6 = v4 * self.i(at(12)) / 100 + v5;
        let v7 = v4 * self.i(at(14)) / 100 + self.i(at(21));
        self.si(at(19), v6);
        self.si(at(16), v6);
        let v8 = v6 / 100;
        self.si(at(21), v7);
        self.si(at(18), v7);
        let mut hi = v7 / 100;
        self.si(at(6), v8);
        self.si(at(8), v7 / 100);
        // Breaking pitches, by type (F[500]).
        let kind = self.f(500);
        if kind == 2 || kind == 3 {
            let d = f64::from(v7 / 100) - 3000.0;
            let bend = ftol(60.0 - d * d * 0.00005999999999999999);
            let x = if kind == 2 { v8 - bend } else { bend + v8 };
            hi = v7 / 100;
            self.si(at(6), x);
        }
        let kind = self.f(500);
        if kind == 4 || kind == 5 {
            let bend = if hi <= 2000 {
                (hi - 2000) / 2
            } else {
                let q = 4000.0 - f64::from((hi - 4000).wrapping_mul(hi - 4000)) * 0.0005 - 3000.0;
                ftol(30.0 - q * q * 0.00003)
            };
            let x = self.i(at(6));
            self.si(at(6), if kind == 4 { x - bend } else { bend + x });
        }
        if self.f(500) == 6 {
            let jitter = self.rand() % 40 - 20;
            self.add(at(6), jitter);
            let jitter = self.rand() % 40 - 20;
            self.add(at(8), jitter);
        }
        let v16 = self.i(at(23));
        let v17 = self.i(at(24)) + 1;
        self.si(at(24), v17);
        self.si(at(25), 0);
        let mut height = f64::from(v17 * v16) - f64::from(v17) * f64::from(v17) * 0.3
            + f64::from(self.i(at(22)));
        if height < 0.0 {
            height = 0.0;
            let slow = 2 * (-4 - v16 / 20);
            let speed = self.i(at(15)) + slow;
            self.si(at(15), speed);
            if speed < 0 {
                self.si(at(15), 0);
            }
            self.si(at(22), 0);
            self.si(at(23), ftol(-(f64::from(v16) - f64::from(v17 - 1) * 0.6)));
            self.si(at(24), 0);
            let v = ftol(f64::from(self.i(at(23))) * 0.7);
            self.si(at(23), v);
            if v < 0 {
                self.si(at(23), 0);
            }
            bounced = true;
        }
        self.si(at(7), ftol(height));
        bounced
    }

    /// `sub_10003140`: copies the pitch record into the ball record
    /// (x doubled).
    fn f_10003140(&mut self, from: u32, to: u32) {
        self.si(to + 24, 2 * self.i(from + 24));
        self.si(to + 28, self.i(from + 28));
        self.si(to + 32, self.i(from + 32));
        self.si(to + 36, 2 * self.i(from + 36));
        self.si(to + 40, self.i(from + 40));
        self.si(to + 44, self.i(from + 44));
        self.si(to, self.i(from));
    }

    /// `sub_10003180`: the batted ball (record `F[250]`).
    fn f_10003180(&mut self) {
        let record = FBASE + 1000;
        let fence = FBASE + 1640;
        let old_lift = self.f(270);
        let mut flags = [0u8; 5];
        self.f_10003280(record, fence, true, &mut flags);
        if flags[3] != 0 {
            self.f_10014040(4, 255);
        } else if flags[4] != 0 {
            self.f_10014040(5, 255);
        } else if flags[0] != 0 {
            if flags[1] != 0 {
                let v = self.i(record + 80);
                if v > 30 && self.i(record) == 1 {
                    self.f_10014040(2, v - 30);
                }
            } else if flags[2] != 0 {
                if old_lift > 30 {
                    self.f_10014040(3, old_lift - 30);
                    self.si(record + 116, 1);
                }
            } else {
                let v = self.i(record + 80);
                if v > 30 && self.i(record) == 1 {
                    self.f_10014040(1, v - 30);
                }
            }
        }
        self.f_10003460(record, fence);
        self.f_100035f0(FBASE + 880, record);
    }

    /// `sub_10003280`: moves a batted ball one frame. `flags` receives
    /// bounced, on the slope, a fence hit, over the fence (home run) and
    /// out of the ground.
    fn f_10003280(&mut self, a2: u32, fence: u32, collide: bool, flags: &mut [u8; 5]) {
        let at = |i: u32| a2 + 4 * i;
        let v6 = self.i(at(18));
        self.si(at(9), self.i(at(6)));
        self.si(at(10), self.i(at(7)));
        self.si(at(11), self.i(at(8)));
        let speed = self.i(at(15));
        let x16 = speed
            .wrapping_mul(self.i(at(12)))
            .wrapping_add(self.i(at(16)));
        let z18 = speed.wrapping_mul(self.i(at(14))).wrapping_add(v6);
        self.si(at(16), x16);
        self.si(at(18), z18);
        let x = x16 / 10000;
        let z = z18 / 10000;
        self.si(at(6), x);
        self.si(at(8), z);
        let (slope, over) = {
            let mut s = [false; 2];
            self.f_10004310(x, z, Some(&mut s));
            (s[0], s[1])
        };
        flags[1] = u8::from(slope);
        flags[2] = u8::from(over);
        let mut ground = self.f(320);
        self.si(at(22), ground);
        if collide {
            self.f_10003640(a2, fence, flags, &mut ground);
            self.f_100038f0(a2, fence, flags);
        }
        let lift = self.i(at(20));
        let t = self.i(at(21)) + 1;
        self.si(at(21), t);
        let mut height = (f64::from(t.wrapping_mul(lift)) - f64::from(t) * f64::from(t) * 6.0)
            * 0.1
            + f64::from(self.i(at(19)));
        let floor = f64::from(ground);
        if height < floor {
            self.si(at(19), ground);
            if flags[3] == 0 {
                height = floor;
            }
            if flags[1] == 1 {
                self.add(at(15), 2 * (-4 - lift / 20));
            } else if flags[1] == 0 {
                self.add(at(15), -4 - lift / 20);
            }
            if self.i(at(15)) < 0 {
                self.si(at(15), 0);
            }
            let bounce = ftol(-(f64::from(lift) - f64::from(t - 1) * 12.0));
            let v = ftol(f64::from(bounce) * 0.7);
            self.si(at(20), v);
            if v < 0 {
                self.si(at(20), 0);
            }
            if flags[2] == 1 {
                self.si(at(20), 0);
                self.si(at(15), 0);
                self.si(at(0), 0);
            }
            self.si(at(21), 0);
            flags[0] = 1;
        }
        self.si(at(7), ftol(height));
    }

    /// `sub_10003460`: predicts 100 frames of the batted ball into the
    /// ball object and computes where it will land (`F[273..=277]`).
    fn f_10003460(&mut self, a2: u32, fence: u32) {
        // Simulate on copies, as the DLL does (31 words of the record,
        // 28 bytes of the fence state).
        let mut record = [0i32; 31];
        for (i, v) in record.iter_mut().enumerate() {
            *v = self.i(a2 + 4 * i as u32);
        }
        let mut fence_copy = [0i32; 7];
        for (i, v) in fence_copy.iter_mut().enumerate() {
            *v = self.i(fence + 4 * i as u32);
        }
        // Work in scratch memory (the heap end) to reuse the same code.
        let scratch = super::HEAP + 0xF000;
        let scratch_fence = scratch + 0x100;
        for (i, v) in record.iter().enumerate() {
            self.si(scratch + 4 * i as u32, *v);
        }
        for (i, v) in fence_copy.iter().enumerate() {
            self.si(scratch_fence + 4 * i as u32, *v);
        }
        let mut flags = [0u8; 5];
        for step in 0..100u32 {
            self.f_10003280(scratch, scratch_fence, true, &mut flags);
            let point = BALL + 12 * step;
            self.si(point, self.i(scratch + 24));
            self.si(point + 4, self.i(scratch + 28));
            self.si(point + 8, self.i(scratch + 32));
        }
        let at = |i: u32| a2 + 4 * i;
        let v8 = f64::from(self.i(at(20)));
        let v9 = self.i(at(22));
        let drop = 2 * (5 * (self.i(at(19)) - v9));
        let disc = v8 * v8 - f64::from(drop) * -24.0;
        let t = if disc < 0.0 {
            v8 * 0.1666666666666667
        } else {
            let root = disc.sqrt();
            let a = (-v8 - root) * -0.08333333333333333;
            let b = (root - v8) * -0.08333333333333333;
            if a <= b { b } else { a }
        };
        let v14 = self.i(at(21));
        let speed = self.i(at(15));
        let frames = ftol(t) - v14;
        let travel = speed.wrapping_mul(frames);
        let angle = (t * 12.0 - v8).atan2(f64::from(speed));
        self.si(at(26), frames);
        self.si(at(24), v9);
        let x = self
            .i(at(16))
            .wrapping_add(travel.wrapping_mul(self.i(at(12))));
        self.si(at(23), x / 10000);
        let z = self
            .i(at(18))
            .wrapping_add(travel.wrapping_mul(self.i(at(14))));
        self.si(at(25), z / 10000);
        self.si(at(27), ftol(angle * 57.29577951471995));
    }

    /// `sub_100035F0`: copies the ball record back into the pitch record.
    fn f_100035f0(&mut self, to: u32, from: u32) {
        self.si(to + 24, self.i(from + 24) / 2);
        self.si(to + 28, self.i(from + 28));
        self.si(to + 32, self.i(from + 32));
        self.si(to + 36, self.i(from + 36) / 2);
        self.si(to + 40, self.i(from + 40));
        self.si(to + 44, self.i(from + 44));
        self.si(to, self.i(from));
    }

    /// Reflects the ball's horizontal motion off a surface through
    /// `fence` (shared by the fence and the net).
    fn reflect(&mut self, a1: u32, fence: u32) -> bool {
        let v12 = self.i(a1 + 24);
        let mut v29 = f64::from(self.i(a1 + 12) - v12);
        let mut v30 = f64::from(self.i(a1 + 20) - self.i(a1 + 32));
        let mut v28 = f64::from(v12 - self.i(fence));
        let mut v27 = f64::from(self.i(a1 + 32) - self.i(fence + 8));
        normalize(&mut v29, &mut v30);
        normalize(&mut v28, &mut v27);
        let dot = v27 * v30 + v28 * v29;
        if dot < 0.0 {
            return false;
        }
        let mut rx = dot * v28 + dot * v28 - v29;
        let mut rz = dot * v27 + dot * v27 - v30;
        normalize(&mut rx, &mut rz);
        self.si(a1 + 12, self.i(a1 + 24));
        self.si(a1 + 16, self.i(a1 + 28));
        self.si(a1 + 20, self.i(a1 + 32));
        self.si(a1 + 48, ftol(rx * 1000.0));
        self.si(a1 + 52, 0);
        self.si(a1 + 56, ftol(rz * 1000.0));
        self.si(a1 + 60, ftol(f64::from(self.i(a1 + 60)) * 0.75));
        true
    }

    /// `sub_10003640`: the outfield fence.
    fn f_10003640(&mut self, a1: u32, fence: u32, flags: &mut [u8; 5], ground: &mut i32) {
        if self.i(fence + 12) == 1 {
            let n = self.i(fence + 20) + 1;
            self.si(fence + 20, n);
            if n > 60 {
                self.si(fence + 12, 0);
                self.si(fence + 20, 0);
            }
            return;
        }
        let dx = f64::from(self.i(a1 + 24) - self.i(fence));
        let dy = f64::from(self.i(a1 + 28) - self.i(fence + 4));
        let dz = f64::from(self.i(a1 + 32) - self.i(fence + 8));
        let r = (dz * dz + dx * dx).sqrt();
        let hit = (r < 460.0 && (700.0..900.0).contains(&dy))
            || (r < 410.0 && (900.0..1100.0).contains(&dy))
            || (r < 330.0 && (1100.0..1300.0).contains(&dy))
            || (r < 180.0 && (1300.0..1500.0).contains(&dy));
        if hit {
            flags[3] = 1;
        }
        if flags[3] != 1 {
            if flags[3] == 0 {
                self.si(fence + 12, 0);
                self.si(fence + 20, 0);
            }
            return;
        }
        self.reflect(a1, fence);
        *ground = self.i(a1 + 28);
        self.si(fence + 12, 1);
        self.si(fence + 20, 0);
    }

    /// `sub_100038F0`: the backstop net.
    fn f_100038f0(&mut self, a1: u32, fence: u32, flags: &mut [u8; 5]) {
        if self.i(fence + 16) == 1 {
            let n = self.i(fence + 24) + 1;
            self.si(fence + 24, n);
            if n > 60 {
                self.si(fence + 16, 0);
                self.si(fence + 24, 0);
            }
            return;
        }
        let dx = f64::from(self.i(a1 + 24) - self.i(fence));
        let dz = f64::from(self.i(a1 + 32) - self.i(fence + 8));
        if (dz * dz + dx * dx).sqrt() < 100.0
            && f64::from(self.i(a1 + 28) - self.i(fence + 4)) < 700.0
        {
            flags[4] = 1;
        }
        if flags[4] != 1 {
            if flags[4] == 0 {
                self.si(fence + 16, 0);
                self.si(fence + 24, 0);
            }
            return;
        }
        if self.reflect(a1, fence) {
            self.si(fence + 16, 1);
            self.si(fence + 24, 0);
        }
    }

    /// `sub_10003AE0`: the play is over.
    fn f_10003ae0(&mut self) {
        self.sf(20, 3);
        self.sf(210, 0);
        self.f_10004550(0, false);
    }

    /// `sub_10003B10`: the ball is dead; the play ends in 120 frames.
    pub(crate) fn f_10003b10(&mut self) {
        self.sb(BALL + 1204, 1);
        self.si(BALL + 1208, 120);
        self.si(BALL + 1212, 0);
        self.sf(252, 0);
    }

    // ---- the bat (call 50) ----------------------------------------------------------

    /// `sub_10003B50`: does the swing (`F[620..=626]`) meet the pitch?
    pub(crate) fn f_10003b50(&mut self) {
        let v36 = f64::from(self.f(229));
        let v29 = f64::from(self.f(231));
        let v32 = f64::from(self.f(226));
        let v41 = f64::from(self.f(228));
        let v39 = f64::from(self.f(624));
        let angle = (v39 + 2700.0) * 0.001745329251944444;
        let v27 = f64::from(self.f(620));
        let v31 = f64::from(self.f(622));
        let v34 = angle.cos() * 150.0 + v27;
        let v4 = angle.sin() * 150.0 + v31;
        let v5 = v32 - v27;
        let v42 = v41 - v31;
        let reach = (v42 * v42 + v5 * v5).sqrt();
        let mut in_arc = false;
        if (50.0..150.0).contains(&reach) {
            let mut a = if v5 != 0.0 {
                let a = (v42 / v5).atan();
                if v5 >= 0.0 {
                    a + HALF_PI
                } else {
                    a + THREE_HALF_PI
                }
            } else if v42 < 0.0 {
                0.0
            } else {
                PI
            };
            a *= 572.9577951471996;
            if a >= 0.0 && a < 1800.0 && v39 - 350.0 < a && a < v39 {
                in_arc = true;
            }
        }
        let v46 = v29 - v41;
        let v40 = v34 - v27;
        let v45 = v36 - v32;
        let v43 = v4 - v31;
        if (v4 + v46 - v31) * v40 - (v34 + v45 - v27) * v43 <= 0.0 || !in_arc {
            return;
        }
        // Where the ball's path crosses the bat.
        let Some(hit) = line_intersection([v36, v29, v32, v41, v27, v31, v34, v4]) else {
            return;
        };
        let (v10, v14) = (hit.x, hit.y);
        let v30 = v14 + v46;
        let v35 = if v40 == 0.0 {
            HALF_PI
        } else {
            (v43 / v40).atan()
        };
        let v15 = v10 + v45 - v10;
        let (c, s) = ((-v35).cos(), (-v35).sin());
        let v33 = c * v15 - s * (v30 - v14);
        let v17 = v15 * s + c * (v30 - v14);
        let base = if v33 == 0.0 {
            if v17 < 0.0 { THREE_HALF_PI } else { HALF_PI }
        } else {
            let a = (v17 / v33).atan();
            match (v33 >= 0.0, v17 >= 0.0) {
                (true, true) => HALF_PI - a + HALF_PI,
                (true, false) => THREE_HALF_PI - (HALF_PI + a),
                (false, true) => HALF_PI - (HALF_PI + a),
                (false, false) => HALF_PI - a + THREE_HALF_PI,
            }
        };
        let jitter = f64::from(self.rand() % 400 - 200) * 0.001745329251944444;
        let direction = jitter + base + v35;
        let (mut dx, mut dz) = (direction.cos(), direction.sin());
        normalize(&mut dx, &mut dz);
        self.sf(625, 1);
        self.sf(210, 2);
        self.sf(221, 255);
        self.sf(250, 1);
        self.sf(251, 255);
        self.sf(252, 1);
        self.sf(253, 2 * self.f(226));
        self.sf(254, self.f(227));
        self.sf(255, self.f(228));
        for i in 0..3 {
            self.sf(256 + i, self.f(253 + i));
            self.sf(259 + i, self.f(256 + i));
        }
        self.sf(262, ftol(dx * 2000.0));
        self.sf(263, 0);
        self.sf(264, ftol(dz * 1000.0));
        let speed = self.rand() % 300 + 300;
        self.sf(265, speed);
        self.sf(266, 10000 * self.f(256));
        self.sf(267, 10000 * self.f(257));
        self.sf(268, 10000 * self.f(258));
        self.sf(269, self.f(254));
        let lift = self.rand() % 200 + 300;
        self.sf(270, lift);
        self.sf(271, 0);
        self.sf(272, 0);
        self.sf(280, 0);
        self.fill(BALL, 0x4B0, 0);
        match self.f(626) {
            1 => {
                self.sf(270, self.f(270) + 300);
                self.f_10014040(13, 255);
            }
            2 => {
                self.sf(270, self.f(270) / 2);
                self.f_10014040(11, 255);
            }
            _ => self.f_10014040(12, 255),
        }
        self.f_10004400(1, 0, false);
    }

    // ---- the field and camera (calls 60/61) -------------------------------------------

    /// `sub_10004310`: ground height at (x, z) → `F[320]`. `flags`
    /// receives (on the slope, beyond the fence).
    pub(crate) fn f_10004310(&mut self, x: i32, z: i32, flags: Option<&mut [bool; 2]>) -> i32 {
        let mut slope = false;
        let mut beyond = false;
        let height = if x < 0 {
            let d = -z - x / 2;
            if d < 280 {
                0
            } else if d < 1255 {
                slope = true;
                (d - 280) / 3
            } else {
                325
            }
        } else {
            let d = x / 2 - z;
            if d < 310 {
                0
            } else if d < 1240 {
                slope = true;
                (d - 310) / 3
            } else if d < 2260 {
                310
            } else if d < 4113 {
                slope = true;
                (3190 - d) / 3
            } else if d >= 4523 {
                beyond = true;
                -567
            } else {
                -307
            }
        };
        self.sf(320, height);
        if let Some(flags) = flags {
            *flags = [slope, beyond];
        }
        self.f(320)
    }

    /// `sub_10004400`: points the camera (`F[350..=362]`) at target
    /// `mode` (1 ball, 3/4 special targets, 10+n actor n).
    pub(crate) fn f_10004400(&mut self, mode: i32, speed: i32, force: bool) {
        if self.f(351) != 0 && !force {
            return;
        }
        self.sf(350, mode);
        self.sf(355, self.f(352));
        self.sf(356, self.f(353));
        self.sf(357, self.f(354));
        self.sf(361, 0);
        self.sf(362, speed);
        for n in 0..20 {
            if mode == n + 10 {
                let record = 1010 + 40 * n as u32;
                self.sf(358, self.f(record));
                self.sf(359, self.f(record + 1));
                self.sf(360, self.f(record + 2));
            }
        }
        let source = match mode {
            1 => Some(256),
            3 => Some(1835),
            4 => Some(1852),
            _ => None,
        };
        if let Some(from) = source {
            self.sf(358, self.f(from));
            self.sf(359, self.f(from + 1));
            self.sf(360, self.f(from + 2));
        }
    }

    /// `sub_10004550`: points the camera at a random active fielder (or,
    /// failing that, a random active cat).
    pub(crate) fn f_10004550(&mut self, speed: i32, force: bool) {
        if self.f(351) != 0 && !force {
            return;
        }
        let mut candidates: Vec<i32> = (0..=9)
            .filter(|&n| n != 6 && self.f(1000 + 40 * n as u32) == 1)
            .collect();
        candidates.push(-1);
        let pick = candidates[(self.rand() as usize) % candidates.len()];
        let target = if pick == -1 {
            let cats: Vec<i32> = (0..9)
                .filter(|&n| self.f(1440 + 40 * n as u32) == 1)
                .map(|n| n + 11)
                .collect();
            if cats.is_empty() {
                return;
            }
            cats[(self.rand() as usize) % cats.len()]
        } else {
            pick
        };
        self.sf(350, target + 10);
        self.sf(355, self.f(352));
        self.sf(356, self.f(353));
        self.sf(357, self.f(354));
        self.sf(361, 0);
        self.sf(362, speed);
    }

    /// `sub_10004770`: is (x, z) within 2000 of the camera?
    pub(crate) fn f_10004770(&self, x: i32, _y: i32, z: i32) -> bool {
        (x - self.f(352)).abs() < 2000 && (z - self.f(354)).abs() < 2000
    }

    // ---- sounds -------------------------------------------------------------------------

    /// `sub_10014040`: a sound effect for the script (`F[50]`, volume
    /// `F[51]`).
    pub(crate) fn f_10014040(&mut self, se: i32, volume: i32) {
        self.sf(50, se);
        self.sf(51, volume.clamp(0, 255));
    }

    /// `sub_10014080`: a voice/line for the script (`F[48]`, `F[49]`).
    pub(crate) fn f_10014080(&mut self, line: i32) {
        self.sf(48, line);
        self.sf(49, 150);
    }

    /// `sub_100140A0`: a cut-in (`F[40]`).
    pub(crate) fn f_100140a0(&mut self, a2: i32) {
        self.sf(40, a2);
        for i in 42..=45 {
            self.sf(i, 0);
        }
    }

    // ---- the actor base class --------------------------------------------------------------

    /// Word `n` of an actor object.
    #[inline]
    pub(crate) fn w(&self, this: u32, n: u32) -> i32 {
        self.i(this + 4 * n)
    }

    #[inline]
    pub(crate) fn sw(&mut self, this: u32, n: u32, v: i32) {
        self.si(this + 4 * n, v);
    }

    /// The actor's record (`this[2]`), field at byte `off`.
    #[inline]
    pub(crate) fn r(&self, this: u32, off: u32) -> i32 {
        self.i(self.p(this + 8) + off)
    }

    #[inline]
    pub(crate) fn sr(&mut self, this: u32, off: u32, v: i32) {
        let record = self.p(this + 8);
        self.si(record + off, v);
    }

    /// `intF` through the actor (`this[1]`), field at byte `off`.
    #[inline]
    pub(crate) fn g(&self, this: u32, off: u32) -> i32 {
        self.i(self.p(this + 4) + off)
    }

    #[inline]
    pub(crate) fn sg(&mut self, this: u32, off: u32, v: i32) {
        let base = self.p(this + 4);
        self.si(base + off, v);
    }

    /// `sub_10004A70` (slot 1): binds the actor to `intF` and its record.
    pub(crate) fn f_10004a70(&mut self, this: u32, index: i32) {
        self.sw(this, 1, FBASE as i32);
        self.sw(this, 3, (FBASE + 840) as i32);
        self.sw(this, 4, (FBASE + 1000) as i32);
        self.sw(this, 312, index);
        self.sw(this, 2, (FBASE + 32 * (5 * index as u32 + 125)) as i32);
    }

    /// `sub_10004AB0` (slot 2): resets the delayed action and timer.
    pub(crate) fn f_10004ab0(&mut self, this: u32) {
        self.sw(this, 5, 0);
        self.sw(this, 6, 0);
        self.sw(this, 7, 0);
        self.sb(this + 32, 0);
        self.sw(this, 9, 0);
        self.sw(this, 10, 0);
        self.sw(this, 311, 0);
    }

    /// `sub_10004AD0` (call 72): the fielding position for the actor's
    /// position number (`record[2]`).
    pub(crate) fn f_10004ad0(&mut self, this: u32) {
        let position = match self.r(this, 8) {
            -1 => Some((0, 0, 0)),
            0 => Some((0, 0, 4050)),
            1 => Some((0, 0, 1500)),
            2 => Some((4000, 0, 4000)),
            3 => Some((2000, 0, 6000)),
            4 => Some((-4000, 0, 4000)),
            5 => Some((-2000, 0, 6000)),
            6 => Some((4500, 0, 7000)),
            7 => Some((0, 0, 8000)),
            8 => Some((-4500, 0, 7000)),
            9 => Some((6000, 0, 3500)),
            10 => Some((-6000, 0, 3500)),
            11 => Some((-7000, 0, 5000)),
            12 => Some((7000, 0, 5000)),
            13 => Some((0, 0, 2000)),
            14 => Some((-4500, 0, 5000)),
            15 => Some((2000, 0, 2000)),
            16 => Some((7500, 320, 2000)),
            _ => None,
        };
        if let Some((x, y, z)) = position {
            self.sr(this, 28, x);
            self.sr(this, 32, y);
            self.sr(this, 36, z);
        }
        let number = self.w(this, 312);
        if number == 3 && self.r(this, 12) == 7 {
            self.sr(this, 28, self.r(this, 28) + 500);
            self.sr(this, 36, self.r(this, 36) - 200);
        }
        if number == 5 && self.r(this, 12) == 7 {
            self.sr(this, 28, self.r(this, 28) - 500);
            self.sr(this, 36, self.r(this, 36) - 200);
        }
        if number == 1 && self.r(this, 8) == 17 {
            self.sr(this, 28, -2000);
            self.sr(this, 32, 0);
            self.sr(this, 36, 8000);
        }
        if number == 3 && self.r(this, 8) == 17 {
            self.sr(this, 28, 2000);
            self.sr(this, 32, 0);
            self.sr(this, 36, 8000);
        }
        // Position 18: somewhere random, away from the other cats.
        while self.r(this, 8) == 18 {
            let angle = f64::from(self.rand() % 360) * 0.01745329251944444;
            let radius = f64::from(self.rand() % 500 + 500);
            let c = angle.cos();
            self.sr(this, 28, ftol(c * radius + c * radius));
            self.sr(this, 32, 0);
            self.sr(this, 36, ftol(angle.sin() * radius) + 4000);
            let mut clash = false;
            for n in 0..9u32 {
                if n as i32 + 11 == number {
                    continue;
                }
                let other = 1440 + 40 * n;
                if self.f(other) != 0
                    && self.f(other + 2) == 18
                    && distance(
                        f64::from(self.r(this, 28)),
                        f64::from(self.r(this, 36)),
                        f64::from(self.f(other + 7)),
                        f64::from(self.f(other + 9)),
                    ) < 500.0
                {
                    clash = true;
                    break;
                }
            }
            if !clash {
                return;
            }
        }
    }

    /// `sub_10004DF0` (slot 3, call 71): one frame of an actor.
    pub(crate) fn f_10004df0(&mut self, this: u32) {
        if self.r(this, 0) != 1 {
            return;
        }
        let v3 = self.r(this, 24);
        if v3 != 2 {
            if self.i(self.p(this + 12)) == 2 && self.i(self.p(this + 16) + 8) == 1 {
                self.vcall(this, 24, 0, 0);
            } else {
                self.sr(this, 24, 0);
            }
        }
        let run_slot7 = if self.r(this, 20) == 0 {
            true
        } else if v3 != 0 {
            v3 == 1 && self.r(this, 24) == 0
        } else {
            self.r(this, 24) == 1
        };
        if run_slot7 {
            self.vcall(this, 28, 0, 0);
        }
        self.vcall(this, 32, 0, 0);
        // Shift the position history.
        for k in (1..100).rev() {
            for j in 0..3 {
                let v = self.w(this, 11 + 3 * (k - 1) + j);
                self.sw(this, 11 + 3 * k + j, v);
            }
        }
        self.sw(this, 11, self.r(this, 40));
        self.sw(this, 12, self.r(this, 44));
        self.sw(this, 13, self.r(this, 48));
        if self.w(this, 5) != 0 {
            let n = self.w(this, 7) + 1;
            self.sw(this, 7, n);
            if n >= self.w(this, 6) {
                let pending = self.w(this, 5);
                self.vcall(this, 20, pending, 0);
                self.sw(this, 5, 0);
                self.sw(this, 6, 0);
                self.sw(this, 7, 0);
            }
        }
        if self.b(this + 32) != 0 {
            let n = self.w(this, 10) + 1;
            self.sw(this, 10, n);
            if n >= self.w(this, 9) {
                self.f_100054a0(this);
                self.sb(this + 32, 0);
                self.sw(this, 9, 0);
                self.sw(this, 10, 0);
            }
        }
        self.sr(this, 124, self.r(this, 124) + 1);
    }

    /// `sub_10004EF0` (slot 6): is the ball coming this way?
    pub(crate) fn f_10004ef0(&mut self, this: u32) {
        let ball = self.p(this + 16);
        let bi = |pt: &Self, n: u32| pt.i(ball + 4 * n);
        let moving =
            bi(self, 3) != bi(self, 6) || bi(self, 4) != bi(self, 7) || bi(self, 5) != bi(self, 8);
        if moving {
            if self.r(this, 8) <= 5 {
                let v4 = bi(self, 23);
                let limit = if v4 < 0 { v4 / 2 + 8000 } else { 8000 - v4 / 2 };
                if bi(self, 25) > limit {
                    self.sr(this, 24, 0);
                    return;
                }
            }
            if self.f_10004310(bi(self, 6), bi(self, 8), None) != 0 {
                self.sr(this, 24, 0);
                return;
            }
            let ball_dir =
                f64::from(bi(self, 6) - bi(self, 3)).atan2(f64::from(bi(self, 8) - bi(self, 5)));
            let me_dir = f64::from(self.r(this, 28) - bi(self, 3))
                .atan2(f64::from(self.r(this, 36) - bi(self, 5)));
            let diff = ball_dir - me_dir;
            if !(0.5235987755833333..=5.759586531416667).contains(&diff) {
                self.sr(this, 24, 1);
                return;
            }
        }
        self.sr(this, 24, 0);
    }

    /// `sub_10005010` (slot 4): back to the idle state.
    pub(crate) fn f_10005010(&mut self, this: u32) {
        for off in [16, 20, 24, 108, 112, 116, 128] {
            self.sr(this, off, 0);
        }
        self.sr(this, 132, 1);
    }

    /// `sub_10005050` (slot 5): enters state `a2` with parameter `a3`.
    pub(crate) fn f_10005050(&mut self, this: u32, a2: i32, a3: i32) {
        if self.r(this, 16) != a2 {
            self.sr(this, 16, a2);
            self.sr(this, 20, 1);
            self.sr(this, 108, 0);
            self.sr(this, 112, a3);
            self.sr(this, 116, 0);
        }
    }

    /// `sub_10005090`: shows animation pattern `a2` at speed `a3`.
    pub(crate) fn f_10005090(&mut self, this: u32, a2: i32, a3: i32) {
        self.sr(this, 80, a2);
        self.sr(this, 84, a3);
        self.sr(this, 88, 1);
    }

    /// `sub_100050C0`: shows an animation range.
    pub(crate) fn f_100050c0(&mut self, this: u32, a2: i32, a3: i32, a4: i32, restart: bool) {
        self.sr(this, 92, a2);
        self.sr(this, 96, a3);
        self.sr(this, 100, a4);
        self.sr(this, 80, -i32::from(restart));
        self.sr(this, 104, 1);
    }

    /// `sub_10005100`: moves towards the target at `step` per frame.
    pub(crate) fn f_10005100(&mut self, this: u32, step: f64) {
        let dx = f64::from(self.r(this, 68) - self.r(this, 40));
        let dz = f64::from(self.r(this, 76) - self.r(this, 48));
        let d = length3(dx, 0.0, dz);
        if d >= step {
            self.sr(this, 40, self.r(this, 40) + ftol(dx / d * step));
            self.sr(this, 48, self.r(this, 48) + ftol(dz / d * step));
        } else {
            self.sr(this, 40, self.r(this, 68));
            self.sr(this, 48, self.r(this, 76));
        }
    }

    /// Facing towards the target in eight directions (`record[13]`).
    fn update_facing(&mut self, this: u32) {
        let dz = f64::from(self.r(this, 76) - self.r(this, 48));
        let dx = f64::from(self.r(this, 68) - self.r(this, 40));
        let mut dir = direction8(dx, dz, -1);
        if dir == -1 {
            dir = self.r(this, 52);
            if dir == -1 {
                dir = 5;
            }
        }
        self.sr(this, 52, dir);
    }

    /// `sub_100051C0`: a walking animation facing the target (mirrored
    /// for the left-hand directions).
    pub(crate) fn f_100051c0(&mut self, this: u32, a2: i32, a3: i32, a4: i32) {
        self.update_facing(this);
        let (mirror, pattern) = match self.r(this, 52) {
            0 => (-100, a3),
            1 => (-100, a3 + 4),
            2 => (-100, a3 + 8),
            3 => (-100, a3 + 12),
            4 => (100, a3 + 16),
            5 => (100, a3 + 12),
            6 => (100, a3 + 8),
            7 => (100, a3 + 4),
            _ => (a4, a4),
        };
        self.walk(this, a2, mirror, pattern, a4);
    }

    /// `sub_10005310`: as above with eight separate patterns.
    pub(crate) fn f_10005310(&mut self, this: u32, a2: i32, a3: i32, a4: i32) {
        self.update_facing(this);
        let (mirror, pattern) = match self.r(this, 52) {
            d @ 0..=7 => (100, a3 + 4 * d),
            _ => (a4, a4),
        };
        self.walk(this, a2, mirror, pattern, a4);
    }

    fn walk(&mut self, this: u32, a2: i32, mirror: i32, pattern: i32, a4: i32) {
        if (a4 & 0xff) != 0 {
            self.f_100050c0(this, a2, mirror, pattern, false);
        } else if mirror != self.r(this, 96) || pattern != self.r(this, 100) {
            self.f_100050c0(this, a2, mirror, pattern, true);
        }
    }

    /// `sub_10005460`: runs for the ball.
    pub(crate) fn f_10005460(&mut self, this: u32) {
        let ball = self.p(this + 16);
        self.si(ball + 8, 0);
        self.sr(this, 24, 2);
    }

    /// `sub_10005480`: catches the ball.
    pub(crate) fn f_10005480(&mut self, this: u32) {
        let ball = self.p(this + 16);
        self.si(ball, 0);
        self.si(ball + 60, 0);
        self.si(ball + 120, 1);
    }

    /// `sub_100054A0`: the play ends.
    pub(crate) fn f_100054a0(&mut self, this: u32) {
        self.sr(this, 24, 0);
        self.sg(this, 840, 0);
        self.sg(this, 80, 3);
        self.f_10004550(0, false);
        self.vcall(this, 16, 0, 0);
    }

    /// `sub_100054E0`: as above without moving the camera.
    pub(crate) fn f_100054e0(&mut self, this: u32) {
        self.sr(this, 24, 0);
        self.sg(this, 840, 0);
        self.sg(this, 80, 3);
        self.vcall(this, 16, 0, 0);
    }

    /// `sub_10005500`.
    pub(crate) fn f_10005500(&self, this: u32) -> bool {
        self.r(this, 12) == 7
    }

    /// `sub_10005510`: the position `frames` ago (or the home position).
    pub(crate) fn f_10005510(&self, this: u32, frames: i32) -> [i32; 3] {
        if frames > 0 && frames <= 100 && self.r(this, 124) >= frames {
            let n = 3 * frames as u32 + 8;
            return [self.w(this, n), self.w(this, n + 1), self.w(this, n + 2)];
        }
        [self.r(this, 28), self.r(this, 32), self.r(this, 36)]
    }

    /// `sub_10005570`: a delayed state change.
    pub(crate) fn f_10005570(&mut self, this: u32, state: i32, delay: i32) {
        self.sw(this, 5, state);
        self.sw(this, 6, delay);
        self.sw(this, 7, 0);
    }

    /// `sub_10005590`: ends the play after `frames`.
    pub(crate) fn f_10005590(&mut self, this: u32, frames: i32) {
        self.sb(this + 32, 1);
        self.sw(this, 9, frames);
        self.sw(this, 10, 0);
        self.sg(this, 120, 7);
    }

    /// `sub_100055B0`: four dust clouds around the actor (`F[450..=461]`).
    pub(crate) fn f_100055b0(&mut self, this: u32) {
        for i in (0..48).step_by(12) {
            let x = self.rand() % 400 + self.r(this, 40) - 200;
            self.sg(this, 1800 + i, x);
            self.sg(this, 1804 + i, self.r(this, 44));
            let z = self.rand() % 400 + self.r(this, 48) - 200;
            self.sg(this, 1808 + i, z);
            let a = f64::from(2 - self.r(this, 52)) * 0.785398163375;
            self.sg(
                this,
                1800 + i,
                ftol(a.cos() * 300.0 + f64::from(self.g(this, 1800 + i))),
            );
            self.sg(
                this,
                1808 + i,
                ftol(a.sin() * 300.0 + f64::from(self.g(this, 1808 + i))),
            );
        }
        self.sg(this, 1920, 1);
    }

    /// `sub_10005690`: a point on a circle round the field.
    pub(crate) fn f_10005690(&self, this: u32, a3: i32) -> [i32; 3] {
        let a = f64::from(a3 + self.r(this, 124)) * 0.003333333333333334 + PI;
        [ftol(a.cos() * 4500.0), 0, ftol(a.sin() * 4500.0 + 5000.0)]
    }

    /// `sub_100056F0`: the facing for that point.
    pub(crate) fn f_100056f0(&self, this: u32, a2: i32) -> i32 {
        let a = f64::from(a2 + self.r(this, 124)) * 0.003333333333333334 + PI;
        facing_from(ftol(a * 1.273239544771555 + 0.3926990816875) % 8)
    }

    /// `sub_100057A0`: another circle point (50 frames behind).
    pub(crate) fn f_100057a0(&self, this: u32, a3: i32) -> [i32; 3] {
        let a = f64::from(self.r(this, 124) + a3 - 50) * 0.003333333333333334 + PI;
        [ftol(a.cos() * 4500.0), 0, ftol(a.sin() * 4500.0 + 5000.0)]
    }

    /// `sub_10005800`.
    pub(crate) fn f_10005800(&self, this: u32, a2: i32) -> i32 {
        let a = f64::from(self.r(this, 124) + a2 - 50) * 0.003333333333333334 + 9.4247779605;
        facing_from(ftol(a * 1.273239544771555 + 0.3926990816875) % 8)
    }

    // ---- lookup tables --------------------------------------------------------------------

    /// `sub_10002270` on first use: prefix sums of `count` durations at
    /// `source` into `table`.
    pub(crate) fn prefix_table(&mut self, table: u32, source: u32, count_at: u32) {
        let count = self.i(count_at);
        self.prefix_n(table, source, count);
    }

    pub(crate) fn prefix_n(&mut self, table: u32, source: u32, count: i32) {
        if self.i(table) != 0 {
            return;
        }
        let mut sum = 0i32;
        for n in 0..count.max(0) as u32 {
            sum = sum.wrapping_add(self.i(source + 4 * n));
            self.si(table + 4 * n, sum);
        }
    }

    /// The pattern of an animation table at frame `record[112]`,
    /// advancing `record[108]` by at most one step (the common form).
    pub(crate) fn table_step(&mut self, this: u32, table: u32, patterns: u32) -> i32 {
        let index = self.r(this, 108);
        if self.i(table + 4 * index as u32) <= self.r(this, 112) {
            self.sr(this, 108, index + 1);
        }
        self.i(patterns + 4 * self.r(this, 108) as u32)
    }

    /// The pattern with `record[108]` recomputed from scratch.
    pub(crate) fn table_seek(
        &mut self,
        this: u32,
        table: u32,
        patterns: u32,
        count_at: u32,
    ) -> i32 {
        let count = self.i(count_at);
        self.seek_n(this, table, patterns, count)
    }

    pub(crate) fn seek_n(&mut self, this: u32, table: u32, patterns: u32, count: i32) -> i32 {
        self.sr(this, 108, 0);
        let mut index = 0;
        while index < count - 1 {
            if self.r(this, 112) < self.i(table + 4 * index as u32) {
                break;
            }
            index += 1;
            self.sr(this, 108, index);
        }
        self.i(patterns + 4 * self.r(this, 108) as u32)
    }

    /// Throws the ball from (x, y, z) towards the target at
    /// (`dword_100229F8`, `dword_10022A00`) and follows it with the camera
    /// (`sub_10006F00`, `sub_100093F0`, `sub_100094D0`, `sub_1000AC70`, …).
    pub(crate) fn throw_at_target(&mut self, x: i32, y: i32, z: i32, speed: i32, lift: i32) {
        let dx = self.i(0x1002_29F8) - x;
        let dz = self.i(0x1002_2A00) - z;
        let length = f64::from(dx * dx + dz * dz).sqrt();
        let (ux, uy, uz) = if length == 0.0 {
            (0, 0, 0)
        } else {
            (
                ftol(f64::from(dx) * 1000.0 / length),
                ftol(0.0 / length),
                ftol(f64::from(dz) * 1000.0 / length),
            )
        };
        self.f_10002bd0(x, y, z, ux, uy, uz, speed, lift);
        self.f_10004400(1, 50, false);
    }

    /// The total length of an animation table (its last prefix sum).
    pub(crate) fn table_total(&self, table: u32, count_at: u32) -> i32 {
        self.i(table - 4 + 4 * self.i(count_at) as u32)
    }

    /// The field object's `intF` (for functions that take it as `this`).
    #[allow(dead_code)]
    pub(crate) fn field(&self) -> u32 {
        self.p(FIELD)
    }
}

/// The DLL's mapping from the angle octant to a facing.
fn facing_from(octant: i32) -> i32 {
    match octant {
        0 => 0,
        1 => 7,
        2 => 6,
        3 => 5,
        4 => 4,
        5 => 3,
        6 => 2,
        7 => 1,
        other => other,
    }
}
