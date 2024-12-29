pub struct Texture {
    pub id: &'static str,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(id: &'static str, view: wgpu::TextureView, sampler: wgpu::Sampler) -> Self {
        Self { id, view, sampler }
    }
}

impl std::hash::Hash for Texture {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Texture {}
