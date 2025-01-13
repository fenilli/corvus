pub struct Sprite {
    pub tint: [f32; 4],
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl Sprite {
    pub fn new(tint: [f32; 4], flip_horizontal: bool, flip_vertical: bool) -> Self {
        Self {
            tint,
            flip_horizontal,
            flip_vertical,
        }
    }
}
