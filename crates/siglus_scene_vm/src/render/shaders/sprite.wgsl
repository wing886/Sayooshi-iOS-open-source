struct VsIn {
  @location(0) pos: vec3<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) alpha: f32,
  @location(3) vertex_color_rgb: vec4<f32>,
  @location(4) vertex_color_alpha: vec4<f32>,
  @location(5) world_normal: vec4<f32>,
  @location(6) world_tangent: vec4<f32>,
  @location(7) world_binormal: vec4<f32>,
  @location(8) bone_indices: vec4<f32>,
  @location(9) bone_weights: vec4<f32>,
};

struct VsIn2d {
  @location(0) pos: vec3<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) uv_aux: vec2<f32>,
  @location(3) alpha: f32,
  @location(4) world_pos: vec4<f32>,
  @location(5) world_normal: vec4<f32>,
};

struct VsOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) alpha: f32,
  @location(2) vertex_color: vec4<f32>,
  @location(3) world_pos: vec4<f32>,
  @location(4) world_normal: vec4<f32>,
  @location(5) world_tangent: vec4<f32>,
  @location(6) world_binormal: vec4<f32>,
  @location(7) shadow_pos: vec4<f32>,
  @location(8) proj_pos: vec4<f32>,
};

struct VsOut2d {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) uv_aux: vec2<f32>,
  @location(2) alpha: f32,
  @location(3) world_pos: vec4<f32>,
  @location(4) world_normal: vec4<f32>,
};

struct ShadowVsOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) depth: f32,
  @location(1) uv: vec2<f32>,
  @location(2) alpha_test: f32,
};

struct VsUniform {
  model_col0: vec4<f32>,
  model_col1: vec4<f32>,
  model_col2: vec4<f32>,
  model_col3: vec4<f32>,
  normal_col0: vec4<f32>,
  normal_col1: vec4<f32>,
  normal_col2: vec4<f32>,
  frame_col0: vec4<f32>,
  frame_col1: vec4<f32>,
  frame_col2: vec4<f32>,
  frame_col3: vec4<f32>,
  frame_normal0: vec4<f32>,
  frame_normal1: vec4<f32>,
  frame_normal2: vec4<f32>,
  camera_eye: vec4<f32>,
  camera_forward: vec4<f32>,
  camera_right: vec4<f32>,
  camera_up: vec4<f32>,
  camera_params: vec4<f32>,
  shadow_eye: vec4<f32>,
  shadow_forward: vec4<f32>,
  shadow_right: vec4<f32>,
  shadow_up: vec4<f32>,
  shadow_params: vec4<f32>,
  mtrl_diffuse: vec4<f32>,
  mtrl_ambient: vec4<f32>,
  mtrl_specular: vec4<f32>,
  mtrl_emissive: vec4<f32>,
  mtrl_params: vec4<f32>,
  mtrl_rim: vec4<f32>,
  mtrl_extra: vec4<f32>,
  light_diffuse_u: vec4<f32>,
  light_ambient_u: vec4<f32>,
  light_specular_u: vec4<f32>,
  sprite_effects: array<vec4<f32>, 11>,
  single_light_pos_kind: vec4<f32>,
  single_light_dir_shadow: vec4<f32>,
  single_light_atten: vec4<f32>,
  single_light_cone: vec4<f32>,
  mesh_flags: vec4<f32>,
  mesh_mrbd: vec4<f32>,
  mesh_rgb_rate: vec4<f32>,
  mesh_add_rgb: vec4<f32>,
  mesh_misc: vec4<f32>,
  mesh_light_counts: vec4<f32>,
  dir_light_diffuse: array<vec4<f32>, 4>,
  dir_light_ambient: array<vec4<f32>, 4>,
  dir_light_specular: array<vec4<f32>, 4>,
  dir_light_dir: array<vec4<f32>, 4>,
  point_light_diffuse: array<vec4<f32>, 4>,
  point_light_ambient: array<vec4<f32>, 4>,
  point_light_specular: array<vec4<f32>, 4>,
  point_light_pos: array<vec4<f32>, 4>,
  point_light_atten: array<vec4<f32>, 4>,
  spot_light_diffuse: array<vec4<f32>, 4>,
  spot_light_ambient: array<vec4<f32>, 4>,
  spot_light_specular: array<vec4<f32>, 4>,
  spot_light_pos: array<vec4<f32>, 4>,
  spot_light_dir: array<vec4<f32>, 4>,
  spot_light_atten: array<vec4<f32>, 4>,
  spot_light_cone: array<vec4<f32>, 4>,
  flags: vec4<f32>,
};

struct BoneUniform {
  matrices: array<mat4x4<f32>, 64>,
};

@group(0) @binding(10) var shadow_tex: texture_2d<f32>;
@group(0) @binding(11) var shadow_smp: sampler;
@group(0) @binding(12) var<uniform> vs_u: VsUniform;
@group(0) @binding(13) var<uniform> bone_u: BoneUniform;

fn apply_model(local: vec3<f32>) -> vec3<f32> {
  return vs_u.model_col0.xyz * local.x + vs_u.model_col1.xyz * local.y + vs_u.model_col2.xyz * local.z + vs_u.model_col3.xyz;
}

fn apply_normal(local: vec3<f32>) -> vec3<f32> {
  let n = vs_u.normal_col0.xyz * local.x + vs_u.normal_col1.xyz * local.y + vs_u.normal_col2.xyz * local.z;
  if (length(n) <= 1e-6) {
    return vec3<f32>(0.0, 0.0, 1.0);
  }
  return normalize(n);
}

fn apply_frame(local: vec3<f32>) -> vec3<f32> {
  return vs_u.frame_col0.xyz * local.x + vs_u.frame_col1.xyz * local.y + vs_u.frame_col2.xyz * local.z + vs_u.frame_col3.xyz;
}

fn apply_frame_normal(local: vec3<f32>) -> vec3<f32> {
  let n = vs_u.frame_normal0.xyz * local.x + vs_u.frame_normal1.xyz * local.y + vs_u.frame_normal2.xyz * local.z;
  if (length(n) <= 1e-6) {
    return vec3<f32>(0.0, 0.0, 1.0);
  }
  return normalize(n);
}

fn apply_bone_point(m: mat4x4<f32>, local: vec3<f32>) -> vec3<f32> {
  return m[0].xyz * local.x + m[1].xyz * local.y + m[2].xyz * local.z + m[3].xyz;
}

fn skin_local(local: vec3<f32>, bone_indices: vec4<f32>, bone_weights: vec4<f32>) -> vec3<f32> {
  let sum_w = bone_weights.x + bone_weights.y + bone_weights.z + bone_weights.w;
  if (vs_u.flags.w <= 0.5 || sum_w <= 1e-6) {
    return apply_frame(local);
  }
  var out = vec3<f32>(0.0, 0.0, 0.0);
  if (bone_weights.x > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.x, 0.0)), 63u)];
    out = out + apply_bone_point(m, local) * bone_weights.x;
  }
  if (bone_weights.y > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.y, 0.0)), 63u)];
    out = out + apply_bone_point(m, local) * bone_weights.y;
  }
  if (bone_weights.z > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.z, 0.0)), 63u)];
    out = out + apply_bone_point(m, local) * bone_weights.z;
  }
  if (bone_weights.w > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.w, 0.0)), 63u)];
    out = out + apply_bone_point(m, local) * bone_weights.w;
  }
  return out;
}

