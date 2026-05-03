struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) pixel_position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> time: f32;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pixel_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    const WATER_COLOR = vec3(0.01, 0.05, 0.95);

    var final_color = WATER_COLOR;
    if floor(in.pixel_position.x) % 20 == 0 && floor(in.pixel_position.y) % 20 == 0 {
        final_color = vec3(0.95, 0.95, 0.80);
    }

    return vec4<f32>(final_color, 1.0);
}
