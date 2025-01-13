use std::sync::Arc;

use winit::{event::WindowEvent, window::Window};

use crate::core::{
    assets::{AssetServer, Assets, Image},
    ecs::{
        components::{OrthoCamera, Sprite, Transform},
        systems::{asset_system, render_system},
        World,
    },
    render::{graphics, Rect, SpriteRenderer},
    resources::Resources,
};

pub struct Game {
    surface: wgpu::Surface<'static>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,

    sprite_renderer: SpriteRenderer,

    assets: Assets,
    resources: Resources,
    asset_server: AssetServer,
    world: World,

    window: Arc<Window>,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let window = std::sync::Arc::new(window);
        let (surface, device, queue) = graphics::initialize_wgpu(window.clone());

        let sprite_renderer = SpriteRenderer::new(device.clone(), queue.clone());

        let assets = Assets::new();
        let resources = Resources::new();
        let mut asset_server = AssetServer::new();

        let mut world = World::new();
        world.register_component::<Transform>();
        world.register_component::<Sprite>();
        world.register_component::<OrthoCamera>();

        let c = world.spawn();
        world.insert_component(
            c,
            OrthoCamera::new(glam::vec2(0.0, 0.0), window.inner_size(), 1.0),
        );

        let e = world.spawn();
        world.insert_component(
            e,
            Transform::new(
                glam::vec3(16.0, 0.0, 0.0),
                glam::vec2(1.0, 1.0),
                0.0,
                glam::vec2(0.0, 1.0),
            ),
        );

        world.insert_component(
            e,
            Sprite::new(
                asset_server.load::<Image>("assets/character/idle.png"),
                Rect::new(32, 32, 16, 16),
                [1.0, 1.0, 1.0, 1.0],
                false,
                false,
            ),
        );

        Self {
            surface,
            device,
            queue,

            sprite_renderer,

            assets,
            resources,
            asset_server,
            world,

            window,
        }
    }

    pub fn process_window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: winit::event::WindowEvent,
    ) {
        asset_system::load_pending_assets(
            &self.device,
            &self.queue,
            &mut self.asset_server,
            &mut self.assets,
            &mut self.resources,
        );

        match event {
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.surface
                    .configure(&self.device, &graphics::create_surface_config(size));
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        render_system::set_camera_projection(&self.world, &mut self.sprite_renderer);
        render_system::draw_sprites(&self.world, &self.assets, &mut self.sprite_renderer);

        self.sprite_renderer
            .render(&self.resources, &view, &mut encoder);

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
