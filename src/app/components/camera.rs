pub struct Camera {
    pub viewport: winit::dpi::PhysicalSize<u32>,
    pub zoom: f32,
}

impl Camera {
    pub fn new(viewport: winit::dpi::PhysicalSize<u32>, zoom: f32) -> Self {
        Self { viewport, zoom }
    }

    pub fn get_view_projection(&self, position: glam::Vec2) -> glam::Mat4 {
        glam::Mat4::orthographic_rh(
            0.0,
            self.viewport.width as f32 / self.zoom,
            0.0,
            self.viewport.height as f32 / self.zoom,
            -10.0,
            10.0,
        ) * glam::Mat4::from_translation(position.extend(0.0))
    }
}