fn skin_normal(local: vec3<f32>, bone_indices: vec4<f32>, bone_weights: vec4<f32>) -> vec3<f32> {
  let sum_w = bone_weights.x + bone_weights.y + bone_weights.z + bone_weights.w;
  if (vs_u.flags.w <= 0.5 || sum_w <= 1e-6) {
    return apply_frame_normal(local);
  }
  var out = vec3<f32>(0.0, 0.0, 0.0);
  if (bone_weights.x > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.x, 0.0)), 63u)];
    out = out + (m[0].xyz * local.x + m[1].xyz * local.y + m[2].xyz * local.z) * bone_weights.x;
  }
  if (bone_weights.y > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.y, 0.0)), 63u)];
    out = out + (m[0].xyz * local.x + m[1].xyz * local.y + m[2].xyz * local.z) * bone_weights.y;
  }
  if (bone_weights.z > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.z, 0.0)), 63u)];
    out = out + (m[0].xyz * local.x + m[1].xyz * local.y + m[2].xyz * local.z) * bone_weights.z;
  }
  if (bone_weights.w > 0.0) {
    let m = bone_u.matrices[min(u32(max(bone_indices.w, 0.0)), 63u)];
    out = out + (m[0].xyz * local.x + m[1].xyz * local.y + m[2].xyz * local.z) * bone_weights.w;
  }
  if (length(out) <= 1e-6) {
    return vec3<f32>(0.0, 0.0, 1.0);
  }
  return normalize(out);
}

fn project_main(world: vec3<f32>) -> vec4<f32> {
  if (vs_u.flags.y > 0.5) {
    let rel = world - vs_u.camera_eye.xyz;
    let cx = dot(rel, vs_u.camera_right.xyz);
    let cy = dot(rel, vs_u.camera_up.xyz);
    let cz = dot(rel, vs_u.camera_forward.xyz);
    let near = 1.0;
    let far = 10000.0;
    let x_clip = cx / max(vs_u.camera_params.x, 1e-3);
    let y_clip = cy / max(vs_u.camera_params.y, 1e-3);
    let z_clip = far / (far - near) * cz - near * far / (far - near);
    // Preserve camera-space Z in clip W.  D3DXMatrixPerspectiveOffCenterLH
    // does this in the original renderer; using pre-divided NDC with W=1
    // makes texture coordinates interpolate affinely across 3D triangles.
    return vec4<f32>(x_clip, y_clip, z_clip, cz);
  }
  let x_ndc = (world.x / max(vs_u.camera_params.z, 1.0)) * 2.0 - 1.0;
  let y_ndc = 1.0 - (world.y / max(vs_u.camera_params.w, 1.0)) * 2.0;
  let z_ndc = clamp(-world.z / 50000.0, 0.0, 1.0);
  return vec4<f32>(x_ndc, y_ndc, z_ndc, 1.0);
}

fn project_shadow(world: vec3<f32>) -> vec4<f32> {
  if (vs_u.shadow_params.z <= 0.5) {
    return vec4<f32>(0.0, 0.0, 1.0, 1.0);
  }
  let rel = world - vs_u.shadow_eye.xyz;
  let cx = dot(rel, vs_u.shadow_right.xyz);
  let cy = dot(rel, vs_u.shadow_up.xyz);
  let cz = dot(rel, vs_u.shadow_forward.xyz);
  if (cz <= 1e-3) {
    return vec4<f32>(0.0, 0.0, 1.0, 1.0);
  }
  let x_ndc = cx / (cz * max(vs_u.shadow_params.x, 1e-3));
  let y_ndc = cy / (cz * max(vs_u.shadow_params.x, 1e-3));
  let depth = clamp(cz / max(vs_u.shadow_params.y, 1.0), 0.0, 1.0);
  return vec4<f32>(x_ndc, y_ndc, depth, 1.0);
}

fn vs_common(v: VsIn) -> VsOut {
  var o: VsOut;
  let local_world = skin_local(v.pos, v.bone_indices, v.bone_weights);
  let local_normal = skin_normal(v.world_normal.xyz, v.bone_indices, v.bone_weights);
  let local_tangent = skin_normal(v.world_tangent.xyz, v.bone_indices, v.bone_weights);
  let local_binormal = skin_normal(v.world_binormal.xyz, v.bone_indices, v.bone_weights);
  let world = apply_model(local_world);
  let normal = apply_normal(local_normal);
  let tangent = apply_normal(local_tangent);
  let binormal = apply_normal(local_binormal);
  o.pos = project_main(world);
  o.proj_pos = o.pos;
  o.uv = v.uv;
  o.alpha = v.alpha;
  o.vertex_color = vec4<f32>(
    v.vertex_color_rgb.xyz,
    v.vertex_color_alpha.x,
  );
  o.world_pos = vec4<f32>(world, 1.0);
  o.world_normal = vec4<f32>(normal, 1.0);
  o.world_tangent = vec4<f32>(tangent, 0.0);
  o.world_binormal = vec4<f32>(binormal, 0.0);
  o.shadow_pos = project_shadow(world);
  return o;
}

fn vs_shadow_common(v: VsIn) -> ShadowVsOut {
  var o: ShadowVsOut;
  let local_world = skin_local(v.pos, v.bone_indices, v.bone_weights);
  let world = apply_model(local_world);
  let shadow = project_shadow(world);
  o.pos = vec4<f32>(shadow.xyz, 1.0);
  o.depth = clamp(shadow.z / max(abs(shadow.w), 1e-6), 0.0, 1.0);
  o.uv = v.uv;
  o.alpha_test = vs_u.sprite_effects[3].y;
  return o;
}

fn vs_common_2d(v: VsIn2d) -> VsOut2d {
  var o: VsOut2d;
  // For CPU-projected 3D quads, restore the original homogeneous W before
  // rasterization. NDC remains unchanged, while UV/world varyings regain the
  // perspective-correct interpolation that tona3 gets from g_mat_view_proj.
  let has_world = v.world_normal.w > 0.5;
  let clip_w = select(1.0, max(v.world_pos.w, 1e-6), has_world);
  o.pos = vec4<f32>(v.pos * clip_w, clip_w);
  o.uv = v.uv;
  o.uv_aux = v.uv_aux;
  o.alpha = v.alpha;
  o.world_pos = v.world_pos;
  o.world_normal = v.world_normal;
  return o;
}

@group(0) @binding(0) var tex0: texture_2d<f32>;
@group(0) @binding(1) var smp0: sampler;
@group(0) @binding(2) var tex1: texture_2d<f32>;
@group(0) @binding(3) var smp1: sampler;
@group(0) @binding(4) var tex2: texture_2d<f32>;
@group(0) @binding(5) var smp2: sampler;
@group(0) @binding(6) var tex3: texture_2d<f32>;
@group(0) @binding(7) var smp3: sampler;
@group(0) @binding(8) var tex4: texture_2d<f32>;
@group(0) @binding(9) var smp4: sampler;
@group(0) @binding(14) var tex5: texture_2d<f32>;
@group(0) @binding(15) var smp5: sampler;
@group(0) @binding(16) var tex6: texture_2d<f32>;
@group(0) @binding(17) var smp6: sampler;
fn sample_mask(uv: vec2<f32>) -> vec4<f32> {
  // `my_sampler_mask` in the original tona3 effect uses CLAMP addressing.
  // The bound wgpu sampler is ClampToEdge as well, so coordinates outside the
  // normalized range must sample the nearest edge texel instead of becoming
  // transparent black.
  return textureSampleLevel(tex1, smp1, uv, 0.0);
}

