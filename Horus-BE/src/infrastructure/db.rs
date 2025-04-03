// src/infrastructure/db.rs
use std::time::Duration;
use tracing::{error, info};

use deadpool::managed::{Manager, Metrics, Object, Pool, PoolConfig, RecycleError};
use deadpool_runtime::Runtime;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

pub type DbPool = Pool<PgConnectionManager>;
pub type DbConnection = Object<PgConnectionManager>;

#[derive(Debug)]
pub struct PgConnectionManager {
    connection_url: String,
}

impl PgConnectionManager {
    pub fn new(connection_url: String) -> Self {
        Self { connection_url }
    }
}

#[async_trait::async_trait]
impl Manager for PgConnectionManager {
    type Type = AsyncPgConnection;
    type Error = DieselError;

    async fn create(&self) -> Result<AsyncPgConnection, DieselError> {
        AsyncPgConnection::establish(&self.connection_url)
            .await
            .map_err(|e| {
                DieselError::DatabaseError(
                    diesel::result::DatabaseErrorKind::Unknown,
                    Box::new(e.to_string()),
                )
            })
    }

    async fn recycle(
        &self,
        _conn: &mut AsyncPgConnection,
        _metrics: &Metrics,
    ) -> Result<(), RecycleError<DieselError>> {
        Ok(())
    }
}

pub async fn create_pool(
    database_url: &str,
    max_connections: usize,
    timeout_seconds: u64,
) -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    info!(
        "Creating database pool with {} connections",
        max_connections
    );

    let manager = PgConnectionManager::new(database_url.to_string());
    let pool_config = PoolConfig {
        max_size: max_connections,
        queue_mode: deadpool::managed::QueueMode::Fifo,
        timeouts: deadpool::managed::Timeouts {
            wait: Some(Duration::from_secs(timeout_seconds)),
            create: Some(Duration::from_secs(timeout_seconds)),
            recycle: Some(Duration::from_secs(timeout_seconds)),
        },
    };

    let pool = Pool::builder(manager)
        .config(pool_config)
        .runtime(Runtime::Tokio1)
        .build()?;

    // Test connection
    let mut conn: DbConnection = pool.get().await?;
    diesel::sql_query("SELECT 1")
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("Database verification query failed: {:?}", e);
            e
        })?;

    info!("Successfully connected to database");
    Ok(pool)
}
