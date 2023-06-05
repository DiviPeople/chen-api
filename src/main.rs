mod config;
mod entity;
mod handlers;
mod jwt_auth;
mod serializers;

use crate::config::{AppConfig, AppState, CorsConfig};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use env_logger::Env;
use sea_orm::Database;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = AppConfig::from_env();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    );
    let server_addr = format!("{}:{}", config.server_host, config.server_port);

    let conn = Database::connect(&db_url).await.unwrap();

    let cors_config = CorsConfig::from_env();

    HttpServer::new(move || {
        let cors =
            CorsConfig::set_cors(&cors_config.protocol, &cors_config.host, &cors_config.port);
        App::new()
            .app_data(Data::new(AppState { conn: conn.clone() }))
            .configure(handlers::config)
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(server_addr)?
    .run()
    .await
}
