use std::sync::Arc;

use glam::Vec2;

use crate::{ecs::World, render::GraphicsDevice};

use super::{
    components::{Camera, Quad, Transform},
    frame_clock::FrameClock,
    input::Input,
    systems::QuadRendererSystem,
};

pub struct App {
    input: Input,
    world: World,

    frame_clock: FrameClock,
    graphics_device: GraphicsDevice,

    quad_render_system: QuadRendererSystem,

    window: Arc<winit::window::Window>,
}

impl App {
    pub fn new(window: winit::window::Window) -> Self {
        let window = Arc::new(window);
        let window_size = window.inner_size();
        let input = Input::new();
        let mut world = World::new();

        world.register_component::<Camera>();
        world.register_component::<Transform>();
        world.register_component::<Quad>();

        let camera = world.spawn();
        world.insert_component(
            camera,
            Camera {
                position: Vec2::new(0.0, 0.0),
                width: window_size.width,
                height: window_size.height,
                zoom: 1.0,
            },
        );

        let player = world.spawn();
        world.insert_component(
            player,
            Transform {
                position: Vec2::new(100.0, 100.0),
                scale: Vec2::new(1.0, 1.0),
            },
        );
        world.insert_component(
            player,
            Quad {
                height: 100,
                width: 100,
            },
        );

        let frame_clock = FrameClock::new(60);
        let graphics_device = GraphicsDevice::new(window.clone());

        let quad_render_system = QuadRendererSystem::new(&graphics_device);

        Self {
            input,
            world,

            frame_clock,
            graphics_device,

            quad_render_system,

            window,
        }
    }

    pub fn window_event(
        &mut self,
        event: winit::event::WindowEvent,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) {
        self.input.process(&event);

        match event {
            winit::event::WindowEvent::RedrawRequested => {
                let (fixed_deltas, delta_time) = self.frame_clock.update();

                for delta_time in fixed_deltas {
                    // println!("FDT: {}", delta_time);
                }

                // println!("DT: {}", delta_time);

                match self.graphics_device.surface.get_current_texture() {
                    Ok(frame) => {
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = self
                            .graphics_device
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        self.quad_render_system
                            .prepare(&self.world, &self.graphics_device);

                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Render Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                                r: 0.0,
                                                g: 0.0,
                                                b: 0.0,
                                                a: 1.0,
                                            }),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    ..Default::default()
                                });

                            self.quad_render_system.render(&mut render_pass);
                        }

                        self.graphics_device
                            .queue
                            .submit(std::iter::once(encoder.finish()));
                        self.window.pre_present_notify();
                        frame.present();
                    }
                    _ => (),
                }

                self.input.cleanup();
                self.window.request_redraw();
            }
            winit::event::WindowEvent::Resized(size) => self.graphics_device.resize(size),
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        };
    }
}
