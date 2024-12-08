use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetHandle(pub u64);

pub struct Assets<T> {
    items: HashMap<AssetHandle, T>,
    next_id: u64,
}

#[allow(dead_code)]
impl<T> Assets<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, asset: T) -> AssetHandle {
        let handle = AssetHandle(self.next_id);
        self.items.insert(handle, asset);
        self.next_id += 1;

        handle
    }

    pub fn remove(&mut self, handle: AssetHandle) -> Option<T> {
        self.items.remove(&handle)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn get(&self, handle: AssetHandle) -> Option<&T> {
        self.items.get(&handle)
    }

    pub fn get_mut(&mut self, handle: AssetHandle) -> Option<&mut T> {
        self.items.get_mut(&handle)
    }
}
