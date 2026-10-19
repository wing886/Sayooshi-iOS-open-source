//! Engine behaviour on small hand-assembled scenarios.

use std::path::{Path, PathBuf};

use avg32::{Avg32Engine, EngineOptions, Key};

/// AVG32 compact number (`0x80` marks a variable reference).
fn num(value: i32) -> Vec<u8> {
    encode(value as u32, false)
}

fn var(index: i32) -> Vec<u8> {
    encode(index as u32, true)
}

fn encode(value: u32, variable: bool) -> Vec<u8> {
    let mut length = 1;
    while length < 5 && (value >> 4) >> (8 * (length - 1)) != 0 {
        length += 1;
    }
    let mut out =
        vec![((length as u8) << 4) | (value & 0x0f) as u8 | if variable { 0x80 } else { 0 }];
    for byte in 0..length - 1 {
        out.push(((value >> 4) >> (8 * byte)) as u8);
    }
    out
}

struct Menu {
    id: u8,
    title: &'static str,
    subs: Vec<(u8, &'static str)>,
}

/// A `TPC32` scene; `menus` adds a scenario-menu header whose blocks follow
/// `code` (which must then start with the `0x00` menu point).
fn scene(code: &[u8], menus: &[Menu], blocks: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = b"TPC32".to_vec();
    bytes.extend_from_slice(&[0; 0x13]);
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    let mut fixed = [0u8; 0x30];
    fixed[0x08..0x0c].copy_from_slice(&1u32.to_le_bytes());
    fixed[0x0c..0x10].copy_from_slice(&(menus.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&fixed);
    let mut strings: Vec<&str> = Vec::new();
    for menu in menus {
        bytes.extend_from_slice(&[menu.id, menu.subs.len() as u8]);
        strings.push(menu.title);
        for (id, title) in &menu.subs {
            bytes.extend_from_slice(&[*id, 1]);
            strings.push(title);
            bytes.push(0); // one repetition, no flag conditions
        }
    }
    for text in strings {
        bytes.push(text.len() as u8 + 1);
        bytes.extend_from_slice(text.as_bytes());
        bytes.push(0);
    }
    let mut trailer = [0u8; 0x13];
    trailer[0x0f..0x13].copy_from_slice(&1u32.to_le_bytes());
    bytes.extend_from_slice(&trailer);
    bytes.extend_from_slice(code);
    for block in blocks {
        bytes.extend_from_slice(&(block.len() as u32).to_le_bytes());
        bytes.extend_from_slice(block);
    }
    bytes
}

fn game(scenes: &[(i32, Vec<u8>)]) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "avg32-synthetic-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    std::fs::create_dir_all(root.join("DAT")).unwrap();
    std::fs::create_dir_all(root.join("PDT")).unwrap();
    std::fs::write(
        root.join("GAMEEXE.INI"),
        "#SEEN_START=001\n#DIRC.TXT=\"DAT\" =N\n#DIRC.PDT=\"PDT\" =N\n#MESSAGE_SIZE=020,003\n\
#MSG_MOJI_SIZE=012,024\n#WINDOW_MSG_POS=020,340\n#SEL_BLINK_COUNT=000\n#SEL_BLINK_SPEED=000\n",
    )
    .unwrap();
    for (number, bytes) in scenes {
        std::fs::write(root.join("DAT").join(format!("SEEN{number:03}.TXT")), bytes).unwrap();
    }
    root
}

fn open(root: &Path) -> Avg32Engine {
    Avg32Engine::open(
        root,
        EngineOptions {
            audio: false,
            virtual_clock: true,
            persist: false,
            ..EngineOptions::default()
        },
    )
    .unwrap()
}

fn frames(engine: &mut Avg32Engine, count: usize, input: impl Fn(usize, &mut Avg32Engine)) {
    for frame in 0..count {
        input(frame, engine);
        engine.tick();
        engine.advance_clock(20);
    }
}

fn value(engine: &Avg32Engine, index: i32) -> i32 {
    engine.system().flags.value(index)
}

fn end() -> Vec<u8> {
    // `0x60:20`: quit.
    vec![0x60, 0x20]
}

#[test]
fn arithmetic_conditions_and_strings() {
    let mut code = Vec::new();
    code.extend([0x3b].iter().chain(num(1).iter()).chain(num(40).iter())); // V1 = 40
    code.extend([0x3c].iter().chain(num(1).iter()).chain(num(2).iter())); // V1 += 2
    code.extend([0x3b].iter().chain(num(2).iter()).chain(num(1).iter())); // V2 = 1
    code.extend([0x49].iter().chain(num(3).iter()).chain(var(2).iter())); // V3 = V[V2] (indirect)
    // if (V1 == 42) skip the next assignment
    code.push(0x15);
    code.extend(
        [0x28, 0x3b]
            .iter()
            .chain(num(1).iter())
            .chain(num(42).iter())
            .chain([0x29].iter()),
    );
    let jump_at = code.len();
    code.extend_from_slice(&0u32.to_le_bytes());
    code.extend([0x3b].iter().chain(num(4).iter()).chain(num(99).iter())); // V4 = 99 (true branch)
    let target = code.len() as u32;
    code[jump_at..jump_at + 4].copy_from_slice(&target.to_le_bytes());
    // Str[0] = "12"; V5 = atoi(Str[0]); V6 = strlen(Str[0])
    code.extend(
        [0x59, 0x01]
            .iter()
            .chain(num(0).iter())
            .chain(b"12\0".iter()),
    );
    code.extend(
        [0x59, 0x08]
            .iter()
            .chain(num(0).iter())
            .chain(num(5).iter()),
    );
    code.extend(
        [0x59, 0x02]
            .iter()
            .chain(num(6).iter())
            .chain(num(0).iter()),
    );
    // V7 = 100 * 3 / 4 via percentage command 0x5f:10 (3 of 4 = 75)
    code.extend(
        [0x5f, 0x10]
            .iter()
            .chain(num(7).iter())
            .chain(num(3).iter())
            .chain(num(4).iter()),
    );
    code.extend(end());
    let root = game(&[(1, scene(&code, &[], &[]))]);
    let mut engine = open(&root);
    frames(&mut engine, 10, |_, _| {});
    assert_eq!(value(&engine, 1), 42);
    assert_eq!(value(&engine, 3), 42, "indirect index goes through V2");
    assert_eq!(value(&engine, 4), 99, "true condition falls through");
    assert_eq!(value(&engine, 5), 12);
    assert_eq!(value(&engine, 6), 2);
    assert_eq!(value(&engine, 7), 75);
    assert!(!engine.running());
}

#[test]
fn calls_and_returns_share_one_stack_across_scenes() {
    // SEEN001: call SEEN002; V1 += 1; local call; quit.
    let mut first = Vec::new();
    first.extend([0x16, 0x02].iter().chain(num(2).iter()));
    first.extend([0x3c].iter().chain(num(1).iter()).chain(num(1).iter()));
    first.push(0x1b);
    let call_at = first.len();
    first.extend_from_slice(&0u32.to_le_bytes());
    first.extend(end());
    let subroutine = first.len() as u32;
    first[call_at..call_at + 4].copy_from_slice(&subroutine.to_le_bytes());
    first.extend([0x3c].iter().chain(num(2).iter()).chain(num(5).iter()));
    first.extend([0x20, 0x01]);
    // SEEN002: V1 = 10; return to caller scene.
    let mut second = Vec::new();
    second.extend([0x3b].iter().chain(num(1).iter()).chain(num(10).iter()));
    second.extend([0x20, 0x02]);
    let root = game(&[(1, scene(&first, &[], &[])), (2, scene(&second, &[], &[]))]);
    let mut engine = open(&root);
    frames(&mut engine, 10, |_, _| {});
    assert_eq!(value(&engine, 1), 11);
    assert_eq!(value(&engine, 2), 5);
    assert!(engine.system().flags.stack().is_empty());
    assert!(!engine.running());
}

#[test]
fn choices_number_hidden_items_and_honour_colours() {
    // 0x58:02 V10 { "A", (V1==1)"B" (hidden), "C" (disabled, colour 2), "D" }
    let mut code = Vec::new();
    code.extend([0x58, 0x02].iter().chain(num(10).iter()));
    code.extend([0x22, 0x00]);
    code.extend([0xff].iter().chain(b"A\0".iter()));
    code.push(0x00);
    code.extend(
        [0x28, 0x3b]
            .iter()
            .chain(num(1).iter())
            .chain(num(1).iter())
            .chain([0x29].iter()),
    );
    code.extend([0xff].iter().chain(b"B\0".iter()));
    code.push(0x00);
    // ((V0 == 0) $58 $22 colour): shown in colour 2 but not selectable.
    code.extend(
        [0x28, 0x28, 0x3b]
            .iter()
            .chain(num(0).iter())
            .chain(num(0).iter())
            .chain([0x29, 0x58, 0x22].iter())
            .chain(num(2).iter())
            .chain([0x29].iter()),
    );
    code.extend([0xff].iter().chain(b"C\0".iter()));
    code.push(0x00);
    code.extend([0xff].iter().chain(b"D\0".iter()));
    code.push(0x00);
    code.push(0x23);
    code.extend(end());
    let root = game(&[(1, scene(&code, &[], &[]))]);
    let mut engine = open(&root);
    // Down x2 skips the disabled "C" and lands on "D"; Enter confirms.
    frames(&mut engine, 60, |frame, engine| match frame {
        5 | 8 => engine.key_down(Key::Down),
        12 => engine.key_down(Key::Enter),
        _ => {}
    });
    assert_eq!(
        value(&engine, 10),
        4,
        "D is the fourth item including hidden B"
    );
}

#[test]
fn scenario_menu_runs_the_chosen_block_and_returns() {
    let block_a = {
        let mut block = Vec::new();
        block.extend([0x3b].iter().chain(num(1).iter()).chain(num(11).iter()));
        block.push(0x00);
        block
    };
    let block_b = {
        let mut block = Vec::new();
        block.extend([0x3b].iter().chain(num(1).iter()).chain(num(22).iter()));
        block.push(0x00);
        block
    };
    let menus = [
        Menu {
            id: 1,
            title: "first",
            subs: vec![(1, "a")],
        },
        Menu {
            id: 1,
            title: "second",
            subs: vec![(1, "b")],
        },
    ];
    let root = game(&[(1, scene(&[0x00], &menus, &[block_a, block_b]))]);
    let mut engine = open(&root);
    frames(&mut engine, 40, |frame, engine| match frame {
        3 | 5 => engine.key_down(Key::Down),
        8 => engine.key_down(Key::Enter),
        _ => {}
    });
    assert_eq!(value(&engine, 1), 22, "second chapter's block ran");
}
