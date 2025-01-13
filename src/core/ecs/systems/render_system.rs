use crate::core::{
    assets::{Assets, Image},
    ecs::{
        components::{OrthoCamera, Rect, Sprite, Transform},
        World,
    },
    render::{SpriteInstance, SpriteRenderer},
    utils::Handle,
};

pub fn set_camera_projection(world: &World, sprite_renderer: &mut SpriteRenderer) {
    let ortho_camera = world
        .single::<OrthoCamera>()
        .expect("there needs to be a main camera");

    sprite_renderer.update_view_projection(ortho_camera.get_view_projection());
}

pub fn draw_sprites(world: &World, assets: &Assets, sprite_renderer: &mut SpriteRenderer) {
    let mut sprites = world
        .entities()
        .filter_map(|entity| {
            let transform = world.get_component::<Transform>(entity)?;
            let sprite = world.get_component::<Sprite>(entity)?;
            let image = world.get_component::<Handle<Image>>(entity)?;

            Some((entity, transform, sprite, image))
        })
        .collect::<Vec<_>>();

    sprites.sort_by(|(_, a_transform, _, _), (_, b_transform, _, _)| {
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

    for (entity, transform, sprite, image_handle) in sprites {
        let Some(image) = assets.images.get(&image_handle.id()) else {
            continue;
        };

        let rect = world.get_component::<Rect>(entity);

        let (width, height) = if let Some(rect) = &rect {
            (rect.w, rect.h)
        } else {
            image.dimensions
        };

        let position = [[-1.0, 1.0], [-1.0, -1.0], [1.0, -1.0], [1.0, 1.0]]
            .iter()
            .map(|&[x, y]| {
                let sized = glam::vec2(x * width as f32, y * height as f32);
                let scaled = sized * transform.scale;
                let originated = scaled
                    + glam::vec2(
                        transform.origin.x * width as f32,
                        transform.origin.y * height as f32,
                    );
                let rotated = glam::Mat2::from_angle(transform.rotation.to_radians()) * originated;
                let translated = rotated + transform.position.truncate();

                [translated.x, translated.y]
            })
            .collect::<Vec<_>>();

        let uv_coords = if let Some(rect) = &rect {
            let (width, height) = image.dimensions;

            let u_min = rect.x as f32 / width as f32;
            let v_min = rect.y as f32 / height as f32;
            let u_max = (rect.x + rect.w) as f32 / width as f32;
            let v_max = (rect.y + rect.h) as f32 / height as f32;

            let mut uv = (u_min, v_min, u_max, v_max);

            if sprite.flip_horizontal {
                uv = (uv.2, uv.1, uv.0, uv.3);
            }

            if sprite.flip_vertical {
                uv = (uv.0, uv.3, uv.2, uv.1);
            }

            let (u_min, v_min, u_max, v_max) = uv;

            vec![
                [u_min, v_min],
                [u_min, v_max],
                [u_max, v_max],
                [u_max, v_min],
            ]
        } else {
            vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]]
        };

        let sprite_instance = SpriteInstance {
            handle_image: image_handle.clone(),
            color: [1.0, 1.0, 1.0, 1.0],
            position,
            uv_coords,
        };

        sprite_renderer.draw(sprite_instance);
    }
}
