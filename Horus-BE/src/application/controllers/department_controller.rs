use crate::application::AppState;
use crate::common::errors::ErrorResponse;
use crate::common::types::PaginationParams;
use crate::domain::department::entities::Department;
use crate::domain::department::services::DepartmentService;
use crate::domain::department::{
    AssignUserToDepartmentRequest, BatchAssignUsersToDepartmentRequest, CreateDepartmentRequest,
    DepartmentListResponse, DepartmentResponse, UpdateDepartmentRequest,
};
use crate::domain::roles::DepartmentRole;
use crate::infrastructure::database::{
    DepartmentRepositoryImpl, DepartmentRoleRepositoryImpl, RoleRepositoryImpl, UserRepositoryImpl,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, ResponseError};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Controller for handling department-related HTTP requests
pub struct DepartmentController {
    app_state: web::Data<AppState>,
}

impl DepartmentController {
    /// Creates a new department controller with the given app state
    fn new(app_state: web::Data<AppState>) -> Self {
        Self { app_state }
    }

    /// Creates a new department
    async fn create_department(&self, dto: CreateDepartmentRequest) -> HttpResponse {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service.create_department(dto).await {
            Ok(department) => {
                let response = DepartmentResponse {
                    id: department.id,
                    organization_id: department.organization_id,
                    name: department.name,
                    created_at: department.created_at,
                    updated_at: department.updated_at,
                };
                HttpResponse::Created().json(response)
            }
            Err(e) => e.error_response(),
        }
    }

    /// Updates an existing department
    async fn update_department(&self, id: i32, dto: UpdateDepartmentRequest) -> HttpResponse {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service.update_department(id, dto).await {
            Ok(department) => {
                let response = DepartmentResponse {
                    id: department.id,
                    organization_id: department.organization_id,
                    name: department.name,
                    created_at: department.created_at,
                    updated_at: department.updated_at,
                };
                HttpResponse::Ok().json(response)
            }
            Err(e) => e.error_response(),
        }
    }

    /// Deletes a department
    async fn delete_department(&self, id: i32) -> HttpResponse {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service.delete_department(id).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => e.error_response(),
        }
    }

    /// Gets a department by ID
    async fn get_department(&self, id: i32) -> HttpResponse {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service.get_department(id).await {
            Ok(Some(department)) => {
                let response = DepartmentResponse {
                    id: department.id,
                    organization_id: department.organization_id,
                    name: department.name,
                    created_at: department.created_at,
                    updated_at: department.updated_at,
                };
                HttpResponse::Ok().json(response)
            }
            Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
                status: "error".to_string(),
                message: format!("Department not found with id: {}", id),
                error_code: Some("DEPARTMENT_NOT_FOUND".to_string()),
                details: None,
            }),
            Err(e) => e.error_response(),
        }
    }

    /// Handles listing all departments with pagination
    pub async fn list_departments(
        &self,
        organization_id: i32,
        params: &PaginationParams,
    ) -> Result<(Vec<Department>, i64), HttpResponse> {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service.list_departments(organization_id, params).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Handles adding a user to a department
    pub async fn add_user_to_department(
        &self,
        department_id: i32,
        data: AssignUserToDepartmentRequest,
        default_role_id: i32,
    ) -> Result<DepartmentRole, HttpResponse> {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        if let Err(err) = data.validate() {
            return Err(HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            }));
        }

        match service
            .add_user_to_department(department_id, data, default_role_id)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Handles removing a user from a department
    pub async fn remove_user_from_department(
        &self,
        department_id: i32,
        user_id: i32,
    ) -> Result<bool, HttpResponse> {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service
            .remove_user_from_department(department_id, user_id)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Handles getting all users in a department
    pub async fn get_department_users(
        &self,
        department_id: i32,
        params: &PaginationParams,
    ) -> Result<(Vec<i32>, i64), HttpResponse> {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service.get_department_users(department_id, params).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Handles batch adding users to a department
    pub async fn batch_add_users_to_department(
        &self,
        department_id: i32,
        data: BatchAssignUsersToDepartmentRequest,
        default_role_id: i32,
    ) -> Result<Vec<DepartmentRole>, HttpResponse> {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        if let Err(err) = data.validate() {
            return Err(HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            }));
        }

        match service
            .batch_add_users_to_department(department_id, data, default_role_id)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }

    /// Handles listing all departments with pagination (admin only)
    pub async fn list_all_departments(
        &self,
        params: &PaginationParams,
    ) -> Result<(Vec<Department>, i64), HttpResponse> {
        let repository = DepartmentRepositoryImpl::new(self.app_state.db_pool.clone());
        let role_repository = RoleRepositoryImpl::new(self.app_state.db_pool.clone());
        let user_repository = UserRepositoryImpl::new(self.app_state.db_pool.clone());
        let dept_role_repository =
            DepartmentRoleRepositoryImpl::new(self.app_state.db_pool.clone());

        let service = DepartmentService::new(
            repository,
            user_repository,
            dept_role_repository,
            role_repository,
            self.app_state.redis.clone(),
        );

        match service.list_all_departments(params).await {
            Ok(result) => Ok(result),
            Err(e) => Err(e.error_response()),
        }
    }
}

