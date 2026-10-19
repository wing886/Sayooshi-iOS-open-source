//! GXM on vita2d's context: programs, GPU memory, textures, render targets
//! and draws for the PS Vita renderer (`super`).
//!
//! vita2d still owns the display, the context and the shader patcher; every
//! draw here sets its own state (programs, blending, depth, stencil,
//! viewport, textures, uniforms), so nothing depends on what vita2d or an
//! earlier draw left behind. The shaders are the desktop renderer's WGSL
//! ported to Cg (`shaders/*.cg`), compiled to GXP by `platform/vita/shaderc`
//! and embedded: no shader compiler is needed on the device.

use std::collections::HashMap;
use std::ffi::{CStr, CString, c_void};

use vitasdk_sys::*;

/// vita2d's texture (vita2d.h): a GXM texture followed by its memory and,
/// for render targets, the target and surfaces.
#[repr(C)]
pub(super) struct Vita2dTexture {
    gxm_tex: SceGxmTexture,
    _private: [u8; 0],
}

// vita2d (VitaSDK's package) and the system stubs it calls, linked after
// this crate's objects.
#[link(name = "vita2d", kind = "static")]
#[link(name = "SceDisplay_stub", kind = "static")]
#[link(name = "SceSysmodule_stub", kind = "static")]
#[link(name = "SceCommonDialog_stub", kind = "static")]
#[link(name = "SceAppUtil_stub", kind = "static")]
#[link(name = "SceAppMgr_stub", kind = "static")]
#[link(name = "SceGxm_stub", kind = "static")]
#[link(name = "SceSysmem_stub", kind = "static")]
unsafe extern "C" {
    fn vita2d_init_advanced(temp_pool_size: u32) -> i32;
    fn vita2d_fini() -> i32;
    fn vita2d_get_context() -> *mut SceGxmContext;
    fn vita2d_get_shader_patcher() -> *mut SceGxmShaderPatcher;
    fn vita2d_start_drawing_advanced(target: *mut Vita2dTexture, flags: u32);
    fn vita2d_end_drawing();
    fn vita2d_wait_rendering_done();
    fn vita2d_swap_buffers();
    fn vita2d_set_vblank_wait(enable: i32);
    fn vita2d_pool_reset();
    fn vita2d_texture_set_alloc_memblock_type(kind: u32);
    fn vita2d_create_empty_texture_format(
        w: u32,
        h: u32,
        format: SceGxmTextureFormat,
    ) -> *mut Vita2dTexture;
    fn vita2d_create_empty_texture_rendertarget(
        w: u32,
        h: u32,
        format: SceGxmTextureFormat,
    ) -> *mut Vita2dTexture;
    fn vita2d_free_texture(texture: *mut Vita2dTexture);
    fn vita2d_texture_get_stride(texture: *const Vita2dTexture) -> u32;
    fn vita2d_texture_get_datap(texture: *const Vita2dTexture) -> *mut c_void;
    fn vita2d_get_current_fb() -> *mut c_void;
}

/// vita2d's display buffers: 960x544, rows 1024 pixels apart.
const DISPLAY_STRIDE_PIXELS: usize = 1024;

/// vita2d's own per-frame pool (its clear and clip rectangles); ours is
/// `VERTEX_RING_BYTES`.
const VITA2D_POOL_BYTES: u32 = 256 * 1024;
/// Vertex data of one frame. Three rings rotate: Vita3K's renderer can still
/// read a frame's buffers after `sceGxmFinish` has returned.
const VERTEX_RING_BYTES: usize = 8 * 1024 * 1024;
const VERTEX_RINGS: usize = 3;
/// Textures kept on the GPU; the least recently drawn go first. CDRAM has
/// ~110 MiB free after vita2d starts; small textures live in ordinary
/// memory (`CDRAM_MIN_TEXTURE_BYTES`) and a failed CDRAM allocation falls
/// back to it too.
pub(super) const TEXTURE_BUDGET: usize = 72 * 1024 * 1024;
/// CDRAM blocks are allocated in 256 KiB units; smaller textures (text,
/// icons) go to ordinary uncached memory instead of wasting most of a unit.
const CDRAM_MIN_TEXTURE_BYTES: usize = 256 * 1024;
/// Frames a retired texture's memory is kept: Vita3K reads textures on its
/// own thread and can still reach one after `sceGxmFinish` has returned
/// (it crashed uploading freed memory).
const RETIRE_FRAMES: u64 = 4;
/// The GPU's largest texture side.
pub(super) const MAX_TEXTURE_SIDE: u32 = 4096;

/// A 16-byte aligned GXP program embedded in the executable.
macro_rules! gxp {
    ($name:literal) => {{
        #[repr(C, align(16))]
        struct Aligned<T: ?Sized>(T);
        static DATA: &Aligned<[u8]> = &Aligned(*include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/render/vita/shaders/gxp/",
            $name,
            ".gxp"
        )));
        &DATA.0
    }};
}

/// The embedded programs (`shaders/*.cg`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Program {
    SpriteV,
    SpriteF,
    PlainF,
    SolidV,
    SolidF,
    WipeV,
    WipeF,
    PageV,
    PageF,
    EmoteV,
    EmoteF,
    MeshV,
    MeshF,
    ShadowV,
    ShadowF,
}

