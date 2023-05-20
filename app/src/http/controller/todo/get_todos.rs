use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    domain::{todo::TodoDomain, AppState},
    http::response::error_response,
    infrastructure::resporitory::TodoRepository,
};

/// Get /todos
/// todo一覧を返す
#[get("/todos")]
async fn handler(data: web::Data<AppState>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone());
    let todo = TodoDomain::new(repository);
    match todo.get_all().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => error_response(e),
    }
}