/// Parameter for default role ID
#[derive(Debug, serde::Deserialize, IntoParams, ToSchema)]
pub struct DefaultRoleQuery {
    /// Default role ID to assign
    pub default_role_id: i32,
}

/// Create a new department
#[utoipa::path(
    post,
    path = "/api/v1/departments",
    request_body = CreateDepartmentRequest,
    responses(
        (status = 201, description = "Department created successfully", body = DepartmentResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[post("")]
pub async fn create_department(
    data: web::Json<CreateDepartmentRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = DepartmentController::new(pool);
    controller.create_department(data.into_inner()).await
}

/// Update an existing department
#[utoipa::path(
    put,
    path = "/api/v1/departments/{id}",
    params(
        ("id" = i32, Path, description = "Department ID")
    ),
    request_body = UpdateDepartmentRequest,
    responses(
        (status = 200, description = "Department updated successfully", body = DepartmentResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Department not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[put("/{id}")]
pub async fn update_department(
    id: web::Path<i32>,
    data: web::Json<UpdateDepartmentRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = DepartmentController::new(pool);
    controller
        .update_department(id.into_inner(), data.into_inner())
        .await
}

/// Delete a department
#[utoipa::path(
    delete,
    path = "/api/v1/departments/{id}",
    params(
        ("id" = i32, Path, description = "Department ID")
    ),
    responses(
        (status = 200, description = "Department deleted successfully", body = bool),
        (status = 404, description = "Department not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[delete("/{id}")]
pub async fn delete_department(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = DepartmentController::new(pool);
    controller.delete_department(id.into_inner()).await
}

/// Get a department by ID
#[utoipa::path(
    get,
    path = "/api/v1/departments/{id}",
    params(
        ("id" = i32, Path, description = "Department ID")
    ),
    responses(
        (status = 200, description = "Department retrieved successfully", body = DepartmentResponse),
        (status = 404, description = "Department not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[get("/{id}")]
pub async fn get_department(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = DepartmentController::new(pool);
    controller.get_department(id.into_inner()).await
}

/// List all departments with pagination
#[utoipa::path(
    get,
    path = "/api/v1/organizations/{organization_id}/departments",
    params(
        ("organization_id" = i32, Path, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "Departments retrieved successfully", body = DepartmentListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[get("/organizations/{organization_id}/departments")]
pub async fn list_departments(
    organization_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = DepartmentController::new(pool);

    match controller.list_departments(*organization_id, &params).await {
        Ok((departments, total)) => {
            let department_responses: Vec<DepartmentResponse> = departments
                .into_iter()
                .map(|dept| DepartmentResponse {
                    id: dept.id,
                    organization_id: dept.organization_id,
                    name: dept.name,
                    created_at: dept.created_at,
                    updated_at: dept.updated_at,
                })
                .collect();

            HttpResponse::Ok().json(DepartmentListResponse {
                departments: department_responses,
                total,
                page: params.page,
                page_size: params.per_page,
            })
        }
        Err(response) => response,
    }
}

/// Add a user to a department
#[utoipa::path(
    post,
    path = "/api/v1/departments/{department_id}/users",
    params(
        ("department_id" = i32, Path, description = "Department ID")
    ),
    request_body = AssignUserToDepartmentRequest,
    responses(
        (status = 200, description = "User added to department successfully", body = DepartmentRole),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Department or user not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[post("/departments/{department_id}/users")]
pub async fn add_user_to_department(
    department_id: web::Path<i32>,
    data: web::Json<AssignUserToDepartmentRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = DepartmentController::new(pool);

    // Default role ID - in a real app, this would be configurable or retrieved from a database
    let default_role_id = 1;

    match controller
        .add_user_to_department(*department_id, data.into_inner(), default_role_id)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(response) => response,
    }
}

/// Remove a user from a department
#[utoipa::path(
    delete,
    path = "/api/v1/departments/{department_id}/users/{user_id}",
    params(
        ("department_id" = i32, Path, description = "Department ID"),
        ("user_id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User removed from department successfully", body = bool),
        (status = 404, description = "Department, user, or assignment not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[delete("/departments/{department_id}/users/{user_id}")]
pub async fn remove_user_from_department(
    path: web::Path<(i32, i32)>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let (department_id, user_id) = path.into_inner();
    let controller = DepartmentController::new(pool);

    match controller
        .remove_user_from_department(department_id, user_id)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(response) => response,
    }
}

/// Get users in a department
#[utoipa::path(
    get,
    path = "/api/v1/departments/{department_id}/users",
    params(
        ("department_id" = i32, Path, description = "Department ID")
    ),
    responses(
        (status = 200, description = "Department users retrieved successfully", body = Vec<i32>),
        (status = 404, description = "Department not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[get("/departments/{department_id}/users")]
pub async fn get_department_users(
    department_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = DepartmentController::new(pool);

    match controller
        .get_department_users(*department_id, &params)
        .await
    {
        Ok((user_ids, total)) => HttpResponse::Ok().json(serde_json::json!({
            "user_ids": user_ids,
            "total": total,
            "page": params.page,
            "page_size": params.per_page
        })),
        Err(response) => response,
    }
}

/// Batch add users to a department
#[utoipa::path(
    post,
    path = "/api/v1/departments/{department_id}/users/batch",
    params(
        ("department_id" = i32, Path, description = "Department ID")
    ),
    request_body = BatchAssignUsersToDepartmentRequest,
    responses(
        (status = 200, description = "Users added to department successfully", body = Vec<DepartmentRole>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Department or users not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[post("/departments/{department_id}/users/batch")]
pub async fn batch_add_users_to_department(
    department_id: web::Path<i32>,
    data: web::Json<BatchAssignUsersToDepartmentRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = DepartmentController::new(pool);

    // Default role ID - in a real app, this would be configurable or retrieved from a database
    let default_role_id = 1;

    match controller
        .batch_add_users_to_department(*department_id, data.into_inner(), default_role_id)
        .await
    {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(response) => response,
    }
}

/// List all departments with pagination (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/departments/all",
    responses(
        (status = 200, description = "Departments retrieved successfully", body = DepartmentListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Departments"
)]
#[get("/all")]
pub async fn list_all_departments(
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = DepartmentController::new(pool);

    match controller.list_all_departments(&params).await {
        Ok((departments, total)) => {
            let department_responses: Vec<DepartmentResponse> = departments
                .into_iter()
                .map(|dept| DepartmentResponse {
                    id: dept.id,
                    organization_id: dept.organization_id,
                    name: dept.name,
                    created_at: dept.created_at,
                    updated_at: dept.updated_at,
                })
                .collect();

            HttpResponse::Ok().json(DepartmentListResponse {
                departments: department_responses,
                total,
                page: params.page,
                page_size: params.per_page,
            })
        }
        Err(response) => response,
    }
}
