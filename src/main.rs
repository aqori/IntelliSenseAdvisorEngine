// src/main.rs
/*
 * Main executable for IntelliSenseAdvisorEngine
 */

use clap::Parser;
use intellisenseadvisorengine::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliSenseAdvisorEngine - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
