use crate::ecs::resources::Scene;

pub struct Game;

impl Scene for Game {
    fn enter(&mut self, world: &mut crate::World, resources: &crate::Resources) {
        println!("Entered Game Scene");
    }

    fn update(&mut self, world: &mut crate::World, resources: &crate::Resources) {
        println!("Updating Game Scene");
    }

    fn exit(&mut self, world: &mut crate::World, resources: &crate::Resources) {
        println!("Exiting Game Scene");
    }
}
