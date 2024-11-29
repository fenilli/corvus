use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use super::{entity_pool::Entity, sparse_set::SparseSet};

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
    components: HashMap<TypeId, RefCell<Box<dyn ComponentSet>>>,
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
            .insert(type_id, RefCell::new(Box::new(SparseSet::<T>::new())));
    }

    pub fn unregister<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.components.contains_key(&type_id) {
            return;
        }

        self.components.remove(&type_id);
    }

    pub fn clear(&mut self, entity: Entity) {
        for (_, cell) in self.components.iter_mut() {
            let mut storage = cell.borrow_mut();

            let Some(sparse_set) = storage.as_any_mut().downcast_mut::<SparseSet<()>>() else {
                continue;
            };

            sparse_set.remove(entity);
        }
    }

    pub fn insert<T: 'static>(&self, entity: Entity, component: T) {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return;
        };

        let mut storage = cell.borrow_mut();

        let Some(sparse_set) = storage.as_any_mut().downcast_mut::<SparseSet<T>>() else {
            return;
        };

        sparse_set.insert(entity, component);
    }

    pub fn remove<T: 'static>(&self, entity: Entity) {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return;
        };

        let mut storage = cell.borrow_mut();

        let Some(sparse_set) = storage.as_any_mut().downcast_mut::<SparseSet<T>>() else {
            return;
        };

        sparse_set.remove(entity);
    }

    pub fn iter<T: 'static>(&self) -> Option<Ref<Vec<T>>> {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        Ref::filter_map(cell.borrow(), |component_vec| {
            component_vec
                .as_any()
                .downcast_ref::<SparseSet<T>>()
                .and_then(|sparse_set| Some(sparse_set.iter()))
        })
        .ok()
    }

    pub fn iter_mut<T: 'static>(&self) -> Option<RefMut<Vec<T>>> {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        RefMut::filter_map(cell.borrow_mut(), |component_vec| {
            component_vec
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .and_then(|sparse_set| Some(sparse_set.iter_mut()))
        })
        .ok()
    }
}
