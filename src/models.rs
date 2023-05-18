use serde::Serialize;
use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}
