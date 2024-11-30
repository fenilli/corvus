use std::{
    any::{Any, TypeId},
    cell::{Ref, RefMut},
    collections::HashMap,
};

use super::{index_allocator::Index, sparse_set::SparseSet};

trait ComponentSet {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> ComponentSet for SparseSet<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ComponentManager {
    components: HashMap<TypeId, Box<dyn ComponentSet>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.components.contains_key(&type_id) {
            return;
        }

        self.components
            .insert(type_id, Box::new(SparseSet::<T>::new()));
    }

    pub fn unregister<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.components.contains_key(&type_id) {
            return;
        }

        self.components.remove(&type_id);
    }

    pub fn clear(&mut self, index: Index) {
        for (_, component_set) in self.components.iter_mut() {
            let Some(sparse_set) = component_set.as_any_mut().downcast_mut::<SparseSet<()>>()
            else {
                continue;
            };

            sparse_set.remove(index);
        }
    }

    pub fn insert<T: 'static>(&mut self, index: Index, component: T) {
        let Some(component_set) = self.components.get_mut(&TypeId::of::<T>()) else {
            return;
        };

        let Some(sparse_set) = component_set.as_any_mut().downcast_mut::<SparseSet<T>>() else {
            return;
        };

        sparse_set.insert(index, component);
    }

    pub fn remove<T: 'static>(&mut self, index: Index) {
        let Some(component_set) = self.components.get_mut(&TypeId::of::<T>()) else {
            return;
        };

        let Some(sparse_set) = component_set.as_any_mut().downcast_mut::<SparseSet<T>>() else {
            return;
        };

        sparse_set.remove(index);
    }

    pub fn iter<T: 'static>(&self) -> Option<impl Iterator<Item = (Index, Ref<T>)>> {
        let Some(component_set) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        let Some(sparse_set) = component_set.as_any().downcast_ref::<SparseSet<T>>() else {
            return None;
        };

        Some(sparse_set.iter())
    }

    pub fn iter_mut<T: 'static>(&self) -> Option<impl Iterator<Item = (Index, RefMut<T>)>> {
        let Some(component_set) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        let Some(sparse_set) = component_set.as_any().downcast_ref::<SparseSet<T>>() else {
            return None;
        };

        Some(sparse_set.iter_mut())
    }
}
