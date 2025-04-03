use crate::common::errors::AppError;
use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use chrono::{format::ParseErrorKind, DateTime, Duration, ParseError, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum PeriodQuery {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl PeriodQuery {
    pub fn get_duration(&self) -> Duration {
        match self {
            PeriodQuery::Daily => Duration::days(1),
            PeriodQuery::Weekly => Duration::weeks(1),
            PeriodQuery::Monthly => Duration::days(30),
            PeriodQuery::Yearly => Duration::days(365),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct DateRangeQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl DateRangeQuery {
    pub fn get_date_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let end_date = match &self.end_date {
            Some(date_str) => parse_date_string(date_str).unwrap_or_else(|_| Utc::now()),
            None => Utc::now(),
        };

        let start_date = match &self.start_date {
            Some(date_str) => {
                parse_date_string(date_str).unwrap_or_else(|_| end_date - Duration::days(30))
            }
            None => end_date - Duration::days(30),
        };

        (start_date, end_date)
    }
}

// Helper function to parse date strings in various formats
fn parse_date_string(date_str: &str) -> Result<DateTime<Utc>, String> {
    // Try parsing as RFC 3339 format (e.g., "2022-01-01T00:00:00Z")
    if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(date.with_timezone(&Utc));
    }

    // Try parsing as YYYY-MM-DD format
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(DateTime::<Utc>::from_naive_utc_and_offset(
            date.and_hms_opt(0, 0, 0).unwrap(),
            Utc,
        ));
    }

    // If all parsing attempts fail, return an error
    Err(format!("Unable to parse date: {}", date_str))
}

pub trait MetricsQuery {
    fn get_date_range(&self) -> (DateTime<Utc>, DateTime<Utc>);
    fn get_identifier(&self) -> &str;
}

#[derive(Deserialize, ToSchema)]
pub struct BaseMetricsQuery {
    #[serde(flatten)]
    pub date_range: Option<DateRangeQuery>,
    pub period: Option<PeriodQuery>,
    pub identifier: String,
}

impl MetricsQuery for BaseMetricsQuery {
    fn get_date_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let end_date = Utc::now();

        match (&self.period, &self.date_range) {
            (Some(period), _) => {
                // If period is specified, it takes precedence
                let duration = period.get_duration();
                (end_date - duration, end_date)
            }
            (None, Some(date_range)) => date_range.get_date_range(),
            (None, None) => {
                // Default to last 30 days if neither is specified
                (end_date - Duration::days(30), end_date)
            }
        }
    }

    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[async_trait]
pub trait MetricsController<S, Q>
where
    S: Send + Sync,
    Q: MetricsQuery + Send + Sync + 'static,
{
    fn create_service(github_sdk: crate::infrastructure::github_sdk::client::GitHubSdk) -> S;

    async fn handle_metrics_request(
        service: S,
        query: web::Query<Q>,
        date_range: (DateTime<Utc>, DateTime<Utc>),
    ) -> Result<HttpResponse, AppError>;

    async fn process_request(
        data: web::Data<crate::application::AppState>,
        query: web::Query<Q>,
    ) -> Result<HttpResponse, AppError> {
        let service = Self::create_service(data.github.clone());
        let date_range = query.get_date_range();
        Self::handle_metrics_request(service, query, date_range).await
    }
}
