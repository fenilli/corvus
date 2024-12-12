struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) instance_position: vec2<f32>,
    @location(1) instance_scale: vec2<f32>,
    @location(2) instance_color: vec3<f32>,
) -> VertexOutput {
    let positions = array<vec2<f32>, 4>(
        vec2(-0.5, -0.5),
        vec2( 0.5, -0.5),
        vec2( 0.5,  0.5),
        vec2(-0.5,  0.5),
    );

    let uvs = array<vec2<f32>, 4>(
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(1.0, 1.0),
        vec2(0.0, 1.0),
    );

    var out: VertexOutput;

    // Transform the position by scale and offset by instance position
    let local_position = positions[vertex_index];
    let scaled_position = local_position * instance_scale;
    let world_position = scaled_position + instance_position;

    // Output clip position and color
    out.clip_position = projection * vec4<f32>(world_position, 0.0, 1.0);
    out.color = instance_color;
    out.uv = uvs[vertex_index];

    return out;
}

@group(1) @binding(0)
var texture_sampler: sampler;

@group(1) @binding(1)
var texture: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0) * textureSample(texture, texture_sampler, in.uv);
}