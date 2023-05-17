use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    domain::{
        repository::{self, TodoRepository},
        todo::Todo,
        AppState,
    },
    types::ApplicationError,
};

/// Post /todo
/// todoを登録する
#[post("/todo")]
async fn handler(data: web::Data<AppState>, todo: web::Json<Todo>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone());
    if let Err(e) = repository.create(&todo.title, &todo.body).await {
        return HttpResponse::Ok().body("error");
    }
    HttpResponse::Ok().body("post /todo")
}
