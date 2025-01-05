use crate::{
    app::components::{Sprite, Transform},
    ecs::World,
    render::{Renderer, Vertex},
};

pub struct RenderSystem;

impl RenderSystem {
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
            let vertex_pos = vec![[-1.0, 1.0], [-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]];
            let vertex_color = [1.0, 1.0, 1.0, 1.0];
            let vertex_uv = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

            renderer.draw(
                sprite.texture_id,
                transform
                    .apply_transform(sprite.apply_size(vertex_pos))
                    .iter()
                    .zip(&vertex_uv)
                    .map(|(&pos, &uv)| Vertex::new(pos, vertex_color, uv))
                    .collect(),
            );
        }
    }
}
