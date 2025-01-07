use super::region::AtlasRegion;

pub struct Atlas {
    pub image: image::RgbaImage,
    pub path: &'static str,
    pub regions: Vec<AtlasRegion>,
}

impl Atlas {
    fn new(image: image::RgbaImage, path: &'static str, regions: Vec<AtlasRegion>) -> Self {
        Self {
            image,
            path,
            regions,
        }
    }

    fn load_image(path: &'static str) -> image::RgbaImage {
        let image = image::open(path).unwrap().to_rgba8();

        image
    }

    pub fn from_regions(path: &'static str, regions: Vec<AtlasRegion>) -> Self {
        let image = Self::load_image(path);

        Self::new(image, path, regions)
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

        let mut regions = Vec::new();

        let mut y = 0;
        while y + cell_height <= image.height() {
            let mut x = 0;
            while x + cell_width <= image.width() {
                regions.push(AtlasRegion::new(
                    x as f32,
                    y as f32,
                    cell_width,
                    cell_height,
                ));
                x += cell_width + spacing_x;
            }
            y += cell_height + spacing_y;
        }

        Self::new(image, path, regions)
    }
}
