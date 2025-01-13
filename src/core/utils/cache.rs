use std::collections::HashMap;

pub struct Cache<K: PartialEq + Eq + std::hash::Hash, V> {
    items: HashMap<K, V>,
}

impl<K: PartialEq + Eq + std::hash::Hash, V> Cache<K, V> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, item: V) {
        self.items.entry(key).or_insert(item);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.items.remove(key)
    }
}
