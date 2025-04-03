use crate::application::AppState;
use crate::common::errors::ErrorResponse;
use crate::common::types::PaginationParams;
use crate::domain::roles::{
    AssignDepartmentRoleRequest, CreateRoleRequest, DepartmentRole, Role, RoleListResponse,
    RoleResponse, RoleService, UpdateRoleRequest,
};
use crate::domain::user::repositories::user_repository::UserRepository;
use crate::infrastructure::database::{DepartmentRoleRepositoryImpl, RoleRepositoryImpl};
use actix_web::{web, HttpResponse, Responder, ResponseError};
use async_trait::async_trait;

/// A mock implementation of the UserRepository trait for the Role controller
#[derive(Debug, Clone, Default)]
struct MockUserRepository;

#[async_trait]
impl UserRepository for MockUserRepository {
    // Mock implementations similar to those in department_controller.rs
    // These can be implemented as needed for role controller functionality
    async fn create_user(
        &self,
        _dto: crate::domain::user::CreateUserRequest,
    ) -> crate::domain::user::UserResult<crate::domain::user::User> {
        Ok(crate::domain::user::User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
    }

    async fn update_user(
        &self,
        _id: i32,
        _dto: crate::domain::user::UpdateUserRequest,
    ) -> crate::domain::user::UserResult<crate::domain::user::User> {
        Ok(crate::domain::user::User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
    }

    async fn delete_user(&self, _id: i32) -> crate::domain::user::UserResult<bool> {
        Ok(true)
    }

    async fn get_user_by_id(
        &self,
        _id: i32,
    ) -> crate::domain::user::UserResult<Option<crate::domain::user::User>> {
        Ok(Some(crate::domain::user::User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }))
    }

    async fn get_user_by_username(
        &self,
        _username: &str,
    ) -> crate::domain::user::UserResult<Option<crate::domain::user::User>> {
        Ok(Some(crate::domain::user::User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }))
    }

    async fn get_user_by_email(
        &self,
        _email: &str,
    ) -> crate::domain::user::UserResult<Option<crate::domain::user::User>> {
        Ok(Some(crate::domain::user::User {
            id: 1,
            organization_id: 1,
            username: "mock_user".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }))
    }

    async fn username_exists(&self, _username: &str) -> crate::domain::user::UserResult<bool> {
        Ok(false)
    }

    async fn email_exists(&self, _email: &str) -> crate::domain::user::UserResult<bool> {
        Ok(false)
    }

    async fn find_users_by_username(
        &self,
        _organization_id: i32,
        _username_pattern: &str,
        _params: &PaginationParams,
    ) -> crate::domain::user::UserResult<(Vec<crate::domain::user::User>, i64)> {
        Ok((Vec::new(), 0))
    }

    async fn list_users(
        &self,
        _organization_id: i32,
        _params: &PaginationParams,
    ) -> crate::domain::user::UserResult<(Vec<crate::domain::user::User>, i64)> {
        Ok((Vec::new(), 0))
    }

    async fn get_users_by_ids(
        &self,
        _ids: Vec<i32>,
    ) -> crate::domain::user::UserResult<Vec<crate::domain::user::User>> {
        Ok(Vec::new())
    }

    async fn update_password(
        &self,
        _id: i32,
        _hashed_password: &str,
    ) -> crate::domain::user::UserResult<bool> {
        Ok(true)
    }
}

/// Controller for handling role-related HTTP requests
pub struct RoleController {
    service: RoleService<RoleRepositoryImpl, DepartmentRoleRepositoryImpl, MockUserRepository>,
}

impl RoleController {
    /// Creates a new RoleController
    pub fn new(pool: web::Data<AppState>) -> Self {
        let role_repository = RoleRepositoryImpl::new(pool.db_pool.clone());
        let department_role_repository = DepartmentRoleRepositoryImpl::new(pool.db_pool.clone());
        let user_repository = MockUserRepository::default();

        Self {
            service: RoleService::new(
                role_repository,
                department_role_repository,
                user_repository,
                pool.redis.clone(),
            ),
        }
    }

