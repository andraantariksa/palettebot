use crate::image::image_source::ImageSource;
use std::error::Error;
use async_trait::async_trait;
use crate::image::image::Image;
use crate::error::Result;

pub struct Pexels<'a> {
    api_key: &'a str
}

impl<'a> Pexels<'a> {
    pub fn new(api_key: &'a str) -> Self {
        Self {
            api_key
        }
    }

    pub fn get_random_image_uri(&self) -> &'a str {
        unimplemented!()
    }
}

#[async_trait]
impl<'a> ImageSource for Pexels<'a> {
    async fn get_random_image(&self) -> Result<Image> {
        unimplemented!()
    }
}
