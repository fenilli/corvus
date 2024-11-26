use winit::window::Window;

use super::{gpu_state::GpuState, tick_state::TickState};
use crate::ecs::ECS;

pub struct AppState {
    world: ECS,
    tick_state: TickState,
    gpu_state: GpuState,
}

impl AppState {
    pub fn new(window: Window) -> Self {
        Self {
            world: ECS::new(),
            tick_state: TickState::new(60),
            gpu_state: GpuState::new(window),
        }
    }

    pub fn update(&mut self) {
        for _ in 0..self.tick_state.update() {
            println!("I'm ticking");
        }

        std::thread::sleep(self.tick_state.tick_duration());
    }

    pub fn window(&self) -> &Window {
        &self.gpu_state.window()
    }
}
