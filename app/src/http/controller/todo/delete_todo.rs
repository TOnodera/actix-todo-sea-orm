use actix_web::{delete, web, HttpResponse, Responder};

use crate::{
    domain::{todo::TodoDomain, AppState},
    http::response::error_response,
    infrastructure::resporitory::TodoRepository,
};

/// Delete /todo/{id}
/// idで指定されたtodoを削除する
#[delete("/todo/{id}")]
async fn handler(data: web::Data<AppState>, path_params: web::Path<i32>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone());
    let todo = TodoDomain::new(repository);
    let id = path_params.into_inner();
    match todo.delete(id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => error_response(e),
    }
}
