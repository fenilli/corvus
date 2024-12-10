use nalgebra::Matrix4;

pub struct Camera {
    pub projection: Matrix4<f32>,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            projection: Matrix4::new_orthographic(0.0, width as f32, height as f32, 0.0, 1.0, -1.0),
        }
    }
}
