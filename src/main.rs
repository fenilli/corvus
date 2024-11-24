mod ecs;

use std::{thread, time::Duration};

use ecs::ECS;

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

fn main() {
    let mut ticker = Ticker { ticks: 0 };

    let mut ecs = ECS::new();
    ecs.register_component::<PlayerTag>();
    ecs.register_component::<PositionComponent>();

    let player = ecs.create_entity();
    ecs.set_component(player, PlayerTag);
    ecs.set_component(player, PositionComponent::new(100, 100));

    let enemy = ecs.create_entity();
    ecs.set_component(enemy, PositionComponent::new(200, 200));

    loop {
        ticker.ticks += 1;

        update_player(&ecs, &ticker);

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
