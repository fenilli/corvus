mod ecs;

use ecs::ECS;

fn main() {
    let mut ecs = ECS::new();

    let e1 = ecs.create_entity();
    let e2 = ecs.create_entity();

    ecs.set_component::<u32>(e1, 10);
    ecs.set_component::<u32>(e2, 20);

    if let Some(health) = ecs.get_component::<u32>(e1) {
        println!("{}", health);
    }

    if let Some(health) = ecs.get_component::<u32>(e2) {
        println!("{}", health);
    }

    ecs.destroy_entity(e1);

    if let Some(health) = ecs.get_component::<u32>(e1) {
        println!("{}", health);
    }
}
