use crate::{
    app::{
        asset_loader::AssetLoader,
        components::{Camera, Sprite, Transform},
    },
    ecs::World,
    render::{GraphicsDevice, ResourceLoader, SpriteRenderer, Vertex},
};

pub struct SpriteRenderSystem;

impl SpriteRenderSystem {
    pub fn prepare(
        graphics_device: &GraphicsDevice,
        asset_loader: &AssetLoader,
        sprite_renderer: &mut SpriteRenderer,
        world: &mut World,
    ) {
        let camera_view_projection = world
            .entities()
            .find_map(|entity| match world.get_component::<Camera>(entity) {
                Some(camera) => Some(camera.world_to_projection()),
                None => None,
            })
            .expect("app can only run if there is an camera entity");

        sprite_renderer.prepare(graphics_device, camera_view_projection);

        for (transform, sprite) in world.entities().filter_map(|entity| {
            match (
                world.get_component::<Transform>(entity),
                world.get_component::<Sprite>(entity),
            ) {
                (Some(transform), Some(sprite)) => Some((transform, sprite)),
                _ => None,
            }
        }) {
            let dimensions = asset_loader.dimensions(sprite.texture_handle);

            let vertex_data = [
                (glam::Vec2::new(-1.0, 1.0), glam::Vec2::new(0.0, 0.0)),
                (glam::Vec2::new(-1.0, -1.0), glam::Vec2::new(0.0, 1.0)),
                (glam::Vec2::new(1.0, -1.0), glam::Vec2::new(1.0, 1.0)),
                (glam::Vec2::new(1.0, 1.0), glam::Vec2::new(1.0, 0.0)),
            ]
            .iter()
            .map(|&(vertex, uv)| {
                let scaled = vertex
                    * (glam::Vec2::new(dimensions.0 as f32, dimensions.1 as f32) * transform.scale);
                let rotated = glam::Mat2::from_angle(transform.rotation.to_radians()) * scaled;

                (rotated + transform.position, uv)
            })
            .map(|(vertex, uv)| Vertex::new(vertex.into(), [1.0, 1.0, 1.0, 1.0], uv.into()))
            .collect::<Vec<_>>();

            sprite_renderer.draw(sprite.texture_handle, vertex_data);
        }
    }

    pub fn render(
        graphics_device: &GraphicsDevice,
        resource_loader: &ResourceLoader,
        sprite_renderer: &mut SpriteRenderer,
        render_pass: &mut wgpu::RenderPass,
    ) {
        sprite_renderer.render(resource_loader, graphics_device, render_pass);
    }
}
