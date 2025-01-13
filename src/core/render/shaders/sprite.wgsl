struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera_view_projection: mat4x4<f32>;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.clip_position = camera_view_projection * vec4(vertex.position, 0.0, 1.0);
    output.color = vertex.color;
    output.uv = vertex.uv;

    return output;
}

// @group(1) @binding(0) var texture: texture_2d<f32>;
// @group(1) @binding(1) var texture_sampler: sampler;

struct FragmentInput {
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@fragment
fn fs_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
    return fragment.color;
    // return fragment.color * textureSample(texture, texture_sampler, fragment.uv);
}