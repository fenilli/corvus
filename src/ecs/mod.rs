mod component_manager;
mod entity_pool;

use std::cell::{Ref, RefCell, RefMut};

use component_manager::ComponentManager;
use entity_pool::{Entity, EntityPool};

pub struct ECS {
    entity_pool: EntityPool,
    component_manager: ComponentManager,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_pool: EntityPool::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_pool.allocate()
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        if self.entity_pool.deallocate(entity) {
            self.component_manager.clean(entity.id());

            true
        } else {
            false
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register::<T>();
    }

    pub fn set_component<T: 'static>(&self, entity: Entity, component: T) {
        if !self.entity_pool.is_valid(entity) {
            return;
        }

        self.component_manager.insert(entity.id(), component);
    }

    pub fn get_components<T: 'static>(&self) -> Option<Ref<Vec<T>>> {
        self.component_manager.get_all::<T>()
    }

    pub fn get_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<T>>> {
        self.component_manager.get_all_mut::<T>()
    }
}
