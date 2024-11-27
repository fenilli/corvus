use std::sync::Arc;

use winit::window::Window;

use crate::{clock::Clock, input::Input, world::World};

pub struct Game {
    world: World,
    input: Input,
    clock: Clock,

    window: Arc<Window>,
}

impl Game {
    pub fn new(window: Window) -> Self {
        Self {
            world: World::new(),
            input: Input::new(),
            clock: Clock::new(60),

            window: Arc::new(window),
        }
    }

    pub fn update(&mut self) {
        for delta_time in self.clock.update() {
            println!("{}", delta_time);
        }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
