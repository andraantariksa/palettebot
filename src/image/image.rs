use image::{GenericImage, ImageResult, EncodableLayout};
use image::GenericImageView as _;
use image::ImageFormat;
use std::error::Error;
use crate::error::Result;

pub struct Image {
    image: image_extern::DynamicImage,
}

impl Image {
    pub fn from_file(path: &str) -> Result<Self> {
        Ok(Self {
            image: image_extern::open(&std::path::Path::new(path))?,
        })
    }

    pub async fn from_url<T: AsRef<str>>(url: T) -> Result<Self> {
        // let bytes = surf::get(url).recv_bytes().await?;
        // Ok(Self {
        //     image: image_extern::load_from_memory(bytes.as_bytes())?
        // })
        unimplemented!()
    }

    pub fn resize_with_aspect_ratio_one_side(&mut self, side_size: u32) {
        if self.image.width() > self.image.height() {
            self.image = self
                .image
                .resize(self.image.height(), side_size, image::imageops::FilterType::Nearest);
        } else {
            self.image = self
                .image
                .resize(side_size, self.image.width(), image::imageops::FilterType::Nearest);
        }
    }

    pub fn get_png_bytes<'a>(&self) -> ImageResult<Vec<u8>> {
        let mut bytes = Vec::new();
        image_extern::codecs::png::PngEncoder::new(&mut bytes)
            .encode(
                self.image.as_bytes(),
                self.image.width(),
                self.image.height(),
                self.image.color())?;
        Ok(bytes)
    }
}
