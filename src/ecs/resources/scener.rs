use crate::{Resources, World};

pub trait Scene: 'static {
    fn enter(&mut self, world: &mut World, resources: &Resources);
    fn update(&mut self, world: &mut World, resources: &Resources);
    fn exit(&mut self, world: &mut World, resources: &Resources);
}

pub struct Scener {
    current_scene: Option<Box<dyn Scene>>,
    next_scene: Option<Box<dyn Scene>>,
}

impl Scener {
    pub fn new<T: Scene>(initial_scene: T) -> Self {
        Self {
            current_scene: None,
            next_scene: Some(Box::new(initial_scene)),
        }
    }

    pub fn change<T: Scene>(&mut self, new_scene: T) {
        self.next_scene = Some(Box::new(new_scene));
    }

    pub fn process(&mut self, world: &mut World, resources: &Resources) {
        let Some(mut next) = self.next_scene.take() else {
            return;
        };

        if let Some(current_scene) = &mut self.current_scene {
            current_scene.exit(world, resources);
        };

        next.enter(world, resources);
        self.current_scene = Some(next);
    }

    pub fn update(&mut self, world: &mut World, resources: &Resources) {
        let Some(current_scene) = &mut self.current_scene else {
            return;
        };

        current_scene.update(world, resources);
    }

    pub fn cleanup(&mut self, world: &mut World, resources: &Resources) {
        let Some(current_scene) = &mut self.current_scene else {
            return;
        };

        current_scene.exit(world, resources);
    }
}
