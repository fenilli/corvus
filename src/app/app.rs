use std::sync::Arc;

use crate::{
    assets::AssetLoader,
    ecs::World,
    render::{GraphicsDevice, SpriteRenderer},
};

use super::{
    color::Color,
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

    sprite_renderer: SpriteRenderer,

    window: Arc<winit::window::Window>,
}

impl App {
    pub fn new(window: winit::window::Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();
        let graphics_device = GraphicsDevice::new(window.clone());
        let mut asset_loader = App::load_all_assets(&graphics_device);

        let sprite_renderer = SpriteRenderer::new(&asset_loader, &graphics_device);

        let mut world = App::register_all_components();
        let camera = world.spawn();
        world.insert_component(
            camera,
            Camera::new(
                glam::Vec2::new(0.0, 0.0),
                winit::dpi::PhysicalSize::new(size.width, size.height),
                1.0,
            ),
        );

        let player = world.spawn();
        world.insert_component(player, Label::new("Player"));
        world.insert_component(
            player,
            Transform::new(glam::Vec2::new(0.0, 0.0), glam::Vec2::new(1.0, 1.0), 0.0),
        );
        world.insert_component(
            player,
            Sprite::new(
                asset_loader.load_texture("./assets/uv_test.png"),
                Color::WHITE,
            ),
        );

        let frame_clock = FrameClock::new(60);
        let input = Input::new();

        Self {
            asset_loader,
            input,
            world,

            frame_clock,
            graphics_device,

            sprite_renderer,

            window,
        }
    }

    fn load_all_assets(graphics_device: &GraphicsDevice) -> AssetLoader {
        let mut asset_loader = AssetLoader::new();
        asset_loader.load_texture("./assets/uv_test.png");

        asset_loader.load_gpu_textures(graphics_device);

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

                        self.sprite_renderer.prepare(
                            &mut self.world,
                            &self.asset_loader,
                            &self.graphics_device,
                        );

                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Render Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                                r: 0.1,
                                                g: 0.2,
                                                b: 0.3,
                                                a: 1.0,
                                            }),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    ..Default::default()
                                });

                            self.sprite_renderer.render(&mut render_pass);
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
