use std::{collections::HashMap, sync::Arc};

use wgpu::include_wgsl;

use crate::core::assets::{handle::HandleId, Image};

use super::{
    graphics, resources::specifications::GpuImage, Camera, Resources, SpriteInstance, Vertex,
};

#[derive(Default)]
pub struct DrawCall {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u16>,
    instances: u16,
}

pub struct SpriteRenderer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    camera: Camera,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    resources: Resources,

    draw_calls: HashMap<HandleId, DrawCall>,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl SpriteRenderer {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let camera = Camera::new(&device, queue.clone());
        let resources = Resources::new();

        let draw_calls = HashMap::new();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("SpriteRenderer:texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("SpriteRenderer:pipeline_layout"),
            bind_group_layouts: &[&camera.bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SpriteRenderer:vertex_buffer"),
            size: (4 * std::mem::size_of::<Vertex>() * 2048) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SpriteRenderer:index_buffer"),
            size: (6 * std::mem::size_of::<u16>() * 2048) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader = device.create_shader_module(include_wgsl!("shaders/sprite.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("SpriteRenderer:pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            multisample: Default::default(),
            depth_stencil: None,
            multiview: None,
            cache: None,
        });

        Self {
            device,
            queue,
            camera,
            texture_bind_group_layout,
            resources,

            draw_calls,

            vertex_buffer,
            index_buffer,
            pipeline,
        }
    }

    pub fn upload_texture(&mut self, handle_id: HandleId, image: &Image) {
        if self.resources.textures.exists(&handle_id) {
            return;
        }

        let dimensions = image.dimensions;

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture_desc = &wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture = self.device.create_texture(&texture_desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image.data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(dimensions.0 * 4),
                rows_per_image: None,
            },
            size,
        );

        let gpu_image = GpuImage::new(texture, view);
        self.resources.textures.insert(handle_id, gpu_image);
    }

    pub fn draw(&mut self, sprite_instance: SpriteInstance) {
        let batch = self
            .draw_calls
            .entry(sprite_instance.image_id.id())
            .or_default();

        let vertex_data: Vec<Vertex> = sprite_instance
            .position
            .iter()
            .zip(sprite_instance.uv_coords)
            .map(|(&xy, uv)| Vertex::new(xy, sprite_instance.color, uv))
            .collect();

        let index_data = vec![
            batch.instances * 4,
            batch.instances * 4 + 1,
            batch.instances * 4 + 2,
            batch.instances * 4 + 2,
            batch.instances * 4 + 3,
            batch.instances * 4,
        ];

        batch.instances += 1;
        batch.vertex_data.extend(vertex_data);
        batch.index_data.extend(index_data);
    }

    pub fn update_view_projection(&mut self, view_projection: glam::Mat4) {
        self.camera.update_view_projection(view_projection);
    }

    pub fn render(&mut self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("SpriteRenderer:render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                resolve_target: None,
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, &self.camera.bind_group, &[]);

        let mut offsets = (0, 0);
        for (handle_id, draw_call) in &self.draw_calls {
            let texture = self.resources.textures.get(handle_id).unwrap();

            let texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(format!("SpriteRenderer:{}", handle_id.id()).as_str()),
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&graphics::create_sampler(
                            &self.device,
                        )),
                    },
                ],
            });

            render_pass.set_bind_group(1, &texture_bind_group, &[]);

            let vertex_data = bytemuck::cast_slice(&draw_call.vertex_data);
            let index_data = bytemuck::cast_slice(&draw_call.index_data);

            self.queue
                .write_buffer(&self.vertex_buffer, offsets.0, vertex_data);
            self.queue
                .write_buffer(&self.index_buffer, offsets.1, index_data);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(offsets.0..));
            render_pass.set_index_buffer(
                self.index_buffer.slice(offsets.1..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..draw_call.index_data.len() as u32, 0, 0..1);

            offsets.0 += std::mem::size_of_val(vertex_data) as u64;
            offsets.1 += std::mem::size_of_val(index_data) as u64;
        }

        self.draw_calls.clear();
    }
}
