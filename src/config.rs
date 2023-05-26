use argon2::{self, Config, ThreadMode, Variant, Version};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use std::env;

pub struct AppState {
    pub conn: DatabaseConnection,
}

pub struct AppConfig {
    pub server_host: String,
    pub server_port: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
}

impl AppConfig {
    pub fn from_env() -> AppConfig {
        dotenv().ok();

        AppConfig {
            server_host: env::var("SERVER_HOST").expect("SERVER_HOST must be set"),
            server_port: env::var("SERVER_PORT").expect("SERVER_PORT must be set"),
            db_name: env::var("DB_NAME").expect("DB_NAME must be set"),
            db_host: env::var("DB_HOST").expect("DB_HOST must be set"),
            db_port: env::var("DB_PORT").expect("DB_PORT must be set"),
            db_user: env::var("DB_USER").expect("DB_USER must be set"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        }
    }
}

pub const ARGON2_CONFIG: Config<'_> = Config {
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