fn apply_tonecurve_from_mono(color_in: vec3<f32>, mono_y: f32, row: f32, sat: f32) -> vec3<f32> {
  // shader.cfx computes mono_y before tonecurve/reverse and then uses that
  // preserved value for saturation reduction.  CLAMP is supplied by smp2.
  var color = mix(color_in, vec3<f32>(mono_y, mono_y, mono_y), sat);
  let r = textureSampleLevel(tex2, smp2, vec2<f32>(color.r, row), 0.0).r;
  let g = textureSampleLevel(tex2, smp2, vec2<f32>(color.g, row), 0.0).g;
  let b = textureSampleLevel(tex2, smp2, vec2<f32>(color.b, row), 0.0).b;
  return vec3<f32>(r, g, b);
}

fn apply_tonecurve(color_in: vec3<f32>, row: f32, sat: f32) -> vec3<f32> {
  let mono_y = dot(color_in, vec3<f32>(0.2989, 0.5886, 0.1145));
  return apply_tonecurve_from_mono(color_in, mono_y, row, sat);
}

fn sample_tex0_safe(uv: vec2<f32>) -> vec4<f32> {
  if (uv.x < 0.0 || uv.y < 0.0 || uv.x > 1.0 || uv.y > 1.0) {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
  }
  return textureSample(tex0, smp0, uv);
}

fn sample_tex3_safe(uv: vec2<f32>) -> vec4<f32> {
  if (uv.x < 0.0 || uv.y < 0.0 || uv.x > 1.0 || uv.y > 1.0) {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
  }
  return textureSample(tex3, smp3, uv);
}

fn sample_tex4_safe(uv: vec2<f32>) -> vec4<f32> {
  if (uv.x < 0.0 || uv.y < 0.0 || uv.x > 1.0 || uv.y > 1.0) {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
  }
  return textureSample(tex4, smp4, uv);
}

fn sample_mosaic_tex3(uv: vec2<f32>, cut_u: f32, tex_rate_for_square: f32) -> vec4<f32> {
  let cu = max(cut_u, 1e-5);
  let cv = max(cut_u * max(tex_rate_for_square, 1e-5), 1e-5);
  let tc = vec2<f32>(floor(uv.x / cu) * cu, floor(uv.y / cv) * cv);
  return sample_tex3_safe(tc);
}

fn raster_amp(progress: f32) -> f32 {
  let rp = clamp(1.0 - progress, 1e-4, 1.0);
  let lv = max((1.0 - rp) * 100.0, 1e-4);
  return 1.0 - ((log(lv) / log(10.0)) + 1.0) / 3.0;
}

fn sample_raster_h_tex3(uv: vec2<f32>, fraction_num: f32, wave_num: f32, power: f32, progress: f32) -> vec4<f32> {
  let fnn = max(fraction_num, 1.0);
  var tex_coord_for_sin = uv.y * fnn;
  tex_coord_for_sin = fract(tex_coord_for_sin);
  tex_coord_for_sin = (tex_coord_for_sin - fnn * 0.1) / fnn;
  let dx = sin(3.14159265 * progress * power + tex_coord_for_sin * 3.14159265 * wave_num) * raster_amp(progress);
  return sample_tex3_safe(vec2<f32>(uv.x + dx, uv.y));
}

fn sample_raster_v_tex3(uv: vec2<f32>, fraction_num: f32, wave_num: f32, power: f32, progress: f32) -> vec4<f32> {
  let fnn = max(fraction_num, 1.0);
  var tex_coord_for_sin = uv.x * fnn;
  tex_coord_for_sin = fract(tex_coord_for_sin);
  tex_coord_for_sin = (tex_coord_for_sin - fnn * 0.1) / fnn;
  let dy = sin(3.14159265 * progress * power + tex_coord_for_sin * 3.14159265 * wave_num) * raster_amp(progress);
  return sample_tex3_safe(vec2<f32>(uv.x, uv.y + dy));
}

fn sample_explosion_blur_tex3(uv: vec2<f32>, center_uv: vec2<f32>, blur_power: f32, blur_coeff: f32) -> vec4<f32> {
  let dims_u = textureDimensions(tex3, 0);
  let dims = vec2<f32>(f32(dims_u.x), f32(dims_u.y));
  let texel = 1.0 / max(max(dims.x, dims.y), 1.0);
  var dir = center_uv - uv;
  let len = length(dir);
  if (len <= 1e-5 || blur_power <= 1e-5) {
    return sample_tex3_safe(uv);
  }
  dir = normalize(dir) * texel * blur_power * len * max(blur_coeff, 0.0);
  return
      sample_tex3_safe(uv) * 0.19 +
      sample_tex3_safe(uv + dir * 1.0) * 0.17 +
      sample_tex3_safe(uv + dir * 2.0) * 0.15 +
      sample_tex3_safe(uv + dir * 3.0) * 0.13 +
      sample_tex3_safe(uv + dir * 4.0) * 0.11 +
      sample_tex3_safe(uv + dir * 5.0) * 0.09 +
      sample_tex3_safe(uv + dir * 6.0) * 0.07 +
      sample_tex3_safe(uv + dir * 7.0) * 0.05 +
      sample_tex3_safe(uv + dir * 8.0) * 0.03 +
      sample_tex3_safe(uv + dir * 9.0) * 0.01;
}

fn sample_mosaic(uv: vec2<f32>, cut_u: f32, tex_rate_for_square: f32) -> vec4<f32> {
  let cu = max(cut_u, 1e-5);
  let cv = max(cut_u * max(tex_rate_for_square, 1e-5), 1e-5);
  let tc = vec2<f32>(floor(uv.x / cu) * cu, floor(uv.y / cv) * cv);
  return sample_tex0_safe(tc);
}

fn sample_raster_h(uv: vec2<f32>, fraction_num: f32, wave_num: f32, power: f32, progress: f32) -> vec4<f32> {
  let fnn = max(fraction_num, 1.0);
  var tex_coord_for_sin = uv.y * fnn;
  tex_coord_for_sin = fract(tex_coord_for_sin);
  tex_coord_for_sin = (tex_coord_for_sin - fnn * 0.1) / fnn;
  let dx = sin(3.14159265 * progress * power + tex_coord_for_sin * 3.14159265 * wave_num) * raster_amp(progress);
  return sample_tex0_safe(vec2<f32>(uv.x + dx, uv.y));
}

fn sample_raster_v(uv: vec2<f32>, fraction_num: f32, wave_num: f32, power: f32, progress: f32) -> vec4<f32> {
  let fnn = max(fraction_num, 1.0);
  var tex_coord_for_sin = uv.x * fnn;
  tex_coord_for_sin = fract(tex_coord_for_sin);
  tex_coord_for_sin = (tex_coord_for_sin - fnn * 0.1) / fnn;
  let dy = sin(3.14159265 * progress * power + tex_coord_for_sin * 3.14159265 * wave_num) * raster_amp(progress);
  return sample_tex0_safe(vec2<f32>(uv.x, uv.y + dy));
}

