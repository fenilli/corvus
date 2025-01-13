use crate::core::{assets::Image, render::Rect, utils::Handle};

pub struct Sprite {
    pub texture_handle: Handle<Image>,
    pub source_rect: Rect,
    pub tint: [f32; 4],
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl Sprite {
    pub fn new(
        texture_handle: Handle<Image>,
        source_rect: Rect,
        tint: [f32; 4],
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> Self {
        Self {
            texture_handle,
            source_rect,
            tint,
            flip_horizontal,
            flip_vertical,
        }
    }
}
