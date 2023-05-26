use crate::{
    config::AppState,
    entity::{
        prelude::Users,
        users,
        users::{ActiveModel, Model},
    },
    serializers::Status,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QuerySelect, Set,
};

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let users: Vec<serde_json::Value> = Users::find()
        .select_only()
        .columns([
            users::Column::FullName,
            users::Column::Email,
            users::Column::IsStaff,
        ])
        .into_json()
        .all(conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(users)
}

#[get("/users/{id}")]
async fn get_user(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let id: i32 = path.into_inner();
    let user: Option<serde_json::Value> =
        Users::find_by_id(id).into_json().one(conn).await.unwrap();

    HttpResponse::Ok().json(user)
}

#[post("/users")]
async fn create_user(data: web::Data<AppState>, obj: web::Json<Model>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;

    let mut user: ActiveModel = ActiveModel {
        full_name: Set(obj.full_name.to_owned()),
        email: Set(obj.email.to_owned()),
        is_superuser: Set(obj.is_superuser.to_owned()),
        is_staff: Set(obj.is_staff.to_owned()),
        img_url: Set(obj.img_url.to_owned()),
        created_at: Set(obj.created_at.to_owned()),
        updated_at: Set(obj.updated_at.to_owned()),
        integrations: Set(obj.integrations.to_owned()),
        ..Default::default()
    };
    user.encrypt("pass".to_string());

    user.insert(conn).await.unwrap();

    HttpResponse::Ok()
}

#[put("/users/{id}")]
async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    obj: web::Json<Model>,
) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let id: i32 = path.into_inner();
    let user: Option<Model> = Users::find_by_id(id).one(conn).await.unwrap();
    let mut user: ActiveModel = user.unwrap().into();

    user.full_name = Set(obj.full_name.to_owned());
    user.email = Set(obj.email.to_owned());
    user.password_hash = Set(obj.password_hash.to_owned());
    user.salt = Set(obj.salt.to_owned());
    user.is_superuser = Set(obj.is_superuser.to_owned());
    user.is_staff = Set(obj.is_staff.to_owned());
    user.img_url = Set(obj.img_url.to_owned());
    user.created_at = Set(obj.created_at.to_owned());
    user.updated_at = Set(obj.updated_at.to_owned());
    user.integrations = Set(obj.integrations.to_owned());

    user.update(conn).await.unwrap();

    HttpResponse::Ok()
}

#[delete("/users/{id}")]
async fn delete_user(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let id: i32 = path.into_inner();
    let user: Option<Model> = Users::find_by_id(id).one(conn).await.unwrap();
    let user: Model = user.unwrap();
    users::Entity::delete(user.into_active_model())
        .exec(conn)
        .await
        .unwrap();

    HttpResponse::Ok()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(status)
        .service(get_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}
