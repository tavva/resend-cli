# Resend CLI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI for managing the Resend email platform, designed for Claude Code skill integration.

**Architecture:** Clap-based CLI with subcommands matching SDK structure (`resend emails send`). YAML config with profiles, env var override. Human-readable table output by default, JSON via `--json` flag. Follows `lf` CLI patterns exactly.

**Tech Stack:** Rust, clap 4, tokio, reqwest, serde, tabled, wiremock

---

### Task 1: Project Setup

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "resend"
version = "0.1.0"
edition = "2021"
description = "A command-line interface for the Resend email platform"
authors = ["Ben"]
license = "MIT"

[[bin]]
name = "resend"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive", "env"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
chrono = { version = "0.4", features = ["serde"] }
tabled = "0.16"
directories = "5"
dirs = "5"
thiserror = "2"
anyhow = "1"
dialoguer = "0.11"
dotenvy = "0.15"

[dev-dependencies]
wiremock = "0.6"
tempfile = "3"

[profile.release]
lto = true
codegen-units = 1
strip = true
```

**Step 2: Create minimal main.rs**

```rust
// ABOUTME: Entry point for the Resend CLI.
// ABOUTME: Parses arguments and dispatches to command handlers.

use anyhow::Result;
use clap::Parser;

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
```

**Step 3: Verify it compiles and runs**

Run: `cargo build`
Expected: Compiles successfully

Run: `cargo run`
Expected: Prints "resend-cli"

Run: `cargo run -- --help`
Expected: Shows help with version info

**Step 4: Commit**

```bash
git add Cargo.toml src/main.rs
git commit -m "feat: initial project setup with clap skeleton"
```

---

### Task 2: Types Module

**Files:**
- Create: `src/types.rs`
- Modify: `src/main.rs`

**Step 1: Create types.rs with OutputFormat enum**

```rust
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
```

**Step 2: Add module to main.rs**

Add after the imports in `src/main.rs`:

```rust
mod types;
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/types.rs src/main.rs
git commit -m "feat: add types module with API request/response structs"
```

---

### Task 3: Config Module

**Files:**
- Create: `src/config.rs`
- Modify: `src/main.rs`

**Step 1: Write failing test for config loading**

Create `src/config.rs`:

```rust
// ABOUTME: Configuration management for the Resend CLI.
// ABOUTME: Handles YAML config files, profiles, and environment variables.

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::types::OutputFormat;

const DEFAULT_PROFILE: &str = "default";

/// Profile configuration stored in config file
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    pub api_key: Option<String>,
}

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

/// Runtime configuration with resolved values
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub profile: String,
    pub format: OutputFormat,
    pub output: Option<String>,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            profile: DEFAULT_PROFILE.to_string(),
            format: OutputFormat::Table,
            output: None,
            verbose: false,
        }
    }
}

