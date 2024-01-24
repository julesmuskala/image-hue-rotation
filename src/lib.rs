#![feature(portable_simd)]

pub mod cli;
mod rgb_rotate;

use cli::ExecutionMode;
use image::io::Reader as ImageReader;

use crate::rgb_rotate::RGBRotate;

pub fn run(
    input_path: &str,
    output_path: &str,
    angle: i32,
    execution_mode: ExecutionMode,
) -> Result<(), &'static str> {
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

    let rotated_bytes = match execution_mode {
        ExecutionMode::Regular => rgb_rotate.rotate_pixels(bytes),
        ExecutionMode::PortableSimd => rgb_rotate.rotate_pixels_portable_simd(bytes),
        ExecutionMode::Asm => rgb_rotate.rotate_pixels_asm(bytes),
    };

    if let Err(_) = image::save_buffer(
        output_path,
        &rotated_bytes,
        image.width(),
        image.height(),
        image::ColorType::Rgb8,
    ) {
        return Err("couldn't save file");
    }

    Ok(())
}
