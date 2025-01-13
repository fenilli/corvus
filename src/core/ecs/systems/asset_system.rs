use crate::core::{
    assets::{AssetServer, Assets, Image},
    render::graphics,
    resources::{specifications::GpuImage, Resources},
};

pub fn load_pending_assets(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    asset_server: &mut AssetServer,
    assets: &mut Assets,
    resources: &mut Resources,
) {
    for pending_path in asset_server.get_pending_to_load() {
        if let Some(handle_id) = asset_server.get_id_by_path(&pending_path) {
            let image = Image::new(&pending_path);

            let (texture, view) =
                graphics::create_texture(device, queue, &image.data, image.dimensions);
            let gpu_image = GpuImage::new(texture, view);
            resources.textures.insert(handle_id.clone(), gpu_image);

            assets.images.insert(handle_id.clone(), image);
        }
    }
}
