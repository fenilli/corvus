mod ecs;

use ecs::ECS;

#[derive(Debug)]
pub struct Resource {
    key: &'static str,
}

impl Resource {
    pub fn new(key: &'static str) -> Self {
        Self { key }
    }
}

fn main() {
    let mut ecs = ECS::new();

    ecs.register_resource(Resource::new("Hello"));

    let e1 = ecs.create_entity();
    let e2 = ecs.create_entity();

    ecs.set_component::<u32>(e1, 10);
    ecs.set_component::<u32>(e2, 20);

    if let Some(health) = ecs.get_component::<u32>(e1) {
        let resource = ecs.get_resource::<Resource>();
        println!("{}, {:?}", health, resource);
    }

    if let Some(health) = ecs.get_component::<u32>(e2) {
        println!("{}", health);
    }

    ecs.destroy_entity(e1);

    if let Some(health) = ecs.get_component::<u32>(e1) {
        println!("{}", health);
    }
}
