use crate::{
    app::components::{Camera, Color, Sprite, Transform},
    ecs::World,
    render::{Renderer, Vertex},
};

pub struct RenderSystem;

impl RenderSystem {
    pub fn prepare_projection(world: &World, renderer: &mut Renderer) {
        let Some(camera) = world.single::<Camera>() else {
            panic!("we need a camera with projection to show to the screen.");
        };

        let view_projection = camera.get_view_projection();
        renderer.set_view_projection(view_projection);
    }

    pub fn prepare_sprites(world: &World, renderer: &mut Renderer) {
        for (transform, sprite) in world.entities().filter_map(|entity| {
            match (
                world.get_component::<Transform>(entity),
                world.get_component::<Sprite>(entity),
            ) {
                (Some(transform), Some(sprite)) => Some((transform, sprite)),
                _ => None,
            }
        }) {
            let vertex_data: Vec<Vertex> = [[-1.0, 1.0], [-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]]
                .iter()
                .zip([[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]])
                .map(|([x, y], uv)| {
                    let width = sprite.source_rect.width as f32;
                    let height = sprite.source_rect.height as f32;

                    let point = glam::vec2(x * width, y * height) * transform.scale;
                    let rotated = glam::Mat2::from_angle(transform.rotation.to_radians()) * point;
                    let translated = rotated + transform.position;

                    Vertex::new([translated.x, translated.y], Color::WHITE.into(), uv)
                })
                .collect();

            renderer.draw(sprite.texture_id, vertex_data);
        }
    }
}
