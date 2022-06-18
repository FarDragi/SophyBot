use std::fmt::{Display, Formatter};

use deadpool_redis::PoolError;
use poise::serenity_prelude::Error as SerenityError;
use redis::RedisError;

#[derive(Debug)]
pub enum AppError {
    Serenity(SerenityError),
    Redis(RedisError),
    Pool(PoolError),
    Custom(&'static str),
    None,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait MapError<T> {
    fn map_app_err(self) -> Result<T, AppError>;
    fn custom_error(self, message: &'static str) -> Result<T, AppError>;
}

impl<T> MapError<T> for Result<T, SerenityError> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Serenity)
    }

    fn custom_error(self, message: &'static str) -> Result<T, AppError> {
        self.map_err(|_| AppError::Custom(message))
    }
}

impl<T> MapError<T> for Result<T, RedisError> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Redis)
    }

    fn custom_error(self, message: &'static str) -> Result<T, AppError> {
        self.map_err(|_| AppError::Custom(message))
    }
}

impl<T> MapError<T> for Option<T> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.ok_or(AppError::None)
    }

    fn custom_error(self, message: &'static str) -> Result<T, AppError> {
        self.ok_or(AppError::Custom(message))
    }
}
