use crate::{ecs::Commands, resources::AssetManager, World};

pub struct AppContext<'a> {
    pub asset_manager: &'a mut AssetManager,
    pub commands: &'a mut Commands,
    pub world: &'a mut World,
}
