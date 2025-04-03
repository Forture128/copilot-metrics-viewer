#[cfg(test)]
mod tests {
    use crate::config::AppConfig;

    #[test]
    fn test_load_base_config() {
        let config = AppConfig::load().expect("Failed to load config");

        // Verify server config
        assert_eq!(config.server.address, "0.0.0.0:3000");
        assert!(config.server.workers > 0);

        // Verify database config exists
        assert!(!config.database.url.is_empty());
        assert!(config.database.max_connections > 0);
        assert!(config.database.timeout_seconds > 0);

        // Verify redis config exists
        assert!(!config.redis.url.is_empty());
        assert!(config.redis.timeout_seconds > 0);
    }
}
