use crate::domain::developer_metrics::{
    entities::{Developer, PullRequest},
    value_objects::Metrics,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait MetricsRepository {
    async fn save_metrics(&self, metrics: &Metrics) -> Result<()>;
    async fn get_metrics(
        &self,
        developer_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Option<Metrics>>;
    async fn get_team_metrics(
        &self,
        team_ids: &[String],
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Metrics>>;
}

#[async_trait]
pub trait DeveloperRepository {
    async fn save_developer(&self, developer: &Developer) -> Result<()>;
    async fn get_developer(&self, id: &str) -> Result<Option<Developer>>;
    async fn get_team_members(&self, team_id: &str) -> Result<Vec<Developer>>;
}

#[async_trait]
pub trait PullRequestRepository {
    async fn save_pull_request(&self, pr: &PullRequest) -> Result<()>;
    async fn get_pull_requests(
        &self,
        developer_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<PullRequest>>;
}
