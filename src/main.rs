mod config;
mod handlers;
mod entity;
mod serializers;
mod models;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use env_logger::Env;
use std::io;
use sea_orm::Database;
use crate::config::Config;
use self::config::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Config::from_env();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    );
    let server_addr = format!("{}:{}", config.server_host, config.server_port);

    let conn = Database::connect(&db_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                conn: conn.clone(),
            }))
            .configure(handlers::config)
            .wrap(Logger::default())
    })
        .bind(server_addr)?
        .run()
        .await
}
