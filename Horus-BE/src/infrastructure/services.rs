use crate::config::app_config::GitHubConfig;
use crate::infrastructure::github_sdk::GitHubSdk;
use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppServices {
    pub github: Arc<GitHubSdk>,
}

impl AppServices {
    pub fn new(github_config: &GitHubConfig) -> Result<Self> {
        let github = GitHubSdk::builder()
            .with_token(github_config.token.clone().unwrap_or_default())
            .with_timeout(std::time::Duration::from_secs(github_config.timeout_secs));

        // Add base URL if configured
        let github = if let Some(url) = &github_config.base_url {
            github.with_base_url(url)
        } else {
            github
        };

        let github = Arc::new(github.build()?);

        Ok(Self { github })
    }
}
