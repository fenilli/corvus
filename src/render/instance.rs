use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    transform: [[f32; 4]; 4],
    color: [f32; 3],
}

impl Instance {
    pub fn new(transform: glam::Mat4, color: [f32; 3]) -> Self {
        Self {
            transform: transform.to_cols_array_2d(),
            color,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 5] = vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4, 6 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBUTES,
        }
    }
}
