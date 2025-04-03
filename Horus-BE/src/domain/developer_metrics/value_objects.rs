use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CommitFrequency {
    pub total_commits: usize,
    pub commits_per_day: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReviewMetrics {
    pub total_reviews: usize,
    pub average_review_time: Duration,
    pub reviews_per_week: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CodeChangeMetrics {
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub files_changed: usize,
    pub changes_per_commit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Metrics {
    pub developer_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub commit_frequency: CommitFrequency,
    pub review_metrics: ReviewMetrics,
    pub code_changes: CodeChangeMetrics,
}
