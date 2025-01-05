pub struct Transform {
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: glam::Vec2, scale: glam::Vec2, rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn apply_transform(&self, vertices: Vec<[f32; 2]>) -> Vec<[f32; 2]> {
        vertices
            .iter()
            .map(|&[x, y]| {
                let point = glam::vec2(x, y) * self.scale;
                let rotated = glam::Mat2::from_angle(self.rotation.to_radians()) * point;
                let translated = rotated + self.position;

                [translated.x, translated.y]
            })
            .collect()
    }
}
