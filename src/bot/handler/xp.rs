use poise::serenity_prelude::{Guild, User};

use crate::bot::state::State;

pub async fn add_global_xp(user: &User, state: &State) {
    let cache = state.cache.set("xp", "1").await;
}

pub async fn add_guild_xp(user: &User, guild: &Guild, state: &State) {}
