use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    domain::{repository::TodoRepository, AppState},
    router::error_response,
};

/// Get /todo/{id}
/// idで指定されたtodoを取得する
#[get("/todo/{id}")]
async fn handler(data: web::Data<AppState>, path_params: web::Path<i32>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone(), data.tz.clone());
    let id = path_params.into_inner();
    match repository.get(id).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => error_response(e),
    }
}
