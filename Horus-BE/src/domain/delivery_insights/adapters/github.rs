use crate::common::errors::AppError;
use crate::{
    domain::delivery_insights::traits::GitHubDeliveryProvider,
    infrastructure::github_sdk::{
        client::GitHubSdk,
        models::{CustomDeployment, CustomDeploymentStatus},
    },
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
pub struct GitHubDeliveryAdapter {
    github_sdk: GitHubSdk,
}

impl GitHubDeliveryAdapter {
    pub fn new(github_sdk: GitHubSdk) -> Self {
        Self { github_sdk }
    }
}

#[async_trait]
impl GitHubDeliveryProvider for GitHubDeliveryAdapter {
    async fn list_deployments(
        &self,
        repository: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<CustomDeployment>, AppError> {
        self.github_sdk
            .fetch_deployments("moneyforward", repository, since, until)
            .await
    }

    async fn list_releases(
        &self,
        repository: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<octocrab::models::repos::Release>, AppError> {
        self.github_sdk
            .fetch_releases("moneyforward", repository, since, until)
            .await
    }

    async fn list_deployment_statuses(
        &self,
        repository: &str,
        deployment_id: i64,
    ) -> Result<Vec<CustomDeploymentStatus>, AppError> {
        self.github_sdk
            .fetch_deployment_statuses("moneyforward", repository, deployment_id)
            .await
    }
}
