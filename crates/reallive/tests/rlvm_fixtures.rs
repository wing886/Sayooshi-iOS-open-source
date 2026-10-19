//! Scenario-level tests against the compiled fixtures of rlvm's test suite
//! (`rlvm/test`). Set `RLVM_TEST_DATA` to that directory; the tests are
//! skipped when it is not set.

use std::path::PathBuf;
use std::rc::Rc;

use reallive::expr::bank;
use reallive::machine::StrTarget;
use reallive::memory::IntRef;
use reallive::system::System;
use reallive::{Archive, Gameexe, Machine, Nls};

fn fixture(name: &str) -> Option<PathBuf> {
    let root = PathBuf::from(std::env::var_os("RLVM_TEST_DATA")?);
    Some(root.join(name))
}

fn machine(name: &str, setup: impl FnOnce(&mut Machine)) -> Option<Machine> {
    let path = fixture(name)?;
    let archive = Rc::new(Archive::open(&path, "", Nls::Sjis).expect("open fixture"));
    let mut machine =
        Machine::new(archive, Rc::new(Gameexe::default()), System::default()).expect("machine");
    machine.halt_on_error = true;
    setup(&mut machine);
    for _ in 0..10_000 {
        if machine.halted {
            break;
        }
        machine.run();
    }
    assert!(machine.halted, "{name} did not halt");
    assert!(
        machine.diagnostics.errors.is_empty(),
        "{name}: {:?}",
        machine.diagnostics.errors
    );
    Some(machine)
}

fn int(machine: &Machine, bank: &str, index: i32) -> i32 {
    machine
        .read_int(IntRef::named(bank, index).unwrap())
        .unwrap()
}

fn set(machine: &mut Machine, bank: &str, index: i32, value: i32) {
    machine
        .write_int(IntRef::named(bank, index).unwrap(), value)
        .unwrap();
}

fn str_s(machine: &Machine, index: i32) -> String {
    machine
        .read_string(StrTarget {
            bank: bank::STR_S,
            index,
        })
        .unwrap()
}

fn sjis(bytes: &[u8]) -> String {
    Nls::Sjis.decode(bytes)
}

macro_rules! run {
    ($name:expr) => {
        run!($name, |_| {})
    };
    ($name:expr, $setup:expr) => {
        match machine($name, $setup) {
            Some(machine) => machine,
            None => return,
        }
    };
}

