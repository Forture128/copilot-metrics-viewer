use crate::config::AppConfig;
use crate::domain::auth::AuthServiceFactory;
use crate::infrastructure::{db::DbPool, github_sdk::client::GitHubSdk, redis::Redis};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: DbPool,
    pub redis: Redis,
    pub github: GitHubSdk,
    pub auth_service_factory: AuthServiceFactory,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: DbPool, redis: Redis, github: GitHubSdk) -> Self {
        // Create auth service factory
        let auth_service_factory = AuthServiceFactory::new(
            db_pool.clone(),
            config.server.jwt_secret.clone(),
            3600, // 1 hour token expiry
        );

        Self {
            config,
            db_pool,
            redis,
            github,
            auth_service_factory,
        }
    }
}
