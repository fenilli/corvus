use crate::core::assets::{AssetServer, Assets, Image};

pub fn load_pending_assets(asset_server: &mut AssetServer, assets: &mut Assets) {
    for pending_path in asset_server.get_pending_to_load() {
        if let Some(handle_id) = asset_server.get_id_by_path(&pending_path) {
            let image = Image::new(&pending_path);
            assets.images.insert(handle_id.clone(), image);
        }
    }
}
