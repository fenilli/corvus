use glam::Vec2;
use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    position: [f32; 2],
    size: [f32; 2],
}

impl Instance {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position: position.into(),
            size: size.into(),
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            vertex_attr_array![2 => Float32x2, 3 => Float32x2];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBUTES,
        }
    }
}