impl Program {
    const ALL: [Program; 15] = [
        Program::SpriteV,
        Program::SpriteF,
        Program::PlainF,
        Program::SolidV,
        Program::SolidF,
        Program::WipeV,
        Program::WipeF,
        Program::PageV,
        Program::PageF,
        Program::EmoteV,
        Program::EmoteF,
        Program::MeshV,
        Program::MeshF,
        Program::ShadowV,
        Program::ShadowF,
    ];

    fn binary(self) -> &'static [u8] {
        match self {
            Program::SpriteV => gxp!("sprite_v"),
            Program::SpriteF => gxp!("sprite_f"),
            Program::PlainF => gxp!("plain_f"),
            Program::SolidV => gxp!("solid_v"),
            Program::SolidF => gxp!("solid_f"),
            Program::WipeV => gxp!("wipe_v"),
            Program::WipeF => gxp!("wipe_f"),
            Program::PageV => gxp!("page_v"),
            Program::PageF => gxp!("page_f"),
            Program::EmoteV => gxp!("emote_v"),
            Program::EmoteF => gxp!("emote_f"),
            Program::MeshV => gxp!("mesh_v"),
            Program::MeshF => gxp!("mesh_f"),
            Program::ShadowV => gxp!("shadow_v"),
            Program::ShadowF => gxp!("shadow_f"),
        }
    }

    /// The vertex layout a vertex program reads.
    fn layout(self) -> Option<&'static Layout> {
        match self {
            Program::SpriteV => Some(&SPRITE_LAYOUT),
            Program::SolidV => Some(&SOLID_LAYOUT),
            Program::WipeV => Some(&WIPE_LAYOUT),
            Program::PageV => Some(&PAGE_LAYOUT),
            Program::EmoteV => Some(&EMOTE_LAYOUT),
            Program::MeshV | Program::ShadowV => Some(&MESH_LAYOUT),
            _ => None,
        }
    }

}

/// A vertex attribute: name in the program, float count, byte offset.
struct Layout {
    stride: u16,
    attributes: &'static [(&'static CStr, u8, u16)],
}

/// `render_plan::VertexSprite2dData`.
static SPRITE_LAYOUT: Layout = Layout {
    stride: 64,
    attributes: &[
        (c"pos", 3, 0),
        (c"uv", 2, 12),
        (c"uv_aux", 2, 20),
        (c"alpha", 1, 28),
        (c"world_pos", 4, 32),
        (c"world_normal", 4, 48),
    ],
};

/// `[f32; 4]` clip positions.
static SOLID_LAYOUT: Layout = Layout {
    stride: 16,
    attributes: &[(c"clip_position", 4, 0)],
};

/// `WipeVertex`.
static WIPE_LAYOUT: Layout = Layout {
    stride: 16,
    attributes: &[(c"position", 2, 0), (c"uv", 2, 8)],
};

/// `render_plan::PageWipeVertex`.
static PAGE_LAYOUT: Layout = Layout {
    stride: 24,
    attributes: &[(c"clip_position", 4, 0), (c"uv", 2, 16)],
};

/// `render_plan::emote::EmoteVertex`.
static EMOTE_LAYOUT: Layout = Layout {
    stride: 72,
    attributes: &[
        (c"clip_position", 2, 0),
        (c"model_position", 2, 8),
        (c"texcoord", 2, 16),
        (c"color", 4, 24),
        (c"blend_mode", 1, 40),
        (c"clip_rect", 4, 44),
        (c"wipe", 3, 60),
    ],
};

/// `render_plan::Vertex` (its ten mesh attributes, as `Vertex::ATTRS`).
static MESH_LAYOUT: Layout = Layout {
    stride: 384,
    attributes: &[
        (c"pos", 3, 0),
        (c"uv", 2, 12),
        (c"alpha", 1, 28),
        (c"vertex_color_rgb", 4, 144),
        (c"vertex_color_alpha", 4, 160),
        (c"world_normal", 4, 224),
        (c"world_tangent", 4, 240),
        (c"world_binormal", 4, 256),
        (c"bone_indices", 4, 288),
        (c"bone_weights", 4, 304),
    ],
};

/// A corner of the wipe compositor's full-screen triangle.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct WipeVertex {
    pub(super) position: [f32; 2],
    pub(super) uv: [f32; 2],
}

/// How a fragment program's output meets the target (`SceGxmBlendInfo`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Blend {
    /// Written as is.
    Replace,
    /// Colour writes off (stencil-only draws).
    NoColor,
    /// `SceGxmBlendInfo` bytes: colour mask, colour | alpha function << 4,
    /// colour src | dst << 4, alpha src | dst << 4.
    Info([u8; 4]),
}

