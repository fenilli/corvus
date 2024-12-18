use crate::{
    ecs::{resources::Scener, Resources},
    World,
};

pub struct ScenerSystem;

impl ScenerSystem {
    pub fn process(world: &mut World, resources: &Resources) {
        let Some(mut scener) = resources.get_resource_mut::<Scener>() else {
            return;
        };

        scener.process(world, resources);
    }

    pub fn update(world: &mut World, resources: &Resources) {
        let Some(mut scener) = resources.get_resource_mut::<Scener>() else {
            return;
        };

        scener.update(world, resources);
    }

    pub fn cleanup(world: &mut World, resources: &Resources) {
        let Some(mut scener) = resources.get_resource_mut::<Scener>() else {
            return;
        };

        scener.cleanup(world, resources);
    }
}
