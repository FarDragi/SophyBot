use poise::serenity_prelude::{Guild, User};

use crate::{bot::state::State, error::AppError};

pub async fn add_xp(user: &User, guild: &Guild, state: &State) -> Result<(), AppError> {
    Ok(())
}
