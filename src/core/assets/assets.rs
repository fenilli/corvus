use crate::core::utils::Cache;

use super::{handle::HandleId, Image};

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
