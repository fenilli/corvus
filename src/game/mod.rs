mod components;
mod systems;

use std::thread::sleep;
use systems::quad_system;
use winit::window::Window;

use components::{MeshComponent, QuadComponent};

use crate::{
    renderer::Renderer,
    resources::{AssetLoader, Clock, Input},
    world::World,
};

pub struct Game {
    input: Input,
    clock: Clock,
    asset_loader: AssetLoader,

    world: World,

    renderer: Renderer,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let mut world = World::new();
        world.register::<MeshComponent>();
        world.register::<QuadComponent>();

        let player = world.spawn();
        world.insert(
            player,
            QuadComponent {
                height: 100.0,
                width: 100.0,
            },
        );

        println!("@Init -> {:?}\n", world);

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            asset_loader: AssetLoader::new(),

            world,

            renderer: Renderer::new(window),
        }
    }

    pub fn update(&mut self) {
        for _delta_time in self.clock.update() {
            quad_system(&mut self.world);

            println!("@Update -> {:?}\n", self.world);
        }

        sleep(self.clock.frame_duration());
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
