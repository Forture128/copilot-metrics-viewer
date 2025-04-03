use horus_be::config::AppConfig;

#[test]
fn test_load_base_config() {
    // Set test environment variables
    std::env::set_var("APP_ENV", "test");
    std::env::set_var("APP_SERVER__ADDRESS", "0.0.0.0:3000");
    std::env::set_var("APP_SERVER__WORKERS", "2");

    let config = AppConfig::load().expect("Failed to load config");
    assert_eq!(config.server.address, "0.0.0.0:3000");
    assert_eq!(config.server.workers, 2);
}
