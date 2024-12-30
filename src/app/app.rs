use crate::{
    ecs::World,
    render::{GraphicsDevice, ResourceLoader, SpriteRenderer},
};

use super::{
    asset_loader::AssetLoader,
    components::{Camera, Label, Sprite, Transform},
    frame_clock::FrameClock,
    input::Input,
    systems::{GpuResourcesSystem, SpriteRenderSystem},
};

pub struct App {
    graphics_device: GraphicsDevice,
    sprite_renderer: SpriteRenderer,

    asset_loader: AssetLoader,
    resource_loader: ResourceLoader,
    input: Input,
    frame_clock: FrameClock,

    world: World,
}

impl App {
    pub fn new(window: winit::window::Window) -> Self {
        let graphics_device = GraphicsDevice::new(window);
        let sprite_renderer = SpriteRenderer::new(&graphics_device);

        let mut asset_loader = AssetLoader::new();
        let resource_loader = ResourceLoader::new();
        let frame_clock = FrameClock::new(60);
        let input = Input::new();

        let mut world = App::register_all_components();

        let size = graphics_device.window.inner_size();
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
            Sprite::new(asset_loader.load_texture("./assets/uv_test.png")),
        );

        Self {
            graphics_device,
            sprite_renderer,

            asset_loader,
            resource_loader,
            input,
            frame_clock,

            world,
        }
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

                GpuResourcesSystem::load_resources(
                    &self.graphics_device,
                    &self.asset_loader,
                    &mut self.resource_loader,
                );

                // self.gpu_resources_system
                // .load_resources(&self.asset_loader, &self.graphics_device);

                match self.graphics_device.surface.get_current_texture() {
                    Ok(frame) => {
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = self
                            .graphics_device
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        SpriteRenderSystem::prepare(
                            &self.graphics_device,
                            &self.asset_loader,
                            &mut self.sprite_renderer,
                            &mut self.world,
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

                            SpriteRenderSystem::render(
                                &self.graphics_device,
                                &self.resource_loader,
                                &mut self.sprite_renderer,
                                &mut render_pass,
                            );
                        }

                        self.graphics_device
                            .queue
                            .submit(std::iter::once(encoder.finish()));
                        self.graphics_device.window.pre_present_notify();
                        frame.present();
                    }
                    _ => (),
                }

                self.input.cleanup();
                self.graphics_device.window.request_redraw();
            }
            winit::event::WindowEvent::Resized(size) => self.graphics_device.resize(size),
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        };
    }
}
