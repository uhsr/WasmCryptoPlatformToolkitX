// src/main.rs
/*
 * Main executable for WasmCryptoPlatformToolkitX
 */

use clap::Parser;
use wasmcryptoplatformtoolkitx::{Result, run};

#[derive(Parser)]
#[command(version, about = "WasmCryptoPlatformToolkitX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
