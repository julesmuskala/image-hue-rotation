use clap::Parser;
use image_hue_rotation::{cli::Args, run};

fn main() {
    let args = Args::parse();

    match run(&args.input_path, &args.output_path, args.angle) {
        Ok(_) => println!("done"),
        Err(e) => eprintln!("{}", e),
    };
}
