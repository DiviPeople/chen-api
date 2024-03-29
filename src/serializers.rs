use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: Uuid,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePassword {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}
