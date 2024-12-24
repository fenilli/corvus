use crate::{
    app::color::Color,
    assets::{Asset, Texture},
};

use super::SourceRect;

pub struct Sprite {
    pub texture_handle: Asset<Texture>,
    pub color: Color,
    pub source_rect: Option<SourceRect>,
}

impl Sprite {
    pub fn new(
        texture_handle: Asset<Texture>,
        color: Color,
        source_rect: Option<SourceRect>,
    ) -> Self {
        Self {
            texture_handle,
            color,
            source_rect,
        }
    }
}
