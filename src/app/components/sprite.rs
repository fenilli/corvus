use crate::assets::{
    atlas::{Atlas, AtlasRegionId},
    Handle,
};

pub struct Sprite {
    pub atlas_handle: Handle<Atlas>,
    pub region_name: AtlasRegionId,
}

impl Sprite {
    pub fn new(atlas_handle: Handle<Atlas>, region_name: AtlasRegionId) -> Self {
        Self {
            atlas_handle,
            region_name,
        }
    }
}
