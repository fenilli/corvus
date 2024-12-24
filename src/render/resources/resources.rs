use crate::{
    assets::{Asset, Texture},
    render::GraphicsDevice,
};

pub struct Resources {
    textures: std::collections::HashMap<Asset<Texture>, wgpu::Texture>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn insert_texture(
        &mut self,
        handle: Asset<Texture>,
        texture: &Texture,
        graphics_device: &GraphicsDevice,
    ) -> &wgpu::Texture {
        self.textures.entry(handle).or_insert({
            let dimensions = texture.dimensions();

            let gpu_texture = graphics_device
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(format!("Texture of: {}", handle.id()).as_str()),
                    size: wgpu::Extent3d {
                        width: dimensions.0,
                        height: dimensions.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: graphics_device.surface_config.format,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });

            graphics_device.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &gpu_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &texture.data(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: None,
                    rows_per_image: None,
                },
                wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                },
            );

            gpu_texture
        })
    }
}