// SceGxmBlendFactor / SceGxmBlendFunc values.
const ZERO: u8 = 0;
const ONE: u8 = 1;
const SRC_COLOR: u8 = 2;
const ONE_MINUS_SRC_COLOR: u8 = 3;
const SRC_ALPHA: u8 = 4;
const ONE_MINUS_SRC_ALPHA: u8 = 5;
const DST_COLOR: u8 = 6;
const ONE_MINUS_DST_COLOR: u8 = 7;
const ADD: u8 = 1;
const REVERSE_SUBTRACT: u8 = 3;

const fn blend(func: u8, color: (u8, u8), alpha_func: u8, alpha: (u8, u8)) -> Blend {
    Blend::Info([
        0x0f,
        func | (alpha_func << 4),
        color.0 | (color.1 << 4),
        alpha.0 | (alpha.1 << 4),
    ])
}

impl Blend {
    /// The desktop sprite pipelines' blend states (`ensure_pipeline`).
    pub(super) fn sprite(mode: crate::layer::SpriteBlend) -> Blend {
        use crate::layer::SpriteBlend;
        match mode {
            SpriteBlend::Normal => blend(ADD, (SRC_ALPHA, ONE_MINUS_SRC_ALPHA), ADD, (ONE, ONE)),
            SpriteBlend::Add => blend(ADD, (SRC_ALPHA, ONE), ADD, (ONE, ONE)),
            SpriteBlend::Sub => blend(REVERSE_SUBTRACT, (SRC_ALPHA, ONE), ADD, (ONE, ONE)),
            SpriteBlend::Mul => blend(ADD, (ZERO, SRC_COLOR), ADD, (ONE, ONE)),
            SpriteBlend::Screen => blend(ADD, (ONE, ONE_MINUS_SRC_COLOR), ADD, (ONE, ONE)),
            SpriteBlend::Overlay => blend(
                ADD,
                (ONE, ONE_MINUS_SRC_ALPHA),
                ADD,
                (ONE, ONE_MINUS_SRC_ALPHA),
            ),
        }
    }

    /// wgpu's `BlendState::ALPHA_BLENDING` (page wipes).
    pub(super) const ALPHA: Blend = blend(
        ADD,
        (SRC_ALPHA, ONE_MINUS_SRC_ALPHA),
        ADD,
        (ONE, ONE_MINUS_SRC_ALPHA),
    );

    /// E-mote's native blend modes (`render/emote.rs` `native_blend_state`).
    pub(super) fn emote(mode: usize) -> Blend {
        let alpha = if mode == 0 {
            (ONE, ONE_MINUS_SRC_ALPHA)
        } else {
            (ZERO, ONE)
        };
        match mode {
            1 => blend(ADD, (SRC_ALPHA, ONE), ADD, alpha),
            2 | 5 => blend(REVERSE_SUBTRACT, (SRC_ALPHA, ONE), ADD, alpha),
            3 => blend(ADD, (DST_COLOR, ONE_MINUS_SRC_ALPHA), ADD, alpha),
            4 => blend(ADD, (ONE_MINUS_DST_COLOR, ONE), ADD, alpha),
            _ => blend(ADD, (SRC_ALPHA, ONE_MINUS_SRC_ALPHA), ADD, alpha),
        }
    }
}

/// Depth test and write for a draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Depth {
    pub(super) test: bool,
    pub(super) write: bool,
}

impl Depth {
    pub(super) const OFF: Depth = Depth {
        test: false,
        write: false,
    };
}

/// Stencil use of a draw (E-mote masks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stencil {
    Off,
    /// Passes where the stencil equals the reference; with `op`, the
    /// stencil then changes (masks), else it is kept.
    Equal { reference: u8, op: SceGxmStencilOp },
    /// Writes the reference everywhere drawn (clearing the stencil).
    Replace { reference: u8 },
}

/// Which triangles are dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Cull {
    None,
    /// Counter-clockwise triangles on screen (D3D's `D3DCULL_CCW`: the
    /// desktop's `FrontFace::Cw` with back-face culling).
    CounterClockwise,
    /// Clockwise triangles on screen (`FrontFace::Ccw` with back-face
    /// culling, the page wipe).
    Clockwise,
}

/// A texture as a draw binds it: GXM's texture words with the sampler
/// state already applied.
#[derive(Clone, Copy)]
pub(super) struct Bound(SceGxmTexture);

/// Viewport in target pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Viewport {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) w: f32,
    pub(super) h: f32,
}

/// One draw call.
pub(super) struct Draw<'a> {
    pub(super) vertex: Program,
    pub(super) fragment: Program,
    pub(super) blend: Blend,
    pub(super) depth: Depth,
    pub(super) stencil: Stencil,
    pub(super) cull: Cull,
    pub(super) viewport: Viewport,
    /// Texture units 0.. (None: unbound).
    pub(super) textures: &'a [Option<Bound>],
    /// Vertex data (in the frame's ring) and vertex count.
    pub(super) vertices: GpuSlice,
    pub(super) vertex_count: u32,
    /// Uniform arrays by name, of the vertex and the fragment program.
    pub(super) vertex_uniforms: &'a [(&'static CStr, &'a [f32])],
    pub(super) fragment_uniforms: &'a [(&'static CStr, &'a [f32])],
    /// The vertex program's uniform buffer 0 (bone palettes).
    pub(super) vertex_buffer: Option<GpuSlice>,
}

