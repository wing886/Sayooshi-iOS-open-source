//! Hex-dumps the decompressed bytecode of one scenario.
//!
//! ```text
//! rl_code <SEEN.TXT> <scene> <regname> [from] [length]
//! ```
use anyhow::{Context, Result};
use reallive::scenario::Header;
use reallive::{Archive, Nls};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: rl_code <SEEN.TXT> <scene> <regname> [from] [length]";
    let path = args.first().context(usage)?;
    let scene: i32 = args.get(1).context(usage)?.parse()?;
    let regname = args.get(2).map_or("", String::as_str);
    let hex = |s: &String| usize::from_str_radix(s.trim_start_matches("0x"), 16);
    let from = args.get(3).map(hex).transpose()?.unwrap_or(0);
    let length = args.get(4).map(hex).transpose()?.unwrap_or(0x100);
    let archive = Archive::open(path.as_ref(), regname, Nls::Sjis)?;
    let raw = archive.raw(scene)?;
    let header = Header::parse(&raw)?;
    let mut code = header.decompress(&raw)?;
    if header.uses_xor2
        && let Some(key) = archive.xor2_key()
    {
        key.apply(&mut code);
    }
    let end = (from + length).min(code.len());
    for (row, chunk) in code[from..end].chunks(16).enumerate() {
        let bytes: Vec<String> = chunk.iter().map(|b| format!("{b:02x}")).collect();
        let text: String = chunk
            .iter()
            .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
            .collect();
        println!("{:06x}  {:<48}  {text}", from + row * 16, bytes.join(" "));
    }
    Ok(())
}
