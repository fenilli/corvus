use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    position: [f32; 2],
    scale: [f32; 2],
    color: [f32; 3],
}

impl Instance {
    pub fn new(position: [f32; 2], scale: [f32; 2], color: [f32; 3]) -> Self {
        Self {
            position,
            scale,
            color,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
            vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBUTES,
        }
    }
}
