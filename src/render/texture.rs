pub struct Texture {
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(view: wgpu::TextureView, sampler: wgpu::Sampler) -> Self {
        Self { view, sampler }
    }
}
