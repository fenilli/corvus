use std::{collections::HashMap, sync::Arc};

use image::DynamicImage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetHandle(&'static str);

pub struct AssetLoader {
    images: HashMap<AssetHandle, Arc<DynamicImage>>,
    next_handle: usize,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            next_handle: 0,
        }
    }

    pub fn load_image(
        &mut self,
        path: &'static str,
    ) -> Result<AssetHandle, Box<dyn std::error::Error + '_>> {
        let handle = AssetHandle(path);

        if self.images.contains_key(&handle) {
            return Ok(handle);
        };

        let asset = image::open(path)?;
        self.next_handle += 1;

        self.images.insert(handle, Arc::new(asset));

        return Ok(handle);
    }

    pub fn get_image(&self, handle: AssetHandle) -> Option<Arc<DynamicImage>> {
        self.images.get(&handle).cloned()
    }
}
