use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn remove(&mut self, index: usize);
}

struct SparseSet<T> {
    data: Vec<T>,
    dense: Vec<usize>,
    sparse: HashMap<usize, usize>,
}

impl<T: 'static> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            dense: Vec::new(),
            sparse: HashMap::new(),
        }
    }

    fn insert(&mut self, index: usize, data: T) {
        if let Some(&dense_idx) = self.sparse.get(&index) {
            self.data[dense_idx] = data;
        } else {
            self.sparse.insert(index, self.dense.len());
            self.dense.push(index);
            self.data.push(data);
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.sparse
            .get(&index)
            .and_then(|&dense_idx| self.data.get(dense_idx))
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.sparse
            .get(&index)
            .and_then(|&dense_idx| self.data.get_mut(dense_idx))
    }
}

impl<T: 'static> ComponentVec for SparseSet<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, index: usize) {
        if let Some(&dense_idx) = self.sparse.get(&index) {
            let last_idx = self.dense.len() - 1;

            if dense_idx != last_idx {
                let temp_last_idx = self.dense[last_idx];
                self.dense[dense_idx] = temp_last_idx;
                self.sparse.insert(temp_last_idx, dense_idx);
                self.data.swap(dense_idx, last_idx);
            }

            self.dense.pop();
            self.data.pop();
            self.sparse.remove(&index);
        }
    }
}

pub struct ComponentManager {
    components: HashMap<TypeId, RefCell<Box<dyn ComponentVec>>>,
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

    pub fn insert<T: 'static>(&self, index: usize, component: T) {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return;
        };

        let mut storage = cell.borrow_mut();

        let Some(sparse_set) = storage.as_any_mut().downcast_mut::<SparseSet<T>>() else {
            return;
        };

        sparse_set.insert(index, component);
    }

    pub fn remove<T: 'static>(&self, index: usize) {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return;
        };

        let mut storage = cell.borrow_mut();

        let Some(sparse_set) = storage.as_any_mut().downcast_mut::<SparseSet<T>>() else {
            return;
        };

        sparse_set.remove(index);
    }

    pub fn clean(&mut self, index: usize) {
        for (_, component_vec) in self.components.iter_mut() {
            component_vec.borrow_mut().remove(index);
        }
    }

    pub fn get_all<T: 'static>(&self) -> Option<Ref<Vec<T>>> {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        Ref::filter_map(cell.borrow(), |component_vec| {
            component_vec
                .as_any()
                .downcast_ref::<SparseSet<T>>()
                .and_then(|sparse_set| Some(&sparse_set.data))
        })
        .ok()
    }

    pub fn get_all_mut<T: 'static>(&self) -> Option<RefMut<Vec<T>>> {
        let Some(cell) = self.components.get(&TypeId::of::<T>()) else {
            return None;
        };

        RefMut::filter_map(cell.borrow_mut(), |component_vec| {
            component_vec
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .and_then(|sparse_set| Some(&mut sparse_set.data))
        })
        .ok()
    }
}
