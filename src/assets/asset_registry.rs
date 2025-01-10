use super::handle::Handle;

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
    pub image: image::RgbaImage,
    regions: std::collections::HashMap<String, AtlasRegion>,
}

impl Atlas {
    pub fn get_region(&self, region_id: &'static str) -> Option<&AtlasRegion> {
        self.regions.get(region_id)
    }

    pub fn calculate_uv(&self, region_id: &'static str) -> (f32, f32, f32, f32) {
        let Some(atlas_region) = self.get_region(region_id) else {
            return (0.0, 0.0, 1.0, 1.0);
        };

        let (width, height) = self.image.dimensions();

        let u_min = atlas_region.x as f32 / width as f32;
        let v_min = atlas_region.y as f32 / height as f32;
        let u_max = (atlas_region.x + atlas_region.width) as f32 / width as f32;
        let v_max = (atlas_region.y + atlas_region.height) as f32 / height as f32;

        (u_min, v_min, u_max, v_max)
    }
}

pub struct AtlasDescriptor {
    pub region_width: u32,
    pub region_height: u32,
    pub padding_x: u32,
    pub padding_y: u32,
    pub rows: u32,
    pub cols: u32,
}

type Cache<K, V> = std::collections::HashMap<K, std::sync::Arc<V>>;

pub struct AssetRegistry {
    atlases: Cache<&'static str, Atlas>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            atlases: Cache::new(),
        }
    }

    fn load_image(path: &'static str) -> image::RgbaImage {
        let image = image::open(path).unwrap().to_rgba8();

        image
    }

    pub fn load_atlas(&mut self, path: &'static str, descriptor: AtlasDescriptor) -> Handle<Atlas> {
        let handle: Handle<Atlas> = Handle::new(path);

        if self.atlases.contains_key(&handle.id) {
            return handle;
        }

        println!("atlas {}", path);

        let AtlasDescriptor {
            region_width,
            region_height,
            padding_x,
            padding_y,
            rows,
            cols,
        } = descriptor;

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

        let mut regions = std::collections::HashMap::new();

        for row in 0..rows {
            for col in 0..cols {
                let x = padding_x + col * (region_width + 2 * padding_x);
                let y = padding_y + row * (region_height + 2 * padding_y);
                let name = format!("{}_{}", row, col);

                regions.insert(name, AtlasRegion::new(x, y, region_width, region_height));
            }
        }

        let atlas = Atlas { image, regions };

        self.atlases.insert(handle.id, std::sync::Arc::new(atlas));

        handle
    }

    pub fn get_atlas(&self, handle: &Handle<Atlas>) -> Option<&std::sync::Arc<Atlas>> {
        self.atlases.get(&handle.id)
    }
}
