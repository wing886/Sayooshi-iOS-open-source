//! Renders a contact sheet of every transition style at 25/50/75 %.
//!
//! ```text
//! rl_transitions <out.png> [direction]
//! ```
use anyhow::{Context, Result};
use reallive::effects::{Transition, eased, render};
use reallive::surface::Surface;

const STYLES: &[i32] = &[
    0, 4, 5, 10, 15, 16, 17, 18, 20, 21, 25, 27, 30, 31, 34, 35, 36, 38, 40, 45, 50, 61, 62, 63,
    64, 65, 100, 101, 102, 110, 111, 112, 113, 114, 116, 118, 120, 130, 140, 150, 160, 161, 162,
    165, 170, 180, 181, 185, 186, 190, 194, 200, 202, 220, 230, 260, 262, 265, 269, 900,
];

fn picture(w: i32, h: i32, new: bool) -> Surface {
    let mut s = Surface::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let i = ((y * w + x) * 4) as usize;
            let (r, g, b) = if new {
                ((x * 255 / w) as u8, 80, (y * 255 / h) as u8)
            } else {
                let c = if (x / 8 + y / 8) % 2 == 0 { 230 } else { 170 };
                (c, c, 60)
            };
            s.rgba[i..i + 4].copy_from_slice(&[r, g, b, 255]);
        }
    }
    s
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let out = args
        .next()
        .context("usage: rl_transitions <out.png> [direction]")?;
    let direction: i32 = args.next().and_then(|d| d.parse().ok()).unwrap_or(0);
    let (w, h) = (96, 72);
    let before = picture(w, h, false);
    let after = picture(w, h, true);
    let columns = 3 * 4;
    let rows = STYLES.len().div_ceil(4) as i32;
    let mut sheet = Surface::new(columns * (w + 4), rows * (h + 4));
    for (index, &style) in STYLES.iter().enumerate() {
        let tr = Transition {
            style,
            direction,
            xsize: 8,
            ysize: 8,
            interpolation: 1,
            opacity: 255,
            time: 1000,
            ..Transition::default()
        };
        for (k, t) in [0.25, 0.5, 0.75].into_iter().enumerate() {
            let frame = render(&tr, eased(style, t), &before, &after);
            let cell = (index as i32 % 4) * 3 + k as i32;
            let (x, y) = (cell * (w + 4), (index as i32 / 4) * (h + 4));
            sheet.blit(
                &frame,
                frame.rect(),
                x,
                y,
                255,
                reallive::surface::Blend::Copy,
                None,
            );
        }
    }
    image::save_buffer(
        &out,
        &sheet.rgba,
        sheet.width as u32,
        sheet.height as u32,
        image::ColorType::Rgba8,
    )?;
    println!("{} styles", STYLES.len());
    Ok(())
}
