use clap::Parser;
use cli::Args;

mod cli;

fn main() {
    Args::parse();

    println!("Hello, world!");
}
