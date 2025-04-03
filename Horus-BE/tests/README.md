# 📌 Test Structure Guide

This project follows a structured approach to testing, with **unit tests** inside each module and **integration tests** inside the `tests/` directory.

---

## **1️⃣ Unit Tests (`src/**/tests.rs`)\*\*

### ✅ **Purpose**

- Unit tests verify **business logic** without dependencies.
- These tests run **in isolation** (no database, API calls, or real external services).

### 📌 **Where to Find Them?**

- Each domain module contains a `mod tests` section for unit tests.
- Example:

```bash
    src/
    ├── domain/
    │   ├── developer_metrics/
    │   │   ├── services/
    │   │   │   ├── developer_metrics_service.rs  # Contains a mod tests section
```

### 📝 **Example Unit Test (`src/domain/developer_metrics/services/developer_metrics_service.rs`):**

```rust
#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{Duration, Utc};

  #[tokio::test]
  async fn test_calculate_deployment_frequency() {
      let service = DeveloperMetricsService::new(MockGitHubProvider::default());

      let start_date = Utc::now() - Duration::days(7);
      let end_date = Utc::now();
      let deployments = vec![Deployment::new("test-repo", start_date)];

      let frequency = service.calculate_deployment_frequency(&deployments, start_date, end_date);

      assert_eq!(frequency.total_deployments, 1);
  }
}
```

## **2️⃣ Integration Tests (`tests/`)**

### ✅ **Purpose**

- Integration tests verify API behavior by testing real endpoints.
- These tests involve database queries, external API calls, and service interactions.

### 📌 **Where to Find Them?**

- Integration tests are located in the `tests/` directory.
- Example:

```bash
tests/integration/
├── test_developer_metrics.rs   # Integration tests for Developer API
├── test_collaboration.rs       # Integration tests for Collaboration API
├── test_delivery.rs            # Integration tests for Delivery Insights API
```

### 📝 **Example Integration Test (`tests/integration/test_delivery.rs`):**

```rust
#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::application::controllers::delivery_metrics_controller::get_deployments;

    #[actix_rt::test]
    async fn test_get_deployments() {
        let app = test::init_service(
            App::new().route("/deployments", web::get().to(get_deployments)),
        )
        .await;

        let req = test::TestRequest::get().uri("/deployments").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }
}
```

## **3️⃣ Infrastructure Tests (tests/unit/infrastructure/)**

### 📌 **Purpose**

- These tests focus on database queries, Redis caching, and GitHub API calls.
- They mock external dependencies to test infrastructure logic.

### 📌 **Where to Find Them?**

- Infrastructure tests are located in the `tests/unit/infrastructure/` directory.
- Example:

```bash
tests/unit/infrastructure/
├── github_sdk/
│   ├── test_github_client.rs  # Test GitHub API Client
├── database/
│   ├── test_postgres.rs       # Test PostgreSQL queries
├── redis/
│   ├── test_redis.rs          # Test Redis caching logic
```

### 📝 **Shared Test Utilities (`tests/integration/common.rs`):**

✅ Purpose

- This file contains helper functions for setting up test databases, API mocks, etc.
- Used by both unit and integration tests.

### 📝 **Example Shared Test Utility (`tests/integration/common.rs`):**

```rust
use actix_web::{App, web};
use crate::application::controllers::delivery_metrics_controller::get_deployments;

pub async fn setup_test_app() -> App {
    test::init_service(
        App::new().route("/deployments", web::get().to(get_deployments)),
    )
    .await
}
```

## **4️⃣ Running Tests**

### 📝 **Unit Tests Only**

```bash
cargo test --lib
```

### 📝 **Integration Tests**

```bash
cargo test --test integration
```

### 📝 **Infrastructure Tests**

```bash
cargo test --test unit
```

### 📝 **All Tests**

```bash
cargo test
```
