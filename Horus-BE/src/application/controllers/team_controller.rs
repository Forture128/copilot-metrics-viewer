use crate::application::AppState;
use crate::common::errors::ErrorResponse;
use crate::common::types::PaginationParams;
use crate::domain::teams::{
    AddTeamMemberRequest, CreateTeamRequest, Team, TeamListResponse, TeamMember, TeamResponse,
    TeamService, TeamWithMembersResponse, UpdateTeamRequest,
};
use crate::domain::user::repositories::user_repository::UserRepository;
use crate::domain::user::{CreateUserRequest, UpdateUserRequest, User, UserResult};
use crate::infrastructure::database::{
    TeamMemberRepositoryImpl, TeamRepoRepositoryImpl, TeamRepositoryImpl,
};
use actix_web::{web, HttpResponse, Responder, ResponseError};
use async_trait::async_trait;

/// A mock implementation of the UserRepository trait for the Team controller
#[derive(Debug, Clone, Default)]
struct MockUserRepository;

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create_user(&self, _dto: CreateUserRequest) -> UserResult<User> {
        Ok(User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
    }

    async fn update_user(&self, _id: i32, _dto: UpdateUserRequest) -> UserResult<User> {
        Ok(User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
    }

    async fn delete_user(&self, _id: i32) -> UserResult<bool> {
        Ok(true)
    }

    async fn get_user_by_id(&self, _id: i32) -> UserResult<Option<User>> {
        Ok(Some(User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }))
    }

    async fn get_user_by_username(&self, _username: &str) -> UserResult<Option<User>> {
        Ok(Some(User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }))
    }

    async fn get_user_by_email(&self, _email: &str) -> UserResult<Option<User>> {
        Ok(Some(User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }))
    }

    async fn username_exists(&self, _username: &str) -> UserResult<bool> {
        Ok(false)
    }

    async fn email_exists(&self, _email: &str) -> UserResult<bool> {
        Ok(false)
    }

    async fn find_users_by_username(
        &self,
        _organization_id: i32,
        _username_pattern: &str,
        _params: &PaginationParams,
    ) -> UserResult<(Vec<User>, i64)> {
        Ok((Vec::new(), 0))
    }

    async fn list_users(
        &self,
        _organization_id: i32,
        _params: &PaginationParams,
    ) -> UserResult<(Vec<User>, i64)> {
        Ok((Vec::new(), 0))
    }

    async fn get_users_by_ids(&self, _ids: Vec<i32>) -> UserResult<Vec<User>> {
        Ok(Vec::new())
    }

    async fn update_password(&self, _id: i32, _hashed_password: &str) -> UserResult<bool> {
        Ok(true)
    }
}

/// Controller for handling team-related HTTP requests
pub struct TeamController {
    service: TeamService<
        TeamRepositoryImpl,
        TeamMemberRepositoryImpl,
        TeamRepoRepositoryImpl,
        MockUserRepository,
    >,
}

impl TeamController {
    /// Creates a new TeamController
    pub fn new(pool: web::Data<AppState>) -> Self {
        let team_repository = TeamRepositoryImpl::new(pool.db_pool.clone());
        let member_repository = TeamMemberRepositoryImpl::new(pool.db_pool.clone());
        let repo_repository = TeamRepoRepositoryImpl::new(pool.db_pool.clone());
        let user_repository = MockUserRepository::default();

        Self {
            service: TeamService::new(
                team_repository,
                member_repository,
                repo_repository,
                user_repository,
                pool.redis.clone(),
            ),
        }
    }

