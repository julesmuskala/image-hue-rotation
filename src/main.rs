use clap::Parser;
use image_hue_rotation::{cli::Args, run};
use std::process;

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args.input_path, &args.output_path, args.angle, args.mode) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
