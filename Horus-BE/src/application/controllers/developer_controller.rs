use crate::application::controllers::base::{DateRangeQuery, MetricsController, MetricsQuery};
use crate::application::AppState;
use crate::common::errors::AppError;
use crate::domain::developer_metrics::{
    adapters::GitHubDeveloperAdapter,
    services::{DeveloperAnalytics, DeveloperMetricsAnalyzer},
};
use actix_web::{get, web, HttpResponse, Responder};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct DeveloperMetricsQuery {
    #[serde(flatten)]
    #[schema(example = "2024-01-01T00:00:00Z")]
    date_range: DateRangeQuery,
    #[schema(example = "johndoe")]
    username: String,
}

impl MetricsQuery for DeveloperMetricsQuery {
    fn get_date_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        self.date_range.get_date_range()
    }

    fn get_identifier(&self) -> &str {
        &self.username
    }
}

pub struct DeveloperController;

#[async_trait]
impl MetricsController<DeveloperMetricsAnalyzer<GitHubDeveloperAdapter>, DeveloperMetricsQuery>
    for DeveloperController
{
    fn create_service(
        github_sdk: crate::infrastructure::github_sdk::client::GitHubSdk,
    ) -> DeveloperMetricsAnalyzer<GitHubDeveloperAdapter> {
        let github_adapter = GitHubDeveloperAdapter::new(github_sdk);
        DeveloperMetricsAnalyzer::new(github_adapter)
    }

    async fn handle_metrics_request(
        service: DeveloperMetricsAnalyzer<GitHubDeveloperAdapter>,
        query: web::Query<DeveloperMetricsQuery>,
        date_range: (DateTime<Utc>, DateTime<Utc>),
    ) -> Result<HttpResponse, AppError> {
        let metrics = service
            .get_developer_metrics(&query.0.username, date_range.0, date_range.1)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(HttpResponse::Ok().json(metrics))
    }
}

/// Get metrics for a developer
#[utoipa::path(
    get,
    path = "/api/v1/developer/metrics",
    params(
        ("username" = String, Query, description = "GitHub username of the developer"),
        ("start_date" = String, Query, description = "Start date in ISO 8601 format"),
        ("end_date" = String, Query, description = "End date in ISO 8601 format")
    ),
    responses(
        (status = 200, description = "Successfully retrieved developer metrics", body = Metrics),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Developer Metrics"
)]
#[get("/metrics")]
pub async fn get_developer_metrics(
    data: web::Data<AppState>,
    query: web::Query<DeveloperMetricsQuery>,
) -> impl Responder {
    DeveloperController::process_request(data, query).await
}

/// Get commit statistics for a developer
#[utoipa::path(
    get,
    path = "/api/v1/developer/commits",
    params(
        ("username" = String, Query, description = "GitHub username of the developer"),
        ("start_date" = String, Query, description = "Start date in ISO 8601 format"),
        ("end_date" = String, Query, description = "End date in ISO 8601 format")
    ),
    responses(
        (status = 200, description = "Successfully retrieved commit statistics", body = Commit),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Developer Metrics"
)]
#[get("/commits")]
pub async fn get_developer_commits(
    data: web::Data<AppState>,
    query: web::Query<DeveloperMetricsQuery>,
) -> impl Responder {
    let service = DeveloperController::create_service(data.github.clone());
    let (start_date, end_date) = query.0.get_date_range();

    match service
        .get_commits(&query.0.username, start_date, end_date)
        .await
    {
        Ok(commits) => HttpResponse::Ok().json(commits),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get pull request statistics for a developer
#[utoipa::path(
    get,
    path = "/api/v1/developer/pull-requests",
    params(
        ("username" = String, Query, description = "GitHub username of the developer"),
        ("start_date" = String, Query, description = "Start date in ISO 8601 format"),
        ("end_date" = String, Query, description = "End date in ISO 8601 format")
    ),
    responses(
        (status = 200, description = "Successfully retrieved pull request statistics", body = PullRequest),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Developer Metrics"
)]
#[get("/pull-requests")]
pub async fn get_developer_pull_requests(
    data: web::Data<AppState>,
    query: web::Query<DeveloperMetricsQuery>,
) -> impl Responder {
    let service = DeveloperController::create_service(data.github.clone());
    let (start_date, end_date) = query.0.get_date_range();
    println!("start_date: {:?}", start_date);
    println!("end_date: {:?}", end_date);
    match service
        .get_pull_requests(&query.0.username, start_date, end_date)
        .await
    {
        Ok(pull_requests) => HttpResponse::Ok().json(pull_requests),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
