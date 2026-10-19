use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::runtime::{CommandContext, Value};

use super::prop_access;
use super::syscom;

const CHECK_ACTIVE: i32 = 0;
const SHELL_OPEN_FILE: i32 = 1;
const CHECK_DUMMY_FILE_ONCE: i32 = 2;
const OPEN_DIALOG_FOR_CHIHAYA_BENCH: i32 = 3;
const GET_SPEC_INFO_FOR_CHIHAYA_BENCH: i32 = 4;
const SHELL_OPEN_WEB: i32 = 5;
const CHECK_FILE_EXIST: i32 = 6;
const DEBUG_MESSAGEBOX_OK: i32 = 7;
const DEBUG_MESSAGEBOX_OKCANCEL: i32 = 8;
const DEBUG_MESSAGEBOX_YESNO: i32 = 9;
const DEBUG_MESSAGEBOX_YESNOCANCEL: i32 = 10;
const DEBUG_WRITE_LOG: i32 = 11;
const CHECK_FILE_EXIST_SAVE_DIR: i32 = 12;
const CHECK_DEBUG_FLAG: i32 = 13;
const GET_CALENDAR: i32 = 14;
const GET_UNIX_TIME: i32 = 15;
const GET_LANGUAGE: i32 = 16;
const MESSAGEBOX_OK: i32 = 17;
const MESSAGEBOX_OKCANCEL: i32 = 18;
const MESSAGEBOX_YESNO: i32 = 19;
const MESSAGEBOX_YESNOCANCEL: i32 = 20;
const CLEAR_DUMMY_FILE: i32 = 21;

struct Call<'a> {
    op: i32,
    params: &'a [Value],
}

fn parse_call<'a>(ctx: &CommandContext, form_id: u32, args: &'a [Value]) -> Option<Call<'a>> {
    let (chain_pos, chain) = prop_access::parse_element_chain_ctx(ctx, form_id, args)?;
    if chain.len() < 2 {
        return None;
    }
    let params = prop_access::script_args(args, chain_pos);
    Some(Call {
        op: chain[1],
        params,
    })
}

fn p_str(params: &[Value], idx: usize) -> &str {
    params.get(idx).and_then(|v| v.as_str()).unwrap_or("")
}

fn join_game_path(base: &Path, raw: &str) -> PathBuf {
    if raw.is_empty() {
        return base.to_path_buf();
    }
    let norm = raw.replace('\\', "/");
    let p = Path::new(&norm);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        base.join(p)
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = std::process::Command::new(program)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!text.is_empty()).then_some(text)
}

fn squash_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(target_os = "windows")]
fn chihaya_os_name() -> String {
    command_output(
        "powershell.exe",
        &[
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "$o=Get-CimInstance Win32_OperatingSystem; $a=@($o.Caption,$o.CSDVersion,$o.OSArchitecture) | Where-Object { $_ -and $_.Trim() }; $a -join ' '",
        ],
    )
    .map(|s| {
        squash_spaces(&s)
            .replace('™', "(TM)")
            .replace('®', "(R)")
    })
    .unwrap_or_else(|| format!("Windows {}", std::env::consts::ARCH))
}

#[cfg(target_os = "windows")]
fn chihaya_cpu_name() -> String {
    command_output(
        "powershell.exe",
        &[
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty Name",
        ],
    )
    .map(|s| squash_spaces(&s))
    .filter(|s| !s.is_empty())
    .unwrap_or_else(|| "不明な CPU".to_string())
}

#[cfg(target_os = "macos")]
fn chihaya_os_name() -> String {
    let product =
        command_output("sw_vers", &["-productName"]).unwrap_or_else(|| "macOS".to_string());
    let version = command_output("sw_vers", &["-productVersion"]).unwrap_or_default();
    let build = command_output("sw_vers", &["-buildVersion"]).unwrap_or_default();
    let mut fields = vec![product];
    if !version.is_empty() {
        fields.push(version);
    }
    if !build.is_empty() {
        fields.push(format!("({build})"));
    }
    fields.push(std::env::consts::ARCH.to_string());
    fields.join(" ")
}

