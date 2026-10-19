fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("vita") {
        return;
    }
    // Link published VitaSDK packages; no third-party source is copied into
    // this workspace. vita2d carries its shaders precompiled, so no runtime
    // shader compiler (libshacccg.suprx) is needed on the device.
    for library in [
        "vita2d",
        "SceGxm_stub",
        "SceDisplay_stub",
        "SceSysmodule_stub",
        "SceCommonDialog_stub",
        "SceAppUtil_stub",
        "SceAppMgr_stub",
        "SceSysmem_stub",
        "m",
    ] {
        println!("cargo:rustc-link-lib=static={library}");
    }
    // vita-elf-create appends the module's SCE tables (a few KiB) to the
    // code segment; VitaSDK's linker script leaves `__sce_headroom` bytes
    // for them before the 64 KiB-aligned data segment. Without it a build
    // whose code happened to end just below a 64 KiB boundary failed with
    // "segment 1 overlaps".
    println!("cargo:rustc-link-arg=-Wl,--defsym=__sce_headroom=0x4000");
}
