use winit::window::Window;

use super::gpu_state::GpuState;
use crate::ecs::ECS;

#[derive(Debug)]
pub struct PlayerTag;

#[derive(Debug)]
pub struct PositionComponent {
    x: u32,
    y: u32,
}

impl PositionComponent {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

struct Ticker {
    ticks: u32,
}

fn update_player(ecs: &ECS, ticker: &Ticker) {
    let tags = ecs.get_components::<PlayerTag>().unwrap();
    let mut positions = ecs.get_components_mut::<PositionComponent>().unwrap();

    let iter = tags
        .iter()
        .zip(positions.iter_mut())
        .filter_map(|(tag, position)| Some((tag, position)));

    for (tag, position) in iter {
        println!("Tag: {:?} | Position: {:?}", tag, position);

        if ticker.ticks % 120 == 0 {
            position.x += 10;
        }
    }
}

pub struct AppState {
    world: ECS,
    ticker: Ticker,

    gpu_state: GpuState,
}

impl AppState {
    pub fn new(window: Window) -> Self {
        let mut world = ECS::new();
        world.register_component::<PlayerTag>();
        world.register_component::<PositionComponent>();

        let player = world.create_entity();
        world.set_component(player, PlayerTag);
        world.set_component(player, PositionComponent::new(100, 100));

        Self {
            world,
            ticker: Ticker { ticks: 0 },

            gpu_state: GpuState::new(window),
        }
    }

    pub fn update(&mut self) {
        self.ticker.ticks += 1;

        update_player(&self.world, &self.ticker);
    }

    pub fn window(&self) -> &Window {
        &self.gpu_state.window()
    }
}
