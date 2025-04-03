use actix_http;
use actix_web::{dev, test, web, App};
use diesel::sql_query;
use diesel::Connection as DieselConnection;
use diesel::PgConnection;
use diesel::QueryableByName;
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use horus_be::application::AppState;
use horus_be::config::AppConfig;
use horus_be::domain::auth::AuthServiceFactory;
use horus_be::infrastructure::db::create_pool;
use horus_be::infrastructure::github_sdk::client::GitHubSdk;
use horus_be::infrastructure::redis::Redis;
use std::env;
use std::time::Duration;

// Embed migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn create_test_app_state() -> AppState {
    // Get database URL from environment variable or use a default test database
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/horus_test".to_string());

    // Create test configuration
    let config = horus_be::config::AppConfig {
        env: horus_be::config::RunMode::Test,
        server: horus_be::config::ServerConfig {
            address: "127.0.0.1:8081".to_string(),
            workers: 1,
            cors_origin: "http://localhost:3000".to_string(),
            jwt_secret: "test_secret".to_string(),
            token_expiry: 3600,
        },
        database: horus_be::config::DatabaseConfig {
            url: "postgres://postgres:postgres@localhost:5432/horus_test".to_string(),
            max_connections: 5,
            timeout_seconds: 5,
        },
        redis: horus_be::config::RedisConfig {
            url: "redis://localhost:6379/0".to_string(),
            timeout_seconds: 5,
        },
    };

    // Initialize database pool
    let db_pool = create_pool(
        &database_url,
        config.database.max_connections,
        config.database.timeout_seconds,
    )
    .await
    .expect("Failed to init DB pool");

    // Check database health using a simpler approach
    match db_pool.get().await {
        Ok(_) => println!("Database connection successful"),
        Err(e) => println!("Warning: Failed to get database connection: {}", e),
    }

    // Initialize Redis
    let redis = Redis::new(&config.redis).expect("Failed to init Redis pool");

    // Initialize GitHub SDK
    let github = GitHubSdk::builder()
        .with_base_url("https://api.github.com")
        .with_timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to init GitHub SDK");

    // Check GitHub SDK (just verify it was initialized)
    println!("GitHub SDK initialized successfully");

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

