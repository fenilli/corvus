use super::source_rect::SourceRect;

pub struct Sprite {
    pub texture_id: &'static str,
    pub source_rect: SourceRect,
}

impl Sprite {
    pub fn new(texture_id: &'static str, source_rect: SourceRect) -> Self {
        Self {
            texture_id,
            source_rect,
        }
    }
}
