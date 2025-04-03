use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::rc::Rc;
use tracing::{error, warn};

use crate::common::types::Claims;
use crate::domain::auth::AuthErrorResponse;

/// User ID extracted from JWT token
#[derive(Clone, Debug)]
pub struct UserId(pub i32);

/// Organization ID extracted from JWT token
#[derive(Clone, Debug)]
pub struct OrganizationId(pub i32);

/// User role extracted from JWT token
#[derive(Clone, Debug)]
pub struct UserRole(pub String);

/// Middleware for JWT authentication
pub struct AuthMiddleware {
    jwt_secret: String,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = futures::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let jwt_secret = self.jwt_secret.clone();

        // Skip authentication for certain paths
        let path = req.path().to_string();

        // Skip authentication for OPTIONS requests (preflight CORS requests)
        if req.method() == actix_web::http::Method::OPTIONS {
            let fut = service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }

        // List of paths that don't require authentication
        let public_paths = vec![
            "/",
            "/health",
            "/api-docs/openapi.json",
            "/api/v1/auth/login",
            "/users",
            "/api/v1/auth/token-helper",
        ];

        // Check if path starts with /swagger-ui or is a public path
        if path.starts_with("/swagger-ui") || public_paths.contains(&path.as_str()) {
            let fut = service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }

        // Check authorization header
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Remove "Bearer " prefix
                    println!("Token: {}", token);
                    // Decode and validate JWT token
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(jwt_secret.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(token_data) => {
                            // Extract user ID and organization ID from claims
                            if let Ok(user_id) = token_data.claims.sub.parse::<i32>() {
                                let organization_id = token_data.claims.org;
                                let role = token_data.claims.role;
                                // Store user ID and organization ID in request extensions
                                req.extensions_mut().insert(UserId(user_id));
                                req.extensions_mut().insert(OrganizationId(organization_id));
                                req.extensions_mut().insert(UserRole(role));

                                let fut = service.call(req);
                                return Box::pin(async move {
                                    let res = fut.await?;
                                    Ok(res.map_into_left_body())
                                });
                            } else {
                                warn!("Invalid user ID in token: {}", token_data.claims.sub);
                            }
                        }
                        Err(e) => {
                            error!("Token validation error: {}", e);
                        }
                    }
                }
            }
        }

        // Return unauthorized response if authentication fails
        let response = HttpResponse::Unauthorized().json(AuthErrorResponse {
            error: "Invalid or missing token".to_string(),
        });

        Box::pin(ready(Ok(ServiceResponse::new(
            req.into_parts().0,
            response.map_into_right_body(),
        ))))
    }
}
