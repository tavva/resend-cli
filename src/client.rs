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

    #[test]
    fn test_api_error_display() {
        let auth_err = ApiError::AuthenticationError;
        assert!(auth_err.to_string().contains("Authentication failed"));

        let not_found = ApiError::NotFoundError("email-123".to_string());
        assert!(not_found.to_string().contains("email-123"));

        let rate_limit = ApiError::RateLimitError;
        assert!(rate_limit.to_string().contains("Rate limit"));
    }
}
