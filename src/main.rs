mod ecs;

use std::{thread, time::Duration};

use ecs::ECS;

#[derive(Debug)]
pub struct PositionComponent {
    x: f32,
    y: f32,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_u32(x: u32, y: u32) -> Self {
        Self::new(x as f32, y as f32)
    }
}

pub fn movement_system(ecs: &ECS) {
    let entities = ecs.entities();

    for entity in entities {
        if let Some(position) = ecs.get_component::<PositionComponent>(*entity) {
            println!("{:?}", position);
        }
    }
}

fn main() {
    let mut ecs = ECS::new();

    let player = ecs.create_entity();
    ecs.set_component(player, PositionComponent::from_u32(200, 200));

    loop {
        movement_system(&ecs);

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
