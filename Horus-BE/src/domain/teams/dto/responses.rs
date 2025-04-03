use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::{teams::Team, user::UserResponse};

/// Response representing a team
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamResponse {
    /// Team ID
    pub id: i32,
    /// Organization ID
    pub organization_id: i32,
    /// Department ID
    pub department_id: i32,
    /// Team name
    pub name: String,
    /// GitHub team name
    pub name_gh: String,
    /// When the team was created
    pub created_at: NaiveDateTime,
    /// When the team was last updated
    pub updated_at: NaiveDateTime,
}

/// Response representing a team member
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamMemberResponse {
    /// Team member ID
    pub id: i32,
    /// Team ID
    pub team_id: i32,
    /// User ID
    pub user_id: i32,
    /// When the user joined the team
    pub joined_at: NaiveDateTime,
}

/// Response representing a team with its members
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamWithMembersResponse {
    /// Team information
    pub team: TeamResponse,
    /// List of team members
    pub members: Vec<UserResponse>,
}

/// Response for a list of teams
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamListResponse {
    /// List of teams
    pub teams: Vec<TeamResponse>,
    /// Total count of teams (for pagination)
    pub total: i64,
    /// Current page
    pub page: i64,
    /// Page size
    pub page_size: i64,
}

/// Response for team creation or update
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamActionResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Team data if operation was successful
    pub team: Option<TeamResponse>,
    /// Error message if operation failed
    pub message: Option<String>,
}

/// Response for team repository assignment
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamRepoResponse {
    /// Team-Repo association ID
    pub id: i32,
    /// Team ID
    pub team_id: i32,
    /// Repository ID
    pub repo_id: i32,
}

/// Response for team with repositories
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TeamWithReposResponse {
    /// Team information
    pub team: TeamResponse,
    /// List of repository IDs
    pub repo_ids: Vec<i32>,
}

impl From<Team> for TeamResponse {
    fn from(team: Team) -> Self {
        Self {
            id: team.id,
            organization_id: team.organization_id,
            department_id: team.department_id,
            name: team.name,
            name_gh: team.name_gh,
            created_at: team.created_at,
            updated_at: team.updated_at,
        }
    }
}
