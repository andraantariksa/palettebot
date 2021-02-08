use crate::image::image_source::ImageSource;
use serde::{Serialize, Deserialize};
use crate::image::image::Image;
use async_trait::async_trait;
use crate::error::Result;

#[derive(Serialize)]
pub struct Unsplash<'a> {
    access_key: &'a str,
    secret_key: &'a str
}

#[derive(Deserialize)]
struct UnsplashUrls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String
}

#[derive(Deserialize)]
struct UnsplashResp {
    description: String,
    urls: UnsplashUrls
}

impl<'a> Unsplash<'a> {
    pub fn new(access_key: &'a str, secret_key: &'a str) -> Self {
        Self {
            access_key,
            secret_key
        }
    }

    pub async fn get_random_image_data(&self) -> Result<UnsplashResp> {
        Ok(reqwest::Client::new()
            .get("https://api.unsplash.com/photos/random")
            .query(&self)
            .send()
            .await?
            .json()
            .await?)
    }
}

#[async_trait]
impl<'a> ImageSource for Unsplash<'a> {
    async fn get_random_image(&self) -> Result<Image> {
        let data = self.get_random_image_data().await?;
        Ok(Image::from_url(&data.urls.regular).await?)
    }
}
