use horus_be::infrastructure::db;

#[tokio::test]
async fn test_db_connection() {
    // Set test database URL
    std::env::set_var(
        "APP_DATABASE__URL",
        "postgres://postgres:postgres@localhost:5432/postgres",
    );

    let pool = db::create_pool(
        "postgres://postgres:postgres@localhost:5432/postgres",
        5,
        30,
    )
    .await
    .expect("Failed to create database pool");

    assert!(pool.get().await.is_ok());
}
