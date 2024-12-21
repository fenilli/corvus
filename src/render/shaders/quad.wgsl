struct VertexInput {
    @location(0) local_position: vec2<f32>,
    @location(1) instance_position: vec2<f32>,
    @location(2) instance_scale: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) frag_color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let scaled_position = input.local_position * input.instance_scale;
    let world_position = scaled_position + input.instance_position;

    output.clip_position = projection * vec4<f32>(world_position, 0.0, 1.0);
    output.frag_color = vec4<f32>(1.0, 0.0, 0.0, 1.0);

    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.frag_color;
}