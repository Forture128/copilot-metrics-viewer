use crate::application::AppState;
use crate::common::errors::{AppError, ErrorResponse};
use crate::common::types::PaginationParams;
use crate::domain::organization::entities::{Organization, OrganizationConfig};
use crate::domain::organization::services::OrganizationService;
use crate::domain::organization::{
    CreateOrganizationConfigRequest, CreateOrganizationRequest, OrganizationConfigListResponse,
    OrganizationConfigResponse, OrganizationListResponse, OrganizationResponse,
    UpdateOrganizationConfigRequest, UpdateOrganizationRequest,
};
use crate::infrastructure::database::{
    OrganizationConfigRepositoryImpl, OrganizationRepositoryImpl,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, ResponseError};
use validator::Validate;

/// Controller for handling organization-related HTTP requests
pub struct OrganizationController {
    service: OrganizationService<OrganizationRepositoryImpl, OrganizationConfigRepositoryImpl>,
}

impl OrganizationController {
    /// Creates a new instance of the organization controller
    pub fn new(pool: web::Data<AppState>) -> Self {
        let repository = OrganizationRepositoryImpl::new(pool.get_ref().db_pool.clone());
        let config_repository =
            OrganizationConfigRepositoryImpl::new(pool.get_ref().db_pool.clone());
        let service =
            OrganizationService::new(repository, config_repository, pool.get_ref().redis.clone());
        Self { service }
    }

