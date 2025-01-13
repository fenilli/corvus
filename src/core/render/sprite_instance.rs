use crate::core::assets::{handle::Handle, Image};

pub struct SpriteInstance {
    pub image_id: Handle<Image>,
    pub position: Vec<[f32; 2]>,
    pub uv_coords: Vec<[f32; 2]>,
    pub color: [f32; 4],
}
