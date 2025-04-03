use horus_be::application::AppState;
use horus_be::common::types::PaginationParams;
use horus_be::config::{AppConfig, DatabaseConfig, RedisConfig, RunMode, ServerConfig};
use horus_be::domain::auth::AuthServiceFactory;
use horus_be::domain::organization::{
    CreateOrganizationConfigRequest, CreateOrganizationRequest, OrganizationService,
    UpdateOrganizationConfigRequest, UpdateOrganizationRequest,
};
use horus_be::infrastructure::database::{
    OrganizationConfigRepositoryImpl, OrganizationRepositoryImpl,
};
use horus_be::infrastructure::db::create_pool;
use horus_be::infrastructure::github_sdk::client::GitHubSdk;
use horus_be::infrastructure::redis::Redis;
use std::time::Duration;

// Import test helpers
#[path = "helpers/test_helpers.rs"]
mod test_helpers;
use test_helpers::setup_test_database;

// Create a local test helper function
async fn create_test_app_state() -> AppState {
    // Set up the test database
    match setup_test_database().await {
        Ok(_) => println!("Test database setup successful"),
        Err(e) => eprintln!("Test database setup error: {}", e),
    }

    // Create a default config for testing
    let config = AppConfig {
        env: RunMode::Test,
        server: ServerConfig {
            address: "127.0.0.1:8080".to_string(),
            workers: 2,
            cors_origin: "*".to_string(),
            jwt_secret: "test_secret".to_string(),
            token_expiry: 3600,
        },
        database: DatabaseConfig {
            url: "postgres://postgres:postgres@localhost:5432/horus_test".to_string(),
            max_connections: 5,
            timeout_seconds: 30,
        },
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
            timeout_seconds: 30,
        },
    };

    // Initialize database pool
    let db_pool = create_pool(
        &config.database.url,
        config.database.max_connections,
        config.database.timeout_seconds,
    )
    .await
    .expect("Failed to init DB pool");

    // Initialize Redis
    let redis = Redis::new(&config.redis).expect("Failed to init Redis pool");

    // Initialize GitHub SDK
    let github = GitHubSdk::builder()
        .with_base_url("https://api.github.com")
        .with_timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to init GitHub SDK");

    // Create auth service factory
    let auth_service_factory = AuthServiceFactory::new(
        db_pool.clone(),
        config.server.jwt_secret.clone(),
        3600, // 1 hour token expiry
    );

    AppState {
        config,
        db_pool,
        redis,
        github,
        auth_service_factory,
    }
}

#[actix_web::test]
async fn test_organization_crud() {
    let app_state = create_test_app_state().await;
    let repository = OrganizationRepositoryImpl::new(app_state.db_pool.clone());
    let config_repository = OrganizationConfigRepositoryImpl::new(app_state.db_pool.clone());
    let service = OrganizationService::new(repository, config_repository, app_state.redis.clone());

    // Create organization
    let create_dto = CreateOrganizationRequest {
        name: "Test Organization".to_string(),
    };
    let org = service.create_organization(create_dto).await.unwrap();
    assert_eq!(org.name, "Test Organization");

    // Update organization
    let update_dto = UpdateOrganizationRequest {
        name: "Updated Organization".to_string(),
    };
    let updated_org = service
        .update_organization(org.id, update_dto)
        .await
        .unwrap();
    assert_eq!(updated_org.name, "Updated Organization");

    // Get organization
    let retrieved_org = service.get_organization(org.id).await.unwrap().unwrap();
    assert_eq!(retrieved_org.id, org.id);
    assert_eq!(retrieved_org.name, "Updated Organization");

    // List organizations
    let params = PaginationParams {
        page: 1,
        per_page: 10,
    };
    let (orgs, total) = service.list_organizations(&params).await.unwrap();
    assert!(total > 0);
    assert!(!orgs.is_empty());

    // Delete organization
    let deleted = service.delete_organization(org.id).await.unwrap();
    assert!(deleted);

    // Verify deletion
    let non_existent = service.get_organization(org.id).await.unwrap();
    assert!(non_existent.is_none());
}

