struct Globals {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VSOut {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) m0: vec4<f32>,
    @location(2) m1: vec4<f32>,
    @location(3) m2: vec4<f32>,
    @location(4) m3: vec4<f32>,
) -> VSOut {
    var out: VSOut;

    let model = mat4x4<f32>(m0, m1, m2, m3);
    out.pos = globals.view_proj * model * vec4<f32>(position, 0.0, 1.0);

    return out;
}


@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
