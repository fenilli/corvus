use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt::Debug,
};

use super::{index_allocator::Index, world::Component};

pub struct SparseSet<T> {
    data: RefCell<Vec<T>>,
    dense: Vec<Index>,
    sparse: HashMap<Index, usize>,
}

impl<T: Debug> Debug for SparseSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SparseSet")
            .field("data", &self.data.borrow())
            .field("dense", &self.dense)
            .field("sparse", &self.sparse)
            .finish()
    }
}

impl<T: 'static> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(Vec::new()),
            dense: Vec::new(),
            sparse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, index: Index, data: T) {
        let mut storage = self.data.borrow_mut();

        if let Some(&dense_idx) = self.sparse.get(&index) {
            storage[dense_idx] = data;
        } else {
            self.sparse.insert(index, self.dense.len());
            self.dense.push(index);
            storage.push(data);
        }
    }

    pub fn remove(&mut self, index: Index) {
        let mut storage = self.data.borrow_mut();

        if let Some(&dense_idx) = self.sparse.get(&index) {
            let last_idx = self.dense.len() - 1;

            if dense_idx != last_idx {
                let temp_last_idx = self.dense[last_idx];
                self.dense[dense_idx] = temp_last_idx;
                self.sparse.insert(temp_last_idx, dense_idx);
                storage.swap(dense_idx, last_idx);
            }

            self.dense.pop();
            storage.pop();
            self.sparse.remove(&index);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Index, Ref<T>)> {
        self.dense.iter().enumerate().map(|(dense_index, &index)| {
            (
                index,
                Ref::map(self.data.borrow(), |data| &data[dense_index]),
            )
        })
    }

    pub fn iter_mut(&self) -> impl Iterator<Item = (Index, RefMut<T>)> {
        self.dense.iter().enumerate().map(|(dense_index, &index)| {
            (
                index,
                RefMut::map(self.data.borrow_mut(), |data| &mut data[dense_index]),
            )
        })
    }
}

pub trait AnySparseSet: Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn remove(&mut self, index: Index);
}

impl<T: Component> AnySparseSet for SparseSet<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn remove(&mut self, index: Index) {
        self.remove(index);
    }
}
