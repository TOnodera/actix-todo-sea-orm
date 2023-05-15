use actix_web::{web, App, HttpServer};
use domain::AppState;
use logger::log;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub mod configure;
pub mod domain;
pub mod logger;
pub mod types;

pub async fn get_connection(database_url: &str) -> Pool<Postgres> {
    let pool = match PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            log().error("データベースの接続に失敗しました。");
            std::process::exit(1)
        }
    };

    pool
}

pub async fn launch(pool: PgPool) -> std::io::Result<()> {
    log().info("アプリケーションを起動しました。");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(configure::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
