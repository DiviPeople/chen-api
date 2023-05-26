use color_eyre::Result;
use dotenvy::dotenv;
use serde::Deserialize;
use argon2::{self, Config, ThreadMode, Variant, Version};

pub use config::ConfigError;

#[derive(Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: i32,
    pub db_name: String,
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
}

impl AppConfig{
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

pub const ARGON2_CONFIG: argon2::Config<'_> = Config {
    variant: Variant::Argon2id,
    version: Version::Version13,
    mem_cost: 1024,
    time_cost: 10,
    lanes: 4,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 32,
};