/// Vertex data placed in this frame's ring.
#[derive(Debug, Clone, Copy)]
pub(super) struct GpuSlice {
    ptr: *const u8,
    len: usize,
}

impl GpuSlice {
    /// The part starting at `offset` bytes.
    pub(super) fn offset(self, offset: usize) -> GpuSlice {
        let offset = offset.min(self.len);
        GpuSlice {
            ptr: unsafe { self.ptr.add(offset) },
            len: self.len - offset,
        }
    }
}

/// Memory mapped for the GPU.
struct GpuBlock {
    uid: SceUID,
    base: *mut u8,
    size: usize,
}

impl GpuBlock {
    fn new(kind: u32, size: usize) -> Option<GpuBlock> {
        let unit = if kind == SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW {
            256 * 1024
        } else {
            4096
        };
        let size = size.div_ceil(unit) * unit;
        unsafe {
            let uid = sceKernelAllocMemBlock(
                c"siglus-gpu".as_ptr(),
                kind,
                size as SceSize,
                std::ptr::null_mut(),
            );
            if uid < 0 {
                return None;
            }
            let mut base = std::ptr::null_mut();
            if sceKernelGetMemBlockBase(uid, &mut base) < 0
                || sceGxmMapMemory(base, size as SceSize, SCE_GXM_MEMORY_ATTRIB_READ) < 0
            {
                sceKernelFreeMemBlock(uid);
                return None;
            }
            Some(GpuBlock {
                uid,
                base: base.cast(),
                size,
            })
        }
    }
}

impl Drop for GpuBlock {
    fn drop(&mut self) {
        unsafe {
            sceGxmUnmapMemory(self.base.cast());
            sceKernelFreeMemBlock(self.uid);
        }
    }
}

/// A registered program.
struct Registered {
    id: SceGxmShaderPatcherId,
    program: *const SceGxmProgram,
}

/// A texture on the GPU (vita2d's allocation).
pub(super) struct Texture {
    handle: *mut Vita2dTexture,
    pub(super) width: u32,
    pub(super) height: u32,
    bytes: usize,
    /// What was uploaded (for `Gpu::texture` callers to compare).
    pub(super) source: (usize, u64),
    pub(super) last_frame: u64,
    render_target: bool,
}

impl Texture {
    pub(super) fn bind(&self, filter: SceGxmTextureFilter) -> Bound {
        let mut words = unsafe { (*self.handle).gxm_tex };
        unsafe {
            sceGxmTextureSetMinFilter(&mut words, filter);
            sceGxmTextureSetMagFilter(&mut words, filter);
            sceGxmTextureSetUAddrMode(&mut words, SCE_GXM_TEXTURE_ADDR_CLAMP);
            sceGxmTextureSetVAddrMode(&mut words, SCE_GXM_TEXTURE_ADDR_CLAMP);
        }
        Bound(words)
    }

    pub(super) fn linear(&self) -> Bound {
        self.bind(SCE_GXM_TEXTURE_FILTER_LINEAR)
    }

    pub(super) fn point(&self) -> Bound {
        self.bind(SCE_GXM_TEXTURE_FILTER_POINT)
    }

    /// The first row's address and the row pitch in bytes (CPU access to a
    /// finished render target).
    pub(super) fn pixels(&self) -> (*const u8, usize) {
        unsafe {
            (
                vita2d_texture_get_datap(self.handle).cast(),
                vita2d_texture_get_stride(self.handle) as usize,
            )
        }
    }
}

/// Counters for the periodic log (see `super::vita_stats`).
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct GpuCounters {
    pub(super) uploads: u64,
    pub(super) upload_bytes: u64,
    pub(super) created: u64,
    pub(super) draws: u64,
    pub(super) scenes: u64,
    pub(super) wait_us: u64,
}

