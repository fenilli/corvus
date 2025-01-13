use std::{collections::HashMap, sync::Arc};

pub struct Cache<K: PartialEq + Eq + std::hash::Hash, V> {
    items: HashMap<K, Arc<V>>,
}

impl<K: PartialEq + Eq + std::hash::Hash, V> Cache<K, V> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, item: V) {
        self.items.entry(key).or_insert(Arc::new(item));
    }

    pub fn remove(&mut self, key: &K) -> Option<Arc<V>> {
        self.items.remove(key)
    }

    pub fn exists(&self, key: &K) -> bool {
        self.items.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&Arc<V>> {
        self.items.get(key)
    }
}
