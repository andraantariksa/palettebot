pub struct Facebook {
    access_token: String,
    reqwest_client: reqwest::Client,
}

impl Facebook {
    pub fn new(access_token: &str) -> Facebook {
        Facebook {
            access_token: access_token.to_string(),
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub fn post_image(&self, img_path: &str, message: String) -> String {
        let form = reqwest::multipart::Form::new()
            .text("access_token", self.access_token.clone())
            .text("message", message)
            .file("source", img_path)
            .expect("Facebook form error");
        let mut req = self
            .reqwest_client
            .post("https://graph.facebook.com/me/photos")
            .multipart(form)
            .send()
            .expect("Error when uploading an image to Facebook");
        serde_json::from_str::<serde_json::Value>(
            &req.text()
                .expect("Error when trying to extracting body text"),
        )
        .expect("Error when decoding the json text")["post_id"]
            .as_str()
            .expect("Error when trying to process image url data")
            .to_string()
    }

    pub fn comment(&self, id: &str, message: &str) {
        self.reqwest_client
            .post(&format!("https://graph.facebook.com/{}/comments", id))
            .query(&[("message", message), ("access_token", &self.access_token)])
            .send()
            .expect("Error when trying to retrieve post comment");
    }
}
