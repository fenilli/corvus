use crate::ecs::{resources::Timestep, Resources};

pub struct TimestepSystem;

impl TimestepSystem {
    pub fn update(resources: &Resources) {
        let Some(mut timestep) = resources.get_resource_mut::<Timestep>() else {
            return;
        };

        timestep.update();
    }
}
