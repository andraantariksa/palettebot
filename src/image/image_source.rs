use async_trait::async_trait;
use crate::image::image::Image;
use super::super::config::*;
use rand::seq::SliceRandom as _;
use crate::error::{Result, Error};

#[async_trait]
pub trait ImageSource {
    async fn get_random_image(&self) -> Result<Image>;
}

pub struct ImageSources<'a> {
    sources: Vec<Box<dyn ImageSource + 'a>>
}

impl<'a> ImageSources<'a> {
    pub fn from_sources(sources: Vec<Box<dyn ImageSource + 'a>>) -> Self {
        Self {
            sources
        }
    }

    #[inline]
    pub fn get_random_source(&self) -> Option<&dyn ImageSource> {
        let source = self.sources.choose(&mut rand::thread_rng())?;
        Some(source.as_ref())
    }

    #[inline]
    pub async fn get_random_image(&self) -> Result<Image> {
        match self.get_random_source() {
            Some(x) => Ok(x.get_random_image().await?),
            None => Err(Error::OptionNoneError)
        }
    }
}