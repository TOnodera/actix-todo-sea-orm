use actix_web::{HttpServer, App};

mod types;
mod configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new().configure(configure::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}