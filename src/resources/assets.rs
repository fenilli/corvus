use std::{collections::HashMap, sync::Arc};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AssetHandle<T> {
    id: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> AssetHandle<T> {
    fn new(id: usize) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct Assets<T> {
    assets: HashMap<usize, Arc<T>>,
    next_id: usize,
}

#[allow(dead_code)]
impl<T> Assets<T> {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, asset: T) -> AssetHandle<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.assets.insert(id, Arc::new(asset));

        AssetHandle::new(id)
    }

    pub fn remove(&mut self, handle: AssetHandle<T>) {
        self.assets.remove(&handle.id);
    }

    pub fn clear(&mut self) {
        self.assets.clear();
    }

    pub fn get(&self, handle: AssetHandle<T>) -> Option<Arc<T>> {
        self.assets.get(&handle.id).cloned()
    }
}
