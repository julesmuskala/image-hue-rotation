use clap::Parser;
use cli::Args;
use image::io::Reader as ImageReader;
use rgb_rotate::RGBRotate;

mod cli;
mod rgb_rotate;

fn main() {
    let args = Args::parse();

    let image = ImageReader::open(args.input_path)
        .expect("File not found")
        .decode()
        .expect("Could not decode image");

    let bytes = image.as_bytes();

    let rgb_rotate = RGBRotate::new(args.angle);

    let rotated_bytes = bytes
        .chunks_exact(3)
        .map(|pixel| rgb_rotate.rotate_pixel(pixel[0], pixel[1], pixel[2]))
        .flatten()
        .collect::<Vec<u8>>();

    image::save_buffer(
        args.output_path,
        &rotated_bytes,
        image.width(),
        image.height(),
        image::ColorType::Rgb8,
    )
    .expect("Could not save image");
}
