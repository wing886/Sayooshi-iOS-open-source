use std::path::PathBuf;

use anyhow::{Context, Result};
use uk2::{MesProgram, disassemble_reachable};

fn main() -> Result<()> {
    let path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("usage: uk2_disasm <file.MES>"))?;
    let program = MesProgram::from_path(&path)
        .with_context(|| format!("failed to open {}", path.display()))?;
    for (offset, instruction) in disassemble_reachable(&program)? {
        println!("{offset:04X}: {:?}", instruction.kind);
    }
    Ok(())
}
