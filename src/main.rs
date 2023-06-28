mod config;
mod entity;
mod handlers;
mod jwt_auth;
mod serializers;

use crate::{
    config::{AppConfig, AppState, CorsConfig},
    serializers::TokenClaims,
};
use actix_web::{
    dev::ServiceRequest, error::ErrorUnauthorized, http, middleware::Logger, web::Data, App, Error,
    HttpServer,
};
use actix_web_grants::GrantsMiddleware;
use env_logger::Env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::Database;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = AppConfig::from_env();

    let db_url = config.database_url.to_string();
    let server_addr = format!("{}:{}", config.server_host, config.server_port);

    let conn = Database::connect(&db_url).await.unwrap();

    let cors_config = CorsConfig::from_env();

    HttpServer::new(move || {
        let cors =
            CorsConfig::set_cors(&cors_config.protocol, &cors_config.host, &cors_config.port);

        let auth = GrantsMiddleware::with_extractor(extract);
        App::new()
            .app_data(Data::new(AppState { conn: conn.clone() }))
            .wrap(auth)
            .configure(handlers::config)
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(server_addr)?
    .run()
    .await
}

async fn extract(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let config = AppConfig::from_env();

    let token = req
        .cookie("token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get(http::header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
        });

    let token = match token {
        Some(token) => token,
        None => {
            return Ok(vec!["ROLE_USER".to_string()]);
        }
    };

    let claims = match decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(c) => c.claims,
        Err(_) => {
            return Err(ErrorUnauthorized("You are not logged in"));
        }
    };

    let is_superuser = claims.is_superuser;
    let is_staff = claims.is_staff;

    if is_superuser {
        Ok(vec!["ROLE_SUPERUSER".to_string()])
    } else if is_staff {
        Ok(vec!["ROLE_STAFF".to_string()])
    } else {
        Ok(vec!["ROLE_INTERN".to_string()])
    }
}