#[actix_web::test]
async fn test_organization_batch_operations() {
    let app_state = create_test_app_state().await;
    let repository = OrganizationRepositoryImpl::new(app_state.db_pool.clone());
    let config_repository = OrganizationConfigRepositoryImpl::new(app_state.db_pool.clone());
    let service = OrganizationService::new(repository, config_repository, app_state.redis.clone());

    // Cleanup any leftover test data from previous runs
    let params = PaginationParams {
        page: 1,
        per_page: 100, // Get a large batch to ensure we clean up everything
    };

    // Find and delete any organizations containing "Batch Org" or "Updated"
    let (orgs_batch, _) = service
        .find_organizations_by_name("Batch Org", &params)
        .await
        .unwrap();

    let (orgs_updated, _) = service
        .find_organizations_by_name("Updated", &params)
        .await
        .unwrap();

    // Combine the IDs and delete them
    let ids_to_delete: Vec<i32> = orgs_batch
        .iter()
        .chain(orgs_updated.iter())
        .map(|org| org.id)
        .collect();

    if !ids_to_delete.is_empty() {
        let _ = service
            .batch_delete_organizations(ids_to_delete)
            .await
            .unwrap();
    }

    // Batch create organizations
    let create_dtos = vec![
        CreateOrganizationRequest {
            name: "Batch Org 1".to_string(),
        },
        CreateOrganizationRequest {
            name: "Batch Org 2".to_string(),
        },
        CreateOrganizationRequest {
            name: "Batch Org 3".to_string(),
        },
    ];
    let orgs = service
        .batch_create_organizations(create_dtos)
        .await
        .unwrap();
    assert_eq!(orgs.len(), 3);
    assert_eq!(orgs[0].name, "Batch Org 1");
    assert_eq!(orgs[1].name, "Batch Org 2");
    assert_eq!(orgs[2].name, "Batch Org 3");

    // Batch update organizations
    let update_dtos = vec![
        (
            orgs[0].id,
            UpdateOrganizationRequest {
                name: "Updated Batch Org 1".to_string(),
            },
        ),
        (
            orgs[1].id,
            UpdateOrganizationRequest {
                name: "Updated Batch Org 2".to_string(),
            },
        ),
    ];
    let updated_orgs = service
        .batch_update_organizations(update_dtos)
        .await
        .unwrap();
    assert_eq!(updated_orgs.len(), 2);
    assert_eq!(updated_orgs[0].name, "Updated Batch Org 1");
    assert_eq!(updated_orgs[1].name, "Updated Batch Org 2");

    // Get organizations by IDs
    let retrieved_orgs = service
        .get_organizations_by_ids(vec![orgs[0].id, orgs[2].id])
        .await
        .unwrap();
    assert_eq!(retrieved_orgs.len(), 2);

    // Find organizations by name
    let params = PaginationParams {
        page: 1,
        per_page: 10,
    };
    let (found_orgs, total) = service
        .find_organizations_by_name("Updated", &params)
        .await
        .unwrap();
    assert_eq!(total, 2);
    assert_eq!(found_orgs.len(), 2);

    // Check organization exists
    let exists = service.organization_exists(orgs[0].id).await.unwrap();
    assert!(exists);

    // Batch delete organizations
    let deleted_count = service
        .batch_delete_organizations(vec![orgs[0].id, orgs[1].id, orgs[2].id])
        .await
        .unwrap();
    assert_eq!(deleted_count, 3);
}

