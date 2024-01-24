use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ExecutionMode {
    Regular,
    PortableSimd,
    Asm,
}

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

    /// Execution mode
    #[arg(short, long, value_enum, default_value_t=ExecutionMode::Regular)]
    pub mode: ExecutionMode,
}