fn sample_explosion_blur(uv: vec2<f32>, center_uv: vec2<f32>, blur_power: f32, blur_coeff: f32) -> vec4<f32> {
  let dims_u = textureDimensions(tex0, 0);
  let dims = vec2<f32>(f32(dims_u.x), f32(dims_u.y));
  let texel = 1.0 / max(max(dims.x, dims.y), 1.0);
  var dir = center_uv - uv;
  let len = length(dir);
  if (len <= 1e-5 || blur_power <= 1e-5) {
    return sample_tex0_safe(uv);
  }
  dir = normalize(dir) * texel * blur_power * len * max(blur_coeff, 0.0);
  return
      sample_tex0_safe(uv) * 0.19 +
      sample_tex0_safe(uv + dir * 1.0) * 0.17 +
      sample_tex0_safe(uv + dir * 2.0) * 0.15 +
      sample_tex0_safe(uv + dir * 3.0) * 0.13 +
      sample_tex0_safe(uv + dir * 4.0) * 0.11 +
      sample_tex0_safe(uv + dir * 5.0) * 0.09 +
      sample_tex0_safe(uv + dir * 6.0) * 0.07 +
      sample_tex0_safe(uv + dir * 7.0) * 0.05 +
      sample_tex0_safe(uv + dir * 8.0) * 0.03 +
      sample_tex0_safe(uv + dir * 9.0) * 0.01;
}

fn rgb_brightness(color: vec4<f32>) -> f32 {
  return dot(vec3<f32>(0.2989, 0.5886, 0.1145), color.rgb);
}

fn sample_shimi(uv: vec2<f32>, fade_multiplier: f32, threshold: f32) -> vec4<f32> {
  var color = sample_tex0_safe(uv);
  // tec_tex1_shimi: pixels whose luminance is at or below c1.w have
  // their alpha multiplied by c2.x. RGB is not modified.
  if (rgb_brightness(color) <= threshold) {
    color.a = color.a * fade_multiplier;
  }
  return color;
}

fn sample_shimi_inv(uv: vec2<f32>, fade_multiplier: f32, threshold: f32) -> vec4<f32> {
  var color = sample_tex0_safe(uv);
  // tec_tex1_shimi_inv performs the complementary comparison.
  if (rgb_brightness(color) >= threshold) {
    color.a = color.a * fade_multiplier;
  }
  return color;
}

fn overlay_channel(dst: f32, src: f32) -> f32 {
  if (dst <= 0.5) {
    return 2.0 * dst * src;
  }
  return 1.0 - 2.0 * (1.0 - dst) * (1.0 - src);
}

fn overlay_rgb(dst: vec3<f32>, src: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(
    overlay_channel(dst.r, src.r),
    overlay_channel(dst.g, src.g),
    overlay_channel(dst.b, src.b)
  );
}

fn sample_normal_tex(uv: vec2<f32>) -> vec3<f32> {
  if (uv.x < 0.0 || uv.y < 0.0 || uv.x > 1.0 || uv.y > 1.0) {
    return vec3<f32>(0.5, 0.5, 1.0);
  }
  let dims_u = textureDimensions(tex5, 0);
  if (dims_u.x <= 1u && dims_u.y <= 1u) {
    return vec3<f32>(0.5, 0.5, 1.0);
  }
  return textureSample(tex5, smp5, uv).xyz;
}

fn sample_toon_tex(value: f32) -> vec3<f32> {
  let u = clamp(value, 0.0, 1.0);
  let dims_u = textureDimensions(tex6, 0);
  if (dims_u.x <= 1u && dims_u.y <= 1u) {
    let q = floor(u * 4.0) / 3.0;
    return vec3<f32>(q, q, q);
  }
  return textureSample(tex6, smp6, vec2<f32>(u, 0.5)).rgb;
}

fn apply_parallax_uv(base_n: vec3<f32>, base_t: vec3<f32>, base_b: vec3<f32>, uv: vec2<f32>, view_dir_world: vec3<f32>, max_height: f32) -> vec2<f32> {
  let dims_u = textureDimensions(tex5, 0);
  if (dims_u.x <= 1u && dims_u.y <= 1u || max_height <= 1e-6) {
    return uv;
  }
  let N = normalize(base_n);
  var T = normalize(base_t);
  var B = normalize(base_b);
  if (length(T) <= 1e-5 || length(B) <= 1e-5) {
    let up = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(0.0, 1.0, 0.0), abs(N.z) > 0.9);
    T = normalize(cross(up, N));
    B = normalize(cross(N, T));
  }
  let Vt = vec3<f32>(dot(view_dir_world, T), dot(view_dir_world, B), dot(view_dir_world, N));
  let height = textureSample(tex5, smp5, uv).a;
  let denom = select(-1e-4, Vt.z, abs(Vt.z) > 1e-4);
  let shift = (height - 0.5) * max_height;
  return uv + (Vt.xy / denom) * shift;
}

fn apply_normal_map(base_n: vec3<f32>, base_t: vec3<f32>, base_b: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
  let tex_n = sample_normal_tex(uv) * 2.0 - vec3<f32>(1.0, 1.0, 1.0);
  let N = normalize(base_n);
  var T = normalize(base_t);
  var B = normalize(base_b);
  if (length(T) <= 1e-5 || length(B) <= 1e-5) {
    let up = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(0.0, 1.0, 0.0), abs(N.z) > 0.9);
    T = normalize(cross(up, N));
    B = normalize(cross(N, T));
  }
  let mapped = normalize(T * tex_n.x + B * tex_n.y + N * tex_n.z);
  return mapped;
}

fn sample_shadow_visibility(shadow_pos: vec4<f32>) -> f32 {
  let ndc = shadow_pos.xyz / max(abs(shadow_pos.w), 1e-5);
  let uv = vec2<f32>(ndc.x * 0.5 + 0.5, 1.0 - (ndc.y * 0.5 + 0.5));
  // The tona3 shadow sampler is POINT with a white BORDER. Emulate the
  // border explicitly because portable WebGPU border samplers are limited.
  if (uv.x < 0.0 || uv.y < 0.0 || uv.x > 1.0 || uv.y > 1.0) {
    return 1.0;
  }
  let current = clamp(ndc.z, 0.0, 1.0);
  let stored = textureSampleLevel(shadow_tex, shadow_smp, uv, 0.0).r;
  let bias = max(vs_u.mesh_misc.y, 0.0);
  // Original code: shadow * Depth.w < Depth.z - bias.
  return select(0.0, 1.0, current - bias <= stored);
}

