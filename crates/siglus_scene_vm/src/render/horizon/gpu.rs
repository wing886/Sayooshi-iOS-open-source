//! The Switch runtime's deko3d layer (platform/switch/runtime/source/
//! gpu.h), and the render states the desktop pipelines use, in deko3d's
//! enum values.

use std::ffi::{CString, c_char, c_void};

pub(super) const MAX_TEXTURE_UNITS: usize = 8;
pub(super) const MAX_VERTEX_ATTRIBS: usize = 16;
pub(super) const TEXTURE_RENDER_TARGET: u32 = 1;
pub(super) const SAMPLER_LINEAR: u8 = 0;
pub(super) const SAMPLER_POINT: u8 = 1;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct VertexAttrib {
    pub(super) offset: u32,
    pub(super) components: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Uniform {
    pub(super) data: *const c_void,
    pub(super) size: u32,
}

impl Uniform {
    pub(super) const NONE: Uniform = Uniform {
        data: std::ptr::null(),
        size: 0,
    };

    pub(super) fn of<T: bytemuck::Pod>(value: &T) -> Uniform {
        Uniform {
            data: (value as *const T).cast(),
            size: size_of::<T>() as u32,
        }
    }
}

/// `SiglusGpuDraw`.
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct GpuDraw {
    pub(super) vertex_program: u32,
    pub(super) fragment_program: u32,
    pub(super) attribs: [VertexAttrib; MAX_VERTEX_ATTRIBS],
    pub(super) stride: u32,
    pub(super) vertices: *const c_void,
    pub(super) vertex_count: u32,
    pub(super) blend: BlendState,
    pub(super) color_write_mask: u8,
    pub(super) depth_test: u8,
    pub(super) depth_write: u8,
    pub(super) depth_compare: u8,
    pub(super) stencil_enable: u8,
    pub(super) stencil_compare: u8,
    pub(super) stencil_fail: u8,
    pub(super) stencil_depth_fail: u8,
    pub(super) stencil_pass: u8,
    pub(super) stencil_ref: u8,
    pub(super) cull_mode: u8,
    pub(super) front_face: u8,
    pub(super) textures: [i32; MAX_TEXTURE_UNITS],
    pub(super) samplers: [u8; MAX_TEXTURE_UNITS],
    pub(super) vertex_uniforms: [Uniform; 2],
    pub(super) fragment_uniforms: [Uniform; 2],
    pub(super) viewport: [f32; 4],
    pub(super) scissor: [i32; 4],
}

/// The blend fields of `SiglusGpuDraw` (`blend_enable` .. `alpha_dst`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BlendState {
    pub(super) enable: u8,
    pub(super) color_op: u8,
    pub(super) color_src: u8,
    pub(super) color_dst: u8,
    pub(super) alpha_op: u8,
    pub(super) alpha_src: u8,
    pub(super) alpha_dst: u8,
}

// DkBlendOp, DkBlendFactor.
const ADD: u8 = 1;
const REV_SUB: u8 = 3;
const ZERO: u8 = 1;
const ONE: u8 = 2;
const SRC_COLOR: u8 = 3;
const INV_SRC_COLOR: u8 = 4;
const SRC_ALPHA: u8 = 5;
const INV_SRC_ALPHA: u8 = 6;
const DST_COLOR: u8 = 9;
const INV_DST_COLOR: u8 = 10;
// DkCompareOp.
pub(super) const COMPARE_EQUAL: u8 = 3;
pub(super) const COMPARE_LEQUAL: u8 = 4;
pub(super) const COMPARE_ALWAYS: u8 = 8;
// DkStencilOp.
pub(super) const STENCIL_KEEP: u8 = 1;
pub(super) const STENCIL_INCR_WRAP: u8 = 7;
pub(super) const STENCIL_DECR_WRAP: u8 = 8;
// DkFace, DkFrontFace.
pub(super) const FACE_NONE: u8 = 0;
pub(super) const FACE_BACK: u8 = 2;
pub(super) const FRONT_CW: u8 = 0;
pub(super) const FRONT_CCW: u8 = 1;
// DkColorMask.
pub(super) const COLOR_MASK_RGBA: u8 = 0xf;

const fn blend(op: u8, src: u8, dst: u8, alpha_op: u8, alpha_src: u8, alpha_dst: u8) -> BlendState {
    BlendState {
        enable: 1,
        color_op: op,
        color_src: src,
        color_dst: dst,
        alpha_op,
        alpha_src,
        alpha_dst,
    }
}

impl BlendState {
    pub(super) const NONE: BlendState = BlendState {
        enable: 0,
        color_op: ADD,
        color_src: ONE,
        color_dst: ZERO,
        alpha_op: ADD,
        alpha_src: ONE,
        alpha_dst: ZERO,
    };

