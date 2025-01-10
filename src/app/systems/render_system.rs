use crate::{
    app::components::{Camera, Color, Flip, Sprite, Transform},
    assets::AssetRegistry,
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
        let mut sprites = world
            .entities()
            .filter_map(|entity| {
                let transform = world.get_component::<Transform>(entity)?;
                let sprite = world.get_component::<Sprite>(entity)?;
                let flip = world.get_component::<Flip>(entity);

                Some((transform, sprite, flip))
            })
            .collect::<Vec<_>>();

        sprites.sort_by(|(a_transform, _, _), (b_transform, _, _)| {
            a_transform
                .position
                .z
                .partial_cmp(&b_transform.position.z)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    let a_y = a_transform.position.y - (a_transform.origin.y * a_transform.scale.y);
                    let b_y = b_transform.position.y - (b_transform.origin.y * b_transform.scale.y);

                    b_y.partial_cmp(&a_y).unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        for (transform, sprite, flip) in sprites {
            let atlas = asset_registry
                .get_atlas(&sprite.atlas_handle)
                .expect("sprite should contain loaded atlas");

            let atlas_region = atlas
                .get_region(&sprite.region_id)
                .expect("atlas should contain region from sprite");

            let (u_min, v_min, u_max, v_max) = {
                let mut uvs = atlas.calculate_uv(&sprite.region_id);

                if let Some(flip) = flip {
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

            renderer.draw(sprite.atlas_handle.id, vertex_data);
        }
    }
}
