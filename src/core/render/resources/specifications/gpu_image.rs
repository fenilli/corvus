#[derive(Debug)]
pub struct GpuImage {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl GpuImage {
    pub fn new(texture: wgpu::Texture, view: wgpu::TextureView) -> Self {
        Self { texture, view }
    }
}
