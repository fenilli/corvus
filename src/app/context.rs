use crate::ecs::Commands;

pub struct AppContext<'a> {
    // pub asset_manager: &'a mut AssetManager,
    pub commands: &'a mut Commands,
    // pub world: &'a mut World,
}
