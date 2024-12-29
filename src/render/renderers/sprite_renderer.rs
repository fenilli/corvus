use wgpu::include_wgsl;

use crate::render::{GraphicsDevice, ResourceLoader, Vertex};

pub struct BatchDrawCall {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u16>,
}

impl BatchDrawCall {
    pub fn new(vertex_data: Vec<Vertex>, index_data: Vec<u16>) -> Self {
        Self {
            vertex_data,
            index_data,
        }
    }
}

pub struct SpriteRenderer {
    batch_draws: std::collections::HashMap<&'static str, BatchDrawCall>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
}

impl SpriteRenderer {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        let batch_draws = std::collections::HashMap::new();

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

        let shader = graphics_device
            .device
            .create_shader_module(include_wgsl!("../shaders/sprite.wgsl"));

        let texture_bind_group_layout =
            graphics_device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture Bind Group Layout"),
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
            batch_draws,
            vertex_buffer,
            index_buffer,
            texture_bind_group_layout,
            pipeline,
        }
    }

    pub fn draw(&mut self, texture_handle: &'static str, vertex_data: Vec<Vertex>) {
        let batch = self
            .batch_draws
            .entry(texture_handle)
            .or_insert(BatchDrawCall::new(Vec::new(), Vec::new()));

        let index_offset = batch.vertex_data.len() as u16;

        let index_data: Vec<u16> = vec![
            index_offset * 4,
            index_offset * 4 + 1,
            index_offset * 4 + 2,
            index_offset * 4 + 2,
            index_offset * 4 + 3,
            index_offset * 4,
        ];

        batch.vertex_data.extend(vertex_data);
        batch.index_data.extend(index_data);
    }

    // pub fn prepare(&mut self, graphics_device: &GraphicsDevice) {}

    pub fn render(
        &mut self,
        resource_loader: &ResourceLoader,
        graphics_device: &GraphicsDevice,
        render_pass: &mut wgpu::RenderPass,
    ) {
        render_pass.set_pipeline(&self.pipeline);

        for (&texture_handle, draw_call) in &self.batch_draws {
            let texture = resource_loader.get_texture(texture_handle);

            let texture_bind_group =
                graphics_device
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("Texture Bind Group"),
                        layout: &self.texture_bind_group_layout,
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

            graphics_device.queue.write_buffer(
                &self.vertex_buffer,
                0,
                bytemuck::cast_slice(&draw_call.vertex_data),
            );

            graphics_device.queue.write_buffer(
                &self.index_buffer,
                0,
                bytemuck::cast_slice(&draw_call.index_data),
            );

            render_pass.set_bind_group(0, &texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..draw_call.index_data.len() as u32, 0, 0..1);
        }

        self.batch_draws.clear();
    }
}
