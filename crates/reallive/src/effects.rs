//! Screen transitions (`#SEL` effects).
//!
//! A transition blends the screen before a command (`before`) into the
//! screen after it (`after`) over `time` ms. Styles and their parameters:
//!
//! * `direction` — for wipes and slides 0 top→bottom, 1 bottom→top,
//!   2 left→right, 3 right→left; for centred shapes 0 is the centre and
//!   1-4 the corners;
//! * `interpolation` — soft edge: `2^n * 2.5` pixels for wipes, extra
//!   intermediate steps for dithered fades;
//! * `xsize`, `ysize` — pattern cell size for block and dither styles,
//!   strip width for row/column styles.
//!
//! Most styles are "reveal" styles: every pixel has a time at which the
//! new picture appears there. The others move pictures around (scrolls,
//! slides, zooms, ripples).

use crate::surface::{Rect, Surface};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Transition {
    pub time: i32,
    pub style: i32,
    pub direction: i32,
    pub interpolation: i32,
    pub xsize: i32,
    pub ysize: i32,
    pub a: i32,
    pub b: i32,
    pub opacity: i32,
    pub c: i32,
}

impl Transition {
    /// From the 16 values of `#SEL.nnn` or the long form of `grpOpen`:
    /// `x1 y1 x2 y2 dx dy time style direction interpolation xsize ysize a
    /// b opacity c`.
    pub fn from_sel(values: &[i32]) -> Self {
        let get = |i: usize| values.get(i).copied().unwrap_or(0);
        Self {
            time: get(6),
            style: get(7),
            direction: get(8),
            interpolation: get(9),
            xsize: get(10),
            ysize: get(11),
            a: get(12),
            b: get(13),
            opacity: if values.len() > 14 { get(14) } else { 255 },
            c: get(15),
        }
    }

    pub fn instant() -> Self {
        Self {
            opacity: 255,
            ..Self::default()
        }
    }
}

/// Deterministic per-cell noise in `[0, 1)`.
fn noise(seed: i64) -> f64 {
    let mut z = (seed as u64).wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64
}

const BAYER4: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

fn lerp_pixel(a: [u8; 4], b: [u8; 4], t: f64) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    let mix = |x: u8, y: u8| (f64::from(x) + (f64::from(y) - f64::from(x)) * t).round() as u8;
    [mix(a[0], b[0]), mix(a[1], b[1]), mix(a[2], b[2]), 255]
}

/// Where a pixel of a geometric style comes from.
enum Sample {
    Before(f64, f64),
    After(f64, f64),
    /// `before` at one place blended with `after` at another.
    Blend {
        before: (f64, f64),
        after: (f64, f64),
        amount: f64,
    },
    Solid([u8; 4]),
}

fn sample(surface: &Surface, (x, y): (f64, f64)) -> [u8; 4] {
    let (xi, yi) = (x.floor() as i32, y.floor() as i32);
    if xi < 0 || yi < 0 || xi >= surface.width || yi >= surface.height {
        return [0, 0, 0, 255];
    }
    surface.pixel(xi, yi)
}