impl Config {
    /// Get the config file path
    pub fn config_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "resend") {
            let config_dir = proj_dirs.config_dir();
            Some(config_dir.join("config.yml"))
        } else {
            dirs::home_dir().map(|home| home.join(".resend").join("config.yml"))
        }
    }

    /// Load configuration file
    pub fn load_config_file() -> Result<ConfigFile> {
        let path = Self::config_path();

        if let Some(path) = path {
            if path.exists() {
                let contents = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read config file: {path:?}"))?;
                let config: ConfigFile = serde_yaml::from_str(&contents)
                    .with_context(|| "Failed to parse config file")?;
                return Ok(config);
            }
        }

        Ok(ConfigFile::default())
    }

    /// Save configuration file
    pub fn save_config_file(config_file: &ConfigFile) -> Result<()> {
        let path = Self::config_path()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config file path"))?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {parent:?}"))?;
        }

        let contents =
            serde_yaml::to_string(config_file).with_context(|| "Failed to serialize config")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write config file: {path:?}"))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    /// Load configuration with priority: env vars > config file > defaults
    pub fn load(
        profile: Option<&str>,
        format: Option<OutputFormat>,
        output: Option<&str>,
        verbose: bool,
    ) -> Result<Self> {
        let profile_name = profile
            .map(|s| s.to_string())
            .or_else(|| std::env::var("RESEND_PROFILE").ok())
            .unwrap_or_else(|| DEFAULT_PROFILE.to_string());

        let config_file = Self::load_config_file().unwrap_or_default();
        let file_profile = config_file.profiles.get(&profile_name);

        // Resolve API key: env > config file
        let resolved_api_key = std::env::var("RESEND_API_KEY")
            .ok()
            .or_else(|| file_profile.and_then(|p| p.api_key.clone()));

        Ok(Self {
            api_key: resolved_api_key,
            profile: profile_name,
            format: format.unwrap_or(OutputFormat::Table),
            output: output.map(|s| s.to_string()),
            verbose,
        })
    }

    /// Check if configuration has required credentials
    pub fn is_valid(&self) -> bool {
        self.api_key.is_some()
    }

    /// Set a profile in the config file
    pub fn set_profile(profile_name: &str, api_key: &str) -> Result<()> {
        let mut config_file = Self::load_config_file().unwrap_or_default();

        config_file.profiles.insert(
            profile_name.to_string(),
            Profile {
                api_key: Some(api_key.to_string()),
            },
        );

        Self::save_config_file(&config_file)
    }

    /// List all profiles
    pub fn list_profiles() -> Result<Vec<String>> {
        let config_file = Self::load_config_file()?;
        Ok(config_file.profiles.keys().cloned().collect())
    }

    /// Mask a key for display (show first 8 chars + asterisks)
    pub fn mask_key(key: &str) -> String {
        let char_count = key.chars().count();
        if char_count <= 8 {
            "*".repeat(char_count)
        } else {
            let prefix: String = key.chars().take(8).collect();
            format!("{prefix}********")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.api_key.is_none());
        assert_eq!(config.profile, "default");
        assert_eq!(config.format, OutputFormat::Table);
        assert!(!config.verbose);
    }

    #[test]
    fn test_config_is_valid_with_key() {
        let config = Config {
            api_key: Some("re_test_123".to_string()),
            ..Default::default()
        };
        assert!(config.is_valid());
    }

    #[test]
    fn test_config_is_invalid_without_key() {
        let config = Config::default();
        assert!(!config.is_valid());
    }

    #[test]
    fn test_mask_key_short() {
        assert_eq!(Config::mask_key("abc"), "***");
        assert_eq!(Config::mask_key("12345678"), "********");
    }

    #[test]
    fn test_mask_key_long() {
        assert_eq!(Config::mask_key("re_123456789"), "re_12345********");
    }

    #[test]
    fn test_config_file_default() {
        let config_file = ConfigFile::default();
        assert!(config_file.profiles.is_empty());
    }

    #[test]
    fn test_profile_serialize() {
        let profile = Profile {
            api_key: Some("re_test".to_string()),
        };
        let yaml = serde_yaml::to_string(&profile).unwrap();
        assert!(yaml.contains("api_key: re_test"));
    }
}
```

**Step 2: Add module to main.rs**

Add to `src/main.rs` after other mod declarations:

```rust
mod config;
```

**Step 3: Run tests**

Run: `cargo test config`
Expected: All tests pass

**Step 4: Commit**

```bash
git add src/config.rs src/main.rs
git commit -m "feat: add config module with profiles and env var support"
```

---

### Task 4: API Client

**Files:**
- Create: `src/client.rs`
- Modify: `src/main.rs`

**Step 1: Create client.rs with error types and client struct**

```rust
// ABOUTME: HTTP client for the Resend API.
// ABOUTME: Handles authentication, requests, and error mapping.

use anyhow::{Context, Result};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::types::*;

const BASE_URL: &str = "https://api.resend.com";

/// API errors
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication failed. Check your API key.")]
    AuthenticationError,

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Rate limit exceeded. Please try again later.")]
    RateLimitError,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Resend API client
#[derive(Debug)]
pub struct ResendClient {
    client: Client,
    api_key: String,
}

