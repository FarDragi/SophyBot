use poise::{
    command,
    serenity_prelude::{Member, RoleId},
};

use crate::{
    bot::commands::CommandContext,
    error::{AppError, MapError},
};

#[command(
    slash_command,
    required_permissions = "KICK_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: CommandContext<'_>,
    #[description = "Usuário a ser expulso"]
    #[rename = "user"]
    target_user: Member,
) -> Result<(), AppError> {
    let user = ctx
        .author_member()
        .await
        .custom_error("Você não está na guilda")?;

    let (_, position) = user
        .highest_role_info(ctx.discord())
        .custom_error("Você não tem permissão para usar este comando")?;
    let (_, target_position) = target_user
        .highest_role_info(ctx.discord())
        .unwrap_or((RoleId(0), 0));

    if position > target_position {
        target_user.kick(ctx.discord()).await.map_app_err()?;
    }
    Ok(())
}
