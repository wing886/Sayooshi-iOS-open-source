//! Renders a UK2 PMD score (`.MMM`) to a 16-bit stereo WAV file.
//!
//! `uk2_music <score.MMM> <out.wav> [seconds]`

use std::io::Write;

use anyhow::{Context, Result};
use uk2::engine::opna::RATE;
use uk2::engine::pmd::Pmd;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let input = args
        .next()
        .context("usage: uk2_music <score.MMM> <out.wav> [seconds]")?;
    let output = args.next().context("missing output path")?;
    let seconds: f64 = args.next().map(|s| s.parse()).transpose()?.unwrap_or(60.0);
    let score = std::fs::read(&input).with_context(|| format!("read {input}"))?;
    let mut pmd = Pmd::new();
    pmd.start(&score);
    let rate = RATE.round() as u32;
    let frames = (seconds * RATE) as usize;
    let mut data = Vec::with_capacity(frames * 4);
    let mut peak = 0.0f32;
    for _ in 0..frames {
        let (left, right) = pmd.sample();
        peak = peak.max(left.abs()).max(right.abs());
        for value in [left, right] {
            let sample = (value.clamp(-1.0, 1.0) * 32767.0) as i16;
            data.extend_from_slice(&sample.to_le_bytes());
        }
    }
    let mut file = std::fs::File::create(&output)?;
    file.write_all(b"RIFF")?;
    file.write_all(&(36 + data.len() as u32).to_le_bytes())?;
    file.write_all(b"WAVEfmt ")?;
    file.write_all(&16u32.to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&2u16.to_le_bytes())?;
    file.write_all(&rate.to_le_bytes())?;
    file.write_all(&(rate * 4).to_le_bytes())?;
    file.write_all(&4u16.to_le_bytes())?;
    file.write_all(&16u16.to_le_bytes())?;
    file.write_all(b"data")?;
    file.write_all(&(data.len() as u32).to_le_bytes())?;
    file.write_all(&data)?;
    eprintln!("wrote {output}: {seconds}s, peak {peak:.3}");
    Ok(())
}
