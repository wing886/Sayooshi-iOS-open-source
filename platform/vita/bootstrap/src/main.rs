#[cfg(not(target_os = "vita"))]
fn main() {
    eprintln!("siglus_vita_bootstrap must be packaged with cargo-vita for PS Vita");
}

#[cfg(target_os = "vita")]
mod vita {
    use std::ffi::c_void;
    use std::fs::{self, OpenOptions};
    use std::io::{self, Write};
    use std::mem::{MaybeUninit, size_of};
    use std::path::Path;

    use vitasdk_sys::{
        SCE_CTRL_START, SCE_DISPLAY_PIXELFORMAT_A8B8G8R8, SCE_DISPLAY_SETBUF_NEXTFRAME,
        SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW, SceCtrlData, SceDisplayFrameBuf,
        SceKernelFreeMemorySizeInfo, sceCtrlPeekBufferPositive, sceDisplaySetFrameBuf,
        sceDisplayWaitVblankStart, sceKernelAllocMemBlock, sceKernelGetFreeMemorySize,
        sceKernelGetMemBlockBase,
    };

    const WIDTH: usize = 960;
    const HEIGHT: usize = 544;
    const DISPLAY_BYTES: usize = WIDTH * HEIGHT * 4;
    const CDRAM_GRANULARITY: usize = 256 * 1024;
    const CDRAM_BYTES: usize = DISPLAY_BYTES.div_ceil(CDRAM_GRANULARITY) * CDRAM_GRANULARITY;
    const DATA_DIR: &str = "ux0:data/siglus_rs";

    fn log_line(message: &str) {
        let _ = fs::create_dir_all(DATA_DIR);
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{DATA_DIR}/vita-bootstrap.log"))
        {
            let _ = writeln!(file, "{message}");
        }
    }

    fn log_free_memory(stage: &str) {
        let mut info = SceKernelFreeMemorySizeInfo {
            size: size_of::<SceKernelFreeMemorySizeInfo>() as i32,
            size_user: 0,
            size_cdram: 0,
            size_phycont: 0,
        };
        let code = unsafe { sceKernelGetFreeMemorySize(&mut info) };
        log_line(&format!(
            "{stage}: free-memory result={code:#x} user={} cdram={} phycont={}",
            info.size_user, info.size_cdram, info.size_phycont
        ));
    }

    fn run() -> Result<(), String> {
        log_line("bootstrap start");
        log_free_memory("before display allocation");

        let uid = unsafe {
            sceKernelAllocMemBlock(
                c"siglus-vita-frame".as_ptr(),
                SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW,
                CDRAM_BYTES as u32,
                std::ptr::null_mut(),
            )
        };
        if uid < 0 {
            return Err(format!("CDRAM display allocation failed: {uid:#x}"));
        }
        let mut base: *mut c_void = std::ptr::null_mut();
        let code = unsafe { sceKernelGetMemBlockBase(uid, &mut base) };
        if code < 0 || base.is_null() {
            return Err(format!("CDRAM display mapping failed: {code:#x}"));
        }
        log_line(&format!("display allocation: {CDRAM_BYTES} bytes"));
        log_free_memory("after display allocation");

        // A single fixed-size display buffer establishes the initial memory
        // cost without allocating a second RGBA frame or a texture cache.
        let pixels = unsafe { std::slice::from_raw_parts_mut(base.cast::<u32>(), WIDTH * HEIGHT) };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let red = (x * 255 / WIDTH) as u32;
                let green = (y * 255 / HEIGHT) as u32;
                let blue = if (x / 32 + y / 32) & 1 == 0 { 96 } else { 160 };
                pixels[y * WIDTH + x] = 0xff00_0000 | (blue << 16) | (green << 8) | red;
            }
        }
        let display = SceDisplayFrameBuf {
            size: size_of::<SceDisplayFrameBuf>() as u32,
            base,
            pitch: WIDTH as u32,
            pixelformat: SCE_DISPLAY_PIXELFORMAT_A8B8G8R8,
            width: WIDTH as u32,
            height: HEIGHT as u32,
        };
        let code = unsafe { sceDisplaySetFrameBuf(&display, SCE_DISPLAY_SETBUF_NEXTFRAME) };
        if code < 0 {
            return Err(format!("display setup failed: {code:#x}"));
        }

        let marker = Path::new(DATA_DIR).join("bootstrap.txt");
        match fs::read_to_string(&marker) {
            Ok(contents) => log_line(&format!("read {} bytes from bootstrap.txt", contents.len())),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                log_line("bootstrap.txt absent; display/input probe continues")
            }
            Err(error) => log_line(&format!("bootstrap.txt read failed: {error}")),
        }

        loop {
            let mut pad = MaybeUninit::<SceCtrlData>::zeroed();
            let count = unsafe { sceCtrlPeekBufferPositive(0, pad.as_mut_ptr(), 1) };
            if count > 0 {
                let pad = unsafe { pad.assume_init() };
                if pad.buttons & SCE_CTRL_START != 0 {
                    break;
                }
            }
            unsafe { sceDisplayWaitVblankStart() };
        }
        // The display service may still scan this buffer. The process exits
        // immediately after return, so ownership remains with the process.
        log_line("START pressed; bootstrap exit");
        Ok(())
    }

    pub fn main() {
        if let Err(error) = run() {
            log_line(&format!("bootstrap error: {error}"));
            eprintln!("{error}");
        }
    }
}

#[cfg(target_os = "vita")]
fn main() {
    vita::main();
}
