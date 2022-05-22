mod commands;
mod error;
mod state;

use std::{collections::HashMap, sync::Arc, time::Duration};

use poise::{
    builtins::create_application_commands,
    serenity_prelude::{self, GatewayIntents, GuildId},
    Context, Framework, FrameworkBuilder, FrameworkOptions,
};
use tokio::{sync::RwLock, task};

use crate::{
    bot::commands::{get_commands, CommandState},
    config::Config,
    error::{AppError, MapError},
};

use self::{error::on_error, state::State};

pub struct Bot {
    client: FrameworkBuilder<State, AppError>,
}

impl Bot {
    pub fn new(config: Arc<Config>) -> Bot {
        let options = FrameworkOptions {
            commands: get_commands(),
            on_error: |error| Box::pin(on_error(error)),
            command_check: Some(|ctx| Box::pin(command_check(ctx))),
            ..Default::default()
        };

        let client = Framework::build()
            .options(options)
            .intents(GatewayIntents::non_privileged())
            .token(config.token.to_owned())
            .user_data_setup(move |ctx, ready, framework| {
                Box::pin(user_data_setup(ctx, ready, framework, config))
            });

        Bot { client }
    }

    pub async fn run(self) {
        self.client.run().await.expect("Failed to start bot");
    }
}

pub async fn user_data_setup<'a>(
    ctx: &serenity_prelude::Context,
    ready: &serenity_prelude::Ready,
    framework: &Framework<State, AppError>,
    config: Arc<Config>,
) -> Result<State, AppError> {
    info!("{}", ready.user.tag());

    GuildId(config.owner_guild_id)
        .set_application_commands(ctx, |app_commands| {
            *app_commands = create_application_commands(&framework.options().commands);
            app_commands
        })
        .await
        .map_app_err()?;

    let shards = Arc::new(RwLock::new(HashMap::new()));

    {
        let shards = shards.clone();
        let manager = framework.shard_manager();
        task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let mut shards = shards.write().await;

                let manager = manager.lock().await;
                let runners = manager.runners.lock().await;

                runners.iter().for_each(|(id, info)| {
                    let latency = info.latency.unwrap_or(Duration::from_secs(0));

                    shards.insert(id.0, latency);

                    debug!("Shard latency: [{}: {:?}] {:?}", id, info.stage, latency);
                });
            }
        });
    }

    Ok(State {
        config,
        shards,
        ..Default::default()
    })
}

pub async fn command_check(ctx: Context<'_, State, AppError>) -> Result<bool, AppError> {
    ctx.set_invocation_data(CommandState).await;

    info!(
        "{} executed command {}",
        ctx.author().tag(),
        ctx.command().name
    );

    Ok(true)
}