fn mesh_light_contrib(
  base_rgb: vec3<f32>,
  world_pos: vec3<f32>,
  N: vec3<f32>,
  shaded_uv: vec2<f32>,
  light_diffuse: vec3<f32>,
  light_ambient: vec3<f32>,
  light_specular: vec3<f32>,
  kind: i32,
  light_pos: vec3<f32>,
  light_dir: vec3<f32>,
  light_atten: vec4<f32>,
  light_cone: vec4<f32>,
  shadow_pos: vec4<f32>,
  shadow_enabled: bool
) -> vec3<f32> {
  let lighting_type = i32(round(vs_u.mtrl_params.y));
  let shading_type = i32(round(vs_u.mtrl_params.z));
  let mtrl_ambient = vs_u.mtrl_ambient.rgb;
  let mtrl_specular = vs_u.mtrl_specular.rgb;
  let mtrl_power = max(vs_u.mtrl_params.x, 1.0);

  var L = normalize(-light_dir);
  var distance_attenuation = 1.0;
  var spot_power = 1.0;
  if (kind != 0) {
    let dir_point = light_pos - world_pos;
    let distance_point = max(length(dir_point), 1e-5);
    L = dir_point / distance_point;
    distance_attenuation = 1.0 / max(
      light_atten.x + light_atten.y * distance_point + light_atten.z * distance_point * distance_point,
      1e-5
    );
    if (light_atten.w > 0.0) {
      distance_attenuation = distance_attenuation * clamp(1.0 - distance_point / light_atten.w, 0.0, 1.0);
    }
    if (kind >= 2) {
      let rho = dot(L, normalize(-light_dir));
      if (rho >= light_cone.x) {
        spot_power = 1.0;
      } else if (rho <= light_cone.y) {
        spot_power = 0.0;
      } else {
        spot_power = pow(
          (rho - light_cone.y) / max(light_cone.x - light_cone.y, 1e-5),
          max(light_cone.z, 0.01)
        );
      }
    }
  }

  let V = normalize(vs_u.camera_eye.xyz - world_pos);
  let H = normalize(L + V);
  let ndotl_raw = dot(N, L);
  let ndotl = max(ndotl_raw, 0.0);
  // tona3 half-Lambert squares the remapped term.
  let half_lambert = pow(clamp(ndotl_raw * 0.5 + 0.5, 0.0, 1.0), 2.0);
  let ndoth = max(dot(N, H), 0.0);
  let rdotv = max(dot(reflect(-L, N), V), 0.0);

  var visibility = 1.0;
  if (shadow_enabled && (shading_type == 1 || kind == 3)) {
    visibility = sample_shadow_visibility(shadow_pos);
  }

  let ambient_term = base_rgb * mtrl_ambient * light_ambient;
  var diffuse_strength = ndotl;
  if (lighting_type == 4) {
    diffuse_strength = half_lambert;
  }

  var diffuse_term = base_rgb * light_diffuse * diffuse_strength * distance_attenuation * spot_power;
  if (lighting_type == 5) {
    // Original toon coordinate is 0.0001 + mean RGB light brightness.
    let lbrightness = light_ambient + light_diffuse * diffuse_strength * distance_attenuation * spot_power;
    let toon = 0.0001 + (lbrightness.x + lbrightness.y + lbrightness.z) * 0.333;
    diffuse_term = base_rgb * sample_toon_tex(toon);
  }

  var specular_strength = pow(ndoth, mtrl_power);
  if (lighting_type == 6 || lighting_type == 7) {
    specular_strength = pow(rdotv, mtrl_power);
  }
  if (lighting_type == 0 || lighting_type == 1 || lighting_type == 4 || lighting_type == 5) {
    specular_strength = 0.0;
  }

  // The per-pixel FFP generator deliberately omits SpotPower from the
  // specular accumulation; vertex FFP includes it.
  var specular_spot = spot_power;
  if (lighting_type == 7) {
    specular_spot = 1.0;
  }
  let specular_term = mtrl_specular * light_specular * specular_strength * distance_attenuation * specular_spot;
  return ambient_term + (diffuse_term + specular_term) * visibility;
}

fn mesh_lighting(
  base_rgb: vec3<f32>,
  world_pos: vec3<f32>,
  world_normal: vec3<f32>,
  world_tangent: vec3<f32>,
  world_binormal: vec3<f32>,
  shaded_uv: vec2<f32>,
  shadow_pos: vec4<f32>
) -> vec3<f32> {
  let lighting_type = i32(round(vs_u.mtrl_params.y));
  let rim_power = max(vs_u.mtrl_params.w, 0.0);
  var N = normalize(world_normal);
  if (lighting_type == 8 || lighting_type == 9) {
    N = apply_normal_map(N, world_tangent, world_binormal, shaded_uv);
  }

  var accum = vs_u.mtrl_emissive.rgb;
  let dir_count = i32(round(vs_u.mesh_light_counts.x));
  let point_count = i32(round(vs_u.mesh_light_counts.y));
  let spot_count = i32(round(vs_u.mesh_light_counts.z));
  if (dir_count + point_count + spot_count > 0) {
    for (var li: i32 = 0; li < 4; li = li + 1) {
      if (li < dir_count) {
        accum = accum + mesh_light_contrib(
          base_rgb, world_pos, N, shaded_uv,
          vs_u.dir_light_diffuse[li].rgb, vs_u.dir_light_ambient[li].rgb, vs_u.dir_light_specular[li].rgb,
          0, vec3<f32>(0.0), vs_u.dir_light_dir[li].xyz,
          vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0), shadow_pos, false
        );
      }
      if (li < point_count) {
        accum = accum + mesh_light_contrib(
          base_rgb, world_pos, N, shaded_uv,
          vs_u.point_light_diffuse[li].rgb, vs_u.point_light_ambient[li].rgb, vs_u.point_light_specular[li].rgb,
          1, vs_u.point_light_pos[li].xyz, vec3<f32>(0.0, 0.0, -1.0),
          vs_u.point_light_atten[li], vec4<f32>(0.0), shadow_pos, false
        );
      }
      if (li < spot_count) {
        let receives_shadow = vs_u.spot_light_cone[li].w > 0.5;
        accum = accum + mesh_light_contrib(
          base_rgb, world_pos, N, shaded_uv,
          vs_u.spot_light_diffuse[li].rgb, vs_u.spot_light_ambient[li].rgb, vs_u.spot_light_specular[li].rgb,
          select(2, 3, receives_shadow), vs_u.spot_light_pos[li].xyz, vs_u.spot_light_dir[li].xyz,
          vs_u.spot_light_atten[li], vs_u.spot_light_cone[li], shadow_pos, receives_shadow
        );
      }
    }
  } else {
    let kind = i32(round(vs_u.single_light_pos_kind.w));
    accum = accum + mesh_light_contrib(
      base_rgb, world_pos, N, shaded_uv,
      vs_u.light_diffuse_u.rgb, vs_u.light_ambient_u.rgb, vs_u.light_specular_u.rgb,
      kind, vs_u.single_light_pos_kind.xyz, vs_u.single_light_dir_shadow.xyz,
      vs_u.single_light_atten, vs_u.single_light_cone, shadow_pos,
      kind == 3 && vs_u.single_light_dir_shadow.w > 0.5
    );
  }

  let shader_option_bits = i32(round(vs_u.mtrl_extra.z));
  if (rim_power > 0.0 && (shader_option_bits & 1) != 0) {
    let V = normalize(vs_u.camera_eye.xyz - world_pos);
    let rim = pow(clamp(1.0 - max(dot(N, V), 0.0), 0.0, 1.0), max(rim_power, 1e-3));
    accum = accum + vs_u.mtrl_rim.rgb * rim;
  }
  return accum;
}