impl ResendClient {
    /// Create a new client with API key
    pub fn new(api_key: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
        })
    }

    /// Make an authenticated GET request
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ApiError::NetworkError("Request timeout".to_string())
                } else {
                    ApiError::NetworkError(e.to_string())
                }
            })?;

        self.handle_response(response).await
    }

    /// Make an authenticated POST request
    async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ApiError::NetworkError("Request timeout".to_string())
                } else {
                    ApiError::NetworkError(e.to_string())
                }
            })?;

        self.handle_response(response).await
    }

    /// Make an authenticated PATCH request
    async fn patch<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .patch(&url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ApiError::NetworkError("Request timeout".to_string())
                } else {
                    ApiError::NetworkError(e.to_string())
                }
            })?;

        self.handle_response(response).await
    }

    /// Make an authenticated DELETE request
    async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ApiError::NetworkError("Request timeout".to_string())
                } else {
                    ApiError::NetworkError(e.to_string())
                }
            })?;

        let status = response.status();

        match status {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ApiError::AuthenticationError.into())
            }
            StatusCode::NOT_FOUND => {
                let message = response.text().await.unwrap_or_default();
                Err(ApiError::NotFoundError(message).into())
            }
            StatusCode::TOO_MANY_REQUESTS => Err(ApiError::RateLimitError.into()),
            StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => {
                let message = response.text().await.unwrap_or_default();
                Err(ApiError::ValidationError(message).into())
            }
            _ => {
                let message = response.text().await.unwrap_or_default();
                Err(ApiError::ApiError {
                    status: status.as_u16(),
                    message,
                }
                .into())
            }
        }
    }

    /// Handle response and map errors
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        match status {
            StatusCode::OK | StatusCode::CREATED => {
                let body = response
                    .json::<T>()
                    .await
                    .context("Failed to parse response")?;
                Ok(body)
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(ApiError::AuthenticationError.into())
            }
            StatusCode::NOT_FOUND => {
                let message = response.text().await.unwrap_or_default();
                Err(ApiError::NotFoundError(message).into())
            }
            StatusCode::TOO_MANY_REQUESTS => Err(ApiError::RateLimitError.into()),
            StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => {
                let message = response.text().await.unwrap_or_default();
                Err(ApiError::ValidationError(message).into())
            }
            _ => {
                let message = response.text().await.unwrap_or_default();
                Err(ApiError::ApiError {
                    status: status.as_u16(),
                    message,
                }
                .into())
            }
        }
    }

    // ========== Emails API ==========

    /// Send an email
    pub async fn send_email(&self, req: SendEmailRequest) -> Result<SendEmailResponse> {
        self.post("/emails", &req).await
    }

    /// Get an email by ID
    pub async fn get_email(&self, id: &str) -> Result<Email> {
        self.get(&format!("/emails/{}", id)).await
    }

    /// List emails
    pub async fn list_emails(&self) -> Result<Vec<Email>> {
        let response: EmailsResponse = self.get("/emails").await?;
        Ok(response.data)
    }

    /// Cancel a scheduled email
    pub async fn cancel_email(&self, id: &str) -> Result<Email> {
        self.post(&format!("/emails/{}/cancel", id), &serde_json::json!({}))
            .await
    }

    /// Update a scheduled email
    pub async fn update_email(&self, id: &str, req: UpdateEmailRequest) -> Result<Email> {
        self.patch(&format!("/emails/{}", id), &req).await
    }

    // ========== Domains API ==========

    /// Create a domain
    pub async fn create_domain(&self, req: CreateDomainRequest) -> Result<Domain> {
        self.post("/domains", &req).await
    }

    /// List domains
    pub async fn list_domains(&self) -> Result<Vec<Domain>> {
        let response: DomainsResponse = self.get("/domains").await?;
        Ok(response.data)
    }

    /// Get a domain by ID
    pub async fn get_domain(&self, id: &str) -> Result<Domain> {
        self.get(&format!("/domains/{}", id)).await
    }

    /// Verify a domain
    pub async fn verify_domain(&self, id: &str) -> Result<Domain> {
        self.post(&format!("/domains/{}/verify", id), &serde_json::json!({}))
            .await
    }

    /// Update a domain
    pub async fn update_domain(&self, id: &str, req: UpdateDomainRequest) -> Result<Domain> {
        self.patch(&format!("/domains/{}", id), &req).await
    }

    /// Delete a domain
    pub async fn delete_domain(&self, id: &str) -> Result<()> {
        self.delete(&format!("/domains/{}", id)).await
    }

    // ========== API Keys API ==========

    /// Create an API key
    pub async fn create_api_key(&self, req: CreateApiKeyRequest) -> Result<ApiKey> {
        self.post("/api-keys", &req).await
    }

    /// List API keys
    pub async fn list_api_keys(&self) -> Result<Vec<ApiKey>> {
        let response: ApiKeysResponse = self.get("/api-keys").await?;
        Ok(response.data)
    }

    /// Delete an API key
    pub async fn delete_api_key(&self, id: &str) -> Result<()> {
        self.delete(&format!("/api-keys/{}", id)).await
    }

    // ========== Templates API ==========

    /// Create a template
    pub async fn create_template(&self, req: CreateTemplateRequest) -> Result<Template> {
        self.post("/templates", &req).await
    }

    /// List templates
    pub async fn list_templates(&self) -> Result<Vec<Template>> {
        let response: TemplatesResponse = self.get("/templates").await?;
        Ok(response.data)
    }

    /// Get a template by ID
    pub async fn get_template(&self, id: &str) -> Result<Template> {
        self.get(&format!("/templates/{}", id)).await
    }

    /// Update a template
    pub async fn update_template(&self, id: &str, req: UpdateTemplateRequest) -> Result<Template> {
        self.patch(&format!("/templates/{}", id), &req).await
    }

    /// Delete a template
    pub async fn delete_template(&self, id: &str) -> Result<()> {
        self.delete(&format!("/templates/{}", id)).await
    }

    // ========== Connection Test ==========

    /// Test API connection
    pub async fn test_connection(&self) -> Result<bool> {
        let result: Result<DomainsResponse> = self.get("/domains").await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{bearer_token, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(base_url: &str) -> ResendClient {
        ResendClient {
            client: Client::new(),
            api_key: "re_test_key".to_string(),
        }
    }

    #[test]
    fn test_api_error_display() {
        let auth_err = ApiError::AuthenticationError;
        assert!(auth_err.to_string().contains("Authentication failed"));

        let not_found = ApiError::NotFoundError("email-123".to_string());
        assert!(not_found.to_string().contains("email-123"));

        let rate_limit = ApiError::RateLimitError;
        assert!(rate_limit.to_string().contains("Rate limit"));
    }

    #[tokio::test]
    async fn test_list_domains_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/domains"))
            .and(bearer_token("re_test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {"id": "domain-1", "name": "example.com", "status": "verified", "region": "us-east-1"}
                ]
            })))
            .mount(&mock_server)
            .await;

        // Note: This test won't work directly because BASE_URL is hardcoded
        // In real tests, we'd need to make BASE_URL configurable
    }

    #[tokio::test]
    async fn test_authentication_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/domains"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        // Note: This test won't work directly because BASE_URL is hardcoded
    }
}
```

**Step 2: Add module to main.rs**

Add to `src/main.rs`:

```rust
mod client;
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/client.rs src/main.rs
git commit -m "feat: add API client with email, domain, api-key, and template methods"
```

---

### Task 5: Formatters Module

**Files:**
- Create: `src/formatters/mod.rs`
- Create: `src/formatters/table.rs`
- Create: `src/formatters/json.rs`
- Modify: `src/main.rs`

**Step 1: Create formatters/mod.rs**

```rust
// ABOUTME: Output formatting for CLI results.
// ABOUTME: Supports table and JSON output formats.

