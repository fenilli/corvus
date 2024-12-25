use crate::assets::{Asset, Texture as CpuTexture};

use super::GraphicsDevice;

pub struct ResourceLoader {
    textures: std::collections::HashMap<Asset<CpuTexture>, wgpu::TextureView>,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self {
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        graphics_device: &GraphicsDevice,
        handle: &Asset<CpuTexture>,
        texture: &CpuTexture,
    ) {
        let dimensions = texture.dimensions();
        let data = texture.data();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = graphics_device
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some(format!("Texture {}", handle.index).as_str()),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

        graphics_device.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(dimensions.0 * 4),
                rows_per_image: None,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.textures.insert(*handle, view);
    }

    pub fn get_all_texture_views(&self) -> Vec<&wgpu::TextureView> {
        self.textures.values().collect::<Vec<&wgpu::TextureView>>()
    }
}
