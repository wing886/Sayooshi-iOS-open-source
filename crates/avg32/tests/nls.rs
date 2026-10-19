//! Non-Shift-JIS scenarios. The encoding is process-wide, so everything
//! runs in one test of its own binary.

use avg32::nls::{self, Nls};
use avg32::{Avg32Engine, EngineOptions};

fn scene(code: &[u8]) -> Vec<u8> {
    let mut bytes = b"TPC32".to_vec();
    bytes.extend_from_slice(&[0; 0x13]);
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    let mut fixed = [0u8; 0x30];
    fixed[0x08..0x0c].copy_from_slice(&1u32.to_le_bytes());
    bytes.extend_from_slice(&fixed);
    let mut trailer = [0u8; 0x13];
    trailer[0x0f..0x13].copy_from_slice(&1u32.to_le_bytes());
    bytes.extend_from_slice(&trailer);
    bytes.extend_from_slice(code);
    bytes
}

#[test]
fn gbk_text_layout_and_sjis_file_name_fallback() {
    let gbk = |text: &str| Nls::Gbk.encode(text);

    // A Shift-JIS file name read back as GBK is found through the fallback.
    nls::set(Nls::Gbk);
    let root = std::env::temp_dir().join(format!("avg32-nls-{}", std::process::id()));
    std::fs::create_dir_all(root.join("DAT")).unwrap();
    std::fs::write(root.join("背景.PDT"), b"").unwrap();
    let as_gbk = Nls::Gbk.decode(&Nls::Sjis.encode("背景"));
    assert_ne!(as_gbk, "背景");
    assert_eq!(nls::sjis_fallback(&as_gbk).as_deref(), Some("背景"));
    assert!(avg32::resource::find_component(&root, &format!("{as_gbk}.PDT")).is_some());

    // Engine-generated full-width digits follow the encoding.
    assert_eq!(avg32::scenario::han_to_zen(b"12"), gbk("１２"));

    // A GBK message: 【name】 and brackets are recognised, every character
    // takes one full-width cell.
    let mut code = vec![0xff];
    code.extend(gbk("【小明】「你好，世界。」"));
    code.push(0);
    code.push(0x01);
    code.extend([0x60, 0x20]);
    std::fs::write(
        root.join("GAMEEXE.INI"),
        gbk(
            "#SEEN_START=001\n#DIRC.TXT=\"DAT\" =N\n#MESSAGE_SIZE=020,003\n\
#MSG_MOJI_SIZE=012,024\n#WINDOW_MSG_POS=020,340\n#CAPTION=\"中文测试\"\n",
        ),
    )
    .unwrap();
    std::fs::write(root.join("DAT").join("SEEN001.TXT"), scene(&code)).unwrap();
    let mut engine = Avg32Engine::open(
        &root,
        EngineOptions {
            audio: false,
            virtual_clock: true,
            persist: false,
            nls: Nls::Gbk,
        },
    )
    .unwrap();
    for _ in 0..200 {
        engine.tick();
        engine.advance_clock(20);
    }
    let mes = &engine.system().mes;
    let expected = gbk("「你好，世界。」");
    assert!(
        mes.buf
            .windows(expected.len())
            .any(|window| window == expected),
        "message buffer holds the GBK text"
    );
    // 【 and 】 are layout marks, not drawn: 2 + 8 characters of two cells.
    assert_eq!(mes.cur_x, 20);
    std::fs::remove_dir_all(&root).ok();
}
