use std::{
    any::TypeId,
    cell::{Ref, RefMut},
    collections::HashMap,
    fmt::Debug,
};

use super::{
    component_vec::{AnyVec, ComponentVec},
    entity_allocator::{Entity, EntityAllocator},
};

pub trait Component: Debug + Send + Sync + 'static {}
impl<T: Debug + Send + Sync + 'static> Component for T {}

pub struct World {
    entity_allocator: EntityAllocator,
    components: HashMap<TypeId, Box<dyn AnyVec>>,
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("entity_allocator", &self.entity_allocator)
            .field("components", &self.components)
            .finish()
    }
}

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

    pub fn register<T: Component>(&mut self) {
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

    pub fn unregister<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.components.contains_key(&type_id) {
            panic!(
                "Component \"{}\" was never registered!",
                std::any::type_name::<T>()
            );
        }

        self.components.remove(&type_id);
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

    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        let Some(index) = self.entity_allocator.find_entity_index(entity) else {
            return;
        };

        let type_id = TypeId::of::<T>();

        let Some(any_vec) = self.components.get_mut(&type_id) else {
            return;
        };

        let Some(storage) = any_vec.as_any_mut().downcast_mut::<ComponentVec<T>>() else {
            return;
        };

        storage.insert(index, component);
    }

    pub fn remove<T: Component>(&mut self, entity: Entity) {
        let Some(index) = self.entity_allocator.find_entity_index(entity) else {
            return;
        };

        let type_id = TypeId::of::<T>();

        let Some(any_vec) = self.components.get_mut(&type_id) else {
            return;
        };

        let Some(storage) = any_vec.as_any_mut().downcast_mut::<ComponentVec<T>>() else {
            return;
        };

        storage.remove(index);
    }

    pub fn components<T: Component>(&self) -> Option<Ref<Vec<Option<T>>>> {
        let type_id = TypeId::of::<T>();

        let Some(any_vec) = self.components.get(&type_id) else {
            return None;
        };

        let Some(storage) = any_vec.as_any().downcast_ref::<ComponentVec<T>>() else {
            return None;
        };

        Some(storage.components())
    }

    pub fn components_mut<T: Component>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        let type_id = TypeId::of::<T>();

        let Some(any_vec) = self.components.get(&type_id) else {
            return None;
        };

        let Some(storage) = any_vec.as_any().downcast_ref::<ComponentVec<T>>() else {
            return None;
        };

        Some(storage.components_mut())
    }
}
