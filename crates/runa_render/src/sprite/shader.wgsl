struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct Globals {
    view_proj: mat4x4<f32>,
    aspect: f32,
    _padding: vec3<f32>,
};

// Globals (view_proj matrix)
@group(0) @binding(0) var<uniform> globals: Globals;

// Sprite texture
@group(0) @binding(1) var t_diffuse: texture_2d<f32>;
@group(0) @binding(2) var s_sampler: sampler;

@vertex
fn vs_main(
    @location(0) a_position: vec2<f32>,
    @location(1) a_tex_coords: vec2<f32>
) -> VertexOutput {
    var out: VertexOutput;

    // Корректируем X-координату для сохранения квадратных пикселей
    let corrected_position = vec2<f32>(
        a_position.x * globals.aspect,
        a_position.y
    );

    out.position = globals.view_proj * vec4<f32>(corrected_position, 0.0, 1.0);
    out.tex_coords = a_tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_sampler, in.tex_coords);
}