    /// Creates a new role
    pub async fn create_role(&self, data: CreateRoleRequest) -> Result<Role, HttpResponse> {
        match self.service.create_role(data).await {
            Ok(role) => Ok(role),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Updates an existing role
    pub async fn update_role(
        &self,
        id: i32,
        data: UpdateRoleRequest,
    ) -> Result<Role, HttpResponse> {
        match self.service.update_role(id, data).await {
            Ok(role) => Ok(role),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Deletes a role
    pub async fn delete_role(&self, id: i32) -> Result<bool, HttpResponse> {
        match self.service.delete_role(id).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Gets a role by ID
    pub async fn get_role(&self, id: i32) -> Result<Option<Role>, HttpResponse> {
        match self.service.get_role(id).await {
            Ok(role) => Ok(role),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Lists roles with pagination
    pub async fn list_roles(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> Result<(Vec<Role>, i64), HttpResponse> {
        match self.service.list_roles(organization_id, params).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Assigns a role to a user in a department
    pub async fn assign_department_role(
        &self,
        organization_id: i32,
        data: AssignDepartmentRoleRequest,
    ) -> Result<DepartmentRole, HttpResponse> {
        match self
            .service
            .assign_department_role(organization_id, data)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }
}

/// Handler for creating a new role
#[utoipa::path(
    post,
    path = "/api/v1/roles",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created successfully", body = RoleResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn create_role(
    data: web::Json<CreateRoleRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = RoleController::new(pool);
    match controller.create_role(data.into_inner()).await {
        Ok(role) => HttpResponse::Created().json(RoleResponse::from(role)),
        Err(response) => response,
    }
}

/// Handler for updating an existing role
#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    request_body = UpdateRoleRequest,
    params(
        ("id" = i32, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role updated successfully", body = RoleResponse),
        (status = 404, description = "Role not found", body = ErrorResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn update_role(
    id: web::Path<i32>,
    data: web::Json<UpdateRoleRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = RoleController::new(pool);
    match controller.update_role(*id, data.into_inner()).await {
        Ok(role) => HttpResponse::Ok().json(RoleResponse::from(role)),
        Err(response) => response,
    }
}

/// Handler for deleting a role
#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}",
    params(
        ("id" = i32, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role deleted successfully"),
        (status = 404, description = "Role not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn delete_role(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = RoleController::new(pool);
    match controller.delete_role(*id).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            status: "Not Found".to_string(),
            message: format!("Role with ID {} not found", *id),
            error_code: None,
            details: None,
        }),
        Err(response) => response,
    }
}

/// Handler for getting a role by ID
#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}",
    params(
        ("id" = i32, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role retrieved successfully", body = RoleResponse),
        (status = 404, description = "Role not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn get_role(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = RoleController::new(pool);
    match controller.get_role(*id).await {
        Ok(Some(role)) => HttpResponse::Ok().json(RoleResponse::from(role)),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            status: "Not Found".to_string(),
            message: format!("Role with ID {} not found", *id),
            error_code: None,
            details: None,
        }),
        Err(response) => response,
    }
}

/// Handler for listing roles with pagination
#[utoipa::path(
    get,
    path = "/api/v1/roles-org/{organization_id}/roles",
    params(
        ("organization_id" = i32, Path, description = "Organization ID"),
        ("page" = Option<i64>, Query, description = "Page number, default is 1"),
        ("page_size" = Option<i64>, Query, description = "Items per page, default is 10")
    ),
    responses(
        (status = 200, description = "Roles listed successfully", body = RoleListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn list_roles(
    organization_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = RoleController::new(pool);
    match controller.list_roles(*organization_id, &params).await {
        Ok((roles, total)) => {
            let role_responses: Vec<RoleResponse> =
                roles.into_iter().map(RoleResponse::from).collect();
            HttpResponse::Ok().json(RoleListResponse {
                roles: role_responses,
                total,
                page: params.page,
                page_size: params.per_page,
            })
        }
        Err(response) => response,
    }
}

/// Handler for assigning a role to a user in a department
#[utoipa::path(
    post,
    path = "/api/v1/roles-org/{organization_id}/department-roles",
    request_body = AssignDepartmentRoleRequest,
    params(
        ("organization_id" = i32, Path, description = "Organization ID")
    ),
    responses(
        (status = 201, description = "Role assigned successfully"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Resource not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Roles"
)]
pub async fn assign_department_role(
    organization_id: web::Path<i32>,
    data: web::Json<AssignDepartmentRoleRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = RoleController::new(pool);
    match controller
        .assign_department_role(*organization_id, data.into_inner())
        .await
    {
        Ok(department_role) => HttpResponse::Created().json(department_role),
        Err(response) => response,
    }
}
