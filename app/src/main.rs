use actix_web::{web, App, HttpServer};
use domain::AppState;
use sqlx::postgres::PgPoolOptions;

mod configure;
mod domain;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = configure::env()?;
    let pool = match PgPoolOptions::new()
        .max_connections(50)
        .connect(&env.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => std::process::exit(1),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(configure::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
