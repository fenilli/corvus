pub struct Sprite {
    pub texture_id: &'static str,
    pub source_rect: winit::dpi::PhysicalSize<u32>,
    pub tint: [f32; 4],
}

impl Sprite {
    pub fn new(
        texture_id: &'static str,
        source_rect: winit::dpi::PhysicalSize<u32>,
        tint: [f32; 4],
    ) -> Self {
        Self {
            texture_id,
            source_rect,
            tint,
        }
    }
}
