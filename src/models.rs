use serde::Serialize;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}
