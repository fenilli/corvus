struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let position = array(
        vec2f(-0.5, -0.5),
        vec2f(0.5, -0.5),
        vec2f(0.5, 0.5),
    );

    var output: VertexOutput;

    output.position = vec4f(position[vertex_index], 0.0, 1.0);
    output.color = vec4f(1.0, 0.0, 0.0, 1.0);

    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}