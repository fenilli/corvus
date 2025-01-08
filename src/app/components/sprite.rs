use crate::assets::{
    atlas::{Atlas, AtlasRegionId},
    Handle,
};

pub struct Sprite {
    pub atlas_handle: Handle<Atlas>,
    pub region_id: AtlasRegionId,
}

impl Sprite {
    pub fn new(atlas_handle: Handle<Atlas>, region_id: AtlasRegionId) -> Self {
        Self {
            atlas_handle,
            region_id,
        }
    }
}
