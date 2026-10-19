//! End-to-end checks against an installed AVG32 title.  They run only when
//! `AVG32_GAME_ROOT` points at a game directory (e.g. AIR).

use avg32::{Avg32Engine, EngineOptions, Key};

fn engine() -> Option<Avg32Engine> {
    let root = std::env::var_os("AVG32_GAME_ROOT")?;
    let save_dir = std::env::temp_dir().join(format!("avg32-test-{}", std::process::id()));
    // SAFETY: tests in this file only read the variable from `System::new`.
    unsafe { std::env::set_var("AVG32_SAVE_DIR", &save_dir) };
    let options = EngineOptions {
        audio: false,
        virtual_clock: true,
        persist: true,
        ..EngineOptions::default()
    };
    Some(Avg32Engine::open(root, options).expect("open game"))
}

fn run(engine: &mut Avg32Engine, frames: usize, click_every: usize) {
    for frame in 0..frames {
        if click_every != 0 && frame % click_every == 0 {
            engine.key_down(Key::Enter);
        }
        engine.tick();
        engine.advance_clock(17);
    }
}

#[test]
fn save_and_load_resume_at_the_save_point() {
    let Some(mut engine) = engine() else {
        return;
    };
    let seen = std::env::var("AVG32_TEST_SEEN")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(163);
    engine.jump_to_scene(seen);
    run(&mut engine, 1500, 60);
    let saved = (
        engine.scenario().last_save_seen,
        engine.scenario().last_save_pos,
    );
    engine.save(0);
    run(&mut engine, 1, 0);
    assert!(engine.system().slots[0].valid, "slot 0 was not written");
    run(&mut engine, 1500, 30);
    engine.load(0);
    run(&mut engine, 200, 0);
    assert_eq!(engine.scenario().seen, saved.0);
    assert!(engine.running());
    assert!(
        !engine.system().backlog.is_empty(),
        "no message reached the backlog"
    );
    let frame = engine.frame_rgba();
    assert!(
        frame
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[..3] != [0, 0, 0]),
        "screen is black after loading"
    );
}

#[test]
fn installed_game_history_controls_and_separate_title_encoding() {
    let Some(mut engine) = engine() else { return; };
    let sys = engine.system_mut();
    sys.clear_backlog();
    sys.backlog_enabled = true;
    sys.record_backlog(b"first");
    assert_eq!(sys.backlog.len(), 1);
    sys.backlog_enabled = false;
    sys.record_backlog(b"disabled");
    assert_eq!(sys.backlog.len(), 1);
    sys.backlog_view = Some(0);
    sys.clear_backlog();
    assert!(sys.backlog.is_empty());
    assert_eq!(sys.backlog_view, None);
    sys.backlog_enabled = true;
    sys.record_backlog(b"after clear");
    assert_eq!(sys.backlog.len(), 1);
    sys.game.ini.title_nls = Some(avg32::Nls::Gbk);
    let title = avg32::Nls::Gbk.encode("第一天，保健室");
    assert_eq!(sys.decode_title(&title), "第一天，保健室");
    sys.set_window_title(title);
    assert!(sys.window_title.ends_with("第一天，保健室"));
}
