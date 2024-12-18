use crate::World;

pub struct RendererSystem;

impl RendererSystem {
    pub fn prepare(world: &World) {}

    pub fn render(world: &World, _render_pass: &mut wgpu::RenderPass) {}
}