pub mod json;
pub mod table;

use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::io::{self, Write};

use crate::types::{OutputFormat, Tabular};

/// Format and output data based on format setting
pub fn format_and_output<T: Serialize + Tabular>(
    data: &[T],
    format: OutputFormat,
    output_path: Option<&str>,
) -> Result<()> {
    let formatted = match format {
        OutputFormat::Table => table::format_table(data),
        OutputFormat::Json => json::format_json(data)?,
    };

    write_output(&formatted, output_path)
}

/// Format and output a single item
pub fn format_and_output_single<T: Serialize + Tabular>(
    data: &T,
    format: OutputFormat,
    output_path: Option<&str>,
) -> Result<()> {
    let formatted = match format {
        OutputFormat::Table => table::format_single(data),
        OutputFormat::Json => json::format_json_single(data)?,
    };

    write_output(&formatted, output_path)
}

/// Write output to file or stdout
fn write_output(content: &str, output_path: Option<&str>) -> Result<()> {
    match output_path {
        Some(path) => {
            fs::write(path, content)?;
            Ok(())
        }
        None => {
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "{}", content)?;
            Ok(())
        }
    }
}

/// Output an error to stderr as JSON
pub fn output_error(error: &str, message: &str) {
    let error_json = serde_json::json!({
        "error": error,
        "message": message
    });
    eprintln!("{}", serde_json::to_string(&error_json).unwrap_or_default());
}
```

**Step 2: Create formatters/table.rs**

```rust
// ABOUTME: Table formatting using the tabled crate.
// ABOUTME: Renders data as human-readable tables.

