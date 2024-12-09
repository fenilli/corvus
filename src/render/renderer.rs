use std::sync::Arc;

use wgpu::{util::DeviceExt, Buffer, RenderPipeline};
use winit::{dpi::PhysicalSize, window::Window};

use super::{GpuContext, Pipeline, Vertex};

pub struct Renderer {
    gpu: GpuContext,
    render_pipeline: RenderPipeline,

    quad_vertex_buffer: Buffer,
    quad_index_buffer: Buffer,
    quad_num_indices: u32,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let gpu = GpuContext::new(window);

        let pipeline = Pipeline::new(&gpu.device, &[]);

        const VERTICES: &[Vertex] = &[
            Vertex {
                position: [-0.0868241, 0.49240386, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // A
            Vertex {
                position: [-0.49513406, 0.06958647, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // B
            Vertex {
                position: [-0.21918549, -0.44939706, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // C
            Vertex {
                position: [0.35966998, -0.3473291, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // D
            Vertex {
                position: [0.44147372, 0.2347359, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // E
        ];

        const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

        let quad_vertex_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let quad_index_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
        let quad_num_indices = INDICES.len() as u32;

        Self {
            gpu,
            render_pipeline: pipeline.render_pipeline,

            quad_vertex_buffer,
            quad_index_buffer,
            quad_num_indices,
        }
    }

    pub fn render(&mut self) {
        match self.gpu.surface.get_current_texture() {
            Ok(output) => {
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
