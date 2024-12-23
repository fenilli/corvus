struct VertexInput {
    @location(0) v_pos: vec3<f32>,
    @location(1) v_color: vec3<f32>,
    @location(2) i_transform_row0: vec4<f32>,
    @location(3) i_transform_row1: vec4<f32>,
    @location(4) i_transform_row2: vec4<f32>,
    @location(5) i_transform_row3: vec4<f32>,
    @location(6) i_color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) o_pos: vec4<f32>,
    @location(0) o_color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> world_to_projection: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let i_transform = mat4x4<f32>(
        in.i_transform_row0,
        in.i_transform_row1,
        in.i_transform_row2,
        in.i_transform_row3
    );

    out.o_pos = world_to_projection * i_transform * vec4<f32>(in.v_pos, 1.0);
    out.o_color = vec4<f32>(in.i_color, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.o_color;
}