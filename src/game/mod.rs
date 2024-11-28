mod components;
mod systems;

// use components::{Position, Rotation, Scale, Transform, Velocity};
use std::sync::Arc;
use winit::window::Window;

use crate::{
    renderer::Renderer,
    resources::{Clock, Input},
    world::World,
};

// fn player_input_system(world: &World, input: &Input) {
//     let player_speed = 100.0;

//     for velocity in world.get_components_mut::<Velocity>().unwrap().iter_mut() {
//         if input.key_held(winit::keyboard::KeyCode::KeyA) {
//             velocity.y = -player_speed;
//         }

//         if input.key_held(winit::keyboard::KeyCode::KeyD) {
//             velocity.y = player_speed;
//         }
//     }
// }

// fn movement_system(world: &World, delta_time: f32) {
//     let velocities = world.get_components::<Velocity>().unwrap();
//     let mut transforms = world.get_components_mut::<Transform>().unwrap();

//     let iter = velocities.iter().zip(transforms.iter_mut());

//     for (velocity, transform) in iter {
//         transform.position.y += velocity.y * delta_time;

//         println!("x: {} y: {}", transform.position.x, transform.position.y);
//     }
// }

pub struct Game {
    input: Input,
    clock: Clock,
    renderer: Renderer,

    world: World,

    window: Arc<Window>,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let world = World::new();
        // world.register_component::<Velocity>();
        // world.register_component::<Transform>();

        // let player = world.create_entity();
        // world.set_component(
        //     player,
        //     Transform::new(
        //         Position::new(100.0, 100.0),
        //         Rotation::new(0.0),
        //         Scale::new(0.0),
        //     ),
        // );
        // world.set_component(player, Velocity::new(0.0, 0.0));

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            renderer: Renderer::new(),

            world,

            window: Arc::new(window),
        }
    }

    pub fn update(&mut self) {
        // player_input_system(&self.world, &self.input);

        for _delta_time in self.clock.update() {
            // movement_system(&self.world, delta_time);
        }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