#[cfg(target_os = "macos")]
fn chihaya_cpu_name() -> String {
    if let Some(name) = command_output("sysctl", &["-n", "machdep.cpu.brand_string"]) {
        let name = squash_spaces(&name);
        if !name.is_empty() {
            return name;
        }
    }
    if let Some(info) = command_output("system_profiler", &["SPHardwareDataType"]) {
        for line in info.lines() {
            let line = line.trim();
            for key in ["Chip:", "Processor Name:"] {
                if let Some(value) = line.strip_prefix(key) {
                    let value = squash_spaces(value.trim());
                    if !value.is_empty() {
                        return value;
                    }
                }
            }
        }
    }
    command_output("sysctl", &["-n", "hw.model"])
        .map(|s| squash_spaces(&s))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "不明な CPU".to_string())
}

#[cfg(target_os = "linux")]
fn chihaya_os_name() -> String {
    let pretty = fs::read_to_string("/etc/os-release").ok().and_then(|text| {
        text.lines().find_map(|line| {
            let raw = line.strip_prefix("PRETTY_NAME=")?;
            let raw = raw.trim();
            let value = raw
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(raw)
                .replace("\\\"", "\"")
                .replace("\\\\", "\\");
            (!value.is_empty()).then_some(value)
        })
    });
    format!(
        "{} {}",
        pretty.unwrap_or_else(|| "Linux".to_string()),
        std::env::consts::ARCH
    )
}

#[cfg(target_os = "linux")]
fn chihaya_cpu_name() -> String {
    if let Ok(text) = fs::read_to_string("/proc/cpuinfo") {
        for key in ["model name", "Hardware", "Processor"] {
            if let Some(value) = text.lines().find_map(|line| {
                let (name, value) = line.split_once(':')?;
                (name.trim() == key).then(|| squash_spaces(value.trim()))
            }) && !value.is_empty()
            {
                return value;
            }
        }
    }
    "不明な CPU".to_string()
}

#[cfg(target_os = "android")]
fn chihaya_os_name() -> String {
    format!("Android {}", std::env::consts::ARCH)
}

#[cfg(target_os = "android")]
fn chihaya_cpu_name() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| {
            text.lines().find_map(|line| {
                let (name, value) = line.split_once(':')?;
                matches!(name.trim(), "model name" | "Hardware" | "Processor")
                    .then(|| squash_spaces(value.trim()))
            })
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| std::env::consts::ARCH.to_string())
}

#[cfg(target_os = "ios")]
fn chihaya_os_name() -> String {
    format!("iOS {}", std::env::consts::ARCH)
}

#[cfg(target_os = "ios")]
fn chihaya_cpu_name() -> String {
    std::env::consts::ARCH.to_string()
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn chihaya_os_name() -> String {
    "WebAssembly".to_string()
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn chihaya_cpu_name() -> String {
    "wasm32".to_string()
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "android",
    target_os = "ios",
    all(target_arch = "wasm32", target_os = "unknown")
)))]
fn chihaya_os_name() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "android",
    target_os = "ios",
    all(target_arch = "wasm32", target_os = "unknown")
)))]
fn chihaya_cpu_name() -> String {
    std::env::consts::ARCH.to_string()
}

fn chihaya_spec_info(ctx: &CommandContext) -> String {
    let mut result = format!("OS = {}\nCPU = {}", chihaya_os_name(), chihaya_cpu_name());
    let adapter = ctx.globals.system.chihaya_display_adapter_name.trim();
    if !adapter.is_empty() {
        result.push_str("\nビデオカード = ");
        result.push_str(adapter);
    }
    result
}

