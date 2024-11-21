mod entity_manager;
mod storages;

use std::{any::TypeId, collections::HashMap};

use entity_manager::{Entity, EntityManager};
use storages::{component_storage::ComponentStorage, dyn_storage::DynStorage};

pub struct ECS {
    entity_manager: EntityManager,

    component_storages: HashMap<TypeId, Box<dyn DynStorage>>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),

            component_storages: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.allocate()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if self.entity_manager.deallocate(entity) {
            for component_storage in self.component_storages.values_mut() {
                component_storage.remove(entity);
            }
        }
    }

    pub fn set_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.component_storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStorage::<T>::new()))
            .downcast_mut::<ComponentStorage<T>>()
            .and_then(|component_storage| Some(component_storage.insert(entity, component)));
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.component_storages
            .get(&TypeId::of::<T>())
            .and_then(|component_storage| {
                component_storage
                    .downcast_ref::<ComponentStorage<T>>()?
                    .get(entity)
            })
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.component_storages
            .get_mut(&TypeId::of::<T>())
            .and_then(|component_storage| {
                component_storage
                    .downcast_mut::<ComponentStorage<T>>()?
                    .get_mut(entity)
            })
    }
}
