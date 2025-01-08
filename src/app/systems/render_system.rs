use crate::{
    app::components::{Camera, Color, Sprite, Transform},
    assets::{atlas::Atlas, AssetRegistry},
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

    pub fn prepare_sprites(world: &World, asset_registry: &AssetRegistry, renderer: &mut Renderer) {
        for (transform, sprite) in world.entities().filter_map(|entity| {
            match (
                world.get_component::<Transform>(entity),
                world.get_component::<Sprite>(entity),
            ) {
                (Some(transform), Some(sprite)) => Some((transform, sprite)),
                _ => None,
            }
        }) {
            let atlas: &std::sync::Arc<Atlas> =
                asset_registry.get_atlas(&sprite.atlas_handle).unwrap();

            let atlas_region = atlas.get_region(&sprite.region_id).unwrap();
            let (u_min, v_min, u_max, v_max) = atlas.calculate_uv(&sprite.region_id);

            let vertex_data: Vec<Vertex> = [[-1.0, 1.0], [-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]]
                .iter()
                .map(|[x, y]| {
                    let (width, height) = atlas_region.dimensions();

                    let point = glam::vec2(x * width as f32, y * height as f32) * transform.scale;
                    let rotated = glam::Mat2::from_angle(transform.rotation.to_radians()) * point;
                    let translated = rotated + transform.position;

                    let uv = match (x, y) {
                        (-1.0, 1.0) => [u_min, v_min],
                        (-1.0, -1.0) => [u_min, v_max],
                        (1.0, -1.0) => [u_max, v_max],
                        (1.0, 1.0) => [u_max, v_min],
                        _ => [0.0, 0.0],
                    };

                    Vertex::new([translated.x, translated.y], Color::WHITE.into(), uv)
                })
                .collect();

            renderer.draw(atlas, vertex_data);
        }
    }
}
