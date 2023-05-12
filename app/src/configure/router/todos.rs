use actix_web::{get, post, patch, delete, web::{self, ServiceConfig}, Responder, HttpResponse};

use crate::types::Todo;

/// Get /todos
/// todo一覧を返す 
#[get("/todos")]
async fn get_todos() -> impl Responder {
    HttpResponse::Ok().body("/todos")
}

/// Get /todo/{id}
/// idで指定されたtodoを取得する
#[get("/todo/{id}")]
async fn get_todo(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("/todo/{}", id))
}

/// Post /todo
/// todoを登録する
#[post("/todo")]
async fn post_todo(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo)
}

/// Patch /todo
/// todoを更新する
#[patch("/todo")]
async fn patch_todo(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo)
}

/// Delete /todo/{id}
/// idで指定されたtodoを削除する
#[delete("/todo/{id}")]
async fn delete_todo(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", id))
}

/// /todos,/todoのルートを設定
pub fn todos_route(cfg: &mut ServiceConfig) {
    cfg
        .service(get_todos)
        .service(get_todo)
        .service(post_todo)
        .service(patch_todo)
        .service(delete_todo);
}