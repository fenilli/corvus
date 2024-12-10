use std::sync::Arc;

use wgpu::{util::DeviceExt, BindGroup, Buffer, BufferUsages, RenderPipeline};
use winit::{dpi::PhysicalSize, window::Window};

use crate::ecs::components::Camera;

use super::{GpuContext, Pipeline, Vertex};

pub struct Renderer {
    gpu: GpuContext,

    camera: Camera,

    projection_uniform_buffer: Buffer,
    quad_vertex_buffer: Buffer,
    quad_index_buffer: Buffer,
    quad_num_indices: u32,
    projection_bind_group: BindGroup,

    render_pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let gpu = GpuContext::new(window);
        let size = gpu.window.inner_size();
        let camera = Camera::new(size.width, size.height);

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

        let pipeline = Pipeline::new(&gpu.device, &[&projection_bind_group_layout]);

        let projection_uniform_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Projection Uniform Buffer"),
            size: std::mem::size_of_val(&camera.projection) as wgpu::BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertices: &[Vertex] = &[
            // Top-left
            Vertex::new(
                [0.0, 0.0], // Position
                [0.0, 0.0], // UV
                [1.0, 1.0, 1.0],
            ),
            // Top-right
            Vertex::new(
                [0.0 + 100.0, 0.0], // Position
                [0.0, 1.0],         // UV
                [1.0, 1.0, 1.0],
            ),
            // Bottom-right
            Vertex::new(
                [0.0 + 100.0, 0.0 + 100.0], // Position
                [1.0, 1.0],                 // UV
                [1.0, 1.0, 1.0],
            ),
            // Bottom-left
            Vertex::new(
                [0.0, 0.0 + 100.0], // Position
                [1.0, 0.0],         // UV
                [1.0, 1.0, 1.0],
            ),
        ];

        let indices: &[u16] = &[
            0, 1, 2, // First triangle
            0, 2, 3, // Second triangle
        ];

        let quad_vertex_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let quad_index_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let quad_num_indices = indices.len() as u32;

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

        Self {
            gpu,

            camera,

            projection_uniform_buffer,
            quad_vertex_buffer,
            quad_index_buffer,
            quad_num_indices,
            projection_bind_group,

            render_pipeline: pipeline.render_pipeline,
        }
    }

    pub fn render(&mut self) {
        match self.gpu.surface.get_current_texture() {
            Ok(output) => {
                self.gpu.queue.write_buffer(
                    &self.projection_uniform_buffer,
                    0,
                    bytemuck::cast_slice(self.camera.projection.as_slice().into()),
                );

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
                    render_pass.set_index_buffer(
                        self.quad_index_buffer.slice(..),
                        wgpu::IndexFormat::Uint16,
                    );
                    render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                    render_pass.set_bind_group(0, &self.projection_bind_group, &[]);
                    render_pass.draw_indexed(0..self.quad_num_indices, 0, 0..1);
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
