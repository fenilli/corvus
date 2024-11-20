mod any_map;
mod entity;

use std::any::Any;

use any_map::AnyMap;
use entity::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexVec};

pub struct ECS {
    entity_allocator: GenerationalIndexAllocator,

    entity_components: AnyMap,
    resources: AnyMap,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_allocator: GenerationalIndexAllocator::new(),
            entity_components: AnyMap::new(),
            resources: AnyMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> GenerationalIndex {
        self.entity_allocator.allocate()
    }

    pub fn set_component<T: 'static>(&mut self, index: GenerationalIndex, component: T) {
        let component_map = self.entity_components.get_mut::<GenerationalIndexVec<T>>();

        match component_map {
            None => {
                self.entity_components.set(GenerationalIndexVec::<T>::new());
                let component_map = self
                    .entity_components
                    .get_mut::<GenerationalIndexVec<T>>()
                    .unwrap();

                component_map.set(index, component);
            }
            Some(component_map) => {
                component_map.set(index, component);
            }
        };
    }

    pub fn get_component<T: 'static>(&self, index: GenerationalIndex) -> Option<&T> {
        self.entity_components
            .get::<GenerationalIndexVec<T>>()
            .and_then(|component_map| component_map.get(index))
    }

    pub fn get_component_mut<T: 'static>(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        self.entity_components
            .get_mut::<GenerationalIndexVec<T>>()
            .and_then(|component_map| component_map.get_mut(index))
    }
}
