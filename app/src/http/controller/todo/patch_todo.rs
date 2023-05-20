use actix_web::{
    patch,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    domain::{repository::TodoRepositoryTrait, AppState},
    http::{request::PatchTodoRequest, response::error_response}, infrastructure::resporitory::TodoRepository,
};

/// Patch /todo
/// todoを更新する
#[patch("/todo/{id}")]
async fn handler(
    data: web::Data<AppState>,
    path_params: web::Path<i32>,
    todo: web::Json<PatchTodoRequest>,
) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone());
    let id = path_params.into_inner();
    match repository.update(id, &todo.title, &todo.body).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => error_response(e),
    }
}
