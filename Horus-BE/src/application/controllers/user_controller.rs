use crate::application::AppState;
use crate::common::errors::ErrorResponse;
use crate::common::types::PaginationParams;
use crate::domain::user::{
    AssignRoleRequest, ChangePasswordRequest, CreateUserRequest, LoginRequest, UpdateUserRequest,
    UserResponse, UserService,
};
use crate::infrastructure::database::{UserRepositoryImpl, UserRoleRepositoryImpl};
use actix_web::{web, HttpResponse, Responder, ResponseError};
use validator::Validate;

/// Controller that handles user-related HTTP requests
pub struct UserController {
    app_state: web::Data<AppState>,
}

impl UserController {
    /// Creates a new instance of the user controller
    fn new(app_state: web::Data<AppState>) -> Self {
        Self { app_state }
    }

    /// Registers a new user
    async fn register(&self, data: CreateUserRequest) -> HttpResponse {
        // Validate input
        if let Err(err) = data.validate() {
            return HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            });
        }

        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Create user
        match service.create_user(data).await {
            Ok(user) => {
                let response = UserResponse {
                    id: user.id,
                    organization_id: user.organization_id,
                    username: user.username,
                    email: user.email,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                };
                HttpResponse::Created().json(response)
            }
            Err(e) => e.error_response(),
        }
    }

    /// Authenticates a user and returns a JWT token
    async fn login(&self, data: LoginRequest) -> HttpResponse {
        // Validate input
        if let Err(err) = data.validate() {
            return HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            });
        }

        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Login user
        match service.login(data).await {
            Ok(auth_response) => HttpResponse::Ok().json(auth_response),
            Err(e) => e.error_response(),
        }
    }

    /// Gets a user by ID
    async fn get_user(&self, id: i32) -> HttpResponse {
        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Get user
        match service.get_user(id).await {
            Ok(Some(user)) => {
                let response = UserResponse {
                    id: user.id,
                    organization_id: user.organization_id,
                    username: user.username,
                    email: user.email,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                };
                HttpResponse::Ok().json(response)
            }
            Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
                status: "error".to_string(),
                message: format!("User with ID {} not found", id),
                error_code: Some("USER_NOT_FOUND".to_string()),
                details: None,
            }),
            Err(e) => e.error_response(),
        }
    }

    /// Gets a user with their roles
    async fn get_user_with_roles(&self, id: i32) -> HttpResponse {
        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Get user with roles
        match service.get_user_with_roles(id).await {
            Ok(user_with_roles) => HttpResponse::Ok().json(user_with_roles),
            Err(e) => e.error_response(),
        }
    }

    /// Updates a user
    async fn update_user(&self, id: i32, data: UpdateUserRequest) -> HttpResponse {
        // Validate input
        if let Err(err) = data.validate() {
            return HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            });
        }

        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Update user
        match service.update_user(id, data).await {
            Ok(user) => {
                let response = UserResponse {
                    id: user.id,
                    organization_id: user.organization_id,
                    username: user.username,
                    email: user.email,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                };
                HttpResponse::Ok().json(response)
            }
            Err(e) => e.error_response(),
        }
    }

    /// Changes a user's password
    async fn change_password(&self, id: i32, data: ChangePasswordRequest) -> HttpResponse {
        // Validate input
        if let Err(err) = data.validate() {
            return HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            });
        }

        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Change password
        match service.change_password(id, data).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
            Err(e) => e.error_response(),
        }
    }

    /// Lists users
    async fn list_users(&self, organization_id: i32, params: &PaginationParams) -> HttpResponse {
        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // List users
        match service.list_users(organization_id, params).await {
            Ok((users, total)) => {
                let user_responses: Vec<UserResponse> = users
                    .into_iter()
                    .map(|user| UserResponse {
                        id: user.id,
                        organization_id: user.organization_id,
                        username: user.username,
                        email: user.email,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    })
                    .collect();

                HttpResponse::Ok().json(serde_json::json!({
                    "users": user_responses,
                    "total": total,
                    "page": params.page,
                    "page_size": params.per_page
                }))
            }
            Err(e) => e.error_response(),
        }
    }

    /// Assigns a role to a user
    async fn assign_role(&self, data: AssignRoleRequest) -> HttpResponse {
        // Validate input
        if let Err(err) = data.validate() {
            return HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            });
        }

        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Assign role
        match service.assign_role(data).await {
            Ok(user_role) => HttpResponse::Ok().json(user_role),
            Err(e) => e.error_response(),
        }
    }

    /// Removes a role from a user
    async fn remove_role(&self, user_id: i32, role_id: i32) -> HttpResponse {
        // Create service
        let repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = UserRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = UserService::new(
            repository,
            role_repository,
            self.app_state.redis.clone(),
            self.app_state.config.server.jwt_secret.clone(),
            self.app_state.config.server.token_expiry,
        );

        // Remove role
        match service.remove_role(user_id, role_id).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
            Err(e) => e.error_response(),
        }
    }
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/users/register",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn register_user(
    data: web::Json<CreateUserRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = UserController::new(pool);
    controller.register(data.into_inner()).await
}

/// Login a user
#[utoipa::path(
    post,
    path = "/api/users/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn user_login(
    data: web::Json<LoginRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    println!("[Go here] data: {:?}", data);
    let controller = UserController::new(pool);
    controller.login(data.into_inner()).await
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn get_user(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = UserController::new(pool);
    controller.get_user(id.into_inner()).await
}

/// Get user with roles by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}/roles",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User with roles found", body = UserWithRolesResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn get_user_with_roles(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = UserController::new(pool);
    controller.get_user_with_roles(id.into_inner()).await
}

/// Update user by ID
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn update_user(
    id: web::Path<i32>,
    data: web::Json<UpdateUserRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = UserController::new(pool);
    controller
        .update_user(id.into_inner(), data.into_inner())
        .await
}

/// Change user password
#[utoipa::path(
    put,
    path = "/api/users/{id}/change-password",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn change_password(
    id: web::Path<i32>,
    data: web::Json<ChangePasswordRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = UserController::new(pool);
    controller
        .change_password(id.into_inner(), data.into_inner())
        .await
}

/// List users with pagination
#[utoipa::path(
    get,
    path = "/api/users-orgs/{organization_id}/users",
    params(
        ("organization_id" = i32, Path, description = "Organization ID"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of users"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn list_users(
    organization_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = UserController::new(pool);
    controller
        .list_users(organization_id.into_inner(), &params)
        .await
}

/// Assign role to user
#[utoipa::path(
    post,
    path = "/api/users/roles",
    request_body = AssignRoleRequest,
    responses(
        (status = 200, description = "Role assigned successfully"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "User or role not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn assign_role(
    data: web::Json<AssignRoleRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = UserController::new(pool);
    controller.assign_role(data.into_inner()).await
}

/// Remove role from user
#[utoipa::path(
    delete,
    path = "/api/users/{user_id}/roles/{role_id}",
    params(
        ("user_id" = i32, Path, description = "User ID"),
        ("role_id" = i32, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role removed successfully"),
        (status = 404, description = "User or role not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn remove_role(path: web::Path<(i32, i32)>, pool: web::Data<AppState>) -> impl Responder {
    let (user_id, role_id) = path.into_inner();
    let controller = UserController::new(pool);
    controller.remove_role(user_id, role_id).await
}
