// ABOUTME: Template management commands.
// ABOUTME: Create, list, get, update, and delete email templates.

use anyhow::Result;
use clap::Subcommand;

use crate::client::ResendClient;
use crate::commands::{build_config, require_valid_config, CommonArgs};
use crate::formatters::{format_and_output, format_and_output_single};
use crate::types::{CreateTemplateRequest, UpdateTemplateRequest};

#[derive(Debug, Subcommand)]
pub enum TemplatesCommands {
    /// Create a new template
    Create {
        /// Template name
        name: String,

        /// Email subject
        #[arg(long)]
        subject: String,

        /// HTML content
        #[arg(long)]
        html: Option<String>,

        /// Plain text content
        #[arg(long)]
        text: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// List all templates
    List {
        #[command(flatten)]
        common: CommonArgs,
    },

    /// Get a template by ID
    Get {
        /// Template ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Update a template
    Update {
        /// Template ID
        id: String,

        /// New name
        #[arg(long)]
        name: Option<String>,

        /// New subject
        #[arg(long)]
        subject: Option<String>,

        /// New HTML content
        #[arg(long)]
        html: Option<String>,

        /// New plain text content
        #[arg(long)]
        text: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Delete a template
    Delete {
        /// Template ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },
}

impl TemplatesCommands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            TemplatesCommands::Create {
                name,
                subject,
                html,
                text,
                common,
            } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;

                let req = CreateTemplateRequest {
                    name: name.clone(),
                    subject: subject.clone(),
                    html: html.clone(),
                    text: text.clone(),
                };

                let template = client.create_template(req).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&template)?);
                } else {
                    println!("Template created successfully!");
                    println!("ID: {}", template.id);
                    println!("Name: {}", template.name);
                }

                Ok(())
            }

            TemplatesCommands::List { common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let templates = client.list_templates().await?;

                format_and_output(&templates, config.format, config.output.as_deref())
            }

            TemplatesCommands::Get { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let template = client.get_template(id).await?;

                format_and_output_single(&template, config.format, config.output.as_deref())
            }

            TemplatesCommands::Update {
                id,
                name,
                subject,
                html,
                text,
                common,
            } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;

                let req = UpdateTemplateRequest {
                    name: name.clone(),
                    subject: subject.clone(),
                    html: html.clone(),
                    text: text.clone(),
                };

                let template = client.update_template(id, req).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&template)?);
                } else {
                    println!("Template updated successfully!");
                    println!("ID: {}", template.id);
                }

                Ok(())
            }

            TemplatesCommands::Delete { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                client.delete_template(id).await?;

                if !common.json {
                    println!("Template deleted successfully!");
                }

                Ok(())
            }
        }
    }
}
