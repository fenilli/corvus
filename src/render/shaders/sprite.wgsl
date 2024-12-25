struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tex_index: u32
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) tex_index: u32
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.clip_position = vec4(vertex.position, 1.0);
    output.color = vertex.color;
    output.uv = vertex.uv;
    output.tex_index = vertex.tex_index;

    return output;
}

struct FragmentInput {
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) tex_index: u32
};

@group(0) @binding(0) var textures: binding_array<texture_2d<f32>>;
@group(0) @binding(1) var tex_sampler: sampler;

@fragment
fn fs_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
    let color = fragment.color * textureSample(textures[fragment.tex_index], tex_sampler, fragment.uv);
    return color;
}