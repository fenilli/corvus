mod components;
mod systems;

use components::Quad;
use components::{Position, Rotation, Scale, Transform, Velocity};
use systems::quad_system;
use winit::window::Window;

use crate::{
    renderer::Renderer,
    resources::{Clock, Input},
    world::World,
};

fn player_input_system(world: &World, input: &Input) {
    let player_speed = 100.0;

    let velocities = world.iter_components_mut::<Velocity>().unwrap();

    for (_, mut velocity) in velocities {
        if input.key_held(winit::keyboard::KeyCode::KeyA) {
            velocity.y = -player_speed;
        }

        if input.key_held(winit::keyboard::KeyCode::KeyD) {
            velocity.y = player_speed;
        }
    }
}

fn movement_system(world: &World, delta_time: f32) {
    let velocities = world.iter_components::<Velocity>().unwrap();
    let transforms = world.iter_components_mut::<Transform>().unwrap();

    let iter = velocities
        .zip(transforms)
        .filter_map(|((entity, velocity), (_, transform))| Some((entity, velocity, transform)));

    for (_, velocity, mut transform) in iter {
        transform.position.y += velocity.y * delta_time;

        println!("x: {} y: {}", transform.position.x, transform.position.y);
    }
}

pub struct Game {
    input: Input,
    clock: Clock,
    renderer: Renderer,

    world: World,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let mut world = World::new();
        // world.register_component::<Quad>();
        world.register_component::<Transform>();
        world.register_component::<Velocity>();

        let player = world.create_entity();
        world.set_component(
            player,
            Transform::new(
                Position::new(100.0, 100.0),
                Rotation::new(0.0),
                Scale::new(0.0),
            ),
        );
        world.set_component(player, Velocity::new(0.0, 0.0));

        Self {
            input: Input::new(),
            clock: Clock::new(60),

            world,

            renderer: Renderer::new(window),
        }
    }

    pub fn update(&mut self) {
        player_input_system(&self.world, &self.input);
        // quad_system(&self.world, &mut self.renderer);

        for delta_time in self.clock.update() {
            movement_system(&self.world, delta_time);
        }

        // std::thread::sleep(std::time::Duration::from_secs(1));
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
