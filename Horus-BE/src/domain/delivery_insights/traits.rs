use crate::common::errors::AppError;
use crate::infrastructure::github_sdk::models::{CustomDeployment, CustomDeploymentStatus};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::{
    entities::{Deployment, Incident, Release},
    value_objects::DeliveryMetrics,
};

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait GitHubDeliveryProvider: Send + Sync {
    async fn list_deployments(
        &self,
        repository: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<CustomDeployment>, AppError>;

    async fn list_releases(
        &self,
        repository: &str,
        since: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Result<Vec<octocrab::models::repos::Release>, AppError>;

    async fn list_deployment_statuses(
        &self,
        repository: &str,
        deployment_id: i64,
    ) -> Result<Vec<CustomDeploymentStatus>, AppError>;
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait DeliveryAnalytics {
    async fn get_delivery_metrics(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<DeliveryMetrics, AppError>;

    // async fn get_delivery_metrics_concurrent(
    //     &self,
    //     repository: &str,
    //     start_date: DateTime<Utc>,
    //     end_date: DateTime<Utc>,
    // ) -> Result<DeliveryMetrics>;

    async fn get_deployments(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Deployment>, AppError>;

    async fn get_releases(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Release>, AppError>;

    async fn get_incidents(
        &self,
        repository: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Incident>, AppError>;
}
