use crate::{
    app::systems::RenderSystem,
    assets::{atlas::Atlas, AssetRegistry},
    ecs::World,
    render::Renderer,
};

use super::{
    components::{Animation, AnimationSet, AnimationState, Camera, Flip, Sprite, Transform},
    systems::{AnimationSystem, AssetSystem},
    utils::{FrameTimer, Input},
};

pub struct App {
    input: Input,
    frame_timer: FrameTimer,
    asset_registry: AssetRegistry,
    world: World,

    renderer: Renderer,

    window: std::sync::Arc<winit::window::Window>,
}

impl App {
    pub fn new(window: winit::window::Window) -> Self {
        let window = std::sync::Arc::new(window);

        let input = Input::new();
        let frame_timer = FrameTimer::new(60);
        let mut asset_registry = AssetRegistry::new();
        let mut world = World::new();
        world.register_component::<Camera>();
        world.register_component::<Sprite>();
        world.register_component::<Transform>();
        world.register_component::<AnimationSet>();
        world.register_component::<AnimationState>();
        world.register_component::<Flip>();

        {
            let window_size = window.inner_size();

            let camera = world.spawn();
            world.insert_component(
                camera,
                Camera::new(glam::Vec2::new(0.0, 0.0), window_size, 1.0),
            );

            let e1 = world.spawn();
            world.insert_component(
                e1,
                Transform::new(
                    glam::vec3(window_size.width as f32 / 2.0, 0.0, 0.0),
                    glam::vec2(1.0, 1.0),
                    0.0,
                    glam::vec2(0.0, 1.0),
                ),
            );

            world.insert_component(
                e1,
                Sprite::new(
                    asset_registry.load_atlas(Atlas::from_grid(
                        "assets/idle.png",
                        16,
                        16,
                        32,
                        32,
                        3,
                        4,
                        Some(|row, col| format!("idle_{}_{}", row, col)),
                    )),
                    "idle_0_3",
                ),
            );

            world.insert_component(e1, Flip::new(true, false));

            let mut animation_set = AnimationSet::new();
            animation_set.add_animation(
                "idle_side",
                Animation::with_duration(
                    vec!["idle_0_0", "idle_0_1", "idle_0_2", "idle_0_3"],
                    true,
                    5.0,
                ),
            );
            world.insert_component(e1, animation_set);
            world.insert_component(e1, AnimationState::new("idle_side"));
        }

        let renderer = Renderer::new(window.clone());

        Self {
            input,
            frame_timer,
            asset_registry,
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

                let (target, view) = self.renderer.create_render_target();
                let mut encoder = self.renderer.create_encoder();

                AssetSystem::load_textures_from_assets(
                    &self.world,
                    &self.asset_registry,
                    &mut self.renderer,
                );

                AnimationSystem::run_animations(
                    &self.world,
                    self.frame_timer.interpolation_alpha(),
                );

                RenderSystem::prepare_projection(&self.world, &mut self.renderer);
                RenderSystem::prepare_sprites(
                    &self.world,
                    &self.asset_registry,
                    &mut self.renderer,
                );

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
