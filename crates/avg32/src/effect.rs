//! AVG32 screen transitions.
//!
//! An [`Effect`] is a transition record: a source
//! rectangle on `srcpdt`, a destination origin on `dstpdt`, a transition
//! number (`cmd`), a per-step delay and a handful of pattern arguments.
//! [`EffectRunner::step`] advances one step whenever the step delay has
//! elapsed (or immediately while skipping) and clears `cmd` once the
//! transition has finished — exactly how the scenario loop polls it.

use crate::buffer::{PdtBuffer, blend_pixel, fade_pixel};
use crate::pdtmgr::{Blit, PdtManager, Rect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Effect {
    pub sx1: i32,
    pub sy1: i32,
    pub sx2: i32,
    pub sy2: i32,
    pub dx: i32,
    pub dy: i32,
    pub cmd: i32,
    /// Milliseconds between steps.
    pub steptime: u32,
    pub prevtime: u64,
    pub srcpdt: i32,
    pub dstpdt: i32,
    pub mask: i32,
    pub step: i32,
    pub curcount: i32,
    pub arg1: i32,
    pub arg2: i32,
    pub arg3: i32,
    pub arg4: i32,
    pub arg5: i32,
    pub arg6: i32,
}

impl Effect {
    /// A `#SEL` table entry (`sx1, sy1, sx2, sy2, dx, dy, time, cmd, mask,
    /// arg2, arg3, arg4, step, arg5, arg6`).
    pub fn from_sel(values: &[i32]) -> Self {
        let value = |index: usize| values.get(index).copied().unwrap_or(0);
        Self {
            sx1: value(0),
            sy1: value(1),
            sx2: value(2),
            sy2: value(3),
            dx: value(4),
            dy: value(5),
            steptime: value(6).max(0) as u32,
            cmd: value(7),
            mask: value(8),
            arg2: value(9),
            arg3: value(10),
            arg4: value(11),
            step: value(12),
            arg5: value(13),
            arg6: value(14),
            srcpdt: 1,
            dstpdt: 0,
            ..Self::default()
        }
    }

    pub fn active(&self) -> bool {
        self.cmd != 0
    }

    fn width(&self) -> i32 {
        self.sx2 - self.sx1
    }

    fn height(&self) -> i32 {
        self.sy2 - self.sy1
    }

    fn whole_destination(&self) -> Rect {
        Rect::new(
            self.dx,
            self.dy,
            self.dx + self.sx2 - self.sx1,
            self.dy + self.sy2 - self.sy1,
        )
    }
}

/// Mutable scratch state shared by the transitions (random line tables,
/// the fan-angle table, and snapshots of the pre-transition screen).
#[derive(Debug, Clone, Default)]
pub struct EffectRunner {
    lines: Vec<i32>,
    angles: Vec<u8>,
    snapshot: Option<PdtBuffer>,
    /// Transition 1000 finishes by re-arranging the working buffer, which
    /// can only happen once both buffers are back in the bank.
    pan_finish: Option<(bool, i32, Rect)>,
}

const EFF4X: [i32; 16] = [0, 2, 0, 2, 1, 3, 1, 3, 0, 2, 0, 2, 1, 3, 1, 3];
const EFF4Y: [i32; 16] = [0, 2, 2, 0, 1, 3, 3, 1, 1, 3, 3, 1, 0, 2, 2, 0];
const EFF61X: [i32; 16] = [0, 1, 2, 3, 3, 3, 3, 2, 1, 0, 0, 0, 1, 2, 2, 1];
const EFF61Y: [i32; 16] = [0, 0, 0, 0, 1, 2, 3, 3, 3, 3, 2, 1, 1, 1, 2, 2];
const EFF62X: [i32; 16] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
const EFF62Y: [i32; 16] = [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];

/// Source and destination buffers for one transition step.  When source
/// and destination are the same slot the source is a snapshot.
struct Pair {
    src: PdtBuffer,
    dst: PdtBuffer,
    src_index: usize,
    dst_index: usize,
    same: bool,
}

impl Pair {
    fn blend(&mut self, sx: i32, sy: i32, dx: i32, dy: i32) {
        self.blend_masked(sx, sy, dx, dy, sx, sy);
    }

    /// Composites `src(sx, sy)` using the mask of `src(mx, my)`.
    fn blend_masked(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, mx: i32, my: i32) {
        if !self.src.contains(sx, sy) || !self.dst.contains(dx, dy) || !self.src.contains(mx, my) {
            return;
        }
        let s = self.src.at(sx, sy);
        let m = self.src.mask[self.src.at(mx, my)];
        let d = self.dst.at(dx, dy);
        blend_pixel(self.src.pixel(s), &mut self.dst.rgb[d * 3..d * 3 + 3], m);
    }

    fn copy(&mut self, sx: i32, sy: i32, dx: i32, dy: i32) {
        if !self.src.contains(sx, sy) || !self.dst.contains(dx, dy) {
            return;
        }
        let s = self.src.at(sx, sy);
        let d = self.dst.at(dx, dy);
        self.dst.set_pixel(d, self.src.pixel(s));
    }

    /// Copies a destination pixel onto another destination pixel.
    fn shift(&mut self, fx: i32, fy: i32, tx: i32, ty: i32) {
        if !self.dst.contains(fx, fy) || !self.dst.contains(tx, ty) {
            return;
        }
        let from = self.dst.at(fx, fy);
        let to = self.dst.at(tx, ty);
        let pixel = self.dst.pixel(from);
        self.dst.set_pixel(to, pixel);
    }

    fn black(&mut self, x: i32, y: i32) {
        if self.dst.contains(x, y) {
            let at = self.dst.at(x, y);
            self.dst.set_pixel(at, [0, 0, 0]);
        }
    }
}

impl EffectRunner {
    /// Advances `effect` by at most one step.  Returns the destination
    /// region touched (already presented when the destination is buffer 0).
    pub fn step(
        &mut self,
        effect: &mut Effect,
        manager: &mut PdtManager,
        now: u64,
        skip: bool,
        random: &mut dyn FnMut(i32) -> i32,
    ) {
        if effect.cmd == 0 {
            return;
        }
        if !skip && now.saturating_sub(effect.prevtime) < u64::from(effect.steptime) {
            return;
        }
        effect.prevtime = now;
        let Some((src_index, _)) = crate::pdtmgr::resolve(effect.srcpdt) else {
            effect.cmd = 0;
            return;
        };
        let Some((dst_index, update)) = crate::pdtmgr::resolve(effect.dstpdt) else {
            effect.cmd = 0;
            return;
        };
        if manager.get(src_index).is_none() {
            effect.cmd = 0;
            return;
        }
        manager.ensure(dst_index);
        if effect.curcount == 0 && effect.cmd != 9999 {
            match Blit::clip(
                Rect::new(effect.sx1, effect.sy1, effect.sx2, effect.sy2),
                effect.dx,
                effect.dy,
                (640, 480),
            ) {
                Some(blit) => {
                    effect.sx1 = blit.source.x1;
                    effect.sy1 = blit.source.y1;
                    effect.sx2 = blit.source.x2;
                    effect.sy2 = blit.source.y2;
                    effect.dx = blit.dx;
                    effect.dy = blit.dy;
                }
                None => {
                    effect.cmd = 0;
                    return;
                }
            }
        }
        // Transitions that are expressed through other primitives.
        match effect.cmd {
            160 | 163 | 9999 => {
                self.stretch_family(effect, manager, dst_index);
                return;
            }
            _ => {}
        }
        let same = src_index == dst_index;
        let dst = manager.take(dst_index).expect("ensured");
        let src = if same {
            dst.clone()
        } else {
            manager.take(src_index).expect("checked")
        };
        let mut pair = Pair {
            src,
            dst,
            src_index,
            dst_index,
            same,
        };
        let mut backup = false;
        let region = self.run(effect, &mut pair, manager, random, &mut backup);
        let Pair {
            src,
            dst,
            src_index,
            dst_index,
            same,
        } = pair;
        if !same {
            manager.set(src_index, Some(src));
        }
        manager.set(dst_index, Some(dst));
        if let Some((down, lines, rect)) = self.pan_finish.take() {
            let Rect { x1, y1, x2, y2 } = rect;
            if down {
                manager.copy(Rect::new(x1, y1, x2, y2 - lines), 1, x1, y1 + lines, 1, 0);
                manager.copy(Rect::new(x1, y2 - lines + 1, x2, y2), 3, x1, y1, 1, 0);
            } else {
                manager.copy(Rect::new(x1, y1 + lines, x2, y2), 1, x1, y1, 1, 0);
                manager.copy(
                    Rect::new(x1, y1, x2, y1 + lines - 1),
                    3,
                    x1,
                    y2 - lines + 1,
                    1,
                    0,
                );
            }
        }
        if dst_index == 0 && update {
            manager.present(region);
        }
        if backup {
            manager.all_copy(0, 3, 0);
        }
    }

    fn run(
        &mut self,
        e: &mut Effect,
        p: &mut Pair,
        manager: &mut PdtManager,
        random: &mut dyn FnMut(i32) -> i32,
        backup: &mut bool,
    ) -> Rect {
        let w = e.width();
        let h = e.height();
        let whole = e.whole_destination();
        match e.cmd {
            0 => {
                e.cmd = 0;
                whole
            }
            4 | 5 => {
                let phase = (e.curcount as usize).min(15);
                let mut y = e.sy1 + EFF4Y[phase];
                while y <= e.sy2 {
                    let mut x = e.sx1 + EFF4X[phase];
                    while x <= e.sx2 {
                        p.blend(x, y, e.dx + x - e.sx1, e.dy + y - e.sy1);
                        x += 4;
                    }
                    y += 4;
                }
                e.curcount += 1;
                if e.curcount >= 16 {
                    e.cmd = 0;
                    *backup = true;
                }
                whole
            }
            10 | 11 => {
                if e.step == 0 {
                    e.step = 1;
                }
                for y in e.curcount * e.step..(e.curcount + 1) * e.step {
                    if y <= h {
                        let (sy, ty) = if e.cmd == 10 {
                            (e.sy1 + y, e.dy + y)
                        } else {
                            (e.sy2 - y, e.dy + h - y)
                        };
                        for x in e.sx1..=e.sx2 {
                            p.blend(x, sy, e.dx + x - e.sx1, ty);
                        }
                    } else {
                        e.cmd = 0;
                    }
                }
                e.curcount += 1;
                whole
            }
            12 | 13 => {
                if e.step == 0 {
                    e.step = 1;
                }
                for x in e.curcount * e.step..(e.curcount + 1) * e.step {
                    if x <= w {
                        let (sx, tx) = if e.cmd == 12 {
                            (e.sx1 + x, e.dx + x)
                        } else {
                            (e.sx2 - x, e.dx + w - x)
                        };
                        for y in e.sy1..=e.sy2 {
                            p.blend(sx, y, tx, e.dy + y - e.sy1);
                        }
                    } else {
                        e.cmd = 0;
                    }
                }
                e.curcount += 1;
                whole
            }
            15..=18 => {
                if e.step == 0 {
                    e.step = 1;
                }
                let span = if e.cmd <= 16 { h } else { w };
                e.curcount += e.step;
                if e.curcount > span {
                    e.step -= e.curcount - (span + 1);
                    e.curcount = span;
                }
                let c = e.curcount;
                let step = e.step;
                match e.cmd {
                    15 => {
                        for y in (e.dy..=e.dy + h).rev() {
                            if y - step < e.dy {
                                let sy = e.sy2 - (e.dy - (y - c));
                                for x in 0..=w {
                                    p.copy(e.sx1 + x, sy, e.dx + x, y);
                                }
                            } else {
                                for x in 0..=w {
                                    p.shift(e.dx + x, y - step, e.dx + x, y);
                                }
                            }
                        }
                    }
                    16 => {
                        for y in e.dy..=e.dy + h {
                            if y + step > e.dy + h {
                                let sy = e.sy1 + y + c - (e.dy + h);
                                for x in 0..=w {
                                    p.copy(e.sx1 + x, sy, e.dx + x, y);
                                }
                            } else {
                                for x in 0..=w {
                                    p.shift(e.dx + x, y + step, e.dx + x, y);
                                }
                            }
                        }
                    }
                    17 => {
                        for x in (e.dx..=e.dx + w).rev() {
                            if x - step < e.dx {
                                let sx = e.sx2 - (e.dx - (x - c));
                                for y in 0..=h {
                                    p.copy(sx, e.sy1 + y, x, e.dy + y);
                                }
                            } else {
                                for y in 0..=h {
                                    p.shift(x - step, e.dy + y, x, e.dy + y);
                                }
                            }
                        }
                    }
                    _ => {
                        for x in e.dx..=e.dx + w {
                            if x + step > e.dx + w {
                                let sx = e.sx1 + x + c - (e.dx + w);
                                for y in 0..=h {
                                    p.copy(sx, e.sy1 + y, x, e.dy + y);
                                }
                            } else {
                                for y in 0..=h {
                                    p.shift(x + step, e.dy + y, x, e.dy + y);
                                }
                            }
                        }
                    }
                }
                if e.curcount >= span {
                    e.cmd = 0;
                }
                whole
            }
            20..=23 => {
                if e.step == 0 {
                    e.step = 1;
                }
                let span = if e.cmd <= 21 { h } else { w };
                e.curcount += e.step;
                if e.curcount > span {
                    e.step -= e.curcount - (span + 1);
                    e.curcount = span;
                }
                let c = e.curcount;
                match e.cmd {
                    20 => {
                        for y in (e.dy..=e.dy + h).rev() {
                            if y - c < e.dy {
                                let sy = e.sy2 - (e.dy - (y - c));
                                for x in 0..=w {
                                    p.copy(e.sx1 + x, sy, e.dx + x, y);
                                }
                            }
                        }
                    }
                    21 => {
                        for y in e.dy..=e.dy + h {
                            if y + c > e.dy + h {
                                let sy = e.sy1 + y + c - (e.dy + h);
                                for x in 0..=w {
                                    p.copy(e.sx1 + x, sy, e.dx + x, y);
                                }
                            }
                        }
                    }
                    22 => {
                        for x in (e.dx..=e.dx + w).rev() {
                            if x - c < e.dx {
                                let sx = e.sx2 - (e.dx - (x - c));
                                for y in 0..=h {
                                    p.copy(sx, e.sy1 + y, x, e.dy + y);
                                }
                            }
                        }
                    }
                    _ => {
                        for x in e.dx..=e.dx + w {
                            if x + c > e.dx + w {
                                let sx = e.sx1 + x + c - (e.dx + w);
                                for y in 0..=h {
                                    p.copy(sx, e.sy1 + y, x, e.dy + y);
                                }
                            }
                        }
                    }
                }
                if e.curcount >= span {
                    e.cmd = 0;
                }
                whole
            }
            25 | 26 | 45 => {
                let c = e.curcount;
                let n = if e.cmd == 26 { w.max(h) / 2 - c } else { c };
                let (mut xx, mut yy) = (w / 2 - n, h / 2 - n);
                let (mut xx2, mut yy2) = (w / 2 + n + 1, h / 2 + n + 1);
                let (mut left, mut top, mut right, mut bottom) = (true, true, true, true);
                if xx < 0 {
                    xx = 0;
                    left = false;
                }
                if yy < 0 {
                    yy = 0;
                    top = false;
                }
                if xx2 > w {
                    xx2 = w;
                    right = false;
                }
                if yy2 > h {
                    yy2 = h;
                    bottom = false;
                }
                let mut hline = |p: &mut Pair, y: i32, from: i32, to: i32| {
                    for x in from..=to {
                        p.blend(e.sx1 + x, e.sy1 + y, e.dx + x, e.dy + y);
                    }
                };
                match e.cmd {
                    25 => {
                        hline(p, yy, xx, xx2);
                        hline(p, yy2, xx, xx2);
                        for y in yy..=yy2 {
                            p.blend(e.sx1 + xx, e.sy1 + y, e.dx + xx, e.dy + y);
                            p.blend(e.sx1 + xx2, e.sy1 + y, e.dx + xx2, e.dy + y);
                        }
                    }
                    26 => {
                        if top {
                            hline(p, yy, xx, xx2);
                        }
                        if bottom {
                            hline(p, yy2, xx, xx2);
                        }
                        for y in yy..=yy2 {
                            if left {
                                p.blend(e.sx1 + xx, e.sy1 + y, e.dx + xx, e.dy + y);
                            }
                            if right {
                                p.blend(e.sx1 + xx2, e.sy1 + y, e.dx + xx2, e.dy + y);
                            }
                        }
                    }
                    _ => {
                        if top {
                            hline(p, yy, 0, w / 2);
                        }
                        if bottom {
                            hline(p, yy2, w / 2, w);
                        }
                        if right {
                            for y in 0..=h / 2 {
                                p.blend(e.sx1 + xx2, e.sy1 + y, e.dx + xx2, e.dy + y);
                            }
                        }
                        if left {
                            for y in h / 2..=h {
                                p.blend(e.sx1 + xx, e.sy1 + y, e.dx + xx, e.dy + y);
                            }
                        }
                    }
                }
                e.curcount += 1;
                let finished = if e.cmd == 26 {
                    n <= 0
                } else {
                    e.curcount > w / 2 && e.curcount > h / 2
                };
                if finished {
                    e.cmd = 0;
                }
                whole
            }
            30 => {
                let x = (e.sx1 + e.curcount) & !1;
                if (e.sx1..=e.sx2).contains(&x) {
                    for y in e.sy1..=e.sy2 {
                        p.blend(x, y, x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                    }
                }
                let x = ((e.sx2 - e.curcount) & !1) + 1;
                if (e.sx1..=e.sx2).contains(&x) {
                    for y in e.sy1..=e.sy2 {
                        p.blend(x, y, x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                    }
                }
                if e.curcount > w {
                    e.cmd = 0;
                }
                e.curcount += 2;
                whole
            }
            31 => {
                let y = (e.sy1 + e.curcount) & !1;
                if (e.sy1..=e.sy2).contains(&y) {
                    for x in e.sx1..=e.sx2 {
                        p.blend(x, y, x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                    }
                }
                let y = ((e.sy2 - e.curcount) & !1) + 1;
                if (e.sy1..=e.sy2).contains(&y) {
                    for x in e.sx1..=e.sx2 {
                        p.blend(x, y, x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                    }
                }
                if e.curcount > h {
                    e.cmd = 0;
                }
                e.curcount += 2;
                whole
            }
            35 | 36 => {
                if e.step == 0 {
                    e.step = 32;
                }
                if e.arg5 == 0 {
                    e.arg5 = 1;
                }
                if e.curcount == 0 {
                    self.snapshot = Some(p.dst.clone());
                }
                let Some(old) = self.snapshot.clone() else {
                    e.cmd = 0;
                    return whole;
                };
                let span = if e.cmd == 35 { w } else { h };
                e.curcount = (e.curcount + e.step).min(span);
                let c = e.curcount;
                let mut over_old = |p: &mut Pair, sx: i32, sy: i32, tx: i32, ty: i32| {
                    if !p.src.contains(sx, sy) || !p.dst.contains(tx, ty) || !old.contains(tx, ty) {
                        return;
                    }
                    let s = p.src.at(sx, sy);
                    let under = old.pixel(old.at(tx, ty));
                    let d = p.dst.at(tx, ty);
                    p.dst.set_pixel(d, under);
                    blend_pixel(
                        p.src.pixel(s),
                        &mut p.dst.rgb[d * 3..d * 3 + 3],
                        p.src.mask[s],
                    );
                };
                if e.cmd == 35 {
                    for y in 0..=h {
                        for i in 0..=c {
                            if (y / e.arg5) & 1 != 0 {
                                over_old(p, e.sx1 + i, e.sy1 + y, e.dx + w - c + i, e.dy + y);
                            } else {
                                over_old(p, e.sx2 - c + i, e.sy1 + y, e.dx + i, e.dy + y);
                            }
                        }
                    }
                } else {
                    for x in 0..=w {
                        for i in 0..=c {
                            if (x / e.arg5) & 1 != 0 {
                                over_old(p, e.sx1 + x, e.sy1 + i, e.dx + x, e.dy + h - c + i);
                            } else {
                                over_old(p, e.sx1 + x, e.sy2 - c + i, e.dx + x, e.dy + i);
                            }
                        }
                    }
                }
                if c >= span {
                    e.cmd = 0;
                    self.snapshot = None;
                }
                whole
            }
            40 => {
                let maxx = (w / 2).max(1);
                let xx = w / 2 + e.sx1;
                let yy = h / 2 + e.sy1;
                let yy2 = ((e.curcount + 1) * 19) / maxx;
                let c = e.curcount;
                let map = |x: i32, y: i32| (x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                for row in [yy - yy2, yy + yy2 + 1] {
                    if row >= e.sy1 && row <= e.sy2 {
                        for x in (xx - c)..=(xx + c + 1) {
                            let (tx, ty) = map(x, row);
                            p.blend(x, row, tx, ty);
                        }
                    }
                }
                for column in [xx - c, xx + c + 1] {
                    for y in (yy - yy2)..=(yy + yy2 + 1) {
                        let (tx, ty) = map(column, y);
                        p.blend(column, y, tx, ty);
                    }
                }
                e.curcount += 1;
                if e.curcount >= maxx {
                    for y in e.sy1..=e.sy2 {
                        if y < yy - 19 || y > yy + 20 {
                            for x in e.sx1..=e.sx2 {
                                let (tx, ty) = map(x, y);
                                p.blend(x, y, tx, ty);
                            }
                        }
                    }
                    e.cmd = 0;
                }
                whole
            }
            41 => {
                let maxx = (w / 2).max(1);
                let yy = h / 2 + e.sy1;
                let yy2 = 19 - ((e.curcount + 1) * 19) / maxx;
                let map = |x: i32, y: i32| (x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                if e.curcount == 0 {
                    for y in e.sy1..=e.sy2 {
                        if y < yy - 19 || y > yy + 20 {
                            for x in e.sx1..=e.sx2 {
                                let (tx, ty) = map(x, y);
                                p.blend(x, y, tx, ty);
                            }
                        }
                    }
                }
                for row in [yy - yy2, yy + yy2 + 1] {
                    if row >= e.sy1 && row <= e.sy2 {
                        for x in e.sx1..=e.sx2 {
                            let (tx, ty) = map(x, row);
                            p.blend(x, row, tx, ty);
                        }
                    }
                }
                for column in [e.sx1 + e.curcount, e.sx2 - e.curcount] {
                    for y in e.sy1..=e.sy2 {
                        let (tx, ty) = map(column, y);
                        p.blend(column, y, tx, ty);
                    }
                }
                e.curcount += 1;
                if e.curcount >= maxx {
                    e.cmd = 0;
                }
                whole
            }
            50 => {
                if e.step == 0 {
                    e.step = 16;
                }
                if e.curcount == 0 {
                    self.snapshot = Some(p.dst.clone());
                }
                let Some(old) = self.snapshot.clone() else {
                    e.cmd = 0;
                    return whole;
                };
                e.curcount += 1;
                let fade = ((e.curcount << 8) / e.step).clamp(0, 256) as u32;
                for y in e.sy1..=e.sy2 {
                    for x in e.sx1..=e.sx2 {
                        let (tx, ty) = (x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                        if !p.src.contains(x, y) || !p.dst.contains(tx, ty) {
                            continue;
                        }
                        let s = p.src.at(x, y);
                        let d = p.dst.at(tx, ty);
                        let under = old.pixel(d);
                        fade_pixel(
                            p.src.pixel(s),
                            under,
                            &mut p.dst.rgb[d * 3..d * 3 + 3],
                            u32::from(p.src.mask[s]),
                            fade,
                        );
                    }
                }
                if e.curcount >= e.step {
                    e.cmd = 0;
                    self.snapshot = None;
                    *backup = true;
                }
                whole
            }
            54 => {
                if e.step == 0 {
                    e.step = 16;
                }
                let high = (0x100 * (e.curcount + 1)) / e.step;
                let low = (0x100 * e.curcount) / e.step;
                for y in e.sy1..=e.sy2 {
                    for x in e.sx1..=e.sx2 {
                        let (tx, ty) = (x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                        if !p.src.contains(x, y) || !p.dst.contains(tx, ty) {
                            continue;
                        }
                        let s = p.src.at(x, y);
                        let d = p.dst.at(tx, ty);
                        let [r, g, b] = if e.arg5 != 0 {
                            p.dst.pixel(d)
                        } else {
                            p.src.pixel(s)
                        }
                        .map(i32::from);
                        let luminance = (r * 299 + g * 587 + b * 114) / 1000;
                        if luminance >= low && luminance < high {
                            p.dst.set_pixel(d, p.src.pixel(s));
                        }
                    }
                }
                e.curcount += 1;
                if e.curcount >= e.step {
                    e.cmd = 0;
                    *backup = true;
                }
                whole
            }
            60..=63 => {
                let c = (e.curcount as usize).min(15);
                let (ox, oy) = match e.cmd {
                    60 => (EFF4X[c], EFF4Y[c]),
                    61 => (EFF61X[c], EFF61Y[c]),
                    62 => (EFF62X[c], EFF62Y[c]),
                    _ => (EFF61X[15 - c], EFF61Y[15 - c]),
                };
                let mut yy = e.sy1 + oy * 4;
                while yy <= e.sy2 {
                    for y in yy..(yy + 4).min(e.sy2 + 1) {
                        let mut xx = e.sx1 + ox * 4;
                        while xx <= e.sx2 {
                            for x in xx..(xx + 4).min(e.sx2 + 1) {
                                p.blend(x, y, x - e.sx1 + e.dx, y - e.sy1 + e.dy);
                            }
                            xx += 16;
                        }
                    }
                    yy += 16;
                }
                e.curcount += 1;
                if e.curcount >= 16 {
                    *backup = e.cmd == 60;
                    e.cmd = 0;
                }
                whole
            }
            70..=73 | 80..=83 => {
                if e.step == 0 {
                    e.step = 32;
                }
                if e.arg5 == 0 {
                    e.arg5 = 64;
                }
                let horizontal = matches!(e.cmd, 70 | 71 | 80 | 81);
                let count = if horizontal { h + 1 } else { w + 1 } as usize;
                let span = if horizontal { w } else { h };
                if e.curcount == 0 || self.lines.len() != count {
                    self.lines = (0..count).map(|_| -(e.step * random(e.arg5))).collect();
                }
                let mut finished = 0usize;
                for line in 0..count {
                    let position = self.lines[line];
                    if position != 10_000 {
                        let position = position + e.step;
                        self.lines[line] = position;
                        if position > 0 {
                            let from = (position - e.step).max(0);
                            let to = position.min(span);
                            let i = line as i32;
                            match e.cmd {
                                70 => {
                                    for x in from..=to {
                                        p.blend(e.sx1 + x, e.sy1 + i, e.dx + x, e.dy + i);
                                    }
                                }
                                71 => {
                                    for k in 0..=(to - from) {
                                        p.blend(
                                            e.sx2 - to + k,
                                            e.sy1 + i,
                                            e.dx + w - to + k,
                                            e.dy + i,
                                        );
                                    }
                                }
                                72 => {
                                    for y in from..=to {
                                        p.blend(e.sx1 + i, e.sy1 + y, e.dx + i, e.dy + y);
                                    }
                                }
                                73 => {
                                    for k in 0..=(to - from) {
                                        p.blend(
                                            e.sx1 + i,
                                            e.sy2 - to + k,
                                            e.dx + i,
                                            e.dy + h - to + k,
                                        );
                                    }
                                }
                                80 => {
                                    for k in 0..=to {
                                        p.blend_masked(
                                            e.sx2 - to + k,
                                            e.sy1 + i,
                                            e.dx + k,
                                            e.dy + i,
                                            e.sx1 + k,
                                            e.sy1 + i,
                                        );
                                    }
                                }
                                81 => {
                                    for k in 0..=to {
                                        p.blend_masked(
                                            e.sx1 + k,
                                            e.sy1 + i,
                                            e.dx + w - to + k,
                                            e.dy + i,
                                            e.sx2 - to + k,
                                            e.sy1 + i,
                                        );
                                    }
                                }
                                82 => {
                                    for k in 0..=to {
                                        p.blend_masked(
                                            e.sx1 + i,
                                            e.sy2 - to + k,
                                            e.dx + i,
                                            e.dy + k,
                                            e.sx1 + i,
                                            e.sy1 + k,
                                        );
                                    }
                                }
                                _ => {
                                    for k in 0..=to {
                                        p.blend_masked(
                                            e.sx1 + i,
                                            e.sy1 + k,
                                            e.dx + i,
                                            e.dy + h - to + k,
                                            e.sx1 + i,
                                            e.sy2 - to + k,
                                        );
                                    }
                                }
                            }
                            if to == span {
                                self.lines[line] = 10_000;
                            }
                        }
                    }
                    if self.lines[line] == 10_000 {
                        finished += 1;
                    }
                }
                e.curcount += 1;
                if finished == count {
                    e.cmd = 0;
                    self.lines.clear();
                }
                whole
            }
            100..=103 => {
                if e.step <= 0 {
                    e.step = 16;
                }
                let offset = if matches!(e.cmd, 100 | 102) {
                    e.curcount
                } else {
                    e.step - e.curcount - 1
                };
                if e.cmd <= 101 {
                    let mut x = e.sx1;
                    while x <= e.sx2 {
                        let sx = x + offset;
                        if sx <= e.sx2 {
                            for y in e.sy1..=e.sy2 {
                                p.blend(sx, y, sx - e.sx1 + e.dx, y - e.sy1 + e.dy);
                            }
                        }
                        x += e.step;
                    }
                } else {
                    let mut y = e.sy1;
                    while y <= e.sy2 {
                        let sy = y + offset;
                        if sy <= e.sy2 {
                            for x in e.sx1..=e.sx2 {
                                p.blend(x, sy, x - e.sx1 + e.dx, sy - e.sy1 + e.dy);
                            }
                        }
                        y += e.step;
                    }
                }
                e.curcount += 1;
                if e.curcount >= e.step {
                    e.cmd = 0;
                }
                whole
            }
            110 | 111 => {
                if e.step == 0 {
                    e.step = 1;
                }
                let columns = e.cmd == 110;
                let (lo, hi, half) = if columns {
                    (e.sx1, e.sx2, w / 2)
                } else {
                    (e.sy1, e.sy2, h / 2)
                };
                for n in 0..e.step {
                    let dif = e.curcount + n;
                    let first = if e.arg5 != 0 {
                        dif + lo
                    } else {
                        half + lo - dif
                    };
                    let second = if e.arg5 != 0 {
                        hi - dif
                    } else {
                        half + lo + dif
                    };
                    for line in [first, second] {
                        if line < lo || line > hi {
                            continue;
                        }
                        if columns {
                            for y in e.sy1..=e.sy2 {
                                p.blend(line, y, line - e.sx1 + e.dx, y - e.sy1 + e.dy);
                            }
                        } else {
                            for x in e.sx1..=e.sx2 {
                                p.blend(x, line, x - e.sx1 + e.dx, line - e.sy1 + e.dy);
                            }
                        }
                    }
                }
                e.curcount += e.step;
                // Stopping at the midpoint leaves one line on odd-sized areas;
                // run until both halves reach their edges.
                if e.curcount > (hi - lo) - half {
                    e.cmd = 0;
                }
                whole
            }
            112 | 113 => {
                if e.step == 0 {
                    e.step = 1;
                }
                let columns = e.cmd == 112;
                let half = if columns { w / 2 } else { h / 2 };
                e.curcount = (e.curcount + e.step).min(half);
                let c = e.curcount;
                let mut line_copy = |p: &mut Pair, from: i32, to: i32| {
                    if columns {
                        for y in 0..=h {
                            p.copy(e.sx1 + from, e.sy1 + y, e.dx + to, e.dy + y);
                        }
                    } else {
                        for x in 0..=w {
                            p.copy(e.sx1 + x, e.sy1 + from, e.dx + x, e.dy + to);
                        }
                    }
                };
                if e.arg5 != 0 {
                    for i in (half - c)..=half {
                        line_copy(p, i, i - (half - c));
                    }
                    for i in half..=half + c {
                        line_copy(p, i, i + half - c);
                    }
                } else {
                    let span = if columns { w } else { h };
                    for i in (half - c)..=half {
                        line_copy(p, i - (half - c), i);
                    }
                    for i in half..=(half + c + (span - 2 * half)).min(span) {
                        line_copy(p, i + half - c, i);
                    }
                }
                if e.curcount >= half {
                    e.cmd = 0;
                }
                whole
            }
            114 | 115 => {
                if e.step == 0 {
                    e.step = 1;
                }
                let columns = e.cmd == 114;
                let span = if columns { w } else { h };
                let half = span / 2;
                if e.curcount + e.step >= half {
                    e.step = half - e.curcount;
                }
                let c = e.curcount;
                let step = e.step;
                let from_source = |p: &mut Pair, source: i32, to: i32| {
                    if columns {
                        for y in 0..=h {
                            p.copy(e.sx1 + source, e.sy1 + y, e.dx + to, e.dy + y);
                        }
                    } else {
                        for x in 0..=w {
                            p.copy(e.sx1 + x, e.sy1 + source, e.dx + x, e.dy + to);
                        }
                    }
                };
                let from_self = |p: &mut Pair, from: i32, to: i32| {
                    if columns {
                        for y in 0..=h {
                            p.shift(e.dx + from, e.dy + y, e.dx + to, e.dy + y);
                        }
                    } else {
                        for x in 0..=w {
                            p.shift(e.dx + x, e.dy + from, e.dx + x, e.dy + to);
                        }
                    }
                };
                if e.arg5 != 0 {
                    for i in (0..=half).rev() {
                        if i <= step {
                            from_source(p, half - (c + step - i), i);
                        } else {
                            from_self(p, i - step, i);
                        }
                    }
                    for i in half + 1..=span {
                        if i >= span - step {
                            from_source(p, half + c + (i - (span - step)), i);
                        } else {
                            from_self(p, i + step, i);
                        }
                    }
                } else {
                    for i in 0..=half {
                        if i >= half - step {
                            from_source(p, c + (i - (half - step)), i);
                        } else {
                            from_self(p, i + step, i);
                        }
                    }
                    for i in (half + 1..=span).rev() {
                        if i <= half + step {
                            from_source(p, span - (c + (half + step - i)), i);
                        } else {
                            from_self(p, i - step, i);
                        }
                    }
                }
                e.curcount += step;
                if e.curcount >= half {
                    e.cmd = 0;
                }
                whole
            }
            120..=123 => {
                if e.step <= 0 {
                    e.step = 16;
                }
                let columns = e.cmd <= 121;
                let forward = matches!(e.cmd, 120 | 122);
                let (lo, hi) = if columns {
                    (e.sx1, e.sx2)
                } else {
                    (e.sy1, e.sy2)
                };
                let mut n = 0;
                let mut line = if forward { lo } else { hi };
                while (forward && line <= hi) || (!forward && line >= lo) {
                    let dif = e.curcount - n;
                    let target = if forward { line + dif } else { line - dif };
                    if dif >= 0 && dif < e.step && target >= lo && target <= hi {
                        if columns {
                            for y in e.sy1..=e.sy2 {
                                p.blend(target, y, target - e.sx1 + e.dx, y - e.sy1 + e.dy);
                            }
                        } else {
                            for x in e.sx1..=e.sx2 {
                                p.blend(x, target, x - e.sx1 + e.dx, target - e.sy1 + e.dy);
                            }
                        }
                    }
                    line = if forward {
                        line + e.step
                    } else {
                        line - e.step
                    };
                    n += 1;
                }
                e.curcount += 1;
                let span = if columns { w } else { h };
                if e.curcount >= span / e.step + e.step {
                    e.cmd = 0;
                }
                whole
            }
            150 => {
                if e.step <= 0 {
                    e.step = 1;
                }
                let maxx = (w + 1).max(1) as usize;
                let maxy = (h + 1).max(1) as usize;
                if e.curcount == 0 || self.angles.len() != maxx * maxy {
                    self.angles = fan_table(maxx, maxy, e.step);
                }
                for y in 0..maxy {
                    for x in 0..maxx {
                        if i32::from(self.angles[y * maxx + x]) == e.curcount {
                            let (sx, sy) = (e.sx1 + x as i32, e.sy1 + y as i32);
                            p.blend(sx, sy, e.dx + x as i32, e.dy + y as i32);
                        }
                    }
                }
                e.curcount += 1;
                if e.curcount * e.step >= 90 {
                    e.cmd = 0;
                    *backup = true;
                    self.angles.clear();
                }
                whole
            }
            170 => {
                if e.curcount == 0 {
                    e.arg5 = 0;
                    manager.set(3, Some(p.dst.clone()));
                }
                let mut wave = manager.get(3).cloned().unwrap_or_else(|| p.dst.clone());
                if e.curcount <= 63 {
                    e.arg5 += 10;
                } else if e.curcount >= 104 {
                    e.arg5 -= 10;
                } else {
                    let n = (e.curcount - 64) << 2;
                    let mut y = e.sy1 + n;
                    while y <= e.sy2 {
                        for row in y..(y + 4).min(e.sy2 + 1) {
                            for x in e.sx1..=e.sx2 {
                                if p.src.contains(x, row) && wave.contains(x, row) {
                                    let pixel = p.src.pixel(p.src.at(x, row));
                                    let at = wave.at(x, row);
                                    wave.set_pixel(at, pixel);
                                }
                            }
                        }
                        y += 160;
                    }
                    manager.set(3, Some(wave.clone()));
                }
                let mut angle = (e.curcount % 90) << 2;
                let mut k = 0;
                while k <= h {
                    let shift = (f64::from(e.arg5)
                        * (f64::from(angle) * std::f64::consts::PI / 180.0).sin())
                        as i32;
                    for row in k..(k + 4).min(h + 1) {
                        let sy = e.sy1 + row;
                        for x in 0..=w {
                            let source = x - shift;
                            let (tx, ty) = (e.dx + x, e.dy + row);
                            if (0..=w).contains(&source) && wave.contains(e.sx1 + source, sy) {
                                let pixel = wave.pixel(wave.at(e.sx1 + source, sy));
                                if p.dst.contains(tx, ty) {
                                    let at = p.dst.at(tx, ty);
                                    p.dst.set_pixel(at, pixel);
                                }
                            } else {
                                p.black(tx, ty);
                            }
                        }
                    }
                    angle = (angle + 4) % 360;
                    k += 4;
                }
                e.curcount += 1;
                if e.curcount >= 168 {
                    e.cmd = 0;
                }
                whole
            }
            180 => {
                let maxx = ((w + 16) >> 4).max(1) as usize;
                let maxy = ((h + 16) >> 4).max(1) as usize;
                if e.curcount == 0 || self.lines.len() != maxx * maxy {
                    self.lines = (0..maxx * maxy).map(|_| random(32)).collect();
                }
                for by in 0..maxy {
                    for bx in 0..maxx {
                        if self.lines[by * maxx + bx] != e.curcount {
                            continue;
                        }
                        for y in 0..16 {
                            for x in 0..16 {
                                let (ox, oy) = (bx as i32 * 16 + x, by as i32 * 16 + y);
                                if ox <= w && oy <= h {
                                    p.blend(e.sx1 + ox, e.sy1 + oy, e.dx + ox, e.dy + oy);
                                }
                            }
                        }
                    }
                }
                e.curcount += 1;
                if e.curcount >= 32 {
                    e.cmd = 0;
                    *backup = true;
                    self.lines.clear();
                }
                whole
            }
            1000 => {
                if e.step == 0 {
                    e.step = 16;
                }
                if e.curcount == 0 {
                    manager.set(3, Some(p.dst.clone()));
                }
                e.curcount += e.step;
                if e.curcount > e.arg6 {
                    e.step -= e.curcount - e.arg6;
                    e.curcount = e.arg6;
                }
                let (c, step) = (e.curcount, e.step);
                let height = e.sy2 - e.sy1;
                if e.arg4 != 0 {
                    for y in (e.sy1..=e.sy2).rev() {
                        for x in e.sx1..=e.sx2 {
                            if y - step < e.sy1 {
                                p.copy(x, e.sy2 - c + (y - e.sy1), x, y);
                            } else {
                                p.shift(x, y - step, x, y);
                            }
                        }
                    }
                } else {
                    for y in e.sy1..=e.sy2 {
                        for x in e.sx1..=e.sx2 {
                            if y + step > e.sy2 {
                                p.copy(x, y + c - height, x, y);
                            } else {
                                p.shift(x, y + step, x, y);
                            }
                        }
                    }
                }
                if e.curcount >= e.arg6 {
                    e.cmd = 0;
                    // Leave the working buffer describing the continuation of
                    // the pan.
                    self.pan_finish =
                        Some((e.arg4 != 0, e.arg6, Rect::new(e.sx1, e.sy1, e.sx2, e.sy2)));
                }
                Rect::new(e.sx1, e.sy1, e.sx2, e.sy2)
            }
            // Unknown transitions still have
            // to show the new picture; present it at once.
            _ => {
                for y in e.sy1..=e.sy2 {
                    for x in e.sx1..=e.sx2 {
                        p.blend(x, y, e.dx + x - e.sx1, e.dy + y - e.sy1);
                    }
                }
                e.cmd = 0;
                whole
            }
        }
    }

    fn stretch_family(&mut self, e: &mut Effect, manager: &mut PdtManager, dst_index: usize) {
        let w = e.width();
        let h = e.height();
        match e.cmd {
            160 => {
                if e.step == 0 {
                    e.step = 32;
                }
                e.curcount += 1;
                let (xx, yy) = (w / 2, h / 2);
                let (xx2, yy2);
                if xx > yy {
                    let grown = if e.curcount * e.step >= xx {
                        e.cmd = 0;
                        xx
                    } else {
                        e.curcount * e.step
                    };
                    xx2 = grown;
                    yy2 = if xx == 0 { 0 } else { yy * grown / xx };
                } else {
                    let grown = if e.curcount * e.step >= yy {
                        e.cmd = 0;
                        yy
                    } else {
                        e.curcount * e.step
                    };
                    yy2 = grown;
                    xx2 = if yy == 0 { 0 } else { xx * grown / yy };
                }
                manager.stretch_copy(
                    Rect::new(e.sx1, e.sy1, e.sx2, e.sy2),
                    e.srcpdt,
                    Rect::new(
                        e.dx + xx - xx2,
                        e.dy + yy - yy2,
                        e.dx + xx + xx2 + 1,
                        e.dy + yy + yy2 + 1,
                    ),
                    e.dstpdt,
                );
                if e.curcount == e.step {
                    e.cmd = 0;
                }
            }
            163 => {
                if e.curcount == 0 {
                    e.steptime <<= 5;
                    let snapshot = manager.get(dst_index).cloned();
                    manager.set(3, snapshot);
                }
                let (xx, yy) = (w / 2, h / 2);
                let (n, source) = if e.curcount < 8 {
                    (e.curcount, 3)
                } else {
                    (15 - e.curcount, e.srcpdt)
                };
                let (xx2, yy2) = (xx >> n, yy >> n);
                manager.stretch_copy(
                    Rect::new(
                        e.sx1 + xx - xx2,
                        e.sy1 + yy - yy2,
                        e.sx1 + xx + xx2 + 1,
                        e.sy1 + yy + yy2 + 1,
                    ),
                    source,
                    Rect::new(e.dx, e.dy, e.dx + w, e.dy + h),
                    e.dstpdt,
                );
                e.curcount += 1;
                if e.curcount >= 16 {
                    e.cmd = 0;
                }
            }
            _ => {
                if e.step <= 0 {
                    e.step = 16;
                }
                if e.curcount == 0 {
                    let Some((src_index, _)) = crate::pdtmgr::resolve(e.srcpdt) else {
                        e.cmd = 0;
                        return;
                    };
                    self.snapshot = manager.get(src_index).cloned();
                    if e.sx1 > e.sx2 {
                        std::mem::swap(&mut e.sx1, &mut e.sx2);
                    }
                    if e.sy1 > e.sy2 {
                        std::mem::swap(&mut e.sy1, &mut e.sy2);
                    }
                    if e.dx > e.arg1 {
                        std::mem::swap(&mut e.dx, &mut e.arg1);
                    }
                    if e.dy > e.arg2 {
                        std::mem::swap(&mut e.dy, &mut e.arg2);
                    }
                    if e.arg3 > e.arg5 {
                        std::mem::swap(&mut e.arg3, &mut e.arg5);
                    }
                    if e.arg4 > e.arg6 {
                        std::mem::swap(&mut e.arg4, &mut e.arg6);
                    }
                }
                let Some(snapshot) = self.snapshot.clone() else {
                    e.cmd = 0;
                    return;
                };
                e.curcount += 1;
                let c = e.curcount;
                let lerp = |from: i32, to: i32| ((to - from) * c) / e.step + from;
                manager.stretch_from(
                    &snapshot,
                    Rect::new(
                        lerp(e.sx1, e.arg3),
                        lerp(e.sy1, e.arg4),
                        lerp(e.sx2, e.arg5),
                        lerp(e.sy2, e.arg6),
                    ),
                    Rect::new(e.dx, e.dy, e.arg1, e.arg2),
                    e.dstpdt,
                );
                if e.curcount >= e.step {
                    e.cmd = 0;
                    self.snapshot = None;
                }
            }
        }
    }
}

/// Per-pixel sweep angle (in `step`-degree buckets) for transition 150,
/// four 90° fans rotating about the centre.
fn fan_table(maxx: usize, maxy: usize, step: i32) -> Vec<u8> {
    let xx = ((maxx as i32 - 1) >> 1) + 1;
    let yy = ((maxy as i32 - 1) >> 1) + 1;
    let degrees = |a: i32, b: i32| f64::from(a).atan2(f64::from(b)).to_degrees() as i32;
    let mut table = vec![0u8; maxx * maxy];
    for y in 0..maxy as i32 {
        for x in 0..maxx as i32 {
            let n = if y < yy {
                if x < xx {
                    if xx - x - 1 == 0 {
                        90
                    } else {
                        degrees(yy - y - 1, xx - x - 1)
                    }
                } else if yy - y - 1 == 0 {
                    90
                } else {
                    degrees(x - xx, yy - y - 1)
                }
            } else if x < xx {
                if y - yy == 0 {
                    90
                } else {
                    degrees(xx - x - 1, y - yy)
                }
            } else if x - xx == 0 {
                90
            } else {
                degrees(y - yy, x - xx)
            };
            table[y as usize * maxx + x as usize] = (n.clamp(0, 89) / step) as u8;
        }
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_to_end(effect: &mut Effect, manager: &mut PdtManager) -> usize {
        let mut runner = EffectRunner::default();
        let mut seed = 7u32;
        let mut random = move |n: i32| {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345) & 0x7fff_ffff;
            if n <= 0 { 0 } else { (seed % n as u32) as i32 }
        };
        let mut steps = 0;
        while effect.active() && steps < 10_000 {
            runner.step(effect, manager, steps as u64 * 1000, false, &mut random);
            steps += 1;
        }
        steps
    }

    #[test]
    fn every_transition_terminates_and_shows_the_source() {
        for cmd in [
            2, 4, 10, 11, 12, 13, 20, 21, 22, 23, 25, 26, 30, 31, 35, 36, 40, 41, 45, 50, 54, 60,
            61, 62, 63, 70, 71, 72, 73, 80, 81, 82, 83, 100, 101, 102, 103, 110, 111, 112, 113,
            114, 115, 120, 121, 122, 123, 150, 160, 163, 180, 999,
        ] {
            let mut manager = PdtManager::new();
            manager.fill_rect(Rect::full(), 1, [200, 10, 10]);
            let mut effect = Effect::from_sel(&[0, 0, 639, 479, 0, 0, 1, cmd]);
            let steps = run_to_end(&mut effect, &mut manager);
            assert!(!effect.active(), "effect {cmd} never finished");
            assert!(steps > 0);
            if !matches!(cmd, 40 | 41 | 45 | 160) {
                assert_eq!(
                    manager.screen()[(240 * 640 + 320) * 3],
                    200,
                    "effect {cmd} did not reveal the centre"
                );
            }
        }
    }
}