/// Renders the transition at progress `t` (0..=1).
pub fn render(tr: &Transition, t: f64, before: &Surface, after: &Surface) -> Surface {
    let t = t.clamp(0.0, 1.0);
    let (w, h) = (after.width, after.height);
    let mut out = Surface::new(w, h);
    if t >= 1.0 {
        out.rgba.clone_from(&after.rgba);
        return out;
    }
    let soft = if tr.interpolation > 0 {
        (2f64.powi(tr.interpolation.min(8)) * 2.5).max(1.0)
    } else {
        1.0
    };
    // Reveal styles return the pixel's reveal time and the length (in
    // progress units) of its soft edge.
    let reveal = reveal_function(tr, w, h, soft);
    if let Some(reveal) = reveal {
        for y in 0..h {
            for x in 0..w {
                let (at, edge) = reveal(x, y);
                let alpha = if edge <= 0.0 {
                    if t >= at { 1.0 } else { 0.0 }
                } else {
                    ((t - at) / edge + 0.5).clamp(0.0, 1.0)
                };
                let pixel = if alpha <= 0.0 {
                    before.pixel(x, y)
                } else if alpha >= 1.0 {
                    after.pixel(x, y)
                } else {
                    lerp_pixel(before.pixel(x, y), after.pixel(x, y), alpha)
                };
                let i = ((y * w + x) * 4) as usize;
                out.rgba[i..i + 4].copy_from_slice(&pixel);
            }
        }
        return out;
    }
    let geometric = geometric_function(tr, w, h, t);
    for y in 0..h {
        for x in 0..w {
            let pixel = match geometric(f64::from(x) + 0.5, f64::from(y) + 0.5) {
                Sample::Before(sx, sy) => sample(before, (sx, sy)),
                Sample::After(sx, sy) => sample(after, (sx, sy)),
                Sample::Blend {
                    before: from,
                    after: to,
                    amount,
                } => lerp_pixel(sample(before, from), sample(after, to), amount),
                Sample::Solid(colour) => colour,
            };
            let i = ((y * w + x) * 4) as usize;
            out.rgba[i..i + 4].copy_from_slice(&pixel);
        }
    }
    out
}

type Reveal = Box<dyn Fn(i32, i32) -> (f64, f64)>;

/// Linear sweep along an axis given a pixel position `p` in `0..len`.
fn sweep(p: f64, len: f64, soft: f64) -> (f64, f64) {
    let span = len + soft;
    ((p + soft / 2.0) / span, soft / span)
}

