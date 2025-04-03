use std::sync::Once;
use tracing::Level;
pub mod test_helpers;

static INIT: Once = Once::new();

/// Initialize test environment
pub fn init() {
    INIT.call_once(|| {
        // Initialize logging for tests
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .compact()
            .init();

        // Load environment variables
        dotenvy::dotenv().ok();
    });
}
