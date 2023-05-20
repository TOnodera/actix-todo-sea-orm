use chrono::FixedOffset;
use sea_orm::DatabaseConnection;

pub mod repository;
pub mod todo;
pub mod value;

// 環境変数用の構造体
pub struct Env {
    pub database_url: String,
}
impl Env {
    pub fn new(database_url: &str) -> Self {
        Self {
            database_url: database_url.to_string(),
        }
    }
}

// アプリケーション全体で必要な状態
pub struct AppState {
    pub db: DatabaseConnection,
    pub tz: FixedOffset,
}
