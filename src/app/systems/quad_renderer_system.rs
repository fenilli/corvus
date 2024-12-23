use crate::{
    app::components::{Quad, Transform},
    ecs::World,
    render::{renderers::QuadRenderer, GraphicsDevice, Instance},
};

pub struct QuadRendererSystem {
    quad_renderer: QuadRenderer,
}

impl QuadRendererSystem {
    pub fn new(
        graphics_device: &GraphicsDevice,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Self {
        Self {
            quad_renderer: QuadRenderer::new(graphics_device, bind_group_layouts),
        }
    }

    pub fn prepare(&mut self, world: &World, graphics_device: &GraphicsDevice) {
        let mut instances: Vec<Instance> = Vec::new();

        for (_entity, transform, quad) in world.entities().filter_map(|entity| {
            let (Some(transform), Some(quad)) = (
                world.get_component::<Transform>(entity),
                world.get_component::<Quad>(entity),
            ) else {
                return None;
            };

            Some((entity, transform, quad))
        }) {
            instances.push(Instance::new(
                transform.0
                    * glam::Mat4::from_scale(glam::Vec3::new(
                        quad.width as f32,
                        quad.height as f32,
                        1.0,
                    )),
                [1.0, 0.0, 0.0],
            ));
        }

        self.quad_renderer
            .prepare(graphics_device, instances.as_slice());
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        self.quad_renderer.render(render_pass);
    }
}
