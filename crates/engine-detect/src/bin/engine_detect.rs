//! `engine_detect <game-directory>...`: prints the engine each game uses.

use std::process::ExitCode;

fn main() -> ExitCode {
    let roots: Vec<String> = std::env::args().skip(1).collect();
    if roots.is_empty() {
        eprintln!("usage: engine_detect <game-directory>...");
        return ExitCode::from(2);
    }
    let mut status = ExitCode::SUCCESS;
    for root in roots {
        match engine_detect::detect_game_root(&root) {
            Ok(layout) => {
                println!("{}: {}", layout.root.display(), layout.kind);
                for reason in &layout.evidence {
                    println!("  - {reason}");
                }
            }
            Err(error) => {
                eprintln!("{root}: {error}");
                status = ExitCode::FAILURE;
            }
        }
    }
    status
}
