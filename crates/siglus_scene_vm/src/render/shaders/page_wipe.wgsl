@group(0) @binding(0) var page_tex: texture_2d<f32>;
@group(0) @binding(1) var page_smp: sampler;

struct VsIn {
    @location(0) clip_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
};
struct VsOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};
@vertex
fn vs_main(input: VsIn) -> VsOut {
    var output: VsOut;
    output.position = input.clip_position;
    output.uv = input.uv;
    return output;
}
@fragment
fn fs_main(input: VsOut) -> @location(0) vec4<f32> {
    let color = textureSample(page_tex, page_smp, input.uv);
    if (color.a <= 0.0) { discard; }
    return color;
}
