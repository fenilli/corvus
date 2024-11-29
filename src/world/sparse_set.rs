use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use super::entity_pool::Entity;

pub struct SparseSet<T> {
    data: RefCell<Vec<T>>,
    dense: Vec<Entity>,
    sparse: HashMap<Entity, usize>,
}

impl<T: 'static> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(Vec::new()),
            dense: Vec::new(),
            sparse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, data: T) {
        let mut storage = self.data.borrow_mut();

        if let Some(&dense_idx) = self.sparse.get(&entity) {
            storage[dense_idx] = data;
        } else {
            self.sparse.insert(entity, self.dense.len());
            self.dense.push(entity);
            storage.push(data);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        let mut storage = self.data.borrow_mut();

        if let Some(&dense_idx) = self.sparse.get(&entity) {
            let last_idx = self.dense.len() - 1;

            if dense_idx != last_idx {
                let temp_last_idx = self.dense[last_idx];
                self.dense[dense_idx] = temp_last_idx;
                self.sparse.insert(temp_last_idx, dense_idx);
                storage.swap(dense_idx, last_idx);
            }

            self.dense.pop();
            storage.pop();
            self.sparse.remove(&entity);
        }
    }

    pub fn get(&self, entity: Entity) -> Option<Ref<T>> {
        Ref::filter_map(self.data.borrow(), |storage| {
            self.sparse
                .get(&entity)
                .and_then(|&dense_idx| storage.get(dense_idx))
        })
        .ok()
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<RefMut<T>> {
        RefMut::filter_map(self.data.borrow_mut(), |storage| {
            self.sparse
                .get(&entity)
                .and_then(|&dense_idx| storage.get_mut(dense_idx))
        })
        .ok()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, Ref<T>)> {
        self.dense.iter().enumerate().map(|(dense_index, &entity)| {
            (
                entity,
                Ref::map(self.data.borrow(), |data| &data[dense_index]),
            )
        })
    }

    pub fn iter_mut(&self) -> impl Iterator<Item = (Entity, RefMut<T>)> {
        self.dense.iter().enumerate().map(|(dense_index, &entity)| {
            (
                entity,
                RefMut::map(self.data.borrow_mut(), |data| &mut data[dense_index]),
            )
        })
    }
}
