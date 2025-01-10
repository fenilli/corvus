use crate::assets::{Atlas, Handle};

pub struct Sprite {
    pub atlas_handle: Handle<Atlas>,
    pub region_id: &'static str,
}

impl Sprite {
    pub fn new(atlas_handle: Handle<Atlas>, region_id: &'static str) -> Self {
        Self {
            atlas_handle,
            region_id,
        }
    }
}
