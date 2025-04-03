use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// Request to create a new team
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTeamRequest {
    /// Organization ID the team belongs to
    pub organization_id: i32,

    /// Department ID the team belongs to
    pub department_id: i32,

    /// Name of the team
    #[validate(length(
        min = 2,
        max = 100,
        message = "Team name must be between 2 and 100 characters"
    ))]
    pub name: String,

    /// GitHub team name
    #[validate(length(
        min = 2,
        max = 100,
        message = "GitHub team name must be between 2 and 100 characters"
    ))]
    pub name_gh: String,
}

/// Request to update an existing team
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTeamRequest {
    /// Updated department ID (optional)
    pub department_id: Option<i32>,

    /// Updated name (optional)
    #[validate(length(
        min = 2,
        max = 100,
        message = "Team name must be between 2 and 100 characters"
    ))]
    pub name: Option<String>,

    /// Updated GitHub team name (optional)
    #[validate(length(
        min = 2,
        max = 100,
        message = "GitHub team name must be between 2 and 100 characters"
    ))]
    pub name_gh: Option<String>,
}

/// Request to add a member to a team
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddTeamMemberRequest {
    /// User ID to add to the team
    pub user_id: i32,
}

/// Request to assign a repository to a team
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignRepoToTeamRequest {
    /// Repository ID to assign to the team
    pub repo_id: i32,
}

/// Request to batch add members to a team
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchAddTeamMembersRequest {
    /// List of user IDs to add to the team
    #[validate(length(min = 1, message = "At least one user ID must be provided"))]
    pub user_ids: Vec<i32>,
}

/// Request to batch assign repositories to a team
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchAssignReposToTeamRequest {
    /// List of repository IDs to assign to the team
    #[validate(length(min = 1, message = "At least one repository ID must be provided"))]
    pub repo_ids: Vec<i32>,
}