fn fs_common_2d(i: VsOut2d) -> vec4<f32> {
  let e1 = vs_u.sprite_effects[0];
  let e2 = vs_u.sprite_effects[1];
  let e3 = vs_u.sprite_effects[2];
  let e4 = vs_u.sprite_effects[3];
  let e5 = vs_u.sprite_effects[4];
  let e6 = vs_u.sprite_effects[5];
  let e7 = vs_u.sprite_effects[6];
  let e8 = vs_u.sprite_effects[7];
  let e9 = vs_u.sprite_effects[8];
  let e10 = vs_u.sprite_effects[9];
  let e11 = vs_u.sprite_effects[10];

  let tr = e1.x;
  let mono = e1.y;
  let rev = e1.z;
  let bright = e1.w;
  let dark = e2.x;
  let color_rate = e2.y;
  let color_add = vec3<f32>(e2.z, e2.w, e3.x);
  let color_tgt = e3.yzw;
  let mask_mode = e4.x;
  let alpha_test = e4.y;
  let light_on = e4.z;
  let fog_on = e4.w;
  let has_mask = e5.x;
  let has_tonecurve = e5.y;
  let tonecurve_row = e5.z;
  let tonecurve_sat = e5.w;
  let wipe_mode = e6.x;
  let wipe_p0 = e6.y;
  let wipe_p1 = e6.z;
  let wipe_p2 = e6.w;
  let wipe_p3 = e7.x;
  let has_wipe_src = e7.y;
  let blend_code = e7.z;
  let wipe_aux1 = e7.w;
  let light_factor = e8.w;
  let fog_scroll_x = e9.w;
  let fog_color_fallback = vec4<f32>(e10.xyz, 1.0);
  let sprite_z = e10.w;
  let fog_near = e11.x;
  let fog_far = e11.y;
  let has_fog_tex = e11.z;
  let camera_z = e11.w;
  let alpha_ref = 1.0 / 255.0;

  var c = textureSample(tex0, smp0, i.uv);
  if (wipe_mode > 0.5 && wipe_mode < 1.5) {
    c = sample_mosaic(i.uv, wipe_p0, wipe_p1);
    c.a = 1.0;
  } else if (wipe_mode > 1.5 && wipe_mode < 2.5) {
    c = sample_raster_h(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3);
  } else if (wipe_mode > 2.5 && wipe_mode < 3.5) {
    c = sample_raster_v(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3);
  } else if (wipe_mode > 3.5 && wipe_mode < 4.5) {
    c = sample_explosion_blur(i.uv, vec2<f32>(wipe_p0, wipe_p1), wipe_p2, wipe_p3);
    c.a = 1.0;
  } else if (wipe_mode > 4.5 && wipe_mode < 5.5) {
    c = sample_shimi(i.uv, wipe_p0, wipe_p1);
  } else if (wipe_mode > 5.5 && wipe_mode < 6.5) {
    c = sample_shimi_inv(i.uv, wipe_p0, wipe_p1);
  } else if (wipe_mode > 9.5 && wipe_mode < 10.5 && has_wipe_src > 0.5) {
    let oldc = sample_mosaic_tex3(i.uv, wipe_p0, wipe_p1);
    let newc = sample_mosaic(i.uv, wipe_p0, wipe_p1);
    if (wipe_p3 < 230.5) {
      c = select(oldc, newc, wipe_p2 >= 0.5);
    } else {
      c = mix(select(newc, oldc, wipe_aux1 < 0.5), select(oldc, newc, wipe_aux1 < 0.5), clamp(wipe_p2, 0.0, 1.0));
    }
  } else if (wipe_mode > 10.5 && wipe_mode < 11.5 && has_wipe_src > 0.5) {
    c = mix(
      sample_raster_h_tex3(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3),
      sample_raster_h(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3),
      clamp(wipe_p3, 0.0, 1.0)
    );
  } else if (wipe_mode > 11.5 && wipe_mode < 12.5 && has_wipe_src > 0.5) {
    c = mix(
      sample_raster_v_tex3(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3),
      sample_raster_v(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3),
      clamp(wipe_p3, 0.0, 1.0)
    );
  } else if (wipe_mode > 12.5 && wipe_mode < 13.5 && has_wipe_src > 0.5) {
    c = mix(
      sample_explosion_blur_tex3(i.uv, vec2<f32>(wipe_p0, wipe_p1), wipe_p2, wipe_p3),
      sample_explosion_blur(i.uv, vec2<f32>(wipe_p0, wipe_p1), wipe_p2, wipe_p3),
      clamp(tonecurve_row, 0.0, 1.0)
    );
    c.a = 1.0;
  }

  var color = c * vec4<f32>(1.0, 1.0, 1.0, i.alpha * tr);
  let color_org = color;

  // tona3's d3-rect/PCT v2 effect performs lighting per pixel from the
  // interpolated world position and normal. Keep the CPU light_factor only as
  // a fallback for non-world-space sprite paths.
  let world_has_pos = i.world_normal.w > 0.5;
  if (light_on > 0.5) {
    if (world_has_pos && length(i.world_normal.xyz) > 0.25) {
      let normal = normalize(i.world_normal.xyz);
      let dir_point = vs_u.single_light_pos_kind.xyz - i.world_pos.xyz;
      let distance_point = length(dir_point);
      let light_dir = dir_point / max(distance_point, 1e-6);
      var light_power = dot(normal, light_dir);
      light_power = light_power * (1.0 - distance_point / 2000.0);
      light_power = clamp(light_power, 0.0, 1.0);
      color = color * vec4<f32>(e9.xyz, 1.0) * light_power;
    } else {
      color = color * vec4<f32>(e9.xyz, 1.0) * light_factor;
    }
  }

  if (fog_on > 0.5) {
    var depth = abs(sprite_z - camera_z);
    if (world_has_pos) {
      depth = length(vs_u.camera_eye.xyz - i.world_pos.xyz);
    }
    let fog_t = clamp((depth - fog_near) / max(fog_far - fog_near, 1e-5), 0.0, 1.0);
    if (fog_t > 0.0) {
      var fog_color = fog_color_fallback;
      if (has_fog_tex > 0.5) {
        let dims_u = textureDimensions(tex4, 0);
        let tw = max(f32(dims_u.x), 1.0);
        let th = max(f32(dims_u.y), 1.0);
        let vw = max(vs_u.camera_params.z, 1.0);
        let vh = max(vs_u.camera_params.w, 1.0);
        let aspect = th / vh;
        let fog_w = vw / tw * aspect;
        let fog_h = vh / th;
        let fog_x = -fog_scroll_x / tw * aspect - 0.5 / vw;
        let fog_y = 0.5 / vh;
        let proj01 = vec2<f32>(i.pos.x / vw, i.pos.y / vh);
        let fog_base = vec2<f32>(proj01.x * fog_w + fog_x, proj01.y);
        let fog_uv = fog_base * fog_h + vec2<f32>(fog_y);
        fog_color = textureSampleLevel(tex4, smp4, fog_uv, 0.0);
      }
      color = mix(color, fog_color, fog_t);
    }
  }

  let mono_y = dot(color.rgb, vec3<f32>(0.2989, 0.5886, 0.1145));
  if (has_tonecurve > 0.5) {
    color = vec4<f32>(apply_tonecurve_from_mono(color.rgb, mono_y, tonecurve_row, tonecurve_sat), color.a);
  }
  color = vec4<f32>(mix(color.rgb, vec3<f32>(1.0) - color.rgb, rev), color.a);
  color = vec4<f32>(mix(color.rgb, vec3<f32>(mono_y), mono), color.a);
  color = vec4<f32>(color.rgb + vec3<f32>(bright), color.a);
  color = vec4<f32>(color.rgb - vec3<f32>(dark), color.a);
  color = vec4<f32>(mix(color.rgb, color_tgt, color_rate), color.a);
  color = vec4<f32>(color.rgb + color_add, color.a);

  if (blend_code > 2.5 && blend_code < 3.5) {
    color = mix(vec4<f32>(1.0), color, color_org.a);
  } else if (blend_code > 3.5 && blend_code < 4.5) {
    color = mix(vec4<f32>(0.0), color, color_org.a);
  }
  color.a = color_org.a;

  let final_gray = dot(color.rgb, vec3<f32>(0.2989, 0.5886, 0.1145));
  if (has_mask > 0.5) {
    color = color * sample_mask(i.uv_aux);
  }
  if (mask_mode > 0.5 && mask_mode < 1.5) {
    color.a = final_gray;
  }
  if (alpha_test > 0.5 && color.a < alpha_ref) {
    discard;
  }

  if (blend_code > 4.5 && blend_code < 5.5) {
    let dims_u = textureDimensions(tex3, 0);
    let screen_uv = vec2<f32>(
      clamp(i.pos.x / max(f32(dims_u.x), 1.0), 0.0, 1.0),
      clamp(i.pos.y / max(f32(dims_u.y), 1.0), 0.0, 1.0)
    );
    let dst = sample_tex3_safe(screen_uv);
    let ov = overlay_rgb(dst.rgb, color.rgb);
    return vec4<f32>(mix(dst.rgb, ov, color.a), 1.0);
  }
  return color;
}

