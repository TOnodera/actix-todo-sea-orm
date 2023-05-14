use actix_web::{
    patch,
    web::{self},
    HttpResponse, Responder,
};

use crate::domain::todo::Todo;

/// Patch /todo
/// todoを更新する
#[patch("/todo")]
async fn handler(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo)
}
