use image::GenericImageView;

use crate::render::{GraphicsDevice, Texture};

pub struct ResourceLoader {
    textures: std::collections::HashMap<&'static str, Texture>,
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
        handle: &'static str,
        texture: &image::DynamicImage,
    ) {
        if self.textures.contains_key(handle) {
            return;
        }

        let dimensions = texture.dimensions();
        let data = texture.to_rgba8();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let wgpu_texture = graphics_device
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: None,
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
                texture: &wgpu_texture,
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

        let view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = graphics_device
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label: Some("Sprite Texture Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                compare: None,
                anisotropy_clamp: 1,
                border_color: None,
            });

        self.textures
            .insert(handle, Texture::new(handle, view, sampler));
    }

    pub fn get_texture(&self, handle: &'static str) -> &Texture {
        self.textures.get(handle).unwrap()
    }
}
