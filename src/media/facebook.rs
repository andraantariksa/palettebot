use serde::{Deserialize, Serialize};
use crate::error::Result;
use std::borrow::Cow;

pub struct Facebook<'a> {
    access_token: &'a str
}

#[derive(Deserialize, Debug)]
pub struct PostImageFacebookResp {
    pub post_id: String
}

#[derive(Serialize, Debug)]
struct CommentImageFacebookReq<'a> {
    access_token: &'a str,
    message: &'a str
}

#[derive(Deserialize, Debug)]
struct CommentImageFacebookResp {
    success: bool
}

impl<'a> Facebook<'a> {
    pub fn new(access_token: &'a str) -> Facebook {
        Facebook {
            access_token
        }
    }

    pub async fn post_image<T, U>(
        &self,
        image_buf: T,
        message: U) -> Result<PostImageFacebookResp>
    where
        T: Into<Cow<'static, [u8]>>,
        U: Into<Cow<'static, str>> {
        let image_part = reqwest::multipart::Part::bytes(image_buf)
            .file_name("palettebot.png")
            .mime_str("image/png")
            .unwrap();

        let multipart_form = reqwest::multipart::Form::new()
            .text("access_token", self.access_token.to_owned())
            .text("message", message)
            .part("source", image_part);
        Ok(reqwest::Client::new()
            .post("https://graph.facebook.com/me/photos")
            .multipart(multipart_form)
            .send()
            .await?
            .json::<PostImageFacebookResp>()
            .await?)
    }

    pub async fn comment<'b>(
        &self,
        id: &'b str,
        message: &'b str) -> Result<CommentImageFacebookResp> {
        Ok(reqwest::Client::new()
            .post(&format!("https://graph.facebook.com/{}/comments", id))
            .json(&CommentImageFacebookReq {
                access_token: self.access_token,
                message
            })
            .send()
            .await?
            .json::<CommentImageFacebookResp>()
            .await?)
    }
}
