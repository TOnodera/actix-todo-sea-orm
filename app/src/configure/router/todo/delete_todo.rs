use actix_web::{
    delete,
    web::{self},
    HttpResponse, Responder,
};

/// Delete /todo/{id}
/// idで指定されたtodoを削除する
#[delete("/todo/{id}")]
async fn handler(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", id))
}
