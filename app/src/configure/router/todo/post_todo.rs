use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    domain::{repository::TodoRepository, todo::Todo, AppState},
    types::ApplicationError,
};

/// Post /todo
/// todoを登録する
#[post("/todo")]
async fn handler(data: web::Data<AppState>, todo: web::Json<Todo>) -> impl Responder {
    let repository = TodoRepository::new(&data.db);
    match repository.create(&todo.title, &todo.body).await {
        Ok(id) => HttpResponse::Ok().json(json!({ "id": id })),
        Err(e) => match e {
            ApplicationError::DomainError(e) => {
                return HttpResponse::BadRequest().json(json!({ "message": e }));
            }
            ApplicationError::DatabaseError(e) => {
                return HttpResponse::InternalServerError().json(json!({ "message": e }));
            }
            _ => {
                return HttpResponse::InternalServerError().json(json!({
                    "message": "サーバーエラーが発生しました。"
                }));
            }
        },
    }
}
