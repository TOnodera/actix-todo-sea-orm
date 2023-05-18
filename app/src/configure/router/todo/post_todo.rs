use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    configure::router::error_response,
    domain::{repository::TodoRepository, todo::Todo, AppState},
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
        Ok(id) => HttpResponse::Created().json(json!({ "id": id })),
        // それ以外の場合はエラーレスポンスを生成する
        Err(error) => error_response(error),
    }
}