#[actix_web::test]
async fn test_organization_config_crud() {
    let app_state = create_test_app_state().await;
    let repository = OrganizationRepositoryImpl::new(app_state.db_pool.clone());
    let config_repository = OrganizationConfigRepositoryImpl::new(app_state.db_pool.clone());
    let service = OrganizationService::new(repository, config_repository, app_state.redis.clone());

    // Create organization first
    let create_dto = CreateOrganizationRequest {
        name: "Config Test Org".to_string(),
    };
    let org = service.create_organization(create_dto).await.unwrap();

    // Create config
    let config_dto = CreateOrganizationConfigRequest {
        config_key: "test.key".to_string(),
        config_value: "test value".to_string(),
    };
    let config = service.create_config(org.id, config_dto).await.unwrap();
    assert_eq!(config.config_key, "test.key");
    assert_eq!(config.config_value, "test value");

    // Update config
    let update_dto = UpdateOrganizationConfigRequest {
        config_value: "updated value".to_string(),
    };
    let updated_config = service
        .update_config(org.id, config.id, update_dto)
        .await
        .unwrap();
    assert_eq!(updated_config.config_key, "test.key");
    assert_eq!(updated_config.config_value, "updated value");

    // Get config
    let retrieved_config = service.get_config(org.id, config.id).await.unwrap();
    assert_eq!(retrieved_config.id, config.id);
    assert_eq!(retrieved_config.config_key, "test.key");

    // List configs
    let params = PaginationParams {
        page: 1,
        per_page: 10,
    };
    let (configs, total) = service.list_configs(org.id, &params).await.unwrap();
    assert_eq!(total, 1);
    assert_eq!(configs.len(), 1);

    // Delete config
    let deleted = service.delete_config(org.id, config.id).await.unwrap();
    assert!(deleted);

    // Clean up
    let _ = service.delete_organization(org.id).await;
}

#[actix_web::test]
async fn test_organization_config_batch_operations() {
    let app_state = create_test_app_state().await;
    let repository = OrganizationRepositoryImpl::new(app_state.db_pool.clone());
    let config_repository = OrganizationConfigRepositoryImpl::new(app_state.db_pool.clone());
    let service = OrganizationService::new(repository, config_repository, app_state.redis.clone());

    // Create organization first
    let create_dto = CreateOrganizationRequest {
        name: "Config Batch Test Org".to_string(),
    };
    let org = service.create_organization(create_dto).await.unwrap();

    // Batch create configs
    let config_dtos = vec![
        CreateOrganizationConfigRequest {
            config_key: "batch.key1".to_string(),
            config_value: "batch value 1".to_string(),
        },
        CreateOrganizationConfigRequest {
            config_key: "batch.key2".to_string(),
            config_value: "batch value 2".to_string(),
        },
        CreateOrganizationConfigRequest {
            config_key: "batch.key3".to_string(),
            config_value: "batch value 3".to_string(),
        },
    ];
    let configs = service
        .batch_create_configs(org.id, config_dtos)
        .await
        .unwrap();
    assert_eq!(configs.len(), 3);
    assert_eq!(configs[0].config_key, "batch.key1");
    assert_eq!(configs[1].config_key, "batch.key2");
    assert_eq!(configs[2].config_key, "batch.key3");

    // Batch update configs
    let update_dtos = vec![
        (
            configs[0].id,
            UpdateOrganizationConfigRequest {
                config_value: "updated batch value 1".to_string(),
            },
        ),
        (
            configs[1].id,
            UpdateOrganizationConfigRequest {
                config_value: "updated batch value 2".to_string(),
            },
        ),
    ];
    let updated_configs = service
        .batch_update_configs(org.id, update_dtos)
        .await
        .unwrap();
    assert_eq!(updated_configs.len(), 2);
    assert_eq!(updated_configs[0].config_key, "batch.key1");
    assert_eq!(updated_configs[1].config_key, "batch.key2");

    // Find configs by key
    let found_configs = service
        .find_configs_by_key(org.id, "batch.key")
        .await
        .unwrap();
    assert_eq!(found_configs.len(), 3);

    // Get configs by IDs
    let retrieved_configs = service
        .get_configs_by_ids(org.id, vec![configs[0].id, configs[2].id])
        .await
        .unwrap();
    assert_eq!(retrieved_configs.len(), 2);
    assert_eq!(retrieved_configs[0].id, configs[0].id);
    assert_eq!(retrieved_configs[1].id, configs[2].id);

    // Batch delete configs
    let deleted_count = service
        .batch_delete_configs(org.id, vec![configs[0].id, configs[1].id, configs[2].id])
        .await
        .unwrap();
    assert!(
        deleted_count >= 3,
        "Expected at least 3 configs to be deleted, got {}",
        deleted_count
    );

    // Clean up
    let _ = service.delete_organization(org.id).await;
}
