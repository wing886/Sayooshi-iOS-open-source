//! Runs every opcode of a case list through the interpreter and reports
//! how each one fares.
//!
//! ```text
//! rl_opcodes <game dir> <cases.tsv>
//! rl_opcodes <game dir> --game
//! ```
//!
//! With `--game`, the cases are the game's own: every distinct opcode in
//! its scenarios, run once with the parameters of its first occurrence
//! (from inside that scenario); the output adds how often it occurs.
//!
//! Each line of the case list is `modtype module opcode overload name
//! argc params [comment]`, tab separated; `params` is a space-separated
//! list of synthetic parameters: `I` an integer, `V` an integer variable,
//! `S` a string, `T` a string variable, `C(...)` a tuple and
//! `A<tag>(...)` a tagged parameter. (A list can be generated from
//! RLdev's `reallive.kfn`.) Every case runs on a freshly opened engine;
//! the output is one tab-separated line per case: the opcode, the name,
//! and `ok`, `blocking` (it started a long operation), `unimplemented`,
//! `error` or `panic` with details.
use std::panic::{AssertUnwindSafe, catch_unwind};

use anyhow::{Context, Result, bail};
use reallive::bytecode::{Command, CommandKind, Opcode, Param};
use reallive::engine::{Engine, EngineOptions};
use reallive::expr::Parser;

fn int(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(b"$\xff");
    out.extend_from_slice(&value.to_le_bytes());
}

/// Encodes one synthetic parameter as bytecode.
fn encode(spec: &[u8], at: &mut usize, out: &mut Vec<u8>) -> Result<()> {
    while spec.get(*at) == Some(&b' ') {
        *at += 1;
    }
    let Some(&kind) = spec.get(*at) else {
        bail!("empty parameter");
    };
    *at += 1;
    match kind {
        b'I' => int(out, 0),
        b'V' => {
            out.extend_from_slice(b"$\x00[");
            int(out, 0);
            out.push(b']');
        }
        b'S' => out.extend_from_slice(b"\"zz_audit\""),
        b'T' => {
            out.extend_from_slice(b"$\x12[");
            int(out, 0);
            out.push(b']');
        }
        b'C' | b'A' => {
            let mut tag = 0u8;
            if kind == b'A' {
                let start = *at;
                while spec.get(*at).is_some_and(u8::is_ascii_digit) {
                    *at += 1;
                }
                tag = std::str::from_utf8(&spec[start..*at])?.parse()?;
            }
            if spec.get(*at) != Some(&b'(') {
                bail!("expected ( in {}", String::from_utf8_lossy(spec));
            }
            *at += 1;
            let mut inner = Vec::new();
            let mut count = 0;
            loop {
                while spec.get(*at) == Some(&b' ') {
                    *at += 1;
                }
                if spec.get(*at) == Some(&b')') {
                    *at += 1;
                    break;
                }
                encode(spec, at, &mut inner)?;
                count += 1;
            }
            if kind == b'A' {
                out.push(b'a');
                out.push(tag);
                if count == 1 {
                    out.extend_from_slice(&inner);
                    return Ok(());
                }
            }
            out.push(b'(');
            out.extend_from_slice(&inner);
            out.push(b')');
        }
        other => bail!("unknown parameter kind {}", other as char),
    }
    Ok(())
}

/// Runs one command on a fresh engine; `(status, detail)`.
fn experiment(root: &str, scene: Option<i32>, command: &Command) -> Result<(&'static str, String)> {
    let mut engine = Engine::open(EngineOptions::headless(root))?;
    if let Some(scene) = scene {
        let _ = engine.machine.farcall(scene, 0);
    }
    let machine = &mut engine.machine;
    let before = machine.diagnostics.unimplemented.clone();
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        reallive::modules::dispatch(machine, command)
    }));
    Ok(match outcome {
        Err(panic) => {
            let message = panic
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            ("panic", message)
        }
        Ok(Err(error)) => ("error", format!("{error:#}")),
        Ok(Ok(_)) => {
            let notes: Vec<String> = machine
                .diagnostics
                .unimplemented
                .iter()
                .filter(|(key, count)| before.get(*key) != Some(count))
                .map(|(key, _)| key.clone())
                .collect();
            if !notes.is_empty() {
                ("unimplemented", notes.join("; "))
            } else if let Some(long_op) = machine.current_long_op() {
                ("blocking", long_op.name().to_owned())
            } else {
                ("ok", String::new())
            }
        }
    })
}

fn game_ops(root: &str) -> Result<()> {
    use std::collections::BTreeMap;
    let engine = Engine::open(EngineOptions::headless(root))?;
    let archive = engine.machine.archive.clone();
    let mut first: BTreeMap<(u8, u8, u16, u8), (i32, Command, usize)> = BTreeMap::new();
    for number in archive.numbers().collect::<Vec<_>>() {
        let Ok(scenario) = archive.scenario(number) else {
            continue;
        };
        for element in &scenario.script.elements {
            if let reallive::bytecode::Element::Command(command) = element
                && matches!(command.kind, CommandKind::Plain)
            {
                let op = command.op;
                let key = (op.modtype, op.module, op.opcode, op.overload);
                first
                    .entry(key)
                    .and_modify(|e| e.2 += 1)
                    .or_insert((number, command.clone(), 1));
            }
        }
    }
    for (scene, command, count) in first.values() {
        let (status, detail) = experiment(root, Some(*scene), command)?;
        let name = reallive::opcodes::name(command.op).unwrap_or("-");
        println!(
            "{}	{name}	{count}	SEEN{scene:04}	{status}	{detail}",
            command.op
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: rl_opcodes <game dir> <cases.tsv | --game>";
    let root = args.first().context(usage)?;
    std::panic::set_hook(Box::new(|_| {}));
    if args.get(1).map(String::as_str) == Some("--game") {
        return game_ops(root);
    }
    let cases = std::fs::read_to_string(args.get(1).context(usage)?)?;
    for line in cases.lines().filter(|l| !l.trim().is_empty()) {
        let fields: Vec<&str> = line.split('\t').collect();
        let [modtype, module, opcode, overload, name, argc, params, ..] = fields[..] else {
            continue;
        };
        let op = Opcode::new(
            modtype.parse()?,
            module.parse()?,
            opcode.parse()?,
            overload.parse()?,
        );
        let nls = reallive::Nls::Sjis;
        let mut command = Command {
            op,
            argc: argc.parse()?,
            params: Vec::new(),
            kind: CommandKind::Plain,
        };
        let spec = params.as_bytes();
        let mut at = 0;
        while spec[at..].iter().any(|b| *b != b' ') {
            let mut raw = Vec::new();
            encode(spec, &mut at, &mut raw)?;
            let value = Parser::new(&raw, nls).data()?;
            command.params.push(Param { raw, value });
        }
        let (status, detail) = experiment(root, None, &command)?;
        println!("{op}\t{name}\t{params}\t{status}\t{detail}");
    }
    Ok(())
}
