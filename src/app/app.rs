use std::sync::Arc;

use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use super::{
    context::AppContext, input::Input, scene::SceneManager, scenes::game::Game, timestep::Timestep,
};

use crate::{
    ecs::{systems::render_system, Commands},
    render::Renderer,
    World,
};

pub struct SystemConfiguration {
    pub window_size: PhysicalSize<u32>,
}

#[allow(dead_code)]
pub struct App {
    input: Input,
    timestep: Timestep,
    world: World,
    commands: Commands,

    scene_manager: SceneManager,

    renderer: Renderer,
    window: Arc<Window>,
}

impl App {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);

        let mut scene_manager = SceneManager::new();
        scene_manager.change(Game::new());

        Self {
            input: Input::new(),
            timestep: Timestep::new(60),
            world: World::new(),
            commands: Commands::new(),

            scene_manager,

            renderer: Renderer::new(window.clone()),
            window,
        }
    }

    pub fn window_event(&mut self, event: winit::event::WindowEvent) -> bool {
        self.input.start_step(&event);

        match event {
            WindowEvent::RedrawRequested => {
                let mut context = AppContext {
                    system_configuration: SystemConfiguration {
                        window_size: self.window.inner_size(),
                    },
                    input: &mut self.input,
                    commands: &mut self.commands,
                    world: &mut self.world,
                };

                self.scene_manager.process(&mut context);

                let (fixed_deltas, variable_delta) = self.timestep.update();

                for delta_time in fixed_deltas {
                    self.scene_manager.fixed_update(delta_time, &mut context);
                }

                self.scene_manager.update(variable_delta, &mut context);

                render_system(&mut self.world, &mut self.renderer);

                self.commands.execute(&mut self.world);
                self.input.end_step();
            }
            WindowEvent::CloseRequested => return false,
            WindowEvent::Resized(size) => self.renderer.resize(size),
            _ => (),
        };

        true
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
