use std::sync::Arc;

use crate::{assets::AssetLoader, ecs::World, render::GraphicsDevice};

use super::{
    components::{Camera, Label, Sprite, Transform},
    frame_clock::FrameClock,
    input::Input,
};

pub struct App {
    asset_loader: AssetLoader,
    input: Input,
    world: World,

    frame_clock: FrameClock,
    graphics_device: GraphicsDevice,

    window: Arc<winit::window::Window>,
}

impl App {
    pub fn new(window: winit::window::Window) -> Self {
        let window = Arc::new(window);
        let graphics_device = GraphicsDevice::new(window.clone());

        let asset_loader = App::load_all_assets();
        let world = App::register_all_components();

        let frame_clock = FrameClock::new(60);
        let input = Input::new();

        Self {
            asset_loader,
            input,
            world,

            frame_clock,
            graphics_device,

            window,
        }
    }

    fn load_all_assets() -> AssetLoader {
        let mut asset_loader = AssetLoader::new();
        asset_loader.load_texture("./assets/uv_test.png");

        asset_loader
    }

    fn register_all_components() -> World {
        let mut world = World::new();

        world.register_component::<Label>();
        world.register_component::<Camera>();
        world.register_component::<Transform>();
        world.register_component::<Sprite>();

        world
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
