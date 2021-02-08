mod util;

use palettebot_lib::config::Config;
use palettebot_lib::media::*;
use palettebot_lib::image::image::Image;

#[tokio::test]
async fn post_image_to_facebook() -> Result<(), Box<dyn std::error::Error>> {
    let config = util::setup_config();

    let img = match Image::from_file("C:\\Users\\andra\\Projects\\palettebot\\target\\debug\\output.png") {
        Ok(x) => x,
        Err(_) => panic!("Cannot load the file!")
    };

    // let fb = facebook::Facebook::new(
    //     config.get_facebook_access_token().unwrap());
    // let resp = fb.post_image(
    //     &img
    //         .get_png_bytes()
    //         .unwrap()
    //         .as_slice(),
    //     "test")
    //     .await
    //     .ok()
    //     .unwrap();
    //
    // let mut body = surf::Body::from_json(
    //     &PostImageFacebookReq {
    //         access_token: "",
    //         message: "test",
    //         source:
    //     }
    // )?;
    //body.set_mime(surf::http::mime::MULTIPART_FORM);

    let img = reqwest::multipart::Part::bytes(img.get_png_bytes().unwrap())
        .file_name("palettebot.png")
        .mime_str("image/png")
        .unwrap();

    let multipart_form = reqwest::multipart::Form::new()
        .text("access_token", "")
        .text("message", "test")
        .part("source", img);

    let g = reqwest::Client::new()
        .post("https://graph.facebook.com/me/photos")
        .multipart(multipart_form)
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", g);
    Ok(())
}
