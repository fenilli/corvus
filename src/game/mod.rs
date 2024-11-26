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

    pub fn update(&mut self) {
        // if self.input.key_pressed(winit::keyboard::KeyCode::KeyW) {
        //     println!("Pressed W");
        // }

        // if self.input.key_held(winit::keyboard::KeyCode::KeyW) {
        //     println!("Holding W");
        // }

        // if self.input.key_released(winit::keyboard::KeyCode::KeyW) {
        //     println!("Released W");
        // }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