pub(super) struct Gpu {
    ctx: *mut SceGxmContext,
    patcher: *mut SceGxmShaderPatcher,
    programs: HashMap<Program, Registered>,
    /// Uniform parameters by program and name (null: the program has none).
    parameters: HashMap<(Program, &'static CStr), *const SceGxmProgramParameter>,
    vertex_programs: HashMap<Program, *mut SceGxmVertexProgram>,
    fragment_programs: HashMap<(Program, Program, Blend), *mut SceGxmFragmentProgram>,
    rings: Vec<GpuBlock>,
    ring: usize,
    ring_used: usize,
    /// 0, 1, 2, ... for non-indexed draws.
    indices: GpuBlock,
    pub(super) frame: u64,
    in_scene: bool,
    retired: Vec<(u64, Texture)>,
    pub(super) texture_bytes: usize,
    pub(super) counters: GpuCounters,
    /// The fragment program and blend of the last draw (most draws share it).
    current: Option<(Program, Program, Blend)>,
}

/// Indices in one `sceGxmDraw` (u16 indices).
const MAX_DRAW_VERTICES: u32 = 65535 / 3 * 3;

impl Gpu {
    pub(super) fn new() -> Result<Gpu, String> {
        let code = unsafe { vita2d_init_advanced(VITA2D_POOL_BYTES) };
        if code < 0 {
            return Err(format!("vita2d init failed: {code:#x}"));
        }
        let ctx = unsafe { vita2d_get_context() };
        let patcher = unsafe { vita2d_get_shader_patcher() };
        if ctx.is_null() || patcher.is_null() {
            return Err("vita2d has no GXM context".into());
        }
        let mut programs = HashMap::new();
        for program in Program::ALL {
            let binary = program.binary();
            let mut id = std::ptr::null_mut();
            let code = unsafe {
                sceGxmShaderPatcherRegisterProgram(patcher, binary.as_ptr().cast(), &mut id)
            };
            if code < 0 {
                return Err(format!("registering {program:?} failed: {code:#x}"));
            }
            let gxp = unsafe { sceGxmShaderPatcherGetProgramFromId(id) };
            programs.insert(program, Registered { id, program: gxp });
        }
        let rings = (0..VERTEX_RINGS)
            .map(|_| GpuBlock::new(SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE, VERTEX_RING_BYTES))
            .collect::<Option<Vec<_>>>()
            .ok_or("vertex memory allocation failed")?;
        let indices = GpuBlock::new(SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE, 65536 * 2)
            .ok_or("index memory allocation failed")?;
        unsafe {
            let slice = std::slice::from_raw_parts_mut(indices.base.cast::<u16>(), 65536);
            for (i, index) in slice.iter_mut().enumerate() {
                *index = i as u16;
            }
        }
        Ok(Gpu {
            ctx,
            patcher,
            programs,
            parameters: HashMap::new(),
            vertex_programs: HashMap::new(),
            fragment_programs: HashMap::new(),
            rings,
            ring: 0,
            ring_used: 0,
            indices,
            frame: 0,
            in_scene: false,
            retired: Vec::new(),
            texture_bytes: 0,
            counters: GpuCounters::default(),
            current: None,
        })
    }

    /// Starts a frame: the next vertex ring and texture reclamation.
    pub(super) fn begin_frame(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        self.ring = (self.ring + 1) % self.rings.len();
        self.ring_used = 0;
        unsafe { vita2d_pool_reset() };
        let frame = self.frame;
        let mut freed = 0;
        self.retired.retain(|(retired, texture)| {
            let expired = frame.wrapping_sub(*retired) >= RETIRE_FRAMES;
            if expired {
                free_texture(texture);
                freed += texture.bytes;
            }
            !expired
        });
        let _ = freed;
    }

    /// Ends the frame: waits for the GPU (textures may be replaced or freed
    /// from the next frame on) and shows it.
    /// With `dump`, the finished frame is also saved there as a PNG (a
    /// diagnosis aid; Vita3K fills the buffer only with surface sync on).
    pub(super) fn end_frame(&mut self, wait_vsync: bool, dump: Option<&std::path::Path>) {
        if self.in_scene {
            self.end_scene();
        }
        let start = crate::platform_time::Instant::now();
        unsafe { vita2d_wait_rendering_done() };
        self.counters.wait_us += start.elapsed().as_micros() as u64;
        if let Some(path) = dump {
            let fb = unsafe { vita2d_get_current_fb() }.cast::<u8>();
            if !fb.is_null() {
                let (w, h) = (960usize, 544usize);
                let mut rgba = vec![0u8; w * h * 4];
                for y in 0..h {
                    let row = unsafe {
                        std::slice::from_raw_parts(fb.add(y * DISPLAY_STRIDE_PIXELS * 4), w * 4)
                    };
                    rgba[y * w * 4..][..w * 4].copy_from_slice(row);
                }
                for px in rgba.chunks_exact_mut(4) {
                    px[3] = 255;
                }
                let _ = image::save_buffer(path, &rgba, w as u32, h as u32, image::ColorType::Rgba8);
            }
        }
        unsafe {
            vita2d_set_vblank_wait(i32::from(wait_vsync));
            vita2d_swap_buffers();
        }
    }

    /// Waits for all submitted GPU work (a render target's pixels are then
    /// readable).
    pub(super) fn finish(&mut self) {
        if self.in_scene {
            self.end_scene();
        }
        unsafe { vita2d_wait_rendering_done() };
    }

    /// Starts drawing into `target` (None: the display). Every scene waits
    /// for the render targets drawn before it and lets later scenes wait
    /// for it. Depth starts at 1 and stencil at 0.
    pub(super) fn begin_scene(&mut self, target: Option<&Texture>) {
        if self.in_scene {
            self.end_scene();
        }
        let (handle, flags) = match target {
            Some(texture) => (
                texture.handle,
                SCE_GXM_SCENE_FRAGMENT_SET_DEPENDENCY | SCE_GXM_SCENE_VERTEX_WAIT_FOR_DEPENDENCY,
            ),
            None => (std::ptr::null_mut(), SCE_GXM_SCENE_VERTEX_WAIT_FOR_DEPENDENCY),
        };
        unsafe { vita2d_start_drawing_advanced(handle, flags) };
        self.in_scene = true;
        self.current = None;
        self.counters.scenes += 1;
    }

    pub(super) fn end_scene(&mut self) {
        if self.in_scene {
            unsafe { vita2d_end_drawing() };
            self.in_scene = false;
        }
    }

