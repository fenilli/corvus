use crate::ecs::{resources::Input, Resources};

pub struct InputSystem;

impl InputSystem {
    pub fn process(resources: &Resources, event: &winit::event::WindowEvent) {
        let Some(mut input) = resources.get_resource_mut::<Input>() else {
            return;
        };

        input.process(event);
    }

    pub fn cleanup(resources: &Resources) {
        let Some(mut input) = resources.get_resource_mut::<Input>() else {
            return;
        };

        input.cleanup();
    }
}
