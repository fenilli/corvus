use glam::{Mat4, Vec2, Vec3};

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub zoom: f32,
}

impl Camera {
    fn projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            0.0,
            self.width as f32 / self.zoom,
            self.height as f32 / self.zoom,
            0.0,
            -1.0,
            1.0,
        )
    }

    pub fn view_projection_matrix(&self, position: Vec2) -> Mat4 {
        self.projection_matrix() * Mat4::from_translation(Vec3::new(-position.x, -position.y, 0.0))
    }
}
