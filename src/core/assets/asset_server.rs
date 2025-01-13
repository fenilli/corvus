use std::collections::{HashMap, VecDeque};

use super::handle::{Handle, HandleId};

pub struct AssetServer {
    path_to_handle_id: HashMap<String, HandleId>,
    pending_to_load: VecDeque<String>,
}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            path_to_handle_id: HashMap::new(),
            pending_to_load: VecDeque::new(),
        }
    }

    pub fn load<T>(&mut self, path: &str) -> Handle<T> {
        if let Some(handle_id) = self.path_to_handle_id.get(path) {
            return Handle::new(handle_id.clone());
        }

        let handle_id = HandleId::new(path);

        self.path_to_handle_id
            .insert(path.to_string(), handle_id.clone());

        self.pending_to_load.push_back(path.to_string());

        Handle::new(handle_id)
    }

    pub fn get_id_by_path(&self, path: &str) -> Option<&HandleId> {
        self.path_to_handle_id.get(path)
    }

    pub fn get_pending_to_load(&mut self) -> Vec<String> {
        self.pending_to_load.drain(..).collect()
    }
}
