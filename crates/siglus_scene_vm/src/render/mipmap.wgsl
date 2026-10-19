@group(0) @binding(0) var source: texture_2d<f32>;

@vertex
fn vs_main(@builtin(vertex_index) vertex: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    return vec4<f32>(positions[vertex], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let upper = vec2<i32>(textureDimensions(source)) - vec2<i32>(1);
    let p0 = vec2<i32>(position.xy) * 2;
    let p1 = min(p0 + vec2<i32>(1), upper);
    // Preserve the previous byte-space 2x2 box filter with round-half-up.
    // Rgba8Unorm is deliberately used instead of sRGB, matching the D3D9
    // AUTOGENMIPMAP texture format used by the original renderer.
    let a = vec4<u32>(round(textureLoad(source, p0, 0) * 255.0));
    let b = vec4<u32>(round(textureLoad(source, vec2<i32>(p1.x, p0.y), 0) * 255.0));
    let c = vec4<u32>(round(textureLoad(source, vec2<i32>(p0.x, p1.y), 0) * 255.0));
    let d = vec4<u32>(round(textureLoad(source, p1, 0) * 255.0));
    return vec4<f32>((a + b + c + d + vec4<u32>(2)) / vec4<u32>(4)) / 255.0;
}
