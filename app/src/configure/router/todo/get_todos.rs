use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::domain::AppState;

/// Get /todos
/// todo一覧を返す
#[get("/todos")]
async fn handler(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("/done")
}
