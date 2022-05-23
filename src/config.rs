use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub owner_guild_id: u64,
    pub redis: RedisConfig,
}

#[derive(Deserialize, Debug)]
pub struct RedisConfig {
    pub host: String,
    pub port: Option<u16>,
    pub slot: Option<u16>,
}

impl Config {
    pub fn new() -> Config {
        info!("Loading config");
        Config::figment().extract().expect("Fail get config")
    }

    pub fn figment() -> Figment {
        Figment::new()
            .merge(Env::prefixed("SOPHY_"))
            .merge(Toml::file("Sophy.toml"))
    }
}
