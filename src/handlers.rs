use crate::{
    config::{AppConfig, AppState, ARGON2_CONFIG},
    entity::{
        prelude::Users,
        users::{self, ActiveModel, DeleteUser, Model, User},
    },
    jwt_auth,
    serializers::{ChangePassword, LoginUserSchema, Status, TokenClaims},
};
use actix_web::{
    cookie::{time::Duration as CookieDuration, Cookie},
    delete, get, post, put, web, HttpResponse, Responder,
};
use actix_web_grants::proc_macro::has_any_role;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::distributions::{Alphanumeric, DistString};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect, Set,
};

#[get("/")]
#[has_any_role("SUPERUSER", "STAFF", "INTERN", "USER")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[post("/login")]
#[has_any_role("SUPERUSER", "STAFF", "INTERN", "USER")]
async fn login(data: web::Data<AppState>, body: web::Json<LoginUserSchema>) -> impl Responder {
    let conn = &data.conn;
    let user: Option<users::Model> = Users::find()
        .filter(users::Column::Email.contains(&body.email))
        .one(conn)
        .await
        .unwrap();

    if user.is_none() {
        return HttpResponse::Unauthorized().json("Invalid email or password");
    }

    let user = user.unwrap();

    let user_hash = user.password_hash;
    let login_hash = argon2::hash_encoded(
        body.password.trim().as_bytes(),
        user.salt.as_bytes(),
        &ARGON2_CONFIG,
    )
    .unwrap();

    if !user_hash.eq(&login_hash) {
        return HttpResponse::BadRequest().json("Invalid email or password");
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(AppConfig::from_env().jwt_expires_in.parse().unwrap()))
        .timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id,
        is_superuser: user.is_superuser,
        is_staff: user.is_staff,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(AppConfig::from_env().jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(CookieDuration::minutes(1))
        .http_only(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(token)
}

#[get("/logout")]
#[has_any_role("SUPERUSER", "STAFF", "INTERN")]
async fn logout(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(CookieDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).json("Logged out")
}

#[get("/me")]
#[has_any_role("SUPERUSER", "STAFF", "INTERN", "USER")]
async fn get_me(data: web::Data<AppState>, jwt: jwt_auth::JwtMiddleware) -> impl Responder {
    let conn = &data.conn;
    let uid = jwt.user_id;

    let user: Option<serde_json::Value> = Users::find_by_id(uid)
        .select_only()
        .columns([
            users::Column::UserName,
            users::Column::FullName,
            users::Column::Email,
            users::Column::IsStaff,
        ])
        .into_json()
        .one(conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(user)
}

#[get("/users")]
#[has_any_role("SUPERUSER", "STAFF", "INTERN")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let users: Vec<serde_json::Value> = Users::find()
        .select_only()
        .columns([
            users::Column::Id,
            users::Column::UserName,
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

#[post("/user")]
#[has_any_role("SUPERUSER", "STAFF")]
async fn create_user(data: web::Data<AppState>, obj: web::Json<User>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let mut user: ActiveModel = ActiveModel {
        user_name: Set(obj.user_name.to_owned()),
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
    let pass = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    user.send_password(&obj.email, &pass).await;
    user.set_id();
    user.rc_create_user(&obj.full_name, &obj.email, &pass, &obj.user_name)
        .await;
    user.nc_create_user(&obj.full_name, &obj.email, &pass, &obj.user_name)
        .await;
    user.send_invitation(&obj.email).await;
    user.encrypt(pass.to_string());
    user.insert(conn).await.unwrap();

    HttpResponse::Ok().await
}

#[put("/user")]
#[has_any_role("SUPERUSER", "STAFF")]
async fn update_user(data: web::Data<AppState>, obj: web::Json<Model>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let id = obj.id.to_owned();
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

    HttpResponse::Ok().await
}

#[delete("/user")]
#[has_any_role("SUPERUSER", "STAFF")]
async fn delete_user(data: web::Data<AppState>, obj: web::Json<DeleteUser>) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let user: Option<users::Model> = Users::find()
        .filter(users::Column::UserName.contains(&obj.user_name))
        .one(conn)
        .await
        .unwrap();
    let user: Model = user.unwrap();
    users::Entity::delete(user.into_active_model())
        .exec(conn)
        .await
        .unwrap();

    HttpResponse::Ok().await
}

#[post("/change_password")]
#[has_any_role("SUPERUSER", "STAFF", "INTERN")]
async fn change_password(
    data: web::Data<AppState>,
    obj: web::Json<ChangePassword>,
) -> impl Responder {
    let conn: &DatabaseConnection = &data.conn;
    let user: Option<users::Model> = Users::find()
        .filter(users::Column::Email.contains(&obj.email))
        .one(conn)
        .await
        .unwrap();

    let user: Model = user.unwrap();

    let input_old_hash = argon2::hash_encoded(
        obj.old_password.trim().as_bytes(),
        user.salt.as_bytes(),
        &ARGON2_CONFIG,
    )
    .unwrap();

    if !user.password_hash.eq(&input_old_hash) {
        return HttpResponse::BadRequest().json("Invalid old password");
    }

    let input_new_hash = argon2::hash_encoded(
        obj.new_password.trim().as_bytes(),
        user.salt.as_bytes(),
        &ARGON2_CONFIG,
    )
    .unwrap();

    let mut user: ActiveModel = user.into();

    user.password_hash = Set(input_new_hash.to_owned());

    user.update(conn).await.unwrap();

    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(status)
        .service(get_users)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
        .service(login)
        .service(logout)
        .service(get_me)
        .service(change_password);
}
