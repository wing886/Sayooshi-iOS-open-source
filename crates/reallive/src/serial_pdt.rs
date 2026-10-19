//! Serial animations (`snm*`, `SERIALPDT_*`, module 1:34): images shown
//! one after another on DC 0 (optionally stretched to an area), or one
//! image tiled over an area and scrolled. Each of 256 buffers has a
//! foreground and a background slot; the background ones start when the
//! background is promoted (`grpOpenBg` and friends).

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::image::Image;
use crate::surface::{Blend, Rect, Surface};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// Frames placed at a point.
    At(i32, i32),
    /// Frames stretched over an area.
    Stretch(Rect),
    /// The first frame tiled over an area and moved by (dx, dy) over the
    /// animation's time.
    Scroll { area: Rect, dx: i32, dy: i32 },
}

#[derive(Debug, Clone)]
pub struct SerialPdt {
    pub shape: Shape,
    pub frames: Vec<(Rc<Image>, u64)>,
    pub looped: bool,
    start: u64,
    /// While paused: when it stopped (or `None` for ready but not
    /// started).
    paused: Option<Option<u64>>,
    /// What was drawn last (frame index, or scroll step), to redraw only
    /// on changes.
    drawn: Option<i64>,
    pub finished: bool,
    /// Promoted from the background: the clock starts at the next update.
    restart: bool,
}

impl SerialPdt {
    pub fn new(shape: Shape, frames: Vec<(Rc<Image>, u64)>, looped: bool, now: u64) -> Self {
        let finished = frames.is_empty();
        Self {
            shape,
            frames,
            looped,
            start: now,
            paused: None,
            drawn: None,
            finished,
            restart: false,
        }
    }

    /// Set up without running until `resume` (the `*_READY` / `*_PAUSE`
    /// forms).
    pub fn hold(mut self) -> Self {
        self.paused = Some(None);
        self
    }

    fn total(&self) -> u64 {
        match self.shape {
            Shape::Scroll { .. } => self.frames.first().map_or(0, |f| f.1),
            _ => self.frames.iter().map(|f| f.1).sum(),
        }
    }

    pub fn pause(&mut self, now: u64) {
        if self.paused.is_none() {
            self.paused = Some(Some(now));
        }
    }

    pub fn resume(&mut self, now: u64) {
        match self.paused.take() {
            Some(Some(at)) => self.start += now.saturating_sub(at),
            Some(None) => self.start = now,
            None => {}
        }
    }

    pub fn add(&mut self, frames: Vec<(Rc<Image>, u64)>) {
        self.frames.extend(frames);
        if !self.frames.is_empty() && !self.looped {
            self.finished = false;
        }
    }

    /// Advances to `now`, drawing onto `dc` when what shows changed.
    /// Returns whether it drew.
    pub fn update(&mut self, dc: &mut Surface, now: u64) -> bool {
        if self.finished {
            return false;
        }
        if std::mem::take(&mut self.restart) {
            self.start = now;
        }
        let at = match self.paused {
            Some(None) => return false,
            Some(Some(at)) => at,
            None => now,
        };
        let total = self.total();
        let mut elapsed = at.saturating_sub(self.start);
        if total == 0 || elapsed >= total {
            if self.looped && total > 0 {
                elapsed %= total;
            } else {
                elapsed = total;
                self.finished = self.paused.is_none();
            }
        }
        match self.shape.clone() {
            Shape::Scroll { area, dx, dy } => {
                let Some((image, _)) = self.frames.first() else {
                    return false;
                };
                let p = if total == 0 {
                    1.0
                } else {
                    elapsed as f64 / total as f64
                };
                let (ox, oy) = ((f64::from(dx) * p) as i32, (f64::from(dy) * p) as i32);
                let step = i64::from(ox) << 32 | i64::from(oy as u32);
                if self.drawn == Some(step) {
                    return false;
                }
                self.drawn = Some(step);
                let src = &image.surface;
                let (w, h) = (src.width, src.height);
                if w <= 0 || h <= 0 {
                    return false;
                }
                // The first tile starts at or before the area's corner.
                let x0 = area.x + ox.rem_euclid(w) - if ox.rem_euclid(w) > 0 { w } else { 0 };
                let y0 = area.y + oy.rem_euclid(h) - if oy.rem_euclid(h) > 0 { h } else { 0 };
                let mut y = y0;
                while y < area.bottom() {
                    let mut x = x0;
                    while x < area.right() {
                        let d = Rect::new(x, y, w, h).intersect(&area);
                        if !d.is_empty() {
                            let s = Rect::new(d.x - x, d.y - y, d.w, d.h);
                            dc.blit(src, s, d.x, d.y, 255, Blend::Mask, None);
                        }
                        x += w;
                    }
                    y += h;
                }
                true
            }
            shape => {
                let mut index = self.frames.len() - 1;
                let mut left = elapsed;
                for (i, (_, time)) in self.frames.iter().enumerate() {
                    if left < *time {
                        index = i;
                        break;
                    }
                    left -= time;
                }
                if self.drawn == Some(index as i64) {
                    return false;
                }
                self.drawn = Some(index as i64);
                let image = &self.frames[index].0;
                let region = image
                    .regions
                    .first()
                    .copied()
                    .unwrap_or(crate::image::Region::full(
                        image.surface.width,
                        image.surface.height,
                    ));
                let src = Rect::from_corners(region.x1, region.y1, region.x2 + 1, region.y2 + 1);
                match shape {
                    Shape::Stretch(area) => {
                        dc.stretch_blit(&image.surface, src, area, 255, Blend::Mask)
                    }
                    Shape::At(x, y) => dc.blit(&image.surface, src, x, y, 255, Blend::Mask, None),
                    Shape::Scroll { .. } => unreachable!(),
                }
                true
            }
        }
    }
}

