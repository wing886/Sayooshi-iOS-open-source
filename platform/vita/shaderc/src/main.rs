//! Compiles the Vita renderer's Cg shaders to GXP with Sony's compiler
//! (`libshacccg.suprx`). Run it once in Vita3K (or on
//! a Vita with the module) after changing a shader; the player embeds the
//! results, so players need no shader compiler. See
//! `platform/vita/shaderc/compile-shaders.sh`.

#[cfg(not(target_os = "vita"))]
fn main() {
    eprintln!("siglus_vita_shaderc must be built with cargo-vita for PS Vita");
}

// The compiler needs far more than newlib's default heap.
#[cfg(target_os = "vita")]
#[used]
#[unsafe(export_name = "_newlib_heap_size_user")]
pub static NEWLIB_HEAP_SIZE_USER: u32 = 128 * 1024 * 1024;

/// `libshacccg.suprx` allocates from SceLibc's heap, which a homebrew app
/// only gets when it exports this size.
#[cfg(target_os = "vita")]
#[used]
#[unsafe(export_name = "sceLibcHeapSize")]
pub static SCE_LIBC_HEAP_SIZE: u32 = 64 * 1024 * 1024;

#[cfg(target_os = "vita")]
mod sources {
    include!(concat!(env!("OUT_DIR"), "/sources.rs"));
}