    /// The desktop sprite pipelines' blend states (`ensure_pipeline`).
    pub(super) fn sprite(mode: crate::layer::SpriteBlend) -> BlendState {
        use crate::layer::SpriteBlend;
        match mode {
            SpriteBlend::Normal => blend(ADD, SRC_ALPHA, INV_SRC_ALPHA, ADD, ONE, ONE),
            SpriteBlend::Add => blend(ADD, SRC_ALPHA, ONE, ADD, ONE, ONE),
            SpriteBlend::Sub => blend(REV_SUB, SRC_ALPHA, ONE, ADD, ONE, ONE),
            SpriteBlend::Mul => blend(ADD, ZERO, SRC_COLOR, ADD, ONE, ONE),
            SpriteBlend::Screen => blend(ADD, ONE, INV_SRC_COLOR, ADD, ONE, ONE),
            SpriteBlend::Overlay => blend(ADD, ONE, INV_SRC_ALPHA, ADD, ONE, INV_SRC_ALPHA),
        }
    }

    /// wgpu's `BlendState::ALPHA_BLENDING` (page wipes).
    pub(super) const ALPHA: BlendState = blend(ADD, SRC_ALPHA, INV_SRC_ALPHA, ADD, ONE, INV_SRC_ALPHA);

    /// E-mote's native modes (`render/emote.rs` `native_blend_state`).
    pub(super) fn emote(mode: usize) -> BlendState {
        let (alpha_src, alpha_dst) = if mode == 0 {
            (ONE, INV_SRC_ALPHA)
        } else {
            (ZERO, ONE)
        };
        match mode {
            1 => blend(ADD, SRC_ALPHA, ONE, ADD, alpha_src, alpha_dst),
            2 | 5 => blend(REV_SUB, SRC_ALPHA, ONE, ADD, alpha_src, alpha_dst),
            3 => blend(ADD, DST_COLOR, INV_SRC_ALPHA, ADD, alpha_src, alpha_dst),
            4 => blend(ADD, INV_DST_COLOR, ONE, ADD, alpha_src, alpha_dst),
            _ => blend(ADD, SRC_ALPHA, INV_SRC_ALPHA, ADD, alpha_src, alpha_dst),
        }
    }
}

impl GpuDraw {
    pub(super) fn new(vertex_program: u32, fragment_program: u32) -> GpuDraw {
        GpuDraw {
            vertex_program,
            fragment_program,
            attribs: [VertexAttrib::default(); MAX_VERTEX_ATTRIBS],
            stride: 4,
            vertices: std::ptr::null(),
            vertex_count: 0,
            blend: BlendState::NONE,
            color_write_mask: COLOR_MASK_RGBA,
            depth_test: 0,
            depth_write: 0,
            depth_compare: COMPARE_ALWAYS,
            stencil_enable: 0,
            stencil_compare: COMPARE_ALWAYS,
            stencil_fail: STENCIL_KEEP,
            stencil_depth_fail: STENCIL_KEEP,
            stencil_pass: STENCIL_KEEP,
            stencil_ref: 0,
            cull_mode: FACE_NONE,
            front_face: FRONT_CW,
            textures: [-1; MAX_TEXTURE_UNITS],
            samplers: [SAMPLER_LINEAR; MAX_TEXTURE_UNITS],
            vertex_uniforms: [Uniform::NONE; 2],
            fragment_uniforms: [Uniform::NONE; 2],
            viewport: [0.0; 4],
            scissor: [0; 4],
        }
    }

    /// Vertex attributes by location: (location, float count, byte offset).
    pub(super) fn layout(&mut self, stride: u32, attributes: &[(usize, u32, u32)]) {
        self.stride = stride;
        for &(location, components, offset) in attributes {
            self.attribs[location] = VertexAttrib { offset, components };
        }
    }

    pub(super) fn vertices<T: bytemuck::Pod>(&mut self, vertices: &[T]) {
        self.vertices = vertices.as_ptr().cast();
        self.vertex_count = vertices.len() as u32;
    }
}

unsafe extern "C" {
    fn siglus_gpu_display_width() -> u32;
    fn siglus_gpu_display_height() -> u32;
    fn siglus_gpu_program(name: *const c_char) -> u32;
    fn siglus_gpu_texture_create(width: u32, height: u32, mip_levels: u32, flags: u32) -> i32;
    fn siglus_gpu_texture_upload(id: i32, level: u32, rgba: *const u8, width: u32, height: u32);
    fn siglus_gpu_texture_destroy(id: i32);
    fn siglus_gpu_texture_read(id: i32, rgba: *mut u8) -> bool;
    fn siglus_gpu_begin_frame() -> bool;
    fn siglus_gpu_begin_pass(target: i32, clear_color: *const f32, clear_depth: bool, clear_stencil: i32);
    fn siglus_gpu_clear_stencil(value: u8);
    fn siglus_gpu_draw(draw: *const GpuDraw);
    fn siglus_gpu_end_frame(present: bool);
}

pub(super) fn display_size() -> (u32, u32) {
    unsafe { (siglus_gpu_display_width(), siglus_gpu_display_height()) }
}

