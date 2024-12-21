use glam::Mat4;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub zoom: f32,
}

impl Camera {
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            0.0,
            self.width as f32 / self.zoom,
            self.height as f32 / self.zoom,
            0.0,
            -1.0,
            1.0,
        )
    }
}