    /// Copies `data` into this frame's vertex ring (None when it is full).
    pub(super) fn upload<T: bytemuck::Pod>(&mut self, data: &[T]) -> Option<GpuSlice> {
        let bytes: &[u8] = bytemuck::cast_slice(data);
        let start = self.ring_used.div_ceil(16) * 16;
        let ring = &self.rings[self.ring];
        if start + bytes.len() > ring.size {
            return None;
        }
        let ptr = unsafe { ring.base.add(start) };
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len()) };
        self.ring_used = start + bytes.len();
        Some(GpuSlice {
            ptr,
            len: bytes.len(),
        })
    }

    fn vertex_program(&mut self, program: Program) -> Option<*mut SceGxmVertexProgram> {
        if let Some(&vp) = self.vertex_programs.get(&program) {
            return Some(vp);
        }
        let registered = self.programs.get(&program)?;
        let layout = program.layout()?;
        let mut attributes = Vec::new();
        for &(name, count, offset) in layout.attributes {
            let param =
                unsafe { sceGxmProgramFindParameterByName(registered.program, name.as_ptr()) };
            if param.is_null() {
                continue;
            }
            let mut attribute: SceGxmVertexAttribute = unsafe { std::mem::zeroed() };
            attribute.streamIndex = 0;
            attribute.offset = offset;
            attribute.format = SCE_GXM_ATTRIBUTE_FORMAT_F32 as u8;
            attribute.componentCount = count;
            attribute.regIndex =
                unsafe { sceGxmProgramParameterGetResourceIndex(param) } as u16;
            attributes.push(attribute);
        }
        let stream = SceGxmVertexStream {
            stride: layout.stride,
            indexSource: SCE_GXM_INDEX_SOURCE_INDEX_16BIT as u16,
        };
        let mut vp = std::ptr::null_mut();
        let code = unsafe {
            sceGxmShaderPatcherCreateVertexProgram(
                self.patcher,
                registered.id,
                attributes.as_ptr(),
                attributes.len() as u32,
                &stream,
                1,
                &mut vp,
            )
        };
        if code < 0 || vp.is_null() {
            log_once(&format!("vertex program {program:?}: {code:#x}"));
            return None;
        }
        self.vertex_programs.insert(program, vp);
        Some(vp)
    }

    fn fragment_program(
        &mut self,
        fragment: Program,
        vertex: Program,
        blend: Blend,
    ) -> Option<*mut SceGxmFragmentProgram> {
        let key = (fragment, vertex, blend);
        if let Some(&fp) = self.fragment_programs.get(&key) {
            return Some(fp);
        }
        let registered = self.programs.get(&fragment)?;
        let vertex_program = self.programs.get(&vertex)?.program;
        let info: Option<SceGxmBlendInfo> = match blend {
            Blend::Replace => None,
            Blend::NoColor => Some(unsafe { std::mem::transmute::<[u8; 4], _>([0, 0, 0, 0]) }),
            Blend::Info(bytes) => Some(unsafe { std::mem::transmute::<[u8; 4], _>(bytes) }),
        };
        let mut fp = std::ptr::null_mut();
        let code = unsafe {
            sceGxmShaderPatcherCreateFragmentProgram(
                self.patcher,
                registered.id,
                SCE_GXM_OUTPUT_REGISTER_FORMAT_UCHAR4,
                SCE_GXM_MULTISAMPLE_NONE,
                info.as_ref().map_or(std::ptr::null(), |info| info),
                vertex_program,
                &mut fp,
            )
        };
        if code < 0 || fp.is_null() {
            log_once(&format!("fragment program {fragment:?} {blend:?}: {code:#x}"));
            return None;
        }
        self.fragment_programs.insert(key, fp);
        Some(fp)
    }

    fn parameter(&mut self, program: Program, name: &'static CStr) -> *const SceGxmProgramParameter {
        *self.parameters.entry((program, name)).or_insert_with(|| {
            let gxp = self.programs[&program].program;
            let param = unsafe { sceGxmProgramFindParameterByName(gxp, name.as_ptr()) };
            if param.is_null() {
                log_once(&format!("{program:?} has no uniform {name:?}"));
            }
            param
        })
    }

    /// Issues one draw with all of its state.
    pub(super) fn draw(&mut self, draw: &Draw) -> bool {
        if !self.in_scene || draw.vertex_count == 0 {
            return false;
        }
        let Some(vp) = self.vertex_program(draw.vertex) else {
            return false;
        };
        let Some(fp) = self.fragment_program(draw.fragment, draw.vertex, draw.blend) else {
            return false;
        };
        let ctx = self.ctx;
        unsafe {
            if self.current != Some((draw.vertex, draw.fragment, draw.blend)) {
                sceGxmSetVertexProgram(ctx, vp);
                sceGxmSetFragmentProgram(ctx, fp);
                self.current = Some((draw.vertex, draw.fragment, draw.blend));
            }
            let v = draw.viewport;
            sceGxmSetViewport(
                ctx,
                v.x + v.w * 0.5,
                v.w * 0.5,
                v.y + v.h * 0.5,
                -v.h * 0.5,
                0.0,
                1.0,
            );
            sceGxmSetViewportEnable(ctx, SCE_GXM_VIEWPORT_ENABLED);
            sceGxmSetFrontDepthFunc(
                ctx,
                if draw.depth.test {
                    SCE_GXM_DEPTH_FUNC_LESS_EQUAL
                } else {
                    SCE_GXM_DEPTH_FUNC_ALWAYS
                },
            );
            sceGxmSetFrontDepthWriteEnable(
                ctx,
                if draw.depth.write {
                    SCE_GXM_DEPTH_WRITE_ENABLED
                } else {
                    SCE_GXM_DEPTH_WRITE_DISABLED
                },
            );
            match draw.stencil {
                Stencil::Off => {
                    sceGxmSetFrontStencilFunc(
                        ctx,
                        SCE_GXM_STENCIL_FUNC_ALWAYS,
                        SCE_GXM_STENCIL_OP_KEEP,
                        SCE_GXM_STENCIL_OP_KEEP,
                        SCE_GXM_STENCIL_OP_KEEP,
                        0xff,
                        0,
                    );
                }
                Stencil::Equal { reference, op } => {
                    sceGxmSetFrontStencilFunc(
                        ctx,
                        SCE_GXM_STENCIL_FUNC_EQUAL,
                        SCE_GXM_STENCIL_OP_KEEP,
                        op,
                        op,
                        0xff,
                        0xff,
                    );
                    sceGxmSetFrontStencilRef(ctx, u32::from(reference));
                }
                Stencil::Replace { reference } => {
                    sceGxmSetFrontStencilFunc(
                        ctx,
                        SCE_GXM_STENCIL_FUNC_ALWAYS,
                        SCE_GXM_STENCIL_OP_REPLACE,
                        SCE_GXM_STENCIL_OP_REPLACE,
                        SCE_GXM_STENCIL_OP_REPLACE,
                        0xff,
                        0xff,
                    );
                    sceGxmSetFrontStencilRef(ctx, u32::from(reference));
                }
            }
            // The viewport flips y (NDC up, target rows down), which turns
            // the triangles' winding around on the target.
            sceGxmSetCullMode(
                ctx,
                match draw.cull {
                    Cull::None => SCE_GXM_CULL_NONE,
                    Cull::CounterClockwise => SCE_GXM_CULL_CCW,
                    Cull::Clockwise => SCE_GXM_CULL_CW,
                },
            );
            sceGxmSetTwoSidedEnable(ctx, SCE_GXM_TWO_SIDED_DISABLED);
            for (unit, texture) in draw.textures.iter().enumerate() {
                if let Some(Bound(words)) = texture {
                    sceGxmSetFragmentTexture(ctx, unit as u32, words);
                }
            }
            for (program, uniforms, vertex) in [
                (draw.vertex, draw.vertex_uniforms, true),
                (draw.fragment, draw.fragment_uniforms, false),
            ] {
                if uniforms.is_empty() {
                    continue;
                }
                let mut buffer = std::ptr::null_mut();
                if vertex {
                    sceGxmReserveVertexDefaultUniformBuffer(ctx, &mut buffer);
                } else {
                    sceGxmReserveFragmentDefaultUniformBuffer(ctx, &mut buffer);
                }
                for &(name, data) in uniforms {
                    let param = self.parameter(program, name);
                    if !param.is_null() && !data.is_empty() {
                        sceGxmSetUniformDataF(buffer, param, 0, data.len() as u32, data.as_ptr());
                    }
                }
            }
            if let Some(buffer) = draw.vertex_buffer {
                sceGxmSetVertexUniformBuffer(ctx, 0, buffer.ptr.cast());
            }
            let stride = draw.vertex.layout().map_or(16, |layout| layout.stride as usize);
            let mut first = 0u32;
            while first < draw.vertex_count {
                let count = (draw.vertex_count - first).min(MAX_DRAW_VERTICES);
                let data = draw.vertices.offset(first as usize * stride);
                if data.len < count as usize * stride {
                    break;
                }
                sceGxmSetVertexStream(ctx, 0, data.ptr.cast());
                sceGxmDraw(
                    ctx,
                    SCE_GXM_PRIMITIVE_TRIANGLES,
                    SCE_GXM_INDEX_FORMAT_U16,
                    self.indices.base.cast(),
                    count,
                );
                first += count;
            }
        }
        self.counters.draws += 1;
        true
    }

    /// A texture holding `pixels` (straight RGBA rows), or None when the
    /// GPU has no memory for it.
    pub(super) fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        pixels: &[u8],
        source: (usize, u64),
    ) -> Option<Texture> {
        let bytes = width as usize * height as usize * 4;
        if width == 0 || height == 0 || pixels.len() < bytes {
            return None;
        }
        let handle = allocate_texture(width, height, false)?;
        self.counters.created += 1;
        let texture = Texture {
            handle,
            width,
            height,
            bytes,
            source,
            last_frame: self.frame,
            render_target: false,
        };
        self.write_texture(&texture, pixels);
        self.texture_bytes += bytes;
        Some(texture)
    }

    /// Rewrites a texture's pixels in place (only while no queued draw of
    /// this frame reads it).
    pub(super) fn write_texture(&mut self, texture: &Texture, pixels: &[u8]) {
        let row = texture.width as usize * 4;
        self.counters.uploads += 1;
        self.counters.upload_bytes += (row * texture.height as usize) as u64;
        unsafe {
            let stride = vita2d_texture_get_stride(texture.handle) as usize;
            let data = vita2d_texture_get_datap(texture.handle).cast::<u8>();
            for y in 0..texture.height as usize {
                std::ptr::copy_nonoverlapping(
                    pixels.as_ptr().add(y * row),
                    data.add(y * stride),
                    row,
                );
            }
        }
    }

    /// A render target (with depth and stencil), in CDRAM when it fits.
    pub(super) fn create_target(&mut self, width: u32, height: u32) -> Option<Texture> {
        let handle = allocate_texture(width, height, true)?;
        let bytes = width as usize * height as usize * 4;
        self.counters.created += 1;
        Some(Texture {
            handle,
            width,
            height,
            bytes,
            source: (0, 0),
            last_frame: self.frame,
            render_target: true,
        })
    }

    /// Frees a texture once no frame in flight can use it.
    pub(super) fn retire(&mut self, texture: Texture) {
        if !texture.render_target {
            self.texture_bytes = self.texture_bytes.saturating_sub(texture.bytes);
        }
        self.retired.push((self.frame, texture));
    }
}

