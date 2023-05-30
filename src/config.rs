use color_eyre::Result;
use dotenvy::dotenv;
use std::env;
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

#[derive(Deserialize)]
pub struct EmailConfig {
   pub email_from: String,
   pub email_password: String,
   pub email_reply_to: String,
}

impl EmailConfig {
    pub fn from_env() -> EmailConfig{
        dotenv().ok();
        
        EmailConfig {
            email_from: env::var("EMAIL_FROM").expect("EMAIL_FROM must be set"),
            email_password: env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set"),
            email_reply_to: env::var("EMAIL_REPLY_TO").expect("EMAIL_REPLY_TO must be set"),
        }
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
