use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use chrono::{DateTime, Utc};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;
use tracing::info;

use super::auth::{OrganizationId, UserId};

/// Middleware for request and response logging
pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: Rc<S>,
}

/// Structure to store request information for logging
struct RequestInfo {
    method: String,
    path: String,
    start_time: DateTime<Utc>,
    user_id: Option<i32>,
    organization_id: Option<i32>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        // Record request start time
        let start_time = Utc::now();

        // Get request method and path
        let method = req.method().to_string();
        let path = req.path().to_string();

        // Get user ID and organization ID if available
        let user_id = req.extensions().get::<UserId>().map(|id| id.0);
        let organization_id = req.extensions().get::<OrganizationId>().map(|id| id.0);

        // Create request info for logging
        let req_info = RequestInfo {
            method: method.clone(),
            path: path.clone(),
            start_time,
            user_id,
            organization_id,
        };

        // Log request
        if let Some(user_id) = req_info.user_id {
            info!(
                "Request: {} {} - User: {}, Org: {}",
                req_info.method,
                req_info.path,
                user_id,
                req_info.organization_id.unwrap_or(-1)
            );
        } else {
            info!("Request: {} {}", req_info.method, req_info.path);
        }

        // Process request
        let fut = service.call(req);

        Box::pin(async move {
            // Wait for response
            let res = fut.await?;

            // Calculate request duration
            let duration = Utc::now()
                .signed_duration_since(req_info.start_time)
                .num_milliseconds();

            // Log response
            info!(
                "Response: {} {} - Status: {} - Duration: {}ms",
                req_info.method,
                req_info.path,
                res.status().as_u16(),
                duration
            );

            Ok(res.map_into_left_body())
        })
    }
}
