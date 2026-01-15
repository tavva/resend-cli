// ABOUTME: Domain management commands.
// ABOUTME: Create, list, verify, update, and delete domains.

use anyhow::Result;
use clap::Subcommand;

use crate::client::ResendClient;
use crate::commands::{build_config, require_valid_config, CommonArgs};
use crate::formatters::{format_and_output, format_and_output_single};
use crate::types::{CreateDomainRequest, UpdateDomainRequest};

#[derive(Debug, Subcommand)]
pub enum DomainsCommands {
    /// Create a new domain
    Create {
        /// Domain name
        name: String,

        /// Region (us-east-1, eu-west-1, sa-east-1)
        #[arg(long)]
        region: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// List all domains
    List {
        #[command(flatten)]
        common: CommonArgs,
    },

    /// Get a domain by ID
    Get {
        /// Domain ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Verify a domain
    Verify {
        /// Domain ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Update a domain
    Update {
        /// Domain ID
        id: String,

        /// Enable click tracking
        #[arg(long)]
        click_tracking: Option<bool>,

        /// Enable open tracking
        #[arg(long)]
        open_tracking: Option<bool>,

        /// TLS setting (enforced, opportunistic)
        #[arg(long)]
        tls: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Delete a domain
    Delete {
        /// Domain ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },
}

impl DomainsCommands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            DomainsCommands::Create {
                name,
                region,
                common,
            } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;

                let req = CreateDomainRequest {
                    name: name.clone(),
                    region: region.clone(),
                };

                let domain = client.create_domain(req).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&domain)?);
                } else {
                    println!("Domain created successfully!");
                    println!("ID: {}", domain.id);
                    println!("Name: {}", domain.name);
                    if let Some(records) = &domain.records {
                        println!();
                        println!("DNS Records to add:");
                        for record in records {
                            println!(
                                "  {} {} -> {}",
                                record.record, record.name, record.value
                            );
                        }
                    }
                }

                Ok(())
            }

            DomainsCommands::List { common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let domains = client.list_domains().await?;

                format_and_output(&domains, config.format, config.output.as_deref())
            }

            DomainsCommands::Get { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let domain = client.get_domain(id).await?;

                format_and_output_single(&domain, config.format, config.output.as_deref())
            }

            DomainsCommands::Verify { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let domain = client.verify_domain(id).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&domain)?);
                } else {
                    println!("Verification initiated!");
                    println!("ID: {}", domain.id);
                    println!(
                        "Status: {}",
                        domain.status.as_deref().unwrap_or("pending")
                    );
                }

                Ok(())
            }

            DomainsCommands::Update {
                id,
                click_tracking,
                open_tracking,
                tls,
                common,
            } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;

                let req = UpdateDomainRequest {
                    click_tracking: *click_tracking,
                    open_tracking: *open_tracking,
                    tls: tls.clone(),
                };

                let domain = client.update_domain(id, req).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&domain)?);
                } else {
                    println!("Domain updated successfully!");
                    println!("ID: {}", domain.id);
                }

                Ok(())
            }

            DomainsCommands::Delete { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                client.delete_domain(id).await?;

                if !common.json {
                    println!("Domain deleted successfully!");
                }

                Ok(())
            }
        }
    }
}
