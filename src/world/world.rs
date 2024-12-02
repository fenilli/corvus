use std::{any::TypeId, collections::HashMap, fmt::Debug};

use super::{
    index_allocator::{Index, IndexAllocator},
    sparse_set::{AnySparseSet, SparseSet},
};

pub type Entity = Index;

pub trait Component: Debug + Send + Sync + 'static {}
impl<T: Debug + Send + Sync + 'static> Component for T {}

pub struct World {
    index_allocator: IndexAllocator,
    components: HashMap<TypeId, Box<dyn AnySparseSet>>,
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("index_allocator", &self.index_allocator)
            .field("components", &self.components)
            .finish()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            index_allocator: IndexAllocator::new(),
            components: HashMap::new(),
        }
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
            .insert(type_id, Box::new(SparseSet::<T>::new()));
    }

    pub fn unregister_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.components.contains_key(&type_id) {
            panic!(
                "Component \"{}\" was never registered!",
                std::any::type_name::<T>()
            );
        }

        self.components.remove(&type_id);
    }

    pub fn create_entity(&mut self) -> Entity {
        self.index_allocator.allocate()
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        if !self.index_allocator.deallocate(entity) {
            return;
        };

        for (_, any_sparse_set) in self.components.iter_mut() {
            any_sparse_set.remove(entity);
        }
    }

    pub fn set_component<T: Component>(&mut self, entity: Entity, component: T) {
        if !self.index_allocator.is_valid(entity) {
            panic!("Trying to add a component to an invalid entity")
        }

        let Some(any_sparse_set) = self.components.get_mut(&TypeId::of::<T>()) else {
            panic!(
                "Component {} is not registered!",
                std::any::type_name::<T>()
            )
        };

        let Some(sparse_set) = any_sparse_set.as_any_mut().downcast_mut::<SparseSet<T>>() else {
            panic!(
                "Component {} could not be downcasted!",
                std::any::type_name::<T>()
            )
        };

        sparse_set.insert(entity, component);
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        if !self.index_allocator.is_valid(entity) {
            panic!("Trying to add a component to an invalid entity")
        }

        let Some(component_set) = self.components.get_mut(&TypeId::of::<T>()) else {
            panic!(
                "Component {} is not registered!",
                std::any::type_name::<T>()
            )
        };

        component_set.remove(entity);
    }
}
