use clap::Parser;

/// Rotate image hue
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input_path: String,

    /// Output file path
    #[arg(short, long)]
    pub output_path: String,

    /// Hue rotation angle in degrees
    #[arg(short, long, default_value = "0", allow_hyphen_values = true)]
    pub angle: i32,
}
