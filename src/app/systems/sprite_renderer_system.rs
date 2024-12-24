use crate::{
    app::components::{Sprite, Transform},
    assets::{Asset, AssetLoader, Texture},
    ecs::World,
    render::{renderers::SpriteRenderer, GraphicsDevice, Instance},
};

pub struct SpriteRendererSystem {
    sprite_renderer: SpriteRenderer,
}

impl SpriteRendererSystem {
    pub fn new(
        graphics_device: &GraphicsDevice,
        world_to_projection_bind_group_layouts: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            sprite_renderer: SpriteRenderer::new(
                graphics_device,
                world_to_projection_bind_group_layouts,
            ),
        }
    }

    pub fn render(
        &mut self,
        world: &World,
        asset_loader: &AssetLoader,
        graphics_device: &GraphicsDevice,
        render_pass: &mut wgpu::RenderPass,
    ) {
        let mut batches: std::collections::HashMap<Asset<Texture>, Vec<Instance>> =
            std::collections::HashMap::new();

        for (_entity, transform, sprite) in world.entities().filter_map(|entity| {
            let (Some(transform), Some(sprite)) = (
                world.get_component::<Transform>(entity),
                world.get_component::<Sprite>(entity),
            ) else {
                return None;
            };

            Some((entity, transform, sprite))
        }) {
            let batch = batches.entry(sprite.texture_handle).or_insert(Vec::new());

            let dimensions = asset_loader
                .get_texture(sprite.texture_handle)
                .unwrap()
                .dimensions();
            let uv_coords: [f32; 4] = if let Some(source_rect) = &sprite.source_rect {
                let u = source_rect.u / dimensions.0 as f32;
                let v = source_rect.v / dimensions.1 as f32;
                let width = (source_rect.u + source_rect.width as f32) / dimensions.0 as f32;
                let height = (source_rect.v + source_rect.height as f32) / dimensions.1 as f32;

                [u, v, width, height]
            } else {
                [0.0, 0.0, 1.0, 1.0]
            };

            batch.push(Instance::new(
                transform.0
                    * glam::Mat4::from_scale(glam::Vec3::new(
                        dimensions.0 as f32,
                        dimensions.1 as f32,
                        1.0,
                    )),
                uv_coords,
                [1.0, 1.0, 1.0],
            ));
        }

        self.sprite_renderer
            .render(graphics_device, asset_loader, batches, render_pass);
    }
}
