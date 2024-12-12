use image::GenericImageView;
use wgpu::{Device, Queue};

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(device: &Device, queue: &Queue, path: &'static str) -> Self {
        let Result::Ok(image) = image::open(path) else {
            panic!("Could not load image from {}", path);
        };
        let image_data = image.to_rgba8();
        let dimensions = image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(format!("Texture from {}", path).as_str()),
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            size: texture_size,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}
