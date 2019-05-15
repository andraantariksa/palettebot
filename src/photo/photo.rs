use image::GenericImage as _;
use image::GenericImageView as _;

pub struct Photo {
    //pub extension: image::ImageFormat,
    pub data: image::DynamicImage,
}

impl Photo {
    pub fn from_file(path: &str) -> Self {
        Self {
            data: image::open(&std::path::Path::new(path)).expect("Error when opening the file"),
        }
    }

    pub fn from_url(url: &str) -> Self {
        let mut image_request = reqwest::Client::new().get(url).send().unwrap();

        /*
        // Analyse file extension by it's MIME
        let mime = image_request
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .expect("Can't extract MIME");
        let extension = mime
            .to_str()
            .expect("Error when trying to convert MIME to string")
            .split("image/")
            .last()
            .expect("Can't get file extension, it might not an image file!")
            .to_owned();
        */

        let mut image_data: Vec<u8> = Vec::new();
        image_request
            .copy_to(&mut image_data)
            .expect("Error when copying the image");
        Self {
            //extension,
            data: image::load_from_memory(&image_data).unwrap(),
        }
    }

    pub fn resize_with_aspect_ratio_one_side(&mut self, side_size: u32) {
        if self.data.width() > self.data.height() {
            self.data = self
                .data
                .resize(self.data.height(), side_size, image::Nearest);
        } else {
            self.data = self
                .data
                .resize(side_size, self.data.width(), image::Nearest);
        }
    }
}
