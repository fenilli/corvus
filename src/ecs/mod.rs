mod component_manager;
mod entity_pool;

use std::cell::{Ref, RefMut};

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

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register::<T>();
    }

    pub fn create_entity(&self) -> Entity {
        self.entity_pool.allocate()
    }

    pub fn destroy_entity(&self, entity: Entity) -> bool {
        self.entity_pool.deallocate(entity)
    }

    pub fn set_component<T: 'static>(&self, entity: Entity, component: T) {
        if !self.entity_pool.is_valid(entity) {
            return;
        }

        self.component_manager.insert(entity.id(), component);
    }

    pub fn get_components<T: 'static>(&self) -> Option<Ref<'_, Vec<T>>> {
        self.component_manager.get_components()
    }

    pub fn get_components_mut<T: 'static>(&self) -> Option<RefMut<'_, Vec<T>>> {
        self.component_manager.get_components_mut()
    }
}
