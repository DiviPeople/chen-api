use dotenvy::dotenv;
use std::env;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub conn: DatabaseConnection,
}

pub struct Config {
    pub server_host: String,
    pub server_port: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();

        Config {
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
