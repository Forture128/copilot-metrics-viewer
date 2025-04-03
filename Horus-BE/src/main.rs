#![allow(non_snake_case)]
// main.rs

// Import the necessary modules
mod application;
mod common;
mod config;
mod constants;
mod domain;
mod infrastructure;
mod schema;

use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use application::openapi::ApiDoc;
use application::AppState;
// Add middleware imports
use application::middleware::{AuthMiddleware, LoggingMiddleware, RbacMiddleware};
use common::errors::AppError;
use config::AppConfig;
use config::RunMode;
use dotenvy::dotenv;
use infrastructure::{db::create_pool, github_sdk::client::GitHubSdk, logging, redis::Redis};
use tracing::{self, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    logging::init_logging();

    // Load environment variables
    dotenv().ok();

    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");
    let server_address = config.server.address.clone();
    let num_workers = config.server.workers;
    let cors_origin = config.server.cors_origin.clone();

    info!("Starting server at: http://{}", server_address);
    info!("Number of workers: {}", num_workers);

    // Initialize database pool
    let db_pool = create_pool(
        &config.database.url,
        config.database.max_connections,
        config.database.timeout_seconds,
    )
    .await
    .expect("Failed to create database pool");
    info!("Database connected with url: {}", config.database.url);

    // Initialize Redis
    let redis = Redis::new(&config.redis).expect("Failed to connect to Redis");
    info!("Redis connected with url: {}", config.redis.url);

    // Initialize GitHub SDK
    tracing::debug!("Initializing GitHub SDK");
    let github = match GitHubSdk::from_env() {
        Ok(sdk) => {
            if let Some(token) = sdk.config.token.as_ref() {
                tracing::info!("GitHub SDK initialized with token: {}...", &token[..7]);
            } else {
                tracing::error!("GitHub SDK initialized without token!");
            }
            sdk
        }
        Err(e) => {
            tracing::error!("Failed to initialize GitHub SDK: {:#?}", e);
            return Err(e);
        }
    };
    tracing::debug!("GitHub SDK initialized successfully");

    // Create app state
    let state = web::Data::new(AppState::new(
        config.clone(),
        db_pool.clone(),
        redis,
        github,
    ));

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS - more permissive for development
        let cors = if matches!(config.env, RunMode::Development) {
            // Allow all origins in development mode
            Cors::permissive()
        } else {
            // In production mode, use strict CORS
            Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    "Authorization",
                    "Content-Type",
                    "X-Requested-With",
                    "Accept",
                    "Origin",
                ])
                .expose_headers(vec!["content-length", "X-Pagination"])
                .max_age(3600)
                .supports_credentials()
        };
        // Print the CORS settings
        println!("CORS settings: {:?}", cors);
        App::new()
            // Default logging middleware
            .wrap(middleware::Logger::default())
            // Custom logging middleware
            .wrap(LoggingMiddleware::new())
            // RBAC middleware
            .wrap(RbacMiddleware::new())
            // Authentication middleware
            .wrap(AuthMiddleware::new(config.server.jwt_secret.clone()))
            // CORS middleware
            .wrap(cors)
            .app_data(state.clone())
            .app_data(web::Data::new(db_pool.clone()))
            .service(index)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .configure(application::routes::config_routes)
    })
    .bind(server_address)
    .map_err(|e| AppError::ServerError(e.to_string()))?
    .workers(num_workers)
    .run()
    .await
    .map_err(|e| AppError::ServerError(e.to_string()))?;

    Ok(())
}
