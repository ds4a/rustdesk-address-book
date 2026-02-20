use crate::config::Config;
use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
}
