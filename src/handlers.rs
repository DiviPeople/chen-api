use actix_web::{get, post, web, Responder, HttpResponse};
use crate::models::Status;

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})
}

#[post("/create-new-user")]
pub async fn create_new_user() -> impl Responder {
    format!("hello from create_new_user")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(status)
        .service(create_new_user);
}
