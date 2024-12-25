use wgpu::include_wgsl;

use crate::{
    app::components::{Sprite, Transform},
    ecs::World,
};

use super::{GraphicsDevice, ResourceLoader, Vertex};

pub struct SpriteRenderer {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices_len: u32,
}

impl SpriteRenderer {
    pub fn new(resource_loader: &ResourceLoader, graphics_device: &GraphicsDevice) -> Self {
        let shader = graphics_device
            .device
            .create_shader_module(include_wgsl!("shaders/sprite.wgsl"));

        let vertex_buffer = graphics_device
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Sprite Vertex Buffer"),
                size: (4 * std::mem::size_of::<Vertex>() * 1024) as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let index_buffer = graphics_device
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Sprite Index Buffer"),
                size: (6 * std::mem::size_of::<u16>() * 1024) as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let texture_bind_group_layout =
            graphics_device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Sprite Texture Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: Some(std::num::NonZero::new(1).unwrap()),
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let sprite_sampler = graphics_device
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label: Some("Sprite Texture Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                compare: None,
                anisotropy_clamp: 1,
                border_color: None,
            });

        let views = resource_loader.get_all_texture_views();

        let texture_bind_group =
            graphics_device
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Sprite Bind Group"),
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureViewArray(views.as_slice()),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sprite_sampler),
                        },
                    ],
                });

        let pipeline =
            graphics_device
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Sprite Render Pipeline"),
                    layout: Some(&graphics_device.device.create_pipeline_layout(
                        &wgpu::PipelineLayoutDescriptor {
                            label: Some("Sprite Pipeline Layout"),
                            bind_group_layouts: &[&texture_bind_group_layout],
                            push_constant_ranges: &[],
                        },
                    )),
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
                            format: graphics_device.surface_config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    multisample: wgpu::MultisampleState::default(),
                    depth_stencil: None,
                    cache: None,
                    multiview: None,
                });

        Self {
            pipeline,
            texture_bind_group,
            vertex_buffer,
            index_buffer,
            indices_len: 0,
        }
    }

    pub fn prepare(&mut self, world: &mut World, graphics_device: &GraphicsDevice) {
        let mut vertex_data: Vec<Vertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();
        let mut index_offset: u16 = 0;

        for (transform, sprite) in world.entities().filter_map(|entity| {
            match (
                world.get_component::<Transform>(entity),
                world.get_component::<Sprite>(entity),
            ) {
                (Some(transform), Some(sprite)) => Some((transform, sprite)),
                _ => None,
            }
        }) {
            let local_vertices = vec![
                Vertex::new(
                    [-1.0, 1.0, 1.0],
                    // transform
                    //     .0
                    //     .transform_point3(glam::Vec3::from_slice(&[-0.5, 0.5, 1.0]))
                    //     .to_array(),
                    sprite.color.into(),
                    [0.0, 0.0],
                    sprite.texture_handle,
                ),
                Vertex::new(
                    [-1.0, -1.0, 1.0],
                    // transform
                    //     .0
                    //     .transform_point3(glam::Vec3::from_slice(&[0.5, 0.5, 1.0]))
                    //     .to_array(),
                    sprite.color.into(),
                    [0.0, 1.0],
                    sprite.texture_handle,
                ),
                Vertex::new(
                    [1.0, -1.0, 1.0],
                    // transform
                    //     .0
                    //     .transform_point3(glam::Vec3::from_slice(&[0.5, -0.5, 1.0]))
                    //     .to_array(),
                    sprite.color.into(),
                    [1.0, 1.0],
                    sprite.texture_handle,
                ),
                Vertex::new(
                    [1.0, 1.0, 1.0],
                    // transform
                    //     .0
                    //     .transform_point3(glam::Vec3::from_slice(&[-0.5, -0.5, 1.0]))
                    //     .to_array(),
                    sprite.color.into(),
                    [1.0, 0.0],
                    sprite.texture_handle,
                ),
            ];

            vertex_data.extend(local_vertices);

            let local_indices = vec![
                index_offset,
                index_offset + 1,
                index_offset + 2,
                index_offset + 2,
                index_offset + 3,
                index_offset,
            ];

            index_data.extend(local_indices);

            index_offset += 4;
        }

        graphics_device.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&vertex_data),
        );

        graphics_device.queue.write_buffer(
            &self.index_buffer,
            0,
            bytemuck::cast_slice(&index_data),
        );

        self.indices_len = index_data.len() as u32;
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices_len, 0, 0..1);
    }
}
