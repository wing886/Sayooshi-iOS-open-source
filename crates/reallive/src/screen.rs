//! Composition of the whole screen: DC 0, objects and text windows.

use crate::graphics::ShowFlags;
use crate::surface::{Blend, Rect, Surface};
use crate::system::System;

fn show_flags(sys: &System) -> ShowFlags {
    ShowFlags {
        object1: sys.settings.show_object[0],
        object2: sys.settings.show_object[1],
        weather: sys.settings.show_weather,
    }
}

fn round(offset: f64) -> i32 {
    offset.round() as i32
}

/// The scene as it currently stands (ignores a running transition).
pub fn compose_scene(sys: &mut System) -> Surface {
    compose_layers(sys, true)
}

/// The scene with or without the text windows (`CAPTUREBANK`).
pub fn compose_layers(sys: &mut System, windows: bool) -> Surface {
    let now = sys.now();
    let show = show_flags(sys);
    let (_, layers, layer) = sys.gfx.shake_offsets(now);
    let offset = (round(layer.dx), round(layer.dy));
    let dc0 = sys.gfx.dc(0).expect("DC 0 exists");
    let mut frame = if layers.background && offset != (0, 0) {
        let mut frame = Surface::new(dc0.width, dc0.height);
        frame.blit(&dc0, dc0.rect(), offset.0, offset.1, 255, Blend::Copy, None);
        frame
    } else {
        (*dc0).clone()
    };
    if let Some(hik) = &mut sys.gfx.hik {
        hik.render(&mut frame, now);
    }
    if let Some(table) = sys.gfx.background_tone {
        tone(sys, &mut frame, table);
    }
    let object_offset = if layers.objects { offset } else { (0, 0) };
    sys.gfx
        .draw_objects_offset(&mut frame, now, show, object_offset);
    if let Some(table) = sys.gfx.foreground_tone {
        tone(sys, &mut frame, table);
    }
    if windows {
        let window_offset = if layers.window { offset } else { (0, 0) };
        let text_offset = if layers.text { offset } else { (0, 0) };
        crate::textout::draw_windows(sys, &mut frame, window_offset, text_offset);
        crate::select::draw_buttons(sys, &mut frame);
    }
    frame
}

fn tone(sys: &mut System, frame: &mut Surface, table: usize) {
    sys.gfx.tone_curves.apply_to(frame, table);
}

/// What the player sees this frame.
pub fn compose(sys: &mut System) -> Surface {
    let now = sys.now();
    let mut frame = match &sys.gfx.transition_frame {
        Some(frame) => (**frame).clone(),
        None => compose_scene(sys),
    };
    let (screen, ..) = sys.gfx.shake_offsets(now);
    if screen.dx != 0.0 || screen.dy != 0.0 || screen.zoom != 0.0 {
        let mut shaken = Surface::new(frame.width, frame.height);
        let scale = 1.0 + screen.zoom;
        let (w, h) = (f64::from(frame.width), f64::from(frame.height));
        let dest = Rect::new(
            round(w * (1.0 - scale) / 2.0 + screen.dx),
            round(h * (1.0 - scale) / 2.0 + screen.dy),
            round(w * scale),
            round(h * scale),
        );
        shaken.stretch_blit(&frame, frame.rect(), dest, 255, Blend::Copy);
        frame = shaken;
    }
    if let Some((percent, cx, cy)) = sys.gfx.screen_zoom
        && percent > 0
        && percent != 100
    {
        // The part of the screen that fills it, centred on (cx, cy).
        let (w, h) = (frame.width * 100 / percent, frame.height * 100 / percent);
        let x = (cx - w / 2).clamp(0, (frame.width - w).max(0));
        let y = (cy - h / 2).clamp(0, (frame.height - h).max(0));
        let mut zoomed = Surface::new(frame.width, frame.height);
        zoomed.stretch_blit(
            &frame,
            Rect::new(x, y, w, h),
            frame.rect(),
            255,
            Blend::Copy,
        );
        frame = zoomed;
    }
    if let Some(flash) = &sys.gfx.flash {
        let alpha = flash.alpha(now);
        if alpha > 0 {
            let [r, g, b] = flash.colour;
            let area = flash.area.unwrap_or(frame.rect());
            frame.fill(area, [r, g, b, 255], alpha);
        }
    }
    if let Some(movie) = &sys.movie {
        if let Some(picture) = &movie.current {
            let dest = movie.dest.unwrap_or(frame.rect());
            frame.stretch_blit(picture, picture.rect(), dest, 255, Blend::Copy);
        }
    }
    crate::ui_render::draw_overlay(sys, &mut frame);
    frame
}
