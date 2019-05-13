extern crate image;
extern crate pexels;
extern crate rand;
extern crate reqwest;
extern crate serde_json;

const PEXELS_API_KEY: &'static str = "";
const FACEBOOK_PAGE_ACCESS_TOKEN: &'static str = "";

mod media;

use image::GenericImage as _;
use image::GenericImageView as _;
use rand::Rng as _;
use std::error::Error as _;

fn main() {
    // Get an image from Pexels
    let con_pexels = pexels::Pexels::new(PEXELS_API_KEY.to_string());
    let searched = con_pexels.curated_photo(1, rand::thread_rng().gen::<u32>() % 1000 + 1);
    let image_url = searched["photos"][0]["src"]["original"].as_str().unwrap();
    let url = searched["photos"][0]["url"].as_str().unwrap();

    println!("Pexels URL: {}", url);
    println!("Image URL: {}", image_url);

    let file_ext = image_url.split('.').last().unwrap();

    println!("Extension: {}", file_ext);

    let mut image_file = reqwest::Client::new().get(image_url).send().unwrap();
    let file_path = format!("{}{}", "tmp/input.", file_ext);
    let path = std::path::Path::new(&file_path);
    let display = path.display();
    let mut file = match std::fs::File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    match std::io::copy(&mut image_file, &mut file) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Cuccessfully wrote to {}", display),
    }

    // Open the image
    let mut img = image::open(&file_path).unwrap();
    // Resize image if the the height is too large
    if img.width() > img.height() {
        //img.resize(600, 600, image::Nearest);
        img = img.resize(img.height(), 600, image::Nearest);
    } else {
        img = img.resize(600, img.width(), image::Nearest);
    }
    //img.save("tmp/output_resized.png").unwrap();

    // Preparing the canvas
    let mut canvas = image::ImageBuffer::new(600, 700);
    for (_, _, pixel) in canvas.enumerate_pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }
    // Crop the image and copy the cropped image to canvas
    img = img.crop(0, 0, 600, 600);
    canvas.copy_from(&img, 0, 0);

    let mut colors_list = Vec::new();
    for i in (0..6 * 2).step_by(2) {
        let color = img.get_pixel(50 * i, rand::thread_rng().gen::<u32>() % 600);
        colors_list.push(format!(
            "#{:02X}{:02X}{:02X}",
            color.data[0], color.data[1], color.data[2]
        ));
        let mut square = image::ImageBuffer::new(50, 50);
        for (_, _, pixel) in square.enumerate_pixels_mut() {
            *pixel = color;
        }
        canvas.copy_from(&square, 25 + 50 * i, 625);
    }

    // Save the output
    canvas.save("tmp/output.png").unwrap();

    let caption = format!(
        "Colors extracted: {}\n Image from: {}",
        colors_list.join(", "),
        url
    );

    // Connect to Facebook
    let con_fb = media::facebook::Facebook::new(FACEBOOK_PAGE_ACCESS_TOKEN);
    let con_fb_image_post_id = con_fb.post_image("tmp/output.png", String::new());
    con_fb.comment(&con_fb_image_post_id, &caption)
}
