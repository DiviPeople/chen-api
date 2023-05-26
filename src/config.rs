use color_eyre::Result;
use dotenvy::dotenv;
use serde::Deserialize;

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

