@group(0) @binding(0) var sprite_tex: texture_2d<f32>;
@group(0) @binding(1) var sprite_sampler: sampler;

struct VertexIn {
    @location(0) clip_position: vec2<f32>,
    @location(1) model_position: vec2<f32>,
    @location(2) texcoord: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) blend_mode: f32,
    @location(5) clip_rect: vec4<f32>,
    @location(6) wipe: vec3<f32>,
};
struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) model_position: vec2<f32>,
    @location(1) texcoord: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) blend_mode: f32,
    @location(4) clip_rect: vec4<f32>,
    @location(5) wipe: vec3<f32>,
};

@vertex
fn vs_main(input: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(input.clip_position, 0.0, 1.0);
    out.model_position = input.model_position;
    out.texcoord = input.texcoord;
    out.color = input.color;
    out.blend_mode = input.blend_mode;
    out.clip_rect = input.clip_rect;
    out.wipe = input.wipe;
    return out;
}

fn native_texture_stage(input_c: vec4<f32>, blend_mode: u32) -> vec4<f32> {
    var rgb = input_c.rgb;
    let alpha = input_c.a;
    if ((blend_mode & 0xF0u) == 0x10u) {
        rgb = clamp(rgb * 2.0, vec3<f32>(0.0), vec3<f32>(1.0));
    }
    let low = blend_mode & 0xFF0Fu;
    if (low == 3u || low == 4u) {
        rgb = rgb * alpha;
    } else if (low == 5u) {
        rgb = vec3<f32>(1.0) - rgb;
    }
    return vec4<f32>(rgb, alpha);
}

@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32> {
    if (input.model_position.x < input.clip_rect.x || input.model_position.y < input.clip_rect.y ||
        input.model_position.x > input.clip_rect.z || input.model_position.y > input.clip_rect.w) {
        discard;
    }
    var c = textureSample(sprite_tex, sprite_sampler, input.texcoord) * input.color;
    if (input.wipe.z > 0.5) {
        c = vec4<f32>(c.rgb, clamp(c.a * input.wipe.x + input.wipe.y, 0.0, 1.0));
    }
    c = native_texture_stage(c, u32(input.blend_mode + 0.5));
    if (c.a <= 0.003) {
        discard;
    }
    return c;
}