#[cfg(target_os = "vita")]
fn main() {
    use std::ffi::{CStr, CString, c_char, c_int, c_void};
    use std::io::Write;

    const OUT: &str = "ux0:data/siglus_shaderc";
    const PROFILE_VP: c_int = 0;
    const PROFILE_FP: c_int = 1;
    const TRIVIAL_CALLBACKS: c_int = 1;

    #[repr(C)]
    struct SourceFile {
        file_name: *const c_char,
        text: *const c_char,
        size: u32,
    }
    #[repr(C)]
    struct CallbackList {
        open_file: Option<
            extern "C" fn(*const c_char, *const c_void, *const c_void, *mut *const c_char)
                -> *mut SourceFile,
        >,
        rest: [*const c_void; 5],
    }
    /// `SceShaccCgCompileOptions` (psp2/shacccg.h).
    #[repr(C)]
    struct CompileOptions {
        main_source_file: *const c_char,
        target_profile: c_int,
        entry_function_name: *const c_char,
        search_path_count: u32,
        search_paths: *const c_void,
        macro_definition_count: u32,
        macro_definitions: *const c_void,
        include_file_count: u32,
        include_files: *const c_void,
        suppressed_warnings_count: u32,
        suppressed_warnings: *const c_void,
        locale: c_int,
        use_fx: i32,
        no_stdlib: i32,
        optimization_level: i32,
        use_fastmath: i32,
        use_fastprecision: i32,
        use_fastint: i32,
        field_48: i32,
        warnings_as_errors: i32,
        performance_warnings: i32,
        warning_level: i32,
        pedantic: i32,
        pedantic_error: i32,
        field_60: i32,
        field_64: i32,
    }
    #[repr(C)]
    struct Diagnostic {
        level: c_int,
        code: u32,
        location: *const [u32; 3],
        message: *const c_char,
    }
    #[repr(C)]
    struct CompileOutput {
        program_data: *const u8,
        program_size: u32,
        diagnostic_count: i32,
        diagnostics: *const Diagnostic,
    }
    const _: () = assert!(std::mem::size_of::<CompileOptions>() == 0x68);
    const _: () = assert!(std::mem::size_of::<CallbackList>() == 0x18);

    unsafe extern "C" {
        fn sceKernelLoadStartModule(
            path: *const c_char,
            args: u32,
            argp: *const c_void,
            flags: u32,
            option: *const c_void,
            status: *mut c_int,
        ) -> c_int;
        fn sceShaccCgSetDefaultAllocator(
            malloc: unsafe extern "C" fn(usize) -> *mut c_void,
            free: unsafe extern "C" fn(*mut c_void),
        ) -> c_int;
        fn sceShaccCgInitializeCallbackList(callbacks: *mut CallbackList, defaults: c_int);
        fn sceShaccCgInitializeCompileOptions(options: *mut CompileOptions) -> c_int;
        fn sceShaccCgCompileProgram(
            options: *const CompileOptions,
            callbacks: *const CallbackList,
            unk: c_int,
        ) -> *const CompileOutput;
        fn sceShaccCgDestroyCompileOutput(output: *const CompileOutput);
        fn sceShaccCgGetVersionString() -> *const c_char;
        fn malloc(size: usize) -> *mut c_void;
        fn free(ptr: *mut c_void);
        fn sceKernelExitProcess(status: c_int) -> c_int;
    }

    /// The source being compiled, handed to the compiler's open callback.
    static mut SOURCE: SourceFile = SourceFile {
        file_name: c"<built-in>".as_ptr(),
        text: std::ptr::null(),
        size: 0,
    };
    /// The main source, or an included `.cgh` by name.
    extern "C" fn open_file(
        name: *const c_char,
        _from: *const c_void,
        _options: *const c_void,
        error: *mut *const c_char,
    ) -> *mut SourceFile {
        let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();
        if name == "<built-in>" {
            return &raw mut SOURCE;
        }
        let file_name = name.rsplit(['/', ':']).next().unwrap_or(&name);
        match sources::INCLUDES.iter().find(|(include, _)| *include == file_name) {
            Some((include, text)) => Box::leak(Box::new(SourceFile {
                file_name: Box::leak(CString::new(*include).unwrap().into_boxed_c_str()).as_ptr(),
                text: Box::leak(CString::new(*text).unwrap().into_boxed_c_str()).as_ptr(),
                size: text.len() as u32,
            })),
            None => {
                unsafe { *error = c"no such include".as_ptr() };
                std::ptr::null_mut()
            }
        }
    }

    let _ = std::fs::remove_dir_all(OUT);
    std::fs::create_dir_all(OUT).expect("output directory");
    // Appended line by line (opened and closed each time) so the report
    // survives a crash.
    let report = |line: &str| {
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{OUT}/report.txt"))
        {
            let _ = writeln!(file, "{line}");
        }
    };
    let module = unsafe {
        sceKernelLoadStartModule(
            c"ur0:data/libshacccg.suprx".as_ptr(),
            0,
            std::ptr::null(),
            0,
            std::ptr::null(),
            std::ptr::null_mut(),
        )
    };
    if module < 0 {
        report(&format!("loading ur0:data/libshacccg.suprx failed: {module:#x}"));
        report("DONE failed=all");
        unsafe { sceKernelExitProcess(0) };
        return;
    }
    unsafe { sceShaccCgSetDefaultAllocator(malloc, free) };
    let version = unsafe { CStr::from_ptr(sceShaccCgGetVersionString()) };
    report(&format!("compiler {}", version.to_string_lossy()));
    let mut failed = 0;
    for &(name, fragment, source) in sources::SOURCES {
        let text = CString::new(source).expect("shader source");
        unsafe {
            SOURCE.text = text.as_ptr();
            SOURCE.size = source.len() as u32;
        }
        let mut callbacks: CallbackList = unsafe { std::mem::zeroed() };
        unsafe { sceShaccCgInitializeCallbackList(&mut callbacks, TRIVIAL_CALLBACKS) };
        callbacks.open_file = Some(open_file);
        let mut options: CompileOptions = unsafe { std::mem::zeroed() };
        unsafe { sceShaccCgInitializeCompileOptions(&mut options) };
        options.main_source_file = c"<built-in>".as_ptr();
        options.target_profile = if fragment { PROFILE_FP } else { PROFILE_VP };
        options.entry_function_name = c"main".as_ptr();
        options.locale = 0;
        options.use_fx = 1;
        options.no_stdlib = 0;
        options.optimization_level = 3;
        options.use_fastmath = 1;
        options.use_fastprecision = 0;
        options.use_fastint = 1;
        options.performance_warnings = 1;
        options.warning_level = 3;
        let output = unsafe { sceShaccCgCompileProgram(&options, &callbacks, 0) };
        let mut messages = String::new();
        let mut program = None;
        if let Some(output) = unsafe { output.as_ref() } {
            for i in 0..output.diagnostic_count.max(0) as usize {
                let diagnostic = unsafe { &*output.diagnostics.add(i) };
                let text = unsafe { diagnostic.message.as_ref() }
                    .map(|_| unsafe { CStr::from_ptr(diagnostic.message) }.to_string_lossy())
                    .unwrap_or_default();
                let line = unsafe { diagnostic.location.as_ref() }.map_or(0, |loc| loc[1]);
                messages.push_str(&format!(
                    "  level {} code {} line {}: {}\n",
                    diagnostic.level, diagnostic.code, line, text
                ));
            }
            if !output.program_data.is_null() {
                program = Some(
                    unsafe {
                        std::slice::from_raw_parts(output.program_data, output.program_size as usize)
                    }
                    .to_vec(),
                );
            }
            unsafe { sceShaccCgDestroyCompileOutput(output) };
        } else {
            messages.push_str("  no compile output\n");
        }
        match program {
            Some(bytes) => {
                std::fs::write(format!("{OUT}/{name}.gxp"), &bytes).expect("write gxp");
                report(&format!("ok {name} {} bytes\n{messages}", bytes.len()));
            }
            None => {
                failed += 1;
                report(&format!("FAIL {name}\n{messages}"));
            }
        }
    }
    report(&format!("DONE failed={failed}"));
    unsafe { sceKernelExitProcess(0) };
}
