use crate::core::{
    ecs::{
        components::{OrthoCamera, Sprite, Transform},
        World,
    },
    render::{SpriteInstance, SpriteRenderer},
};

pub fn set_camera_projection(world: &World, sprite_renderer: &mut SpriteRenderer) {
    let ortho_camera = world
        .single::<OrthoCamera>()
        .expect("there needs to be a main camera");

    sprite_renderer.update_view_projection(ortho_camera.get_view_projection());
}

pub fn draw_sprites(world: &World, sprite_renderer: &mut SpriteRenderer) {
    let mut sprites = world
        .entities()
        .filter_map(|entity| {
            let transform = world.get_component::<Transform>(entity)?;
            let sprite = world.get_component::<Sprite>(entity)?;

            Some((transform, sprite))
        })
        .collect::<Vec<_>>();

    sprites.sort_by(|(a_transform, _), (b_transform, _)| {
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

    for (transform, sprite) in sprites {
        // let (u_min, v_min, u_max, v_max) = {
        //     let mut uvs = atlas.calculate_uv(&sprite.region_id);

        //     if let Some(flip) = flip {
        //         if flip.horizontal {
        //             uvs = (uvs.2, uvs.1, uvs.0, uvs.3);
        //         }

        //         if flip.vertical {
        //             uvs = (uvs.0, uvs.3, uvs.2, uvs.1);
        //         }
        //     }

        //     uvs
        // };

        let position = [[-1.0, 1.0], [-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]]
            .iter()
            .map(|&[x, y]| {
                let sized = glam::vec2(x * sprite.size.0 as f32, y * sprite.size.1 as f32);
                let scaled = sized * transform.scale;
                let originated = scaled
                    + glam::vec2(
                        transform.origin.x * sprite.size.0 as f32,
                        transform.origin.y * sprite.size.1 as f32,
                    );
                let rotated = glam::Mat2::from_angle(transform.rotation.to_radians()) * originated;
                let translated = rotated + transform.position.truncate();

                [translated.x, translated.y]
            })
            .collect::<Vec<_>>();

        let sprite_instance = SpriteInstance {
            image_id: sprite.image_id.clone(),
            color: [1.0, 1.0, 1.0, 1.0],
            position,
            uv_coords: vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
        };

        sprite_renderer.draw(sprite_instance);
    }
}