fn reveal_function(tr: &Transition, w: i32, h: i32, soft: f64) -> Option<Reveal> {
    let (wf, hf) = (f64::from(w), f64::from(h));
    let dir = tr.direction;
    let cell_x = tr.xsize.max(1);
    let cell_y = if tr.ysize > 0 { tr.ysize } else { cell_x };
    let strip = if tr.xsize > 0 {
        tr.xsize
    } else if tr.ysize > 0 {
        tr.ysize
    } else {
        16
    };
    // Centre point for centred shapes: 0 centre, 1-4 corners.
    let centre = match dir {
        1 => (0.0, 0.0),
        2 => (wf, 0.0),
        3 => (0.0, hf),
        4 => (wf, hf),
        _ => (wf / 2.0, hf / 2.0),
    };
    let style = tr.style;
    let boxed: Reveal = match style {
        // Fades: every pixel together (the easing of 50 and 54 is applied
        // to `t` by the caller).
        0 | 50 | 54 => Box::new(|_, _| (0.5, 1.0)),
        1 => Box::new(|_, _| (0.0, 0.0)),
        2 => Box::new(|_, _| (1.0, 0.0)),
        // Dithered fades: ordered-dither thresholds over cells.
        4 | 5 => {
            let steps = 1.0 + f64::from(tr.interpolation.max(0));
            let diagonal = style == 5;
            let cx = if tr.xsize > 0 { tr.xsize } else { 1 };
            let cy = if tr.ysize > 0 { tr.ysize } else { 1 };
            Box::new(move |x, y| {
                let (mut i, mut j) = ((x / cx) as usize, (y / cy) as usize);
                if diagonal {
                    let (a, b) = (i + j, i + 3 * j);
                    i = a;
                    j = b;
                }
                let threshold = f64::from(BAYER4[j % 4][i % 4]) / 16.0;
                (threshold * (1.0 - 1.0 / 16.0) + 0.5 / 16.0, steps / 16.0)
            })
        }
        // Wipes.
        10 | 280..=285 | 290..=295 => Box::new(move |x, y| match dir % 4 {
            0 => sweep(f64::from(y), hf, soft),
            1 => sweep(hf - f64::from(y) - 1.0, hf, soft),
            2 => sweep(f64::from(x), wf, soft),
            _ => sweep(wf - f64::from(x) - 1.0, wf, soft),
        }),
        // Square / diamond / plus shapes shrinking inwards.
        25 | 27 | 45 | 140 => {
            let shape = style;
            let (cx, cy) = centre;
            let reach = match shape {
                25 => wf.max(hf) / 2.0,
                27 => (wf + hf) / 2.0,
                _ => wf.max(hf) / 2.0,
            };
            let reach = if dir == 0 { reach } else { reach * 2.0 };
            Box::new(move |x, y| {
                let (dx, dy) = (
                    (f64::from(x) + 0.5 - cx).abs(),
                    (f64::from(y) + 0.5 - cy).abs(),
                );
                let d = match shape {
                    25 => dx.max(dy),
                    27 => dx + dy,
                    // Plus: distance to the nearest arm.
                    _ => dx.min(dy),
                };
                // Inwards: the outside first.
                sweep(reach - d, reach, soft)
            })
        }
        // Rows / alternate columns wiping inwards.
        30 | 31 | 100 => {
            let vertical_strips = style == 31 || dir >= 2;
            let alternate = style == 31;
            Box::new(move |x, y| {
                let (pos, len, index) = if vertical_strips {
                    (f64::from(y), hf, x / strip)
                } else {
                    (f64::from(x), wf, y / strip)
                };
                let reversed = (alternate && index % 2 == 1) || dir % 2 == 1;
                let p = if reversed { len - pos - 1.0 } else { pos };
                sweep(p, len, soft)
            })
        }
        // Blocks wiping in alternate directions / brick pattern.
        101 | 102 => {
            let brick = style == 102;
            Box::new(move |x, y| {
                let row = y / cell_y;
                let shift = if brick && row % 2 == 1 { cell_x / 2 } else { 0 };
                let column = (x + shift) / cell_x;
                let local = f64::from((x + shift) % cell_x);
                let reversed = (row + column) % 2 == 1;
                let p = if reversed {
                    f64::from(cell_x) - local - 1.0
                } else {
                    local
                };
                sweep(p, f64::from(cell_x), soft)
            })
        }
        // Wipe inwards / outwards (from both edges).
        110 | 111 => {
            let outwards = style == 111;
            Box::new(move |x, y| {
                let (pos, len) = if dir >= 2 {
                    (f64::from(x), wf)
                } else {
                    (f64::from(y), hf)
                };
                let from_edge = pos.min(len - pos - 1.0);
                let half = len / 2.0;
                let p = if outwards {
                    half - from_edge
                } else {
                    from_edge
                };
                sweep(p, half, soft)
            })
        }
        // Venetian blinds.
        120 => Box::new(move |x, y| {
            let (pos, reversed) = match dir % 4 {
                0 => (y % strip, false),
                1 => (y % strip, true),
                2 => (x % strip, false),
                _ => (x % strip, true),
            };
            let p = if reversed { strip - pos - 1 } else { pos };
            sweep(f64::from(p), f64::from(strip), soft.min(f64::from(strip)))
        }),
        // Diagonal wipe.
        130 => Box::new(move |x, y| {
            let (fx, fy) = (f64::from(x), f64::from(y));
            let d = match dir % 4 {
                0 => fx + fy,
                1 => fx + (hf - fy),
                2 => (wf - fx) + fy,
                _ => (wf - fx) + (hf - fy),
            };
            sweep(d, wf + hf, soft)
        }),
        // Rotating plus: angular sweep around the centre, four arms.
        150 => {
            let (cx, cy) = centre;
            Box::new(move |x, y| {
                let angle = (f64::from(y) + 0.5 - cy).atan2(f64::from(x) + 0.5 - cx);
                let quarter = std::f64::consts::FRAC_PI_2;
                let phase = angle.rem_euclid(quarter) / quarter;
                (phase * 0.95 + 0.025, 0.05)
            })
        }
        // Square fades: spiral, linear, rotating, wiping; wiping diamonds.
        61..=65 => {
            let (cx, cy) = (cell_x.max(8), cell_y.max(8));
            let columns = (w + cx - 1) / cx;
            let rows = (h + cy - 1) / cy;
            let total = f64::from(columns * rows).max(1.0);
            let order = spiral_or_linear(style, columns, rows);
            Box::new(move |x, y| {
                let (i, j) = (x / cx, y / cy);
                let rank = order(i, j);
                let start = f64::from(rank) / total * 0.75;
                let local = match style {
                    // Rotating: fade by angle inside the cell.
                    63 => {
                        let a = (f64::from(y % cy) - f64::from(cy) / 2.0)
                            .atan2(f64::from(x % cx) - f64::from(cx) / 2.0);
                        (a + std::f64::consts::PI) / std::f64::consts::TAU
                    }
                    // Wiping inside the cell.
                    64 => f64::from(x % cx) / f64::from(cx),
                    // Diamonds.
                    65 => {
                        let dx = (f64::from(x % cx) - f64::from(cx) / 2.0).abs() / f64::from(cx);
                        let dy = (f64::from(y % cy) - f64::from(cy) / 2.0).abs() / f64::from(cy);
                        dx + dy
                    }
                    _ => 0.5,
                };
                (
                    start + local * 0.25,
                    if style == 61 || style == 62 {
                        0.25
                    } else {
                        0.05
                    },
                )
            })
        }
        // Random block / row fades.
        180 | 181 => {
            let rows = style == 181;
            Box::new(move |x, y| {
                let key = if rows {
                    i64::from(if dir >= 2 { x / strip } else { y / strip })
                } else {
                    i64::from(y / cell_y) * 10_007 + i64::from(x / cell_x)
                };
                (noise(key) * 0.8 + 0.1, 0.2)
            })
        }
        // Columns wiping randomly.
        200..=204 => {
            let variant = style - 200;
            Box::new(move |x, y| {
                let (along, len, index) = if dir >= 2 {
                    (f64::from(x), wf, y / strip)
                } else {
                    (f64::from(y), hf, x / strip)
                };
                let delay = noise(i64::from(index)) * 0.5;
                let p = match variant {
                    // Inwards: from both ends; outwards: from the middle.
                    2 => along.min(len - along - 1.0) / (len / 2.0),
                    3 => 1.0 - along.min(len - along - 1.0) / (len / 2.0),
                    4 => {
                        if index % 2 == 0 {
                            along.min(len - along - 1.0) / (len / 2.0)
                        } else {
                            1.0 - along.min(len - along - 1.0) / (len / 2.0)
                        }
                    }
                    1 if index % 2 == 1 => 1.0 - along / len,
                    _ => along / len,
                };
                (delay + p * 0.5, 0.02)
            })
        }
        // Mask wipes: the new picture appears in order of brightness of a
        // procedural gradient (no mask file is referenced by #SEL).
        900..=903 => Box::new(move |x, y| {
            let v = (f64::from(x) / wf + f64::from(y) / hf) / 2.0;
            let v = if style % 2 == 1 { 1.0 - v } else { v };
            sweep(v * 100.0, 100.0, soft / 4.0)
        }),
        _ => return None,
    };
    Some(boxed)
}

