//! `game_launcher_run <game-folder> [nls]`: detects the engine and runs the
//! game in a window.  `game_launcher_run --probe <folder>...` prints the
//! launcher metadata instead.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--probe") {
        for root in &args[1..] {
            let found = game_launcher::scan(std::path::Path::new(root), 3);
            for game in found {
                match game_launcher::probe(&game, None, true) {
                    Ok(info) => {
                        let cover =
                            info.write_cover(&std::env::temp_dir().join("game-launcher-covers"));
                        println!(
                            "{}",
                            info.to_json(game_launcher::json::Value::opt_string(
                                cover.map(|path| path.to_string_lossy().into_owned())
                            ))
                            .to_json()
                        );
                    }
                    Err(error) => eprintln!("{}: {error}", game.display()),
                }
            }
        }
        return;
    }
    let Some(root) = args.first() else {
        eprintln!("usage: game_launcher_run <game-folder> [nls] | --probe <folder>...");
        std::process::exit(2);
    };
    let nls = args.get(1).and_then(|text| game_launcher::Nls::parse(text));
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    std::process::exit(game_launcher::desktop::run(std::path::Path::new(root), nls));
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        let _ = (root, nls);
        eprintln!("no desktop runner on this platform");
        std::process::exit(1);
    }
}
