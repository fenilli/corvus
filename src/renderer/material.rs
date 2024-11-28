use wgpu::{BindGroup, BindGroupLayout, Device};

use super::texture::Texture;

pub struct Material {
    pub bind_group: BindGroup,
}

impl Material {
    pub fn new(device: &Device, texture: &Texture, bind_group_layout: &BindGroupLayout) -> Self {
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
