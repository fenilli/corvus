pub struct Camera {
    position: glam::Vec3,
    viewport: winit::dpi::PhysicalSize<u32>,
    zoom: f32,
}

impl Camera {
    pub fn new(position: glam::Vec3, viewport: winit::dpi::PhysicalSize<u32>, zoom: f32) -> Self {
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
            self.viewport.height as f32 / self.zoom,
            0.0,
            -100.0,
            100.0,
        )
    }

    fn world_to_camera(&self) -> glam::Mat4 {
        glam::Mat4::from_translation(self.position)
    }

    pub fn world_to_projection(&self) -> glam::Mat4 {
        self.world_to_camera() * self.camera_to_projection()
    }
}
