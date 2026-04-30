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

// Var uniform lets gpu know this is in read only access space and is global
// In wgpu, every @binding declared in your shader must have a corresponding entry in
// the BindGroupLayout and BindGroup. Layout defines interface, bind group defines data
struct RandomColors {
    r: f32,
    g: f32,
    b: f32,
};

@group(0) @binding(0)
var<uniform> colors: RandomColors;

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

const SHALLOW_WATER_COLOR = vec3(0.0, 0.9, 0.8);
const DEEP_WATER_COLOR = vec3(0.01, 0.05, 0.2);

// Fragment shader runs for each pixel, GPU already decided color for pixel by interpolation,
// but we get to decide final color here, each user defined value in the vertex struct
// is then interpolated here for each fragment/pixel
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var final_color = DEEP_WATER_COLOR;

    if in.clip_position.y < 450 {
        final_color = vec3(0.1, 0.2, 0.3);
    } else if in.clip_position.y < 500 {
        final_color = SHALLOW_WATER_COLOR;
    }

    return vec4<f32>(final_color, 1.0);
}
