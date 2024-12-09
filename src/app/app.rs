use std::sync::Arc;

use winit::{event::WindowEvent, window::Window};

use super::{
    context::AppContext,
    input::Input,
    scene::{Scene, SceneManager},
    timestep::Timestep,
};

use crate::{
    ecs::{components::Transform, Commands},
    render::Renderer,
    resources::AssetManager,
    World,
};

pub struct Menu;
impl Scene for Menu {
    fn enter(&mut self, context: &mut AppContext) {
        println!("enter");

        context.world.register::<Transform>();

        context.commands.schedule(move |world| {
            let player = world.spawn();
            world.insert(player, Transform::from_xy(100., 100.));
        });
    }

    fn fixed_update(&mut self, _delta_time: f32, context: &mut AppContext) {
        println!("fixed_update");

        let iter = context.world.entities().filter_map(|entity| {
            let Some(transform) = context.world.get_component::<Transform>(entity) else {
                return None;
            };

            Some((entity, transform))
        });

        for (entity, transform) in iter {
            println!("@E: {} -> @T: {}", entity.id, transform.position);
        }
    }

    fn update(&mut self, _delta_time: f32, _context: &mut AppContext) {
        // println!("update {}", delta_time);
    }

    fn exit(&mut self, context: &mut AppContext) {
        println!("exit");
    }
}

#[allow(dead_code)]
pub struct App {
    input: Input,
    timestep: Timestep,
    world: World,
    commands: Commands,

    asset_manager: AssetManager,
    scene_manager: SceneManager,

    renderer: Renderer,
    window: Arc<Window>,
}

impl App {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);

        let mut scene_manager = SceneManager::new();
        scene_manager.change(Menu);

        Self {
            input: Input::new(),
            timestep: Timestep::new(60),
            world: World::new(),
            commands: Commands::new(),

            asset_manager: AssetManager::new(),
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
                    asset_manager: &mut self.asset_manager,
                    commands: &mut self.commands,
                    world: &mut self.world,
                };

                self.scene_manager.process(&mut context);

                let (fixed_deltas, variable_delta) = self.timestep.update();

                for delta_time in fixed_deltas {
                    self.scene_manager.fixed_update(delta_time, &mut context);
                }

                self.scene_manager.update(variable_delta, &mut context);

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
