type ResourceCache<K, V> = std::collections::HashMap<K, std::sync::Arc<V>>;

pub struct ResourceRegistry {
    textures: ResourceCache<String, wgpu::Texture>,
    bind_groups: ResourceCache<String, wgpu::BindGroup>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            textures: ResourceCache::new(),
            bind_groups: ResourceCache::new(),
        }
    }

    pub fn create_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: &str,
        image: &image::RgbaImage,
    ) {
        self.textures.entry(name.to_string()).or_insert({
            let dimensions = image.dimensions();

            let size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };

            let texture_desc = &wgpu::TextureDescriptor {
                label: Some(name),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            };

            let texture = device.create_texture(&texture_desc);

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &image,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(dimensions.0 * 4),
                    rows_per_image: None,
                },
                size,
            );

            std::sync::Arc::new(texture)
        });
    }

    pub fn get_texture(&self, key: &str) -> &std::sync::Arc<wgpu::Texture> {
        self.textures.get(key).unwrap()
    }

    pub fn create_bind_group(
        &mut self,
        device: &wgpu::Device,
        key: &str,
        desc: &wgpu::BindGroupDescriptor,
    ) -> &std::sync::Arc<wgpu::BindGroup> {
        self.bind_groups
            .entry(key.to_string())
            .or_insert(std::sync::Arc::new(device.create_bind_group(desc)))
    }
}
