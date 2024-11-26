use std::sync::Arc;

use winit::window::Window;

use crate::{input::Input, world::World};

pub struct Game {
    input: Input,
    world: World,

    window: Arc<Window>,
}

impl Game {
    pub fn new(window: Window) -> Self {
        Self {
            input: Input::new(),
            world: World::new(),
            window: Arc::new(window),
        }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
