mod config;
mod handlers;
mod models;
mod entity;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use env_logger::Env;
use std::io;
use sea_orm::Database;
use migration::{Migrator, MigratorTrait};
use crate::config::Config;
use crate::models::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Config::from_env().unwrap();
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                conn: conn.clone(),
            }))
            .configure(handlers::config)
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
