use image::GenericImageView;

pub struct Texture {
    image: image::DynamicImage,
}

impl Texture {
    pub(super) fn new(path: &'static str) -> Result<Self, image::ImageError> {
        let image = image::open(path)?;

        Ok(Self { image })
    }

    pub fn data(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        self.image.to_rgb8()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }
}
