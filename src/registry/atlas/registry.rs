use crate::registry::handle::Handle;

use super::atlas::Atlas;

type Cache<K, V> = std::collections::HashMap<K, std::sync::Arc<V>>;

pub struct AtlasRegistry {
    atlases: Cache<u64, Atlas>,
    next_id: u64,
}

impl AtlasRegistry {
    pub fn new() -> Self {
        Self {
            atlases: Cache::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, atlas: Atlas) -> Handle<Atlas> {
        let handle: Handle<Atlas> = Handle::new(self.next_id);

        if self.atlases.contains_key(&handle.id) {
            panic!("An atlas with the name '{}' already exists.", atlas.path);
        }

        self.atlases.insert(handle.id, std::sync::Arc::new(atlas));
        self.next_id += 1;

        handle
    }

    pub fn get(&self, handle: &Handle<Atlas>) -> Option<&std::sync::Arc<Atlas>> {
        self.atlases.get(&handle.id)
    }
}
