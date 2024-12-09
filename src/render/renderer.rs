use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use super::{GpuContext, SpritePipeline};

pub struct Renderer {
    gpu: GpuContext,
    sprite_pipeline: SpritePipeline,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let gpu = GpuContext::new(window);

        let sprite_pipeline = SpritePipeline::new(&gpu.device, &[]);

        Self {
            gpu,
            sprite_pipeline,
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

                    render_pass.set_pipeline(&self.sprite_pipeline.render_pipeline);
                    render_pass.draw(0..3, 0..1);
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
