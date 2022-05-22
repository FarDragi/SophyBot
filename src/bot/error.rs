use chrono::Utc;
use poise::{serenity_prelude::Permissions, FrameworkError};

use crate::{constants::embed_colors, error::AppError};

use super::{commands::CommandContext, state::State};

pub async fn on_error(error: FrameworkError<'_, State, AppError>) {
    match error {
        FrameworkError::Command { error, ctx } => {
            command(error, ctx).await;
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
        } => {
            missing_bot_permissions(missing_permissions, ctx).await;
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
        } => {
            missing_user_permissions(missing_permissions, ctx).await;
        }
        error => error!("{:?}", error),
    }
}

async fn command(error: AppError, ctx: CommandContext<'_>) {
    if let AppError::Custom(message) = error {
        ctx.send(|reply| {
            reply
                .embed(|embed| {
                    embed
                        .description(message)
                        .color(embed_colors::ERROR)
                        .timestamp(Utc::now())
                })
                .ephemeral(true)
        })
        .await
        .ok();
    }
}

async fn missing_bot_permissions(missing_permissions: Permissions, ctx: CommandContext<'_>) {
    let missing_permissions = missing_permissions.to_string();
    ctx.send(|reply| {
        reply
            .embed(|embed| {
                embed
                    .description(format!("Missing permissions: {}", missing_permissions))
                    .color(embed_colors::ERROR)
                    .timestamp(Utc::now())
            })
            .ephemeral(true)
    })
    .await
    .ok();
}

async fn missing_user_permissions(_: Option<Permissions>, ctx: CommandContext<'_>) {
    ctx.send(|reply| {
        reply
            .embed(|embed| {
                embed
                    .description("Sem permissões suficientes para executar este comando")
                    .color(embed_colors::ERROR)
                    .timestamp(Utc::now())
            })
            .ephemeral(true)
    })
    .await
    .ok();
}
