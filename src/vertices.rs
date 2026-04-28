use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

// Apply C alignment rules, prevent compiler from rearranging fields
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    // This tells GPU how to handle the vertices
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                // At byte offset 0, read a f32 by 3 and store it in shader location 0
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
            ],
        }
    }
}

// These will be turned into a single buffer that GPU can loop over
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.7, 0.0],
        tex_coords: [0.5, 0.0],
    },
    Vertex {
        position: [-0.9, -0.7, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.9, -0.7, 0.0],
        tex_coords: [1.0, 1.0],
    },
];
