use actix_web::{get, post, web, Responder, HttpResponse};
use sea_orm::EntityTrait;
use crate::entity::prelude::Users;
use crate::models::{AppState, Status};

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})
}

#[get("/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let users: Vec<serde_json::Value> = Users::find().into_json().all(conn).await.unwrap();

    HttpResponse::Ok().json(users)
}

#[post("/create-new-User")]
pub async fn create_new_user() -> impl Responder {
    format!("hello from create_new_user")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(status)
        .service(create_new_user)
        .service(get_entries);
}
