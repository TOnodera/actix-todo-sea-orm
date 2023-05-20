use actix_web::{web, App, HttpServer};
use app::configure;
use app::domain::AppState;
use app::logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = configure::env()?;
    let tz = configure::tz();
    let log = logger::log();
    let db = sea_orm::Database::connect(env.database_url)
        .await
        .expect("データベース接続に失敗しました。");

    log.info("アプリケーションを起動しました。");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: db.clone(),
                tz,
                log: log.clone(),
            }))
            .configure(configure::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
