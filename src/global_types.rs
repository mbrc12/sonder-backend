pub type DbPool = sqlx::Pool<sqlx::sqlite::Sqlite>;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Secrets {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub secrets: Secrets
}
