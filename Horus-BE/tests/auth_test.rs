use actix_web::test;
use serde::{Deserialize, Serialize};
use serde_json;

// Import test helpers directly
#[path = "helpers/test_helpers.rs"]
mod test_helpers;
use test_helpers::{create_authenticated_request, setup_test_environment};

// Define a simple response struct for our tests
#[derive(Debug, Serialize, Deserialize)]
struct SimpleUserResponse {
    pub user_id: i32,
    pub organization_id: i32,
}

#[actix_web::test]
async fn test_auth_me_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    // Set up complete test environment with a single call
    let (app, token) = setup_test_environment().await;

    // Create authenticated request using our helper
    let req = create_authenticated_request("/api/v1/auth/me", &token).to_request();

    // Send request and verify response
    let resp = test::call_service(&app, req).await;

    // Check status code first
    assert_eq!(resp.status().as_u16(), 200, "Expected 200 OK response");

    // Deserialize response
    let body = test::read_body(resp).await;
    let user_info: SimpleUserResponse = serde_json::from_slice(&body)?;

    // Verify user info contains valid data - the ID might vary depending on the test environment
    assert_ne!(user_info.user_id, 0, "Expected user_id to be non-zero");
    assert_ne!(
        user_info.organization_id, 0,
        "Expected organization_id to be non-zero"
    );

    Ok(())
}

#[actix_web::test]
async fn test_unauthorized_access() -> Result<(), Box<dyn std::error::Error>> {
    // Set up complete test environment (we'll ignore the token for this test)
    let (app, _) = setup_test_environment().await;

    // Create request without authorization header
    let req = test::TestRequest::get().uri("/api/v1/auth/me").to_request();

    // Send request and verify response is unauthorized
    let resp = test::call_service(&app, req).await;

    // Check that we got a 401 Unauthorized response
    assert_eq!(
        resp.status().as_u16(),
        401,
        "Expected 401 Unauthorized, got {}",
        resp.status().as_u16()
    );

    Ok(())
}
