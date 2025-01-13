use crate::core::{assets::handle::HandleId, utils::Cache};

pub struct Resources {
    pub texture_bind_groups: Cache<HandleId, wgpu::BindGroup>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            texture_bind_groups: Cache::new(),
        }
    }
}
