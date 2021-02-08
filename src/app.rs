use crate::config::Config;
use crate::image::image_source::ImageSource;
use crate::api;
use crate::image::image_source::ImageSources;
use std::time::Duration;

pub struct App<'a> {
    config: &'a Config,
    image_sources: ImageSources<'a>
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut sources: Vec<Box<dyn ImageSource>> = Vec::new();

        // TODO: Macro?
        if config.is_pexels_available() {
            sources.push(
                Box::new(
                    api::pexels::Pexels::new(config.get_pexels_key().unwrap())
                )
            );
        }

        if config.is_unsplash_available() {
            sources.push(
                Box::new(
                    api::unsplash::Unsplash::new(
                        config.get_unsplash_access_key().unwrap(),
                        config.get_unsplash_secret_key().unwrap())
                )
            );
        }

        Self {
            config,
            image_sources: ImageSources::from_sources(sources)
        }
    }

    pub async fn do_once(&self) {
        let image = self.image_sources.get_random_image();
        
    }

    pub async fn do_loop(&self, delay_sec: u64) {
        loop {
            self.do_once().await;
            std::thread::sleep(Duration::from_secs(delay_sec));
        }
    }
}
