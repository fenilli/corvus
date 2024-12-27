pub struct Transform {
    pub position: glam::Vec3,
    pub scale: glam::Vec2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: glam::Vec3, scale: glam::Vec2, rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }
}
