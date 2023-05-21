use actix_web::{web, App, HttpServer};
use app::configure;
use app::domain::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = configure::env()?;
    let tz = configure::tz();
    let db = sea_orm::Database::connect(env.database_url)
        .await
        .expect("データベース接続に失敗しました。");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone(), tz }))
            .configure(configure::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
