pub struct Sprite {
    pub texture_id: &'static str,
    pub source_rect: winit::dpi::PhysicalSize<u32>,
}

impl Sprite {
    pub fn new(texture_id: &'static str, source_rect: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            texture_id,
            source_rect,
        }
    }

    pub fn apply_size(&self, vertices: Vec<[f32; 2]>) -> Vec<[f32; 2]> {
        let width = self.source_rect.width as f32;
        let height = self.source_rect.height as f32;

        vertices
            .iter()
            .map(|&[x, y]| [x * width, y * height])
            .collect()
    }
}
