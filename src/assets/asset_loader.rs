use super::{texture::Texture, Asset};

pub struct AssetLoader {
    textures: std::collections::HashMap<Asset<Texture>, Texture>,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, path: &'static str) -> Asset<Texture> {
        let handle = Asset::<Texture>::new(path);

        if self.textures.contains_key(&handle) {
            return handle;
        }

        self.textures.insert(handle, Texture::new(path).unwrap());

        handle
    }

    pub fn get_texture(&self, handle: Asset<Texture>) -> Option<&Texture> {
        self.textures.get(&handle)
    }
}
