use crate::render::GraphicsDevice;

use super::{texture::Texture, Asset};

pub struct AssetLoader {
    textures: std::collections::HashMap<Asset<Texture>, Texture>,
    gpu_textures: std::collections::HashMap<Asset<Texture>, wgpu::TextureView>,
    next: u32,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            textures: std::collections::HashMap::new(),
            gpu_textures: std::collections::HashMap::new(),
            next: 0,
        }
    }

    pub fn load_texture(&mut self, path: &'static str) -> Asset<Texture> {
        let mut handle = Asset::<Texture>::new(path);

        if self.textures.contains_key(&handle) {
            return handle;
        }

        handle.index = self.next;
        self.textures.insert(handle, Texture::new(path).unwrap());
        self.next += 1;

        handle
    }

    pub fn load_gpu_textures(&mut self, graphics_device: &GraphicsDevice) {
        for (handle, texture) in self.textures.iter() {
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

            self.gpu_textures.insert(*handle, view);
        }
    }

    pub fn get_texture_dimension(&self, handle: Asset<Texture>) -> (u32, u32) {
        let Some(texture) = self.textures.get(&handle) else {
            return (1, 1);
        };

        texture.dimensions()
    }

    pub fn get_all_texture_views(&self) -> Vec<&wgpu::TextureView> {
        self.gpu_textures
            .values()
            .collect::<Vec<&wgpu::TextureView>>()
    }
}
