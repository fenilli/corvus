mod components;
mod systems;

use systems::quad_system;
use winit::window::Window;

use components::{Mesh, Quad};

use crate::{
    renderer::Renderer,
    resources::{Clock, Input},
    world::World,
};

pub struct Game {
    input: Input,
    clock: Clock,
    renderer: Renderer,
    world: World,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let mut world = World::new();
        world.register::<Quad>();
        world.register::<Mesh>();

        let player = world.spawn();
        world.insert(
            player,
            Quad {
                width: 100,
                height: 100,
            },
        );

        println!("@Init -> {:?}", world);

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            renderer: Renderer::new(window),
            world,
        }
    }

    pub fn update(&mut self) {
        quad_system(&mut self.world, &mut self.renderer);

        for _delta_time in self.clock.update() {}

        println!("@Update -> {:?}", self.world);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