/// Rank of cell `(i, j)` for the square fades.
fn spiral_or_linear(style: i32, columns: i32, rows: i32) -> impl Fn(i32, i32) -> i32 {
    let mut ranks = vec![0i32; (columns * rows).max(1) as usize];
    if style == 61 {
        // Spiral from the outside in.
        let (mut top, mut left, mut bottom, mut right) = (0, 0, rows - 1, columns - 1);
        let mut rank = 0;
        while top <= bottom && left <= right {
            for i in left..=right {
                ranks[(top * columns + i) as usize] = rank;
                rank += 1;
            }
            for j in top + 1..=bottom {
                ranks[(j * columns + right) as usize] = rank;
                rank += 1;
            }
            if top < bottom {
                for i in (left..right).rev() {
                    ranks[(bottom * columns + i) as usize] = rank;
                    rank += 1;
                }
            }
            if left < right {
                for j in (top + 1..bottom).rev() {
                    ranks[(j * columns + left) as usize] = rank;
                    rank += 1;
                }
            }
            top += 1;
            left += 1;
            bottom -= 1;
            right -= 1;
        }
    } else {
        for (index, rank) in ranks.iter_mut().enumerate() {
            let (i, j) = (index as i32 % columns, index as i32 / columns);
            *rank = if style == 62 { j * columns + i } else { i + j };
        }
        if style != 62 {
            // Diagonal ranks are not unique; scale to the cell count.
            let max = (columns + rows - 2).max(1);
            for rank in &mut ranks {
                *rank = *rank * (columns * rows - 1) / max;
            }
        }
    }
    move |i, j| {
        ranks
            .get((j.clamp(0, rows - 1) * columns + i.clamp(0, columns - 1)) as usize)
            .copied()
            .unwrap_or(0)
    }
}