impl Drop for Gpu {
    fn drop(&mut self) {
        unsafe {
            if self.in_scene {
                vita2d_end_drawing();
            }
            vita2d_wait_rendering_done();
        }
        for (_, texture) in self.retired.drain(..) {
            free_texture(&texture);
        }
        unsafe {
            for (_, fp) in self.fragment_programs.drain() {
                sceGxmShaderPatcherReleaseFragmentProgram(self.patcher, fp);
            }
            for (_, vp) in self.vertex_programs.drain() {
                sceGxmShaderPatcherReleaseVertexProgram(self.patcher, vp);
            }
            for (_, registered) in self.programs.drain() {
                sceGxmShaderPatcherUnregisterProgram(self.patcher, registered.id);
            }
        }
        self.rings.clear();
        unsafe { vita2d_fini() };
    }
}

fn allocate_texture(width: u32, height: u32, render_target: bool) -> Option<*mut Vita2dTexture> {
    if width == 0 || height == 0 || width > MAX_TEXTURE_SIDE || height > MAX_TEXTURE_SIDE {
        return None;
    }
    let bytes = width as usize * height as usize * 4;
    let create = |kind: u32| unsafe {
        vita2d_texture_set_alloc_memblock_type(kind);
        if render_target {
            vita2d_create_empty_texture_rendertarget(
                width,
                height,
                SCE_GXM_TEXTURE_FORMAT_U8U8U8U8_ABGR,
            )
        } else {
            vita2d_create_empty_texture_format(width, height, SCE_GXM_TEXTURE_FORMAT_U8U8U8U8_ABGR)
        }
    };
    let mut handle = std::ptr::null_mut();
    if render_target || bytes >= CDRAM_MIN_TEXTURE_BYTES {
        handle = create(SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW);
    }
    if handle.is_null() {
        handle = create(SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE);
    }
    if handle.is_null() {
        log_once(&format!("texture allocation of {width}x{height} failed"));
        return None;
    }
    if !render_target {
        unsafe { clear_render_target_fields(handle) };
    }
    Some(handle)
}

