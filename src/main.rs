extern crate image;
extern crate pexels;
extern crate rand;
extern crate reqwest;
extern crate serde_json;

mod constant;

mod api;
mod media;
mod photo;

use image::GenericImage as _;
use image::GenericImageView as _;
use rand::Rng as _;

fn main() {
    let random_photo = photo::random::RandomPhoto::new();
    println!("Post Source: {}", random_photo.post_source);
    println!("Image URL: {}", random_photo.file_url);

    let mut img = photo::photo::Photo::from_url(&random_photo.file_url);
    img.resize_with_aspect_ratio_one_side(600);

    // Preparing the image buffer
    let mut canvas = image::ImageBuffer::new(600, 700);
    // White background
    for (_, _, pixel) in canvas.enumerate_pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }
    // Crop the photo and copy the cropped image to image buffer
    img.data = img.data.crop(0, 0, 600, 600);
    canvas.copy_from(&img.data, 0, 0);

    // Choosing random colors from the image buffer
    let mut colors_list = Vec::new();
    for i in (0..6 * 2).step_by(2) {
        let color = img
            .data
            .get_pixel(50 * i, rand::thread_rng().gen::<u32>() % 600);
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
        random_photo.post_source
    );

    // Connect to Facebook
    let con_fb = media::facebook::Facebook::new(constant::FACEBOOK_PAGE_ACCESS_TOKEN);
    let con_fb_image_post_id = con_fb.post_image("tmp/output.png", String::new());
    // Post to Facebook
    con_fb.comment(&con_fb_image_post_id, &caption)
}
