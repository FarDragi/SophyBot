use std::{fmt::Debug, sync::Arc, time::Duration};

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
    pub async fn new(config: Arc<Config>) -> Cache {
        let url = format!(
            "redis://{}:{}/{}",
            config.redis.host,
            config.redis.port.unwrap_or(6379),
            config.redis.slot.unwrap_or(0)
        );

        let pool = PoolConfig::from_url(&url)
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create redis pool");

        info!("Connected to redis: {}", &url);

        Cache { pool }
    }

    async fn get_connection(&self) -> Result<Connection, AppError> {
        self.pool.get().await.map_err(AppError::Pool)
    }

    pub async fn set(&self, key: &str, value: &str, duration: Duration) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;

        conn.set_ex(key, value, duration.as_secs() as usize)
            .await
            .map_app_err()?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.get_connection().await?;

        conn.get(key).await.map_app_err()
    }
}
