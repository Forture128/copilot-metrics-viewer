use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Deployment {
    pub id: String,
    pub repository: String,
    pub environment: String,
    pub version: String,
    pub status: DeploymentStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deployed_by: String,
    pub changes: Vec<DeploymentChange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DeploymentChange {
    pub commit_id: String,
    pub pull_request_id: Option<String>,
    pub author: String,
    pub message: String,
    pub files_changed: usize,
    pub change_type: ChangeType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub enum DeploymentStatus {
    InProgress,
    Success,
    Failed,
    Rolled,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum ChangeType {
    Feature,
    Bugfix,
    Hotfix,
    Refactor,
    Infrastructure,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Release {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub author: String,
    pub deployments: Vec<Deployment>,
    pub changes: Vec<DeploymentChange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Incident {
    pub id: String,
    pub title: String,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub started_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub related_deployment: Option<String>,
    pub affected_services: Vec<String>,
    pub resolution_time: Option<chrono::Duration>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub enum IncidentSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub enum IncidentStatus {
    Active,
    Investigating,
    Resolved,
    Monitoring,
}
