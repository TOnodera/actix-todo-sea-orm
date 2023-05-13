use sqlx::PgPool;

pub mod todo;
pub mod repository;

// 環境変数用の構造体
pub struct Env {
    pub database_url: String
}
impl Env {
    pub fn new(DATABASE_URL: &str) -> Self {
        Self {
            database_url: DATABASE_URL.to_string()
        }
    }
}

// アプリケーション全体で必要な状態
pub struct AppState {
    pub db: PgPool
}