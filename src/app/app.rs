use winit::{event::WindowEvent, window::Window};

use super::{
    context::AppContext,
    input::Input,
    scene::{Scene, SceneManager},
    timestep::Timestep,
};
use crate::{
    ecs::{
        components::{Mesh, Transform},
        Commands,
    },
    geometry::Rectangle,
    resources::ResourceManager,
    World,
};

pub struct Menu;
impl Scene for Menu {
    fn enter(&mut self, context: &mut AppContext) {
        println!("enter");

        context.world.register::<Transform>();
        context.world.register::<Mesh>();

        let handle = context
            .resource_manager
            .meshes
            .add(Rectangle::new(100., 100.));

        context.commands.schedule(move |world| {
            let player = world.spawn();
            world.insert(player, Transform::from_xy(100., 100.));
            world.insert(player, Mesh::new(handle));
        });
    }

    fn fixed_update(&mut self, _delta_time: f32, context: &mut AppContext) {
        println!("fixed_update");

        let iter = context.world.entities().filter_map(|entity| {
            let (Some(transform), Some(mesh)) = (
                context.world.get_component::<Transform>(entity),
                context.world.get_component::<Mesh>(entity),
            ) else {
                return None;
            };

            Some((entity, transform, mesh))
        });

        for (entity, transform, mesh) in iter {
            println!(
                "@E: {} -> @T: {} -> @M: {}",
                entity.id, transform.position, mesh.handle.0
            );
        }
    }

    fn update(&mut self, _delta_time: f32, _context: &mut AppContext) {
        // println!("update {}", delta_time);
    }

    fn exit(&mut self, context: &mut AppContext) {
        println!("exit");

        context.world.unregister::<Transform>();
        context.world.unregister::<Mesh>();

        context.resource_manager.meshes.clear();
    }
}

#[allow(dead_code)]
pub struct App {
    input: Input,
    timestep: Timestep,
    world: World,
    commands: Commands,
    resource_manager: ResourceManager,
    scene_manager: SceneManager,

    window: Window,
}

impl App {
    pub fn new(window: Window) -> Self {
        let mut scene_manager = SceneManager::new();
        scene_manager.change(Menu);

        Self {
            input: Input::new(),
            timestep: Timestep::new(60),
            world: World::new(),
            commands: Commands::new(),
            resource_manager: ResourceManager::new(),
            scene_manager,

            window,
        }
    }

    pub fn window_event(&mut self, event: winit::event::WindowEvent) -> bool {
        self.input.start_step(&event);

        match event {
            WindowEvent::RedrawRequested => {
                let mut context = AppContext {
                    resource_manager: &mut self.resource_manager,
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
            _ => (),
        };

        true
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