type Geometric = Box<dyn Fn(f64, f64) -> Sample>;

/// The six basic motions of scroll, squash and slide styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Motion {
    /// The new picture pushes the old one out (15).
    ScrollScroll,
    /// The new picture pushes the old one, which is squashed (16).
    ScrollSquash,
    /// The new picture unfolds while the old one scrolls away (17).
    SquashScroll,
    /// Both pictures are squashed (18).
    SquashSquash,
    /// The new picture slides over the still old one (20).
    SlideOn,
    /// The old picture slides away off the still new one (21).
    SlideOff,
}

/// Which picture shows at `u` (distance from the edge the motion starts
/// at, `0..len`) after travelling `p`, and at which `u` of that picture.
fn motion(kind: Motion, u: f64, len: f64, p: f64) -> (bool, f64) {
    let rest = (len - p).max(1e-6);
    let covered = p.max(1e-6);
    if u < p {
        let source = match kind {
            Motion::SquashScroll | Motion::SquashSquash => u / covered * len,
            Motion::SlideOff => u,
            _ => len - p + u,
        };
        (true, source)
    } else {
        let source = match kind {
            Motion::ScrollSquash | Motion::SquashSquash => (u - p) / rest * len,
            Motion::SlideOn => u,
            _ => u - p,
        };
        (false, source)
    }
}

/// Screen axis helpers: `(along, across, length)` for a direction, and the
/// inverse mapping.
#[derive(Debug, Clone, Copy)]
struct Axis {
    horizontal: bool,
    reversed: bool,
    len: f64,
}

impl Axis {
    fn new(direction: i32, w: f64, h: f64) -> Self {
        let horizontal = direction % 4 >= 2;
        Self {
            horizontal,
            reversed: direction % 2 == 1,
            len: if horizontal { w } else { h },
        }
    }

    fn along(&self, x: f64, y: f64) -> f64 {
        let v = if self.horizontal { x } else { y };
        if self.reversed { self.len - v } else { v }
    }

    fn across(&self, x: f64, y: f64) -> f64 {
        if self.horizontal { y } else { x }
    }

    fn point(&self, u: f64, across: f64) -> (f64, f64) {
        let v = if self.reversed { self.len - u } else { u };
        let v = v.clamp(0.0, self.len - 1e-6);
        if self.horizontal {
            (v, across)
        } else {
            (across, v)
        }
    }
}

fn motion_sample(
    axis: Axis,
    kind: Motion,
    u: f64,
    across: f64,
    len: f64,
    p: f64,
    offset: f64,
) -> Sample {
    let (new, source) = motion(kind, u, len, p);
    let (x, y) = axis.point(source + offset, across);
    if new {
        Sample::After(x, y)
    } else {
        Sample::Before(x, y)
    }
}

