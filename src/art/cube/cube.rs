use wgpu::{
    BindGroup, BindGroupLayout, Buffer, Device, RenderPass, RenderPipeline, SurfaceConfiguration,
    util::DeviceExt,
};

use crate::art::vertex::Vertex;

pub struct Cube {
    vertex_buffer: Buffer,
    render_pipeline: RenderPipeline,
}

impl Cube {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        camera_bind_group_layout: &BindGroupLayout,
    ) -> Cube {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("cube Render Pipeline Layout"),
                bind_group_layouts: &[Some(camera_bind_group_layout)],
                immediate_size: 0,
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("cube Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Cube {
            vertex_buffer,
            render_pipeline,
        }
    }

    pub fn submit_cube_rendering_data(
        &self,
        render_pass: &mut RenderPass<'_>,
        camera_bind_group: &BindGroup,
    ) {
        // Set the pipeline to render and tell wgpu to draw something with 3 vertices and 1 instance
        // Submit commands to render pass which is a recording session in command encoder
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        // If we were using index buffers we would use draw_indexed instead of draw here
        render_pass.draw(0..CUBE_VERTICES.len() as u32, 0..1);
    }
}

const CUBE_VERTICES: &[Vertex] = &[
    // Front face
    Vertex {
        position: [-1.0, -1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
    },
    // Back face
    Vertex {
        position: [-1.0, -1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
    },
    // Top face
    Vertex {
        position: [-1.0, 1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
    },
    // Bottom face
    Vertex {
        position: [-1.0, -1.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
    },
    // Left face
    Vertex {
        position: [-1.0, -1.0, -1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
    },
    // Right face
    Vertex {
        position: [1.0, -1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
    },
];
