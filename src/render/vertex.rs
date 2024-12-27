use crate::assets::{Asset, Texture};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 4],
    uv: [f32; 2],
    tex_index: u32,
}

impl Vertex {
    pub fn new(
        pos: [f32; 3],
        color: [f32; 4],
        uv: [f32; 2],
        texture_handle: Asset<Texture>,
    ) -> Self {
        Self {
            pos,
            color,
            uv,
            tex_index: texture_handle.index,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 4] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32x2, 3 => Uint32];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}
