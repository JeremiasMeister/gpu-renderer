use std::mem;

pub trait Vertex {
    fn descriptor() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coord0: [f32; 4],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex for ModelVertex {
    fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { //position
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3, //3 floats for position
                },
                wgpu::VertexAttribute { // tex_coord0
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // offset of 3 from position
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4, //4 floats for texcoords
                },
                wgpu::VertexAttribute { //normal
                    offset: mem::size_of::<[f32; 7]>() as wgpu::BufferAddress, // offset of 3 from position + 4 from texcoord0 = 7
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3, //3 floats for normal
                },
                wgpu::VertexAttribute { //color
                    offset: mem::size_of::<[f32; 10]>() as wgpu::BufferAddress, // offset of 3 from position + 4 from texcoord0 + 3 from normal = 10
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4, //4 floats for color
                }
            ],
        }
    }
}
