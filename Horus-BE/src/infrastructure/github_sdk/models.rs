/// Custom models for GitHub SDK
use chrono::{DateTime, Utc};
use octocrab::models::Author;
use serde::{Deserialize, Serialize};

/// Represents a GitHub Deployment response
/// Refer:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDeployment {
    pub id: i64,                      // Unique deployment ID
    pub node_id: String,              // GitHub node ID
    pub url: String,                  // API URL for the deployment
    pub sha: String,                  // Commit SHA being deployed
    pub reference: String,            // Git reference (branch/tag)
    pub task: String,                 // Task name (e.g., "deploy")
    pub payload: serde_json::Value,   // Arbitrary payload data
    pub original_environment: String, // Original environment before override
    pub environment: String,          // Target deployment environment
    pub description: Option<String>,  // Description of the deployment
    pub creator: Option<Author>,      // User who triggered the deployment
    pub created_at: DateTime<Utc>,    // Deployment creation timestamp
    pub updated_at: DateTime<Utc>,    // Last update timestamp
    pub statuses_url: String,         // URL to check deployment statuses
    pub repository_url: String,       // Repository URL
    pub transient_environment: bool,  // Temporary environment flag
    pub production_environment: bool, // Whether this is a production deployment
}

/// Represents a GitHub Deployment Status
/// Refer: https://docs.github.com/en/rest/deployments/statuses?apiVersion=2022-11-28#list-deployment-statuses

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDeploymentStatus {
    pub id: i64,                         // Unique status ID
    pub node_id: String,                 // GitHub node ID
    pub url: String,                     // API URL for this deployment status
    pub state: CustomDeploymentState,    // Deployment state (e.g., success, failure)
    pub creator: Author,                 // The user who created the status
    pub description: Option<String>,     // Optional deployment status description
    pub environment: String,             // Target environment
    pub target_url: Option<String>,      // Optional link to deployment logs/output
    pub created_at: DateTime<Utc>,       // Timestamp when status was created
    pub updated_at: DateTime<Utc>,       // Timestamp when status was updated
    pub deployment_url: String,          // URL to the deployment
    pub repository_url: String,          // URL to the repository
    pub environment_url: Option<String>, // Link to environment if available
    pub log_url: Option<String>,         // Link to deployment logs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomDeploymentState {
    Success,
    Failure,
    Pending,
    InProgress,
    Error,
    Other(String), // Fallback for unknown states
}
