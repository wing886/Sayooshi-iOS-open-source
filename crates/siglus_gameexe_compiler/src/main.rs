use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use siglus_compiler_common::{RECOVERED_GAMEEXE_KEY, load_key16_from_toml, parse_hex};
use siglus_gameexe_compiler::{CompileOptions, compile, derive_exe_key_from_cp932};

#[derive(Parser)]
#[command(name = "siglus-gameexe-compiler")]
#[command(about = "Compile Gameexe.ini into the original Siglus Gameexe.dat format")]
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
        #[arg(long, value_enum, default_value_t = SourceEncoding::ShiftJis)]
        encoding: SourceEncoding,
        /// CP932 text used by the original executable-key derivation loop.
        #[arg(long, conflicts_with_all = ["exe_key_hex", "key_toml"])]
        exe_key_string: Option<String>,
        /// Already-derived 16-byte executable XOR key.
        #[arg(long, conflicts_with = "key_toml")]
        exe_key_hex: Option<String>,
        /// Existing project key.toml (`key` or `key_hex` field).
        #[arg(long, conflicts_with = "exe_key_hex")]
        key_toml: Option<PathBuf>,
        /// Override the missing TNM_GAMEEXE_DAT_ANGOU_CODE constant.
        #[arg(long, conflicts_with = "fixed_key_file")]
        fixed_key_hex: Option<String>,
        /// Read the fixed XOR key as raw bytes.
        #[arg(long)]
        fixed_key_file: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum SourceEncoding {
    ShiftJis,
    Utf8,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let Cli { command } = Cli::parse();
    match command {
        Command::Compile {
            input,
            output,
            encoding,
            exe_key_string,
            exe_key_hex,
            key_toml,
            fixed_key_hex,
            fixed_key_file,
        } => {
            let bytes = fs::read(&input).with_context(|| format!("read {}", input.display()))?;
            let source = decode_source(&bytes, encoding)?;
            let fixed_key = if let Some(raw) = fixed_key_hex {
                parse_hex(&raw).context("parse --fixed-key-hex")?
            } else if let Some(path) = fixed_key_file {
                fs::read(&path).with_context(|| format!("read {}", path.display()))?
            } else {
                RECOVERED_GAMEEXE_KEY.to_vec()
            };
            let exe_key = if let Some(text) = exe_key_string {
                Some(derive_exe_key_from_cp932(&text)?)
            } else if let Some(raw) = exe_key_hex {
                let bytes = parse_hex(&raw).context("parse --exe-key-hex")?;
                Some(bytes.try_into().map_err(|v: Vec<u8>| {
                    anyhow::anyhow!("--exe-key-hex needs 16 bytes, got {}", v.len())
                })?)
            } else if let Some(path) = key_toml {
                load_key16_from_toml(&path)?
            } else {
                Some([0u8; 16])
            };
            let output_bytes = compile(&source, &CompileOptions { fixed_key, exe_key })?;
            write_file(&output, &output_bytes)?;
            eprintln!("wrote {} ({} bytes)", output.display(), output_bytes.len());
        }
    }
    Ok(())
}

fn decode_source(bytes: &[u8], encoding: SourceEncoding) -> Result<String> {
    match encoding {
        SourceEncoding::Utf8 => {
            let bytes = bytes.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(bytes);
            Ok(std::str::from_utf8(bytes)?.to_owned())
        }
        SourceEncoding::ShiftJis => {
            let (text, _, errors) = encoding_rs::SHIFT_JIS.decode(bytes);
            if errors {
                bail!("input contains invalid Windows CP932/Shift-JIS bytes");
            }
            Ok(text.into_owned())
        }
    }
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}
