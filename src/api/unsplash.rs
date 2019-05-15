pub struct Unsplash {
    client_key: &'static str,
    secret_key: &'static str,
    reqwest_client: reqwest::Client,
}

impl Unsplash {
    pub fn new(client_key: &'static str, secret_key: &'static str) -> Self {
        Self {
            client_key,
            secret_key,
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub fn random_photo(&self) -> serde_json::Value {
        let mut req = self
            .reqwest_client
            .get("https://api.unsplash.com/photos/random")
            .query(&[
                ("client_id", self.client_key),
                ("client_secret", self.secret_key),
            ])
            .send()
            .expect("Error when trying to connect to unsplash");
        serde_json::from_str::<serde_json::Value>(
            &req.text()
                .expect("Error when trying to extracting body text"),
        )
        .expect("Error when decoding the json text")
    }
}
