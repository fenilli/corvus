use crate::{
    app::components::{Quad, Transform},
    ecs::World,
    render::{GraphicsDevice, Instance, QuadRenderer},
};

pub struct QuadRendererSystem {
    quad_renderer: QuadRenderer,
}

impl QuadRendererSystem {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        Self {
            quad_renderer: QuadRenderer::new(graphics_device),
        }
    }

    pub fn prepare(&mut self, world: &World, graphics_device: &GraphicsDevice) {
        let mut instances: Vec<Instance> = Vec::new();

        for (_entity, transform, _quad) in world.entities().filter_map(|entity| {
            let (Some(transform), Some(quad)) = (
                world.get_component::<Transform>(entity),
                world.get_component::<Quad>(entity),
            ) else {
                return None;
            };

            Some((entity, transform, quad))
        }) {
            instances.push(Instance {
                position: transform.position,
            });
        }

        self.quad_renderer
            .prepare(graphics_device, instances.as_slice());
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        self.quad_renderer.render(render_pass);
    }
}
