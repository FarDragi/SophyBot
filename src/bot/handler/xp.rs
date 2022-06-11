use chrono::Utc;
use poise::serenity_prelude::{Guild, User};

use crate::{bot::state::State, cache::user::UserCache, error::AppError};

pub async fn add_xp(user: &User, guild: &Guild, state: &State) -> Result<(), AppError> {
    let now = Utc::now().timestamp();

    let last_update = state.cache.get_user_last_message(user.id.0).await;

    Ok(())
}
