// ABOUTME: Data types for Resend API requests and responses.
// ABOUTME: Includes serialization and table formatting traits.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Output format for CLI results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
}

/// Trait for types that can be displayed as tables
pub trait Tabular {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
}

// === Email Types ===

#[derive(Debug, Serialize)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateEmailRequest {
    pub scheduled_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendEmailResponse {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Email {
    pub id: String,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<Vec<String>>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub last_event: Option<String>,
}

impl Tabular for Email {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "TO", "SUBJECT", "STATUS", "CREATED"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.to.as_ref().map(|v| v.join(", ")).unwrap_or_default(),
            self.subject.clone().unwrap_or_default(),
            self.last_event.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailsResponse {
    pub data: Vec<Email>,
}

// === Domain Types ===

#[derive(Debug, Serialize)]
pub struct CreateDomainRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateDomainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Domain {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub records: Option<Vec<DnsRecord>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DnsRecord {
    pub record: String,
    pub name: String,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub ttl: Option<String>,
    pub value: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub priority: Option<i32>,
}

impl Tabular for Domain {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "STATUS", "REGION"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.status.clone().unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainsResponse {
    pub data: Vec<Domain>,
}

// === API Key Types ===

#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl Tabular for ApiKey {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "CREATED"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.created_at.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKeysResponse {
    pub data: Vec<ApiKey>,
}

// === Template Types ===

#[derive(Debug, Serialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl Tabular for Template {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "SUBJECT", "CREATED"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.subject.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplatesResponse {
    pub data: Vec<Template>,
}

// === Error Response ===

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}
