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
    fn new(
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

    pub fn from_grid(
        path: &'static str,
        region_width: u32,
        region_height: u32,
        padding_x: u32,
        padding_y: u32,
        rows: u32,
        cols: u32,
        name_generator: Option<fn(u32, u32) -> String>,
    ) -> Self {
        assert!(cols > 0, "The number of columns must be greater than 0.");
        assert!(rows > 0, "The number of rows must be greater than 0.");

        let image = Self::load_image(path);
        let image_width = image.width();
        let image_height = image.height();

        assert!(
            region_width > 0
                && region_width <= image_width
                && region_height > 0
                && region_height <= image_height,
            "Region dimensions must be greater than 0 and fit within the image dimensions."
        );

        let grid_width = region_width * cols;
        let grid_height = region_height * rows;

        assert!(
            grid_width <= image_width && grid_height <= image_height,
            "Grid dimensions exceed the image dimensions."
        );

        let name_generator = name_generator.unwrap_or(|row, col| format!("{}_{}", row, col));

        let mut regions = std::collections::HashMap::new();

        for row in 0..rows {
            for col in 0..cols {
                let x = padding_x + col * (region_width + 2 * padding_x);
                let y = padding_y + row * (region_height + 2 * padding_y);
                let name = name_generator(row, col);

                println!("Region: {}, Coordinates: ({}, {})", name, x, y);

                regions.insert(name, AtlasRegion::new(x, y, region_width, region_height));
            }
        }

        Self::new(path, image, regions)
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
