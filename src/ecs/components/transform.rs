use nalgebra::Vector2;

#[derive(Debug)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub scale: f32,
    pub rotation: f32,
}

impl Transform {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            scale: 1.,
            rotation: 0.,
        }
    }
}
