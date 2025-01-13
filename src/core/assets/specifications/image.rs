#[derive(Debug)]
pub struct Image {
    pub path: String,
    pub data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub dimensions: (u32, u32),
}

impl Image {
    pub fn new(path: &str) -> Self {
        let image = image::open(path).unwrap();
        let data = image.to_rgba8();
        let dimensions = data.dimensions();

        Self {
            path: path.to_string(),
            data,
            dimensions,
        }
    }
}
