// ABOUTME: Entry point for the Resend CLI.
// ABOUTME: Parses arguments and dispatches to command handlers.

use anyhow::Result;
use clap::{Parser, Subcommand};

mod client;
mod commands;
mod config;
mod formatters;
mod types;

use commands::api_keys::ApiKeysCommands;
use commands::config::ConfigCommands;
use commands::domains::DomainsCommands;
use commands::emails::EmailsCommands;
use commands::templates::TemplatesCommands;

/// Resend CLI - Command-line interface for the Resend email platform
#[derive(Parser)]
#[command(name = "resend")]
#[command(author = "Ben")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Command-line interface for the Resend email platform", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage configuration profiles
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Send and manage emails
    #[command(subcommand)]
    Emails(EmailsCommands),

    /// Manage domains
    #[command(subcommand)]
    Domains(DomainsCommands),

    /// Manage API keys
    #[command(subcommand, name = "api-keys")]
    ApiKeys(ApiKeysCommands),

    /// Manage email templates
    #[command(subcommand)]
    Templates(TemplatesCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();

    match cli.command {
        Commands::Config(cmd) => cmd.execute().await,
        Commands::Emails(cmd) => cmd.execute().await,
        Commands::Domains(cmd) => cmd.execute().await,
        Commands::ApiKeys(cmd) => cmd.execute().await,
        Commands::Templates(cmd) => cmd.execute().await,
    }
}