// Helper function to setup test database - run migrations and prepare data
pub async fn setup_test_database() -> Result<(), Box<dyn std::error::Error>> {
    // Get the database URL for admin connection (to postgres database)
    let admin_database_url = env::var("ADMIN_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());

    // Get the test database URL
    let test_database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/horus_test".to_string());

    // Extract the database name from the test database URL
    let db_name = test_database_url
        .split('/')
        .last()
        .unwrap_or("horus_test")
        .split('?')
        .next()
        .unwrap_or("horus_test");

    println!("Setting up test database: {}", db_name);

    // Try to create the test database using SQL
    let create_db_sql = format!(
        "CREATE DATABASE {} WITH OWNER = postgres ENCODING = 'UTF8' CONNECTION LIMIT = -1;",
        db_name
    );

    // We need to use tokio to run synchronous operations in an async context
    let admin_url = admin_database_url.clone();
    let test_url = test_database_url.clone();
    let create_sql = create_db_sql.clone();

    // Create a tokio task to run synchronous DB operations
    tokio::task::spawn_blocking(move || {
        // Connect to the admin database (synchronous)
        match PgConnection::establish(&test_url) {
            Ok(mut conn) => {
                // Try to create the test database (synchronous)
                // Use fully qualified syntax to avoid ambiguity
                match RunQueryDsl::execute(sql_query(&create_sql), &mut conn) {
                    Ok(_) => println!("Test database created successfully"),
                    Err(e) => {
                        // If the database already exists, that's fine
                        if e.to_string().contains("already exists") {
                            println!("Test database already exists");
                        } else {
                            eprintln!("Error creating test database: {}", e);
                            // Continue anyway, as the database might already exist
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error connecting to admin database: {}", e);
                // Continue anyway, as the test database might already exist
            }
        }
    })
    .await?;

    // Set DATABASE_URL for diesel migrations
    env::set_var("DATABASE_URL", &test_database_url);

    // Create a tokio task to run migrations
    let test_url = test_database_url.clone();
    tokio::task::spawn_blocking(move || {
        // Connect to the test database (synchronous)
        match PgConnection::establish(&test_url) {
            Ok(mut conn) => {
                // Run migrations
                match conn.run_pending_migrations(MIGRATIONS) {
                    Ok(migrations) => {
                        println!("Successfully ran {} migrations", migrations.len());
                    }
                    Err(e) => {
                        eprintln!("Error running migrations: {}", e);
                        // Continue anyway, as some tests might still work
                    }
                }
            }
            Err(e) => {
                eprintln!("Error connecting to test database for migrations: {}", e);
                // Continue anyway, as some tests might still work
            }
        }
    })
    .await?;

    Ok(())
}

pub async fn teardown_test_database() -> Result<(), Box<dyn std::error::Error>> {
    // Get the test database URL
    let test_database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/horus_test".to_string());

    println!("Cleaning test database...");

    // Instead of dropping the database, truncate all tables using synchronous operations
    // Use tokio to run synchronous operations in an async context
    let test_url = test_database_url.clone();

    tokio::task::spawn_blocking(move || {
        // Connect to the test database (synchronous)
        match PgConnection::establish(&test_url) {
            Ok(mut conn) => {
                // Get a list of all tables in the public schema
                let tables_query = "
                    SELECT tablename FROM pg_tables 
                    WHERE schemaname = 'public' AND 
                    tablename != '__diesel_schema_migrations'";

                // Define a struct to hold the table name result
                #[derive(QueryableByName)]
                struct TableName {
                    #[diesel(sql_type = diesel::sql_types::Text)]
                    tablename: String,
                }

                // Use fully qualified syntax to avoid ambiguity
                match RunQueryDsl::load::<TableName>(sql_query(tables_query), &mut conn) {
                    Ok(tables) => {
                        // Disable triggers and truncate all tables
                        for table in tables {
                            let truncate_query =
                                format!("TRUNCATE TABLE {} CASCADE", table.tablename);
                            // Use fully qualified syntax to avoid ambiguity
                            if let Err(e) =
                                RunQueryDsl::execute(sql_query(&truncate_query), &mut conn)
                            {
                                eprintln!("Error truncating table {}: {}", table.tablename, e);
                            }
                        }
                        println!("All tables truncated");
                    }
                    Err(e) => {
                        eprintln!("Error getting table list: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error connecting to test database for cleanup: {}", e);
            }
        }
    })
    .await?;

    Ok(())
}

pub async fn create_test_app() -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = dev::ServiceResponse,
    Error = actix_web::Error,
> {
    // Setup test database before creating app
    if let Err(e) = setup_test_database().await {
        eprintln!("Error setting up test database: {}", e);
    }

    // Create app state
    let app_state = create_test_app_state().await;

    // Create test app with routes
    test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .configure(horus_be::application::routes::config_routes),
    )
    .await
}

/// Creates a test admin user for authentication testing
/// Returns (username, password, token) for use in tests
pub async fn create_test_admin_user(app_state: &AppState) -> (String, String, String) {
    // Connect to the database
    let mut conn = app_state
        .db_pool
        .get()
        .await
        .expect("Failed to get DB connection");

    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    // Only import diesel_async::RunQueryDsl, not diesel::RunQueryDsl
    use diesel::prelude::*;
    use horus_be::schema::{organizations, roles, user_roles, users};

    // Test admin credentials - using fixed values for simplicity
    let username = "testadmin";
    let email = "testadmin@example.com";
    let password = "testadmin123";

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    // 1. Check if System organization exists
    let system_org_exists = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            organizations::table.filter(organizations::name.eq("System")),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if System organization exists");

    // Create or get System organization
    let org_id = if system_org_exists {
        // Get existing organization ID
        diesel_async::RunQueryDsl::get_result::<i32>(
            organizations::table
                .filter(organizations::name.eq("System"))
                .select(organizations::id),
            &mut conn,
        )
        .await
        .expect("Failed to get System organization ID")
    } else {
        // Create new organization
        diesel_async::RunQueryDsl::get_result::<i32>(
            diesel::insert_into(organizations::table)
                .values(organizations::name.eq("System"))
                .returning(organizations::id),
            &mut conn,
        )
        .await
        .expect("Failed to create System organization")
    };

    // 2. Check if super_admin role exists for this organization
    let role_exists = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            roles::table
                .filter(roles::name.eq("super_admin"))
                .filter(roles::organization_id.eq(org_id)),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if super_admin role exists");

    // Create or get super_admin role
    let role_id = if role_exists {
        // Get existing role ID
        diesel_async::RunQueryDsl::get_result::<i32>(
            roles::table
                .filter(roles::name.eq("super_admin"))
                .filter(roles::organization_id.eq(org_id))
                .select(roles::id),
            &mut conn,
        )
        .await
        .expect("Failed to get super_admin role ID")
    } else {
        // Create new role
        diesel_async::RunQueryDsl::get_result::<i32>(
            diesel::insert_into(roles::table)
                .values((
                    roles::name.eq("super_admin"),
                    roles::description.eq("Super Administrator with full system access"),
                    roles::organization_id.eq(org_id),
                ))
                .returning(roles::id),
            &mut conn,
        )
        .await
        .expect("Failed to create super_admin role")
    };

    // 3. Check if user exists
    let user_exists = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            users::table.filter(users::username.eq(username)),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if user exists");

    // Create or update user
    let user_id = if user_exists {
        // Get existing user ID
        let user_id = diesel_async::RunQueryDsl::get_result::<i32>(
            users::table
                .filter(users::username.eq(username))
                .select(users::id),
            &mut conn,
        )
        .await
        .expect("Failed to get user ID");

        // Update user
        diesel_async::RunQueryDsl::execute(
            diesel::update(users::table.filter(users::id.eq(user_id))).set((
                users::email.eq(email),
                users::password.eq(&password_hash),
                users::organization_id.eq(org_id),
            )),
            &mut conn,
        )
        .await
        .expect("Failed to update user");

        user_id
    } else {
        // Create new user
        diesel_async::RunQueryDsl::get_result::<i32>(
            diesel::insert_into(users::table)
                .values((
                    users::username.eq(username),
                    users::email.eq(email),
                    users::password.eq(&password_hash),
                    users::organization_id.eq(org_id),
                ))
                .returning(users::id),
            &mut conn,
        )
        .await
        .expect("Failed to create admin user")
    };

    // 4. Check if role is already assigned
    let role_assigned = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            user_roles::table
                .filter(user_roles::user_id.eq(user_id))
                .filter(user_roles::role_id.eq(role_id)),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if role is assigned");

    // Assign role if not already assigned
    if !role_assigned {
        diesel_async::RunQueryDsl::execute(
            diesel::insert_into(user_roles::table).values((
                user_roles::user_id.eq(user_id),
                user_roles::role_id.eq(role_id),
                user_roles::organization_id.eq(org_id),
            )),
            &mut conn,
        )
        .await
        .expect("Failed to assign super_admin role to admin");
    }

    // Generate JWT token for the test admin
    let auth_service = app_state.auth_service_factory.create_service();
    let login_result = auth_service
        .login(horus_be::domain::auth::LoginRequest {
            username_or_email: username.to_string(),
            password: password.to_string(),
        })
        .await
        .expect("Failed to login test admin user");

    (
        username.to_string(),
        password.to_string(),
        login_result.access_token,
    )
}

/// Creates an authenticated request for testing API endpoints that require authentication
/// Usage: let req = create_authenticated_request("/api/v1/some/endpoint", token);
pub fn create_authenticated_request(url: &str, token: &str) -> actix_web::test::TestRequest {
    use actix_web::{http::header, test};

    test::TestRequest::get()
        .uri(url)
        .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
}

/// Creates an authenticated request with a specific HTTP method
/// Usage: let req = create_authenticated_request_with_method(Method::POST, "/api/v1/some/endpoint", token);
pub fn create_authenticated_request_with_method(
    method: actix_web::http::Method,
    url: &str,
    token: &str,
) -> actix_web::test::TestRequest {
    use actix_web::{http::header, test};

    test::TestRequest::with_uri(url)
        .method(method)
        .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
}

/// Creates an authenticated request with body for testing API endpoints that require authentication and request body
/// Usage: let req = create_authenticated_request_with_body("/api/v1/some/endpoint", token, json_body);
pub fn create_authenticated_request_with_body<T: serde::Serialize>(
    url: &str,
    token: &str,
    body: &T,
) -> actix_web::test::TestRequest {
    use actix_web::{http::header, test};

    test::TestRequest::post()
        .uri(url)
        .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .set_json(body)
}

/// Sets up a complete test environment with admin user in one call
/// Returns (app, token) for use in tests
pub async fn setup_test_environment() -> (
    impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
    >,
    String,
) {
    // Setup test database
    if let Err(e) = setup_test_database().await {
        eprintln!("Error setting up test database: {}", e);
    }

    // Create app state
    let app_state = create_test_app_state().await;

    // Create test admin user
    let (_, _, token) = create_test_admin_user(&app_state).await;

    // Get JWT secret from config
    let jwt_secret = app_state.config.server.jwt_secret.clone();

    // Create test app with middleware (similar to main.rs)
    let app = actix_web::test::init_service(
        actix_web::App::new()
            // Add the Auth middleware
            .wrap(horus_be::application::middleware::AuthMiddleware::new(
                jwt_secret,
            ))
            // Add the RBAC middleware
            .wrap(horus_be::application::middleware::RbacMiddleware::new())
            .app_data(actix_web::web::Data::new(app_state))
            .configure(horus_be::application::routes::config_routes),
    )
    .await;

    (app, token)
}
