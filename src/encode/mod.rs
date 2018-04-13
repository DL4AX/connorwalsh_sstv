extern crate image;

use image::ImageResult;

pub fn resize_img(fname: &str) -> ImageResult<DynamicImage> {
    /// reads in an image
    println!("{}", fname);
    println!(env!("CARGO_MANIFEST_DIR"));
}
