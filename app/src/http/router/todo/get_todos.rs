use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    domain::{repository::TodoRepository, AppState},
    http::router::error_response,
};

/// Get /todos
/// todo一覧を返す
#[get("/todos")]
async fn handler(data: web::Data<AppState>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone(), data.tz.clone());
    match repository.gets_all().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => error_response(e),
    }
}