    /// Handles the creation of a new organization
    pub async fn create_organization(
        &self,
        data: CreateOrganizationRequest,
    ) -> Result<Organization, HttpResponse> {
        if let Err(err) = data.validate() {
            return Err(HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            }));
        }

        match self.service.create_organization(data).await {
            Ok(org) => Ok(org),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn update_organization(
        &self,
        id: i32,
        data: UpdateOrganizationRequest,
    ) -> Result<Organization, HttpResponse> {
        if let Err(err) = data.validate() {
            return Err(HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            }));
        }

        match self.service.update_organization(id, data).await {
            Ok(org) => Ok(org),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn delete_organization(&self, id: i32) -> Result<bool, HttpResponse> {
        match self.service.delete_organization(id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn get_organization(&self, id: i32) -> Result<Organization, HttpResponse> {
        match self.service.get_organization(id).await {
            Ok(Some(org)) => Ok(org),
            Ok(None) => Err(
                AppError::NotFound(format!("Organization with id {} not found", id))
                    .error_response(),
            ),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn list_organizations(
        &self,
        params: &PaginationParams,
    ) -> Result<(Vec<Organization>, i64), HttpResponse> {
        match self.service.list_organizations(&params).await {
            Ok((orgs, total)) => Ok((orgs, total)),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn create_config(
        &self,
        org_id: i32,
        data: CreateOrganizationConfigRequest,
    ) -> Result<OrganizationConfig, HttpResponse> {
        if let Err(err) = data.validate() {
            return Err(HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            }));
        }

        match self.service.create_config(org_id, data).await {
            Ok(config) => Ok(config),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn update_config(
        &self,
        org_id: i32,
        config_id: i32,
        data: UpdateOrganizationConfigRequest,
    ) -> Result<OrganizationConfig, HttpResponse> {
        if let Err(err) = data.validate() {
            return Err(HttpResponse::BadRequest().json(ErrorResponse {
                status: "error".to_string(),
                message: "Validation error".to_string(),
                error_code: Some("VALIDATION_ERROR".to_string()),
                details: Some(err.to_string()),
            }));
        }

        match self.service.update_config(org_id, config_id, data).await {
            Ok(config) => Ok(config),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn delete_config(&self, org_id: i32, config_id: i32) -> Result<bool, HttpResponse> {
        match self.service.delete_config(org_id, config_id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn get_config(
        &self,
        org_id: i32,
        config_id: i32,
    ) -> Result<OrganizationConfig, HttpResponse> {
        match self.service.get_config(org_id, config_id).await {
            Ok(config) => Ok(config),
            Err(e) => Err(e.error_response()),
        }
    }

    pub async fn list_configs(
        &self,
        org_id: i32,
        params: &PaginationParams,
    ) -> Result<(Vec<OrganizationConfig>, i64), HttpResponse> {
        match self.service.list_configs(org_id, params).await {
            Ok((configs, total)) => Ok((configs, total)),
            Err(e) => Err(e.error_response()),
        }
    }
}

/// Struct for deserializing search_key query parameter in organization config searches
/// Used to prevent issues when parsing query parameters from the URL
#[derive(serde::Deserialize)]
pub struct SearchKeyQuery {
    pub search_key: String,
}

/// Struct for deserializing name query parameter in organization searches
/// Used to prevent issues when parsing query parameters from the URL
#[derive(serde::Deserialize)]
pub struct SearchNameQuery {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations",
    request_body = CreateOrganizationRequest,
    responses(
        (status = 201, description = "Organization created", body = OrganizationResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[post("")]
pub async fn create_organization(
    data: web::Json<CreateOrganizationRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller.create_organization(data.into_inner()).await {
        Ok(org) => HttpResponse::Created().json(OrganizationResponse::from(org)),
        Err(e) => e,
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/organizations/{id}",
    params(
        ("id" = i32, Path, description = "Organization ID")
    ),
    request_body = UpdateOrganizationRequest,
    responses(
        (status = 200, description = "Organization updated", body = OrganizationResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Organization not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[put("/{id}")]
pub async fn update_organization(
    id: web::Path<i32>,
    data: web::Json<UpdateOrganizationRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .update_organization(id.into_inner(), data.into_inner())
        .await
    {
        Ok(org) => HttpResponse::Ok().json(OrganizationResponse::from(org)),
        Err(e) => e,
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/organizations/{id}",
    params(
        ("id" = i32, Path, description = "Organization ID")
    ),
    responses(
        (status = 204, description = "Organization deleted"),
        (status = 404, description = "Organization not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[delete("/{id}")]
pub async fn delete_organization(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller.delete_organization(id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => e,
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/organizations/{id}",
    params(
        ("id" = i32, Path, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "Organization details", body = OrganizationResponse),
        (status = 404, description = "Organization not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[get("/{id}")]
pub async fn get_organization(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller.get_organization(id.into_inner()).await {
        Ok(org) => HttpResponse::Ok().json(OrganizationResponse::from(org)),
        Err(e) => e,
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/organizations",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of organizations", body = OrganizationListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[get("")]
pub async fn list_organizations(
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller.list_organizations(&params).await {
        Ok((orgs, total)) => {
            let response = OrganizationListResponse {
                organizations: orgs
                    .into_iter()
                    .map(|org| OrganizationResponse::from(org))
                    .collect(),
                total: total as usize,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => e,
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations/{org_id}/configs",
    params(
        ("org_id" = i32, Path, description = "Organization ID")
    ),
    request_body = CreateOrganizationConfigRequest,
    responses(
        (status = 201, description = "Configuration created", body = OrganizationConfigResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Organization not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[post("")]
pub async fn create_config(
    org_id: web::Path<i32>,
    data: web::Json<CreateOrganizationConfigRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    println!("Creating config for org_id");
    let controller = OrganizationController::new(pool);
    match controller
        .create_config(org_id.into_inner(), data.into_inner())
        .await
    {
        Ok(config) => HttpResponse::Created().json(OrganizationConfigResponse::from(config)),
        Err(e) => e,
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/organizations/{org_id}/configs/{config_id}",
    params(
        ("org_id" = i32, Path, description = "Organization ID"),
        ("config_id" = i32, Path, description = "Configuration ID")
    ),
    request_body = UpdateOrganizationConfigRequest,
    responses(
        (status = 200, description = "Configuration updated", body = OrganizationConfigResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Configuration not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[put("/{config_id}")]
pub async fn update_config(
    path: web::Path<(i32, i32)>,
    data: web::Json<UpdateOrganizationConfigRequest>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    let (org_id, config_id) = path.into_inner();
    match controller
        .update_config(org_id, config_id, data.into_inner())
        .await
    {
        Ok(config) => HttpResponse::Ok().json(OrganizationConfigResponse::from(config)),
        Err(e) => e,
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/organizations/{org_id}/configs/{config_id}",
    params(
        ("org_id" = i32, Path, description = "Organization ID"),
        ("config_id" = i32, Path, description = "Configuration ID")
    ),
    responses(
        (status = 204, description = "Configuration deleted"),
        (status = 404, description = "Configuration not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[delete("/{config_id}")]
pub async fn delete_config(
    path: web::Path<(i32, i32)>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    let (org_id, config_id) = path.into_inner();
    match controller.delete_config(org_id, config_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => e,
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/organizations/{org_id}/configs/{config_id}",
    params(
        ("org_id" = i32, Path, description = "Organization ID"),
        ("config_id" = i32, Path, description = "Configuration ID")
    ),
    responses(
        (status = 200, description = "Configuration details", body = OrganizationConfigResponse),
        (status = 404, description = "Configuration not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[get("/{config_id}")]
pub async fn get_config(path: web::Path<(i32, i32)>, pool: web::Data<AppState>) -> impl Responder {
    let controller = OrganizationController::new(pool);
    let (org_id, config_id) = path.into_inner();
    match controller.get_config(org_id, config_id).await {
        Ok(config) => HttpResponse::Ok().json(OrganizationConfigResponse::from(config)),
        Err(e) => e,
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/organizations/{org_id}/configs",
    params(
        ("org_id" = i32, Path, description = "Organization ID"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of configurations", body = OrganizationConfigListResponse),
        (status = 404, description = "Organization not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[get("")]
pub async fn list_configs(
    org_id: web::Path<i32>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller.list_configs(org_id.into_inner(), &params).await {
        Ok((configs, total)) => {
            let response = OrganizationConfigListResponse {
                configs: configs
                    .into_iter()
                    .map(|config| OrganizationConfigResponse::from(config))
                    .collect(),
                total: total as usize,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => e,
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations/batch",
    request_body = Vec<CreateOrganizationRequest>,
    responses(
        (status = 201, description = "Organizations created", body = Vec<OrganizationResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[post("/batch")]
pub async fn batch_create_organizations(
    data: web::Json<Vec<CreateOrganizationRequest>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .batch_create_organizations(data.into_inner())
        .await
    {
        Ok(orgs) => HttpResponse::Created().json(
            orgs.into_iter()
                .map(OrganizationResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/organizations/batch",
    request_body = Vec<(i32, UpdateOrganizationRequest)>,
    responses(
        (status = 200, description = "Organizations updated", body = Vec<OrganizationResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "One or more organizations not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[put("/batch")]
pub async fn batch_update_organizations(
    data: web::Json<Vec<(i32, UpdateOrganizationRequest)>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .batch_update_organizations(data.into_inner())
        .await
    {
        Ok(orgs) => HttpResponse::Ok().json(
            orgs.into_iter()
                .map(OrganizationResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/organizations/batch",
    request_body = Vec<i32>,
    responses(
        (status = 200, description = "Organizations deleted", body = usize),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[delete("/batch")]
pub async fn batch_delete_organizations(
    ids: web::Json<Vec<i32>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .batch_delete_organizations(ids.into_inner())
        .await
    {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/organizations/search",
    params(
        ("name" = String, Query, description = "Name pattern to search for"),
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u32>, Query, description = "Items per page (default: 10)")
    ),
    responses(
        (status = 200, description = "Organizations found", body = OrganizationListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[get("/search")]
pub async fn find_organizations_by_name(
    query: web::Query<SearchNameQuery>,
    params: web::Query<PaginationParams>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .find_organizations_by_name(&query.name, &params.into_inner())
        .await
    {
        Ok((orgs, total)) => HttpResponse::Ok().json(OrganizationListResponse {
            organizations: orgs.into_iter().map(OrganizationResponse::from).collect(),
            total: total as usize,
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations/get-by-ids",
    request_body = Vec<i32>,
    responses(
        (status = 200, description = "Organizations retrieved", body = Vec<OrganizationResponse>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[post("/get-by-ids")]
pub async fn get_organizations_by_ids(
    ids: web::Json<Vec<i32>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .get_organizations_by_ids(ids.into_inner())
        .await
    {
        Ok(orgs) => HttpResponse::Ok().json(
            orgs.into_iter()
                .map(OrganizationResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/organizations/{id}/exists",
    params(
        ("id" = i32, Path, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "Organization existence check result", body = bool),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[get("/{id}/exists")]
pub async fn organization_exists(id: web::Path<i32>, pool: web::Data<AppState>) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .organization_exists(id.into_inner())
        .await
    {
        Ok(exists) => HttpResponse::Ok().json(exists),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations/{org_id}/configs/batch",
    params(("org_id" = i32, Path, description = "Organization ID")),
    request_body = Vec<CreateOrganizationConfigRequest>,
    responses(
        (status = 201, description = "Configurations created", body = Vec<OrganizationConfigResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[post("/batch")]
pub async fn batch_create_configs(
    org_id: web::Path<i32>,
    data: web::Json<Vec<CreateOrganizationConfigRequest>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .batch_create_configs(org_id.into_inner(), data.into_inner())
        .await
    {
        Ok(configs) => HttpResponse::Created().json(
            configs
                .into_iter()
                .map(OrganizationConfigResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/organizations/{org_id}/configs/batch",
    params(("org_id" = i32, Path, description = "Organization ID")),
    request_body = Vec<(i32, UpdateOrganizationConfigRequest)>,
    responses(
        (status = 200, description = "Configurations updated", body = Vec<OrganizationConfigResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[put("/batch")]
pub async fn batch_update_configs(
    org_id: web::Path<i32>,
    data: web::Json<Vec<(i32, UpdateOrganizationConfigRequest)>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .batch_update_configs(org_id.into_inner(), data.into_inner())
        .await
    {
        Ok(configs) => HttpResponse::Ok().json(
            configs
                .into_iter()
                .map(OrganizationConfigResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/organizations/{org_id}/configs/batch",
    params(("org_id" = i32, Path, description = "Organization ID")),
    request_body = Vec<i32>,
    responses(
        (status = 200, description = "Configurations deleted", body = usize),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[delete("/batch")]
pub async fn batch_delete_configs(
    org_id: web::Path<i32>,
    config_ids: web::Json<Vec<i32>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .batch_delete_configs(org_id.into_inner(), config_ids.into_inner())
        .await
    {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/organizations/{org_id}/configs/search",
    params(
        ("org_id" = i32, Path, description = "Organization ID"),
        ("search_key" = String, Query, description = "Key pattern to search for")
    ),
    responses(
        (status = 200, description = "Configurations found", body = Vec<OrganizationConfigResponse>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[get("/search")]
pub async fn find_configs_by_key(
    org_id: web::Path<i32>,
    query: web::Query<SearchKeyQuery>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .find_configs_by_key(org_id.into_inner(), &query.search_key)
        .await
    {
        Ok(configs) => HttpResponse::Ok().json(
            configs
                .into_iter()
                .map(OrganizationConfigResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations/{org_id}/configs/get-by-ids",
    params(("org_id" = i32, Path, description = "Organization ID")),
    request_body = Vec<i32>,
    responses(
        (status = 200, description = "Configurations retrieved", body = Vec<OrganizationConfigResponse>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Organizations"
)]
#[post("/get-by-ids")]
pub async fn get_configs_by_ids(
    org_id: web::Path<i32>,
    config_ids: web::Json<Vec<i32>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let controller = OrganizationController::new(pool);
    match controller
        .service
        .get_configs_by_ids(org_id.into_inner(), config_ids.into_inner())
        .await
    {
        Ok(configs) => HttpResponse::Ok().json(
            configs
                .into_iter()
                .map(OrganizationConfigResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => e.error_response(),
    }
}