use tabled::{Table, Tabled};

use crate::types::Tabular;

/// Wrapper for tabled rendering
#[derive(Tabled)]
struct TableRow {
    #[tabled(inline)]
    values: Vec<String>,
}

/// Format a list of items as a table
pub fn format_table<T: Tabular>(items: &[T]) -> String {
    if items.is_empty() {
        return "No results found.".to_string();
    }

    let headers = T::headers();
    let mut rows: Vec<Vec<String>> = vec![headers.iter().map(|s| s.to_string()).collect()];

    for item in items {
        rows.push(item.row());
    }

    format_rows(&rows)
}

/// Format a single item as key-value pairs
pub fn format_single<T: Tabular>(item: &T) -> String {
    let headers = T::headers();
    let values = item.row();

    let mut output = String::new();
    for (header, value) in headers.iter().zip(values.iter()) {
        output.push_str(&format!("{}: {}\n", header, value));
    }
    output
}

/// Format rows into a table string
fn format_rows(rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }

    // Calculate column widths
    let num_cols = rows[0].len();
    let mut widths: Vec<usize> = vec![0; num_cols];

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }

    // Build output
    let mut output = String::new();

    for (row_idx, row) in rows.iter().enumerate() {
        let line: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                if i < widths.len() {
                    format!("{:<width$}", cell, width = widths[i])
                } else {
                    cell.clone()
                }
            })
            .collect();

        output.push_str(&line.join("  "));
        output.push('\n');

        // Add separator after header
        if row_idx == 0 {
            let separator: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
            output.push_str(&separator.join("  "));
            output.push('\n');
        }
    }

    output.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Email;

    #[test]
    fn test_format_table_empty() {
        let emails: Vec<Email> = vec![];
        let output = format_table(&emails);
        assert_eq!(output, "No results found.");
    }

    #[test]
    fn test_format_table_with_items() {
        let emails = vec![Email {
            id: "email-123".to_string(),
            from: Some("from@example.com".to_string()),
            to: Some(vec!["to@example.com".to_string()]),
            subject: Some("Test Subject".to_string()),
            created_at: Some("2025-01-15".to_string()),
            last_event: Some("delivered".to_string()),
        }];
        let output = format_table(&emails);
        assert!(output.contains("email-123"));
        assert!(output.contains("Test Subject"));
        assert!(output.contains("delivered"));
    }
}
```

**Step 3: Create formatters/json.rs**

```rust
// ABOUTME: JSON formatting for structured output.
// ABOUTME: Used when --json flag is provided.

use anyhow::Result;
use serde::Serialize;

/// Format a list of items as JSON
pub fn format_json<T: Serialize>(items: &[T]) -> Result<String> {
    Ok(serde_json::to_string_pretty(items)?)
}

