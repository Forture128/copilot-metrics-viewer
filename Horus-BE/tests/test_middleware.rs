use actix_web::http::StatusCode;
use actix_web::test;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};

// Import test helpers
#[path = "helpers/test_helpers.rs"]
mod test_helpers;
use test_helpers::{setup_test_database, setup_test_environment};

// JWT Claims structure for authentication
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,  // User ID
    pub org: i32,     // Organization ID
    pub role: String, // User role
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at time
}

// Helper function to generate a valid JWT token
fn generate_token(
    user_id: i32,
    org_id: i32,
    role: &str,
    jwt_secret: &str,
    expires_in_seconds: u64,
) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: user_id.to_string(),
        org: org_id,
        role: role.to_string(),
        iat: now as usize,
        exp: (now + expires_in_seconds) as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .unwrap()
}

#[actix_web::test]
async fn test_auth_me_with_custom_token() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test database using horus_test
    match setup_test_database().await {
        Ok(_) => println!("Test database setup successful"),
        Err(e) => eprintln!("Test database setup error: {}", e),
    }

    // Set up test environment
    let (app, _) = setup_test_environment().await;

    // Create a test token with custom user_id and org_id
    let test_user_id = 42;
    let test_org_id = 99;
    let jwt_secret = "test_secret"; // Should match the one used in test config

    let token = generate_token(test_user_id, test_org_id, "super_admin", jwt_secret, 3600);

    // Create request with the custom token
    let req = test::TestRequest::get()
        .uri("/api/v1/auth/me")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    // Execute the request
    let resp = test::call_service(&app, req).await;

    // Check if we get a success response (this assumes the jwt_secret in the test environment
    // matches our jwt_secret variable - if not, you'll get a 401)
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Expected OK response from /auth/me, got {}",
        resp.status()
    );

    // Read the response body
    let body = test::read_body(resp).await;

    // Parse the JSON response
    let response: serde_json::Value = serde_json::from_slice(&body)?;

    // Assert that the user_id and organization_id match our test values
    assert_eq!(
        response["user_id"].as_i64().unwrap_or(0),
        test_user_id as i64,
        "Expected user_id to be {}, got {}",
        test_user_id,
        response["user_id"]
    );

    assert_eq!(
        response["organization_id"].as_i64().unwrap_or(0),
        test_org_id as i64,
        "Expected organization_id to be {}, got {}",
        test_org_id,
        response["organization_id"]
    );

    Ok(())
}

#[actix_web::test]
async fn test_public_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test database using horus_test
    match setup_test_database().await {
        Ok(_) => println!("Test database setup successful"),
        Err(e) => eprintln!("Test database setup error: {}", e),
    }

    // Set up test environment
    let (app, _) = setup_test_environment().await;

    // Test a public endpoint
    let req = test::TestRequest::get()
        .uri("/test-public-endpoint")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check that we can access public endpoint without a token
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Expected OK response from public endpoint, got {}",
        resp.status()
    );

    Ok(())
}

#[actix_web::test]
async fn test_protected_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test database using horus_test
    match setup_test_database().await {
        Ok(_) => println!("Test database setup successful"),
        Err(e) => eprintln!("Test database setup error: {}", e),
    }

    // Set up test environment
    let (app, token) = setup_test_environment().await;

    // Try accessing a protected endpoint without a token
    let req = test::TestRequest::get().uri("/test-endpoint").to_request();

    let resp = test::call_service(&app, req).await;

    // Should get Unauthorized
    assert_eq!(
        resp.status(),
        StatusCode::UNAUTHORIZED,
        "Expected Unauthorized response without token, got {}",
        resp.status()
    );

    // Now try with a valid token
    let req = test::TestRequest::get()
        .uri("/test-endpoint")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should get OK
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Expected OK response with token, got {}",
        resp.status()
    );

    Ok(())
}

#[actix_web::test]
async fn test_user_data_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test database using horus_test
    match setup_test_database().await {
        Ok(_) => println!("Test database setup successful"),
        Err(e) => eprintln!("Test database setup error: {}", e),
    }

    // Set up test environment
    let (app, token) = setup_test_environment().await;

    // Access an endpoint that returns user data from request extensions
    let req = test::TestRequest::get()
        .uri("/test-user-data")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should get OK
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Expected OK response from user data endpoint, got {}",
        resp.status()
    );

    // Read the response body
    let body = test::read_body(resp).await;

    // Parse the JSON response
    let response: serde_json::Value = serde_json::from_slice(&body)?;

    // Verify user data is present
    assert!(
        response["user_id"].as_i64().unwrap_or(0) > 0,
        "Expected user_id to be non-zero, got {}",
        response["user_id"]
    );

    assert!(
        response["organization_id"].as_i64().unwrap_or(0) > 0,
        "Expected organization_id to be non-zero, got {}",
        response["organization_id"]
    );

    Ok(())
}
