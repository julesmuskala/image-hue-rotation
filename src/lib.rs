pub mod cli;
mod rgb_rotate;

use image::io::Reader as ImageReader;

use crate::rgb_rotate::RGBRotate;

pub fn run(input_path: &str, output_path: &str, angle: i32) -> Result<(), &'static str> {
    let image = match match ImageReader::open(input_path) {
        Ok(buf) => buf,
        Err(_) => return Err("couldn't open file"),
    }
    .decode()
    {
        Ok(image) => image,
        Err(_) => return Err("couldn't decode file"),
    };

    let bytes = image.as_bytes();

    let rgb_rotate = RGBRotate::new(angle);

    let rotated_bytes = rgb_rotate.rotate_pixels(bytes);

    match image::save_buffer(
        output_path,
        &rotated_bytes,
        image.width(),
        image.height(),
        image::ColorType::Rgb8,
    ) {
        Ok(_) => (),
        Err(_) => return Err("couldn't save file"),
    };

    Ok(())
}
