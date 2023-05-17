use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    configure::router::error_response,
    domain::{repository::TodoRepository, AppState},
};

/// Get /todos
/// todo一覧を返す
#[get("/todos")]
async fn handler(data: web::Data<AppState>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone());
    match repository.gets_all().await {
        Ok(todos) => HttpResponse::Ok().json(json!({ "todos": todos })),
        Err(e) => error_response(e),
    }
}
