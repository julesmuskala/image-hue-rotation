use std::path::PathBuf;

use clap::Parser;

/// Rotate image hue
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    input_path: PathBuf,

    /// Output file path
    #[arg(short, long)]
    output_path: PathBuf,

    /// Hue rotation angle in degrees
    #[arg(short, long, default_value = "0")]
    angle: i32,
}