use actix_web::{
    patch,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    domain::{repository::TodoRepository, todo::Todo, AppState},
    router::error_response,
};

/// Patch /todo
/// todoを更新する
#[patch("/todo/{id}")]
async fn handler(
    data: web::Data<AppState>,
    path_params: web::Path<i32>,
    todo: web::Json<Todo>,
) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone(), data.tz.clone());
    let id = path_params.into_inner();
    match repository.update(id, todo.into()).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => error_response(e),
    }
}
