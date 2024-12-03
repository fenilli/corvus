mod components;
mod systems;

use winit::window::Window;

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

        Self {
            input: Input::new(),
            clock: Clock::new(60),
            renderer: Renderer::new(window),
            world,
        }
    }

    pub fn update(&mut self) {
        for _delta_time in self.clock.update() {}

        // println!("@Update -> {:?}", self.world);
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.renderer.window()
    }
}
