//! Draws the engine's own dialogs and the backlog over the game screen.

use crate::nls::cell_width;
use crate::surface::{Rect, Surface};
use crate::system::System;

const TEXT: [u8; 3] = [240, 240, 240];
const DIM: [u8; 3] = [120, 120, 120];
const HIGHLIGHT: [u8; 3] = [255, 220, 120];

fn darken(frame: &mut Surface, rect: Rect, amount: u32) {
    let rect = rect.intersect(&frame.rect());
    for y in rect.y..rect.bottom() {
        for x in rect.x..rect.right() {
            let at = ((y * frame.width + x) * 4) as usize;
            for c in 0..3 {
                frame.rgba[at + c] = (u32::from(frame.rgba[at + c]) * (255 - amount) / 255) as u8;
            }
        }
    }
}

fn text(
    sys: &mut System,
    frame: &mut Surface,
    s: &str,
    x: i32,
    y: i32,
    size: i32,
    colour: [u8; 3],
) -> i32 {
    let ascent = sys.gfx.fonts.ascent(size as u32).round() as i32;
    let mut pen = x;
    for c in s.chars() {
        if let Some(glyph) = sys.gfx.fonts.glyph(c, size as u32, false) {
            for gy in 0..glyph.height {
                for gx in 0..glyph.width {
                    let a = u32::from(glyph.coverage[gy * glyph.width + gx]);
                    let tx = pen + glyph.left + gx as i32;
                    let ty = y + ascent + glyph.top + gy as i32;
                    if a == 0 || tx < 0 || ty < 0 || tx >= frame.width || ty >= frame.height {
                        continue;
                    }
                    let at = ((ty * frame.width + tx) * 4) as usize;
                    for ch in 0..3 {
                        let d = u32::from(frame.rgba[at + ch]);
                        frame.rgba[at + ch] =
                            ((d * (255 - a) + u32::from(colour[ch]) * a) / 255) as u8;
                    }
                }
            }
        }
        pen += cell_width(c) as i32 * size / 2;
    }
    pen - x
}

fn draw_backlog(sys: &mut System, frame: &mut Surface, page: usize) {
    let (w, h) = (frame.width, frame.height);
    darken(frame, frame.rect(), 170);
    let size = (h / 24).clamp(14, 32);
    let Some(entry) = sys.text.backlog.get(page).cloned() else {
        return;
    };
    let mut y = size * 2;
    let margin = w / 12;
    if !entry.name.is_empty() {
        text(
            sys,
            frame,
            &format!("【{}】", entry.name),
            margin,
            y,
            size,
            HIGHLIGHT,
        );
        y += size * 3 / 2;
    }
    for line in &entry.lines {
        text(sys, frame, line, margin, y, size, TEXT);
        y += size * 3 / 2;
    }
    let footer = format!("{} / {}", page + 1, sys.text.backlog.len());
    text(
        sys,
        frame,
        &footer,
        w - margin - size * 4,
        h - size * 2,
        size * 3 / 4,
        DIM,
    );
}

/// Draws the active dialog (if any) and records its hit rectangles.
pub fn draw_overlay(sys: &mut System, frame: &mut Surface) {
    if let Some(page) = sys.text.backlog_view {
        draw_backlog(sys, frame, page);
    }
    let Some(mut overlay) = sys.ui.overlay.take() else {
        return;
    };
    let (w, h) = (frame.width, frame.height);
    let size = (h / 26).clamp(12, 28);
    let row_h = size * 3 / 2;
    let visible = overlay.rows.len().min(10);
    let box_w = (w * 3 / 4).max(200).min(w);
    let box_h = row_h * (visible as i32 + 2) + size;
    let bx = (w - box_w) / 2;
    let by = ((h - box_h) / 2).max(0);
    darken(frame, Rect::new(bx, by, box_w, box_h), 190);
    let title = overlay.title.clone();
    text(
        sys,
        frame,
        &title,
        bx + size,
        by + size / 2,
        size,
        HIGHLIGHT,
    );
    overlay.hit_rects.clear();
    let first = overlay
        .scroll
        .min(overlay.rows.len().saturating_sub(visible));
    for (slot, index) in (first..first + visible).enumerate() {
        let row = overlay.rows[index].clone();
        let y = by + row_h * (slot as i32 + 1) + size / 2;
        let rect = Rect::new(bx + size / 2, y - size / 4, box_w - size, row_h);
        if index == overlay.selected {
            darken(frame, rect, 120);
        }
        let colour = if !row.enabled {
            DIM
        } else if index == overlay.selected {
            HIGHLIGHT
        } else {
            TEXT
        };
        let mut label = row.text;
        if index == overlay.selected {
            if let Some(editing) = &overlay.editing {
                if !label.ends_with(editing.as_str()) {
                    label.push_str(editing);
                }
                label.push('_');
            }
        }
        text(sys, frame, &label, bx + size, y, size, colour);
        overlay
            .hit_rects
            .push((index, rect.x, rect.y, rect.w, rect.h));
    }
    sys.ui.overlay = Some(overlay);
}
