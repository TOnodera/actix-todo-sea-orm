use app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = app::configure::env()?;
    let pool = app::get_connection(&env.database_url).await;
    app::launch(pool).await
}
