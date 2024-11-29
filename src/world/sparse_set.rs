use std::collections::HashMap;

use super::entity_pool::Entity;

pub struct SparseSet<T> {
    data: Vec<T>,
    dense: Vec<Entity>,
    sparse: HashMap<Entity, usize>,
}

impl<T: 'static> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            dense: Vec::new(),
            sparse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, data: T) {
        if let Some(&dense_idx) = self.sparse.get(&entity) {
            self.data[dense_idx] = data;
        } else {
            self.sparse.insert(entity, self.dense.len());
            self.dense.push(entity);
            self.data.push(data);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(&dense_idx) = self.sparse.get(&entity) {
            let last_idx = self.dense.len() - 1;

            if dense_idx != last_idx {
                let temp_last_idx = self.dense[last_idx];
                self.dense[dense_idx] = temp_last_idx;
                self.sparse.insert(temp_last_idx, dense_idx);
                self.data.swap(dense_idx, last_idx);
            }

            self.dense.pop();
            self.data.pop();
            self.sparse.remove(&entity);
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.sparse
            .get(&entity)
            .and_then(|&dense_idx| self.data.get(dense_idx))
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.sparse
            .get(&entity)
            .and_then(|&dense_idx| self.data.get_mut(dense_idx))
    }

    pub fn iter(&self) -> &Vec<T> {
        &self.data
    }

    pub fn iter_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
}