fn fs_common(i: VsOut) -> vec4<f32> {
  let e1 = vs_u.sprite_effects[0];
  let e2 = vs_u.sprite_effects[1];
  let e3 = vs_u.sprite_effects[2];
  let e4 = vs_u.sprite_effects[3];
  let e5 = vs_u.sprite_effects[4];
  let e6 = vs_u.sprite_effects[5];
  let e7 = vs_u.sprite_effects[6];
  let e8 = vs_u.sprite_effects[7];
  let e9 = vs_u.sprite_effects[8];
  let e10 = vs_u.sprite_effects[9];
  let e11 = vs_u.sprite_effects[10];

  let tr = e1.x;
  let mono = e1.y;
  let rev = e1.z;
  let bright = e1.w;
  let dark = e2.x;
  let color_rate = e2.y;
  let color_add = vec3<f32>(e2.z, e2.w, e3.x);
  let color_tgt = e3.yzw;
  let mask_mode = e4.x;
  let alpha_test = e4.y;
  let light_on = e4.z;
  let fog_on = e4.w;
  let has_mask = e5.x;
  let has_tonecurve = e5.y;
  let tonecurve_row = e5.z;
  let tonecurve_sat = e5.w;
  let wipe_mode = e6.x;
  let wipe_p0 = e6.y;
  let wipe_p1 = e6.z;
  let wipe_p2 = e6.w;
  let wipe_p3 = e7.x;
  let has_wipe_src = e7.y;
  let blend_code = e7.z;
  let wipe_aux1 = e7.w;
  let light_factor = e8.w;
  let fog_scroll_x = e9.w;
  let fog_color_fallback = vec4<f32>(e10.xyz, 1.0);
  let fog_near = e11.x;
  let fog_far = e11.y;
  let has_fog_tex = e11.z;
  let alpha_ref = max(vs_u.mtrl_extra.y, 1.0 / 255.0);

  let world_pos = i.world_pos.xyz;
  let world_has_pos = i.world_pos.w > 0.5;
  let world_normal = i.world_normal.xyz;
  let world_tangent = i.world_tangent.xyz;
  let world_binormal = i.world_binormal.xyz;
  let mesh_pipeline = vs_u.flags.x > 0.5;
  let mesh_use_tex = vs_u.mesh_flags.x > 0.5;
  let mesh_use_mrbd = vs_u.mesh_flags.y > 0.5;
  let mesh_use_rgb = vs_u.mesh_flags.z > 0.5;
  let mesh_use_vertex_color = vs_u.mesh_flags.w > 0.5;

  var shaded_uv = i.uv;
  if (mesh_pipeline && world_has_pos && length(world_normal) > 0.25 && i32(round(vs_u.mtrl_params.y)) == 9) {
    let view_dir_world = normalize(vs_u.camera_eye.xyz - world_pos);
    shaded_uv = apply_parallax_uv(
      world_normal, world_tangent, world_binormal, i.uv, view_dir_world, max(vs_u.mtrl_extra.x, 0.0)
    );
  }

  var c = select(vec4<f32>(1.0), textureSample(tex0, smp0, shaded_uv), mesh_use_tex);
  if (wipe_mode > 0.5 && wipe_mode < 1.5) {
    c = sample_mosaic(i.uv, wipe_p0, wipe_p1);
    c.a = 1.0;
  } else if (wipe_mode > 1.5 && wipe_mode < 2.5) {
    c = sample_raster_h(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3);
  } else if (wipe_mode > 2.5 && wipe_mode < 3.5) {
    c = sample_raster_v(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3);
  } else if (wipe_mode > 3.5 && wipe_mode < 4.5) {
    c = sample_explosion_blur(i.uv, vec2<f32>(wipe_p0, wipe_p1), wipe_p2, wipe_p3);
    c.a = 1.0;
  } else if (wipe_mode > 4.5 && wipe_mode < 5.5) {
    c = sample_shimi(i.uv, wipe_p0, wipe_p1);
  } else if (wipe_mode > 5.5 && wipe_mode < 6.5) {
    c = sample_shimi_inv(i.uv, wipe_p0, wipe_p1);
  } else if (wipe_mode > 9.5 && wipe_mode < 10.5 && has_wipe_src > 0.5) {
    let oldc = sample_mosaic_tex3(i.uv, wipe_p0, wipe_p1);
    let newc = sample_mosaic(i.uv, wipe_p0, wipe_p1);
    if (wipe_p3 < 230.5) {
      c = select(oldc, newc, wipe_p2 >= 0.5);
    } else {
      c = mix(select(newc, oldc, wipe_aux1 < 0.5), select(oldc, newc, wipe_aux1 < 0.5), clamp(wipe_p2, 0.0, 1.0));
    }
  } else if (wipe_mode > 10.5 && wipe_mode < 11.5 && has_wipe_src > 0.5) {
    c = mix(sample_raster_h_tex3(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3), sample_raster_h(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3), clamp(wipe_p3, 0.0, 1.0));
  } else if (wipe_mode > 11.5 && wipe_mode < 12.5 && has_wipe_src > 0.5) {
    c = mix(sample_raster_v_tex3(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3), sample_raster_v(i.uv, wipe_p0, wipe_p1, wipe_p2, wipe_p3), clamp(wipe_p3, 0.0, 1.0));
  } else if (wipe_mode > 12.5 && wipe_mode < 13.5 && has_wipe_src > 0.5) {
    c = mix(sample_explosion_blur_tex3(i.uv, vec2<f32>(wipe_p0, wipe_p1), wipe_p2, wipe_p3), sample_explosion_blur(i.uv, vec2<f32>(wipe_p0, wipe_p1), wipe_p2, wipe_p3), clamp(tonecurve_row, 0.0, 1.0));
    c.a = 1.0;
  }

  var color = c * vec4<f32>(1.0, 1.0, 1.0, i.alpha * tr);
  if (mesh_pipeline) {
    color = color * vs_u.mtrl_diffuse;
    if (mesh_use_vertex_color) {
      color = color * mix(vec4<f32>(1.0), i.vertex_color, clamp(vs_u.mesh_misc.x, 0.0, 1.0));
    }
  }
  let color_org = color;

  if (light_on > 0.5) {
    if (mesh_pipeline && world_has_pos && length(world_normal) > 0.25) {
      color = vec4<f32>(
        mesh_lighting(
          color.rgb, world_pos, world_normal, world_tangent, world_binormal, shaded_uv, i.shadow_pos
        ),
        color.a
      );
    } else {
      color = color * vec4<f32>(e9.xyz, 1.0) * light_factor;
    }
  } else if (mesh_pipeline) {
    color = vec4<f32>(color.rgb + vs_u.mtrl_emissive.rgb, color.a);
  }

  if (fog_on > 0.5) {
    var depth = abs(e10.w - e11.w);
    if (world_has_pos) {
      depth = length(vs_u.camera_eye.xyz - world_pos);
    }
    let fog_t = clamp((depth - fog_near) / max(fog_far - fog_near, 1e-5), 0.0, 1.0);
    if (fog_t > 0.0) {
      var fog_color = fog_color_fallback;
      if (has_fog_tex > 0.5) {
        let dims_u = textureDimensions(tex4, 0);
        let tw = max(f32(dims_u.x), 1.0);
        let th = max(f32(dims_u.y), 1.0);
        let vw = max(vs_u.camera_params.z, 1.0);
        let vh = max(vs_u.camera_params.w, 1.0);
        let aspect = th / vh;
        let fog_w = vw / tw * aspect;
        let fog_h = vh / th;
        let fog_x = -fog_scroll_x / tw * aspect - 0.5 / vw;
        let fog_y = 0.5 / vh;
        let ndc = i.proj_pos.xy / max(abs(i.proj_pos.w), 1e-5);
        let fog_base = vec2<f32>((ndc.x + 1.0) * 0.5 * fog_w + fog_x, 1.0 - (ndc.y + 1.0) * 0.5);
        let fog_uv = fog_base * fog_h + vec2<f32>(fog_y);
        fog_color = textureSampleLevel(tex4, smp4, fog_uv, 0.0);
      }
      color = mix(color, fog_color, fog_t);
    }
  }

  // Material MRBD/RGB belongs after lighting/fog and before the shared CFX
  // tonecurve/reverse/mono/bright/dark/RGB sequence.
  if (mesh_use_mrbd) {
    let mesh_mono_y = dot(color.rgb, vec3<f32>(0.2989, 0.5886, 0.1145));
    color = vec4<f32>(mix(color.rgb, vec3<f32>(1.0) - color.rgb, vs_u.mesh_mrbd.y), color.a);
    color = vec4<f32>(mix(color.rgb, vec3<f32>(mesh_mono_y), vs_u.mesh_mrbd.x), color.a);
    color = vec4<f32>(color.rgb + vec3<f32>(vs_u.mesh_mrbd.z), color.a);
    color = vec4<f32>(color.rgb - vec3<f32>(vs_u.mesh_mrbd.w), color.a);
  }
  if (mesh_use_rgb) {
    color = vec4<f32>(mix(color.rgb, vs_u.mesh_rgb_rate.xyz, vs_u.mesh_rgb_rate.w), color.a);
    color = vec4<f32>(color.rgb + vs_u.mesh_add_rgb.xyz, color.a);
  }

  let mono_y = dot(color.rgb, vec3<f32>(0.2989, 0.5886, 0.1145));
  if (has_tonecurve > 0.5) {
    color = vec4<f32>(apply_tonecurve_from_mono(color.rgb, mono_y, tonecurve_row, tonecurve_sat), color.a);
  }
  color = vec4<f32>(mix(color.rgb, vec3<f32>(1.0) - color.rgb, rev), color.a);
  color = vec4<f32>(mix(color.rgb, vec3<f32>(mono_y), mono), color.a);
  color = vec4<f32>(color.rgb + vec3<f32>(bright), color.a);
  color = vec4<f32>(color.rgb - vec3<f32>(dark), color.a);
  color = vec4<f32>(mix(color.rgb, color_tgt, color_rate), color.a);
  color = vec4<f32>(color.rgb + color_add, color.a);

  if (blend_code > 2.5 && blend_code < 3.5) {
    color = mix(vec4<f32>(1.0), color, color_org.a);
  } else if (blend_code > 3.5 && blend_code < 4.5) {
    color = mix(vec4<f32>(0.0), color, color_org.a);
  }
  color.a = color_org.a;

  let final_gray = dot(color.rgb, vec3<f32>(0.2989, 0.5886, 0.1145));
  if (has_mask > 0.5) {
    color = color * sample_mask(i.uv);
  }
  if (mask_mode > 0.5 && mask_mode < 1.5) {
    color.a = final_gray;
  }
  if (alpha_test > 0.5 && color.a < alpha_ref) {
    discard;
  }

  if (blend_code > 4.5 && blend_code < 5.5) {
    let dims_u = textureDimensions(tex3, 0);
    let screen_uv = vec2<f32>(
      clamp(i.pos.x / max(f32(dims_u.x), 1.0), 0.0, 1.0),
      clamp(i.pos.y / max(f32(dims_u.y), 1.0), 0.0, 1.0)
    );
    let dst = sample_tex3_safe(screen_uv);
    let ov = overlay_rgb(dst.rgb, color.rgb);
    return vec4<f32>(mix(dst.rgb, ov, color.a), 1.0);
  }
  return color;
}

