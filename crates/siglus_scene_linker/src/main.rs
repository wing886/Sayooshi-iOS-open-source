use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use siglus_compiler_common::{RECOVERED_SCENE_KEY, load_key16_from_toml, parse_hex};
use siglus_scene_linker::{
    Definitions, LinkOptions, SceneInput, SourceEncryptionParameters, SourceInput, link,
    parse_inc_declarations,
};

#[derive(Parser)]
#[command(name = "siglus-scene-linker")]
#[command(about = "Link compiled Siglus scene .dat files into Scene.pck")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Link {
        #[arg(required = true)]
        scenes: Vec<PathBuf>,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long)]
        inc: Vec<PathBuf>,
        /// Use original easy-link storage (uncompressed scene chunks).
        #[arg(long)]
        no_compress: bool,
        /// Source directory scanned like the original linker (top-level
        /// Gameexe.ini, 暗号.dat, *.inc, then *.ss).
        #[arg(long, requires = "source_parameters")]
        source_root: Option<PathBuf>,
        /// JSON values recovered from the absent tnm_source_angou.h.
        #[arg(long, requires = "source_root")]
        source_parameters: Option<PathBuf>,
        #[arg(long, conflicts_with = "key_toml")]
        exe_key_hex: Option<String>,
        /// Existing project key.toml (`key` or `key_hex` field).
        #[arg(long, conflicts_with = "exe_key_hex")]
        key_toml: Option<PathBuf>,
        #[arg(long)]
        fixed_scene_key_hex: Option<String>,
    },
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
        Command::Link {
            scenes,
            output,
            inc,
            no_compress,
            source_root,
            source_parameters,
            exe_key_hex,
            key_toml,
            fixed_scene_key_hex,
        } => {
            let definitions = read_definitions(&inc)?;
            let scene_inputs = scenes
                .iter()
                .map(|path| {
                    let name = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
                        anyhow::anyhow!("scene path has no Unicode file stem: {}", path.display())
                    })?;
                    Ok(SceneInput {
                        name: name.to_owned(),
                        data: fs::read(path).with_context(|| format!("read {}", path.display()))?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let fixed_scene_key = fixed_scene_key_hex
                .map(|raw| parse_hex(&raw))
                .transpose()?
                .unwrap_or_else(|| RECOVERED_SCENE_KEY.to_vec());
            let exe_key = if let Some(path) = key_toml {
                load_key16_from_toml(&path)?
            } else {
                exe_key_hex
                    .map(|raw| {
                        let bytes = parse_hex(&raw)?;
                        bytes.try_into().map_err(|v: Vec<u8>| {
                            anyhow::anyhow!("--exe-key-hex needs 16 bytes, got {}", v.len())
                        })
                    })
                    .transpose()?
            };
            let source_encryption = source_parameters
                .as_ref()
                .map(|path| {
                    let data =
                        fs::read(path).with_context(|| format!("read {}", path.display()))?;
                    serde_json::from_slice::<SourceEncryptionParameters>(&data)
                        .with_context(|| format!("parse {}", path.display()))
                })
                .transpose()?;
            let original_sources = source_root
                .as_ref()
                .map(|path| read_original_sources(path))
                .transpose()?
                .unwrap_or_default();
            let compress = !no_compress;
            if compress && source_encryption.is_none() {
                bail!(
                    "compressed output needs --source-root and --source-parameters; use --no-compress for the original easy-link format"
                );
            }
            let bytes = link(
                &scene_inputs,
                &definitions,
                &LinkOptions {
                    compress,
                    fixed_scene_key,
                    exe_key,
                    original_sources,
                    source_encryption,
                },
            )?;
            write_file(&output, &bytes)?;
            eprintln!(
                "wrote {} ({} scenes, {} bytes)",
                output.display(),
                scene_inputs.len(),
                bytes.len()
            );
        }
    }
    Ok(())
}

fn read_original_sources(root: &Path) -> Result<Vec<SourceInput>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root).with_context(|| format!("read {}", root.display()))? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.path());
        }
    }
    let mut ordered = Vec::new();
    for (stem_filter, extension) in [
        (Some("gameexe"), "ini"),
        (Some("暗号"), "dat"),
        (None, "inc"),
        (None, "ss"),
    ] {
        let mut category = files
            .iter()
            .filter(|path| {
                let stem = path
                    .file_stem()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default();
                let ext = path
                    .extension()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default();
                ext.eq_ignore_ascii_case(extension)
                    && stem_filter.is_none_or(|filter| stem.eq_ignore_ascii_case(filter))
            })
            .cloned()
            .collect::<Vec<_>>();
        category.sort_by_key(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_lowercase()
        });
        for path in category {
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(|| anyhow::anyhow!("non-Unicode source name: {}", path.display()))?;
            ordered.push(SourceInput {
                name: name.to_owned(),
                data: fs::read(&path).with_context(|| format!("read {}", path.display()))?,
            });
        }
    }
    Ok(ordered)
}

fn read_definitions(paths: &[PathBuf]) -> Result<Definitions> {
    let forms = built_in_forms();
    let mut all = Definitions::default();
    for path in paths {
        let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
        let text = match std::str::from_utf8(&bytes) {
            Ok(text) => text.to_owned(),
            Err(_) => {
                let (text, _, errors) = encoding_rs::SHIFT_JIS.decode(&bytes);
                if errors {
                    bail!("{} is neither UTF-8 nor valid CP932", path.display());
                }
                text.into_owned()
            }
        };
        let parsed = parse_inc_declarations(&text, &forms)
            .with_context(|| format!("parse {}", path.display()))?;
        all.properties.extend(parsed.properties);
        all.commands.extend(parsed.commands);
    }
    Ok(all)
}

fn built_in_forms() -> HashMap<String, i32> {
    let mut forms = HashMap::new();
    for (name, code) in [
        ("void", 0),
        ("voidlist", 1),
        ("int", 10),
        ("intlist", 11),
        ("intlistlist", 12),
        ("intref", 13),
        ("intlistref", 14),
        ("intevent", 15),
        ("inteventlist", 16),
        ("allevent", 17),
        ("str", 20),
        ("strlist", 21),
        ("strlistlist", 22),
        ("strref", 23),
        ("strlistref", 24),
        ("label", 30),
    ] {
        forms.insert(name.to_owned(), code);
    }
    forms
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}
