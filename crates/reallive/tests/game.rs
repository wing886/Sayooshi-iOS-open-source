//! Tests against an installed game (set `REALLIVE_TEST_GAME` to its
//! directory; Little Busters! was used). Skipped when unset.

use reallive::engine::{Engine, EngineOptions};
use reallive::input::{Button, InputEvent};
use reallive::surface::Surface;

fn game() -> Option<std::path::PathBuf> {
    std::env::var_os("REALLIVE_TEST_GAME").map(Into::into)
}

fn click(engine: &mut Engine) {
    engine.input(InputEvent::Press(Button::Left));
    engine.tick(16);
    engine.input(InputEvent::Release(Button::Left));
    engine.tick(16);
}

/// Runs until the game waits for the player, clicking through `pages`
/// pages first.
fn advance(engine: &mut Engine, mut pages: usize, limit: usize) {
    for _ in 0..limit {
        if engine.waiting_for_input() {
            if pages == 0 {
                return;
            }
            pages -= 1;
            click(engine);
        } else {
            engine.tick(16);
        }
    }
    panic!("the game did not wait for input");
}

fn difference(a: &Surface, b: &Surface) -> f64 {
    let differing = a
        .rgba
        .chunks(4)
        .zip(b.rgba.chunks(4))
        .filter(|(p, q)| p[..3].iter().zip(&q[..3]).any(|(x, y)| x.abs_diff(*y) > 8))
        .count();
    differing as f64 / (a.rgba.len() / 4) as f64
}

#[test]
fn save_and_load_restore_the_scene() {
    let Some(root) = game() else {
        return;
    };
    let mut options = EngineOptions::headless(root);
    options.fonts = true;
    let mut engine = Engine::open(options).unwrap();
    // Title screen: "New Game".
    for _ in 0..6000 {
        engine.tick(16);
    }
    engine.machine.sys.input.mouse = (670, 479);
    click(&mut engine);
    advance(&mut engine, 5, 20_000);
    let saved_scene = engine.machine.scene_number();
    let before = engine.render();
    reallive::save::save_slot(&mut engine.machine, 0).unwrap();

    advance(&mut engine, 20, 40_000);
    reallive::save::load_slot(&mut engine.machine, 0).unwrap();
    advance(&mut engine, 0, 20_000);
    let after = engine.render();
    assert_eq!(engine.machine.scene_number(), saved_scene);
    let diff = difference(&before, &after);
    if let Some(dir) = std::env::var_os("REALLIVE_TEST_SHOTS") {
        let dir = std::path::PathBuf::from(dir);
        for (name, frame) in [("before.png", &before), ("after.png", &after)] {
            image::save_buffer(
                dir.join(name),
                &frame.rgba,
                frame.width as u32,
                frame.height as u32,
                image::ColorType::Rgba8,
            )
            .unwrap();
        }
    }
    assert!(
        diff < 0.002,
        "{:.1}% of the screen differs after loading",
        diff * 100.0
    );
    assert!(
        engine.machine.diagnostics.errors.is_empty(),
        "{:?}",
        engine.machine.diagnostics.errors
    );
}
