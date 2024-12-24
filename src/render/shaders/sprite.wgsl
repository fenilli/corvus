struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct InstanceInput {
    @location(2) transform_row0: vec4<f32>,
    @location(3) transform_row1: vec4<f32>,
    @location(4) transform_row2: vec4<f32>,
    @location(5) transform_row3: vec4<f32>,
    @location(6) uv_coords: vec4<f32>,
    @location(7) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> world_to_projection: mat4x4<f32>;

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    let transform = mat4x4<f32>(
        instance.transform_row0,
        instance.transform_row1,
        instance.transform_row2,
        instance.transform_row3
    );

    out.pos = world_to_projection * transform * vec4<f32>(vertex.pos, 1.0);
    out.color = vec4<f32>(instance.color, 1.0);
    out.uv = vertex.uv * vec2<f32>(instance.uv_coords.z, instance.uv_coords.w) + vec2<f32>(instance.uv_coords.x, instance.uv_coords.y);

    return out;
}

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var texture: texture_2d<f32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vertex.color * textureSample(texture, texture_sampler, vertex.uv);
}