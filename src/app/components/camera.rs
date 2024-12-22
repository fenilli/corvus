use glam::{Mat4, Vec3};
use winit::dpi::PhysicalSize;

pub struct Camera {
    position: Vec3,
    viewport: PhysicalSize<u32>,
    zoom: f32,
}

impl Camera {
    pub fn new(position: Vec3, viewport: PhysicalSize<u32>, zoom: f32) -> Self {
        Self {
            position,
            viewport,
            zoom,
        }
    }

    fn camera_to_projection(&self) -> Mat4 {
        Mat4::orthographic_rh_gl(
            0.0,
            self.viewport.width as f32 / self.zoom,
            self.viewport.height as f32 / self.zoom,
            0.0,
            -1.0,
            1.0,
        )
    }

    fn world_to_camera(&self) -> Mat4 {
        Mat4::from_translation(-self.position)
    }

    pub fn world_to_projection(&self) -> Mat4 {
        self.world_to_camera() * self.camera_to_projection()
    }
}
