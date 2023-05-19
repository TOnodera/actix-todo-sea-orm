use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    configure::response::PostTodoResponse,
    domain::{repository::TodoRepository, todo::Todo, AppState},
    router::error_response,
};

/// Post /todo
/// todoを登録する
#[post("/todo")]
async fn handler(data: web::Data<AppState>, todo: web::Json<Todo>) -> impl Responder {
    // リポジトリ生成
    let repository = TodoRepository::new(data.db.clone(), data.tz.clone());
    // 登録実行
    match repository.create(&todo.title, &todo.body).await {
        // OkならIDを返す
        Ok(id) => HttpResponse::Created().json(PostTodoResponse::new(id)),
        // それ以外の場合はエラーレスポンスを生成する
        Err(error) => error_response(error),
    }
}
