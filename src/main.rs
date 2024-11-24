mod ecs;

use std::{thread, time::Duration};

use ecs::ECS;

#[derive(Debug)]
pub struct HealthComponent(i16);

#[derive(Debug)]
pub struct NameComponent(&'static str);

fn main() {
    let mut ecs = ECS::new();
    ecs.register_component::<HealthComponent>();
    ecs.register_component::<NameComponent>();

    let player = ecs.create_entity();
    ecs.set_component(player, HealthComponent(10));
    ecs.set_component(player, NameComponent("Fenilli"));

    let enemy = ecs.create_entity();
    ecs.set_component(enemy, HealthComponent(5));
    ecs.set_component(enemy, NameComponent("Globin"));

    loop {
        let names = ecs.get_components::<NameComponent>().unwrap();
        let mut healths = ecs.get_components_mut::<HealthComponent>().unwrap();

        let iter = names
            .iter()
            .zip(healths.iter_mut())
            .filter_map(|(name, health)| Some((name, health)));

        for (name, health) in iter {
            println!("Name {:?} - Health {:?}", name.0, health.0);
        }

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