fn free_texture(texture: &Texture) {
    unsafe { vita2d_free_texture(texture.handle) };
}

/// `vita2d_create_empty_texture_format` mallocs its `vita2d_texture`
/// without clearing it and sets only `palette_UID`, but `vita2d_free_texture`
/// also destroys `gxm_rtgt` and frees `depth_UID` when they are non-zero. On
/// reused heap memory those are garbage: freeing a texture then unmapped
/// and freed an unrelated memblock (`sceGxmUnmapMemory` INVALID_POINTER,
/// then a corrupted heap). Offsets are from vita2d.h: SceGxmTexture (16
/// bytes), data_UID @16, palette_UID @20, gxm_rtgt @24, gxm_sfc @28,
/// gxm_sfd @52, depth_UID @96 (100 bytes in all).
unsafe fn clear_render_target_fields(texture: *mut Vita2dTexture) {
    unsafe {
        let base = texture.cast::<u8>();
        base.add(24)
            .cast::<*mut c_void>()
            .write(std::ptr::null_mut());
        base.add(96).cast::<i32>().write(0);
    }
}

/// Logs a GPU problem once per message.
pub(super) fn log_once(message: &str) {
    use std::sync::Mutex;
    static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());
    if let Ok(mut seen) = SEEN.lock() {
        if seen.iter().any(|m| m == message) || seen.len() > 64 {
            return;
        }
        seen.push(message.to_owned());
    }
    if let Ok(text) = CString::new(format!("gpu: {message}\n")) {
        super::report_marker(text.as_bytes_with_nul());
    }
}
