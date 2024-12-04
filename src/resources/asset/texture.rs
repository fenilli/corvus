use image::DynamicImage;

use super::asset_loader::Asset;

pub struct Texture {
    pub image: DynamicImage,
}

impl Asset for Texture {
    fn load_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self { image })
    }
}
