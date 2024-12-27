pub struct Camera {
    position: glam::Vec2,
    viewport: winit::dpi::PhysicalSize<u32>,
    zoom: f32,
}

impl Camera {
    pub fn new(position: glam::Vec2, viewport: winit::dpi::PhysicalSize<u32>, zoom: f32) -> Self {
        Self {
            position,
            viewport,
            zoom,
        }
    }

    fn camera_to_projection(&self) -> glam::Mat4 {
        glam::Mat4::orthographic_rh(
            0.0,
            self.viewport.width as f32 / self.zoom,
            0.0,
            self.viewport.height as f32 / self.zoom,
            -10.0,
            10.0,
        )
    }

    fn world_to_camera(&self) -> glam::Mat4 {
        glam::Mat4::from_translation(self.position.extend(0.0))
    }

    pub fn world_to_projection(&self) -> glam::Mat4 {
        self.camera_to_projection() * self.world_to_camera()
    }
}
