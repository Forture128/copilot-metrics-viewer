use horus_be::infrastructure::github_sdk::{builder::GitHubSdkBuilder, client::GitHubSdk};
use std::time::Duration;
#[test]
fn test_builder_default() {
    let builder = GitHubSdkBuilder::new();
    let config = builder.get_config();
    assert!(config.token.is_none());
    assert!(config.base_url.is_some());
    assert!(config.timeout.is_none());
}

#[test]
fn test_builder_with_token() {
    let builder = GitHubSdkBuilder::new().with_token("test-token");
    assert_eq!(builder.get_config().token, Some("test-token".to_string()));
}

#[test]
fn test_builder_with_base_url() {
    let builder = GitHubSdkBuilder::new().with_base_url("https://api.github.enterprise.com");
    assert_eq!(
        builder.get_config().base_url,
        Some("https://api.github.enterprise.com".to_string())
    );
}

#[test]
fn test_builder_with_timeout() {
    let timeout = Duration::from_secs(60);
    let builder = GitHubSdkBuilder::new().with_timeout(timeout);
    assert_eq!(builder.get_config().timeout, Some(timeout));
}

#[test]
fn test_builder_chain() {
    let builder = GitHubSdkBuilder::new()
        .with_token("test-token")
        .with_base_url("https://api.github.enterprise.com")
        .with_timeout(Duration::from_secs(60));

    let config = builder.get_config();
    assert_eq!(config.token, Some("test-token".to_string()));
    assert_eq!(
        config.base_url,
        Some("https://api.github.enterprise.com".to_string())
    );
    assert_eq!(config.timeout, Some(Duration::from_secs(60)));
}

#[tokio::test]
async fn test_github_sdk_env() {
    std::env::set_var("GITHUB_TOKEN", "test-token");
    std::env::set_var("GITHUB_API_URL", "https://api.github.enterprise.com");
    std::env::set_var("GITHUB_TIMEOUT_SECS", "60");

    let sdk = GitHubSdk::from_env().expect("Failed to create SDK from env");

    assert_eq!(sdk.config.token, Some("test-token".to_string()));
    assert_eq!(
        sdk.config.base_url,
        Some("https://api.github.enterprise.com".to_string())
    );
    assert_eq!(sdk.config.timeout, Some(Duration::from_secs(60)));
}
