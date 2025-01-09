use crate::{
    app::components::{Camera, Color, Flip, Sprite, Transform},
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
        for (entity, transform, sprite) in world.entities().filter_map(|entity| {
            match (
                world.get_component::<Transform>(entity),
                world.get_component::<Sprite>(entity),
            ) {
                (Some(transform), Some(sprite)) => Some((entity, transform, sprite)),
                _ => None,
            }
        }) {
            let atlas: &std::sync::Arc<Atlas> = asset_registry
                .get_atlas(&sprite.atlas_handle)
                .expect("sprite should contain loaded atlas");

            let atlas_region = atlas
                .get_region(&sprite.region_id)
                .expect("atlas should contain region from sprite");

            let (u_min, v_min, u_max, v_max) = {
                let mut uvs = atlas.calculate_uv(&sprite.region_id);

                if let Some(flip) = world.get_component::<Flip>(entity) {
                    if flip.horizontal {
                        uvs = (uvs.2, uvs.1, uvs.0, uvs.3);
                    }

                    if flip.vertical {
                        uvs = (uvs.0, uvs.3, uvs.2, uvs.1);
                    }
                }

                uvs
            };

            let vertex_data: Vec<Vertex> = [[-1.0, 1.0], [-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]]
                .iter()
                .zip([
                    [u_min, v_min],
                    [u_min, v_max],
                    [u_max, v_max],
                    [u_max, v_min],
                ])
                .map(|([x, y], uv)| {
                    let (width, height) = atlas_region.dimensions();

                    let sized = glam::vec2(x * width as f32, y * height as f32);
                    let scaled = sized * transform.scale;
                    let originated = scaled
                        + glam::vec2(
                            transform.origin.x * width as f32,
                            transform.origin.y * height as f32,
                        );
                    let rotated =
                        glam::Mat2::from_angle(transform.rotation.to_radians()) * originated;
                    let translated = rotated + transform.position.truncate();

                    Vertex::new([translated.x, translated.y], Color::WHITE.into(), uv)
                })
                .collect();

            renderer.draw(atlas, vertex_data);
        }
    }
}
