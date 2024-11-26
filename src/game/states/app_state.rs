use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use super::{gpu_state::GpuState, tick_state::TickState};
use crate::{
    ecs::ECS,
    game::{components::TransformComponent, systems::movement_system},
};

pub struct AppState {
    world: ECS,
    tick_state: TickState,
    // test key input
    key_strength: i32,

    gpu_state: GpuState,
}

impl AppState {
    pub fn new(window: Window) -> Self {
        let mut world = ECS::new();
        world.register_component::<TransformComponent>();

        let player = world.create_entity();
        world.set_component(player, TransformComponent::new((100, 100), 0.0, 1));

        Self {
            world,
            tick_state: TickState::new(60),
            key_strength: 0,
            gpu_state: GpuState::new(window),
        }
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key,
                        state,
                        ..
                    },
                ..
            } => match state {
                ElementState::Pressed => match physical_key {
                    PhysicalKey::Code(code) => match code {
                        KeyCode::KeyA => self.key_strength = -10,
                        KeyCode::KeyD => self.key_strength = 10,
                        _ => (),
                    },
                    _ => (),
                },
                ElementState::Released => match physical_key {
                    PhysicalKey::Code(code) => match code {
                        KeyCode::KeyA => self.key_strength = 0,
                        KeyCode::KeyD => self.key_strength = 0,
                        _ => (),
                    },
                    _ => (),
                },
            },
            _ => (),
        }
    }

    pub fn update(&mut self) {
        for _ in 0..self.tick_state.update() {
            movement_system(&self.world, self.key_strength);
        }

        println!("{:?}", self.world.get_components::<TransformComponent>());

        std::thread::sleep(self.tick_state.tick_duration());
    }

    pub fn window(&self) -> &Window {
        &self.gpu_state.window()
    }
}
