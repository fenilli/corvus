use std::{collections::HashMap, sync::Arc};

use super::texture::Texture;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetHandle(usize);

pub trait Asset: Sized + 'static {
    fn load_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error + 'static>>;
}

pub struct AssetLoader {
    images: HashMap<AssetHandle, Arc<Texture>>,

    paths: HashMap<String, AssetHandle>,
    next_handle: usize,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),

            paths: HashMap::new(),
            next_handle: 0,
        }
    }

    pub fn load_texture(
        &mut self,
        path: &str,
    ) -> Result<AssetHandle, Box<dyn std::error::Error + 'static>> {
        if let Some(&handle) = self.paths.get(path) {
            return Ok(handle);
        };

        let asset = self.load_asset::<Texture>(path)?;

        let handle = self.new_handle();

        self.images.insert(handle, asset);
        self.paths.insert(path.to_string(), handle);

        return Ok(handle);
    }

    pub fn get_texture(&self, handle: AssetHandle) -> Option<Arc<Texture>> {
        self.images.get(&handle).cloned()
    }

    fn new_handle(&mut self) -> AssetHandle {
        let handle = AssetHandle(self.next_handle);
        self.next_handle += 1;

        handle
    }

    fn load_asset<T: Asset>(
        &mut self,
        path: &str,
    ) -> Result<(Arc<T>), Box<dyn std::error::Error + 'static>> {
        let bytes = std::fs::read(path)?;
        let asset = T::load_from_bytes(&bytes)?;
        let asset = Arc::new(asset);

        Ok(asset)
    }
}
