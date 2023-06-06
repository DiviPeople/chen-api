use actix_cors::Cors;
use actix_web::http;
use argon2::{self, Config, ThreadMode, Variant, Version};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
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
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub github_token: String,
    pub org_url: String,
    pub rc_org_url: String,
    pub rc_token: String,
    pub rc_admin_id: String,
    pub nc_org_url: String,
    pub nc_login_admin: String,
    pub nc_password_admin: String,
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
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expires_in: env::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN must be set"),
            github_token: env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set"),
            org_url: env::var("ORG_URL").expect("ORG_URL must be set"),
            rc_org_url: env::var("RC_ORG_URL").expect("RC_ORG_URL must be set"),
            rc_token: env::var("RC_TOKEN").expect("RC_TOKEN must be set"),
            rc_admin_id: env::var("RC_ADMIN_ID").expect("RC_ADMIN_ID must be set"),
            nc_org_url: env::var("NC_ORG_URL").expect("NC_ORG_URL must be set"),
            nc_login_admin: env::var("NC_LOGIN_ADMIN").expect("NC_LOGIN_ADMIN must be set"),
            nc_password_admin: env::var("NC_PASSWORD_ADMIN")
                .expect("NC_PASSWORD_ADMIN must be set"),
        }
    }
}

#[derive(Deserialize)]
pub struct EmailConfig {
    pub email_from: String,
    pub email_password: String,
    pub email_reply_to: String,
}

impl EmailConfig {
    pub fn from_env() -> EmailConfig {
        dotenv().ok();

        EmailConfig {
            email_from: env::var("EMAIL_FROM").expect("EMAIL_FROM must be set"),
            email_password: env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set"),
            email_reply_to: env::var("EMAIL_REPLY_TO").expect("EMAIL_REPLY_TO must be set"),
        }
    }
}

#[derive(Deserialize)]
pub struct CorsConfig {
    pub protocol: String,
    pub host: String,
    pub port: String,
}

impl CorsConfig {
    pub fn from_env() -> CorsConfig {
        dotenv().ok();

        CorsConfig {
            protocol: env::var("CORS_PROTOCOL").expect("CORS_PROTOCOL must be set"),
            host: env::var("CORS_HOST").expect("CORS_HOST must be set"),
            port: env::var("CORS_PORT").expect("CORS_PORT must be set"),
        }
    }

    pub fn set_cors(protocol: &String, host: &String, port: &String) -> Cors {
        Cors::default()
            .allowed_origin(format!("{}://{}:{}", protocol, host, port).as_str())
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600)
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
