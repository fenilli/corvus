use image::GenericImageView;

pub struct AssetLoader {
    textures: std::collections::HashMap<&'static str, image::DynamicImage>,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, path: &'static str) -> &'static str {
        if self.textures.contains_key(path) {
            return path;
        }

        let image = image::open(path).unwrap();

        self.textures.insert(path, image);

        path
    }

    pub fn dimensions(&self, path: &'static str) -> (u32, u32) {
        self.textures.get(path).unwrap().dimensions()
    }

    pub fn iter_texture(
        &self,
    ) -> std::collections::hash_map::Iter<'_, &'static str, image::DynamicImage> {
        self.textures.iter()
    }
}