pub fn dispatch(ctx: &mut CommandContext, form_id: u32, args: &[Value]) -> Result<bool> {
    let Some(call) = parse_call(ctx, form_id, args) else {
        return Ok(false);
    };

    match call.op {
        GET_CALENDAR => {
            let tm = local_time_fields();
            let vals = [tm.0, tm.1, tm.2, tm.3, tm.4, tm.5, tm.6, tm.7];
            for (idx, value) in vals.iter().enumerate() {
                if let Some(Value::Element(chain)) = call.params.get(idx) {
                    prop_access::assign_to_chain(ctx, chain, Value::Int(*value));
                }
            }
            return Ok(true);
        }
        GET_UNIX_TIME => {
            let t = crate::platform_time::unix_time_secs() as i64;
            ctx.push(Value::Int(t));
            return Ok(true);
        }
        CHECK_ACTIVE => {
            ctx.push(Value::Int(if ctx.globals.system.active_flag {
                1
            } else {
                0
            }));
            return Ok(true);
        }
        CHECK_DEBUG_FLAG => {
            ctx.push(Value::Int(if ctx.globals.system.debug_flag {
                1
            } else {
                0
            }));
            return Ok(true);
        }
        SHELL_OPEN_FILE => {
            let requested = join_game_path(&ctx.project_dir, p_str(call.params, 0));
            let resolved = crate::resource::resolve_game_path(&requested)
                .ok()
                .flatten();
            if let Some(path) = resolved.as_ref() {
                let _ = ctx.net.open_file(path);
            }
            ctx.globals.system.debug_logs.push(format!(
                "shell_open_file:{}",
                resolved.as_deref().unwrap_or(&requested).display()
            ));
            return Ok(true);
        }
        SHELL_OPEN_WEB => {
            let url = p_str(call.params, 0);
            let _ = ctx.net.open_url(url);
            ctx.globals
                .system
                .debug_logs
                .push(format!("shell_open_web:{url}"));
            return Ok(true);
        }
        CHECK_FILE_EXIST => {
            let path = join_game_path(&ctx.project_dir, p_str(call.params, 0));
            ctx.push(Value::Int(if crate::resource::game_path_exists(&path) {
                1
            } else {
                0
            }));
            return Ok(true);
        }
        CHECK_FILE_EXIST_SAVE_DIR => {
            let save_dir = syscom::save_dir(&ctx.project_dir);
            let path = join_game_path(&save_dir, p_str(call.params, 0));
            ctx.push(Value::Int(if crate::resource::game_path_exists(&path) {
                1
            } else {
                0
            }));
            return Ok(true);
        }
        CHECK_DUMMY_FILE_ONCE => {
            let name = p_str(call.params, 0);
            let key = call.params.get(1).and_then(|v| v.as_i64()).unwrap_or(0);
            let code = p_str(call.params, 2);
            let sig = format!("{name}:{key}:{code}");
            ctx.globals.system.dummy_checks.insert(sig);
            return Ok(true);
        }
        CLEAR_DUMMY_FILE => {
            ctx.globals.system.dummy_checks.clear();
            return Ok(true);
        }
        MESSAGEBOX_OK | MESSAGEBOX_OKCANCEL | MESSAGEBOX_YESNO | MESSAGEBOX_YESNOCANCEL => {
            let text = messagebox_text(ctx, call.params);
            if let Some(ret) = handle_messagebox(ctx, call.op, false, text) {
                ctx.push(Value::Int(ret));
            }
            return Ok(true);
        }
        DEBUG_MESSAGEBOX_OK
        | DEBUG_MESSAGEBOX_OKCANCEL
        | DEBUG_MESSAGEBOX_YESNO
        | DEBUG_MESSAGEBOX_YESNOCANCEL => {
            let text = messagebox_text(ctx, call.params);
            if ctx.globals.system.debug_flag {
                if let Some(ret) = handle_messagebox(ctx, call.op, true, text) {
                    ctx.push(Value::Int(ret));
                }
            } else {
                ctx.push(Value::Int(0));
            }
            return Ok(true);
        }
        DEBUG_WRITE_LOG => {
            if ctx.globals.system.debug_flag {
                let s = match call.params.first() {
                    Some(Value::Int(v)) => v.to_string(),
                    Some(Value::Str(s)) => s.clone(),
                    _ => String::new(),
                };
                write_debug_log(
                    &ctx.project_dir,
                    &s,
                    ctx.current_scene_name.as_deref(),
                    ctx.current_line_no,
                );
                ctx.globals.system.debug_logs.push(s);
            }
            return Ok(true);
        }
        GET_SPEC_INFO_FOR_CHIHAYA_BENCH => {
            // eng_chihaya.cpp::tnm_get_spec_info_for_chihaya_bench().
            ctx.push(Value::Str(chihaya_spec_info(ctx)));
            return Ok(true);
        }
        OPEN_DIALOG_FOR_CHIHAYA_BENCH => {
            // eng_chihaya.cpp::tnm_open_chihaya_bench_dialog().  Keep a
            // history entry for diagnostics, but unlike the old placeholder
            // this is a modal operation and script execution stops here.
            let text = p_str(call.params, 0).to_string();
            ctx.globals.system.bench_dialogs.push(text.clone());
            ctx.request_chihaya_bench_dialog(text);
            return Ok(true);
        }
        GET_LANGUAGE => {
            ctx.push(Value::Str(ctx.globals.system.language_code.clone()));
            return Ok(true);
        }
        _ => {}
    }

    Ok(false)
}

