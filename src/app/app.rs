use crate::{
    app::systems::{CameraSystem, RenderSystem},
    ecs::World,
    render::{Renderer, Vertex},
};

use super::{
    components::{Camera, Sprite, Transform},
    utils::{FrameTimer, Input},
};

pub struct App {
    input: Input,
    frame_timer: FrameTimer,
    world: World,

    renderer: Renderer,

    window: std::sync::Arc<winit::window::Window>,
}

impl App {
    pub fn new(window: winit::window::Window) -> Self {
        let window = std::sync::Arc::new(window);

        let input = Input::new();
        let frame_timer = FrameTimer::new(60);
        let mut world = World::new();
        world.register_component::<Camera>();
        world.register_component::<Sprite>();
        world.register_component::<Transform>();

        {
            let window_size = window.inner_size();

            let camera = world.spawn();
            world.insert_component(camera, Camera::new(window_size, 1.0));

            let e1 = world.spawn();
            world.insert_component(
                e1,
                Transform::new(glam::vec2(0.0, 0.0), glam::vec2(1.0, 1.0), 0.0),
            );
            world.insert_component(
                e1,
                Sprite::new(
                    "assets/uv_test.png",
                    winit::dpi::PhysicalSize::new(248, 248),
                ),
            );

            let e2 = world.spawn();
            world.insert_component(
                e2,
                Transform::new(
                    glam::vec2(0.0, window_size.height as f32),
                    glam::vec2(1.0, 1.0),
                    0.0,
                ),
            );
            world.insert_component(
                e2,
                Sprite::new(
                    "assets/uv_test.png",
                    winit::dpi::PhysicalSize::new(248, 248),
                ),
            );

            let e3 = world.spawn();
            world.insert_component(
                e3,
                Transform::new(
                    glam::vec2(window_size.width as f32, 0.0),
                    glam::vec2(1.0, 1.0),
                    0.0,
                ),
            );
            world.insert_component(
                e3,
                Sprite::new(
                    "assets/uv_test_2.png",
                    winit::dpi::PhysicalSize::new(248, 248),
                ),
            );

            let e4 = world.spawn();
            world.insert_component(
                e4,
                Transform::new(
                    glam::vec2(window_size.width as f32, window_size.height as f32),
                    glam::vec2(1.0, 1.0),
                    0.0,
                ),
            );
            world.insert_component(
                e4,
                Sprite::new(
                    "assets/uv_test_2.png",
                    winit::dpi::PhysicalSize::new(248, 248),
                ),
            );
        }

        let renderer = Renderer::new(window.clone());

        Self {
            input,
            frame_timer,
            world,

            renderer,

            window,
        }
    }

    pub fn window_event(
        &mut self,
        event: winit::event::WindowEvent,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) {
        match event {
            winit::event::WindowEvent::RedrawRequested => {
                let (fixed_deltas, delta) = self.frame_timer.advance();

                for fixed_delta in fixed_deltas {
                    // Fixed Delta
                }

                // Delta

                println!("input: {:?}", self.input.cursor_position);

                let (target, view) = self.renderer.create_render_target();
                let mut encoder = self.renderer.create_encoder();

                CameraSystem::prepare_projection(&self.world, &mut self.renderer);
                RenderSystem::prepare_sprites(&self.world, &mut self.renderer);

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("render_pass"),
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

                    self.renderer.render(&mut render_pass);
                }

                self.renderer.submit_and_present(target, encoder);

                self.window.request_redraw();
                self.input.process_end_frame();
            }
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                self.input.process_keyboard_input(&event);
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                self.input.process_mouse_input(&state, &button);
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.input.process_cursor_position(&position);
            }
            winit::event::WindowEvent::Resized(size) => self.renderer.resize(size),
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        };
    }
}
