use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::application::middleware::auth::{OrganizationId, UserId};

pub fn config(cfg: &mut web::ServiceConfig) {
    // Configure test routes
    cfg.service(
        web::scope("/test")
            // A public endpoint that doesn't require authentication
            .route("/public-endpoint", web::get().to(test_public_endpoint))
            // A protected endpoint that requires authentication
            .route("/endpoint", web::get().to(test_protected_endpoint))
            // An endpoint that returns user data from request extensions
            .route("/user-data", web::get().to(test_user_data)),
    );
}

/// Public test endpoint - accessible without authentication
async fn test_public_endpoint() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "This is a public endpoint"
    }))
}

/// Protected test endpoint - requires authentication
async fn test_protected_endpoint() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "This is a protected endpoint"
    }))
}

/// User data test endpoint - extracts and returns user data from request extensions
async fn test_user_data(req: HttpRequest) -> impl Responder {
    // Extract user ID from request extensions
    let user_id = req.extensions().get::<UserId>().map(|id| id.0).unwrap_or(0);

    // Extract organization ID from request extensions
    let organization_id = req
        .extensions()
        .get::<OrganizationId>()
        .map(|id| id.0)
        .unwrap_or(0);

    if user_id == 0 || organization_id == 0 {
        return HttpResponse::Unauthorized().json(json!({
            "error": "Not authenticated"
        }));
    }

    HttpResponse::Ok().json(json!({
        "user_id": user_id,
        "organization_id": organization_id
    }))
}
