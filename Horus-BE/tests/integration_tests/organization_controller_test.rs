use crate::helpers::test_helpers::{create_test_app, teardown_test_database};
use actix_http;
use actix_web::test;
use horus_be::domain::organization::{Organization, OrganizationConfig};
use serde_json::json;

const BASE_URL: &str = "http://localhost:8080/api/v1";

/// Helper function to clean up test data
/// Call this at the end of tests to ensure no test data remains
async fn cleanup_test_data(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
) {
    // List all organizations with "Test" or "API" in the name
    let resp = test::TestRequest::get()
        .uri(&format!("{}/organizations/search?name=Test", BASE_URL))
        .send_request(app)
        .await;

    if resp.status().is_success() {
        // Updated to use the new response format which is a map with 'organizations' and 'total' fields
        let response: serde_json::Value = test::read_body_json(resp).await;
        // Create a longer-lived empty vector
        let empty_vec: Vec<serde_json::Value> = Vec::new();
        let orgs_array = response["organizations"].as_array().unwrap_or(&empty_vec);

        // Clone the data into a Vec we can use
        let orgs: Vec<serde_json::Value> = orgs_array.to_vec();

        // Delete each test organization
        for org in orgs {
            let id = org["id"].as_i64().unwrap_or(0);
            let _ = test::TestRequest::delete()
                .uri(&format!("{}/organizations/{}", BASE_URL, id))
                .send_request(app)
                .await;
        }
    }
}

#[actix_web::test]
async fn test_organization_crud_endpoints() {
    // Create test app
    let app = create_test_app().await;

    // Create organization
    let create_req = json!({
        "name": "Test API Organization"
    });
    let resp = test::TestRequest::post()
        .uri(&format!("{}/organizations", BASE_URL))
        .set_json(&create_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 201);
    let org: Organization = test::read_body_json(resp).await;
    assert_eq!(org.name, "Test API Organization");

    // Update organization
    let update_req = json!({
        "name": "Updated API Organization"
    });
    let resp = test::TestRequest::put()
        .uri(&format!("{}/organizations/{}", BASE_URL, org.id))
        .set_json(&update_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let updated_org: Organization = test::read_body_json(resp).await;
    assert_eq!(updated_org.name, "Updated API Organization");

    // Get organization
    let resp = test::TestRequest::get()
        .uri(&format!("{}/organizations/{}", BASE_URL, org.id))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let retrieved_org: Organization = test::read_body_json(resp).await;
    assert_eq!(retrieved_org.id, org.id);

    // List organizations
    let resp = test::TestRequest::get()
        .uri(&format!("{}/organizations?page=1&per_page=10", BASE_URL))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    // Updated to use the new response format
    let response: serde_json::Value = test::read_body_json(resp).await;
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let organizations = response["organizations"].as_array().unwrap_or(&empty_vec);
    let total = response["total"].as_u64().unwrap_or(0);
    assert!(total > 0);
    assert!(!organizations.is_empty());

    // Check organization exists
    let resp = test::TestRequest::get()
        .uri(&format!("{}/organizations/{}/exists", BASE_URL, org.id))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let exists: bool = test::read_body_json(resp).await;
    assert!(exists);

    // Delete organization
    let resp = test::TestRequest::delete()
        .uri(&format!("{}/organizations/{}", BASE_URL, org.id))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 204);

    // Verify deletion
    let resp = test::TestRequest::get()
        .uri(&format!("{}/organizations/{}", BASE_URL, org.id))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 404);

    // Clean up any remaining test data
    cleanup_test_data(&app).await;

    // Tear down the test database after tests complete
    let _ = teardown_test_database().await;
}

