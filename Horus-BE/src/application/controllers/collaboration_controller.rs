use crate::application::controllers::base::{DateRangeQuery, MetricsController, MetricsQuery};
use crate::application::AppState;
use crate::common::errors::AppError;
use crate::domain::collaboration_quality::{
    adapters::GitHubCollaborationAdapter,
    repositories::InMemoryCollaborationRepository,
    services::{CollaborationAnalytics, CollaborationQualityAnalyzer},
};
use actix_web::{get, web, HttpResponse, Responder};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CollaborationMetricsQuery {
    #[serde(flatten)]
    #[schema(example = "2024-01-01T00:00:00Z")]
    date_range: DateRangeQuery,
    #[schema(example = "my-team")]
    team_name: String,
}

impl MetricsQuery for CollaborationMetricsQuery {
    fn get_date_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        self.date_range.get_date_range()
    }

    fn get_identifier(&self) -> &str {
        &self.team_name
    }
}

pub struct CollaborationController;

#[async_trait]
impl
    MetricsController<
        CollaborationQualityAnalyzer<GitHubCollaborationAdapter, InMemoryCollaborationRepository>,
        CollaborationMetricsQuery,
    > for CollaborationController
{
    fn create_service(
        github_sdk: crate::infrastructure::github_sdk::client::GitHubSdk,
    ) -> CollaborationQualityAnalyzer<GitHubCollaborationAdapter, InMemoryCollaborationRepository>
    {
        let github_adapter = GitHubCollaborationAdapter::new(github_sdk);
        let repository = InMemoryCollaborationRepository;
        CollaborationQualityAnalyzer::new(github_adapter, repository)
    }

    async fn handle_metrics_request(
        service: CollaborationQualityAnalyzer<
            GitHubCollaborationAdapter,
            InMemoryCollaborationRepository,
        >,
        query: web::Query<CollaborationMetricsQuery>,
        date_range: (DateTime<Utc>, DateTime<Utc>),
    ) -> Result<HttpResponse, AppError> {
        let metrics = service
            .get_collaboration_metrics(&query.0.team_name, date_range.0, date_range.1)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(HttpResponse::Ok().json(metrics))
    }
}

/// Get collaboration metrics for a team
#[utoipa::path(
    get,
    path = "/api/v1/collaboration/metrics",
    params(
        ("team_name" = String, Query, description = "Name of the team"),
        ("start_date" = String, Query, description = "Start date in ISO 8601 format"),
        ("end_date" = String, Query, description = "End date in ISO 8601 format")
    ),
    responses(
        (status = 200, description = "Successfully retrieved collaboration metrics", body = CollaborationMetrics),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Collaboration Metrics"
)]
#[get("/metrics")]
pub async fn get_collaboration_metrics(
    data: web::Data<AppState>,
    query: web::Query<CollaborationMetricsQuery>,
) -> impl Responder {
    CollaborationController::process_request(data, query).await
}

/// Get code review statistics for a team
#[utoipa::path(
    get,
    path = "/api/v1/collaboration/code-review-stats",
    params(
        ("team_name" = String, Query, description = "Name of the team"),
        ("start_date" = String, Query, description = "Start date in ISO 8601 format"),
        ("end_date" = String, Query, description = "End date in ISO 8601 format")
    ),
    responses(
        (status = 200, description = "Successfully retrieved code review statistics", body = CodeReview),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Collaboration Metrics"
)]
#[get("/code-review-stats")]
pub async fn get_code_review_stats(
    data: web::Data<AppState>,
    query: web::Query<CollaborationMetricsQuery>,
) -> impl Responder {
    let service = CollaborationController::create_service(data.github.clone());
    let (start_date, end_date) = query.0.get_date_range();

    match service
        .get_code_reviews(&query.0.team_name, start_date, end_date)
        .await
    {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get team interaction metrics
#[utoipa::path(
    get,
    path = "/api/v1/collaboration/team-interactions",
    params(
        ("team_name" = String, Query, description = "Name of the team"),
        ("start_date" = String, Query, description = "Start date in ISO 8601 format"),
        ("end_date" = String, Query, description = "End date in ISO 8601 format")
    ),
    responses(
        (status = 200, description = "Successfully retrieved team interactions", body = TeamInteraction),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Collaboration Metrics"
)]
#[get("/team-interactions")]
pub async fn get_team_interactions(
    data: web::Data<AppState>,
    query: web::Query<CollaborationMetricsQuery>,
) -> impl Responder {
    let service = CollaborationController::create_service(data.github.clone());
    let (start_date, end_date) = query.0.get_date_range();

    match service
        .get_team_interactions(&query.0.team_name, start_date, end_date)
        .await
    {
        Ok(interactions) => HttpResponse::Ok().json(interactions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
