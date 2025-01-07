use super::{
    atlas::{Atlas, AtlasRegistry},
    handle::Handle,
};

pub struct AssetRegistry {
    atlas_registry: AtlasRegistry,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            atlas_registry: AtlasRegistry::new(),
        }
    }

    pub fn load_atlas(&mut self, atlas: Atlas) -> Handle<Atlas> {
        self.atlas_registry.insert(atlas)
    }

    pub fn get_atlas(&self, handle: Handle<Atlas>) -> Option<&std::sync::Arc<Atlas>> {
        self.atlas_registry.get(handle)
    }
}
