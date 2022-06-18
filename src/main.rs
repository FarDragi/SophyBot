mod bot;
mod cache;
mod config;
mod constants;
mod error;
mod logs;

use std::sync::Arc;

use bot::Bot;
use config::Config;
use logs::Logs;

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() {
    Logs::new();
    let config = Arc::new(Config::new());

    let bot = Bot::new(config);
    bot.run().await;
}
