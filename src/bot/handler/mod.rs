mod xp;

use poise::{serenity_prelude::Context, Event, FrameworkContext};

use crate::error::AppError;

use self::xp::{add_global_xp, add_guild_xp};

use super::state::State;

pub async fn event_handler(
    ctx: &Context,
    event: &Event<'_>,
    _framework: FrameworkContext<'_, State, AppError>,
    state: &State,
) -> Result<(), AppError> {
    match event {
        Event::Message { new_message } => {
            let user = &new_message.author;

            add_global_xp(user, state).await;

            if let Some(guild) = new_message.guild(ctx) {
                add_guild_xp(user, &guild, state).await;
            }
        }
        _ => {}
    }
    Ok(())
}
