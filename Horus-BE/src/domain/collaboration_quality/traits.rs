use crate::domain::collaboration_quality::{
    entities::{CodeReview, TeamInteraction},
    value_objects::{CollaborationMetrics, ReviewQualityMetrics, TeamCollaborationMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// External service interface for fetching GitHub collaboration data
#[async_trait]
pub trait GitHubCollaborationProvider: Send + Sync {
    async fn fetch_code_reviews(
        &self,
        team_id: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<CodeReview>>;

    async fn fetch_team_interactions(
        &self,
        team_id: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<TeamInteraction>>;
}

/// Core domain service for collaboration analytics
#[async_trait]
pub trait CollaborationAnalytics {
    /// Get comprehensive collaboration metrics for a team
    async fn get_collaboration_metrics(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<CollaborationMetrics>;

    /// Get code review history
    async fn get_code_reviews(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<CodeReview>>;

    /// Get team interaction history
    async fn get_team_interactions(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<TeamInteraction>>;

    /// Get review quality metrics
    #[allow(dead_code)]
    async fn get_review_quality_metrics(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<ReviewQualityMetrics>;

    /// Get team collaboration metrics
    #[allow(dead_code)]
    async fn get_team_collaboration_metrics(
        &self,
        team_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<TeamCollaborationMetrics>;
}
