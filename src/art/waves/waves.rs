use std::time::Instant;

use wgpu::{
    BindGroup, Buffer, Device, Queue, RenderPass, RenderPipeline, SurfaceConfiguration,
    util::DeviceExt,
};

use crate::art::vertex::Vertex;

pub struct Waves {
    creation_time: Instant,
    time_buffer: Buffer,
    vertex_buffer: Buffer,
    render_pipeline: RenderPipeline,
    bind_group: BindGroup,
}

impl Waves {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Waves {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let time_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("random_colors_buffer"),
            size: 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("water_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("time_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Water Render Pipeline Layout"),
                bind_group_layouts: &[Some(&bind_group_layout)],
                immediate_size: 0,
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Water Render Pipeline"),
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
            // This tells wgpu to assemble triangles from the vertices
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
            contents: bytemuck::cast_slice(WAVE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Waves {
            creation_time: Instant::now(),
            time_buffer,
            vertex_buffer,
            render_pipeline,
            bind_group,
        }
    }

    pub fn submit_wave_rendering_data(&self, render_pass: &mut RenderPass<'_>, queue: &Queue) {
        let time_since_creation = self.creation_time.elapsed().as_secs_f32();
        queue.write_buffer(
            &self.time_buffer,
            0,
            bytemuck::cast_slice(&[time_since_creation]),
        );

        // Set the pipeline to render and tell wgpu to draw something with 3 vertices and 1 instance
        // Submit commands to render pass which is a recording session in command encoder
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        // If we were using index buffers we would use draw_indexed instead of draw here
        render_pass.draw(0..WAVE_VERTICES.len() as u32, 0..1);
    }
}

// These will be turned into a single buffer that GPU can loop over
const WAVE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 0.7, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
    },
    // Second triangle
    Vertex {
        position: [-1.0, 0.7, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.7, 0.0],
    },
];
