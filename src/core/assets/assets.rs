use crate::core::utils::{Cache, HandleId};

use super::Image;

pub struct Assets {
    pub images: Cache<HandleId, Image>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            images: Cache::new(),
        }
    }
}
