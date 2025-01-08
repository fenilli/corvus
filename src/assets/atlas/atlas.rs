use crate::assets::Handle;

#[derive(Debug, Clone, Copy)]
pub struct AtlasRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl AtlasRegion {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub struct Atlas {
    pub path: &'static str,
    pub image: image::RgbaImage,
    pub regions: std::collections::HashMap<String, AtlasRegion>,
}

impl Atlas {
    pub fn new(
        path: &'static str,
        image: image::RgbaImage,
        regions: std::collections::HashMap<String, AtlasRegion>,
    ) -> Self {
        Self {
            path,
            image,
            regions,
        }
    }

    fn load_image(path: &'static str) -> image::RgbaImage {
        let image = image::open(path).unwrap().to_rgba8();

        image
    }

    pub fn get_region(&self, region_id: &'static str) -> Option<&AtlasRegion> {
        self.regions.get(region_id)
    }

    pub fn calculate_uv(&self, region_id: &'static str) -> (f32, f32, f32, f32) {
        let Some(atlas_region) = self.get_region(region_id) else {
            return (0.0, 0.0, 1.0, 1.0);
        };

        let (width, height) = self.image.dimensions();

        // Calculate UV coordinates for the top-left corner (u_min, v_min)
        let u_min = atlas_region.x as f32 / width as f32;
        let v_min = atlas_region.y as f32 / height as f32;

        // Calculate UV coordinates for the bottom-right corner (u_max, v_max)
        let u_max = (atlas_region.x + atlas_region.width) as f32 / width as f32;
        let v_max = (atlas_region.y + atlas_region.height) as f32 / height as f32;

        (u_min, v_min, u_max, v_max)
    }
}

type Cache<K, V> = std::collections::HashMap<K, std::sync::Arc<V>>;

pub struct AtlasRegistry {
    atlases: Cache<u64, Atlas>,
    next_id: u64,
}

impl AtlasRegistry {
    pub fn new() -> Self {
        Self {
            atlases: Cache::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, atlas: Atlas) -> Handle<Atlas> {
        let handle: Handle<Atlas> = Handle::new(self.next_id);

        if self.atlases.contains_key(&handle.id) {
            panic!("An atlas with the name '{}' already exists.", atlas.path);
        }

        self.atlases.insert(handle.id, std::sync::Arc::new(atlas));
        self.next_id += 1;

        handle
    }

    pub fn get(&self, handle: &Handle<Atlas>) -> Option<&std::sync::Arc<Atlas>> {
        self.atlases.get(&handle.id)
    }
}