pub(super) fn program(name: &str) -> u32 {
    let name = CString::new(name).expect("program name");
    unsafe { siglus_gpu_program(name.as_ptr()) }
}

/// A texture on the GPU, freed when dropped (after the GPU is done).
#[derive(Debug)]
pub(super) struct Texture {
    pub(super) id: i32,
    pub(super) width: u32,
    pub(super) height: u32,
    /// What was uploaded, to tell when an image changed.
    pub(super) source: (usize, u64),
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { siglus_gpu_texture_destroy(self.id) };
    }
}

impl Texture {
    /// A sampled texture of `image` with its full mip chain.
    pub(super) fn from_image(image: &crate::assets::RgbaImage, source: (usize, u64)) -> Option<Texture> {
        let (width, height) = (image.width.max(1), image.height.max(1));
        if image.rgba.len() < width as usize * height as usize * 4 {
            return None;
        }
        let levels = 32 - width.max(height).leading_zeros();
        let id = unsafe { siglus_gpu_texture_create(width, height, levels, 0) };
        if id < 0 {
            return None;
        }
        let texture = Texture {
            id,
            width,
            height,
            source,
        };
        texture.write(image);
        Some(texture)
    }

    /// Rewrites the pixels (same size) and the mip levels.
    ///
    /// The levels are reduced here rather than blitted on the GPU: the 2D
    /// engine's block-linear mip blits are not reliable under emulation,
    /// and every scaled sprite samples them.
    pub(super) fn write(&self, image: &crate::assets::RgbaImage) {
        let (mut width, mut height) = (self.width, self.height);
        unsafe { siglus_gpu_texture_upload(self.id, 0, image.rgba.as_ptr(), width, height) };
        let mut level = 1;
        let mut above: std::borrow::Cow<'_, [u8]> = std::borrow::Cow::Borrowed(&image.rgba);
        while width > 1 || height > 1 {
            let next = reduce(&above, width, height);
            width = (width / 2).max(1);
            height = (height / 2).max(1);
            unsafe { siglus_gpu_texture_upload(self.id, level, next.as_ptr(), width, height) };
            above = std::borrow::Cow::Owned(next);
            level += 1;
        }
    }

    /// A render target with depth and stencil.
    pub(super) fn target(width: u32, height: u32) -> Option<Texture> {
        let id = unsafe { siglus_gpu_texture_create(width.max(1), height.max(1), 1, TEXTURE_RENDER_TARGET) };
        (id >= 0).then_some(Texture {
            id,
            width: width.max(1),
            height: height.max(1),
            source: (0, 0),
        })
    }

    /// The pixels of a render target, after the GPU finished drawing it.
    pub(super) fn read(&self) -> Option<Vec<u8>> {
        let mut rgba = vec![0u8; self.width as usize * self.height as usize * 4];
        unsafe { siglus_gpu_texture_read(self.id, rgba.as_mut_ptr()) }.then_some(rgba)
    }
}

/// The next mip level of `rgba`: each texel the mean of the 2x2 block
/// above it (the desktop generator's linear sample at the block centre),
/// odd edges clamped.
fn reduce(rgba: &[u8], width: u32, height: u32) -> Vec<u8> {
    let (width, height) = (width as usize, height as usize);
    let (next_width, next_height) = ((width / 2).max(1), (height / 2).max(1));
    let mut out = vec![0u8; next_width * next_height * 4];
    for y in 0..next_height {
        let y0 = (y * 2).min(height - 1);
        let y1 = (y * 2 + 1).min(height - 1);
        for x in 0..next_width {
            let x0 = (x * 2).min(width - 1);
            let x1 = (x * 2 + 1).min(width - 1);
            let texels = [(x0, y0), (x1, y0), (x0, y1), (x1, y1)];
            let dst = (y * next_width + x) * 4;
            for channel in 0..4 {
                let sum: u32 = texels
                    .iter()
                    .map(|&(tx, ty)| rgba[(ty * width + tx) * 4 + channel] as u32)
                    .sum();
                out[dst + channel] = ((sum + 2) / 4) as u8;
            }
        }
    }
    out
}

pub(super) fn begin_frame() -> bool {
    unsafe { siglus_gpu_begin_frame() }
}

/// Starts drawing into `target` (None: the display).
pub(super) fn begin_pass(target: Option<&Texture>, clear_color: Option<[f32; 4]>, clear_depth: bool) {
    let color = clear_color.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
    unsafe { siglus_gpu_begin_pass(target.map_or(-1, |t| t.id), color, clear_depth, -1) };
}

pub(super) fn clear_stencil(value: u8) {
    unsafe { siglus_gpu_clear_stencil(value) };
}

pub(super) fn draw(draw: &GpuDraw) {
    unsafe { siglus_gpu_draw(draw) };
}

pub(super) fn end_frame(present: bool) {
    unsafe { siglus_gpu_end_frame(present) };
}
