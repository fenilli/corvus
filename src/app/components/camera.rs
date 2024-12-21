use glam::{Mat4, Vec2, Vec3};

pub struct Camera {
    pub position: Vec2,
    pub width: u32,
    pub height: u32,
    pub zoom: f32,
}

impl Camera {
    pub fn view_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh_gl(
            0.0,
            self.width as f32 / self.zoom,
            self.height as f32 / self.zoom,
            0.0,
            -1.0,
            1.0,
        ) * Mat4::from_translation(-Vec3::new(self.position.x, self.position.y, 0.0))
    }
}
