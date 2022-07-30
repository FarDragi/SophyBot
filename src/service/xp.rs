use poise::serenity_prelude::{GuildId, UserId};
use tonic::Request;

use crate::{error::AppError, models::xp::Level};

use super::{proto::User, Service};

#[async_trait]
pub trait XpService {
    async fn add_xp(&self, guild_id: GuildId, user_id: UserId) -> Result<Level, AppError>;
}

#[async_trait]
impl XpService for Service {
    async fn add_xp(&self, guild_id: GuildId, user_id: UserId) -> Result<Level, AppError> {
        let request = Request::new(User {
            guild_id: guild_id.0,
            user_id: user_id.0,
        });

        let result = {
            let mut client = self.xp.lock().await;

            client.add_xp(request).await
        };

        match result {
            Ok(response) => {
                let level_up = response.get_ref();

                let mut level_info = Level::default();

                if let Some(info) = &level_up.global {
                    if info.up {
                        level_info.global = Some(info.level);
                    }
                }

                if let Some(info) = &level_up.guild {
                    if info.up {
                        level_info.guild = Some(info.level);
                    }
                }

                Ok(level_info)
            }
            Err(status) => {
                error!("{}", status);

                Err(AppError::Custom("Fail add xp"))
            }
        }
    }
}