fn fs_shadow_common(i: ShadowVsOut) -> vec4<f32> {
  let base = textureSample(tex0, smp0, i.uv);
  if ((i.alpha_test > 0.5 || base.a < 0.999) && base.a <= max(vs_u.mtrl_extra.y, 0.001)) {
    discard;
  }
  return vec4<f32>(i.depth, i.depth, i.depth, 1.0);
}

@vertex
fn vs_sprite_2d(v: VsIn2d) -> VsOut2d {
  return vs_common_2d(v);
}

@vertex
fn vs_mesh_static(v: VsIn) -> VsOut {
  return vs_common(v);
}

@vertex
fn vs_mesh_skinned(v: VsIn) -> VsOut {
  return vs_common(v);
}

@vertex
fn vs_shadow_static(v: VsIn) -> ShadowVsOut {
  return vs_shadow_common(v);
}

@vertex
fn vs_shadow_skinned(v: VsIn) -> ShadowVsOut {
  return vs_shadow_common(v);
}

@fragment
fn fs_sprite_2d(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_overlay_gpu(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_mosaic(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_raster_h(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_raster_v(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_explosion_blur(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_shimi(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_shimi_inv(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_cross_mosaic(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_cross_raster_h(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_cross_raster_v(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_wipe_cross_explosion_blur(i: VsOut2d) -> @location(0) vec4<f32> {
  return fs_common_2d(i);
}

@fragment
fn fs_mesh_unlit(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_lambert(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_blinn_phong(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_pp_blinn_phong(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_pp_half_lambert(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_toon(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_ffp(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_pp_ffp(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_bump(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_mesh_parallax(i: VsOut) -> @location(0) vec4<f32> {
  return fs_common(i);
}

@fragment
fn fs_shadow_map(i: ShadowVsOut) -> @location(0) vec4<f32> {
  return fs_shadow_common(i);
}