/// The foreground and background slots, by buffer.
#[derive(Debug, Clone, Default)]
pub struct SerialPdts {
    pub fg: BTreeMap<i32, SerialPdt>,
    pub bg: BTreeMap<i32, SerialPdt>,
}

impl SerialPdts {
    pub fn layer(&mut self, back: bool) -> &mut BTreeMap<i32, SerialPdt> {
        if back { &mut self.bg } else { &mut self.fg }
    }

    /// The background slots take over the foreground ones (on a
    /// background promotion).
    pub fn promote(&mut self) {
        for (buf, mut snm) in std::mem::take(&mut self.bg) {
            if snm.paused.is_none() {
                snm.restart = true;
            }
            self.fg.insert(buf, snm);
        }
    }

    /// Per frame; true when DC 0 changed.
    pub fn update(&mut self, dc: &mut Surface, now: u64) -> bool {
        let mut drew = false;
        for snm in self.fg.values_mut() {
            drew |= snm.update(dc, now);
        }
        drew
    }

    pub fn running(&self, back: bool, buf: Option<i32>) -> bool {
        // Background slots wait for a promotion, not for time.
        if back {
            return false;
        }
        let layer = &self.fg;
        match buf {
            Some(buf) => layer.get(&buf).is_some_and(|s| !s.finished),
            None => layer.values().any(|s| !s.finished),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid(colour: [u8; 4]) -> Rc<Image> {
        let mut image = Image::new(2, 2);
        let rect = image.surface.rect();
        image.surface.fill(rect, colour, 255);
        Rc::new(image)
    }

    #[test]
    fn frames_follow_their_times_and_finish() {
        let (red, blue) = (solid([255, 0, 0, 255]), solid([0, 0, 255, 255]));
        let mut snm = SerialPdt::new(Shape::At(1, 1), vec![(red, 100), (blue, 100)], false, 0);
        let mut dc = Surface::new(4, 4);
        assert!(snm.update(&mut dc, 50));
        assert_eq!(dc.pixel(1, 1), [255, 0, 0, 255]);
        // Nothing changes within a frame.
        assert!(!snm.update(&mut dc, 60));
        assert!(snm.update(&mut dc, 150));
        assert_eq!(dc.pixel(2, 2), [0, 0, 255, 255]);
        assert!(!snm.finished);
        snm.update(&mut dc, 250);
        assert!(snm.finished);
    }

    #[test]
    fn looping_and_paused_animations_do_not_finish() {
        let red = solid([255, 0, 0, 255]);
        let mut snm = SerialPdt::new(Shape::At(0, 0), vec![(red.clone(), 100)], true, 0);
        let mut dc = Surface::new(4, 4);
        snm.update(&mut dc, 1000);
        assert!(!snm.finished);
        let mut held = SerialPdt::new(Shape::At(0, 0), vec![(red, 100)], false, 0).hold();
        assert!(!held.update(&mut dc, 500));
        held.resume(500);
        assert!(held.update(&mut dc, 550));
        assert!(!held.finished);
    }

    #[test]
    fn scrolling_tiles_the_area() {
        let red = solid([255, 0, 0, 255]);
        let area = Rect::new(0, 0, 4, 4);
        let mut snm = SerialPdt::new(
            Shape::Scroll { area, dx: 2, dy: 0 },
            vec![(red, 100)],
            false,
            0,
        );
        let mut dc = Surface::new(6, 6);
        snm.update(&mut dc, 50);
        assert_eq!(dc.pixel(0, 0), [255, 0, 0, 255]);
        assert_eq!(dc.pixel(3, 3), [255, 0, 0, 255]);
        // Outside the area nothing is drawn.
        assert_eq!(dc.pixel(4, 4)[0], 0);
    }

    #[test]
    fn background_slots_start_when_promoted() {
        let red = solid([255, 0, 0, 255]);
        let mut all = SerialPdts::default();
        all.bg.insert(
            3,
            SerialPdt::new(Shape::At(0, 0), vec![(red, 100)], false, 0),
        );
        let mut dc = Surface::new(2, 2);
        assert!(!all.update(&mut dc, 500));
        all.promote();
        assert!(all.update(&mut dc, 1000));
        assert!(all.running(false, Some(3)));
    }
}
