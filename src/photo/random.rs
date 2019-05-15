use super::super::constant::*;
use rand::Rng as _;

pub struct RandomPhoto {
    pub post_source: String,
    pub file_url: String,
}

impl RandomPhoto {
    pub fn new() -> Self {
        match rand::thread_rng().gen::<u32>() % 2 {
            0 => {
                let data = pexels::Pexels::new(PEXELS_API_KEY.to_string())
                    .curated_photo(1, rand::thread_rng().gen::<u32>() % 1000 + 1);
                Self {
                    post_source: data["photos"][0]["url"].as_str().unwrap().to_owned(),
                    file_url: data["photos"][0]["src"]["original"]
                        .as_str()
                        .unwrap()
                        .to_owned(),
                }
            }
            1 => {
                let data = super::super::api::unsplash::Unsplash::new(
                    UNSPLASH_PUBLIC_KEY,
                    UNSPLASH_SECRET_KEY,
                )
                .random_photo();
                Self {
                    post_source: data["links"]["html"].as_str().unwrap().to_owned(),
                    file_url: data["urls"]["full"].as_str().unwrap().to_owned(),
                }
            }
            _ => panic!("Error when choosing random image"),
        }
    }
}
