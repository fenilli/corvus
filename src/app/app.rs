use std::sync::Arc;

use pollster::FutureExt;

use crate::{
    ecs::{
        resources::{Input, Scener, SpriteRenderer, Timestep},
        systems::{InputSystem, RendererSystem, ScenerSystem, TimestepSystem},
    },
    Resources, World,
};

use super::scenes::Game;

pub struct App {
    resources: Resources,
    world: World,

    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,

    window: Arc<winit::window::Window>,
}

impl App {
    pub fn new(window: winit::window::Window) -> Self {
        let window = Arc::new(window);
        let physical_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase::default())
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .block_on()
            .unwrap();

        let surface = instance.create_surface(window.clone()).unwrap();
        let swapchain_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: physical_size.width,
            height: physical_size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let mut resources = Resources::new();
        resources.insert_resource(Timestep::new(60));
        resources.insert_resource(Input::new());
        resources.insert_resource(SpriteRenderer::new());
        resources.insert_resource(Scener::new(Game));

        let world = World::new();

        Self {
            resources,
            world,

            surface,
            surface_config,
            device,
            queue,

            window,
        }
    }

    pub fn window_event(&mut self, event: winit::event::WindowEvent) -> bool {
        InputSystem::process(&self.resources, &event);
        ScenerSystem::process(&mut self.world, &self.resources);

        match event {
            winit::event::WindowEvent::RedrawRequested => {
                TimestepSystem::update(&self.resources);
                ScenerSystem::update(&mut self.world, &self.resources);

                match self.surface.get_current_texture() {
                    Ok(frame) => {
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = self
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        RendererSystem::prepare(&self.world);

                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Render Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                            store: wgpu::StoreOp::Store,
                                        },
                                        resolve_target: None,
                                    })],
                                    ..Default::default()
                                });

                            RendererSystem::render(&self.world, &mut render_pass);
                        }

                        self.queue.submit(std::iter::once(encoder.finish()));
                        self.window.pre_present_notify();
                        frame.present();
                    }
                    _ => (),
                };

                InputSystem::cleanup(&self.resources);
            }
            winit::event::WindowEvent::Resized(size) => {
                self.surface_config.width = size.width;
                self.surface_config.height = size.height;

                self.surface.configure(&self.device, &self.surface_config);
            }
            winit::event::WindowEvent::CloseRequested => {
                ScenerSystem::cleanup(&mut self.world, &self.resources);

                return false;
            }
            _ => (),
        };

        true
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }
}
