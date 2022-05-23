use std::fmt::Debug;

use deadpool_redis::{Config as PoolConfig, Connection, Pool, Runtime};
use redis::AsyncCommands;

use crate::{
    config::Config,
    error::{AppError, MapError},
};

pub struct Cache {
    pool: Pool,
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache").finish()
    }
}

impl Cache {
    pub async fn new(config: &Config) -> Cache {
        let url = format!(
            "redis://{}:{}/{}",
            config.redis.host,
            config.redis.port.unwrap_or(6379),
            config.redis.slot.unwrap_or(0)
        );

        let pool = PoolConfig::from_url(url)
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create redis pool");

        Cache { pool }
    }

    async fn get_connection(&self) -> Connection {
        self.pool
            .get()
            .await
            .expect("Failed to get redis connection")
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self.get_connection().await;

        conn.set(key, value).await.map_app_err()?;

        Ok(())
    }
}
