use poise::{command, serenity_prelude::User};

use crate::{
    commands::CommandContext,
    error::{AppError, MapError},
};

#[command(slash_command)]
pub async fn kick(
    ctx: CommandContext<'_>,
    #[description = "Usuário a ser expulso"]
    #[rename = "user"]
    target_user: User,
) -> Result<(), AppError> {
    Ok(())
}
