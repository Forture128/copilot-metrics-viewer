use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct CodeReview {
    pub id: String,
    pub pull_request_id: String,
    pub reviewer: String,
    pub author: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub comments: Vec<ReviewComment>,
    pub status: ReviewStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct ReviewComment {
    pub id: String,
    pub body: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub line_number: Option<i32>,
    pub file_path: Option<String>,
    pub comment_type: CommentType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub enum ReviewStatus {
    Pending,
    Approved,
    ChangesRequested,
    Commented,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub enum CommentType {
    General,
    CodeSuggestion,
    Question,
    BugReport,
    Praise,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct TeamInteraction {
    pub id: String,
    pub team_id: String,
    pub interaction_type: InteractionType,
    pub participants: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub enum InteractionType {
    CodeReview,
    PairProgramming,
    Discussion,
    CodeCollaboration,
}
