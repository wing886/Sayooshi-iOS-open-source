//! Runs single functions of the DT00 translation from a given memory
//! state, for comparing with the original DLL.
//!
//! ```text
//! rl_dt00_check <dt00.dll> <job> [--dump N]
//! ```
//!
//! The job file holds the memory to start from (`u32` length, then the
//! bytes from `.text` up) and a list of calls (`u32` count, then per call
//! `u32` address, `u32` argument count and the arguments). Every call
//! starts from that memory; for each one a line is printed: the result,
//! then `address=value` for every 32-bit word that changed outside the
//! translation's stack.
//! `--dump N` writes the memory after call `N` to `dump.bin`.
use anyhow::{Context, Result, bail};
use reallive::dt00::Dt00;

const STACK: std::ops::Range<u32> = 0x1009_0000..0x100B_0000;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: rl_dt00_check <dt00.dll> <job> [--dump N]";
    let dll = std::fs::read(args.first().context(usage)?)?;
    let job = std::fs::read(args.get(1).context(usage)?)?;
    let dump: Option<usize> = args
        .iter()
        .position(|a| a == "--dump")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok());
    let mut at = 0;
    let mut word = || -> Result<u32> {
        let w = job.get(at..at + 4).context("truncated job")?;
        at += 4;
        Ok(u32::from_le_bytes(w.try_into()?))
    };
    let len = word()? as usize;
    let base_at = 4;
    let mut calls = Vec::new();
    {
        // skip the memory
        let mut pos = base_at + len;
        let mut next = || -> Result<u32> {
            let w = job.get(pos..pos + 4).context("truncated job")?;
            pos += 4;
            Ok(u32::from_le_bytes(w.try_into()?))
        };
        let n = next()?;
        for _ in 0..n {
            let address = next()?;
            let count = next()?;
            let args: Vec<u32> = (0..count).map(|_| next()).collect::<Result<_>>()?;
            calls.push((address, args));
        }
    }
    let base = &job[base_at..base_at + len];
    let mut dt = Dt00::load(&dll, 0)?;
    for (i, (address, args)) in calls.iter().enumerate() {
        let (start, mem) = dt.memory_mut();
        if mem.len() != base.len() {
            bail!("memory size {} != job {}", mem.len(), base.len());
        }
        mem.copy_from_slice(base);
        dt.set_rand(0);
        let result = dt
            .call_by_address(*address, args)
            .with_context(|| format!("no function at {address:#x}"))?;
        let (_, mem) = dt.memory_mut();
        let mut line = result.to_string();
        for (offset, (new, old)) in mem
            .as_chunks::<4>()
            .0
            .iter()
            .zip(base.as_chunks::<4>().0)
            .enumerate()
        {
            let address = start + 4 * offset as u32;
            if new != old && !STACK.contains(&address) {
                let v = u32::from_le_bytes(*new);
                line.push_str(&format!(" {address:x}={v:x}"));
            }
        }
        println!("{line}");
        if dump == Some(i) {
            std::fs::write("dump.bin", &*mem)?;
        }
    }
    Ok(())
}
