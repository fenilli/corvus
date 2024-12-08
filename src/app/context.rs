use crate::{ecs::Commands, resources::ResourceManager, World};

pub struct AppContext<'a> {
    pub resource_manager: &'a mut ResourceManager,
    pub commands: &'a mut Commands,
    pub world: &'a mut World,
}
