// ABOUTME: Entry point for the Resend CLI.
// ABOUTME: Parses arguments and dispatches to command handlers.

use anyhow::Result;
use clap::Parser;

mod client;
mod config;
mod formatters;
mod types;

/// Resend CLI - Command-line interface for the Resend email platform
#[derive(Parser)]
#[command(name = "resend")]
#[command(author = "Ben")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Command-line interface for the Resend email platform", long_about = None)]
#[command(propagate_version = true)]
struct Cli {}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let _cli = Cli::parse();
    println!("resend-cli");
    Ok(())
}