    /// Creates a new team
    pub async fn create_team(&self, data: CreateTeamRequest) -> Result<Team, HttpResponse> {
        match self.service.create_team(data).await {
            Ok(team) => Ok(team),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Updates an existing team
    pub async fn update_team(
        &self,
        id: i32,
        data: UpdateTeamRequest,
    ) -> Result<Team, HttpResponse> {
        match self.service.update_team(id, data).await {
            Ok(team) => Ok(team),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Deletes a team
    pub async fn delete_team(&self, id: i32) -> Result<bool, HttpResponse> {
        match self.service.delete_team(id).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Gets a team by ID
    pub async fn get_team(&self, id: i32) -> Result<Option<Team>, HttpResponse> {
        match self.service.get_team(id).await {
            Ok(team) => Ok(team),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Lists teams with pagination
    pub async fn list_teams(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> Result<(Vec<Team>, i64), HttpResponse> {
        match self.service.list_teams(organization_id, params).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Lists teams by department with pagination
    pub async fn list_teams_by_department(
        &self,
        organization_id: i32,
        department_id: i32,
        params: &PaginationParams,
    ) -> Result<(Vec<Team>, i64), HttpResponse> {
        match self
            .service
            .list_teams_by_department(organization_id, department_id, params)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Adds a member to a team
    pub async fn add_team_member(
        &self,
        team_id: i32,
        data: AddTeamMemberRequest,
    ) -> Result<TeamMember, HttpResponse> {
        match self.service.add_team_member(team_id, data).await {
            Ok(member) => Ok(member),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Removes a member from a team
    pub async fn remove_team_member(
        &self,
        team_id: i32,
        user_id: i32,
    ) -> Result<bool, HttpResponse> {
        match self.service.remove_team_member(team_id, user_id).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Gets a team with its members
    pub async fn get_team_with_members(
        &self,
        team_id: i32,
        params: &PaginationParams,
    ) -> Result<TeamWithMembersResponse, HttpResponse> {
        match self.service.get_team_with_members(team_id, params).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }
}

/// Handler for creating a new team
#[utoipa::path(
    post,
    path = "/api/v1/teams",
    request_body = CreateTeamRequest,
    responses(
        (status = 201, description = "Team created successfully", body = TeamResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn create_team(
    data: web::Json<CreateTeamRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller.create_team(data.into_inner()).await {
        Ok(team) => HttpResponse::Created().json(TeamResponse::from(team)),
        Err(response) => response,
    }
}

/// Handler for updating an existing team
#[utoipa::path(
    put,
    path = "/api/v1/teams/{id}",
    request_body = UpdateTeamRequest,
    params(
        ("id" = i32, Path, description = "Team ID")
    ),
    responses(
        (status = 200, description = "Team updated successfully", body = TeamResponse),
        (status = 404, description = "Team not found", body = ErrorResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn update_team(
    id: web::Path<i32>,
    data: web::Json<UpdateTeamRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller.update_team(*id, data.into_inner()).await {
        Ok(team) => HttpResponse::Ok().json(TeamResponse::from(team)),
        Err(response) => response,
    }
}

/// Handler for deleting a team
#[utoipa::path(
    delete,
    path = "/api/v1/teams/{id}",
    params(
        ("id" = i32, Path, description = "Team ID")
    ),
    responses(
        (status = 200, description = "Team deleted successfully"),
        (status = 404, description = "Team not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn delete_team(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller.delete_team(*id).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            status: "Not Found".to_string(),
            message: "Team not found".to_string(),
            details: None,
            error_code: None,
        }),
        Err(response) => response,
    }
}

/// Handler for getting a team by ID
#[utoipa::path(
    get,
    path = "/api/v1/teams/{id}",
    params(
        ("id" = i32, Path, description = "Team ID")
    ),
    responses(
        (status = 200, description = "Team retrieved successfully", body = TeamResponse),
        (status = 404, description = "Team not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn get_team(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller.get_team(*id).await {
        Ok(Some(team)) => HttpResponse::Ok().json(TeamResponse::from(team)),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            status: "Not Found".to_string(),
            message: format!("Team with ID {} not found", *id),
            details: None,
            error_code: None,
        }),
        Err(response) => response,
    }
}

/// Handler for listing teams with pagination
#[utoipa::path(
    get,
    path = "/api/v1/teams-org/{organization_id}/teams",
    params(
        ("organization_id" = i32, Path, description = "Organization ID"),
        ("page" = Option<i64>, Query, description = "Page number, default is 1"),
        ("page_size" = Option<i64>, Query, description = "Items per page, default is 10")
    ),
    responses(
        (status = 200, description = "Teams listed successfully", body = TeamListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn list_teams(
    organization_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller.list_teams(*organization_id, &params).await {
        Ok((teams, total)) => {
            let team_responses: Vec<TeamResponse> =
                teams.into_iter().map(TeamResponse::from).collect();
            HttpResponse::Ok().json(TeamListResponse {
                teams: team_responses,
                total,
                page: params.page,
                page_size: params.per_page,
            })
        }
        Err(response) => response,
    }
}

/// Handler for listing teams by department with pagination
#[utoipa::path(
    get,
    path = "/api/v1/teams-org/{organization_id}/teams/by-department/{department_id}",
    params(
        ("organization_id" = i32, Path, description = "Organization ID"),
        ("department_id" = i32, Path, description = "Department ID"),
        ("page" = Option<i64>, Query, description = "Page number, default is 1"),
        ("page_size" = Option<i64>, Query, description = "Items per page, default is 10")
    ),
    responses(
        (status = 200, description = "Teams listed successfully", body = TeamListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn list_teams_by_department(
    organization_id: web::Path<i32>,
    department_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller
        .list_teams_by_department(*organization_id, *department_id, &params)
        .await
    {
        Ok((teams, total)) => {
            let team_responses: Vec<TeamResponse> =
                teams.into_iter().map(TeamResponse::from).collect();
            HttpResponse::Ok().json(TeamListResponse {
                teams: team_responses,
                total,
                page: params.page,
                page_size: params.per_page,
            })
        }
        Err(response) => response,
    }
}

/// Handler for adding a member to a team
#[utoipa::path(
    post,
    path = "/api/v1/teams/{team_id}/members",
    request_body = AddTeamMemberRequest,
    params(
        ("team_id" = i32, Path, description = "Team ID")
    ),
    responses(
        (status = 201, description = "Member added successfully", body = TeamMember),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Team not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn add_team_member(
    team_id: web::Path<i32>,
    data: web::Json<AddTeamMemberRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller
        .add_team_member(*team_id, data.into_inner())
        .await
    {
        Ok(member) => HttpResponse::Created().json(member),
        Err(response) => response,
    }
}

/// Handler for removing a member from a team
#[utoipa::path(
    delete,
    path = "/api/v1/teams/{team_id}/members/{user_id}",
    params(
        ("team_id" = i32, Path, description = "Team ID"),
        ("user_id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Member removed successfully"),
        (status = 404, description = "Team not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn remove_team_member(
    team_id: web::Path<i32>,
    user_id: web::Path<i32>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller.remove_team_member(*team_id, *user_id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(response) => response,
    }
}

/// Handler for getting a team with its members
#[utoipa::path(
    get,
    path = "/api/v1/teams/{team_id}/members",
    params(
        ("team_id" = i32, Path, description = "Team ID"),
        ("page" = Option<i64>, Query, description = "Page number, default is 1"),
        ("page_size" = Option<i64>, Query, description = "Items per page, default is 10")
    ),
    responses(
        (status = 200, description = "Team retrieved successfully", body = TeamWithMembersResponse),
        (status = 404, description = "Team not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Teams"
)]
pub async fn get_team_with_members(
    team_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = TeamController::new(pool);
    match controller.get_team_with_members(*team_id, &params).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(response) => response,
    }
}
