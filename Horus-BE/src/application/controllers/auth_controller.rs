use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use tracing::debug;

use crate::application::middleware::auth::{OrganizationId, UserId, UserRole};
use crate::application::AppState;
use crate::domain::auth::{LoginRequest, UserInfoResponse};

/// Get current user information
/// This endpoint retrieves the current user's ID and organization ID from the request extensions
/// It's useful for testing the auth middleware
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "User information retrieved", body = UserInfoResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Auth"
)]
pub async fn auth_me(req: HttpRequest) -> impl Responder {
    // Extract user ID from request extensions
    let user_id = req.extensions().get::<UserId>().map(|id| id.0).unwrap_or(0);

    // Extract organization ID from request extensions
    let organization_id = req
        .extensions()
        .get::<OrganizationId>()
        .map(|id| id.0)
        .unwrap_or(0);

    let role = req
        .extensions()
        .get::<UserRole>()
        .map(|role| role.0.clone())
        .unwrap_or("".to_string());

    debug!(
        "User info request: user_id={}, org_id={}, role={}",
        user_id, organization_id, role
    );

    // If user_id is 0, the user is not authenticated properly
    if user_id == 0 {
        return HttpResponse::Unauthorized().json(crate::domain::auth::AuthErrorResponse {
            error: "Not authenticated".to_string(),
        });
    }

    HttpResponse::Ok().json(UserInfoResponse {
        user_id,
        organization_id,
        role,
    })
}

/// Login endpoint
/// Authenticates a user and generates a JWT token
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful. Copy the access_token value and use it as your Bearer token for other API calls.", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Auth"
)]
pub async fn auth_login(
    login_req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    // Create auth service
    let auth_service = app_state.auth_service_factory.create_service();

    // Attempt to login
    match auth_service.login(login_req.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => error.into(),
    }
}
