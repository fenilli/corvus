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

        if !self.components.contains_key(&type_id) {
            self.components
                .insert(type_id, RefCell::new(Box::new(SparseSet::<T>::new())));
        }
    }

    pub fn clear_components(&self, index: usize) {
        for (_, cell) in self.components.iter() {
            cell.borrow_mut().remove(index);
        }
    }

    pub fn insert<T: 'static>(&self, index: usize, component: T) {
        if let Some(cell) = self.components.get(&TypeId::of::<T>()) {
            let mut storage = cell.borrow_mut();
            let sparse_set = storage
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .expect("Type mismatch in the component storage");
            sparse_set.insert(index, component);
        } else {
            panic!("Component type is not registered!");
        }
    }

    pub fn remove_component<T: 'static>(&self, index: usize) {
        if let Some(cell) = self.components.get(&TypeId::of::<T>()) {
            let mut storage = cell.borrow_mut();
            let sparse_set = storage
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .expect("Type mismatch in the component storage");
            sparse_set.remove(index);
        } else {
            panic!("Component type is not registered!");
        }
    }

    // pub fn get_component<T: 'static>(&self, index: usize) -> Option<Ref<T>> {
    //     if let Some(cell) = self.components.get(&TypeId::of::<T>()) {
    //         return Some(Ref::map(cell.borrow(), |sparse_set| {
    //             sparse_set
    //                 .as_any()
    //                 .downcast_ref::<SparseSet<T>>()
    //                 .unwrap()
    //                 .get(index)
    //                 .unwrap()
    //         }));
    //     }

    //     None
    // }

    // pub fn get_component_mut<T: 'static>(&self, index: usize) -> Option<RefMut<T>> {
    //     if let Some(cell) = self.components.get(&TypeId::of::<T>()) {
    //         return Some(RefMut::map(cell.borrow_mut(), |sparse_set| {
    //             sparse_set
    //                 .as_any_mut()
    //                 .downcast_mut::<SparseSet<T>>()
    //                 .unwrap()
    //                 .get_mut(index)
    //                 .unwrap()
    //         }));
    //     }

    //     None
    // }

    pub fn get_components<T: 'static>(&self) -> Option<Ref<Vec<T>>> {
        if let Some(cell) = self.components.get(&TypeId::of::<T>()) {
            return Some(Ref::map(cell.borrow(), |sparse_set| {
                &sparse_set
                    .as_any()
                    .downcast_ref::<SparseSet<T>>()
                    .unwrap()
                    .data
            }));
        }

        None
    }

    pub fn get_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<T>>> {
        if let Some(dyn_component_vec) = self.components.get(&TypeId::of::<T>()) {
            return Some(RefMut::map(
                dyn_component_vec.borrow_mut(),
                |component_vec| {
                    &mut component_vec
                        .as_any_mut()
                        .downcast_mut::<SparseSet<T>>()
                        .unwrap()
                        .data
                },
            ));
        }

        None
    }
}
