mod entity_manager;
mod storages;

use std::any::TypeId;

use entity_manager::{Entity, EntityManager};
use storages::{
    component_storage::{ComponentMap, ComponentStorage, DynStorage},
    resource_storage::{ResourceMap, ResourceStorage},
};

pub struct ECS {
    entity_manager: EntityManager,

    component_map: ComponentMap,
    resource_map: ResourceMap,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),

            component_map: ComponentMap::new(),
            resource_map: ResourceMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.allocate()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if self.entity_manager.deallocate(entity) {
            for component_storage in self.component_map.values_mut() {
                component_storage.remove(entity);
            }
        }
    }

    pub fn set_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.component_map
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStorage::<T>::new()))
            .downcast_mut::<ComponentStorage<T>>()
            .and_then(|component_storage| Some(component_storage.insert(entity, component)));
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        if let Some(dyn_storage) = self.component_map.get_mut(&TypeId::of::<T>()) {
            if let Some(component_storage) = dyn_storage.downcast_mut::<ComponentStorage<T>>() {
                component_storage.remove(entity);
            }
        }
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.component_map
            .get(&TypeId::of::<T>())
            .and_then(|component_storage| {
                component_storage
                    .downcast_ref::<ComponentStorage<T>>()?
                    .get(entity)
            })
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.component_map
            .get_mut(&TypeId::of::<T>())
            .and_then(|component_storage| {
                component_storage
                    .downcast_mut::<ComponentStorage<T>>()?
                    .get_mut(entity)
            })
    }

    pub fn register_resource<T: 'static>(&mut self, resource: T) {
        self.resource_map.insert(
            TypeId::of::<T>(),
            Box::new(ResourceStorage::insert(resource)),
        );
    }

    pub fn remove_resource<T: 'static>(&mut self) {
        self.resource_map.remove(&TypeId::of::<T>());
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resource_map
            .get(&TypeId::of::<T>())
            .and_then(|dyn_storage| Some(dyn_storage.downcast_ref::<ResourceStorage<T>>()?.get()))
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resource_map
            .get_mut(&TypeId::of::<T>())
            .and_then(|dyn_storage| {
                Some(dyn_storage.downcast_mut::<ResourceStorage<T>>()?.get_mut())
            })
    }
}
