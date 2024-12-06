mod components;
mod systems;

use std::thread::sleep;
use winit::window::Window;

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