#[test]
fn str_module() {
    let m = run!("Module_Str_SEEN/strcpy_0.TXT");
    assert_eq!(str_s(&m, 0), "valid");
    let m = run!("Module_Str_SEEN/strcpy_1.TXT");
    assert_eq!(str_s(&m, 0), "va");
    let m = run!("Module_Str_SEEN/strclear_0.TXT");
    assert_eq!((str_s(&m, 0), str_s(&m, 1)), ("".into(), "valid".into()));
    let m = run!("Module_Str_SEEN/strclear_1.TXT");
    assert_eq!(
        (str_s(&m, 0), str_s(&m, 1), str_s(&m, 2)),
        ("".into(), "".into(), "valid".into())
    );
    let m = run!("Module_Str_SEEN/strcat_0.TXT");
    assert_eq!(str_s(&m, 0), "valid");
    let m = run!("Module_Str_SEEN/strlen_0.TXT");
    assert_eq!(int(&m, "A", 0), 5);
    let m = run!("Module_Str_SEEN/strcmp_0.TXT");
    assert_eq!(
        (int(&m, "A", 0), int(&m, "A", 1), int(&m, "A", 2)),
        (-1, 0, -1)
    );
    let m = run!("Module_Str_SEEN/strsub_0.TXT");
    assert_eq!(str_s(&m, 1), "lid");
    let m = run!("Module_Str_SEEN/strsub_1.TXT");
    assert_eq!(
        str_s(&m, 1),
        sjis(b"\x82\xDC\x82\xBE\x8A\x6F\x82\xA6\x82\xC4\x82\xE9\x81\x48")
    );
    let m = run!("Module_Str_SEEN/strsub_2.TXT");
    assert_eq!(str_s(&m, 1), "al");
    let m = run!("Module_Str_SEEN/strsub_3.TXT");
    assert_eq!(str_s(&m, 1), sjis(b"\x96\xBC\x91\x4F"));
    let m = run!("Module_Str_SEEN/strrsub_0.TXT");
    assert_eq!(str_s(&m, 1), "id");
    let m = run!("Module_Str_SEEN/strrsub_1.TXT");
    assert_eq!(str_s(&m, 1), "i");
    let m = run!("Module_Str_SEEN/strcharlen_0.TXT");
    assert_eq!(int(&m, "A", 0), 5);
    let m = run!("Module_Str_SEEN/strcharlen_1.TXT");
    assert_eq!(int(&m, "A", 0), 14);
    let m = run!("Module_Str_SEEN/strtrunc_0.TXT");
    assert_eq!(str_s(&m, 0), "va");
    let m = run!("Module_Str_SEEN/strtrunc_1.TXT");
    assert_eq!(str_s(&m, 0), sjis(b"\x82\xED\x82\xBD\x82\xB5"));
    let m = run!("Module_Str_SEEN/hantozen_0.TXT");
    assert_eq!(
        str_s(&m, 0),
        sjis(b"\x82\x50\x82\x51\x82\x52\x82\x53\x82\x54")
    );
    let m = run!("Module_Str_SEEN/hantozen_1.TXT");
    assert_eq!(
        str_s(&m, 0),
        sjis(b"\x83\x8F\x83\x5E\x83\x56\x83\x6D\x83\x69\x83\x7D\x83\x47")
    );
    let m = run!("Module_Str_SEEN/zentohan_0.TXT");
    assert_eq!(str_s(&m, 0), "12345");
    let m = run!("Module_Str_SEEN/zentohan_1.TXT");
    assert_eq!(str_s(&m, 0), sjis(b"\xDC\xC0\xBC\xC9\xC5\xCF\xB4"));
    let m = run!("Module_Str_SEEN/uppercase_0.TXT");
    assert_eq!(str_s(&m, 0), "VALID");
    let m = run!("Module_Str_SEEN/uppercase_1.TXT");
    assert_eq!(
        (str_s(&m, 0), str_s(&m, 1)),
        ("Valid".into(), "VALID".into())
    );
    let m = run!("Module_Str_SEEN/lowercase_0.TXT");
    assert_eq!(str_s(&m, 0), "valid");
    let m = run!("Module_Str_SEEN/lowercase_1.TXT");
    assert_eq!(
        (str_s(&m, 0), str_s(&m, 1)),
        ("Valid".into(), "valid".into())
    );
    let m = run!("Module_Str_SEEN/itoa_ws_0.TXT");
    assert_eq!(str_s(&m, 0), sjis(b"\x81\x7C\x82\x50"));
    assert_eq!(str_s(&m, 1), sjis(b"\x81\x7C\x81\x40\x81\x40\x82\x50"));
    assert_eq!(str_s(&m, 2), sjis(b"\x82\x52"));
    assert_eq!(str_s(&m, 3), sjis(b"\x81\x40\x81\x40\x82\x50"));
    let m = run!("Module_Str_SEEN/itoa_s_0.TXT");
    assert_eq!(
        [str_s(&m, 0), str_s(&m, 1), str_s(&m, 2), str_s(&m, 3)],
        ["-1", "-  1", "3", "  1"]
    );
    let m = run!("Module_Str_SEEN/itoa_w_0.TXT");
    assert_eq!(str_s(&m, 0), sjis(b"\x81\x7C\x82\x50"));
    assert_eq!(str_s(&m, 1), sjis(b"\x81\x7C\x82\x4F\x82\x4F\x82\x50"));
    assert_eq!(str_s(&m, 2), sjis(b"\x82\x52"));
    assert_eq!(str_s(&m, 3), sjis(b"\x82\x4F\x82\x4F\x82\x50"));
    let m = run!("Module_Str_SEEN/itoa_0.TXT");
    assert_eq!(
        [str_s(&m, 0), str_s(&m, 1), str_s(&m, 2), str_s(&m, 3)],
        ["-1", "-001", "3", "001"]
    );
    let m = run!("Module_Str_SEEN/atoi_0.TXT");
    assert_eq!(
        (0..5).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![15, 15, -12, 5, 0]
    );
    let m = run!("Module_Str_SEEN/digits_0.TXT");
    assert_eq!(
        (int(&m, "A", 0), int(&m, "A", 1), int(&m, "A", 2)),
        (1, 2, 2)
    );
    let m = run!("Module_Str_SEEN/strpos_0.TXT");
    assert_eq!(
        (int(&m, "A", 0), int(&m, "A", 1), int(&m, "A", 2)),
        (0, 8, -1)
    );
    let m = run!("Module_Str_SEEN/strlpos_0.TXT");
    assert_eq!(
        (int(&m, "A", 0), int(&m, "A", 1), int(&m, "A", 2)),
        (0, 12, -1)
    );
    let m = run!("Module_Str_SEEN/strused_0.TXT");
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (0, 1));
}

