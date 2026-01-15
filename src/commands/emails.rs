// ABOUTME: Email management commands.
// ABOUTME: Send, list, get, cancel, and update emails.

use anyhow::Result;
use clap::Subcommand;

use crate::client::ResendClient;
use crate::commands::{build_config, require_valid_config, CommonArgs};
use crate::formatters::{format_and_output, format_and_output_single};
use crate::types::{SendEmailRequest, UpdateEmailRequest};

#[derive(Debug, Subcommand)]
pub enum EmailsCommands {
    /// Send an email
    Send {
        /// Sender email address
        #[arg(long)]
        from: String,

        /// Recipient email address(es)
        #[arg(long, required = true)]
        to: Vec<String>,

        /// Email subject
        #[arg(long)]
        subject: String,

        /// HTML content
        #[arg(long)]
        html: Option<String>,

        /// Plain text content
        #[arg(long)]
        text: Option<String>,

        /// CC recipients
        #[arg(long)]
        cc: Option<Vec<String>>,

        /// BCC recipients
        #[arg(long)]
        bcc: Option<Vec<String>>,

        /// Reply-to addresses
        #[arg(long)]
        reply_to: Option<Vec<String>>,

        /// Schedule send time (ISO 8601)
        #[arg(long)]
        scheduled_at: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Get an email by ID
    Get {
        /// Email ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// List emails
    List {
        #[command(flatten)]
        common: CommonArgs,
    },

    /// Cancel a scheduled email
    Cancel {
        /// Email ID
        id: String,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Update a scheduled email
    Update {
        /// Email ID
        id: String,

        /// New scheduled time (ISO 8601)
        #[arg(long)]
        scheduled_at: String,

        #[command(flatten)]
        common: CommonArgs,
    },
}

impl EmailsCommands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            EmailsCommands::Send {
                from,
                to,
                subject,
                html,
                text,
                cc,
                bcc,
                reply_to,
                scheduled_at,
                common,
            } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;

                let req = SendEmailRequest {
                    from: from.clone(),
                    to: to.clone(),
                    subject: subject.clone(),
                    html: html.clone(),
                    text: text.clone(),
                    cc: cc.clone(),
                    bcc: bcc.clone(),
                    reply_to: reply_to.clone(),
                    scheduled_at: scheduled_at.clone(),
                };

                let response = client.send_email(req).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&response)?);
                } else {
                    println!("Email sent successfully!");
                    println!("ID: {}", response.id);
                }

                Ok(())
            }

            EmailsCommands::Get { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let email = client.get_email(id).await?;

                format_and_output_single(&email, config.format, config.output.as_deref())
            }

            EmailsCommands::List { common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let emails = client.list_emails().await?;

                format_and_output(&emails, config.format, config.output.as_deref())
            }

            EmailsCommands::Cancel { id, common } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;
                let email = client.cancel_email(id).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&email)?);
                } else {
                    println!("Email cancelled successfully!");
                    println!("ID: {}", email.id);
                }

                Ok(())
            }

            EmailsCommands::Update {
                id,
                scheduled_at,
                common,
            } => {
                let config = build_config(common)?;
                require_valid_config(&config);

                let client = ResendClient::new(config.api_key.as_ref().unwrap())?;

                let req = UpdateEmailRequest {
                    scheduled_at: scheduled_at.clone(),
                };

                let email = client.update_email(id, req).await?;

                if common.json {
                    println!("{}", serde_json::to_string_pretty(&email)?);
                } else {
                    println!("Email updated successfully!");
                    println!("ID: {}", email.id);
                }

                Ok(())
            }
        }
    }
}