fn messagebox_text(ctx: &CommandContext, params: &[Value]) -> String {
    match params.first() {
        Some(Value::Int(v)) => v.to_string(),
        Some(Value::Str(s)) => s.clone(),
        Some(v) => v.as_str().unwrap_or("").to_string(),
        None => {
            if let Some(name) = ctx.current_scene_name.as_deref() {
                format!("{name}:{}", ctx.current_line_no)
            } else {
                String::new()
            }
        }
    }
}

fn handle_messagebox(
    ctx: &mut CommandContext,
    kind: i32,
    debug_only: bool,
    text: String,
) -> Option<i64> {
    ctx.globals
        .system
        .messagebox_history
        .push(crate::runtime::globals::SystemMessageBoxRecord {
            kind,
            text: text.clone(),
            debug_only,
        });

    let buttons = messagebox_buttons(kind);
    let max_value = buttons.iter().map(|b| b.value).max().unwrap_or(0);
    if !ctx.globals.system.messagebox_response_queue.is_empty() {
        let v = ctx.globals.system.messagebox_response_queue.remove(0);
        return Some(v.clamp(0, max_value));
    }

    ctx.request_system_messagebox(kind, debug_only, text, buttons);
    None
}

fn messagebox_buttons(kind: i32) -> Vec<crate::runtime::globals::SystemMessageBoxButton> {
    let raw: &[(&str, i64)] = match kind {
        MESSAGEBOX_OK | DEBUG_MESSAGEBOX_OK => &[("OK", 0)],
        MESSAGEBOX_OKCANCEL | DEBUG_MESSAGEBOX_OKCANCEL => &[("OK", 0), ("CANCEL", 1)],
        MESSAGEBOX_YESNO | DEBUG_MESSAGEBOX_YESNO => &[("YES", 0), ("NO", 1)],
        MESSAGEBOX_YESNOCANCEL | DEBUG_MESSAGEBOX_YESNOCANCEL => {
            &[("YES", 0), ("NO", 1), ("CANCEL", 2)]
        }
        _ => &[("OK", 0)],
    };
    raw.iter()
        .map(
            |(label, value)| crate::runtime::globals::SystemMessageBoxButton {
                label: (*label).to_string(),
                value: *value,
            },
        )
        .collect()
}

fn local_time_fields() -> (i64, i64, i64, i64, i64, i64, i64, i64) {
    let now = crate::platform_time::local_time_fields();
    (
        now.year as i64,
        now.month as i64,
        now.day as i64,
        now.weekday_sunday0 as i64,
        now.hour as i64,
        now.minute as i64,
        now.second as i64,
        now.millisecond as i64,
    )
}

fn write_debug_log(project_dir: &Path, msg: &str, scene_name: Option<&str>, line_no: i64) {
    if msg.is_empty() {
        return;
    }
    let dir = project_dir.join("__DEBUG_LOG");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("debug_log.txt");
    let stamp = crate::platform_time::local_log_timestamp();
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
        if let Some(scene) = scene_name {
            let _ = writeln!(f, "{}\t({}.ss line={})\t{}", stamp, scene, line_no, msg);
        } else {
            let _ = writeln!(f, "{}\t{}", stamp, msg);
        }
    }
}
