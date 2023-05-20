use actix_web::{
    patch,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    domain::{todo::TodoDomain, AppState},
    http::{request::PatchTodoRequest, response::error_response},
    infrastructure::resporitory::TodoRepository,
};

/// Patch /todo
/// todoを更新する
#[patch("/todo/{id}")]
async fn handler(
    data: web::Data<AppState>,
    path_params: web::Path<i32>,
    request: web::Json<PatchTodoRequest>,
) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone());
    let todo = TodoDomain::new(repository);
    let id = path_params.into_inner();
    match todo.update(id, &request.title, &request.body).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => error_response(e),
    }
}
