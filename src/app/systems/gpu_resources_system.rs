use crate::{
    app::asset_loader::AssetLoader,
    render::{GraphicsDevice, ResourceLoader},
};

pub struct GpuResourcesSystem;

impl GpuResourcesSystem {
    pub fn load_resources(
        graphics_device: &GraphicsDevice,
        asset_loader: &AssetLoader,
        resource_loader: &mut ResourceLoader,
    ) {
        for (&handle, texture) in asset_loader.iter_texture() {
            resource_loader.load_texture(graphics_device, handle, texture);
        }
    }
}
