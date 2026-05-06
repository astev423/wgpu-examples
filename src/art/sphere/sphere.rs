use std::f32::consts::PI;

use wgpu::{
    BindGroup, BindGroupLayout, Buffer, Device, RenderPass, RenderPipeline, SurfaceConfiguration,
    util::DeviceExt,
};

pub struct Sphere {
    vertex_buffer: Buffer,
    vertex_count: u32,
    render_pipeline: RenderPipeline,
}

impl Sphere {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        camera_bind_group_layout: &BindGroupLayout,
    ) -> Sphere {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Sphere Render Pipeline Layout"),
                bind_group_layouts: &[Some(camera_bind_group_layout)],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sphere Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<ModelData>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: 12,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        let sphere_vertices = make_sphere_vertices(32, 64);
        let vertex_count = sphere_vertices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Vertex Buffer"),
            contents: bytemuck::cast_slice(&sphere_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Sphere {
            vertex_buffer,
            vertex_count,
            render_pipeline,
        }
    }

    pub fn submit_sphere_rendering_data(
        &self,
        render_pass: &mut RenderPass<'_>,
        camera_bind_group: &BindGroup,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, camera_bind_group, &[]);

        render_pass.draw(0..self.vertex_count, 0..1);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ModelData {
    position: [f32; 3],
    normal: [f32; 3],
}

fn make_sphere_vertices(stacks: usize, slices: usize) -> Vec<ModelData> {
    assert!(stacks >= 2);
    assert!(slices >= 3);

    let mut vertices = Vec::with_capacity(stacks * slices * 6);

    fn vertex(phi: f32, theta: f32) -> ModelData {
        let y = phi.cos();
        let radius = phi.sin();

        let x = radius * theta.cos();
        let z = radius * theta.sin();

        ModelData {
            position: [x, y, z],
            normal: [x, y, z],
        }
    }

    fn push_triangle(vertices: &mut Vec<ModelData>, a: ModelData, b: ModelData, c: ModelData) {
        vertices.push(a);
        vertices.push(b);
        vertices.push(c);
    }

    for stack in 0..stacks {
        let phi0 = PI * stack as f32 / stacks as f32;
        let phi1 = PI * (stack + 1) as f32 / stacks as f32;

        for slice in 0..slices {
            let theta0 = 2.0 * PI * slice as f32 / slices as f32;
            let theta1 = 2.0 * PI * (slice + 1) as f32 / slices as f32;

            let p00 = vertex(phi0, theta0);
            let p01 = vertex(phi0, theta1);
            let p10 = vertex(phi1, theta0);
            let p11 = vertex(phi1, theta1);

            if stack == 0 {
                // Top cap
                push_triangle(&mut vertices, p00, p11, p10);
            } else if stack == stacks - 1 {
                // Bottom cap
                push_triangle(&mut vertices, p00, p01, p10);
            } else {
                // Body quad split into two triangles
                push_triangle(&mut vertices, p00, p01, p11);
                push_triangle(&mut vertices, p00, p11, p10);
            }
        }
    }

    vertices
}
