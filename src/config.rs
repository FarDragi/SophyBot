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
    pub service: ServiceConfig,
}

#[derive(Deserialize, Debug)]
pub struct RedisConfig {
    pub host: String,
    pub port: Option<u16>,
    pub slot: Option<u16>,
}

#[derive(Deserialize, Debug)]
pub struct ServiceConfig {
    pub url: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        info!("Loading config");
        Config::figment().extract().expect("Fail get config")
    }

    pub fn figment() -> Figment {
        Figment::new()
            .merge(Env::prefixed("SOPHY_").split("_"))
            .merge(Toml::file("Sophy.toml"))
    }
}
