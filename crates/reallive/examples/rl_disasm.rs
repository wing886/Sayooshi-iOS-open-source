//! Decodes every scenario of an archive and prints its elements.
//!
//! ```text
//! rl_disasm <SEEN.TXT> [scene] [--quiet]
//! ```
use anyhow::{Context, Result};
use reallive::bytecode::Element;
use reallive::{Archive, Nls};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .context("usage: rl_disasm <SEEN.TXT> [scene] [--quiet]")?;
    let rest: Vec<String> = args.collect();
    let quiet = rest.iter().any(|arg| arg == "--quiet");
    let only: Option<i32> = rest.iter().find_map(|arg| arg.parse().ok());
    let seen: &std::path::Path = path.as_ref();
    // The second-level key is chosen by `#REGNAME` when a Gameexe.ini is
    // next to the archive (or one directory up).
    let regname = [seen.parent(), seen.parent().and_then(|p| p.parent())]
        .into_iter()
        .flatten()
        .map(|dir| dir.join("Gameexe.ini"))
        .find(|ini| ini.is_file())
        .and_then(|ini| reallive::Gameexe::open(&ini, Nls::Sjis).ok())
        .and_then(|exe| exe.str("REGNAME").map(str::to_owned))
        .unwrap_or_default();
    let archive = Archive::open(seen, &regname, Nls::Sjis)?;
    eprintln!("key: {:?}", archive.key_source);
    let mut failures = 0;
    let numbers: Vec<i32> = archive.numbers().collect();
    for number in numbers {
        if only.is_some_and(|only| only != number) {
            continue;
        }
        match archive.scenario(number) {
            Ok(scenario) => {
                if quiet {
                    continue;
                }
                println!(
                    "== SEEN{number:04} ({} elements)",
                    scenario.script.elements.len()
                );
                for (index, element) in scenario.script.elements.iter().enumerate() {
                    let offset = scenario.script.offsets[index];
                    match element {
                        Element::Textout(raw) => println!(
                            "{index:5} {offset:06x} text {:?}",
                            scenario.nls.decode(&Element::text(raw, scenario.nls))
                        ),
                        other => println!("{index:5} {offset:06x} {other:?}"),
                    }
                }
            }
            Err(error) => {
                failures += 1;
                println!("SEEN{number:04}: {error:#}");
            }
        }
    }
    eprintln!("{failures} failures");
    Ok(())
}
