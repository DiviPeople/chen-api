use color_eyre::Result;
use deadpool_postgres::Pool;
use dotenvy::dotenv;
use serde::Deserialize;
use tokio_postgres::NoTls;

pub use config::ConfigError;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::load_dotenv();

        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("."))?;
        cfg.try_into()
    }

    fn load_dotenv() {
        dotenv().ok();
    }

    pub fn configure_pool(&self) -> Pool {
        self.pg.create_pool(NoTls).unwrap()
    }
}
