pub struct Camera {
    pub position: glam::Vec2,
    pub viewport: winit::dpi::PhysicalSize<u32>,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: glam::Vec2, viewport: winit::dpi::PhysicalSize<u32>, zoom: f32) -> Self {
        Self {
            position,
            viewport,
            zoom,
        }
    }

    pub fn get_view_projection(&self) -> glam::Mat4 {
        glam::Mat4::orthographic_rh(
            0.0,
            self.viewport.width as f32 / self.zoom,
            0.0,
            self.viewport.height as f32 / self.zoom,
            -10.0,
            10.0,
        ) * glam::Mat4::from_translation(self.position.extend(0.0))
    }
}
