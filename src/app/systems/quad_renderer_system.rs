use glam::{Mat4, Vec2, Vec3};

use crate::{
    app::components::{Camera, Quad, Transform},
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
                transform.position,
                Vec2::new(
                    quad.width as f32 * transform.scale.x,
                    quad.height as f32 * transform.scale.y,
                ),
            ));
        }

        let view_projection_matrix = world
            .entities()
            .find_map(|entity| {
                if let Some(camera) = world.get_component::<Camera>(entity) {
                    Some(camera.view_projection_matrix())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                let window_size = graphics_device.window.inner_size();
                Mat4::orthographic_rh_gl(
                    0.0,
                    window_size.width as f32,
                    window_size.height as f32,
                    0.0,
                    -1.0,
                    1.0,
                ) * Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0))
            });

        self.quad_renderer.prepare(
            graphics_device,
            view_projection_matrix,
            instances.as_slice(),
        );
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        self.quad_renderer.render(render_pass);
    }
}
