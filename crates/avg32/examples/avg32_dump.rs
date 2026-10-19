//! Extracts every scenario from an AVG32 installation into a directory.

use anyhow::{Context, Result};
use avg32::game::Avg32Game;

fn main() -> Result<()> {
    if let Ok(nls) = std::env::var("AVG32_NLS") {
        avg32::nls::set(nls.parse()?);
    }
    let mut args = std::env::args().skip(1);
    let root = args
        .next()
        .context("usage: avg32_dump <game-root> <out-dir>")?;
    let out = args
        .next()
        .context("usage: avg32_dump <game-root> <out-dir>")?;
    let game = Avg32Game::open(&root)?;
    std::fs::create_dir_all(&out)?;
    for name in game.scene_names() {
        std::fs::write(
            std::path::Path::new(&out).join(&name),
            game.read_scene(&name)?,
        )?;
    }
    Ok(())
}
