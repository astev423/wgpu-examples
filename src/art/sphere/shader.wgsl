struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) pixel_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pixel_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.normal);
    let light_dir = normalize(vec3<f32>(0.0, 1.0, 0.0));

    let diffuse = max(dot(normal, light_dir), 0.0);
    let ambient = 0.2;

    let brightness = ambient + diffuse * 0.8;

    return vec4<f32>(0.0, 0.0, brightness, 1.0);
}
