//! Extracts one file through the AVG32 resource routes.
use anyhow::{Context, Result};
fn main() -> Result<()> {
    if let Ok(nls) = std::env::var("AVG32_NLS") {
        avg32::nls::set(nls.parse()?);
    }
    let mut args = std::env::args().skip(1);
    let root = args
        .next()
        .context("usage: avg32_extract <root> <kind> <name> <out>")?;
    let kind = args.next().context("kind")?;
    let name = args.next().context("name")?;
    let out = args.next().context("out")?;
    let game = avg32::Avg32Game::open(root)?;
    std::fs::write(out, game.resources.read(&kind, &name)?)?;
    Ok(())
}
