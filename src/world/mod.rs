mod component_manager;
mod entity_pool;
mod sparse_set;

use std::cell::{Ref, RefMut};

use component_manager::ComponentManager;
use entity_pool::{Entity, EntityPool};

pub struct World {
    entity_pool: EntityPool,
    component_manager: ComponentManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_pool: EntityPool::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_pool.allocate()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if self.entity_pool.deallocate(entity) {
            self.component_manager.clear(entity);
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register::<T>();
    }

    pub fn set_component<T: 'static>(&mut self, entity: Entity, component: T) {
        if !self.entity_pool.is_valid(entity) {
            return;
        }

        self.component_manager.insert::<T>(entity, component);
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        if !self.entity_pool.is_valid(entity) {
            return;
        }

        self.component_manager.remove::<T>(entity);
    }

    pub fn iter_components<T: 'static>(&self) -> Option<impl Iterator<Item = (Entity, Ref<T>)>> {
        self.component_manager.iter::<T>()
    }

    pub fn iter_components_mut<T: 'static>(
        &self,
    ) -> Option<impl Iterator<Item = (Entity, RefMut<T>)>> {
        self.component_manager.iter_mut::<T>()
    }
}
