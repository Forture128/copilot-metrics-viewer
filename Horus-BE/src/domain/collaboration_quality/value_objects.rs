use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewQualityMetrics {
    pub average_review_time: Duration,
    pub comments_per_review: f64,
    pub substantive_comments_ratio: f64,
    pub review_coverage: f64,    // Percentage of PRs that received reviews
    pub average_iterations: f64, // Average number of review iterations before approval
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamCollaborationMetrics {
    pub cross_team_reviews: usize,
    pub knowledge_sharing_index: f64, // Based on review patterns and comment quality
    pub review_participation_rate: f64, // Percentage of team members actively reviewing
    pub average_response_time: Duration,
    pub collaboration_network_density: f64, // Measure of team interconnectedness
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CollaborationMetrics {
    pub team_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub review_quality: ReviewQualityMetrics,
    pub team_collaboration: TeamCollaborationMetrics,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewPattern {
    pub reviewer: String,
    pub review_count: usize,
    pub average_comments: f64,
    pub typical_files_reviewed: Vec<String>,
    pub expertise_areas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CollaborationNetwork {
    pub nodes: Vec<String>, // Team members
    pub edges: Vec<CollaborationEdge>,
    pub density: f64,
    pub centrality: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CollaborationEdge {
    pub from: String,
    pub to: String,
    pub weight: f64, // Strength of collaboration
    pub interaction_types: Vec<String>,
}
