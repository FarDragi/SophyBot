use std::time::Duration;

use crate::error::{AppError, MapError};

use super::Cache;

#[async_trait]
pub trait UserCache {
    async fn set_user_last_message(&self, user_id: u64, time: i64) -> Result<(), AppError>;
    async fn get_user_last_message(&self, user_id: u64) -> Result<Option<i64>, AppError>;
}

#[async_trait]
impl UserCache for Cache {
    async fn set_user_last_message(&self, user_id: u64, time: i64) -> Result<(), AppError> {
        let key = format!("user:{}:last_message", user_id);
        self.set(&key, &time.to_string(), Duration::from_secs(600))
            .await
    }

    async fn get_user_last_message(&self, user_id: u64) -> Result<Option<i64>, AppError> {
        let key = format!("user:{}:last_message", user_id);
        let result = self.get(&key).await?;

        let time = result.map(|s| {
            s.parse::<i64>()
                .map_err(|_| AppError::Custom("Failed to parse time"))
        });
    }
}
