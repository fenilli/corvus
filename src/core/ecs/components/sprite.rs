use crate::core::assets::{handle::Handle, Image};

pub struct Sprite {
    pub image_id: Handle<Image>,
    pub tint: [f32; 4],
    pub size: (u32, u32),
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl Sprite {
    pub fn new(
        image_id: Handle<Image>,
        tint: [f32; 4],
        size: (u32, u32),
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> Self {
        Self {
            image_id,
            tint,
            size,
            flip_horizontal,
            flip_vertical,
        }
    }
}
