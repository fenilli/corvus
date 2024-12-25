use super::{texture::Texture, Asset};

pub struct AssetLoader {
    textures: std::collections::HashMap<Asset<Texture>, Texture>,
    next: u32,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            textures: std::collections::HashMap::new(),
            next: 0,
        }
    }

    pub fn load_texture(&mut self, path: &'static str) -> Asset<Texture> {
        let mut handle = Asset::<Texture>::new(path);

        if self.textures.contains_key(&handle) {
            return handle;
        }

        handle.index = self.next;
        self.textures.insert(handle, Texture::new(path).unwrap());
        self.next += 1;

        handle
    }

    pub fn get_all_textures(
        &self,
    ) -> std::collections::hash_map::Iter<'_, Asset<Texture>, Texture> {
        self.textures.iter()
    }
}
