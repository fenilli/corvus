use std::sync::Arc;

use wgpu::{util::DeviceExt, BindGroup, Buffer, BufferUsages, RenderPipeline};
use winit::{dpi::PhysicalSize, window::Window};

use crate::ecs::components::Camera;

use super::{texture::Texture, GpuContext, Instance, Pipeline};

pub struct Renderer {
    gpu: GpuContext,

    camera: Camera,

    projection_uniform_buffer: Buffer,
    instance_buffer: Buffer,
    index_buffer: Buffer,

    max_num_entities: usize,
    num_indices: u32,

    projection_bind_group: BindGroup,
    texture_bind_group: BindGroup,
    render_pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let max_num_entities = 100;

        let gpu = GpuContext::new(window);
        let size = gpu.window.inner_size();
        let camera = Camera::new(size.width, size.height);

        let texture = Texture::new(&gpu.device, &gpu.queue, "assets/uv_test.png");

        let projection_bind_group_layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Projection Bind Group Layout"),
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
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture Bind Group Layout"),
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
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline = Pipeline::new(
            &gpu.device,
            &[&projection_bind_group_layout, &texture_bind_group_layout],
        );

        let projection_uniform_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Projection Uniform Buffer"),
            size: std::mem::size_of_val(&camera.projection) as wgpu::BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let instance_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (4 * std::mem::size_of::<Instance>() * max_num_entities) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];
        let index_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = indices.len() as u32;

        let projection_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Projection Bind Group"),
            layout: &projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    projection_uniform_buffer.as_entire_buffer_binding(),
                ),
            }],
        });

        let texture_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
            ],
        });

        Self {
            gpu,

            camera,

            projection_uniform_buffer,
            instance_buffer,
            index_buffer,

            max_num_entities,
            num_indices,

            projection_bind_group,
            texture_bind_group,
            render_pipeline: pipeline.render_pipeline,
        }
    }

    pub fn render(&mut self, instance_data: &[Instance]) {
        if instance_data.len() > self.max_num_entities {
            self.max_num_entities = instance_data.len() + 100;

            self.instance_buffer = self.gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: (4 * std::mem::size_of::<Instance>() * self.max_num_entities)
                    as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        match self.gpu.surface.get_current_texture() {
            Ok(output) => {
                self.gpu.queue.write_buffer(
                    &self.projection_uniform_buffer,
                    0,
                    bytemuck::cast_slice(self.camera.projection.as_slice().into()),
                );

                self.gpu.queue.write_buffer(
                    &self.instance_buffer,
                    0,
                    bytemuck::cast_slice(instance_data),
                );

                let instances = instance_data.len() as u32;

                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder =
                    self.gpu
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.1,
                                    b: 0.1,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });

                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_bind_group(0, &self.projection_bind_group, &[]);
                    render_pass.set_bind_group(1, &self.texture_bind_group, &[]);

                    render_pass
                        .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
                    render_pass.draw_indexed(0..self.num_indices, 0, 0..instances);
                }

                self.gpu.queue.submit(std::iter::once(encoder.finish()));
                self.gpu.window.pre_present_notify();
                output.present();
            }
            _ => (),
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let gpu = &mut self.gpu;

        gpu.surface_config.width = size.width;
        gpu.surface_config.height = size.height;

        gpu.surface.configure(&gpu.device, &gpu.surface_config);
    }
}
