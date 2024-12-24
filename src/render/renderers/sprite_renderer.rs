use wgpu::{include_wgsl, util::DeviceExt};

use crate::{
    assets::{Asset, AssetLoader, Texture},
    render::{resources::Resources, vertex::Vertex, GraphicsDevice, Instance},
};

pub struct SpriteRenderer {
    resources: Resources,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    index_len: u32,
    instances_len: u32,
}

impl SpriteRenderer {
    pub fn new(
        graphics_device: &GraphicsDevice,
        world_to_projection_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let resources = Resources::new();

        let shader = graphics_device
            .device
            .create_shader_module(include_wgsl!("../shaders/sprite.wgsl"));

        let texture_bind_group_layout =
            graphics_device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Sprite Texture Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                    ],
                });

        let render_pipeline_layout =
            graphics_device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        world_to_projection_bind_group_layout,
                        &texture_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            graphics_device
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[Vertex::desc(), Instance::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: graphics_device.surface_config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        let vertices: &[Vertex; 4] = &[
            Vertex::new([-0.5, 0.5, 1.0], [0.0, 1.0]),
            Vertex::new([0.5, 0.5, 1.0], [1.0, 1.0]),
            Vertex::new([0.5, -0.5, 1.0], [1.0, 0.0]),
            Vertex::new([-0.5, -0.5, 1.0], [0.0, 0.0]),
        ];

        let vertex_buffer =
            graphics_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let indices: &[u16] = &[0, 1, 2, 2, 3, 0];

        let index_buffer =
            graphics_device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let instance_buffer = graphics_device
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: (std::mem::size_of::<Instance>() * 100) as wgpu::BufferAddress, // Max 100 instances
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        Self {
            resources,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            texture_bind_group_layout,
            index_len: indices.len() as u32,
            instances_len: 0,
        }
    }

    pub fn render(
        &mut self,
        graphics_device: &GraphicsDevice,
        asset_loader: &AssetLoader,
        batches: std::collections::HashMap<Asset<Texture>, Vec<Instance>>,
        render_pass: &mut wgpu::RenderPass,
    ) {
        for (texture_handle, instance) in batches {
            let texture = asset_loader.get_texture(texture_handle).unwrap();
            let gpu_texture =
                self.resources
                    .insert_texture(texture_handle, texture, graphics_device);

            let texture_sampler = graphics_device
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
                    anisotropy_clamp: 4,
                    border_color: None,
                });

            let texture_view = gpu_texture.create_view(&wgpu::TextureViewDescriptor::default());

            let texture_bind_group =
                graphics_device
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &self.texture_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Sampler(&texture_sampler),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(&texture_view),
                            },
                        ],
                        label: Some("Texture Bind Group"),
                    });

            graphics_device.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(instance.as_slice()),
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(1, &texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.index_len, 0, 0..self.instances_len);
        }
    }
}
