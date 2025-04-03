use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

/// Initialize the logging system with structured logging and proper formatting
pub fn init_logging() {
    // Set default log level if RUST_LOG is not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "info,horus_be=debug,reqwest=warn,hyper=warn,html5ever=warn",
        );
    }

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info")
            .add_directive("horus_be=debug".parse().unwrap())
            .add_directive("actix_web=info".parse().unwrap())
            .add_directive("sqlx=warn".parse().unwrap())
            .add_directive("reqwest=warn".parse().unwrap())
            .add_directive("hyper=warn".parse().unwrap())
    });

    // Completely disable JSON formatting for now to resolve the issues
    // This will use plain text formatting which is more reliable
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_ansi(true) // Disable ANSI colors everywhere
        .with_span_events(FmtSpan::CLOSE)
        .try_init()
        .expect("Failed to initialize logging subscriber");

    tracing::info!(version = env!("CARGO_PKG_VERSION"), "Application starting");
}

/// Get the log level from environment or default to INFO
#[inline]
pub fn get_log_level() -> Level {
    std::env::var("RUST_LOG")
        .ok()
        .and_then(|l| Level::from_str(&l).ok())
        .unwrap_or(Level::INFO)
}

/// Create a span for tracking function execution
#[macro_export]
macro_rules! trace_fn {
    () => {
        let _span = tracing::span!(tracing::Level::TRACE, stringify!(function)).entered();
    };
    ($name:expr) => {
        let _span = tracing::span!(tracing::Level::TRACE, $name).entered();
    };
}
