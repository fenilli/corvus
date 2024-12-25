use std::sync::Arc;

use crate::{
    assets::AssetLoader,
    ecs::World,
    render::{GraphicsDevice, ResourceLoader, SpriteRenderer},
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
        let graphics_device = GraphicsDevice::new(window.clone());
        let (mut asset_loader, resource_loader) = App::load_all_assets(&graphics_device);

        let sprite_renderer = SpriteRenderer::new(&resource_loader, &graphics_device);

        let mut world = App::register_all_components();
        let player = world.spawn();
        world.insert_component(player, Label::new("Player"));
        world.insert_component(
            player,
            Transform::new(
                glam::Vec3::new(0.0, 0.0, 0.0),
                0.0,
                glam::Vec3::new(1.0, 1.0, 1.0),
            ),
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

    fn load_all_assets(graphics_device: &GraphicsDevice) -> (AssetLoader, ResourceLoader) {
        let mut asset_loader = AssetLoader::new();
        asset_loader.load_texture("./assets/uv_test.png");

        let mut resource_loader = ResourceLoader::new();
        for (handle, texture) in asset_loader.get_all_textures() {
            resource_loader.load_texture(graphics_device, handle, texture);
        }

        (asset_loader, resource_loader)
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

                        self.sprite_renderer
                            .prepare(&mut self.world, &self.graphics_device);

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
