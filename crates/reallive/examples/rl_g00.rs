//! Decodes a g00/pdt bitmap and writes it as PNG, listing its patterns.
//!
//! ```text
//! rl_g00 <file.g00> <out.png>
//! ```
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let input = args.next().context("usage: rl_g00 <file.g00> <out.png>")?;
    let output = args.next().context("usage: rl_g00 <file.g00> <out.png>")?;
    let image = reallive::image::decode(&std::fs::read(&input)?)?;
    println!(
        "{}x{} alpha={}",
        image.width(),
        image.height(),
        image.has_alpha
    );
    for (index, region) in image.regions.iter().enumerate() {
        println!("  pattern {index}: {region:?}");
    }
    image::save_buffer(
        &output,
        &image.surface.rgba,
        image.width() as u32,
        image.height() as u32,
        image::ColorType::Rgba8,
    )?;
    Ok(())
}
