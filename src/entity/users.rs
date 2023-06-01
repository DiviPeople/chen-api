use crate::config::{AppConfig, EmailConfig, ARGON2_CONFIG};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use rand::distributions::{Alphanumeric, DistString};
use reqwest::{header::USER_AGENT, Client};
use sea_orm::{entity::prelude::*, Set};
use serde::Deserialize;
use serde_json::json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub img_url: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub integrations: Option<Json>,
}

#[derive(Deserialize)]
pub struct User {
    pub full_name: String,
    pub email: String,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub img_url: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub integrations: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn set_id(&mut self) {
        self.id = Set(Uuid::new_v4());
    }

    pub fn encrypt(&mut self, password_hash: String) {
        let salt: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let hash: String = argon2::hash_encoded(
            password_hash.trim().as_bytes(),
            salt.as_bytes(),
            &ARGON2_CONFIG,
        )
        .unwrap();

        self.salt = Set(salt);
        self.password_hash = Set(hash);
    }

    pub async fn send_password(&mut self, email: &str, password: &String) {
        let email_cfg = EmailConfig::from_env();

        let email_msg = Message::builder()
            .from(email_cfg.email_from.parse().unwrap())
            .reply_to(email_cfg.email_reply_to.parse().unwrap())
            .to(email.parse().unwrap())
            .subject("Your Chen password")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Пароль для вашего аккаунта Chen:") + password)
            .unwrap();

        let creds = Credentials::new(email_cfg.email_from, email_cfg.email_password);

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email_msg) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => println!("Could not send email: {e:?}"),
        }
    }

    pub async fn send_invitation(&mut self, email: &String) {
        let json = json!({ "email": email });
        let token = AppConfig::from_env().github_token;
        let url = AppConfig::from_env().org_url;

        Client::new()
            .post(url)
            .bearer_auth(&token)
            .header(USER_AGENT, format!("Bearer {}", token.to_owned()))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("Accept", "application/vnd.github+json")
            .body(json.to_string())
            .send()
            .await
            .unwrap();
    }
}
