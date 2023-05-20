use app::{
    configure,
    logger::{self, Logger},
};
use chrono::FixedOffset;
use migration::MigratorTrait;
use sea_orm::DatabaseConnection;

/// テスト用のDBを作成してマイグレーションを実行する
pub async fn setup() -> (FixedOffset, DatabaseConnection, Logger) {
    let tz = configure::tz();
    let log = logger::log();
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("テストデータベースの接続に失敗しました。");
    migration::Migrator::up(&db, None)
        .await
        .expect("テストDBのマイグレーションに失敗しました。");
    (tz, db, log)
}

/// テスト用のDBをリセット
pub async fn tear_down(db: &DatabaseConnection) {
    migration::Migrator::reset(db)
        .await
        .expect("テストDBのリセットに失敗しました。");
}
