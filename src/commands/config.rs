// ABOUTME: Configuration management commands.
// ABOUTME: Handles setup, show, and list operations for profiles.

use anyhow::Result;
use clap::Subcommand;
use dialoguer::Password;

use crate::client::ResendClient;
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Set up a new configuration profile
    Setup {
        /// Profile name
        #[arg(long, default_value = "default")]
        profile: String,
    },

    /// Show current configuration
    Show {
        /// Profile name
        #[arg(long)]
        profile: Option<String>,
    },

    /// List all profiles
    List,
}

impl ConfigCommands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            ConfigCommands::Setup { profile } => setup_config(profile).await,
            ConfigCommands::Show { profile } => show_config(profile.as_deref()),
            ConfigCommands::List => list_profiles(),
        }
    }
}

async fn setup_config(profile: &str) -> Result<()> {
    println!("Setting up profile: {}", profile);
    println!();

    let api_key: String = Password::new()
        .with_prompt("API Key")
        .interact()?;

    if api_key.is_empty() {
        eprintln!("Error: API key cannot be empty");
        std::process::exit(1);
    }

    println!();
    println!("Testing connection...");

    let client = ResendClient::new(&api_key)?;
    match client.test_connection().await {
        Ok(_) => println!("Connection successful!"),
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            std::process::exit(1);
        }
    }

    Config::set_profile(profile, &api_key)?;

    let config_path = Config::config_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!();
    println!("Configuration saved to {}", config_path);

    Ok(())
}

fn show_config(profile: Option<&str>) -> Result<()> {
    let config = Config::load(profile, None, None, false)?;

    println!("Profile: {}", config.profile);
    println!(
        "API Key: {}",
        config
            .api_key
            .as_ref()
            .map(|k| Config::mask_key(k))
            .unwrap_or_else(|| "(not set)".to_string())
    );

    if let Some(path) = Config::config_path() {
        println!("Config file: {}", path.display());
    }

    Ok(())
}

fn list_profiles() -> Result<()> {
    let profiles = Config::list_profiles()?;

    if profiles.is_empty() {
        println!("No profiles configured.");
        println!("Run 'resend config setup' to create one.");
    } else {
        println!("Configured profiles:");
        for profile in profiles {
            println!("  - {}", profile);
        }
    }

    Ok(())
}
