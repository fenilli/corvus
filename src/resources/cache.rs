use std::{collections::HashMap, sync::Arc};

pub struct Cache<T> {
    items: HashMap<&'static str, Arc<T>>,
}

impl<T: 'static> Cache<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn get_or_insert_key(&mut self, key: &'static str, item: T) -> &'static str {
        if self.items.contains_key(key) {
            key
        } else {
            self.items.insert(key, Arc::new(item));

            key
        }
    }

    fn get_item(&mut self, key: &'static str) -> Option<&Arc<T>> {
        self.items.get(&key)
    }
}