#[actix_web::test]
async fn test_organization_batch_endpoints() {
    let app = create_test_app().await;

    // Batch create organizations
    let create_req = json!([
        { "name": "Batch API Org 1" },
        { "name": "Batch API Org 2" },
        { "name": "Batch API Org 3" }
    ]);
    let resp = test::TestRequest::post()
        .uri(&format!("{}/organizations/batch", BASE_URL))
        .set_json(&create_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 201);
    let orgs: Vec<Organization> = test::read_body_json(resp).await;
    assert_eq!(orgs.len(), 3);
    assert_eq!(orgs[0].name, "Batch API Org 1");
    assert_eq!(orgs[1].name, "Batch API Org 2");
    assert_eq!(orgs[2].name, "Batch API Org 3");

    // Batch update organizations
    let update_req = json!([
        [orgs[0].id, { "name": "Updated Batch API Org 1" }],
        [orgs[1].id, { "name": "Updated Batch API Org 2" }]
    ]);
    let resp = test::TestRequest::put()
        .uri(&format!("{}/organizations/batch", BASE_URL))
        .set_json(&update_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let updated_orgs: Vec<Organization> = test::read_body_json(resp).await;
    assert_eq!(updated_orgs.len(), 2);
    assert_eq!(updated_orgs[0].name, "Updated Batch API Org 1");
    assert_eq!(updated_orgs[1].name, "Updated Batch API Org 2");

    // Find organizations by name
    let resp = test::TestRequest::get()
        .uri(&format!("{}/organizations/search?name=Updated", BASE_URL))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    // Updated to use the new response format
    let response: serde_json::Value = test::read_body_json(resp).await;
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let found_orgs = response["organizations"].as_array().unwrap_or(&empty_vec);
    let total = response["total"].as_u64().unwrap_or(0);
    assert!(total > 0);
    assert!(!found_orgs.is_empty());

    // Get organizations by IDs
    let get_by_ids_req = json!([orgs[0].id, orgs[2].id]);
    let resp = test::TestRequest::post()
        .uri(&format!("{}/organizations/get-by-ids", BASE_URL))
        .set_json(&get_by_ids_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let retrieved_orgs: Vec<Organization> = test::read_body_json(resp).await;
    assert_eq!(retrieved_orgs.len(), 2);
    assert_eq!(retrieved_orgs[0].id, orgs[0].id);
    assert_eq!(retrieved_orgs[1].id, orgs[2].id);

    // Batch delete organizations
    let delete_req = json!([orgs[0].id, orgs[1].id, orgs[2].id]);
    let resp = test::TestRequest::delete()
        .uri(&format!("{}/organizations/batch", BASE_URL))
        .set_json(&delete_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let deleted_count: usize = test::read_body_json(resp).await;
    assert_eq!(deleted_count, 3);

    // Clean up any remaining test data
    cleanup_test_data(&app).await;

    // Tear down the test database after tests complete
    let _ = teardown_test_database().await;
}

#[actix_web::test]
async fn test_organization_config_endpoints() {
    let app = create_test_app().await;

    // Create organization first
    let create_req = json!({
        "name": "Config API Test Org"
    });
    let resp = test::TestRequest::post()
        .uri(&format!("{}/organizations", BASE_URL))
        .set_json(&create_req)
        .send_request(&app)
        .await;
    let org: Organization = test::read_body_json(resp).await;

    // Create config
    let config_req = json!({
        "config_key": "api.test.key",
        "config_value": "api test value"
    });
    let resp = test::TestRequest::post()
        .uri(&format!("{}/organizations/{}/configs", BASE_URL, org.id))
        .set_json(&config_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 201);
    let config: OrganizationConfig = test::read_body_json(resp).await;
    assert_eq!(config.config_key, "api.test.key");
    assert_eq!(config.config_value, "api test value");

    // Update config
    let update_req = json!({
        "config_value": "updated api test value"
    });
    let resp = test::TestRequest::put()
        .uri(&format!(
            "{}/organizations/{}/configs/{}",
            BASE_URL, org.id, config.id
        ))
        .set_json(&update_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let updated_config: OrganizationConfig = test::read_body_json(resp).await;
    assert_eq!(updated_config.config_key, "api.test.key");
    assert_eq!(updated_config.config_value, "updated api test value");

    // Get config
    let resp = test::TestRequest::get()
        .uri(&format!(
            "{}/organizations/{}/configs/{}",
            BASE_URL, org.id, config.id
        ))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let retrieved_config: OrganizationConfig = test::read_body_json(resp).await;
    assert_eq!(retrieved_config.id, config.id);
    assert_eq!(retrieved_config.config_key, "api.test.key");

    // List configs
    let resp = test::TestRequest::get()
        .uri(&format!(
            "{}/organizations/{}/configs?page=1&per_page=10",
            BASE_URL, org.id
        ))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    // Updated to use the new response format
    let response: serde_json::Value = test::read_body_json(resp).await;
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let configs = response["configs"].as_array().unwrap_or(&empty_vec);
    let total = response["total"].as_u64().unwrap_or(0);
    assert_eq!(total, 1);
    assert_eq!(configs.len(), 1);

    // Delete config
    let resp = test::TestRequest::delete()
        .uri(&format!(
            "{}/organizations/{}/configs/{}",
            BASE_URL, org.id, config.id
        ))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 204);

    // Clean up
    let _ = test::TestRequest::delete()
        .uri(&format!("{}/organizations/{}", BASE_URL, org.id))
        .send_request(&app)
        .await;

    // Clean up any remaining test data
    cleanup_test_data(&app).await;

    // Tear down the test database after tests complete
    let _ = teardown_test_database().await;
}

#[actix_web::test]
async fn test_organization_config_batch_endpoints() {
    let app = create_test_app().await;

    // Create organization first
    let create_req = json!({
        "name": "Config Batch API Test Org"
    });
    let resp = test::TestRequest::post()
        .uri(&format!("{}/organizations", BASE_URL))
        .set_json(&create_req)
        .send_request(&app)
        .await;
    let org: Organization = test::read_body_json(resp).await;

    // Batch create configs
    let config_req = json!([
        { "config_key": "batch.api.key1", "config_value": "batch api value 1" },
        { "config_key": "batch.api.key2", "config_value": "batch api value 2" },
        { "config_key": "batch.api.key3", "config_value": "batch api value 3" }
    ]);
    let resp = test::TestRequest::post()
        .uri(&format!(
            "{}/organizations/{}/configs/batch",
            BASE_URL, org.id
        ))
        .set_json(&config_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 201);
    let configs: Vec<OrganizationConfig> = test::read_body_json(resp).await;
    assert_eq!(configs.len(), 3);
    assert_eq!(configs[0].config_key, "batch.api.key1");
    assert_eq!(configs[1].config_key, "batch.api.key2");
    assert_eq!(configs[2].config_key, "batch.api.key3");

    // Batch update configs
    let update_req = json!([
        [configs[0].id, { "config_value": "updated batch api value 1" }],
        [configs[1].id, { "config_value": "updated batch api value 2" }]
    ]);
    let resp = test::TestRequest::put()
        .uri(&format!(
            "{}/organizations/{}/configs/batch",
            BASE_URL, org.id
        ))
        .set_json(&update_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let updated_configs: Vec<OrganizationConfig> = test::read_body_json(resp).await;
    assert_eq!(updated_configs.len(), 2);
    assert_eq!(updated_configs[0].config_key, "batch.api.key1");
    assert_eq!(updated_configs[1].config_key, "batch.api.key2");

    // Find configs by key
    let resp = test::TestRequest::get()
        .uri(&format!(
            "{}/organizations/{}/configs/search?search_key=batch.api.key",
            BASE_URL, org.id
        ))
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let found_configs: Vec<OrganizationConfig> = test::read_body_json(resp).await;
    assert_eq!(found_configs.len(), 3);

    // Get configs by IDs
    let get_by_ids_req = json!([configs[0].id, configs[2].id]);
    let resp = test::TestRequest::post()
        .uri(&format!(
            "{}/organizations/{}/configs/get-by-ids",
            BASE_URL, org.id
        ))
        .set_json(&get_by_ids_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let retrieved_configs: Vec<OrganizationConfig> = test::read_body_json(resp).await;
    assert_eq!(retrieved_configs.len(), 2);
    assert_eq!(retrieved_configs[0].id, configs[0].id);
    assert_eq!(retrieved_configs[1].id, configs[2].id);

    // Batch delete configs
    let delete_req = json!([configs[0].id, configs[1].id, configs[2].id]);
    let resp = test::TestRequest::delete()
        .uri(&format!(
            "{}/organizations/{}/configs/batch",
            BASE_URL, org.id
        ))
        .set_json(&delete_req)
        .send_request(&app)
        .await;
    assert_eq!(resp.status(), 200);
    let deleted_count: usize = test::read_body_json(resp).await;
    assert_eq!(deleted_count, 3);

    // Clean up
    let _ = test::TestRequest::delete()
        .uri(&format!("{}/organizations/{}", BASE_URL, org.id))
        .send_request(&app)
        .await;

    // Clean up any remaining test data
    cleanup_test_data(&app).await;

    // Tear down the test database after tests complete
    let _ = teardown_test_database().await;
}
