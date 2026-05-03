// Shaders take input from the pipeline and give output which it uses for the next stage
// Pipeline -> vertex shader for each vertex (programmable), assembly vertices into shapes
// based on pipeline settings (not programmable), raster (not programmable), then for each pixel
// run the fragment shader (programmable)
// WGSL programs need an entry point. An entry point is designated by either @vertex, @fragment or @compute

struct VertexInput {
    // Whatever is sent to shader location 0 is pos and 1 color
    @location(0) position: vec3<f32>,
};

// The clip position is what the gpu used and is required, the other fields are for me to use
struct VertexOutput {
    @builtin(position) pixel_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Var uniform lets gpu know this is in read only access space and is global
// In wgpu, every @binding declared in your shader must have a corresponding entry in
// the BindGroupLayout and BindGroup. Layout defines interface, bind group defines data
@group(0) @binding(0)
var<uniform> time: f32;

// Vertex shader runs for each vertex we give in the draw, every 3 vertices it makes a triangle
// as we defined in the pipeline
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // GPU converts clip space to pixel coords
    out.pixel_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader runs for each pixel, GPU already decided color for pixel by interpolation,
// but we get to decide final color here, each user defined value in the vertex struct
// is then interpolated here for each fragment/pixel
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    const WAVE_HOR_SPEED = 5;
    const WAVE_VER_SPEED = 2;
    const WAVE_AMPLITUDE = 4;
    const WAVELENGTH = 10;
    const MAGNITUDE_OF_UPS_AND_DOWNS = 20;
    const SHALLOW_WATER_COLOR = vec3(0.01, 0.05, 0.95);
    const DEEP_WATER_COLOR = vec3(0.01, 0.05, 0.2);

    let x = in.pixel_position.x;
    let y = in.pixel_position.y;
    let wave_height_offset = (sin(time * WAVE_VER_SPEED) * MAGNITUDE_OF_UPS_AND_DOWNS) + 300;
    let wave_right_shift = time * WAVE_HOR_SPEED;
    let wave2 = sin(x / 5.0 - time * 3.0) * 3.0;
    let wave3 = sin(x / 8.0 - time * 2.0) * 4.5;
    let height_limit_for_x = (sin(x / WAVELENGTH - wave_right_shift) * WAVE_AMPLITUDE) + wave2 + wave3 + wave_height_offset;

    if y < height_limit_for_x {
        return vec4<f32>(0.1, 0.2, 0.3, 1.0);
    }

    let shallow_y = 350.0;
    let deep_y = 600.0;
    let t = smoothstep(shallow_y, deep_y, y);

    let final_color = mix(SHALLOW_WATER_COLOR, DEEP_WATER_COLOR, t);

    return vec4<f32>(final_color, 1.0);
}
