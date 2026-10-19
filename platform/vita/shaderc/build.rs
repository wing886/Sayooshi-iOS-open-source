//! Lists the renderer's Cg sources (`*_v.cg` vertex, `*_f.cg` fragment)
//! for the compiler to embed, and links the compiler module stub.
use std::fmt::Write;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../crates/siglus_scene_vm/src/render/vita/shaders")
        .canonicalize()
        .expect("shader directory");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .expect("read shader directory")
        .filter_map(|entry| {
            let name = entry.ok()?.file_name().into_string().ok()?;
            (name.ends_with(".cg") || name.ends_with(".cgh")).then_some(name)
        })
        .collect();
    names.sort();
    let mut sources = String::from("pub const SOURCES: &[(&str, bool, &str)] = &[\n");
    let mut includes = String::from("pub const INCLUDES: &[(&str, &str)] = &[\n");
    for name in names {
        let path = dir.join(&name);
        println!("cargo:rerun-if-changed={}", path.display());
        let path = path.display().to_string();
        if let Some(stem) = name.strip_suffix(".cg") {
            let fragment = stem.ends_with("_f");
            writeln!(sources, "    ({stem:?}, {fragment}, include_str!({path:?})),").unwrap();
        } else {
            writeln!(includes, "    ({name:?}, include_str!({path:?})),").unwrap();
        }
    }
    let out = format!("{sources}];\n{includes}];\n");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::write(out_dir.join("sources.rs"), out).unwrap();

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("vita") {
        for library in ["SceShaccCg_stub", "m"] {
            println!("cargo:rustc-link-lib=static={library}");
        }
    }
}
