struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) tex_index: i32
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tex_index: i32
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.clip_position = vec4(vertex.position, 1.0);
    output.uv = vertex.uv;
    output.tex_index = vertex.tex_index;

    return output;
}

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) tex_index: i32
};

@group(0) @binding(0) var textures: binding_array<texture_2d<f32>>;
@group(0) @binding(1) var tex_sampler: sampler;

@fragment
fn fs_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
    let color = textureSample(textures[fragment.tex_index], tex_sampler, fragment.uv);
    return color;
}