// ===== UNIFORMS =====
struct MeshUniforms {
  view_proj: mat4x4<f32>,  // 64 bytes
  view: mat4x4<f32>,       // 64 bytes
  color: vec4<f32>,        // 16 bytes - mesh color
  _padding: array<vec4<f32>, 7>, // 112 bytes of padding to align to 256
};
@group(0) @binding(0) var<uniform> globals: MeshUniforms;

// ===== TEXTURES (optional - for textured meshes) =====
@group(0) @binding(1) var t_diffuse: texture_2d<f32>;
@group(0) @binding(2) var s_sampler: sampler;

// ===== VERTEX SHADER =====
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // World-space vertex position
    let world_pos = in.position;

    // World-space normal
    let world_normal = normalize(in.normal);

    // Final clip-space position
    var out: VertexOutput;
    out.position = globals.view_proj * vec4<f32>(world_pos, 1.0);
    out.uv = in.uv;
    out.normal = world_normal;
    out.world_pos = world_pos;

    return out;
}

// ===== Fragment Shader =====
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return globals.color;
}
