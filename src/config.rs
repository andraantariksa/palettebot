use std::env;
use std::borrow::Borrow;

pub struct Config {
    pexels_key: Option<String>,
    unsplash_access_key: Option<String>,
    unsplash_secret_key: Option<String>,
    facebook_access_token: Option<String>
}

impl Config {
    pub fn new() -> Self {
        Config {
            pexels_key: None,
            unsplash_access_key: None,
            unsplash_secret_key: None,
            facebook_access_token: None
        }
    }

    pub fn read_env(&mut self) {
        for (key, val) in env::vars() {
            match key.as_ref() {
                "PEXELS_KEY" => {
                    self.pexels_key = Some(val)
                }
                "UNSPLASH_ACCESS_KEY" => {
                    self.unsplash_access_key = Some(val)
                }
                "UNSPLASH_SECRET_KEY" => {
                    self.unsplash_secret_key = Some(val)
                }
                "FACEBOOK_ACCESS_TOKEN" => {
                    self.facebook_access_token = Some(val)
                }
                _ => {}
            };
        }
    }

    #[inline]
    pub fn get_pexels_key(&self) -> Option<&str> {
        match self.pexels_key {
            Some(ref x) => Some(x),
            None => None
        }
    }

    #[inline]
    pub fn is_pexels_available(&self) -> bool {
        self.pexels_key.is_some()
    }

    #[inline]
    pub fn get_unsplash_access_key(&self) -> Option<&str> {
        match self.unsplash_access_key {
            Some(ref x) => Some(x),
            None => None
        }
    }

    #[inline]
    pub fn get_unsplash_secret_key(&self) -> Option<&str> {
        match self.unsplash_secret_key {
            Some(ref x) => Some(x),
            None => None
        }
    }

    #[inline]
    pub fn is_unsplash_available(&self) -> bool {
        self.unsplash_secret_key.is_some() && self.unsplash_access_key.is_some()
    }

    #[inline]
    pub fn get_facebook_access_token(&self) -> Option<&str> {
        match self.facebook_access_token {
            Some(ref x) => Some(x),
            None => None
        }
    }

    #[inline]
    pub fn is_facebook_access_token_available(&self) -> bool {
        self.facebook_access_token.is_some()
    }
}