fn geometric_function(tr: &Transition, w: i32, h: i32, t: f64) -> Geometric {
    let (wf, hf) = (f64::from(w), f64::from(h));
    let axis = Axis::new(tr.direction, wf, hf);
    let len = axis.len;
    let strip = f64::from(if tr.xsize > 0 {
        tr.xsize
    } else if tr.ysize > 0 {
        tr.ysize
    } else {
        16
    });
    let style = tr.style;
    let motion_of = |style: i32| match style {
        16 | 118 => Motion::ScrollSquash,
        17 | 119 => Motion::SquashScroll,
        18 => Motion::SquashSquash,
        20 | 34 | 36 | 112 | 114 | 190 | 220..=224 => Motion::SlideOn,
        21 | 35 | 37 | 113 | 115 | 191 => Motion::SlideOff,
        _ => Motion::ScrollScroll,
    };
    match style {
        // Whole-screen scrolls, squashes and slides.
        15..=18 | 20 | 21 => {
            let kind = motion_of(style);
            Box::new(move |x, y| {
                motion_sample(
                    axis,
                    kind,
                    axis.along(x, y),
                    axis.across(x, y),
                    len,
                    t * len,
                    0.0,
                )
            })
        }
        // Two halves moving towards the centre (inwards) or away from it
        // (outwards).
        112..=119 => {
            let kind = motion_of(style);
            let outwards = matches!(style, 113 | 114 | 117 | 119);
            let half = len / 2.0;
            Box::new(move |x, y| {
                let along = axis.along(x, y);
                let across = axis.across(x, y);
                let first = along < half;
                // Distance from the edge where this half's motion starts.
                let u = match (first, outwards) {
                    (true, false) => along,
                    (false, false) => len - along,
                    (true, true) => half - along,
                    (false, true) => along - half,
                };
                let (new, source) = motion(kind, u.clamp(0.0, half), half, t * half);
                let back = match (first, outwards) {
                    (true, false) => source,
                    (false, false) => len - source,
                    (true, true) => half - source,
                    (false, true) => half + source,
                };
                let (x, y) = axis.point(back, across);
                if new {
                    Sample::After(x, y)
                } else {
                    Sample::Before(x, y)
                }
            })
        }
        // Strips: alternate directions, transparent slides, random delays.
        34..=39 | 190 | 191 | 220..=224 | 230..=234 | 240..=244 => {
            let kind = motion_of(style);
            let transparent = matches!(style, 38 | 39);
            let random = style >= 220;
            let alternate = matches!(style, 34 | 35 | 190 | 191) || matches!(style % 10, 1 | 4);
            Box::new(move |x, y| {
                let across = axis.across(x, y);
                let index = (across / strip).floor() as i64;
                let delay = if random { noise(index) * 0.5 } else { 0.0 };
                let local = ((t - delay) / (1.0 - delay)).clamp(0.0, 1.0);
                let flipped = alternate && index % 2 == 1;
                let strip_axis = Axis {
                    reversed: axis.reversed != flipped,
                    ..axis
                };
                let u = strip_axis.along(x, y);
                if transparent {
                    // The new picture slides in (38) or the old slides
                    // out (39) while the two blend.
                    let shift = (1.0 - local) * len * 0.25;
                    let (sx, sy) =
                        strip_axis.point(if style == 38 { u + shift } else { u }, across);
                    return Sample::Blend {
                        before: (x, y),
                        after: (sx, sy),
                        amount: local,
                    };
                }
                motion_sample(strip_axis, kind, u, across, len, local * len, 0.0)
            })
        }
        // TV switching off: the old picture collapses to a line, then the
        // new picture opens from it.
        40 => Box::new(move |x, y| {
            let cy = hf / 2.0;
            let (scale, old) = if t < 0.5 {
                ((1.0 - t * 2.0).max(0.002), true)
            } else {
                (((t - 0.5) * 2.0).max(0.002), false)
            };
            let sy = cy + (y - cy) / scale;
            if !(0.0..hf).contains(&sy) {
                Sample::Solid([0, 0, 0, 255])
            } else if old {
                Sample::Before(x, sy)
            } else {
                Sample::After(x, sy)
            }
        }),
        // Zooms (optionally rotating) combined with a fade.
        160..=166 | 260..=270 => {
            let (cx, cy) = match tr.direction {
                1 => (0.0, 0.0),
                2 => (wf, 0.0),
                3 => (0.0, hf),
                4 => (wf, hf),
                _ => (wf / 2.0, hf / 2.0),
            };
            let (cx, cy) = if matches!(style, 165 | 166) {
                (0.0, 0.0)
            } else {
                (cx, cy)
            };
            Box::new(move |x, y| {
                // (new picture grows in, degrees of rotation while zooming)
                let (grow_new, rotate) = match style {
                    160 | 163 | 165 => (true, 0.0),
                    161 | 164 | 166 => (false, 0.0),
                    162 => (t >= 0.5, 0.0),
                    260 => (true, 360.0),
                    261 => (false, 360.0),
                    262 => (t >= 0.5, 360.0),
                    263 => (true, -360.0),
                    264 => (false, -360.0),
                    265 => (true, 90.0),
                    266 => (false, 90.0),
                    267 => (true, 180.0),
                    268 => (false, 180.0),
                    269 => (true, 90.0),
                    _ => (false, 90.0),
                };
                if grow_new {
                    let local = if style == 162 || style == 262 {
                        (t - 0.5) * 2.0
                    } else {
                        t
                    };
                    let (sx, sy) =
                        rotate_scale(x, y, cx, cy, local.max(0.01), (1.0 - local) * rotate);
                    if sample_in(sx, sy, wf, hf) {
                        Sample::Blend {
                            before: (x, y),
                            after: (sx, sy),
                            amount: local.sqrt(),
                        }
                    } else {
                        Sample::Before(x, y)
                    }
                } else {
                    let local = if style == 162 || style == 262 {
                        t * 2.0
                    } else {
                        t
                    };
                    let (sx, sy) =
                        rotate_scale(x, y, cx, cy, (1.0 - local).max(0.01), local * rotate);
                    if sample_in(sx, sy, wf, hf) {
                        Sample::Blend {
                            before: (sx, sy),
                            after: (x, y),
                            amount: local * local,
                        }
                    } else {
                        Sample::After(x, y)
                    }
                }
            })
        }
        // Ripples: rows (or columns) displaced by a wave that grows and
        // settles, switching (170, 171) or crossfading (194-197).
        170 | 171 | 194..=197 => Box::new(move |x, y| {
            let amplitude = (t * std::f64::consts::PI).sin() * 24.0;
            let columns = matches!(style, 171 | 196 | 197);
            let phase = if columns { x } else { y } / 16.0 + t * 12.0;
            let offset = phase.sin() * amplitude;
            let displaced = if columns {
                (x, (y + offset).clamp(0.0, hf - 1e-6))
            } else {
                ((x + offset).clamp(0.0, wf - 1e-6), y)
            };
            let amount = if matches!(style, 170 | 171) {
                if t < 0.5 { 0.0 } else { 1.0 }
            } else {
                t
            };
            Sample::Blend {
                before: displaced,
                after: displaced,
                amount,
            }
        }),
        // Pixellation in, out, and out-and-in.
        185..=187 => Box::new(move |x, y| {
            let (block, new) = match style {
                185 => ((1.0 - t) * 32.0, true),
                186 => (t * 32.0, false),
                _ if t < 0.5 => (t * 64.0, false),
                _ => ((1.0 - t) * 64.0, true),
            };
            let block = block.max(1.0).floor();
            let point = ((x / block).floor() * block, (y / block).floor() * block);
            if new {
                Sample::After(point.0, point.1)
            } else if style == 186 {
                Sample::Blend {
                    before: point,
                    after: (x, y),
                    amount: ((t - 0.8) * 5.0).clamp(0.0, 1.0),
                }
            } else {
                Sample::Before(point.0, point.1)
            }
        }),
        // Anything else: a crossfade.
        _ => Box::new(move |x, y| Sample::Blend {
            before: (x, y),
            after: (x, y),
            amount: t,
        }),
    }
}

