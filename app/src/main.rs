use actix_web::{get, Responder, HttpResponse, HttpServer, App};

#[get("/todos")]
async fn get_todos() -> impl Responder {
    HttpResponse::Ok().body("/todos")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .service(get_todos)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}