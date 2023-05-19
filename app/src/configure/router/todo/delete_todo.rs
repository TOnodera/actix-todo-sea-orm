use actix_web::{
    delete,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    configure::router::error_response,
    domain::{repository::TodoRepository, AppState},
};

/// Delete /todo/{id}
/// idで指定されたtodoを削除する
#[delete("/todo/{id}")]
async fn handler(data: web::Data<AppState>, path_params: web::Path<i32>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone(), data.tz.clone());
    let id = path_params.into_inner();
    match repository.delete(id).await {
        Ok(rows_affected) => HttpResponse::Ok().finish(),
        Err(e) => error_response(e),
    }
}
