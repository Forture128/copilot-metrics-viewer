// src/infrastructure/redis.rs

use crate::config::RedisConfig;
use anyhow::Result;
use redis::Client;
use serde_json;

#[derive(Clone)]
pub struct Redis {
    client: Client,
}

impl Redis {
    pub fn new(config: &RedisConfig) -> Result<Self> {
        let client = Client::open(config.url.as_str())?;
        Ok(Self { client })
    }

    pub async fn set_ex<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
        expiry: u64,
    ) -> Result<bool> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let serialized = serde_json::to_string(value)?;
        let result: bool = redis::cmd("SET")
            .arg(key)
            .arg(serialized)
            .arg("EX")
            .arg(expiry)
            .query_async(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        Ok(value.map(|v| serde_json::from_str(&v).unwrap()))
    }

    pub async fn del(&self, key: &str) -> Result<bool> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let deleted: bool = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        Ok(deleted)
    }
}
