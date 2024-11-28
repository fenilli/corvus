use super::texture::Texture;

pub struct Material {
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        texture: &Texture,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Self { bind_group }
    }
}
