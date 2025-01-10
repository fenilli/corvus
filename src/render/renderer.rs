use pollster::FutureExt;
use wgpu::include_wgsl;

use super::vertex::Vertex;

pub struct DrawCall {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u16>,
    instances: u16,
}

impl DrawCall {
    pub fn new(vertex_data: Vec<Vertex>, index_data: Vec<u16>) -> Self {
        Self {
            vertex_data,
            index_data,
            instances: 0,
        }
    }
}

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,

    batch_draws: std::collections::HashMap<&'static str, DrawCall>,
    textures: std::collections::HashMap<&'static str, wgpu::Texture>,

    view_projection_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,

    texture_bind_groups: std::collections::HashMap<&'static str, wgpu::BindGroup>,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    view_projection_buffer: wgpu::Buffer,

    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(window: std::sync::Arc<winit::window::Window>) -> Self {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap();

        let surface_config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .unwrap_or(wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: window_size.width,
                height: window_size.height,
                desired_maximum_frame_latency: 2,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            });

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Request Device"),
                    required_features:
                        wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING
                        | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                        | wgpu::Features::TEXTURE_BINDING_ARRAY,
                    ..Default::default()
                },
                None,
            )
            .block_on()
            .unwrap();

        surface.configure(&device, &surface_config);

        let batch_draws = std::collections::HashMap::new();
        let textures = std::collections::HashMap::new();

        let view_projection_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("view_projection_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
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
            label: Some("pipeline_layout"),
            bind_group_layouts: &[
                &view_projection_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let texture_bind_groups = std::collections::HashMap::new();

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertex:quad"),
            size: (4 * std::mem::size_of::<Vertex>() * 2048) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("index:quad"),
            size: (6 * std::mem::size_of::<u16>() * 2048) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_projection_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view_projection_buffer"),
            size: std::mem::size_of::<glam::Mat4>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
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
                    format: surface_config.format,
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
            surface,
            surface_config,
            device,
            queue,

            batch_draws,
            textures,

            view_projection_bind_group_layout,
            texture_bind_group_layout,

            texture_bind_groups,

            view_projection_buffer,
            vertex_buffer,
            index_buffer,

            pipeline,
        }
    }

    pub fn load_texture(&mut self, texture_id: &'static str, image: &image::RgbaImage) {
        if self.textures.contains_key(texture_id) {
            return;
        }

        let dimensions = image.dimensions();

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

        self.queue.write_texture(
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

        self.textures.insert(texture_id, texture);
    }

    pub fn create_render_target(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView) {
        let frame = self.surface.get_current_texture().unwrap();

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        (frame, view)
    }

    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }

    pub fn submit_and_present(&self, target: wgpu::SurfaceTexture, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
        target.present();
    }

    pub fn set_view_projection(&mut self, view_projection: glam::Mat4) {
        self.queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::bytes_of(&view_projection),
        );
    }

    pub fn draw(&mut self, texture_id: &'static str, vertex_data: Vec<Vertex>) {
        let batch = self
            .batch_draws
            .entry(texture_id)
            .or_insert(DrawCall::new(Vec::new(), Vec::new()));

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

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);

        let view_projection_bind_group =
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Camera Bind Group"),
                layout: &self.view_projection_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.view_projection_buffer.as_entire_binding(),
                }],
            });

        render_pass.set_bind_group(0, &view_projection_bind_group, &[]);

        let mut offsets = (0, 0);
        for (&texture_handle, draw_call) in &self.batch_draws {
            let texture_bind_group = self.texture_bind_groups.entry(texture_handle).or_insert(
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(
                                &self
                                    .textures
                                    .get(&texture_handle)
                                    .unwrap()
                                    .create_view(&wgpu::TextureViewDescriptor::default()),
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(
                                &self
                                    .device
                                    .create_sampler(&wgpu::SamplerDescriptor::default()),
                            ),
                        },
                    ],
                }),
            );

            render_pass.set_bind_group(1, texture_bind_group as &wgpu::BindGroup, &[]);

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

        self.batch_draws.clear();
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);
    }
}
