use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::developer_metrics::{Commit, Metrics, PullRequest};

/// External service interface for fetching GitHub data
#[async_trait]
pub trait GitHubDataProvider: Send + Sync {
    async fn fetch_user_commits(
        &self,
        username: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<Commit>>;

    async fn fetch_user_pull_requests(
        &self,
        username: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<PullRequest>>;
}

/// Core domain service for developer metrics analysis
#[async_trait]
pub trait DeveloperAnalytics {
    /// Get comprehensive metrics for a developer
    async fn get_developer_metrics(
        &self,
        username: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Metrics>;

    /// Get commit history for a developer
    async fn get_commits(
        &self,
        username: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Commit>>;

    /// Get pull request history for a developer
    async fn get_pull_requests(
        &self,
        username: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<PullRequest>>;
}