#[test]
fn mem_module() {
    let m = run!("Module_Mem_SEEN/setarray_0.TXT");
    assert_eq!(
        (0..4).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![1, 2, 3, -1]
    );
    let m = run!("Module_Mem_SEEN/setrng_0.TXT");
    assert_eq!(
        (0..5).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![0, 0, 0, 0, -1]
    );
    let m = run!("Module_Mem_SEEN/setrng_1.TXT");
    assert_eq!(
        (0..5).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![4, 4, 4, 4, -1]
    );
    let m = run!("Module_Mem_SEEN/cpyrng_0.TXT");
    assert_eq!(
        (0..3)
            .map(|i| (int(&m, "A", i), int(&m, "B", i)))
            .collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3)]
    );
    let m = run!("Module_Mem_SEEN/setarray_stepped_0.TXT");
    assert_eq!(
        (0..6).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![1, -1, 2, -1, 3, -1]
    );
    let m = run!("Module_Mem_SEEN/setrng_stepped_0.TXT");
    assert_eq!(
        (0..6).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![0, -1, 0, -1, 0, -1]
    );
    let m = run!("Module_Mem_SEEN/setrng_stepped_1.TXT");
    assert_eq!(
        (0..6).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![5, -1, 5, -1, 5, -1]
    );
    let m = run!("Module_Mem_SEEN/cpyvars_0.TXT");
    assert_eq!(
        (0..3).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![5, 1, 2]
    );
    let m = run!("Module_Mem_SEEN/sum_0.TXT");
    assert_eq!(int(&m, "A", 10), 6);
}

