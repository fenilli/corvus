use crate::{app::components::Sprite, assets::AssetRegistry, ecs::World, render::Renderer};

pub struct AssetSystem;

impl AssetSystem {
    pub fn load_textures_from_assets(
        world: &World,
        asset_registry: &AssetRegistry,
        renderer: &mut Renderer,
    ) {
        for sprite in world
            .entities()
            .flat_map(|entity| world.get_component::<Sprite>(entity))
        {
            let Some(atlas) = asset_registry.get_atlas(&sprite.atlas_handle) else {
                continue;
            };

            renderer.load_texture(&atlas.path, &atlas.image);
        }
    }
}
