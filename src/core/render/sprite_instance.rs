use crate::core::{assets::Image, utils::Handle};

pub struct SpriteInstance {
    pub handle_image: Handle<Image>,
    pub position: Vec<[f32; 2]>,
    pub uv_coords: Vec<[f32; 2]>,
    pub color: [f32; 4],
}
