use std::collections::HashMap;

pub struct Cache<T> {
    items: HashMap<String, T>,
}

impl<T: 'static> Cache<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, item: T) -> String {
        self.items.entry(key.clone()).or_insert(item);

        key
    }

    pub fn exists(&self, key: &String) -> bool {
        self.items.contains_key(key)
    }

    pub fn get(&self, key: String) -> Option<&T> {
        self.items.get(&key)
    }
}
