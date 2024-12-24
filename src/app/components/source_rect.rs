pub struct SourceRect {
    pub u: f32,
    pub v: f32,
    pub width: u32,
    pub height: u32,
}

impl SourceRect {
    pub fn new(u: f32, v: f32, width: u32, height: u32) -> Self {
        Self {
            u,
            v,
            width,
            height,
        }
    }
}
