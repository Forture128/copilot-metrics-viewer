use crate::application::controllers::base::{BaseMetricsQuery, MetricsController, MetricsQuery};
use crate::application::AppState;
use crate::common::errors::AppError;
use crate::domain::delivery_insights::{
    adapters::GitHubDeliveryAdapter,
    repositories::InMemoryDeliveryInsightsRepository,
    services::{DeliveryAnalytics, DeliveryInsightsAnalyzer},
};
use actix_web::{get, web, HttpResponse};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::{debug, info};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct DeliveryMetricsQuery {
    #[serde(flatten)]
    pub base: BaseMetricsQuery,
}

impl MetricsQuery for DeliveryMetricsQuery {
    fn get_date_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        self.base.get_date_range()
    }

    fn get_identifier(&self) -> &str {
        self.base.get_identifier()
    }
}

pub struct DeliveryController;

#[async_trait]
impl
    MetricsController<
        DeliveryInsightsAnalyzer<GitHubDeliveryAdapter, InMemoryDeliveryInsightsRepository>,
        DeliveryMetricsQuery,
    > for DeliveryController
{
    fn create_service(
        github_sdk: crate::infrastructure::github_sdk::client::GitHubSdk,
    ) -> DeliveryInsightsAnalyzer<GitHubDeliveryAdapter, InMemoryDeliveryInsightsRepository> {
        let github_adapter = GitHubDeliveryAdapter::new(github_sdk);
        let repository = InMemoryDeliveryInsightsRepository;
        DeliveryInsightsAnalyzer::new(github_adapter, repository)
    }

    async fn handle_metrics_request(
        service: DeliveryInsightsAnalyzer<
            GitHubDeliveryAdapter,
            InMemoryDeliveryInsightsRepository,
        >,
        query: web::Query<DeliveryMetricsQuery>,
        date_range: (DateTime<Utc>, DateTime<Utc>),
    ) -> Result<HttpResponse, AppError> {
        info!(
            "Fetching delivery metrics for repository: {} from {} to {}",
            query.base.identifier, date_range.0, date_range.1
        );

        let metrics = service
            .get_delivery_metrics(&query.base.identifier, date_range.0, date_range.1)
            .await?;

        debug!("Successfully retrieved metrics: {:?}", metrics);
        Ok(HttpResponse::Ok().json(metrics))
    }
}

/// Get delivery metrics for a repository
#[utoipa::path(
    get,
    path = "/api/v1/delivery/metrics",
    params(
        ("identifier" = String, Query, description = "Repository name"),
        ("start_date" = Option<String>, Query, description = "Start date in ISO 8601 format"),
        ("end_date" = Option<String>, Query, description = "End date in ISO 8601 format"),
        ("period" = Option<String>, Query, description = "Period (daily, weekly, monthly, yearly)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved delivery metrics", body = DeliveryMetrics),
        (status = 400, description = "Invalid request parameters"),
        (status = 502, description = "GitHub API error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Delivery Metrics"
)]
#[get("/metrics")]
pub async fn get_delivery_metrics(
    data: web::Data<AppState>,
    query: web::Query<DeliveryMetricsQuery>,
) -> Result<HttpResponse, AppError> {
    info!(
        "Processing delivery metrics request for repository: {}",
        query.base.identifier
    );

    DeliveryController::process_request(data, query).await
}

// #[get("/delivery/dora-metrics")]
// pub async fn get_dora_metrics(
//     data: web::Data<AppState>,
//     query: web::Query<DeliveryMetricsQuery>,
// ) -> impl Responder {
//     todo!()
// }

// #[get("/delivery/lead-time")]
// pub async fn get_lead_time(
//     data: web::Data<AppState>,
//     query: web::Query<DeliveryMetricsQuery>,
// ) -> impl Responder {
//     // TODO: Implement this
//     todo!()
// }
