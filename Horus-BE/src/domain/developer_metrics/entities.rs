use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Developer {
    pub id: String,
    pub username: String,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Commit {
    pub id: String,
    pub author: Developer,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub additions: usize,
    pub deletions: usize,
    pub files_changed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PullRequest {
    pub id: String,
    pub number: i32,
    pub title: String,
    pub author: Developer,
    pub created_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
    pub review_time: Option<chrono::Duration>,
    pub commits: Vec<Commit>,
    pub comments: usize,
    pub changed_files: usize,
    pub additions: usize,
    pub deletions: usize,
}
