#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;
mod user;

use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
use std::io;
use crate::config::Config;
use crate::models::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Config::from_env().unwrap();

    let pool = config.configure_pool();

    let server_addr = format!("{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                pool: pool.clone(),
            })
            .configure(handlers::config)
            .wrap(Logger::default())
    })
        .bind(server_addr)?
        .run()
        .await
}
