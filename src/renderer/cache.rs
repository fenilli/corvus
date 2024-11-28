use std::collections::HashMap;

pub struct Cache<T> {
    items: HashMap<&'static str, T>,
}

impl<T: 'static> Cache<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &'static str, item: T) -> &'static str {
        self.items.entry(key).or_insert(item);

        key
    }

    pub fn get(&self, key: &'static str) -> Option<&T> {
        self.items.get(key)
    }
}
