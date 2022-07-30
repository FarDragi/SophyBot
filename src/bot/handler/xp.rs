use poise::serenity_prelude::{Context, Guild, Message, User};

use crate::{
    bot::state::State,
    error::{AppError, MapError},
    service::xp::XpService,
};

pub async fn add_xp(
    ctx: &Context,
    message: &Message,
    user: &User,
    guild: &Guild,
    state: &State,
) -> Result<(), AppError> {
    let result = state.service.add_xp(guild.id, user.id).await?;

    if let Some(level) = result.guild {
        let text = format!("Level up to {}", level);
        message.reply(ctx, text).await.map_app_err()?;
    }

    Ok(())
}
