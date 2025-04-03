use crate::common::errors::AppError;
use anyhow::{Context, Result};
use octocrab::OctocrabBuilder;
use std::time::Duration;

use super::client::GitHubSdk;

/// Configuration for GitHub SDK
#[derive(Clone, Debug)]
pub struct GitHubConfig {
    pub token: Option<String>,
    pub base_url: Option<String>,
    pub timeout: Option<Duration>,
}

impl GitHubConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            token: std::env::var("GITHUB_TOKEN").ok(),
            base_url: std::env::var("GITHUB_API_URL")
                .ok()
                .or(Some("https://api.github.com".to_string())),
            timeout: std::env::var("GITHUB_TIMEOUT_SECS")
                .ok()
                .map(|s| Duration::from_secs(s.parse::<u64>().unwrap())),
        })
    }
}

/// A Builder struct to configure and construct a GitHubSdk instance.
pub struct GitHubSdkBuilder {
    config: GitHubConfig,
}

impl GitHubSdkBuilder {
    pub fn new() -> Self {
        Self {
            config: GitHubConfig::from_env().unwrap(),
        }
    }

    /// Sets the Github Personal Access Token
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.config.token = Some(token.into());
        self
    }

    /// Sets the base URL for GitHub Enterprise
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = Some(url.into());
        self
    }

    /// Sets the timeout for requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = Some(timeout);
        self
    }

    /// Gets the current configuration
    #[allow(dead_code)] // Only Test use this function
    pub fn get_config(&self) -> &GitHubConfig {
        &self.config
    }

    /// Builds the GitHubSdk instance
    pub fn build(self) -> Result<GitHubSdk> {
        let mut builder = OctocrabBuilder::new();

        // Configure token if provided
        if let Some(token) = &self.config.token {
            builder = builder.personal_token(token.clone());
        }

        // Configure custom URL if provided
        if let Some(url) = &self.config.base_url {
            builder = builder.base_uri(url).context("Failed to set base URL")?;
        }

        // Build the client
        let client = builder.build().context("Failed to build GitHub client")?;

        Ok(GitHubSdk::new(client, self.config))
    }
}
