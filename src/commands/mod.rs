// ABOUTME: Command implementations for the Resend CLI.
// ABOUTME: Each submodule handles a resource type (emails, domains, etc.).

pub mod api_keys;
pub mod config;
pub mod domains;
pub mod emails;
pub mod templates;

use anyhow::Result;

use crate::config::Config;
use crate::types::OutputFormat;

/// Common arguments shared across commands
#[derive(Debug, Clone, clap::Args)]
pub struct CommonArgs {
    /// Output format
    #[arg(long)]
    pub json: bool,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Profile name
    #[arg(long)]
    pub profile: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl CommonArgs {
    pub fn format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            OutputFormat::Table
        }
    }
}

/// Build config from common arguments
pub fn build_config(args: &CommonArgs) -> Result<Config> {
    Config::load(
        args.profile.as_deref(),
        Some(args.format()),
        args.output.as_deref(),
        args.verbose,
    )
}

/// Check config validity and exit if invalid
pub fn require_valid_config(config: &Config) {
    if !config.is_valid() {
        crate::formatters::output_error(
            "missing_credentials",
            "Missing API key. Run 'resend config setup' or set RESEND_API_KEY.",
        );
        std::process::exit(1);
    }
}
