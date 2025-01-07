pub struct AtlasRegion {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
}

impl AtlasRegion {
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
