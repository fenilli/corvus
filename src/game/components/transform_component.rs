#[derive(Debug)]
pub struct TransformComponent {
    pub position: (i32, i32),
    pub rotation: f32,
    pub scale: u32,
}

impl TransformComponent {
    pub fn new(position: (i32, i32), rotation: f32, scale: u32) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}
