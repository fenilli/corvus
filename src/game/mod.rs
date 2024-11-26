use std::sync::Arc;

use winit::window::Window;

use crate::world::World;

pub struct Game {
    world: World,
    window: Arc<Window>,
}

impl Game {
    pub fn new(window: Window) -> Self {
        Self {
            world: World::new(),
            window: Arc::new(window),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
