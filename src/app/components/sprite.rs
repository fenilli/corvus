use crate::{
    app::color::Color,
    assets::{Asset, Texture},
};

pub struct Sprite {
    pub texture_handle: Asset<Texture>,
    pub color: Color,
}

impl Sprite {
    pub fn new(texture_handle: Asset<Texture>, color: Color) -> Self {
        Self {
            texture_handle,
            color,
        }
    }
}
