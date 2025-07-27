// src/main.rs
/*
 * Main executable for PulseTech
 */

use clap::Parser;
use pulsetech::{Result, run};

#[derive(Parser)]
#[command(version, about = "PulseTech - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
