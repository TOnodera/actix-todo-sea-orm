use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

/// Get /todo/{id}
/// idで指定されたtodoを取得する
#[get("/todo/{id}")]
async fn handler(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("/todo/{}", id))
}
