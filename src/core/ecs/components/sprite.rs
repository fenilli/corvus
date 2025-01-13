use crate::core::{assets::Image, utils::Handle};

pub struct Sprite {
    pub handle_image: Handle<Image>,
    pub tint: [f32; 4],
    pub size: (u32, u32),
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl Sprite {
    pub fn new(
        handle_image: Handle<Image>,
        tint: [f32; 4],
        size: (u32, u32),
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> Self {
        Self {
            handle_image,
            tint,
            size,
            flip_horizontal,
            flip_vertical,
        }
    }
}