#[test]
fn jmp_module() {
    let m = run!("Module_Jmp_SEEN/goto_0.TXT");
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 0));
    let m = run!("Module_Jmp_SEEN/goto_if_0.TXT");
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 1));
    let m = run!("Module_Jmp_SEEN/goto_if_0.TXT", |m| set(m, "B", 0, 1));
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 0));
    let m = run!("Module_Jmp_SEEN/goto_unless_0.TXT");
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 0));
    for i in 0..3 {
        let m = run!("Module_Jmp_SEEN/goto_on_0.TXT", |m| set(m, "B", 0, i));
        assert_eq!(int(&m, "A", 0), i);
    }
    let m = run!("Module_Jmp_SEEN/goto_on_0.TXT", |m| set(m, "B", 0, 7));
    assert_eq!(int(&m, "A", 0), -1);
    for i in 0..3 {
        let m = run!("Module_Jmp_SEEN/goto_case_0.TXT", |m| set(m, "B", 0, i));
        assert_eq!(int(&m, "A", 0), i);
    }
    let m = run!("Module_Jmp_SEEN/goto_case_0.TXT", |m| set(m, "B", 0, 29));
    assert_eq!(int(&m, "A", 0), 3);
    let m = run!("Module_Jmp_SEEN/gosub_0.TXT");
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 1));
    let m = run!("Module_Jmp_SEEN/gosub_if_0.TXT");
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 0));
    let m = run!("Module_Jmp_SEEN/gosub_if_0.TXT", |m| set(m, "B", 0, 1));
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 1));
    let m = run!("Module_Jmp_SEEN/gosub_unless_0.TXT");
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 1));
    let m = run!("Module_Jmp_SEEN/gosub_unless_0.TXT", |m| set(m, "B", 0, 1));
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (1, 0));
    for i in 0..3 {
        let m = run!("Module_Jmp_SEEN/gosub_case_0.TXT", |m| set(m, "B", 0, i));
        assert_eq!(int(&m, "A", 0), i);
    }
    let m = run!("Module_Jmp_SEEN/gosub_case_0.TXT", |m| set(m, "B", 0, 29));
    assert_eq!(int(&m, "A", 0), 3);
    for i in 1..4 {
        let m = run!("Module_Jmp_SEEN/jump_0.TXT", |m| set(m, "B", 0, i));
        assert_eq!(int(&m, "A", 0), i);
        let m = run!("Module_Jmp_SEEN/jumpTest.TXT", |m| set(m, "B", 0, i));
        assert_eq!(int(&m, "A", 0), i);
        let m = run!("Module_Jmp_SEEN/farcallTest_0.TXT", |m| set(m, "B", 0, i));
        assert_eq!(
            (int(&m, "A", 0), int(&m, "A", 1), int(&m, "A", 2)),
            (1, i, 1)
        );
    }
    let m = run!("Module_Jmp_SEEN/gosub_with_0.TXT");
    assert_eq!((int(&m, "B", 0), int(&m, "B", 1)), (1, 2));
    assert_eq!((str_s(&m, 0), str_s(&m, 1)), ("one".into(), "two".into()));
    assert_eq!(str_s(&m, 3), "onetwo");
    assert_eq!(int(&m, "D", 0), 3);
    assert_eq!((int(&m, "A", 0), int(&m, "A", 1)), (3, 6));
    fn fib(n: i32) -> i32 {
        if n < 2 { n } else { fib(n - 1) + fib(n - 2) }
    }
    for i in 0..10 {
        let m = run!("Module_Jmp_SEEN/fibonacci.TXT", |m| set(m, "D", 0, i));
        assert_eq!(int(&m, "E", 0), fib(i));
    }
    for offset in 0..2 {
        for entrypoint in 1..4 {
            let m = run!("Module_Jmp_SEEN/farcall_withTest.TXT", |m| {
                set(m, "B", 0, entrypoint);
                set(m, "B", 1, offset);
            });
            assert_eq!(int(&m, "A", 1), entrypoint + offset);
        }
    }
    let m = run!("Module_Jmp_SEEN/pushStringValueUp.TXT");
    assert_eq!(
        m.read_string(StrTarget {
            bank: bank::STR_M,
            index: 0
        })
        .unwrap(),
        "GOOD"
    );
}

#[test]
fn sys_module() {
    let m = run!("Module_Sys_SEEN/SceneNum.TXT");
    assert_eq!(
        (int(&m, "A", 0), int(&m, "A", 1), int(&m, "A", 2)),
        (1, 248, 639)
    );
    let m = run!("Module_Sys_SEEN/builtins.TXT");
    let v: Vec<i32> = (0..6).map(|i| int(&m, "A", i)).collect();
    assert_eq!(v[0], 0);
    assert!(v[1] == 0 || v[1] == -1);
    assert_eq!(v[2], -1);
    assert!((-10..0).contains(&v[3]));
    assert!((-10..=10).contains(&v[4]));
    assert!((-10..=10).contains(&v[5]));
}

#[test]
fn expressions() {
    let m = run!("ExpressionTest_SEEN/basicOperators.TXT");
    assert_eq!(
        (0..10).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![2, 3, 0, 3, 10, 30, 10, 2, 2, 1]
    );
    let m = run!("ExpressionTest_SEEN/comparisonOperators.TXT");
    assert_eq!(
        (0..14).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0]
    );
    let m = run!("ExpressionTest_SEEN/logicalOperators.TXT");
    assert_eq!(
        (0..7).map(|i| int(&m, "A", i)).collect::<Vec<_>>(),
        vec![1, 0, 1, 1, 1, 0, 0]
    );
    let m = run!("ExpressionTest_SEEN/previousErrors.TXT");
    assert_eq!(
        (0..6).map(|i| int(&m, "B", i)).collect::<Vec<_>>(),
        vec![1, 1, 1, 0, 0, 10]
    );
}
