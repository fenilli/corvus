use std::{
    any::TypeId,
    cell::{Ref, RefMut},
    collections::HashMap,
};

use super::{
    component::{AnyVec, Component, ComponentVec},
    entity_allocator::EntityAllocator,
};

pub use super::entity_allocator::Entity;

pub struct World {
    entity_allocator: EntityAllocator,
    components: HashMap<TypeId, Box<dyn AnyVec>>,
}

#[allow(dead_code)]
impl World {
    pub fn new() -> Self {
        Self {
            entity_allocator: EntityAllocator::new(),
            components: HashMap::new(),
        }
    }

    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entity_allocator.entities()
    }

    pub fn register_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.components.contains_key(&type_id) {
            panic!(
                "Component \"{}\" was already registered!",
                std::any::type_name::<T>()
            );
        }

        self.components
            .insert(type_id, Box::new(ComponentVec::<T>::new()));
    }

    pub fn insert_component<T: Component>(&mut self, entity: Entity, component: T) {
        let Some(index) = self.entity_allocator.find_entity_index(entity) else {
            return;
        };

        let Some(any_vec) = self.components.get_mut(&TypeId::of::<T>()) else {
            return;
        };

        let Some(storage) = any_vec.as_any_mut().downcast_mut::<ComponentVec<T>>() else {
            return;
        };

        storage.insert(index, component);
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        let Some(index) = self.entity_allocator.find_entity_index(entity) else {
            return;
        };

        let Some(any_vec) = self.components.get_mut(&TypeId::of::<T>()) else {
            return;
        };

        let Some(storage) = any_vec.as_any_mut().downcast_mut::<ComponentVec<T>>() else {
            return;
        };

        storage.remove(index);
    }

    pub fn single<T: Component>(&self) -> Option<Ref<T>> {
        self.entities()
            .filter_map(|entity| self.get_component::<T>(entity))
            .next()
    }

    pub fn single_mut<T: Component>(&self) -> Option<RefMut<T>> {
        self.entities()
            .filter_map(|entity| self.get_component_mut::<T>(entity))
            .next()
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<Ref<T>> {
        let Some(any_vec) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        let Some(storage) = any_vec.as_any().downcast_ref::<ComponentVec<T>>() else {
            return None;
        };

        storage.get(entity.id)
    }

    pub fn get_component_mut<T: Component>(&self, entity: Entity) -> Option<RefMut<T>> {
        let Some(any_vec) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        let Some(storage) = any_vec.as_any().downcast_ref::<ComponentVec<T>>() else {
            return None;
        };

        storage.get_mut(entity.id)
    }

    pub fn spawn(&mut self) -> Entity {
        let entity = self.entity_allocator.allocate();

        for storage in self.components.values_mut() {
            storage.default();
        }

        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entity_allocator.deallocate(entity) {
            return false;
        };

        for storage in self.components.values_mut() {
            storage.swap_remove(entity.id);
        }

        true
    }
}
