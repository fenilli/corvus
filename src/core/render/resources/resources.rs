use crate::core::{assets::handle::HandleId, utils::Cache};

use super::specifications::GpuImage;

pub struct Resources {
    pub textures: Cache<HandleId, GpuImage>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            textures: Cache::new(),
        }
    }
}
