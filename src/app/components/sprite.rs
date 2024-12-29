pub struct Sprite {
    pub texture_handle: &'static str,
}

impl Sprite {
    pub fn new(texture_handle: &'static str) -> Self {
        Self { texture_handle }
    }
}
