#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use app::{configure, domain::AppState};
    use chrono::FixedOffset;
    use migration::MigratorTrait;
    use sea_orm::DatabaseConnection;

    /// テスト用のDBを作成してマイグレーションを実行する
    async fn setup() -> (FixedOffset, DatabaseConnection) {
        let tz = configure::tz();
        let db = sea_orm::Database::connect("sqlite::memory:")
            .await
            .expect("テストデータベースの接続に失敗しました。");
        migration::Migrator::up(&db, None).await;
        (tz, db)
    }

    /// テスト用のDBをリセット
    async fn tear_down(db: &DatabaseConnection) {
        migration::Migrator::reset(db).await;
    }

    #[actix_web::test]
    async fn get_todos() {
        let (tz, db) = setup().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { db: db.clone(), tz }))
                .configure(configure::config),
        )
        .await;

        let request = test::TestRequest::get().uri("/todos").to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());

        tear_down(&db).await;
    }
}
