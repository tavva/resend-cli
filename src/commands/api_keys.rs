// ABOUTME: API key management commands.
// ABOUTME: Create, list, and delete API keys.

use anyhow::Result;
use clap::Subcommand;

use crate::client::ResendClient;
use crate::commands::{build_config, require_valid_config, CommonArgs};
use crate::formatters::format_and_output;
use crate::types::CreateApiKeyRequest;

#[derive(Debug, Subcommand)]
pub enum ApiKeysCommands {
    /// Create a new API key
    Create {
        /// Key name
        name: String,

        /// Permission level (full_access, sending_access)
        #[arg(long)]
        permission: Option<String>,

        /// Restrict to domain ID
        #[arg(long)]
        domain_id: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// List all API keys
    List {
        #[command(flatten)]
        common: CommonArgs,
    },

    /// Delete an API key
    Delete {
        /// API key ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },
}

impl ApiKeysCommands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            ApiKeysCommands::Create {
                name,
                permission,
                domain_id,
                common,
            } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;

                let req = CreateApiKeyRequest {
                    name: name.clone(),
                    permission: permission.clone(),
                    domain_id: domain_id.clone(),
                };

                let api_key = client.create_api_key(req).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&api_key)?);
                } else {
                    println!("API key created successfully!");
                    println!("ID: {}", api_key.id);
                    println!("Name: {}", api_key.name);
                    if let Some(token) = &api_key.token {
                        println!();
                        println!("Token: {}", token);
                        println!();
                        println!("Save this token - it won't be shown again!");
                    }
                }

                Ok(())
            }

            ApiKeysCommands::List { common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let api_keys = client.list_api_keys().await?;

                format_and_output(&api_keys, config.format, config.output.as_deref())
            }

            ApiKeysCommands::Delete { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                client.delete_api_key(id).await?;

                if !common.json {
                    println!("API key deleted successfully!");
                }

                Ok(())
            }
        }
    }
}
