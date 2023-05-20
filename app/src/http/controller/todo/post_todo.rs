use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    domain::{repository::TodoRepository, AppState},
    http::response::error_response,
    http::{request::PostTodoRequest, response::PostTodoResponse},
};

/// Post /todo
/// todoを登録する
#[post("/todo")]
async fn handler(data: web::Data<AppState>, todo: web::Json<PostTodoRequest>) -> impl Responder {
    // リポジトリ生成
    let repository = TodoRepository::new(data.db.clone());
    // 登録実行
    match repository.create(&todo.title, &todo.body).await {
        // OkならIDを返す
        Ok(id) => HttpResponse::Ok().json(PostTodoResponse::new(id)),
        // それ以外の場合はエラーレスポンスを生成する
        Err(error) => error_response(error),
    }
}