fn rotate_scale(x: f64, y: f64, cx: f64, cy: f64, scale: f64, degrees: f64) -> (f64, f64) {
    let radians = degrees.to_radians();
    let (dx, dy) = (x - cx, y - cy);
    let (c, s) = (radians.cos(), radians.sin());
    (
        cx + (dx * c + dy * s) / scale,
        cy + (-dx * s + dy * c) / scale,
    )
}

fn sample_in(x: f64, y: f64, w: f64, h: f64) -> bool {
    (0.0..w).contains(&x) && (0.0..h).contains(&y)
}

/// Progress curve for a style: accelerating (50) and decelerating (54)
/// fades bend the linear time.
pub fn eased(style: i32, t: f64) -> f64 {
    match style {
        50 => t * t,
        54 => 1.0 - (1.0 - t) * (1.0 - t),
        _ => t,
    }
}

/// The destination rectangle an effect copies into (for callers that
/// compose `after` themselves).
pub fn sel_rects(values: &[i32]) -> (Rect, (i32, i32)) {
    let get = |i: usize| values.get(i).copied().unwrap_or(0);
    (
        Rect::from_corners(get(0), get(1), get(2), get(3)),
        (get(4), get(5)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pictures() -> (Surface, Surface) {
        let mut before = Surface::new(64, 48);
        let mut after = Surface::new(64, 48);
        before.fill(before.rect(), [0, 0, 0, 255], 255);
        after.fill(after.rect(), [200, 100, 50, 255], 255);
        (before, after)
    }

    const STYLES: &[i32] = &[
        0, 1, 2, 4, 5, 10, 15, 16, 17, 18, 20, 21, 25, 27, 30, 31, 34, 35, 36, 37, 38, 39, 40, 45,
        50, 54, 61, 62, 63, 64, 65, 100, 101, 102, 110, 111, 112, 113, 114, 115, 116, 117, 118,
        119, 120, 130, 140, 150, 160, 161, 162, 163, 164, 165, 166, 170, 171, 180, 181, 185, 186,
        187, 190, 191, 194, 195, 196, 197, 200, 201, 202, 203, 204, 220, 221, 222, 223, 224, 230,
        231, 232, 233, 234, 240, 241, 242, 243, 244, 260, 261, 262, 263, 264, 265, 266, 267, 268,
        269, 270, 280, 290, 900, 901, 902, 903,
    ];

    #[test]
    fn every_style_starts_old_and_ends_new() {
        let (before, after) = pictures();
        for &style in STYLES {
            for direction in 0..4 {
                let tr = Transition {
                    style,
                    direction,
                    time: 100,
                    opacity: 255,
                    ..Transition::default()
                };
                let end = render(&tr, 1.0, &before, &after);
                assert_eq!(end.rgba, after.rgba, "style {style} dir {direction} end");
                let start = render(&tr, 0.0, &before, &after);
                if !matches!(style, 1 | 40 | 185 | 187) {
                    let old = start.rgba.chunks_exact(4).filter(|p| p[0] == 0).count();
                    assert!(
                        old * 10 >= (64 * 48) * 7,
                        "style {style} dir {direction} starts mostly old ({old})"
                    );
                }
            }
        }
    }

    #[test]
    fn wipe_progresses_monotonically() {
        let (before, after) = pictures();
        let tr = Transition {
            style: 10,
            direction: 2,
            opacity: 255,
            ..Transition::default()
        };
        let revealed = |t: f64| {
            render(&tr, t, &before, &after)
                .rgba
                .chunks_exact(4)
                .filter(|p| p[0] == 200)
                .count()
        };
        assert!(revealed(0.25) < revealed(0.5));
        assert!(revealed(0.5) < revealed(0.75));
    }
}
