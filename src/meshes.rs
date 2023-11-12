use std::mem;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3
        ];

    pub fn descriptor() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 3]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x2,
                }
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [1.0, 0.0, 0.0], tex_coords: [0.4131759, 1.0 - 0.99240386] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.0, 1.0, 0.0], tex_coords: [0.0048659444, 1.0 - 0.56958647] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.0, 0.0, 1.0], tex_coords: [0.28081453, 1.0 - 0.05060294] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [1.0, 1.0, 0.0], tex_coords: [0.85967, 1.0 - 0.1526709] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 1.0, 1.0], tex_coords: [0.9414737, 1.0 - 0.7347359] }, // E
];


pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];