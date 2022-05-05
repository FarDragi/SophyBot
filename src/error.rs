use std::fmt::{Display, Formatter};

use poise::serenity_prelude::Error as SerenityError;

#[derive(Debug)]
pub enum AppError {
    Serenity(SerenityError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait MapError<T> {
    fn map_app_err(self) -> Result<T, AppError>;
}

impl<T> MapError<T> for Result<T, SerenityError> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Serenity)
    }
}
