use super::{Assets, Mesh};

pub struct ResourceManager {
    pub meshes: Assets<Mesh>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            meshes: Assets::new(),
        }
    }
}
