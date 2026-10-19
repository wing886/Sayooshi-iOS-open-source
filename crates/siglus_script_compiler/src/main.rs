use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use encoding_rs::SHIFT_JIS;
use siglus_script_compiler::{CompileOptions, compile};

#[derive(Parser)]
#[command(
    name = "siglus-script-compiler",
    about = "Compile Siglus scene source into original .dat scene bytecode"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Compile {
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long, value_enum, default_value_t = Encoding::ShiftJis)]
        encoding: Encoding,
        #[arg(long = "inc")]
        inc_files: Vec<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Encoding {
    ShiftJis,
    Utf8,
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();
    match command {
        Command::Compile {
            input,
            output,
            encoding,
            inc_files,
        } => {
            let source = decode(
                &fs::read(&input).with_context(|| format!("read {}", input.display()))?,
                encoding,
            )?;
            let mut options = CompileOptions::default();
            for path in inc_files {
                let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
                options.global_inc.push(decode(&bytes, encoding)?);
            }
            let data = compile(&source, &options)
                .with_context(|| format!("compile {}", input.display()))?;
            fs::write(&output, data).with_context(|| format!("write {}", output.display()))?;
        }
    }
    Ok(())
}

fn decode(bytes: &[u8], encoding: Encoding) -> Result<String> {
    match encoding {
        Encoding::Utf8 => Ok(std::str::from_utf8(bytes)?.to_owned()),
        Encoding::ShiftJis => {
            let (text, _, errors) = SHIFT_JIS.decode(bytes);
            if errors {
                bail!("input is not valid Shift-JIS/CP932");
            }
            Ok(text.into_owned())
        }
    }
}
