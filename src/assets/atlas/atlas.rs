use crate::assets::Handle;

#[derive(Debug)]
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
pub enum AtlasRegionId {
    Named(&'static str),
    Grid((u32, u32)),
}

type AtlasCache = std::collections::HashMap<String, AtlasRegion>;

pub struct Atlas {
    pub path: &'static str,
    pub image: image::RgbaImage,
    pub regions: AtlasCache,
}

impl Atlas {
    fn new(path: &'static str, image: image::RgbaImage, regions: AtlasCache) -> Self {
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

    pub fn from_regions(path: &'static str, regions: AtlasCache) -> Self {
        let image = Self::load_image(path);

        Self::new(path, image, regions)
    }

    pub fn from_grid(
        path: &'static str,
        cell_width: u32,
        cell_height: u32,
        spacing_x: u32,
        spacing_y: u32,
    ) -> Self {
        let image = Self::load_image(path);

        assert!(
            cell_width > 0 && cell_height > 0,
            "Cell dimensions must be positive."
        );
        assert!(
            cell_width + spacing_x <= image.width() && cell_height + spacing_y <= image.height(),
            "Grid cells with spacing must fit within the image dimensions."
        );

        let cols = (image.width() + spacing_x) / (cell_width + spacing_x);
        let rows = (image.height() + spacing_y) / (cell_height + spacing_y);

        let mut regions = AtlasCache::new();

        for row in 0..rows {
            for col in 0..cols {
                let x = col * (cell_width + spacing_x);
                let y = row * (cell_height + spacing_y);

                regions.insert(
                    format!("{}_{}", col, row),
                    AtlasRegion::new(x, y, cell_width, cell_height),
                );
            }
        }

        Self::new(path, image, regions)
    }

    pub fn get_region(&self, atlas_region_id: &AtlasRegionId) -> Option<&AtlasRegion> {
        let handle = match atlas_region_id {
            AtlasRegionId::Grid((col, row)) => format!("{}_{}", col, row),
            AtlasRegionId::Named(name) => name.to_string(),
        };

        self.regions.get(&handle)
    }

    pub fn calculate_uv(&self, atlas_region_id: &AtlasRegionId) -> (f32, f32, f32, f32) {
        let handle = match atlas_region_id {
            AtlasRegionId::Grid((col, row)) => format!("{}_{}", col, row),
            AtlasRegionId::Named(name) => name.to_string(),
        };

        let Some(atlas_region) = self.regions.get(&handle) else {
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
