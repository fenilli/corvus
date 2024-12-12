use crate::{ecs::Commands, World};

use super::{app::SystemConfiguration, input::Input};

pub struct AppContext<'a> {
    pub system_configuration: SystemConfiguration,
    pub input: &'a mut Input,
    pub commands: &'a mut Commands,
    pub world: &'a mut World,
}
