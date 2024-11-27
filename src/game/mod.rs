use std::sync::Arc;

use winit::window::Window;

use crate::{clock::Clock, input::Input, world::World};

#[derive(Debug)]
struct TransformComponent {
    position: (f64, f64),
    rotation: f32,
    scale: u32,
}

fn player_movement(world: &World, input: &Input, delta_time: f64) {
    let player_speed = 100.0;

    for transform in world
        .get_components_mut::<TransformComponent>()
        .unwrap()
        .iter_mut()
    {
        if input.key_held(winit::keyboard::KeyCode::KeyA) {
            transform.position.1 -= player_speed * delta_time;
        }

        if input.key_held(winit::keyboard::KeyCode::KeyD) {
            transform.position.1 += player_speed * delta_time;
        }
    }
}

fn print_position(world: &World) {
    for transform in world
        .get_components_mut::<TransformComponent>()
        .unwrap()
        .iter_mut()
    {
        println!("x: {} y: {}", transform.position.0, transform.position.1);
    }
}

pub struct Game {
    world: World,
    input: Input,
    clock: Clock,

    window: Arc<Window>,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let mut world = World::new();
        world.register_component::<TransformComponent>();

        let player = world.create_entity();
        world.set_component(
            player,
            TransformComponent {
                position: (100.0, 100.0),
                rotation: 0.0,
                scale: 1,
            },
        );

        Self {
            world,
            input: Input::new(),
            clock: Clock::new(60),

            window: Arc::new(window),
        }
    }

    pub fn update(&mut self) {
        for delta_time in self.clock.update() {
            player_movement(&self.world, &self.input, delta_time);
        }

        print_position(&self.world);
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
