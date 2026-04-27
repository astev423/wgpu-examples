// Shaders take input from the pipeline and give output which it uses for the next stage
// Pipeline -> vertex shader for each vertex (programmable), assembly vertices into shapes
// based on pipeline settings (not programmable), raster (not programmable), then for each pixel
// run the fragment shader (programmable)
// WGSL programs need an entry point. An entry point is designated by either @vertex, @fragment or @compute

struct VertexInput {
    // Whatever is sent to shader location 0 is pos and 1 color
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// The clip position is what the gpu used and is required, the other fields are for me to use
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Vertex shader runs for each vertex we give in the draw, every 3 vertices it makes a triangle
// as we defined in the pipeline
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    // Clip position is what the GPU uses to determine the position of the vertex, it is a builtin
    // meaning not user defined
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader runs for each pixel, GPU already decided color for pixel by interpolation,
// but we get to decide final color here
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x_normalized = in.clip_position[0] / 1000.0;
    let y_normalized = in.clip_position[1] / 1000.0;
    return vec4<f32>(x_normalized, y_normalized, 1.0, 1.0);
}