/// Format a single item as JSON
pub fn format_json_single<T: Serialize>(item: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(item)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Email;

    #[test]
    fn test_format_json_empty() {
        let emails: Vec<Email> = vec![];
        let output = format_json(&emails).unwrap();
        assert_eq!(output, "[]");
    }

    #[test]
    fn test_format_json_with_items() {
        let emails = vec![Email {
            id: "email-123".to_string(),
            from: Some("from@example.com".to_string()),
            to: Some(vec!["to@example.com".to_string()]),
            subject: Some("Test".to_string()),
            created_at: None,
            last_event: None,
        }];
        let output = format_json(&emails).unwrap();
        assert!(output.contains("email-123"));
        assert!(output.contains("from@example.com"));
    }
}
```

**Step 4: Add module to main.rs**

Add to `src/main.rs`:

```rust
mod formatters;
```

**Step 5: Run tests**

Run: `cargo test formatters`
Expected: All tests pass

**Step 6: Commit**

```bash
git add src/formatters/ src/main.rs
git commit -m "feat: add formatters module with table and JSON output"
```

---

### Task 6: Commands Module Base

**Files:**
- Create: `src/commands/mod.rs`
- Modify: `src/main.rs`

**Step 1: Create commands/mod.rs with helper functions**

```rust
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
```

**Step 2: Add module to main.rs**

Add to `src/main.rs`:

```rust
mod commands;
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles (will have warnings about unused modules until we add subcommands)

**Step 4: Commit**

```bash
git add src/commands/mod.rs src/main.rs
git commit -m "feat: add commands module base with common args and helpers"
```

---

### Task 7: Config Commands

**Files:**
- Create: `src/commands/config.rs`
- Modify: `src/main.rs` (integrate commands)

**Step 1: Create commands/config.rs**

```rust
// ABOUTME: Configuration management commands.
// ABOUTME: Handles setup, show, and list operations for profiles.

use anyhow::Result;
use clap::Subcommand;
use dialoguer::{Input, Password};

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
```

**Step 2: Update main.rs with CLI structure**

Replace `src/main.rs` entirely:

```rust
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
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Will fail until we create the other command files (emails, domains, api_keys, templates)

---

### Task 8: Emails Commands

**Files:**
- Create: `src/commands/emails.rs`

**Step 1: Create commands/emails.rs**

```rust
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
```

**Step 2: Verify it compiles (still need other commands)**

Run: `cargo build`
Expected: Will still fail until we create domains, api_keys, templates

---

### Task 9: Domains Commands

**Files:**
- Create: `src/commands/domains.rs`

**Step 1: Create commands/domains.rs**

```rust
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
```

---

### Task 10: API Keys Commands

**Files:**
- Create: `src/commands/api_keys.rs`

**Step 1: Create commands/api_keys.rs**

```rust
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
```

---

### Task 11: Templates Commands

**Files:**
- Create: `src/commands/templates.rs`

**Step 1: Create commands/templates.rs**

```rust
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
```

---

### Task 12: Final Integration and Testing

**Step 1: Build the project**

Run: `cargo build`
Expected: Compiles successfully

**Step 2: Run all tests**

Run: `cargo test`
Expected: All tests pass

**Step 3: Test CLI help**

Run: `cargo run -- --help`
Expected: Shows all subcommands (config, emails, domains, api-keys, templates)

Run: `cargo run -- emails --help`
Expected: Shows emails subcommands (send, get, list, cancel, update)

**Step 4: Test config setup (interactive)**

Run: `cargo run -- config setup`
Expected: Prompts for API key, tests connection, saves config

**Step 5: Commit all changes**

```bash
git add -A
git commit -m "feat: complete CLI with emails, domains, api-keys, and templates commands"
```

---

### Task 13: Release Build

**Step 1: Build release binary**

Run: `cargo build --release`
Expected: Creates optimized binary at `target/release/resend`

**Step 2: Test release binary**

Run: `./target/release/resend --version`
Expected: Shows version 0.1.0

**Step 3: Install locally (optional)**

Run: `cargo install --path .`
Expected: Installs `resend` to `~/.cargo/bin/`

**Step 4: Final commit**

```bash
git add -A
git commit -m "chore: verify release build"
```
