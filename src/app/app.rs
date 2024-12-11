use std::sync::Arc;

use winit::{event::WindowEvent, window::Window};

use super::{
    context::AppContext,
    input::Input,
    scene::{Scene, SceneManager},
    timestep::Timestep,
};

use crate::{
    ecs::{
        components::{Rectangle, Sprite, Transform},
        systems::render_system,
        Commands,
    },
    render::Renderer,
    resources::AssetManager,
    World,
};

pub struct Game;
impl Scene for Game {
    fn enter(&mut self, context: &mut AppContext) {
        println!("enter");

        context.commands.schedule(move |world| {
            let player = world.spawn();
            world.insert(player, Sprite);
            world.insert(player, Transform::from_xy(100., 100.));
            world.insert(player, Rectangle::new(100, 100));

            let enemy = world.spawn();
            world.insert(enemy, Sprite);
            world.insert(enemy, Transform::from_xy(300., 300.));
            world.insert(enemy, Rectangle::new(50, 50));
        });
    }

    fn fixed_update(&mut self, _delta_time: f32, _context: &mut AppContext) {
        // println!("fixed_update");
    }

    fn update(&mut self, _delta_time: f32, _context: &mut AppContext) {
        // println!("update {}", delta_time);
    }

    fn exit(&mut self, _context: &mut AppContext) {
        // println!("exit");
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
        scene_manager.change(Game);

        let mut world = World::new();
        world.register::<Sprite>();
        world.register::<Transform>();
        world.register::<Rectangle>();

        Self {
            input: Input::new(),
            timestep: Timestep::new(60),
            world,
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
                    // asset_manager: &mut self.asset_manager,
                    commands: &mut self.commands,
                    // world: &mut self.world,
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
